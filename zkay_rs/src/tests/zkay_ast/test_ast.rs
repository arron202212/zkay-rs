// use  zkay.tests.zkay_unit_test::ZkayTestCase
use zkay_ast::ast::{
    AssignmentStatement, BooleanLiteralExpr, BuiltinFunction, FunctionCallExpr, Identifier,
    IdentifierExpr, NumberLiteralExpr, RequireStatement,FunctionCallExprBase,
};

#[cfg(test)]
mod tests {
    use super::*;
    // class TestASTSimpleStorageDetailed(ZkayTestCase):
    #[test]
    fn test_require() {
        let e = BooleanLiteralExpr::new(true);
        let r = RequireStatement::new(e.into_expr());
        assert_eq!(&r.to_string(), "require(true);");
    }
    #[test]
    fn test_assignment_statement() {
        let i = Identifier::new("x");
        let lhs = IdentifierExpr::new(i);
        let rhs = BooleanLiteralExpr::new(true);
        let a = AssignmentStatement::new(lhs, rhs);
        assert!(a.is_some());
        assert_eq!(a.to_string(), "x = true;");
        assert_eq!(a.children(), vec![lhs, rhs]);
        assert!(a.names.is_empty());
        assert!(a.parent.is_none());
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
            f,
            vec![NumberLiteralExpr::new(0,false), NumberLiteralExpr::new(0,false)],
        );
        assert_eq!(c.code(), "0 + 0");
    }
}
