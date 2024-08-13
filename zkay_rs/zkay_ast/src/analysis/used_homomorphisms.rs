#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::ast::{
    is_instance, ASTBaseProperty, ASTChildren, ASTFlatten, ASTInstanceOf, ASTType,
    AnnotatedTypeName, ConstructorOrFunctionDefinition, EnumDefinition, Expression,
    ExpressionBaseProperty, IdentifierDeclaration, IntoAST, SourceUnit, StructDefinition, AST,
};
use crate::homomorphism::Homomorphism;
use crate::visitors::visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef};
use rccell::RcCell;
use std::collections::{BTreeMap, BTreeSet};
use zkay_config::config::CFG;
use zkay_crypto::params::CryptoParams;
use zkay_derive::ASTVisitorBaseRefImpl;
// class UsedHomomorphismsVisitor(AstVisitor)
#[derive(ASTVisitorBaseRefImpl)]
pub struct UsedHomomorphismsVisitor {
    pub ast_visitor_base: AstVisitorBase,
}
impl AstVisitor for UsedHomomorphismsVisitor {
    type Return = BTreeSet<String>;
    fn temper_result(&self) -> Self::Return {
        BTreeSet::new()
    }

    fn has_attr(&self, ast: &AST) -> bool {
        matches!(
            ast.get_ast_type(),
            ASTType::AnnotatedTypeName
                | ASTType::ExpressionBase
                | ASTType::IdentifierDeclarationBase
                | ASTType::ConstructorOrFunctionDefinition
                | ASTType::EnumDefinition
                | ASTType::StructDefinition
                | ASTType::SourceUnit
                | ASTType::ASTBase
        ) || matches!(ast, AST::Expression(_))
            || matches!(ast, AST::IdentifierDeclaration(_))
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> eyre::Result<Self::Return> {
        match name {
            ASTType::ASTBase => self.visitAST(ast),
            ASTType::SourceUnit => self.visitSourceUnit(ast),
            ASTType::AnnotatedTypeName => self.visitAnnotatedTypeName(ast),
            ASTType::ConstructorOrFunctionDefinition => {
                self.visitConstructorOrFunctionDefinition(ast)
            }
            ASTType::StructDefinition => self.visitStructDefinition(ast),
            ASTType::EnumDefinition => self.visitEnumDefinition(ast),
            _ if matches!(ast.to_ast(), AST::Expression(_)) => self.visitExpression(ast),
            _ if matches!(ast.to_ast(), AST::IdentifierDeclaration(_)) => {
                self.visitIdentifierDeclaration(ast)
            }
            _ => Ok(BTreeSet::new()),
        }
    }
    fn visit_children(&self, ast: &ASTFlatten) -> eyre::Result<Self::Return> {
        let mut all_homs = BTreeSet::new();
        for c in &ast.children() {
            all_homs = all_homs.union(&self.visit(c)).cloned().collect();
        }
        Ok(all_homs)
    }
}
impl UsedHomomorphismsVisitor {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
        }
    }
    pub fn visit(&self, ast: &ASTFlatten) -> <Self as AstVisitor>::Return {
        // println!("===visit=======out======={:?}",ast.get_ast_type());
        let all_homs = <Self as AstVisitor>::visit(self, ast); //TODO super()

        if is_instance(ast, ASTType::SourceUnit) {
            // println!("===visit=============={:?}",ast.get_ast_type());
            ast.try_as_source_unit_ref()
                .unwrap()
                .borrow_mut()
                .used_homomorphisms = Some(all_homs.clone());

            ast.try_as_source_unit_ref()
                .unwrap()
                .borrow_mut()
                .used_crypto_backends = Some(Self::used_crypto_backends(all_homs.clone()));
        } else if is_instance(ast, ASTType::ConstructorOrFunctionDefinition) {
            // println!("===visit=============={:?}",ast.get_ast_type());
            ast.try_as_constructor_or_function_definition_ref()
                .unwrap()
                .borrow_mut()
                .used_homomorphisms = Some(all_homs.clone());

            ast.try_as_constructor_or_function_definition_ref()
                .unwrap()
                .borrow_mut()
                .used_crypto_backends = Some(Self::used_crypto_backends(all_homs.clone()));
        } else if is_instance(ast, ASTType::ContractDefinition) {
            // println!("===visit=============={:?}",ast.get_ast_type());
            ast.try_as_contract_definition_ref()
                .unwrap()
                .borrow_mut()
                .used_crypto_backends = Self::used_crypto_backends(all_homs.clone());
        }
        all_homs
    }
    pub fn visitAnnotatedTypeName(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        if ast
            .try_as_annotated_type_name_ref()
            .unwrap()
            .borrow()
            .is_private()
        {
            return Ok(BTreeSet::from([ast
                .try_as_annotated_type_name_ref()
                .unwrap()
                .borrow()
                .homomorphism
                .clone()]));
        }
        Ok(BTreeSet::new())
    }

    pub fn visitExpression(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        if ast
            .ast_base_ref()
            .unwrap()
            .borrow()
            .annotated_type()
            .is_some()
            && ast
                .ast_base_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .is_private()
        {
            return Ok(BTreeSet::from([ast
                .ast_base_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .homomorphism
                .clone()])
            .union(&self.visit_children(ast)?)
            .cloned()
            .collect());
        }
        self.visit_children(ast)
    }

    pub fn visitIdentifierDeclaration(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        self.visit_children(ast)
    }
    // Visits annotated type of identifier (and initial value expression)
    pub fn visitConstructorOrFunctionDefinition(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        self.visit_children(ast)
    } // Parameter and return types are children; don"t bother with "function type"

    pub fn visitEnumDefinition(
        &self,
        _ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(BTreeSet::new())
    } // Neither the enum type nor the types of the enum values can be private

    pub fn visitStructDefinition(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        self.visit_children(ast)
    } // Struct types are never private, but they may have private members

    pub fn visitSourceUnit(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        let used_homs = self.visit_children(ast);
        // Now all constructors or functions have been visited and we can do some post-processing
        // If some function f calls some function g, and g uses crypto-backend c, f also uses crypto-backend c
        // We have to do this for all transitively called functions g, being careful around recursive function calls
        let mut all_fcts = ast
            .try_as_source_unit_ref()
            .unwrap()
            .borrow()
            .contracts
            .iter()
            .fold(vec![], |mut a, c| {
                a.extend(c.borrow().constructor_definitions.clone());
                a.extend(c.borrow().function_definitions.clone());
                a
            });
        Self::compute_transitive_homomorphisms(&all_fcts);
        for f in &all_fcts {
            let used_crypto_backends = Some(Self::used_crypto_backends(
                f.borrow().used_homomorphisms.clone().unwrap(),
            ));
            f.borrow_mut().used_crypto_backends = used_crypto_backends;
        }
        used_homs
    }
    // Invert called_functions relation
    pub fn compute_transitive_homomorphisms(fcts: &Vec<RcCell<ConstructorOrFunctionDefinition>>) {
        let mut callers: BTreeMap<_, _> = fcts.iter().map(|f| (f.clone(), vec![])).collect(); //<ConstructorOrFunctionDefinition, Vec<ConstructorOrFunctionDefinition>> ;
        for f in fcts {
            for g in &f.borrow().called_functions {
                if g.borrow().used_homomorphisms.is_none() {
                    // Called function not analyzed, (try to) make sure this is a built-in like transfer, send
                    assert!(
                        !g.borrow().requires_verification
                            && !g
                                .borrow()
                                .body
                                .as_ref()
                                .unwrap()
                                .borrow()
                                .statement_list_base
                                .statements
                                .is_empty()
                    );
                    continue;
                }
                callers.entry(g.clone()).or_insert(vec![]).push(f.clone());
            }
        }

        // If a function uses any homomorphisms and gets called, propagate its homomorphisms to its callers
        let mut dirty = fcts
            .iter()
            .filter(|f| f.borrow().used_homomorphisms.is_some() && !callers[f].is_empty())
            .cloned()
            .collect::<BTreeSet<_>>();
        while !dirty.is_empty() {
            let f = dirty.pop_first().unwrap();
            // Add all of f"s used homomorphisms to all of its callers g.
            // If this added a new homomorphism to g, mark g as dirty (if not already) -> iterate to fixed point
            for g in callers.get(&f).unwrap() {
                if f == *g {
                    continue;
                }
                let old_len = g.borrow().used_homomorphisms.as_ref().unwrap().len();
                println!("================used_homomorphisms===============");
                g.borrow_mut().used_homomorphisms = Some(
                    g.borrow()
                        .used_homomorphisms
                        .as_ref()
                        .unwrap()
                        .union(&f.borrow().used_homomorphisms.clone().unwrap())
                        .cloned()
                        .collect(),
                );
                if g.borrow().used_homomorphisms.as_ref().unwrap().len() > old_len
                    && !callers[g].is_empty()
                {
                    dirty.insert(g.clone());
                }
            }
        }
    }
    // Base case, make sure we don"t miss any annotated types
    pub fn visitAST(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        assert!(
            ast.ast_base_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .is_none(),
            "Unhandled AST element of type {:?} with annotated type",
            ast
        );
        self.visit_children(ast)
    }
    // Guarantee consistent order
    pub fn used_crypto_backends(used_homs: BTreeSet<String>) -> Vec<CryptoParams> {
        let user_config = CFG.lock().unwrap().user_config.clone();
        Homomorphism::fields()
            .iter()
            .filter_map(|hom| {
                used_homs
                    .contains(hom)
                    .then(|| user_config.get_crypto_params(&hom))
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }
}
