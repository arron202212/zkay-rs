// """
// This module defines zkay->solidity transformers for the smaller contract elements (statements, expressions, state variables).
// """

use crate::compiler::privacy::circuit_generation::circuit_helper::CircuitHelper;
use crate::compiler::solidity::fake_solidity_generator::{ID_PATTERN, WS_PATTERN};
use crate::config::CFG;
use crate::zkay_ast::analysis::contains_private_checker::contains_private_expr;
use crate::zkay_ast::ast::{
    is_instance, ASTCode, ASTType, AnnotatedTypeName, AssignmentStatement, BlankLine, Block,
    BooleanLiteralExpr, BooleanLiteralType, BreakStatement, BuiltinFunction, ChildListBuilder,
    Comment, CommentBase, ContinueStatement, DoWhileStatement, EncryptionExpression,
    EnumDefinition, Expression, ForStatement, FunctionCallExpr, HybridArgType, HybridArgumentIdf,
    IdentifierExpr, IdentifierExprUnion, IfStatement, IndexExpr, LiteralExpr, LocationExpr,
    Mapping, MeExpr, MemberAccessExpr, NumberLiteralExpr, NumberLiteralType, Parameter,
    PrimitiveCastExpr, ReclassifyExpr, ReturnStatement, SimpleStatement, StateVariableDeclaration,
    Statement, StatementList, TupleExpr, TypeName, VariableDeclaration,
    VariableDeclarationStatement, WhileStatement, AST,
};
use crate::zkay_ast::homomorphism::Homomorphism;
use crate::zkay_ast::visitor::deep_copy::replace_expr;
use crate::zkay_ast::visitor::transformer_visitor::AstTransformerVisitor;
use regex::Regex;
use regex::RegexSetBuilder;
pub trait TransformerVisitorEx:
    Clone + std::marker::Sync + crate::zkay_ast::visitor::transformer_visitor::AstTransformerVisitor
{
}
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
pub struct ZkayVarDeclTransformer<V: TransformerVisitorEx> {
    expr_trafo: Option<ZkayExpressionTransformer<V>>,
}
impl<V: TransformerVisitorEx> AstTransformerVisitor for ZkayVarDeclTransformer<V> {
    fn default() -> Self {
        Self::new()
    }

    fn visit(&self, ast: AST) -> AST {
        // self._visit_internal(ast)
        AST::None
    }
    fn visitBlock(
        &self,
        ast: AST,
        guard_cond: Option<HybridArgumentIdf>,
        guard_val: Option<bool>,
    ) -> AST {
        // self.visit_children(ast)
        AST::None
    }
}
impl<V: TransformerVisitorEx> ZkayVarDeclTransformer<V> {
    pub fn new() -> Self {
        Self { expr_trafo: None }
    }

    pub fn visitAnnotatedTypeName(&self, ast: AnnotatedTypeName) -> AnnotatedTypeName {
        let t = if ast.is_private() {
            TypeName::cipher_type(ast, ast.homomorphism)
        } else {
            if let AST::TypeName(t) = self.visit((*ast.type_name).get_ast()) {
                t
            } else {
                TypeName::None
            }
        };
        AnnotatedTypeName::new(t, None, String::from("NON_HOMOMORPHISM"))
    }

    pub fn visitVariableDeclaration(&self, ast: VariableDeclaration) -> AST {
        if ast.identifier_declaration_base.annotated_type.is_private() {
            ast.identifier_declaration_base.storage_location = Some(String::from("memory"));
        }
        self.visit_children(ast.get_ast())
    }

    pub fn visitParameter(&self, mut ast: Parameter) -> AST {
        if let AST::IdentifierDeclaration(IdentifierDeclaration::Parameter(ast)) =
            self.visit_children(ast.get_ast())
        {
            if !ast
                .identifier_declaration_base
                .annotated_type
                .type_name
                .is_primitive_type()
            {
                ast.identifier_declaration_base.storage_location = Some(String::from("memory"));
            }
            ast.get_ast()
        } else {
            AST::None
        }
    }

    pub fn visitStateVariableDeclaration(&self, ast: StateVariableDeclaration) -> AST {
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
        ast.expr = Some(
            if let AST::Expression(expr) = self.expr_trafo.unwrap().visit(ast.expr) {
                expr
            } else {
                Expression::None
            },
        );
        self.visit_children(ast.get_ast())
    }

