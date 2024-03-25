#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

// from typing import Tuple, Dict, Union
use crate::ast::{
    is_instance, is_instances, ASTBaseMutRef, ASTBaseProperty, ASTBaseRef, ASTChildren, ASTType,
    AnnotatedTypeName, Array, Block, Comment, ConstructorOrFunctionDefinition, ContractDefinition,
    EnumDefinition, EnumValue, Expression, ExpressionBaseProperty, ForStatement, Identifier,
    IdentifierBase, IdentifierBaseProperty, IdentifierDeclaration,
    IdentifierDeclarationBaseProperty, IdentifierExpr, IndexExpr, IntoAST, LocationExpr,
    LocationExprBaseProperty, Mapping, MemberAccessExpr, NamespaceDefinition, SimpleStatement,
    SourceUnit, Statement, StatementList, StatementListBaseProperty, StructDefinition,
    TupleOrLocationExpr, TypeName, UserDefinedTypeName, UserDefinedTypeNameBaseMutRef,
    UserDefinedTypeNameBaseProperty, UserDefinedTypeNameBaseRef, VariableDeclaration,
    VariableDeclarationStatement, AST,
};
use crate::global_defs::{array_length_member, global_defs, global_vars};
use serde::{Deserialize, Serialize};
// from zkay::crate::pointers::pointer_exceptions import UnknownIdentifierException
use crate::visitor::visitor::{AstVisitorBase, AstVisitorBaseRef, AstVisitorMut};
use zkay_derive::ASTVisitorBaseRefImpl;
pub fn fill_symbol_table(ast: &mut AST) {
    let mut v = SymbolTableFiller::new();
    v.visit(ast);
}

pub fn link_symbol_table(ast: &mut AST) {
    let mut v = SymbolTableLinker::new();
    v.visit(ast);
}

pub fn link_identifiers(ast: &mut AST) {
    fill_symbol_table(ast);
    link_symbol_table(ast);
}
use std::collections::BTreeMap;
pub fn merge_dicts(dict_args: Vec<BTreeMap<String, Identifier>>) -> BTreeMap<String, Identifier>
// """
    // Given any number of dicts, shallow copy and merge into a new dict.
    // Report error on conflicting keys.
    // """
{
    let mut result = BTreeMap::new();
    for dictionary in dict_args {
        for (key, value) in dictionary {
            if let Some(v) = result.get(&key) {
                if *v != value {
                    // raise ValueError("Conflicting definitions for", key)
                    assert!(false, "Conflicting definitions for {}", key);
                }
            }
            result.insert(key.clone(), value.clone());
        }
    }
    result
}

pub fn collect_children_names(ast: &mut AST) -> BTreeMap<String, Identifier> {
    let mut children: Vec<_> = ast
        .children()
        .iter()
        .filter_map(|c| {
            if is_instances(c, vec![ASTType::Block, ASTType::ForStatement]) {
                None
            } else {
                Some(c.clone())
            }
        })
        .collect();
    let names: Vec<_> = children
        .iter()
        .map(|mut c| c.ast_base_ref().unwrap().names.clone())
        .collect();
    let ret = merge_dicts(names);
    for c in children.iter_mut()
    //declared names are not available within the declaration statements
    {
        c.ast_base_mut_ref().unwrap().names.clear();
    }
    ret
}

pub fn get_builtin_globals() -> BTreeMap<String, Identifier> {
    let mut sf = SymbolTableFiller::new();
    sf.get_builtin_globals()
}

#[derive(ASTVisitorBaseRefImpl)]
struct SymbolTableFiller {
    pub ast_visitor_base: AstVisitorBase,
}

