use crate::type_check::contains_private::contains_private;
use crate::type_check::final_checker::check_final;
// use crate::type_check::type_exceptions::{TypeMismatchException, TypeException};
use crate::zkay_ast::homomorphism::{Homomorphism, HOMOMORPHISM_STORE, REHOM_EXPRESSIONS};

use crate::zkay_ast::ast::{
    get_privacy_expr_from_label, is_instance, is_instances, issue_compiler_warning, ASTCode,
    ASTType, AllExpr, AnnotatedTypeName, Array, AsTypeUnion, AssignmentStatement,
    AssignmentStatementUnion, BooleanLiteralType, BuiltinFunction, CombinedPrivacyUnion,
    ConstructorOrFunctionDefinition, ContractDefinition, ElementaryTypeName, EnumDefinition,
    EnumTypeName, EnumValue, EnumValueTypeName, Expression, ForStatement, FunctionCallExpr,
    FunctionTypeName, IdentifierDeclaration, IdentifierExpr, IfStatement, IndexExpr, LiteralUnion,
    LocationExpr, Mapping, MeExpr, MemberAccessExpr, NamespaceDefinition, NewExpr,
    NumberLiteralType, NumberLiteralTypeUnion, NumberTypeName, PrimitiveCastExpr, ReclassifyExpr,
    ReclassifyExprBase, RehomExpr, RequireStatement, ReturnStatement, StateVariableDeclaration,
    TargetDefinition, TupleExpr, TupleType, TypeName, UserDefinedTypeName,
    VariableDeclarationStatement, WhileStatement, AST,
};
use crate::zkay_ast::visitor::deep_copy::replace_expr;
use crate::zkay_ast::visitor::visitor::AstVisitor;

pub fn type_check(ast: AST) {
    check_final(ast);
    let v = TypeCheckVisitor;
    v.visit(ast);
}

// class TypeCheckVisitor(AstVisitor)
pub struct TypeCheckVisitor;
impl AstVisitor for TypeCheckVisitor {
    type Return = Option<String>;
    fn temper_result(&self) -> Self::Return {
        None
    }
    fn log(&self) -> bool {
        false
    }
    fn traversal(&self) -> &'static str {
        "node-or-children"
    }
    fn has_attr(&self, name: &String) -> bool {
        self.get_attr(name).is_some()
    }
    fn get_attr(&self, name: &String) -> Option<String> {
        None
    }
    fn call_visit_function(&self, ast: &AST) -> Self::Return {
        None
    }
}
impl TypeCheckVisitor {
    pub fn get_rhs(&self, rhs: Expression, expected_type: &AnnotatedTypeName) -> Expression {
        if is_instance(&rhs, ASTType::TupleExpr) {
            if !is_instance(&rhs, ASTType::TupleExpr)
                || !is_instance(&*expected_type.type_name, ASTType::TupleType)
                || rhs.elements().len() != expected_type.type_name.types().unwrap().len()
            {
                assert!(
                    false,
                    "{:?},{:?},{:?}",
                    expected_type,
                    rhs.annotated_type(),
                    rhs
                )
            }
            let exprs = expected_type
                .type_name
                .types()
                .unwrap()
                .iter()
                .zip(rhs.elements())
                .map(|(e, a)| self.get_rhs(a, e))
                .collect();
            return replace_expr(&rhs, &mut TupleExpr::new(exprs).to_expr(), false)
                .as_type(AsTypeUnion::TypeName(TypeName::TupleType(TupleType::new(
                    exprs.iter().map(|e| e.annotated_type().clone()).collect(),
                ))))
                .to_expr();
        }

        let mut require_rehom = false;
        let instance = rhs.instance_of(expected_type);
        let trues = String::from("true");
        if trues != instance {
            require_rehom = true;
            let expected_matching_hom =
                expected_type.with_homomorphism(rhs.annotated_type().homomorphism);
            instance = rhs.instance_of(&expected_matching_hom);
        }

        if trues != instance {
            assert!(
                false,
                "{:?},{:?}, {:?}",
                expected_type,
                rhs.annotated_type(),
                rhs
            )
        } else {
            if rhs.annotated_type().type_name != expected_type.type_name {
                rhs = Self::implicitly_converted_to(rhs, &*expected_type.type_name);
            }

            if instance == String::from("make-private") {
                return Self::make_private(
                    rhs,
                    &*expected_type.privacy_annotation.unwrap(),
                    &expected_type.homomorphism,
                );
            } else if require_rehom {
                return Self::try_rehom(rhs, expected_type);
            } else {
                return rhs;
            }
        }
        Expression::None
    }
    //@staticmethod
    pub fn check_for_invalid_private_type(ast: AST) {
        if let Some(at) = &ast.annotated_type() {
            if at.is_private() && !at.type_name.can_be_private() {
                assert!(
                    false,
                    "Type {:?} cannot be private {:?}",
                    at.type_name,
                    ast.annotated_type()
                );
            }
        }
    }
    pub fn check_final(&self, fct: ConstructorOrFunctionDefinition, ast: Expression) {
        if is_instance(&ast, ASTType::IdentifierExpr) {
            if let Some(TargetDefinition::IdentifierDeclaration(
                IdentifierDeclaration::StateVariableDeclaration(target),
            )) = ast.target().map(|t| *t)
            {
                if target
                    .identifier_declaration_base
                    .keywords
                    .contains(&String::from("final"))
                {
                    //assignment allowed
                    // pass
                    assert!(
                        is_instance(&target, ASTType::StateVariableDeclaration)
                            && fct.is_constructor(),
                        r#"Modifying "final" variable{:?}"#,
                        ast
                    );
                }
            }
        } else {
            assert!(is_instance(&ast, ASTType::TupleExpr));
            for elem in ast.elements() {
                self.check_final(fct, elem);
            }
        }
    }

