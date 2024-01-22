// """
// This module defines zkay->solidity transformers for the smaller contract elements (statements, expressions, state variables).
// """

use crate::compiler::privacy::circuit_generation::circuit_helper::CircuitHelper;
use crate::compiler::solidity::fake_solidity_generator::{ID_PATTERN, WS_PATTERN};
use crate::config::CFG;
use crate::zkay_ast::analysis::contains_private_checker::contains_private_expr;
use crate::zkay_ast::ast::{
    is_instance, ASTType, AnnotatedTypeName, AssignmentStatement, BlankLine, Block,
    BooleanLiteralExpr, BooleanLiteralType, BreakStatement, BuiltinFunction, Comment,
    ContinueStatement, DoWhileStatement, EncryptionExpression, EnumDefinition, Expression,
    ForStatement, FunctionCallExpr, HybridArgType, HybridArgumentIdf, IdentifierExpr, IfStatement,
    IndexExpr, LiteralExpr, LocationExpr, Mapping, MeExpr, MemberAccessExpr, NumberLiteralExpr,
    NumberLiteralType, Parameter, PrimitiveCastExpr, ReclassifyExpr, ReturnStatement,
    SimpleStatement, StateVariableDeclaration, Statement, StatementList, TupleExpr, TypeName,
    VariableDeclaration, VariableDeclarationStatement, WhileStatement, AST,
};
use crate::zkay_ast::homomorphism::Homomorphism;
use crate::zkay_ast::visitor::deep_copy::replace_expr;
use crate::zkay_ast::visitor::transformer_visitor::AstTransformerVisitor;
use regex::Regex;
use regex::RegexSetBuilder;

// class ZkayVarDeclTransformer(AstTransformerVisitor)
// """
// Transformer for types, which was left out in the paper.

// This removes all privacy labels and converts the types of non-public variables (not @all)
// to cipher_type.
// """

// pub fn __init__(self)
//     super().__init__()
//     self.expr_trafo = ZkayExpressionTransformer(None)
pub struct ZkayVarDeclTransformer<
    V: Clone
        + std::marker::Sync
        + crate::zkay_ast::visitor::transformer_visitor::AstTransformerVisitor,
> {
    expr_trafo: Option<ZkayExpressionTransformer<V>>,
}
impl<
        V: Clone
            + std::marker::Sync
            + crate::zkay_ast::visitor::transformer_visitor::AstTransformerVisitor,
    > AstTransformerVisitor for ZkayVarDeclTransformer<V>
{
    fn default() -> Self {
        Self::new(false)
    }

    fn visit(self, ast: AST) -> AST {
        self._visit_internal(ast)
    }
    fn visitBlock(
        self,
        ast: AST,
        guard_cond: Option<HybridArgumentIdf>,
        guard_val: Option<bool>,
    ) -> AST {
        self.visit_children(ast)
    }
}
impl<
        V: Clone
            + std::marker::Sync
            + crate::zkay_ast::visitor::transformer_visitor::AstTransformerVisitor,
    > ZkayVarDeclTransformer<V>
{
    pub fn new() -> Self {
        Self { expr_trafo: None }
    }

    pub fn visitAnnotatedTypeName(self, ast: AnnotatedTypeName) {
        let t = if ast.is_private() {
            TypeName::cipher_type(ast, ast.homomorphism)
        } else {
            self.visit(ast.type_name.clone())
        };
        AnnotatedTypeName::new(t)
    }

    pub fn visitVariableDeclaration(self, ast: VariableDeclaration) {
        if ast.annotated_type.is_private() {
            ast.identifier_declaration_base.storage_location = "memory";
        }
        return self.visit_children(ast);
    }

    pub fn visitParameter(self, mut ast: Parameter) -> AST {
        ast = self.visit_children(ast);
        if !ast
            .identifier_declaration_base
            .annotated_type
            .type_name
            .is_primitive_type()
        {
            ast.identifier_declaration_base.storage_location = "memory";
        }
        ast.get_ast()
    }

    pub fn visitStateVariableDeclaration(self, ast: StateVariableDeclaration) {
        ast.identifier_declaration_base.keywords = ast
            .identifier_declaration_base
            .keywords
            .iter()
            .filter_map(|k| if k != "public" { Some(k) } else { None })
            .collect();
        //make sure every state var gets a public getter (required for simulation)
        ast.identifier_declaration_base.keywords.push("public");
        ast.expr = self.expr_trafo.visit(ast.expr);
        return self.visit_children(ast);
    }

    pub fn visitMapping(self, ast: Mapping) {
        if ast.key_label.is_some() {
            ast.key_label = ast.key_label.name;
        }
        return self.visit_children(ast);
    }
}
// class ZkayStatementTransformer(AstTransformerVisitor)
// """Corresponds to T from paper, (with additional handling of return statement and loops)."""
pub struct ZkayStatementTransformer<
    V: Clone
        + std::marker::Sync
        + crate::zkay_ast::visitor::transformer_visitor::AstTransformerVisitor,
