#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit_helper::CircuitHelper;
use rccell::RcCell;
use std::any::Any;
use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};
use zkay_ast::ast::{
    AST, ASTBaseProperty, ASTBaseRef, ASTFlatten, ASTType, Block, BuiltinFunction, Expression,
    ExpressionASType, ExpressionBaseMutRef, ExpressionBaseProperty, FunctionCallExpr,
    FunctionCallExprBase, HybridArgType, HybridArgumentIdf, IdentifierBaseProperty, IdentifierExpr,
    IdentifierExprUnion, IfStatement, IntoAST, IntoExpression, IntoStatement,
    VariableDeclarationStatement, identifier::Identifier, is_instance,
};
use zkay_ast::pointers::symbol_table::SymbolTableLinker;
// Identifier = TypeVar("Identifier")
// HybridArgumentIdf = TypeVar("HybridArgumentIdf")
// class Remapper(Generic[Identifier, HybridArgumentIdf]):
type RemapMapType = BTreeMap<RcCell<Identifier>, HybridArgumentIdf>; //#[(bool, identifier::Identifier), HybridArgumentIdf]
pub struct WithRemapScope {
    prev: RemapMapType,
    rmap: RcCell<RemapMapType>,
    scope_stmt: Option<ASTFlatten>,
}
impl WithRemapScope {
    pub fn new(rmap: RcCell<RemapMapType>, scope_stmt: Option<ASTFlatten>) -> Self {
        let prev = rmap.borrow().clone();
        Self {
            prev,
            rmap,
            scope_stmt,
        }
    }
}
impl Drop for WithRemapScope {
    fn drop(&mut self) {
        if let Some(scope_stmt) = self.scope_stmt.as_ref() {
            self.prev.append(
                &mut self
                    .rmap
                    .borrow()
                    .clone()
                    .into_iter()
                    .filter(|(key, _)| SymbolTableLinker::in_scope_at(key, scope_stmt))
                    .collect(),
            );
        }
        *self.rmap.borrow_mut() = self.prev.clone();
    }
}
#[derive(Clone)]
pub struct Remapper {
    rmap: RcCell<RemapMapType>,
}
// """
// Helper class to simulate static single assignment, mostly used by CircuitHelper
// For a given name it keeps track which value the name currently refers to (e.g. current SSA identifier)

// e.g. if we have:

//     x = 1
//     x = 2
//     x = x + 1

// we can then simulate ssa by using the remapper whenever an identifier is read or written:

//     tmp1 = 1
//     remap(x, tmp1)
//     tmp2 = 2
//     remap(x, tmp2)
//     tmp3 = get_current(x) + 1
//     remap(x, tmp3)

// :param Identifier: name type
// :param HybridArgumentIdf: type of element to which key refers at a code location
// """
impl Remapper {
    pub fn new() -> Self {
        // super().__init__()

        Self {
            rmap: RcCell::new(RemapMapType::new()),
        }
    }

    // """
    // Check if any name is currently remapped.

    // :return: True if there exists at least one key which is currently remapped to a different value
    // """
    pub fn boolean(&self) -> bool {
        !self.rmap.borrow().is_empty()
    }
    // """Discard the entire remap state."""

    pub fn clear(&self) {
        self.rmap.borrow_mut().clear();
    }
    // """Invalidate remapping information for the given key (is_remapped returns false after this)."""
    pub fn reset_key(&self, key: &RcCell<Identifier>) {
        self.rmap.borrow_mut().remove(key);
    }
    // """
    // Remap key to refer to new version element "value".

    // :param key: The key/identifier to update
    // :param value: latest version of the element to which key refers
    // """
    pub fn remap(&self, key: &RcCell<Identifier>, value: HybridArgumentIdf) {
        // assert!(key.parent().is_some());
        self.rmap.borrow_mut().insert(key.clone(), value);
    }

    // @contextmanager
    // """
    // Return a context manager which will automatically rollback the remap state once the end of the with statement is reached.

    // :param scope_stmt: [OPTIONAL] last statement before the scope is entered. If this is not None, remappings for variables which were
    //                               already in scope at scope_stmt will not be reset during rollback
    // :return: context manager
    // """
    pub fn remap_scope(&self, scope_stmt: Option<&ASTFlatten>) -> WithRemapScope {
        // let mut prev = self.rmap.borrow().clone();
        // // yield    MY TODO
        // if let Some(scope_stmt) = scope_stmt {
        //     prev.append(
        //         &mut self
        //             .rmap
        //             .borrow()
        //             .clone()
        //             .into_iter()
        //             .filter(|(key, _)| SymbolTableLinker::in_scope_at(key, scope_stmt))
        //             .collect(),
        //     );
        // }
        // *self.rmap.borrow_mut() = prev;
        WithRemapScope::new(self.rmap.clone(), scope_stmt.cloned())
    }

