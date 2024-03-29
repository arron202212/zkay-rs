// """
// The main zkay package.

// ==========
// Submodules
// ==========
// * :py:mod:`.__main__`: Zkay command line interface
// * :py:mod:`.config`: Global zkay configuration repository for all settings \
//                      (both user-configuration as well as zkay-internal configuration)
// * :py:mod:`.config_version`: Contains pinned version numbers (e.g. solc) used by zkay
// * :py:mod:`.config_user`: Defines options which the user can configure.
// * :py:mod:`.zkay_frontend`: Programmatic access to zkay compilation, transaction and packaging facilities.

// ===========
// Subpackages
// ===========
// * :py:mod:`.compiler`: Internal compilation functionality
// * :py:mod:`.errors`: Defines exceptions which may be raised by public zkay interfaces
// * :py:mod:`.jsnark_interface`: Glue code for interacting with the external jsnark and libsnark interfaces
// * :py:mod:`.my_logging`: Logging facilities
// * :py:mod:`.transaction`: Runtime API (used by the auto-generated offchain transaction simulator classes)
// * :py:mod:`.type_check`: Internal type-checking and program analysis functionality
// * :py:mod:`.utils`: Internal helper functionality
// * :py:mod:`.zkay_ast`: AST-related functionality
// """
pub mod config;
pub mod config_user;
pub mod config_version;
pub mod meta;
