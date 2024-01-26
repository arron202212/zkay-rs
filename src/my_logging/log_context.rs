// import contextlib
// from typing import List
use lazy_static::lazy_static;
use std::sync::Mutex;
lazy_static! {
    pub static ref FULL_LOG_CONTEXT: Mutex<Vec<String>> = Mutex::new(vec![]);
}

// @contextlib.contextmanager
pub fn log_context(key: &str) {
    FULL_LOG_CONTEXT.lock().unwrap().push(key.to_owned());
    // yield
    FULL_LOG_CONTEXT.lock().unwrap().pop();
}
