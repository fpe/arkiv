use axum::{extract, response::Html};
use fourchan::Post;
use sqlx::SqlitePool;
use tera::{Tera, Context};

use crate::{error::{AppError, any_error}, Pagination, THREADS_PER_PAGE};


pub async fn get_board(
    extract::Path(board): extract::Path<String>,
    extract::Query(pagination): extract::Query<Pagination>,
    extract::Extension(pool): extract::Extension<SqlitePool>,
    extract::Extension(t): extract::Extension<Tera>,
) -> Result<Html<String>, AppError> {
    let threads = if let Some(before) = pagination.before {
        query_as!(
            Post,
            r#"SELECT * FROM posts WHERE resto = 0 AND board = ? AND no > ? ORDER BY no DESC LIMIT ?"# ,
            board,
            before,
            THREADS_PER_PAGE
        )
        .fetch_all(&pool)
        .await
    } else if let Some(after) = pagination.after {
        query_as!(
            Post,
            r#"SELECT * FROM posts WHERE resto = 0 AND board = ? AND no < ? ORDER BY no DESC LIMIT ?"# ,
            board,
            after,
            THREADS_PER_PAGE,
        )
        .fetch_all(&pool)
        .await
    } else {
        query_as!(
            Post,
            r#"SELECT * FROM posts WHERE resto = 0 AND board = ? ORDER BY no DESC LIMIT ?"# ,
            board,
            THREADS_PER_PAGE
        )
        .fetch_all(&pool)
        .await
    }
    .map_err(any_error)?;

    let mut context = Context::new();
    context.insert("threads", &threads);
    context.insert("board", &board);
    context.insert("first_thread", &threads.get(0).map(|t| t.no));
    context.insert("last_thread", &threads.iter().last().map(|t| t.no));

    Ok(Html(t.render("board.html", &context).map_err(any_error)?))
}

