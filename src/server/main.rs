use crate::{
    config::Config, handlers::MyKeyValueStore,
    protobuf::key_value_store::key_value_store_server::KeyValueStoreServer,
    signals::shutdown_signal,
};

mod config;
mod handlers;
mod protobuf;
mod signals;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse();
    let kv_store = MyKeyValueStore::new(config.base_path);

    let addr = format!("{}:{}", config.host, config.port).parse()?;
    tonic::transport::Server::builder()
        .add_service(KeyValueStoreServer::new(kv_store))
        .serve_with_shutdown(addr, shutdown_signal())
        .await?;

    println!("gRPC server has stopped. Shutting down...");

    Ok(())
}
