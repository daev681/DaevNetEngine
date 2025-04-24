use async_trait::async_trait;
use quick_xml::{se::to_string, de::from_str};

use super::Protocol;

/// 제네릭 XML 프로토콜 처리 구조
#[derive(Debug, Clone)]
pub struct XmlProtocol<T> {
    pub data: T,
}

#[async_trait]
impl<T> Protocol<T> for XmlProtocol<T>
where
    T: serde::Serialize + for<'de> serde::Deserialize<'de> + Clone + Send + Sync,
{
    fn serialize(&self) -> Vec<u8> {
        to_string(&self.data)
            .unwrap_or_default()
            .into_bytes()
    }

    fn deserialize(data: &[u8]) -> Self {
        let xml_str = String::from_utf8_lossy(data);
        let parsed = from_str::<T>(&xml_str)
            .unwrap_or_else(|_| panic!("Deserialization failed")); // 실전에서는 Result로 처리
        XmlProtocol { data: parsed }
    }

    fn get_data(&self) -> T {
        self.data.clone()
    }
}
