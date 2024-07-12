// """
// This module defines pinned versions and is used internally to configure the concrete solc version to use
// """
// import os
// import sys

// from semantic_version import NpmSpec, Version
use semver_rs::Version;
// lazy_static!{
//     pub static ref zkay_solc_version_compatibility:Version=Version::new("^0.6.0").parse().expect("zkay_solc_version_compatibility");
//     pub static ref ZKAY_VERSION:&'static str=include_str!("./VERSION");
// }
pub struct Versions {
    pub solc_version: Option<String>,
    pub zkay_solc_version_compatibility: Version,
}
impl Default for Versions {
    fn default() -> Self {
        Self::new()
    }
}
impl Versions {
    // pub const zkay_solc_version_compatibility:Version=Version::new("^0.6.0").parse().expect("zkay_solc_version_compatibility");
    pub const ZKAY_SOLC_VERSION_COMPATIBILITY: &'static str = "^0.6.0";
    pub const ZKAY_LIBRARY_SOLC_VERSION: &'static str = "v0.6.12";
    pub const ZKAY_VERSION: &'static str = include_str!("./VERSION");
    pub const SOLC_VERSION: &'static str = "v0.6.12";
    // pub const solc_version:Option<&'static str>=Some("0.6.12");
    pub fn new() -> Self {
        Self {
            solc_version: None,
            zkay_solc_version_compatibility: Version::new("^0.6.0")
                .parse()
                .expect("zkay_solc_version_compatibility"),
        }
    }
    pub fn set_solc_version(&mut self, version: String) {
        self.solc_version = Some(version);
        // .strip_prefix('v')
        // .map(|version| version.to_string())
        // .or(Some(version));
    }
}
// class Versions:
//     zkay_solc_version_compatibility = NpmSpec('^0.6.0')
//     ZKAY_LIBRARY_SOLC_VERSION = '0.6.12'
//     solc_version = None

//     # Read zkay version from VERSION file
//     with open(os.path.join(os.path.realpath(os.path.dirname(__file__)), 'VERSION')) as f:
//         ZKAY_VERSION = f.read().strip()

//     @staticmethod
//     def set_solc_version(version: str):
//         version = version[1:] if version.startswith('v') else version

//         import solcx
//         from solcx.exceptions import SolcNotInstalled
//         if version == 'latest':
//             try:
//                 solcx.set_solc_version_pragma(Versions.zkay_solc_version_compatibility.expression, silent=True, check_new=False)
//             except SolcNotInstalled:
//                 print('ERROR: No compatible solc version is installed.\n'
//                       'Please use "zkay update-solc" to install the latest compatible solc version.')
//                 sys.exit(100)
//         else:
//             try:
//                 v = Version(version)
//                 if not Versions.zkay_solc_version_compatibility.match(v):
//                     raise ValueError(f'Zkay only supports solc versions satisfying {Versions.zkay_solc_version_compatibility.expression}')
//                 solcx.set_solc_version(version, silent=True)
//             except ValueError as e:
//                 raise ValueError(f'Invalid version string {version}\n{e}')
//             except SolcNotInstalled:
//                 try:
//                     solcx.install_solc(version)
//                     solcx.set_solc_version(version, silent=True)
//                 except Exception as e:
//                     print(f'ERROR: Error while trying to install solc version {version}\n{e.args}')
//                     sys.exit(101)

//         Versions.solc_version = f"v{solcx.get_solc_version().truncate(level='patch')}"
