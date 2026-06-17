use key_value_store::key_value_store_server::{KeyValueStore, KeyValueStoreServer};
use key_value_store::{
    ClearRequest, ClearResponse, DeleteRequest, DeleteResponse, GetRequest, GetResponse,
    SetRequest, SetResponse,
};
use kv_store_lib::config::Config;
use kv_store_lib::wal::message::{Key, Value};
use kv_store_lib::{Keyable, LogStructuredMergeTree};
use std::path::Path;
use tokio::sync::RwLock;
use tonic::{Request, Response, Status, transport::Server};

pub mod key_value_store {
    tonic::include_proto!("kv");
}

#[derive(Debug)]
pub struct MyKeyValueStore {
    store: RwLock<LogStructuredMergeTree>,
}

impl MyKeyValueStore {
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        Self {
            store: RwLock::new(LogStructuredMergeTree::new(base_path)),
        }
    }
}

#[tonic::async_trait]
impl KeyValueStore for MyKeyValueStore {
    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let lock = self.store.read().await;
        let value = lock.get(&Key::from(request.into_inner().key.as_str()));
        match value {
            Some(value) => Ok(Response::new(GetResponse {
                value: value.to_string(),
            })),
            None => Err(Status::not_found("Requested key not found")),
        }
    }
    async fn set(&self, request: Request<SetRequest>) -> Result<Response<SetResponse>, Status> {
        let mut lock = self.store.write().await;
        let inner = request.into_inner();
        match lock.put(
            Key::from(inner.key.as_str()),
            Value::from(inner.value.as_str()),
        ) {
            Ok(_) => Ok(Response::new(SetResponse {})),
            Err(err) => Err(Status::from_error(Box::new(err))),
        }
    }
    async fn delete(
        &self,
        request: Request<DeleteRequest>,
    ) -> Result<Response<DeleteResponse>, Status> {
        let mut lock = self.store.write().await;
        let inner = request.into_inner();
        match lock.del(Key::from(inner.key.as_str())) {
            Ok(_) => Ok(Response::new(DeleteResponse {})),
            Err(err) => Err(Status::from_error(Box::new(err))),
        }
    }
    async fn clear(
        &self,
        _request: Request<ClearRequest>,
    ) -> Result<Response<ClearResponse>, Status> {
        let mut lock = self.store.write().await;
        match lock.clear() {
            Ok(_) => Ok(Response::new(ClearResponse {})),
            Err(err) => Err(Status::from_error(Box::new(err))),
        }
    }
}

async fn interrupt_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install SIGINT handler")
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

async fn shutdown_signal() {
    tokio::select! {
        _ = interrupt_signal() => println!("Received SIGINT"),
        _ = terminate_signal() => println!("Received SIGTERM"),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse();
    let kv_store = MyKeyValueStore::new(config.base_path);

    let addr = "[::1]:50051".parse()?;
    Server::builder()
        .add_service(KeyValueStoreServer::new(kv_store))
        .serve_with_shutdown(addr, shutdown_signal())
        .await?;

    println!("gRPC server has stopped. Shutting down...");

    Ok(())
}
