use crate::type_check::contains_private::contains_private;
use crate::type_check::final_checker::check_final;
// use crate::type_check::type_exceptions::{TypeMismatchException, TypeException};
use crate::zkay_ast::ast::{
    get_privacy_expr_from_label, issue_compiler_warning, AllExpr, AnnotatedTypeName, Array,
    AssignmentStatement, BooleanLiteralType, BuiltinFunction, ConstructorOrFunctionDefinition,
    ContractDefinition, EnumDefinition, EnumTypeName, EnumValue, EnumValueTypeName, Expression,
    ForStatement, FunctionCallExpr, FunctionTypeName, IdentifierExpr, IfStatement, IndexExpr,
    LocationExpr, Mapping, MeExpr, MemberAccessExpr, NewExpr, NumberLiteralType, PrimitiveCastExpr,
    ReclassifyExpr, RehomExpr, RequireStatement, ReturnStatement, StateVariableDeclaration,
    TupleExpr, TupleType, TypeName, UserDefinedTypeName, VariableDeclarationStatement,
    WhileStatement,
};
use crate::zkay_ast::homomorphism::Homomorphism;
use crate::zkay_ast::visitor::deep_copy::replace_expr;
use crate::zkay_ast::visitor::visitor::AstVisitor;

pub fn type_check(ast: AST) {
    check_final(ast);
    let v = TypeCheckVisitor::new();
    v.visit(ast);
}

