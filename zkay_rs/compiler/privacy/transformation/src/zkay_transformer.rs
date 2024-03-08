#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

// """
// This module defines zkay->solidity transformers for the smaller contract elements (statements, expressions, state variables).
// """

use circuit_helper::circuit_helper::CircuitHelper;
use regex::Regex;
use regex::RegexSetBuilder;
use solidity::fake_solidity_generator::{ID_PATTERN, WS_PATTERN};
use zkay_ast::analysis::contains_private_checker::contains_private_expr;
use zkay_ast::ast::{
    is_instance, ASTBaseRef, ASTChildren, ASTType, AnnotatedTypeName, AssignmentStatement,
    AssignmentStatementBaseMutRef, AssignmentStatementBaseProperty, BlankLine, Block,
    BooleanLiteralExpr, BooleanLiteralType, BreakStatement, BuiltinFunction, ChildListBuilder,
    Comment, CommentBase, ContinueStatement, DoWhileStatement, ElementaryTypeName,
    EncryptionExpression, EnumDefinition, ExprUnion, Expression, ExpressionBaseMutRef,
    ExpressionBaseProperty, ForStatement, FunctionCallExpr, FunctionCallExprBase,
    FunctionCallExprBaseMutRef, FunctionCallExprBaseProperty, HybridArgType, HybridArgumentIdf,
    Identifier, IdentifierBase, IdentifierBaseMutRef, IdentifierDeclaration, IdentifierExpr,
    IdentifierExprUnion, IfStatement, IndexExpr, IntoAST, IntoExpression, IntoStatement,
    LiteralExpr, LocationExpr, Mapping, MeExpr, MemberAccessExpr, NamespaceDefinition,
    NumberLiteralExpr, NumberLiteralType, NumberTypeName, Parameter, PrimitiveCastExpr,
    ReclassifyExpr, ReclassifyExprBaseMutRef, ReclassifyExprBaseProperty, ReturnStatement,
    SimpleStatement, StateVariableDeclaration, Statement, StatementBaseMutRef, StatementList,
    StatementListBaseMutRef, StatementListBaseProperty, TupleExpr, TypeName, VariableDeclaration,
    VariableDeclarationStatement, WhileStatement, AST,
};
use zkay_ast::homomorphism::Homomorphism;
use zkay_ast::visitor::deep_copy::replace_expr;
use zkay_ast::visitor::transformer_visitor::{AstTransformerVisitor, TransformerVisitorEx};
use zkay_config::config::CFG;

// class ZkayVarDeclTransformer(AstTransformerVisitor)
// """
// Transformer for types, which was left out in the paper.

// This removes all privacy labels and converts the types of non-public variables (not @all)
// to cipher_type.
// """

// pub fn __init__(self)
//     super().__init__()
//     self.expr_trafo = ZkayExpressionTransformer(None)
#[derive(Clone)]
pub struct ZkayVarDeclTransformer {
    expr_trafo: Option<ZkayExpressionTransformer>,
}
impl AstTransformerVisitor for ZkayVarDeclTransformer {
    fn default() -> Self {
        Self::new()
    }

    fn visit(&self, _ast: Option<AST>) -> Option<AST> {
        // self._visit_internal(ast)
        None
    }
    fn visitBlock(
        &self,
        _ast: Option<AST>,
        _guard_cond: Option<HybridArgumentIdf>,
        _guard_val: Option<bool>,
    ) -> Option<AST> {
        // self.visit_children(ast)
        None
    }
}
impl ZkayVarDeclTransformer {
    pub fn new() -> Self {
        Self { expr_trafo: None }
    }

    pub fn visitAnnotatedTypeName(&self, ast: AnnotatedTypeName) -> AnnotatedTypeName {
        let t = if ast.is_private() {
            Some(TypeName::cipher_type(ast.clone(), ast.homomorphism.clone()))
        } else {
            if let Some(AST::TypeName(t)) = self.visit(Some((*ast.type_name).to_ast())) {
                Some(t)
            } else {
                None
            }
        };
        AnnotatedTypeName::new(t.unwrap(), None, String::from("NON_HOMOMORPHISM"))
    }

    pub fn visitVariableDeclaration(&self, ast: &mut VariableDeclaration) -> AST {
        if ast.identifier_declaration_base.annotated_type.is_private() {
            ast.identifier_declaration_base.storage_location = Some(String::from("memory"));
        }
        self.visit_children(Some(ast.to_ast())).unwrap()
    }

    pub fn visitParameter(&self, mut ast: &mut Parameter) -> Option<AST> {
        if let Some(AST::IdentifierDeclaration(IdentifierDeclaration::Parameter(mut ast))) =
            self.visit_children(Some(ast.to_ast()))
        {
            if !ast
                .identifier_declaration_base
                .annotated_type
                .type_name
                .is_primitive_type()
            {
                ast.identifier_declaration_base.storage_location = Some(String::from("memory"));
            }
            Some(ast.to_ast())
        } else {
            None
        }
    }

    pub fn visitStateVariableDeclaration(&self, ast: &mut StateVariableDeclaration) -> AST {
        ast.identifier_declaration_base.keywords = ast
            .identifier_declaration_base
            .keywords
            .iter()
            .filter_map(|k| if k != "public" { Some(k.clone()) } else { None })
            .collect();
        //make sure every state var gets a public getter (required for simulation)
        ast.identifier_declaration_base
            .keywords
            .push(String::from("public"));
        ast.expr = if let Some(AST::Expression(expr)) = self
            .expr_trafo
            .as_ref()
            .unwrap()
            .visit(Some(ast.expr.as_ref().unwrap().to_ast()))
        {
            Some(expr)
        } else {
            None
        };
        self.visit_children(Some(ast.to_ast())).unwrap()
    }

