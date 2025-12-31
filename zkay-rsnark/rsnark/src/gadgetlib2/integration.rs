// use ffec::common::default_types::ec_pp;
use super::pp::{Fr, default_ec_pp};
// use crate::gadgetlib2::protoboard;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::{
    r1cs_constraint, r1cs_constraint_system, r1cs_variable_assignment,
};

// use crate::gadgetlib2::adapters;
// use crate::gadgetlib2::integration;
use crate::gadgetlib2::adapters::{ConvertConfig, GLA, GadgetLibAdapter, linear_combination_t};
use crate::gadgetlib2::protoboard::Protoboard;
use crate::relations::variable::{
    SubLinearCombinationConfig, SubVariableConfig, linear_combination, variable,
};
use ffec::FieldTConfig;
// type FieldT = Fr<default_ec_pp>;
// type GLA = GadgetLibAdapter;
pub fn convert_gadgetlib2_linear_combination<
    FieldT: FieldTConfig,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
>(
    lc: &linear_combination_t,
) -> linear_combination<FieldT, SV, SLC> {
    let mut result = lc.1.clone().into_lc::<FieldT, SV, SLC>() * variable::<FieldT, SV>::from(0);
    for lt in &lc.0 {
        result = result
            + lt.1.clone().into_lc::<FieldT, SV, SLC>()
                * variable::<FieldT, SV>::from(lt.0 as usize + 1);
    }

    return result;
}

pub fn get_constraint_system_from_gadgetlib2<
    FieldT: FieldTConfig,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
>(
    pb: &Protoboard,
) -> r1cs_constraint_system<FieldT, SV, SLC> {
    let mut result = r1cs_constraint_system::<FieldT, SV, SLC>::default();
    // let adapter = GLA::new();

    let converted_pb = GLA::convert(pb);
    let num_constraints = converted_pb.0.len();
    result.constraints.resize(
        num_constraints,
        r1cs_constraint::<FieldT, SV, SLC>::default(),
    );
    print!("Num constraints: {}\n", num_constraints);

    // #ifdef MULTICORE
    // #pragma omp parallel default(none) shared(converted_pb, result), firstprivate(num_constraints)
    //   {
    // #pragma omp single nowait
    //     {
    //#endif
    for i in 0..num_constraints {
        let constr = &converted_pb.0[i];
        // #ifdef MULTICORE
        // #pragma omp task default(none) shared(result, constr, i)
        //         {
        //#endif
        result.constraints[i].a = convert_gadgetlib2_linear_combination(&constr.0);
        result.constraints[i].b = convert_gadgetlib2_linear_combination(&constr.1);
        result.constraints[i].c = convert_gadgetlib2_linear_combination(&constr.2);
    }
    // #ifdef MULTICORE
    //       }
    //     }
    // #pragma omp taskwait
    //   }
    //#endif

    //The number of variables is the highest index created.
    //TODO: If there are multiple protoboards, or variables not assigned to a protoboard, then getNextFreeIndex() is *not* the number of variables! See also in get_variable_assignment_from_gadgetlib2.
    let num_variables = GLA::getNextFreeIndex();
    result.primary_input_size = pb.numInputs();
    result.auxiliary_input_size = num_variables - pb.numInputs();
    return result;
}

pub fn get_variable_assignment_from_gadgetlib2<
    FieldT: FieldTConfig,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
>(
    pb: &Protoboard,
) -> r1cs_variable_assignment<FieldT> {
    // type FieldT=Fr<default_ec_pp>;
    // type GLA=gadgetlib2::GadgetLibAdapter;

    //The number of variables is the highest index created. This is also the required size for the assignment vector.
    //TODO: If there are multiple protoboards, or variables not assigned to a protoboard, then getNextFreeIndex() is *not* the number of variables! See also in get_constraint_system_from_gadgetlib2.
    let num_vars = GLA::getNextFreeIndex();
    // let adapter = GLA;
    let mut result = vec![FieldT::default(); num_vars];
    let assignment = pb.assignment();

    //Go over all assigned values of the protoboard, from every variable-value pair, put the value in the variable.index place of the new assignment.
    for iter in assignment {
        result[GLA::getVariableIndex(iter.0)] = GLA::convert(iter.1);
    }

    return result;
}
