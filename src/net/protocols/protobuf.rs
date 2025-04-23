use crate::net::protocols::Protocol;
use prost::Message; 

#[derive(prost::Message, Clone)]  
pub struct MyProtoMessage {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(int32, tag = "2")]
    pub score: i32,
}

impl Protocol<MyProtoMessage> for MyProtoMessage {
    fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.encode(&mut buf).unwrap();
        buf
    }

    fn deserialize(data: &[u8]) -> Self {
        MyProtoMessage::decode(data).unwrap()
    }

    fn get_data(&self) -> MyProtoMessage {
        self.clone()  
    }
}