// class TypeCheckVisitor(AstVisitor)
pub struct TypeCheckVisitor;
impl TypeCheckVisitor {
    pub fn get_rhs(self, rhs: Expression, expected_type: AnnotatedTypeName) {
        if isinstance(rhs, TupleExpr) {
            if !isinstance(rhs, TupleExpr)
                || !isinstance(expected_type.type_name, TupleType)
                || len(rhs.elements) != len(expected_type.type_name.types)
            {
                assert!(
                    false,
                    "{:?},{:?},{:?}",
                    expected_type, rhs.annotated_type, rhs
                )
            }
            exprs = expected_type
                .type_name
                .types
                .iter()
                .zip(rhs.elements)
                .map(|(e, a)| self.get_rhs(a, e))
                .collect();
            return replace_expr(rhs, TupleExpr(exprs)).as_type(TupleType(
                exprs.iter().map(|e| e.annotated_type.clone()).collect(),
            ));
        }

        require_rehom = False;
        instance = rhs.instanceof(expected_type);

        if !instance {
            require_rehom = True;
            expected_matching_hom =
                expected_type.with_homomorphism(rhs.annotated_type.homomorphism);
            instance = rhs.instanceof(expected_matching_hom);
        }

        if !instance {
            assert!(
                false,
                "{:?},{:?}, {:?}",
                expected_type, rhs.annotated_type, rhs
            )
        } else {
            if rhs.annotated_type.type_name != expected_type.type_name {
                rhs = self.implicitly_converted_to(rhs, expected_type.type_name);
            }

            if instance == "make-private" {
                return self.make_private(
                    rhs,
                    expected_type.privacy_annotation,
                    expected_type.homomorphism,
                );
            } else if require_rehom {
                return self.try_rehom(rhs, expected_type);
            } else {
                return rhs;
            }
        }
    }
    //@staticmethod
    pub fn check_for_invalid_private_type(ast: AST) {
        assert!(hasattr(ast, "annotated_type"));
        at = ast.annotated_type;
        if at.is_private() && !at.type_name.can_be_private() {
            assert!(
                false,
                "Type {:?} cannot be private {:?}",
                at.type_name, ast.annotated_type
            )
        }
    }
    pub fn check_final(self, fct: ConstructorOrFunctionDefinition, ast: Expression) {
        if isinstance(ast, IdentifierExpr) {
            target = ast.target;
            if hasattr(target, "keywords") {
                if target.keywords.contains("final") {
                    if isinstance(target, StateVariableDeclaration) && fct.is_constructor { //assignment allowed
                         // pass
                    } else {
                        assert!(false, r#"Modifying "final" variable{:?}"#, ast);
                    }
                }
            }
        } else {
            assert!(isinstance(ast, TupleExpr));
            for elem in ast.elements {
                self.check_final(fct, elem);
            }
        }
    }

    pub fn visitAssignmentStatement(self, ast: AssignmentStatement) {
        if !isinstance(ast.lhs, (TupleExpr, LocationExpr)) {
            assert!(false, "Assignment target is not a location {:?}", ast.lhs);
        }

        expected_type = ast.lhs.annotated_type;
        ast.rhs = self.get_rhs(ast.rhs, expected_type);

        //prevent modifying final
        f = ast.function;
        if isinstance(ast.lhs, (IdentifierExpr, TupleExpr)) {
            self.check_final(f, ast.lhs);
        }
    }

    pub fn visitVariableDeclarationStatement(self, ast: VariableDeclarationStatement) {
        if ast.expr {
            ast.expr = self.get_rhs(ast.expr, ast.variable_declaration.annotated_type);
        }
    }

    //@staticmethod
    pub fn has_private_type(ast: Expression) {
        return ast.annotated_type.is_private();
    }

    //@staticmethod
    pub fn has_literal_type(ast: Expression) {
        return isinstance(
            ast.annotated_type.type_name,
            (NumberLiteralType, BooleanLiteralType),
        );
    }
    pub fn handle_builtin_function_call(self, ast: FunctionCallExpr, func: BuiltinFunction) {
        if func.is_parenthesis() {
            ast.annotated_type = ast.args[0].annotated_type;
            return;
        }

        let all_args_all_or_me = ast
            .args
            .all(|x| x.annotated_type.is_accessible(ast.analysis));
        let is_public_ite = func.is_ite() && ast.args[0].annotated_type.is_public();
        if all_args_all_or_me || is_public_ite {
            self.handle_unhom_builtin_function_call(ast, func);
        } else {
            self.handle_homomorphic_builtin_function_call(ast, func);
        }
    }

    pub fn handle_unhom_builtin_function_call(self, ast: FunctionCallExpr, func: BuiltinFunction) {
        //handle special cases
        if func.is_ite() {
            cond_t = ast.args[0].annotated_type;

            //Ensure that condition is boolean
            if !cond_t
                .type_name
                .implicitly_convertible_to(TypeName.bool_type())
            {
                assert!(
                    false,
                    "{:?}, {:?}, {:?}",
                    TypeName.bool_type(),
                    cond_t.type_name,
                    ast.args[0]
                )
            }

            res_t = ast.args[1]
                .annotated_type
                .type_name
                .combined_type(ast.args[2].annotated_type.type_name, True);

            if cond_t.is_private()
            //Everything is turned private
            {
                func.is_private = True;
                a = res_t.annotate(Expression.me_expr());
            } else {
                hom = self.combine_homomorphism(ast.args[1], ast.args[2]);
                true_type = ast.args[1].annotated_type.with_homomorphism(hom);
                false_type = ast.args[2].annotated_type.with_homomorphism(hom);
                p = true_type.combined_privacy(ast.analysis, false_type);
                a = res_t.annotate(p).with_homomorphism(hom);
            }
            ast.args[1] = self.get_rhs(ast.args[1], a);
            ast.args[2] = self.get_rhs(ast.args[2], a);

            ast.annotated_type = a;
            return;
        }

        //Check that argument types conform to op signature
        parameter_types = func.input_types();
        if !func.is_eq() {
            for (arg, t) in ast.args.iter().zip(parameter_types) {
                if !arg.instanceof_data_type(t) {
                    assert!(
                        false,
                        "{:?},{:?}, {:?}",
                        t, arg.annotated_type.type_name, arg
                    )
                }
            }
        }

        t1 = ast.args[0].annotated_type.type_name;
        t2 = if len(ast.args) == 1 {
            None
        } else {
            ast.args[1].annotated_type.type_name
        };

        arg_t = if len(ast.args) == 1 {
            if ast.args[0].annotated_type.type_name.is_literal {
                "lit"
            } else {
                t1
            }
        } else {
            assert!(len(ast.args) == 2);
            let is_eq_with_tuples = func.is_eq() && isinstance(t1, TupleType);
            t1.combined_type(t2, is_eq_with_tuples)
        };
        //Infer argument and output types
        if arg_t == "lit" {
            res = func.op_func(
                ast.args
                    .iter()
                    .map(|arg| arg.annotated_type.type_name.value)
                    .collect(),
            );
            if isinstance(res, bool) {
                assert!(func.output_type() == TypeName.bool_type());
                out_t = BooleanLiteralType(res);
            } else {
                assert!(func.output_type() == TypeName.number_type());
                out_t = NumberLiteralType(res);
            }
            if func.is_eq() {
                arg_t = t1
                    .to_abstract_type()
                    .combined_type(t2.to_abstract_type(), True);
            }
        } else if func.output_type() == TypeName.bool_type() {
            out_t = TypeName.bool_type();
        } else {
            out_t = arg_t
        }

        assert!(arg_t.is_some() && (arg_t != "lit" || !func.is_eq()));

        private_args = any(map(self.has_private_type, ast.args));
        if private_args {
            assert!(arg_t != "lit");
            if func.can_be_private() {
                if func.is_shiftop() {
                    if !ast.args[1].annotated_type.type_name.is_literal {
                        assert!(
                            false,
                            "Private shift expressions must use a constant (literal) shift amount {:?}",
                            ast.args[1]
                        )
                    }
                    if ast.args[1].annotated_type.type_name.value < 0 {
                        assert!(false, "Cannot shift by negative amount {:?}", ast.args[1]);
                    }
                }
                if func.is_bitop() || func.is_shiftop() {
                    for arg in ast.args {
                        if arg.annotated_type.type_name.elem_bitwidth == 256 {
                            assert!(false,"Private bitwise and shift operations are only supported for integer types < 256 bit, please use a smaller type {:?}", arg)
                        }
                    }
                }

                if func.is_arithmetic() {
                    for a in ast.args {
                        if a.annotated_type.type_name.elem_bitwidth == 256 {
                            issue_compiler_warning(func, "Possible field prime overflow",
                                                         "Private arithmetic 256bit operations overflow at FIELD_PRIME.\n",
                                                         "If you need correct overflow behavior, use a smaller integer type.");
                            break;
                        }
                    }
                } else if func.is_comp() {
                    for a in ast.args {
                        if a.annotated_type.type_name.elem_bitwidth == 256 {
                            issue_compiler_warning(
                                func,
                                r#"Possible private comparison failure",
                                                         "Private 256bit comparison operations will fail for values >= 2^252.\n",
                                                         "If you cannot guarantee that the value stays in range, you must use ",
                                                         "a smaller integer type to ensure correctness."#,
                            );
                            break;
                        }
                    }
                }

                func.is_private = True;
                p = Expression.me_expr();
            } else {
                assert!(
                    false,
                    r#"Operation \"{}\" does not support private operands{:?}"#,
                    func.op, ast
                );
            }
        } else {
            p = None;
        }

        if arg_t != "lit" {
            //Add implicit casts for arguments
            arg_pt = arg_t.annotate(p);
            if func.is_shiftop() && p.is_some() {
                ast.args[0] = self.get_rhs(ast.args[0], arg_pt);
            } else {
                ast.args = ast
                    .args
                    .iter()
                    .map(|argument| self.get_rhs(argument, arg_pt))
                    .collect();
            }
        }

        ast.annotated_type = out_t.annotate(p);
    }
    pub fn handle_homomorphic_builtin_function_call(
        self,
        ast: FunctionCallExpr,
        func: BuiltinFunction,
    ) {
        //First - same as non-homomorphic - check that argument types conform to op signature
        if !func.is_eq() {
            for (arg, t) in ast.args.iter().zip(func.input_types()) {
                if !arg.instanceof_data_type(t) {
                    assert!(
                        false,
                        "{:?},{:?}, {:?}",
                        t, arg.annotated_type.type_name, arg
                    )
                }
            }
        }

        homomorphic_func = func.select_homomorphic_overload(ast.args, ast.analysis);
        if homomorphic_func.is_none() {
            assert!(
                false,
                r#"Operation \"{}\" requires all arguments to be accessible, i.e. @all or provably equal to @me{:?}"#,
                func.op, ast
            );
        }

        //We could perform homomorphic operations on-chain by using some Solidity arbitrary precision math library.
        //For now, keep it simple and evaluate homomorphic operations in Python and check the result in the circuit.
        func.is_private = True;

        ast.annotated_type = homomorphic_func.output_type();
        func.homomorphism = ast.annotated_type.homomorphism;
        expected_arg_types = homomorphic_func.input_types();

        //Check that the argument types are correct
        ast.args = ast
            .args
            .iter()
            .zip(expected_arg_types)
            .map(|(arg, arg_pt)| self.get_rhs(arg, arg_pt))
            .collect();
    }
    //@staticmethod
    pub fn is_accessible_by_invoker(ast: Expression) {
        return True;
        // return ast.annotated_type.is_public() || ast.is_lvalue() || \
        //     ast.instanceof(AnnotatedTypeName(ast.annotated_type.type_name, Expression.me_expr()))
    }
    //@staticmethod
    pub fn combine_homomorphism(lhs: Expression, rhs: Expression) -> Homomorphism {
        if lhs.annotated_type.homomorphism == rhs.annotated_type.homomorphism {
            return lhs.annotated_type.homomorphism;
        } else if TypeCheckVisitor.can_rehom(lhs) {
            return rhs.annotated_type.homomorphism;
        } else {
            return lhs.annotated_type.homomorphism;
        }
    }

    //@staticmethod
    pub fn can_rehom(ast: Expression) -> bool {
        if ast.annotated_type.is_accessible(ast.analysis) {
            return True;
        }
        if isinstance(ast, ReclassifyExpr) {
            return True;
        }
        if isinstance(ast, PrimitiveCastExpr) {
            return TypeCheckVisitor.can_rehom(ast.expr);
        }
        if isinstance(ast, FunctionCallExpr)
            && isinstance(ast.func, BuiltinFunction)
            && ast.func.is_ite()
            && ast.args[0].annotated_type.is_public()
        {
            return TypeCheckVisitor.can_rehom(ast.args[1])
                && TypeCheckVisitor.can_rehom(ast.args[2]);
        }

        return False;
    }

    //@staticmethod
    pub fn try_rehom(rhs: Expression, expected_type: AnnotatedTypeName) {
        if rhs.annotated_type.is_public() {
            assert!(false, "Cannot change the homomorphism of a public value")
        }

        if rhs.annotated_type.is_private_at_me(rhs.analysis) {
            //The value is @me, so we can just insert a ReclassifyExpr to change
            //the homomorphism of this value, just like we do for public values.
            return TypeCheckVisitor.make_rehom(rhs, expected_type);
        }

        if isinstance(rhs, ReclassifyExpr) && !isinstance(rhs, RehomExpr) {
            //rhs is a valid ReclassifyExpr, i.e. the argument is public or @me-private
            //To create an expression with the correct homomorphism,
            //just change the ReclassifyExpr"s output homomorphism
            rhs.homomorphism = expected_type.homomorphism
        } else if isinstance(rhs, PrimitiveCastExpr) {
            //Ignore primitive cast & recurse
            rhs.expr = TypeCheckVisitor.try_rehom(rhs.expr, expected_type);
        } else if isinstance(rhs, FunctionCallExpr)
            && isinstance(rhs.func, BuiltinFunction)
            && rhs.func.is_ite()
            && rhs.args[0].annotated_type.is_public()
        {
            //Argument is public_cond ? true_val : false_val. Try to rehom both true_val and false_val
            rhs.args[1] = TypeCheckVisitor.try_rehom(rhs.args[1], expected_type);
            rhs.args[2] = TypeCheckVisitor.try_rehom(rhs.args[2], expected_type);
        } else {
            assert!(
                false,
                "{:?}, {:?} ,{:?}",
                expected_type, rhs.annotated_type, rhs
            )
        }

        //Rehom worked without throwing, change annotated_type and return
        rhs.annotated_type = rhs
            .annotated_type
            .with_homomorphism(expected_type.homomorphism);
        return rhs;
    }

    //@staticmethod
    pub fn make_rehom(expr: Expression, expected_type: AnnotatedTypeName) {
        assert!(expected_type
            .privacy_annotation
            .privacy_annotation_label()
            .is_some());
        assert!(expr.annotated_type.is_private_at_me(expr.analysis));
        assert!(expected_type.is_private_at_me(expr.analysis));

        r = RehomExpr(expr, expected_type.homomorphism);

        //set type
        pl = get_privacy_expr_from_label(
            expected_type.privacy_annotation.privacy_annotation_label(),
        );
        r.annotated_type = AnnotatedTypeName(
            expr.annotated_type.type_name,
            pl,
            expected_type.homomorphism,
        );
        TypeCheckVisitor.check_for_invalid_private_type(r);

        //set statement, parents, location
        TypeCheckVisitor.assign_location(r, expr);

        return r;
    }

    //@staticmethod
    pub fn make_private(expr: Expression, privacy: Expression, homomorphism: Homomorphism) {
        assert!(privacy.privacy_annotation_label().is_some());

        pl = get_privacy_expr_from_label(privacy.privacy_annotation_label());
        r = ReclassifyExpr(expr, pl, homomorphism);

        //set type
        r.annotated_type =
            AnnotatedTypeName(expr.annotated_type.type_name, pl.clone(), homomorphism);
        TypeCheckVisitor.check_for_invalid_private_type(r);

        //set statement, parents, location
        TypeCheckVisitor.assign_location(r, expr);

        return r;
    }

    //@staticmethod
    pub fn assign_location(target: Expression, source: Expression) {
        //set statement
        target.statement = source.statement;

        //set parents
        target.parent = source.parent;
        target.annotated_type.parent = target;
        source.parent = target;

        //set source location
        target.line = source.line;
        target.column = source.column;
    }

    //@staticmethod
    pub fn implicitly_converted_to(expr: Expression, t: TypeName) -> Expression {
        if isinstance(expr, ReclassifyExpr) && !expr.privacy.is_all_expr() {
            //Cast the argument of the ReclassifyExpr instead
            expr.expr = TypeCheckVisitor.implicitly_converted_to(expr.expr, t);
            expr.annotated_type.type_name = expr.expr.annotated_type.type_name;
            return expr;
        }

        assert!(expr.annotated_type.type_name.is_primitive_type());
        cast = PrimitiveCastExpr(t.clone(), expr, is_implicit = True).r#override(
            parent = expr.parent,
            statement = expr.statement,
            line = expr.line,
            column = expr.column,
        );
        cast.elem_type.parent = cast;
        expr.parent = cast;
        cast.annotated_type = AnnotatedTypeName(
            t.clone(),
            expr.annotated_type.privacy_annotation.clone(),
            expr.annotated_type.homomorphism,
        )
        .r#override(parent = cast);
        return cast;
    }

