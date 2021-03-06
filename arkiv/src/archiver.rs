use crate::config::{Config, CustomRegex};
use bytes::Bytes;
use fourchan::{BoardsResponse, Post, PostAttachment, ThreadResponse};
use futures::Future;
use scraper::{Html, Node};
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::Semaphore;
use tracing::{debug, info, trace, trace_span, warn};

#[derive(Clone)]
pub struct Archiver<S: arkiv_storage::Storage> {
    client: fourchan::Client,
    pool: sqlx::SqlitePool,
    storage: S,
    config: Config,
    semaphore: Arc<Semaphore>,
}

impl<S> Archiver<S>
where
    S: arkiv_storage::Storage,
{
    pub fn new(pool: sqlx::SqlitePool, storage: S, config: Config) -> Self {
        let client = fourchan::Client::new();
        Archiver {
            client,
            pool,
            storage,
            config,
            semaphore: Arc::new(Semaphore::new(4)),
        }
    }

    #[allow(clippy::missing_errors_doc, clippy::too_many_lines)]
    pub async fn run(self) -> anyhow::Result<()> {
        debug!("archiver running");

        for (board_name, board_cfg) in self
            .config
            .boards
            .iter()
            .map(|(bn, cfg)| (bn.clone(), cfg.clone()))
            .cycle()
        {
            let BoardsResponse { boards } = self.client.get_board_list().await?;

            let board = match boards.iter().find(|bn| bn.board == *board_name) {
                Some(board) => board,
                None => return Ok(()),
            };

            for page in self.client.get_thread_page_list(&board.board).await?.0 {
                debug!(
                    "found {} threads on page {} of {}",
                    page.threads.len(),
                    page.page,
                    &board_name
                );
                'page_loop: for thread_entry in page.threads {
                    debug!("archiving thread no {}", thread_entry.no);

                    let time_started = SystemTime::now();

                    match self
                        .client
                        .get_thread(&board.board, thread_entry.no)
                        .await?
                    {
                        ThreadResponse::Thread(thread) => {
                            if !board_cfg.filters.is_empty() {
                                if let Some(post) = thread.posts.get(0) {
                                    let _span_guard = trace_span!("filter").entered();
                                    let mut filter_match = false;
                                    for CustomRegex(filter) in &board_cfg.filters {
                                        if let Some(sub) = &post.sub {
                                            if filter.is_match(sub) {
                                                filter_match = true;
                                                break;
                                            }
                                        }
                                        if board_cfg.filter_comment {
                                            if let Some(com) = &post.com {
                                                let com_text = Html::parse_document(com)
                                                    .tree
                                                    .nodes()
                                                    .fold(String::new(), |mut s, n| {
                                                        if let Node::Text(t) = n.value() {
                                                            s.push_str(t);
                                                        }
                                                        s
                                                    });
                                                if filter.is_match(&com_text) {
                                                    filter_match = true;
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                    if filter_match ^ board_cfg.reverse_filter {
                                        trace!(?post, "skipping thread");
                                        continue 'page_loop;
                                    }
                                    trace!(?post, "saving thread");
                                }
                            }

                            for post in thread.posts {
                                let permit = Arc::clone(&self.semaphore).acquire_owned().await?;
                                let archiver = self.clone();
                                let board = board.clone();
                                tokio::spawn(async move {
                                    debug!("archiving post no {}", post.no);
                                    archiver.save_post(&post, &board.board).await?;

                                    if let Some(attachment) = post.attachment() {
                                        if board_cfg.full_media {
                                            archiver
                                                .save_attachment(&board.board, &attachment)
                                                .await?;
                                        }
                                        archiver.save_thumbnail(&board.board, &attachment).await?;
                                    }
                                    debug!("archived post no {}", &post.no);

                                    drop(permit);
                                    anyhow::Ok(())
                                });
                            }

                            let elapsed = time_started.elapsed()?.as_secs_f64();
                            info!(
                                "archived thread no {} on /{}/ in {:.2}s",
                                thread_entry.no, &board.board, elapsed
                            );
                        }
                        ThreadResponse::NotModified => {
                            debug!(
                                "thread no {} on /{}/ was not modified",
                                thread_entry.no, &board.board
                            );
                        }
                        ThreadResponse::NotFound => {
                            warn!(
                                "thread no {} on /{}/ could not be found",
                                thread_entry.no, &board.board
                            );
                        }
                    }
                }
            }
            debug!("waiting 10 minutes until next archival");
            tokio::time::sleep(Duration::from_secs(60 * 10)).await;
        }

        Ok(())
    }
    async fn save_file<B>(&self, key: &str, subdir: Option<&str>, body: B) -> anyhow::Result<()>
    where
        B: Future<Output = anyhow::Result<Bytes>>,
    {
        if self.storage.exists(key, subdir).await? {
            debug!("file exists {:?}", (key, subdir));
        } else {
            let body = body.await?;
            self.storage.save(key, subdir, &body).await?;
        }

        Ok(())
    }
    async fn save_attachment(
        &self,
        board: &str,
        attachment: &PostAttachment,
    ) -> anyhow::Result<()> {
        let key = format!("{}{}", &attachment.tim, &attachment.ext);
        let body_fut = self
            .client
            .get_attachment_body(board, attachment.tim, &attachment.ext);
        self.save_file(&key, Some(board), body_fut).await?;
        Ok(())
    }
    async fn save_thumbnail(&self, board: &str, attachment: &PostAttachment) -> anyhow::Result<()> {
        let key = format!("{}s.jpg", &attachment.tim);
        let body_fut = self.client.get_thumbnail_body(board, attachment.tim);
        self.save_file(&key, Some(board), body_fut).await?;
        Ok(())
    }
    async fn save_post(&self, post: &Post, board: &str) -> sqlx::Result<()> {
        query!(
            r#"
            INSERT INTO posts (no, resto, sticky, closed, now, time, name, trip,
                id, capcode, country, country_name, board_flag, flag_name, sub, com,
                tim, filename, ext, fsize, md5, w, h, tn_w,
                tn_h, filedeleted, spoiler, custom_spoiler, replies, images, bumplimit, imagelimit,
                tag, semantic_url, since4pass, unique_ips, m_img, archived, archived_on, board)
            VALUES
            (?, ?, ?, ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(no) DO UPDATE
            SET filedeleted = ?, replies = ?, images = ?, bumplimit = ?,
                imagelimit = ?, unique_ips = ?, archived = ?, archived_on = ?;
            "#,
            post.no,
            post.resto,
            post.sticky,
            post.closed,
            post.now,
            post.time,
            post.name,
            post.trip,
            post.id,
            post.capcode,
            post.country,
            post.country_name,
            post.board_flag,
            post.flag_name,
            post.sub,
            post.com,
            post.tim,
            post.filename,
            post.ext,
            post.fsize,
            post.md5,
            post.w,
            post.h,
            post.tn_w,
            post.tn_h,
            post.filedeleted,
            post.spoiler,
            post.custom_spoiler,
            post.replies,
            post.images,
            post.bumplimit,
            post.imagelimit,
            post.tag,
            post.semantic_url,
            post.since4pass,
            post.unique_ips,
            post.m_img,
            post.archived,
            post.archived_on,
            board,
            post.filedeleted,
            post.replies,
            post.images,
            post.bumplimit,
            post.imagelimit,
            post.unique_ips,
            post.archived,
            post.archived_on,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
