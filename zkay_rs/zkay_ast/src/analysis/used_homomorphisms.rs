#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

use crate::ast::{
    ASTChildren, AnnotatedTypeName, ConstructorOrFunctionDefinition, EnumDefinition, Expression,
    ExpressionBaseProperty, IdentifierDeclaration, IntoAST, SourceUnit, StructDefinition, AST,ASTType,
};
use crate::homomorphism::Homomorphism;
use crate::visitor::visitor::AstVisitor;
use std::collections::{BTreeMap, BTreeSet};
use zkay_config::config::CFG;
use zkay_crypto::params::CryptoParams;
// class UsedHomomorphismsVisitor(AstVisitor)
pub struct UsedHomomorphismsVisitor;

impl AstVisitor for UsedHomomorphismsVisitor {
    type Return = BTreeSet<String>;
    fn temper_result(&self) -> Self::Return {
        BTreeSet::new()
    }
    fn log(&self) -> bool {
        false
    }
    fn traversal(&self) -> &'static str {
        "node-or-children"
    }
    fn has_attr(&self, name: &ASTType) -> bool{
        false
    }
    fn get_attr(&self, name: &ASTType, ast: &AST) -> Option<Self::Return> {
        None
    }
}
impl UsedHomomorphismsVisitor {
    //pub fn __init__(self)
    //     super().__init__(traversal="node-or-children")
    pub fn new() -> Self {
        Self {}
    }
    pub fn visitChildren(&self, mut ast: AST) -> BTreeSet<String> {
        let mut all_homs = BTreeSet::new();
        for c in ast.children().iter_mut() {
            all_homs = all_homs.union(&self.visit(c)).cloned().collect();
        }
        return all_homs;
    }

    pub fn visitAnnotatedTypeName(&self, ast: AnnotatedTypeName) -> BTreeSet<String> {
        if ast.is_private() {
            BTreeSet::from([ast.homomorphism.clone()])
        } else {
            BTreeSet::new()
        }
    }

    pub fn visitExpression(&self, ast: Expression) -> BTreeSet<String> {
        if ast.annotated_type().is_some() && ast.annotated_type().as_ref().unwrap().is_private() {
            BTreeSet::from([ast.annotated_type().as_ref().unwrap().homomorphism.clone()])
                .union(&self.visitChildren(ast.to_ast()))
                .cloned()
                .collect()
        } else {
            self.visitChildren(ast.to_ast())
        }
    }

    pub fn visitIdentifierDeclaration(&self, ast: IdentifierDeclaration) -> BTreeSet<String> {
        self.visitChildren(ast.to_ast())
    } // Visits annotated type of identifier (and initial value expression)

    pub fn visitConstructorOrFunctionDefinition(
        self,
        ast: ConstructorOrFunctionDefinition,
    ) -> BTreeSet<String> {
        self.visitChildren(ast.to_ast())
    } // Parameter and return types are children; don"t bother with "function type"

    pub fn visitEnumDefinition(&self, _ast: EnumDefinition) -> <Self as AstVisitor>::Return {
        BTreeSet::new()
    } // Neither the enum type nor the types of the enum values can be private

    pub fn visitStructDefinition(&self, ast: StructDefinition) -> <Self as AstVisitor>::Return {
        self.visitChildren(ast.to_ast())
    } // Struct types are never private, but they may have private members

    pub fn visitSourceUnit(&self, ast: SourceUnit) -> <Self as AstVisitor>::Return {
        let used_homs = self.visitChildren(ast.to_ast());
        // Now all constructors or functions have been visited and we can do some post-processing
        // If some function f calls some function g, and g uses crypto-backend c, f also uses crypto-backend c
        // We have to do this for all transitively called functions g, being careful around recursive function calls
        let mut all_fcts = ast.contracts.iter().fold(vec![], |mut a, c| {
            a.extend(c.constructor_definitions.clone());
            a.extend(c.function_definitions.clone());
            a
        });
        Self::compute_transitive_homomorphisms(all_fcts.clone());
        for f in all_fcts.iter_mut() {
            f.used_crypto_backends = Some(Self::used_crypto_backends(
                f.used_homomorphisms.clone().unwrap(),
            ));
        }
        used_homs
    }

    pub fn compute_transitive_homomorphisms(fcts: Vec<ConstructorOrFunctionDefinition>)
    // Invert called_functions relation
    {
        let mut callers = BTreeMap::new(); //<ConstructorOrFunctionDefinition, Vec<ConstructorOrFunctionDefinition>> ;
        for f in &fcts {
            callers.insert(f.clone(), vec![]);
        }
        for f in &fcts {
            for g in &f.called_functions {
                if g.used_homomorphisms.is_none()
                // Called function not analyzed, (try to) make sure this is a built-in like transfer, send
                {
                    assert!(
                        !g.requires_verification
                            && !g
                                .body
                                .as_ref()
                                .unwrap()
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
                if f.used_homomorphisms.is_some() && !callers[f].is_empty() {
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
                let old_len = g.used_homomorphisms.as_ref().unwrap().len();
                g.used_homomorphisms = Some(
                    g.used_homomorphisms
                        .as_ref()
                        .unwrap()
                        .union(&f.used_homomorphisms.clone().unwrap())
                        .cloned()
                        .collect(),
                );
                if g.used_homomorphisms.as_ref().unwrap().len() > old_len && !callerss[g].is_empty()
                {
                    dirty.insert(g.clone());
                }
            }
        }
    }
    pub fn visitAST(&self, ast: AST) -> <Self as AstVisitor>::Return
// Base case, make sure we don"t miss any annotated types
    {
        assert!(
            ast.try_as_expression_ref()
                .unwrap()
                .annotated_type()
                .is_none(),
            "Unhandled AST element of type {:?} with annotated type",
            ast
        );
        self.visitChildren(ast)
    }

    pub fn visit(&self, ast: &mut AST) -> <Self as AstVisitor>::Return {
        let all_homs = BTreeSet::new(); //self.visit(ast); //TODO super()
        if let Some(mut ast) = ast
            .try_as_namespace_definition_mut()
            .unwrap()
            .try_as_constructor_or_function_definition_mut()
        {
            if let Some(_) = ast.used_homomorphisms {
                ast.used_homomorphisms = Some(all_homs.clone());
            }
            if let Some(_) = ast.used_crypto_backends {
                ast.used_crypto_backends = Some(Self::used_crypto_backends(all_homs.clone()));
            }
        }
        all_homs
    }

    pub fn used_crypto_backends(used_homs: BTreeSet<String>) -> Vec<CryptoParams>
// Guarantee consistent order
    {
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