    pub fn visitFunctionCallExpr(self, ast: FunctionCallExpr) {
        if isinstance(ast.func, BuiltinFunction) {
            self.handle_builtin_function_call(ast, ast.func);
        } else if ast.is_cast {
            if !isinstance(ast.func.target, EnumDefinition) {
                assert!(false, "User type casts only implemented for enums");
            }
            ast.annotated_type =
                self.handle_cast(ast.args[0], ast.func.target.annotated_type.type_name);
        } else if isinstance(ast.func, LocationExpr) {
            ft = ast.func.annotated_type.type_name;
            assert!(isinstance(ft, FunctionTypeName));

            if len(ft.parameters) != len(ast.args) {
                assert!(false, "Wrong number of arguments {:?}", ast.func)
            }

            //Check arguments
            for i in range(len(ast.args)) {
                ast.args[i] = self.get_rhs(ast.args[i], ft.parameters[i].annotated_type);
            }

            //Set expression type to return type
            if len(ft.return_parameters) == 1 {
                ast.annotated_type = ft.return_parameters[0].annotated_type.clone();
            } else {
                //TODO maybe not None label in the future
                ast.annotated_type = AnnotatedTypeName(
                    TupleType(
                        ft.return_parameters
                            .iter()
                            .map(|t| t.annotated_type.clone())
                            .collect(),
                    ),
                    None,
                );
            }
        } else {
            assert!(false, "Invalid function call{:?}", ast);
        }
    }

