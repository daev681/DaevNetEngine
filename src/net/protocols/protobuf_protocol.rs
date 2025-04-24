use prost::Message;
use async_trait::async_trait;
use super::Protocol;

/// 제네릭 protobuf 프로토콜 구현체
#[derive(Debug, Clone)]
pub struct ProtoProtocol<T> {
    pub data: T,
}

#[async_trait]
impl<T> Protocol<T> for ProtoProtocol<T>
where
    T: Message + Default + Clone + Send + Sync,
{
    fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.data.encode(&mut buf).unwrap();
        buf
    }

    fn deserialize(data: &[u8]) -> Self {
        let decoded = T::decode(data).unwrap();
        ProtoProtocol { data: decoded }
    }

    fn get_data(&self) -> T {
        self.data.clone()
    }
}
