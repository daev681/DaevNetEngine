use DaevNetEngine::engine::runtime::DaevRuntime;

#[tokio::main]
async fn main() {
    let runtime = DaevRuntime::new(8); // 워커 스레드 수 지정
    runtime.start();
}