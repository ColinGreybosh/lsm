use crate::protobuf::key_value_store::key_value_store_server::KeyValueStore;
use crate::protobuf::key_value_store::{
    ClearRequest, ClearResponse, DeleteRequest, DeleteResponse, GetRequest, GetResponse,
    SetRequest, SetResponse,
};
use kv_store_lib::wal::message::{Key, Value};
use kv_store_lib::{Keyable, LogStructuredMergeTree};
use std::path::Path;
use tokio::sync::RwLock;
use tonic::{Request, Response, Status};

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
