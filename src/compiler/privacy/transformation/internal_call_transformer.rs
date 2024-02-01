use crate::compiler::privacy::circuit_generation::circuit_helper::CircuitHelper;
use crate::config::CFG;
use crate::transaction::crypto::params::CryptoParams;
use crate::zkay_ast::ast::{
    ASTCode, ConstructorOrFunctionDefinition, FunctionCallExpr, Identifier, IdentifierExpr,
    IdentifierExprUnion, MeExpr, NamespaceDefinition, NumberLiteralExpr, TargetDefinition,
};
use std::collections::{BTreeMap, BTreeSet};
pub fn compute_transitive_circuit_io_sizes(
    fcts_with_verification: &mut Vec<ConstructorOrFunctionDefinition>,
    cgens: &mut BTreeMap<ConstructorOrFunctionDefinition, CircuitHelper>,
)
// """
// Compute transitive circuit IO sizes (account for called circuits).

// This is only possible if the IO sizes of called circuits no longer change, which means, that this function has to be called in
// a second pass, after all function bodies are already fully transformed.

// IO sizes include public circuit inputs and outputs as well as the private inputs.

// :param fcts_with_verification: All functions which have a circuit associated with them
// :param cgens: [SIDE EFFECT] A map from function to circuit
// :return
// """
{
    for fct in fcts_with_verification.iter_mut() {
        let mut glob_keys = BTreeSet::new();
        let mut called_fcts = BTreeSet::new();
        let (trans_in_size, trans_out_size, trans_priv_size) =
            _compute_transitive_circuit_io_sizes(cgens, fct, &mut glob_keys, &mut called_fcts);
        let mut circuit = cgens.get_mut(&fct).unwrap();
        circuit.trans_in_size = trans_in_size;
        circuit.trans_out_size = trans_out_size;
        circuit.trans_priv_size = trans_priv_size;
        circuit._global_keys = glob_keys;
        circuit.transitively_called_functions = called_fcts;
    }

    for (fct, circ) in cgens.iter_mut() {
        if !fct.requires_verification {
            circ.trans_out_size = 0;
            circ.trans_in_size = 0;
            circ.trans_priv_size = 0;
        }
    }
}

pub fn _compute_transitive_circuit_io_sizes(
    cgens: &mut BTreeMap<ConstructorOrFunctionDefinition, CircuitHelper>,
    fct: &ConstructorOrFunctionDefinition,
    gkeys: &mut BTreeSet<((Option<MeExpr>, Option<Identifier>), CryptoParams)>,
    called_fcts: &mut BTreeSet<ConstructorOrFunctionDefinition>,
) -> (i32, i32, i32) {
    let circuit = cgens.get(fct).unwrap();
    if circuit.trans_in_size != 0 {
        //Memoized
        *gkeys = (*gkeys).union(&cgens[fct]._global_keys).cloned().collect();
        *called_fcts = (*called_fcts)
            .union(&cgens[fct].transitively_called_functions)
            .cloned()
            .collect();
        return (
            circuit.trans_in_size,
            circuit.trans_out_size,
            circuit.trans_priv_size,
        );
    }

    *gkeys = (*gkeys)
        .union(&cgens[fct].requested_global_keys())
        .cloned()
        .collect();
    for call in &cgens[fct].function_calls_with_verification {
        if let Some(TargetDefinition::NamespaceDefinition(
            NamespaceDefinition::ConstructorOrFunctionDefinition(cofd),
        )) = call.func().unwrap().target().map(|t| *t)
        {
            called_fcts.insert(cofd.clone());
        }
    }

    if circuit.function_calls_with_verification.is_empty() {
        (0, 0, 0)
    } else {
        let (mut insum, mut outsum, mut psum) = (0, 0, 0);
        for f in &circuit.function_calls_with_verification.clone() {
            if let Some(TargetDefinition::NamespaceDefinition(
                NamespaceDefinition::ConstructorOrFunctionDefinition(ref mut t),
            )) = f.func().unwrap().target().map(|t| *t)
            {
                let (i, o, p) = _compute_transitive_circuit_io_sizes(cgens, t, gkeys, called_fcts);
                if let Some(target_circuit) = cgens.get(&*t) {
                    insum += i + target_circuit.in_size();
                    outsum += o + target_circuit.out_size();
                    psum += p + target_circuit.priv_in_size();
                }
            }
        }
        (insum, outsum, psum)
    }
}

pub fn transform_internal_calls(
    fcts_with_verification: &mut Vec<ConstructorOrFunctionDefinition>,
    cgens: &mut BTreeMap<ConstructorOrFunctionDefinition, CircuitHelper>,
)
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
{
    for fct in fcts_with_verification {
        let mut circuit = cgens[&fct].clone();
        let (priv_in_size, in_size, out_size) = (
            circuit.priv_in_size(),
            circuit.in_size(),
            circuit.out_size(),
        );
        let (mut i, mut o, mut p) = (0, 0, 0);
        for fc in circuit.function_calls_with_verification.iter_mut() {
            if let FunctionCallExpr::FunctionCallExpr(ref mut fc) = fc {
                fc.sec_start_offset = Some(priv_in_size + p);
                fc.args.extend(vec![
                    IdentifierExpr::new(
                        IdentifierExprUnion::String(CFG.lock().unwrap().zk_in_name()),
                        None,
                    )
                    .to_expr(),
                    IdentifierExpr::new(
                        IdentifierExprUnion::String(format!(
                            "{}_start_idx",
                            CFG.lock().unwrap().zk_in_name()
                        )),
                        None,
                    )
                    .to_expr()
                    .binop(
                        String::from("+"),
                        NumberLiteralExpr::new(in_size + i, false).to_expr(),
                    )
                    .to_expr(),
                    IdentifierExpr::new(
                        IdentifierExprUnion::String(CFG.lock().unwrap().zk_out_name()),
                        None,
                    )
                    .to_expr(),
                    IdentifierExpr::new(
                        IdentifierExprUnion::String(format!(
                            "{}_start_idx",
                            CFG.lock().unwrap().zk_out_name()
                        )),
                        None,
                    )
                    .to_expr()
                    .binop(
                        String::from("+"),
                        NumberLiteralExpr::new(out_size + o, false).to_expr(),
                    )
                    .to_expr(),
                ]);
                if let Some(TargetDefinition::NamespaceDefinition(
                    NamespaceDefinition::ConstructorOrFunctionDefinition(t),
                )) = fc.func.target().map(|t| *t)
                {
                    if let Some(cg) = cgens.get(&t) {
                        i += cg.in_size_trans();
                        o += cg.out_size_trans();
                        p += cg.priv_in_size_trans();
                    }
                }
            }
        }
        assert!(
            i == circuit.trans_in_size
                && o == circuit.trans_out_size
                && p == circuit.trans_priv_size
        );
    }
}
