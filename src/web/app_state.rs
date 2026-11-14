use crate::db::Store;

/// State shared by every request handler.
#[derive(Clone)]
pub struct AppState {
    /// The database.
    pub store: Store,
}