    pub fn visitMapping(&self, ast: Mapping) -> AST {
        if ast.key_label.is_some() {
            ast.key_label = ast.key_label.unwrap().name();
        }
        self.visit_children(ast.get_ast())
    }
}
// class ZkayStatementTransformer(AstTransformerVisitor)
// """Corresponds to T from paper, (with additional handling of return statement and loops)."""
#[derive(Clone)]
pub struct ZkayStatementTransformer<V: TransformerVisitorEx> {
    gen: Option<Box<CircuitHelper<V>>>,
    expr_trafo: ZkayExpressionTransformer<V>,
    var_decl_trafo: ZkayVarDeclTransformer<V>,
}
impl<V: TransformerVisitorEx> AstTransformerVisitor for ZkayStatementTransformer<V> {
    fn default() -> Self {
        Self::new(None)
    }

    fn visit(&self, ast: AST) -> AST {
        // self._visit_internal(ast)
        AST::None
    }
    fn visitBlock(
        &self,
        ast: AST,
        guard_cond: Option<HybridArgumentIdf>,
        guard_val: Option<bool>,
    ) -> AST {
        // self.visit_children(ast)
        AST::None
    }
}
impl<V: TransformerVisitorEx> ZkayStatementTransformer<V> {
    // pub fn __init__(&self, current_gen: CircuitHelper)
    //     super().__init__()
    //     self.gen.unwrap() = current_gen
    //     self.expr_trafo = ZkayExpressionTransformer(self.gen.unwrap())
    //     self.var_decl_trafo = ZkayVarDeclTransformer()
    pub fn new(current_gen: Option<Box<CircuitHelper<V>>>) -> Self {
        Self {
            gen: current_gen,
            expr_trafo: ZkayExpressionTransformer::new(Some(current_gen)),
            var_decl_trafo: ZkayVarDeclTransformer::new(Some(current_gen)),
        }
    }
    pub fn visitStatementList(&self, ast: StatementList) -> AST
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
        for (idx, stmt) in ast.statements().iter().enumerate() {
            let old_code = stmt.code();
            let transformed_stmt = self.visit(stmt.get_ast());
            if transformed_stmt == AST::None {
                continue;
            }

            let old_code_wo_annotations = Regex::new(r"(?=\b)me(?=\b)").unwrap().replace_all(
                &Regex::new(r"@{WS_PATTERN}*{ID_PATTERN}")
                    .unwrap()
                    .replace_all(&old_code, ""),
                "msg.sender",
            );
            let new_code_wo_annotation_comments = Regex::new(r"/\*.*?\*/")
                .unwrap()
                .replace_all(&transformed_stmt.code(), "");
            if old_code_wo_annotations == new_code_wo_annotation_comments {
                new_statements.push(transformed_stmt)
            } else {
                new_statements.extend(CommentBase::comment_wrap_block(
                    old_code,
                    transformed_stmt
                        .pre_statements
                        .iter()
                        .chain([transformed_stmt])
                        .collect(),
                ));
            }
        }
        if !new_statements.is_empty()
            && is_instance(new_statements.last().unwrap(), ASTType::BlankLine)
        {
            new_statements.pop();
        }
        ast.statements = new_statements;
        ast.get_ast()
    }

    pub fn process_statement_child(&self, child: AST) -> AST
// """Default statement child handling. Expressions and declarations are visited by the corresponding transformers."""
    {
        if is_instance(&child, ASTType::Expression) {
            return self.expr_trafo.visit(child);
        } else if child != AST::None {
            assert!(is_instance(&child, ASTType::VariableDeclaration));
            return self.var_decl_trafo.visit(child);
        }
        AST::None
    }

    pub fn visitStatement(&self, ast: Statement) -> AST
// """
    // Rules (3), (4)

    // This is for all the statements where the statements themselves remain untouched and only the children are altered.
    // """
    {
        assert!(
            is_instance(&ast, ASTType::SimpleStatement)
                || is_instance(&ast, ASTType::VariableDeclarationStatement)
        );
        let mut cb = ChildListBuilder::new();
        ast.process_children(&mut cb);
        cb.children.iter().for_each(|c| {
            self.process_statement_child(c);
        });
        ast.get_ast()
    }

    pub fn visitAssignmentStatement(&self, ast: AssignmentStatement) -> AST