    pub fn visitPrimitiveCastExpr(self, ast: PrimitiveCastExpr) {
        ast.annotated_type = self.handle_cast(ast.expr, ast.elem_type);
    }

    pub fn handle_cast(self, expr: Expression, t: TypeName) -> AnnotatedTypeName {
        //because of the fake solidity check we already know that the cast is possible -> don"t have to check if cast possible
        if expr.annotated_type.is_private() {
            expected = AnnotatedTypeName(expr.annotated_type.type_name, Expression.me_expr());
            if !expr.instanceof(expected) {
                assert!(
                    false,
                    "{:?}, {:?}, {:?}",
                    expected, expr.annotated_type, expr
                )
            }
            return AnnotatedTypeName(t.clone(), Expression.me_expr());
        } else {
            return AnnotatedTypeName(t.clone());
        }
    }

    pub fn visitNewExpr(self, ast: NewExpr) { //already has correct type
                                              // pass
    }

    pub fn visitMemberAccessExpr(self, ast: MemberAccessExpr) {
        assert!(ast.target.is_some());
        if ast.expr.annotated_type.is_address() && ast.expr.annotated_type.is_private() {
            assert!(
                false,
                "Cannot access members of private address variable{:?}",
                ast
            );
        }
        ast.annotated_type = ast.target.annotated_type.clone();
    }

