use crate::zkay_ast::ast::{
    is_instance, ASTCode, ASTType, Block, ConstructorOrFunctionDefinition, ReturnStatement, AST,
}; //, AstException
use crate::zkay_ast::visitor::visitor::AstVisitor;

pub fn check_return(ast: &AST) {
    let v = ReturnCheckVisitor;
    v.visit(ast);
}
// class ReturnPositionException(AstException):

// pub fn __init__(&self, ast: ReturnStatement):
//     super().__init__("Return statements are only allowed at the end of a function.", ast)

// class ReturnCheckVisitor(AstVisitor):

struct ReturnCheckVisitor;
impl AstVisitor for ReturnCheckVisitor {
    type Return = Option<String>;
    fn temper_result(&self) -> Self::Return {
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
    fn call_visit_function(&self, ast: &AST) -> Self::Return {
        None
    }
}
impl ReturnCheckVisitor {
    pub fn visitReturnStatement(&self, ast: &mut ReturnStatement) {
        let container = ast.statement_base.ast_base.parent.unwrap();
        // assert!(is_instance(&*container,ASTType::Block));
        let ok = true;
        if container
            .statement_list_base()
            .statements
            .last()
            .map(|v| *v)
            .unwrap_or_default()
            != ast.get_ast()
        {
            ok = false;
        }
        if !is_instance(
            &container.parent().unwrap(),
            ASTType::ConstructorOrFunctionDefinition,
        ) || container
            .parent()
            .unwrap()
            .constructor_or_function_definition()
            .unwrap()
            .is_constructor()
        {
            ok = false;
        }
        // raise ReturnPositionException(ast)}
        assert!(
            ok,
            "Return statements are only allowed at the end of a function. {:?}",
            ast,
        );
    }
}