// """Rule (2)"""
    {
        ast.lhs = self.expr_trafo.visit(ast.lhs);
        ast.rhs = self.expr_trafo.visit(ast.rhs);
        let mut modvals = ast.modified_values.keys().collect();
        if CFG.lock().unwrap().user_config.opt_cache_circuit_outputs()
            && is_instance(&ast.lhs(), ASTType::IdentifierExpr)
            && is_instance(&ast.rhs(), ASTType::MemberAccessExpr)
        {
            //Skip invalidation if rhs is circuit output
            if is_instance(&ast.rhs().member, ASTType::HybridArgumentIdf)
                && ast.rhs().member.arg_type == HybridArgType::PubCircuitArg
            {
                modvals = modvals
                    .iter()
                    .filter_map(|mv| {
                        if mv.target != ast.lhs().target {
                            Some(mv)
                        } else {
                            None
                        }
                    })
                    .collect();
                let ridf = if is_instance(
                    &ast.rhs().member.corresponding_priv_expression,
                    ASTType::EncryptionExpression,
                ) {
                    ast.rhs()
                        .member
                        .corresponding_priv_expression
                        .expr
                        .idf
                        .clone()
                } else {
                    ast.rhs().member.corresponding_priv_expression.idf.clone()
                };
                assert!(is_instance(&ridf, ASTType::HybridArgumentIdf));
                self.gen
                    .unwrap()
                    ._remapper
                    .0
                    .remap(ast.lhs().target.idf, ridf);
            }
        }

        if self.gen.is_some()
        //Invalidate circuit value for assignment targets
        {
            for val in modvals {
                if val.key.is_none() {
                    self.gen.unwrap().invalidate_idf(val.target.idf);
                }
            }
        }
        ast.get_ast()
    }

    pub fn visitIfStatement(&self, ast: IfStatement) -> IfStatement
// """
    // Rule (6) + additional support for private conditions

    // If the condition is public, guard conditions are introduced for both branches if any of the branches contains private expressions.

    // If the condition is private, the whole if statement is inlined into the circuit. The only side-effects which are allowed
    // inside the branch bodies are assignment statements with an lhs@me. (anything else would leak private information).
    // The if statement will be replaced by an assignment statement where the lhs is a tuple of all locations which are written
    // in either branch and rhs is a tuple of the corresponding circuit outputs.
    // """
    {
        if ast.condition.annotated_type().is_public() {
            if contains_private_expr(Some(ast.then_branch.get_ast()))
                || contains_private_expr(ast.else_branch)
            {
                let before_if_state = self.gen.unwrap()._remapper.0.get_state();
                let guard_var = self.gen.unwrap().add_to_circuit_inputs(&mut ast.condition);
                ast.condition = guard_var.get_loc_expr(Some(ast.get_ast())).into();
                self.gen.unwrap().guarded(guard_var, true);
                {
                    ast.then_branch = self.visit(ast.then_branch.get_ast());
                    self.gen.unwrap()._remapper.0.set_state(before_if_state);
                }
                if ast.else_branch.is_some() {
                    self.gen.unwrap().guarded(guard_var, false);
                    {
                        ast.else_branch = self.visit(ast.else_branch.unwrap().get_ast());
                        self.gen.unwrap()._remapper.0.set_state(before_if_state);
                    }
                }

                //Invalidate values modified in either branch
                for val in ast.statement_base.ast_base.modified_values {
                    if val.key().is_none() {
                        self.gen.unwrap().invalidate_idf(val.target().idf);
                    }
                }
            } else {
                ast.condition = self.expr_trafo.visit(ast.condition.get_ast());
                ast.then_branch = self.visit(ast.then_branch.get_ast());
                if ast.else_branch.is_some() {
                    ast.else_branch = self.visit(ast.else_branch.unwrap().get_ast());
                }
            }
            ast
        } else {
            self.gen.unwrap().evaluate_stmt_in_circuit(ast)
        }
    }
    pub fn visitWhileStatement(&self, ast: WhileStatement) -> WhileStatement
//Loops must always be purely public
    {
        assert!(!contains_private_expr(ast.condition));
        assert!(!contains_private_expr(ast.body.get_ast()));
        ast
    }

    pub fn visitDoWhileStatement(&self, ast: DoWhileStatement) -> DoWhileStatement
