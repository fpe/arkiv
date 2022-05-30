use crate::error::{any_error, AppError};
use axum::{extract, response::Html};
use fourchan::Post;
use http::StatusCode;
use tera::Tera;

pub async fn get_thread(
    extract::Path((board, id)): extract::Path<(String, i64)>,
    extract::Extension(pool): extract::Extension<sqlx::SqlitePool>,
    extract::Extension(t): extract::Extension<Tera>,
) -> Result<Html<String>, AppError> {
    let posts = query_as!(
        Post,
        r#"
        SELECT * FROM posts WHERE board = ? AND (no = ? OR resto = ?)
        "#,
        board,
        id,
        id
    )
    .fetch_all(&pool)
    .await
    .map_err(any_error)?;

    if posts.is_empty() {
        return Err(AppError::Status(StatusCode::NOT_FOUND));
    }

    let mut context = tera::Context::new();
    context.insert("board", &board);
    context.insert("posts", &posts);

    Ok(Html(t.render("thread.html", &context).map_err(any_error)?))
}
