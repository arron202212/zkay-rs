// from typing import Tuple, Dict, Union
use crate::zkay_ast::ast::{
    AnnotatedTypeName, Array, Block, Comment, ConstructorOrFunctionDefinition, ContractDefinition,
    EnumDefinition, EnumValue, Expression, ForStatement, Identifier, IdentifierDeclaration,
    IdentifierExpr, IndexExpr, LocationExpr, Mapping, MemberAccessExpr, NamespaceDefinition,
    SimpleStatement, SourceUnit, Statement, StatementList, StructDefinition, TargetDefinition,
    TupleOrLocationExpr, TypeName, UserDefinedTypeName, VariableDeclaration,
    VariableDeclarationStatement, AST,ASTCode,IdentifierBase,
};
use crate::zkay_ast::global_defs::array_length_member;
use serde::{Deserialize, Serialize}; //GlobalDefs, GlobalVars,
                                     // from zkay::zkay_ast::pointers::pointer_exceptions import UnknownIdentifierException
use crate::zkay_ast::visitor::visitor::AstVisitor;

// def fill_symbol_table(ast):
//     v = SymbolTableFiller()
//     v.visit(ast)

// def link_symbol_table(ast):
//     v = SymbolTableLinker()
//     v.visit(ast)

// def link_identifiers(ast):
//     fill_symbol_table(ast)
//     link_symbol_table(ast)

// def merge_dicts(*dict_args):
//     """
//     Given any number of dicts, shallow copy and merge into a new dict.
//     Report error on conflicting keys.
//     """
//     result = {}
//     for dictionary in dict_args:
//         for key, value in dictionary.items():
//             if key in result and result[key] != value:
//                 raise ValueError("Conflicting definitions for", key)
//             result[key] = value
//     return result

// def collect_children_names(ast: AST) -> Dict[str, Identifier]:
//     children = [c for c in ast.children() if not isinstance(c, (Block, ForStatement))]
//     names = [c.names for c in children]
//     ret = merge_dicts(*names)
//     for c in children: # declared names are not available within the declaration statements
//         c.names.clear()
//     return ret

// def get_builtin_globals():
//     sf = SymbolTableFiller()
//     return sf.get_builtin_globals()

// class SymbolTableFiller(AstVisitor):
//     def get_builtin_globals(self):
//         global_defs = [d for d in [getattr(GlobalDefs, var) for var in vars(GlobalDefs) if not var.startswith("__")]]
//         for d in global_defs:
//             self.visit(d)
//         global_defs = {d.idf.name: d.idf for d in global_defs}
//         global_vars = {d.idf.name: d.idf for d in [getattr(GlobalVars, var) for var in vars(GlobalVars) if not var.startswith("__")]}
//         return merge_dicts(global_defs, global_vars)

//     def visitSourceUnit(&self, ast: SourceUnit):
//         ast.names = {c.idf.name: c.idf for c in ast.contracts}
//         ast.names.update(self.get_builtin_globals())

//     def visitContractDefinition(&self, ast: ContractDefinition):
//         state_vars = {d.idf.name: d.idf for d in ast.state_variable_declarations if not isinstance(d, Comment)}
//         funcs = {}
//         for f in ast.function_definitions:
//             if f.idf.name in funcs:
//                 raise UnknownIdentifierException(f"Zkay does not currently support method overloading.", f)
//             funcs[f.idf.name] = f.idf
//         structs = {s.idf.name: s.idf for s in ast.struct_definitions}
//         enums = {e.idf.name: e.idf for e in ast.enum_definitions}
//         ast.names = merge_dicts(state_vars, funcs, structs, enums)

//     def visitConstructorOrFunctionDefinition(&self, ast: ConstructorOrFunctionDefinition):
//         ast.names = {p.idf.name: p.idf for p in ast.parameters}

//     def visitStructDefinition(&self, ast: StructDefinition):
//         ast.names = {m.idf.name: m.idf for m in ast.members}

