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
        ) || matches!(ast, AST::Expression(_))
            || matches!(ast, AST::IdentifierDeclaration(_))
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> eyre::Result<Self::Return> {
        match name {
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
        for c in ast.children().iter_mut() {
            all_homs = all_homs.union(&self.visit(c)).cloned().collect();
        }
        Ok(all_homs)
    }
    fn visit(&self, ast: &ASTFlatten) -> Self::Return {
        let all_homs = BTreeSet::new(); //AstVisitor::visit(self, ast); //TODO super()
        if is_instance(ast, ASTType::ConstructorOrFunctionDefinition) {
            if ast
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .borrow()
                .used_homomorphisms
                .is_some()
            {
                ast.try_as_constructor_or_function_definition_ref()
                    .unwrap()
                    .borrow_mut()
                    .used_homomorphisms = Some(all_homs.clone());
            }
            if ast
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .borrow()
                .used_crypto_backends
                .is_some()
            {
                ast.try_as_constructor_or_function_definition_ref()
                    .unwrap()
                    .borrow_mut()
                    .used_crypto_backends = Some(Self::used_crypto_backends(all_homs.clone()));
            }
        }
        all_homs
    }
}
impl UsedHomomorphismsVisitor {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", false),
        }
    }

    pub fn visitAnnotatedTypeName(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(
            if ast
                .try_as_annotated_type_name_ref()
                .unwrap()
                .borrow()
                .is_private()
            {
                BTreeSet::from([ast
                    .try_as_annotated_type_name_ref()
                    .unwrap()
                    .borrow()
                    .homomorphism
                    .clone()])
            } else {
                BTreeSet::new()
            },
        )
    }

    pub fn visitExpression(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        if ast
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .annotated_type()
            .is_some()
            && ast
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .is_private()
        {
            Ok(BTreeSet::from([ast
                .try_as_expression_ref()
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
            .collect())
        } else {
            self.visit_children(ast)
        }
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
        for f in all_fcts.iter_mut() {
            f.borrow_mut().used_crypto_backends = Some(Self::used_crypto_backends(
                f.borrow().used_homomorphisms.clone().unwrap(),
            ));
        }
        used_homs
    }
    // Invert called_functions relation
    pub fn compute_transitive_homomorphisms(fcts: &Vec<RcCell<ConstructorOrFunctionDefinition>>) {
        let mut callers = BTreeMap::new(); //<ConstructorOrFunctionDefinition, Vec<ConstructorOrFunctionDefinition>> ;
        for f in fcts {
            callers.insert(f.clone(), vec![]);
        }
        for f in fcts {
            for g in &f.borrow().called_functions {
                if g.borrow().used_homomorphisms.is_none()
                // Called function not analyzed, (try to) make sure this is a built-in like transfer, send
                {
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
            .filter_map(|f| {
                if f.borrow().used_homomorphisms.is_some() && !callers[f].is_empty() {
                    Some(f.clone())
                } else {
                    None
                }
            })
            .collect::<BTreeSet<_>>();
        let callerss = callers.clone();
        while !dirty.is_empty() {
            let f = dirty.pop_first().unwrap();
            // Add all of f"s used homomorphisms to all of its callers g.
            // If this added a new homomorphism to g, mark g as dirty (if not already) -> iterate to fixed point
            for g in callers.get_mut(&f).unwrap() {
                if f == *g {
                    continue;
                }
                let old_len = g.borrow().used_homomorphisms.as_ref().unwrap().len();
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
                    && !callerss[g].is_empty()
                {
                    dirty.insert(g.clone());
                }
            }
        }
    }
    // Base case, make sure we don"t miss any annotated types
    pub fn visitAST(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        assert!(
            ast.try_as_expression_ref()
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
        let mut result = vec![];
        for hom in Homomorphism::fields() {
            if used_homs.contains(&hom) {
                let crypto_backend = CFG.lock().unwrap().user_config.get_crypto_params(&hom);
                if !result.contains(&crypto_backend) {
                    result.push(crypto_backend);
                }
            }
        }
        result
    }
}
