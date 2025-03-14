// @parameterized_class(('name', 'example'), all_examples)
// class TestParentSetter(TestExamples):
#[cfg(test)]
mod tests {
    use super::*;
    use ast_builder::build_ast::build_ast;
    // use rccell::{RcCell, WeakCell};
    use zkay_ast::ast::{AST, ASTBaseProperty, ASTChildren, ASTFlatten, ASTType, is_instance};
    use zkay_ast::pointers::parent_setter::set_parents;
    use zkay_ast::visitors::visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef};
    use zkay_derive::ASTVisitorBaseRefImpl;
    use zkay_examples::examples::ALL_EXAMPLES;
    #[derive(ASTVisitorBaseRefImpl)]
    struct ParentChecker {
        pub ast_visitor_base: AstVisitorBase,
    }
    impl ParentChecker {
        fn new() -> Self {
            Self {
                ast_visitor_base: AstVisitorBase::new("post", false),
            }
        }
    }
    impl AstVisitor for ParentChecker {
        fn visit(&self, ast: &ASTFlatten) -> eyre::Result<Self::Return> {
            if !is_instance(ast, ASTType::SourceUnit) {
                assert!(ast.ast_base_ref().unwrap().borrow().parent().is_some());
            }
            self._visit_internal(ast)
        }
        type Return = ();
        fn temper_result(&self) -> Self::Return {}
        fn has_attr(&self, _name: &ASTType, _ast: &AST) -> bool {
            false
        }
        fn get_attr(&self, _name: &ASTType, _ast: &ASTFlatten) -> eyre::Result<Self::Return> {
            Err(eyre::eyre!("unreach"))
        }
    }
    #[test]
    pub fn test_root_children_have_parent() {
        for (_name, example) in ALL_EXAMPLES.iter() {
            let mut ast = build_ast(&example.code());
            set_parents(&mut ast);

            // test
            for c in ast.children() {
                assert_eq!(
                    c.ast_base_ref().unwrap().borrow().parent(),
                    Some(ast.clone().downgrade())
                );
            }
        }
    }
    #[test]
    pub fn test_contract_identifier() {
        for (_name, example) in ALL_EXAMPLES.iter() {
            let mut ast = build_ast(&example.code());
            // println!("{:?},====={:?}",name,ast);
            set_parents(&mut ast);
            // println!("{:?},==after==={:?}",name,ast);
            // test
            let contract = &ast.try_as_source_unit_ref().unwrap().borrow().contracts[0];
            let idf = contract.borrow().idf();
            assert_eq!(
                idf.unwrap()
                    .borrow()
                    .parent()
                    .as_ref()
                    .and_then(|p| p.clone().upgrade()),
                Some(contract.clone().into())
            );
        }
    }
    #[test]
    pub fn test_all_nodes_have_parent() {
        for (_name, example) in ALL_EXAMPLES.iter() {
            let ast = build_ast(&example.code());
            set_parents(&ast);
            println!("================={_name}");
            // test
            let v = ParentChecker::new();
            let _ = v.visit(&ast);
        }
    }
}
