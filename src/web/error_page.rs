use askama::Template;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{Html, IntoResponse, Response};

use crate::error::AppError;
use crate::web::relative_root::RelativeRoot;

const INTERNAL_MESSAGE: &str = "Une erreur interne est survenue. Veuillez réessayer.";

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorPage<'a> {
    root: RelativeRoot,
    message: &'a str,
}

/// The French message of a failed request, turned into a page by [`render_error_pages`], which
/// knows the request's path and so the relative links of the page.
#[derive(Clone)]
struct ErrorMessage(String);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            error if error.is_internal() => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::GameNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::BAD_REQUEST,
        };
        let message = if self.is_internal() {
            tracing::error!(error = ?self, "request failed");
            INTERNAL_MESSAGE.to_string()
        } else {
            self.to_string()
        };
        let mut response = (status, message.clone()).into_response();
        response.extensions_mut().insert(ErrorMessage(message));
        response
    }
}

/// Middleware drawing the error page of every failed request.
pub async fn render_error_pages(request: Request, next: Next) -> Response {
    let root = RelativeRoot::of_path(request.uri().path());
    let response = next.run(request).await;
    let Some(ErrorMessage(message)) = response.extensions().get::<ErrorMessage>().cloned() else {
        return response;
    };
    let status = response.status();
    match (ErrorPage {
        root,
        message: &message,
    })
    .render()
    {
        Ok(page) => (status, Html(page)).into_response(),
        Err(error) => {
            tracing::error!(?error, "error page rendering failed");
            response
        }
    }
}