//     def visitEnumDefinition(&self, ast: EnumDefinition):
//         ast.names = {v.idf.name: v.idf for v in ast.values}

//     def visitEnumValue(&self, ast: EnumValue):
//         pass

//     def visitVariableDeclaration(&self, ast: VariableDeclaration):
//         ast.names = {ast.idf.name: ast.idf}

//     def visitStatementList(&self, ast: StatementList):
//         ast.names = collect_children_names(ast)

//     def visitSimpleStatement(&self, ast: SimpleStatement):
//         ast.names = collect_children_names(ast)

//     def visitForStatement(&self, ast: ForStatement):
//         ast.names = collect_children_names(ast)

//     def visitMapping(&self, ast: Mapping):
//         ast.names = {}
//         if isinstance(ast.key_label, Identifier):
//             ast.names = {ast.key_label.name: ast.key_label}
use std::collections::HashMap;
pub struct SymbolTableLinker;
// class SymbolTableLinker(AstVisitor)
impl AstVisitor for SymbolTableLinker {
    type Return = Option<String>;
    fn temper_result(&self) -> Option<Self::Return> {
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
                }) || !decl.clone().unwrap_or_default().parent().unwrap_or_default().is_parent_of(&ast)
                {
                    return (_ancestor, decl.unwrap_or_default().into())
                }
            }
            ancestor = _ancestor.parent();
        }
        // raise UnknownIdentifierException(f"Undefined identifier {name}", ast)
        // assert!(false, "Undefined identifier {name},{:?}", ast);
        (AST::None,TargetDefinition::None)
    }

    pub fn _find_lca(mut ast1: AST, mut ast2: AST, root: AST) -> (StatementList, AST, AST) {
        assert!(ast1 != ast2);

        // Gather ast1"s ancestors + immediate child towards ast1 (for each)
        let mut ancs = HashMap::new();
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
                return (ast2.clone().map(|a| if let AST::Statement(Statement::StatementList(a))=a{a}else{StatementList::default()}).unwrap_or_default(), ast2v.clone(), old_ast);
            }
        }
    }

    pub fn find_type_declaration(&self,t: UserDefinedTypeName) -> NamespaceDefinition {
         if let TargetDefinition::NamespaceDefinition(s)=SymbolTableLinker::_find_next_decl(AST::TypeName(TypeName::UserDefinedTypeName(t.clone())), t.names()[0].name())
            .1
            {s}else{NamespaceDefinition::default()}
    }

    pub fn find_identifier_declaration(&self,mut ast: &IdentifierExpr) -> TargetDefinitionUnion {
        let mut ast=ast.get_ast();
        let name = ast.idf().name();
        loop {
            let (anc, decl) = SymbolTableLinker::_find_next_decl(ast.clone(), name.clone());
            if let (
                AST::Statement(Statement::ForStatement(anc))
                ,
                TargetDefinition::IdentifierDeclaration(IdentifierDeclaration::VariableDeclaration(decl)),
            ) = (&anc, &decl)
            {
                // Check if identifier really references this declaration (does not come before declaration)
                let (lca, ref_anchor, decl_anchor) = SymbolTableLinker::_find_lca(ast.clone(), decl.get_ast(), anc.get_ast());
                if lca.statements().iter().find(|x| AST::Statement((*x).clone()) == ref_anchor)
                    <= lca.statements().iter().find(|x| AST::Statement((*x).clone()) == decl_anchor)
                {
                    ast = anc.get_ast();
                    continue
                }
            }
 if let (
                 AST::Statement(Statement::StatementList(StatementList::Block(anc))),
                TargetDefinition::IdentifierDeclaration(IdentifierDeclaration::VariableDeclaration(decl)),
            ) = (&anc, &decl)
            {
                // Check if identifier really references this declaration (does not come before declaration)
                let (lca, ref_anchor, decl_anchor) = SymbolTableLinker::_find_lca(ast.get_ast(), decl.get_ast(), anc.get_ast());
                if lca.statements().iter().find(|x| AST::Statement((*x).clone()) == ref_anchor)
                    <= lca.statements().iter().find(|x| AST::Statement((*x).clone()) == decl_anchor)
                {
                    ast = anc.get_ast();
                    continue
                }
            }
            return TargetDefinitionUnion::TargetDefinition(decl.clone())
        }
    }

    pub fn in_scope_at(target_idf: Identifier, ast: AST) -> bool {
        let mut ancestor = ast.parent();
        while let Some(_ancestor)=ancestor {
            if let Some(name) = _ancestor.names().get(&target_idf.name())
            // found name
            {
                if name == &target_idf {
                    return true
                }
            }
            ancestor = _ancestor.parent();
        }
        false
    }

    pub fn visitIdentifierExpr(&self, mut ast: IdentifierExpr) -> IdentifierExpr {
        let decl = self.find_identifier_declaration(&ast);
        if let TargetDefinitionUnion::TargetDefinition(decl)=decl{
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
                if let Some(AST::NamespaceDefinition(parent))=_idf.parent(){
                    type_def = parent;
                }
            } else {
                return ast
            }
        }
        ast=match ast {
            UserDefinedTypeName::EnumTypeName(mut ast) => {ast.user_defined_type_name_base.target = Some(Box::new(type_def));UserDefinedTypeName::EnumTypeName(ast)}
            UserDefinedTypeName::EnumValueTypeName(mut ast) => {ast.user_defined_type_name_base.target = Some(Box::new(type_def));UserDefinedTypeName::EnumValueTypeName( ast)}
            UserDefinedTypeName::StructTypeName(mut ast) => {ast.user_defined_type_name_base.target = Some(Box::new(type_def));UserDefinedTypeName::StructTypeName( ast)}
            UserDefinedTypeName::ContractTypeName(mut ast) => {ast.user_defined_type_name_base.target = Some(Box::new(type_def)); UserDefinedTypeName::ContractTypeName( ast)}
            UserDefinedTypeName::AddressTypeName(mut ast) => {ast.user_defined_type_name_base.target = Some(Box::new(type_def)); UserDefinedTypeName::AddressTypeName( ast)}
            UserDefinedTypeName::AddressPayableTypeName(mut ast) => {ast.user_defined_type_name_base.target = Some(Box::new(type_def));UserDefinedTypeName::AddressPayableTypeName( ast)}
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
        if let Some(TargetDefinition::NamespaceDefinition(target) )= ast.expr.target().map(|t|*t) {
            if let Some(idf) = target.names().get(&ast.member.name()) {
                ast.location_expr_base.target = idf.parent().map(|t| Box::new(t.into()));
            }
        } else {
            let mut t = ast.expr.target().unwrap_or_default().annotated_type().type_name;
            if let TypeName::Array(_) = *t {
                assert!(ast.member.name() == "length");
                ast.location_expr_base.target = Some(Box::new(TargetDefinition::IdentifierDeclaration(IdentifierDeclaration::VariableDeclaration(array_length_member.clone()))));
            } else if let TypeName::UserDefinedTypeName(mut t) = *t {
                // assert!(isinstance(t, UserDefinedTypeName));
                if let Some(target) = t.target() {
                    if let Some(idf) = target.names().get(&ast.member.name()) {
                        ast.location_expr_base.target = idf.parent().map(|p|Box::new(p.into()));
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
        let source_t = ast.arr.target().unwrap_or_default().annotated_type().type_name;
        ast.location_expr_base.target =
            Some(Box::new(TargetDefinition::IdentifierDeclaration(IdentifierDeclaration::VariableDeclaration(VariableDeclaration::new(
                vec![],
                source_t.value_type(),
                Identifier::Identifier(IdentifierBase::new(String::new())),
                None,
            )))));
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
