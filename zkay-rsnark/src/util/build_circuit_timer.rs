#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

use std::time::{Duration, Instant};

pub struct WithTimeMeasure {
    pub key: String,
    pub start: Instant,
}
impl WithTimeMeasure {
    pub fn new(key: String) -> Self {
        let start = Instant::now();
        println!("Begin Name {} ", key);
        Self { key, start }
    }
}
impl Drop for WithTimeMeasure {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        println!("End Name {} Time: Took {} s", self.key, elapsed.as_secs());
    }
}
pub fn time_measure(key: &str) -> WithTimeMeasure {
    WithTimeMeasure::new(key.to_owned())
}

#[macro_export]
macro_rules! with_timer_block {
    ($initializer:expr => $body:expr) => {{
        let _ = time_measure($initializer);
        $body;
    }};
}
