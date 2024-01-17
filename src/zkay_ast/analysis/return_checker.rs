use crate::zkay_ast::ast::{Block, ConstructorOrFunctionDefinition, ReturnStatement,AST}; //, AstException
use crate::zkay_ast::visitor::visitor::AstVisitor;

pub fn check_return(ast: AST) {
    let v = ReturnCheckVisitor();
    v.visit(ast);
}
// class ReturnPositionException(AstException):

// pub fn __init__(&self, ast: ReturnStatement):
//     super().__init__("Return statements are only allowed at the end of a function.", ast)

struct ReturnCheckVisitor;
impl AstVisitor for ReturnCheckVisitor {
    type Return = Option<String>;
    fn temper_result(&self) -> Option<Self::Return> {
        None
    }
    fn log(&self) -> bool {
        false
    }
    fn traversal(&self) -> &'static str {
        "node-or-children"
    }
    fn has_attr(&self, name: &String) -> bool {
        self.get_attr(name).is_some()
    }
    fn get_attr(&self, name: &String) -> Option<String> {
        None
    }
    fn call_visit_function(&self, ast: &AST) -> Option<Self::Return> {
        None
    }
}
// class ReturnCheckVisitor(AstVisitor):
impl ReturnCheckVisitor {
    pub fn visitReturnStatement(&self, ast: &mut ReturnStatement) {
        let container = ast.parent;
        assert!(container.is_block());
        let ok = true;
        if container.statements.last().unwrap_or_default() != ast {
            ok = false;
        }
        if !container.parent.is_constructor_or_function_definition()
            || container.parent.is_constructor()
        {
            ok = false;
        }
        if !ok
        // raise ReturnPositionException(ast)}
        {
            assert!(
                false,
                "Return statements are only allowed at the end of a function. {:?}",
                ast,
            );
        }
    }
}
