#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

use crate::contains_private::contains_private;
use crate::final_checker::check_final;
// use crate::type_exceptions::{TypeMismatchException, TypeException};
use zkay_ast::homomorphism::{Homomorphism, HOMOMORPHISM_STORE, REHOM_EXPRESSIONS};

use std::ops::DerefMut;
use zkay_ast::ast::{
    get_privacy_expr_from_label, is_instance, is_instances, issue_compiler_warning, ASTBaseMutRef,
    ASTBaseProperty, ASTBaseRef, ASTType, AllExpr, AnnotatedTypeName, Array, ArrayBaseProperty,
    AssignmentStatement, AssignmentStatementBaseMutRef, AssignmentStatementBaseProperty,
    BooleanLiteralType, BuiltinFunction, CombinedPrivacyUnion, ConstructorOrFunctionDefinition,
    ContractDefinition, ElementaryTypeName, EnumDefinition, EnumTypeName, EnumValue,
    EnumValueTypeName, Expression, ExpressionBaseMutRef, ExpressionBaseProperty, ExpressionBaseRef,
    ForStatement, FunctionCallExpr, FunctionCallExprBaseMutRef, FunctionCallExprBaseProperty,
    FunctionCallExprBaseRef, FunctionTypeName, IdentifierDeclaration, IdentifierDeclarationBaseRef,
    IdentifierExpr, IfStatement, IndexExpr, IntoAST, IntoExpression, IntoStatement, LiteralUnion,
    LocationExpr, LocationExprBaseProperty, Mapping, MeExpr, MemberAccessExpr, NamespaceDefinition,
    NewExpr, NumberLiteralType, NumberLiteralTypeUnion, NumberTypeName, PrimitiveCastExpr,
    RRWrapper, ReclassifyExpr, ReclassifyExprBase, ReclassifyExprBaseMutRef,
    ReclassifyExprBaseProperty, RehomExpr, RequireStatement, ReturnStatement,
    StateVariableDeclaration, StatementBaseMutRef, StatementBaseProperty, TupleExpr, TupleType,
    TypeName, UserDefinedTypeName, UserDefinedTypeNameBaseProperty, VariableDeclarationStatement,
    WhileStatement, AST,
};
use zkay_ast::visitor::deep_copy::replace_expr;
use zkay_ast::visitor::visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef};
use zkay_derive::ASTVisitorBaseRefImpl;
pub fn type_check(ast: AST) {
    check_final(ast.clone());
    let v = TypeCheckVisitor::new();
    v.visit(&ast);
}

