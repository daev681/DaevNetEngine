// src/engine/runtime.rs
use std::sync::Arc;
use tokio::runtime::{Builder, Runtime};
use crate::net::tcp::listener::start_tcp_server;

pub struct DaevRuntime {
    runtime: Arc<Runtime>,
}

impl DaevRuntime {
    pub fn new(worker_threads: usize) -> Self {
        let runtime = Builder::new_multi_thread()
            .worker_threads(worker_threads)  // 사용할 스레드 수 설정 (예: CPU 수 기반)
            .enable_all()
            .build()
            .expect("Failed to build Tokio runtime");

        DaevRuntime {
            runtime: Arc::new(runtime),
        }
    }

    pub fn start(&self) {
        let rt = self.runtime.clone();
        rt.block_on(async {
            // TCP 서버 시작 (예: 기본 포트)
            start_tcp_server("127.0.0.1", 8080).await.unwrap();
        });
    }
}