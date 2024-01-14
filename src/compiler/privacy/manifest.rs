//::json
//::os
// from contextlib::contextmanager
// from typing::ContextManager

use crate::config::CFG
use crate::utils::progress_printer::warn_print;

pub struct  Manifest;
impl Manifest{
    // """Static class, which holds the string keys of all supported zkay manifest keys """
    pub const zkay_version:&str = "zkay-version";
    pub const solc_version:&str = "solc-version";
    pub const zkay_options:&str = "zkay-options";

    // @staticmethod
    pub fn load(project_dir:&str)
        // """Returned parsed manifest json file located in project dir::"""
     {   let f= open(os::path::join(project_dir, "manifest::json"));
           let  j = json::loads(f::read());
         j}

    // @staticmethod
    pub fn import_manifest_config(manifest:&str)
        // Check if zkay version matches
       { if manifest[Manifest::zkay_version] != CFG.lock().unwrap().zkay_version
            { warn_print();
                print!(
                    "Zkay version in manifest ({manifest[Manifest::zkay_version]}) does not match current zkay version ({CFG.lock().unwrap().zkay_version})\nCompilation or integrity check with deployed bytecode might fail due to version differences");}

        CFG.lock().unwrap().override_solc(manifest[Manifest::solc_version]);
        CFG.lock().unwrap().import_compiler_settings(manifest[Manifest::zkay_options]);}

    // @staticmethod
    // @contextmanager
    pub fn with_manifest_config(manifest:&str)
       { let old_solc = CFG.lock().unwrap().solc_version;
        let old_settings = CFG.lock().unwrap().export_compiler_settings();
        // try
            Manifest::import_manifest_config(manifest);
            // yield
        // finally
            CFG.lock().unwrap().override_solc(old_solc);
            CFG.lock().unwrap().import_compiler_settings(old_settings);}
}