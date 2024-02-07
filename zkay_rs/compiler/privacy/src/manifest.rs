//::json
//::os
// from contextlib::contextmanager
// from typing::ContextManager

use zkay_config::config::CFG;
use zkay_utils::progress_printer::warn_print;
use serde_json::{Map, Result, Value};
use std::fs::File;
use std::io::Read;
use std::path::Path;
pub struct Manifest;
impl Manifest {
    // """Static class, which holds the string keys of all supported zkay manifest keys """
    pub const zkay_version: &str = "zkay-version";
    pub const solc_version: &str = "solc-version";
    pub const zkay_options: &str = "zkay-options";

    // @staticmethod
    pub fn load(project_dir: &str) -> Value
// """Returned parsed manifest json file located in project dir::"""
    {
        let f = File::open(Path::new(project_dir).join("manifest::json"));
        let mut s = String::new();
        f.unwrap().read_to_string(&mut s).unwrap();
        let j: Value = serde_json::from_str(&s).unwrap();
        j
    }

    // @staticmethod
    pub fn import_manifest_config(manifest: Value)
    // Check if zkay version matches
    {
        if let Value::Object(manifest) = manifest {
            if let Some(v) = manifest.get(&Manifest::zkay_version.to_string()) {
                if v != &Value::String(CFG.lock().unwrap().zkay_version()) {
                    warn_print();
                    print!(
                    "Zkay version in manifest ({:?}) does not match current zkay version ({})\nCompilation or integrity check with deployed bytecode might fail due to version differences",v,CFG.lock().unwrap().zkay_version());
                }
            }

            CFG.lock().unwrap().override_solc(
                manifest[&Manifest::solc_version.to_string()]
                    .as_str()
                    .unwrap()
                    .to_string(),
            );
            CFG.lock()
                .unwrap()
                .import_compiler_settings(manifest[&Manifest::zkay_options.to_string()].clone());
        }
    }

    // @staticmethod
    // @contextmanager
    pub fn with_manifest_config(manifest: &str) {
        let old_solc = CFG.lock().unwrap().solc_version();
        let old_settings = CFG.lock().unwrap().export_compiler_settings();
        // try
        Manifest::import_manifest_config(Value::String(manifest.to_string()));
        // yield
        // finally
        CFG.lock().unwrap().override_solc(old_solc);
        CFG.lock().unwrap().import_compiler_settings(old_settings);
    }
}
