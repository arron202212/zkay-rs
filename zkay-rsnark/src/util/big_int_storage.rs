#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::util::util::{BigInteger, Util};

use std::collections::HashMap;
use std::sync::OnceLock;
static instance: OnceLock<BigIntStorage> = OnceLock::new();
pub fn init() {
    instance.get_or_init(|| BigIntStorage::new());
}
/**
 * shares big integer constants
 *
 */
pub struct BigIntStorage {
    pub bigIntegerSet: HashMap<BigInteger, BigInteger>,
}
impl BigIntStorage {
    pub fn new() -> Self {
        Self {
            bigIntegerSet: HashMap::new(),
        }
    }

    pub fn getBigInteger(&mut self, x: BigInteger) -> BigInteger {
        self.bigIntegerSet.entry(x.clone()).or_insert(x.clone());
        self.bigIntegerSet.get(&x).unwrap().clone()
    }
}
