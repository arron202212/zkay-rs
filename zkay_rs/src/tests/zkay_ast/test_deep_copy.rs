// from parameterized::parameterized_class

// from zkay.examples.examples::all_examples
// from zkay.tests.utils.test_examples::TestExamples
// from zkay.zkay_ast.build_ast::build_ast
// from zkay.zkay_ast.visitor.deep_copy::deep_copy


// @parameterized_class(('name', 'example'), all_examples)
// class TestParentSetter(TestExamples):

//     def test_deep_copy(self):
//         ast = build_ast(self.example.code())
//         ast_2 = deep_copy(ast)
//         self.assertEqual(str(ast), str(ast_2))
#[cfg(test)]
mod tests {
    use zkay_examples::examples::ALL_EXAMPLES;
use zkay_ast::global_defs::{global_defs, global_vars};
    // use  zkay_tests::utils::test_examples::TestExamples;
    // use  zkay_solidity_parser::emit::normalize_code;
    use super::*;
    use ast_builder::build_ast::build_ast;
use zkay_ast::visitors::deep_copy::deep_copy;
    // @parameterized_class(('name', 'example'), all_examples)
    // class TestBuildAST(TestExamples):
    #[test]
    fn test_deep_copy() {
let global_vars = RcCell::new(global_vars(RcCell::new(global_defs())));
        for (_name, example) in ALL_EXAMPLES.iter() {
            // println!("{:?}", name);
            let ast = build_ast(&example.code());
            let ast_2 = deep_copy(ast,false,false,global_vars.clone());
            assert!(ast.to_string(),ast_2.to_string());
        }
        // self.assertIsNotNone(ast)
    }

   
}