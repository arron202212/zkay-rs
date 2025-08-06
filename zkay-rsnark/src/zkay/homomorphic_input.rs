#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::zkay::typed_wire::TypedWire;

/**
 * The input to a homomorphic operation which can either be a ciphertext wire array or a plaintext wire.
 *
 * This class exists because some homomorphic operations require plaintext operands, such as when performing
 * multiplication on additively homomorphic ciphertexts encrypted with Paillier or Dummy-Hom, and having
 * arguments of this type is preferable to having dozens of overloads with different combinations of Vec<TypedWire>
 * and TypedWire or having to tell apart plaintext and ciphertext inputs from the length of the Vec<TypedWire> input.
 */
pub struct HomomorphicInput {
    array: Vec<TypedWire>,
    isCipher: bool,
}
impl HomomorphicInput {
    pub fn new(array: Vec<TypedWire>, isCipher: bool) -> Self {
        Self { array, isCipher }
    }

    pub fn of(cipher: Vec<TypedWire>) -> Self {
        HomomorphicInput::new(cipher, true)
    }

    pub fn of(plain: TypedWire) -> Self {
        HomomorphicInput::new(vec![plain], false)
    }

    pub fn isCipher(&self) -> bool {
        self.isCipher
    }

    pub fn isPlain(&self) -> bool {
        !self.isCipher
    }

    pub fn getCipher(&self) -> &Vec<TypedWire> {
        assert!(self.isCipher(), "Homomorphic input was not a ciphertext");
        &self.array
    }

    pub fn getPlain(&self) -> TypedWire {
        assert!(!self.isCipher(), "Homomorphic input was not a plaintext");
        self.array[0].clone()
    }

    pub fn getLength(&self) -> i32 {
        self.array.len()
    }

    pub fn getName(&self) -> &String {
        &self.array[0].name
    }
}
