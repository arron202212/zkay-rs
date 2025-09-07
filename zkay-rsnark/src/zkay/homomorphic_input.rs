#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::zkay::typed_wire::TypedWire;

//  * The input to a homomorphic operation which can either be a ciphertext wire array or a plaintext wire.
//  *
//  * This class exists because some homomorphic operations require plaintext operands, such as when performing
//  * multiplication on additively homomorphic ciphertexts encrypted with Paillier or Dummy-Hom, and having
//  * arguments of this type is preferable to having dozens of overloads with different combinations of Vec<TypedWire>
//  * and TypedWire or having to tell apart plaintext and ciphertext inputs from the length of the Vec<TypedWire> input.

#[derive(Debug, Clone)]
pub struct HomomorphicInput {
    pub array: Vec<TypedWire>,
    pub is_cipher: bool,
}
impl HomomorphicInput {
    pub fn new(array: Vec<TypedWire>, is_cipher: bool) -> Self {
        Self { array, is_cipher }
    }

    pub fn ofv(cipher: Vec<TypedWire>) -> Self {
        HomomorphicInput::new(cipher, true)
    }

    pub fn of(plain: TypedWire) -> Self {
        HomomorphicInput::new(vec![plain], false)
    }

    pub fn is_cipher(&self) -> bool {
        self.is_cipher
    }

    pub fn is_plain(&self) -> bool {
        !self.is_cipher
    }

    pub fn get_cipher(&self) -> &Vec<TypedWire> {
        assert!(self.is_cipher(), "Homomorphic input was not a ciphertext");
        &self.array
    }

    pub fn get_plain(&self) -> TypedWire {
        assert!(!self.is_cipher(), "Homomorphic input was not a plaintext");
        self.array[0].clone()
    }

    pub fn get_length(&self) -> i32 {
        self.array.len() as _
    }

    pub fn get_name(&self) -> &String {
        &self.array[0].name
    }
}
