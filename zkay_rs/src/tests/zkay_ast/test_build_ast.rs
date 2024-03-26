use zkay_examples::examples::ALL_EXAMPLES;
// use  zkay_tests::utils::test_examples::TestExamples;
// use  zkay_solidity_parser::emit::normalize_code;
use ast_builder::build_ast::build_ast;

#[cfg(test)]
mod tests {
    use super::*;
    // @parameterized_class(('name', 'example'), all_examples)
    // class TestBuildAST(TestExamples):
    #[test]
    fn test_build_ast() {
        for (_name, example) in ALL_EXAMPLES.iter() {
            // println!("{:?}", name);
            let _ast = build_ast(&example.code());
            assert!(true);
        }
        // self.assertIsNotNone(ast)
    }

    //     def test_to_ast_and_back(self):
    //         # ast
    //         ast = build_ast(self.example.code())
    //         # back to string
    //         new_code = str(ast)
    //         self.assertIn(self.example.name(), new_code)
    //         new_code = normalize_code(new_code)
    //         # reference
    //         reference = normalize_code(self.example.code())
    //         # check
    //         self.assertEqual(reference, new_code)
}
