#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use circuit_helper::circuit_helper::CircuitHelper;
use circuit_helper_config::circuit_helper_config::CircuitHelperConfig;
use rccell::RcCell;
use std::collections::{BTreeMap, BTreeSet};
use zkay_ast::ast::{
    identifier::Identifier, ASTFlatten, ConstructorOrFunctionDefinition, FunctionCallExpr,
    FunctionCallExprBaseMutRef, FunctionCallExprBaseProperty, IdentifierExpr, IdentifierExprUnion,
    IntoAST, IntoExpression, IntoStatement, MeExpr, NamespaceDefinition, NumberLiteralExpr, AST,
};
use zkay_config::config::CFG;
use zkay_transaction_crypto_params::params::CryptoParams;
// """
// Compute transitive circuit IO sizes (account for called circuits).

// This is only possible if the IO sizes of called circuits no longer change, which means, that this function has to be called in
// a second pass, after all function bodies are already fully transformed.

// IO sizes include public circuit inputs and outputs as well as the private inputs.

// :param fcts_with_verification: All functions which have a circuit associated with them
// :param cgens: [SIDE EFFECT] A map from function to circuit
// :return
// """
pub fn compute_transitive_circuit_io_sizes(
    fcts_with_verification: &mut Vec<RcCell<ConstructorOrFunctionDefinition>>,
    cgens: &RcCell<BTreeMap<RcCell<ConstructorOrFunctionDefinition>, RcCell<CircuitHelper>>>,
) {
    for fct in fcts_with_verification.iter_mut() {
        let mut glob_keys = BTreeSet::new();
        let mut called_fcts = BTreeSet::new();
        let (trans_in_size, trans_out_size, trans_priv_size) =
            _compute_transitive_circuit_io_sizes(cgens, fct, &mut glob_keys, &mut called_fcts);
        let mut circuit = cgens.borrow_mut().get_mut(&fct).unwrap().clone();
        circuit.borrow_mut().trans_in_size = trans_in_size;
        circuit.borrow_mut().trans_out_size = trans_out_size;
        circuit.borrow_mut().trans_priv_size = trans_priv_size;
        circuit.borrow_mut()._global_keys = RcCell::new(glob_keys);
        circuit.borrow_mut().transitively_called_functions = called_fcts;
    }

    for (fct, circ) in cgens.borrow().iter() {
        if !fct.borrow().requires_verification {
            circ.borrow_mut().trans_out_size = 0;
            circ.borrow_mut().trans_in_size = 0;
            circ.borrow_mut().trans_priv_size = 0;
        }
    }
}

pub fn _compute_transitive_circuit_io_sizes(
    cgens: &RcCell<BTreeMap<RcCell<ConstructorOrFunctionDefinition>, RcCell<CircuitHelper>>>,
    fct: &RcCell<ConstructorOrFunctionDefinition>,
    gkeys: &mut BTreeSet<(Option<ASTFlatten>, CryptoParams)>,
    called_fcts: &mut BTreeSet<RcCell<ConstructorOrFunctionDefinition>>,
) -> (i32, i32, i32) {
    let circuit = cgens.borrow().get(fct).unwrap().clone();
    if circuit.borrow().trans_in_size != 0 {
        //Memoized
        *gkeys = (*gkeys)
            .union(&cgens.borrow()[fct].borrow()._global_keys.borrow())
            .cloned()
            .collect();
        *called_fcts = (*called_fcts)
            .union(&cgens.borrow()[fct].borrow().transitively_called_functions)
            .cloned()
            .collect();
        return (
            circuit.borrow().trans_in_size,
            circuit.borrow().trans_out_size,
            circuit.borrow().trans_priv_size,
        );
    }

    *gkeys = (*gkeys)
        .union(&cgens.borrow()[fct].borrow().requested_global_keys())
        .cloned()
        .collect();
    for call in &*cgens.borrow()[fct]
        .borrow()
        .function_calls_with_verification
        .borrow()
    {
        if let Some(cofd) = call
            .borrow()
            .func()
            .ast_base_ref()
            .unwrap()
            .borrow()
            .target
            .clone()
            .unwrap()
            .upgrade()
            .unwrap()
            .try_as_constructor_or_function_definition_ref()
        {
            called_fcts.insert(cofd.clone());
        }
    }

    if circuit
        .borrow()
        .function_calls_with_verification
        .borrow()
        .is_empty()
    {
        (0, 0, 0)
    } else {
        let (mut insum, mut outsum, mut psum) = (0, 0, 0);
        for f in &*circuit.borrow().function_calls_with_verification.borrow() {
            if let Some(ref mut t) = f
                .borrow()
                .func()
                .ast_base_ref()
                .unwrap()
                .borrow()
                .target
                .clone()
                .unwrap()
                .upgrade()
                .unwrap()
                .try_as_constructor_or_function_definition_ref()
            {
                let (i, o, p) = _compute_transitive_circuit_io_sizes(cgens, t, gkeys, called_fcts);
                if let Some(target_circuit) = cgens.borrow().get(&t) {
                    insum += i + target_circuit.borrow().in_size();
                    outsum += o + target_circuit.borrow().out_size();
                    psum += p + target_circuit.borrow().priv_in_size();
                }
            }
        }
        (insum, outsum, psum)
    }
}
// """
// Add required additional args for public calls to functions which require verification.

