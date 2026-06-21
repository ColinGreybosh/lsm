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
    #[must_use]
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
    #[must_use]
    pub fn from(value: &str) -> Self {
        Value(value.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Message {
    Set { key: Key, value: Value },
    Delete { key: Key },
    Clear {},
}

impl Message {
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_string(&self).unwrap().into_bytes()
    }

    #[must_use]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        serde_json::from_slice::<Message>(bytes).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_serialize_key() {
        let key = Key::from("my_key");
        std::assert_eq!(key.to_string(), "my_key");
    }

    #[test]
    fn should_serialize_value() {
        let value = Value::from("my_value");
        std::assert_eq!(value.to_string(), "my_value");
    }

    #[test]
    fn deserialized_clear_message_should_equal_original_message() {
        let original_message = Message::Clear {};
        let serialized_message = original_message.to_bytes();
        let deserialized_message = Message::from_bytes(&serialized_message);
        std::assert_eq!(original_message, deserialized_message);
    }

    #[test]
    fn deserialized_set_message_should_equal_original_message() {
        let original_message = Message::Set {
            key: Key::from("my_key"),
            value: Value::from("my_value"),
        };
        let serialized_message = original_message.to_bytes();
        let deserialized_message = Message::from_bytes(&serialized_message);
        std::assert_eq!(original_message, deserialized_message);
    }

    #[test]
    fn deserialized_delete_message_should_equal_original_message() {
        let original_message = Message::Delete {
            key: Key::from("my_key"),
        };
        let serialized_message = original_message.to_bytes();
        let deserialized_message = Message::from_bytes(&serialized_message);
        std::assert_eq!(original_message, deserialized_message);
    }
}
