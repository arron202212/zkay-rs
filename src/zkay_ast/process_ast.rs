use crate::compiler::solidity::compiler::check_for_zkay_solc_errors;
//, SolcException;
use crate::config::CFG;
// use crate::errors::exceptions::ZkayCompilerError, PreprocessAstException, TypeCheckException, AnalysisException,ZkaySyntaxError;
// use crate::solidity_parser::parse::SyntaxException;
use crate::type_check::type_checker::type_check as t;
// use crate::type_check::type_exceptions::TypeMismatchException, TypeException, RequireException, ReclassifyException;
use crate::utils::progress_printer::print_step;
use crate::zkay_ast::analysis::alias_analysis::alias_analysis as a;
use crate::zkay_ast::analysis::call_graph::call_graph_analysis;
use crate::zkay_ast::analysis::circuit_compatibility_checker::check_circuit_compliance;
use crate::zkay_ast::analysis::hybrid_function_detector::detect_hybrid_functions;
use crate::zkay_ast::analysis::loop_checker::check_loops;
use crate::zkay_ast::analysis::return_checker::check_return as r;
use crate::zkay_ast::analysis::side_effects::{
    check_for_undefined_behavior_due_to_eval_order, compute_modified_sets,
};
use crate::zkay_ast::ast::{SourceUnit, AST}; //, AstException;
use crate::zkay_ast::build_ast::build_ast;
use crate::zkay_ast::pointers::parent_setter::set_parents;
// use crate::zkay_ast::pointers::pointer_exceptions::UnknownIdentifierException;
use crate::zkay_ast::pointers::symbol_table::link_identifiers as link;
use bitflags::bitflags;
use std::fmt;

bitflags! {
    struct ASTFlags: u32 {
        const PARENTS           = 0b00000001;
        const LINK_IDENTIFIERS  = 0b00000010;
        const CHECK_RETURN      = 0b00000100;
        const ALIAS_ANALYSIS      = 0b00100000;
        const TYPE_CHECK        = 0b00001000;
        const SOLC_CHECK        = 0b00010000;
        const FLAG_ALL    = Self::PARENTS.bits
                           | Self::LINK_IDENTIFIERS.bits
                           | Self::CHECK_RETURN.bits
                           | Self::ALIAS_ANALYSIS.bits
                           | Self::TYPE_CHECK.bits
                           | Self::SOLC_CHECK.bits;
    }
}

impl ASTFlags {
    pub fn new(flag: Option<u32>) -> Self {
        Self {
            bits: flag.unwrap_or(ASTFlags::FLAG_ALL),
        }
    }
    pub fn clear(&mut self) -> &mut ASTFlags {
        self.bits = 0;
        self
    }
    pub fn parents(&self) -> bool {
        self.bits & Self::PARENTS == Self::PARENTS
    }
    pub fn link_identifiers(&self) -> bool {
        self.bits & Self::LINK_IDENTIFIERS == Self::LINK_IDENTIFIERS
    }
    pub fn check_return(&self) -> bool {
        self.bits & Self::CHECK_RETURN == Self::CHECK_RETURN
    }
    pub fn alias_analysis(&self) -> bool {
        self.bits & Self::ALIAS_ANALYSIS == Self::ALIAS_ANALYSIS
    }
    pub fn type_check(&self) -> bool {
        self.bits & Self::TYPE_CHECK == Self::TYPE_CHECK
    }
    pub fn solc_check(&self) -> bool {
        self.bits & Self::SOLC_CHECK == Self::SOLC_CHECK
    }
}

impl fmt::Display for ASTFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:032b}", self.bits)
    }
}
fn get_parsed_ast_and_fake_code(code: &str, solc_check: bool) -> (AST, String) {
    print_step("Parsing");
    let _ast = build_ast(code);
    // except SyntaxException as e:
    //     raise ZkaySyntaxError(f"\n\nSYNTAX ERROR: {e}")

    let fake_code = crate::compiler::solidity::fake_solidity_generator::fake_solidity_code(code);
    if solc_check {
        // Solc type checking
        print_step("Type checking with solc");
        // try:
        check_for_zkay_solc_errors(code, fake_code);
        // except SolcException as e:
        //     raise ZkayCompilerError(f"{e}")
    }
    (_ast, fake_code)
}

//parents:bool, link_identifiers:bool, check_return:bool, alias_analysis:bool, type_check:bool, solc_check:bool
pub fn get_processed_ast(code: &str, flag: Option<u32>) -> AST {
    let flag = ASTFlags::new(flag);

    let (ast, _) =
        get_parsed_ast_and_fake_code(code, flag & ASTFlags::SOLC_CHECK == ASTFlags::SOLC_CHECK); //solc_check

    // Zkay preprocessing and type checking
    process_ast(
        ast,
        flag.parents(),
        flag.link_identifiers(),
        flag.check_return(),
        flag.alias_analysis(),
        flag.type_check(),
    );

    ast
}

fn process_ast(
    ast: AST,
    parents: bool,
    link_identifiers: bool,
    check_return: bool,
    alias_analysis: bool,
    type_check: bool,
) {
    print_step("Preprocessing AST");
    if parents {
        set_parents(ast);
    }
    if link_identifiers
    // try:
    {
        link(ast);
    }
    // except UnknownIdentifierException as e:
    //     raise PreprocessAstException(f"\n\nSYMBOL ERROR: {e}")
    // try:
    if check_return {
        r(ast);
    }
    if alias_analysis {
        a(ast);
    }
    call_graph_analysis(ast);
    compute_modified_sets(ast);
    check_for_undefined_behavior_due_to_eval_order(ast);
    // except AstException as e:
    //     raise AnalysisException(f"\n\nANALYSIS ERROR: {e}")
    if type_check {
        print_step("Zkay type checking");
        // try:
        t(ast);
        check_circuit_compliance(ast);
        detect_hybrid_functions(ast);
        check_loops(ast);
        // except (TypeMismatchException, TypeException, RequireException, ReclassifyException) as e:
        //     raise TypeCheckException(f"\n\nCOMPILER ERROR: {e}")
    }
}

pub fn get_verification_contract_names(
    code_or_ast: (Option<String>, Option<SourceUnit>),
) -> Vec<String> {
    let ast = if let (Some(code_or_ast), None) = code_or_ast {
        get_processed_ast(code_or_ast)
    } else if let (None, Some(code_or_ast)) = code_or_ast {
        code_or_ast.get_ast()
    } else {
        assert!(false, "Invalid AST (no source unit at root)");
    };

    let mut vc_names = vec![];
    for contract in ast.contracts {
        let cname = contract.idf.name;
        let fcts = contract
            .function_definitions
            .iter()
            .chain(contract.constructor_definitions)
            .filter_map(|fct| {
                if fct.requires_verification_when_external && fct.has_side_effects {
                    Some(fct)
                } else {
                    None
                }
            })
            .collect();
        vc_names += fcts
            .iter()
            .map(|fct| {
                CFG.lock()
                    .unwrap()
                    .get_verification_contract_name(cname, fct.name)
            })
            .collect();
    }
    vc_names
}
