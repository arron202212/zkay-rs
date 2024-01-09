use crate::zkay_ast::ast::{
    ConstructorOrFunctionDefinition, Expression, Identifier, NamespaceDefinition, SourceUnit,
    Statement, AST,
};
use crate::zkay_ast::visitor::visitor::AstVisitor;

struct ParentSetterVisitor {
    traversal: String,
}

impl AstVisitor for ParentSetterVisitor {
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
// class ParentSetterVisitor(AstVisitor)
//     """
//     Links parents
//     """
impl ParentSetterVisitor {
    pub fn new() -> Self {
        Self {
            traversal: String::from("pre"),
        }
    }

    //     pub fn __init__(self)
    //         super().__init__(traversal='pre')

    pub fn visitSourceUnit(&self, ast: &mut SourceUnit) {
        ast.namespace = [];
    }

    pub fn visitNamespaceDefinition(&self, ast: NamespaceDefinition) {
        ast.namespace = (if let Some(parent) = ast.parent() {
            parent.namespace.clone()
        } else {
            vec![]
        })
        .into_iter()
        .chain([ast.idf.clone()])
        .collect();
    }

    pub fn visitConstructorOrFunctionDefinition(&self, ast: &mut ConstructorOrFunctionDefinition) {
        ast.namespace = (if let Some(parent) = ast.parent() {
            parent.namespace.clone()
        } else {
            vec![]
        })
        .into_iter()
        .chain([ast.idf.clone()])
        .collect();
    }

    pub fn visitChildren(&self, ast: &mut AST) {
        for c in ast.children() {
            if let Some(c) = c {
                c.parent = ast.clone();
                c.namespace = ast.namespace.clone();
                self.visit(c);
            } else {
                print!(c, ast, ast.children());
            }
        }
    }
}

struct ExpressionToStatementVisitor;

impl AstVisitor for ExpressionToStatementVisitor {
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
// class ExpressionToStatementVisitor(AstVisitor)

impl ExpressionToStatementVisitor {
    pub fn visitExpression(&self, ast: &mut Expression) {
        let parent = ast.clone();
        while let Some(p) = parent {
            if let AST::Statement(_) = p {
                break;
            }
            parent = p.parent();
        }
        if parent.is_some() {
            ast.statement = parent;
        }
    }

    pub fn visitStatement(&self, ast: &mut Statement) {
        let parent = ast.clone();
        while let Some(p) = parent {
            if let AST::ConstructorOrFunctionDefinition(_) = p {
                break;
            }
            parent = p.parent();
        }
        if parent.is_some() {
            ast.function = parent;
        }
    }
}

pub fn set_parents(ast: AST) {
    let v = ParentSetterVisitor::default();
    v.visit(ast);
    let v = ExpressionToStatementVisitor::default();
    v.visit(ast);
}
