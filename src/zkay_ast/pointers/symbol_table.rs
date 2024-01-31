// from typing import Tuple, Dict, Union
use crate::zkay_ast::ast::{
    is_instance, ASTChildren, ASTCode, ASTType, AnnotatedTypeName, Array, Block, Comment,
    ConstructorOrFunctionDefinition, ContractDefinition, EnumDefinition, EnumValue, Expression,
    ForStatement, Identifier, IdentifierBase, IdentifierDeclaration, IdentifierExpr, IndexExpr,
    LocationExpr, Mapping, MemberAccessExpr, NamespaceDefinition, SimpleStatement, SourceUnit,
    Statement, StatementList, StructDefinition, TargetDefinition, TupleOrLocationExpr, TypeName,
    UserDefinedTypeName, VariableDeclaration, VariableDeclarationStatement, AST,
};
use crate::zkay_ast::global_defs::{ARRAY_LENGTH_MEMBER, GLOBAL_DEFS, GLOBAL_VARS};
use serde::{Deserialize, Serialize};
// from zkay::zkay_ast::pointers::pointer_exceptions import UnknownIdentifierException
use crate::zkay_ast::visitor::visitor::AstVisitor;

pub fn fill_symbol_table(ast: &AST) {
    let v = SymbolTableFiller;
    v.visit(ast);
}

pub fn link_symbol_table(ast: &AST) {
    let v = SymbolTableLinker;
    v.visit(ast.clone());
}

pub fn link_identifiers(ast: &AST) {
    fill_symbol_table(ast);
    link_symbol_table(ast);
}
use std::collections::BTreeMap;
pub fn merge_dicts(dict_args: Vec<BTreeMap<String, Identifier>>) -> BTreeMap<String, Identifier>
// """
    // Given any number of dicts, shallow copy and merge into a new dict.
    // Report error on conflicting keys.
    // """
{
    let mut result = BTreeMap::new();
    for dictionary in dict_args {
        for (key, value) in dictionary {
            if let Some(&v) = result.get(&key) {
                if v != value {
                    // raise ValueError("Conflicting definitions for", key)
                    assert!(false, "Conflicting definitions for {}", key);
                }
            }
            result.insert(key, value);
        }
    }
    result
}

pub fn collect_children_names(ast: &mut AST) -> BTreeMap<String, Identifier> {
    let children = ast
        .children()
        .iter()
        .filter_map(|c| {
            if c.is_block() || c.is_for_statement() {
                None
            } else {
                Some(c)
            }
        })
        .collect();
    let names: Vec<_> = children.iter().map(|c| c.names.clone()).collect();
    let ret = merge_dicts(names);
    for c in children.iter_mut()
    //declared names are not available within the declaration statements
    {
        c.names.clear();
    }
    ret
}

pub fn get_builtin_globals() -> BTreeMap<String, Identifier> {
    let sf = SymbolTableFiller;
    sf.get_builtin_globals()
}

struct SymbolTableFiller;
impl AstVisitor for SymbolTableFiller {
    type Return = Option<String>;
    fn temper_result(&self) -> Self::Return {
        None
    }
    fn log(&self) -> bool {
        false
    }
    fn traversal(&self) -> &'static str {
        "node-or-children"
    }
    fn has_attr(&self, name: &String) -> bool {
        self.get_attr(name).is_some()
    }
    fn get_attr(&self, name: &String) -> Option<String> {
        None
    }
    fn call_visit_function(&self, ast: &AST) -> Self::Return {
        None
    }
}
// class SymbolTableFiller(AstVisitor)
impl SymbolTableFiller {
    pub fn get_builtin_globals(&self) -> BTreeMap<String, Identifier> {
        let mut global_defs = GLOBAL_DEFS.vars();
        for d in global_defs.iter_mut() {
            self.visit((*d).get_ast());
        }
        let global_defs = global_defs
            .iter()
            .map(|d| {
                (
                    d.namespace_definition_base.idf.name().clone(),
                    d.namespace_definition_base.idf.clone(),
                )
            })
            .collect();
        let global_vars = GLOBAL_VARS
            .vars()
            .into_iter()
            .map(|d| {
                (
                    d.dentifier_declaration_base.idf.name.clone(),
                    d.dentifier_declaration_base.idf.clone(),
                )
            })
            .collect();
        merge_dicts(vec![global_defs, global_vars])
    }

    pub fn visitSourceUnit(&self, ast: &mut SourceUnit) {
        ast.ast_base.names = ast
            .contracts
            .iter()
            .map(|d| {
                (
                    d.namespace_definition_base.idf.name.clone(),
                    d.namespace_definition_base.idf.clone(),
                )
            })
            .collect();
        ast.ast_base.names.append(&mut self.get_builtin_globals());
    }