impl AstVisitorMut for SymbolTableFiller {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}

    fn has_attr(&self, name: &ASTType) -> bool {
        &ASTType::SourceUnit == name
            || &ASTType::ContractDefinition == name
            || &ASTType::ConstructorOrFunctionDefinition == name
            || &ASTType::StructDefinition == name
            || &ASTType::EnumDefinition == name
            || &ASTType::VariableDeclaration == name
            || &ASTType::StatementListBase == name
            || &ASTType::SimpleStatementBase == name
            || &ASTType::ForStatement == name
            || &ASTType::Mapping == name
    }
    fn get_attr(&mut self, name: &ASTType, ast: &mut AST) -> Self::Return {
        match name {
            ASTType::SourceUnit => self.visitSourceUnit(ast.try_as_source_unit_mut().unwrap()),
            ASTType::ContractDefinition => self.visitContractDefinition(
                ast.try_as_namespace_definition_mut()
                    .unwrap()
                    .try_as_contract_definition_mut()
                    .unwrap(),
            ),
            ASTType::ConstructorOrFunctionDefinition => self.visitConstructorOrFunctionDefinition(
                ast.try_as_namespace_definition_mut()
                    .unwrap()
                    .try_as_constructor_or_function_definition_mut()
                    .unwrap(),
            ),
            ASTType::StructDefinition => self.visitStructDefinition(
                ast.try_as_namespace_definition_mut()
                    .unwrap()
                    .try_as_struct_definition_mut()
                    .unwrap(),
            ),
            ASTType::EnumDefinition => self.visitEnumDefinition(
                ast.try_as_namespace_definition_mut()
                    .unwrap()
                    .try_as_enum_definition_mut()
                    .unwrap(),
            ),
            ASTType::EnumValue => self.visitEnumValue(ast.try_as_enum_value_mut().unwrap()),
            ASTType::VariableDeclaration => self.visitVariableDeclaration(
                ast.try_as_identifier_declaration_mut()
                    .unwrap()
                    .try_as_variable_declaration_mut()
                    .unwrap(),
            ),
            ASTType::StatementListBase => self.visitStatementList(
                ast.try_as_statement_mut()
                    .unwrap()
                    .try_as_statement_list_mut()
                    .unwrap(),
            ),
            ASTType::SimpleStatementBase => self.visitSimpleStatement(
                ast.try_as_statement_mut()
                    .unwrap()
                    .try_as_simple_statement_mut()
                    .unwrap(),
            ),
            ASTType::ForStatement => self.visitForStatement(
                ast.try_as_statement_mut()
                    .unwrap()
                    .try_as_for_statement_mut()
                    .unwrap(),
            ),
            ASTType::Mapping => self.visitMapping(
                ast.try_as_type_name_mut()
                    .unwrap()
                    .try_as_mapping_mut()
                    .unwrap(),
            ),
            _ => {}
        }
    }
}
// class SymbolTableFiller(AstVisitorMut)
impl SymbolTableFiller {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
        }
    }
    pub fn get_builtin_globals(&mut self) -> BTreeMap<String, Identifier> {
        let mut global_defs = global_defs().vars();
        for d in global_defs.iter_mut() {
            self.visit(&mut (*d).to_ast());
        }
        let global_defs = global_defs
            .iter()
            .map(|d| {
                (
                    d.namespace_definition_base.idf.name().clone(),
                    d.namespace_definition_base.idf.clone(),
                )
            })
            .collect();
        let global_vars = global_vars()
            .vars()
            .into_iter()
            .map(|d| {
                (
                    d.identifier_declaration_base.idf.name().clone(),
                    *d.identifier_declaration_base.idf.clone(),
                )
            })
            .collect();
        merge_dicts(vec![global_defs, global_vars])
    }

    pub fn visitSourceUnit(&mut self, ast: &mut SourceUnit) {
        ast.ast_base.names = ast
            .contracts
            .iter()
            .map(|d| {
                (
                    d.namespace_definition_base.idf.name().clone(),
                    d.namespace_definition_base.idf.clone(),
                )
            })
            .collect();
        ast.ast_base.names.append(&mut self.get_builtin_globals());
    }

    pub fn visitContractDefinition(&mut self, ast: &mut ContractDefinition) {
        let state_vars = ast
            .state_variable_declarations
            .iter()
            .filter_map(|d| {
                if is_instance(d, ASTType::CommentBase) {
                    None
                } else {
                    Some((
                        d.try_as_identifier_declaration_ref()
                            .unwrap()
                            .idf()
                            .name()
                            .clone(),
                        *d.try_as_identifier_declaration_ref().unwrap().idf().clone(),
                    ))
                }
            })
            .collect();
        let mut funcs = BTreeMap::new();
        for f in &ast.function_definitions {
            if funcs.contains_key(f.namespace_definition_base.idf.name()) {
                // raise UnknownIdentifierException(f"Zkay does not currently support method overloading.", f)
                assert!(
                    false,
                    "Zkay does not currently support method overloading.{:?}",
                    f
                );
            }
            funcs.insert(
                f.namespace_definition_base.idf.name().clone(),
                f.namespace_definition_base.idf.clone(),
            );
        }
        let structs = ast
            .struct_definitions
            .iter()
            .map(|d| {
                (
                    d.namespace_definition_base.idf.name().clone(),
                    d.namespace_definition_base.idf.clone(),
                )
            })
            .collect();
        let enums = ast
            .enum_definitions
            .iter()
            .map(|d| {
                (
                    d.namespace_definition_base.idf.name().clone(),
                    d.namespace_definition_base.idf.clone(),
                )
            })
            .collect();
        ast.namespace_definition_base.ast_base.names =
            merge_dicts(vec![state_vars, funcs, structs, enums]);
    }

    pub fn visitConstructorOrFunctionDefinition(
        &mut self,
        ast: &mut ConstructorOrFunctionDefinition,
    ) {
        ast.namespace_definition_base.ast_base.names = ast
            .parameters
            .iter()
            .map(|d| {
                (
                    d.identifier_declaration_base.idf.name().clone(),
                    *d.identifier_declaration_base.idf.clone(),
                )
            })
            .collect();
    }

    pub fn visitStructDefinition(&mut self, ast: &mut StructDefinition) {
        ast.namespace_definition_base.ast_base.names = ast
            .members
            .iter()
            .map(|d| {
                (
                    d.try_as_identifier_declaration_ref()
                        .unwrap()
                        .idf()
                        .name()
                        .clone(),
                    *d.try_as_identifier_declaration_ref().unwrap().idf().clone(),
                )
            })
            .collect();
    }
    pub fn visitEnumDefinition(&mut self, ast: &mut EnumDefinition) {
        ast.namespace_definition_base.ast_base.names = ast
            .values
            .iter()
            .map(|d| {
                (
                    d.idf.as_ref().unwrap().name().clone(),
                    d.idf.clone().unwrap(),
                )
            })
            .collect();
    }
    pub fn visitEnumValue(&mut self, _ast: &mut EnumValue) {}

    pub fn visitVariableDeclaration(&mut self, ast: &mut VariableDeclaration) {
        ast.identifier_declaration_base.ast_base.names = BTreeMap::from([(
            ast.identifier_declaration_base.idf.name().clone(),
            *ast.identifier_declaration_base.idf.clone(),
        )]);
    }

    pub fn visitStatementList(&mut self, ast: &mut StatementList) {
        ast.ast_base_mut_ref().names = collect_children_names(&mut ast.to_ast());
    }

    pub fn visitSimpleStatement(&mut self, ast: &mut SimpleStatement) {
        ast.ast_base_mut_ref().names = collect_children_names(&mut ast.to_ast());
    }

    pub fn visitForStatement(&mut self, ast: &mut ForStatement) {
        ast.ast_base_mut_ref().names = collect_children_names(&mut ast.to_ast());
    }

    pub fn visitMapping(&mut self, ast: &mut Mapping) {
        ast.type_name_base.ast_base.names = BTreeMap::new();
        if is_instance(ast.key_label.as_ref().unwrap(), ASTType::IdentifierBase) {
            ast.type_name_base.ast_base.names = BTreeMap::from([(
                ast.key_label.clone().unwrap().name().clone(),
                ast.key_label.clone().unwrap(),
            )]);
        }
    }
}

