//! `img/` and `static/`, compiled into the binary so that a release is a single file (read from
//! disk in debug builds, so edits show without rebuilding).

use axum::extract::Path;
use axum::http::header::{CONTENT_TYPE, ETAG, IF_NONE_MATCH};
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use rust_embed::RustEmbed;

/// The board, the counters and the scanned rules.
#[derive(RustEmbed)]
#[folder = "img/"]
pub struct Images;

/// The stylesheet and the scripts.
#[derive(RustEmbed)]
#[folder = "static/"]
pub struct StaticFiles;

/// `GET /img/{*file}` and `GET /static/{*file}`: an embedded file, with an `ETag` so that the
/// browser's revalidation gets a `304` when it did not change.
pub async fn serve_asset<A: RustEmbed>(Path(file): Path<String>, headers: HeaderMap) -> Response {
    let Some(asset) = A::get(&file) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let hash: String = asset
        .metadata
        .sha256_hash()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    let Ok(etag) = HeaderValue::from_str(&format!("\"{hash}\"")) else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };
    if headers.get(IF_NONE_MATCH) == Some(&etag) {
        return (StatusCode::NOT_MODIFIED, [(ETAG, etag)]).into_response();
    }
    let content_type = HeaderValue::from_str(asset.metadata.mimetype())
        .unwrap_or(HeaderValue::from_static("application/octet-stream"));
    (
        [(CONTENT_TYPE, content_type), (ETAG, etag)],
        asset.data.into_owned(),
    )
        .into_response()
}