    pub fn visitReclassifyExpr(self, ast: ReclassifyExpr) {
        if !ast.privacy.privacy_annotation_label() {
            assert!(
                false,
                r#"Second argument of "reveal" cannot be used as a privacy type{:?}"#,
                ast
            );
        }

        homomorphism = ast.homomorphism || ast.expr.annotated_type.homomorphism;
        assert(homomorphism.is_some());

        //Prevent ReclassifyExpr to all with homomorphic type
        if ast.privacy.is_all_expr() && homomorphism != Homomorphism.NonHomomorphic {
            //If the target privacy is all, we infer a target homomorphism of NonHomomorphic
            ast.homomorphism = homomorphism = Homomorphism.NonHomomorphic;
        }

        //Make sure the first argument to reveal / rehom is public or private provably equal to @me
        is_expr_at_all = ast.expr.annotated_type.is_public();
        is_expr_at_me = ast.expr.annotated_type.is_private_at_me(ast.analysis);
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
            && isinstance(ast, RehomExpr)
            && ast.homomorphism == Homomorphism.NonHomomorphic
        {
            assert!(
                false,
                r#"Cannot use "{}" on a public value{:?}"#,
                ast.homomorphism.rehom_expr_name, ast
            );
        }

        //NB prevent any redundant reveal (not just for public)
        ast.annotated_type =
            AnnotatedTypeName(ast.expr.annotated_type.type_name, ast.privacy, homomorphism);
        if ast.instanceof(ast.expr.annotated_type) {
            assert!(
                false,
                r#"Redundant "{}": Expression is already @{}{homomorphism}"{:?}"#,
                ast.func_name(),
                ast.privacy.code(),
                ast
            );
        }
        self.check_for_invalid_private_type(ast);
    }