    pub fn visitMapping(&self, ast: &mut Mapping) -> AST {
        if ast.key_label.is_some() {
            ast.key_label = Some(Identifier::Identifier(IdentifierBase::new(
                ast.key_label.as_ref().unwrap().name(),
            )));
        }
        self.visit_children(Some(ast.to_ast())).unwrap()
    }
}
// class ZkayStatementTransformer(AstTransformerVisitor)
// """Corresponds to T from paper, (with additional handling of return statement and loops)."""
#[derive(Clone)]
pub struct ZkayStatementTransformer {
    gen: Option<Box<CircuitHelper>>,
    expr_trafo: ZkayExpressionTransformer,
    var_decl_trafo: ZkayVarDeclTransformer,
}
impl AstTransformerVisitor for ZkayStatementTransformer {
    fn default() -> Self {
        Self::new(None)
    }

    fn visit(&self, _ast: Option<AST>) -> Option<AST> {
        // self._visit_internal(ast)
        None
    }
    fn visitBlock(
        &self,
        _ast: Option<AST>,
        _guard_cond: Option<HybridArgumentIdf>,
        _guard_val: Option<bool>,
    ) -> Option<AST> {
        // self.visit_children(ast)
        None
    }
}
impl ZkayStatementTransformer {
    // pub fn __init__(&self, current_gen: CircuitHelper)
    //     super().__init__()
    //     self.gen.unwrap() = current_gen
    //     self.expr_trafo = ZkayExpressionTransformer(self.gen.unwrap())
    //     self.var_decl_trafo = ZkayVarDeclTransformer()
    pub fn new(current_gen: Option<Box<CircuitHelper>>) -> Self {
        Self {
            gen: current_gen.clone(),
            expr_trafo: ZkayExpressionTransformer::new(current_gen),
            var_decl_trafo: ZkayVarDeclTransformer::new(),
        }
    }
    pub fn visitStatementList(&self, ast: &mut StatementList) -> AST
// """
    // Rule (1)

    // All statements are transformed individually.
    // Whenever the transformation of a statement requires the introduction of additional statements
    // (the CircuitHelper indicates this by storing them in the statement"s pre_statements list), they are prepended to the transformed
    // statement in the list.

    // If transformation changes the appearance of a statement (apart from type changes),
    // the statement is wrapped in a comment block which displays the original statement"s code.
    // """
    {
        let mut new_statements = vec![];
        for (_idx, stmt) in ast.statements().iter().enumerate() {
            let old_code = stmt.to_ast().code();
            let transformed_stmt = self.visit(Some(stmt.to_ast()));
            if transformed_stmt.is_none() {
                continue;
            }
            let r = Regex::new(r"@{WS_PATTERN}*{ID_PATTERN}")
                .unwrap()
                .replace_all(&old_code, "");
            let old_code_wo_annotations = Regex::new(r"(?=\b)me(?=\b)")
                .unwrap()
                .replace_all(&r, "msg.sender");
            let code = transformed_stmt.as_ref().unwrap().code();
            let new_code_wo_annotation_comments = Regex::new(r"/\*.*?\*/")
                .unwrap()
                .replace_all(code.as_str(), "");
            if old_code_wo_annotations == new_code_wo_annotation_comments {
                new_statements.push(transformed_stmt.unwrap())
            } else {
                new_statements.extend(CommentBase::comment_wrap_block(
                    old_code,
                    transformed_stmt
                        .as_ref()
                        .unwrap()
                        .pre_statements()
                        .iter()
                        .cloned()
                        .chain([transformed_stmt.unwrap()])
                        .collect(),
                ));
            }
        }
        if !new_statements.is_empty()
            && is_instance(new_statements.last().unwrap(), ASTType::BlankLine)
        {
            new_statements.pop();
        }
        ast.statement_list_base_mut_ref().statements = new_statements;
        ast.to_ast()
    }

    pub fn process_statement_child(&self, child: AST) -> Option<AST>
// """Default statement child handling. Expressions and declarations are visited by the corresponding transformers."""
    {
        if is_instance(&child, ASTType::ExpressionBase) {
            self.expr_trafo.visit(Some(child))
        } else {
            assert!(is_instance(&child, ASTType::VariableDeclaration));
            self.var_decl_trafo.visit(Some(child))
        }
    }

    pub fn visitStatement(&self, ast: &mut Statement) -> AST
// """
    // Rules (3), (4)

    // This is for all the statements where the statements themselves remain untouched and only the children are altered.
    // """
    {
        assert!(
            is_instance(&*ast, ASTType::SimpleStatementBase)
                || is_instance(&*ast, ASTType::VariableDeclarationStatement)
        );
        let mut cb = ChildListBuilder::new();
        ast.process_children(&mut cb);
        cb.children.iter_mut().for_each(|c| {
            self.process_statement_child(c.clone());
        });
        ast.to_ast()
    }

