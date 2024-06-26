// """
// This package contains modules with functionality related to building, traversing, transforming and outputting
// ASTs.

// ==========
// Submodules
// ==========
// * :py:mod:`.ast.py`: Defines all AST elements
// * :py:mod:`.build_ast.py`: Parser wrapper, which manually constructs AST elements for which there is no 1:1 correspondence to the antlr parser output.
// * :py:mod:`.global_defs.py`: Definitions for globally accessible solidity builtin functions and variables.
// * :py:mod:`.process_ast.py`: Takes a raw AST and performs pre-processing and type-checking.

// ===========
// Subpackages
// ===========
// * :py:mod:`.analysis`: Program analysis functionality
// * :py:mod:`.pointers`: Functionality for resolving references to other AST elements.
// * :py:mod:`.visitor`: Visitor classes for processing ASTs
// """
pub mod analysis;
pub mod ast;
// pub mod build_ast;
pub mod global_defs;
pub mod homomorphism;
pub mod pointers;
// pub mod process_ast;
pub mod circuit_constraints;
pub mod visitors;
