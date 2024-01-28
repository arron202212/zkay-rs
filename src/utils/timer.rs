// import contextlib
// import time
use crate::config::CFG;
use crate::my_logging::logger;
use crate::zk_print;
use std::time::{Duration, Instant};
// @contextlib.contextmanager
pub fn time_measure(key: &str, should_print: bool, skip: bool) {
    let start = Instant::now();
    // yield
    let elapsed = start.elapsed();

    if !skip {
        if should_print {
            zk_print!("Took {} s", elapsed.as_secs());
        }
        logger::data(&("time_".to_owned() + key), &elapsed.as_secs().to_string());
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

    // pub fn __call__(
    //     &self,
    //     method: impl FnOnce(Vec<String>, Vec<String>) -> String,
    // ) -> impl FnOnce(Vec<String>, Vec<String>) -> String {
    //     let  timed = |args: Vec<String>, kw: Vec<String>| -> String{
    //         time_measure(&self.key,false,false);
    //         method(args, kw)
    //     };
    //     timed
    // }
}