    pub fn visitAssignmentStatement(&mut self, ast: &mut AssignmentStatement) -> AST
// """Rule (2)"""
    {
        let a: AST = self
            .expr_trafo
            .visit(ast.lhs().clone().map(|l| *l))
            .unwrap();
        ast.assignment_statement_base_mut_ref().lhs = Some(Box::new(a));
        ast.assignment_statement_base_mut_ref().rhs = self
            .expr_trafo
            .visit(Some(ast.rhs().as_ref().unwrap().to_ast()))
            .unwrap()
            .try_as_expression();
        let mut modvals = ast.modified_values();
        if CFG.lock().unwrap().user_config.opt_cache_circuit_outputs()
            && is_instance(&**ast.lhs().as_ref().unwrap(), ASTType::IdentifierExpr)
            && is_instance(ast.rhs().as_ref().unwrap(), ASTType::MemberAccessExpr)
        {
            //Skip invalidation if rhs is circuit output
            if is_instance(
                &ast.rhs().as_ref().unwrap().member().unwrap(),
                ASTType::HybridArgumentIdf,
            ) && ast.rhs().as_ref().unwrap().member().unwrap().arg_type()
                == HybridArgType::PubCircuitArg
            {
                modvals = modvals
                    .iter()
                    .filter_map(|mv| {
                        if mv.target()
                            != ast
                                .lhs()
                                .as_ref()
                                .unwrap()
                                .try_as_expression_ref()
                                .unwrap()
                                .try_as_tuple_or_location_expr_ref()
                                .unwrap()
                                .try_as_location_expr_ref()
                                .unwrap()
                                .target()
                        {
                            Some(mv.clone())
                        } else {
                            None
                        }
                    })
                    .collect();
                let ridf = if is_instance(
                    &ast.rhs()
                        .as_ref()
                        .unwrap()
                        .member()
                        .unwrap()
                        .corresponding_priv_expression()
                        .unwrap(),
                    ASTType::EncryptionExpression,
                ) {
                    ast.rhs()
                        .as_ref()
                        .unwrap()
                        .member()
                        .unwrap()
                        .corresponding_priv_expression()
                        .unwrap()
                        .idf()
                } else {
                    ast.rhs()
                        .as_ref()
                        .unwrap()
                        .member()
                        .unwrap()
                        .corresponding_priv_expression()
                        .unwrap()
                        .idf()
                };
                assert!(is_instance(
                    ridf.as_ref().unwrap(),
                    ASTType::HybridArgumentIdf
                ));
                if let Some(Identifier::HybridArgumentIdf(ridf)) = ridf {
                    self.gen.as_mut().unwrap()._remapper.0.remap(
                        ast.lhs()
                            .as_ref()
                            .unwrap()
                            .try_as_expression_ref()
                            .unwrap()
                            .try_as_tuple_or_location_expr_ref()
                            .unwrap()
                            .try_as_location_expr_ref()
                            .unwrap()
                            .target()
                            .unwrap()
                            .idf()
                            .unwrap(),
                        ridf,
                    );
                }
            }
        }

        if self.gen.is_some()
        //Invalidate circuit value for assignment targets
        {
            for val in modvals {
                if val.key().is_none() {
                    self.gen
                        .as_mut()
                        .unwrap()
                        .invalidate_idf(&val.target().unwrap().idf().unwrap());
                }
            }
        }
        ast.to_ast()
    }

    pub fn visitIfStatement(&mut self, ast: &mut IfStatement) -> Statement
// """
    // Rule (6) + additional support for private conditions

    // If the condition is public, guard conditions are introduced for both branches if any of the branches contains private expressions.

    // If the condition is private, the whole if statement is inlined into the circuit. The only side-effects which are allowed
    // inside the branch bodies are assignment statements with an lhs@me. (anything else would leak private information).
    // The if statement will be replaced by an assignment statement where the lhs is a tuple of all locations which are written
    // in either branch and rhs is a tuple of the corresponding circuit outputs.
    // """
    {
        if ast.condition.annotated_type().as_ref().unwrap().is_public() {
            if contains_private_expr(Some(ast.then_branch.to_ast()))
                || contains_private_expr(ast.else_branch.as_ref().map(|v| v.to_ast()))
            {
                let before_if_state = self.gen.as_ref().unwrap()._remapper.0.get_state();
                let guard_var = self
                    .gen
                    .as_mut()
                    .unwrap()
                    .add_to_circuit_inputs(&mut ast.condition);
                ast.condition = guard_var
                    .get_loc_expr(Some(ast.to_ast()))
                    .try_as_expression()
                    .unwrap();
                self.gen.as_mut().unwrap().guarded(guard_var.clone(), true);
                {
                    ast.then_branch = self
                        .visit(Some(ast.then_branch.to_ast()))
                        .unwrap()
                        .block()
                        .unwrap();
                    self.gen
                        .as_mut()
                        .unwrap()
                        ._remapper
                        .0
                        .set_state(&before_if_state);
                }
                if ast.else_branch.is_some() {
                    self.gen.as_mut().unwrap().guarded(guard_var, false);

                    ast.else_branch = self
                        .visit(Some(ast.else_branch.as_ref().unwrap().to_ast()))
                        .unwrap()
                        .block();
                    self.gen
                        .as_mut()
                        .unwrap()
                        ._remapper
                        .0
                        .set_state(&before_if_state);
                }

                //Invalidate values modified in either branch
                for val in &ast.statement_base.ast_base.modified_values {
                    if val.key().is_none() {
                        self.gen
                            .as_mut()
                            .unwrap()
                            .invalidate_idf(&val.target().unwrap().idf().unwrap());
                    }
                }
            } else {
                ast.condition = self
                    .expr_trafo
                    .visit(Some(ast.condition.to_ast()))
                    .unwrap()
                    .try_as_expression()
                    .unwrap();
                ast.then_branch = self
                    .visit(Some(ast.then_branch.to_ast()))
                    .unwrap()
                    .block()
                    .unwrap();
                if ast.else_branch.is_some() {
                    ast.else_branch = self
                        .visit(Some(ast.else_branch.as_ref().unwrap().to_ast()))
                        .unwrap()
                        .block();
                }
            }
            (*ast).to_statement()
        } else {
            self.gen
                .as_mut()
                .unwrap()
                .evaluate_stmt_in_circuit(ast.to_statement())
                .to_statement()
        }
    }
    pub fn visitWhileStatement(&self, ast: WhileStatement) -> WhileStatement
//Loops must always be purely public
    {
        assert!(!contains_private_expr(Some(ast.condition.to_ast())));
        assert!(!contains_private_expr(Some(ast.body.to_ast())));
        ast
    }

