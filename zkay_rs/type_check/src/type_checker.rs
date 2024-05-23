#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::contains_private::contains_private;
use crate::final_checker::check_final;
use rccell::{RcCell, WeakCell};
// use crate::type_exceptions::{TypeMismatchException, TypeException};
use zkay_ast::homomorphism::{Homomorphism, HOMOMORPHISM_STORE, REHOM_EXPRESSIONS};

use std::ops::DerefMut;
use zkay_ast::ast::{
    get_privacy_expr_from_label, is_instance, is_instances, issue_compiler_warning, ASTBaseMutRef,
    ASTBaseProperty, ASTBaseRef, ASTFlatten, ASTInstanceOf, ASTType, AllExpr, AnnotatedTypeName,
    Array, ArrayBaseProperty, AssignmentStatement, AssignmentStatementBaseMutRef,
    AssignmentStatementBaseProperty, BooleanLiteralType, BuiltinFunction, CombinedPrivacyUnion,
    ConstructorOrFunctionDefinition, ContractDefinition, ElementaryTypeName, EnumDefinition,
    EnumTypeName, EnumValue, EnumValueTypeName, Expression, ExpressionASType, ExpressionBaseMutRef,
    ExpressionBaseProperty, ExpressionBaseRef, ForStatement, FunctionCallExpr,
    FunctionCallExprBaseMutRef, FunctionCallExprBaseProperty, FunctionCallExprBaseRef,
    FunctionTypeName, IdentifierDeclaration, IdentifierDeclarationBaseProperty,
    IdentifierDeclarationBaseRef, IdentifierExpr, IfStatement, IndexExpr, IntoAST, IntoExpression,
    IntoStatement, LiteralUnion, LocationExpr, LocationExprBaseProperty, Mapping, MeExpr,
    MemberAccessExpr, NamespaceDefinition, NewExpr, NumberLiteralType, NumberLiteralTypeUnion,
    NumberTypeName, PrimitiveCastExpr, ReclassifyExpr, ReclassifyExprBase,
    ReclassifyExprBaseMutRef, ReclassifyExprBaseProperty, RehomExpr, RequireStatement,
    ReturnStatement, SimpleStatement, StateVariableDeclaration, Statement, StatementBaseMutRef,
    StatementBaseProperty, TupleExpr, TupleType, TypeName, UserDefinedTypeName,
    UserDefinedTypeNameBaseProperty, VariableDeclarationStatement, WhileStatement, AST,
};
use zkay_ast::visitor::deep_copy::replace_expr;
use zkay_ast::visitor::visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef};
use zkay_derive::ASTVisitorBaseRefImpl;
pub fn type_check(ast: &ASTFlatten) {
    println!("==========type_check=====================");
    check_final(ast);
    let v = TypeCheckVisitor::new();
    v.visit(&ast);
}

