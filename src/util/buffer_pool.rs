use std::sync::{Mutex, Arc};
use std::collections::VecDeque;

pub struct BufferPool {
    pool: Mutex<VecDeque<Vec<u8>>>,
    buffer_size: usize,
}

impl BufferPool {
    pub fn new(buffer_size: usize, capacity: usize) -> Self {
        let mut deque = VecDeque::with_capacity(capacity);
        for _ in 0..capacity {
            deque.push_back(vec![0; buffer_size]);
        }

        BufferPool {
            pool: Mutex::new(deque),
            buffer_size,
        }
    }

    pub fn get_buf(&self) -> Vec<u8> {
        self.pool.lock().unwrap().pop_front().unwrap_or_else(|| vec![0; self.buffer_size])
    }

    pub fn release_buf(&self, mut buf: Vec<u8>) {
        buf.clear();
        self.pool.lock().unwrap().push_back(buf);
    }
}

// 전역 싱글톤
use once_cell::sync::Lazy;

pub static BUFFER_POOL: Lazy<BufferPool> = Lazy::new(|| BufferPool::new(1024, 100));
