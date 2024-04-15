#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

// from typing import Tuple, Dict, Union
use crate::ast::{
    is_instance, is_instances, ASTBaseMutRef, ASTBaseProperty, ASTBaseRef, ASTChildren, ASTFlatten,
    ASTInstanceOf, ASTType, AnnotatedTypeName, Array, Block, Comment,
    ConstructorOrFunctionDefinition, ContractDefinition, EnumDefinition, EnumValue, Expression,
    ExpressionBaseProperty, ForStatement, Identifier, IdentifierBase, IdentifierBaseProperty,
    IdentifierDeclaration, IdentifierDeclarationBaseProperty, IdentifierExpr, IndexExpr, IntoAST,
    LocationExpr, LocationExprBaseProperty, Mapping, MemberAccessExpr, NamespaceDefinition,
    SimpleStatement, SourceUnit, Statement, StatementList, StatementListBaseProperty,
    StructDefinition, TupleOrLocationExpr, TypeName, UserDefinedTypeName,
    UserDefinedTypeNameBaseMutRef, UserDefinedTypeNameBaseProperty, UserDefinedTypeNameBaseRef,
    VariableDeclaration, VariableDeclarationStatement, AST,
};
use crate::global_defs::{array_length_member, global_defs, global_vars};
use rccell::{RcCell, WeakCell};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::ops::DerefMut;
// from zkay::crate::pointers::pointer_exceptions import UnknownIdentifierException
use crate::visitor::visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef};
use zkay_derive::ASTVisitorBaseRefImpl;
pub fn fill_symbol_table(ast: &ASTFlatten) {
    let mut v = SymbolTableFiller::new();
    v.visit(ast);
}

pub fn link_symbol_table(ast: &ASTFlatten) {
    let mut v = SymbolTableLinker::new();
    v.visit(ast);
}

pub fn link_identifiers(ast: &ASTFlatten) {
    fill_symbol_table(ast);
    link_symbol_table(ast);
}

pub fn merge_dicts(
    dict_args: Vec<BTreeMap<String, WeakCell<Identifier>>>,
) -> BTreeMap<String, WeakCell<Identifier>>
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

pub fn collect_children_names(ast: &ASTFlatten) -> BTreeMap<String, WeakCell<Identifier>> {
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
        .map(|mut c| c.ast_base_ref().unwrap().borrow().names().clone())
        .collect();
    // println!("======{:?}=====888====={:?}",ast.get_ast_type(),names);
    let ret = merge_dicts(names);
    for c in children.iter_mut()
    //declared names are not available within the declaration statements
    {
        c.ast_base_ref().unwrap().borrow_mut().names.clear();
    }
    ret
}

pub fn get_builtin_globals() -> BTreeMap<String, WeakCell<Identifier>> {
    let mut sf = SymbolTableFiller::new();
    sf.get_builtin_globals()
}

#[derive(ASTVisitorBaseRefImpl)]
struct SymbolTableFiller {
    pub ast_visitor_base: AstVisitorBase,
}

