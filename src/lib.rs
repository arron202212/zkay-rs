// """
// This package contains modules which resolve references to other AST elements.

// ==========
// Submodules
// ==========
// * :py:mod:`.parent_setter`: Visitor which sets the parent, statement and function references for each element to the correct values.
// * :py:mod:`.pointer_exceptions`: Exceptions raised within this module
// * :py:mod:`.symbol_table`: Construct symbol table from AST and resolve identifier and user-defined-type targets.
// """
pub mod compiler;
pub mod config;
pub mod config_user;
pub mod config_version;
pub mod solidity_parser;
pub mod transaction;
pub mod utils;
pub mod zkay_ast;
pub mod zkay_frontend;
