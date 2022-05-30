use anyhow::Context;
use arkiv_storage::Storage;
use axum::{extract, response::IntoResponse};
use http::{header, HeaderMap};

use crate::error::{AppError, any_error};


pub async fn cdn<S: Storage>(
    extract::Path((board, key)): extract::Path<(String, String)>,
    extract::Extension(storage): extract::Extension<S>,
) -> Result<impl IntoResponse, AppError> {
    let content_type = mime_guess::from_path(&key).first_or_octet_stream();
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_DISPOSITION,
        "inline"
            .parse()
            .context("failed to parse content disposition header")
            .map_err(any_error)?,
    );
    headers.insert(
        header::CONTENT_TYPE,
        content_type
            .to_string()
            .parse()
            .context("failed to parse content type ehader")
            .map_err(any_error)?,
    );

    let body = storage.get(&key, Some(&board)).await.map_err(any_error)?;

    Ok((headers, body))
}