//Loops must always be purely public
    {
        assert!(!contains_private_expr(Some(ast.condition.get_ast())));
        assert!(!contains_private_expr(Some(ast.body.get_ast())));
        ast
    }

    pub fn visitForStatement(&self, ast: ForStatement) -> ForStatement {
        if ast.init.is_some()
        //Init is the only part of a for loop which may contain private expressions
        {
            ast.init = self.visit(ast.init.unwrap().get_ast());
            ast.statement_base.pre_statements += ast.init.pre_statements;
        }
        assert!(!contains_private_expr(ast.condition.get_ast()));
        assert!(!ast.update.is_some() || !contains_private_expr(ast.update.map(|v| v.get_ast())));
        assert!(!contains_private_expr(Some(ast.body.get_ast()))); //OR fixed size loop -> static analysis can prove that loop terminates in fixed //iterations
        ast
    }

    pub fn visitContinueStatement(&self, ast: ContinueStatement) -> ContinueStatement {
        ast
    }

    pub fn visitBreakStatement(&self, ast: BreakStatement) -> BreakStatement {
        ast
    }

    pub fn visitReturnStatement(&self, ast: ReturnStatement) -> ReturnStatement
// """
    // Handle return statement.

    // If the function requires verification, the return statement is replaced by an assignment to a return variable.
    // (which will be returned at the very end of the function body, after any verification wrapper code).
    // Otherwise only the expression is transformed.
    // """
    {
        if ast.statement_base.function.requires_verification {
            if ast.expr == Expression::None {
                return ReturnStatement::default();
            }
            assert!(!self.gen.unwrap().has_return_var);
            self.gen.unwrap().has_return_var = true;
            let expr = self.expr_trafo.visit(ast.expr.get_ast());
            let ret_args = ast
                .statement_base
                .function
                .unwrap()
                .return_var_decls
                .iter()
                .map(|vd| {
                    let mut idf = IdentifierExpr::new(vd.idf.clone(), None);
                    idf.location_expr_base.target = vd;
                })
                .collect();
            let mut te = TupleExpr::new(ret_args).assign(if let AST::Expression(expr) = expr {
                expr
            } else {
                Expression::None
            });
            te.pre_statements = ast.statement_base.pre_statements;
            te
        } else {
            ast.expr = if let AST::Expression(expr) = self.expr_trafo.visit(ast.expr.get_ast()) {
                expr
            } else {
                Expression::None
            };
            ast
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
pub struct ZkayExpressionTransformer<V: TransformerVisitorEx> {
    gen: Option<Box<CircuitHelper<V>>>,
}

impl<V: TransformerVisitorEx> AstTransformerVisitor for ZkayExpressionTransformer<V> {
    fn default() -> Self {
        Self::new()
    }

    fn visit(&self, ast: AST) -> AST {
        // self._visit_internal(ast)
        AST::None
    }
    fn visitBlock(
        &self,
        ast: AST,
        guard_cond: Option<HybridArgumentIdf>,
        guard_val: Option<bool>,
    ) -> AST {
        // self.visit_children(ast)
        AST::None
    }
}
impl<V: TransformerVisitorEx> ZkayExpressionTransformer<V> {
    pub fn vv(current_generator: Option<Box<CircuitHelper<V>>>) -> V {
        Self::new(current_generator)
    }
    pub fn new(current_generator: Option<Box<CircuitHelper<V>>>) -> Self
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
            ast.to_expr(),
            &mut LocationExpr::IdentifierExpr(IdentifierExpr::new(
                IdentifierExprUnion::String(String::from("msg")),
                None,
            )),
            false,
        )
        .dot(IdentifierExprUnion::String(String::from("sender")))
        .as_type(AsTypeUnion::AnnotatedTypeName(
            AnnotatedTypeName::address_all(),
        ))
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
            ast,
            &mut self
                .visit((*ast.arr).get_ast())
                .index(self.visit((*ast.key).get_ast())),
            false,
        )
    }

    pub fn visitMemberAccessExpr(&self, ast: MemberAccessExpr) {
        self.visit_children(ast.get_ast());
    }

    pub fn visitTupleExpr(&self, ast: TupleExpr) {
        self.visit_children(ast.get_ast());
    }

    pub fn visitReclassifyExpr(&self, ast: ReclassifyExpr) -> LocationExpr
