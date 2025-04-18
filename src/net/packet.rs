/// 패킷의 기본 구조를 나타내는 구조체입니다.
pub struct Packet {
    pub data: Vec<u8>,
}

impl Packet {
    /// 새로운 패킷을 생성합니다.
    pub fn new(data: Vec<u8>) -> Self {
        Packet { data }
    }

    /// 패킷의 데이터를 반환합니다.
    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    /// 패킷의 데이터를 설정합니다.
    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }
}
