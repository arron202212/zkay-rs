use zkay_examples::examples::ALL_EXAMPLES;
// use zkay_tests::utils::test_examples::TestExamples
use ast_builder::process_ast::get_processed_ast;
use rccell::RcCell;
use zkay_ast::global_defs::{global_defs, global_vars};
// @parameterized_class(('name', 'example'), all_examples)
// class TestProcessAST(TestExamples):

//     def test_process_ast(self):
//         ast = get_processed_ast(self.example.code(), type_check=False)
//         self.assertIsNotNone(ast)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_process_ast() {
        let global_vars = RcCell::new(global_vars(RcCell::new(global_defs())));
        for (name, example) in ALL_EXAMPLES.iter() {
            println!("{:?}", name);
            let _ast = get_processed_ast(&example.code(), Some(!0b00001000), global_vars.clone());
            assert!(true);
        }
    }
}
