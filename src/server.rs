//! Serving the router on TCP or on a Unix socket, until SIGTERM or Ctrl-C.

use std::io::ErrorKind;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use axum::Router;
use tokio::net::{TcpListener, UnixListener};
use tokio::signal::unix::{SignalKind, signal};

use crate::config::ListenOn;

/// Serves the application until a shutdown signal, then lets the open requests finish.
///
/// # Errors
/// Fails when the address or socket cannot be bound, or when serving fails.
pub async fn serve(router: Router, listen_on: &ListenOn) -> std::io::Result<()> {
    match listen_on {
        ListenOn::Tcp(address) => {
            let listener = TcpListener::bind(address).await?;
            tracing::info!("listening on http://{address}");
            axum::serve(listener, router)
                .with_graceful_shutdown(shutdown_signal())
                .await
        }
        ListenOn::UnixSocket(path) => serve_on_socket(router, path).await,
    }
}

/// Binds the socket world-writable (nginx runs under another uid) and removes it once stopped.
async fn serve_on_socket(router: Router, path: &Path) -> std::io::Result<()> {
    remove_socket(path)?;
    let listener = UnixListener::bind(path)?;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o666))?;
    tracing::info!("listening on unix socket {}", path.display());
    let served = axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await;
    if let Err(error) = remove_socket(path) {
        tracing::warn!(%error, "could not remove the socket {}", path.display());
    }
    served
}

/// Removes a socket file, stale or ours, if there is one.
fn remove_socket(path: &Path) -> std::io::Result<()> {
    match std::fs::remove_file(path) {
        Err(error) if error.kind() != ErrorKind::NotFound => Err(error),
        _ => Ok(()),
    }
}

/// Resolves on SIGTERM (systemd stopping the service) or Ctrl-C.
async fn shutdown_signal() {
    let terminate = async {
        match signal(SignalKind::terminate()) {
            Ok(mut terminate) => {
                terminate.recv().await;
            }
            Err(error) => {
                tracing::error!(%error, "cannot listen for SIGTERM");
                std::future::pending::<()>().await;
            }
        }
    };
    let interrupt = async {
        if let Err(error) = tokio::signal::ctrl_c().await {
            tracing::error!(%error, "cannot listen for Ctrl-C");
            std::future::pending::<()>().await;
        }
    };
    tokio::select! {
        () = terminate => {}
        () = interrupt => {}
    }
    tracing::info!("shutting down");
}
