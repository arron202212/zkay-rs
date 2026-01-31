// use crate::common::default_types::r1cs_gg_ppzksnark_pp;
// use crate::gadgetlib2::adapters;
// use crate::gadgetlib2::integration;
use crate::gadgetlib2::adapters::GLA;
use crate::jsnark_interface::circuit_reader::{CircuitReader, FieldT};
use crate::relations::constraint_satisfaction_problems::r1cs::examples::r1cs_examples::r1cs_example;
// use crate::zk_proof_systems::ppzksnark::r1cs_gg_ppzksnark::examples::run_r1cs_gg_ppzksnark;
// use crate::zk_proof_systems::ppzksnark::r1cs_gg_ppzksnark::r1cs_gg_ppzksnark;
// use crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::r1cs_ppzksnark;
use crate::common::default_types::r1cs_gg_ppzksnark_pp::default_r1cs_gg_ppzksnark_pp;
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable};
use crate::gadgetlib2::adapters::GadgetLibAdapter;
use crate::gadgetlib2::integration::{
    get_constraint_system_from_gadgetlib2, get_variable_assignment_from_gadgetlib2,
};
use crate::gadgetlib2::pp::initPublicParamsFromDefaultPp;
use crate::gadgetlib2::protoboard::Protoboard;
use crate::gadgetlib2::variable::FieldType;
use crate::zk_proof_systems::ppzksnark::r1cs_gg_ppzksnark::examples::run_r1cs_gg_ppzksnark::run_r1cs_gg_ppzksnark;
use crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::examples::run_r1cs_ppzksnark::run_r1cs_ppzksnark;
use ff_curves::default_ec_pp;
use ffec::common::profiling::start_profiling;

fn main(argc: i32, argv: Vec<String>) -> i32 {
    start_profiling();
    initPublicParamsFromDefaultPp();
    <GLA as GadgetLibAdapter>::resetVariableIndex();
    let pb = Protoboard::create(FieldType::R1P, None);

    let mut inputStartIndex = 0;
    if argc == 4 {
        if argv[1] == "gg" {
            println!("Invalid Argument - Terminating..");
            return -1;
        } else {
            println!("Using ppzsknark in the generic group model [Gro16].");
        }
        inputStartIndex = 1;
    }

    // Read the circuit, evaluate, and translate constraints
    let reader = CircuitReader::new(
        &argv[1 + inputStartIndex],
        &argv[2 + inputStartIndex],
        pb.clone(),
    );
    let mut cs = get_constraint_system_from_gadgetlib2(&pb.as_ref().unwrap().borrow());
    let full_assignment = get_variable_assignment_from_gadgetlib2::<
        pb_variable,
        pb_linear_combination,
    >(&pb.as_ref().unwrap().borrow());
    cs.primary_input_size = (reader.getNumInputs() + reader.getNumOutputs()) as usize;
    cs.auxiliary_input_size = full_assignment.len() - cs.num_inputs() as usize;

    // extract primary and auxiliary input
    let (primary_input, auxiliary_input) = full_assignment.split_at(cs.num_inputs());
    let (primary_input, auxiliary_input) = (primary_input.to_vec(), auxiliary_input.to_vec());
    // only print the circuit output values if both flags MONTGOMERY and BINARY outputs are off (see CMakeLists file)
    // In the default case, these flags should be ON for faster performance.
    // !defined(MONTGOMERY_OUTPUT) && !defined(OUTPUT_BINARY)
    if false {
        println!("\nPrinting output assignment in readable format:: \n");
        let outputList = reader.getOutputWireIds();
        let start = reader.getNumInputs() as usize;
        let end = (reader.getNumInputs() + reader.getNumOutputs()) as usize;
        for i in start..end {
            println!(
                "[output] Value of Wire # {} :: {} \n",
                outputList[i - reader.getNumInputs() as usize],
                primary_input[i]
            )
        }
        println!("");
    }

    //assert!(cs.is_valid());

    // removed cs.is_valid() check due to a suspected (off by 1) issue in a newly added check in their method.
    // A follow-up will be added.
    if !cs.is_satisfied(&primary_input, &auxiliary_input) {
        println!("The constraint system is  not satisifed by the value assignment - Terminating.");
        return -1;
    }

    let example = r1cs_example::<FieldT, pb_variable, pb_linear_combination>::new(
        cs,
        primary_input,
        auxiliary_input,
    );

    let test_serialization = false;
    let mut successBit = false;
    if argc == 3 {
        successBit = run_r1cs_ppzksnark::<default_ec_pp>(&example, test_serialization);
    } else {
        // The following code makes use of the observation that
        // crate::default_r1cs_gg_ppzksnark_pp is the same as default_ec_pp (see r1cs_gg_ppzksnark_pp.hpp)
        // otherwise, the following code won't work properly, as GadgetLib2 is hardcoded to use default_ec_pp.
        successBit =
            run_r1cs_gg_ppzksnark::<default_r1cs_gg_ppzksnark_pp>(&example, test_serialization);
    }

    if !successBit {
        println!("Problem occurred while running the ppzksnark algorithms .. ");
        return -1;
    }
    0
}
