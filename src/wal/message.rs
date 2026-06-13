use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Key(String);

impl Key {
    pub fn from(key: &str) -> Self {
        Key(key.to_string())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Value(String);

impl Value {
    pub fn from(value: &str) -> Self {
        Value(value.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Set { key: Key, value: Value },
}

impl Message {
    pub fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_string(&self).unwrap().into_bytes()
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        serde_json::from_slice::<Message>(&bytes).unwrap()
    }
}

impl Message {
    pub fn set(key: Key, value: Value) -> Self {
        return Message::Set { key, value };
    }
}
