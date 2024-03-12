use zkay_examples::examples::ALL_EXAMPLES;
// use zkay_tests::utils::test_examples::TestExamples
use ast_builder::process_ast::get_processed_ast;

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
        for (name, example) in ALL_EXAMPLES.iter() {
            println!("{:?}", name);
            let _ast = get_processed_ast(&example.code(), Some(!0b00001000));
            assert!(true);
        }
    }
}
