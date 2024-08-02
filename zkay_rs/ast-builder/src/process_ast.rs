#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use rccell::RcCell;
use solidity::compiler::check_for_zkay_solc_errors;
//, SolcException;
use zkay_config::config::CFG;
// use crate::errors::exceptions::ZkayCompilerError, PreprocessAstException, TypeCheckException, AnalysisException,ZkaySyntaxError;
// use solidity_parser::parse::SyntaxException;
use type_check::type_checker::type_check as t;
// use type_check::type_exceptions::TypeMismatchException, TypeException, RequireException, ReclassifyException;
use crate::build_ast::build_ast;
use zkay_ast::analysis::{
    alias_analysis::alias_analysis as a,
    call_graph::call_graph_analysis,
    circuit_compatibility_checker::check_circuit_compliance,
    hybrid_function_detector::detect_hybrid_functions,
    loop_checker::check_loops,
    return_checker::check_return as r,
    side_effects::{check_for_undefined_behavior_due_to_eval_order, compute_modified_sets},
};
use zkay_ast::ast::{ASTBaseProperty, ASTFlatten, IdentifierBaseProperty, SourceUnit, AST}; //, AstException;
use zkay_ast::pointers::{parent_setter::set_parents, symbol_table::link_identifiers as link};
use zkay_utils::progress_printer::print_step;
// use crate::pointers::pointer_exceptions::UnknownIdentifierException;
use bitflags::bitflags;
use std::fmt;
use zkay_ast::global_defs::{
    array_length_member, global_defs, global_vars, GlobalDefs, GlobalVars,
};
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ASTFlags(u32);
bitflags! {
    impl ASTFlags: u32 {
        const PARENTS           = 0b00000001;
        const LINK_IDENTIFIERS  = 0b00000010;
        const CHECK_RETURN      = 0b00000100;
        const ALIAS_ANALYSIS      = 0b00100000;
        const TYPE_CHECK        = 0b00001000;
        const SOLC_CHECK        = 0b00010000;
        const FLAG_ALL    = Self::PARENTS.bits()
                           | Self::LINK_IDENTIFIERS.bits()
                           | Self::CHECK_RETURN.bits()
                           | Self::ALIAS_ANALYSIS.bits()
                           | Self::TYPE_CHECK.bits()
                           | Self::SOLC_CHECK.bits();
    }
}
#[warn(dead_code)]
impl ASTFlags {
    pub fn new(flag: Option<u32>) -> Self {
        Self(flag.unwrap_or(0x3f))
    }
    // pub fn clear(&mut self) -> &mut ASTFlags {
    //     self
    // }
    pub fn parents(&self) -> bool {
        *self & Self::PARENTS == Self::PARENTS
    }
    pub fn link_identifiers(&self) -> bool {
        *self & Self::LINK_IDENTIFIERS == Self::LINK_IDENTIFIERS
    }
    pub fn check_return(&self) -> bool {
        *self & Self::CHECK_RETURN == Self::CHECK_RETURN
    }
    pub fn alias_analysis(&self) -> bool {
        *self & Self::ALIAS_ANALYSIS == Self::ALIAS_ANALYSIS
    }
    pub fn type_check(&self) -> bool {
        *self & Self::TYPE_CHECK == Self::TYPE_CHECK
    }

    // pub fn solc_check(&self) -> bool {
    //     *self & Self::SOLC_CHECK == Self::SOLC_CHECK
    // }
}

impl fmt::Display for ASTFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:032b}", self.bits())
    }
}
fn get_parsed_ast_and_fake_code(code: &str, solc_check: bool) -> (ASTFlatten, String) {
    print_step("Parsing");
    let _ast = build_ast(code);
    // except SyntaxException as e:
    //     raise ZkaySyntaxError(f"\n\nSYNTAX ERROR: {e}")

    let fake_code = solidity::fake_solidity_generator::fake_solidity_code(code);
    if solc_check {
        // Solc type checking
        print_step("Type checking with solc");
        // try:
        check_for_zkay_solc_errors(code, &fake_code);
        // except SolcException as e:
        //     raise ZkayCompilerError(f"{e}")
    }
    (_ast, fake_code)
}