// """
    // Rule (11), trigger a boundary crossing.

    // The reclassified expression is evaluated in the circuit and its result is made available in solidity.
    // """
    {
        self.gen.unwrap().evaluate_expr_in_circuit(
            &mut ast.expr().unwrap(),
            ast.privacy().unwrap().privacy_annotation_label(),
            ast.annotated_type().homomorphism,
        )
    }

    pub fn visitBuiltinFunction(&self, ast: BuiltinFunction) -> BuiltinFunction {
        ast
    }

    pub fn visitFunctionCallExpr(&self, mut ast: FunctionCallExpr) -> FunctionCallExpr {
        if is_instance(&ast.func(), ASTType::BuiltinFunction) {
            if ast.func().is_private
            // """
            // Modified Rule (12) builtin functions with private operands and homomorphic operations on ciphertexts
            // are evaluated inside the circuit.

            // A private expression on its own (like an IdentifierExpr referring to a private variable) is not enough to trigger a
            // boundary crossing (assignment of private variables is a public operation).
            // """
            {
                let privacy_label = ast
                    .annotated_type()
                    .privacy_annotation
                    .privacy_annotation_label();
                return self.gen.unwrap().evaluate_expr_in_circuit(
                    &mut ast.to_expr(),
                    privacy_label,
                    ast.func().homomorphism,
                );
            } else
            // """
            // Rule (10) with additional short-circuit handling.

            // Builtin operations on public operands are normally left untransformed, but if the builtin function has
            // short-circuiting semantics, guard conditions must be added if any of the public operands contains
            // nested private expressions.
            // """
            //handle short-circuiting
            {
                let mut args = ast.args();
                if ast.func().has_shortcircuiting()
                    && args()[1..].iter().any(|arg| contains_private_expr(arg))
                {
                    let op = ast.func().op;
                    let guard_var = self.gen.unwrap().add_to_circuit_inputs(ast.args[0]);
                    args[0] = guard_var.get_loc_expr(Some(ast.get_ast()));
                    if op == "ite" {
                        args[1] = self.visit_guarded_expression(guard_var, true, args[1]);
                        args[2] = self.visit_guarded_expression(guard_var, false, args[2]);
                    } else if op == "||" {
                        args[1] = self.visit_guarded_expression(guard_var, false, args[1]);
                    } else if op == "&&" {
                        args[1] = self.visit_guarded_expression(guard_var, true, args[1]);
                    }
                    ast.set_args(args);
                    ast
                }

                return self.visit_children(ast);
            }
        } else if ast.is_cast()
        // """Casts are handled either in public or inside the circuit depending on the privacy of the casted expression."""
        {
            assert!(is_instance(&ast.func.target, ASTType::EnumDefinition));
            if ast.args[0].evaluate_privately {
                let privacy_label = ast
                    .annotated_type
                    .privacy_annotation
                    .privacy_annotation_label();
                return self.gen.unwrap().evaluate_expr_in_circuit(
                    ast,
                    privacy_label,
                    ast.annotated_type.homomorphism,
                );
            } else {
                return self.visit_children(ast);
            }
        } else
        // """
        // Handle normal function calls (outside private expression case).

        // The called functions are allowed to have side effects,
        // if the function does not require verification it can even be recursive.
        // """
        {
            assert!(is_instance(&ast.func(), ASTType::LocationExpr));
            ast = self.visit_children(ast);
            if ast.func().target.requires_verification_when_external
            //Reroute the function call to the corresponding internal function if the called function was split into external/internal.
            {
                if !is_instance(&ast.func(), ASTType::IdentifierExpr) {
                    unimplemented!();
                }
                ast.func.idf.name = CFG.lock().unwrap().get_internal_name(ast.func().target);
            }

            if ast.func().target.requires_verification
            //If the target function has an associated circuit, make this function"s circuit aware of the call.
            {
                self.gen.unwrap().call_function(ast);
            } else if ast.func().target.has_side_effects && self.gen.is_some()
            //Invalidate modified state variables for the current circuit
            {
                for val in ast.modified_values {
                    if val.key.is_none()
                        && is_instance(&val.target, ASTType::StateVariableDeclaration)
                    {
                        self.gen.unwrap().invalidate_idf(val.target.idf);
                    }
                }
            }

            //The call will be present as a normal function call in the output solidity code.
            ast
        }
    }
    pub fn visit_guarded_expression(
        &self,
        guard_var: HybridArgumentIdf,
        if_true: bool,
        expr: Expression,
    ) -> AST {
        let prelen = expr.statement().pre_statements.len();

        //Transform expression with guard condition in effect
        self.gen.unwrap().guarded(guard_var, if_true);
        let ret = self.visit(expr.get_ast());

        //If new pre statements were added, they must be guarded using an if statement in the public solidity code
        let new_pre_stmts = expr.statement().pre_statements[prelen..];
        if new_pre_stmts {
            let cond_expr = guard_var.get_loc_expr(None).into();
            if is_instance(&cond_expr, ASTType::BooleanLiteralExpr) {
                cond_expr = LocationExprUnion::BooleanLiteralExpr(BooleanLiteralExpr::new(
                    cond_expr.value == if_true,
                ));
            } else if !if_true {
                cond_expr = cond_expr.into().unop(String::from("!"));
            }
            expr.statement.pre_statements = expr.statement.pre_statements[..prelen]
                .iter()
                .chain([IfStatement::new(
                    cond_expr.into(),
                    Block::new(new_pre_stmts),
                    None,
                )])
                .collect();
        }
        ret
    }

    pub fn visitPrimitiveCastExpr(&self, ast: PrimitiveCastExpr) -> AST