> {
    gen: CircuitHelper<V>,
    expr_trafo: ZkayExpressionTransformer<V>,
    var_decl_trafo: ZkayVarDeclTransformer<V>,
}
impl<
        V: Clone
            + std::marker::Sync
            + crate::zkay_ast::visitor::transformer_visitor::AstTransformerVisitor,
    > ZkayStatementTransformer<V>
{
    // pub fn __init__(self, current_gen: CircuitHelper)
    //     super().__init__()
    //     self.gen = current_gen
    //     self.expr_trafo = ZkayExpressionTransformer(self.gen)
    //     self.var_decl_trafo = ZkayVarDeclTransformer()
    pub fn new(current_gen: CircuitHelper<V>) -> Self {
        Self {
            gen: current_gen.clone(),
            expr_trafo: ZkayExpressionTransformer::new(current_gen),
            var_decl_trafo: ZkayVarDeclTransformer::new(),
        }
    }
    pub fn visitStatementList(self, ast: StatementList)
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
        for (idx, stmt) in ast.statements.iter().enumerate() {
            let old_code = stmt.code();
            let transformed_stmt = self.visit(stmt);
            if transformed_stmt.is_none() {
                continue;
            }

            let old_code_wo_annotations = Regex::new(r"(?=\b)me(?=\b)").unwrap().replace_all(
                Regex::new(r"@{WS_PATTERN}*{ID_PATTERN}")
                    .unwrap()
                    .replace_all(old_code, ""),
                "msg.sender",
            );
            let new_code_wo_annotation_comments = Regex::new(r"/\*.*?\*/")
                .unwrap()
                .replace_all(transformed_stmt.code(), "");
            if old_code_wo_annotations == new_code_wo_annotation_comments {
                new_statements.push(transformed_stmt)
            } else {
                new_statements.extend(Comment::comment_wrap_block(
                    old_code,
                    transformed_stmt
                        .pre_statements
                        .iter()
                        .chain([transformed_stmt])
                        .collect(),
                ));
            }
        }
        if new_statements && is_instance(&new_statements[-1], ASTType::BlankLine) {
            new_statements.pop();
        }
        ast.statements = new_statements;
        ast
    }

    pub fn process_statement_child(self, child: AST)
    // """Default statement child handling. Expressions and declarations are visited by the corresponding transformers."""
    {
        if is_instance(&child, ASTType::Expression) {
            return self.expr_trafo.visit(child);
        } else if child.is_some() {
            assert!(is_instance(&child, ASTType::VariableDeclaration));
            return self.var_decl_trafo.visit(child);
        }
    }

    pub fn visitStatement(self, ast: Statement)
    // """
    // Rules (3), (4)

    // This is for all the statements where the statements themselves remain untouched and only the children are altered.
    // """
    {
        assert!(
            is_instance(&ast, ASTType::SimpleStatement)
                || is_instance(&ast, ASTType::VariableDeclarationStatement)
        );
        ast.process_children(self.process_statement_child);
        ast
    }

    pub fn visitAssignmentStatement(self, ast: AssignmentStatement)
    // """Rule (2)"""
    {
        ast.lhs = self.expr_trafo.visit(ast.lhs);
        ast.rhs = self.expr_trafo.visit(ast.rhs);
        let mut modvals = ast.modified_values.keys().collect();
        if CFG.lock().unwrap().opt_cache_circuit_outputs
            && is_instance(&ast.lhs, ASTType::IdentifierExpr)
            && is_instance(&ast.rhs, ASTType::MemberAccessExpr)
        {
            //Skip invalidation if rhs is circuit output
            if is_instance(&ast.rhs.member, ASTType::HybridArgumentIdf)
                && ast.rhs.member.arg_type == HybridArgType::PubCircuitArg
            {
                modvals = modvals
                    .iter()
                    .filter_map(|mv| {
                        if mv.target != ast.lhs.target {
                            Some(mv)
                        } else {
                            None
                        }
                    })
                    .collect();
                let ridf = if is_instance(
                    &ast.rhs.member.corresponding_priv_expression,
                    ASTType::EncryptionExpression,
                ) {
                    ast.rhs
                        .member
                        .corresponding_priv_expression
                        .expr
                        .idf
                        .clone()
                } else {
                    ast.rhs.member.corresponding_priv_expression.idf.clone()
                };
                assert!(is_instance(&ridf, ASTType::HybridArgumentIdf));
                self.gen._remapper.remap(ast.lhs.target.idf, ridf);
            }
        }

        if self.gen.is_some()
        //Invalidate circuit value for assignment targets
        {
            for val in modvals {
                if val.key.is_none() {
                    self.gen.invalidate_idf(val.target.idf);
                }
            }
        }
        ast
    }

    pub fn visitIfStatement(self, ast: IfStatement)
    // """
    // Rule (6) + additional support for private conditions

    // If the condition is public, guard conditions are introduced for both branches if any of the branches contains private expressions.

    // If the condition is private, the whole if statement is inlined into the circuit. The only side-effects which are allowed
    // inside the branch bodies are assignment statements with an lhs@me. (anything else would leak private information).
    // The if statement will be replaced by an assignment statement where the lhs is a tuple of all locations which are written
    // in either branch and rhs is a tuple of the corresponding circuit outputs.
    // """
    {
        if ast.condition.annotated_type.is_public() {
            if contains_private_expr(ast.then_branch) || contains_private_expr(ast.else_branch) {
                let before_if_state = self.gen._remapper.get_state();
                let guard_var = self.gen.add_to_circuit_inputs(ast.condition);
                ast.condition = guard_var.get_loc_expr(ast);
                self.gen.guarded(guard_var, true);
                {
                    ast.then_branch = self.visit(ast.then_branch);
                    self.gen._remapper.set_state(before_if_state);
                }
                if ast.else_branch.is_some() {
                    self.gen.guarded(guard_var, false);
                    {
                        ast.else_branch = self.visit(ast.else_branch);
                        self.gen._remapper.set_state(before_if_state);
                    }
                }

                //Invalidate values modified in either branch
                for val in ast.modified_values {
                    if val.key.is_none() {
                        self.gen.invalidate_idf(val.target.idf);
                    }
                }
            } else {
                ast.condition = self.expr_trafo.visit(ast.condition);
                ast.then_branch = self.visit(ast.then_branch);
                if ast.else_branch.is_some() {
                    ast.else_branch = self.visit(ast.else_branch);
                }
            }
            ast
        } else {
            self.gen.evaluate_stmt_in_circuit(ast)
        }
    }
    pub fn visitWhileStatement(self, ast: WhileStatement)
    //Loops must always be purely public
    {
        assert!(!contains_private_expr(ast.condition));
        assert!(!contains_private_expr(ast.body));
        ast
    }

    pub fn visitDoWhileStatement(self, ast: DoWhileStatement)
    //Loops must always be purely public
    {
        assert!(!contains_private_expr(ast.condition));
        assert!(!contains_private_expr(ast.body));
        ast
    }

    pub fn visitForStatement(self, ast: ForStatement) {
        if ast.init.is_some()
        //Init is the only part of a for loop which may contain private expressions
        {
            ast.init = self.visit(ast.init);
            ast.pre_statements += ast.init.pre_statements;
        }
        assert!(!contains_private_expr(ast.condition));
        assert!(!ast.update || !contains_private_expr(ast.update));
        assert!(!contains_private_expr(ast.body)); //OR fixed size loop -> static analysis can prove that loop terminates in fixed //iterations
        ast
    }

    pub fn visitContinueStatement(self, ast: ContinueStatement) {
        ast
    }

    pub fn visitBreakStatement(self, ast: BreakStatement) {
        ast
    }

    pub fn visitReturnStatement(self, ast: ReturnStatement)
    // """
    // Handle return statement.

    // If the function requires verification, the return statement is replaced by an assignment to a return variable.
    // (which will be returned at the very end of the function body, after any verification wrapper code).
    // Otherwise only the expression is transformed.
    // """
    {
        if ast.function.requires_verification {
            if ast.expr.is_none() {
                return None;
            }
            assert!(!self.gen.has_return_var);
            self.gen.has_return_var = true;
            let expr = self.expr_trafo.visit(ast.expr);
            let ret_args = ast
                .function
                .return_var_decls
                .iter()
                .map(|vd| {
                    let mut idf = IdentifierExpr::new(vd.idf.clone());
                    idf.target = vd;
                })
                .collect();
            let mut te = TupleExpr::new(ret_args).assign(expr);
            te.pre_statements = ast.pre_statements;
            te
        } else {
            ast.expr = self.expr_trafo.visit(ast.expr);
            ast
        }
    }

    pub fn visitExpression(self, ast: Expression)
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
pub struct ZkayExpressionTransformer<
    V: Clone
        + std::marker::Sync
        + crate::zkay_ast::visitor::transformer_visitor::AstTransformerVisitor,
