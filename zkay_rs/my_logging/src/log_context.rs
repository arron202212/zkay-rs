// import contextlib
// from typing import List
use lazy_static::lazy_static;
use std::sync::Mutex;
lazy_static! {
    pub static ref FULL_LOG_CONTEXT: Mutex<Vec<String>> = Mutex::new(vec![]);
}

pub struct WithLogContext;
impl WithLogContext {
    pub fn new(key: &str) -> Self {
        FULL_LOG_CONTEXT.lock().unwrap().push(key.to_owned());
        Self
    }
}
impl Drop for WithLogContext {
    fn drop(&mut self) {
        FULL_LOG_CONTEXT.lock().unwrap().pop();
    }
}

// @contextlib.contextmanager
pub fn log_context(key: &str) -> WithLogContext {
    // yield
    WithLogContext::new(key)
}