// This must be called after compute_transitive_circuit_io_sizes.

// Whenever a function which requires verification is called, the caller needs to pass along the circuit input and output arrays,
// as well as the correct start indices for them, such that the callee deserializes/serializes from/into the correct segment of the
// output/input array. This function thus transforms function calls to functions requiring verification, by adding these additional
// arguments. This must be done in a second pass, after all function bodies in the contract are fully transformed,
// since the correct start indices depend on the circuit IO sizes of the caller function
// (see ZkayTransformer documentation for more information).

// :param fcts_with_verification: [SIDE EFFECT] All functions which have a circuit associated with them
// :param cgens: A map from function to circuit
// """
pub fn transform_internal_calls(
    fcts_with_verification: &Vec<RcCell<ConstructorOrFunctionDefinition>>,
    cgens: &RcCell<BTreeMap<RcCell<ConstructorOrFunctionDefinition>, RcCell<CircuitHelper>>>,
) {
    for fct in fcts_with_verification {
        let mut circuit = cgens.borrow()[&fct].clone();
        let (priv_in_size, in_size, out_size) = (
            circuit.borrow().priv_in_size(),
            circuit.borrow().in_size(),
            circuit.borrow().out_size(),
        );
        let (mut i, mut o, mut p) = (0, 0, 0);
        let zk_in_name = CFG.lock().unwrap().zk_in_name();
        let zk_out_name = CFG.lock().unwrap().zk_out_name();
        for fc in circuit
            .borrow_mut()
            .function_calls_with_verification
            .borrow_mut()
            .iter_mut()
        {
            fc.borrow_mut()
                .function_call_expr_base_mut_ref()
                .sec_start_offset = Some(priv_in_size + p);
            fc.borrow_mut()
                .function_call_expr_base_mut_ref()
                .args
                .extend(vec![
                    RcCell::new(
                        IdentifierExpr::new(IdentifierExprUnion::String(zk_in_name.clone()), None)
                            .into_ast(),
                    )
                    .into(),
                    RcCell::new(
                        IdentifierExpr::new(
                            IdentifierExprUnion::String(format!(
                                "{}_start_idx",
                                zk_in_name.clone()
                            )),
                            None,
                        )
                        .into_expr()
                        .binop(
                            String::from("+"),
                            NumberLiteralExpr::new(in_size + i, false).into_expr(),
                        ),
                    )
                    .into(),
                    RcCell::new(
                        IdentifierExpr::new(IdentifierExprUnion::String(zk_out_name.clone()), None)
                            .into_ast(),
                    )
                    .into(),
                    RcCell::new(
                        IdentifierExpr::new(
                            IdentifierExprUnion::String(format!(
                                "{}_start_idx",
                                zk_out_name.clone()
                            )),
                            None,
                        )
                        .into_expr()
                        .binop(
                            String::from("+"),
                            NumberLiteralExpr::new(out_size + o, false).into_expr(),
                        ),
                    )
                    .into(),
                ]);
            if let Some(t) = fc
                .borrow()
                .func()
                .ast_base_ref()
                .unwrap()
                .borrow()
                .target
                .clone()
                .unwrap()
                .upgrade()
                .unwrap()
                .try_as_constructor_or_function_definition_ref()
            {
                if let Some(cg) = cgens.borrow().get(&t) {
                    i += cg.borrow().in_size_trans();
                    o += cg.borrow().out_size_trans();
                    p += cg.borrow().priv_in_size_trans();
                }
            }
        }
        assert!(
            i == circuit.borrow().trans_in_size
                && o == circuit.borrow().trans_out_size
                && p == circuit.borrow().trans_priv_size
        );
    }
}