    pub fn visitContractDefinition(&self, ast: &mut ContractDefinition) {
        let state_vars = ast
            .state_variable_declarations
            .iter()
            .filter_map(|d| {
                if is_instance(d, ASTType::Comment) {
                    None
                } else {
                    Some((d.idf.name.clone(), d.idf.clone()))
                }
            })
            .collect();
        let mut funcs = BTreeMap::new();
        for f in ast.function_definitions {
            if funcs.contains_key(&f.namespace_definition_base.idf.name) {
                // raise UnknownIdentifierException(f"Zkay does not currently support method overloading.", f)
                assert!(
                    false,
                    "Zkay does not currently support method overloading.{:?}",
                    f
                );
            }
            funcs.insert(
                f.namespace_definition_base.idf.name,
                f.namespace_definition_base.idf,
            );
        }
        let structs = ast
            .struct_definitions
            .iter()
            .map(|d| {
                (
                    d.namespace_definition_base.idf.name.clone(),
                    d.namespace_definition_base.idf.clone(),
                )
            })
            .collect();
        let enums = ast
            .enum_definitions
            .iter()
            .map(|d| {
                (
                    d.namespace_definition_base.idf.name.clone(),
                    d.namespace_definition_base.idf.clone(),
                )
            })
            .collect();
        ast.namespace_definition_base.ast_base.names =
            merge_dicts(vec![state_vars, funcs, structs, enums]);
    }

    pub fn visitConstructorOrFunctionDefinition(&self, ast: &mut ConstructorOrFunctionDefinition) {
        ast.namespace_definition_base.ast_base.names = ast
            .parameters
            .iter()
            .map(|d| {
                (
                    d.identifier_declaration_base.idf.name.clone(),
                    d.identifier_declaration_base.idf.clone(),
                )
            })
            .collect();
    }

    pub fn visitStructDefinition(&self, ast: &mut StructDefinition) {
        ast.namespace_definition_base.ast_base.names = ast
            .members
            .iter()
            .map(|d| (d.idf.name.clone(), d.idf.clone()))
            .collect();
    }
    pub fn visitEnumDefinition(&self, ast: EnumDefinition) {
        ast.namespace_definition_base.ast_base.names = ast
            .values
            .iter()
            .map(|d| (d.idf.name.clone(), d.idf.clone()))
            .collect();
    }
    pub fn visitEnumValue(&self, ast: &mut EnumValue) {}

    pub fn visitVariableDeclaration(&self, ast: &mut VariableDeclaration) {
        ast.identifier_declaration_base.ast_base.names = BTreeMap::from([(
            ast.identifier_declaration_base.idf.name,
            ast.identifier_declaration_base.idf,
        )]);
    }

    pub fn visitStatementList(&self, ast: &mut StatementList) {
        ast.identifier_declaration_base.ast_base.names = collect_children_names(&mut ast.get_ast());
    }

    pub fn visitSimpleStatement(&self, ast: &mut SimpleStatement) {
        ast.names = collect_children_names(&mut ast.get_ast());
    }

    pub fn visitForStatement(&self, ast: &mut ForStatement) {
        ast.statement_base.ast_base.names = collect_children_names(&mut ast.get_ast());
    }

    pub fn visitMapping(&self, ast: &mut Mapping) {
        ast.type_name_base.ast_base.names = BTreeMap::new();
        if ast.key_label.unwrap().is_identifier() {
            ast.type_name_base.ast_base.names =
                BTreeMap::from([(ast.key_label.unwrap().name.clone(), ast.key_label.clone())]);
        }
    }
}

pub struct SymbolTableLinker;
// class SymbolTableLinker(AstVisitor)
impl AstVisitor for SymbolTableLinker {
    type Return = Option<String>;
    fn temper_result(&self) -> Self::Return {
        None
    }
    fn log(&self) -> bool {
        false
    }
    fn traversal(&self) -> &'static str {
        "node-or-children"
    }
    fn has_attr(&self, name: &String) -> bool {
        self.get_attr(name).is_some()
    }
    fn get_attr(&self, name: &String) -> Option<String> {
        None
    }
    fn call_visit_function(&self, ast: &AST) -> Self::Return {
        None
    }
}
impl SymbolTableLinker {
    pub fn _find_next_decl(ast: AST, name: String) -> (AST, TargetDefinition) {
        let mut ancestor = ast.parent();
        while let Some(_ancestor) = ancestor {
            if let Some(nameo) = _ancestor.names().get(&name) {
                let decl = nameo.parent();
                if !(if let Some(AST::Statement(Statement::SimpleStatement(
                    SimpleStatement::VariableDeclarationStatement(_),
                ))) = &decl.clone().unwrap_or_default().parent()
                {
                    true
                } else {
                    false
                }) || !decl
                    .clone()
                    .unwrap_or_default()
                    .parent()
                    .unwrap_or_default()
                    .is_parent_of(&ast)
                {
                    return (_ancestor, decl.unwrap_or_default().into());
                }
            }
            ancestor = _ancestor.parent();
        }
        // raise UnknownIdentifierException(f"Undefined identifier {name}", ast)
        // assert!(false, "Undefined identifier {name},{:?}", ast);
        (AST::None, TargetDefinition::None)
    }

