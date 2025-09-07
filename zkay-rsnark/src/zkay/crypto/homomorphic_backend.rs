#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]

use crate::{
    circuit::structure::circuit_generator::CircuitGenerator,
    zkay::{homomorphic_input::HomomorphicInput, typed_wire::TypedWire},
};

use rccell::RcCell;

pub trait HomomorphicBackend {
    //Perform the unary homomorphic operation 'op' on the ciphertext 'cipher'.
    //
    //@param op
    //		a char identifying the operation; one of {'-', '~', '!'}
    //@param arg
    //		the operand, either a ciphertext or a plain wire
    //@param key_name
    //		the qualified name of the key to be used
    //
    //@return the resulting ciphertext
    //
    //@throws UnsupportedOperationException
    //		if the backend does not support operation 'op'

    fn do_homomorphic_opu(
        &self,
        op: char,
        arg: &HomomorphicInput,
        key_name: &String,
        generator: RcCell<CircuitGenerator>,
    ) -> Vec<TypedWire> {
        panic!("Unary operation {op} not supported");
    }

    //Perform the binary homomorphic operation 'op' on the ciphertexts 'lhs' and 'rhs'.
    //
    //@param lhs
    //		the left-hand side operand, either a ciphertext or a plain wire
    //@param op
    //		a char identifying the operation; one of {'+', '-', '*', '/', '%', '|', '&', '^', '<', '>'}
    //@param rhs
    //		the right-hand side operand, either a ciphertext or a plain wire
    //@param key_name
    //		the qualified name of the key to be used
    //
    //@return the resulting ciphertext
    //
    //@throws UnsupportedOperationException
    //		if the backend does not support operation 'op'

    fn do_homomorphic_op(
        &self,
        lhs: &HomomorphicInput,
        op: char,
        rhs: &HomomorphicInput,
        key_name: &String,
        generator: RcCell<CircuitGenerator>,
    ) -> Vec<TypedWire> {
        panic!("Binary operation {op} not supported");
    }

    //Perform the bool / comparison homomorphic operation 'op' on the ciphertexts 'lhs' and 'rhs'.
    //
    //@param lhs
    //		the left-hand side operand, either a ciphertext or a plain wire
    //@param op
    //		a char identifying the operation; one of {"==", "!=", "<=", ">=", "&&", "||"}
    //@param rhs
    //		the right-hand side operand, either a ciphertext or a plain wire
    //@param key_name
    //		the qualified name of the key to be used
    //
    //@return the resulting ciphertext
    //
    //@throws UnsupportedOperationException
    //		if the backend does not support operation 'op'

    fn do_homomorphic_ops(
        &self,
        lhs: &HomomorphicInput,
        op: &str,
        rhs: &HomomorphicInput,
        key_name: &String,
    ) -> Vec<TypedWire> {
        panic!("Boolean / comparison operation {op} not supported");
    }

    //Perform the ternary conditional operation on the ciphertexts 'cond', 'true_val', 'false_val'.
    //
    //@param cond
    //		the condition, either a ciphertext or a plain wire
    //@param true_val
    //		the value if cond is true, either a ciphertext or a plain wire
    //@param false_val
    //		the value if cond is false, either a ciphertext or a plain wire
    //@param key_name
    //		the qualified name of the key to be used
    //
    //@return the resulting ciphertext
    //
    //@throws UnsupportedOperationException
    //		if the backend does not support operation 'op'

    fn do_homomorphic_cond(
        &self,
        cond: &HomomorphicInput,
        true_val: &HomomorphicInput,
        false_val: &HomomorphicInput,
        key_name: &String,
    ) -> Vec<TypedWire> {
        panic!("Ternary conditional not supported");
    }

    //Re-randomizes the ciphertext in 'arg' by 'randomness'.
    //
    //@param arg
    //		the ciphertext to be re-randomized
    //@param key_name
    //		the qualified name of the key under which arg is encrypted
    //@param randomness
    //		the randomness to use for re-randomization
    //
    //@return the re-randomized ciphertext

    fn do_homomorphic_rerand(
        &self,
        arg: &Vec<TypedWire>,
        key_name: &String,
        randomness: &TypedWire,
        generator: RcCell<CircuitGenerator>,
    ) -> Vec<TypedWire> {
        panic!("Homomorphic re-randomization not supported");
    }
}
