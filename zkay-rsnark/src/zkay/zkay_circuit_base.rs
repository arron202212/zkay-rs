#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    circuit::{
        eval::circuit_evaluator::CircuitEvaluator,
        operations::gadget::GadgetConfig,
        structure::{
            circuit_generator::CGConfigFields,
            circuit_generator::CGInstance,
            circuit_generator::{
                CGConfig, CircuitGenerator, CircuitGeneratorExtend, add_to_evaluation_queue,
                get_active_circuit_generator,
            },
            wire::GetWireId,
            wire::WireConfig,
            wire_array::WireArray,
            wire_type::WireType,
        },
    },
    util::util::{BigInteger, Util},
    zkay::{
        crypto::crypto_backend::{
            Backend, CryptoBackend, CryptoBackendConfig, CryptoBackendConfigs,
        },
        crypto::homomorphic_backend::HomomorphicBackend,
        homomorphic_input::HomomorphicInput,
        typed_wire::TypedWire,
        zkay_sha256_gadget::ZkaySHA256Gadget,
        zkay_type::{ZkayType, zkbool},
        zkay_util::ZkayUtil,
    },
};

use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    fs::File,
    io::Write,
    ops::{Add, Mul, Neg, Sub},
};

use num_bigint::Sign;
use rccell::RcCell;
const ADD_OP_LABELS: bool = true;
const LEGACY_CRYPTO_BACKEND: &str = "LEGACY_CRYPTO_BACKEND";

#[derive(Debug, Clone)]
pub struct ZkayCircuitBase<T> {
    //Whether to include comments for the more complex operations in the circuit.arith file
    pub real_circuit_name: String,
    pub crypto_backends: HashMap<String, Backend>,
    pub current_pub_in_idx: i32,
    pub current_pub_out_idx: i32,
    pub all_pub_io_wires: Vec<Option<WireType>>,
    pub current_priv_in_idx: i32,
    pub all_priv_in_wires: Vec<Option<WireType>>,
    pub pub_in_names: Vec<String>,
    pub pub_out_names: Vec<String>,
    pub priv_in_names: Vec<String>,
    pub pub_in_count: i32,
    pub use_input_hashing: bool,
    pub vars: HashMap<String, Vec<TypedWire>>,
    pub current_guard_condition: VecDeque<TypedWire>,
    pub serialized_arguments: Vec<BigInteger>,
    pub name_prefix_indices: HashMap<String, i32>,
    pub name_prefix: VecDeque<String>,
    pub guard_prefixes: VecDeque<VecDeque<String>>,
    pub guard_prefix_indices: VecDeque<HashMap<String, i32>>,
    pub t: T,
}
impl<T: std::fmt::Debug + std::clone::Clone> ZkayCircuitBase<T> {
    pub fn new(
        name: String,
        crypto_backend_id: Option<String>,
        crypto_backend: Option<String>,
        key_bits: i32,
        pub_in_size: i32,
        pub_out_size: i32,
        priv_size: i32,
        use_input_hashing: bool,
        t: T,
    ) -> CircuitGeneratorExtend<ZkayCircuitBase<T>> {
        let mut _self = CircuitGeneratorExtend::new(
            "circuit",
            Self {
                real_circuit_name: name,
                crypto_backends: HashMap::new(),
                current_pub_in_idx: 0,
                current_pub_out_idx: pub_in_size,
                all_pub_io_wires: vec![None; (pub_in_size + pub_out_size) as usize],
                current_priv_in_idx: 0,
                all_priv_in_wires: vec![None; priv_size as usize],
                pub_in_names: vec![],
                pub_out_names: vec![],
                priv_in_names: vec![],
                pub_in_count: pub_in_size,
                use_input_hashing,
                vars: HashMap::new(),
                current_guard_condition: VecDeque::new(),
                serialized_arguments: vec![],
                name_prefix_indices: HashMap::new(),
                name_prefix: VecDeque::new(),
                guard_prefixes: VecDeque::new(),
                guard_prefix_indices: VecDeque::new(),
                t,
            },
        );
        Self::clear_prefix(&mut _self.t.name_prefix, &mut _self.t.name_prefix_indices);
        Self::push_guard_prefix(
            &mut _self.t.guard_prefixes,
            &mut _self.t.guard_prefix_indices,
        );

        if let Some(crypto_backend) = crypto_backend {
            // Legacy handling: add default "main" crypto backend
            let id = crypto_backend_id.unwrap_or(LEGACY_CRYPTO_BACKEND.to_owned());
            assert!(
                _self
                    .t
                    .crypto_backends
                    .insert(
                        id,
                        Backend::create(&crypto_backend, key_bits, _self.cg.clone()),
                    )
                    .is_none()
            );
        }

        _self
    }

    fn clear_prefix(prefix: &mut VecDeque<String>, indices: &mut HashMap<String, i32>) {
        prefix.clear();
        prefix.push_front("".to_owned());
        indices.clear();
    }

    fn push_prefix(
        prefix: &mut VecDeque<String>,
        prefix_indices: &mut HashMap<String, i32>,
        new_str: &str,
    ) {
        let mut new_prefix = format!("{}{}.", prefix.front().unwrap(), new_str);
        let count = *prefix_indices.get(&new_prefix).unwrap_or(&0);
        prefix_indices.insert(new_prefix.clone(), count + 1);
        prefix.push_front(format!("{}{}.", new_prefix, count));
    }

    fn push_guard_prefix(
        guard_prefixes: &mut VecDeque<VecDeque<String>>,
        guard_prefix_indices: &mut VecDeque<HashMap<String, i32>>,
    ) {
        let mut new_prefix = VecDeque::new();
        let mut new_prefix_indices = HashMap::new();
        Self::clear_prefix(&mut new_prefix, &mut new_prefix_indices);
        guard_prefixes.push_front(new_prefix);
        guard_prefix_indices.push_front(new_prefix_indices);
    }
}

