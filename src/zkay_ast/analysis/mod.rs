// """
// This package contains modules implementing static analysis.

// ==========
// Submodules
// ==========
// * :py:mod:`.alias_analysis`: Alias analysis to determine whether the privacy labels at different locations refer to the same address.
// * :py:mod:`.call_graph`: Compute sets of transitively called functions for each function.
// * :py:mod:`.circuit_compatibility_checker`: Determine whether the private parts of an AST can be expressed using proof circuits.
// * :py:mod:`.contains_private_checker`: Determine whether element contains any private expressions.
// * :py:mod:`.hybrid_function_detector`: Determine which functions require verification.
// * :py:mod:`.loop_checker`: Ensure that loops do not contain private expressions.
// * :py:mod:`.partition_state`: Helper class to store alias analysis state.
// * :py:mod:`.return_checker`: Ensure that there is at most one return statement per function at the end of the body.
// * :py:mod:`.side_effects`: Determine whether element contains side effects.
// """
pub mod alias_analysis;
pub mod call_graph;
pub mod circuit_compatibility_checker;
pub mod hybrid_function_detector;
pub mod loop_checker;
pub mod partition_state;
pub mod return_checker;
pub mod side_effects;