// """Casts are handled either in public or inside the circuit depending on the privacy of the casted expression."""
    {
        if ast.expression_base.evaluate_privately {
            let privacy_label = ast
                .expression_base
                .annotated_type
                .privacy_annotation
                .privacy_annotation_label();
            self.gen.unwrap().evaluate_expr_in_circuit(
                &mut ast.to_expr(),
                privacy_label,
                ast.expression_base.annotated_type.homomorphism,
            )
        } else {
            self.visit_children(ast)
        }
    }

    pub fn visitExpression(&self, ast: Expression) {
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
pub struct ZkayCircuitTransformer<V: TransformerVisitorEx> {
    gen: Option<Box<CircuitHelper<V>>>,
}

impl<V: TransformerVisitorEx> AstTransformerVisitor for ZkayCircuitTransformer<V> {
    fn default() -> Self {
        Self::new(None)
    }

    fn visit(&self, ast: AST) -> AST {
        // self._visit_internal(ast)
        AST::None
    }
    fn visitBlock(
        &self,
        ast: AST,
        guard_cond: Option<HybridArgumentIdf>,
        guard_val: Option<bool>,
    ) -> AST {
        // self.visit_children(ast)
        AST::None
    }
}
impl<V: TransformerVisitorEx> ZkayCircuitTransformer<V> {
    pub fn vv(current_generator: Option<Box<CircuitHelper<V>>>) -> V {
        Self::new(current_generator)
    }
    pub fn new(current_generator: Option<Box<CircuitHelper<V>>>) -> Self {
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

    pub fn visitIndexExpr(&self, ast: IndexExpr) -> IndexExpr {
        self.transform_location(ast)
    }

    pub fn visitIdentifierExpr(&self, ast: IdentifierExpr) -> IdentifierExpr {
        if !is_instance(&*ast.idf, ASTType::HybridArgumentIdf)
        //If ast is not already transformed, get current SSA version
        {
            ast = self.gen.unwrap().get_remapped_idf_expr(ast);
        }
        if is_instance(&ast, ASTType::IdentifierExpr)
            && is_instance(&*ast.idf, ASTType::HybridArgumentIdf)
        //The current version of ast.idf is already in the circuit
        {
            assert!(ast.idf.arg_type() != HybridArgType::PubContractVal);
            ast
        } else
        //ast is not yet in the circuit -> move it in
        {
            self.transform_location(LocationExpr::IdentifierExpr(ast))
        }
    }

    pub fn transform_location(&self, loc: LocationExpr) -> LocationExpr
// """Rule (14), move location into the circuit."""
    {
        self.gen
            .unwrap()
            .add_to_circuit_inputs(&mut loc.to_expr())
            .get_idf_expr(&None)
    }

    pub fn visitReclassifyExpr(&self, ast: ReclassifyExpr) -> AST
// """Rule (15), boundary crossing if analysis determined that it is """
    {
        if ast.annotated_type().is_cipher()
        //We need a homomorphic ciphertext -> make sure the correct encryption of the value is available
        {
            let orig_type = ast.annotated_type().zkay_type;
            let orig_privacy = orig_type.privacy_annotation.privacy_annotation_label();
            let orig_homomorphism = orig_type.homomorphism;
            self.gen
                .unwrap()
                .evaluate_expr_in_circuit(&mut ast.expr().unwrap(), orig_privacy, orig_homomorphism)
                .get_ast()
        } else if ast.expr().evaluate_privately {
            self.visit(ast.expr().unwrap().get_ast())
        } else {
            assert!(ast.expr().annotated_type.is_public());
            self.gen
                .unwrap()
                .add_to_circuit_inputs(&mut ast.expr().unwrap())
                .get_idf_expr(&None)
                .get_ast()
        }
    }

    pub fn visitExpression(&self, ast: Expression) -> AST
// """Rule (16), other expressions don"t need special treatment."""
    {
        self.visit_children(ast.get_ast())
    }

    pub fn visitFunctionCallExpr(&self, ast: FunctionCallExpr) -> Expression {
        let t = &ast.annotated_type().type_name;

        //Constant folding for literal types
        if is_instance(t, ASTType::BooleanLiteralType) {
            return replace_expr(
                ast.to_expr(),
                &mut BooleanLiteralExpr::new(t.value).to_expr(),
                false,
            );
        } else if is_instance(t, ASTType::NumberLiteralType) {
            return replace_expr(
                ast.to_expr(),
                &mut NumberLiteralExpr::new(t.value, false).to_expr(),
                false,
            );
        }

        if is_instance(&ast.func().unwrap(), ASTType::BuiltinFunction) {
            if ast.func().unwrap().homomorphism() != Some(Homomorphism::non_homomorphic())
            //To perform homomorphic operations, we require the recipient"s public key
            {
                let crypto_params = CFG
                    .lock()
                    .unwrap()
                    .user_config
                    .get_crypto_params(ast.func().homomorphism);
                let recipient = ast
                    .annotated_type()
                    .zkay_type
                    .privacy_annotation
                    .privacy_annotation_label();
                ast.public_key = self.gen.unwrap()._require_public_key_for_label_at(
                    ast.statement,
                    recipient,
                    crypto_params,
                );

                if ast.func().op == "*"
                //special case: private scalar multiplication using additive homomorphism
                //TODO ugly hack below removes ReclassifyExpr
                {
                    let mut new_args = vec![];
                    for mut arg in ast.args() {
                        if is_instance(&arg, ASTType::ReclassifyExpr) {
                            arg = arg.expr;
                            ast.func.rerand_using =
                                self.gen.unwrap().get_randomness_for_rerand(ast.to_expr());
                        //result requires re-randomization
                        } else if arg.annotated_type.is_private() {
                            arg.annotated_type = AnnotatedTypeName::cipher_type(
                                arg.annotated_type,
                                ast.func.homomorphism,
                            );
                        }
                        new_args.push(arg);
                    }
                    ast.args = new_args;
                } else
                //We require all non-public arguments to be present as ciphertexts
                {
                    for arg in ast.args.iter_mut() {
                        if arg.annotated_type.is_private() {
                            arg.annotated_type = AnnotatedTypeName::cipher_type(
                                arg.annotated_type,
                                ast.func.homomorphism,
                            );
                        }
                    }
                }
            }

            //Builtin functions are supported natively by the circuit
            return self.visit_children(ast);
        }

        let fdef = &ast.func.target;
        assert!(fdef.is_function);
        assert!(fdef.return_parameters);
        assert!(fdef.has_static_body);

        //Function call inside private expression -> entire body will be inlined into circuit.
        //Function must not have side-effects (only pure and view is allowed) and cannot have a nonstatic body (i.e. recursion)
        return self.gen.unwrap().inline_function_call_into_circuit(ast);
    }

    pub fn visitReturnStatement(&self, ast: ReturnStatement) {
        self.gen.unwrap().add_return_stmt_to_circuit(ast)
    }

    pub fn visitAssignmentStatement(&self, ast: AssignmentStatement) {
        self.gen.unwrap().add_assignment_to_circuit(ast)
    }

    pub fn visitVariableDeclarationStatement(&self, ast: VariableDeclarationStatement) {
        self.gen.unwrap().add_var_decl_to_circuit(ast)
    }

    pub fn visitIfStatement(&self, ast: &mut IfStatement) {
        self.gen.unwrap().add_if_statement_to_circuit(ast)
    }

    pub fn visitBlock(
        &self,
        ast: Block,
        guard_cond: Option<HybridArgumentIdf>,
        guard_val: Option<bool>,
    ) {
        self.gen
            .unwrap()
            .add_block_to_circuit(ast, guard_cond, guard_val)
    }

    pub fn visitStatement(&self, ast: Statement)
    // """Fail if statement type was not handled."""
    // raise NotImplementedError("Unsupported statement")
    {
        unimplemented!("Unsupported statement")
    }
}
