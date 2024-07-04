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
    ASTInstanceOf, ASTType, AnnotatedTypeName, Array, ArrayBaseProperty, Block, Comment,
    ConstructorOrFunctionDefinition, ContractDefinition, EnumDefinition, EnumValue, Expression,
    ExpressionBaseProperty, ForStatement, Identifier, IdentifierBase, IdentifierBaseProperty,
    IdentifierDeclaration, IdentifierDeclarationBaseProperty, IdentifierExpr, IndexExpr, IntoAST,
    LocationExpr, LocationExprBaseMutRef, Mapping, MemberAccessExpr, NamespaceDefinition,
    SimpleStatement, SourceUnit, StateVariableDeclaration, Statement, StatementList,
    StatementListBase, StatementListBaseProperty, StructDefinition, TupleOrLocationExpr, TypeName,
    UserDefinedTypeName, UserDefinedTypeNameBaseMutRef, UserDefinedTypeNameBaseProperty,
    UserDefinedTypeNameBaseRef, VariableDeclaration, VariableDeclarationStatement, AST,
};
use crate::global_defs::{array_length_member, global_defs, global_vars, GlobalDefs, GlobalVars};
use rccell::{RcCell, WeakCell};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::ops::DerefMut;
// from zkay::crate::pointers::pointer_exceptions import UnknownIdentifierException
use crate::visitors::visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef};
use zkay_derive::ASTVisitorBaseRefImpl;
pub fn fill_symbol_table(ast: &ASTFlatten, global_vars: RcCell<GlobalVars>) {
    let mut v = SymbolTableFiller::new(global_vars);
    v.visit(ast);
}

pub fn link_symbol_table(ast: &ASTFlatten, global_vars: RcCell<GlobalVars>) {
    let mut v = SymbolTableLinker::new(global_vars);
    v.visit(ast);
}

pub fn link_identifiers(ast: &ASTFlatten, global_vars: RcCell<GlobalVars>) {
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
            // //println!("===merge_dicts======={:?},{:?},{:?}",key,value.weak_count(),value.strong_count());
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
        .map(|c| c.ast_base_ref().unwrap().borrow().names().clone())
        .collect();
    // //println!("======{:?}=====888====={:?}",ast.get_ast_type(),names);
    let ret = merge_dicts(names);
    for c in children.iter_mut() {
        //declared names are not available within the declaration statements
        c.ast_base_ref().unwrap().borrow_mut().names.clear();
    }
    ret
}