impl AstVisitor for SymbolTableFiller {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}

    fn has_attr(&self, name: &ASTType) -> bool {
        matches!(
            name,
            ASTType::SourceUnit
                | ASTType::ContractDefinition
                | ASTType::ConstructorOrFunctionDefinition
                | ASTType::StructDefinition
                | ASTType::EnumDefinition
                | ASTType::VariableDeclaration
                | ASTType::StatementListBase
                | ASTType::Block
                | ASTType::IndentBlock
                | ASTType::SimpleStatementBase
                | ASTType::ExpressionStatement
                | ASTType::RequireStatement
                | ASTType::AssignmentStatementBase
                | ASTType::CircuitInputStatement
                | ASTType::VariableDeclarationStatement
                | ASTType::ForStatement
                | ASTType::Mapping
        )
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> Self::Return {
        match name {
            ASTType::SourceUnit => self.visitSourceUnit(ast),
            ASTType::ContractDefinition => self.visitContractDefinition(ast),
            ASTType::ConstructorOrFunctionDefinition => {
                self.visitConstructorOrFunctionDefinition(ast)
            }
            ASTType::StructDefinition => self.visitStructDefinition(ast),
            ASTType::EnumDefinition => self.visitEnumDefinition(ast),
            ASTType::EnumValue => self.visitEnumValue(ast),
            ASTType::VariableDeclaration => self.visitVariableDeclaration(ast),
            ASTType::StatementListBase | ASTType::Block | ASTType::IndentBlock => {
                self.visitStatementList(ast)
            }
            ASTType::SimpleStatementBase
            | ASTType::ExpressionStatement
            | ASTType::RequireStatement
            | ASTType::AssignmentStatementBase
            | ASTType::CircuitInputStatement
            | ASTType::VariableDeclarationStatement => self.visitSimpleStatement(ast),
            ASTType::ForStatement => self.visitForStatement(ast),
            ASTType::Mapping => self.visitMapping(ast),
            _ => {}
        }
    }
}
// class SymbolTableFiller(AstVisitor)
impl SymbolTableFiller {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("post", false),
        }
    }
    pub fn get_builtin_globals(&self) -> BTreeMap<String, WeakCell<Identifier>> {
        let mut global_defs = global_defs().vars();
        for d in &global_defs {
            self.visit(&d.clone().into());
        }
        let global_defs = global_defs
            .iter()
            .map(|d| {
                (
                    d.borrow()
                        .namespace_definition_base
                        .idf
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .name()
                        .clone(),
                    d.borrow()
                        .namespace_definition_base
                        .idf
                        .as_ref()
                        .unwrap()
                        .downgrade(),
                )
            })
            .collect();
        let global_vars = global_vars()
            .vars()
            .into_iter()
            .map(|d| {
                (
                    d.borrow()
                        .identifier_declaration_base
                        .idf
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .name()
                        .clone(),
                    d.borrow().identifier_declaration_base.idf(),
                )
            })
            .collect();
        merge_dicts(vec![global_defs, global_vars])
    }

    pub fn visitSourceUnit(&self, ast: &ASTFlatten) {
        ast.try_as_source_unit_ref()
            .unwrap()
            .borrow_mut()
            .ast_base
            .borrow_mut()
            .names = ast
            .try_as_source_unit_ref()
            .unwrap()
            .borrow()
            .contracts
            .iter()
            .map(|d| {
                (
                    d.borrow()
                        .namespace_definition_base
                        .idf
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .name()
                        .clone(),
                    d.borrow()
                        .namespace_definition_base
                        .idf
                        .as_ref()
                        .unwrap()
                        .downgrade(),
                )
            })
            .collect();
        // println!("==ast
        //     .contracts==s=={:?}====",s.len());
        let mut vars = self.get_builtin_globals().clone();
        ast.try_as_source_unit_ref()
            .unwrap()
            .borrow_mut()
            .ast_base
            .borrow_mut()
            .names
            .append(&mut vars);
        // println!("====ast.visitSourceUnit.names.len()========{:?}",ast.ast_base.names().len());
    }

    pub fn visitContractDefinition(&self, ast: &ASTFlatten) {
        let state_vars = ast
            .try_as_contract_definition_ref()
            .unwrap()
            .borrow()
            .state_variable_declarations
            .iter()
            .filter_map(|d| {
                if is_instance(d, ASTType::CommentBase) {
                    None
                } else {
                    Some((
                        d.borrow()
                            .try_as_identifier_declaration_ref()
                            .unwrap()
                            .idf()
                            .upgrade()
                            .unwrap()
                            .borrow()
                            .name()
                            .clone(),
                        d.borrow()
                            .try_as_identifier_declaration_ref()
                            .unwrap()
                            .idf()
                            .clone(),
                    ))
                }
            })
            .collect();
        let mut funcs = BTreeMap::new();
        for f in &ast
            .try_as_contract_definition_ref()
            .unwrap()
            .borrow()
            .function_definitions
        {
            // raise UnknownIdentifierException(f"Zkay does not currently support method overloading.", f)
            assert!(
                !funcs.contains_key(
                    &f.borrow()
                        .namespace_definition_base
                        .idf
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .name()
                ),
                "Zkay does not currently support method overloading.{:?}",
                f
            );
            //    println!("==function_definitions=={:?}=====",f.namespace_definition_base.idf.name().clone());
            funcs.insert(
                f.borrow()
                    .namespace_definition_base
                    .idf
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .name()
                    .clone(),
                f.borrow()
                    .namespace_definition_base
                    .idf
                    .as_ref()
                    .unwrap()
                    .downgrade(),
            );
        }
        let structs = ast
            .try_as_contract_definition_ref()
            .unwrap()
            .borrow()
            .struct_definitions
            .iter()
            .map(|d| {
                (
                    d.borrow()
                        .namespace_definition_base
                        .idf
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .name()
                        .clone(),
                    d.borrow()
                        .namespace_definition_base
                        .idf
                        .as_ref()
                        .unwrap()
                        .downgrade(),
                )
            })
            .collect();
        let enums = ast
            .try_as_contract_definition_ref()
            .unwrap()
            .borrow()
            .enum_definitions
            .iter()
            .map(|d| {
                (
                    d.borrow()
                        .namespace_definition_base
                        .idf
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .name()
                        .clone(),
                    d.borrow()
                        .namespace_definition_base
                        .idf
                        .as_ref()
                        .unwrap()
                        .downgrade(),
                )
            })
            .collect();
        ast.try_as_contract_definition_ref()
            .unwrap()
            .borrow_mut()
            .namespace_definition_base
            .ast_base
            .borrow_mut()
            .names = merge_dicts(vec![state_vars, funcs, structs, enums]);
        // println!("====visitContractDefinition========{:?}",ast.ast_base_ref().names().len());
    }

    pub fn visitConstructorOrFunctionDefinition(&self, ast: &ASTFlatten) {
        ast.try_as_constructor_or_function_definition_ref()
            .unwrap()
            .borrow_mut()
            .namespace_definition_base
            .ast_base
            .borrow_mut()
            .names = ast
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .borrow()
            .parameters
            .iter()
            .map(|d| {
                //    println!("==parameters===={:?}======{:?}=====",d.identifier_declaration_base.idf.to_string(),d.identifier_declaration_base.idf.name().clone());
                (
                    d.borrow()
                        .identifier_declaration_base
                        .idf
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .name()
                        .clone(),
                    d.borrow().identifier_declaration_base.idf().clone(),
                )
            })
            .collect();
        // println!("====visitConstructorOrFunctionDefinition========{:?}",ast.ast_base_ref().names().len());
    }

    pub fn visitStructDefinition(&self, ast: &ASTFlatten) {
        ast.try_as_struct_definition_ref()
            .unwrap()
            .borrow_mut()
            .namespace_definition_base
            .ast_base
            .borrow_mut()
            .names = ast
            .try_as_struct_definition_ref()
            .unwrap()
            .borrow()
            .members
            .iter()
            .filter_map(|d| {
                if let Some(id) = d.borrow().try_as_identifier_declaration_ref() {
                    let idf = id.idf();
                    Some((idf.upgrade().unwrap().borrow().name().clone(), idf.clone()))
                } else {
                    None
                }
            })
            .collect();
        // println!("====visitStructDefinition========{:?}",ast.ast_base_ref().names().len());
    }
    pub fn visitEnumDefinition(&self, ast: &ASTFlatten) {
        ast.try_as_enum_definition_ref()
            .unwrap()
            .borrow_mut()
            .namespace_definition_base
            .ast_base
            .borrow_mut()
            .names = ast
            .try_as_enum_definition_ref()
            .unwrap()
            .borrow()
            .values
            .iter()
            .map(|d| {
                (
                    d.borrow().idf.as_ref().unwrap().borrow().name().clone(),
                    d.borrow().idf.as_ref().map(|f| f.downgrade()).unwrap(),
                )
            })
            .collect();
        // println!("====visitEnumDefinition========{:?}",ast.ast_base_ref().names().len());
    }
    pub fn visitEnumValue(&self, _ast: &ASTFlatten) {}

    pub fn visitVariableDeclaration(&self, ast: &ASTFlatten) {
        ast.try_as_variable_declaration_ref()
            .unwrap()
            .borrow_mut()
            .identifier_declaration_base
            .ast_base
            .borrow_mut()
            .names = BTreeMap::from([(
            ast.try_as_variable_declaration_ref()
                .unwrap()
                .borrow()
                .identifier_declaration_base
                .idf
                .as_ref()
                .unwrap()
                .borrow()
                .name()
                .clone(),
            ast.try_as_variable_declaration_ref()
                .unwrap()
                .borrow()
                .identifier_declaration_base
                .idf()
                .clone(),
        )]);
        // println!("=={:?}==visitVariableDeclaration========{:?}",ast.identifier_declaration_base.idf.name(),ast.ast_base_ref().names().len());
    }

    pub fn visitStatementList(&self, ast: &ASTFlatten) {
        ast.try_as_statement_list_ref()
            .unwrap()
            .borrow_mut()
            .ast_base_mut_ref()
            .borrow_mut()
            .names = collect_children_names(ast);
        // println!("=={:?}==visitStatementList========{:?}",ast.get_ast_type(),ast.ast_base_ref().names().len());
    }

    pub fn visitSimpleStatement(&self, ast: &ASTFlatten) {
        ast.try_as_simple_statement_ref()
            .unwrap()
            .borrow_mut()
            .ast_base_mut_ref()
            .borrow_mut()
            .names = collect_children_names(ast);
        // println!("=={:?}==visitSimpleStatement========{:?}",ast.get_ast_type(),ast.ast_base_ref().names().len());
    }

    pub fn visitForStatement(&self, ast: &ASTFlatten) {
        ast.try_as_for_statement_ref()
            .unwrap()
            .borrow_mut()
            .ast_base_mut_ref()
            .borrow_mut()
            .names = collect_children_names(ast);
        // println!("====visitForStatement========{:?}",ast.ast_base_ref().names().len());
    }

    pub fn visitMapping(&self, ast: &ASTFlatten) {
        ast.try_as_mapping_ref()
            .unwrap()
            .borrow_mut()
            .ast_base_ref()
            .borrow_mut()
            .names = BTreeMap::new();
        // println!("====visitMapping===1====={:?}",ast.ast_base_ref().names().len());
        if is_instance(
            ast.try_as_mapping_ref()
                .unwrap()
                .borrow()
                .key_label
                .as_ref()
                .unwrap(),
            ASTType::IdentifierBase,
        ) {
            ast.try_as_mapping_ref()
                .unwrap()
                .borrow_mut()
                .ast_base_ref()
                .borrow_mut()
                .names = BTreeMap::from([(
                ast.try_as_mapping_ref()
                    .unwrap()
                    .borrow()
                    .key_label
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .name()
                    .clone(),
                ast.try_as_mapping_ref()
                    .unwrap()
                    .borrow()
                    .key_label
                    .as_ref()
                    .map(|kl| kl.downgrade())
                    .unwrap(),
            )]);
        }
        // println!("====visitMapping========{:?}",ast.ast_base_ref().names().len());
    }
}

