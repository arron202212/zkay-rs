use crate::compiler::privacy::circuit_generation::circuit_helper::CircuitHelper;
use crate::config::CFG;
use crate::zkay_ast::ast::{ConstructorOrFunctionDefinition, IdentifierExpr, NumberLiteralExpr};

pub fn compute_transitive_circuit_io_sizes(
    fcts_with_verification: Vec<ConstructorOrFunctionDefinition>,
    cgens: BTreeMap<ConstructorOrFunctionDefinition, CircuitHelper>,
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
    for fct in fcts_with_verification {
        glob_keys = OrderedDict();
        called_fcts = OrderedDict();
        circuit = cgens[fct];
        let (trans_in_size, trans_out_size, trans_priv_size) =
            _compute_transitive_circuit_io_sizes(cgens, fct, glob_keys, called_fcts);
        circuit.trans_in_size = trans_in_size;
        circuit.trans_out_size = trans_out_size;
        circuit.trans_priv_size = trans_priv_size;
        circuit._global_keys = glob_keys;
        circuit.transitively_called_functions = called_fcts;
    }

    for (fct, circ) in cgens {
        if !fct.requires_verification {
            circ.trans_out_size = 0;
            circ.trans_in_size = 0;
            circ.trans_priv_size = 0;
        }
    }
}

pub fn _compute_transitive_circuit_io_sizes(
    cgens: BTreeMap<ConstructorOrFunctionDefinition, CircuitHelper>,
    fct: ConstructorOrFunctionDefinition,
    gkeys: BTreeSet<((Option<MeExpr>, Option<Identifier>), CryptoParams)>,
    called_fcts: BTreeSet<ConstructorOrFunctionDefinition>,
) {
    let circuit = cgens[fct].clone();
    if circuit.trans_in_size.is_some() {
        //Memoized
        gkeys.append(cgens[fct]._global_keys);
        called_fcts.append(cgens[fct].transitively_called_functions);
        return (
            circuit.trans_in_size,
            circuit.trans_out_size,
            circuit.trans_priv_size,
        );
    }

    gkeys.append(cgens[fct].requested_global_keys);
    for call in cgens[fct].function_calls_with_verification {
        called_fcts[call.func.target] = None;
    }

    if !circuit.function_calls_with_verification {
        return (0, 0, 0);
    } else {
        let (mut insum, mut outsum, mut psum) = (0, 0, 0);
        for f in circuit.function_calls_with_verification {
            let (i, o, p) =
                _compute_transitive_circuit_io_sizes(cgens, f.func.target, gkeys, called_fcts);
            target_circuit = cgens[f.func.target];
            insum += i + target_circuit.in_size;
            outsum += o + target_circuit.out_size;
            psum += p + target_circuit.priv_in_size;
        }
        return (insum, outsum, psum);
    }
}

pub fn transform_internal_calls(
    fcts_with_verification: Vec<ConstructorOrFunctionDefinition>,
    cgens: BTreeMap<ConstructorOrFunctionDefinition, CircuitHelper>,
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
        let circuit = cgens[fct].clone();
        let (mut i, mut o, mut p) = (0, 0, 0);
        for fc in &circuit.function_calls_with_verification {
            let fdef = fc.func.target.clone();
            fc.sec_start_offset = circuit.priv_in_size + p;
            fc.args += [
                IdentifierExpr(cfg.zk_in_name),
                IdentifierExpr(format!("{cfg.zk_in_name}_start_idx"))
                    .binop('+', NumberLiteralExpr(circuit.in_size + i)),
                IdentifierExpr(cfg.zk_out_name),
                IdentifierExpr(format!("{cfg.zk_out_name}_start_idx"))
                    .binop('+', NumberLiteralExpr(circuit.out_size + o)),
            ];
            i += cgens[fdef].in_size_trans;
            o += cgens[fdef].out_size_trans;
            p += cgens[fdef].priv_in_size_trans;
        }
        assert! (i == circuit.trans_in_size and o == circuit.trans_out_size and p == circuit.trans_priv_size);
    }
}