    pub fn is_remapped(&self, key: &RcCell<Identifier>) -> bool {
        self.rmap.borrow().contains_key(key)
    }
    // """
    // Return the value to which key currently refers.

    // :param key: Name to lookup
    // :param default: If set, this will be returned if key is not currently remapped

    // :except KeyError: raised if key not currently mapped and default=None
    // :return: The current value
    // """
    pub fn get_current(
        &self,
        key: &RcCell<Identifier>,
        default: Option<HybridArgumentIdf>,
    ) -> HybridArgumentIdf {
        let k = key;
        if let Some(v) = self.rmap.borrow().get(&k) {
            v.clone()
        } else {
            if default.is_none() {
                panic!("default is none");
            }
            default.unwrap()
        }
    }
    // """ Return an opaque copy of the internal state. """

    pub fn get_state(&self) -> RemapMapType {
        self.rmap.borrow().clone()
    }
    // """ Restore internal state from an opaque copy previously obtained using get_state. """
    pub fn set_state(&self, state: &dyn Any) {
        // assert!(isinstance(state, BTreeMap));
        if let Some(state) = state.downcast_ref::<BTreeMap<RcCell<Identifier>, HybridArgumentIdf>>()
        {
            *self.rmap.borrow_mut() = state.clone();
        } else {
            assert!(false);
        }
    }

    // """
    // Perform an SSA join for two branches.

    // | i.e. if key is not remapped in any branch -> keep previous remapping
    // |      if key is altered in at least one branch -> remap to conditional assignment of latest remapped version in either branch

    // :param stmt: the branch statement, variables which are not already in scope at that statement are not included in the joined state
    // :param true_cond_for_other_branch: IdentifierExpression which evaluates to true at runtime if other_branch is taken
    // :param other_branch_state: remap state at the end of other branch (obtained using get_state)
    // :param create_val_for_name_and_expr_fct: function to introduce a new temporary variable to which the given expression is assigned

    // :Example use

    // :

