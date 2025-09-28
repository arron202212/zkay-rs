
use  libff::common::default_types::ec_pp;

type libff::Fr<libff::default_ec_pp> FieldT;

fn  readIds(str:&str,  vec:&Vec<u32>){

	for id in str.lines() {
		vec.push_back(id);
	}
}

 fn readFieldElementFromHex(inputStr:&str)->FieldT{
    use num_bigint::{BigInt, ToBigInt};
	// let mut  constStrDecimal=[' ';150];
	let mut  integ;
   let integ= BigInt::parse_bytes(inputStr.as_bytes(), 16).unwrap();
    let constStrDecimal=integ.to_str_radix(10);
	// mpz_init_set_str(integ, inputStr, 16);
	// mpz_get_str(constStrDecimal, 10, integ);
	// mpz_clear(integ);
	let  f = FieldT::new(constStrDecimal);
	 f

}
