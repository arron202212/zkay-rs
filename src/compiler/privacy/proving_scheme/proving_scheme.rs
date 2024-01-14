// from abc import ABCMeta, abstractmethod
// from typing import List

use crate::compiler::privacy::circuit_generation::circuit_helper::CircuitHelper;

pub struct G1Point {
    x: String,
    y: String,
}
// class G1Point
// """Data class to represent curve points"""

pub fn new(self, x: String, y: String)
// """Construct G1Point from coordinate integer literal strings."""
// self.x: String = x
// self.y: String = y
{
    Self { x, y }
}

pub fn negated(self) {
    let q = "21888242871839275222246405745257275088696311157297823662689037894645226208583";
    if self.x == "0" && self.y == "0" {
        G1Point("0", "0")
    } else {
        G1Point(self.x, hex(q - (int(self.y, 0) % q)))
    }
}

// @staticmethod
pub fn from_seq(seq: Vec<String>) -> Self
// """
        // Construct G1Point from a sequence of length 2 of integer literal strings
        // First entry makes up the X coordinate, second entry makes up the Y coordinate
        // """
{
    assert!(len(seq) == 2);
    return G1Point::new(seq[0], seq[1]);
}

// @staticmethod
pub fn from_it<'a>(it: &mut impl Iterator<Item = &'a String>) -> Self {
    G1Point::new(it.next().unwrap(), it.next().unwrap())
}

// pub fn __str__(G1Point)
//     return f"uint256({self.x}), uint256({self.y})"

use std::fmt;

impl fmt::Display for G1Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "uint256({}), uint256({})", self.x, self.y)
    }
}

// class G2Point
// """Data class to represent curve points which are encoded using two field elements"""
pub struct G2Point {
    x: G1Point,
    y: G1Point,
}
impl G2Point {
    pub fn new(x1: String, x2: String, y1: String, y2: String) -> Self {
        Self {
            x: G1Point::new(x1, x2), // not really a G1Point, but can reuse __str__
            y: G1Point::new(y1, y2),
        }
    }

    // @staticmethod
    pub fn from_seq(seq: Vec<String>) -> Self
// """
        // Construct G1Point from a sequence of length 4 of integer literal strings
        // First two entries make up the X coordinate, last two entries make up the Y coordinate
        // """
        //
    {
        assert!(seq.len() == 4);
        G2Point(seq[0], seq[1], seq[2], seq[3])
    }

    // @staticmethod
    pub fn from_it<'a>(it: &mut impl Iterator<Item = &'a String>) -> Self {
        G2Point(
            it.next().unwrap(),
            it.next().unwrap(),
            it.next().unwrap(),
            it.next().unwrap(),
        )
    }

    // pub fn __str__(self)
    //     return f"[{self.x}], [{self.y}]"
}
impl fmt::Display for G2Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}], [{}]", self.x, self.y)
    }
}
// class VerifyingKey(metaclass=ABCMeta)
// """Abstract base data class for verification keys"""
pub trait VerifyingKeyMeta{
    // @classmethod
    // @abstractmethod
    // pub fn create_dummy_key(cls)
    //     """Generate a dummy key."""
    fn create_dummy_key();
    //     pass
}

// class ProvingScheme(metaclass=ABCMeta)
// """
// Abstract base class for proving schemes

// A proving scheme provides functionality to generate a verification contract from a proving-scheme dependent verification-key
// and an abstract circuit representation
// """
pub struct ProvingSchemeBase {
    // verify_libs_contract_filename = "./verify_libs.sol"
    // snark_scalar_field_var_name = "snark_scalar_field"
    // hash_var_name = "hash"
    // """Special variable names usable by the verification contract"""

    // name = "none"
    // """Proving scheme name, overridden by child classes"""
    verify_libs_contract_filename: String,
    snark_scalar_field_var_name: String,
    hash_var_name: String,
    name: String,
}
impl ProvingSchemeBase {
    pub fn new() -> Self {
        Self {
            verify_libs_contract_filename: String::from("./verify_libs.sol"),
            snark_scalar_field_var_name: String::from("snark_scalar_field"),
            hash_var_name: String::from("hash"),
            name: String::from("none"),
        }
    }
}
// class VerifyingKey(VerifyingKey, metaclass=ABCMeta)
//     pass

pub trait ProvingScheme {
    const NAME: &'static str;

    type VerifyingKey;
    // @abstractmethod
    fn generate_verification_contract(
        &self,
        verification_key: VerifyingKey,
        circuit: CircuitHelper,
        primary_inputs: Vec<String>,
        prover_key_hash: bytes,
    ) -> String;
    // """
    // Generate a verification contract for the zk-snark corresponding to circuit.

    // :param verification_key: parsed verification key which was previously generated for circuit
    // :param circuit: the circuit for which to generate the verification contract
    // :param primary_inputs: list of all public input locations (strings which represent either identifiers or array index expressions)
    // :param prover_key_hash: sha3 hash of the prover key
    // :return: verification contract text
    // """
    // pass
}
