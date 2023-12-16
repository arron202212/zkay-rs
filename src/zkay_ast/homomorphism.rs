use serde::{Deserialize, Serialize};
// from enum import Enum
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Homomorphism {
    NON_HOMOMORPHIC, //(("<>", "unhom")),
    ADDITIVE,        //(("<+>", "addhom")),
    #[default]
    Never,
}
// class Homomorphism(Enum):
//     NON_HOMOMORPHIC = ('<>', 'unhom')
//     ADDITIVE = ('<+>', 'addhom')

//     def __init__(self, type_annotation: str, rehom_expr_name: str):
//         self.type_annotation = type_annotation
//         self.rehom_expr_name = rehom_expr_name

//     def __str__(self):
//         return self.type_annotation if self != Homomorphism.NON_HOMOMORPHIC else ''

//     def code(self) -> str:
//         return super().__str__()  // i.e. Homomorphism.ENUM_NAME
pub struct HomomorphismStore {
    pub value: Homomorphism,
    pub type_annotation: String,
    pub rehom_expr_name: String,
}
impl HomomorphismStore {
    fn new(type_annotation: String, rehom_expr_name: String) -> Self {
        Self {
            value: Homomorphism::NON_HOMOMORPHIC,
            type_annotation,
            rehom_expr_name,
        }
    }
    fn code(&self) -> String {
        self.to_string()
    }
}
use std::fmt;
impl fmt::Display for HomomorphismStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            if self.value != Homomorphism::NON_HOMOMORPHIC {
                self.type_annotation.to_owned()
            } else {
                String::new()
            }
        )
    }
}
