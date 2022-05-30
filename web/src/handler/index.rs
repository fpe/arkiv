use axum::{extract, response::Html};
use sqlx::SqlitePool;
use tera::{Tera, Context};

use crate::{BoardListing, error::{AppError, any_error}};

pub async fn get_index(
    extract::Extension(pool): extract::Extension<SqlitePool>,
    extract::Extension(t): extract::Extension<Tera>,
) -> Result<Html<String>, AppError> {
    let boards = query_as!(BoardListing,
        r#"SELECT board, ifnull(count(no), 0) as thread_count FROM posts WHERE resto = 0 GROUP BY board"#,
    )
    .fetch_all(&pool)
    .await
    .map_err(any_error)?;

    let mut context = Context::new();
    context.insert("boards", &boards);

    Ok(Html(t.render("index.html", &context).map_err(any_error)?))
}
