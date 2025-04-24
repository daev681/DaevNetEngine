use prost::Message;

/// 예시: 유저 점수 패킷
#[derive(Message, Clone, Debug)]
pub struct MyProtoMessage {
    #[prost(string, tag = "1")]
    pub id: String,

    #[prost(int32, tag = "2")]
    pub score: i32,
}
