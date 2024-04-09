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
        .map(|mut c| c.ast_base_ref().unwrap().names().clone())
        .collect();
    // println!("======{:?}=====888====={:?}",ast.get_ast_type(),names);
    let ret = merge_dicts(names);
    for c in children.iter_mut()
    //declared names are not available within the declaration statements
    {
        c.ast_base_mut_ref().unwrap().names.clear();
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
    pub fn get_builtin_globals(&mut self) -> BTreeMap<String, WeakCell<Identifier>> {
        let mut global_defs = global_defs().vars();
        for d in global_defs.iter_mut() {
            self.visit(&mut (*d).to_ast());
        }
        let global_defs = global_defs
            .iter()
            .map(|d| {
                (
                    d.namespace_definition_base
                        .idf
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .name()
                        .clone(),
                    d.namespace_definition_base
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
                    d.identifier_declaration_base
                        .idf
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .name()
                        .clone(),
                    d.identifier_declaration_base.idf(),
                )
            })
            .collect();
        merge_dicts(vec![global_defs, global_vars])
    }

    pub fn visitSourceUnit(&self, ast: RcCell<SourceUnit>) {
        ast.ast_base.names = ast
            .contracts
            .iter()
            .map(|d| {
                (
                    d.namespace_definition_base
                        .idf
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .name()
                        .clone(),
                    d.namespace_definition_base
                        .idf
                        .as_ref()
                        .unwrap()
                        .downgrade(),
                )
            })
            .collect();
        // println!("==ast
        //     .contracts==s=={:?}====",s.len());
        ast.ast_base.names.append(&mut self.get_builtin_globals());
        // println!("====ast.visitSourceUnit.names.len()========{:?}",ast.ast_base.names().len());
    }

    pub fn visitContractDefinition(&self, ast: RcCell<ContractDefinition>) {
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
                            .upgrade()
                            .unwrap()
                            .borrow()
                            .name()
                            .clone(),
                        d.try_as_identifier_declaration_ref().unwrap().idf().clone(),
                    ))
                }
            })
            .collect();
        let mut funcs = BTreeMap::new();
        for f in &ast.function_definitions {
            // raise UnknownIdentifierException(f"Zkay does not currently support method overloading.", f)
            assert!(
                !funcs.contains_key(
                    &f.namespace_definition_base
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
                f.namespace_definition_base
                    .idf
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .name()
                    .clone(),
                f.namespace_definition_base
                    .idf
                    .as_ref()
                    .unwrap()
                    .downgrade(),
            );
        }
        let structs = ast
            .struct_definitions
            .iter()
            .map(|d| {
                (
                    d.namespace_definition_base
                        .idf
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .name()
                        .clone(),
                    d.namespace_definition_base
                        .idf
                        .as_ref()
                        .unwrap()
                        .downgrade(),
                )
            })
            .collect();
        let enums = ast
            .enum_definitions
            .iter()
            .map(|d| {
                (
                    d.namespace_definition_base
                        .idf
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .name()
                        .clone(),
                    d.namespace_definition_base
                        .idf
                        .as_ref()
                        .unwrap()
                        .downgrade(),
                )
            })
            .collect();
        ast.namespace_definition_base.ast_base.names =
            merge_dicts(vec![state_vars, funcs, structs, enums]);
        // println!("====visitContractDefinition========{:?}",ast.ast_base_ref().names().len());
    }

    pub fn visitConstructorOrFunctionDefinition(
        &self,
        ast: RcCell<ConstructorOrFunctionDefinition>,
    ) {
        ast.namespace_definition_base.ast_base.names = ast
            .parameters
            .iter()
            .map(|d| {
                //    println!("==parameters===={:?}======{:?}=====",d.identifier_declaration_base.idf.to_string(),d.identifier_declaration_base.idf.name().clone());
                (
                    d.identifier_declaration_base
                        .idf
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .name()
                        .clone(),
                    d.identifier_declaration_base.idf().clone(),
                )
            })
            .collect();
        // println!("====visitConstructorOrFunctionDefinition========{:?}",ast.ast_base_ref().names().len());
    }

    pub fn visitStructDefinition(&self, ast: RcCell<StructDefinition>) {
        ast.namespace_definition_base.ast_base.names = ast
            .members
            .iter()
            .filter_map(|d| {
                if let Some(id) = d.try_as_identifier_declaration_ref() {
                    let idf = id.idf();
                    Some((idf.upgrade().unwrap().borrow().name().clone(), idf.clone()))
                } else {
                    None
                }
            })
            .collect();
        // println!("====visitStructDefinition========{:?}",ast.ast_base_ref().names().len());
    }
    pub fn visitEnumDefinition(&self, ast: RcCell<EnumDefinition>) {
        ast.namespace_definition_base.ast_base.names = ast
            .values
            .iter()
            .map(|d| {
                (
                    d.idf.as_ref().unwrap().borrow().name().clone(),
                    d.idf.as_ref().map(|f| f.downgrade()).unwrap(),
                )
            })
            .collect();
        // println!("====visitEnumDefinition========{:?}",ast.ast_base_ref().names().len());
    }
    pub fn visitEnumValue(&self, _ast: RcCell<EnumValue>) {}

    pub fn visitVariableDeclaration(&self, ast: RcCell<VariableDeclaration>) {
        ast.identifier_declaration_base.ast_base.names = BTreeMap::from([(
            ast.identifier_declaration_base
                .idf
                .as_ref()
                .unwrap()
                .borrow()
                .name()
                .clone(),
            ast.identifier_declaration_base.idf().clone(),
        )]);
        // println!("=={:?}==visitVariableDeclaration========{:?}",ast.identifier_declaration_base.idf.name(),ast.ast_base_ref().names().len());
    }

    pub fn visitStatementList(&self, ast: RcCell<StatementList>) {
        ast.ast_base_mut_ref().names = collect_children_names(&mut ast.to_ast());
        // println!("=={:?}==visitStatementList========{:?}",ast.get_ast_type(),ast.ast_base_ref().names().len());
    }

    pub fn visitSimpleStatement(&self, ast: RcCell<SimpleStatement>) {
        ast.ast_base_mut_ref().names = collect_children_names(&mut ast.to_ast());
        // println!("=={:?}==visitSimpleStatement========{:?}",ast.get_ast_type(),ast.ast_base_ref().names().len());
    }

    pub fn visitForStatement(&self, ast: RcCell<ForStatement>) {
        ast.ast_base_mut_ref().names = collect_children_names(ast.clone().into());
        // println!("====visitForStatement========{:?}",ast.ast_base_ref().names().len());
    }

    pub fn visitMapping(&self, ast: RcCell<Mapping>) {
        ast.type_name_base.ast_base.names = BTreeMap::new();
        // println!("====visitMapping===1====={:?}",ast.ast_base_ref().names().len());
        if is_instance(
            &*ast.key_label.as_ref().unwrap().borrow(),
            ASTType::IdentifierBase,
        ) {
            ast.type_name_base.ast_base.names = BTreeMap::from([(
                ast.key_label.as_ref().unwrap().borrow().name().clone(),
                ast.key_label.as_ref().map(|kl| kl.downgrade()).unwrap(),
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
            ast_visitor_base: AstVisitorBase::new("post", false),
        }
    }
    pub fn _find_next_decl(ast: &AST, name: &String) -> (Option<AST>, Option<AST>) {
        println!("=============={name}=========={:?}===", ast.to_string());
        let mut ancestor = ast.ast_base_ref().unwrap().parent().clone();
        while let Some(_ancestor) = ancestor {
            // for (k,v) in _ancestor.ast_base_ref().unwrap().names(){
            // println!("={:?}====names={:?}==={:?}",_ancestor.to_string(),k.to_string(),v.to_string());}
            if let Some(nameo) = _ancestor.ast_base_ref().unwrap().names().get(name) {
                // println!("={:?}====names==in=={:?}", name, nameo.to_string());
                let decl = nameo.upgrade().unwrap().borrow().parent();
                let decl_parent = decl.clone().unwrap().ast_base_ref().unwrap().parent();
                println!(
                    "={:?}====insta==in=={:?},========{:?}",
                    is_instance(
                        &**decl_parent.as_ref().unwrap(),
                        ASTType::VariableDeclarationStatement,
                    ),
                    decl_parent.as_ref().unwrap().is_parent_of(ast),
                    !is_instance(
                        &**decl_parent.as_ref().unwrap(),
                        ASTType::VariableDeclarationStatement,
                    ) || !decl_parent.as_ref().unwrap().is_parent_of(ast)
                );

                if !is_instance(
                    &**decl_parent.as_ref().unwrap(),
                    ASTType::VariableDeclarationStatement,
                ) || !decl_parent.as_ref().unwrap().is_parent_of(ast)
                {
                    println!("=========return ======");
                    return (Some(*_ancestor.clone()), decl.as_ref().map(|d| *d.clone()));
                }
            }
            ancestor = _ancestor.ast_base_ref().unwrap().parent().clone();
        }
        // raise UnknownIdentifierException(f"Undefined identifier {name}", ast)
        println!("Undefined identifier {name},{:?}", ast.code());
        (None, None)
    }

    pub fn _find_lca(
        ast1: &AST,
        ast2: &AST,
        root: &AST,
    ) -> (StatementList, AST, ASTFlatten, AST, ASTFlatten) {
        assert!(ast1 != ast2);
        let (mut ast1, mut ast2) = (ast1.clone(), ast2.clone());
        // Gather ast1"s ancestors + immediate child towards ast1 (for each)
        let mut ancs = BTreeMap::new();
        for _ in 0..100 {
            assert!(ast1.ast_base_ref().unwrap().parent().is_some());
            ancs.insert(
                ast1.ast_base_ref()
                    .unwrap()
                    .parent()
                    .as_ref()
                    .unwrap()
                    .to_string(),
                ast1.clone(),
            );
            ast1 = *ast1
                .ast_base_ref()
                .unwrap()
                .parent()
                .as_ref()
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
            assert!(ast2.ast_base_ref().unwrap().parent().is_some());
            let old_ast = ast2.clone();
            ast2 = *ast2.ast_base_ref().unwrap().parent().unwrap();
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
                        .try_as_statement_list()
                        .unwrap(),
                    ast2v.clone(),
                    old_ast,
                );
            }
        }
        println!("======_find_lca=======failed======");
        (
            ast1.try_as_statement()
                .unwrap()
                .try_as_statement_list()
                .unwrap(),
            ast2.clone(),
            root.clone(),
        )
    }

    pub fn find_type_declaration(&self, t: &UserDefinedTypeName) -> Option<NamespaceDefinition> {
        Self::_find_next_decl(
            &t.to_ast(),
            &t.user_defined_type_name_base_ref().names[0].borrow().name(),
        )
        .1
        .unwrap()
        .try_as_namespace_definition_ref()
        .map(|nd| nd.clone())
    }

    pub fn find_identifier_declaration(&self, ast: &mut IdentifierExpr) -> Option<AST> {
        let name = ast.idf.as_ref().unwrap().borrow().name();
        let mut _ans = ast.to_ast(); //TODO side effect
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
                    .position(|x| (*x).clone() == ref_anchor)
                    <= lca
                        .statements()
                        .iter()
                        .position(|x| (*x).clone() == decl_anchor)
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
            *ast = _ans
                .try_as_expression()
                .unwrap()
                .try_as_tuple_or_location_expr()
                .unwrap()
                .try_as_location_expr()
                .unwrap()
                .try_as_identifier_expr()
                .unwrap();
            println!("=======find_identifier_declaration========return ======");
            return decl;
        }
        println!("======find_identifier_declaration======= ===fail===");
        None
    }

    pub fn in_scope_at(target_idf: &Identifier, ast: AST) -> bool {
        let mut ancestor = ast.ast_base_ref().unwrap().parent().clone();
        while let Some(_ancestor) = ancestor {
            if let Some(name) = _ancestor
                .ast_base_ref()
                .unwrap()
                .names()
                .get(&target_idf.name())
            // found name
            {
                if *name.upgrade().as_ref().unwrap().borrow() == *target_idf {
                    return true;
                }
            }
            ancestor = _ancestor.ast_base_ref().unwrap().parent().clone();
        }
        false
    }

    pub fn visitIdentifierExpr(&self, ast: &mut IdentifierExpr) {
        println!(
            "====visitIdentifierExpr================{:?}",
            (*ast).to_string()
        );
        if let Some(decl) = self.find_identifier_declaration(ast) {
            *ast.location_expr_base
                .target
                .as_mut()
                .unwrap()
                .deref_mut()
                .borrow_mut() = Some(decl.to_ast());
            assert!(
                ast.location_expr_base.target.as_ref().is_some()
                    && ast
                        .location_expr_base
                        .target
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .is_some()
            );
        } else {
            assert!(false, "find_identifier_declaration is none");
        }
    }

    pub fn visitUserDefinedTypeName(&self, ast: &mut UserDefinedTypeName) {
        let mut type_def = self.find_type_declaration(&*ast);
        for idf in &ast.user_defined_type_name_base_ref().names[1..] {
            if let Some(_idf) = type_def.as_ref().unwrap().names().get(&idf.borrow().name()) {
                if let Some(AST::NamespaceDefinition(parent)) =
                    idf.borrow().parent().as_ref().map(|p| *p.clone())
                {
                    type_def = Some(parent.clone());
                }
            }
        }
        *ast.user_defined_type_name_base_mut_ref()
            .target
            .as_mut()
            .unwrap()
            .deref_mut()
            .borrow_mut() = Some(type_def.unwrap().to_ast());

        // ast
    }
    pub fn visitMemberAccessExpr(&self, ast: &mut MemberAccessExpr) {
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
            if let Some(idf) = target.names().get(&ast.member.name()) {
                *ast.location_expr_base
                    .target
                    .as_mut()
                    .unwrap()
                    .deref_mut()
                    .borrow_mut() = idf
                    .upgrade()
                    .unwrap()
                    .borrow()
                    .parent()
                    .as_ref()
                    .map(|f| *f.clone());
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
                assert!(&ast.member.name() == "length");
                *ast.location_expr_base
                    .target
                    .as_mut()
                    .unwrap()
                    .deref_mut()
                    .borrow_mut() = Some(array_length_member().into_ast());
            } else if let TypeName::UserDefinedTypeName(ref mut t) = t {
                // assert!(isinstance(t, UserDefinedTypeName));
                if let Some(target) = t.target() {
                    if let Some(idf) = target
                        .ast_base_ref()
                        .unwrap()
                        .names()
                        .get(&ast.member.name())
                    {
                        *ast.location_expr_base
                            .target
                            .as_mut()
                            .unwrap()
                            .deref_mut()
                            .borrow_mut() =
                            idf.upgrade().unwrap().borrow().parent().map(|p| *p.clone());
                    }
                } else {
                    *t = t.clone();
                    t.ast_base_mut_ref().parent = Some(Box::new(ast.to_ast()));
                    self.visit(&mut t.to_ast());
                }
            } else {
                assert!(false);
            }
        }
        // ast
    }

    pub fn visitIndexExpr(&self, ast: &mut IndexExpr) {
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
        *ast.location_expr_base
            .target
            .as_mut()
            .unwrap()
            .deref_mut()
            .borrow_mut() = Some(
            VariableDeclaration::new(
                vec![],
                *source_t.try_as_mapping_ref().unwrap().value_type.clone(),
                Identifier::Identifier(IdentifierBase::new(String::new())),
                None,
            )
            .to_ast(),
        );
        // ast
    }
}