// class TypeCheckVisitor(AstVisitor)
#[derive(ASTVisitorBaseRefImpl)]
pub struct TypeCheckVisitor {
    pub ast_visitor_base: AstVisitorBase,
}
impl AstVisitor for TypeCheckVisitor {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}
    fn has_attr(&self, ast: &AST) -> bool {
        matches!(
            ast.get_ast_type(),
            ASTType::AssignmentStatementBase
                | ASTType::VariableDeclarationStatement
                | ASTType::FunctionCallExprBase
                | ASTType::PrimitiveCastExpr
                | ASTType::NewExpr
                | ASTType::MemberAccessExpr
                | ASTType::ReclassifyExpr
                | ASTType::IfStatement
                | ASTType::WhileStatement
                | ASTType::ForStatement
                | ASTType::ReturnStatement
                | ASTType::TupleExpr
                | ASTType::MeExpr
                | ASTType::IdentifierExpr
                | ASTType::IndexExpr
                | ASTType::ConstructorOrFunctionDefinition
                | ASTType::EnumDefinition
                | ASTType::EnumValue
                | ASTType::StateVariableDeclaration
                | ASTType::Mapping
                | ASTType::RequireStatement
                | ASTType::AnnotatedTypeName
        ) || matches!(
            ast.to_ast(),
            AST::Statement(Statement::SimpleStatement(
                SimpleStatement::AssignmentStatement(_)
            ))
        ) || matches!(
            ast.to_ast(),
            AST::Expression(Expression::FunctionCallExpr(_))
        )
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> eyre::Result<Self::Return> {
        match name {
            _ if matches!(
                ast.to_ast(),
                AST::Statement(Statement::SimpleStatement(
                    SimpleStatement::AssignmentStatement(_)
                ))
            ) =>
            {
                self.visitAssignmentStatement(ast)
            }
            ASTType::VariableDeclarationStatement => self.visitVariableDeclarationStatement(ast),
            _ if matches!(
                ast.to_ast(),
                AST::Expression(Expression::FunctionCallExpr(_))
            ) =>
            {
                self.visitFunctionCallExpr(ast)
            }
            ASTType::PrimitiveCastExpr => self.visitPrimitiveCastExpr(ast),
            ASTType::NewExpr => self.visitNewExpr(ast),
            ASTType::MemberAccessExpr => self.visitMemberAccessExpr(ast),
            ASTType::ReclassifyExpr => self.visitReclassifyExpr(ast),
            ASTType::IfStatement => self.visitIfStatement(ast),
            ASTType::WhileStatement => self.visitWhileStatement(ast),
            ASTType::ForStatement => self.visitForStatement(ast),
            ASTType::ReturnStatement => self.visitReturnStatement(ast),
            ASTType::TupleExpr => self.visitTupleExpr(ast),
            ASTType::MeExpr => self.visitMeExpr(ast),
            ASTType::IdentifierExpr => self.visitIdentifierExpr(ast),
            ASTType::IndexExpr => self.visitIndexExpr(ast),
            ASTType::ConstructorOrFunctionDefinition => {
                self.visitConstructorOrFunctionDefinition(ast)
            }
            ASTType::EnumDefinition => self.visitEnumDefinition(ast),
            ASTType::EnumValue => self.visitEnumValue(ast),
            ASTType::StateVariableDeclaration => self.visitStateVariableDeclaration(ast),
            ASTType::Mapping => self.visitMapping(ast),
            ASTType::RequireStatement => self.visitRequireStatement(ast),
            ASTType::AnnotatedTypeName => self.visitAnnotatedTypeName(ast),
            _ => Err(eyre::eyre!("unimplemented")),
        }
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
        rhs: &ASTFlatten,
        expected_type: &RcCell<AnnotatedTypeName>,
    ) -> Option<ASTFlatten> {
        if is_instance(rhs, ASTType::TupleExpr) {
            assert!(
                is_instance(
                    expected_type.borrow().type_name.as_ref().unwrap(),
                    ASTType::TupleType,
                ) && rhs
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .borrow()
                    .try_as_tuple_expr_ref()
                    .unwrap()
                    .elements
                    .len()
                    == expected_type
                        .borrow()
                        .type_name
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .try_as_tuple_type_ref()
                        .unwrap()
                        .types
                        .len(),
                "{:?},{:?},{:?}",
                expected_type,
                rhs.try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type(),
                rhs
            );
            let exprs: Vec<_> = expected_type
                .borrow()
                .type_name
                .as_ref()
                .unwrap()
                .borrow()
                .try_as_tuple_type_ref()
                .unwrap()
                .types
                .iter()
                .zip(
                    rhs.try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .borrow()
                        .try_as_tuple_expr_ref()
                        .unwrap()
                        .elements
                        .clone(),
                )
                .filter_map(|(e, a)| self.get_rhs(&a, e))
                .collect();
            return replace_expr(
                &rhs.clone().into(),
                &RcCell::new(TupleExpr::new(exprs.clone())).into(),
                false,
            )
            .map(|_expr| {
                _expr.try_as_expression_ref().unwrap().borrow().as_type(
                    &RcCell::new(TupleType::new(
                        exprs
                            .iter()
                            .map(|e| {
                                e.try_as_expression_ref()
                                    .unwrap()
                                    .borrow()
                                    .annotated_type()
                                    .clone()
                                    .unwrap()
                            })
                            .collect(),
                    ))
                    .into(),
                )
            });
        }

        let mut require_rehom = false;
        let mut instance = rhs
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .instance_of(expected_type);
        if instance.is_none() {
            require_rehom = true;
            let expected_matching_hom = expected_type.borrow().with_homomorphism(
                rhs.try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .clone()
                    .borrow()
                    .homomorphism
                    .clone(),
            );
            instance = rhs
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .instance_of(&expected_matching_hom);
        }

        assert!(
            instance.is_some(),
            "{:?},{:?}, {:?}",
            expected_type,
            rhs.try_as_expression_ref()
                .unwrap()
                .borrow()
                .annotated_type(),
            rhs
        );
        let rhs = if rhs
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .annotated_type()
            .as_ref()
            .unwrap()
            .borrow()
            .type_name
            != expected_type.borrow().type_name
        {
            Self::implicitly_converted_to(rhs, expected_type.borrow().type_name.as_ref().unwrap())
                .clone()
        } else {
            rhs.clone()
        };
        let rhs = &rhs;
        Some(if instance == Some(String::from("make-private")) {
            Self::make_private(
                rhs,
                &expected_type.borrow().privacy_annotation,
                &expected_type.borrow().homomorphism,
            )
        } else if require_rehom {
            Self::try_rehom(rhs, expected_type)
        } else {
            rhs.clone()
        })
    }
    //@staticmethod
    pub fn check_for_invalid_private_type(ast: &ASTFlatten) {
        if let Some(at) = &ast
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .annotated_type()
        {
            assert!(
                !(at.borrow().is_private()
                    && !at
                        .borrow()
                        .type_name
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .can_be_private()),
                "Type {:?} cannot be private {:?}",
                at.borrow().type_name,
                ast.try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
            );
        }
    }
    pub fn check_final(&self, fct: &ASTFlatten, ast: &ASTFlatten) {
        if is_instance(ast, ASTType::IdentifierExpr) {
            if let Some(target) = ast
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .try_as_identifier_expr_ref()
                .unwrap()
                .target()
                .clone()
                .unwrap()
                .upgrade()
                .unwrap()
                .try_as_identifier_declaration_ref()
                .unwrap()
                .borrow()
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
                            && fct
                                .try_as_constructor_or_function_definition_ref()
                                .unwrap()
                                .borrow()
                                .is_constructor(),
                        r#"Modifying "final" variable{:?}"#,
                        ast
                    );
                }
            }
        } else {
            assert!(is_instance(ast, ASTType::TupleExpr));
            for elem in &ast
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_tuple_expr_ref()
                .unwrap()
                .elements
            {
                self.check_final(fct, elem);
            }
        }
    }

    pub fn visitAssignmentStatement(
        &self,
        mut ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        assert!(
            ast.try_as_assignment_statement_ref()
                .unwrap()
                .borrow()
                .lhs()
                .is_some(),
            "Assignment target is not a location {:?}",
            ast.try_as_assignment_statement_ref()
                .unwrap()
                .borrow()
                .lhs()
        );

        let expected_type = ast
            .try_as_assignment_statement_ref()
            .unwrap()
            .borrow()
            .lhs()
            .as_ref()
            .unwrap()
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .annotated_type()
            .clone();
        ast.try_as_assignment_statement_ref()
            .unwrap()
            .borrow_mut()
            .assignment_statement_base_mut_ref()
            .rhs = self.get_rhs(
            ast.try_as_assignment_statement_ref()
                .unwrap()
                .borrow()
                .rhs()
                .as_ref()
                .unwrap(),
            expected_type.as_ref().unwrap(),
        );

        //prevent modifying final
        if is_instance(
            ast.try_as_assignment_statement_ref()
                .unwrap()
                .borrow()
                .lhs()
                .as_ref()
                .unwrap(),
            ASTType::TupleExpr,
        ) || is_instance(
            ast.try_as_assignment_statement_ref()
                .unwrap()
                .borrow()
                .lhs()
                .as_ref()
                .unwrap(),
            ASTType::LocationExprBase,
        ) {
            self.check_final(
                ast.try_as_assignment_statement_ref()
                    .unwrap()
                    .borrow()
                    .function()
                    .clone()
                    .unwrap()
                    .upgrade()
                    .as_ref()
                    .unwrap(),
                ast.try_as_assignment_statement_ref()
                    .unwrap()
                    .borrow()
                    .lhs()
                    .as_ref()
                    .unwrap(),
            );
        }
        Ok(())
    }

    pub fn visitVariableDeclarationStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        if ast
            .try_as_variable_declaration_statement_ref()
            .unwrap()
            .borrow()
            .expr
            .is_some()
        {
            ast.try_as_variable_declaration_statement_ref()
                .unwrap()
                .borrow_mut()
                .expr = self.get_rhs(
                ast.try_as_variable_declaration_statement_ref()
                    .unwrap()
                    .borrow()
                    .expr
                    .as_ref()
                    .unwrap(),
                ast.try_as_variable_declaration_statement_ref()
                    .unwrap()
                    .borrow()
                    .variable_declaration
                    .borrow()
                    .identifier_declaration_base
                    .annotated_type
                    .as_ref()
                    .unwrap(),
            );
        }
        Ok(())
    }

    //@staticmethod
    pub fn has_private_type(ast: &ASTFlatten) -> bool {
        ast.try_as_expression_ref()
            .unwrap()
            .borrow()
            .annotated_type()
            .as_ref()
            .unwrap()
            .borrow()
            .is_private()
    }

    //@staticmethod
    pub fn has_literal_type(ast: Expression) -> bool {
        is_instances(
            ast.annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .type_name
                .as_ref()
                .unwrap(),
            vec![ASTType::NumberLiteralType, ASTType::BooleanLiteralType],
        )
    }
    pub fn handle_builtin_function_call(
        &self,
        mut ast: &RcCell<FunctionCallExpr>,
        func: &ASTFlatten,
    ) {
        if func
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .try_as_builtin_function_ref()
            .unwrap()
            .is_parenthesis()
        {
            ast.borrow_mut().expression_base_mut_ref().annotated_type = ast.borrow().args()[0]
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .clone();
            return;
        }

        let all_args_all_or_me = ast.borrow().args().iter().all(|x| {
            x.try_as_expression_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .is_accessible(&ast.borrow().to_expr().analysis())
        });
        let is_public_ite = func
            .try_as_builtin_function_ref()
            .unwrap()
            .borrow()
            .is_ite()
            && ast.borrow().args()[0]
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .is_public();
        if all_args_all_or_me || is_public_ite {
            self.handle_unhom_builtin_function_call(&ast, &func);
        } else {
            self.handle_homomorphic_builtin_function_call(&ast, &func);
        }
    }

    pub fn handle_unhom_builtin_function_call(
        &self,
        mut ast: &RcCell<FunctionCallExpr>,
        mut func: &ASTFlatten,
    ) {
        let mut args = ast.borrow().args().clone();
        //handle special cases
        if func
            .try_as_builtin_function_ref()
            .unwrap()
            .borrow()
            .is_ite()
        {
            let cond_t = &args[0]
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .clone();

            //Ensure that condition is boolean
            assert!(
                cond_t
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .type_name
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .implicitly_convertible_to(&RcCell::new(TypeName::bool_type())),
                "{:?}, {:?}, {:?}",
                TypeName::bool_type(),
                cond_t.as_ref().unwrap().borrow().type_name,
                args[0]
            );

            let res_t = args[1]
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .type_name
                .as_ref()
                .unwrap()
                .borrow()
                .combined_type(
                    &args[2]
                        .try_as_expression_ref()
                        .unwrap()
                        .borrow()
                        .annotated_type()
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .type_name
                        .clone()
                        .unwrap(),
                    true,
                );

            let a = if cond_t.as_ref().unwrap().borrow().is_private()
            //Everything is turned private
            {
                func.try_as_builtin_function_ref()
                    .unwrap()
                    .borrow_mut()
                    .is_private = true;
                res_t
                    .unwrap()
                    .borrow()
                    .annotate(CombinedPrivacyUnion::AST(Some(
                        RcCell::new(Expression::me_expr(None)).into(),
                    )))
            } else {
                let hom = Self::combine_homomorphism(args[1].clone(), args[2].clone());
                let true_type = args[1]
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .with_homomorphism(hom.clone());
                let false_type = args[2]
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .with_homomorphism(hom.clone());
                let p = true_type
                    .borrow()
                    .combined_privacy(ast.borrow().to_expr().analysis(), &false_type)
                    .unwrap();
                res_t
                    .unwrap()
                    .borrow()
                    .annotate(p)
                    .borrow()
                    .with_homomorphism(hom)
            };
            args[1] = self.get_rhs(&args[1], &a).unwrap();
            args[2] = self.get_rhs(&args[2], &a).unwrap();
            ast.borrow_mut().function_call_expr_base_mut_ref().args = args;
            ast.borrow_mut().expression_base_mut_ref().annotated_type = Some(a);
            return;
        }

        //Check that argument types conform to op signature
        let parameter_types = func
            .try_as_builtin_function_ref()
            .unwrap()
            .borrow()
            .input_types();
        if !func.try_as_builtin_function_ref().unwrap().borrow().is_eq() {
            for (arg, t) in args.iter().zip(&parameter_types) {
                if !arg
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .instanceof_data_type(t.as_ref().unwrap())
                {
                    assert!(
                        false,
                        "{:?},{:?}, {:?}",
                        t,
                        arg.try_as_expression_ref()
                            .unwrap()
                            .borrow()
                            .annotated_type()
                            .as_ref()
                            .unwrap()
                            .borrow()
                            .type_name,
                        arg
                    );
                }
            }
        }

        let t1 = args[0]
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .annotated_type()
            .as_ref()
            .unwrap()
            .borrow()
            .type_name
            .clone()
            .unwrap();
        let t2 = if args.len() == 1 {
            None
        } else {
            Some(
                args[1]
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .type_name
                    .clone()
                    .unwrap(),
            )
        };

        let mut arg_t = if args.len() == 1 {
            Some(
                if args[0]
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .type_name
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .is_literal()
                {
                    RcCell::new(TypeName::Literal(String::from("lit")))
                } else {
                    t1.clone()
                },
            )
        } else {
            assert!(args.len() == 2);
            let is_eq_with_tuples = func.try_as_builtin_function_ref().unwrap().borrow().is_eq()
                && is_instance(&t1, ASTType::TupleType);
            t1.borrow()
                .combined_type(t2.as_ref().unwrap(), is_eq_with_tuples)
        };
        //Infer argument and output types
        let out_t = if arg_t == Some(RcCell::new(TypeName::Literal(String::from("lit")))) {
            let res = func
                .try_as_builtin_function_ref()
                .unwrap()
                .borrow()
                .op_func(
                    args.iter()
                        .map(|arg| {
                            arg.try_as_expression_ref()
                                .unwrap()
                                .borrow()
                                .annotated_type()
                                .as_ref()
                                .unwrap()
                                .borrow()
                                .type_name
                                .as_ref()
                                .unwrap()
                                .borrow()
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
                    assert!(
                        func.try_as_builtin_function_ref()
                            .unwrap()
                            .borrow()
                            .output_type()
                            == Some(TypeName::bool_type())
                    );
                    RcCell::new(
                        BooleanLiteralType::new(value)
                            .into_ast()
                            .try_as_type_name()
                            .unwrap(),
                    )
                }
                LiteralUnion::Number(value) => {
                    assert!(
                        func.try_as_builtin_function_ref()
                            .unwrap()
                            .borrow()
                            .output_type()
                            == Some(TypeName::number_type())
                    );
                    RcCell::new(
                        NumberLiteralType::new(NumberLiteralTypeUnion::I32(value))
                            .into_ast()
                            .try_as_type_name()
                            .unwrap(),
                    )
                }
            };
            if func.try_as_builtin_function_ref().unwrap().borrow().is_eq() {
                arg_t = t1
                    .borrow()
                    .try_as_elementary_type_name_ref()
                    .unwrap()
                    .try_as_number_type_name_ref()
                    .unwrap()
                    .try_as_number_literal_type_ref()
                    .unwrap()
                    .to_abstract_type()
                    .borrow()
                    .combined_type(
                        &t2.unwrap()
                            .borrow()
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
        } else if func
            .try_as_builtin_function_ref()
            .unwrap()
            .borrow()
            .output_type()
            == Some(TypeName::bool_type())
        {
            Some(RcCell::new(TypeName::bool_type()))
        } else {
            arg_t.clone()
        };

        assert!(
            arg_t.is_some()
                && (arg_t != Some(RcCell::new(TypeName::Literal(String::from("lit"))))
                    || !func.try_as_builtin_function_ref().unwrap().borrow().is_eq())
        );
        let mut p = None;
        let private_args = args.iter().any(|arg| Self::has_private_type(arg));
        if private_args {
            assert!(arg_t != Some(RcCell::new(TypeName::Literal(String::from("lit")))));
            assert!(
                func.try_as_builtin_function_ref()
                    .unwrap()
                    .borrow()
                    .can_be_private(),
                r#"Operation \"{}\" does not support private operands{:?}"#,
                func.try_as_builtin_function_ref().unwrap().borrow().op,
                ast
            );

            if func
                .try_as_builtin_function_ref()
                .unwrap()
                .borrow()
                .is_shiftop()
            {
                assert!(
                    args[1]
                        .try_as_expression_ref()
                        .unwrap()
                        .borrow()
                        .annotated_type()
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .type_name
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .is_literal(),
                    "Private shift expressions must use a constant (literal) shift amount {:?}",
                    args[1]
                );

                assert!(
                    args[1]
                        .try_as_expression_ref()
                        .unwrap()
                        .borrow()
                        .annotated_type()
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .type_name
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .try_as_elementary_type_name_ref()
                        .unwrap()
                        .try_as_number_type_name_ref()
                        .unwrap()
                        .try_as_number_literal_type_ref()
                        .unwrap()
                        .value()
                        >= 0,
                    "Cannot shift by negative amount {:?}",
                    args[1]
                );
            }
            if func
                .try_as_builtin_function_ref()
                .unwrap()
                .borrow()
                .is_bitop()
                || func
                    .try_as_builtin_function_ref()
                    .unwrap()
                    .borrow()
                    .is_shiftop()
            {
                for arg in &args {
                    assert!(arg.try_as_expression_ref().unwrap().borrow()
                            .annotated_type()
                            .as_ref()
                            .unwrap().borrow()
                            .type_name
                            .as_ref()
                            .unwrap().borrow()
                            .elem_bitwidth()
                            != 256,"Private bitwise and shift operations are only supported for integer types < 256 bit, please use a smaller type {:?}", arg)
                }
            }

            if func
                .try_as_builtin_function_ref()
                .unwrap()
                .borrow()
                .is_arithmetic()
            {
                for a in &args {
                    if a.try_as_expression_ref()
                        .unwrap()
                        .borrow()
                        .annotated_type()
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .type_name
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .elem_bitwidth()
                        == 256
                    {
                        issue_compiler_warning(
                            func.try_as_builtin_function_ref()
                                .unwrap()
                                .borrow()
                                .to_ast(),
                            String::from("Possible field prime overflow"),
                            String::from(
                                r#"Private arithmetic 256bit operations overflow at FIELD_PRIME.\nIf you need correct overflow behavior, use a smaller integer type."#,
                            ),
                        );
                        break;
                    }
                }
            } else if func
                .try_as_builtin_function_ref()
                .unwrap()
                .borrow()
                .is_comp()
            {
                for a in &args {
                    if a.try_as_expression_ref()
                        .unwrap()
                        .borrow()
                        .annotated_type()
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .type_name
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .elem_bitwidth()
                        == 256
                    {
                        issue_compiler_warning(
                            func.try_as_builtin_function_ref()
                                .unwrap()
                                .borrow()
                                .to_ast(),
                            String::from("Possible private comparison failure"),
                            String::from(
                                r#"Private 256bit comparison operations will fail for values >= 2^252.\n If you cannot guarantee that the value stays in range, you must use a smaller integer type to ensure correctness."#,
                            ),
                        );
                        break;
                    }
                }
            }

            func.try_as_builtin_function_ref()
                .unwrap()
                .borrow_mut()
                .is_private = true;
            p = Some(Expression::me_expr(None));
        }

        if arg_t != Some(RcCell::new(TypeName::Literal(String::from("lit")))) {
            //Add implicit casts for arguments
            let arg_pt = arg_t
                .unwrap()
                .borrow()
                .annotate(CombinedPrivacyUnion::AST(Some(
                    RcCell::new(p.as_ref().unwrap().clone()).into(),
                )));
            if func
                .try_as_builtin_function_ref()
                .unwrap()
                .borrow()
                .is_shiftop()
                && p.is_some()
            {
                args[0] = self.get_rhs(&args[0], &arg_pt).unwrap();
            } else {
                args = ast
                    .borrow()
                    .args()
                    .iter()
                    .map(|argument| self.get_rhs(argument, &arg_pt).unwrap())
                    .collect();
            }
            ast.borrow_mut().function_call_expr_base_mut_ref().args = args;
        }

        ast.borrow_mut().expression_base_mut_ref().annotated_type = Some(
            out_t
                .unwrap()
                .borrow()
                .annotate(CombinedPrivacyUnion::AST(Some(
                    RcCell::new(p.unwrap().clone()).into(),
                ))),
        );
    }
    pub fn handle_homomorphic_builtin_function_call(
        &self,
        mut ast: &RcCell<FunctionCallExpr>,
        mut func: &ASTFlatten,
    ) {
        //First - same as non-homomorphic - check that argument types conform to op signature
        if !func.try_as_builtin_function_ref().unwrap().borrow().is_eq() {
            for (arg, t) in ast.borrow().args().iter().zip(
                &func
                    .try_as_builtin_function_ref()
                    .unwrap()
                    .borrow()
                    .input_types(),
            ) {
                assert!(
                    arg.try_as_expression_ref()
                        .unwrap()
                        .borrow()
                        .instanceof_data_type(t.as_ref().unwrap()),
                    "{:?},{:?}, {:?}",
                    t,
                    arg.try_as_expression_ref()
                        .unwrap()
                        .borrow()
                        .annotated_type()
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .type_name,
                    arg
                );
            }
        }

        let homomorphic_func = func
            .try_as_builtin_function_ref()
            .unwrap()
            .borrow()
            .select_homomorphic_overload(ast.borrow().args(), ast.borrow().to_expr().analysis());
        assert!(
            homomorphic_func.is_some(),
            r#"Operation \"{}\" requires all arguments to be accessible, i.e. @all or provably equal to @me{:?}"#,
            func.try_as_builtin_function_ref().unwrap().borrow().op,
            ast
        );

        //We could perform homomorphic operations on-chain by using some Solidity arbitrary precision math library.
        //For now, keep it simple and evaluate homomorphic operations in Python and check the result in the circuit.
        func.try_as_builtin_function_ref()
            .unwrap()
            .borrow_mut()
            .is_private = true;

        ast.borrow_mut().expression_base_mut_ref().annotated_type =
            Some(homomorphic_func.clone().unwrap().output_type());
        func.try_as_builtin_function_ref()
            .unwrap()
            .borrow_mut()
            .homomorphism = ast
            .borrow()
            .annotated_type()
            .as_ref()
            .unwrap()
            .borrow()
            .homomorphism
            .clone();
        let expected_arg_types = homomorphic_func.unwrap().input_types();

        //Check that the argument types are correct
        ast.borrow_mut().function_call_expr_base_mut_ref().args = ast
            .borrow()
            .args()
            .iter()
            .zip(expected_arg_types)
            .map(|(arg, arg_pt)| self.get_rhs(arg, &arg_pt).unwrap())
            .collect();
    }
    //@staticmethod
    pub fn is_accessible_by_invoker(_ast: &Expression) -> bool {
        // return ast.annotated_type.is_public() || ast.is_lvalue() || \
        //     ast.instance_of(AnnotatedTypeName(ast.annotated_type.type_name, Expression::me_expr(None)))
        true
    }
    //@staticmethod
    pub fn combine_homomorphism(lhs: ASTFlatten, rhs: ASTFlatten) -> String {
        if lhs
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .annotated_type()
            .as_ref()
            .unwrap()
            .borrow()
            .homomorphism
            == rhs
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .homomorphism
        {
            lhs.try_as_expression_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .homomorphism
                .clone()
        } else if Self::can_rehom(&lhs) {
            rhs.try_as_expression_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .homomorphism
                .clone()
        } else {
            lhs.try_as_expression_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .homomorphism
                .clone()
        }
    }

    //@staticmethod
    pub fn can_rehom(ast: &ASTFlatten) -> bool {
        if ast
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .annotated_type()
            .as_ref()
            .unwrap()
            .borrow()
            .is_accessible(&ast.try_as_expression_ref().unwrap().borrow().analysis())
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
                ast.try_as_function_call_expr_ref().unwrap().borrow().func(),
                ASTType::BuiltinFunction,
            )
            && ast
                .try_as_function_call_expr_ref()
                .unwrap()
                .borrow()
                .func()
                .try_as_builtin_function_ref()
                .unwrap()
                .borrow()
                .is_ite()
            && ast.try_as_function_call_expr_ref().unwrap().borrow().args()[0]
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .is_public()
        {
            return Self::can_rehom(
                &ast.try_as_function_call_expr_ref().unwrap().borrow().args()[1],
            ) && Self::can_rehom(
                &ast.try_as_function_call_expr_ref().unwrap().borrow().args()[2],
            );
        }

        false
    }

    //@staticmethod
    pub fn try_rehom(
        mut rhs: &ASTFlatten,
        expected_type: &RcCell<AnnotatedTypeName>,
    ) -> ASTFlatten {
        assert!(
            !rhs.try_as_expression_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .is_public(),
            "Cannot change the homomorphism of a public value"
        );

        if rhs
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .annotated_type()
            .as_ref()
            .unwrap()
            .borrow()
            .is_private_at_me(&rhs.try_as_expression_ref().unwrap().borrow().analysis())
        {
            //The value is @me, so we can just insert a ReclassifyExpr to change
            //the homomorphism of this value, just like we do for public values.
            return Self::make_rehom(rhs, expected_type);
        }
        if is_instance(rhs, ASTType::ReclassifyExpr) && !is_instance(rhs, ASTType::RehomExpr) {
            //rhs is a valid ReclassifyExpr, i.e. the argument is public or @me-private
            //To create an expression with the correct homomorphism,
            //just change the ReclassifyExpr"s output homomorphism
            rhs.try_as_expression_ref()
                .unwrap()
                .borrow_mut()
                .try_as_reclassify_expr_mut()
                .unwrap()
                .reclassify_expr_base_mut_ref()
                .homomorphism = Some(expected_type.borrow().homomorphism.clone());
        } else if is_instance(rhs, ASTType::PrimitiveCastExpr) {
            //Ignore primitive cast & recurse
            rhs.try_as_expression_ref()
                .unwrap()
                .borrow_mut()
                .try_as_primitive_cast_expr_mut()
                .unwrap()
                .expr = Self::try_rehom(rhs, expected_type);
        } else if is_instance(rhs, ASTType::FunctionCallExprBase)
            && is_instance(
                rhs.try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .func(),
                ASTType::BuiltinFunction,
            )
            && rhs
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .try_as_function_call_expr_ref()
                .unwrap()
                .func()
                .try_as_builtin_function_ref()
                .unwrap()
                .borrow()
                .is_ite()
            && rhs
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .try_as_function_call_expr_ref()
                .unwrap()
                .args()[0]
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .is_public()
        {
            //Argument is public_cond ? true_val : false_val. Try to rehom both true_val and false_val
            let mut args = rhs
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .try_as_function_call_expr_ref()
                .unwrap()
                .args()
                .clone();
            args[1] = Self::try_rehom(&args[1], expected_type);
            args[2] = Self::try_rehom(&args[2], expected_type);
            rhs.try_as_function_call_expr_ref()
                .unwrap()
                .borrow_mut()
                .function_call_expr_base_mut_ref()
                .args = args;
        } else {
            assert!(
                false,
                "{:?}, {:?} ,{:?}",
                expected_type,
                rhs.try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type(),
                rhs
            )
        }

        //Rehom worked without throwing, change annotated_type and return
        rhs.try_as_expression_ref()
            .unwrap()
            .borrow_mut()
            .expression_base_mut_ref()
            .annotated_type = Some(
            rhs.try_as_expression_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .with_homomorphism(expected_type.borrow().homomorphism.clone()),
        );
        rhs.clone()
    }

    //@staticmethod
    pub fn make_rehom(
        mut expr: &ASTFlatten,
        expected_type: &RcCell<AnnotatedTypeName>,
    ) -> ASTFlatten {
        assert!(expected_type
            .borrow()
            .privacy_annotation
            .as_ref()
            .unwrap()
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .privacy_annotation_label()
            .is_some());
        assert!(expr
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .annotated_type()
            .as_ref()
            .unwrap()
            .borrow()
            .is_private_at_me(&expr.try_as_expression_ref().unwrap().borrow().analysis()));
        assert!(expected_type
            .borrow()
            .is_private_at_me(&expr.try_as_expression_ref().unwrap().borrow().analysis()));

        let mut r = RcCell::new(RehomExpr::new(
            expr.clone(),
            Some(expected_type.borrow().homomorphism.clone()),
        ));

        //set type
        let pl = get_privacy_expr_from_label(
            expected_type
                .borrow()
                .privacy_annotation
                .as_ref()
                .unwrap()
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .privacy_annotation_label()
                .unwrap(),
        );
        r.borrow_mut().expression_base_mut_ref().annotated_type =
            Some(RcCell::new(AnnotatedTypeName::new(
                expr.try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .type_name
                    .clone(),
                Some(pl),
                expected_type.borrow().homomorphism.clone(),
            )));
        Self::check_for_invalid_private_type(&r.clone().into());

        //set statement, parents, location
        Self::assign_location(&r.clone().into(), expr);

        r.into()
    }

    //@staticmethod
    pub fn make_private(
        mut expr: &ASTFlatten,
        privacy: &Option<ASTFlatten>,
        homomorphism: &String,
    ) -> ASTFlatten {
        assert!(privacy
            .as_ref()
            .unwrap()
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .privacy_annotation_label()
            .is_some());

        let pl = get_privacy_expr_from_label(
            privacy
                .as_ref()
                .unwrap()
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .privacy_annotation_label()
                .unwrap()
                .into(),
        );
        let mut r = RcCell::new(ReclassifyExprBase::new(
            expr.clone(),
            pl.clone(),
            Some(homomorphism.clone()),
        ));

        //set type
        r.borrow_mut().expression_base_mut_ref().annotated_type =
            Some(RcCell::new(AnnotatedTypeName::new(
                expr.try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .type_name
                    .clone(),
                Some(pl),
                homomorphism.clone(),
            )));
        Self::check_for_invalid_private_type(&r.clone().into());
        //set statement, parents, location
        Self::assign_location(&r.clone().into(), expr);

        r.into()
    }

    //@staticmethod
    pub fn assign_location(target: &ASTFlatten, source: &ASTFlatten) {
        //set statement
        target
            .try_as_expression_ref()
            .unwrap()
            .borrow_mut()
            .expression_base_mut_ref()
            .statement = source
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .statement()
            .clone();
        let t = ASTFlatten::from(target.clone()).downgrade();
        //set parents
        target
            .try_as_expression_ref()
            .unwrap()
            .borrow_mut()
            .ast_base_mut_ref()
            .borrow_mut()
            .parent = source
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .parent()
            .clone();
        let mut annotated_type = target
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .annotated_type()
            .clone();
        annotated_type
            .as_mut()
            .unwrap()
            .borrow_mut()
            .ast_base
            .borrow_mut()
            .parent = Some(t.clone());
        target
            .try_as_expression_ref()
            .unwrap()
            .borrow_mut()
            .expression_base_mut_ref()
            .annotated_type = annotated_type;
        source
            .try_as_expression_ref()
            .unwrap()
            .borrow_mut()
            .ast_base_mut_ref()
            .borrow_mut()
            .parent = Some(t);

        //set source location
        target
            .try_as_expression_ref()
            .unwrap()
            .borrow_mut()
            .ast_base_mut_ref()
            .borrow_mut()
            .line = source
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .ast_base_ref()
            .borrow()
            .line;
        target
            .try_as_expression_ref()
            .unwrap()
            .borrow_mut()
            .ast_base_mut_ref()
            .borrow_mut()
            .column = source
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .ast_base_ref()
            .borrow()
            .column;
    }

    //@staticmethod
    pub fn implicitly_converted_to(expr: &ASTFlatten, t: &RcCell<TypeName>) -> ASTFlatten {
        if is_instance(expr, ASTType::ReclassifyExpr)
            && !expr
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .try_as_reclassify_expr_ref()
                .unwrap()
                .privacy()
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .is_all_expr()
        {
            //Cast the argument of the ReclassifyExpr instead
            let exp = Self::implicitly_converted_to(
                expr.try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .try_as_reclassify_expr_ref()
                    .unwrap()
                    .expr(),
                t,
            );
            expr.try_as_expression_ref()
                .unwrap()
                .borrow_mut()
                .try_as_reclassify_expr_mut()
                .unwrap()
                .reclassify_expr_base_mut_ref()
                .expr = exp;
            let mut expr_annotated_type = expr
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .clone();
            expr_annotated_type.as_mut().unwrap().borrow_mut().type_name = expr
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .try_as_reclassify_expr_ref()
                .unwrap()
                .expr()
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .type_name
                .clone();
            expr.try_as_expression_ref()
                .unwrap()
                .borrow_mut()
                .expression_base_mut_ref()
                .annotated_type = expr_annotated_type;
            return expr.clone();
        }

        assert!(expr
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .annotated_type()
            .as_ref()
            .unwrap()
            .borrow()
            .type_name
            .as_ref()
            .unwrap()
            .borrow()
            .is_primitive_type());
        let mut cast = RcCell::new(PrimitiveCastExpr::new(t.clone(), expr.clone(), true));
        let cast_weak = ASTFlatten::from(cast.clone()).downgrade();
        cast.borrow_mut().ast_base_mut_ref().borrow_mut().parent = expr
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .parent()
            .clone();
        cast.borrow_mut().expression_base_mut_ref().statement = expr
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .statement()
            .clone();
        cast.borrow_mut().ast_base_mut_ref().borrow_mut().line =
            expr.try_as_expression_ref().unwrap().borrow().line();
        cast.borrow_mut().ast_base_mut_ref().borrow_mut().column =
            expr.try_as_expression_ref().unwrap().borrow().column();
        cast.borrow_mut()
            .elem_type
            .borrow_mut()
            .ast_base_mut_ref()
            .unwrap()
            .borrow_mut()
            .parent = Some(cast_weak.clone());
        expr.try_as_expression_ref()
            .unwrap()
            .borrow_mut()
            .ast_base_mut_ref()
            .borrow_mut()
            .parent = Some(cast_weak.clone());
        cast.borrow_mut().expression_base_mut_ref().annotated_type =
            Some(RcCell::new(AnnotatedTypeName::new(
                Some(t.clone()),
                expr.try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .privacy_annotation
                    .clone(),
                expr.try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .homomorphism
                    .clone(),
            )));
        cast.borrow_mut()
            .expression_base_mut_ref()
            .annotated_type
            .as_mut()
            .unwrap()
            .borrow_mut()
            .ast_base
            .borrow_mut()
            .parent = Some(cast_weak);
        cast.into()
    }

    pub fn visitFunctionCallExpr(
        &self,
        mut ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        if is_instance(
            ast.try_as_function_call_expr_ref().unwrap().borrow().func(),
            ASTType::BuiltinFunction,
        ) {
            self.handle_builtin_function_call(
                ast.try_as_function_call_expr_ref().unwrap(),
                &ast.try_as_function_call_expr_ref()
                    .unwrap()
                    .borrow()
                    .function_call_expr_base_ref()
                    .func,
            );
        } else if ast
            .try_as_function_call_expr_ref()
            .unwrap()
            .borrow()
            .is_cast()
        {
            assert!(
                is_instance(
                    &ast.try_as_function_call_expr_ref()
                        .unwrap()
                        .borrow()
                        .func()
                        .try_as_expression_ref()
                        .unwrap()
                        .borrow()
                        .try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .try_as_location_expr_ref()
                        .unwrap()
                        .target()
                        .clone()
                        .unwrap()
                        .upgrade()
                        .unwrap(),
                    ASTType::EnumDefinition
                ),
                "User type casts only implemented for enums"
            );
            ast.try_as_function_call_expr_ref()
                .unwrap()
                .borrow_mut()
                .expression_base_mut_ref()
                .annotated_type = Some(
                self.handle_cast(
                    &ast.try_as_function_call_expr_ref().unwrap().borrow().args()[0],
                    ast.try_as_function_call_expr_ref()
                        .unwrap()
                        .borrow()
                        .func()
                        .try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .borrow()
                        .try_as_location_expr_ref()
                        .unwrap()
                        .target()
                        .clone()
                        .unwrap()
                        .upgrade()
                        .unwrap()
                        .try_as_expression_ref()
                        .unwrap()
                        .borrow()
                        .annotated_type()
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .type_name
                        .as_ref()
                        .unwrap(),
                ),
            );
        } else if is_instance(
            ast.try_as_function_call_expr_ref().unwrap().borrow().func(),
            ASTType::LocationExprBase,
        ) {
            let ft = ast
                .try_as_function_call_expr_ref()
                .unwrap()
                .borrow()
                .func()
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .type_name
                .clone()
                .unwrap();
            assert!(is_instance(&ft, ASTType::FunctionTypeName));

            assert!(
                ft.borrow()
                    .try_as_function_type_name_ref()
                    .unwrap()
                    .parameters
                    .len()
                    == ast
                        .try_as_function_call_expr_ref()
                        .unwrap()
                        .borrow()
                        .args()
                        .len(),
                "Wrong number of arguments {:?}",
                ast.try_as_function_call_expr_ref().unwrap().borrow().func()
            );

            //Check arguments
            let mut args = ast
                .try_as_function_call_expr_ref()
                .unwrap()
                .borrow()
                .args()
                .clone();
            for i in 0..ast
                .try_as_function_call_expr_ref()
                .unwrap()
                .borrow()
                .args()
                .len()
            {
                args[i] = self
                    .get_rhs(
                        &args[i],
                        ft.borrow()
                            .try_as_function_type_name_ref()
                            .unwrap()
                            .parameters[i]
                            .borrow()
                            .identifier_declaration_base
                            .annotated_type
                            .as_ref()
                            .unwrap(),
                    )
                    .unwrap();
            }
            ast.try_as_function_call_expr_ref()
                .unwrap()
                .borrow_mut()
                .function_call_expr_base_mut_ref()
                .args = args;

            //Set expression type to return type
            ast.try_as_function_call_expr_ref()
                .unwrap()
                .borrow_mut()
                .expression_base_mut_ref()
                .annotated_type = Some(
                if ft
                    .borrow()
                    .try_as_function_type_name_ref()
                    .unwrap()
                    .return_parameters
                    .len()
                    == 1
                {
                    ft.borrow()
                        .try_as_function_type_name_ref()
                        .unwrap()
                        .return_parameters[0]
                        .borrow()
                        .identifier_declaration_base
                        .annotated_type
                        .clone()
                        .unwrap()
                } else {
                    //TODO maybe not None label in the future
                    RcCell::new(AnnotatedTypeName::new(
                        Some(RcCell::new(TypeName::TupleType(TupleType::new(
                            ft.borrow()
                                .try_as_function_type_name_ref()
                                .unwrap()
                                .return_parameters
                                .iter()
                                .filter_map(|t| {
                                    t.borrow()
                                        .identifier_declaration_base
                                        .annotated_type
                                        .clone()
                                })
                                .collect(),
                        )))),
                        None,
                        String::from("NON_HOMOMORPHISM"),
                    ))
                },
            );
        } else {
            assert!(false, "Invalid function call{:?}", ast);
        }
        Ok(())
    }

    pub fn visitPrimitiveCastExpr(
        &self,
        mut ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        ast.try_as_primitive_cast_expr_ref()
            .unwrap()
            .borrow_mut()
            .expression_base
            .annotated_type = Some(
            self.handle_cast(
                &ast.try_as_primitive_cast_expr_ref().unwrap().borrow().expr,
                &ast.try_as_primitive_cast_expr_ref()
                    .unwrap()
                    .borrow()
                    .elem_type,
            ),
        );
        Ok(())
    }

    pub fn handle_cast(
        &self,
        expr: &ASTFlatten,
        t: &RcCell<TypeName>,
    ) -> RcCell<AnnotatedTypeName> {
        //because of the fake solidity check we already know that the cast is possible -> don"t have to check if cast possible
        RcCell::new(
            if expr
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .is_private()
            {
                let expected = RcCell::new(AnnotatedTypeName::new(
                    expr.try_as_expression_ref()
                        .unwrap()
                        .borrow()
                        .annotated_type()
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .type_name
                        .clone(),
                    Some(RcCell::new(Expression::me_expr(None)).into()),
                    String::from("NON_HOMOMORPHISM"),
                ));
                assert!(
                    Some(String::from("true"))
                        != expr
                            .try_as_expression_ref()
                            .unwrap()
                            .borrow()
                            .instance_of(&expected),
                    "{:?}, {:?}, {:?}",
                    expected,
                    expr.try_as_expression_ref()
                        .unwrap()
                        .borrow()
                        .annotated_type(),
                    expr
                );
                AnnotatedTypeName::new(
                    Some(t.clone()),
                    Some(RcCell::new(Expression::me_expr(None)).into()),
                    String::from("NON_HOMOMORPHISM"),
                )
            } else {
                AnnotatedTypeName::new(Some(t.clone()), None, String::from("NON_HOMOMORPHISM"))
            },
        )
    }

    pub fn visitNewExpr(&self, _ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        //already has correct type
        // pass
        Ok(())
    }

    pub fn visitMemberAccessExpr(
        &self,
        mut ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        assert!(ast
            .try_as_member_access_expr_ref()
            .unwrap()
            .borrow()
            .location_expr_base
            .target
            .is_some());

        assert!(
            !(ast
                .try_as_member_access_expr_ref()
                .unwrap()
                .borrow()
                .expr
                .as_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .is_address()
                && ast
                    .try_as_member_access_expr_ref()
                    .unwrap()
                    .borrow()
                    .expr
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .is_private()),
            "Cannot access members of private address variable{:?}",
            ast
        );
        ast.try_as_member_access_expr_ref()
            .unwrap()
            .borrow_mut()
            .expression_base_mut_ref()
            .annotated_type = ast
            .try_as_member_access_expr_ref()
            .unwrap()
            .borrow()
            .target()
            .clone()
            .unwrap()
            .upgrade()
            .unwrap()
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .annotated_type()
            .clone();
        Ok(())
    }

    pub fn visitReclassifyExpr(
        &self,
        mut ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        assert!(
            ast.try_as_reclassify_expr_ref()
                .unwrap()
                .borrow()
                .privacy()
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .privacy_annotation_label()
                .is_none(),
            r#"Second argument of "reveal" cannot be used as a privacy type{:?}"#,
            ast
        );

        let mut homomorphism = Homomorphism::non_homomorphic();
        assert!(!homomorphism.is_empty());

        //Prevent ReclassifyExpr to all with homomorphic type
        if ast
            .try_as_reclassify_expr_ref()
            .unwrap()
            .borrow()
            .privacy()
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .is_all_expr()
            && (ast
                .try_as_reclassify_expr_ref()
                .unwrap()
                .borrow()
                .homomorphism()
                != &Some(Homomorphism::non_homomorphic())
                || ast
                    .try_as_reclassify_expr_ref()
                    .unwrap()
                    .borrow()
                    .expr()
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .homomorphism
                    != Homomorphism::non_homomorphic())
        {
            //If the target privacy is all, we infer a target homomorphism of NonHomomorphic
            homomorphism = Homomorphism::non_homomorphic();
            ast.try_as_reclassify_expr_ref()
                .unwrap()
                .borrow_mut()
                .reclassify_expr_base_mut_ref()
                .homomorphism = Some(homomorphism.clone());
        }

        //Make sure the first argument to reveal / rehom is public or private provably equal to @me
        let is_expr_at_all = ast
            .try_as_reclassify_expr_ref()
            .unwrap()
            .borrow()
            .expr()
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .annotated_type()
            .as_ref()
            .unwrap()
            .borrow()
            .is_public();
        let is_expr_at_me = ast
            .try_as_reclassify_expr_ref()
            .unwrap()
            .borrow()
            .expr()
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .annotated_type()
            .as_ref()
            .unwrap()
            .borrow()
            .is_private_at_me(
                &ast.try_as_reclassify_expr_ref()
                    .unwrap()
                    .borrow()
                    .to_expr()
                    .analysis(),
            );
        assert!(
            is_expr_at_all || is_expr_at_me,
            r#"First argument of "{}" must be accessible,"i.e. @all or provably equal to @me{:?}"#,
            ast.try_as_reclassify_expr_ref()
                .unwrap()
                .borrow()
                .func_name(),
            ast
        );

        //Prevent unhom(public_value)

        assert!(
            !(is_expr_at_all
                && is_instance(ast, ASTType::RehomExpr)
                && ast
                    .try_as_reclassify_expr_ref()
                    .unwrap()
                    .borrow()
                    .homomorphism()
                    == &Some(Homomorphism::non_homomorphic())),
            r#"Cannot use "{}" on a public value{:?}"#,
            HOMOMORPHISM_STORE
                .lock()
                .unwrap()
                .get(
                    ast.try_as_reclassify_expr_ref()
                        .unwrap()
                        .borrow()
                        .homomorphism()
                        .as_ref()
                        .unwrap()
                )
                .unwrap()
                .rehom_expr_name,
            ast
        );

        //NB prevent any redundant reveal (not just for public)
        ast.try_as_reclassify_expr_ref()
            .unwrap()
            .borrow_mut()
            .expression_base_mut_ref()
            .annotated_type = Some(RcCell::new(AnnotatedTypeName::new(
            ast.try_as_reclassify_expr_ref()
                .unwrap()
                .borrow()
                .expr()
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .type_name
                .clone(),
            Some(ast.clone()),
            homomorphism.clone(),
        )));

        assert!(
            Some(String::from("true"))
                != ast
                    .try_as_reclassify_expr_ref()
                    .unwrap()
                    .borrow()
                    .to_expr()
                    .instance_of(
                        &ast.try_as_reclassify_expr_ref()
                            .unwrap()
                            .borrow()
                            .expr()
                            .try_as_expression_ref()
                            .unwrap()
                            .borrow()
                            .annotated_type()
                            .as_ref()
                            .unwrap()
                    ),
            r#"Redundant "{}": Expression is already @{}{homomorphism}"{:?}"#,
            ast.try_as_reclassify_expr_ref()
                .unwrap()
                .borrow()
                .func_name(),
            ast.try_as_reclassify_expr_ref()
                .unwrap()
                .borrow()
                .privacy()
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .to_ast()
                .code(),
            ast
        );
        Self::check_for_invalid_private_type(ast);
        Ok(())
    }

    pub fn visitIfStatement(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        let b = &ast.try_as_if_statement_ref().unwrap().borrow().condition;
        assert!(
            b.try_as_expression_ref()
                .unwrap()
                .borrow()
                .instanceof_data_type(&RcCell::new(TypeName::bool_type())),
            "{:?}, {:?} ,{:?}",
            TypeName::bool_type(),
            b.try_as_expression_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .type_name,
            b
        );
        if ast
            .try_as_if_statement_ref()
            .unwrap()
            .borrow()
            .condition
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .annotated_type()
            .as_ref()
            .unwrap()
            .borrow()
            .is_private()
        {
            let expected = RcCell::new(AnnotatedTypeName::new(
                Some(RcCell::new(TypeName::bool_type())),
                Some(RcCell::new(Expression::me_expr(None)).into()),
                String::from("NON_HOMOMORPHISM"),
            ));
            assert!(
                Some(String::from("true"))
                    == b.try_as_expression_ref()
                        .unwrap()
                        .borrow()
                        .instance_of(&expected),
                "{:?}, {:?} ,{:?}",
                expected,
                b.try_as_expression_ref().unwrap().borrow().annotated_type(),
                b
            )
        }
        Ok(())
    }

    pub fn visitWhileStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        assert!(
            Some(String::from("true"))
                == ast
                    .try_as_while_statement_ref()
                    .unwrap()
                    .borrow()
                    .condition
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .instance_of(&AnnotatedTypeName::bool_all()),
            "{:?}, {:?} ,{:?}",
            AnnotatedTypeName::bool_all(),
            ast.try_as_while_statement_ref()
                .unwrap()
                .borrow()
                .condition
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .annotated_type(),
            ast.try_as_while_statement_ref().unwrap().borrow().condition
        );
        //must also later check that body and condition do not contain private expressions
        Ok(())
    }

    pub fn visitForStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        assert!(
            Some(String::from("true"))
                == ast
                    .try_as_for_statement_ref()
                    .unwrap()
                    .borrow()
                    .condition
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .instance_of(&AnnotatedTypeName::bool_all()),
            "{:?}, {:?} ,{:?}",
            AnnotatedTypeName::bool_all(),
            ast.try_as_for_statement_ref()
                .unwrap()
                .borrow()
                .condition
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .annotated_type(),
            ast.try_as_for_statement_ref().unwrap().borrow().condition
        );
        //must also later check that body, update and condition do not contain private expressions
        Ok(())
    }
    pub fn visitReturnStatement(
        &self,
        mut ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        assert!(ast
            .try_as_return_statement_ref()
            .unwrap()
            .borrow()
            .statement_base
            .function
            .clone()
            .unwrap()
            .upgrade()
            .unwrap()
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .borrow()
            .is_function());
        let rt = RcCell::new(AnnotatedTypeName::new(
            Some(RcCell::new(TypeName::TupleType(
                ast.try_as_return_statement_ref()
                    .unwrap()
                    .borrow()
                    .statement_base
                    .function
                    .clone()
                    .unwrap()
                    .upgrade()
                    .unwrap()
                    .try_as_constructor_or_function_definition_ref()
                    .unwrap()
                    .borrow()
                    .return_type(),
            ))),
            None,
            String::from("NON_HOMOMORPHISM"),
        ));
        if ast
            .try_as_return_statement_ref()
            .unwrap()
            .borrow()
            .expr
            .is_some()
        {
            self.get_rhs(&RcCell::new(TupleExpr::new(vec![])).into(), &rt);
        } else if !is_instance(
            ast.try_as_return_statement_ref()
                .unwrap()
                .borrow()
                .expr
                .as_ref()
                .unwrap(),
            ASTType::TupleExpr,
        ) {
            ast.try_as_return_statement_ref().unwrap().borrow_mut().expr = self.get_rhs(
                &RcCell::new(TupleExpr::new(vec![ast
                    .try_as_return_statement_ref()
                    .unwrap()
                    .borrow()
                    .expr
                    .clone()
                    .unwrap()
                    .into()]))
                .into(),
                &rt,
            );
        } else {
            ast.try_as_return_statement_ref().unwrap().borrow_mut().expr = self.get_rhs(
                ast.try_as_return_statement_ref()
                    .unwrap()
                    .borrow()
                    .expr
                    .as_ref()
                    .unwrap(),
                &rt,
            );
        }
        Ok(())
    }
    pub fn visitTupleExpr(
        &self,
        mut ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        ast.try_as_tuple_expr_ref()
            .unwrap()
            .borrow_mut()
            .expression_base_mut_ref()
            .annotated_type = Some(RcCell::new(AnnotatedTypeName::new(
            Some(RcCell::new(TypeName::TupleType(TupleType::new(
                ast.try_as_tuple_expr_ref()
                    .unwrap()
                    .borrow()
                    .elements
                    .iter()
                    .map(|elem| {
                        elem.try_as_expression_ref()
                            .unwrap()
                            .borrow()
                            .annotated_type()
                            .as_ref()
                            .unwrap()
                            .clone()
                    })
                    .collect(),
            )))),
            None,
            String::from("NON_HOMOMORPHISM"),
        )));
        Ok(())
    }

    pub fn visitMeExpr(&self, mut ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        ast.try_as_me_expr_ref()
            .unwrap()
            .borrow_mut()
            .expression_base
            .annotated_type = Some(AnnotatedTypeName::address_all());
        Ok(())
    }

    pub fn visitIdentifierExpr(
        &self,
        mut ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // if is_instance(&ast.location_expr_base.target, ASTType::Mapping) { //no action necessary, the identifier will be replaced later
        // pass
        let target = ast
            .to_ast()
            .try_as_expression_ref()
            .unwrap()
            .try_as_tuple_or_location_expr_ref()
            .unwrap()
            .try_as_location_expr_ref()
            .unwrap()
            .try_as_identifier_expr_ref()
            .unwrap()
            .location_expr_base
            .target()
            .as_ref()
            .and_then(|t| t.clone().upgrade())
            .unwrap();
        if is_instance(&target, ASTType::Mapping) {
            // no action necessary, the identifier will be replaced later
            return Ok(());
        }

        assert!(
            !is_instance(&target, ASTType::ContractDefinition),
            "Unsupported use of contract type in expression {:?}",
            ast
        );
        if ast.is_identifier_expr() {
            ast.try_as_identifier_expr_ref()
                .unwrap()
                .borrow_mut()
                .annotated_type = target
                .to_ast()
                .try_as_identifier_declaration_ref()
                .unwrap()
                .annotated_type()
                .clone();
        } else if ast.is_expression() {
            ast.try_as_expression_ref()
                .unwrap()
                .borrow_mut()
                .try_as_tuple_or_location_expr_mut()
                .unwrap()
                .try_as_location_expr_mut()
                .unwrap()
                .try_as_identifier_expr_mut()
                .unwrap()
                .annotated_type = target
                .to_ast()
                .try_as_identifier_declaration_ref()
                .unwrap()
                .annotated_type()
                .clone();
        } else {
            println!("===========else===========");
            eyre::bail!("======else==============");
        }

        assert!(
            Self::is_accessible_by_invoker(
                &ast.to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .try_as_identifier_expr_ref()
                    .unwrap()
                    .to_expr()
            ),
            "Tried to read value which cannot be proven to be owned by the transaction invoker{:?}",
            ast
        );

        Ok(())
    }
    pub fn visitIndexExpr(
        &self,
        mut ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let arr = ast
            .try_as_index_expr_ref()
            .unwrap()
            .borrow()
            .arr
            .clone()
            .unwrap();
        let index = ast.try_as_index_expr_ref().unwrap().borrow().key.clone();
        let mut map_t = arr.borrow().annotated_type().as_ref().unwrap().clone();
        //should have already been checked
        assert!(map_t
            .borrow()
            .privacy_annotation
            .as_ref()
            .unwrap()
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .is_all_expr());
        let mut tn = map_t.borrow().type_name.clone().unwrap();
        //do actual type checking
        if is_instance(&tn, ASTType::Mapping) {
            let key_type = tn.borrow().try_as_mapping_ref().unwrap().key_type.clone();
            let expected = RcCell::new(AnnotatedTypeName::new(
                Some(key_type),
                Some(RcCell::new(Expression::all_expr()).into()),
                String::from("NON_HOMOMORPHISM"),
            ));
            let instance = index
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .instance_of(&expected);
            assert!(
                Some(String::from("true")) == instance,
                "{:?}, {:?} ,{:?}",
                expected,
                index
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type(),
                ast
            );

            //record indexing information
            if tn
                .borrow()
                .try_as_mapping_ref()
                .unwrap()
                .key_label
                .is_some()
            //TODO modification correct?
            {
                assert!(
                    index
                        .try_as_expression_ref()
                        .unwrap()
                        .borrow()
                        .privacy_annotation_label()
                        .is_some(),
                    "Index cannot be used as a privacy type for array of type {:?}{:?}",
                    map_t,
                    ast
                );
                tn.borrow_mut()
                    .try_as_mapping_mut()
                    .unwrap()
                    .instantiated_key = Some(index);
            }
            //determine value type
            ast.try_as_index_expr_ref()
                .unwrap()
                .borrow_mut()
                .expression_base_mut_ref()
                .annotated_type =
                Some(tn.borrow().try_as_mapping_ref().unwrap().value_type.clone());

            assert!(Self::is_accessible_by_invoker(&ast.try_as_index_expr_ref().unwrap().borrow().to_expr()) ,"Tried to read value which cannot be proven to be owned by the transaction invoker{:?}", ast);
        } else if let TypeName::Array(type_name) =
            &*map_t.borrow().type_name.as_ref().unwrap().borrow()
        {
            assert!(
                !ast.try_as_index_expr_ref()
                    .unwrap()
                    .borrow()
                    .key
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .is_private(),
                "No private array index{:?}",
                ast
            );
            assert!(
                ast.try_as_index_expr_ref()
                    .unwrap()
                    .borrow()
                    .key
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .instanceof_data_type(&RcCell::new(TypeName::number_type())),
                "Array index must be numeric{:?}",
                ast
            );
            ast.try_as_index_expr_ref()
                .unwrap()
                .borrow_mut()
                .expression_base_mut_ref()
                .annotated_type = Some(type_name.value_type().clone());
        } else {
            assert!(false, "Indexing into non-mapping{:?}", ast);
        }
        Ok(())
    }
    pub fn visitConstructorOrFunctionDefinition(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        for t in ast
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .borrow()
            .parameter_types()
            .types
        {
            assert!(
                is_instances(
                    t.borrow().privacy_annotation.as_ref().unwrap(),
                    vec![ASTType::MeExpr, ASTType::AllExpr],
                ),
                "Only me/all accepted as privacy type of function parameters{:?}",
                ast
            );
        }

        if ast
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .borrow()
            .can_be_external()
        {
            for t in ast
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .borrow()
                .return_type()
                .types
            {
                assert!(is_instances(
                    t.borrow().privacy_annotation.as_ref().unwrap(),
                    vec![ASTType::MeExpr, ASTType::AllExpr],
                ),"Only me/all accepted as privacy type of return values for public functions{:?}", ast);
            }
        }
        Ok(())
    }
    pub fn visitEnumDefinition(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let mut etn = EnumTypeName::new(
            ast.try_as_enum_definition_ref()
                .unwrap()
                .borrow()
                .qualified_name(),
            None,
        );
        etn.user_defined_type_name_base.target = Some(ast.clone().downgrade());
        ast.try_as_enum_definition_ref()
            .unwrap()
            .borrow_mut()
            .annotated_type = Some(AnnotatedTypeName::new(
            Some(RcCell::new(TypeName::UserDefinedTypeName(
                UserDefinedTypeName::EnumTypeName(etn),
            ))),
            None,
            String::from("NON_HOMOMORPHIM"),
        ));
        Ok(())
    }

    pub fn visitEnumValue(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        let mut evtn = EnumValueTypeName::new(
            ast.try_as_enum_value_ref()
                .unwrap()
                .borrow()
                .qualified_name(),
            None,
        );
        evtn.user_defined_type_name_base.target = Some(ast.clone().downgrade());
        ast.try_as_enum_value_ref()
            .unwrap()
            .borrow_mut()
            .annotated_type = Some(AnnotatedTypeName::new(
            Some(RcCell::new(TypeName::UserDefinedTypeName(
                UserDefinedTypeName::EnumValueTypeName(evtn),
            ))),
            None,
            String::from("NON_HOMOMORPHISM"),
        ));
        Ok(())
    }

    pub fn visitStateVariableDeclaration(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        if let Some(expr) = &ast
            .try_as_state_variable_declaration_ref()
            .unwrap()
            .borrow()
            .expr
        {
            //prevent private operations in declaration
            assert!(
                !contains_private(ast),
                "Private assignments to state variables must be in the constructor{:?}",
                ast
            );

            //check type
            self.get_rhs(
                expr,
                ast.try_as_state_variable_declaration_ref()
                    .unwrap()
                    .borrow()
                    .identifier_declaration_base
                    .annotated_type
                    .as_ref()
                    .unwrap(),
            );
        }

        //prevent "me" annotation
        let p = ast
            .try_as_state_variable_declaration_ref()
            .unwrap()
            .borrow()
            .identifier_declaration_base
            .annotated_type
            .as_ref()
            .unwrap()
            .borrow()
            .privacy_annotation
            .as_ref()
            .unwrap()
            .clone();
        assert!(
            !p.try_as_expression_ref().unwrap().borrow().is_me_expr(),
            "State variables cannot be annotated as me{:?}",
            ast
        );
        Ok(())
    }

    pub fn visitMapping(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        if ast
            .to_ast()
            .try_as_type_name_ref()
            .unwrap()
            .try_as_mapping_ref()
            .unwrap()
            .key_label
            .is_some()
        {
            assert!(
                ast.to_ast()
                    .try_as_type_name_ref()
                    .unwrap()
                    .try_as_mapping_ref()
                    .unwrap()
                    .key_type
                    .borrow()
                    .get_ast_type()
                    == TypeName::address_type().get_ast_type(),
                "Only addresses can be annotated {:?}",
                ast
            );
        }
        Ok(())
    }

    pub fn visitRequireStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        assert!(
            ast.try_as_require_statement_ref()
                .unwrap()
                .borrow()
                .condition
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .privacy_annotation
                .as_ref()
                .unwrap()
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .is_all_expr(),
            "require needs public argument{:?}",
            ast
        );
        Ok(())
    }

    pub fn visitAnnotatedTypeName(
        &self,
        mut ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        if ast
            .try_as_annotated_type_name_ref()
            .unwrap()
            .borrow()
            .type_name
            .is_some()
            && ast
                .try_as_annotated_type_name_ref()
                .unwrap()
                .borrow()
                .type_name
                .as_ref()
                .unwrap()
                .get_ast_type()
                == ASTType::UserDefinedTypeNameBase
        {
            assert!(
                is_instance(
                    &ast.try_as_annotated_type_name_ref()
                        .unwrap()
                        .borrow()
                        .type_name
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .try_as_user_defined_type_name_ref()
                        .unwrap()
                        .target()
                        .clone()
                        .unwrap()
                        .upgrade()
                        .unwrap(),
                    ASTType::EnumDefinition
                ),
                "Unsupported use of user-defined type {:?},===={:?}==={:?}",
                ast.try_as_annotated_type_name_ref()
                    .unwrap()
                    .borrow()
                    .type_name,
                ast,
                ast.try_as_annotated_type_name_ref()
                    .unwrap()
                    .borrow()
                    .type_name
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .try_as_user_defined_type_name_ref()
                    .unwrap()
                    .target()
                    .clone()
                    .unwrap()
                    .upgrade()
            );
            let tn = ast
                .try_as_annotated_type_name_ref()
                .unwrap()
                .borrow()
                .type_name
                .as_ref()
                .unwrap()
                .borrow()
                .try_as_user_defined_type_name_ref()
                .unwrap()
                .target()
                .clone()
                .unwrap()
                .upgrade()
                .unwrap()
                .to_ast()
                .try_as_namespace_definition_ref()
                .unwrap()
                .try_as_enum_definition_ref()
                .unwrap()
                .annotated_type
                .as_ref()
                .unwrap()
                .type_name
                .clone();
            ast.try_as_annotated_type_name_ref()
                .unwrap()
                .borrow_mut()
                .type_name = tn;
        }

        if ast
            .try_as_annotated_type_name_ref()
            .unwrap()
            .borrow()
            .privacy_annotation
            .as_ref()
            .map_or(false, |pa| !is_instance(pa, ASTType::AllExpr))
        {
            // println!("========can_be_private========================{ast:?}");
            assert!(
                ast.try_as_annotated_type_name_ref()
                    .unwrap()
                    .borrow()
                    .type_name
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .can_be_private(),
                "Currently, we do not support private {:?},{:?}",
                ast.try_as_annotated_type_name_ref()
                    .unwrap()
                    .borrow()
                    .type_name,
                ast
            );
            if ast
                .try_as_annotated_type_name_ref()
                .unwrap()
                .borrow()
                .homomorphism
                != Homomorphism::non_homomorphic()
            {
                //only support uint8, uint16, uint24, uint32 homomorphic data types
                assert!(
                    ast.try_as_annotated_type_name_ref()
                        .unwrap()
                        .borrow()
                        .type_name
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .is_numeric(),
                    "Homomorphic type not supported for {:?}: Only numeric types supported{:?}",
                    ast.try_as_annotated_type_name_ref()
                        .unwrap()
                        .borrow()
                        .type_name,
                    ast
                );
                assert!(
                    !ast.try_as_annotated_type_name_ref()
                        .unwrap()
                        .borrow()
                        .type_name
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .signed(),
                    "Homomorphic type not supported for {:?}: Only unsigned types supported{:?}",
                    ast.try_as_annotated_type_name_ref()
                        .unwrap()
                        .borrow()
                        .type_name,
                    ast
                );
                assert!(ast.try_as_annotated_type_name_ref().unwrap().borrow().type_name.as_ref().unwrap().borrow().elem_bitwidth() <= 32,"Homomorphic type not supported for {:?}: Only up to 32-bit numeric types supported{:?}", ast.try_as_annotated_type_name_ref().unwrap().borrow().type_name,ast);
            }
        }
        let p = ast
            .try_as_annotated_type_name_ref()
            .unwrap()
            .borrow()
            .privacy_annotation
            .as_ref()
            .unwrap()
            .clone();
        if is_instance(&p, ASTType::IdentifierExpr) {
            let t = p
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .target()
                .as_ref()
                .and_then(|t| t.clone().upgrade())
                .unwrap();
            if !is_instance(&t, ASTType::Mapping) {
                //no action necessary, this is the case: mapping(address!x => uint@x)
                // pass
                assert!(
                    t.to_ast()
                        .try_as_identifier_declaration_ref()
                        .unwrap()
                        .identifier_declaration_base_ref()
                        .is_final()
                        || t.to_ast()
                            .try_as_identifier_declaration_ref()
                            .unwrap()
                            .identifier_declaration_base_ref()
                            .is_constant(),
                    r#"Privacy annotations must be "final" or "constant", if they are expressions {:?}'s {:?}"#,
                    p,
                    t
                );
                assert!(
                    t.to_ast()
                        .try_as_identifier_declaration_ref()
                        .unwrap()
                        .annotated_type()
                        == &Some(AnnotatedTypeName::address_all()),
                    r#"Privacy type is not a public address, but {:?},{:?}"#,
                    t.to_ast()
                        .try_as_identifier_declaration_ref()
                        .unwrap()
                        .annotated_type(),
                    p
                );
            }
        }
        Ok(())
    }
}