#[derive(ASTVisitorBaseRefImpl)]
pub struct SymbolTableLinker {
    pub ast_visitor_base: AstVisitorBase,
}
// class SymbolTableLinker(AstVisitor)
impl AstVisitor for SymbolTableLinker {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}
    fn has_attr(&self, name: &ASTType) -> bool {
        matches!(
            name,
            ASTType::IdentifierExpr
                | ASTType::UserDefinedTypeNameBase
                | ASTType::MemberAccessExpr
                | ASTType::IndexExpr
        )
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> Self::Return {
        match name {
            ASTType::IdentifierExpr => self.visitIdentifierExpr(ast),
            ASTType::UserDefinedTypeNameBase => self.visitUserDefinedTypeName(ast),
            ASTType::MemberAccessExpr => self.visitMemberAccessExpr(ast),
            ASTType::IndexExpr => self.visitIndexExpr(ast),

            _ => {}
        }
    }
}
impl SymbolTableLinker {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("post", false),
        }
    }
    pub fn _find_next_decl(
        ast: &ASTFlatten,
        name: &String,
    ) -> (Option<ASTFlatten>, Option<ASTFlatten>) {
        println!("=============={name}=========={:?}===", ast.to_string());
        let mut ancestor = ast.ast_base_ref().unwrap().borrow().parent().clone();
        while let Some(_ancestor) = &ancestor {
            // for (k,v) in _ancestor.ast_base_ref().unwrap().names(){
            // println!("={:?}====names={:?}==={:?}",_ancestor.to_string(),k.to_string(),v.to_string());}
            if let Some(nameo) = _ancestor
                .clone()
                .upgrade()
                .unwrap()
                .ast_base_ref()
                .unwrap()
                .borrow()
                .names()
                .get(name)
            {
                // println!("={:?}====names==in=={:?}", name, nameo.to_string());
                let decl = nameo.upgrade().unwrap().borrow().parent();
                let decl_parent = decl
                    .clone()
                    .unwrap()
                    .upgrade()
                    .unwrap()
                    .ast_base_ref()
                    .unwrap()
                    .borrow()
                    .parent()
                    .clone()
                    .unwrap()
                    .upgrade()
                    .unwrap();
                println!(
                    "={:?}====insta==in=={:?},========{:?}",
                    is_instance(&decl_parent, ASTType::VariableDeclarationStatement,),
                    decl_parent.is_parent_of(ast),
                    !is_instance(&decl_parent, ASTType::VariableDeclarationStatement,)
                        || !decl_parent.is_parent_of(ast)
                );

                if !is_instance(&decl_parent, ASTType::VariableDeclarationStatement)
                    || !decl_parent.is_parent_of(ast)
                {
                    println!("=========return ======");
                    return (
                        ancestor.map(|a| a.upgrade()).flatten(),
                        decl.as_ref().map(|d| d.clone().upgrade()).flatten(),
                    );
                }
            }
            ancestor = _ancestor
                .clone()
                .upgrade()
                .unwrap()
                .ast_base_ref()
                .unwrap()
                .borrow()
                .parent()
                .clone();
        }
        // raise UnknownIdentifierException(f"Undefined identifier {name}", ast)
        println!("Undefined identifier {name},{:?}", ast.code());
        (None, None)
    }

    pub fn _find_lca(
        ast1: &ASTFlatten,
        ast2: &ASTFlatten,
        root: &ASTFlatten,
    ) -> (StatementList, ASTFlatten, ASTFlatten) {
        assert!(ast1 != ast2);
        let (mut ast1, mut ast2) = (ast1.clone(), ast2.clone());
        // Gather ast1"s ancestors + immediate child towards ast1 (for each)
        let mut ancs = BTreeMap::new();
        for _ in 0..100 {
            assert!(ast1.ast_base_ref().unwrap().borrow().parent().is_some());
            ancs.insert(
                ast1.ast_base_ref()
                    .unwrap()
                    .borrow()
                    .parent()
                    .clone()
                    .unwrap()
                    .upgrade()
                    .unwrap()
                    .to_string(),
                ast1.clone(),
            );
            ast1 = ast1
                .ast_base_ref()
                .unwrap()
                .borrow()
                .parent()
                .clone()
                .unwrap()
                .upgrade()
                .unwrap()
                .clone();
            if &ast1 == root {
                println!("======_find_lca======= &ast1 == root ======");
                break;
            }
        }
        println!(
            "======_find_lca======= &ast1 == root ===end=={}=",
            ancs.len()
        );
        // Find least common ancestor with ast2 + immediate child towards ast2
        for _ in 0..100 {
            assert!(ast2.ast_base_ref().unwrap().borrow().parent().is_some());
            let old_ast = ast2.clone();
            ast2 = ast2
                .ast_base_ref()
                .unwrap()
                .borrow()
                .parent()
                .unwrap()
                .upgrade()
                .unwrap();
            println!("=ast2=={}===={:?}======", ancs.len(), ast2.to_string());
            for k in ancs.keys() {
                println!("=ast2=={}==key=={:?}======", ancs.len(), k.to_string());
            }
            if let Some(ast2v) = ancs.get(&ast2.to_string()) {
                println!("=ast2===1==={:?}======", ast2.to_string());
                assert!(is_instances(
                    &ast2,
                    vec![
                        ASTType::ForStatement,
                        ASTType::StatementListBase,
                        ASTType::Block,
                        ASTType::IndentBlock
                    ],
                ));
                println!("=ast2====2=={:?}======", ast2.to_string());
                return (
                    ast2.clone()
                        .try_as_statement()
                        .unwrap()
                        .borrow()
                        .try_as_statement_list_ref()
                        .unwrap()
                        .clone(),
                    ast2v.clone(),
                    old_ast,
                );
            }
        }
        println!("======_find_lca=======failed======");
        (
            ast1.try_as_statement()
                .unwrap()
                .borrow()
                .try_as_statement_list_ref()
                .unwrap()
                .clone(),
            ast2.clone(),
            root.clone(),
        )
    }

    pub fn find_type_declaration(&self, t: &ASTFlatten) -> Option<ASTFlatten> {
        Self::_find_next_decl(
            &t,
            &t.try_as_user_defined_type_name_ref()
                .unwrap()
                .borrow()
                .user_defined_type_name_base_ref()
                .names[0]
                .borrow()
                .name(),
        )
        .1
    }

    pub fn find_identifier_declaration(&self, ast: &mut ASTFlatten) -> Option<ASTFlatten> {
        let name = ast
            .try_as_identifier_expr_ref()
            .unwrap()
            .borrow()
            .idf
            .as_ref()
            .unwrap()
            .borrow()
            .name();
        let mut _ans = ast.clone(); //TODO side effect
        for _ in 0..100 {
            let (anc, decl) = Self::_find_next_decl(&_ans, &name);
            println!(
                "===={}===={}====={}=={}=====",
                anc.is_some(),
                is_instances(
                    anc.as_ref().unwrap(),
                    vec![ASTType::ForStatement, ASTType::Block],
                ),
                decl.is_some(),
                is_instance(decl.as_ref().unwrap(), ASTType::VariableDeclaration)
            );
            println!(
                "====anc ,decl===={:?}=={:?}=====",
                anc.as_ref().unwrap().get_ast_type(),
                decl.as_ref().unwrap().get_ast_type()
            );
            if anc.is_some()
                && is_instances(
                    anc.as_ref().unwrap(),
                    vec![ASTType::ForStatement, ASTType::Block],
                )
                && decl.is_some()
                && is_instance(decl.as_ref().unwrap(), ASTType::VariableDeclaration)
            {
                // Check if identifier really references this declaration (does not come before declaration)

                let (lca, ref_anchor, decl_anchor) =
                    Self::_find_lca(&_ans, decl.as_ref().unwrap(), anc.as_ref().unwrap());
                println!(
                    "==statements().len()====={}=========",
                    lca.statements().len()
                );
                if lca
                    .statements()
                    .iter()
                    .position(|x| ref_anchor == x.clone().into())
                    <= lca
                        .statements()
                        .iter()
                        .position(|x| decl_anchor == x.clone().into())
                {
                    _ans = anc.unwrap().clone();
                    continue;
                }
            }
            // if let (
            //     Some(AST::Statement(Statement::StatementList(StatementList::Block(anc)))),
            //     Some(decl),
            // ) = (
            //     &anc,
            //     &decl
            //         .as_ref()
            //         .unwrap()
            //         .try_as_identifier_declaration_ref()
            //         .unwrap()
            //         .try_as_variable_declaration_ref(),
            // ) {
            //     // Check if identifier really references this declaration (does not come before declaration)
            //     let (lca, ref_anchor, decl_anchor) =
            //         SymbolTableLinker::_find_lca(ast.to_ast(), decl.to_ast(), anc.to_ast());
            //     if lca.statements().iter().find(|x| (*x).clone() == ref_anchor)
            //         <= lca
            //             .statements()
            //             .iter()
            //             .find(|x| (*x).clone() == decl_anchor)
            //     {
            //         _ans = anc.to_ast();
            //         continue;
            //     }
            // }
            *ast = _ans;
            println!("=======find_identifier_declaration========return ======");
            return decl;
        }
        println!("======find_identifier_declaration======= ===fail===");
        None
    }

    pub fn in_scope_at(target_idf: &Identifier, mut ast: &ASTFlatten) -> bool {
        let mut ancestor = ast.ast_base_ref().unwrap().borrow().parent().clone();
        while let Some(_ancestor) = ancestor {
            if let Some(name) = _ancestor
                .clone()
                .upgrade()
                .unwrap()
                .ast_base_ref()
                .unwrap()
                .borrow()
                .names()
                .get(&target_idf.name())
            // found name
            {
                if *name.upgrade().as_ref().unwrap().borrow() == *target_idf {
                    return true;
                }
            }
            ancestor = _ancestor
                .clone()
                .upgrade()
                .unwrap()
                .ast_base_ref()
                .unwrap()
                .borrow()
                .parent()
                .clone();
        }
        false
    }

    pub fn visitIdentifierExpr(&self, mut ast: &ASTFlatten) {
        println!(
            "====visitIdentifierExpr================{:?}",
            (*ast).to_string()
        );
        let mut ast = ast.clone();
        let ta = self
            .find_identifier_declaration(&mut ast)
            .map(|d| d.downgrade());

        ast.try_as_identifier_expr_ref()
            .unwrap()
            .borrow_mut()
            .location_expr_base
            .target = ta;
        assert!(
            ast.try_as_identifier_expr_ref()
                .unwrap()
                .borrow()
                .location_expr_base
                .target
                .as_ref()
                .is_some()
                && ast
                    .try_as_identifier_expr_ref()
                    .unwrap()
                    .borrow()
                    .location_expr_base
                    .target
                    .clone()
                    .unwrap()
                    .upgrade()
                    .is_some()
        );
    }

    pub fn visitUserDefinedTypeName(&self, ast: &ASTFlatten) {
        let mut type_def = self.find_type_declaration(ast);
        for idf in &ast
            .try_as_user_defined_type_name_ref()
            .unwrap()
            .borrow()
            .user_defined_type_name_base_ref()
            .names[1..]
        {
            if let Some(_idf) = type_def
                .clone()
                .unwrap()
                .try_as_namespace_definition_ref()
                .unwrap()
                .borrow()
                .names()
                .get(&idf.borrow().name())
            {
                let parent = idf
                    .borrow()
                    .parent()
                    .as_ref()
                    .map(|p| p.clone().upgrade())
                    .flatten();
                if parent.is_some() {
                    type_def = parent;
                }
            }
        }
        ast.try_as_user_defined_type_name_ref()
            .unwrap()
            .borrow_mut()
            .user_defined_type_name_base_mut_ref()
            .target = type_def.map(|t| t.downgrade());

        // ast
    }
    pub fn visitMemberAccessExpr(&self, ast: &ASTFlatten) {
        assert!(
            is_instance(
                &**ast
                    .try_as_member_access_expr_ref()
                    .unwrap()
                    .borrow()
                    .expr
                    .as_ref()
                    .unwrap(),
                ASTType::LocationExprBase
            ),
            "Function call return value member access not yet supported"
        );
        if is_instance(
            &ast.try_as_member_access_expr_ref()
                .unwrap()
                .borrow()
                .expr
                .as_ref()
                .unwrap()
                .borrow()
                .target()
                .clone()
                .unwrap()
                .upgrade()
                .unwrap(),
            ASTType::NamespaceDefinitionBase,
        ) {
            if let Some(idf) = ast
                .try_as_member_access_expr_ref()
                .unwrap()
                .borrow()
                .expr
                .as_ref()
                .unwrap()
                .borrow()
                .target()
                .clone()
                .unwrap()
                .upgrade()
                .unwrap()
                .try_as_namespace_definition_ref()
                .unwrap()
                .borrow()
                .names()
                .get(
                    &ast.try_as_member_access_expr_ref()
                        .unwrap()
                        .borrow()
                        .member
                        .borrow()
                        .name(),
                )
            {
                ast.try_as_member_access_expr_ref()
                    .unwrap()
                    .borrow_mut()
                    .location_expr_base
                    .target = idf.upgrade().unwrap().borrow().parent();
            }
        } else {
            let mut t = ast
                .try_as_member_access_expr_ref()
                .unwrap()
                .borrow()
                .expr
                .as_ref()
                .unwrap()
                .borrow()
                .target()
                .clone()
                .unwrap()
                .upgrade()
                .unwrap()
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .type_name
                .clone();
            if t.clone()
                .map_or(false, |t| is_instance(&t, ASTType::ArrayBase))
            {
                assert!(
                    &ast.try_as_member_access_expr_ref()
                        .unwrap()
                        .borrow()
                        .member
                        .borrow()
                        .name()
                        == "length"
                );
                let ta = array_length_member();
                ast.try_as_member_access_expr_ref()
                    .unwrap()
                    .borrow_mut()
                    .location_expr_base
                    .target_rc = Some(ta.clone());
                ast.try_as_member_access_expr_ref()
                    .unwrap()
                    .borrow_mut()
                    .location_expr_base
                    .target = Some(ta.downgrade());
            } else if t
                .clone()
                .map_or(false, |t| is_instance(&t, ASTType::UserDefinedTypeNameBase))
            {
                // assert!(isinstance(t, UserDefinedTypeName));
                if let Some(target) = t
                    .clone()
                    .unwrap()
                    .borrow()
                    .try_as_user_defined_type_name_ref()
                    .unwrap()
                    .target()
                {
                    if let Some(idf) = target
                        .clone()
                        .upgrade()
                        .unwrap()
                        .ast_base_ref()
                        .unwrap()
                        .borrow()
                        .names()
                        .get(
                            &ast.try_as_member_access_expr_ref()
                                .unwrap()
                                .borrow()
                                .member
                                .borrow()
                                .name(),
                        )
                    {
                        ast.try_as_member_access_expr_ref()
                            .unwrap()
                            .borrow_mut()
                            .location_expr_base
                            .target = idf.upgrade().unwrap().borrow().parent();
                    }
                } else {
                    t.as_mut()
                        .unwrap()
                        .borrow_mut()
                        .ast_base_mut_ref()
                        .unwrap()
                        .borrow_mut()
                        .parent = Some(ast.clone().downgrade());
                    self.visit(&t.clone().unwrap().into());
                }
            } else {
                assert!(false);
            }
        }
        // ast
    }

    pub fn visitIndexExpr(&self, ast: &ASTFlatten) {
        assert!(
            is_instance(
                &**ast
                    .try_as_index_expr_ref()
                    .unwrap()
                    .borrow()
                    .arr
                    .as_ref()
                    .unwrap(),
                ASTType::LocationExprBase
            ),
            "Function call return value indexing not yet supported"
        );
        let source_t = ast
            .try_as_index_expr_ref()
            .unwrap()
            .borrow()
            .arr
            .as_ref()
            .unwrap()
            .borrow()
            .target()
            .clone()
            .unwrap()
            .upgrade()
            .unwrap()
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .annotated_type()
            .as_ref()
            .unwrap()
            .borrow()
            .type_name
            .clone()
            .unwrap();
        let ta: ASTFlatten = RcCell::new(VariableDeclaration::new(
            vec![],
            source_t
                .borrow()
                .try_as_mapping_ref()
                .unwrap()
                .value_type
                .clone(),
            Identifier::identifier(""),
            None,
        ))
        .into();
        // VARIABLE_DECLARATIONS_CACHE.insert(ta.clone());
        ast.try_as_index_expr_ref()
            .unwrap()
            .borrow_mut()
            .location_expr_base
            .target_rc = Some(ta.clone());
        ast.try_as_index_expr_ref()
            .unwrap()
            .borrow_mut()
            .location_expr_base
            .target = Some(ta.downgrade());
        // ast
    }
}