// class TypeCheckVisitor(AstVisitor)
#[derive(ASTVisitorBaseRefImpl)]
pub struct TypeCheckVisitor {
    pub ast_visitor_base: AstVisitorBase,
}
impl AstVisitor for TypeCheckVisitor {
    type Return = Option<String>;
    fn temper_result(&self) -> Self::Return {
        None
    }
    fn has_attr(&self, _name: &ASTType) -> bool {
        false
    }
    fn get_attr(&self, _name: &ASTType, _ast: &AST) -> Self::Return {
        self.temper_result()
    }
}
impl TypeCheckVisitor {
    pub fn new() -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("post", false),
        }
    }
    pub fn get_rhs(
        &self,
        mut rhs: Expression,
        expected_type: &AnnotatedTypeName,
    ) -> Option<Expression> {
        if is_instance(&rhs, ASTType::TupleExpr) {
            if !is_instance(&rhs, ASTType::TupleExpr)
                || !is_instance(
                    &**expected_type.type_name.as_ref().unwrap(),
                    ASTType::TupleType,
                )
                || rhs
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_tuple_expr_ref()
                    .unwrap()
                    .elements
                    .len()
                    != expected_type
                        .type_name
                        .as_ref()
                        .unwrap()
                        .try_as_tuple_type_ref()
                        .unwrap()
                        .types
                        .len()
            {
                assert!(
                    false,
                    "{:?},{:?},{:?}",
                    expected_type,
                    rhs.annotated_type(),
                    rhs
                )
            }
            let exprs: Vec<_> = expected_type
                .type_name
                .as_ref()
                .unwrap()
                .try_as_tuple_type_ref()
                .unwrap()
                .types
                .iter()
                .zip(
                    rhs.try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .try_as_tuple_expr_ref()
                        .unwrap()
                        .elements
                        .clone(),
                )
                .map(|(e, a)| self.get_rhs(a, e).unwrap())
                .collect();
            return Some(
                replace_expr(&rhs, &mut TupleExpr::new(exprs.clone()).to_expr(), false).as_type(
                    AST::TypeName(TypeName::TupleType(TupleType::new(
                        exprs
                            .iter()
                            .map(|e| e.annotated_type().clone().unwrap())
                            .collect(),
                    ))),
                ),
            );
        }

        let mut require_rehom = false;
        let mut instance = rhs.instance_of(expected_type);
        if instance.is_none() {
            require_rehom = true;
            let expected_matching_hom = expected_type
                .with_homomorphism(rhs.annotated_type().as_ref().unwrap().clone().homomorphism);
            instance = rhs.instance_of(&expected_matching_hom);
        }

        assert!(
            instance.is_some(),
            "{:?},{:?}, {:?}",
            expected_type,
            rhs.annotated_type(),
            rhs
        );
        if rhs.annotated_type().as_ref().unwrap().type_name != expected_type.type_name {
            rhs = Self::implicitly_converted_to(rhs, &**expected_type.type_name.as_ref().unwrap());
        }

        Some(if instance == Some(String::from("make-private")) {
            Self::make_private(
                rhs,
                expected_type
                    .privacy_annotation
                    .as_ref()
                    .unwrap()
                    .try_as_expression_ref()
                    .unwrap(),
                &expected_type.homomorphism,
            )
        } else if require_rehom {
            Self::try_rehom(rhs, expected_type)
        } else {
            rhs
        })
    }
    //@staticmethod
    pub fn check_for_invalid_private_type(ast: AST) {
        if let Some(at) = &ast.try_as_expression_ref().unwrap().annotated_type() {
            if at.is_private() && !at.type_name.as_ref().unwrap().can_be_private() {
                assert!(
                    false,
                    "Type {:?} cannot be private {:?}",
                    at.type_name,
                    ast.try_as_expression_ref().unwrap().annotated_type()
                );
            }
        }
    }
    pub fn check_final(&self, fct: ConstructorOrFunctionDefinition, ast: Expression) {
        if is_instance(&ast, ASTType::IdentifierExpr) {
            if let Some(target) = ast
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .try_as_identifier_expr_ref()
                .unwrap()
                .target()
                .as_ref()
                .unwrap()
                .try_as_identifier_declaration_ref()
                .unwrap()
                .try_as_state_variable_declaration_ref()
            {
                if target
                    .identifier_declaration_base
                    .keywords
                    .contains(&String::from("final"))
                {
                    //assignment allowed
                    // pass
                    assert!(
                        is_instance(target, ASTType::StateVariableDeclaration)
                            && fct.is_constructor(),
                        r#"Modifying "final" variable{:?}"#,
                        ast
                    );
                }
            }
        } else {
            assert!(is_instance(&ast, ASTType::TupleExpr));
            for elem in &ast
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_tuple_expr_ref()
                .unwrap()
                .elements
            {
                self.check_final(fct.clone(), elem.clone());
            }
        }
    }

    pub fn visitAssignmentStatement(&self, mut ast: AssignmentStatement) {
        assert!(
            ast.lhs().is_some(),
            "Assignment target is not a location {:?}",
            ast.lhs()
        );

        let expected_type = &ast
            .lhs()
            .as_ref()
            .unwrap()
            .try_as_expression_ref()
            .unwrap()
            .annotated_type();
        ast.assignment_statement_base_mut_ref().rhs = self.get_rhs(
            ast.rhs().as_ref().unwrap().clone(),
            expected_type.as_ref().unwrap(),
        );

        //prevent modifying final
        let f = ast.function().as_ref().unwrap();
        if is_instance(&**ast.lhs().as_ref().unwrap(), ASTType::TupleExpr)
            || is_instance(&**ast.lhs().as_ref().unwrap(), ASTType::LocationExprBase)
        {
            self.check_final(
                *f.clone(),
                ast.lhs()
                    .as_ref()
                    .unwrap()
                    .try_as_expression_ref()
                    .unwrap()
                    .clone(),
            );
        }
    }

    pub fn visitVariableDeclarationStatement(&self, mut ast: VariableDeclarationStatement) {
        if ast.expr.is_some() {
            ast.expr = self.get_rhs(
                ast.expr.unwrap(),
                &*ast
                    .variable_declaration
                    .identifier_declaration_base
                    .annotated_type,
            );
        }
    }

    //@staticmethod
    pub fn has_private_type(ast: &Expression) -> bool {
        ast.annotated_type().as_ref().unwrap().is_private()
    }

    //@staticmethod
    pub fn has_literal_type(ast: Expression) -> bool {
        is_instances(
            &**ast
                .annotated_type()
                .as_ref()
                .unwrap()
                .type_name
                .as_ref()
                .unwrap(),
            vec![ASTType::NumberLiteralType, ASTType::BooleanLiteralType],
        )
    }
    pub fn handle_builtin_function_call(
        &mut self,
        mut ast: FunctionCallExpr,
        func: &mut BuiltinFunction,
    ) {
        if func.is_parenthesis() {
            ast.expression_base_mut_ref().annotated_type = ast.args()[0].annotated_type().clone();
            return;
        }

        let all_args_all_or_me = ast.args().iter().all(|x| {
            x.annotated_type()
                .as_ref()
                .unwrap()
                .is_accessible(&ast.to_expr().analysis())
        });
        let is_public_ite =
            func.is_ite() && ast.args()[0].annotated_type().as_ref().unwrap().is_public();
        if all_args_all_or_me || is_public_ite {
            self.handle_unhom_builtin_function_call(ast, func);
        } else {
            self.handle_homomorphic_builtin_function_call(ast, func.clone());
        }
    }

    pub fn handle_unhom_builtin_function_call(
        &self,
        mut ast: FunctionCallExpr,
        mut func: &mut BuiltinFunction,
    ) {
        let mut args = ast.args().clone();
        //handle special cases
        if func.is_ite() {
            let cond_t = &args[0].annotated_type();

            //Ensure that condition is boolean
            assert!(
                cond_t
                    .as_ref()
                    .unwrap()
                    .type_name
                    .as_ref()
                    .unwrap()
                    .implicitly_convertible_to(&TypeName::bool_type()),
                "{:?}, {:?}, {:?}",
                TypeName::bool_type(),
                cond_t.as_ref().unwrap().type_name,
                args[0]
            );

            let res_t = args[1]
                .annotated_type()
                .as_ref()
                .unwrap()
                .type_name
                .as_ref()
                .unwrap()
                .combined_type(
                    *args[2]
                        .annotated_type()
                        .as_ref()
                        .unwrap()
                        .type_name
                        .clone()
                        .unwrap(),
                    true,
                );

            let a = if cond_t.as_ref().unwrap().is_private()
            //Everything is turned private
            {
                func.is_private = true;
                res_t.unwrap().annotate(CombinedPrivacyUnion::AST(Some(
                    Expression::me_expr(None).to_ast(),
                )))
            } else {
                let hom = Self::combine_homomorphism(args[1].clone(), args[2].clone());
                let true_type = args[1]
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .with_homomorphism(hom.clone());
                let false_type = args[2]
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .with_homomorphism(hom.clone());
                let p = true_type
                    .combined_privacy(ast.to_expr().analysis(), false_type)
                    .unwrap();
                res_t.unwrap().annotate(p).with_homomorphism(hom)
            };
            args[1] = self.get_rhs(args[1].clone(), &a).unwrap();
            args[2] = self.get_rhs(args[2].clone(), &a).unwrap();
            ast.function_call_expr_base_mut_ref().args = args;
            ast.expression_base_mut_ref().annotated_type = Some(a);
            return;
        }

        //Check that argument types conform to op signature
        let parameter_types = func.input_types();
        if !func.is_eq() {
            for (arg, t) in args.iter().zip(&parameter_types) {
                if !arg.instanceof_data_type(t.as_ref().unwrap()) {
                    assert!(
                        false,
                        "{:?},{:?}, {:?}",
                        t,
                        arg.annotated_type().as_ref().unwrap().type_name,
                        arg
                    );
                }
            }
        }

        let t1 = *args[0]
            .annotated_type()
            .as_ref()
            .unwrap()
            .type_name
            .clone()
            .unwrap();
        let t2 = if args.len() == 1 {
            None
        } else {
            Some(
                *args[1]
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .type_name
                    .clone()
                    .unwrap(),
            )
        };

        let mut arg_t = if args.len() == 1 {
            Some(
                if args[0]
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .type_name
                    .as_ref()
                    .unwrap()
                    .is_literal()
                {
                    TypeName::Literal(String::from("lit"))
                } else {
                    t1.clone()
                },
            )
        } else {
            assert!(args.len() == 2);
            let is_eq_with_tuples = func.is_eq() && is_instance(&t1, ASTType::TupleType);
            t1.combined_type(t2.clone().unwrap(), is_eq_with_tuples)
        };
        //Infer argument and output types
        let out_t = if arg_t == Some(TypeName::Literal(String::from("lit"))) {
            let res = func.op_func(
                args.iter()
                    .map(|arg| {
                        arg.annotated_type()
                            .as_ref()
                            .unwrap()
                            .type_name
                            .as_ref()
                            .unwrap()
                            .try_as_elementary_type_name_ref()
                            .unwrap()
                            .try_as_number_type_name_ref()
                            .unwrap()
                            .try_as_number_literal_type_ref()
                            .unwrap()
                            .value()
                    })
                    .collect(),
            );
            let out_t = match res {
                LiteralUnion::Bool(value) => {
                    assert!(func.output_type() == Some(TypeName::bool_type()));
                    TypeName::ElementaryTypeName(ElementaryTypeName::BooleanLiteralType(
                        BooleanLiteralType::new(value),
                    ))
                }
                LiteralUnion::Number(value) => {
                    assert!(func.output_type() == Some(TypeName::number_type()));
                    TypeName::ElementaryTypeName(ElementaryTypeName::NumberTypeName(
                        NumberTypeName::NumberLiteralType(NumberLiteralType::new(
                            NumberLiteralTypeUnion::I32(value),
                        )),
                    ))
                }
            };
            if func.is_eq() {
                arg_t = t1
                    .try_as_elementary_type_name_ref()
                    .unwrap()
                    .try_as_number_type_name_ref()
                    .unwrap()
                    .try_as_number_literal_type_ref()
                    .unwrap()
                    .to_abstract_type()
                    .combined_type(
                        t2.unwrap()
                            .try_as_elementary_type_name_ref()
                            .unwrap()
                            .try_as_number_type_name_ref()
                            .unwrap()
                            .try_as_number_literal_type_ref()
                            .unwrap()
                            .to_abstract_type(),
                        true,
                    );
            }
            Some(out_t)
        } else if func.output_type() == Some(TypeName::bool_type()) {
            Some(TypeName::bool_type())
        } else {
            arg_t.clone()
        };

        assert!(
            arg_t.is_some()
                && (arg_t != Some(TypeName::Literal(String::from("lit"))) || !func.is_eq())
        );
        let mut p = None;
        let private_args = args.iter().any(|arg| Self::has_private_type(arg));
        if private_args {
            assert!(arg_t != Some(TypeName::Literal(String::from("lit"))));
            if func.can_be_private() {
                if func.is_shiftop() {
                    if !args[1]
                        .annotated_type()
                        .as_ref()
                        .unwrap()
                        .type_name
                        .as_ref()
                        .unwrap()
                        .is_literal()
                    {
                        assert!(
                            false,
                            "Private shift expressions must use a constant (literal) shift amount {:?}",
                            args[1]
                        )
                    }
                    if args[1]
                        .annotated_type()
                        .as_ref()
                        .unwrap()
                        .type_name
                        .as_ref()
                        .unwrap()
                        .try_as_elementary_type_name_ref()
                        .unwrap()
                        .try_as_number_type_name_ref()
                        .unwrap()
                        .try_as_number_literal_type_ref()
                        .unwrap()
                        .value()
                        < 0
                    {
                        assert!(false, "Cannot shift by negative amount {:?}", args[1]);
                    }
                }
                if func.is_bitop() || func.is_shiftop() {
                    for arg in &args {
                        if arg
                            .annotated_type()
                            .as_ref()
                            .unwrap()
                            .type_name
                            .as_ref()
                            .unwrap()
                            .elem_bitwidth()
                            == 256
                        {
                            assert!(false,"Private bitwise and shift operations are only supported for integer types < 256 bit, please use a smaller type {:?}", arg)
                        }
                    }
                }

                if func.is_arithmetic() {
                    for a in &args {
                        if a.annotated_type()
                            .as_ref()
                            .unwrap()
                            .type_name
                            .as_ref()
                            .unwrap()
                            .elem_bitwidth()
                            == 256
                        {
                            issue_compiler_warning(
                                func.to_ast(),
                                String::from("Possible field prime overflow"),
                                String::from(
                                    r#"Private arithmetic 256bit operations overflow at FIELD_PRIME.\nIf you need correct overflow behavior, use a smaller integer type."#,
                                ),
                            );
                            break;
                        }
                    }
                } else if func.is_comp() {
                    for a in &args {
                        if a.annotated_type()
                            .as_ref()
                            .unwrap()
                            .type_name
                            .as_ref()
                            .unwrap()
                            .elem_bitwidth()
                            == 256
                        {
                            issue_compiler_warning(
                                func.to_ast(),
                                String::from("Possible private comparison failure"),
                                String::from(
                                    r#"Private 256bit comparison operations will fail for values >= 2^252.\n If you cannot guarantee that the value stays in range, you must use a smaller integer type to ensure correctness."#,
                                ),
                            );
                            break;
                        }
                    }
                }

                func.is_private = true;
                p = Some(Expression::me_expr(None));
            } else {
                assert!(
                    false,
                    r#"Operation \"{}\" does not support private operands{:?}"#,
                    func.op, ast
                );
            }
        }

        if arg_t != Some(TypeName::Literal(String::from("lit"))) {
            //Add implicit casts for arguments
            let arg_pt = arg_t.unwrap().annotate(CombinedPrivacyUnion::AST(Some(
                p.as_ref().unwrap().to_ast(),
            )));
            if func.is_shiftop() && p.is_some() {
                args[0] = self.get_rhs(args[0].clone(), &arg_pt).unwrap();
            } else {
                args = ast
                    .args()
                    .iter()
                    .map(|argument| self.get_rhs(argument.clone(), &arg_pt).unwrap())
                    .collect();
            }
            ast.function_call_expr_base_mut_ref().args = args;
        }

        ast.expression_base_mut_ref().annotated_type = Some(
            out_t
                .unwrap()
                .annotate(CombinedPrivacyUnion::AST(Some(p.unwrap().to_ast()))),
        );
    }
    pub fn handle_homomorphic_builtin_function_call(
        &self,
        mut ast: FunctionCallExpr,
        mut func: BuiltinFunction,
    ) {
        //First - same as non-homomorphic - check that argument types conform to op signature
        if !func.is_eq() {
            for (arg, t) in ast.args().iter().zip(&func.input_types()) {
                if !arg.instanceof_data_type(t.as_ref().unwrap()) {
                    assert!(
                        false,
                        "{:?},{:?}, {:?}",
                        t,
                        arg.annotated_type().as_ref().unwrap().type_name,
                        arg
                    )
                }
            }
        }

        let homomorphic_func =
            func.select_homomorphic_overload(ast.args(), ast.to_expr().analysis());
        if homomorphic_func.is_none() {
            assert!(
                false,
                r#"Operation \"{}\" requires all arguments to be accessible, i.e. @all or provably equal to @me{:?}"#,
                func.op, ast
            );
        }

        //We could perform homomorphic operations on-chain by using some Solidity arbitrary precision math library.
        //For now, keep it simple and evaluate homomorphic operations in Python and check the result in the circuit.
        func.is_private = true;

        ast.expression_base_mut_ref().annotated_type =
            Some(homomorphic_func.clone().unwrap().output_type());
        func.homomorphism = ast.annotated_type().as_ref().unwrap().homomorphism.clone();
        let expected_arg_types = homomorphic_func.unwrap().input_types();

        //Check that the argument types are correct
        ast.function_call_expr_base_mut_ref().args = ast
            .args()
            .iter()
            .zip(expected_arg_types)
            .map(|(arg, arg_pt)| self.get_rhs(arg.clone(), &arg_pt).unwrap())
            .collect();
    }
    //@staticmethod
    pub fn is_accessible_by_invoker(_ast: &Expression) -> bool {
        // return ast.annotated_type.is_public() || ast.is_lvalue() || \
        //     ast.instance_of(AnnotatedTypeName(ast.annotated_type.type_name, Expression::me_expr(None)))
        true
    }
    //@staticmethod
    pub fn combine_homomorphism(lhs: Expression, rhs: Expression) -> String {
        if lhs.annotated_type().as_ref().unwrap().homomorphism
            == rhs.annotated_type().as_ref().unwrap().homomorphism
        {
            lhs.annotated_type().as_ref().unwrap().homomorphism.clone()
        } else if Self::can_rehom(&lhs) {
            rhs.annotated_type().as_ref().unwrap().homomorphism.clone()
        } else {
            lhs.annotated_type().as_ref().unwrap().homomorphism.clone()
        }
    }

    //@staticmethod
    pub fn can_rehom(ast: &Expression) -> bool {
        if ast
            .annotated_type()
            .as_ref()
            .unwrap()
            .is_accessible(&ast.analysis())
        {
            return true;
        }
        if is_instance(ast, ASTType::ReclassifyExpr) {
            return true;
        }
        if is_instance(ast, ASTType::PrimitiveCastExpr) {
            return Self::can_rehom(ast);
        }
        if is_instance(ast, ASTType::FunctionCallExprBase)
            && is_instance(
                &**ast.try_as_function_call_expr_ref().unwrap().func(),
                ASTType::BuiltinFunction,
            )
            && ast
                .try_as_function_call_expr_ref()
                .unwrap()
                .func()
                .try_as_builtin_function_ref()
                .unwrap()
                .is_ite()
            && ast.try_as_function_call_expr_ref().unwrap().args()[0]
                .annotated_type()
                .as_ref()
                .unwrap()
                .is_public()
        {
            return Self::can_rehom(&ast.try_as_function_call_expr_ref().unwrap().args()[1])
                && Self::can_rehom(&ast.try_as_function_call_expr_ref().unwrap().args()[2]);
        }

        false
    }

    //@staticmethod
    pub fn try_rehom(mut rhs: Expression, expected_type: &AnnotatedTypeName) -> Expression {
        assert!(
            !rhs.annotated_type().as_ref().unwrap().is_public(),
            "Cannot change the homomorphism of a public value"
        );

        if rhs
            .annotated_type()
            .as_ref()
            .unwrap()
            .is_private_at_me(&rhs.analysis())
        {
            //The value is @me, so we can just insert a ReclassifyExpr to change
            //the homomorphism of this value, just like we do for public values.
            return Self::make_rehom(rhs, expected_type);
        }
        if is_instance(&rhs, ASTType::ReclassifyExpr) && !is_instance(&rhs, ASTType::RehomExpr) {
            //rhs is a valid ReclassifyExpr, i.e. the argument is public or @me-private
            //To create an expression with the correct homomorphism,
            //just change the ReclassifyExpr"s output homomorphism
            rhs.try_as_reclassify_expr_mut()
                .unwrap()
                .reclassify_expr_base_mut_ref()
                .homomorphism = Some(expected_type.homomorphism.clone());
        } else if is_instance(&rhs, ASTType::PrimitiveCastExpr) {
            //Ignore primitive cast & recurse
            rhs.try_as_primitive_cast_expr_mut().unwrap().expr =
                Box::new(Self::try_rehom(rhs.clone(), expected_type));
        } else if is_instance(&rhs, ASTType::FunctionCallExprBase)
            && is_instance(
                &**rhs.try_as_function_call_expr_ref().unwrap().func(),
                ASTType::BuiltinFunction,
            )
            && rhs
                .try_as_function_call_expr_ref()
                .unwrap()
                .func()
                .try_as_builtin_function_ref()
                .unwrap()
                .is_ite()
            && rhs.try_as_function_call_expr_ref().unwrap().args()[0]
                .annotated_type()
                .as_ref()
                .unwrap()
                .is_public()
        {
            //Argument is public_cond ? true_val : false_val. Try to rehom both true_val and false_val
            let mut args = rhs.try_as_function_call_expr_ref().unwrap().args().clone();
            args[1] = Self::try_rehom(args[1].clone(), expected_type);
            args[2] = Self::try_rehom(args[2].clone(), expected_type);
            rhs.try_as_function_call_expr_mut()
                .unwrap()
                .function_call_expr_base_mut_ref()
                .args = args;
        } else {
            assert!(
                false,
                "{:?}, {:?} ,{:?}",
                expected_type,
                rhs.annotated_type(),
                rhs
            )
        }

        //Rehom worked without throwing, change annotated_type and return
        rhs.expression_base_mut_ref().annotated_type = Some(
            rhs.annotated_type()
                .as_ref()
                .unwrap()
                .with_homomorphism(expected_type.homomorphism.clone()),
        );
        rhs
    }

    //@staticmethod
    pub fn make_rehom(mut expr: Expression, expected_type: &AnnotatedTypeName) -> Expression {
        assert!(expected_type
            .privacy_annotation
            .as_ref()
            .unwrap()
            .try_as_expression_ref()
            .unwrap()
            .privacy_annotation_label()
            .is_some());
        assert!(expr
            .annotated_type()
            .as_ref()
            .unwrap()
            .is_private_at_me(&expr.analysis()));
        assert!(expected_type.is_private_at_me(&expr.analysis()));

        let mut r = RehomExpr::new(expr.clone(), Some(expected_type.homomorphism.clone()));

        //set type
        let pl = get_privacy_expr_from_label(
            expected_type
                .privacy_annotation
                .as_ref()
                .unwrap()
                .try_as_expression_ref()
                .unwrap()
                .privacy_annotation_label()
                .unwrap()
                .into(),
        );
        r.reclassify_expr_base.expression_base.annotated_type = Some(AnnotatedTypeName::new(
            expr.annotated_type()
                .as_ref()
                .unwrap()
                .type_name
                .as_ref()
                .map(|t| *t.clone()),
            Some(pl.into_ast()),
            expected_type.homomorphism.clone(),
        ));
        Self::check_for_invalid_private_type(r.to_ast());

        //set statement, parents, location
        Self::assign_location(&mut r.to_expr(), &mut expr);

        r.to_expr()
    }

    //@staticmethod
    pub fn make_private(
        mut expr: Expression,
        privacy: &Expression,
        homomorphism: &String,
    ) -> Expression {
        assert!(privacy.privacy_annotation_label().is_some());

        let pl = get_privacy_expr_from_label(privacy.privacy_annotation_label().unwrap().into());
        let mut r = ReclassifyExprBase::new(expr.clone(), pl.clone(), Some(homomorphism.clone()));

        //set type
        r.expression_base.annotated_type = Some(AnnotatedTypeName::new(
            expr.annotated_type()
                .as_ref()
                .unwrap()
                .type_name
                .as_ref()
                .map(|t| *t.clone()),
            Some(pl.to_ast()),
            homomorphism.clone(),
        ));
        Self::check_for_invalid_private_type(r.to_ast());
        let mut r = r.to_expr();
        //set statement, parents, location
        Self::assign_location(&mut r, &mut expr);

        r
    }

    //@staticmethod
    pub fn assign_location(target: &mut Expression, source: &mut Expression) {
        //set statement
        target.expression_base_mut_ref().statement = source.statement().clone();

        //set parents
        target
            .ast_base_mut_ref()
            .parent_namespace
            .as_mut()
            .unwrap()
            .deref_mut()
            .borrow_mut()
            .parent = source.parent().clone();
        let mut annotated_type = target.annotated_type().clone();
        annotated_type
            .as_mut()
            .unwrap()
            .ast_base
            .parent_namespace
            .as_mut()
            .unwrap()
            .deref_mut()
            .borrow_mut()
            .parent = Some(Box::new((*target).to_ast()));
        target.expression_base_mut_ref().annotated_type = annotated_type;
        source
            .ast_base_mut_ref()
            .parent_namespace
            .as_mut()
            .unwrap()
            .deref_mut()
            .borrow_mut()
            .parent = Some(Box::new(target.clone().to_ast()));

        //set source location
        target.ast_base_mut_ref().line = source.ast_base_ref().line;
        target.ast_base_mut_ref().column = source.ast_base_ref().column;
    }

    //@staticmethod
    pub fn implicitly_converted_to(mut expr: Expression, t: &TypeName) -> Expression {
        if is_instance(&expr, ASTType::ReclassifyExpr)
            && !expr
                .try_as_reclassify_expr_ref()
                .unwrap()
                .privacy()
                .is_all_expr()
        {
            //Cast the argument of the ReclassifyExpr instead
            expr.try_as_reclassify_expr_mut()
                .unwrap()
                .reclassify_expr_base_mut_ref()
                .expr = Box::new(Self::implicitly_converted_to(
                *expr.try_as_reclassify_expr_ref().unwrap().expr().clone(),
                t,
            ));
            let mut expr_annotated_type = expr.annotated_type().clone();
            expr_annotated_type.as_mut().unwrap().type_name = expr
                .try_as_reclassify_expr_ref()
                .unwrap()
                .expr()
                .as_ref()
                .annotated_type()
                .as_ref()
                .unwrap()
                .type_name
                .clone();
            expr.expression_base_mut_ref().annotated_type = expr_annotated_type;
            return expr;
        }

        assert!(expr
            .annotated_type()
            .as_ref()
            .unwrap()
            .type_name
            .as_ref()
            .unwrap()
            .is_primitive_type());
        let mut cast = PrimitiveCastExpr::new(t.clone(), expr.clone(), true);
        cast.expression_base
            .ast_base
            .parent_namespace
            .as_mut()
            .unwrap()
            .deref_mut()
            .borrow_mut()
            .parent = expr.parent().clone();
        cast.expression_base.statement = expr.statement().clone();
        cast.expression_base.ast_base.line = expr.line();
        cast.expression_base.ast_base.column = expr.column();
        cast.elem_type
            .ast_base_mut_ref()
            .unwrap()
            .parent_namespace
            .as_mut()
            .unwrap()
            .deref_mut()
            .borrow_mut()
            .parent = Some(Box::new(cast.to_ast()));
        expr.ast_base_mut_ref()
            .parent_namespace
            .as_mut()
            .unwrap()
            .deref_mut()
            .borrow_mut()
            .parent = Some(Box::new(cast.to_ast()));
        cast.expression_base.annotated_type = Some(AnnotatedTypeName::new(
            Some(t.clone()),
            expr.annotated_type()
                .as_ref()
                .unwrap()
                .privacy_annotation
                .as_ref()
                .map(|p| *p.clone()),
            expr.annotated_type().as_ref().unwrap().homomorphism.clone(),
        ));
        cast.expression_base
            .annotated_type
            .as_mut()
            .unwrap()
            .ast_base
            .parent_namespace
            .as_mut()
            .unwrap()
            .deref_mut()
            .borrow_mut()
            .parent = Some(Box::new(cast.to_ast()));
        Expression::PrimitiveCastExpr(cast)
    }

    pub fn visitFunctionCallExpr(&mut self, mut ast: FunctionCallExpr) {
        if is_instance(&**ast.func(), ASTType::BuiltinFunction) {
            self.handle_builtin_function_call(
                ast.clone(),
                ast.function_call_expr_base_mut_ref()
                    .func
                    .try_as_builtin_function_mut()
                    .unwrap(),
            );
        } else if ast.is_cast() {
            assert!(
                is_instance(
                    &*ast
                        .func()
                        .try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .try_as_location_expr_ref()
                        .unwrap()
                        .target()
                        .as_ref()
                        .unwrap(),
                    ASTType::EnumDefinition
                ),
                "User type casts only implemented for enums"
            );
            ast.expression_base_mut_ref().annotated_type = Some(
                self.handle_cast(
                    ast.args()[0].clone(),
                    *ast.func()
                        .try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .try_as_location_expr_ref()
                        .unwrap()
                        .target()
                        .as_ref()
                        .unwrap()
                        .try_as_expression_ref()
                        .unwrap()
                        .annotated_type()
                        .as_ref()
                        .unwrap()
                        .type_name
                        .clone()
                        .unwrap(),
                ),
            );
        } else if is_instance(&**ast.func(), ASTType::LocationExprBase) {
            let ft = *ast
                .func()
                .annotated_type()
                .as_ref()
                .unwrap()
                .type_name
                .clone()
                .unwrap();
            assert!(is_instance(&ft, ASTType::FunctionTypeName));

            assert!(
                ft.try_as_function_type_name_ref().unwrap().parameters.len() == ast.args().len(),
                "Wrong number of arguments {:?}",
                ast.func()
            );

            //Check arguments
            let mut args = ast.args().clone();
            for i in 0..ast.args().len() {
                args[i] = self
                    .get_rhs(
                        args[i].clone(),
                        &*ft.try_as_function_type_name_ref().unwrap().parameters[i]
                            .identifier_declaration_base
                            .annotated_type,
                    )
                    .unwrap();
            }
            ast.function_call_expr_base_mut_ref().args = args;

            //Set expression type to return type
            ast.expression_base_mut_ref().annotated_type = Some(
                if ft
                    .try_as_function_type_name_ref()
                    .unwrap()
                    .return_parameters
                    .len()
                    == 1
                {
                    *ft.try_as_function_type_name_ref()
                        .unwrap()
                        .return_parameters[0]
                        .identifier_declaration_base
                        .annotated_type
                        .clone()
                } else {
                    //TODO maybe not None label in the future
                    AnnotatedTypeName::new(
                        Some(TypeName::TupleType(TupleType::new(
                            ft.try_as_function_type_name_ref()
                                .unwrap()
                                .return_parameters
                                .iter()
                                .map(|t| *t.identifier_declaration_base.annotated_type.clone())
                                .collect(),
                        ))),
                        None,
                        String::from("NON_HOMOMORPHISM"),
                    )
                },
            );
        } else {
            assert!(false, "Invalid function call{:?}", ast);
        }
    }

    pub fn visitPrimitiveCastExpr(&self, mut ast: PrimitiveCastExpr) {
        ast.expression_base.annotated_type = Some(self.handle_cast(*ast.expr, *ast.elem_type));
    }

    pub fn handle_cast(&self, expr: Expression, t: TypeName) -> AnnotatedTypeName {
        //because of the fake solidity check we already know that the cast is possible -> don"t have to check if cast possible
        if expr.annotated_type().as_ref().unwrap().is_private() {
            let expected = AnnotatedTypeName::new(
                expr.annotated_type()
                    .as_ref()
                    .unwrap()
                    .type_name
                    .as_ref()
                    .map(|t| *t.clone()),
                Some(Expression::me_expr(None).into_ast()),
                String::from("NON_HOMOMORPHISM"),
            );
            if Some(String::from("true")) == expr.instance_of(&expected) {
                assert!(
                    false,
                    "{:?}, {:?}, {:?}",
                    expected,
                    expr.annotated_type(),
                    expr
                )
            }
            AnnotatedTypeName::new(
                Some(t.clone()),
                Some(Expression::me_expr(None).into_ast()),
                String::from("NON_HOMOMORPHISM"),
            )
        } else {
            AnnotatedTypeName::new(Some(t.clone()), None, String::from("NON_HOMOMORPHISM"))
        }
    }

    pub fn visitNewExpr(&self, _ast: NewExpr) { //already has correct type
                                                // pass
    }

    pub fn visitMemberAccessExpr(&self, mut ast: MemberAccessExpr) {
        assert!(ast.location_expr_base.target.is_some());

        assert!(
            !(ast
                .expr
                .as_ref()
                .unwrap()
                .annotated_type()
                .as_ref()
                .unwrap()
                .is_address()
                && ast
                    .expr
                    .as_ref()
                    .unwrap()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .is_private()),
            "Cannot access members of private address variable{:?}",
            ast
        );
        ast.location_expr_base
            .tuple_or_location_expr_base
            .expression_base
            .annotated_type = ast
            .location_expr_base
            .target
            .as_ref()
            .unwrap()
            .borrow()
            .as_ref()
            .unwrap()
            .try_as_expression_ref()
            .unwrap()
            .annotated_type()
            .clone();
    }

    pub fn visitReclassifyExpr(&self, mut ast: ReclassifyExpr) {
        assert!(
            ast.privacy().privacy_annotation_label().is_none(),
            r#"Second argument of "reveal" cannot be used as a privacy type{:?}"#,
            ast
        );

        let mut homomorphism = Homomorphism::non_homomorphic();
        assert!(!homomorphism.is_empty());

        //Prevent ReclassifyExpr to all with homomorphic type
        if ast.privacy().is_all_expr()
            && (ast.homomorphism() != &Some(Homomorphism::non_homomorphic())
                || ast.expr().annotated_type().as_ref().unwrap().homomorphism
                    != Homomorphism::non_homomorphic())
        {
            //If the target privacy is all, we infer a target homomorphism of NonHomomorphic
            homomorphism = Homomorphism::non_homomorphic();
            ast.reclassify_expr_base_mut_ref().homomorphism = Some(homomorphism.clone());
        }

        //Make sure the first argument to reveal / rehom is public or private provably equal to @me
        let is_expr_at_all = ast.expr().annotated_type().as_ref().unwrap().is_public();
        let is_expr_at_me = ast
            .expr()
            .annotated_type()
            .as_ref()
            .unwrap()
            .is_private_at_me(&ast.to_expr().analysis());
        assert!(
            is_expr_at_all || is_expr_at_me,
            r#"First argument of "{}" must be accessible,"i.e. @all or provably equal to @me{:?}"#,
            ast.func_name(),
            ast
        );

        //Prevent unhom(public_value)

        assert!(
            !(is_expr_at_all
                && is_instance(&ast, ASTType::RehomExpr)
                && ast.homomorphism() == &Some(Homomorphism::non_homomorphic())),
            r#"Cannot use "{}" on a public value{:?}"#,
            HOMOMORPHISM_STORE
                .lock()
                .unwrap()
                .get(ast.homomorphism().as_ref().unwrap())
                .unwrap()
                .rehom_expr_name,
            ast
        );

        //NB prevent any redundant reveal (not just for public)
        ast.expression_base_mut_ref().annotated_type = Some(AnnotatedTypeName::new(
            ast.expr()
                .annotated_type()
                .as_ref()
                .unwrap()
                .type_name
                .as_ref()
                .map(|t| *t.clone()),
            Some(ast.privacy().to_ast()),
            homomorphism.clone(),
        ));

        assert!(
            Some(String::from("true"))
                != ast
                    .to_expr()
                    .instance_of(&ast.expr().annotated_type().as_ref().unwrap()),
            r#"Redundant "{}": Expression is already @{}{homomorphism}"{:?}"#,
            ast.func_name(),
            ast.privacy().to_ast().code(),
            ast
        );
        Self::check_for_invalid_private_type(ast.to_ast());
    }

    pub fn visitIfStatement(&self, ast: IfStatement) {
        let b = &ast.condition;
        assert!(
            b.instanceof_data_type(&TypeName::bool_type()),
            "{:?}, {:?} ,{:?}",
            TypeName::bool_type(),
            b.annotated_type().as_ref().unwrap().type_name,
            b
        );
        if ast
            .condition
            .annotated_type()
            .as_ref()
            .unwrap()
            .is_private()
        {
            let expected = AnnotatedTypeName::new(
                Some(TypeName::bool_type()),
                Some(Expression::me_expr(None).into_ast()),
                String::from("NON_HOMOMORPHISM"),
            );
            assert!(
                Some(String::from("true")) == b.instance_of(&expected),
                "{:?}, {:?} ,{:?}",
                expected,
                b.annotated_type(),
                b
            )
        }
    }

    pub fn visitWhileStatement(&self, ast: WhileStatement) {
        assert!(
            Some(String::from("true")) == ast.condition.instance_of(&AnnotatedTypeName::bool_all()),
            "{:?}, {:?} ,{:?}",
            AnnotatedTypeName::bool_all(),
            ast.condition.annotated_type(),
            ast.condition
        )
        //must also later check that body and condition do not contain private expressions
    }

    pub fn visitForStatement(&self, ast: ForStatement) {
        assert!(
            Some(String::from("true")) == ast.condition.instance_of(&AnnotatedTypeName::bool_all()),
            "{:?}, {:?} ,{:?}",
            AnnotatedTypeName::bool_all(),
            ast.condition.annotated_type(),
            ast.condition
        )
        //must also later check that body, update and condition do not contain private expressions
    }
    pub fn visitReturnStatement(&self, mut ast: ReturnStatement) {
        assert!(ast.statement_base.function.as_ref().unwrap().is_function());
        let rt = AnnotatedTypeName::new(
            Some(TypeName::TupleType(
                (*ast.statement_base.function.as_ref().unwrap()).return_type(),
            )),
            None,
            String::from("NON_HOMOMORPHISM"),
        );
        if ast.expr.is_some() {
            self.get_rhs(TupleExpr::new(vec![]).to_expr(), &rt);
        } else if !is_instance(ast.expr.as_ref().unwrap(), ASTType::TupleExpr) {
            ast.expr = self.get_rhs(
                TupleExpr::new(vec![ast.expr.clone().unwrap()]).to_expr(),
                &rt,
            );
        } else {
            ast.expr = self.get_rhs(ast.expr.clone().unwrap(), &rt);
        }
    }
    pub fn visitTupleExpr(&self, mut ast: TupleExpr) {
        ast.tuple_or_location_expr_base
            .expression_base
            .annotated_type = Some(AnnotatedTypeName::new(
            Some(TypeName::TupleType(TupleType::new(
                ast.elements
                    .iter()
                    .map(|elem| elem.annotated_type().as_ref().unwrap().clone())
                    .collect(),
            ))),
            None,
            String::from("NON_HOMOMORPHISM"),
        ));
    }

    pub fn visitMeExpr(&self, mut ast: MeExpr) {
        ast.expression_base.annotated_type = Some(AnnotatedTypeName::address_all());
    }

    pub fn visitIdentifierExpr(&self, mut ast: IdentifierExpr) {
        // if is_instance(&ast.location_expr_base.target, ASTType::Mapping) { //no action necessary, the identifier will be replaced later
        // pass
        let target = ast.location_expr_base.target();
        if let Some(target) = target {
            assert!(
                is_instance(&target, ASTType::ContractDefinition),
                "Unsupported use of contract type in expression{:?}",
                ast
            );
            ast.annotated_type = target
                .try_as_expression_ref()
                .unwrap()
                .annotated_type()
                .as_ref()
                .map(|t| Box::new(t.clone()));

            assert!(Self::is_accessible_by_invoker(&ast.to_expr()) ,"Tried to read value which cannot be proven to be owned by the transaction invoker{:?}", ast);
        }
    }
    pub fn visitIndexExpr(&self, mut ast: IndexExpr) {
        let arr = ast.arr.clone().unwrap();
        let index = ast.key.clone();
        let mut map_t = arr.annotated_type().as_ref().unwrap();
        //should have already been checked
        assert!(map_t
            .privacy_annotation
            .as_ref()
            .unwrap()
            .try_as_expression_ref()
            .unwrap()
            .is_all_expr());
        let mut tn = *map_t.type_name.clone().unwrap();
        //do actual type checking
        if let TypeName::Mapping(ref mut type_name) = tn {
            let key_type = *type_name.key_type.clone();
            let expected = AnnotatedTypeName::new(
                Some(key_type),
                Some(Expression::all_expr().into_ast()),
                String::from("NON_HOMOMORPHISM"),
            );
            let instance = index.instance_of(&expected);
            assert!(
                Some(String::from("true")) == instance,
                "{:?}, {:?} ,{:?}",
                expected,
                index.annotated_type(),
                ast
            );

            //record indexing information
            if type_name.key_label.is_some()
            //TODO modification correct?
            {
                assert!(
                    index.privacy_annotation_label().is_some(),
                    "Index cannot be used as a privacy type for array of type {:?}{:?}",
                    map_t,
                    ast
                );
                type_name.instantiated_key = Some(*index);
            }
            //determine value type
            ast.location_expr_base
                .tuple_or_location_expr_base
                .expression_base
                .annotated_type = Some(*type_name.value_type.clone());

            assert!(Self::is_accessible_by_invoker(&ast.to_expr()) ,"Tried to read value which cannot be proven to be owned by the transaction invoker{:?}", ast);
        } else if let TypeName::Array(type_name) = &**map_t.type_name.as_ref().unwrap() {
            assert!(
                !ast.key.annotated_type().as_ref().unwrap().is_private(),
                "No private array index{:?}",
                ast
            );
            assert!(
                ast.key.instanceof_data_type(&TypeName::number_type()),
                "Array index must be numeric{:?}",
                ast
            );
            ast.location_expr_base
                .tuple_or_location_expr_base
                .expression_base
                .annotated_type = Some(type_name.value_type().clone());
        } else {
            assert!(false, "Indexing into non-mapping{:?}", ast);
        }
    }
    pub fn visitConstructorOrFunctionDefinition(&self, ast: ConstructorOrFunctionDefinition) {
        for t in ast.parameter_types().types {
            assert!(
                is_instances(
                    &*t.privacy_annotation.unwrap(),
                    vec![ASTType::MeExpr, ASTType::AllExpr],
                ),
                "Only me/all accepted as privacy type of function parameters{:?}",
                ast
            );
        }

        if ast.can_be_external() {
            for t in ast.return_type().types {
                assert!(is_instances(
                    &*t.privacy_annotation.unwrap(),
                    vec![ASTType::MeExpr, ASTType::AllExpr],
                ),"Only me/all accepted as privacy type of return values for public functions{:?}", ast);
            }
        }
    }
    pub fn visitEnumDefinition(&self, mut ast: EnumDefinition) {
        let mut etn = EnumTypeName::new(ast.qualified_name(), None);
        etn.user_defined_type_name_base.target = Some(RRWrapper::new(Some(ast.to_ast())));
        ast.annotated_type = Some(AnnotatedTypeName::new(
            Some(TypeName::UserDefinedTypeName(
                UserDefinedTypeName::EnumTypeName(etn),
            )),
            None,
            String::from("NON_HOMOMORPHIM"),
        ));
    }

    pub fn visitEnumValue(&self, mut ast: EnumValue) {
        let mut evtn = EnumValueTypeName::new(ast.qualified_name(), None);
        evtn.user_defined_type_name_base.target = Some(RRWrapper::new(Some(ast.to_ast())));
        ast.annotated_type = Some(AnnotatedTypeName::new(
            Some(TypeName::UserDefinedTypeName(
                UserDefinedTypeName::EnumValueTypeName(evtn),
            )),
            None,
            String::from("NON_HOMOMORPHISM"),
        ));
    }

    pub fn visitStateVariableDeclaration(&self, ast: StateVariableDeclaration) {
        if let Some(expr) = &ast.expr {
            //prevent private operations in declaration
            assert!(
                !contains_private(ast.to_ast()),
                "Private assignments to state variables must be in the constructor{:?}",
                ast
            );

            //check type
            self.get_rhs(
                expr.clone(),
                &*ast.identifier_declaration_base.annotated_type,
            );
        }

        //prevent "me" annotation
        let p = ast
            .identifier_declaration_base
            .annotated_type
            .privacy_annotation
            .as_ref()
            .unwrap();
        assert!(
            !p.try_as_expression_ref().unwrap().is_me_expr(),
            "State variables cannot be annotated as me{:?}",
            ast
        );
    }

    pub fn visitMapping(&self, ast: Mapping) {
        if ast.key_label.is_some() {
            assert!(
                *ast.key_type.clone() == TypeName::address_type(),
                "Only addresses can be annotated{:?}",
                ast
            );
        }
    }

    pub fn visitRequireStatement(&self, ast: RequireStatement) {
        assert!(
            ast.condition
                .annotated_type()
                .as_ref()
                .unwrap()
                .privacy_annotation
                .as_ref()
                .unwrap()
                .try_as_expression_ref()
                .unwrap()
                .is_all_expr(),
            "require needs public argument{:?}",
            ast
        );
    }

    pub fn visitAnnotatedTypeName(&mut self, mut ast: AnnotatedTypeName) {
        if is_instance(
            &**ast.type_name.as_ref().unwrap(),
            ASTType::UserDefinedTypeNameBase,
        ) {
            assert!(
                is_instance(
                    &*ast
                        .type_name
                        .as_ref()
                        .unwrap()
                        .try_as_user_defined_type_name_ref()
                        .unwrap()
                        .target()
                        .as_ref()
                        .unwrap(),
                    ASTType::EnumDefinition
                ),
                "Unsupported use of user-defined type {:?}",
                ast.type_name
            );
            ast.type_name = ast
                .type_name
                .as_ref()
                .unwrap()
                .try_as_user_defined_type_name_ref()
                .unwrap()
                .target()
                .as_ref()
                .unwrap()
                .try_as_namespace_definition_ref()
                .unwrap()
                .try_as_enum_definition_ref()
                .unwrap()
                .annotated_type
                .as_ref()
                .unwrap()
                .type_name
                .clone();
        }

        if ast.privacy_annotation != Some(Box::new(Expression::all_expr().into_ast())) {
            assert!(
                ast.type_name.as_ref().unwrap().can_be_private(),
                "Currently, we do not support private {:?},{:?}",
                ast.type_name,
                ast
            );
            if ast.homomorphism != Homomorphism::non_homomorphic() {
                //only support uint8, uint16, uint24, uint32 homomorphic data types
                assert!(
                    ast.type_name.as_ref().unwrap().is_numeric(),
                    "Homomorphic type not supported for {:?}: Only numeric types supported{:?}",
                    ast.type_name,
                    ast
                );
                assert!(
                    !ast.type_name.as_ref().unwrap().signed(),
                    "Homomorphic type not supported for {:?}: Only unsigned types supported{:?}",
                    ast.type_name,
                    ast
                );
                assert!(ast.type_name.as_ref().unwrap().elem_bitwidth() <= 32,"Homomorphic type not supported for {:?}: Only up to 32-bit numeric types supported{:?}", ast.type_name,ast);
            }
        }
        let p = *ast.privacy_annotation.unwrap();
        if is_instance(&p, ASTType::IdentifierExpr) {
            let t = p
                .try_as_expression_ref()
                .unwrap()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .target();
            if let Some(t) = t {
                //no action necessary, this is the case: mapping(address!x => uint@x)
                // pass
                assert!(
                    t.try_as_identifier_declaration_ref()
                        .unwrap()
                        .identifier_declaration_base_ref()
                        .is_final()
                        || t.try_as_identifier_declaration_ref()
                            .unwrap()
                            .identifier_declaration_base_ref()
                            .is_constant(),
                    r#"Privacy annotations must be "final" or "constant", if they are expressions {:?}"#,
                    p
                );
                assert!(
                    t.try_as_expression_ref().unwrap().annotated_type()
                        == &Some(AnnotatedTypeName::address_all()),
                    r#"Privacy type is not a public address, but {:?},{:?}"#,
                    t.try_as_expression_ref().unwrap().annotated_type(),
                    p
                );
            }
        }
    }
}