    pub fn visitDoWhileStatement(&self, ast: DoWhileStatement) -> DoWhileStatement
//Loops must always be purely public
    {
        assert!(!contains_private_expr(Some(ast.condition.to_ast())));
        assert!(!contains_private_expr(Some(ast.body.to_ast())));
        ast
    }

    pub fn visitForStatement(&self, ast: &mut ForStatement) -> ForStatement {
        if ast.init.is_some()
        //Init is the only part of a for loop which may contain private expressions
        {
            ast.init = self
                .visit(Some(ast.init.as_ref().unwrap().to_ast()))
                .unwrap()
                .init()
                .map(|i| Box::new(i));
            ast.statement_base
                .pre_statements
                .extend(ast.init.as_ref().unwrap().pre_statements());
        }
        assert!(!contains_private_expr(Some(ast.condition.to_ast())));
        assert!(
            !ast.update.is_some()
                || !contains_private_expr(ast.update.as_ref().map(|v| v.to_ast()))
        );
        assert!(!contains_private_expr(Some(ast.body.to_ast()))); //OR fixed size loop -> static analysis can prove that loop terminates in fixed //iterations
        ast.clone()
    }

    pub fn visitContinueStatement(&self, ast: ContinueStatement) -> ContinueStatement {
        ast
    }

    pub fn visitBreakStatement(&self, ast: BreakStatement) -> BreakStatement {
        ast
    }

    pub fn visitReturnStatement(&mut self, ast: &mut ReturnStatement) -> Option<AST>
// """
    // Handle return statement.

    // If the function requires verification, the return statement is replaced by an assignment to a return variable.
    // (which will be returned at the very end of the function body, after any verification wrapper code).
    // Otherwise only the expression is transformed.
    // """
    {
        if ast
            .statement_base
            .function
            .as_ref()
            .unwrap()
            .requires_verification
        {
            if ast.expr.is_none() {
                return None;
            }
            assert!(!self.gen.as_ref().unwrap().has_return_var);
            self.gen.as_mut().unwrap().has_return_var = true;
            let expr = self
                .expr_trafo
                .visit(Some(ast.expr.as_ref().unwrap().to_ast()));
            let ret_args = ast
                .statement_base
                .function
                .as_ref()
                .unwrap()
                .return_var_decls
                .iter()
                .map(|vd| {
                    let mut idf = IdentifierExpr::new(
                        IdentifierExprUnion::Identifier(
                            *vd.identifier_declaration_base.idf.clone(),
                        ),
                        None,
                    );
                    idf.location_expr_base.target = Some(Box::new(vd.to_ast()));
                    idf.to_expr()
                })
                .collect();
            let mut te =
                TupleExpr::new(ret_args).assign(expr.unwrap().try_as_expression().unwrap());
            te.statement_base_mut_ref().pre_statements = ast.statement_base.pre_statements.clone();
            Some(te.to_ast())
        } else {
            ast.expr = if let Some(AST::Expression(expr)) =
                self.expr_trafo.visit(ast.expr.clone().map(|e| e.to_ast()))
            {
                Some(expr)
            } else {
                None
            };
            Some(ast.to_ast())
        }
    }

    pub fn visitExpression(&self, ast: Expression)
    // """Fail if there are any untransformed expressions left."""
    {
        assert!(false, "Missed an expression of type {:?}", ast);
    }
}
// class ZkayExpressionTransformer(AstTransformerVisitor)
// """
// Roughly corresponds to T_L / T_e from paper.

// T_L and T_e are equivalent here, because parameter encryption checks are handled in the verification wrapper of the function body.
// In addition to the features described in the paper, this transformer also supports primitive type casting,
// tuples (multiple return values), operations with short-circuiting and function calls.
// """
#[derive(Clone)]
pub struct ZkayExpressionTransformer {
    gen: Option<Box<CircuitHelper>>,
}
impl TransformerVisitorEx for ZkayExpressionTransformer {}
impl AstTransformerVisitor for ZkayExpressionTransformer {
    fn default() -> Self {
        Self::new(None)
    }

    fn visit(&self, _ast: Option<AST>) -> Option<AST> {
        // self._visit_internal(ast)
        None
    }
    fn visitBlock(
        &self,
        _ast: Option<AST>,
        _guard_cond: Option<HybridArgumentIdf>,
        _guard_val: Option<bool>,
    ) -> Option<AST> {
        // self.visit_children(ast)
        None
    }
}
impl ZkayExpressionTransformer {
    pub fn new(current_generator: Option<Box<CircuitHelper>>) -> Self
// super().__init__()
        // self.gen.unwrap() = current_generator
    {
        Self {
            gen: current_generator,
        }
    }