impl<T: crate::circuit::StructNameConfig + std::fmt::Debug> crate::circuit::StructNameConfig
    for CircuitGeneratorExtend<ZkayCircuitBase<T>>
{
    fn name(&self) -> String {
        self.t.t.name()
    }
}
pub trait ZkayCircuitBaseFields {
    fn real_circuit_name(&self) -> &String;
    fn crypto_backends(&self) -> &HashMap<String, Backend>;
    fn current_pub_in_idx(&self) -> i32;
    fn current_pub_out_idx(&self) -> i32;
    fn all_pub_io_wires(&self) -> &Vec<Option<WireType>>;
    fn current_priv_in_idx(&self) -> i32;
    fn all_priv_in_wires(&self) -> &Vec<Option<WireType>>;
    fn pub_in_names(&self) -> &Vec<String>;
    fn pub_out_names(&self) -> &Vec<String>;
    fn priv_in_names(&self) -> &Vec<String>;
    fn pub_in_count(&self) -> i32;
    fn use_input_hashing(&self) -> bool;
    fn vars(&self) -> &HashMap<String, Vec<TypedWire>>;
    fn current_guard_condition(&self) -> &VecDeque<TypedWire>;
    fn serialized_arguments(&self) -> &Vec<BigInteger>;
    fn name_prefix_indices(&self) -> &HashMap<String, i32>;
    fn name_prefix(&self) -> &VecDeque<String>;
    fn guard_prefixes(&self) -> &VecDeque<VecDeque<String>>;
    fn guard_prefix_indices(&self) -> &VecDeque<HashMap<String, i32>>;
    fn serializedArguments_mut(&mut self) -> &mut Vec<BigInteger>;
}
impl<T: crate::circuit::StructNameConfig + std::fmt::Debug> ZkayCircuitBaseFields
    for CircuitGeneratorExtend<ZkayCircuitBase<T>>
{
    fn real_circuit_name(&self) -> &String {
        &self.t.real_circuit_name
    }
    fn crypto_backends(&self) -> &HashMap<String, Backend> {
        &self.t.crypto_backends
    }
    fn current_pub_in_idx(&self) -> i32 {
        self.t.current_pub_in_idx
    }
    fn current_pub_out_idx(&self) -> i32 {
        self.t.current_pub_out_idx
    }
    fn all_pub_io_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.all_pub_io_wires
    }
    fn current_priv_in_idx(&self) -> i32 {
        self.t.current_priv_in_idx
    }
    fn all_priv_in_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.all_priv_in_wires
    }
    fn pub_in_names(&self) -> &Vec<String> {
        &self.t.pub_in_names
    }
    fn pub_out_names(&self) -> &Vec<String> {
        &self.t.pub_out_names
    }
    fn priv_in_names(&self) -> &Vec<String> {
        &self.t.priv_in_names
    }
    fn pub_in_count(&self) -> i32 {
        self.t.pub_in_count
    }
    fn use_input_hashing(&self) -> bool {
        self.t.use_input_hashing
    }
    fn vars(&self) -> &HashMap<String, Vec<TypedWire>> {
        &self.t.vars
    }
    fn current_guard_condition(&self) -> &VecDeque<TypedWire> {
        &self.t.current_guard_condition
    }
    fn serialized_arguments(&self) -> &Vec<BigInteger> {
        &self.t.serialized_arguments
    }
    fn name_prefix_indices(&self) -> &HashMap<String, i32> {
        &self.t.name_prefix_indices
    }
    fn name_prefix(&self) -> &VecDeque<String> {
        &self.t.name_prefix
    }
    fn guard_prefixes(&self) -> &VecDeque<VecDeque<String>> {
        &self.t.guard_prefixes
    }
    fn guard_prefix_indices(&self) -> &VecDeque<HashMap<String, i32>> {
        &self.t.guard_prefix_indices
    }

    fn serializedArguments_mut(&mut self) -> &mut Vec<BigInteger> {
        &mut self.t.serialized_arguments
    }
}
pub trait ZkayCircuitBaseConfig: ZkayCircuitBaseFields + CGConfig {
    fn run(&mut self, args: &Vec<String>) {
        match args[0].as_str() {
            "compile" => self.compile_circuit(),
            "prove" => {
                self.compile_circuit();
                self.parse_inputs(&args[1..].to_vec());
                //println!("Evaluating circuit '" + real_circuit_name + "'");
                self.eval_circuit();
            }

            _ => panic!("invalid command"),
        }
        self.prep_files(None);
    }

    fn parse_inputs(&mut self, args: &Vec<String>) {
        let tot_count = self.all_pub_io_wires().len() + self.all_priv_in_wires().len();
        assert!(
            args.len() == tot_count,
            "Input count mismatch, expected {}, was {}",
            tot_count,
            args.len()
        );
        let mut serialized_arguments = vec![BigInteger::default(); tot_count];
        for i in 0..tot_count {
            let v = Util::parse_big_int_x(&args[i]);
            assert!(
                v.sign() != Sign::Minus,
                "No signed inputs (signed must be converted to unsigned beforehand)"
            );
            serialized_arguments[i] = v;
        }
        //self.serializedArguments_mut() = serialized_arguments;
    }
    fn compile_circuit(&mut self) {
        println!("Compiling circuit '{}'", self.real_circuit_name());
        // let mut generator=self.cg.borrow().clone();
        self.generate_circuit();
        assert!(
            self.current_pub_in_idx() == self.pub_in_count()
                && self.current_pub_out_idx() == self.all_pub_io_wires().len() as i32,
            "Not all pub inputs assigned {},{},{},{}",
            self.current_pub_in_idx(),
            self.pub_in_count(),
            self.current_pub_out_idx(),
            self.all_pub_io_wires().len()
        );
        assert!(
            self.current_priv_in_idx() == self.all_priv_in_wires().len() as i32,
            "Not all  inputs assigned {},{}",
            self.current_priv_in_idx(),
            self.all_priv_in_wires().len()
        );
        if self.use_input_hashing() {
            CircuitGenerator::make_output_array_with_str(
                self.cg(),
                ZkaySHA256Gadget::new(self.all_pub_io_wires().clone(), 253, self.cg())
                    .get_output_wires(),
                "digest",
            );
        }
        //println!("Done with generate_circuit, preparing dummy files...");
    }
}

impl<T: crate::circuit::StructNameConfig + std::fmt::Debug + std::clone::Clone>
    CircuitGeneratorExtend<ZkayCircuitBase<T>>
{
    pub fn super_build_circuit(&mut self) {
        // let generator = &self.generator;
        let pub_in_count = self.t.pub_in_count as usize;
        // Create IO wires
        let pub_out_count = self.t.all_pub_io_wires.len() - pub_in_count;
        let (in_array, out_array) = if self.t.use_input_hashing {
            (
                CircuitGenerator::create_prover_witness_wire_array_with_str(
                    self.cg.clone(),
                    pub_in_count,
                    "in_",
                ),
                CircuitGenerator::create_prover_witness_wire_array_with_str(
                    self.cg.clone(),
                    pub_out_count,
                    "out_",
                ),
            )
        } else {
            (
                CircuitGenerator::create_input_wire_array_with_str(
                    self.cg.clone(),
                    pub_in_count,
                    "in_",
                ),
                CircuitGenerator::create_input_wire_array_with_str(
                    self.cg.clone(),
                    pub_out_count,
                    "out_",
                ),
            )
        };
        let priv_in_array = CircuitGenerator::create_prover_witness_wire_array_with_str(
            self.cg.clone(),
            self.t.all_priv_in_wires.len(),
            "priv_",
        );

        // Legacy handling
        let legacy_crypto_backend = self.t.crypto_backends.get(LEGACY_CRYPTO_BACKEND);
        if legacy_crypto_backend.is_some_and(|v| v.is_symmetric()) {
            let my_pk = in_array[0].as_ref().unwrap();
            let my_sk = priv_in_array[0].as_ref().unwrap();
            self.set_key_pair(LEGACY_CRYPTO_BACKEND, my_pk, my_sk);
        }

        self.t.all_pub_io_wires[..pub_in_count].clone_from_slice(&in_array[..pub_in_count]);
        self.t.all_pub_io_wires[pub_in_count..pub_in_count + pub_out_count]
            .clone_from_slice(&out_array[..pub_out_count]);
        let len = self.t.all_priv_in_wires.len();
        self.t.all_priv_in_wires[..len].clone_from_slice(&priv_in_array[..len]);
    }

    pub fn super_generate_sample_input(&self, evaluator: &mut CircuitEvaluator) {
        assert!(
            !self.t.serialized_arguments.is_empty(),
            "No inputs specified, this should not have been called"
        );

        assert!(
            self.t.serialized_arguments.len()
                == self.t.all_pub_io_wires.len() + self.t.all_priv_in_wires.len(),
            "Invalid serialized argument count, expected {} was {}",
            self.t.all_pub_io_wires.len(),
            self.t.serialized_arguments.len()
        );

        let mut idx = 0;
        for io_name_list in [
            &self.t.pub_in_names,
            &self.t.pub_out_names,
            &self.t.priv_in_names,
        ] {
            for name in io_name_list {
                let wires = self.t.vars.get(name).unwrap();
                let mut sb = format!("Setting '{name}' to [");
                for w in wires {
                    let val = &self.t.serialized_arguments[idx];
                    idx += 1;
                    evaluator.set_wire_value(&w.wire, val);
                    sb.push_str(&format!("wid {}={}, ", w.wire.get_wire_id(), val));
                }
                sb.pop();
                sb.pop();
                sb.push(']');
                println!("{sb}");
            }
        }

        assert!(
            idx == self.t.all_pub_io_wires.len() + self.t.all_priv_in_wires.len(),
            "Not all inputs consumed"
        );
    }
    pub fn super_prep_files(&self, circuit_evaluator: Option<CircuitEvaluator>) {
        if !self.t.serialized_arguments.is_empty() {
            // CGConfig::prep_files(self, circuit_evaluator);
        } else {
            self.cg.write_circuit_file();
            self.write_dummy_input_file();
        }
    }
}