    //     with remapper.remap_scope(persist_globals=False)
    //         <process true branch>
    //         true_state = remapper.get_state()
    //     if <has false branch>
    //         <process false branch>
    //     remapper.join_branch(cond_idf_expr, true_state, <create_tmp_var(idf, expr) function>)
    // """
    pub fn join_branch(
        &self,
        stmt: &ASTFlatten,
        true_cond_for_other_branch: &ASTFlatten,
        other_branch_state: RemapMapType,
        // create_val_for_name_and_expr_fct: impl FnMut(String, Expression) -> HybridArgumentIdf,
        ch: &RcCell<CircuitHelper>,
    ) {
        let true_state = other_branch_state;
        let false_state = self.rmap.borrow().clone();
        self.rmap.borrow_mut().clear();
        // """Return new temporary HybridArgumentIdf with value cond ? then_idf : else_idf."""
        fn join(
            then_idf: &ASTFlatten,
            else_idf: &ASTFlatten,
            key: &ASTFlatten,
            val: &HybridArgumentIdf,
            true_cond_for_other_branch: &ASTFlatten,
            // create_val_for_name_and_expr_fct: impl FnMut(String, Expression) -> HybridArgumentIdf,
            ch: &RcCell<CircuitHelper>,
        ) -> HybridArgumentIdf {
            let rhs = FunctionCallExpr::FunctionCallExpr(FunctionCallExprBase::new(
                RcCell::new(BuiltinFunction::new("ite")).into(),
                vec![
                    true_cond_for_other_branch.clone(),
                    then_idf.clone(),
                    else_idf.clone(),
                ],
                None,
                None,
            ))
            .as_type(&val.t.clone().into());
            // create_val_for_name_and_expr_fct(key.name(), rhs)
            ch.borrow_mut()
                ._create_temp_var(&key.try_as_identifier_ref().unwrap().borrow().name(), &rhs)
        }

        for (key, val) in true_state
            .iter()
            .filter(|(key, _)| SymbolTableLinker::in_scope_at(key, stmt))
        {
            // Don"t keep local values filter

            if false_state.get(key).map_or(false, |v| {
                v.identifier_base.name == val.identifier_base.name
            }) {
                // key was not modified in either branch -> simply keep
                assert!(&false_state[key] == val);
                self.rmap.borrow_mut().insert(key.clone(), val.clone());
            } else if !false_state.contains_key(key) {
                // If value was only read (remapping points to a circuit input) -> can just take as-is,
                // otherwise have to use conditional assignment
                if is_instance(val, ASTType::HybridArgumentIdf)
                    && (val.arg_type == HybridArgType::PubCircuitArg
                        || val.arg_type == HybridArgType::PrivCircuitVal)
                {
                    self.rmap.borrow_mut().insert(key.clone(), val.clone());
                } else {
                    // key was only modified in true branch
                    // remap key -> new temporary with value cond ? new_value : old_value
                    let key_decl = key.borrow().parent();
                    assert!(
                        key_decl
                            .clone()
                            .unwrap()
                            .upgrade()
                            .unwrap()
                            .try_as_expression_ref()
                            .unwrap()
                            .borrow()
                            .annotated_type()
                            .is_some()
                    );
                    let mut prev_val =
                        IdentifierExpr::new(IdentifierExprUnion::Identifier(key.clone()), None)
                            .as_type(
                                &RcCell::new(
                                    key_decl
                                        .clone()
                                        .unwrap()
                                        .upgrade()
                                        .unwrap()
                                        .try_as_expression_ref()
                                        .unwrap()
                                        .borrow()
                                        .annotated_type()
                                        .as_ref()
                                        .unwrap()
                                        .borrow()
                                        .zkay_type(),
                                )
                                .into(),
                            );
                    prev_val
                        .try_as_identifier_expr_mut()
                        .unwrap()
                        .borrow_mut()
                        .ast_base_ref()
                        .borrow_mut()
                        .target = key_decl.clone();
                    prev_val
                        .try_as_identifier_expr_mut()
                        .unwrap()
                        .borrow_mut()
                        .ast_base_ref()
                        .borrow_mut()
                        .parent = Some(stmt.clone().downgrade());
                    prev_val
                        .try_as_identifier_expr_mut()
                        .unwrap()
                        .borrow_mut()
                        .expression_base_mut_ref()
                        .statement = Some(stmt.clone().downgrade());
                    self.rmap.borrow_mut().insert(
                        key.clone(),
                        join(
                            true_state[&key].get_idf_expr(Some(stmt)).as_ref().unwrap(),
                            &prev_val,
                            &key.clone().into(),
                            &val,
                            &true_cond_for_other_branch,
                            // &create_val_for_name_and_expr_fct,
                            ch,
                        ),
                    );
                }
            } else {
                // key was modified in both branches
                // remap key -> new temporary with value cond ? true_val : false_val
                self.rmap.borrow_mut().insert(
                    key.clone(),
                    join(
                        true_state[&key].get_idf_expr(Some(stmt)).as_ref().unwrap(),
                        false_state[&key].get_idf_expr(Some(stmt)).as_ref().unwrap(),
                        &key.clone().into(),
                        &val,
                        &true_cond_for_other_branch,
                        // &create_val_for_name_and_expr_fct,
                        ch,
                    ),
                );
            }
        }
        for (key, val) in false_state.iter().filter(|(key, _)| {
            SymbolTableLinker::in_scope_at(key, stmt) && !true_state.contains_key(key)
        }) {
            // Don"t keep local values filter
            if is_instance(val, ASTType::HybridArgumentIdf)
                && (val.arg_type == HybridArgType::PubCircuitArg
                    || val.arg_type == HybridArgType::PrivCircuitVal)
            {
                self.rmap.borrow_mut().insert(key.clone(), val.clone());
            } else {
                // key was only modified in false branch
                // remap key -> new temporary with value cond ? old_value : new_value
                let key_decl = key.borrow().parent();
                assert!(
                    key_decl
                        .clone()
                        .unwrap()
                        .upgrade()
                        .unwrap()
                        .try_as_expression_ref()
                        .unwrap()
                        .borrow()
                        .annotated_type()
                        .is_some()
                );
                let mut prev_val =
                    IdentifierExpr::new(IdentifierExprUnion::Identifier(key.clone()), None)
                        .as_type(
                            &RcCell::new(
                                key_decl
                                    .clone()
                                    .unwrap()
                                    .upgrade()
                                    .unwrap()
                                    .try_as_expression_ref()
                                    .unwrap()
                                    .borrow()
                                    .annotated_type()
                                    .as_ref()
                                    .unwrap()
                                    .borrow()
                                    .zkay_type(),
                            )
                            .into(),
                        );
                prev_val.ast_base_ref().unwrap().borrow_mut().target = key_decl.clone();
                prev_val.ast_base_ref().unwrap().borrow_mut().parent =
                    Some(stmt.clone().downgrade());
                prev_val
                    .try_as_identifier_expr_mut()
                    .unwrap()
                    .borrow_mut()
                    .location_expr_base
                    .tuple_or_location_expr_base
                    .expression_base
                    .statement = Some(stmt.clone().downgrade());
                self.rmap.borrow_mut().insert(
                    key.clone(),
                    join(
                        &prev_val,
                        false_state[&key].get_idf_expr(Some(stmt)).as_ref().unwrap(),
                        &key.clone().into(),
                        &val,
                        &true_cond_for_other_branch,
                        // &create_val_for_name_and_expr_fct,
                        ch,
                    ),
                );
            }
        }
    }
}
// class CircVarRemapper(Remapper[Identifier, HybridArgumentIdf])
//     """Remapper class used by CircuitHelper"""
//     pass
#[derive(Clone)]
pub struct CircVarRemapper(pub Remapper);
impl CircVarRemapper {
    pub fn new() -> Self {
        // println!("=====CircVarRemapper==before=={}=", line!());
        Self(Remapper::new())
    }
}