    pub fn _find_lca(mut ast1: AST, mut ast2: AST, root: AST) -> (StatementList, AST, AST) {
        assert!(ast1 != ast2);

        // Gather ast1"s ancestors + immediate child towards ast1 (for each)
        let mut ancs = BTreeMap::new();
        loop {
            assert!(ast1.parent().is_some());
            ancs.insert(ast1.parent().unwrap(), ast1.clone());
            ast1 = ast1.parent().unwrap_or_default();
            if ast1 == root {
                break;
            }
        }

        // Find least common ancestor with ast2 + immediate child towards ast2
        loop {
            assert!(ast2.parent().is_some());
            let old_ast = ast2.clone();
            let ast2 = ast2.parent();
            if let Some(ast2v) = ancs.get(&ast2.clone().unwrap_or_default()) {
                assert!(if let Some(AST::Statement(Statement::ForStatement(_)))
                | Some(AST::Statement(Statement::StatementList(_))) = &ast2
                {
                    true
                } else {
                    false
                });
                return (
                    ast2.clone()
                        .map(|a| {
                            if let AST::Statement(Statement::StatementList(a)) = a {
                                a
                            } else {
                                StatementList::default()
                            }
                        })
                        .unwrap_or_default(),
                    ast2v.clone(),
                    old_ast,
                );
            }
        }
    }

    pub fn find_type_declaration(&self, t: UserDefinedTypeName) -> NamespaceDefinition {
        if let TargetDefinition::NamespaceDefinition(s) = SymbolTableLinker::_find_next_decl(
            AST::TypeName(TypeName::UserDefinedTypeName(t.clone())),
            t.names()[0].name(),
        )
        .1
        {
            s
        } else {
            NamespaceDefinition::default()
        }
    }

    pub fn find_identifier_declaration(&self, mut ast: &IdentifierExpr) -> TargetDefinitionUnion {
        let mut ast = ast.get_ast();
        let name = ast.idf().name();
        loop {
            let (anc, decl) = SymbolTableLinker::_find_next_decl(ast.clone(), name.clone());
            if let (
                AST::Statement(Statement::ForStatement(anc)),
                TargetDefinition::IdentifierDeclaration(
                    IdentifierDeclaration::VariableDeclaration(decl),
                ),
            ) = (&anc, &decl)
            {
                // Check if identifier really references this declaration (does not come before declaration)
                let (lca, ref_anchor, decl_anchor) =
                    SymbolTableLinker::_find_lca(ast.clone(), decl.get_ast(), anc.get_ast());
                if lca.statements().iter().find(|x| (*x).clone() == ref_anchor)
                    <= lca
                        .statements()
                        .iter()
                        .find(|x| (*x).clone() == decl_anchor)
                {
                    ast = anc.get_ast();
                    continue;
                }
            }
            if let (
                AST::Statement(Statement::StatementList(StatementList::Block(anc))),
                TargetDefinition::IdentifierDeclaration(
                    IdentifierDeclaration::VariableDeclaration(decl),
                ),
            ) = (&anc, &decl)
            {
                // Check if identifier really references this declaration (does not come before declaration)
                let (lca, ref_anchor, decl_anchor) =
                    SymbolTableLinker::_find_lca(ast.get_ast(), decl.get_ast(), anc.get_ast());
                if lca.statements().iter().find(|x| (*x).clone() == ref_anchor)
                    <= lca
                        .statements()
                        .iter()
                        .find(|x| (*x).clone() == decl_anchor)
                {
                    ast = anc.get_ast();
                    continue;
                }
            }
            return TargetDefinitionUnion::TargetDefinition(decl.clone());
        }
    }

    pub fn in_scope_at(target_idf: &Identifier, ast: AST) -> bool {
        let mut ancestor = ast.parent();
        while let Some(_ancestor) = ancestor {
            if let Some(name) = _ancestor.names().get(&target_idf.name())
            // found name
            {
                if name == target_idf {
                    return true;
                }
            }
            ancestor = _ancestor.parent();
        }
        false
    }

    pub fn visitIdentifierExpr(&self, mut ast: IdentifierExpr) -> IdentifierExpr {
        let decl = self.find_identifier_declaration(&ast);
        if let TargetDefinitionUnion::TargetDefinition(decl) = decl {
            ast.location_expr_base.target = Some(Box::new(decl));
        }

        assert!(ast.location_expr_base.target.as_ref().is_some());
        ast
    }

