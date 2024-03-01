#![cfg(test)]
// This lets us ensure that the generated methods get doc comments.
#![deny(missing_docs)]
#![deny(unreachable_patterns)]

/// Tests for `#[derive(is_enum_variant)]`.

#[macro_use]
extern crate zkay_derive;
// extern crate diff;

use zkay_derive::impl_trait;
pub struct ASTBase2 {
    a: i32,
}
pub struct TestStruct2 {
    ast_base2: ASTBase2,
}
pub struct TestStruct3 {
    ast_base2: ASTBase2,
}
#[impl_trait(TestStruct2,TestStruct3)]
pub trait ASTBase2Ref {
    fn ast_base2_ref(&self) -> &ASTBase2;
}

#[test]
fn test_impl_trait() {
    let x = TestStruct2 {
        ast_base2: ASTBase2 { a: 2 },
    };
    assert!(x.ast_base2_ref().a == 2);
    let x = TestStruct3 {
        ast_base2: ASTBase2 { a: 3 },
    };
    assert!(x.ast_base2_ref().a == 3);
}
