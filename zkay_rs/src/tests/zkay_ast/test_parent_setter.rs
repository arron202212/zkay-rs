use ast_builder::build_ast::build_ast;
use zkay_ast::ast::{
    is_instance, ASTBaseProperty, ASTChildren, ASTType, IntoAST, NamespaceDefinitionBaseProperty,
    SourceUnit, AST,
};
use zkay_ast::pointers::parent_setter::set_parents;
use zkay_ast::visitor::visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef};
use zkay_derive::ASTVisitorBaseRefImpl;
use zkay_examples::examples::ALL_EXAMPLES;
#[derive(ASTVisitorBaseRefImpl)]
struct ParentChecker {
    pub ast_visitor_base: AstVisitorBase,
}
impl ParentChecker {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
        }
    }
}
impl AstVisitor for ParentChecker {
    fn visit(&self, ast: &AST) -> Self::Return {
        if !is_instance(ast, ASTType::SourceUnit) {
            assert!(ast.ast_base_ref().unwrap().parent().is_some());
        }
        self._visit_internal(ast);
        None
    }
    type Return = Option<()>;
    fn temper_result(&self) -> Self::Return {
        None
    }
    fn has_attr(&self, _name: &ASTType) -> bool {
        false
    }
    fn get_attr(&self, _name: &ASTType, _ast: &AST) -> Self::Return {
        self.temper_result()
    }
}

// @parameterized_class(('name', 'example'), all_examples)
// class TestParentSetter(TestExamples):
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_root_children_have_parent() {
        for (name, example) in ALL_EXAMPLES.iter() {
            let mut ast = build_ast(&example.code());
            set_parents(&mut ast);

            // test
            for c in ast.children() {
                assert_eq!(
                    c.ast_base_ref().unwrap().parent(),
                    Some(Box::new(ast.clone()))
                );
            }
        }
    }
    #[test]
    pub fn test_contract_identifier() {
        for (name, example) in ALL_EXAMPLES.iter() {
            let mut ast = build_ast(&example.code());
            // println!("{:?},====={:?}",name,ast);
            set_parents(&mut ast);
            // println!("{:?},==after==={:?}",name,ast);
            // test
            let contract = &ast.try_as_source_unit_ref().unwrap().contracts[0];
            let idf = contract.idf();
            assert_eq!(idf.parent(), Some(Box::new(contract.to_ast())));
        }
    }
    #[test]
    pub fn test_all_nodes_have_parent() {
        for (name, example) in ALL_EXAMPLES.iter() {
            let mut ast = build_ast(&example.code());
            set_parents(&mut ast);

            // test
            let mut v = ParentChecker::new();
            v.visit(&ast);
        }
    }
}