impl<T: std::fmt::Debug> CircuitGeneratorExtend<ZkayCircuitBase<T>> {
    fn write_dummy_input_file(&self) {
        let mut print_writer = File::create(self.get_name() + ".in").unwrap();
        write!(print_writer, "0 1");
        let mut all_io_wires = Vec::with_capacity(
            self.get_in_wires().len()
                + self.get_out_wires().len()
                + self.get_prover_witness_wires().len(),
        );
        all_io_wires.append(&mut self.get_in_wires()[1..].to_vec());
        all_io_wires.append(&mut self.get_out_wires());
        all_io_wires.append(&mut self.get_prover_witness_wires());
        for w in all_io_wires {
            write!(print_writer, "{} 0", w.as_ref().unwrap().get_wire_id());
        }
    }
}

impl<T: crate::circuit::StructNameConfig + std::fmt::Debug + std::clone::Clone>
    CircuitGeneratorExtend<ZkayCircuitBase<T>>
{
    pub fn add_io(
        type_name: &str,
        name: String,
        name_list: &mut Vec<String>,
        src: &Vec<Option<WireType>>,
        start_idx: i32,
        size: i32,
        t: ZkayType,
        restrict: bool,
        vars: &mut HashMap<String, Vec<TypedWire>>,
        generator: RcCell<CircuitGenerator>,
    ) -> Vec<Option<WireType>> {
        // let name = self.get_qualified_name(&name);
        println!(
            "Adding '{name}' = {type_name}[{start_idx}:{}]",
            start_idx + size
        );
        let input = src[start_idx as usize..(start_idx + size) as usize].to_vec();
        // Enforce size and associate wire with type (technically restrict is only required for  inputs)

        let mut t_input: Vec<_> = input
            .iter()
            .map(|i| {
                TypedWire::new(
                    i.clone().unwrap(),
                    t.clone(),
                    name.clone(),
                    &vec![restrict],
                    generator.clone(),
                )
            })
            .collect();

        vars.insert(name.to_owned(), t_input);
        name_list.push(name.to_owned());
        input
    }

    // CRYPTO BACKENDS
    pub fn add_crypto_backend(
        &mut self,
        crypto_backend_id: &str,
        crypto_backend_name: &str,
        key_bits: i32,
    ) {
        assert!(
            !self.t.crypto_backends.contains_key(crypto_backend_id),
            "Crypto backend {crypto_backend_id} already registered"
        );

        self.t.crypto_backends.insert(
            crypto_backend_id.to_owned(),
            Backend::create(crypto_backend_name, key_bits, self.cg()),
        );
    }

    pub fn set_key_pairn(&mut self, crypto_backend_id: &str, pk_name: &str, skName: &str) {
        self.set_key_pair(
            crypto_backend_id,
            &self.get(pk_name).wire,
            &self.get(skName).wire,
        );
    }

    pub fn set_key_pair(&mut self, crypto_backend_id: &str, my_pk: &WireType, my_sk: &WireType) {
        let generator = self.cg();
        let crypto_backend = self.get_crypto_backend_mut(crypto_backend_id);
        assert!(
            crypto_backend.is_symmetric(),
            "Crypto backend is not symmetric"
        );

        let symmetric_crypto = crypto_backend;
        symmetric_crypto.set_key_pair(my_pk, my_sk, generator);
    }
    #[inline]
    pub fn get_crypto_backend(&self, crypto_backend_id: &str) -> &Backend {
        self.t
            .crypto_backends
            .get(crypto_backend_id)
            .expect(&format!("Unknown crypto backend: {crypto_backend_id}"))
    }
    #[inline]
    pub fn get_crypto_backend_mut(&mut self, crypto_backend_id: &str) -> &mut Backend {
        self.t
            .crypto_backends
            .get_mut(crypto_backend_id)
            .expect(&format!("Unknown crypto backend: {crypto_backend_id}"))
    }
    pub fn get_homomorphic_crypto_backend(
        &mut self,
        crypto_backend_id: &str,
    ) -> Box<dyn HomomorphicBackend + '_> {
        let mut crypto_backend = self.get_crypto_backend(crypto_backend_id);
        crypto_backend.homomorphic_backend().expect(&format!(
            "Crypto backend {crypto_backend_id} is not homomorphic"
        ))
    }

    // CIRCUIT IO

    pub fn add_in(&mut self, name: &str, size: i32, t: ZkayType) {
        let generator = self.cg();
        Self::add_io(
            "in",
            self.get_qualified_name(name),
            &mut self.t.pub_in_names,
            &self.t.all_pub_io_wires,
            self.t.current_pub_in_idx,
            size,
            t,
            false,
            &mut self.t.vars,
            generator,
        );
        self.t.current_pub_in_idx += size;
    }

    pub fn add_ki(&mut self, crypto_backend_id: &str, name: &str, size: i32) {
        let generator = self.cg();

        let (name, all_pub_io_wires, current_pub_in_idx) = (
            self.get_qualified_name(name),
            &self.t.all_pub_io_wires,
            self.t.current_pub_in_idx,
        );
        self.t.current_pub_in_idx += size;
        let mut crypto_backend = self.get_crypto_backend(crypto_backend_id);
        let chunk_size = crypto_backend.get_key_chunk_size();
        let input = Self::add_io(
            "in",
            name.clone(),
            &mut self.t.pub_in_names,
            all_pub_io_wires,
            current_pub_in_idx,
            size,
            ZkayType::zk_uint(chunk_size),
            false,
            &mut self.t.vars,
            generator.clone(),
        );

        let mut crypto_backend = self.get_crypto_backend_mut(crypto_backend_id);
        crypto_backend.add_key(&name, &input, generator);
    }

    pub fn add_k(&mut self, name: &str, size: i32) {
        self.add_ki(LEGACY_CRYPTO_BACKEND, name, size);
    }

    pub fn add_out(&mut self, name: &str, size: i32, t: ZkayType) {
        let generator = self.cg();
        Self::add_io(
            "out",
            self.get_qualified_name(name),
            &mut self.t.pub_out_names,
            &self.t.all_pub_io_wires,
            self.t.current_pub_out_idx,
            size,
            t,
            false,
            &mut self.t.vars,
            generator,
        );
        self.t.current_pub_out_idx += size;
    }

    pub fn add_s(&mut self, name: &str, size: i32, t: ZkayType) {
        let generator = self.cg();
        Self::add_io(
            "priv",
            self.get_qualified_name(name),
            &mut self.t.priv_in_names,
            &self.t.all_priv_in_wires,
            self.t.current_priv_in_idx,
            size,
            t,
            true,
            &mut self.t.vars,
            generator,
        );
        self.t.current_priv_in_idx += size;
    }

    // CONTROL FLOW

    pub fn step_in(&mut self, fct: &str) {
        Self::push_prefix(
            &mut self.t.name_prefix,
            &mut self.t.name_prefix_indices,
            &(self
                .t
                .guard_prefixes
                .front()
                .unwrap()
                .front()
                .unwrap()
                .to_owned()
                + fct),
        );
        Self::push_guard_prefix(&mut self.t.guard_prefixes, &mut self.t.guard_prefix_indices);
    }

    pub fn step_out(&mut self) {
        self.t.name_prefix.pop_front();
        self.t.guard_prefixes.pop_front();
        self.t.guard_prefix_indices.pop_front();
    }

    pub fn add_guard(&mut self, name: &str, is_true: bool) {
        let mut new_wire = self.get(name).wire.clone();

        Self::push_prefix(
            self.t.guard_prefixes.front_mut().unwrap(),
            self.t.guard_prefix_indices.front_mut().unwrap(),
            &format!("{name}_{is_true}"),
        );

        if !is_true {
            new_wire = new_wire.inv_as_bit().unwrap();
        }

        if let Some(v) = self.t.current_guard_condition.front() {
            new_wire = v.wire.and(&new_wire);
        }
        self.t.current_guard_condition.push_front(TypedWire::new(
            new_wire,
            zkbool().clone(),
            format!("guard_cond_top_{name}_{is_true}"),
            &vec![],
            self.cg(),
        ));
    }

    pub fn pop_guard(&mut self) {
        self.t.current_guard_condition.pop_front();
        self.t.guard_prefixes.pop_front();
    }

    pub fn ite(
        &self,
        condition: &TypedWire,
        true_val: &TypedWire,
        false_val: &TypedWire,
    ) -> TypedWire {
        ZkayType::check_type(zkbool(), &condition.zkay_type);
        ZkayType::check_type(&true_val.zkay_type, &false_val.zkay_type);
        TypedWire::new(
            self.cond_expr(&condition.wire, &true_val.wire, &false_val.wire),
            true_val.zkay_type.clone(),
            format!(
                "if {}  {{{}}}  {{{}}}",
                condition.name, true_val.name, false_val.name
            ),
            &vec![],
            self.cg(),
        )
    }

    // UNARY OPS

    pub fn negate(&self, val: &TypedWire) -> TypedWire {
        let bits = val.zkay_type.bitwidth;
        if bits < 256 {
            // Take two's complement
            let inv_bits = TypedWire::new(
                val.wire.inv_bits(val.zkay_type.bitwidth as u64),
                val.zkay_type.clone(),
                format!("~{}", val.name),
                &vec![],
                self.cg(),
            );
            inv_bits.plus(&self.val_iz(1, val.zkay_type.clone()))
        } else {
            TypedWire::new(
                val.wire
                    .clone()
                    .muli_with_option(-1, &Some(format!("-{}", val.name))),
                val.zkay_type.clone(),
                format!("-{}", val.name),
                &vec![],
                self.cg(),
            )
        }
    }

    pub fn bit_inv(&self, val: &TypedWire) -> TypedWire {
        let result_type = ZkayType::check_typeb(&val.zkay_type, &val.zkay_type, false);
        let res = val
            .wire
            .inv_bits_with_option(result_type.bitwidth as u64, &Some(format!("~{}", val.name)));
        TypedWire::new(
            res,
            result_type,
            format!("~{}", val.name),
            &vec![],
            self.cg(),
        )
    }

    pub fn not(&self, val: &TypedWire) -> TypedWire {
        ZkayType::check_type(zkbool(), &val.zkay_type);
        TypedWire::new(
            val.wire
                .inv_as_bit_with_option(&Some(format!("!{}", val.name)))
                .unwrap(),
            zkbool().clone(),
            format!("!{}", val.name),
            &vec![],
            self.cg(),
        )
    }

    // String op interface
    pub fn o_(&self, op: char, wire: &TypedWire) -> TypedWire {
        match op {
            '-' => self.negate(wire),
            '~' => self.bit_inv(wire),
            '!' => self.not(wire),
            _ => panic!(),
        }
    }

    pub fn o_tct(&self, lhs: &TypedWire, op: char, rhs: &TypedWire) -> TypedWire {
        match op {
            '+' => lhs.plus(rhs),
            '-' => lhs.minus(rhs),
            '*' => lhs.times(rhs),
            '/' => lhs.divide_by(rhs,self.cg()),
            '%' => lhs.modulo(rhs,self.cg()),
            '|' => lhs.bit_or(rhs),
            '&' => lhs.bit_and(rhs),
            '^' => lhs.bit_xor(rhs),
            '<' => lhs.is_less_thans(rhs),
            '>'/*'*/=> lhs.is_greater_thans(rhs),
            _ => panic!(),
        }
    }

    pub fn o_tctct(
        &self,
        cond: &TypedWire,
        cond_char: char,
        true_val: &TypedWire,
        alt_char: char,
        false_val: &TypedWire,
    ) -> TypedWire {
        assert!(cond_char == '?' && alt_char == ':');
        self.ite(cond, true_val, false_val)
    }

    pub fn o_tsi(&self, lhs: &TypedWire, op: &str, rhs: i32) -> TypedWire {
        match op {
            "<<" => lhs.shift_left_by(rhs),
            ">>" => lhs.shift_right_by(rhs),
            _ => panic!(),
        }
    }

    pub fn o_tst(&self, lhs: &TypedWire, op: &str, rhs: &TypedWire) -> TypedWire {
        match op {
            "==" => lhs.is_equal_tos(rhs),
            "!=" => lhs.is_not_equal_to(rhs),
            "<=" => lhs.is_less_than_or_equals(rhs),
            ">=" => lhs.is_greater_than_or_equals(rhs),
            "&&" => lhs.and(rhs),
            "||" => lhs.or(rhs),
            _ => panic!(),
        }
    }

    // Homomorphic operations

    pub fn o_hom(
        &mut self,
        crypto_backend_id: &str,
        key: &str,
        op: char,
        arg: &HomomorphicInput,
    ) -> Vec<TypedWire> {
        let generator = self.cg();
        let key = self.get_qualified_name(key);
        let backend = self.get_homomorphic_crypto_backend(crypto_backend_id);
        backend.do_homomorphic_opu(op, arg, &key, generator)
    }

    pub fn o_hom_sshch(
        &mut self,
        crypto_backend_id: &str,
        key: &str,
        lhs: &HomomorphicInput,
        op: char,
        rhs: &HomomorphicInput,
    ) -> Vec<TypedWire> {
        let generator = self.cg();
        let key = self.get_qualified_name(key);
        let backend = self.get_homomorphic_crypto_backend(crypto_backend_id);
        backend.do_homomorphic_op(lhs, op, rhs, &key, generator)
    }

    pub fn o_hom_sshchch(
        &mut self,
        crypto_backend_id: &str,
        key: &str,
        cond: &HomomorphicInput,
        cond_char: char,
        true_val: &HomomorphicInput,
        alt_char: char,
        false_val: &HomomorphicInput,
    ) -> Vec<TypedWire> {
        assert!(cond_char == '?' && alt_char == ':');
        let key = self.get_qualified_name(key);
        let backend = self.get_homomorphic_crypto_backend(crypto_backend_id);
        backend.do_homomorphic_cond(cond, true_val, false_val, &key)
    }

    pub fn o_hom_sshsh(
        &mut self,
        crypto_backend_id: &str,
        key: &str,
        lhs: &HomomorphicInput,
        op: &str,
        rhs: &HomomorphicInput,
    ) -> Vec<TypedWire> {
        let key = self.get_qualified_name(key);
        let backend = self.get_homomorphic_crypto_backend(crypto_backend_id);
        backend.do_homomorphic_ops(lhs, op, rhs, &key)
    }

    pub fn o_rerand(
        &mut self,
        arg: &Vec<TypedWire>,
        crypto_backend_id: &str,
        key: &str,
        randomness: &TypedWire,
    ) -> Vec<TypedWire> {
        let generator = self.cg();
        let key = self.get_qualified_name(key);
        let backend = self.get_homomorphic_crypto_backend(crypto_backend_id);
        backend.do_homomorphic_rerand(arg, &key, randomness, generator)
    }

    // TYPE CASTING
    pub fn cast(&self, w: &TypedWire, target_type: ZkayType) -> TypedWire {
        self.convert_to(w, target_type)
    }

    // SOURCE

    pub fn get(&self, name: &str) -> TypedWire {
        let w = self.get_typed_arr(name);
        assert!(w.len() == 1, "Tried to treat array as a single wire");
        w[0].clone()
    }

    pub fn get_cipher(&self, name: &str) -> &Vec<TypedWire> {
        self.get_typed_arr(name)
    }

    pub fn val(&self, val: bool) -> TypedWire {
        TypedWire::new(
            if val {
                self.cg.get_one_wire().unwrap()
            } else {
                self.cg.get_zero_wire().unwrap()
            },
            zkbool().clone(),
            format!("const_{val}"),
            &vec![],
            self.cg(),
        )
    }

    pub fn val_iz(&self, val: i32, t: ZkayType) -> TypedWire {
        if val == 0 {
            TypedWire::new(
                self.get_zero_wire().unwrap(),
                t,
                format!("const_{val}"),
                &vec![],
                self.cg(),
            )
        } else if val == 1 {
            TypedWire::new(
                self.get_one_wire().unwrap(),
                t,
                format!("const_{val}"),
                &vec![],
                self.cg(),
            )
        } else {
            self.val_sz(&val.to_string(), t)
        }
    }

    pub fn val_sz(&self, val: &str, t: ZkayType) -> TypedWire {
        let v = Util::parse_big_int(val);
        let w = if v.sign() == Sign::Minus {
            assert!(!t.signed, "Cannot store negative constant in unsigned wire");
            let v_neg = ZkayType::_get_negative_constant(&v.clone().neg(), t.bitwidth);
            assert!(v_neg.sign() != Sign::Minus, "Constant is still negative");
            CircuitGenerator::create_constant_wire_with_option(
                self.cg(),
                &v_neg,
                &Some(format!("const_{v}")),
            )
        } else {
            CircuitGenerator::create_constant_wire_with_option(
                self.cg(),
                &v,
                &Some(format!("const_{v}")),
            )
        };
        TypedWire::new(w, t, format!("const_{v}"), &vec![], self.cg())
    }

    // SINK

    pub fn decl(&mut self, lhs: &str, val: TypedWire) {
        // assert!(val.zkay_type.is_some(), "Tried to use untyped wires");

        // Get old value and check type
        let mut old_val;
        if self.t.vars.contains_key(lhs) {
            old_val = self.get(lhs);
            ZkayType::check_type(&old_val.zkay_type, &val.zkay_type);
        } else {
            old_val = self.val_iz(0, val.zkay_type.clone());
        }

        // Only assign value if guard condition is met
        if self.t.current_guard_condition.is_empty() {
            self.set(
                lhs,
                &TypedWire::new(val.wire, val.zkay_type, lhs.to_owned(), &vec![], self.cg()),
            );
        } else {
            self.set(
                lhs,
                &TypedWire::new(
                    self.cond_expr(
                        &self.t.current_guard_condition.front().unwrap().wire,
                        &val.wire,
                        &old_val.wire,
                    ),
                    val.zkay_type.clone(),
                    lhs.to_owned(),
                    &vec![],
                    self.cg(),
                ),
            );
        }
    }

    pub fn decl_svt(&mut self, lhs: &str, val: &Vec<TypedWire>) {
        assert!(!val.is_empty(), "val");
        // assert!(val[0].zkay_type.is_some(), "Tried to use untyped wires");
        // Check that all types match; else this gets really strange
        for i in 0..val.len() - 1 {
            ZkayType::check_type(&val[i].zkay_type, &val[i + 1].zkay_type);
        }

        // Get old value and check type and length
        let mut old_val;
        if self.t.vars.contains_key(lhs) {
            old_val = self.get_typed_arr(lhs).clone();
            ZkayType::check_type(&old_val[0].zkay_type, &val[0].zkay_type);
            assert!(
                val.len() == old_val.len(),
                "WireType amounts differ - old ={}, new = {}",
                old_val.len(),
                val.len()
            );
        } else {
            old_val = vec![self.val_iz(0, val[0].zkay_type.clone()); val.len()];
        }

        // Only assign value if guard condition is met

        let guard = self.t.current_guard_condition.front(); // Null if empty
        let res_val: Vec<_> = val
            .iter()
            .zip(&old_val)
            .map(|(v, ov)| {
                if let Some(g) = guard {
                    TypedWire::new(
                        self.cond_expr(&g.wire, &v.wire, &ov.wire),
                        v.zkay_type.clone(),
                        lhs.to_owned(),
                        &vec![],
                        self.cg(),
                    )
                } else {
                    TypedWire::new(
                        v.wire.clone(),
                        v.zkay_type.clone(),
                        lhs.to_owned(),
                        &vec![],
                        self.cg(),
                    ) // No guard, just rename
                }
            })
            .collect();
        self.set_svt(lhs, &res_val);
    }

    pub fn cond_expr(
        &self,
        cond: &WireType,
        true_val: &WireType,
        false_val: &WireType,
    ) -> WireType {
        if ZkayUtil::ZKAY_RESTRICT_EVERYTHING {
            CircuitGenerator::add_binary_assertion(self.cg(), cond);
        }
        cond.mulw_with_str(true_val, "ite_true").addw_with_str(
            &cond
                .inv_as_bit()
                .unwrap()
                .mulw_with_str(false_val, "ite_false"),
            "ite_res",
        )
    }

    pub fn convert_to(&self, w: &TypedWire, target_type: ZkayType) -> TypedWire {
        let from_type = &w.zkay_type;

        let from_bitwidth = from_type.bitwidth;
        let was_signed = from_type.signed;
        let to_bit_width = target_type.bitwidth;

        let new_wire = if from_bitwidth < to_bit_width {
            // Upcast -> sign/zero extend
            if !was_signed && w.wire.get_bit_wires_if_exist_already().is_none() {
                // If this wire was not yet split we can return it without splitting as an optimization
                // -> upcasts from an unsigned type (most common ) are for free this way
                w.wire.clone()
            } else {
                let bit_wires = w.wire.get_bit_wiresi(from_bitwidth as u64);
                if was_signed && to_bit_width == 256 {
                    // Special  -> sign extension not possible since not enough bits,
                    // want -1 to be field_prime - 1
                    let sign_bit = bit_wires[from_bitwidth as usize - 1].clone().unwrap();
                    sign_bit.mux(&self.negate(w).wire.muli(-1), &w.wire)
                } else {
                    let extendBit = if was_signed {
                        bit_wires[from_bitwidth as usize - 1].clone().unwrap()
                    } else {
                        self.get_zero_wire().unwrap()
                    };
                    let mut new_ws = vec![None; to_bit_width as usize];
                    new_ws[..from_bitwidth as usize].clone_from_slice(&bit_wires.as_array());
                    new_ws[from_bitwidth as usize..to_bit_width as usize].fill(Some(extendBit));
                    WireArray::new(new_ws, self.cg().downgrade())
                        .pack_as_bits_with_to(to_bit_width as usize)
                }
            }
        } else if from_bitwidth > to_bit_width {
            // Downcast -> only keep lower bits
            w.wire
                .get_bit_wiresi_with_option(
                    from_bitwidth as u64,
                    &Some(format!("downcast1 {} ", w.name)),
                )
                .pack_as_bits_with_to_and_desc(
                    to_bit_width as usize,
                    format!("downcast2 {}", w.name),
                )
        } else {
            // Type stays the same -> no expensive bitwise operations necessary
            w.wire.clone()
        };
        TypedWire::new(
            new_wire,
            target_type.clone(),
            format!("({}) {}", target_type, w.name),
            &vec![],
            self.cg(),
        )
    }

    pub fn crypto_enc(
        &mut self,
        crypto_backend_id: &str,
        plain: &str,
        key: &str,
        rnd: &str,
        is_dec: bool,
        generator: RcCell<CircuitGenerator>,
    ) -> Vec<Option<WireType>> {
        let desc = ADD_OP_LABELS.then(|| {
            format!(
                "enc{}({}, {}, {})",
                if is_dec { "[dec]" } else { "" },
                self.get_qualified_name(plain),
                self.get_qualified_name(key),
                self.get_qualified_name(rnd)
            )
        });
        let (plain, key, rnd) = (
            &self.get(plain),
            &self.get_qualified_name(key),
            &self.get_arr(rnd),
        );
        let mut crypto_backend = self.get_crypto_backend_mut(crypto_backend_id);
        assert!(
            !crypto_backend.is_symmetric(),
            "Crypto backend is not asymmetric"
        );

        let enc = crypto_backend.create_encryption_gadget(plain, key, rnd, &desc, generator);
        enc.get_output_wires().clone()
    }

    pub fn crypto_dec(
        &mut self,
        crypto_backend_id: &str,
        cipher: &str,
        pkey: &str,
        skey: &str,
        exp_plain: &str,
    ) -> WireType {
        let generator = self.cg();
        let desc = ADD_OP_LABELS.then(|| {
            format!(
                "dec({}, {}, {})",
                self.get_qualified_name(cipher),
                self.get_qualified_name(pkey),
                self.get_qualified_name(skey)
            )
        });
        let (exp_plain, cipher, pkey, skey) = (
            &self.get(exp_plain),
            &self.get_arr(cipher),
            &self.get_qualified_name(pkey),
            &self.get_arr(skey),
        );
        let mut crypto_backend = self.get_crypto_backend(crypto_backend_id);

        let dec = crypto_backend
            .create_decryption_gadget_with_option(exp_plain, cipher, pkey, skey, &desc, generator);
        dec.get_output_wires()[0].clone().unwrap()
    }

    pub fn crypto_symm_enc(
        &mut self,
        crypto_backend_id: &str,
        plain: &str,
        other_pk: &str,
        iv_cipher: &str,
        is_dec: bool,
        generator: RcCell<CircuitGenerator>,
    ) -> Vec<Option<WireType>> {
        let desc = ADD_OP_LABELS
            .then(|| format!("enc{}({}, k, iv)", if is_dec { "[dec]" } else { "" }, plain));
        let (plain, other_pk, iv_cipher) = (
            &self.get(plain),
            &self.get_qualified_name(other_pk),
            &self.get_arr(iv_cipher),
        );
        let mut crypto_backend = self.get_crypto_backend_mut(crypto_backend_id);
        assert!(
            crypto_backend.is_symmetric(),
            "Crypto backend is not symmetric"
        );

        let enc =
            crypto_backend.create_encryption_gadget(plain, other_pk, iv_cipher, &desc, generator);
        enc.get_output_wires().clone()
    }

    pub fn add_guarded_encryption_assertion(
        &self,
        expected_cipher: &str,
        computed_cipher: &Vec<Option<WireType>>,
    ) {
        let exp_cipher = self.get_arr(expected_cipher);
        let comp_str = if ADD_OP_LABELS {
            format!("{} == cipher", self.get_qualified_name(expected_cipher))
        } else {
            "".to_owned()
        };
        self.add_guarded_one_assertion(
            &self.is_equal(&exp_cipher, &expected_cipher, &computed_cipher, "cipher"),
            &Some(comp_str),
        );
    }

    pub fn add_guarded_non_zero_assertion(&self, value: &Vec<Option<WireType>>, name: &str) {
        self.add_guarded_one_assertion(
            &self.is_non_zero(value, name),
            &Some(format!("assert {} != 0", self.get_qualified_name(name))),
        );
    }

    //Asymmetric Encryption

    pub fn check_enc(
        &mut self,
        crypto_backend_id: &str,
        plain: &str,
        key: &str,
        rnd: &str,
        expected_cipher: &str,
    ) {
        // let mut crypto_backend = self.get_crypto_backend(crypto_backend_id);

        // 1. Check that expected cipher != 0 (since 0 is reserved for default initialization)
        self.add_guarded_non_zero_assertion(&self.get_arr(expected_cipher), expected_cipher);

        // 2. Encrypt
        let computed_cipher = self.crypto_enc(crypto_backend_id, plain, key, rnd, false, self.cg());

        // 3. Check encryption == expected cipher
        self.add_guarded_encryption_assertion(expected_cipher, &computed_cipher);
    }

    //Symmetric Encryption

    pub fn check_symm_enc(
        &mut self,
        crypto_backend_id: &str,
        plain: &str,
        other_pk: &str,
        iv_cipher: &str,
        generator: RcCell<CircuitGenerator>,
    ) {
        // let mut crypto_backend = self.get_crypto_backend(crypto_backend_id);

        // 1. Check that expected cipher != 0 (since 0 is reserved for default initialization)
        self.add_guarded_non_zero_assertion(&self.get_arr(iv_cipher), iv_cipher);

        // 2. Encrypt
        let computed_cipher = self.crypto_symm_enc(
            crypto_backend_id,
            plain,
            other_pk,
            iv_cipher,
            false,
            generator,
        );

        // 3. Check encryption == expected cipher
        self.add_guarded_encryption_assertion(iv_cipher, &computed_cipher);
    }

    //
    //  * Asymmetric Decryption
    //
    pub fn check_dec(
        &mut self,
        crypto_backend_id: &str,
        plain: &str,
        key: &str,
        rnd: &str,
        cipher: &str,
    ) {
        let mut crypto_backend = self.get_crypto_backend(crypto_backend_id);

        if crypto_backend.uses_decryption_gadget() {
            // TODO we are misusing the randomness wire for the secret key, which is extremely ugly...
            let msg_ok = self.crypto_dec(crypto_backend_id, cipher, key, rnd, plain);

            let exp_cipher = self.get_arr(cipher);
            let exp_cipher_is_non_zero = self.is_non_zero(&exp_cipher, cipher); // "!= 0"
            let exp_cipher_is_zero = exp_cipher_is_non_zero
                .inv_as_bit_with_option(&Some(format!("{cipher} == 0")))
                .unwrap();
            let plain_zero = self.is_zero(&self.get_arr(plain), plain);

            // handle uninitialized ciphertext: cipher == 0 => plain == 0
            self.add_guarded_one_assertion(&exp_cipher_is_non_zero.orw(&plain_zero), &None);

            // else: cipher != 0 ==> msg_ok == 1
            self.add_guarded_one_assertion(&exp_cipher_is_zero.orw(&msg_ok), &None);
        } else {
            // 1. Decrypt [dec(cipher, rnd, sk) -> enc(plain, rnd, pk)] (compute inverse op)
            let computed_cipher =
                self.crypto_enc(crypto_backend_id, plain, key, rnd, true, self.cg());

            let exp_cipher = self.get_arr(cipher);
            let exp_cipher_is_non_zero = self.is_non_zero(&exp_cipher, cipher); // "!= 0"
            let exp_cipher_is_zero = exp_cipher_is_non_zero
                .inv_as_bit_with_option(&Some(format!("{cipher} == 0")))
                .unwrap();
            let plain_zero = self.is_zero(&self.get_arr(plain), plain);
            let rnd_zero = self.is_zero(&self.get_arr(rnd), rnd);

            // 2. Check that: expected_cipher == 0 => plain == 0 && rnd == 0
            self.add_guarded_one_assertion(
                &exp_cipher_is_non_zero.orw(&plain_zero.and(&rnd_zero)),
                &None,
            );

            // 3. Check that expected_cipher != 0 => expected_cipher == computed_cipher
            self.add_guarded_one_assertion(
                &exp_cipher_is_zero.orw(&self.is_equal(
                    &exp_cipher,
                    cipher,
                    &computed_cipher,
                    "cipher",
                )),
                &None,
            );
        }
    }

    //Symmetric Decryption

    pub fn check_symm_dec(
        &mut self,
        crypto_backend_id: &str,
        plain: &str,
        other_pk: &str,
        iv_cipher: &str,
        generator: RcCell<CircuitGenerator>,
    ) {
        // let mut crypto_backend = self.get_crypto_backend(crypto_backend_id);

        // 1. Decrypt [dec(cipher, rnd, sk) -> encSymm(plain, ecdh(my_sk, other_pk), iv)] (compute inverse op)
        let computed_cipher = self.crypto_symm_enc(
            crypto_backend_id,
            plain,
            other_pk,
            iv_cipher,
            true,
            generator,
        );

        let exp_iv_cipher = self.get_arr(iv_cipher);
        let exp_cipher_non_zero = self.is_non_zero(&exp_iv_cipher, iv_cipher);
        let exp_cipher_zero = exp_cipher_non_zero
            .inv_as_bit_with_option(&Some(format!("{iv_cipher} == 0")))
            .unwrap();
        let other_pk_non_zero = self
            .get(other_pk)
            .wire
            .check_non_zero_with_option(&Some(other_pk.to_owned() + "!= 0"));
        let other_pk_zero = other_pk_non_zero
            .inv_as_bit_with_option(&Some(format!("{other_pk} == 0")))
            .unwrap();
        let plain_zero = self.is_zero(&self.get_arr(plain), plain);

        // Some of these checks are probably not necessary, as zkay should already enforce that
        // other_pk == 0 <=> exp_cipher == 0

        // 2. Check that: iv_cipher == 0 => plain == 0 && other_pk == 0
        self.add_guarded_one_assertion(
            &exp_cipher_non_zero.orw(&plain_zero.and(&other_pk_zero)),
            &ADD_OP_LABELS
                .then(|| format!("{} == 0 => {} == 0 && {} == 0", iv_cipher, plain, other_pk)),
        );

        // 3. Check that: other_pk == 0 => plain == 0 && iv_cipher == 0
        self.add_guarded_one_assertion(
            &other_pk_non_zero.orw(&plain_zero.and(&exp_cipher_zero)),
            &ADD_OP_LABELS
                .then(|| format!("{} == 0 => {} == 0 && {} == 0", other_pk, plain, iv_cipher)),
        );

        // 4. Check that: (iv_cipher != 0 && other_pk != 0) => iv_cipher == computed_cipher
        let cipher_zero_or_pk_zero = exp_cipher_zero.orw(&other_pk_zero);
        self.add_guarded_one_assertion(
            &cipher_zero_or_pk_zero.orw(&self.is_equal(
                &exp_iv_cipher,
                iv_cipher,
                &computed_cipher,
                "cipher",
            )),
            &ADD_OP_LABELS.then(|| {
                format!(
                    "({} != 0 && {} != 0) => {} == {}",
                    iv_cipher, other_pk, iv_cipher, "cipher"
                )
            }),
        );
    }

    // Legacy handling

    pub fn check_encs(&mut self, plain: &str, key: &str, rnd: &str, expected_cipher: &str) {
        self.check_enc(LEGACY_CRYPTO_BACKEND, plain, key, rnd, expected_cipher);
    }

    pub fn check_encss(&mut self, plain: &str, other_pk: &str, iv_cipher: &str) {
        self.check_symm_enc(LEGACY_CRYPTO_BACKEND, plain, other_pk, iv_cipher, self.cg());
    }

    pub fn check_decs(&mut self, plain: &str, key: &str, rnd: &str, expected_cipher: &str) {
        self.check_dec(LEGACY_CRYPTO_BACKEND, plain, key, rnd, expected_cipher);
    }

    pub fn check_decsss(&mut self, plain: &str, other_pk: &str, iv_cipher: &str) {
        self.check_symm_dec(LEGACY_CRYPTO_BACKEND, plain, other_pk, iv_cipher, self.cg());
    }

    pub fn check_eq(&self, lhs: &str, rhs: &str) {
        let (l, r) = (self.get_arr(lhs), self.get_arr(rhs));
        let len = l.len();
        assert!(r.len() == len, "Size mismatch for equality check");

        for i in 0..len {
            let comp_str = if ADD_OP_LABELS {
                &Some(format!(
                    "{}[{}] == {}[{}]",
                    self.get_qualified_name(lhs),
                    i,
                    self.get_qualified_name(rhs),
                    i
                ))
            } else {
                &None
            };
            self.add_guarded_equality_assertion(
                l[i].as_ref().unwrap(),
                r[i].as_ref().unwrap(),
                comp_str,
            );
        }
    }

    pub fn is_non_zero(&self, value: &Vec<Option<WireType>>, name: &str) -> WireType {
        let mut res = value[0]
            .as_ref()
            .unwrap()
            .check_non_zero_with_option(&Some(name.to_owned() + "[0] != 0"));
        for i in 1..value.len() {
            res = res.addw_with_option(
                &value[i]
                    .as_ref()
                    .unwrap()
                    .check_non_zero_with_option(&Some(format!("{}[{}] != 0", name, i))),
                &Some(format!("or {name}[{i}] != 0")),
            );
        }
        res.check_non_zero_with_option(&Some(name.to_owned() + " != 0"))
    }

    pub fn is_zero(&self, value: &Vec<Option<WireType>>, name: &str) -> WireType {
        self.is_non_zero(value, name)
            .inv_as_bit_with_option(&Some(format!("{name} == 0")))
            .unwrap()
    }

    pub fn is_equal(
        &self,
        wires1: &Vec<Option<WireType>>,
        name1: &str,
        wires2: &Vec<Option<WireType>>,
        name2: &str,
    ) -> WireType {
        assert!(wires1.len() == wires2.len(), "WireType array size mismatch");
        let mut res = self.get_one_wire().unwrap();
        for i in 0..wires1.len() {
            res = res.and(&wires1[i].as_ref().unwrap().is_equal_tos_with_option(
                wires2[i].as_ref().unwrap(),
                &Some(format!("{}[{}] == {}[{}]", name1, i, name2, i)),
            ));
        }
        res
    }

    pub fn clear_prefix(prefix: &mut VecDeque<String>, indices: &mut HashMap<String, i32>) {
        prefix.clear();
        prefix.push_front("".to_owned());
        indices.clear();
    }

    pub fn push_prefix(
        prefix: &mut VecDeque<String>,
        prefix_indices: &mut HashMap<String, i32>,
        new_str: &str,
    ) {
        let new_prefix = format!("{}{}.", prefix.front().unwrap(), new_str);
        let count = *prefix_indices.get(&new_prefix).unwrap_or(&0);
        prefix_indices.insert(new_prefix.clone(), count + 1);
        prefix.push_front(format!("{}{}.", new_prefix, count));
    }

    pub fn push_guard_prefix(
        guard_prefixes: &mut VecDeque<VecDeque<String>>,
        guard_prefix_indices: &mut VecDeque<HashMap<String, i32>>,
    ) {
        let mut new_prefix = VecDeque::new();
        let mut new_prefix_indices = HashMap::new();
        Self::clear_prefix(&mut new_prefix, &mut new_prefix_indices);
        guard_prefixes.push_front(new_prefix);
        guard_prefix_indices.push_front(new_prefix_indices);
    }

    pub fn get_qualified_name(&self, name: &str) -> String {
        if name.starts_with("glob_") {
            name.to_owned()
        } else {
            self.t.name_prefix.front().unwrap().clone() + &name
        }
    }

    pub fn add_guarded_equality_assertion(
        &self,
        lhs: &WireType,
        rhs: &WireType,
        desc: &Option<String>,
    ) {
        if self.t.current_guard_condition.is_empty() {
            CircuitGenerator::add_equality_assertion_with_option(self.cg(), lhs, rhs, desc);
        } else {
            let eq = lhs.is_equal_tos(rhs);
            CircuitGenerator::add_one_assertion_with_option(
                self.cg(),
                &self
                    .t
                    .current_guard_condition
                    .front()
                    .unwrap()
                    .wire
                    .inv_as_bit()
                    .unwrap()
                    .orw(&eq),
                desc,
            ); // guard => lhs == rhs
        }
    }

    pub fn add_guarded_one_assertion(&self, val: &WireType, desc: &Option<String>) {
        if self.t.current_guard_condition.is_empty() {
            CircuitGenerator::add_one_assertion_with_option(self.cg(), val, desc);
        } else {
            CircuitGenerator::add_one_assertion_with_option(
                self.cg(),
                &self
                    .t
                    .current_guard_condition
                    .front()
                    .unwrap()
                    .wire
                    .inv_as_bit()
                    .unwrap()
                    .orw(val),
                desc,
            ); // guard => val
        }
    }

    pub fn get_typed_arr(&self, name: &str) -> &Vec<TypedWire> {
        let name = self.get_qualified_name(name);
        self.t
            .vars
            .get(&name)
            .expect(&format!("Variable {name} is not associated with a wire"))
    }

    pub fn get_arr(&self, name: &str) -> Vec<Option<WireType>> {
        self.get_typed_arr(name)
            .iter()
            .map(|v| Some(v.wire.clone()))
            .collect()
    }

    pub fn set(&mut self, name: &str, val: &TypedWire) {
        self.set_svt(name, &vec![val.clone()]);
    }

    pub fn set_svt(&mut self, name: &str, val: &Vec<TypedWire>) {
        let name = self.get_qualified_name(name);
        assert!(!val.is_empty(), "Tried to set value {name} to None");
        assert!(
            self.t.vars.insert(name.clone(), val.clone()).is_none(),
            "SSA violation when trying to write to {name}"
        );
    }
}