    pub fn visitIfStatement(self, ast: IfStatement) {
        b = ast.condition;
        if !b.instanceof_data_type(TypeName.bool_type()) {
            assert!(
                false,
                "{:?}, {:?} ,{:?}",
                TypeName.bool_type(),
                b.annotated_type.type_name,
                b
            )
        }
        if ast.condition.annotated_type.is_private() {
            expected = AnnotatedTypeName(TypeName.bool_type(), Expression.me_expr());
            if !b.instanceof(expected) {
                assert!(false, "{:?}, {:?} ,{:?}", expected, b.annotated_type, b)
            }
        }
    }

    pub fn visitWhileStatement(self, ast: WhileStatement) {
        if !ast.condition.instanceof(AnnotatedTypeName.bool_all()) {
            assert!(
                false,
                "{:?}, {:?} ,{:?}",
                AnnotatedTypeName.bool_all(),
                ast.condition.annotated_type,
                ast.condition
            )
        }
        //must also later check that body and condition do not contain private expressions
    }

    pub fn visitForStatement(self, ast: ForStatement) {
        if !ast.condition.instanceof(AnnotatedTypeName.bool_all()) {
            assert!(
                false,
                "{:?}, {:?} ,{:?}",
                AnnotatedTypeName.bool_all(),
                ast.condition.annotated_type,
                ast.condition
            )
        }
        //must also later check that body, update and condition do not contain private expressions
    }
    pub fn visitReturnStatement(self, ast: ReturnStatement) {
        assert!(ast.function.is_function);
        rt = AnnotatedTypeName(ast.function.return_type);
        if ast.expr.is_none() {
            self.get_rhs(TupleExpr([]), rt);
        } else if !isinstance(ast.expr, TupleExpr) {
            ast.expr = self.get_rhs(TupleExpr([ast.expr]), rt);
        } else {
            ast.expr = self.get_rhs(ast.expr, rt);
        }
    }
    pub fn visitTupleExpr(self, ast: TupleExpr) {
        ast.annotated_type = AnnotatedTypeName(TupleType(
            ast.elements
                .iter()
                .map(|elem| elem.annotated_type.clone())
                .collect(),
        ));
    }