> {
    gen: Option<CircuitHelper<V>>,
}
impl<
        V: Clone
            + std::marker::Sync
            + crate::zkay_ast::visitor::transformer_visitor::AstTransformerVisitor,
    > ZkayExpressionTransformer<V>
{
    pub fn new(current_generator: Option<CircuitHelper<V>>) -> Self
// super().__init__()
        // self.gen = current_generator
    {
        Self {
            gen: current_generator,
        }
    }

    // @staticmethod
    pub fn visitMeExpr(ast: MeExpr)
    // """Replace me with msg.sender."""
    {
        replace_expr(ast, IdentifierExpr::new("msg").dot("sender"))
            .as_type(AnnotatedTypeName::address_all())
    }

    pub fn visitLiteralExpr(self, ast: LiteralExpr)
    // """Rule (7), don"t modify constants."""
    {
        ast
    }

    pub fn visitIdentifierExpr(self, ast: IdentifierExpr)
    // """Rule (8), don"t modify identifiers."""
    {
        ast
    }

    pub fn visitIndexExpr(self, ast: IndexExpr)
    // """Rule (9), transform location and index expressions separately."""
    {
        replace_expr(ast, self.visit(ast.arr).index(self.visit(ast.key)))
    }

    pub fn visitMemberAccessExpr(self, ast: MemberAccessExpr) {
        self.visit_children(ast)
    }

    pub fn visitTupleExpr(self, ast: TupleExpr) {
        self.visit_children(ast)
    }

    pub fn visitReclassifyExpr(self, ast: ReclassifyExpr)
    // """
    // Rule (11), trigger a boundary crossing.

    // The reclassified expression is evaluated in the circuit and its result is made available in solidity.
    // """
    {
        self.gen.evaluate_expr_in_circuit(
            ast.expr,
            ast.privacy.privacy_annotation_label(),
            ast.annotated_type.homomorphism,
        )
    }

    pub fn visitBuiltinFunction(self, ast: BuiltinFunction) {
        ast
    }

    pub fn visitFunctionCallExpr(self, ast: FunctionCallExpr) {
        if is_instance(&ast.func, ASTType::BuiltinFunction) {
            if ast.func.is_private
            // """
            // Modified Rule (12) builtin functions with private operands and homomorphic operations on ciphertexts
            // are evaluated inside the circuit.

            // A private expression on its own (like an IdentifierExpr referring to a private variable) is not enough to trigger a
            // boundary crossing (assignment of private variables is a public operation).
            // """
            {
                let privacy_label = ast
                    .annotated_type
                    .privacy_annotation
                    .privacy_annotation_label();
                return self.gen.evaluate_expr_in_circuit(
                    ast,
                    privacy_label,
                    ast.func.homomorphism,
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
                if ast.func.has_shortcircuiting()
                    && ast.args[1..].iter().any(|arg| contains_private_expr(arg))
                {
                    let op = ast.func.op;
                    let guard_var = self.gen.add_to_circuit_inputs(ast.args[0]);
                    ast.args[0] = guard_var.get_loc_expr(ast);
                    if op == "ite" {
                        ast.args[1] = self.visit_guarded_expression(guard_var, true, ast.args[1]);
                        ast.args[2] = self.visit_guarded_expression(guard_var, false, ast.args[2]);
                    } else if op == "||" {
                        ast.args[1] = self.visit_guarded_expression(guard_var, false, ast.args[1]);
                    } else if op == "&&" {
                        ast.args[1] = self.visit_guarded_expression(guard_var, true, ast.args[1]);
                    }
                    ast
                }

                return self.visit_children(ast);
            }
        } else if ast.is_cast
        // """Casts are handled either in public or inside the circuit depending on the privacy of the casted expression."""
        {
            assert!(is_instance(&ast.func.target, ASTType::EnumDefinition));
            if ast.args[0].evaluate_privately {
                let privacy_label = ast
                    .annotated_type
                    .privacy_annotation
                    .privacy_annotation_label();
                return self.gen.evaluate_expr_in_circuit(
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
            assert!(is_instance(&ast.func, ASTType::LocationExpr));
            ast = self.visit_children(ast);
            if ast.func.target.requires_verification_when_external
            //Reroute the function call to the corresponding internal function if the called function was split into external/internal.
            {
                if !is_instance(&ast.func, ASTType::IdentifierExpr) {
                    unimplemented!();
                }
                ast.func.idf.name = CFG.lock().unwrap().get_internal_name(ast.func.target);
            }

            if ast.func.target.requires_verification
            //If the target function has an associated circuit, make this function"s circuit aware of the call.
            {
                self.gen.call_function(ast);
            } else if ast.func.target.has_side_effects && self.gen.is_some()
            //Invalidate modified state variables for the current circuit
            {
                for val in ast.modified_values {
                    if val.key.is_none()
                        && is_instance(&val.target, ASTType::StateVariableDeclaration)
                    {
                        self.gen.invalidate_idf(val.target.idf);
                    }
                }
            }

            //The call will be present as a normal function call in the output solidity code.
            ast
        }
    }
    pub fn visit_guarded_expression(
        self,
        guard_var: HybridArgumentIdf,
        if_true: bool,
        expr: Expression,
    ) {
        let prelen = expr.statement.pre_statements.len();

        //Transform expression with guard condition in effect
        self.gen.guarded(guard_var, if_true);
        let ret = self.visit(expr);

        //If new pre statements were added, they must be guarded using an if statement in the public solidity code
        let new_pre_stmts = expr.statement.pre_statements[prelen..];
        if new_pre_stmts {
            let cond_expr = guard_var.get_loc_expr();
            if is_instance(&cond_expr, ASTType::BooleanLiteralExpr) {
                cond_expr = BooleanLiteralExpr::new(cond_expr.value == if_true);
            } else if !if_true {
                cond_expr = cond_expr.unop("!");
            }
            expr.statement.pre_statements = expr.statement.pre_statements[..prelen]
                .iter()
                .chain([IfStatement::new(cond_expr, Block::New(new_pre_stmts), None)])
                .collect();
        }
        ret
    }

    pub fn visitPrimitiveCastExpr(self, ast: PrimitiveCastExpr)
    // """Casts are handled either in public or inside the circuit depending on the privacy of the casted expression."""
    {
        if ast.evaluate_privately {
            let privacy_label = ast
                .annotated_type
                .privacy_annotation
                .privacy_annotation_label();
            self.gen
                .evaluate_expr_in_circuit(ast, privacy_label, ast.annotated_type.homomorphism)
        } else {
            self.visit_children(ast)
        }
    }

    pub fn visitExpression(self, ast: Expression) {
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
pub struct ZkayCircuitTransformer<
    V: Clone
        + std::marker::Sync
        + crate::zkay_ast::visitor::transformer_visitor::AstTransformerVisitor,
> {
    gen: CircuitHelper<V>,
}

impl<
        V: Clone
            + std::marker::Sync
            + crate::zkay_ast::visitor::transformer_visitor::AstTransformerVisitor,
    > ZkayCircuitTransformer<V>
{
    pub fn new(current_generator: CircuitHelper<V>) -> Self {
        Self {
            gen: current_generator,
        }
    }
    // super().__init__()
    // self.gen = current_generator

    pub fn visitLiteralExpr(self, ast: LiteralExpr)
    // """Rule (13), don"t modify constants."""
    {
        ast
    }

    pub fn visitIndexExpr(self, ast: IndexExpr) {
        self.transform_location(ast)
    }

    pub fn visitIdentifierExpr(self, ast: IdentifierExpr) {
        if !is_instance(&ast.idf, ASTType::HybridArgumentIdf)
        //If ast is not already transformed, get current SSA version
        {
            ast = self.gen.get_remapped_idf_expr(ast);
        }
        if is_instance(&ast, ASTType::IdentifierExpr)
            && is_instance(&ast.idf, ASTType::HybridArgumentIdf)
        //The current version of ast.idf is already in the circuit
        {
            assert!(ast.idf.arg_type != HybridArgType::PubContractVal);
            ast
        } else
        //ast is not yet in the circuit -> move it in
        {
            self.transform_location(ast)
        }
    }

    pub fn transform_location(self, loc: LocationExpr)
    // """Rule (14), move location into the circuit."""
    {
        self.gen.add_to_circuit_inputs(loc).get_idf_expr(&None)
    }

    pub fn visitReclassifyExpr(self, ast: ReclassifyExpr)
    // """Rule (15), boundary crossing if analysis determined that it is """
    {
        if ast.annotated_type.is_cipher()
        //We need a homomorphic ciphertext -> make sure the correct encryption of the value is available
        {
            let orig_type = ast.annotated_type.zkay_type;
            let orig_privacy = orig_type.privacy_annotation.privacy_annotation_label();
            let orig_homomorphism = orig_type.homomorphism;
            self.gen
                .evaluate_expr_in_circuit(ast.expr, orig_privacy, orig_homomorphism)
        } else if ast.expr.evaluate_privately {
            self.visit(ast.expr)
        } else {
            assert!(ast.expr.annotated_type.is_public());
            self.gen.add_to_circuit_inputs(ast.expr).get_idf_expr(&None)
        }
    }

    pub fn visitExpression(self, ast: Expression)
    // """Rule (16), other expressions don"t need special treatment."""
    {
        self.visit_children(ast)
    }

    pub fn visitFunctionCallExpr(self, ast: FunctionCallExpr) {
        let t = &ast.annotated_type.type_name;

        //Constant folding for literal types
        if is_instance(t, ASTType::BooleanLiteralType) {
            return replace_expr(ast, BooleanLiteralExpr::new(t.value));
        } else if is_instance(t, ASTType::NumberLiteralType) {
            return replace_expr(ast, NumberLiteralExpr::new(t.value));
        }

        if is_instance(&ast.func, ASTType::BuiltinFunction) {
            if ast.func.homomorphism != Homomorphism::non_homomorphic()
            //To perform homomorphic operations, we require the recipient"s public key
            {
                let crypto_params = CFG.lock().unwrap().get_crypto_params(ast.func.homomorphism);
                let recipient = ast
                    .annotated_type
                    .zkay_type
                    .privacy_annotation
                    .privacy_annotation_label();
                ast.public_key = self.gen._require_public_key_for_label_at(
                    ast.statement,
                    recipient,
                    crypto_params,
                );

                if ast.func.op == "*"
                //special case: private scalar multiplication using additive homomorphism
                //TODO ugly hack below removes ReclassifyExpr
                {
                    let mut new_args = vec![];
                    for mut arg in ast.args.drain(..) {
                        if is_instance(&arg, ASTType::ReclassifyExpr) {
                            arg = arg.expr;
                            ast.func.rerand_using = self.gen.get_randomness_for_rerand(ast);
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
        return self.gen.inline_function_call_into_circuit(ast);
    }

    pub fn visitReturnStatement(self, ast: ReturnStatement) {
        self.gen.add_return_stmt_to_circuit(ast)
    }

    pub fn visitAssignmentStatement(self, ast: AssignmentStatement) {
        self.gen.add_assignment_to_circuit(ast)
    }

    pub fn visitVariableDeclarationStatement(self, ast: VariableDeclarationStatement) {
        self.gen.add_var_decl_to_circuit(ast)
    }

    pub fn visitIfStatement(self, ast: IfStatement) {
        self.gen.add_if_statement_to_circuit(ast)
    }

    pub fn visitBlock(
        self,
        ast: Block,
        guard_cond: Option<HybridArgumentIdf>,
        guard_val: Option<bool>,
    ) {
        self.gen.add_block_to_circuit(ast, guard_cond, guard_val)
    }

    pub fn visitStatement(self, ast: Statement)
    // """Fail if statement type was not handled."""
    // raise NotImplementedError("Unsupported statement")
    {
        unimplemented!("Unsupported statement")
    }
}
