use crate::zkay_ast::ast::{
    is_instance, ASTCode, ASTType, BuiltinFunction, ConstructorOrFunctionDefinition, ForStatement,
    FunctionCallExpr, LocationExpr, NamespaceDefinition, TargetDefinition, WhileStatement, AST,
};
use crate::zkay_ast::visitor::{function_visitor::FunctionVisitor, visitor::AstVisitor};

pub fn call_graph_analysis(ast: AST)
// """
// determines (indirectly) called functions for every function
// and concludes from that whether a function has a static body
// """
{
    let v = DirectCalledFunctionDetector;
    v.visit(ast);

    let v = IndirectCalledFunctionDetector;
    v.visit(ast);

    let v = IndirectDynamicBodyDetector;
    v.visit(ast);
}
struct DirectCalledFunctionDetector;

// class DirectCalledFunctionDetector(FunctionVisitor)
impl FunctionVisitor for DirectCalledFunctionDetector {}
impl AstVisitor for DirectCalledFunctionDetector {
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
impl DirectCalledFunctionDetector {
    pub fn visitFunctionCallExpr(&self, ast: FunctionCallExpr) {
        if !is_instance(&ast.func().unwrap(), ASTType::BuiltinFunction) && !ast.is_cast() {
            assert!(is_instance(&ast.func().unwrap(), ASTType::LocationExpr));
            let fdef = ast.func().unwrap().target().unwrap();
            assert!(fdef.is_function());
            if let TargetDefinition::NamespaceDefinition(
                NamespaceDefinition::ConstructorOrFunctionDefinition(cofd),
            ) = *fdef
            {
                ast.statement()
                    .unwrap()
                    .function()
                    .unwrap()
                    .called_functions
                    .insert(cofd.clone());
            }
        }
        self.visit_children(&ast.get_ast());
    }
    pub fn visitForStatement(&self, ast: ForStatement) {
        ast.statement_base.function.unwrap().has_static_body = false;
        self.visit_children(&ast.get_ast());
    }
    pub fn visitWhileStatement(&self, ast: WhileStatement) {
        ast.statement_base.function.unwrap().has_static_body = false;
        self.visit_children(&ast.get_ast());
    }
}
// class IndirectCalledFunctionDetector(FunctionVisitor)
struct IndirectCalledFunctionDetector;
impl FunctionVisitor for IndirectCalledFunctionDetector {}
impl AstVisitor for IndirectCalledFunctionDetector {
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
impl IndirectCalledFunctionDetector {
    pub fn visitConstructorOrFunctionDefinition(&self, ast: ConstructorOrFunctionDefinition)
    //Fixed point iteration
    {
        let mut size = 0;
        let mut leaves = ast.called_functions.clone();
        while ast.called_functions.len() > size {
            size = ast.called_functions.len();
            leaves = leaves
                .iter()
                .map(|leaf| {
                    leaf.called_functions
                        .iter()
                        .filter_map(|fct| {
                            if !ast.called_functions.contains(fct) {
                                Some(fct.clone())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect();
            ast.called_functions = ast.called_functions.union(&leaves).cloned().collect();
        }

        if ast.called_functions.contains(&ast) {
            ast.is_recursive = true;
            ast.has_static_body = false;
        }
    }
}
// class IndirectDynamicBodyDetector(FunctionVisitor)
pub struct IndirectDynamicBodyDetector;

impl FunctionVisitor for IndirectDynamicBodyDetector {}
impl AstVisitor for IndirectDynamicBodyDetector {
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
impl IndirectDynamicBodyDetector {
    pub fn visitConstructorOrFunctionDefinition(&self, ast: ConstructorOrFunctionDefinition) {
        if !ast.has_static_body {
            return;
        }

        for fct in ast.called_functions {
            if !fct.has_static_body
            // This function (directly or indirectly) calls a recursive function
            {
                ast.has_static_body = false;
                return;
            }
        }
    }
}
