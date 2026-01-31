// use ffec::common::default_types::ec_pp;
use ff_curves::Fr;
use ff_curves::default_ec_pp;

type FieldT = Fr<default_ec_pp>;

pub fn readIds(str: &str, vec: &mut Vec<u32>) {
    for id in str.lines() {
        vec.push(id.parse::<u32>().unwrap());
    }
}

pub fn readFieldElementFromHex(inputStr: &str) -> FieldT {
    use num_bigint::{BigInt, ToBigInt};
    // let mut  constStrDecimal=[' ';150];
    // let mut integ;
    let integ = BigInt::parse_bytes(inputStr.as_bytes(), 16).unwrap();
    let constStrDecimal = integ.to_str_radix(10);
    // mpz_init_set_str(integ, inputStr, 16);
    // mpz_get_str(constStrDecimal, 10, integ);
    // mpz_clear(integ);
    let f = FieldT::from(constStrDecimal.as_str());
    f
}
