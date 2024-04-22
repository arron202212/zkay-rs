// use  zkay.tests.zkay_unit_test::ZkayTestCase
use rccell::RcCell;
use zkay_ast::ast::{
    ASTBaseProperty, ASTChildren, AssignmentStatementBase, BooleanLiteralExpr, BuiltinFunction,
    FunctionCallExprBase, IdentifierExpr, IdentifierExprUnion, IntoAST, IntoExpression,
    NumberLiteralExpr, RequireStatement,
};

#[cfg(test)]
mod tests {
    use super::*;
    // class TestASTSimpleStorageDetailed(ZkayTestCase):
    #[test]
    fn test_require() {
        let e = BooleanLiteralExpr::new(true);
        let r = RequireStatement::new(RcCell::new(e.into_expr()).into(), None);
        assert_eq!(&r.to_string(), "require(true);");
    }
    #[test]
    fn test_assignment_statement() {
        // let i = Identifier::identifier("x");
        let lhs = IdentifierExpr::new(IdentifierExprUnion::String(String::from("x")), None);
        let rhs = BooleanLiteralExpr::new(true);
        let mut a = AssignmentStatementBase::new(
            Some(RcCell::new(lhs.to_ast()).into()),
            Some(RcCell::new(rhs.to_expr()).into()),
            None,
        );
        // assert!(a.is_some());
        assert_eq!(a.to_string(), "x = true;");
        assert_eq!(
            a.children(),
            vec![
                RcCell::new(lhs.into_ast()).into(),
                RcCell::new(rhs.into_ast()).into()
            ]
        );
        assert!(a.names().is_empty());
        assert!(a.parent().is_none());
    }
    #[test]
    fn test_builtin_arity() {
        let f = BuiltinFunction::new("+");
        assert_eq!(f.arity(), 2);
    }
    #[test]
    fn test_builtin_code() {
        let f = BuiltinFunction::new("+");
        let c = FunctionCallExprBase::new(
            RcCell::new(f.into_expr()).into(),
            vec![
                RcCell::new(NumberLiteralExpr::new(0, false).into_expr()).into(),
                RcCell::new(NumberLiteralExpr::new(0, false).into_expr()).into(),
            ],
            None,
        );
        assert_eq!(c.to_ast().code(), "0 + 0");
    }
}
