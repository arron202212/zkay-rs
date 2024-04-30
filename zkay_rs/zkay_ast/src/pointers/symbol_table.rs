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
    LocationExpr, LocationExprBaseMutRef, LocationExprBaseProperty, Mapping, MemberAccessExpr,
    NamespaceDefinition, SimpleStatement, SourceUnit, StateVariableDeclaration, Statement,
    StatementList, StatementListBaseProperty, StructDefinition, TupleOrLocationExpr, TypeName,
    UserDefinedTypeName, UserDefinedTypeNameBaseMutRef, UserDefinedTypeNameBaseProperty,
    UserDefinedTypeNameBaseRef, VariableDeclaration, VariableDeclarationStatement, AST,
};
use crate::global_defs::{array_length_member, global_defs, global_vars, GlobalDefs, GlobalVars};
use rccell::{RcCell, WeakCell};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::ops::DerefMut;
// from zkay::crate::pointers::pointer_exceptions import UnknownIdentifierException
use crate::visitor::visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef};
use zkay_derive::ASTVisitorBaseRefImpl;
pub fn fill_symbol_table(ast: &ASTFlatten, global_vars: RcCell<GlobalVars>) {
    let mut v = SymbolTableFiller::new(global_vars);
    v.visit(ast);
}

pub fn link_symbol_table(ast: &ASTFlatten, global_vars: RcCell<GlobalVars>) {
    let mut v = SymbolTableLinker::new(global_vars);
    v.visit(ast);
}

pub fn link_identifiers(ast: &ASTFlatten) {
    let global_vars = RcCell::new(global_vars(RcCell::new(global_defs())));
    fill_symbol_table(ast, global_vars.clone());
    link_symbol_table(ast, global_vars);
}
// """
// Given any number of dicts, shallow copy and merge into a new dict.
// Report error on conflicting keys.
// """
pub fn merge_dicts(
    dict_args: Vec<BTreeMap<String, WeakCell<Identifier>>>,
) -> BTreeMap<String, WeakCell<Identifier>> {
    let mut result = BTreeMap::new();
    for dictionary in dict_args {
        for (key, value) in dictionary {
            if let Some(v) = result.get(&key) {
                // raise ValueError("Conflicting definitions for", key)
                assert!(*v == value, "Conflicting definitions for {}", key);
            }
            // println!("===merge_dicts======={:?},{:?},{:?}",key,value.weak_count(),value.strong_count());
            result.insert(key.clone(), value.clone());
        }
    }
    result
}

pub fn collect_children_names(ast: &ASTFlatten) -> BTreeMap<String, WeakCell<Identifier>> {
    let mut children: Vec<_> = ast
        .children()
        .iter()
        .filter(|&c| !is_instances(c, vec![ASTType::Block, ASTType::ForStatement]))
        .cloned()
        .collect();
    let names: Vec<_> = children
        .iter()
        .map(|mut c| c.ast_base_ref().unwrap().borrow_mut().names().clone())
        .collect();
    // println!("======{:?}=====888====={:?}",ast.get_ast_type(),names);
    let ret = merge_dicts(names);
    for c in children.iter_mut() {
        //declared names are not available within the declaration statements
        c.ast_base_ref().unwrap().borrow_mut().names.clear();
    }
    ret
}

pub fn get_builtin_globals() -> BTreeMap<String, WeakCell<Identifier>> {
    let global_vars = RcCell::new(global_vars(RcCell::new(global_defs())));
    let mut sf = SymbolTableFiller::new(global_vars);
    sf.get_builtin_globals()
}

#[derive(ASTVisitorBaseRefImpl)]
struct SymbolTableFiller {
    pub ast_visitor_base: AstVisitorBase,
    global_vars: RcCell<GlobalVars>,
}

