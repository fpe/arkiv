use axum::response::IntoResponse;
use http::StatusCode;

#[derive(Debug)]
pub enum AppError {
    Anyhow(anyhow::Error),
    Status(StatusCode),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status;

        match self {
            AppError::Anyhow(err) => {
                if let Some(sqlx::Error::RowNotFound) = err.downcast_ref::<sqlx::Error>() {
                    status = StatusCode::NOT_FOUND;
                } else {
                    status = StatusCode::INTERNAL_SERVER_ERROR;
                    tracing::error!("internal error occured: {}", err);
                }
            }
            AppError::Status(status_code) => status = status_code,
        }

        status.into_response()
    }
}

pub fn any_error<E: Into<anyhow::Error>>(err: E) -> AppError {
    AppError::Anyhow(err.into())
}
