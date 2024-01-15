// import contextlib
// import time

use crate::config::zk_print;
use crate::my_logging;

// @contextlib.contextmanager
pub fn time_measure(key: &str, should_print: bool, skip: bool) {
    let start = time.time();
    // yield
    let end = time.time();
    let elapsed = end - start;

    if !skip {
        if should_print {
            zk_print("Took {elapsed} s");
        }
        my_logging.data("time_" + key, elapsed);
    }
}

pub struct Timer {
    key: String,
}
// class Timer(object):
impl Timer {
    pub fn new(key: String) -> Self {
        Self { key }
    }

    pub fn __call__(
        &self,
        method: impl FnOnce(Vec<&str>, Vec<&str>) -> String,
    ) -> impl FnOnce(Vec<&str>, Vec<&str>) -> String {
        fn timed(args: Vec<&str>, kw: Vec<&str>) -> String {
            time_measure(self.key);
            method(args, kw)
        }
        timed
    }
}
