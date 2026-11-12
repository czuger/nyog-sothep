//! Runtime settings read from the environment.

use std::path::PathBuf;

const DEFAULT_DATABASE_URL: &str = "sqlite://data/nyog_sothep.db";
const DEFAULT_BIND_ADDR: &str = "127.0.0.1:3000";
const DATABASE_FILE: &str = "nyog_sothep.db";

/// SQLite URL, wins over the data directory.
const DATABASE_URL_VAR: &str = "DATABASE_URL";
/// Directory holding the database file (set by `cargo deploy` from `data-dirs`).
const DATA_DIR_VAR: &str = "NYOG_SOTHEP_DATA_DIR";
/// Unix socket to listen on instead of TCP (set by `cargo deploy`).
const SOCKET_VAR: &str = "NYOG_SOTHEP_SOCKET";
/// TCP address, used when no socket is given.
const BIND_ADDR_VAR: &str = "BIND_ADDR";

/// Where the server accepts connections.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListenOn {
    /// A TCP address, for local development.
    Tcp(String),
    /// A Unix socket path, behind the deployed nginx.
    UnixSocket(PathBuf),
}

/// Server settings.
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// SQLite URL, the file is created if missing.
    pub database_url: String,
    /// Where the server listens.
    pub listen_on: ListenOn,
}

impl AppConfig {
    /// Reads the settings from environment variables, falling back to local defaults.
    pub fn from_env() -> AppConfig {
        AppConfig::from_lookup(|name| std::env::var(name).ok())
    }

    /// Builds the settings from a variable lookup; an empty variable counts as unset.
    fn from_lookup(lookup: impl Fn(&str) -> Option<String>) -> AppConfig {
        let var = |name: &str| lookup(name).filter(|value| !value.is_empty());
        let database_url = match (var(DATABASE_URL_VAR), var(DATA_DIR_VAR)) {
            (Some(url), _) => url,
            (None, Some(dir)) => format!("sqlite://{}/{DATABASE_FILE}", dir.trim_end_matches('/')),
            (None, None) => DEFAULT_DATABASE_URL.to_string(),
        };
        let listen_on = match var(SOCKET_VAR) {
            Some(path) => ListenOn::UnixSocket(PathBuf::from(path)),
            None => {
                ListenOn::Tcp(var(BIND_ADDR_VAR).unwrap_or_else(|| DEFAULT_BIND_ADDR.to_string()))
            }
        };
        AppConfig {
            database_url,
            listen_on,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config(vars: &[(&str, &str)]) -> AppConfig {
        AppConfig::from_lookup(|name| {
            vars.iter()
                .find(|(key, _)| *key == name)
                .map(|(_, value)| value.to_string())
        })
    }

    #[test]
    fn defaults_to_local_tcp_and_database() {
        let config = config(&[]);
        assert_eq!(config.database_url, DEFAULT_DATABASE_URL);
        assert_eq!(
            config.listen_on,
            ListenOn::Tcp(DEFAULT_BIND_ADDR.to_string())
        );
    }

    #[test]
    fn the_socket_replaces_tcp() {
        let config = config(&[
            ("NYOG_SOTHEP_SOCKET", "/run/app.sock"),
            ("BIND_ADDR", "0.0.0.0:80"),
        ]);
        assert_eq!(
            config.listen_on,
            ListenOn::UnixSocket("/run/app.sock".into())
        );
    }

    #[test]
    fn the_data_dir_holds_the_database_unless_a_url_is_given() {
        let in_dir = config(&[("NYOG_SOTHEP_DATA_DIR", "/srv/app/data/db/")]);
        assert_eq!(
            in_dir.database_url,
            "sqlite:///srv/app/data/db/nyog_sothep.db"
        );
        let explicit = config(&[
            ("NYOG_SOTHEP_DATA_DIR", "/srv/app/data/db"),
            ("DATABASE_URL", "sqlite://other.db"),
        ]);
        assert_eq!(explicit.database_url, "sqlite://other.db");
    }

    #[test]
    fn empty_variables_count_as_unset() {
        let config = config(&[("NYOG_SOTHEP_SOCKET", ""), ("DATABASE_URL", "")]);
        assert_eq!(config.database_url, DEFAULT_DATABASE_URL);
        assert_eq!(
            config.listen_on,
            ListenOn::Tcp(DEFAULT_BIND_ADDR.to_string())
        );
    }
}