pub fn s_negate(val: &TypedWire, generator: &RcCell<CircuitGenerator>) -> TypedWire {
    let bits = val.zkay_type.bitwidth;
    if bits < 256 {
        // Take two's complement
        let inv_bits = TypedWire::new(
            val.wire.inv_bits(val.zkay_type.bitwidth as u64),
            val.zkay_type.clone(),
            format!("~{}", val.name),
            &vec![],
            generator.clone(),
        );
        inv_bits.plus(&s_val_iz(1, val.zkay_type.clone(), generator))
    } else {
        TypedWire::new(
            val.wire
                .muli_with_option(-1, &Some(format!("-{}", val.name))),
            val.zkay_type.clone(),
            format!("-{}", val.name),
            &vec![],
            generator.clone(),
        )
    }
}

pub fn s_val_iz(val: i32, t: ZkayType, generator: &RcCell<CircuitGenerator>) -> TypedWire {
    if val == 0 {
        TypedWire::new(
            generator.get_zero_wire().unwrap(),
            t,
            format!("const_{val}"),
            &vec![],
            generator.clone(),
        )
    } else if val == 1 {
        TypedWire::new(
            generator.get_one_wire().unwrap(),
            t,
            format!("const_{val}"),
            &vec![],
            generator.clone(),
        )
    } else {
        s_val_sz(&val.to_string(), t, generator)
    }
}

pub fn s_val_sz(val: &str, t: ZkayType, generator: &RcCell<CircuitGenerator>) -> TypedWire {
    let v = Util::parse_big_int(val);
    let w = if v.sign() == Sign::Minus {
        assert!(!t.signed, "Cannot store negative constant in unsigned wire");
        let v_neg = ZkayType::_get_negative_constant(&v.clone().neg(), t.bitwidth);
        assert!(v_neg.sign() != Sign::Minus, "Constant is still negative");
        CircuitGenerator::create_constant_wire_with_option(
            generator.clone(),
            &v_neg,
            &Some(format!("const_{v}")),
        )
    } else {
        CircuitGenerator::create_constant_wire_with_option(
            generator.clone(),
            &v,
            &Some(format!("const_{v}")),
        )
    };
    TypedWire::new(w, t, format!("const_{v}"), &vec![], generator.clone())
}
