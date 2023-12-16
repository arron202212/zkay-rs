// use crate::compiler::solidity::compiler::check_for_zkay_solc_errors, SolcException;
// use crate::config::cfg;
// use crate::errors::exceptions::ZkayCompilerError, PreprocessAstException, TypeCheckException, AnalysisException, \;
//     ZkaySyntaxError;
// use crate::solidity_parser::parse::SyntaxException;
// use crate::type_check::type_checker::type_check as t;
// use crate::type_check::type_exceptions::TypeMismatchException, TypeException, RequireException, ReclassifyException;
// use crate::utils::progress_printer::print_step;
// use crate::zkay_ast::analysis::alias_analysis::alias_analysis as a;
// use crate::zkay_ast::analysis::call_graph::call_graph_analysis;
// use crate::zkay_ast::analysis::circuit_compatibility_checker::check_circuit_compliance;
// use crate::zkay_ast::analysis::hybrid_function_detector::detect_hybrid_functions;
// use crate::zkay_ast::analysis::loop_checker::check_loops;
// use crate::zkay_ast::analysis::return_checker::check_return as r;
// use crate::zkay_ast::analysis::side_effects::compute_modified_sets, check_for_undefined_behavior_due_to_eval_order;
// use crate::zkay_ast::ast::AST, SourceUnit, AstException;
use crate::zkay_ast::build_ast::build_ast;
// use crate::zkay_ast::pointers::parent_setter::set_parents;
// use crate::zkay_ast::pointers::pointer_exceptions::UnknownIdentifierException;
// use crate::zkay_ast::pointers::symbol_table::link_identifiers as link

fn get_parsed_ast_and_fake_code(code: &str, solc_check: bool) //-> (SourceUnit, str)
{
    //  print_step("Parsing");
    let _ast = build_ast(code);
    // except SyntaxException as e:
    //     raise ZkaySyntaxError(f'\n\nSYNTAX ERROR: {e}')

    // from zkay.compiler.solidity.fake_solidity_generator import fake_solidity_code
    // fake_code = fake_solidity_code(str(code))
    // if solc_check:
    //    // Solc type checking
    //     with print_step("Type checking with solc"):
    //         try:
    //             check_for_zkay_solc_errors(code, fake_code)
    //         except SolcException as e:
    //             raise ZkayCompilerError(f'{e}')
    // ( ast, fake_code)
}

//parents:bool, link_identifiers:bool, check_return:bool, alias_analysis:bool, type_check:bool, solc_check:bool
pub fn get_processed_ast(code: &str, flag: Option<i32>) //-> SourceUnit
{
    // ast, _ =
    get_parsed_ast_and_fake_code(code, (flag.unwrap_or(0) >> 5) == 1); //solc_check

    // Zkay preprocessing and type checking
    // process_ast(ast, parents, link_identifiers, check_return, alias_analysis, type_check);

    // ast
}

// fn process_ast(ast, parents:bool, link_identifiers:bool, check_return:bool, alias_analysis:bool, type_check:bool){
//     with print_step("Preprocessing AST"):
//         if parents:
//             set_parents(ast)
//         if link_identifiers:
//             try:
//                 link(ast)
//             except UnknownIdentifierException as e:
//                 raise PreprocessAstException(f'\n\nSYMBOL ERROR: {e}')
//         try:
//             if check_return:
//                 r(ast)
//             if alias_analysis:
//                 a(ast)
//             call_graph_analysis(ast)
//             compute_modified_sets(ast)
//             check_for_undefined_behavior_due_to_eval_order(ast)
//         except AstException as e:
//             raise AnalysisException(f'\n\nANALYSIS ERROR: {e}')
//     if type_check:
//         with print_step("Zkay type checking"):
//             try:
//                 t(ast)
//                 check_circuit_compliance(ast)
//                 detect_hybrid_functions(ast)
//                 check_loops(ast)
//             except (TypeMismatchException, TypeException, RequireException, ReclassifyException) as e:
//                 raise TypeCheckException(f'\n\nCOMPILER ERROR: {e}')
// }

// def get_verification_contract_names(code_or_ast) -> List[str]:
//     if isinstance(code_or_ast, str):
//         ast = get_processed_ast(code_or_ast)
//     else:
//         ast = code_or_ast
//     if not isinstance(ast, SourceUnit):
//         raise ZkayCompilerError('Invalid AST (no source unit at root)')

//     vc_names = []
//     for contract in ast.contracts:
//         cname = contract.idf.name
//         fcts = [fct for fct in contract.function_definitions + contract.constructor_definitions if fct.requires_verification_when_external and fct.has_side_effects]
//         vc_names += [cfg.get_verification_contract_name(cname, fct.name) for fct in fcts]
//     return vc_names