    pub fn visitUserDefinedTypeName(&self, mut ast: UserDefinedTypeName) -> UserDefinedTypeName {
        //  try:
        let mut type_def = self.find_type_declaration(ast.clone());
        for idf in &ast.names()[1..] {
            if let Some(_idf) = type_def.names().get(&idf.name()) {
                if let Some(AST::NamespaceDefinition(parent)) = _idf.parent() {
                    type_def = parent;
                }
            } else {
                return ast;
            }
        }
        ast = match ast {
            UserDefinedTypeName::EnumTypeName(mut ast) => {
                ast.user_defined_type_name_base.target = Some(Box::new(type_def.get_ast()));
                UserDefinedTypeName::EnumTypeName(ast)
            }
            UserDefinedTypeName::EnumValueTypeName(mut ast) => {
                ast.user_defined_type_name_base.target = Some(Box::new(type_def.get_ast()));
                UserDefinedTypeName::EnumValueTypeName(ast)
            }
            UserDefinedTypeName::StructTypeName(mut ast) => {
                ast.user_defined_type_name_base.target = Some(Box::new(type_def.get_ast()));
                UserDefinedTypeName::StructTypeName(ast)
            }
            UserDefinedTypeName::ContractTypeName(mut ast) => {
                ast.user_defined_type_name_base.target = Some(Box::new(type_def.get_ast()));
                UserDefinedTypeName::ContractTypeName(ast)
            }
            UserDefinedTypeName::AddressTypeName(mut ast) => {
                ast.user_defined_type_name_base.target = Some(Box::new(type_def.get_ast()));
                UserDefinedTypeName::AddressTypeName(ast)
            }
            UserDefinedTypeName::AddressPayableTypeName(mut ast) => {
                ast.user_defined_type_name_base.target = Some(Box::new(type_def.get_ast()));
                UserDefinedTypeName::AddressPayableTypeName(ast)
            }
            _ => UserDefinedTypeName::default(),
        };

        // except UnknownIdentifierException:
        //     pass
        ast
    }
    pub fn visitMemberAccessExpr(&self, mut ast: MemberAccessExpr) -> MemberAccessExpr {
        // assert!(
        //     if let
        //         TupleOrLocationExpr::LocationExpr(_),
        //     ) = *ast.expr
        //     {
        //         true
        //     } else {
        //         false
        //     },
        //     "Function call return value member access not yet supported"
        // );
        if let Some(TargetDefinition::NamespaceDefinition(target)) = ast.expr.target().map(|t| *t) {
            if let Some(idf) = target.names().get(&ast.member.name()) {
                ast.location_expr_base.target = idf.parent().map(|t| Box::new(t.into()));
            }
        } else {
            let mut t = ast
                .expr
                .target()
                .unwrap_or_default()
                .annotated_type()
                .type_name;
            if let TypeName::Array(_) = *t {
                assert!(ast.member.name() == "length");
                ast.location_expr_base.target =
                    Some(Box::new(TargetDefinition::IdentifierDeclaration(
                        IdentifierDeclaration::VariableDeclaration(ARRAY_LENGTH_MEMBER.clone()),
                    )));
            } else if let TypeName::UserDefinedTypeName(mut t) = *t {
                // assert!(isinstance(t, UserDefinedTypeName));
                if let Some(target) = t.target() {
                    if let Some(idf) = target.names().get(&ast.member.name()) {
                        ast.location_expr_base.target = idf.parent().map(|p| Box::new(p.into()));
                    }
                } else {
                    t = t.clone();
                    t.set_parent(Some(ast.get_ast()));
                    self.visit(t.get_ast());
                }
            } else {
                assert!(false);
            }
        }
        ast
    }

    pub fn visitIndexExpr(&self, mut ast: IndexExpr) -> IndexExpr {
        // assert!(
        //     if let AST::Expression(expr) = &ast.arr {
        //         if let Expression::TupleOrLocationExpr(tole) = *expr {
        //             if let TupleOrLocationExpr::LocationExpr(_) = *tole {
        //                 true
        //             } else {
        //                 false
        //             }
        //         } else {
        //             false
        //         }
        //     } else {
        //         false
        //     },
        //     "Function call return value indexing not yet supported"
        // );
        let source_t = ast
            .arr
            .target()
            .unwrap_or_default()
            .annotated_type()
            .type_name;
        ast.location_expr_base.target = Some(Box::new(TargetDefinition::IdentifierDeclaration(
            IdentifierDeclaration::VariableDeclaration(VariableDeclaration::new(
                vec![],
                source_t.value_type(),
                Identifier::Identifier(IdentifierBase::new(String::new())),
                None,
            )),
        )));
        ast
    }
}

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(untagged)]
pub enum TargetDefinitionUnion {
    TargetDefinition(TargetDefinition),
    Mapping(Mapping),
    #[default]
    None,
}
