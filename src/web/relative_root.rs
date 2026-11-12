use std::convert::Infallible;
use std::fmt;

use axum::extract::FromRequestParts;
use axum::http::request::Parts;

/// The relative path from the requested page back to the application's root (`./`, `../../`…).
///
/// The application is served under a sub-path that the proxy strips, so every link, form action
/// and redirect is written relative to the page: `{root}static/style.css`, `{root}games/…`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelativeRoot(String);

impl RelativeRoot {
    /// The root as seen from a page at `path`, as the router sees it (`/games/1/token`).
    pub fn of_path(path: &str) -> RelativeRoot {
        let depth = path.trim_start_matches('/').matches('/').count();
        if depth == 0 {
            RelativeRoot("./".to_string())
        } else {
            RelativeRoot("../".repeat(depth))
        }
    }
}

impl fmt::Display for RelativeRoot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl<S: Send + Sync> FromRequestParts<S> for RelativeRoot {
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Ok(RelativeRoot::of_path(parts.uri.path()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn climbs_one_level_per_folder_of_the_page() {
        assert_eq!(RelativeRoot::of_path("/").to_string(), "./");
        assert_eq!(RelativeRoot::of_path("/rules").to_string(), "./");
        assert_eq!(
            RelativeRoot::of_path("/games/1/token").to_string(),
            "../../"
        );
        assert_eq!(
            RelativeRoot::of_path("/games/1/token/moves").to_string(),
            "../../../"
        );
        assert_eq!(
            RelativeRoot::of_path("/games/1/token/").to_string(),
            "../../../"
        );
    }
}