//parents:bool, link_identifiers:bool, check_return:bool, alias_analysis:bool, type_check:bool, solc_check:bool
pub fn get_processed_ast(
    code: &str,
    flag: Option<u32>,
    global_vars: RcCell<GlobalVars>,
) -> ASTFlatten {
    let flag = ASTFlags::new(flag);
    // println!(
    //     "====flag=================={:?}======={:?}",
    //     flag.to_string(),
    //     flag.type_check()
    // );
    let (mut ast, _) =
        get_parsed_ast_and_fake_code(code, flag & ASTFlags::SOLC_CHECK == ASTFlags::SOLC_CHECK); //solc_check

    // Zkay preprocessing and type checking
    process_ast(
        &ast.clone().into(),
        flag.parents(),
        flag.link_identifiers(),
        flag.check_return(),
        flag.alias_analysis(),
        flag.type_check(),
        global_vars,
    );

    ast
}

fn process_ast(
    ast: &ASTFlatten,
    parents: bool,
    link_identifiers: bool,
    check_return: bool,
    alias_analysis: bool,
    type_check: bool,
    global_vars: RcCell<GlobalVars>,
) {
    print_step("Preprocessing AST");
    if parents {
        set_parents(ast);
    }
    // println!("======set_parents===========================");
    if link_identifiers {
        // try:
        link(ast, global_vars.clone());
    }
    // except UnknownIdentifierException as e:
    //     raise PreprocessAstException(f"\n\nSYMBOL ERROR: {e}")
    // try:
    if check_return {
        r(ast);
    }
    if alias_analysis {
        a(ast, global_vars.clone());
    }
    // println!("{:?}", global_vars.borrow().vars().len());
    call_graph_analysis(ast);
    compute_modified_sets(ast);
    check_for_undefined_behavior_due_to_eval_order(ast);
    // except AstException as e:
    //     raise AnalysisException(f"\n\nANALYSIS ERROR: {e}")
    // println!("======{type_check}=========process======before====");
    if type_check {
        // println!("======type_check=========process==========");
        print_step("Zkay type checking");
        // try:
        t(ast, global_vars.clone());
        // println!("======check_circuit_compliance================*****************===========");
        check_circuit_compliance(ast);
        //  println!("======check_circuit_compliance================*******222222**********===========");
        detect_hybrid_functions(ast);
        check_loops(ast);
        //  println!("======check_circuit_compliance================*******3333**********===========");
        // except (TypeMismatchException, TypeException, RequireException, ReclassifyException) as e:
        //     raise TypeCheckException(f"\n\nCOMPILER ERROR: {e}")
    }
}

pub fn get_verification_contract_names(
    code_or_ast: (Option<String>, Option<ASTFlatten>),
    global_vars: RcCell<GlobalVars>,
) -> Vec<String> {
    let ast = if let (Some(code), None) = code_or_ast {
        Some(get_processed_ast(&code, None, global_vars))
    } else {
        code_or_ast.1.clone()
    };
    assert!(ast.is_some(), "Invalid AST (no source unit at root)");
    let ast = ast.unwrap();
    let mut vc_names = vec![];
    for contract in &ast.try_as_source_unit_ref().unwrap().borrow().contracts {
        let cname = contract.borrow().idf().as_ref().unwrap().borrow().name();
        let fcts: Vec<_> = contract
            .borrow()
            .function_definitions
            .iter()
            .chain(&contract.borrow().constructor_definitions)
            .filter(|fct| {
                fct.borrow().requires_verification_when_external && fct.borrow().has_side_effects()
            })
            .cloned()
            .collect();
        vc_names.extend(
            fcts.iter()
                .map(|fct| {
                    CFG.lock()
                        .unwrap()
                        .get_verification_contract_name(cname.clone(), fct.borrow().name())
                })
                .collect::<Vec<_>>(),
        );
    }
    vc_names
}
