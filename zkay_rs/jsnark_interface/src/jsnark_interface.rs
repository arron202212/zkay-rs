#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
//::os
// from typing::List
use circuit_helper_config::circuit_helper_config::CircuitHelperConfig;
use lazy_static::lazy_static;
use rccell::RcCell;
use std::{fs::File, io::Write, path::Path};
use zkay_config::config::{CFG, indent};
use zkay_utils::{
    helpers::hash_file,
    run_command::{run_command, run_commands},
};
#[macro_export]
macro_rules! file_abs_workspace {
    () => {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
            $crate::jsnark_interface::pop_first_two_path_components(file!()),
        )
    };
}

pub fn pop_first_two_path_components(path: &str) -> std::path::PathBuf {
    let mut components = std::path::Path::new(path).components();
    components.next();
    components.next();
    components.as_path().to_path_buf()
}
//path to jsnark interface jar
pub const CIRCUIT_BUILDER_JAR: &str = "JsnarkCircuitBuilder.jar";
lazy_static! {
    pub static ref JARS_DIR: String = file_abs_workspace!()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    pub static ref CIRCUIT_BUILDER_JAR_HASH: String = hex::encode(hash_file(
        &(JARS_DIR.clone() + "/" + CIRCUIT_BUILDER_JAR),
        0
    ));
}
// """
// Compile the given circuit java code and then compile the circuit which it describes using jsnark.

// :param circuit_dir: output directory
// :param javacode: circuit code (java class which uses the custom jsnark wrapper API)
// :raise SubprocessError: if compilation fails
// """
pub fn compile_circuit(circuit_dir: &str, javacode: &str) {
    let class_name = CFG.lock().unwrap().jsnark_circuit_classname();
    let jfile = Path::new(circuit_dir).join(class_name.clone() + ".java");
    let mut f = File::create(jfile.clone()).expect(jfile.as_path().to_str().expect("jfile"));
    let _ = f.write_all(javacode.as_bytes());

    compile_and_run_with_circuit_builder(
        circuit_dir,
        &class_name,
        jfile.to_str().unwrap(),
        vec!["compile"],
    );
}
//Compile the circuit java file

pub fn compile_and_run_with_circuit_builder(
    working_dir: &str,
    class_name: &str,
    java_file_name: &str,
    args: Vec<&str>,
) -> (Option<String>, Option<String>) {
    run_command(
        vec![
            "javac",
            "-cp",
            &format!("{}", (JARS_DIR.clone() + "/" + CIRCUIT_BUILDER_JAR)),
            java_file_name,
        ],
        Some(working_dir),
        false,
    );
    //Run jsnark to generate the circuit
    return run_command(
        [
            "java",
            "-Xms4096m",
            "-Xmx16384m",
            "-cp",
            &format!(
                "{}:{working_dir}",
                (JARS_DIR.clone() + "/" + CIRCUIT_BUILDER_JAR)
            ),
            class_name,
        ]
        .into_iter()
        .chain(args)
        .collect(),
        Some(working_dir),
        true,
    );
}
// """
// Generate a libsnark circuit input file by evaluating the circuit in jsnark using the provided input values.

// :param circuit_dir: directory where the compiled circuit is located
// :param output_dir: directory, where to store the jsnark output files
// :param serialized_args: public inputs, public outputs and private inputs in the order in which they are defined in the circuit
// :raise SubprocessError: if circuit evaluation fails
// """
pub fn prepare_proof(
    circuit_dir: &str,
    output_dir: &str,
    serialized_args: Vec<String>,
) -> (Option<String>, Option<String>) {
    let serialized_arg_str: Vec<_> = serialized_args
        .iter()
        .map(|arg| format!("{}", arg))
        .collect();

    //Run jsnark to evaluate the circuit and compute prover inputs
    run_commands(
        [
            "java",
            "-Xms4096m",
            "-Xmx16384m",
            "-cp",
            &format!("{CIRCUIT_BUILDER_JAR}:{circuit_dir}"),
            &CFG.lock().unwrap().jsnark_circuit_classname(),
            "prove",
        ]
        .into_iter()
        .map(String::from)
        .chain(serialized_arg_str)
        .collect(),
        Some(output_dir),
        true,
    )
}

// """Java circuit code template"""
// """
// Inject circuit and input code into jsnark-wrapper skeleton.

// :param circuit: the abstract circuit to which this java code corresponds
// :param fdefs: java code that calls addCryptoBackend for each used crypto backend
// :param fdefs: java function definition with circuit code for each transitively called function (public calls in this circuit"s function)
// :param circuit_statements: the java code corresponding to this circuit
// :return: complete java file as string
// """
pub fn get_jsnark_circuit_class_str<T: CircuitHelperConfig>(
    circuit: &T,
    crypto_init_stmts: Vec<String>,
    fdefs: Vec<String>,
    circuit_statements: Vec<String>,
) -> String {
    let circuit_class_name = CFG.lock().unwrap().jsnark_circuit_classname();
    let use_input_hashing = CFG
        .lock()
        .unwrap()
        .should_use_hash(circuit.trans_in_size() + circuit.trans_out_size())
        .to_string()
        .to_ascii_lowercase();
    let mut function_definitions = fdefs.join("\n\n");
    if !function_definitions.is_empty() {
        function_definitions = format!("\n{function_definitions}\n");
    }

    return format!(
        r#"
import zkay.ZkayCircuitBase;
import zkay.HomomorphicInput;
import static zkay.ZkayType.ZkUint;
import static zkay.ZkayType.ZkInt;
import static zkay.ZkayType.ZkBool;

public class {circuit_class_name} extends ZkayCircuitBase {{
    public {circuit_class_name}() {{
        super("{circuit_name}", {pub_in_size}, {pub_out_size}, {priv_in_size}, {use_input_hashing});
{crypto_init_stmts}
    }}
{fdefs}
    @Override
    protected void buildCircuit() {{
        super.buildCircuit();
{circuit_statements}
    }}

    public static void main(String[] args) {{
        {circuit_class_name} circuit = new {circuit_class_name}();
        circuit.run(args);
    }}
}}
"#,
        circuit_name = circuit.get_verification_contract_name(),
        crypto_init_stmts = indent(indent(crypto_init_stmts.join("\n"))),
        pub_in_size = circuit.in_size_trans(),
        pub_out_size = circuit.out_size_trans(),
        priv_in_size = circuit.priv_in_size_trans(),
        fdefs = indent(function_definitions),
        circuit_statements = indent(indent(circuit_statements.join("\n")))
    );
}
