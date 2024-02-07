// import functools
// import os
// from abc import ABCMeta, abstractmethod
// from multiprocessing import Pool, Value
// from typing import List, Tuple
use circuit_generation::circuit_helper::CircuitHelper;
use proving_scheme_backends::{
    gm17::ProvingSchemeGm17, groth16::ProvingSchemeGroth16,
};
use proving_scheme::{ProvingScheme, VerifyingKeyMeta};
use zkay_utils::progress_printer::print_step;
use zkay_utils::timer::time_measure;
use zkay_ast::ast::ConstructorOrFunctionDefinition;
use zkay_ast::ast::IntoAST;
use zkay_config::{config::CFG, zk_print};
use rayon::prelude::*;
use std::path::{Path, PathBuf};
extern crate num_cpus;
use lazy_static::lazy_static;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;
pub enum VerifyingKeyType {
    ProvingSchemeGroth16(<ProvingSchemeGroth16 as ProvingScheme>::VerifyingKeyX),
    ProvingSchemeGm17(<ProvingSchemeGm17 as ProvingScheme>::VerifyingKeyX),
}
lazy_static! {
    pub static ref finish_counter: Mutex<i32> = Mutex::new(0);
    pub static ref c_count: Mutex<i32> = Mutex::new(0);
}

pub trait CircuitGenerator {}
// class CircuitGeneratorBase(metaclass=ABCMeta)
pub struct CircuitGeneratorBase
//<T, VK>
// where
//     T: ProvingScheme<VerifyingKeyX = VK> + std::marker::Sync,
//     VK: VerifyingKeyMeta<Output = VK>,
{
    pub circuits: BTreeMap<ConstructorOrFunctionDefinition, CircuitHelper>,
    pub circuits_to_prove: Vec<CircuitHelper>,
    pub proving_scheme: String,
    pub output_dir: String,
    pub parallel_keygen: bool,
    pub p_count: i32,
}