    pub fn visitAssignmentStatement(&self, ast: AssignmentStatement) {
        assert!(
            ast.lhs().is_some() && AssignmentStatementUnion::None != ast.lhs().unwrap(),
            "Assignment target is not a location {:?}",
            ast.lhs()
        );

        let expected_type = &ast.lhs().unwrap().annotated_type();
        ast.set_rhs(Some(
            self.get_rhs(ast.rhs().unwrap(), &expected_type.unwrap()),
        ));

        //prevent modifying final
        let f = *ast.function().unwrap();
        if let Some(AssignmentStatementUnion::TupleExpr(_))
        | Some(AssignmentStatementUnion::LocationExpr(LocationExpr::IdentifierExpr(_))) =
            ast.lhs()
        {
            self.check_final(f, ast.lhs().unwrap().to_expr());
        }
    }

    pub fn visitVariableDeclarationStatement(&self, ast: VariableDeclarationStatement) {
        if ast.expr.is_some() {
            ast.expr = Some(
                self.get_rhs(
                    ast.expr.unwrap(),
                    &*ast
                        .variable_declaration
                        .identifier_declaration_base
                        .annotated_type,
                ),
            );
        }
    }

    //@staticmethod
    pub fn has_private_type(ast: &Expression) -> bool {
        ast.annotated_type().is_private()
    }

    //@staticmethod
    pub fn has_literal_type(ast: Expression) -> bool {
        is_instances(
            &*ast.annotated_type().type_name,
            vec![ASTType::NumberLiteralType, ASTType::BooleanLiteralType],
        )
    }
    pub fn handle_builtin_function_call(&self, ast: FunctionCallExpr, func: BuiltinFunction) {
        if func.is_parenthesis() {
            ast.set_annotated_type(ast.args()[0].annotated_type());
            return;
        }

        let all_args_all_or_me = ast
            .args()
            .iter()
            .all(|x| x.annotated_type().is_accessible(&ast.analysis()));
        let is_public_ite = func.is_ite() && ast.args()[0].annotated_type().is_public();
        if all_args_all_or_me || is_public_ite {
            self.handle_unhom_builtin_function_call(ast, func);
        } else {
            self.handle_homomorphic_builtin_function_call(ast, func);
        }
    }

