
use num_bigint::BigUint;
pub struct Config{
  field_prime:String;
             log2_field_prime:u64;
             libsnark_exec:String;
 running_multi_generators:bool;
	 hex_output_enabled:bool;
	 output_verbose:bool;
	 debug_verbose:bool;
	 print_stack_trace_at_warnings:bool;
}

impl Config {

	pub fn new()->Self {
            let config_dir=".";
            let mut c = std::fs::read_to_string(config_dir.join("config.properties")).unwrap();
			let mut m=std::collections::HashMap::new();
            for item in c.lines(){
                let v:Vec<_>=item.split("=").collect();
                m.insert(v[0].to_owned(),v[1].to_owned());
            }
            let field_prime=m.get("FIELD_PRIME").unwrap();
            let log2_field_prime=BigUint::parse_bytes(field_prime.as_bytes(),10).unwrap().bits();
            let libsnark_exec = m.get("PATH_TO_LIBSNARK_EXEC").unwrap();
let running_multi_generators = m.get("RUNNING_GENERATORS_IN_PARALLEL").unwrap().equals("0");
	let hex_output_enabled = m.get("PRINT_HEX").unwrap().equals("1");
	let output_verbose = m.get("OUTPUT_VERBOSE").unwrap().equals("1");
	let debug_verbose = m.get("DEBUG_VERBOSE").unwrap().equals("1");
	let print_stack_trace_at_warnings = false;
        Self{
  field_prime,
             log2_field_prime,
             libsnark_exec,
 running_multi_generators,
	 hex_output_enabled,
	 output_verbose,
	 debug_verbose,
	 print_stack_trace_at_warnings,
    }
	}


	

	
}