#[derive(ASTVisitorBaseRefImpl)]
pub struct SymbolTableLinker {
    pub ast_visitor_base: AstVisitorBase,
}
// class SymbolTableLinker(AstVisitorMut)
impl AstVisitorMut for SymbolTableLinker {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}
    fn has_attr(&self, name: &ASTType) -> bool {
        &ASTType::IdentifierExpr == name
            || &ASTType::UserDefinedTypeNameBase == name
            || &ASTType::MemberAccessExpr == name
            || &ASTType::IndexExpr == name
    }
    fn get_attr(&mut self, name: &ASTType, ast: &mut AST) -> Self::Return {
        match name {
            ASTType::IdentifierExpr => self.visitIdentifierExpr(
                ast.try_as_expression_mut()
                    .unwrap()
                    .try_as_tuple_or_location_expr_mut()
                    .unwrap()
                    .try_as_location_expr_mut()
                    .unwrap()
                    .try_as_identifier_expr_mut()
                    .unwrap(),
            ),
            ASTType::UserDefinedTypeNameBase => self.visitUserDefinedTypeName(
                ast.try_as_type_name_mut()
                    .unwrap()
                    .try_as_user_defined_type_name_mut()
                    .unwrap(),
            ),
            ASTType::MemberAccessExpr => self.visitMemberAccessExpr(
                ast.try_as_expression_mut()
                    .unwrap()
                    .try_as_tuple_or_location_expr_mut()
                    .unwrap()
                    .try_as_location_expr_mut()
                    .unwrap()
                    .try_as_member_access_expr_mut()
                    .unwrap(),
            ),
            ASTType::IndexExpr => self.visitIndexExpr(
                ast.try_as_expression_mut()
                    .unwrap()
                    .try_as_tuple_or_location_expr_mut()
                    .unwrap()
                    .try_as_location_expr_mut()
                    .unwrap()
                    .try_as_index_expr_mut()
                    .unwrap(),
            ),

            _ => {}
        }
    }
}
impl SymbolTableLinker {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
        }
    }
    pub fn _find_next_decl(ast: AST, name: String) -> (Option<AST>, Option<AST>) {
        let mut ancestor = ast.ast_base_ref().unwrap().parent().clone();
        while let Some(_ancestor) = ancestor {
            if let Some(nameo) = _ancestor.ast_base_ref().unwrap().names().get(&name) {
                let decl = nameo.parent();
                if !is_instance(
                    &**decl
                        .clone()
                        .unwrap()
                        .ast_base_ref()
                        .unwrap()
                        .parent()
                        .as_ref()
                        .unwrap(),
                    ASTType::VariableDeclarationStatement,
                ) || !decl
                    .clone()
                    .unwrap()
                    .ast_base_ref()
                    .unwrap()
                    .parent()
                    .as_ref()
                    .unwrap()
                    .is_parent_of(&ast)
                {
                    return (Some(*_ancestor.clone()), decl.as_ref().map(|d| *d.clone()));
                }
            }
            ancestor = _ancestor.ast_base_ref().unwrap().parent().clone();
        }
        // raise UnknownIdentifierException(f"Undefined identifier {name}", ast)
        // assert!(false, "Undefined identifier {name},{:?}", ast);
        (None, None)
    }

    pub fn _find_lca(mut ast1: AST, mut ast2: AST, root: AST) -> (StatementList, AST, AST) {
        assert!(ast1 != ast2);

        // Gather ast1"s ancestors + immediate child towards ast1 (for each)
        let mut ancs = BTreeMap::new();
        loop {
            assert!(ast1.ast_base_ref().unwrap().parent().is_some());
            ancs.insert(
                ast1.ast_base_ref()
                    .unwrap()
                    .parent()
                    .as_ref()
                    .unwrap()
                    .clone(),
                ast1.clone(),
            );
            ast1 = *ast1
                .ast_base_ref()
                .unwrap()
                .parent()
                .as_ref()
                .unwrap()
                .clone();
            if ast1 == root {
                break;
            }
        }

        // Find least common ancestor with ast2 + immediate child towards ast2
        loop {
            assert!(ast2.ast_base_ref().unwrap().parent().is_some());
            let old_ast = ast2.clone();
            let ast2 = ast2.ast_base_ref().unwrap().parent().clone();
            if let Some(ast2v) = ancs.get(&ast2.clone().unwrap()) {
                assert!(is_instances(
                    &**ast2.as_ref().unwrap(),
                    vec![ASTType::ForStatement, ASTType::StatementListBase],
                ));
                return (
                    ast2.clone()
                        .map(|a| {
                            a.try_as_statement()
                                .unwrap()
                                .try_as_statement_list()
                                .unwrap()
                        })
                        .unwrap(),
                    ast2v.clone(),
                    old_ast,
                );
            }
        }
    }

    pub fn find_type_declaration(&self, t: UserDefinedTypeName) -> Option<NamespaceDefinition> {
        SymbolTableLinker::_find_next_decl(
            AST::TypeName(TypeName::UserDefinedTypeName(t.clone())),
            t.user_defined_type_name_base_ref().names[0].name().clone(),
        )
        .1
        .unwrap()
        .try_as_namespace_definition_ref()
        .map(|nd| nd.clone())
    }

    pub fn find_identifier_declaration(&self, ast: &mut IdentifierExpr) -> AST {
        let mut _ans = ast.to_ast(); //TODO side effect
        let name = ast.idf.name();
        loop {
            let (anc, decl) = SymbolTableLinker::_find_next_decl(ast.to_ast(), name.clone());
            if let (Some(AST::Statement(Statement::ForStatement(anc))), Some(decl)) = (
                &anc,
                &decl
                    .as_ref()
                    .unwrap()
                    .try_as_identifier_declaration_ref()
                    .unwrap()
                    .try_as_variable_declaration_ref(),
            ) {
                // Check if identifier really references this declaration (does not come before declaration)
                let (lca, ref_anchor, decl_anchor) =
                    SymbolTableLinker::_find_lca(ast.to_ast(), decl.to_ast(), anc.to_ast());
                if lca.statements().iter().find(|x| (*x).clone() == ref_anchor)
                    <= lca
                        .statements()
                        .iter()
                        .find(|x| (*x).clone() == decl_anchor)
                {
                    _ans = anc.to_ast();
                    continue;
                }
            }
            if let (
                Some(AST::Statement(Statement::StatementList(StatementList::Block(anc)))),
                Some(decl),
            ) = (
                &anc,
                &decl
                    .as_ref()
                    .unwrap()
                    .try_as_identifier_declaration_ref()
                    .unwrap()
                    .try_as_variable_declaration_ref(),
            ) {
                // Check if identifier really references this declaration (does not come before declaration)
                let (lca, ref_anchor, decl_anchor) =
                    SymbolTableLinker::_find_lca(ast.to_ast(), decl.to_ast(), anc.to_ast());
                if lca.statements().iter().find(|x| (*x).clone() == ref_anchor)
                    <= lca
                        .statements()
                        .iter()
                        .find(|x| (*x).clone() == decl_anchor)
                {
                    _ans = anc.to_ast();
                    continue;
                }
            }
            return decl.unwrap();
        }
    }

    pub fn in_scope_at(target_idf: &Identifier, ast: AST) -> bool {
        let mut ancestor = ast.ast_base_ref().unwrap().parent().clone();
        while let Some(_ancestor) = ancestor {
            if let Some(name) = _ancestor
                .ast_base_ref()
                .unwrap()
                .names()
                .get(target_idf.name())
            // found name
            {
                if name == target_idf {
                    return true;
                }
            }
            ancestor = _ancestor.ast_base_ref().unwrap().parent().clone();
        }
        false
    }

    pub fn visitIdentifierExpr(&mut self, ast: &mut IdentifierExpr) {
        let decl = self.find_identifier_declaration(ast);
        ast.location_expr_base.target = Some(Box::new(decl.to_ast()));
        assert!(ast.location_expr_base.target.as_ref().is_some());
        // ast.clone()
    }

    pub fn visitUserDefinedTypeName(&mut self, ast: &mut UserDefinedTypeName) {
        let mut type_def = self.find_type_declaration(ast.clone());
        for idf in &ast.user_defined_type_name_base_ref().names[1..] {
            if let Some(_idf) = type_def.as_ref().unwrap().names().get(idf.name()) {
                if let Some(AST::NamespaceDefinition(parent)) =
                    _idf.parent().as_ref().map(|p| *p.clone())
                {
                    type_def = Some(parent.clone());
                }
            }
        }
        ast.user_defined_type_name_base_mut_ref().target =
            Some(Box::new(type_def.unwrap().to_ast()));

        // ast
    }
    pub fn visitMemberAccessExpr(&mut self, ast: &mut MemberAccessExpr) {
        assert!(
            is_instance(&**ast.expr.as_ref().unwrap(), ASTType::LocationExprBase),
            "Function call return value member access not yet supported"
        );
        if let Some(target) = ast
            .expr
            .as_ref()
            .unwrap()
            .target()
            .as_ref()
            .unwrap()
            .try_as_namespace_definition_ref()
        {
            if let Some(idf) = target.names().get(ast.member.name()) {
                ast.location_expr_base.target = idf.parent().clone();
            }
        } else {
            let mut t = *ast
                .expr
                .as_ref()
                .unwrap()
                .target()
                .as_ref()
                .unwrap()
                .try_as_expression_ref()
                .unwrap()
                .annotated_type()
                .as_ref()
                .unwrap()
                .type_name
                .clone()
                .unwrap();
            if let TypeName::Array(_) = &t {
                assert!(ast.member.name() == "length");
                ast.location_expr_base.target = Some(Box::new(array_length_member().into_ast()));
            } else if let TypeName::UserDefinedTypeName(ref mut t) = t {
                // assert!(isinstance(t, UserDefinedTypeName));
                if let Some(target) = t.target() {
                    if let Some(idf) = target
                        .ast_base_ref()
                        .unwrap()
                        .names()
                        .get(ast.member.name())
                    {
                        ast.location_expr_base.target = idf.parent().clone();
                    }
                } else {
                    *t = t.clone();
                    t.ast_base_mut_ref()
                        .parent_namespace
                        .as_mut()
                        .unwrap()
                        .borrow_mut()
                        .parent = Some(Box::new(ast.to_ast()));
                    self.visit(&mut t.to_ast());
                }
            } else {
                assert!(false);
            }
        }
        // ast
    }

    pub fn visitIndexExpr(&mut self, ast: &mut IndexExpr) {
        assert!(
            is_instance(&**ast.arr.as_ref().unwrap(), ASTType::LocationExprBase),
            "Function call return value indexing not yet supported"
        );
        let source_t = ast
            .arr
            .as_ref()
            .unwrap()
            .target()
            .as_ref()
            .unwrap()
            .try_as_expression_ref()
            .unwrap()
            .annotated_type()
            .as_ref()
            .unwrap()
            .type_name
            .clone()
            .unwrap();
        ast.location_expr_base.target = Some(Box::new(
            VariableDeclaration::new(
                vec![],
                *source_t.try_as_mapping_ref().unwrap().value_type.clone(),
                Identifier::Identifier(IdentifierBase::new(String::new())),
                None,
            )
            .to_ast(),
        ));
        // ast
    }
}
