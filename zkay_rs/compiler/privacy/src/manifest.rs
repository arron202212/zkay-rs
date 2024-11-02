#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

//::json
//::os
// from contextlib::contextmanager
// from typing::ContextManager

use serde_json::{Map, Result, Value};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use zkay_config::{config::CFG, config_version::Versions, with_context_block};
use zkay_utils::progress_printer::warn_print;
pub struct Manifest;
impl Manifest {
    // """Static class, which holds the string keys of all supported zkay manifest keys """
    pub const zkay_version: &'static str = "zkay-version";
    pub const solc_version: &'static str = "solc-version";
    pub const zkay_options: &'static str = "zkay-options";

    // @staticmethod
    // """Returned parsed manifest json file located in project dir::"""
    pub fn load(project_dir: &str) -> Value {
        let f = File::open(Path::new(project_dir).join("manifest::json"));
        let mut s = String::new();
        f.unwrap().read_to_string(&mut s).unwrap();
        let j: Value = serde_json::from_str(&s).unwrap();
        j
    }

    // @staticmethod
    // Check if zkay version matches
    pub fn import_manifest_config(manifest: Value) {
        if let Value::Object(manifest) = manifest {
            if let Some(v) = manifest.get(&Manifest::zkay_version.to_string()) {
                if v != &Value::String(CFG.lock().unwrap().zkay_version()) {
                    with_context_block!(var _wp=warn_print()=>
                    {print!(
                    "Zkay version in manifest ({:?}) does not match current zkay version ({})\nCompilation or integrity check with deployed bytecode might fail due to version differences",v,CFG.lock().unwrap().zkay_version());
                    });
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
    pub fn with_manifest_config(manifest: &str) -> WithManifestConfig {
        // try
        // yield
        // finally
        WithManifestConfig::new(manifest)
    }
}

pub struct WithManifestConfig {
    old_solc: Option<String>,
    old_settings: Value,
}
impl WithManifestConfig {
    pub fn new(manifest: &str) -> Self {
        let old_solc = Some(CFG.lock().unwrap().solc_version());
        let old_settings = CFG.lock().unwrap().export_compiler_settings();
        Manifest::import_manifest_config(Value::String(manifest.to_string()));
        Self {
            old_solc,
            old_settings,
        }
    }
}
impl Drop for WithManifestConfig {
    fn drop(&mut self) {
        CFG.lock()
            .unwrap()
            .override_solc(self.old_solc.clone().unwrap());
        CFG.lock()
            .unwrap()
            .import_compiler_settings(self.old_settings.clone());
    }
}
