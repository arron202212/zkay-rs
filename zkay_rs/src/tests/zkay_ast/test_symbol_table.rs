use ast_builder::build_ast::build_ast;
use rccell::RcCell;
use std::collections::BTreeMap;
use zkay_ast::ast::{
    is_instance, ASTBaseProperty, ASTInstanceOf, ASTType, AssignmentStatement,
    AssignmentStatementBaseProperty, Block, ConstructorOrFunctionDefinition, ContractDefinition,
    IdentifierBaseProperty, IdentifierDeclarationBaseProperty, IdentifierExpr, IntoAST,
    LocationExprBaseProperty, NamespaceDefinitionBaseProperty, SourceUnit,
    StatementListBaseProperty, VariableDeclaration, VariableDeclarationStatement,
};
use zkay_ast::global_defs::{global_defs, global_vars};
use zkay_ast::pointers::{
    parent_setter::set_parents,
    symbol_table::{fill_symbol_table, get_builtin_globals, link_identifiers},
};
use zkay_examples::examples::{ALL_EXAMPLES, SIMPLE, SIMPLE_STORAGE};
pub struct ASTElements {
    pub contract: RcCell<ContractDefinition>,
    pub f: RcCell<ConstructorOrFunctionDefinition>,
    pub body: Option<Block>,
    pub decl_statement: VariableDeclarationStatement,
    pub decl: RcCell<VariableDeclaration>,
    pub assignment: AssignmentStatement,
    pub identifier_expr: IdentifierExpr,
}
// class TestSimpleAST(ZkayTestCase):

#[cfg(test)]
mod tests {
    use super::*;

    pub fn get_ast_elements(ast: &RcCell<SourceUnit>) -> ASTElements {
        let contract = ast.borrow().contracts[0].clone();
        let f = contract.borrow().function_definitions[0].clone();
        let body = f.borrow().body.as_ref().map(|b| b.borrow().clone());
        let decl_statement = body.as_ref().unwrap().statements()[0].clone();
        assert!(is_instance(
            &decl_statement,
            ASTType::VariableDeclarationStatement
        ));
        let decl = decl_statement
            .clone()
            .try_as_ast()
            .unwrap()
            .borrow()
            .clone()
            .try_as_statement()
            .unwrap()
            .try_as_simple_statement()
            .unwrap()
            .try_as_variable_declaration_statement()
            .unwrap()
            .variable_declaration
            .clone();
        let assignment = body.as_ref().unwrap().statements()[1].clone();
        assert!(is_instance(&assignment, ASTType::AssignmentStatementBase));

        let identifier_expr = assignment
            .try_as_ast_ref()
            .unwrap()
            .borrow()
            .clone()
            .try_as_statement_ref()
            .unwrap()
            .try_as_simple_statement_ref()
            .unwrap()
            .try_as_assignment_statement_ref()
            .unwrap()
            .lhs()
            .clone();
        assert!(is_instance(
            identifier_expr.as_ref().unwrap(),
            ASTType::IdentifierExpr
        ));
        let assignment = assignment
            .try_as_ast_ref()
            .unwrap()
            .borrow()
            .clone()
            .try_as_statement_ref()
            .unwrap()
            .try_as_simple_statement_ref()
            .unwrap()
            .try_as_assignment_statement_ref()
            .unwrap()
            .clone();
        ASTElements {
            contract,
            f,
            body,
            decl_statement: decl_statement
                .try_as_ast()
                .unwrap()
                .borrow()
                .clone()
                .try_as_statement()
                .unwrap()
                .try_as_simple_statement()
                .unwrap()
                .try_as_variable_declaration_statement()
                .unwrap(),
            decl,
            assignment,
            identifier_expr: identifier_expr
                .unwrap()
                .try_as_expression()
                .unwrap()
                .borrow()
                .clone()
                .try_as_tuple_or_location_expr()
                .unwrap()
                .try_as_location_expr()
                .unwrap()
                .try_as_identifier_expr()
                .unwrap(),
        }
    }
    #[test]
    pub fn test_fill_symbol_table_simple() {
        let ast = build_ast(&SIMPLE.code());
        // println!("===ast======{:?}",ast);
        let global_vars = RcCell::new(global_vars(RcCell::new(global_defs())));
        fill_symbol_table(&ast, global_vars.clone());

        let ASTElements {
            contract,
            f,
            body,
            decl,
            ..
        } = get_ast_elements(ast.try_as_source_unit_ref().unwrap());

        let mut s = get_builtin_globals(global_vars.clone());
        s.insert(String::from("Simple"), contract.borrow().idf().clone());

        // assert_eq!(ast.ast_base_ref().unwrap().names(), &s);
        let ss = ast.ast_base_ref().unwrap().borrow().names();
        assert_eq!(s.len(), s.len());
        println!("==len=={:?},{:?}", s.len(), s.len());
        for (k, v) in &ss {
            if let Some(v2) = s.get(k) {
                assert_eq!(
                    v.upgrade().unwrap().borrow().to_string(),
                    v2.upgrade().unwrap().borrow().to_string()
                );
            } else {
                assert!(false);
            }
        }
        for (k, v) in &s {
            if let Some(v2) = ss.get(k) {
                assert_eq!(
                    v.upgrade().unwrap().borrow().to_string(),
                    v2.upgrade().unwrap().borrow().to_string()
                );
            } else {
                assert!(false);
            }
        }
        assert_eq!(
            contract.borrow().names(),
            BTreeMap::from([(String::from("f"), f.borrow().idf().clone())])
        );
        assert_eq!(
            body.unwrap().names(),
            BTreeMap::from([(String::from("x"), decl.borrow().idf().clone())])
        );
    }
    #[test]
    pub fn test_link_identifier_simple() {
        let ast = build_ast(&SIMPLE.code());
        set_parents(&ast);
        link_identifiers(&ast);

        let ASTElements {
            identifier_expr,
            decl,
            ..
        } = get_ast_elements(ast.try_as_source_unit_ref().unwrap());
        println!(
            "=identifier_expr====={:?}======={:?}===",
            identifier_expr.get_ast_type(),
            identifier_expr
        );
        assert_eq!(
            identifier_expr
                .target()
                .clone()
                .unwrap()
                .upgrade()
                .unwrap()
                .get_ast_type(),
            decl.borrow().get_ast_type()
        );
        assert_eq!(
            identifier_expr.get_annotated_type(),
            decl.borrow()
                .annotated_type()
                .as_ref()
                .map(|at| at.borrow().clone())
        );
    }