impl AstVisitor for SymbolTableFiller {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}

    fn has_attr(&self, ast: &AST) -> bool {
        matches!(
            ast.get_ast_type(),
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
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> eyre::Result<Self::Return> {
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
            _ => Err(eyre::eyre!("unreach")),
        }
    }
}
// class SymbolTableFiller(AstVisitor)
impl SymbolTableFiller {
    pub fn new(global_vars: RcCell<GlobalVars>) -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("post", false),
            global_vars,
        }
    }
    pub fn get_builtin_globals(&self) -> BTreeMap<String, WeakCell<Identifier>> {
        let global_defs = self
            .global_vars
            .borrow()
            .global_defs
            .borrow()
            .vars()
            .clone();
        for d in &global_defs {
            self.visit(&d.clone().into());
        }
        let global_defs = global_defs
            .iter()
            .map(|d| {
                let dd = d
                    .borrow()
                    .namespace_definition_base
                    .idf
                    .clone()
                    .unwrap()
                    .downgrade();
                // println!(
                //     "======{:?}======def========{:?},{:?}",
                //     d.borrow()
                //         .namespace_definition_base
                //         .idf
                //         .as_ref()
                //         .unwrap()
                //         .borrow()
                //         .name()
                //         .clone(),
                //     dd.weak_count(),
                //     dd.strong_count()
                // );
                (
                    d.borrow()
                        .namespace_definition_base
                        .idf
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .name()
                        .clone(),
                    dd,
                )
            })
            .collect();
        let global_vars = self
            .global_vars
            .borrow()
            .vars()
            .iter()
            .map(|d| {
                let dd = d
                    .borrow()
                    .identifier_declaration_base
                    .idf
                    .clone()
                    .unwrap()
                    .downgrade();
                println!(
                    "===={:?}================{:?},{:?}",
                    d.borrow()
                        .identifier_declaration_base
                        .idf
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .name()
                        .clone(),
                    dd.weak_count(),
                    dd.strong_count()
                );
                (
                    d.borrow()
                        .identifier_declaration_base
                        .idf
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .name()
                        .clone(),
                    dd,
                )
            })
            .collect();
        merge_dicts(vec![global_defs, global_vars])
    }

    pub fn visitSourceUnit(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        ast.to_ast().ast_base_ref().unwrap().borrow_mut().names = ast
            .to_ast()
            .try_as_source_unit_ref()
            .unwrap()
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
        ast.to_ast()
            .ast_base_ref()
            .unwrap()
            .borrow_mut()
            .names
            .append(&mut vars);
        // println!("====ast.visitSourceUnit.names.len()========{:?}",ast.ast_base.names().len());
        Ok(())
    }

    pub fn visitContractDefinition(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let state_vars = ast
            .try_as_contract_definition_ref()
            .unwrap()
            .borrow()
            .state_variable_declarations
            .iter()
            .filter(|&d| !is_instance(d, ASTType::CommentBase))
            .map(|d| {
                (
                    d.to_ast()
                        .try_as_identifier_declaration_ref()
                        .unwrap()
                        .idf()
                        .upgrade()
                        .unwrap()
                        .borrow()
                        .name()
                        .clone(),
                    d.to_ast()
                        .try_as_identifier_declaration_ref()
                        .unwrap()
                        .idf()
                        .clone(),
                )
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
        Ok(())
    }

    pub fn visitConstructorOrFunctionDefinition(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        ast.to_ast().ast_base_ref().unwrap().borrow_mut().names = ast
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
        Ok(())
    }

    pub fn visitStructDefinition(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        ast.to_ast().ast_base_ref().unwrap().borrow_mut().names = ast
            .try_as_struct_definition_ref()
            .unwrap()
            .borrow()
            .members
            .iter()
            .filter_map(|d| {
                d.try_as_identifier_declaration_ref().map(|id| {
                    let idf = id.borrow().idf().clone();
                    (idf.upgrade().unwrap().borrow().name().clone(), idf.clone())
                })
            })
            .collect();
        // println!("====visitStructDefinition========{:?}",ast.ast_base_ref().names().len());
        Ok(())
    }
    pub fn visitEnumDefinition(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
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
        Ok(())
    }
    pub fn visitEnumValue(&self, _ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(())
    }

    pub fn visitVariableDeclaration(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("====visitVariableDeclaration========{:?}", ast);
        ast.to_ast().ast_base_ref().unwrap().borrow_mut().names = BTreeMap::from([(
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

        Ok(())
    }

    pub fn visitStatementList(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("====visitStatementList========{:?}", ast);

        ast.to_ast().ast_base_ref().unwrap().borrow_mut().names = collect_children_names(ast);
        Ok(())
    }

    pub fn visitSimpleStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        //println!("====visitSimpleStatement========{:?}", ast);
        ast.to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_simple_statement_ref()
            .unwrap()
            .ast_base_ref()
            .borrow_mut()
            .names = collect_children_names(ast);
        Ok(())
    }

    pub fn visitForStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        ast.try_as_for_statement_ref()
            .unwrap()
            .borrow_mut()
            .ast_base_mut_ref()
            .borrow_mut()
            .names = collect_children_names(ast);
        // println!("====visitForStatement========{:?}",ast.ast_base_ref().names().len());
        Ok(())
    }

    pub fn visitMapping(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        //println!("====visitMapping===1====={:?}", ast);
        ast.to_ast()
            .try_as_type_name_ref()
            .unwrap()
            .try_as_mapping_ref()
            .unwrap()
            .ast_base_ref()
            .borrow_mut()
            .names = BTreeMap::new();

        if is_instance(
            ast.to_ast()
                .try_as_type_name_ref()
                .unwrap()
                .try_as_mapping_ref()
                .unwrap()
                .key_label
                .as_ref()
                .unwrap(),
            ASTType::IdentifierBase,
        ) {
            ast.to_ast()
                .try_as_type_name_ref()
                .unwrap()
                .try_as_mapping_ref()
                .unwrap()
                .ast_base_ref()
                .borrow_mut()
                .names = BTreeMap::from([(
                ast.to_ast()
                    .try_as_type_name_ref()
                    .unwrap()
                    .try_as_mapping_ref()
                    .unwrap()
                    .key_label
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .name()
                    .clone(),
                ast.to_ast()
                    .try_as_type_name_ref()
                    .unwrap()
                    .try_as_mapping_ref()
                    .unwrap()
                    .key_label
                    .as_ref()
                    .map(|kl| kl.downgrade())
                    .unwrap(),
            )]);
        }
        // println!("====visitMapping========{:?}",ast.ast_base_ref().names().len());
        Ok(())
    }
}

#[derive(ASTVisitorBaseRefImpl)]
pub struct SymbolTableLinker {
    pub ast_visitor_base: AstVisitorBase,
    global_vars: RcCell<GlobalVars>,
}
// class SymbolTableLinker(AstVisitor)
impl AstVisitor for SymbolTableLinker {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}
    fn has_attr(&self, ast: &AST) -> bool {
        matches!(
            ast.get_ast_type(),
            ASTType::IdentifierExpr
                | ASTType::UserDefinedTypeNameBase
                | ASTType::MemberAccessExpr
                | ASTType::IndexExpr
        ) || matches!(ast, AST::TypeName(TypeName::UserDefinedTypeName(_)))
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> eyre::Result<Self::Return> {
        match name {
            ASTType::IdentifierExpr => self.visitIdentifierExpr(ast),
            _ if matches!(
                ast.to_ast(),
                AST::TypeName(TypeName::UserDefinedTypeName(_))
            ) =>
            {
                self.visitUserDefinedTypeName(ast)
            }
            ASTType::MemberAccessExpr => self.visitMemberAccessExpr(ast),
            ASTType::IndexExpr => self.visitIndexExpr(ast),

            _ => Err(eyre::eyre!("unreach")),
        }
    }
}
impl SymbolTableLinker {
    pub fn new(global_vars: RcCell<GlobalVars>) -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("post", false),
            global_vars,
        }
    }
    pub fn _find_next_decl(
        ast: &ASTFlatten,
        name: &String,
    ) -> (Option<ASTFlatten>, Option<ASTFlatten>) {
        //println!("=============={name}=========={:?}===", ast.to_string());
        let mut ancestor = ast.ast_base_ref().unwrap().borrow().parent().clone();
        while let Some(_ancestor) = &ancestor {
            // for (k,v) in _ancestor    .clone()
            //     .upgrade()
            //     .unwrap().ast_base_ref().unwrap().borrow().names(){
            // println!("={:?}==weak==names={:?}==={:?}",v.weak_count(),k.to_string(),v.weak_count());}
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
                println!("={:?}=={:?}==names==in=={:?}", name, nameo.upgrade(), nameo);
                let decl = nameo.upgrade().unwrap().borrow().parent();
                if decl
                    .clone()
                    .unwrap()
                    .upgrade()
                    .unwrap()
                    .ast_base_ref()
                    .unwrap()
                    .borrow()
                    .parent()
                    .as_ref()
                    .map_or(true, |decl_parent| {
                        let decl_parent = decl_parent.clone().upgrade().unwrap();
                        !is_instance(&decl_parent, ASTType::VariableDeclarationStatement)
                            || !decl_parent.is_parent_of(ast)
                    })
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
        // println!("===find_type_declaration======={:?}======", t);
        Self::_find_next_decl(
            &t,
            &t.to_ast()
                .try_as_type_name_ref()
                .unwrap()
                .try_as_user_defined_type_name_ref()
                .unwrap()
                .user_defined_type_name_base_ref()
                .names[0]
                .borrow()
                .name(),
        )
        .1
    }

    pub fn find_identifier_declaration(&self, ast: &mut ASTFlatten) -> Option<ASTFlatten> {
        // println!("=======find_identifier_declaration============{:?}",ast);
        let name = ast
            .to_ast()
            .try_as_expression_ref()
            .unwrap()
            .try_as_tuple_or_location_expr_ref()
            .unwrap()
            .try_as_location_expr_ref()
            .unwrap()
            .try_as_identifier_expr_ref()
            .unwrap()
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

    pub fn in_scope_at(target_idf: &RcCell<Identifier>, mut ast: &ASTFlatten) -> bool {
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
                .get(&target_idf.borrow().name())
            // found name
            {
                if name.upgrade().as_ref().unwrap() == target_idf {
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

    pub fn visitIdentifierExpr(
        &self,
        mut ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        println!("====visitIdentifierExpr================{:?}", ast);
        let mut ast = ast.clone();
        let ta = self
            .find_identifier_declaration(&mut ast)
            .map(|d| d.downgrade());
        if ast.is_expression() {
            ast.try_as_expression_ref()
                .unwrap()
                .borrow_mut()
                .try_as_tuple_or_location_expr_mut()
                .unwrap()
                .try_as_location_expr_mut()
                .unwrap()
                .try_as_identifier_expr_mut()
                .unwrap()
                .location_expr_base
                .target = ta;
        } else if ast.is_location_expr() {
            ast.try_as_location_expr_ref()
                .unwrap()
                .borrow_mut()
                .location_expr_base_mut_ref()
                .target = ta;
        }
        assert!(
            ast.to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .try_as_identifier_expr_ref()
                .unwrap()
                .location_expr_base
                .target
                .as_ref()
                .is_some()
                && ast
                    .to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .try_as_identifier_expr_ref()
                    .unwrap()
                    .location_expr_base
                    .target
                    .clone()
                    .unwrap()
                    .upgrade()
                    .is_some()
        );
        Ok(())
    }

    pub fn visitUserDefinedTypeName(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let mut type_def = self.find_type_declaration(ast);
        for idf in &ast
            .to_ast()
            .try_as_type_name_ref()
            .unwrap()
            .try_as_user_defined_type_name_ref()
            .unwrap()
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
        ast.try_as_type_name_ref()
            .unwrap()
            .borrow_mut()
            .try_as_user_defined_type_name_mut()
            .unwrap()
            .user_defined_type_name_base_mut_ref()
            .target = type_def.map(|t| t.downgrade());

        // ast
        Ok(())
    }
    pub fn visitMemberAccessExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        assert!(
            is_instance(
                ast.try_as_member_access_expr_ref()
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
        Ok(())
    }

    pub fn visitIndexExpr(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        println!("===visitIndexExpr=============={:?}", ast);
        assert!(
            is_instance(
                ast.to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .try_as_index_expr_ref()
                    .unwrap()
                    .arr
                    .as_ref()
                    .unwrap(),
                ASTType::LocationExprBase
            ),
            "Function call return value indexing not yet supported"
        );
        println!(
            "=======arr.target============={:?}======",
            ast.to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .try_as_index_expr_ref()
                .unwrap()
                .arr
                .as_ref()
                .unwrap()
                .borrow()
                .target()
                .clone()
                .unwrap()
                .upgrade()
        );
        let source_t = ast
            .to_ast()
            .try_as_expression_ref()
            .unwrap()
            .try_as_tuple_or_location_expr_ref()
            .unwrap()
            .try_as_location_expr_ref()
            .unwrap()
            .try_as_index_expr_ref()
            .unwrap()
            .arr
            .as_ref()
            .unwrap()
            .borrow()
            .target()
            .clone()
            .unwrap()
            .upgrade()
            .unwrap()
            .annotated_type()
            .as_ref()
            .unwrap()
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
        ast.try_as_expression_ref().map(|expr| {
            expr.borrow_mut()
                .try_as_tuple_or_location_expr_mut()
                .unwrap()
                .try_as_location_expr_mut()
                .unwrap()
                .location_expr_base_mut_ref()
                .target_rc = Some(ta.clone());
            expr.borrow_mut()
                .try_as_tuple_or_location_expr_mut()
                .unwrap()
                .try_as_location_expr_mut()
                .unwrap()
                .location_expr_base_mut_ref()
                .target = Some(ta.clone().downgrade());
            expr
        });
        ast.try_as_location_expr_ref().map(|expr| {
            expr.borrow_mut().location_expr_base_mut_ref().target_rc = Some(ta.clone());
            expr.borrow_mut().location_expr_base_mut_ref().target = Some(ta.downgrade());
            expr
        });

        // ast
        Ok(())
    }
}
