#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

pub trait CircuitHelperConfig {
    fn trans_in_size(&self) -> i32;
    fn trans_out_size(&self) -> i32;
    fn get_verification_contract_name(&self) -> String;
    fn priv_in_size_trans(&self) -> i32;
    fn out_size_trans(&self) -> i32;
    fn in_size_trans(&self) -> i32;
}
