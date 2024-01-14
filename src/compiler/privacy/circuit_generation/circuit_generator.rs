// import functools
// import os
// from abc import ABCMeta, abstractmethod
// from multiprocessing import Pool, Value
// from typing import List, Tuple

use crate::compiler::privacy::circuit_generation::circuit_helper::CircuitHelper;
use crate::compiler::privacy::proving_scheme::proving_scheme::{ProvingScheme, VerifyingKey};
use crate::config::{CFG, zk_print};
use crate::utils::progress_printer::print_step;
use crate::utils::timer::time_measure;

extern crate num_cpus;
lazy_static!{
 pub static ref  finish_counter:Mutex<i32>=Mutex::new(0);
pub static ref  c_count:Mutex<i32>=Mutex::new(0);
}

// class CircuitGenerator(metaclass=ABCMeta)
pub struct  CircuitGenerator{
circuits:BTreeMap<ConstructorOrFunctionDefinition,CircuitHelper>,
circuits_to_prove: Vec<CircuitHelper>,
proving_scheme: ProvingScheme, 
output_dir: String,
 parallel_keygen: bool,
p_count:i32
}

impl CircuitGenerator{
    // """
    // A circuit generator takes an abstract circuit representation and turns it into a concrete zk-snark circuit.

    // It also handles prover/verification key generation and parsing, and generates the verification contracts using the supplied
    // proving scheme.
    // """

    pub fn new(circuits: Vec<CircuitHelper>, proving_scheme: ProvingScheme, output_dir: String, parallel_keygen: bool)
        // """
        // Create a circuit generator instance

        // :param circuits: list which contains the corresponding circuit helper for every function in the contract which requires verification
        // :param proving_scheme: the proving scheme instance to be used for verification contract generation
        // :param output_dir: base directory where the zkay compilation output is located
        // :param parallel_keygen: if true, multiple python processes are used to generate keys in parallel
        // """
        let circuits_to_prove = circuits.iter().filter_map(|c| if c.requires_verification() && c.fct.can_be_external && c.fct.has_side_effects{Some(c)}else{None}).collect();
        let p_count=(circuits_to_prove.len() as i32).min(num_cpus::get());
        {Self{circuits :circuits.iter().map(|circ| (circ.fct.clone(), circ.clone())).collect(),
        circuits_to_prove,
        proving_scheme,
        output_dir,
         parallel_keygen,
        p_count,
}}

    pub fn generate_circuits(self, import_keys: bool)
        // """
        // Generate circuit code and verification contracts based on the provided circuits and proving scheme.

        // :param import_keys: if false, new verification and prover keys will be generated, otherwise key files for all verifiers
        //                     are expected to be already present in the respective output directories
        // """
        //Generate proof circuit code

        //Compile circuits
      {  let c_count = self.circuits_to_prove.len();
        zk_print("Compiling {c_count} circuits...");

        let gen_circs = functools.partial(self._generate_zkcircuit, import_keys);
        // with 
        time_measure("circuit_compilation", True);
            {if cfg.is_unit_test
                {modified = list(map(gen_circs, self.circuits_to_prove))}
            else
               { with Pool(processes=self.p_count) as pool
                    modified = pool.map(gen_circs, self.circuits_to_prove)}}

        if import_keys
           { for path in self.get_all_key_paths()
                {if not os.path.exists(path)
                    {raise RuntimeError("Zkay contract import failed: Missing keys")}}}
        else
           { modified_circuits_to_prove = [circ for t, circ in zip(modified, self.circuits_to_prove)
                                          if t or not all(map(os.path.exists, self._get_vk_and_pk_paths(circ)))];

            //Generate keys in parallel
            zk_print(f"Generating keys for {c_count} circuits...");
            time_measure("key_generation", True);
                {if self.parallel_keygen and not cfg.is_unit_test
                   { counter = Value("i", 0);
                    with Pool(processes=self.p_count, initializer=self.__init_worker, initargs=(counter, c_count,)) as pool
                        {pool.map(self._generate_keys_par, modified_circuits_to_prove);}}
                else
                    {for circ in modified_circuits_to_prove
                        {self._generate_keys(circ)}}}}

        print_step("Write verification contracts");
            {for circuit in self.circuits_to_prove
                {vk = self._parse_verification_key(circuit);
                pk_hash = self._get_prover_key_hash(circuit);
                with open(os.path.join(self.output_dir, circuit.verifier_contract_filename), "w") as f;
                    {primary_inputs = self._get_primary_inputs(circuit);
                    f.write(self.proving_scheme.generate_verification_contract(vk, circuit, primary_inputs, pk_hash));}}}
}
    pub fn get_all_key_paths(self) -> Vec<String>
        // """Return paths of all key files for this contract."""
        {let paths = vec![];
        for circuit in self.circuits_to_prove
           { paths.extend(self._get_vk_and_pk_paths(circuit));}
        paths}

