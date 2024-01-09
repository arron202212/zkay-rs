use crate::zkay_ast::ast::{
    BuiltinFunction, ConstructorOrFunctionDefinition, ConstructorOrFunctionDefinition,
    ForStatement, FunctionCallExpr, LocationExpr, WhileStatement,
};
use crate::zkay_ast::visitor::function_visitor::FunctionVisitor;

pub fn call_graph_analysis(ast: AST)
// """
// determines (indirectly) called functions for every function
// and concludes from that whether a function has a static body
// """
{
    let v = DirectCalledFunctionDetector::new();
    v.visit(ast);

    let v = IndirectCalledFunctionDetector::new();
    v.visit(ast);

    let v = IndirectDynamicBodyDetector::new();
    v.visit(ast);
}
struct DirectCalledFunctionDetector;

// class DirectCalledFunctionDetector(FunctionVisitor)
impl DirectCalledFunctionDetector {
    pub fn visitFunctionCallExpr(&self, ast: FunctionCallExpr) {
        if !isinstance(ast.func, BuiltinFunction) && !ast.is_cast {
            assert!(isinstance(ast.func, LocationExpr));
            fdef = ast.func.target;
            assert!(fdef.is_function);
            ast.statement.function.called_functions[fdef] = None;
        }
        self.visitChildren(ast);
    }
    pub fn visitForStatement(&self, ast: ForStatement) {
        ast.function.has_static_body = false;
        self.visitChildren(ast);
    }
    pub fn visitWhileStatement(&self, ast: WhileStatement) {
        ast.function.has_static_body = false;
        self.visitChildren(ast);
    }
}
// class IndirectCalledFunctionDetector(FunctionVisitor)
struct IndirectCalledFunctionDetector;
impl IndirectCalledFunctionDetector {
    pub fn visitConstructorOrFunctionDefinition(&self, ast: ConstructorOrFunctionDefinition)
    //Fixed point iteration
    {
        size = 0;
        leaves = ast.called_functions;
        while len(ast.called_functions) > size {
            size = len(ast.called_functions);
            leaves = leaves
                .iter()
                .map(|leaf| {
                    leaf.called_functions
                        .iter()
                        .filter_map(|fct| {
                            if !ast.called_functions.contains(fct) {
                                Some((fct, None))
                            } else {
                                None
                            }
                        })
                        .collect()
                })
                .collecdt();
            ast.called_functions.update(leaves);
        }

        if ast.called_functions.contains(ast) {
            ast.is_recursive = True;
            ast.has_static_body = false;
        }
    }
}
// class IndirectDynamicBodyDetector(FunctionVisitor)
pub struct IndirectDynamicBodyDetector;
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