    // class TestSimpleStorageAST(ZkayTestCase):
    #[test]
    pub fn test_fill_symbol_tables() {
        let ast = build_ast(&SIMPLE_STORAGE.code());
        let global_vars = RcCell::new(global_vars(RcCell::new(global_defs())));
        fill_symbol_table(&ast, global_vars.clone());

        let contract = &ast.try_as_source_unit_ref().unwrap().borrow().contracts[0];
        let mut s = get_builtin_globals(global_vars.clone());
        s.insert(
            String::from("SimpleStorage"),
            contract.borrow().idf().clone(),
        );
        assert_eq!(ast.ast_base_ref().unwrap().borrow().names().len(), s.len());
    }
    #[test]
    pub fn test_link_identifiers_storge() {
        let mut ast = build_ast(&SIMPLE_STORAGE.code());
        set_parents(&mut ast);
        link_identifiers(&mut ast);
        // println!("=======get_item============={:?}",ast
        //     .try_as_source_unit_ref()
        //     .unwrap()
        //     .borrow()
        //     .get_item(&String::from("SimpleStorage")).unwrap()
        //     .try_as_contract_definition_ref()
        //     .as_ref()
        //     .unwrap()
        //     .borrow()
        //     .get_item(&String::from("set")));
        let assignment = &ast
            .try_as_source_unit_ref()
            .unwrap()
            .borrow()
            .get_item(&String::from("SimpleStorage"))
            .unwrap()
            .try_as_contract_definition_ref()
            .as_ref()
            .unwrap()
            .borrow()
            .get_item(&String::from("set"))
            .unwrap()
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .borrow()
            .body
            .as_ref()
            .unwrap()
            .borrow()
            .get_item(0);
        assert!(is_instance(assignment, ASTType::AssignmentStatementBase));
        // println!("=====assignment======{:?}",assignment);

        let stored_data = &assignment
            .try_as_ast_ref()
            .unwrap()
            .borrow()
            .clone()
            .try_as_statement_ref()
            .unwrap()
            .try_as_simple_statement_ref()
            .unwrap()
            .try_as_assignment_statement_ref()
            .unwrap()
            .lhs()
            .as_ref()
            .unwrap()
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .try_as_tuple_or_location_expr_ref()
            .unwrap()
            .try_as_location_expr_ref()
            .unwrap()
            .target()
            .clone()
            .unwrap()
            .upgrade()
            .unwrap();

        assert_eq!(
            Some(stored_data.clone()),
            ast.try_as_source_unit_ref()
                .as_ref()
                .unwrap()
                .borrow()
                .get_item(&String::from("SimpleStorage"))
                .unwrap()
                .try_as_contract_definition_ref()
                .as_ref()
                .unwrap()
                .borrow()
                .get_item(&String::from("storedData"))
        );

        let x = assignment
            .try_as_ast_ref()
            .unwrap()
            .borrow()
            .clone()
            .try_as_statement_ref()
            .unwrap()
            .try_as_simple_statement_ref()
            .unwrap()
            .try_as_assignment_statement_ref()
            .unwrap()
            .rhs()
            .as_ref()
            .unwrap()
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .clone()
            .try_as_tuple_or_location_expr_ref()
            .unwrap()
            .try_as_location_expr_ref()
            .unwrap()
            .target()
            .clone()
            .unwrap()
            .upgrade()
            .unwrap();
        println!("=======x============={:?}", x);
        assert_eq!(
            x.to_ast()
                .try_as_identifier_declaration_ref()
                .unwrap()
                .idf()
                .upgrade()
                .unwrap()
                .borrow()
                .name(),
            String::from("x")
        );
    }

    // @parameterized_class(("name", "example"), all_examples)
    // class TestSymbolTable(TestExamples):
    #[test]
    pub fn test_symbol_tables() {
        for (name, example) in ALL_EXAMPLES.iter() {
            let ast = build_ast(&example.code());
            println!("=test_symbol_tables======{name}");
            set_parents(&ast);
            let global_vars = RcCell::new(global_vars(RcCell::new(global_defs())));
            fill_symbol_table(&ast, global_vars);
            link_identifiers(&ast);
            let contract = &ast.try_as_source_unit_ref().unwrap().borrow().contracts[0];
            assert_eq!(
                &contract.borrow().idf().upgrade().unwrap().borrow().name(),
                name
            );
        }
    }
}