    pub fn get_verification_contract_filenames(self) -> Vec<String>
        // """Return file paths for all verification contracts generated by this CircuitGenerator"""
        {[os.path.join(self.output_dir, circuit.verifier_contract_filename) for circuit in self.circuits_to_prove]}

    // @staticmethod
    pub fn __init_worker(counter, total_count)
       {
        finish_counter.lock().unwrap()=counter;
        c_count.lock().unwrap() = total_count;}

    pub fn _generate_keys_par(self, circuit: CircuitHelper)
       { self._generate_keys(circuit);
       
             finish_counter.lock().unwrap() += 1;
            zk_print(r#"Generated keys for circuit "\"{}\" [{}/{c_count}]"#,circuit.verifier_contract_type.code(),finish_counter.value);}

    pub fn _get_circuit_output_dir(self, circuit: CircuitHelper)
        // """Return the output directory for an individual circuit"""
        { self.output_dir.join(CFG.lock().unwrap().get_circuit_output_dir_name(circuit.get_verification_contract_name()))}

    pub fn _get_vk_and_pk_paths(self, circuit: CircuitHelper) -> Vec<String>
        // """Return a tuple which contains the paths to the verification and prover key files."""
       {let  output_dir = self._get_circuit_output_dir(circuit);
         self.get_vk_and_pk_filenames().iter().map(|fname|output_dir.join(fname).to_string()).collect()}

    // @abstractmethod
    pub fn _generate_zkcircuit(self, import_keys: bool, circuit: CircuitHelper) -> bool
        // """
        // Generate code and compile a single circuit.

        // When implementing a new backend, this function should generate a concrete circuit representation, which has
        // a) circuit IO corresponding to circuit.sec_idfs/output_idfs/input_idfs
        // b) logic corresponding to the non-CircCall statements in circuit.phi
        // c) a), b) and c) for the circuit associated with the target function for every CircCall statement in circuit.phi

        // The output of this function should be in a state where key generation can be invoked immediately without further transformations
        // (i.e. any intermediary compilation steps should also happen here).

        // It should be stored in self._get_circuit_output_dir(circuit)

        // :return: True if the circuit was modified since last generation (need to generate new keys)
        // """
        // pass
        {false}

    // @abstractmethod
    pub fn _generate_keys(self, circuit: CircuitHelper);
        // """Generate prover and verification keys for the circuit stored in self._get_circuit_output_dir(circuit)."""
        // pass

    // @classmethod
    // @abstractmethod
    pub fn get_vk_and_pk_filenames() -> Vec<String>;
        // pass

    // @abstractmethod
    pub fn _parse_verification_key(self, circuit: CircuitHelper) -> VerifyingKey
        // """Parse the generated verificaton key file and return a verification key object compatible with self.proving_scheme"""
        { self.proving_scheme.VerifyingKey.create_dummy_key()}

    // @abstractmethod
    pub fn _get_prover_key_hash(self, circuit: CircuitHelper) -> bytes;
        // pass

    pub fn _get_primary_inputs(self, circuit: CircuitHelper) -> Vec<String>
        // """
        // Return list of all public input locations
        // :param circuit: abstract circuit representation
        // :return: list of location strings, a location is either an identifier name or an array index
        // """
{
        inputs = circuit.public_arg_arrays;

        if cfg.should_use_hash(circuit)
            return {[self.proving_scheme.hash_var_name]}
        else
           { primary_inputs = [];
            for name, count in inputs
               { primary_inputs += [f"{name}[{i}]" for i in range(count)]}
            return primary_inputs}}
}