    // @staticmethod
    pub fn visitMeExpr(ast: MeExpr) -> Expression
// """Replace me with msg.sender."""
    {
        replace_expr(
            &ast.to_expr(),
            &mut LocationExpr::IdentifierExpr(IdentifierExpr::new(
                IdentifierExprUnion::String(String::from("msg")),
                None,
            ))
            .to_expr(),
            false,
        )
        .to_location_expr()
        .unwrap()
        .dot(IdentifierExprUnion::String(String::from("sender")))
        .as_type(AST::AnnotatedTypeName(AnnotatedTypeName::address_all()))
        .to_expr()
    }

    pub fn visitLiteralExpr(&self, ast: LiteralExpr) -> LiteralExpr
// """Rule (7), don"t modify constants."""
    {
        ast
    }

    pub fn visitIdentifierExpr(&self, ast: IdentifierExpr) -> IdentifierExpr
// """Rule (8), don"t modify identifiers."""
    {
        ast
    }

    pub fn visitIndexExpr(&self, ast: IndexExpr) -> Expression
// """Rule (9), transform location and index expressions separately."""
    {
        replace_expr(
            &ast.to_expr(),
            &mut self
                .visit(ast.arr.map(|a| (*a).to_ast()))
                .unwrap()
                .to_location_expr()
                .unwrap()
                .index(ExprUnion::Expression(
                    self.visit(Some((*ast.key).to_ast()))
                        .unwrap()
                        .try_as_expression()
                        .unwrap(),
                ))
                .to_expr(),
            false,
        )
    }

    pub fn visitMemberAccessExpr(&self, ast: MemberAccessExpr) {
        self.visit_children(Some(ast.to_ast()));
    }

    pub fn visitTupleExpr(&self, ast: TupleExpr) {
        self.visit_children(Some(ast.to_ast()));
    }

    pub fn visitReclassifyExpr(&mut self, mut ast: ReclassifyExpr) -> LocationExpr
// """
    // Rule (11), trigger a boundary crossing.

    // The reclassified expression is evaluated in the circuit and its result is made available in solidity.
    // """
    {
        let mut expr = ast.reclassify_expr_base_mut_ref().expr.clone();
        self.gen.as_mut().unwrap().evaluate_expr_in_circuit(
            &mut expr,
            &ast.privacy()
                .unwrap()
                .privacy_annotation_label()
                .unwrap()
                .into(),
            &ast.annotated_type().as_ref().unwrap().homomorphism,
        )
    }

    pub fn visitBuiltinFunction(&self, ast: BuiltinFunction) -> BuiltinFunction {
        ast
    }

