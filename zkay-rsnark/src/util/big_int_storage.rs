#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::util::util::BigInteger;

use std::{collections::HashMap, sync::OnceLock};

static INSTANCE: OnceLock<BigIntStorage> = OnceLock::new();
pub fn init() {
    INSTANCE.get_or_init(|| BigIntStorage::new());
}

//  * shares big integer constants

pub struct BigIntStorage {
    pub big_integer_set: HashMap<BigInteger, BigInteger>,
}
impl BigIntStorage {
    pub fn new() -> Self {
        Self {
            big_integer_set: HashMap::new(),
        }
    }

    pub fn get_big_integer(&mut self, x: BigInteger) -> BigInteger {
        self.big_integer_set.entry(x.clone()).or_insert(x.clone());
        self.big_integer_set.get(&x).unwrap().clone()
    }
}
