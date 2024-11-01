#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

// import contextlib
// import time
use my_logging::logger;
use std::time::{Duration, Instant};
use zkay_config::{config::CFG, config_user::UserConfig, zk_print};

pub struct WithTimeMeasure {
    key: String,
    should_print: bool,
    skip: bool,
    start: Instant,
}
impl WithTimeMeasure {
    pub fn new(key: String, should_print: bool, skip: bool) -> Self {
        let start = Instant::now();
        Self {
            key,
            should_print,
            skip,
            start,
        }
    }
}
impl Drop for WithTimeMeasure {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();

        if !self.skip {
            if self.should_print {
                zk_print!("Took {} s", elapsed.as_secs());
            }
            logger::data(
                &("time_".to_owned() + &self.key),
                &elapsed.as_secs().to_string(),
            );
        }
    }
}
// @contextlib.contextmanager
pub fn time_measure(key: &str, should_print: bool, skip: bool) -> WithTimeMeasure {
    // yield
    WithTimeMeasure::new(key.to_owned(), should_print, skip)
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
