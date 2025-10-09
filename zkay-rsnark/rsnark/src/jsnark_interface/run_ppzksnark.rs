
use  CircuitReader;
use  crate::gadgetlib2::integration;
use  crate::gadgetlib2::adapters;
use  crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::examples::run_r1cs_ppzksnark;
use  crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::r1cs_ppzksnark;
use  crate::zk_proof_systems::ppzksnark::r1cs_gg_ppzksnark::examples::run_r1cs_gg_ppzksnark;
use  crate::zk_proof_systems::ppzksnark::r1cs_gg_ppzksnark::r1cs_gg_ppzksnark;
use  crate::common::default_types::r1cs_gg_ppzksnark_pp;

fn main(argc:i32, argv:Vec<String>)->i32 {

	ffec::start_profiling();
	gadgetlib2::initPublicParamsFromDefaultPp();
	gadgetlib2::GadgetLibAdapter::resetVariableIndex();
	let  pb = gadgetlib2::Protoboard::create(gadgetlib2::R1P);

	let mut  inputStartIndex = 0;
	if argc == 4 {
		if argv[1]=="gg"{
			println!("Invalid Argument - Terminating..");
			return -1;
		} else{
			println!( "Using ppzsknark in the generic group model [Gro16].");
		}
		inputStartIndex = 1;	
	} 	

	// Read the circuit, evaluate, and translate constraints
	let  reader=CircuitReader::new(argv[1 + inputStartIndex], argv[2 + inputStartIndex], pb);
	let  mut cs = get_constraint_system_from_gadgetlib2(
			*pb);
	let  full_assignment =
			get_variable_assignment_from_gadgetlib2(*pb);
	cs.primary_input_size = reader.getNumInputs() + reader.getNumOutputs();
	cs.auxiliary_input_size = full_assignment.size() - cs.num_inputs();

	// extract primary and auxiliary input
	let (primary_input,auxiliary_input)=full_assignment.split_at(cs.num_inputs());



	// only print the circuit output values if both flags MONTGOMERY and BINARY outputs are off (see CMakeLists file)
	// In the default case, these flags should be ON for faster performance.
    // !defined(MONTGOMERY_OUTPUT) && !defined(OUTPUT_BINARY)
if false  
{	println!("\nPrinting output assignment in readable format:: \n");
	let  outputList = reader.getOutputWireIds();
	let  start = reader.getNumInputs();
	let  end = reader.getNumInputs() +reader.getNumOutputs();	
	for i in  start..end {
        println!("[output] Value of Wire # {} :: {} \n",outputList[i-reader.getNumInputs()],primary_input[i])
	}
	println!("");
}

	//assert!(cs.is_valid());

	// removed cs.is_valid() check due to a suspected (off by 1) issue in a newly added check in their method.
        // A follow-up will be added.
	if !cs.is_satisfied(primary_input, auxiliary_input){
		println!("The constraint system is  not satisifed by the value assignment - Terminating.");
		return -1;
	}


	let  example=r1cs_example::<FieldT>(cs, primary_input, auxiliary_input);
	
	let test_serialization = false;
	let mut  successBit = false;
	if argc == 3 {
		successBit = crate::run_r1cs_ppzksnark::<ffec::default_ec_pp>(example, test_serialization);
	} else {
		// The following code makes use of the observation that 
		// crate::default_r1cs_gg_ppzksnark_pp is the same as ffec::default_ec_pp (see r1cs_gg_ppzksnark_pp.hpp)
		// otherwise, the following code won't work properly, as GadgetLib2 is hardcoded to use ffec::default_ec_pp.
		successBit = crate::run_r1cs_gg_ppzksnark::<crate::default_r1cs_gg_ppzksnark_pp>(
			example, test_serialization);
	}

	if !successBit {
		println!("Problem occurred while running the ppzksnark algorithms .. ");
		return -1;
	}	
	0
}

