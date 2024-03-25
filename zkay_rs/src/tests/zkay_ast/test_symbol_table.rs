use ast_builder::build_ast::build_ast;
use std::collections::BTreeMap;
use zkay_ast::ast::{
    is_instance, ASTBaseProperty, ASTType, AssignmentStatement, AssignmentStatementBaseProperty,
    Block, ConstructorOrFunctionDefinition, ContractDefinition, IdentifierBaseProperty,
    IdentifierDeclarationBaseProperty, IdentifierExpr, IntoAST, LocationExprBaseProperty,
    NamespaceDefinitionBaseProperty, SourceUnit, StatementListBaseProperty, VariableDeclaration,
    VariableDeclarationStatement,
};
use zkay_ast::pointers::{
    parent_setter::set_parents,
    symbol_table::{fill_symbol_table, get_builtin_globals, link_identifiers},
};
use zkay_examples::examples::{ALL_EXAMPLES, SIMPLE, SIMPLE_STORAGE};
pub struct ASTElements {
    pub contract: ContractDefinition,
    pub f: ConstructorOrFunctionDefinition,
    pub body: Option<Block>,
    pub decl_statement: VariableDeclarationStatement,
    pub decl: VariableDeclaration,
    pub assignment: AssignmentStatement,
    pub identifier_expr: IdentifierExpr,
}
// class TestSimpleAST(ZkayTestCase):

#[cfg(test)]
mod tests {
    use super::*;

    pub fn get_ast_elements(ast: &SourceUnit) -> ASTElements {
        let contract = ast.contracts[0].clone();
        let f = contract.function_definitions[0].clone();
        let body = f.body.clone();
        let decl_statement = body.as_ref().unwrap().statements()[0].clone();
        assert!(is_instance(
            &decl_statement,
            ASTType::VariableDeclarationStatement
        ));
        let decl = decl_statement
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
            .try_as_statement_ref()
            .unwrap()
            .try_as_simple_statement_ref()
            .unwrap()
            .try_as_assignment_statement_ref()
            .unwrap()
            .lhs()
            .clone();
        assert!(is_instance(
            &**identifier_expr.as_ref().unwrap(),
            ASTType::IdentifierExpr
        ));
        ASTElements {
            contract,
            f,
            body,
            decl_statement: decl_statement
                .try_as_statement()
                .unwrap()
                .try_as_simple_statement()
                .unwrap()
                .try_as_variable_declaration_statement()
                .unwrap(),
            decl,
            assignment: assignment
                .try_as_statement()
                .unwrap()
                .try_as_simple_statement()
                .unwrap()
                .try_as_assignment_statement()
                .unwrap(),
            identifier_expr: identifier_expr
                .unwrap()
                .try_as_expression()
                .unwrap()
                .try_as_tuple_or_location_expr()
                .unwrap()
                .try_as_location_expr()
                .unwrap()
                .try_as_identifier_expr()
                .unwrap(),
        }
    }
    #[test]
    pub fn test_fill_symbol_table() {
        let ast = build_ast(&SIMPLE.code());
        fill_symbol_table(&ast);

        let ASTElements {
            contract,
            f,
            body,
            decl_statement,
            decl,
            assignment,
            identifier_expr,
        } = get_ast_elements(ast.try_as_source_unit_ref().unwrap());

        let mut s = get_builtin_globals();
        s.insert(String::from("Simple"), contract.idf().clone());
        assert_eq!(ast.ast_base_ref().unwrap().names(), &s);
        assert_eq!(
            contract.names(),
            &BTreeMap::from([(String::from("f"), f.idf().clone())])
        );
        assert_eq!(
            body.unwrap().names(),
            &BTreeMap::from([(String::from("x"), *decl.idf().clone())])
        );
    }
    #[test]
    pub fn test_link_identifiers() {
        let mut ast = build_ast(&SIMPLE.code());
        set_parents(&mut ast);
        link_identifiers(&ast);

        let ASTElements {
            contract,
            f,
            body,
            decl_statement,
            decl,
            assignment,
            identifier_expr,
        } = get_ast_elements(ast.try_as_source_unit_ref().unwrap());

        assert_eq!(
            *identifier_expr.target().as_ref().unwrap().clone(),
            decl.to_ast()
        );
        assert_eq!(
            identifier_expr.annotated_type(),
            Some(*decl.annotated_type().clone())
        );
    }

    // class TestSimpleStorageAST(ZkayTestCase):
    #[test]
    pub fn test_fill_symbol_tables() {
        let ast = build_ast(&SIMPLE_STORAGE.code());
        fill_symbol_table(&ast);

        let contract = &ast.try_as_source_unit_ref().unwrap().contracts[0];

        let mut s = get_builtin_globals();
        s.insert(String::from("SimpleStorage"), contract.idf().clone());
        assert_eq!(ast.ast_base_ref().unwrap().names(), &s);
    }
    #[test]
    pub fn test_link_identifierss() {
        let mut ast = build_ast(&SIMPLE_STORAGE.code());
        set_parents(&mut ast);
        link_identifiers(&ast);
        let assignment = &ast
            .try_as_source_unit_ref()
            .unwrap()
            .get_item(&String::from("SimpleStorage"))
            .unwrap()
            .get_item(&String::from("set"))
            .unwrap()
            .try_as_namespace_definition_ref()
            .unwrap()
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .get_item(0);
        assert!(is_instance(assignment, ASTType::AssignmentStatementBase));

        let stored_data = &assignment
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
            .try_as_tuple_or_location_expr_ref()
            .unwrap()
            .try_as_location_expr_ref()
            .unwrap()
            .target();
        assert_eq!(
            stored_data.as_ref().map(|s| *s.clone()),
            ast.try_as_source_unit_ref()
                .as_ref()
                .unwrap()
                .get_item(&String::from("SimpleStorage"))
                .unwrap()
                .get_item(&String::from("storedData"))
        );

        let x = assignment
            .try_as_statement_ref()
            .unwrap()
            .try_as_simple_statement_ref()
            .unwrap()
            .try_as_assignment_statement_ref()
            .unwrap()
            .rhs()
            .as_ref()
            .unwrap()
            .try_as_tuple_or_location_expr_ref()
            .unwrap()
            .try_as_location_expr_ref()
            .unwrap()
            .target();
        assert_eq!(
            x.as_ref()
                .unwrap()
                .try_as_identifier_declaration_ref()
                .unwrap()
                .idf()
                .name(),
            &String::from("x")
        );
    }

    // @parameterized_class(("name", "example"), all_examples)
    // class TestSymbolTable(TestExamples):
    #[test]
    pub fn test_symbol_table() {
        for (name, example) in ALL_EXAMPLES.iter() {
            let mut ast = build_ast(&example.code());
            set_parents(&mut ast);
            fill_symbol_table(&ast);
            link_identifiers(&ast);
            let contract = &ast.try_as_source_unit_ref().unwrap().contracts[0];
            assert_eq!(contract.idf().name(), name);
        }
    }
}