    pub fn visitMeExpr(self, ast: MeExpr) {
        ast.annotated_type = AnnotatedTypeName.address_all();
    }

    pub fn visitIdentifierExpr(self, ast: IdentifierExpr) {
        if isinstance(ast.target, Mapping) { //no action necessary, the identifier will be replaced later
             // pass
        } else {
            target = ast.target;
            if isinstance(target, ContractDefinition) {
                assert!(
                    false,
                    "Unsupported use of contract type in expression{:?}",
                    ast
                );
            }
            ast.annotated_type = target.annotated_type.clone();

            if !self.is_accessible_by_invoker(ast) {
                assert!(false,"Tried to read value which cannot be proven to be owned by the transaction invoker{:?}", ast);
            }
        }
    }
    pub fn visitIndexExpr(self, ast: IndexExpr) {
        arr = ast.arr;
        index = ast.key;
        map_t = arr.annotated_type;
        //should have already been checked
        assert!(map_t.privacy_annotation.is_all_expr());

        //do actual type checking
        if isinstance(map_t.type_name, Mapping) {
            key_type = map_t.type_name.key_type;
            expected = AnnotatedTypeName(key_type, Expression.all_expr());
            instance = index.instanceof(expected);
            if !instance {
                assert!(
                    false,
                    "{:?}, {:?} ,{:?}",
                    expected, index.annotated_type, ast
                )
            }

            //record indexing information
            if map_t.type_name.key_label.is_some()
            //TODO modification correct?
            {
                if index.privacy_annotation_label() {
                    map_t.type_name.instantiated_key = index;
                } else {
                    assert!(
                        false,
                        "Index cannot be used as a privacy type for array of type {map_t}{:?}",
                        ast
                    );
                }
            }
            //determine value type
            ast.annotated_type = map_t.type_name.value_type;

            if !self.is_accessible_by_invoker(ast) {
                assert!(false,"Tried to read value which cannot be proven to be owned by the transaction invoker{:?}", ast);
            }
        } else if isinstance(map_t.type_name, Array) {
            if ast.key.annotated_type.is_private() {
                assert!(false, "No private array index{:?}", ast);
            }
            if !ast.key.instanceof_data_type(TypeName.number_type()) {
                assert!(false, "Array index must be numeric{:?}", ast);
            }
            ast.annotated_type = map_t.type_name.value_type;
        } else {
            assert!(false, "Indexing into non-mapping{:?}", ast);
        }
    }
    pub fn visitConstructorOrFunctionDefinition(self, ast: ConstructorOrFunctionDefinition) {
        for t in ast.parameter_types {
            if !isinstance(t.privacy_annotation, (MeExpr, AllExpr)) {
                assert!(
                    false,
                    "Only me/all accepted as privacy type of function parameters{:?}",
                    ast
                );
            }
        }

        if ast.can_be_external {
            for t in ast.return_type {
                if !isinstance(t.privacy_annotation, (MeExpr, AllExpr)) {
                    assert!(false,"Only me/all accepted as privacy type of return values for public functions{:?}", ast);
                }
            }
        }
    }
    pub fn visitEnumDefinition(self, ast: EnumDefinition) {
        ast.annotated_type =
            AnnotatedTypeName(EnumTypeName(ast.qualified_name).r#override(target = ast));
    }

    pub fn visitEnumValue(self, ast: EnumValue) {
        ast.annotated_type =
            AnnotatedTypeName(EnumValueTypeName(ast.qualified_name).r#override(target = ast));
    }

    pub fn visitStateVariableDeclaration(self, ast: StateVariableDeclaration) {
        if ast.expr {
            //prevent private operations in declaration
            if contains_private(ast) {
                assert!(
                    false,
                    "Private assignments to state variables must be in the constructor{:?}",
                    ast
                );
            }

            //check type
            self.get_rhs(ast.expr, ast.annotated_type);
        }

        //prevent "me" annotation
        p = ast.annotated_type.privacy_annotation;
        if p.is_me_expr() {
            assert!(false, "State variables cannot be annotated as me{:?}", ast);
        }
    }

    pub fn visitMapping(self, ast: Mapping) {
        if ast.key_label.is_some() {
            if ast.key_type != TypeName.address_type() {
                assert!(false, "Only addresses can be annotated{:?}", ast);
            }
        }
    }

    pub fn visitRequireStatement(self, ast: RequireStatement) {
        if !ast
            .condition
            .annotated_type
            .privacy_annotation
            .is_all_expr()
        {
            assert!(false, "require needs public argument{:?}", ast);
        }
    }

    pub fn visitAnnotatedTypeName(self, ast: AnnotatedTypeName) {
        if ast.type_name.get_ast_type() == UserDefinedTypeName {
            if !isinstance(ast.type_name.target, EnumDefinition) {
                assert!(
                    false,
                    "Unsupported use of user-defined type {:?}",
                    ast.type_name
                )
            }
            ast.type_name = ast.type_name.target.annotated_type.type_name.clone();
        }

        if ast.privacy_annotation != Expression.all_expr() {
            if !ast.type_name.can_be_private() {
                assert!(
                    false,
                    "Currently, we do not support private {},{:?}",
                    ast.type_name, ast
                );
            }
            if ast.homomorphism != Homomorphism.NonHomomorphic {
                //only support uint8, uint16, uint24, uint32 homomorphic data types
                if !ast.type_name.is_numeric {
                    assert!(
                        false,
                        "Homomorphic type not supported for {:?}: Only numeric types supported{:?}",
                        ast.type_name, ast
                    );
                } else if ast.type_name.signed {
                    assert!(false,"Homomorphic type not supported for {:?}: Only unsigned types supported{:?}",ast.type_name, ast);
                } else if ast.type_name.elem_bitwidth > 32 {
                    assert!(false,"Homomorphic type not supported for {:?}: Only up to 32-bit numeric types supported{:?}", ast.type_name,ast);
                }
            }
        }
        p = ast.privacy_annotation;
        if isinstance(p, IdentifierExpr) {
            t = p.target;
            if isinstance(t, Mapping) { //no action necessary, this is the case: mapping(address!x => uint@x)
                 // pass
            } else if !t.is_final && !t.is_constant {
                assert!(
                    false,
                    r#"Privacy annotations must be "final" or "constant", if they are expressions {:?}"#,
                    p
                );
            } else if t.annotated_type != AnnotatedTypeName.address_all() {
                assert!(
                    false,
                    r#"Privacy type is not a public address, but {:?},{:?}"#,
                    t.annotated_type, p
                );
            }
        }
    }
}