    pub fn visitFunctionCallExpr(&mut self, mut ast: FunctionCallExpr) -> AST {
        if is_instance(&**ast.func(), ASTType::BuiltinFunction) {
            if ast.func().is_private()
            // """
            // Modified Rule (12) builtin functions with private operands and homomorphic operations on ciphertexts
            // are evaluated inside the circuit.

            // A private expression on its own (like an IdentifierExpr referring to a private variable) is not enough to trigger a
            // boundary crossing (assignment of private variables is a public operation).
            // """
            {
                let privacy_label = ast
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .privacy_annotation
                    .as_ref()
                    .unwrap()
                    .try_as_expression_ref()
                    .unwrap()
                    .privacy_annotation_label();
                return self
                    .gen
                    .as_mut()
                    .unwrap()
                    .evaluate_expr_in_circuit(
                        &mut ast.to_expr(),
                        &(privacy_label.unwrap().into()),
                        &(ast
                            .func()
                            .try_as_builtin_function_ref()
                            .unwrap()
                            .homomorphism
                            .clone()
                            .into()),
                    )
                    .to_ast();
            } else
            // """
            // Rule (10) with additional short-circuit handling.

            // Builtin operations on public operands are normally left untransformed, but if the builtin function has
            // short-circuiting semantics, guard conditions must be added if any of the public operands contains
            // nested private expressions.
            // """
            //handle short-circuiting
            {
                let mut args = ast.args().clone();
                if ast.func().has_shortcircuiting()
                    && args[1..]
                        .iter()
                        .any(|arg| contains_private_expr(Some(arg.to_ast())))
                {
                    let op = &ast.func().op().unwrap();
                    let guard_var = self
                        .gen
                        .as_mut()
                        .unwrap()
                        .add_to_circuit_inputs(&mut args[0]);
                    args[0] = guard_var
                        .get_loc_expr(Some(ast.to_ast()))
                        .try_as_expression()
                        .unwrap();
                    if op == "ite" {
                        args[1] = self
                            .visit_guarded_expression(guard_var.clone(), true, &mut args[1].clone())
                            .try_as_expression()
                            .unwrap();
                        args[2] = self
                            .visit_guarded_expression(
                                guard_var.clone(),
                                false,
                                &mut args[2].clone(),
                            )
                            .try_as_expression()
                            .unwrap();
                    } else if op == "||" {
                        args[1] = self
                            .visit_guarded_expression(
                                guard_var.clone(),
                                false,
                                &mut args[1].clone(),
                            )
                            .try_as_expression()
                            .unwrap();
                    } else if op == "&&" {
                        args[1] = self
                            .visit_guarded_expression(guard_var.clone(), true, &mut args[1].clone())
                            .try_as_expression()
                            .unwrap();
                    }
                    ast.function_call_expr_base_mut_ref().args = args;
                }

                return self.visit_children(Some(ast.to_ast())).unwrap();
            }
        } else if ast.is_cast()
        // """Casts are handled either in public or inside the circuit depending on the privacy of the casted expression."""
        {
            assert!(is_instance(
                &ast.func().target().map(|t| *t).unwrap(),
                ASTType::EnumDefinition
            ));
            if ast.args()[0].evaluate_privately() {
                let privacy_label = ast
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .privacy_annotation
                    .as_ref()
                    .unwrap()
                    .try_as_expression_ref()
                    .unwrap()
                    .privacy_annotation_label();
                return self
                    .gen
                    .as_mut()
                    .unwrap()
                    .evaluate_expr_in_circuit(
                        &mut ast.to_expr(),
                        &privacy_label.unwrap().into(),
                        &ast.annotated_type().as_ref().unwrap().homomorphism,
                    )
                    .to_ast();
            } else {
                return self.visit_children(Some(ast.to_ast())).unwrap();
            }
        } else
        // """
        // Handle normal function calls (outside private expression case).

        // The called functions are allowed to have side effects,
        // if the function does not require verification it can even be recursive.
        // """
        {
            assert!(is_instance(&**ast.func(), ASTType::LocationExprBase));
            let mut ast = self
                .visit_children(Some(ast.to_ast()))
                .unwrap()
                .try_as_expression()
                .unwrap()
                .try_as_function_call_expr()
                .unwrap();
            if ast
                .func()
                .target()
                .unwrap()
                .constructor_or_function_definition()
                .unwrap()
                .requires_verification_when_external
            //Reroute the function call to the corresponding internal function if the called function was split into external/internal.
            {
                if !is_instance(&**ast.func(), ASTType::IdentifierExpr) {
                    unimplemented!();
                }
                ast.function_call_expr_base_mut_ref()
                    .func
                    .try_as_tuple_or_location_expr_mut()
                    .unwrap()
                    .try_as_location_expr_mut()
                    .unwrap()
                    .try_as_identifier_expr_mut()
                    .unwrap()
                    .idf
                    .identifier_base_mut_ref()
                    .name = CFG.lock().unwrap().get_internal_name(
                    &*ast.function_call_expr_base_mut_ref().func.target().unwrap(),
                );
            }

            if ast.func().target().unwrap().requires_verification()
            //If the target function has an associated circuit, make this function"s circuit aware of the call.
            {
                // let cf = if let AST::Expression(Expression::FunctionCallExpr(fce)) = &ast {
                //     Some(fce.clone())
                // } else {
                //     None
                // };
                self.gen.as_mut().unwrap().call_function(&ast);
            } else if ast
                .func()
                .target()
                .unwrap()
                .constructor_or_function_definition()
                .unwrap()
                .has_side_effects()
                && self.gen.is_some()
            //Invalidate modified state variables for the current circuit
            {
                for val in &ast.ast_base_ref().modified_values {
                    if val.key().is_none()
                        && is_instance(
                            &val.target().map(|t| *t).unwrap(),
                            ASTType::StateVariableDeclaration,
                        )
                    {
                        self.gen
                            .as_mut()
                            .unwrap()
                            .invalidate_idf(&val.target().unwrap().idf().unwrap());
                    }
                }
            }

            //The call will be present as a normal function call in the output solidity code.
            ast.to_ast()
        }
    }
    pub fn visit_guarded_expression(
        &mut self,
        guard_var: HybridArgumentIdf,
        if_true: bool,
        expr: &mut Expression,
    ) -> AST {
        let prelen = expr
            .statement()
            .as_ref()
            .unwrap()
            .statement_base_ref()
            .unwrap()
            .pre_statements
            .len();

        //Transform expression with guard condition in effect
        self.gen
            .as_mut()
            .unwrap()
            .guarded(guard_var.clone(), if_true);
        let ret = self.visit(Some(expr.to_ast()));

        //If new pre statements were added, they must be guarded using an if statement in the public solidity code
        let new_pre_stmts = expr
            .statement()
            .as_ref()
            .unwrap()
            .statement_base_ref()
            .unwrap()
            .pre_statements[prelen..]
            .to_vec();
        if !new_pre_stmts.is_empty() {
            let mut cond_expr: AST = guard_var.get_loc_expr(None).into();
            if let AST::Expression(Expression::LiteralExpr(LiteralExpr::BooleanLiteralExpr(
                ref mut cond_expr,
            ))) = cond_expr
            {
                *cond_expr = BooleanLiteralExpr::new(cond_expr.value == if_true);
            } else if !if_true {
                cond_expr = cond_expr
                    .try_as_expression()
                    .unwrap()
                    .unop(String::from("!"))
                    .to_ast();
            }
            let ps = expr
                .statement()
                .as_ref()
                .unwrap()
                .statement_base_ref()
                .unwrap()
                .pre_statements[..prelen]
                .iter()
                .cloned()
                .chain([IfStatement::new(
                    cond_expr.try_as_expression().unwrap(),
                    Block::new(new_pre_stmts, false),
                    None,
                )
                .to_ast()])
                .collect();
            expr.expression_base_mut_ref()
                .statement
                .as_mut()
                .unwrap()
                .statement_base_mut_ref()
                .unwrap()
                .pre_statements = ps;
        }
        ret.unwrap()
    }

    pub fn visitPrimitiveCastExpr(&mut self, ast: PrimitiveCastExpr) -> AST
// """Casts are handled either in public or inside the circuit depending on the privacy of the casted expression."""
    {
        if ast.expression_base.evaluate_privately {
            let privacy_label = ast
                .expression_base
                .annotated_type
                .as_ref()
                .unwrap()
                .privacy_annotation
                .as_ref()
                .unwrap()
                .try_as_expression_ref()
                .unwrap()
                .privacy_annotation_label();
            self.gen
                .as_mut()
                .unwrap()
                .evaluate_expr_in_circuit(
                    &mut ast.to_expr(),
                    &(privacy_label.unwrap().into()),
                    &ast.expression_base
                        .annotated_type
                        .as_ref()
                        .unwrap()
                        .homomorphism,
                )
                .to_ast()
        } else {
            self.visit_children(Some(ast.to_ast())).unwrap()
        }
    }

