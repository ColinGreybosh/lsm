use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Key(String);

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Key {
    pub fn from(key: &str) -> Self {
        Key(key.to_string())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Value(String);

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Value {
    pub fn from(value: &str) -> Self {
        Value(value.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Set { key: Key, value: Value },
    Del { key: Key },
    Clear {},
}

impl Message {
    pub fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_string(&self).unwrap().into_bytes()
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        serde_json::from_slice::<Message>(&bytes).unwrap()
    }
}
