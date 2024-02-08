use crate::ast::{
    ASTChildren, ConstructorOrFunctionDefinition, Expression, Identifier, IntoAST,
    NamespaceDefinition, SourceUnit, Statement, AST,
};
use crate::visitor::visitor::AstVisitor;

struct ParentSetterVisitor {
    traversal: String,
}

impl AstVisitor for ParentSetterVisitor {
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
        ast.ast_base.namespace = Some(vec![]);
    }

    pub fn visitNamespaceDefinition(&self, mut ast: NamespaceDefinition) {
        ast.ast_base_mut().unwrap().namespace = Some(if let Some(mut parent) = ast.parent() {
            parent
                .ast_base()
                .unwrap()
                .namespace
                .clone()
                .unwrap()
                .into_iter()
                .chain([ast.namespace_definition_base().unwrap().idf.clone()])
                .collect()
        } else {
            vec![ast.namespace_definition_base().unwrap().idf.clone()]
        });
    }

    pub fn visitConstructorOrFunctionDefinition(&self, ast: &mut ConstructorOrFunctionDefinition) {
        ast.namespace_definition_base.ast_base.namespace =
            Some(if let Some(parent) = &ast.parent {
                parent
                    .namespace_definition_base
                    .ast_base
                    .namespace
                    .as_ref()
                    .unwrap()
                    .into_iter()
                    .chain([&ast.namespace_definition_base.idf.clone()])
                    .cloned()
                    .collect()
            } else {
                vec![ast.namespace_definition_base.idf.clone()]
            });
    }

    pub fn visitChildren(&self, ast: &mut AST) {
        for c in ast.children().iter_mut() {
            c.ast_base_mut().unwrap().parent = Some(Box::new(ast.clone()));
            c.ast_base_mut().unwrap().namespace = ast.ast_base().unwrap().namespace.clone();
            self.visit(c.clone());
        }
    }
}

struct ExpressionToStatementVisitor;

impl AstVisitor for ExpressionToStatementVisitor {
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
// class ExpressionToStatementVisitor(AstVisitor)

impl ExpressionToStatementVisitor {
    pub fn visitExpression(&self, ast: &mut Expression) {
        let mut parent = Some(ast.to_ast());
        while let Some(p) = &parent {
            if let AST::Statement(_) = p {
                break;
            }
            parent = p.parent();
        }
        if parent.is_some() {
            ast.expression_base_mut().unwrap().statement =
                parent.map(|p| Box::new(p.statement().unwrap()));
        }
    }

    pub fn visitStatement(&self, ast: &mut Statement) {
        let mut parent = Some(ast.to_ast());
        while let Some(p) = &parent {
            if let AST::NamespaceDefinition(NamespaceDefinition::ConstructorOrFunctionDefinition(
                _,
            )) = p
            {
                break;
            }
            parent = p.parent();
        }
        if parent.is_some() {
            ast.statement_base_mut().unwrap().function =
                parent.map(|p| Box::new(p.constructor_or_function_definition().unwrap()));
        }
    }
}

pub fn set_parents(ast: AST) {
    let v = ParentSetterVisitor::new();
    v.visit(ast.clone());
    let v = ExpressionToStatementVisitor;
    v.visit(ast);
}
