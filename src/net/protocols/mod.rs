// src/net/protocols/mod.rs
pub mod protobuf;
use serde::{Deserialize, Serialize};
use serde_json::{Value};
use async_trait::async_trait;

#[async_trait]
pub trait Protocol<T> {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &[u8]) -> Self;
    fn get_data(&self) -> T;  
}
