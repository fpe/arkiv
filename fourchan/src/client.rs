use std::{collections::HashMap, sync::Arc};

use bytes::Bytes;
use chrono::{DateTime, Utc};
use http::{header, StatusCode};
use tokio::sync::RwLock;

use crate::{
    board::BoardsResponse,
    thread::{ThreadPageListResponse, ThreadResponse, ThreadResponseInner},
};

#[derive(Debug, Default)]
pub struct Client {
    http_client: reqwest::Client,
    cache: Arc<RwLock<HashMap<i64, DateTime<Utc>>>>,
}

impl Client {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    pub async fn get_board_list(&self) -> anyhow::Result<BoardsResponse> {
        let req = self
            .http_client
            .get("https://a.4cdn.org/boards.json")
            .build()?;
        Ok(self.http_client.execute(req).await?.json().await?)
    }
    pub async fn get_thread_page_list(
        &self,
        board: &str,
    ) -> anyhow::Result<ThreadPageListResponse> {
        let uri = format!("https://a.4cdn.org/{board}/threads.json", board = board);
        let req = self.http_client.get(&uri).build()?;
        Ok(self.http_client.execute(req).await?.json().await?)
    }
    pub async fn get_thread(&self, board: &str, thread_no: i64) -> anyhow::Result<ThreadResponse> {
        let uri = format!(
            "https://a.4cdn.org/{board}/thread/{thread}.json",
            board = &board,
            thread = &thread_no
        );

        let if_modified_since = {
            let cache = self.cache.read().await;
            cache
                .get(&thread_no)
                .map(|date| date.format("%a, %d %b %Y %H:%M:%S GMT").to_string())
        };

        let req = {
            let mut b = self.http_client.get(&uri);
            if let Some(gmt_date) = if_modified_since {
                b = b.header(header::IF_MODIFIED_SINCE, gmt_date);
            }

            b.build()?
        };

        let resp = self.http_client.execute(req).await?;
        if resp.status() == StatusCode::NOT_MODIFIED {
            return Ok(ThreadResponse::NotModified);
        }
        if resp.status() == 404 {
            return Ok(ThreadResponse::NotFound);
        }
        let resp: ThreadResponseInner = resp.json().await?;

        let mut lock = self.cache.write().await;
        lock.insert(thread_no, Utc::now());
        drop(lock);

        Ok(ThreadResponse::Thread(resp))
    }
    pub async fn get_attachment_body(
        &self,
        board: &str,
        tim: i64,
        ext: &str,
    ) -> anyhow::Result<Bytes> {
        let uri = format!("https://i.4cdn.org/{}/{}{}", &board, tim, ext);
        let req = self.http_client.get(&uri).build()?;
        Ok(self.http_client.execute(req).await?.bytes().await?)
    }
    pub async fn get_thumbnail_body(&self, board: &str, tim: i64) -> anyhow::Result<Bytes> {
        let uri = format!("https://i.4cdn.org/{}/{}s.jpg", &board, tim);
        let req = self.http_client.get(&uri).build()?;
        Ok(self.http_client.execute(req).await?.bytes().await?)
    }
}
