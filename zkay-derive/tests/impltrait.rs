#![cfg(test)]
// This lets us ensure that the generated methods get doc comments.
#![deny(missing_docs)]
#![deny(unreachable_patterns)]

/// Tests for `#[derive(is_enum_variant)]`.
// #[macro_use]
extern crate zkay_derive;
// extern crate diff;
use enum_dispatch::enum_dispatch;
use zkay_derive::{impl_trait, impl_traits, ImplBaseTrait};

#[derive(ImplBaseTrait)]
pub struct ASSTBase2 {
    a: i32,
}
pub struct TestStruct2 {
    asst_base2: ASSTBase2,
}
pub struct TestStruct3 {
    asst_base2: ASSTBase2,
}
#[impl_trait(TestStruct2, TestStruct3)] //,ExpressionBase2,LiteralExprBase2,ArrayLiteralExprBase2
pub trait ASSTBase2Ref {
    fn asst_base2_ref(&self) -> &ASSTBase2;
}
pub trait ExpressionBase2Ref: ASSTBase2Ref {
    fn expression_base2_ref(&self) -> &ExpressionBase2;
}
pub trait LiteralExprBase2Ref: ExpressionBase2Ref {
    fn literal_expr_base2_ref(&self) -> &LiteralExprBase2;
}
pub trait ArrayLiteralExprBase2Ref: LiteralExprBase2Ref {
    fn array_literal_expr_base2_ref(&self) -> &ArrayLiteralExprBase2;
}
#[impl_traits(ASSTBase2)]
#[derive(ImplBaseTrait)]
pub struct ExpressionBase2 {
    pub asst_base2: ASSTBase2,
}
#[impl_traits(ExpressionBase2, ASSTBase2)]
#[derive(ImplBaseTrait)]
pub struct LiteralExprBase2 {
    pub expression_base2: ExpressionBase2,
}
#[impl_traits(LiteralExprBase2, ExpressionBase2, ASSTBase2)]
#[derive(ImplBaseTrait)]
pub struct ArrayLiteralExprBase2 {
    pub literal_expr_base2: LiteralExprBase2,
}
#[impl_traits(ArrayLiteralExprBase2, LiteralExprBase2, ExpressionBase2, ASSTBase2)]
pub struct KeyLiteralExpr2 {
    pub array_literal_expr_base2: ArrayLiteralExprBase2,
}

#[test]
fn test_impl_trait() {
    let x = TestStruct2 {
        asst_base2: ASSTBase2 { a: 2 },
    };
    assert!(x.asst_base2_ref().a == 2);
    let x = TestStruct3 {
        asst_base2: ASSTBase2 { a: 3 },
    };
    assert!(x.asst_base2_ref().a == 3);
}
#[test]
fn test_impl_traits() {
    let x = KeyLiteralExpr2 {
        array_literal_expr_base2: ArrayLiteralExprBase2 {
            literal_expr_base2: LiteralExprBase2 {
                expression_base2: ExpressionBase2 {
                    asst_base2: ASSTBase2 { a: 2 },
                },
            },
        },
    };
    assert!(x.asst_base2_ref().a == 2);
}
#[test]
fn test_impl_base_trait() {
    let x = ASSTBase2 { a: 2 };
    assert!(x.asst_base2_ref().a == 2);
}
