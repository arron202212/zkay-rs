use crate::config::CFG;
use crate::transaction::crypto::params::CryptoParams;
use crate::zkay_ast::ast::{
    AnnotatedTypeName, ConstructorOrFunctionDefinition, EnumDefinition, Expression,
    IdentifierDeclaration, SourceUnit, StructDefinition, AST,
};
use crate::zkay_ast::homomorphism::Homomorphism;
use crate::zkay_ast::visitor::visitor::AstVisitor;
use std::collections::{BTreeMap, BTreeSet};
// class UsedHomomorphismsVisitor(AstVisitor)
pub struct UsedHomomorphismsVisitor;
impl UsedHomomorphismsVisitor {
    //pub fn __init__(self)
    //     super().__init__(traversal="node-or-children")
    pub fn new() -> Self {
        Self {}
    }
    pub fn visitChildren(self, ast: AST) -> BTreeSet<Homomorphism> {
        let mut all_homs = BTreeSet::new();
        for c in ast.children() {
            all_homs = all_homs.union(self.visit(c)).collect();
        }
        return all_homs;
    }

    pub fn visitAnnotatedTypeName(self, ast: AnnotatedTypeName) -> BTreeSet<Homomorphism> {
        if ast.is_private() {
            ast.homomorphism
        } else {
            BTreeSet::new()
        }
    }

    pub fn visitExpression(self, ast: Expression) -> BTreeSet<Homomorphism> {
        if ast.annotated_type.is_some() && ast.annotated_type.is_private() {
            ast.annotated_type
                .homomorphism
                .union(self.visitChildren(ast))
                .collect()
        } else {
            self.visitChildren(ast)
        }
    }

    pub fn visitIdentifierDeclaration(self, ast: IdentifierDeclaration) -> BTreeSet<Homomorphism> {
        self.visitChildren(ast)
    } // Visits annotated type of identifier (and initial value expression)

    pub fn visitConstructorOrFunctionDefinition(
        self,
        ast: ConstructorOrFunctionDefinition,
    ) -> BTreeSet<Homomorphism> {
        self.visitChildren(ast)
    } // Parameter and return types are children; don"t bother with "function type"

    pub fn visitEnumDefinition(self, ast: EnumDefinition) {
        BTreeSet::new()
    } // Neither the enum type nor the types of the enum values can be private

    pub fn visitStructDefinition(self, ast: StructDefinition) {
        self.visitChildren(ast)
    } // Struct types are never private, but they may have private members

    pub fn visitSourceUnit(self, ast: SourceUnit) {
        let used_homs = self.visitChildren(ast);
        // Now all constructors or functions have been visited and we can do some post-processing
        // If some function f calls some function g, and g uses crypto-backend c, f also uses crypto-backend c
        // We have to do this for all transitively called functions g, being careful around recursive function calls
        let all_fcts = ast.contracts.iter().fold(vec![], |mut a, c| {
            a.extend(c.constructor_definitions);
            a.extend(c.function_definitions);
            a
        });
        self.compute_transitive_homomorphisms(all_fcts);
        for f in all_fcts {
            f.used_crypto_backends = self.used_crypto_backends(f.used_homomorphisms);
        }
        return used_homs;
    }

    pub fn compute_transitive_homomorphisms(fcts: Vec<ConstructorOrFunctionDefinition>)
    // Invert called_functions relation
    {
        let callers = BTreeMap::new(); //<ConstructorOrFunctionDefinition, Vec<ConstructorOrFunctionDefinition>> ;
        for f in fcts {
            callers[f] = vec![];
        }
        for f in fcts {
            for g in f.called_functions {
                if g.used_homomorphisms.is_none()
                // Called function not analyzed, (try to) make sure this is a built-in like transfer, send
                {
                    assert!(!g.requires_verification && !g.body.statements);
                    continue;
                }
                callers[g].append(f)
            }
        }

        // If a function uses any homomorphisms and gets called, propagate its homomorphisms to its callers
        let dirty = fcts
            .iter()
            .filter_map(|f| {
                if f.used_homomorphisms && callers[f] {
                    Some(f)
                } else {
                    None
                }
            })
            .collect::<BTreeSet<_>>();
        while let Some(f) = dirty.iter().next() {
            // Add all of f"s used homomorphisms to all of its callers g.
            // If this added a new homomorphism to g, mark g as dirty (if not already) -> iterate to fixed point
            for g in &callers[f] {
                if f == g {
                    continue;
                }
                let old_len = g.used_homomorphisms.len();
                g.used_homomorphisms = g.used_homomorphisms.union(f.used_homomorphisms).collect();
                if g.used_homomorphisms.len() > old_len && callers[g] {
                    dirty.add(g);
                }
            }
        }
    }
    pub fn visitAST(self, ast: AST) -> BTreeSet<Homomorphism>
// Base case, make sure we don"t miss any annotated types
    {
        if ast.has_annotated_type() {
            assert!(
                false,
                "Unhandled AST element of type {:?} with annotated type",
                ast
            )
        }
        return self.visitChildren(ast);
    }

    pub fn visit(self, ast: AST) {
        let all_homs = self.visit(ast); //TODO super()
        if let Some(_) = ast.used_homomorphisms() {
            ast.used_homomorphisms = all_homs.clone();
        }
        if let Some(_) = ast.used_crypto_backends() {
            ast.used_crypto_backends = self.used_crypto_backends(all_homs);
        }
        all_homs
    }

    pub fn used_crypto_backends(used_homs: BTreeSet<Homomorphism>) -> Vec<CryptoParams>
// Guarantee consistent order
    {
        let mut result = vec![];
        for hom in Homomorphism::fields() {
            if used_homs.contains(&hom) {
                let crypto_backend = CFG.lock().unwrap().get_crypto_params(hom);
                if !result.contain(&crypto_backend) {
                    result.push(crypto_backend);
                }
            }
        }
        result
    }
}
