use serde::{Deserialize, Serialize};
// from enum import Enum
// #[derive(Default, Clone, Debug,  PartialEq, PartialOrd, Eq, Ord, Hash)]
// pub enum Homomorphism {
//     NonHomomorphic, //(("<>", "unhom")),
//     ADDITIVE,        //(("<+>", "addhom")),
//     #[default]
//     Never,
// }
// class Homomorphism(Enum):
//     NonHomomorphic = ('<>', 'unhom')
//     ADDITIVE = ('<+>', 'addhom')

//     def __init__(self, type_annotation: str, rehom_expr_name: str):
//         self.type_annotation = type_annotation
//         self.rehom_expr_name = rehom_expr_name

//     def __str__(self):
//         return self.type_annotation if self != Homomorphism.NonHomomorphic else ''

//     def code(self) -> str:
//         return super().__str__()  // i.e. Homomorphism.ENUM_NAME

use lazy_static::lazy_static;
use std::sync::Mutex;
lazy_static! {
    pub static ref HOMOMORPHISM_STORE: Mutex<BTreeMap<String, Homomorphism>> =
        Mutex::new(BTreeMap::from([
            (
                String::from("NON_HOMOMORPHIC"),
                Homomorphism::new(
                    String::from("NON_HOMOMORPHIC"),
                    String::from("<>"),
                    String::from("unhom"),
                ),
            ),
            (
                String::from("ADDITIVE"),
                Homomorphism::new(
                    String::from("ADDITIVE"),
                    String::from("<+>"),
                    String::from("addhom"),
                ),
            ),
        ]),);
    pub static ref REHOM_EXPRESSIONS: Mutex<BTreeMap<String, Homomorphism>> =
        Mutex::new(BTreeMap::from([
            (
                String::from("unhom"),
                Homomorphism::new(
                    String::from("NON_HOMOMORPHIC"),
                    String::from("<>"),
                    String::from("unhom"),
                ),
            ),
            (
                String::from("addhom"),
                Homomorphism::new(
                    String::from("ADDITIVE"),
                    String::from("<+>"),
                    String::from("addhom"),
                ),
            ),
        ]),);
}

#[derive(Default, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Homomorphism {
    pub value: String,
    pub type_annotation: String,
    pub rehom_expr_name: String,
}
impl Homomorphism {
    pub fn new(value: String, type_annotation: String, rehom_expr_name: String) -> Self {
        Self {
            value,
            type_annotation,
            rehom_expr_name,
        }
    }
    pub fn code(&self) -> String {
        self.to_string()
    }
    pub fn non_homomorphic() -> String {
        String::from("NON_HOMOMORPHIC")
    }
    pub fn additive() -> String {
        String::from("ADDITIVE")
    }
    pub fn fields() -> Vec<String> {
        HOMOMORPHISM_STORE.lock().unwrap().keys().cloned().collect()
    }
}
use std::collections::BTreeMap;
use std::fmt;
impl fmt::Display for Homomorphism {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            if self.value != "NON_HOMOMORPHIC" {
                self.type_annotation.to_owned()
            } else {
                String::new()
            }
        )
    }
}
