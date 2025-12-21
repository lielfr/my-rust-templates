pub(crate) async fn graceful_shutdown() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler for SIGTERM")
            .recv()
            .await;
    };

    let interrupt = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())
            .expect("failed to install signal handler for SIGINT")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
            _ = terminate => {},
            _ = interrupt => {},
    }

    tracing::info!("Received termination signal, shutting down");
}