pub fn get_builtin_globals(
    global_vars: RcCell<GlobalVars>,
) -> BTreeMap<String, WeakCell<Identifier>> {
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
                let dd = d.borrow().idf().clone().unwrap().downgrade();
                // println!("=name===={:?}",d.borrow()
                //         .namespace_definition_base
                //         .idf
                //         .as_ref()
                //         .unwrap()
                //         .borrow()
                //         .name());
                (
                    d.borrow().idf().as_ref().unwrap().borrow().name().clone(),
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
                let dd = d.borrow().idf().clone().unwrap().downgrade();

                (
                    d.borrow().idf().as_ref().unwrap().borrow().name().clone(),
                    dd,
                )
            })
            .collect();
        merge_dicts(vec![global_defs, global_vars])
    }

    pub fn visitSourceUnit(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("==visitSourceUnit={:?}====",ast);
        let names: BTreeMap<_, _> = ast
            .to_ast()
            .try_as_source_unit_ref()
            .unwrap()
            .contracts
            .iter()
            .map(|d| {
                // println!(
                //     "=={:?}==",
                //     d.borrow()
                //         .namespace_definition_base
                //         .idf
                //         .as_ref()
                //         .unwrap()
                //         .borrow()
                //         .name()
                //         .clone(),
                // );
                (
                    d.borrow().idf().as_ref().unwrap().borrow().name().clone(),
                    d.borrow().idf().as_ref().unwrap().downgrade(),
                )
            })
            .collect();
        ast.ast_base_ref().unwrap().borrow_mut().names = names;
        let mut vars = self.get_builtin_globals().clone();
        ast.ast_base_ref()
            .unwrap()
            .borrow_mut()
            .names
            .append(&mut vars);

        // println!(
        //     "====ast.visitSourceUnit.names.len()========{:?}",
        //     ast.to_ast().ast_base_ref().unwrap().borrow().names.len()
        // );
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
                // println!(
                //     "=={:?}==",
                //     d.to_ast()
                //         .try_as_identifier_declaration_ref()
                //         .unwrap()
                //         .idf()
                //         .upgrade()
                //         .unwrap()
                //         .borrow()
                //         .name()
                //         .clone()
                // );
                (
                    d.to_ast()
                        .try_as_identifier_declaration_ref()
                        .unwrap()
                        .idf()
                        .unwrap()
                        .borrow()
                        .name()
                        .clone(),
                    d.to_ast()
                        .try_as_identifier_declaration_ref()
                        .unwrap()
                        .idf()
                        .clone()
                        .unwrap()
                        .downgrade(),
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
                !funcs.contains_key(&f.borrow().idf().as_ref().unwrap().borrow().name()),
                "Zkay does not currently support method overloading.{:?}",
                f
            );
            // println!(
            //     "==function_definitions====name====={:?}=====",
            //     f.borrow()
            //         .namespace_definition_base
            //         .idf
            //         .as_ref()
            //         .unwrap()
            //         .borrow()
            //         .name()
            //         .clone()
            // );
            funcs.insert(
                f.borrow().idf().as_ref().unwrap().borrow().name().clone(),
                f.borrow().idf().as_ref().unwrap().downgrade(),
            );
        }
        let structs = ast
            .try_as_contract_definition_ref()
            .unwrap()
            .borrow()
            .struct_definitions
            .iter()
            .map(|d| {
                // println!(
                //     "=={:?}==",
                //     d.borrow()
                //         .namespace_definition_base
                //         .idf
                //         .as_ref()
                //         .unwrap()
                //         .borrow()
                //         .name()
                //         .clone()
                // );
                (
                    d.borrow().idf().as_ref().unwrap().borrow().name().clone(),
                    d.borrow().idf().as_ref().unwrap().downgrade(),
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
                // println!(
                //     "=={:?}==",
                //     d.borrow()
                //         .namespace_definition_base
                //         .idf
                //         .as_ref()
                //         .unwrap()
                //         .borrow()
                //         .name()
                //         .clone()
                // );
                (
                    d.borrow().idf().as_ref().unwrap().borrow().name().clone(),
                    d.borrow().idf().as_ref().unwrap().downgrade(),
                )
            })
            .collect();
        ast.ast_base_ref().unwrap().borrow_mut().names =
            merge_dicts(vec![state_vars, funcs, structs, enums]);
        // println!("====visitContractDefinition========{:?}",ast.ast_base_ref().names().len());
        Ok(())
    }

    pub fn visitConstructorOrFunctionDefinition(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let names: BTreeMap<_, _> = ast
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .borrow()
            .parameters
            .iter()
            .map(|d| {
                // println!(
                //     "==parameters===={:?}======{:?}=====",
                //     d.borrow().identifier_declaration_base.idf,
                //     d.borrow()
                //         .identifier_declaration_base
                //         .idf
                //         .as_ref()
                //         .unwrap()
                //         .borrow()
                //         .name()
                //         .clone()
                // );
                (
                    d.borrow().idf().as_ref().unwrap().borrow().name().clone(),
                    d.borrow()
                        .identifier_declaration_base
                        .idf()
                        .clone()
                        .unwrap()
                        .downgrade(),
                )
            })
            .collect();
        ast.ast_base_ref().unwrap().borrow_mut().names = names;
        // println!("====visitConstructorOrFunctionDefinition========{:?}",ast.to_ast().ast_base_ref().unwrap().borrow().names .len());
        Ok(())
    }

    pub fn visitStructDefinition(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        ast.ast_base_ref().unwrap().borrow_mut().names = ast
            .try_as_struct_definition_ref()
            .unwrap()
            .borrow()
            .members
            .iter()
            .filter_map(|d| {
                d.try_as_variable_declaration_ref()
                    .map(|id| {
                        let idf = id.borrow().idf().clone();
                        println!(
                            "=visitStructDefinition===name=={:?}",
                            idf.as_ref().unwrap().borrow().name().clone()
                        );
                        let x = (
                            idf.as_ref().unwrap().borrow().name().clone(),
                            idf.clone().unwrap().downgrade(),
                        );
                        x
                    })
                    .or(d.try_as_constructor_or_function_definition_ref().map(|id| {
                        let idf = id.borrow().idf().clone();

                        let x = (
                            idf.as_ref().unwrap().borrow().name().clone(),
                            idf.clone().unwrap().downgrade(),
                        );
                        x
                    }))
            })
            .collect();
        // println!(
        //     "=={:?}==visitStructDefinition========{:?}",
        //     ast.ast_base_ref().unwrap().borrow().names().len(),
        //     ast.try_as_struct_definition_ref()
        //         .unwrap()
        //         .borrow()
        //         .members
        //         .len()
        // );
        Ok(())
    }
    pub fn visitEnumDefinition(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let names: BTreeMap<_, _> = ast
            .try_as_enum_definition_ref()
            .unwrap()
            .borrow()
            .values
            .iter()
            .map(|d| {
                (
                    d.borrow().idf().as_ref().unwrap().borrow().name().clone(),
                    d.borrow().idf().as_ref().map(|f| f.downgrade()).unwrap(),
                )
            })
            .collect();
        ast.ast_base_ref().unwrap().borrow_mut().names = names;
        // //println!("====visitEnumDefinition========{:?}",ast.ast_base_ref().names().len());
        Ok(())
    }
    pub fn visitEnumValue(&self, _ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(())
    }

    pub fn visitVariableDeclaration(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        println!(
            "====visitVariableDeclaration===*********************==name==={:?}",
            ast.try_as_variable_declaration_ref()
                .unwrap()
                .borrow()
                .idf()
                .as_ref()
                .unwrap()
                .borrow()
                .name()
                .clone()
        );
        let names = BTreeMap::from([(
            ast.try_as_variable_declaration_ref()
                .unwrap()
                .borrow()
                .idf()
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
                .clone()
                .unwrap()
                .downgrade(),
        )]);
        ast.ast_base_ref().unwrap().borrow_mut().names = names;

        Ok(())
    }

    pub fn visitStatementList(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // //println!("====visitStatementList========{:?}", ast);

        ast.ast_base_ref().unwrap().borrow_mut().names = collect_children_names(ast);
        Ok(())
    }

    pub fn visitSimpleStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        ////println!("====visitSimpleStatement========{:?}", ast);
        ast.ast_base_ref().unwrap().borrow_mut().names = collect_children_names(ast);
        Ok(())
    }

    pub fn visitForStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        //    println!("====visitForStatement========{:?}",ast);
        ast.ast_base_ref().unwrap().borrow_mut().names = collect_children_names(ast);

        Ok(())
    }

    pub fn visitMapping(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        ////println!("====visitMapping===1====={:?}", ast);
        ast.ast_base_ref().unwrap().borrow_mut().names = BTreeMap::new();

        if ast
            .to_ast()
            .try_as_type_name_ref()
            .unwrap()
            .try_as_mapping_ref()
            .unwrap()
            .key_label
            .is_some()
            && is_instance(
                ast.to_ast()
                    .try_as_type_name_ref()
                    .unwrap()
                    .try_as_mapping_ref()
                    .unwrap()
                    .key_label
                    .as_ref()
                    .unwrap(),
                ASTType::IdentifierBase,
            )
        {
            ast.ast_base_ref().unwrap().borrow_mut().names = BTreeMap::from([(
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
        //  println!("=======get_attr=============={name:?}");
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

            _ => {
                println!("=======get_attr===other==========={name:?}");
                Err(eyre::eyre!("unreach"))
            }
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
    ) -> eyre::Result<(ASTFlatten, ASTFlatten)> {
        // println!("=_find_next_decl============={name}=========={:?}===", ast.to_string());
        let mut ancestor = ast.ast_base_ref().unwrap().borrow().parent().clone();
        while let Some(_ancestor) = &ancestor {
            // for (k,v) in _ancestor    .clone()
            //     .upgrade()
            //     .unwrap().ast_base_ref().unwrap().borrow().names(){
            // //println!("={:?}==weak==names={:?}==={:?}",v.weak_count(),k.to_string(),v.weak_count());}
            let _ancestor = _ancestor.clone().upgrade().unwrap();
            // println!("==={}==========_ancestor===={:?}=========={:?}=", name,_ancestor.get_ast_type(),_ancestor
            //     .ast_base_ref()
            //     .unwrap()
            //     .borrow()
            //     .names().len());
            if let Some(nameo) = _ancestor.ast_base_ref().unwrap().borrow().names().get(name) {
                // println!("={:?}=={:?}==names==in=={:?}", name, nameo.upgrade(), nameo);
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
                    .is_none()
                    || !is_instance(
                        &decl
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
                            .unwrap(),
                        ASTType::VariableDeclarationStatement,
                    )
                    || !decl
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
                        .unwrap()
                        .is_parent_of(ast)
                {
                    // println!(
                    //     "===_find_next_decl======return ==={:?}===",
                    //     decl.as_ref().map(|d| d.clone().upgrade()).flatten()
                    // );
                    return Ok((
                        ancestor.unwrap().upgrade().unwrap(),
                        decl.unwrap().upgrade().unwrap(),
                    ));
                }
            }
            ancestor = _ancestor.ast_base_ref().unwrap().borrow().parent().clone();
        }
        // raise UnknownIdentifierException(f"Undefined identifier {name}", ast)
        println!("Undefined identifier {name},{:?}", ast.code());
        eyre::bail!("Undefined identifier {name} fail")
    }

    pub fn _find_lca(
        ast1: &ASTFlatten,
        ast2: &ASTFlatten,
        root: &ASTFlatten,
    ) -> eyre::Result<(StatementList, ASTFlatten, ASTFlatten)> {
        assert!(ast1 != ast2);
        // println!(
        //     "=_find_lca=={:?}=={:?}=={:?}====begin==",
        //     ast1.get_ast_type(),
        //     ast2.get_ast_type(),
        //     root.get_ast_type()
        // );
        let (mut ast1c, mut ast2c) = (ast1.clone(), ast2.clone());
        // Gather ast1"s ancestors + immediate child towards ast1 (for each)
        let mut ancs = HashMap::new();
        for _ in 0..100 {
            assert!(ast1c.ast_base_ref().unwrap().borrow().parent().is_some());
            ancs.insert(
                ast1c
                    .ast_base_ref()
                    .unwrap()
                    .borrow()
                    .parent()
                    .clone()
                    .unwrap()
                    .upgrade()
                    .unwrap(),
                ast1c.clone(),
            );
            ast1c = ast1c
                .ast_base_ref()
                .unwrap()
                .borrow()
                .parent()
                .clone()
                .unwrap()
                .upgrade()
                .unwrap()
                .clone();
            if &ast1c == root {
                // println!("======_find_lca======= &ast1 == root ======");
                break;
            }
        }

        // println!("=ast2=={:?}===={:?}====00==", root, ast2c);
        // Find least common ancestor with ast2 + immediate child towards ast2
        for _ in 0..100 {
            // println!("=ast2=={:?}===={:?}======", 1, ast2c.get_ast_type());
            assert!(ast2c.ast_base_ref().unwrap().borrow().parent().is_some());
            let old_ast = ast2c.clone();
            ast2c = ast2c
                .ast_base_ref()
                .unwrap()
                .borrow()
                .parent()
                .unwrap()
                .upgrade()
                .unwrap();
            //println!("=ast2=={}===={:?}======", ancs.len(), ast2.to_string());
            // for k in ancs.keys() {
            //     //println!("=ast2=={}==key=={:?}======", ancs.len(), k.to_string());
            // }
            if let Some(ast2v) = ancs.get(&ast2c) {
                // println!("=ast2=={:?}=1==={:?}======", ast2v.to_string(),ast2c.to_string());
                assert!(is_instances(
                    &ast2c,
                    vec![
                        ASTType::ForStatement,
                        ASTType::StatementListBase,
                        ASTType::Block,
                        ASTType::IndentBlock
                    ],
                ));
                // println!("=ast2====2=={:?}======", ast2c.get_ast_type());
                return Ok((
                    if is_instance(&ast2c, ASTType::ForStatement) {
                        StatementList::StatementList(StatementListBase::new(
                            ast2c
                                .to_ast()
                                .try_as_statement()
                                .unwrap()
                                .try_as_for_statement_ref()
                                .unwrap()
                                .statements(),
                            false,
                        ))
                    } else {
                        ast2c
                            .to_ast()
                            .try_as_statement()
                            .unwrap()
                            .try_as_statement_list_ref()
                            .unwrap()
                            .clone()
                    },
                    ast2v.clone(),
                    old_ast,
                ));
            }
        }
        println!("======_find_lca=======failed======");
        eyre::bail!("_find_lca=======failed=")
    }

    pub fn find_type_declaration(&self, t: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        // println!("===find_type_declaration======={:?}======", t.to_ast()
        //         .try_as_type_name_ref()
        //         .unwrap()
        //         .try_as_user_defined_type_name_ref()
        //         .unwrap()
        //         .user_defined_type_name_base_ref()
        //         .names[0]
        //         .borrow()
        //         .name());
        Self::_find_next_decl(
            t,
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
        .map(|x| x.1)
    }

    pub fn find_identifier_declaration(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        // //println!("=======find_identifier_declaration============{:?}",ast);
        let name = ast
            .ast_base_ref()
            .unwrap()
            .borrow()
            .idf()
            .as_ref()
            .unwrap()
            .borrow()
            .name();
        // println!("=====name=========={name}");
        let mut _ast = ast.clone(); //TODO side effect
        for _ in 0..100 {
            // println!("=====name====***************************======{name}");
            let (anc, decl) = Self::_find_next_decl(&_ast, &name)?;
            // println!(
            //     "===={}===={}====={}=={}=====",
            //     anc.is_some(),
            //     anc.is_some()
            //         && is_instances(
            //             anc.as_ref().unwrap(),
            //             vec![ASTType::ForStatement, ASTType::Block],
            //         ),
            //     decl.is_some(),
            //     decl.is_some() && is_instance(decl.as_ref().unwrap(), ASTType::VariableDeclaration)
            // );
            // println!(
            //     "====anc ,decl===={:?}=={:?}=====",
            //     anc.get_ast_type(),
            //     decl.get_ast_type()
            // );
            if is_instances(&anc, vec![ASTType::ForStatement, ASTType::Block])
                && is_instance(&decl, ASTType::VariableDeclaration)
            {
                // Check if identifier really references this declaration (does not come before declaration)
                let (lca, ref_anchor, decl_anchor) = Self::_find_lca(&_ast, &decl, &anc)?;
                // println!("={:?}===={:?}=={:?}=",lca
                //     .statements().len(),lca
                //     .statements()
                //     .iter()
                //     .position(|x| ref_anchor == x.clone().into())
                //     , lca
                //         .statements()
                //         .iter()
                //         .position(|x| decl_anchor == x.clone().into()));
                if lca
                    .statements()
                    .iter()
                    .position(|x| ref_anchor == x.clone())
                    <= lca
                        .statements()
                        .iter()
                        .position(|x| decl_anchor == x.clone())
                {
                    // println!("=*********===========");
                    _ast = anc.clone();
                    continue;
                }
            }

            // println!( "=======find_identifier_declaration===return ");
            return Ok(decl);
        }
        println!("=======find_identifier_declaration===fail ");
        eyre::bail!("find_identifier_declaration======= ===fail")
    }

    pub fn in_scope_at(target_idf: &RcCell<Identifier>, mut ast: &ASTFlatten) -> bool {
        let mut ancestor = ast.ast_base_ref().unwrap().borrow().parent().clone();
        while let Some(_ancestor) = ancestor {
            if _ancestor
                .clone()
                .upgrade()
                .unwrap()
                .ast_base_ref()
                .unwrap()
                .borrow()
                .names()
                .get(&target_idf.borrow().name())
                .map_or(false, |name| name.upgrade().as_ref().unwrap() == target_idf)
            {
                // found name
                return true;
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
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("====visitIdentifierExpr=======begin========={:?}", ast.get_ast_type());
        let mut ast = ast.clone();

        // println!("====visitIdentifierExpr=======begin========={:?}",  ast
        //     .ast_base_ref()
        //     .unwrap()
        //     .borrow()
        //     .idf()
        //     .as_ref()
        //     .unwrap()
        //     .borrow()
        //     .name());
        let fid = self.find_identifier_declaration(&ast).ok();
        // println!("====visitIdentifierExpr====fid===ast.get_ast_type()===={:?}====={:?}",fid, ast.get_ast_type());
        let ta = fid.map(|d| d.downgrade());
        // println!("====visitIdentifierExpr=======ta========={:?}", ta);

        ast.ast_base_ref().unwrap().borrow_mut().target = ta;
        // println!("====visitIdentifierExpr======end=========={:?}", ast.to_string());

        assert!(ast
            .ast_base_ref()
            .unwrap()
            .borrow()
            .target
            .as_ref()
            .map_or(false, |t| t.clone().upgrade().is_some()));
        Ok(())
    }

    pub fn visitUserDefinedTypeName(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let mut type_def = self.find_type_declaration(ast)?;
        for idf in &ast
            .to_ast()
            .try_as_type_name_ref()
            .unwrap()
            .try_as_user_defined_type_name_ref()
            .unwrap()
            .user_defined_type_name_base_ref()
            .names[1..]
        {
            let _idf = type_def
                .to_ast()
                .try_as_namespace_definition_ref()
                .unwrap()
                .names()
                .get(&idf.borrow().name())
                .expect("idf not found")
                .clone();

            type_def = _idf
                .clone()
                .upgrade()
                .unwrap()
                .borrow()
                .ast_base_ref()
                .borrow()
                .parent()
                .as_ref()
                .and_then(|p| p.clone().upgrade())
                .clone()
                .expect("parent is none")
                .clone();
        }
        // println!(
        //     "===visitUserDefinedTypeName========================{:?},======{:?}",
        //     ast.get_ast_type(),
        //     type_def.get_ast_type()
        // );
        ast.ast_base_ref().unwrap().borrow_mut().target = Some(type_def.downgrade());

        // ast
        Ok(())
    }

    pub fn visitMemberAccessExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!(
        //     "===visitMemberAccessExpr======begin======={:?}",
        //     ast
        // );
        assert!(
            is_instance(
                ast.to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .try_as_member_access_expr_ref()
                    .unwrap()
                    .expr
                    .as_ref()
                    .unwrap(),
                ASTType::LocationExprBase
            ),
            "Function call return value member access not yet supported"
        );
        if ast
            .to_ast()
            .try_as_expression_ref()
            .unwrap()
            .try_as_tuple_or_location_expr_ref()
            .unwrap()
            .try_as_location_expr_ref()
            .unwrap()
            .try_as_member_access_expr_ref()
            .unwrap()
            .expr
            .as_ref()
            .unwrap()
            .borrow()
            .ast_base_ref()
            .borrow()
            .target
            .is_some()
            && is_instance(
                &ast.to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .try_as_member_access_expr_ref()
                    .unwrap()
                    .expr
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .ast_base_ref()
                    .borrow()
                    .target
                    .clone()
                    .unwrap()
                    .upgrade()
                    .unwrap(),
                ASTType::NamespaceDefinitionBase,
            )
        {
            let idf = ast
                .to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .try_as_member_access_expr_ref()
                .unwrap()
                .expr
                .as_ref()
                .unwrap()
                .borrow()
                .ast_base_ref()
                .borrow()
                .target
                .clone()
                .unwrap()
                .upgrade()
                .unwrap()
                .to_ast()
                .try_as_namespace_definition_ref()
                .unwrap()
                .names()
                .get(
                    &ast.to_ast()
                        .try_as_expression_ref()
                        .unwrap()
                        .try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .try_as_location_expr_ref()
                        .unwrap()
                        .try_as_member_access_expr_ref()
                        .unwrap()
                        .member
                        .borrow()
                        .name(),
                )
                .expect("name is not in names")
                .clone();
            ast.ast_base_ref().unwrap().borrow_mut().target =
                idf.upgrade().unwrap().borrow().parent();

            // println!("===NamespaceDefinitionBase=1==typ={:?}", ast.get_ast_type());
            //   println!(
            //             "===visitMemberAccessExpr======end======={:?}",
            //             ast
            //         );
            return Ok(());
        }

        let mut t = ast
            .to_ast()
            .try_as_expression_ref()
            .unwrap()
            .try_as_tuple_or_location_expr_ref()
            .unwrap()
            .try_as_location_expr_ref()
            .unwrap()
            .try_as_member_access_expr_ref()
            .unwrap()
            .expr
            .as_ref()
            .unwrap()
            .borrow()
            .ast_base_ref()
            .borrow()
            .target
            .clone()
            .unwrap()
            .upgrade()
            .unwrap()
            .ast_base_ref()
            .unwrap()
            .borrow()
            .annotated_type
            .as_ref()
            .unwrap()
            .borrow()
            .type_name
            .clone();
        if t.clone()
            .map_or(false, |tn| is_instance(&tn, ASTType::ArrayBase))
        {
            assert!(
                &ast.to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .try_as_member_access_expr_ref()
                    .unwrap()
                    .member
                    .borrow()
                    .name()
                    == "length"
            );
            let ta = array_length_member();
            ast.ast_base_ref().unwrap().borrow_mut().target = Some(ta.clone().downgrade());
            if ast.is_expression() {
                ast.try_as_expression_ref()
                    .unwrap()
                    .borrow_mut()
                    .try_as_tuple_or_location_expr_mut()
                    .unwrap()
                    .try_as_location_expr_mut()
                    .unwrap()
                    .location_expr_base_mut_ref()
                    .target_rc = Some(ta.clone());
            } else if ast.is_location_expr() {
                ast.try_as_location_expr_ref()
                    .unwrap()
                    .borrow_mut()
                    .location_expr_base_mut_ref()
                    .target_rc = Some(ta.clone());
            } else {
                eyre::bail!("===arr===ta==else target={:?}==", ast.get_ast_type());
            }
            //      println!(
            //     "===visitMemberAccessExpr======end===arr===={:?}",
            //     ast
            // );
            return Ok(());
        }
        assert!(
            t.clone()
                .map_or(false, |t| is_instance(&t, ASTType::UserDefinedTypeNameBase)),
            "is not UserDefinedTypeNameBase"
        );

        // assert!(isinstance(t, UserDefinedTypeName));

        if t.clone()
            .unwrap()
            .borrow()
            .ast_base_ref()
            .unwrap()
            .borrow()
            .target
            .is_none()
        {
            t.as_ref()
                .unwrap()
                .borrow()
                .ast_base_ref()
                .unwrap()
                .borrow_mut()
                .parent = Some(ast.clone().downgrade());

            self.visit(&t.clone().unwrap().into());

            println!("==========target is   none===========");
            //   println!(
            //             "===visitMemberAccessExpr======end======={:?}",
            //             ast
            //         );
            //             return Ok(());
        }

        if t.clone()
            .unwrap()
            .borrow()
            .ast_base_ref()
            .unwrap()
            .borrow()
            .target
            .is_some()
        {
            let idf = t
                .clone()
                .unwrap()
                .borrow()
                .ast_base_ref()
                .unwrap()
                .borrow()
                .target
                .clone()
                .unwrap()
                .upgrade()
                .unwrap()
                .ast_base_ref()
                .unwrap()
                .borrow()
                .names()
                .get(
                    &ast.to_ast()
                        .try_as_expression_ref()
                        .unwrap()
                        .try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .try_as_location_expr_ref()
                        .unwrap()
                        .try_as_member_access_expr_ref()
                        .unwrap()
                        .member
                        .borrow()
                        .name(),
                )
                .expect("name not in names")
                .clone();

            ast.ast_base_ref().unwrap().borrow_mut().target =
                idf.upgrade().unwrap().borrow().parent();

            // println!(
            //     "===target is not  none==end ==={:?}==",
            //     idf.upgrade().unwrap().borrow().parent()
            // );
            //   println!(
            //             "===visitMemberAccessExpr======end======={:?}",
            //             ast
            //         );
            // ast
        }
        Ok(())
    }

    pub fn visitIndexExpr(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        // //println!("===visitIndexExpr=============={:?}", ast);
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
            .ast_base_ref()
            .borrow()
            .target
            .clone()
            .unwrap()
            .upgrade()
            .unwrap()
            .ast_base_ref()
            .unwrap()
            .borrow()
            .annotated_type()
            .as_ref()
            .unwrap()
            .borrow()
            .type_name
            .clone()
            .unwrap();
        // println!("====source_t=================={:?}",source_t);
        let value_type = if source_t.borrow().is_mapping() {
            source_t
                .borrow()
                .try_as_mapping_ref()
                .unwrap()
                .value_type
                .clone()
        } else if source_t.borrow().is_array() {
            source_t
                .borrow()
                .try_as_array_ref()
                .unwrap()
                .value_type()
                .clone()
        } else {
            panic!("===source_t=========={source_t:?}");
        };
        let ta: ASTFlatten = RcCell::new(VariableDeclaration::new(
            vec![],
            Some(value_type),
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
            expr.borrow_mut().ast_base_ref().borrow_mut().target = Some(ta.clone().downgrade());
            expr
        });
        ast.try_as_location_expr_ref().map(|expr| {
            expr.borrow_mut().location_expr_base_mut_ref().target_rc = Some(ta.clone());
            expr.borrow_mut().ast_base_ref().borrow_mut().target = Some(ta.downgrade());
            expr
        });

        // ast
        Ok(())
    }
}
