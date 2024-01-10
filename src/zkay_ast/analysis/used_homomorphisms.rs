
use crate::config::CFG;
use crate::transaction::crypto::params::CryptoParams;
use crate::zkay_ast::ast::{AST, AnnotatedTypeName, ConstructorOrFunctionDefinition, EnumDefinition, 
    Expression, IdentifierDeclaration, SourceUnit, StructDefinition};
use crate::zkay_ast::homomorphism::Homomorphism;
use crate::zkay_ast::visitor::visitor::AstVisitor;


// class UsedHomomorphismsVisitor(AstVisitor)
pub struct UsedHomomorphismsVisitor;
    //pub fn __init__(self)
    //     super().__init__(traversal="node-or-children")
    pub fn new()->Self{
    Self{}}
   pub fn visitChildren(self, ast) -> BTreeSet<Homomorphism>
      {  all_homs = set();
        for c in ast.children()
            {all_homs |= self.visit(c);}
        return all_homs}

   pub fn visitAnnotatedTypeName(self, ast: AnnotatedTypeName) -> BTreeSet<Homomorphism>
         {{ast.homomorphism} if ast.is_private() else set()}

   pub fn visitExpression(self, ast: Expression) -> BTreeSet<Homomorphism>
       { if ast.annotated_type is not None and ast.annotated_type.is_private()
            { {ast.annotated_type.homomorphism} | self.visitChildren(ast)}
        else
             {self.visitChildren(ast)}}

   pub fn visitIdentifierDeclaration(self, ast: IdentifierDeclaration) -> BTreeSet<Homomorphism>
         {self.visitChildren(ast) }// Visits annotated type of identifier (and initial value expression)

   pub fn visitConstructorOrFunctionDefinition(self, ast: ConstructorOrFunctionDefinition) -> BTreeSet<Homomorphism>
         {self.visitChildren(ast)} // Parameter and return types are children; don"t bother with "function type"

   pub fn visitEnumDefinition(self, ast: EnumDefinition)
        { set()} // Neither the enum type nor the types of the enum values can be private

   pub fn visitStructDefinition(self, ast: StructDefinition)
        { self.visitChildren(ast)} // Struct types are never private, but they may have private members

   pub fn visitSourceUnit(self, ast: SourceUnit)
      {  used_homs = self.visitChildren(ast);
       // Now all constructors or functions have been visited and we can do some post-processing
       // If some function f calls some function g, and g uses crypto-backend c, f also uses crypto-backend c
       // We have to do this for all transitively called functions g, being careful around recursive function calls
        all_fcts = sum([c.constructor_definitions + c.function_definitions for c in ast.contracts], []);
        self.compute_transitive_homomorphisms(all_fcts);
        for f in all_fcts
           { f.used_crypto_backends = self.used_crypto_backends(f.used_homomorphisms);}
        return used_homs}

   
   pub fn compute_transitive_homomorphisms(fcts: Vec<ConstructorOrFunctionDefinition>)
       // Invert called_functions relation
      {  callers= BTreeMap::new();//<ConstructorOrFunctionDefinition, Vec<ConstructorOrFunctionDefinition>> ;
        for f in fcts
            {callers[f] = vec![];}
        for f in fcts
            {for g in f.called_functions
                {if g.used_homomorphisms is None
                   // Called function not analyzed, (try to) make sure this is a built-in like transfer, send
                   { assert not g.requires_verification and not g.body.statements;
                    continue}
                callers[g].append(f)}}

       // If a function uses any homomorphisms and gets called, propagate its homomorphisms to its callers
        dirty = set([f for f in fcts if f.used_homomorphisms and callers[f]]);
        while dirty
            {f = dirty.pop();
           // Add all of f"s used homomorphisms to all of its callers g.
           // If this added a new homomorphism to g, mark g as dirty (if not already) -> iterate to fixed point
            for g in callers[f]
                {if f == g
                    {continue}
                old_len = len(g.used_homomorphisms);
                g.used_homomorphisms |= f.used_homomorphisms;
                if len(g.used_homomorphisms) > old_len and callers[g]
                    {dirty.add(g);}}}
}
   pub fn visitAST(self, ast: AST) -> BTreeSet<Homomorphism>
       // Base case, make sure we don"t miss any annotated types
      {  if hasattr(ast, "annotated_type")
            {raise ValueError(f"Unhandled AST element of type {ast.__class__.__name__} with annotated type")}
        return self.visitChildren(ast)}

   pub fn visit(self, ast)
       { all_homs = super().visit(ast);
        if hasattr(ast, "used_homomorphisms")
           { ast.used_homomorphisms = all_homs;}
        if hasattr(ast, "used_crypto_backends")
           { ast.used_crypto_backends = self.used_crypto_backends(all_homs);}
        return all_homs}

   
   pub fn used_crypto_backends(used_homs: BTreeSet<Homomorphism>) -> Vec<CryptoParams>
       // Guarantee consistent order
        {result = vec![];
        for hom in Homomorphism
           { if hom in used_homs
               { crypto_backend = cfg.get_crypto_params(hom);
                if crypto_backend not in result
                    {result.append(crypto_backend);}}}
        return result}
