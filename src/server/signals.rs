pub async fn shutdown_signal() {
    tokio::select! {
        () = interrupt_signal() => println!("Received SIGINT"),
        () = terminate_signal() => println!("Received SIGTERM"),
    };
}

async fn interrupt_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install SIGINT handler");
}

async fn terminate_signal() {
    #[cfg(unix)]
    tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        .expect("failed to install SIGTERM handler")
        .recv()
        .await;
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
}