    pub fn visitExpression(&self, _ast: Expression) {
        // raise NotImplementedError()
        unimplemented!();
    }
}
// class ZkayCircuitTransformer(AstTransformerVisitor)
// """
// Corresponds to T_phi from paper.

// This extends the abstract circuit representation while transforming private expressions and statements.
// Private expressions can never have side effects.
// Private statements may contain assignment statements with lhs@me (no other types of side effects are allowed).
// """
#[derive(Clone)]
pub struct ZkayCircuitTransformer {
    gen: Option<Box<CircuitHelper>>,
}

impl TransformerVisitorEx for ZkayCircuitTransformer {}
impl AstTransformerVisitor for ZkayCircuitTransformer {
    fn default() -> Self {
        Self::new(None)
    }

    fn visit(&self, _ast: Option<AST>) -> Option<AST> {
        // self._visit_internal(ast)
        None
    }
    fn visitBlock(
        &self,
        _ast: Option<AST>,
        _guard_cond: Option<HybridArgumentIdf>,
        _guard_val: Option<bool>,
    ) -> Option<AST> {
        // self.visit_children(ast)
        None
    }
}
impl ZkayCircuitTransformer {
    pub fn new(current_generator: Option<Box<CircuitHelper>>) -> Self {
        Self {
            gen: current_generator,
        }
    }
    // super().__init__()
    // self.gen.unwrap() = current_generator

    pub fn visitLiteralExpr(&self, ast: LiteralExpr) -> LiteralExpr
// """Rule (13), don"t modify constants."""
    {
        ast
    }

    pub fn visitIndexExpr(&mut self, ast: IndexExpr) -> LocationExpr {
        self.transform_location(LocationExpr::IndexExpr(ast))
    }

    pub fn visitIdentifierExpr(&mut self, mut ast: IdentifierExpr) -> LocationExpr {
        if !is_instance(&*ast.idf, ASTType::HybridArgumentIdf)
        //If ast is not already transformed, get current SSA version
        {
            ast = self.gen.as_ref().unwrap().get_remapped_idf_expr(ast);
        }
        if is_instance(&ast, ASTType::IdentifierExpr)
            && is_instance(&*ast.idf, ASTType::HybridArgumentIdf)
        //The current version of ast.idf is already in the circuit
        {
            assert!(ast.idf.arg_type() != HybridArgType::PubContractVal);
            LocationExpr::IdentifierExpr(ast)
        } else
        //ast is not yet in the circuit -> move it in
        {
            self.transform_location(LocationExpr::IdentifierExpr(ast))
        }
    }

    pub fn transform_location(&mut self, loc: LocationExpr) -> LocationExpr
// """Rule (14), move location into the circuit."""
    {
        LocationExpr::IdentifierExpr(
            self.gen
                .as_mut()
                .unwrap()
                .add_to_circuit_inputs(&mut loc.to_expr())
                .get_idf_expr(&None),
        )
    }

    pub fn visitReclassifyExpr(&mut self, mut ast: ReclassifyExpr) -> AST
// """Rule (15), boundary crossing if analysis determined that it is """
    {
        if ast.annotated_type().as_ref().unwrap().is_cipher()
        //We need a homomorphic ciphertext -> make sure the correct encryption of the value is available
        {
            let orig_type = ast.annotated_type().as_ref().unwrap().zkay_type();
            let orig_privacy = orig_type
                .privacy_annotation
                .unwrap()
                .try_as_expression_ref()
                .unwrap()
                .privacy_annotation_label();
            let orig_homomorphism = orig_type.homomorphism;
            self.gen
                .as_mut()
                .unwrap()
                .evaluate_expr_in_circuit(
                    &mut ast.reclassify_expr_base_mut_ref().expr,
                    &orig_privacy.unwrap().into(),
                    &orig_homomorphism,
                )
                .to_ast()
        } else if ast.expr().evaluate_privately() {
            self.visit(Some(ast.expr().to_ast())).unwrap()
        } else {
            assert!(ast.expr().annotated_type().as_ref().unwrap().is_public());
            self.gen
                .as_mut()
                .unwrap()
                .add_to_circuit_inputs(&mut ast.reclassify_expr_base_mut_ref().expr)
                .get_idf_expr(&None)
                .to_ast()
        }
    }

    pub fn visitExpression(&self, ast: Expression) -> AST
// """Rule (16), other expressions don"t need special treatment."""
    {
        self.visit_children(Some(ast.to_ast())).unwrap()
    }