    pub fn handle_unhom_builtin_function_call(&self, ast: FunctionCallExpr, func: BuiltinFunction) {
        let mut args = ast.args();
        //handle special cases
        if func.is_ite() {
            let cond_t = &args[0].annotated_type();

            //Ensure that condition is boolean
            assert!(
                cond_t
                    .type_name
                    .implicitly_convertible_to(&TypeName::bool_type()),
                "{:?}, {:?}, {:?}",
                TypeName::bool_type(),
                cond_t.type_name,
                args[0]
            );

            let res_t = args[1]
                .annotated_type()
                .type_name
                .combined_type(*args[2].annotated_type().type_name, true);

            let a = if cond_t.is_private()
            //Everything is turned private
            {
                func.is_private = true;
                res_t.annotate(CombinedPrivacyUnion::AST(Some(
                    Expression::me_expr(None).get_ast(),
                )))
            } else {
                let hom = Self::combine_homomorphism(args[1], args[2]);
                let true_type = args[1].annotated_type().with_homomorphism(hom);
                let false_type = args[2].annotated_type().with_homomorphism(hom);
                let p = true_type
                    .combined_privacy(ast.analysis(), false_type)
                    .unwrap();
                res_t.annotate(p).with_homomorphism(hom)
            };
            args[1] = self.get_rhs(args[1], &a);
            args[2] = self.get_rhs(args[2], &a);
            ast.set_args(args);
            ast.set_annotated_type(a);
            return;
        }

        //Check that argument types conform to op signature
        let parameter_types = func.input_types();
        if !func.is_eq() {
            for (arg, t) in args.iter().zip(parameter_types) {
                if !arg.instanceof_data_type(t) {
                    assert!(
                        false,
                        "{:?},{:?}, {:?}",
                        t,
                        arg.annotated_type().type_name,
                        arg
                    );
                }
            }
        }

        let t1 = *args[0].annotated_type().type_name;
        let t2 = if args.len() == 1 {
            None
        } else {
            Some(*args[1].annotated_type().type_name.clone())
        };

        let arg_t = if args.len() == 1 {
            if args[0].annotated_type().type_name.is_literal() {
                TypeName::Literal(String::from("lit"))
            } else {
                t1
            }
        } else {
            assert!(args.len() == 2);
            let is_eq_with_tuples = func.is_eq() && is_instance(&t1, ASTType::TupleType);
            t1.combined_type(t2.unwrap(), is_eq_with_tuples)
        };
        //Infer argument and output types
        let out_t = if arg_t == TypeName::Literal(String::from("lit")) {
            let res = func.op_func(
                args.iter()
                    .map(|arg| arg.annotated_type().type_name.value())
                    .collect(),
            );
            let out_t = match res {
                LiteralUnion::Bool(value) => {
                    assert!(func.output_type() == TypeName::bool_type());
                    TypeName::ElementaryTypeName(ElementaryTypeName::BooleanLiteralType(
                        BooleanLiteralType::new(value),
                    ))
                }
                LiteralUnion::Number(value) => {
                    assert!(func.output_type() == TypeName::number_type());
                    TypeName::ElementaryTypeName(ElementaryTypeName::NumberTypeName(
                        NumberTypeName::NumberLiteralType(NumberLiteralType::new(
                            NumberLiteralTypeUnion::I32(value),
                        )),
                    ))
                }
                _ => TypeName::None,
            };
            if func.is_eq() {
                arg_t = t1
                    .to_abstract_type()
                    .combined_type(t2.unwrap().to_abstract_type(), true);
            }
            out_t
        } else if func.output_type() == TypeName::bool_type() {
            TypeName::bool_type()
        } else {
            arg_t
        };

        assert!(
            arg_t != TypeName::None
                && (arg_t != TypeName::Literal(String::from("lit")) || !func.is_eq())
        );
        let mut p = None;
        let private_args = args.iter().any(|arg| Self::has_private_type(arg));
        if private_args {
            assert!(arg_t != TypeName::Literal(String::from("lit")));
            if func.can_be_private() {
                if func.is_shiftop() {
                    if !args[1].annotated_type().type_name.is_literal() {
                        assert!(
                            false,
                            "Private shift expressions must use a constant (literal) shift amount {:?}",
                            args[1]
                        )
                    }
                    if args[1].annotated_type().type_name.value() < 0 {
                        assert!(false, "Cannot shift by negative amount {:?}", args[1]);
                    }
                }
                if func.is_bitop() || func.is_shiftop() {
                    for arg in &args {
                        if arg.annotated_type().type_name.elem_bitwidth() == 256 {
                            assert!(false,"Private bitwise and shift operations are only supported for integer types < 256 bit, please use a smaller type {:?}", arg)
                        }
                    }
                }

                if func.is_arithmetic() {
                    for a in &args {
                        if a.annotated_type().type_name.elem_bitwidth() == 256 {
                            issue_compiler_warning(
                                func.get_ast(),
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
                        if a.annotated_type().type_name.elem_bitwidth() == 256 {
                            issue_compiler_warning(
                                func.get_ast(),
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

        if arg_t != TypeName::Literal(String::from("lit")) {
            //Add implicit casts for arguments
            let arg_pt = arg_t.annotate(CombinedPrivacyUnion::AST(Some(p.unwrap().get_ast())));
            if func.is_shiftop() && p.is_some() {
                args[0] = self.get_rhs(args[0], &arg_pt);
            } else {
                args = ast
                    .args()
                    .iter()
                    .map(|argument| self.get_rhs(argument.clone(), &arg_pt))
                    .collect();
            }
            ast.set_args(args);
        }

        ast.set_annotated_type(
            out_t.annotate(CombinedPrivacyUnion::AST(Some(p.unwrap().get_ast()))),
        );
    }
    pub fn handle_homomorphic_builtin_function_call(
        self,
        ast: FunctionCallExpr,
        func: BuiltinFunction,
    ) {
        //First - same as non-homomorphic - check that argument types conform to op signature
        if !func.is_eq() {
            for (arg, t) in ast.args().iter().zip(func.input_types()) {
                if !arg.instanceof_data_type(t) {
                    assert!(
                        false,
                        "{:?},{:?}, {:?}",
                        t,
                        arg.annotated_type().type_name,
                        arg
                    )
                }
            }
        }

        let homomorphic_func = func.select_homomorphic_overload(ast.args(), ast.analysis());
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

        ast.set_annotated_type(homomorphic_func.unwrap().output_type());
        func.homomorphism = ast.annotated_type().unwrap().homomorphism;
        let expected_arg_types = homomorphic_func.unwrap().input_types();

        //Check that the argument types are correct
        ast.set_args(
            ast.args()
                .iter()
                .zip(expected_arg_types)
                .map(|(arg, arg_pt)| self.get_rhs(arg.clone(), &arg_pt))
                .collect(),
        );
    }
    //@staticmethod
    pub fn is_accessible_by_invoker(ast: &Expression) -> bool {
        // return ast.annotated_type.is_public() || ast.is_lvalue() || \
        //     ast.instance_of(AnnotatedTypeName(ast.annotated_type.type_name, Expression::me_expr(None)))
        return true;
    }
    //@staticmethod
    pub fn combine_homomorphism(lhs: Expression, rhs: Expression) -> String {
        if lhs.annotated_type().homomorphism == rhs.annotated_type().homomorphism {
            lhs.annotated_type().homomorphism.clone()
        } else if Self::can_rehom(lhs) {
            rhs.annotated_type().homomorphism.clone()
        } else {
            lhs.annotated_type().homomorphism.clone()
        }
    }

    //@staticmethod
    pub fn can_rehom(ast: Expression) -> bool {
        if ast.annotated_type().is_accessible(&ast.analysis()) {
            return true;
        }
        if is_instance(&ast, ASTType::ReclassifyExpr) {
            return true;
        }
        if is_instance(&ast, ASTType::PrimitiveCastExpr) {
            return Self::can_rehom(ast.expr());
        }
        if is_instance(&ast, ASTType::FunctionCallExpr)
            && is_instance(&ast.func().unwrap(), ASTType::BuiltinFunction)
            && ast.func().unwrap().is_ite()
            && ast.args()[0].annotated_type().is_public()
        {
            return Self::can_rehom(ast.args()[1]) && Self::can_rehom(ast.args()[2]);
        }

        false
    }

    //@staticmethod
    pub fn try_rehom(rhs: Expression, expected_type: &AnnotatedTypeName) -> Expression {
        assert!(
            !rhs.annotated_type().is_public(),
            "Cannot change the homomorphism of a public value"
        );

        if rhs.annotated_type().is_private_at_me(&rhs.analysis()) {
            //The value is @me, so we can just insert a ReclassifyExpr to change
            //the homomorphism of this value, just like we do for public values.
            return Self::make_rehom(rhs, expected_type);
        }
        if is_instance(&rhs, ASTType::ReclassifyExpr) && !is_instance(&rhs, ASTType::RehomExpr) {
            //rhs is a valid ReclassifyExpr, i.e. the argument is public or @me-private
            //To create an expression with the correct homomorphism,
            //just change the ReclassifyExpr"s output homomorphism
            rhs.set_homomorphism(expected_type.homomorphism);
        } else if is_instance(&rhs, ASTType::PrimitiveCastExpr) {
            //Ignore primitive cast & recurse
            rhs.set_expr(Self::try_rehom(rhs.expr(), expected_type));
        } else if is_instance(&rhs, ASTType::FunctionCallExpr)
            && is_instance(&rhs.func().unwrap(), ASTType::BuiltinFunction)
            && rhs.func().unwrap().is_ite()
            && rhs.args()[0].annotated_type().is_public()
        {
            //Argument is public_cond ? true_val : false_val. Try to rehom both true_val and false_val
            let mut args = rhs.args();
            args[1] = Self::try_rehom(args[1].clone(), expected_type);
            args[2] = Self::try_rehom(args[2].clone(), expected_type);
            rhs.set_args(args);
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
        rhs.set_annotated_type(
            rhs.annotated_type()
                .with_homomorphism(expected_type.homomorphism),
        );
        rhs
    }

    //@staticmethod
    pub fn make_rehom(expr: Expression, expected_type: &AnnotatedTypeName) -> Expression {
        assert!(expected_type
            .privacy_annotation
            .unwrap()
            .privacy_annotation_label()
            .is_some());
        assert!(expr.annotated_type().is_private_at_me(&expr.analysis()));
        assert!(expected_type.is_private_at_me(&expr.analysis()));

        let mut r = RehomExpr::new(expr, Some(expected_type.homomorphism));

        //set type
        let pl = get_privacy_expr_from_label(
            expected_type
                .privacy_annotation
                .unwrap()
                .privacy_annotation_label()
                .unwrap()
                .into(),
        );
        r.reclassify_expr_base.expression_base.annotated_type = Some(AnnotatedTypeName::new(
            *expr.annotated_type().type_name,
            Some(pl),
            expected_type.homomorphism,
        ));
        Self::check_for_invalid_private_type(r.get_ast());

        //set statement, parents, location
        Self::assign_location(&mut r.to_expr(), &mut expr);

        r.to_expr()
    }

    //@staticmethod
    pub fn make_private(
        expr: Expression,
        privacy: &Expression,
        homomorphism: &String,
    ) -> Expression {
        assert!(privacy.privacy_annotation_label().is_some());

        let pl = get_privacy_expr_from_label(privacy.privacy_annotation_label().unwrap().into());
        let mut r = ReclassifyExprBase::new(expr, pl, Some(homomorphism.clone()));

        //set type
        r.expression_base.annotated_type = Some(AnnotatedTypeName::new(
            *expr.annotated_type().type_name,
            Some(pl.clone()),
            homomorphism.clone(),
        ));
        Self::check_for_invalid_private_type(r.get_ast());
        let r = r.to_expr();
        //set statement, parents, location
        Self::assign_location(&mut r, &mut expr);

        r
    }

    //@staticmethod
    pub fn assign_location(target: &mut Expression, source: &mut Expression) {
        //set statement
        target.set_statement(*source.statement().unwrap());

        //set parents
        target.set_parent(source.parent().clone());
        let mut annotated_type = target.annotated_type();
        annotated_type.ast_base.parent = Some(Box::new((*target).get_ast()));
        target.set_annotated_type(annotated_type);
        source.set_parent(Some(Box::new(target.clone().get_ast())));

        //set source location
        target.set_line(source.line());
        target.set_column(source.column());
    }

    //@staticmethod
    pub fn implicitly_converted_to(expr: Expression, t: &TypeName) -> Expression {
        if is_instance(&expr, ASTType::ReclassifyExpr) && !expr.privacy().is_all_expr() {
            //Cast the argument of the ReclassifyExpr instead
            expr.set_expr(Self::implicitly_converted_to(expr.expr(), t));
            let mut expr_annotated_type = expr.annotated_type();
            expr_annotated_type.type_name = expr.expr().annotated_type().type_name;
            expr.set_annotated_type(expr_annotated_type);
            return expr;
        }

        assert!(expr.annotated_type().type_name.is_primitive_type());
        let mut cast = PrimitiveCastExpr::new(t.clone(), expr, true);
        cast.expression_base.ast_base.parent = expr.parent();
        cast.expression_base.statement = expr.statement();
        cast.expression_base.ast_base.line = expr.line();
        cast.expression_base.ast_base.column = expr.column();
        cast.elem_type.set_parent(Some(Box::new(cast.get_ast())));
        expr.set_parent(Some(Box::new(cast.get_ast())));
        cast.expression_base.annotated_type = Some(AnnotatedTypeName::new(
            t.clone(),
            expr.annotated_type().privacy_annotation.map(|p| *p),
            expr.annotated_type().homomorphism.clone(),
        ));
        cast.expression_base.annotated_type.unwrap().ast_base.parent =
            Some(Box::new(cast.get_ast()));
        Expression::PrimitiveCastExpr(cast)
    }

    pub fn visitFunctionCallExpr(&self, ast: FunctionCallExpr) {
        if is_instance(&ast.func().unwrap(), ASTType::BuiltinFunction) {
            self.handle_builtin_function_call(
                ast,
                if let Some(Expression::BuiltinFunction(bf)) = ast.func() {
                    bf
                } else {
                    BuiltinFunction::default()
                },
            );
        } else if ast.is_cast() {
            assert!(
                if let Some(TargetDefinition::NamespaceDefinition(
                    NamespaceDefinition::EnumDefinition(_),
                )) = ast.func().unwrap().target().map(|t| *t)
                {
                    true
                } else {
                    false
                },
                "User type casts only implemented for enums"
            );
            ast.set_annotated_type(
                self.handle_cast(
                    ast.args()[0],
                    *ast.func()
                        .unwrap()
                        .target()
                        .unwrap()
                        .annotated_type()
                        .type_name,
                ),
            );
        } else if is_instance(&ast.func().unwrap(), ASTType::LocationExpr) {
            let ft = ast.func().unwrap().annotated_type().type_name;
            assert!(is_instance(&*ft, ASTType::FunctionTypeName));

            if ft.parameters().len() != ast.args().len() {
                assert!(false, "Wrong number of arguments {:?}", ast.func())
            }

            //Check arguments
            let mut args = ast.args();
            for i in 0..ast.args().len() {
                args[i] = self.get_rhs(
                    args[i],
                    &*ft.parameters()[i]
                        .identifier_declaration_base
                        .annotated_type,
                );
            }
            ast.set_args(args);

            //Set expression type to return type
            ast.set_annotated_type(if ft.return_parameters().len() == 1 {
                *ft.return_parameters()[0]
                    .identifier_declaration_base
                    .annotated_type
                    .clone()
            } else {
                //TODO maybe not None label in the future
                AnnotatedTypeName::new(
                    TypeName::TupleType(TupleType::new(
                        ft.return_parameters()
                            .iter()
                            .map(|t| *t.identifier_declaration_base.annotated_type.clone())
                            .collect(),
                    )),
                    None,
                    String::from("NON_HOMOMORPHISM"),
                )
            });
        } else {
            assert!(false, "Invalid function call{:?}", ast);
        }
    }

    pub fn visitPrimitiveCastExpr(&self, ast: PrimitiveCastExpr) {
        ast.expression_base.annotated_type = Some(self.handle_cast(*ast.expr, *ast.elem_type));
    }

    pub fn handle_cast(&self, expr: Expression, t: TypeName) -> AnnotatedTypeName {
        //because of the fake solidity check we already know that the cast is possible -> don"t have to check if cast possible
        if expr.annotated_type().is_private() {
            let expected = AnnotatedTypeName::new(
                *expr.annotated_type().type_name,
                Some(Expression::me_expr(None)),
                String::from("NON_HOMOMORPHISM"),
            );
            if String::from("true") == expr.instance_of(&expected) {
                assert!(
                    false,
                    "{:?}, {:?}, {:?}",
                    expected,
                    expr.annotated_type(),
                    expr
                )
            }
            AnnotatedTypeName::new(
                t.clone(),
                Some(Expression::me_expr(None)),
                String::from("NON_HOMOMORPHISM"),
            )
        } else {
            AnnotatedTypeName::new(t.clone(), None, String::from("NON_HOMOMORPHISM"))
        }
    }

    pub fn visitNewExpr(&self, ast: NewExpr) { //already has correct type
                                               // pass
    }

    pub fn visitMemberAccessExpr(&self, ast: MemberAccessExpr) {
        assert!(ast.location_expr_base.target.is_some());
        if ast.expr.annotated_type().unwrap().is_address()
            && ast.expr.annotated_type().unwrap().is_private()
        {
            assert!(
                false,
                "Cannot access members of private address variable{:?}",
                ast
            );
        }
        ast.location_expr_base
            .tuple_or_location_expr_base
            .expression_base
            .annotated_type = Some(ast.location_expr_base.target.unwrap().annotated_type());
    }

    pub fn visitReclassifyExpr(&self, ast: ReclassifyExpr) {
        if ast.privacy().unwrap().privacy_annotation_label().is_some() {
            assert!(
                false,
                r#"Second argument of "reveal" cannot be used as a privacy type{:?}"#,
                ast
            );
        }

        let mut homomorphism = Homomorphism::non_homomorphic();
        assert!(!homomorphism.is_empty());

        //Prevent ReclassifyExpr to all with homomorphic type
        if ast.privacy().unwrap().is_all_expr()
            && (ast.homomorphism() != Some(Homomorphism::non_homomorphic())
                || ast.expr().unwrap().annotated_type().homomorphism
                    != Homomorphism::non_homomorphic())
        {
            //If the target privacy is all, we infer a target homomorphism of NonHomomorphic
            homomorphism = Homomorphism::non_homomorphic();
            ast.set_homomorphism(homomorphism.clone());
        }

        //Make sure the first argument to reveal / rehom is public or private provably equal to @me
        let is_expr_at_all = ast.expr().unwrap().annotated_type().is_public();
        let is_expr_at_me = ast
            .expr()
            .unwrap()
            .annotated_type()
            .is_private_at_me(&ast.analysis());
        if !is_expr_at_all && !is_expr_at_me {
            assert!(
                false,
                r#"First argument of "{}" must be accessible,"i.e. @all or provably equal to @me{:?}"#,
                ast.func_name(),
                ast
            );
        }

        //Prevent unhom(public_value)
        if is_expr_at_all
            && is_instance(&ast, ASTType::RehomExpr)
            && ast.homomorphism() == Some(Homomorphism::non_homomorphic())
        {
            assert!(
                false,
                r#"Cannot use "{}" on a public value{:?}"#,
                HOMOMORPHISM_STORE
                    .lock()
                    .unwrap()
                    .get(&ast.homomorphism().unwrap())
                    .unwrap()
                    .rehom_expr_name,
                ast
            );
        }

        //NB prevent any redundant reveal (not just for public)
        ast.set_annotated_type(AnnotatedTypeName::new(
            *ast.expr().unwrap().annotated_type().type_name,
            ast.privacy(),
            homomorphism,
        ));
        if String::from("true")
            == ast
                .to_expr()
                .instance_of(&ast.expr().unwrap().annotated_type())
        {
            assert!(
                false,
                r#"Redundant "{}": Expression is already @{}{homomorphism}"{:?}"#,
                ast.func_name(),
                ast.privacy().unwrap().code(),
                ast
            );
        }
        Self::check_for_invalid_private_type(ast.get_ast());
    }

    pub fn visitIfStatement(&self, ast: IfStatement) {
        let b = ast.condition;
        if !b.instanceof_data_type(TypeName::bool_type()) {
            assert!(
                false,
                "{:?}, {:?} ,{:?}",
                TypeName::bool_type(),
                b.annotated_type().type_name,
                b
            )
        }
        if ast.condition.annotated_type().is_private() {
            let expected = AnnotatedTypeName::new(
                TypeName::bool_type(),
                Some(Expression::me_expr(None)),
                String::from("NON_HOMOMORPHISM"),
            );
            if String::from("true") != b.instance_of(&expected) {
                assert!(false, "{:?}, {:?} ,{:?}", expected, b.annotated_type(), b)
            }
        }
    }

    pub fn visitWhileStatement(&self, ast: WhileStatement) {
        if String::from("true") != ast.condition.instance_of(&AnnotatedTypeName::bool_all()) {
            assert!(
                false,
                "{:?}, {:?} ,{:?}",
                AnnotatedTypeName::bool_all(),
                ast.condition.annotated_type(),
                ast.condition
            )
        }
        //must also later check that body and condition do not contain private expressions
    }

    pub fn visitForStatement(&self, ast: ForStatement) {
        if String::from("true") != ast.condition.instance_of(&AnnotatedTypeName::bool_all()) {
            assert!(
                false,
                "{:?}, {:?} ,{:?}",
                AnnotatedTypeName::bool_all(),
                ast.condition.annotated_type(),
                ast.condition
            )
        }
        //must also later check that body, update and condition do not contain private expressions
    }
    pub fn visitReturnStatement(&self, ast: ReturnStatement) {
        assert!(ast.statement_base.function.unwrap().is_function());
        let rt = AnnotatedTypeName::new(
            TypeName::TupleType((*ast.statement_base.function.unwrap()).return_type()),
            None,
            String::from("NON_HOMOMORPHISM"),
        );
        if ast.expr != Expression::None {
            self.get_rhs(TupleExpr::new(vec![]).to_expr(), &rt);
        } else if !is_instance(&ast.expr, ASTType::TupleExpr) {
            ast.expr = self.get_rhs(TupleExpr::new(vec![ast.expr.clone()]).to_expr(), &rt);
        } else {
            ast.expr = self.get_rhs(ast.expr, &rt);
        }
    }
    pub fn visitTupleExpr(&self, ast: TupleExpr) {
        ast.tuple_or_location_expr_base
            .expression_base
            .annotated_type = Some(AnnotatedTypeName::new(
            TypeName::TupleType(TupleType::new(
                ast.elements
                    .iter()
                    .map(|elem| elem.annotated_type())
                    .collect(),
            )),
            None,
            String::from("NON_HOMOMORPHISM"),
        ));
    }

    pub fn visitMeExpr(&self, ast: MeExpr) {
        ast.expression_base.annotated_type = Some(AnnotatedTypeName::address_all());
    }

    pub fn visitIdentifierExpr(&self, ast: IdentifierExpr) {
        // if is_instance(&ast.location_expr_base.target, ASTType::Mapping) { //no action necessary, the identifier will be replaced later
        // pass
        let target = ast.location_expr_base.target;
        let not_is_mapping = if let Some(TargetDefinition::None) = target.map(|t| *t) {
            false
        } else {
            true
        };
        if not_is_mapping {
            assert!(
                if let Some(TargetDefinition::NamespaceDefinition(
                    NamespaceDefinition::ContractDefinition(_),
                )) = target.map(|t| *t)
                {
                    false
                } else {
                    true
                },
                "Unsupported use of contract type in expression{:?}",
                ast
            );
            ast.annotated_type = Some(Box::new(target.unwrap().annotated_type()));

            if !Self::is_accessible_by_invoker(&ast.to_expr()) {
                assert!(false,"Tried to read value which cannot be proven to be owned by the transaction invoker{:?}", ast);
            }
        }
    }
    pub fn visitIndexExpr(&self, ast: IndexExpr) {
        let arr = ast.arr;
        let index = ast.key;
        let map_t = arr.annotated_type().unwrap();
        //should have already been checked
        assert!(map_t.privacy_annotation.unwrap().is_all_expr());

        //do actual type checking
        if let TypeName::Mapping(type_name) = *map_t.type_name {
            let key_type = type_name.key_type;
            let expected = AnnotatedTypeName::new(
                TypeName::ElementaryTypeName(key_type),
                Some(Expression::all_expr()),
                String::from("NON_HOMOMORPHISM"),
            );
            let instance = index.instance_of(&expected);
            if String::from("true") != instance {
                assert!(
                    false,
                    "{:?}, {:?} ,{:?}",
                    expected,
                    index.annotated_type(),
                    ast
                );
            }

            //record indexing information
            if type_name.key_label.is_some()
            //TODO modification correct?
            {
                if index.privacy_annotation_label().is_some() {
                    type_name.instantiated_key = Some(*index);
                } else {
                    assert!(
                        false,
                        "Index cannot be used as a privacy type for array of type {:?}{:?}",
                        map_t, ast
                    );
                }
            }
            //determine value type
            ast.location_expr_base
                .tuple_or_location_expr_base
                .expression_base
                .annotated_type = Some(*type_name.value_type);

            if !Self::is_accessible_by_invoker(&ast.to_expr()) {
                assert!(false,"Tried to read value which cannot be proven to be owned by the transaction invoker{:?}", ast);
            }
        } else if let TypeName::Array(type_name) = *map_t.type_name {
            if ast.key.annotated_type().is_private() {
                assert!(false, "No private array index{:?}", ast);
            }
            if !ast.key.instanceof_data_type(TypeName::number_type()) {
                assert!(false, "Array index must be numeric{:?}", ast);
            }
            ast.location_expr_base
                .tuple_or_location_expr_base
                .expression_base
                .annotated_type = Some(type_name.value_type());
        } else {
            assert!(false, "Indexing into non-mapping{:?}", ast);
        }
    }
    pub fn visitConstructorOrFunctionDefinition(&self, ast: ConstructorOrFunctionDefinition) {
        for t in ast.parameter_types().types {
            if !is_instances(
                &*t.privacy_annotation.unwrap(),
                vec![ASTType::MeExpr, ASTType::AllExpr],
            ) {
                assert!(
                    false,
                    "Only me/all accepted as privacy type of function parameters{:?}",
                    ast
                );
            }
        }

        if ast.can_be_external() {
            for t in ast.return_type().types {
                if !is_instances(
                    &*t.privacy_annotation.unwrap(),
                    vec![ASTType::MeExpr, ASTType::AllExpr],
                ) {
                    assert!(false,"Only me/all accepted as privacy type of return values for public functions{:?}", ast);
                }
            }
        }
    }
    pub fn visitEnumDefinition(&self, ast: EnumDefinition) {
        let mut etn = EnumTypeName::new(ast.qualified_name(), None);
        etn.user_defined_type_name_base.target = Some(Box::new(ast.get_ast()));
        ast.annotated_type = Some(AnnotatedTypeName::new(
            TypeName::UserDefinedTypeName(UserDefinedTypeName::EnumTypeName(etn)),
            None,
            String::from("NON_HOMOMORPHIM"),
        ));
    }

    pub fn visitEnumValue(&self, ast: EnumValue) {
        let mut evtn = EnumValueTypeName::new(ast.qualified_name(), None);
        evtn.user_defined_type_name_base.target = Some(Box::new(ast.get_ast()));
        ast.annotated_type = Some(AnnotatedTypeName::new(
            TypeName::UserDefinedTypeName(UserDefinedTypeName::EnumValueTypeName(evtn)),
            None,
            String::from("NON_HOMOMORPHISM"),
        ));
    }

    pub fn visitStateVariableDeclaration(&self, ast: StateVariableDeclaration) {
        if ast.expr.is_some() {
            //prevent private operations in declaration
            if contains_private(ast.get_ast()) {
                assert!(
                    false,
                    "Private assignments to state variables must be in the constructor{:?}",
                    ast
                );
            }

            //check type
            self.get_rhs(
                ast.expr.unwrap(),
                &*ast.identifier_declaration_base.annotated_type,
            );
        }

        //prevent "me" annotation
        let p = ast
            .identifier_declaration_base
            .annotated_type
            .privacy_annotation
            .unwrap();
        if p.is_me_expr() {
            assert!(false, "State variables cannot be annotated as me{:?}", ast);
        }
    }

    pub fn visitMapping(&self, ast: Mapping) {
        if ast.key_label.is_some() {
            if TypeName::ElementaryTypeName(ast.key_type) != TypeName::address_type() {
                assert!(false, "Only addresses can be annotated{:?}", ast);
            }
        }
    }

    pub fn visitRequireStatement(&self, ast: RequireStatement) {
        if !ast
            .condition
            .annotated_type()
            .privacy_annotation
            .unwrap()
            .is_all_expr()
        {
            assert!(false, "require needs public argument{:?}", ast);
        }
    }

    pub fn visitAnnotatedTypeName(&mut self, ast: AnnotatedTypeName) {
        if let TypeName::UserDefinedTypeName(udtn) = *ast.type_name {
            if let Some(NamespaceDefinition::EnumDefinition(ed)) = udtn.target() {
                udtn.set_type_name(*ed.annotated_type.unwrap().type_name.clone());
            } else {
                assert!(
                    false,
                    "Unsupported use of user-defined type {:?}",
                    ast.type_name
                )
            }
        }

        if ast.privacy_annotation != Some(Box::new(Expression::all_expr())) {
            if !ast.type_name.can_be_private() {
                assert!(
                    false,
                    "Currently, we do not support private {:?},{:?}",
                    ast.type_name, ast
                );
            }
            if ast.homomorphism != Homomorphism::non_homomorphic() {
                //only support uint8, uint16, uint24, uint32 homomorphic data types
                if !ast.type_name.is_numeric() {
                    assert!(
                        false,
                        "Homomorphic type not supported for {:?}: Only numeric types supported{:?}",
                        ast.type_name, ast
                    );
                } else if ast.type_name.signed() {
                    assert!(false,"Homomorphic type not supported for {:?}: Only unsigned types supported{:?}",ast.type_name, ast);
                } else if ast.type_name.elem_bitwidth() > 32 {
                    assert!(false,"Homomorphic type not supported for {:?}: Only up to 32-bit numeric types supported{:?}", ast.type_name,ast);
                }
            }
        }
        let p = *ast.privacy_annotation.unwrap();
        if is_instance(&p, ASTType::IdentifierExpr) {
            let t = p.target().unwrap();
            let not_is_mapping = if TargetDefinition::None == *t {
                false
            } else {
                true
            };
            if not_is_mapping {
                //no action necessary, this is the case: mapping(address!x => uint@x)
                // pass
                if !t.is_final() && !t.is_constant() {
                    assert!(
                        false,
                        r#"Privacy annotations must be "final" or "constant", if they are expressions {:?}"#,
                        p
                    );
                } else if t.annotated_type() != AnnotatedTypeName::address_all() {
                    assert!(
                        false,
                        r#"Privacy type is not a public address, but {:?},{:?}"#,
                        t.annotated_type(),
                        p
                    );
                }
            }
        }
    }
}
