use crate::zkay_ast::ast::{
    ASTChildren, ConstructorOrFunctionDefinition, Expression, Identifier, NamespaceDefinition,
    SourceUnit, Statement, AST,ASTCode,
};
use crate::zkay_ast::visitor::visitor::AstVisitor;

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

    pub fn visitNamespaceDefinition(&self, ast: NamespaceDefinition) {
        ast.ast_base_mut().namespace = (if let Some(parent) = ast.parent() {
            parent.ast_base().namespace.clone()
        } else {
            None
        })
        .into_iter()
        .chain([ast.namespace_definition_base().idf.clone()])
        .collect();
    }

    pub fn visitConstructorOrFunctionDefinition(&self, ast: &mut ConstructorOrFunctionDefinition) {
        ast.namespace_definition_base.ast_base.namespace = (if let Some(parent) = ast.parent {
            parent.namespace_definition_base.ast_base.namespace.clone()
        } else {
            None
        })
        .into_iter()
        .chain(vec![ast.namespace_definition_base.idf.clone()])
        .collect();
    }

    pub fn visitChildren(&self, ast: &mut AST) {
        for c in ast.children() {
            if AST::default()!=c {
                c.ast_base_mut().parent = Some(Box::new(ast.clone()));
                c.ast_base_mut().namespace = ast.ast_base().namespace.clone();
                self.visit(c);
            } else {
                print!("{:?},{:?}, {:?}", c, ast, ast.children());
            }
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
        let parent = Some(ast.get_ast());
        while let Some(p) = parent {
            if let AST::Statement(_) = p {
                break;
            }
            parent = p.parent();
        }
        if parent.is_some() {
            ast.expression_base_mut().statement = parent.map(|p|Box::new(p));
        }
    }

    pub fn visitStatement(&self, ast: &mut Statement) {
        let mut parent = Some(ast.get_ast());
        while let Some(p) = parent {
            if let AST::NamespaceDefinition(NamespaceDefinition::ConstructorOrFunctionDefinition(
                _,
            )) = p
            {
                break;
            }
            parent = p.parent();
        }
        if parent.is_some() {
            ast.statement_base_mut().function = parent.map(|p|Box::new(p));
        }
    }
}

pub fn set_parents(ast: AST) {
    let v = ParentSetterVisitor::new();
    v.visit(ast);
    let v = ExpressionToStatementVisitor;
    v.visit(ast);
}