    pub fn visitFunctionCallExpr(&mut self, mut ast: FunctionCallExpr) -> Expression {
        let t = *ast.annotated_type().as_ref().unwrap().type_name.clone();

        //Constant folding for literal types
        if let TypeName::ElementaryTypeName(ElementaryTypeName::BooleanLiteralType(t)) = &t {
            return replace_expr(
                &ast.to_expr(),
                &mut BooleanLiteralExpr::new(t.value()).to_expr(),
                false,
            );
        } else if let TypeName::ElementaryTypeName(ElementaryTypeName::NumberTypeName(
            NumberTypeName::NumberLiteralType(t),
        )) = &t
        {
            return replace_expr(
                &ast.to_expr(),
                &mut NumberLiteralExpr::new(t.value(), false).to_expr(),
                false,
            );
        }

        if is_instance(&**ast.func(), ASTType::BuiltinFunction) {
            if ast
                .func()
                .try_as_builtin_function_ref()
                .unwrap()
                .homomorphism
                != Homomorphism::non_homomorphic()
            //To perform homomorphic operations, we require the recipient"s public key
            {
                let crypto_params = CFG.lock().unwrap().user_config.get_crypto_params(
                    &ast.func()
                        .try_as_builtin_function_ref()
                        .unwrap()
                        .homomorphism,
                );
                let recipient = ast
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .zkay_type()
                    .privacy_annotation
                    .unwrap()
                    .try_as_expression_ref()
                    .unwrap()
                    .privacy_annotation_label();
                let mut s = ast.statement().as_ref().unwrap().clone();
                ast.function_call_expr_base_mut_ref().public_key = Some(Box::new(
                    self.gen.as_mut().unwrap()._require_public_key_for_label_at(
                        Some(&mut s),
                        &recipient.unwrap().into(),
                        &crypto_params,
                    ),
                ));
                let mut args = ast.args().clone();
                if &ast.func().op().unwrap() == "*"
                //special case: private scalar multiplication using additive homomorphism
                //TODO ugly hack below removes ReclassifyExpr
                {
                    let mut new_args = vec![];
                    for arg in args {
                        let mut arg = arg.clone();
                        if is_instance(&arg, ASTType::ReclassifyExpr) {
                            arg = *arg.try_as_reclassify_expr_ref().unwrap().expr().clone();
                            ast.function_call_expr_base_mut_ref()
                                .func
                                .try_as_builtin_function_mut()
                                .unwrap()
                                .rerand_using = Some(Box::new(
                                self.gen
                                    .as_mut()
                                    .unwrap()
                                    .get_randomness_for_rerand(ast.to_expr()),
                            ));
                        //result requires re-randomization
                        } else if arg.annotated_type().as_ref().unwrap().is_private() {
                            arg.expression_base_mut_ref().annotated_type =
                                Some(AnnotatedTypeName::cipher_type(
                                    arg.annotated_type().as_ref().unwrap().clone(),
                                    Some(
                                        ast.func()
                                            .try_as_builtin_function_ref()
                                            .unwrap()
                                            .homomorphism
                                            .clone(),
                                    ),
                                ));
                        }
                        new_args.push(arg);
                    }
                    ast.function_call_expr_base_mut_ref().args = new_args;
                } else
                //We require all non-public arguments to be present as ciphertexts
                {
                    for arg in args.iter_mut() {
                        if arg.annotated_type().as_ref().unwrap().is_private() {
                            arg.expression_base_mut_ref().annotated_type =
                                Some(AnnotatedTypeName::cipher_type(
                                    arg.annotated_type().as_ref().unwrap().clone(),
                                    Some(
                                        ast.func()
                                            .try_as_builtin_function_ref()
                                            .unwrap()
                                            .homomorphism
                                            .clone(),
                                    ),
                                ));
                        }
                    }
                }
            }

            //Builtin functions are supported natively by the circuit
            return self
                .visit_children(Some(ast.to_ast()))
                .unwrap()
                .try_as_expression()
                .unwrap();
        }

        let fdef = &*ast.func().target().unwrap();
        assert!(fdef
            .constructor_or_function_definition()
            .unwrap()
            .is_function());
        assert!(!fdef
            .constructor_or_function_definition()
            .unwrap()
            .return_parameters
            .is_empty());
        assert!(
            fdef.constructor_or_function_definition()
                .unwrap()
                .has_static_body
        );

        //Function call inside private expression -> entire body will be inlined into circuit.
        //Function must not have side-effects (only pure and view is allowed) and cannot have a nonstatic body (i.e. recursion)
        let mut fce = if let FunctionCallExpr::FunctionCallExpr(ref mut fce) = ast {
            Some(fce.clone())
        } else {
            None
        };
        return self
            .gen
            .as_mut()
            .unwrap()
            .inline_function_call_into_circuit(&mut fce.unwrap())
            .unwrap()
            .try_as_expression()
            .unwrap();
    }

    pub fn visitReturnStatement(&mut self, ast: &mut ReturnStatement) {
        self.gen.as_mut().unwrap().add_return_stmt_to_circuit(ast)
    }

    pub fn visitAssignmentStatement(&mut self, ast: &mut AssignmentStatement) {
        self.gen.as_mut().unwrap().add_assignment_to_circuit(ast)
    }

    pub fn visitVariableDeclarationStatement(&mut self, ast: &mut VariableDeclarationStatement) {
        self.gen.as_mut().unwrap().add_var_decl_to_circuit(ast)
    }

    pub fn visitIfStatement(&mut self, ast: &mut IfStatement) {
        self.gen.as_mut().unwrap().add_if_statement_to_circuit(ast)
    }

    pub fn visitBlock(
        &mut self,
        ast: &mut Block,
        guard_cond: Option<HybridArgumentIdf>,
        guard_val: Option<bool>,
    ) {
        self.gen
            .as_mut()
            .unwrap()
            .add_block_to_circuit(ast, guard_cond, guard_val)
    }

    pub fn visitStatement(&mut self, _ast: Statement)
    // """Fail if statement type was not handled."""
    // raise NotImplementedError("Unsupported statement")
    {
        unimplemented!("Unsupported statement")
    }
}
