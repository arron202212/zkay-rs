use crate::zkay_ast::ast::{
    BuiltinFunction, Expression, FunctionCallExpr, HybridArgType, HybridArgumentIdf, Identifier,
    IdentifierExpr,
};
use crate::zkay_ast::pointers::symbol_table::SymbolTableLinker;

// K = TypeVar("K")
// V = TypeVar("V")
// class Remapper(Generic[K, V]):
pub struct Remapper<K: Identifier, V: HybridArgumentIdf> {
    rmap: RemapMapType<K, V>,
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

// :param K: name type
// :param V: type of element to which key refers at a code location
// """
impl RemapMapper {
    type RemapMapType = BTreeMap; //#[(bool, K), V]

    pub fn new() -> Self
// super().__init__()
    {
        Self {
            rmap: RemapMapType::new(),
        }
    }

    pub fn boolean(&self) -> bool
// """
        // Check if any name is currently remapped.

        // :return: True if there exists at least one key which is currently remapped to a different value
        // """
    {
        !self.rmap.is_empty()
    }

    pub fn clear(&mut self)
    // """Discard the entire remap state."""
    {
        self.rmap.clear();
    }

    pub fn reset_key(&self, key: K)
    // """Invalidate remapping information for the given key (is_remapped returns false after this)."""
    {
        self.rmap.remove(&key);
    }

    pub fn remap(&self, key: K, value: V)
    // """
    // Remap key to refer to new version element "value".

    // :param key: The key/identifier to update
    // :param value: latest version of the element to which key refers
    // """
    {
        assert!(key.parent().is_some());
        self.rmap(key, value);
    }

    // @contextmanager
    pub fn remap_scope(&mut self, scope_stmt: Option<AST>)
    // """
    // Return a context manager which will automatically rollback the remap state once the end of the with statement is reached.

    // :param scope_stmt: [OPTIONAL] last statement before the scope is entered. If this is not None, remappings for variables which were
    //                               already in scope at scope_stmt will not be reset during rollback
    // :return: context manager
    // """
    {
        let prev = self.rmap.clone();
        // yield
        if let Some(scope_stmt) = scope_stmt {
            prev.update(
                self.rmap
                    .items()
                    .iter()
                    .filter_map(|(key, val)| {
                        if SymbolTableLinker.in_scope_at(key, scope_stmt) {
                            Some((key, val))
                        } else {
                            None
                        }
                    })
                    .collect(),
            );
        }
        self.rmap = prev;
    }

    pub fn is_remapped(&self, key: K) -> bool {
        self.rmap.contains(&key)
    }

    pub fn get_current(&self, key: K, default: Option<V>) -> V
// """
        // Return the value to which key currently refers.

        // :param key: Name to lookup
        // :param default: If set, this will be returned if key is not currently remapped

        // :except KeyError: raised if key not currently mapped and default=None
        // :return: The current value
        // """
    {
        let k = key;
        if let Some(v) = self.rmap.get(&v) {
            v
        } else {
            if default.is_none() {
                assert!(false, "default is none");
            }
            default
        }
    }

    pub fn get_state(&self) -> Any
// """ Return an opaque copy of the internal state. """
    {
        self.rmap.clone()
    }

    pub fn set_state(&mut self, state: Any)
    // """ Restore internal state from an opaque copy previously obtained using get_state. """
    {
        assert!(isinstance(state, BTreeMap));
        self.rmap = state.copy();
    }

    pub fn join_branch(
        &self,
        stmt: AST,
        true_cond_for_other_branch: IdentifierExpr,
        other_branch_state: Any,
        create_val_for_name_and_expr_fct: impl FnOnce(K, Expression) -> V,
    )
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
    {
        let true_state = other_branch_state;
        let false_state = self.rmap;
        self.rmap = BTreeMap::new();

        pub fn join(then_idf: AST, else_idf: AST)
        // """Return new temporary HybridArgumentIdf with value cond ? then_idf : else_idf."""
        {
            let rhs = FunctionCallExpr(
                BuiltinFunction("ite"),
                [true_cond_for_other_branch.clone(), then_idf, else_idf],
            )
            .as_type(val.t);
            return create_val_for_name_and_expr_fct(key.name, rhs);
        }

        for (key, val) in true_state {
            if !SymbolTableLinker.in_scope_at(key, stmt)
            // Don"t keep local values
            {
                continue;
            }

            if false_state.contains(&key) && false_state[&key].name == val.name
            // key was not modified in either branch -> simply keep
            {
                assert!(false_state[&key] == val);
                self.rmap.insert(key, val);
            } else if !false_state.contains(&key)
            // If value was only read (remapping points to a circuit input) -> can just take as-is,
            // otherwise have to use conditional assignment
            {
                if isinstance(val, HybridArgumentIdf)
                    && (val.arg_type == HybridArgType.PUB_CIRCUIT_ARG
                        || val.arg_type == HybridArgType.PrivCircuitVal)
                {
                    self.rmap(key, val);
                } else
                // key was only modified in true branch
                // remap key -> new temporary with value cond ? new_value : old_value
                {
                    let key_decl = key.parent;
                    assert!(key_decl.annotated_type.is_some());
                    let mut prev_val = IdentifierExpr(key.clone())
                        .as_type(key_decl.annotated_type.zkay_type.clone());
                    prev_val.target = key_decl;
                    prev_val.parent = stmt;
                    prev_val.statement = stmt;
                    self.rmap[key] = join(true_state[key].get_idf_expr(stmt), prev_val);
                }
            } else
            // key was modified in both branches
            // remap key -> new temporary with value cond ? true_val : false_val
            {
                self.rmap[key] = join(
                    true_state[key].get_idf_expr(stmt),
                    false_state[key].get_idf_expr(stmt),
                );
            }
        }
        for (key, val) in false_state {
            if !SymbolTableLinker.in_scope_at(key, stmt)
            // Don"t keep local values
            {
                continue;
            }

            if !true_state.contains(&key) {
                if isinstance(val, HybridArgumentIdf)
                    && (val.arg_type == HybridArgType.PUB_CIRCUIT_ARG
                        || val.arg_type == HybridArgType.PrivCircuitVal)
                {
                    self.rmap[key] = val;
                } else
                // key was only modified in false branch
                // remap key -> new temporary with value cond ? old_value : new_value
                {
                    let key_decl = key.parent;
                    assert!(key_decl.annotated_type.is_some());
                    let mut prev_val = IdentifierExpr(key.clone())
                        .as_type(key_decl.annotated_type.zkay_type.clone());
                    prev_val.target = key_decl;
                    prev_val.parent = stmt;
                    prev_val.statement = stmt;
                    self.rmap[key] = join(prev_val, false_state[key].get_idf_expr(stmt));
                }
            }
        }
    }
}
// class CircVarRemapper(Remapper[Identifier, HybridArgumentIdf])
//     """Remapper class used by CircuitHelper"""
//     pass
pub struct CircVarRemapper(pub Remapper<Identifier, HybridArgumentIdf>);