impl CircuitGeneratorBase
//<T, VK>
// where
//     T: ProvingScheme<VerifyingKeyX = VK> + std::marker::Sync,
//     VK: VerifyingKeyMeta<Output = VK>,
{
    // """
    // A circuit generator takes an abstract circuit representation and turns it into a concrete zk-snark circuit.

    // It also handles prover/verification key generation and parsing, and generates the verification contracts using the supplied
    // proving scheme.
    // """

    pub fn new(
        circuits: Vec<CircuitHelper>,
        proving_scheme: String,
        output_dir: String,
        parallel_keygen: bool,
    ) -> Self
// """
    // Create a circuit generator instance

    // :param circuits: list which contains the corresponding circuit helper for every function in the contract which requires verification
    // :param proving_scheme: the proving scheme instance to be used for verification contract generation
    // :param output_dir: base directory where the zkay compilation output is located
    // :param parallel_keygen: if true, multiple python processes are used to generate keys in parallel
    // """
    {
        let circuits_to_prove: Vec<_> = circuits
            .iter()
            .filter_map(|c| {
                if c.requires_verification() && c.fct.can_be_external() && c.fct.has_side_effects()
                {
                    Some(c.clone())
                } else {
                    None
                }
            })
            .collect();
        let p_count = (circuits_to_prove.len() as i32).min(num_cpus::get() as i32);
        Self {
            circuits: circuits
                .into_iter()
                .map(|circ| (circ.fct.clone(), circ.clone()))
                .collect(),
            circuits_to_prove,
            proving_scheme,
            output_dir,
            parallel_keygen,
            p_count,
        }
    }

    pub fn generate_circuits(&self, import_keys: bool)
    // """
    // Generate circuit code and verification contracts based on the provided circuits and proving scheme.

    // :param import_keys: if false, new verification and prover keys will be generated, otherwise key files for all verifiers
    //                     are expected to be already present in the respective output directories
    // """
    //Generate proof circuit code

    //Compile circuits
    {
        let _c_count = self.circuits_to_prove.len();
        zk_print!("Compiling {} circuits...", c_count.lock().unwrap());

        let gen_circs =
            |circuit: &CircuitHelper| -> bool { self._generate_zkcircuit(import_keys, circuit) };
        // with
        time_measure("circuit_compilation", true, false);
        let modified: Vec<_> = if CFG.lock().unwrap().is_unit_test() {
            self.circuits_to_prove.iter().map(gen_circs).collect()
        } else {
            // with Pool(processes=self.p_count) as pool
            (0..self.circuits_to_prove.len())
                .into_par_iter()
                .map(|i| gen_circs(&self.circuits_to_prove[i]))
                .collect()
        };

        if import_keys {
            for path in self.get_all_key_paths() {
                if !Path::new(&path).try_exists().map_or(false, |v| v) {
                    assert!(false, "Zkay contract import failed: Missing keys");
                }
            }
        } else {
            let modified_circuits_to_prove: Vec<_> = modified
                .iter()
                .zip(&self.circuits_to_prove)
                .filter_map(|(&t, circ)| {
                    if t || !self
                        ._get_vk_and_pk_paths(circ)
                        .iter()
                        .all(|p| Path::new(p).try_exists().map_or(false, |v| v))
                    {
                        Some(circ)
                    } else {
                        None
                    }
                })
                .collect();
            //Generate keys in parallel
            zk_print!(
                "Generating keys for {} circuits...",
                c_count.lock().unwrap()
            );
            time_measure("key_generation", true, false);
            {
                if self.parallel_keygen && !CFG.lock().unwrap().is_unit_test() {
                    let counter = 0; // Value("i", 0);
                                     // with Pool(processes=self.p_count, initializer=self.__init_worker, initargs=(counter, c_count,)) as pool
                    {
                        (0..modified_circuits_to_prove.len())
                            .into_par_iter()
                            .for_each(|i| {
                                self._generate_keys_par(modified_circuits_to_prove[i]);
                            });
                    }
                } else {
                    for circ in modified_circuits_to_prove {
                        self._generate_keys(circ);
                    }
                }
            }
        }

        print_step("Write verification contracts");
        {
            for circuit in &self.circuits_to_prove {
                // let vk = self._parse_verification_key(circuit);
                let pk_hash = self._get_prover_key_hash(circuit);
                let mut f = File::create(Path::new(
                    &PathBuf::from(&self.output_dir)
                        .join(&circuit.verifier_contract_filename.clone().unwrap()),
                ))
                .expect("");
                let primary_inputs = self._get_primary_inputs(circuit);
                // if let VerifyingKeyType::ProvingSchemeGroth16(vk) = vk {
                // let vk: <T as ProvingScheme>::VerifyingKey = vk;
                // let vkk=||-><T as ProvingScheme>::VerifyingKeyX {vk};
                match self.proving_scheme.as_str() {
                    "groth16" => {
                        let vk=<ProvingSchemeGroth16 as ProvingScheme>::VerifyingKeyX::create_dummy_key();
                        f.write_all(
                            ProvingSchemeGroth16::generate_verification_contract(
                                vk,
                                circuit,
                                primary_inputs,
                                pk_hash,
                            )
                            .as_bytes(),
                        );
                    }
                    "gm17" => {
                        let vk =
                            <ProvingSchemeGm17 as ProvingScheme>::VerifyingKeyX::create_dummy_key();
                        f.write_all(
                            ProvingSchemeGm17::generate_verification_contract(
                                vk,
                                circuit,
                                primary_inputs,
                                pk_hash,
                            )
                            .as_bytes(),
                        );
                    }
                    other => {
                        println!("Unsupport proving scheme: {:?}", other);
                    }
                }
                // } else if let VerifyingKeyType::ProvingSchemeGm17(vk) = vk {
                //     let vkk=||-><T as ProvingScheme>::VerifyingKeyX {vk};
                //     // let vk: <T as ProvingScheme>::VerifyingKey = vk;
                //     f.write_all(
                //         self.proving_scheme.generate_verification_contract(
                //             vkk(),
                //             circuit,
                //             primary_inputs,
                //             pk_hash,
                //         )
                //         .as_bytes(),
                //     );
                // }
            }
        }
    }
    pub fn get_all_key_paths(&self) -> Vec<String>
// """Return paths of all key files for this contract."""
    {
        let mut paths = vec![];
        for circuit in &self.circuits_to_prove {
            paths.extend(self._get_vk_and_pk_paths(circuit));
        }
        paths
    }

    pub fn get_verification_contract_filenames(&self) -> Vec<String>
// """Return file paths for all verification contracts generated by this CircuitGeneratorBase"""
    {
        self.circuits_to_prove
            .iter()
            .map(|circuit| {
                Path::new(&self.output_dir)
                    .join(&circuit.verifier_contract_filename.clone().unwrap())
                    .to_str()
                    .unwrap()
                    .to_string()
            })
            .collect()
    }

    // @staticmethod
    pub fn __init_worker(counter: i32, total_count: i32) {
        *finish_counter.lock().unwrap() = counter;
        *c_count.lock().unwrap() = total_count;
    }

    pub fn _generate_keys_par(&self, circuit: &CircuitHelper) {
        self._generate_keys(circuit);

        *finish_counter.lock().unwrap() += 1;
        zk_print!(
            r#"Generated keys for circuit "\"{}\" [{}/{}]"#,
            circuit
                .verifier_contract_type
                .as_ref()
                .unwrap()
                .to_ast()
                .code(),
            finish_counter.lock().unwrap(),
            c_count.lock().unwrap()
        );
    }

    pub fn _get_circuit_output_dir(&self, circuit: &CircuitHelper) -> String
// """Return the output directory for an individual circuit"""
    {
        PathBuf::from(&self.output_dir)
            .join(
                &CFG.lock()
                    .unwrap()
                    .get_circuit_output_dir_name(circuit.get_verification_contract_name()),
            )
            .to_str()
            .unwrap()
            .to_string()
    }

    pub fn _get_vk_and_pk_paths(&self, circuit: &CircuitHelper) -> Vec<String>
// """Return a tuple which contains the paths to the verification and prover key files."""
    {
        let output_dir = self._get_circuit_output_dir(circuit);
        Self::get_vk_and_pk_filenames()
            .iter()
            .map(|fname| {
                PathBuf::from(&output_dir)
                    .join(fname)
                    .to_str()
                    .unwrap()
                    .to_string()
            })
            .collect()
    }

    // @abstractmethod
    pub fn _generate_zkcircuit(&self, import_keys: bool, circuit: &CircuitHelper) -> bool
// """
        // Generate code and compile a single circuit.

        // When implementing a new backend, this function should generate a concrete circuit representation, which has
        // a) circuit IO corresponding to circuit.sec_idfs/output_idfs/input_idfs
        // b) logic corresponding to the non-CircCall statements in circuit.phi
        // c) a), b) and c) for the circuit associated with the target function for every CircCall statement in circuit.phi

        // The output of this function should be in a state where key generation can be invoked immediately without further transformations
        // (i.e. any intermediary compilation steps should also happen here).

        // It should be stored in self._get_circuit_output_dir(circuit)

        // :return: true if the circuit was modified since last generation (need to generate new keys)
        // """
        // pass
    {
        false
    }

    // @abstractmethod
    pub fn _generate_keys(&self, circuit: &CircuitHelper) {}
    // """Generate prover and verification keys for the circuit stored in self._get_circuit_output_dir(circuit)."""
    // pass

    // @classmethod
    // @abstractmethod
    pub fn get_vk_and_pk_filenames() -> Vec<String> {
        vec![]
    }
    // pass

    // @abstractmethod
    //     pub fn _parse_verification_key(
    //         &self,
    //         circuit: &CircuitHelper,
    //     ) -> <T as ProvingScheme>::VerifyingKeyX
    // // """Parse the generated verificaton key file and return a verification key object compatible with self.proving_scheme"""
    //     {
    //         <T as ProvingScheme>::VerifyingKeyX::create_dummy_key()
    //     }

    // @abstractmethod
    pub fn _get_prover_key_hash(&self, circuit: &CircuitHelper) -> Vec<u8> {
        vec![]
    }
    // pass

    pub fn _get_primary_inputs(&self, circuit: &CircuitHelper) -> Vec<String>
// """
        // Return list of all public input locations
        // :param circuit: abstract circuit representation
        // :return: list of location strings, a location is either an identifier name or an array index
        // """
    {
        let inputs = circuit.public_arg_arrays().clone();

        if CFG
            .lock()
            .unwrap()
            .should_use_hash(circuit.trans_in_size + circuit.trans_out_size)
        {
            vec![match self.proving_scheme.as_str() {
                "groth16" => <ProvingSchemeGroth16 as ProvingScheme>::hash_var_name(),
                "gm17" => <ProvingSchemeGm17 as ProvingScheme>::hash_var_name(),
                other => {
                    println!("Unsupport proving scheme: {:?}", other);
                    String::new()
                }
            }]
        } else {
            let mut primary_inputs = vec![];
            for (name, count) in inputs {
                primary_inputs.extend(
                    (0..count)
                        .map(|i| format!("{name}[{i}]"))
                        .collect::<Vec<_>>(),
                )
            }
            primary_inputs
        }
    }
}
