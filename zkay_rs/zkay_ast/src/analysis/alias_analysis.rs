#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

use crate::analysis::partition_state::PartitionState;
use crate::analysis::side_effects::has_side_effects;
use crate::ast::{
    is_instance, ASTBaseProperty, ASTType, AllExpr, AssignmentStatement,
    AssignmentStatementBaseProperty, Block, BreakStatement, BuiltinFunction,
    ConstructorOrFunctionDefinition, ContinueStatement, DoWhileStatement, ExpressionStatement,
    ForStatement, FunctionCallExpr, FunctionCallExprBaseProperty,
    IdentifierDeclarationBaseProperty, IfStatement, IntoAST, IntoExpression, LocationExpr, MeExpr,
    RequireStatement, ReturnStatement, Statement, StatementBaseMutRef, StatementBaseProperty,
    StatementBaseRef, StatementList, StatementListBaseProperty, TupleExpr,
    VariableDeclarationStatement, WhileStatement, AST,
};
use crate::visitor::visitor::{AstVisitorBase, AstVisitorBaseRef, AstVisitorMut};
use zkay_derive::ASTVisitorBaseRefImpl;
pub fn alias_analysis(ast: &mut AST) {
    let mut v = AliasAnalysisVisitor::new(false);
    v.cond_analyzer.visit(ast);
}
#[derive(ASTVisitorBaseRefImpl)]
struct AliasAnalysisVisitor {
    pub ast_visitor_base: AstVisitorBase,
    pub cond_analyzer: GuardConditionAnalyzer,
}
// class AliasAnalysisVisitor(AstVisitorMut)

impl AstVisitorMut for AliasAnalysisVisitor {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}
    fn has_attr(&self, name: &ASTType) -> bool {
        &ASTType::SourceUnit == name
            || &ASTType::ContractDefinition == name
            || &ASTType::ConstructorOrFunctionDefinition == name
            || &ASTType::StructDefinition == name
            || &ASTType::EnumDefinition == name
            || &ASTType::VariableDeclaration == name
            || &ASTType::StatementListBase == name
            || &ASTType::SimpleStatementBase == name
            || &ASTType::ForStatement == name
            || &ASTType::Mapping == name
    }
    fn get_attr(&mut self, name: &ASTType, ast: &mut AST) -> Self::Return {
        match name {
            ASTType::ConstructorOrFunctionDefinition => self.visitConstructorOrFunctionDefinition(
                ast.try_as_namespace_definition_mut()
                    .unwrap()
                    .try_as_constructor_or_function_definition_mut()
                    .unwrap(),
            ),
            ASTType::StatementListBase => self.visitStatementList(
                ast.try_as_statement_mut()
                    .unwrap()
                    .try_as_statement_list_mut()
                    .unwrap(),
            ),
            ASTType::Block => self.visitBlock(
                ast.try_as_statement_mut()
                    .unwrap()
                    .try_as_statement_list_mut()
                    .unwrap()
                    .try_as_block_mut()
                    .unwrap(),
            ),
            ASTType::IfStatement => self.visitIfStatement(
                ast.try_as_statement_mut()
                    .unwrap()
                    .try_as_if_statement_mut()
                    .unwrap(),
            ),

            ASTType::WhileStatement => self.visitWhileStatement(
                ast.try_as_statement_mut()
                    .unwrap()
                    .try_as_while_statement_mut()
                    .unwrap(),
            ),
            ASTType::DoWhileStatement => self.visitDoWhileStatement(
                ast.try_as_statement_mut()
                    .unwrap()
                    .try_as_do_while_statement_mut()
                    .unwrap(),
            ),
            ASTType::ForStatement => self.visitForStatement(
                ast.try_as_statement_mut()
                    .unwrap()
                    .try_as_for_statement_mut()
                    .unwrap(),
            ),
            ASTType::VariableDeclarationStatement => self.visitVariableDeclarationStatement(
                ast.try_as_statement_mut()
                    .unwrap()
                    .try_as_simple_statement_mut()
                    .unwrap()
                    .try_as_variable_declaration_statement_mut()
                    .unwrap(),
            ),
            ASTType::RequireStatement => self.visitRequireStatement(
                ast.try_as_statement_mut()
                    .unwrap()
                    .try_as_simple_statement_mut()
                    .unwrap()
                    .try_as_require_statement_mut()
                    .unwrap(),
            ),

            ASTType::AssignmentStatementBase => self.visitAssignmentStatement(
                ast.try_as_statement_mut()
                    .unwrap()
                    .try_as_simple_statement_mut()
                    .unwrap()
                    .try_as_assignment_statement_mut()
                    .unwrap(),
            ),

            ASTType::ExpressionStatement => self.visitExpressionStatement(
                ast.try_as_statement_mut()
                    .unwrap()
                    .try_as_simple_statement_mut()
                    .unwrap()
                    .try_as_expression_statement_mut()
                    .unwrap(),
            ),
            ASTType::ReturnStatement => self.visitReturnStatement(
                ast.try_as_statement_mut()
                    .unwrap()
                    .try_as_return_statement_mut()
                    .unwrap(),
            ),
            ASTType::ContinueStatement => self.visitContinueStatement(
                ast.try_as_statement_mut()
                    .unwrap()
                    .try_as_continue_statement_mut()
                    .unwrap(),
            ),
            ASTType::BreakStatement => self.visitBreakStatement(
                ast.try_as_statement_mut()
                    .unwrap()
                    .try_as_break_statement_mut()
                    .unwrap(),
            ),
            ASTType::StatementBase => self.visitStatement(ast),
            _ => {}
        }
    }
}
impl AliasAnalysisVisitor {
    // pub fn __init__(&self, log=False)
    //     super().__init__("node-or-children", log)
    //     self.cond_analyzer = GuardConditionAnalyzer()
    pub fn new(log: bool) -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", log),
            cond_analyzer: GuardConditionAnalyzer::new(log),
        }
    }
    pub fn visitConstructorOrFunctionDefinition(
        &mut self,
        ast: &mut ConstructorOrFunctionDefinition,
    ) {
        let mut s: PartitionState<AST> = PartitionState::new();
        s.insert(
            MeExpr::new()
                .to_expr()
                .privacy_annotation_label()
                .unwrap()
                .into(),
        );
        s.insert(
            AllExpr::new()
                .to_expr()
                .privacy_annotation_label()
                .unwrap()
                .into(),
        );
        for d in &ast.parent.as_ref().unwrap().state_variable_declarations {
            s.insert(
                d.try_as_identifier_declaration_ref()
                    .unwrap()
                    .try_as_state_variable_declaration_ref()
                    .unwrap()
                    .idf()
                    .to_ast(),
            );
        }
        for p in &ast.parameters {
            s.insert(p.identifier_declaration_base.idf.to_ast());
        }
        ast.body
            .as_mut()
            .unwrap()
            .statement_base_mut_ref()
            .before_analysis = Some(s);
        self.visit(&mut ast.body.as_ref().unwrap().to_ast());
    }

    pub fn propagate(
        &mut self,
        statements: &mut Vec<AST>,
        before_analysis: &PartitionState<AST>,
    ) -> PartitionState<AST> {
        let mut last = before_analysis.clone();
        // push state through each statement
        for statement in statements.iter_mut() {
            statement
                .try_as_statement_mut()
                .unwrap()
                .statement_base_mut_ref()
                .unwrap()
                .before_analysis = Some(last.clone());
            print!("before  {:?},{:?}", statement, last);
            self.visit(&mut statement.to_ast());
            last = statement
                .try_as_statement_ref()
                .unwrap()
                .statement_base_ref()
                .unwrap()
                .after_analysis
                .as_ref()
                .unwrap()
                .clone();
            print!("after {:?},{:?}", statement, last);
        }

        last
    }
    pub fn visitStatementList(&mut self, ast: &mut StatementList) {
        ast.statement_base_mut_ref().after_analysis = Some(self.propagate(
            &mut ast.statements().clone(),
            ast.before_analysis().as_ref().unwrap(),
        ));
    }
    pub fn visitBlock(&mut self, ast: &mut Block) {
        let mut last = ast
            .statement_list_base
            .statement_base
            .before_analysis
            .as_mut()
            .unwrap();

        // add fresh names from this block
        for name in ast
            .statement_list_base
            .statement_base
            .ast_base
            .names()
            .values()
        {
            last.insert(name.to_ast());
        }

        ast.statement_list_base.statement_base.after_analysis =
            Some(self.propagate(&mut ast.statement_list_base.statements, last));

        // remove names falling out of scope
        for name in ast
            .statement_list_base
            .statement_base
            .ast_base
            .names()
            .values()
        {
            ast.statement_list_base
                .statement_base
                .after_analysis
                .as_mut()
                .unwrap()
                .remove(&name.to_ast());
        }
    }
    pub fn visitIfStatement(&mut self, ast: &mut IfStatement) {
        // condition
        let before_then = self.cond_analyzer.analyze(
            ast.condition.to_ast(),
            ast.statement_base.before_analysis.as_ref().unwrap(),
        );

        // then
        ast.then_branch
            .statement_list_base
            .statement_base
            .before_analysis = Some(before_then);
        self.visit(&mut ast.then_branch.to_ast());
        let after_then = ast
            .then_branch
            .statement_list_base
            .statement_base
            .after_analysis
            .clone();

        // else
        let after_else = if ast.else_branch.is_some() {
            ast.else_branch
                .as_mut()
                .unwrap()
                .statement_base_mut_ref()
                .before_analysis = Some(self.cond_analyzer.analyze(
                ast.condition.unop(String::from("!")).to_ast(),
                ast.statement_base.before_analysis.as_ref().unwrap(),
            ));
            self.visit(&mut ast.else_branch.as_ref().unwrap().to_ast());
            ast.else_branch
                .as_ref()
                .unwrap()
                .statement_list_base
                .statement_base
                .after_analysis
                .clone()
        } else {
            ast.statement_base.before_analysis.clone()
        };

        // join branches
        ast.statement_base.after_analysis = Some(after_then.unwrap().join(&after_else.unwrap()));
    }
    pub fn visitWhileStatement(&mut self, ast: &mut WhileStatement)
    // Body always executes after the condition, but it is also possible that it is not executed at all
    // Condition is true before the body
    // After the loop, the condition is false
    {
        if has_side_effects(ast.condition.to_ast()) || has_side_effects(ast.body.to_ast()) {
            ast.statement_base.before_analysis = Some(
                ast.statement_base
                    .before_analysis
                    .clone()
                    .unwrap()
                    .separate_all(),
            );
        }

        ast.body.statement_list_base.statement_base.before_analysis =
            Some(self.cond_analyzer.analyze(
                ast.condition.to_ast(),
                ast.statement_base.before_analysis.as_ref().unwrap(),
            ));
        self.visit(&mut ast.body.to_ast());

        // Either no loop iteration or at least one loop iteration
        let skip_loop = self.cond_analyzer.analyze(
            ast.condition.unop(String::from("!")).to_ast(),
            ast.statement_base.before_analysis.as_ref().unwrap(),
        );
        let did_loop = self.cond_analyzer.analyze(
            ast.condition.unop(String::from("!")).to_ast(),
            ast.body
                .statement_list_base
                .statement_base
                .after_analysis
                .as_ref()
                .unwrap(),
        );

        // join
        ast.statement_base.after_analysis = Some(skip_loop.join(&did_loop));
    }
    pub fn visitDoWhileStatement(&mut self, ast: &mut DoWhileStatement)
    // Body either executes with or without condition, but it is also possible that it is not executed at all
    // No information about condition before the body
    // After the loop, the condition is false

    // Could be subsequent loop iteration after condition with side effect
    {
        let cond_se = has_side_effects(ast.condition.to_ast());
        if cond_se || has_side_effects(ast.body.to_ast()) {
            ast.statement_base.before_analysis = Some(
                ast.statement_base
                    .before_analysis
                    .as_ref()
                    .unwrap()
                    .separate_all(),
            );
        }

        ast.body.statement_list_base.statement_base.before_analysis =
            ast.statement_base.before_analysis.clone();
        self.visit(&mut ast.body.to_ast());

        // ast.before_analysis is only used by expressions inside condition -> body has already happened at that point
        ast.statement_base.before_analysis = ast
            .body
            .statement_list_base
            .statement_base
            .after_analysis
            .clone();
        ast.statement_base.after_analysis = Some(self.cond_analyzer.analyze(
            ast.condition.unop(String::from("!")).to_ast(),
            ast.statement_base.before_analysis.as_ref().unwrap(),
        ));
    }
    pub fn visitForStatement(&mut self, ast: &mut ForStatement) {
        let mut last = ast.statement_base.before_analysis.clone().unwrap();

        // add names introduced in init
        for name in ast.statement_base.ast_base.names().values() {
            last.insert(name.to_ast());
        }

        if ast.init.is_some() {
            ast.init
                .as_mut()
                .unwrap()
                .statement_base_mut_ref()
                .before_analysis = Some(last.clone());
            self.visit(&mut ast.init.as_ref().unwrap().to_ast());
            ast.statement_base.before_analysis =
                ast.init.as_ref().unwrap().after_analysis().clone();
            // init should be taken into account when looking up things in the condition
        }
        if has_side_effects(ast.condition.to_ast())
            || has_side_effects(ast.body.to_ast())
            || (ast.update.is_some() && has_side_effects(ast.update.as_ref().unwrap().to_ast()))
        {
            ast.statement_base.before_analysis = Some(last.separate_all());
        }
        ast.body.statement_list_base.statement_base.before_analysis =
            Some(self.cond_analyzer.analyze(
                ast.condition.to_ast(),
                ast.statement_base.before_analysis.as_ref().unwrap(),
            ));
        self.visit(&mut ast.body.to_ast());
        if let Some(update) = &mut ast.update
        // Update is always executed after the body (if it is executed)
        {
            update.statement_base_mut_ref().before_analysis = ast
                .body
                .statement_list_base
                .statement_base
                .after_analysis
                .clone();
            self.visit(&mut update.to_ast());
        }

        let skip_loop = self.cond_analyzer.analyze(
            ast.condition.unop(String::from("!")).to_ast(),
            ast.init
                .as_ref()
                .unwrap()
                .after_analysis()
                .as_ref()
                .unwrap(),
        );
        let did_loop = self.cond_analyzer.analyze(
            ast.condition.unop(String::from("!")).to_ast(),
            if let Some(update) = &ast.update {
                update.after_analysis().as_ref().unwrap()
            } else {
                ast.body
                    .statement_list_base
                    .statement_base
                    .after_analysis
                    .as_ref()
                    .unwrap()
            },
        );

        // join
        ast.statement_base.after_analysis = Some(skip_loop.join(&did_loop));

        // drop names introduced in init
        for name in ast.statement_base.ast_base.names().values() {
            ast.statement_base
                .after_analysis
                .as_mut()
                .unwrap()
                .remove(&name.to_ast());
        }
    }
    pub fn visitVariableDeclarationStatement(&mut self, ast: &mut VariableDeclarationStatement) {
        let e = &ast.expr;
        if e.is_some() && has_side_effects(e.as_ref().unwrap().to_ast()) {
            ast.simple_statement_base.statement_base.before_analysis = Some(
                ast.simple_statement_base
                    .statement_base
                    .before_analysis
                    .as_ref()
                    .unwrap()
                    .separate_all(),
            );
        }

        // visit expression
        if let Some(e) = e {
            self.visit(&mut e.to_ast());
        }

        // state after declaration
        let after = ast
            .simple_statement_base
            .statement_base
            .before_analysis
            .clone();

        // name of variable is already in list
        let name = &ast.variable_declaration.identifier_declaration_base.idf;
        assert!(after.as_ref().unwrap().has(&name.to_ast()));

        // make state more precise
        if let Some(e) = e {
            if let Some(pal) = e.privacy_annotation_label() {
                after.clone().unwrap().merge(&name.to_ast(), &pal.into());
            }
        }

        ast.simple_statement_base.statement_base.after_analysis = after;
    }
    pub fn visitRequireStatement(&mut self, ast: &mut RequireStatement) {
        if has_side_effects(ast.condition.to_ast()) {
            ast.simple_statement_base.statement_base.before_analysis = Some(
                ast.simple_statement_base
                    .statement_base
                    .before_analysis
                    .as_ref()
                    .unwrap()
                    .separate_all(),
            );
        }

        self.visit(&mut ast.condition.to_ast());

        // state after require
        let mut after = ast
            .simple_statement_base
            .statement_base
            .before_analysis
            .clone();

        // make state more precise
        let c = &ast.condition;
        if is_instance(c, ASTType::FunctionCallExprBase)
            && is_instance(
                &**c.try_as_function_call_expr_ref().unwrap().func(),
                ASTType::BuiltinFunction,
            )
            && &c
                .try_as_function_call_expr_ref()
                .unwrap()
                .func()
                .try_as_builtin_function_ref()
                .unwrap()
                .op
                == "=="
        {
            let lhs =
                c.try_as_function_call_expr_ref().unwrap().args()[0].privacy_annotation_label();
            let rhs =
                c.try_as_function_call_expr_ref().unwrap().args()[1].privacy_annotation_label();
            if lhs.is_some() && rhs.is_some() {
                after
                    .as_mut()
                    .unwrap()
                    .merge(&lhs.clone().unwrap().into(), &rhs.clone().unwrap().into());
            }
        }

        ast.simple_statement_base.statement_base.after_analysis = after;
    }
    pub fn visitAssignmentStatement(&mut self, ast: &mut AssignmentStatement) {
        if has_side_effects(*ast.lhs().as_ref().unwrap().clone())
            || has_side_effects(ast.rhs().as_ref().unwrap().to_ast())
        {
            ast.statement_base_mut_ref().before_analysis =
                Some(ast.before_analysis().as_ref().unwrap().separate_all());
        }
        let lhs = ast.lhs();
        let rhs = ast.rhs();
        // visit expression
        self.visit(&mut *lhs.as_ref().unwrap().clone());
        self.visit(&mut rhs.as_ref().unwrap().to_ast());

        // state after assignment
        let after = ast.before_analysis();
        recursive_assign(
            *lhs.as_ref().unwrap().clone(),
            rhs.as_ref().unwrap().to_ast(),
            after.clone().unwrap(),
        );

        // save state
        ast.statement_base_mut_ref().after_analysis = after.clone();
    }
    pub fn visitExpressionStatement(&mut self, ast: &mut ExpressionStatement) {
        if has_side_effects(ast.expr.to_ast()) {
            ast.simple_statement_base.statement_base.before_analysis = Some(
                ast.simple_statement_base
                    .statement_base
                    .before_analysis
                    .as_ref()
                    .unwrap()
                    .separate_all(),
            );
        }

        // visit expression
        self.visit(&mut ast.expr.to_ast());

        // if expression has effect, we are already at TOP
        ast.simple_statement_base.statement_base.after_analysis = ast
            .simple_statement_base
            .statement_base
            .before_analysis
            .clone();
    }
    pub fn visitReturnStatement(&mut self, ast: &mut ReturnStatement) {
        ast.statement_base.after_analysis = ast.statement_base.before_analysis.clone();
    }

    pub fn visitContinueStatement(&mut self, ast: &mut ContinueStatement) {
        ast.statement_base.after_analysis = ast.statement_base.before_analysis.clone();
    }

    pub fn visitBreakStatement(&mut self, ast: &mut BreakStatement) {
        ast.statement_base.after_analysis = ast.statement_base.before_analysis.clone();
    }

    pub fn visitStatement(&mut self, _: &mut AST) {
        // raise NotImplementedError();
        unimplemented!();
    }
}
#[derive(ASTVisitorBaseRefImpl)]
pub struct GuardConditionAnalyzer {
    pub ast_visitor_base: AstVisitorBase,
    _neg: bool,
    _analysis: Option<PartitionState<AST>>,
}
impl AstVisitorMut for GuardConditionAnalyzer {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}

    fn has_attr(&self, name: &ASTType) -> bool {
        &ASTType::FunctionCallExprBase == name
    }
    fn get_attr(&mut self, name: &ASTType, ast: &mut AST) -> Self::Return {
        match name {
            ASTType::FunctionCallExprBase => self.visitFunctionCallExpr(
                ast.try_as_expression_mut()
                    .unwrap()
                    .try_as_function_call_expr_mut()
                    .unwrap(),
            ),

            _ => {}
        }
    }
}

impl GuardConditionAnalyzer {
    pub fn new(log: bool) -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", log),
            _neg: false,
            _analysis: None,
        }
    }
    pub fn analyze(
        &mut self,
        cond: AST,
        before_analysis: &PartitionState<AST>,
    ) -> PartitionState<AST> {
        if has_side_effects(cond.clone()) {
            before_analysis.separate_all()
        } else {
            self._neg = false;
            self._analysis = Some(before_analysis.clone());
            self.visit(&mut cond.clone());
            self._analysis.clone().unwrap()
        }
    }

    pub fn _negated(&mut self) {
        self._neg = !self._neg;
        // yield
        // self._neg = ! self._neg
    }

    pub fn visitFunctionCallExpr(&mut self, ast: &mut FunctionCallExpr) {
        if is_instance(&**ast.func(), ASTType::BuiltinFunction) {
            let args = ast.args();
            let op = &ast.func().try_as_builtin_function_ref().unwrap().op;
            if op == "!" {
                self._negated();
                self.visit(&mut args[0].to_ast());
                self._negated();
            } else if (op == "&&" && !self._neg) || (op == "||" && self._neg) {
                self.visit(&mut args[0].to_ast());
                self.visit(&mut args[1].to_ast());
            } else if op == "parenthesis" {
                self.visit(&mut args[0].to_ast());
            } else if (op == "==" && !self._neg) || (op == "!=" && self._neg) {
                recursive_merge(
                    args[0].to_ast(),
                    args[1].to_ast(),
                    self._analysis.clone().unwrap(),
                );
            }
        }
    }
}
pub fn _recursive_update(lhs: AST, rhs: AST, mut analysis: PartitionState<AST>, merge: bool) {
    if is_instance(&lhs, ASTType::TupleExpr) && is_instance(&rhs, ASTType::TupleExpr) {
        for (l, r) in lhs
            .try_as_expression_ref()
            .unwrap()
            .try_as_tuple_or_location_expr_ref()
            .unwrap()
            .try_as_tuple_expr_ref()
            .unwrap()
            .elements
            .iter()
            .zip(
                &rhs.try_as_expression_ref()
                    .unwrap()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_tuple_expr_ref()
                    .unwrap()
                    .elements,
            )
        {
            _recursive_update(l.to_ast(), r.to_ast(), analysis.clone(), merge);
        }
    } else {
        let lhs = lhs
            .try_as_expression_ref()
            .unwrap()
            .privacy_annotation_label();
        let rhs = rhs
            .try_as_expression_ref()
            .unwrap()
            .privacy_annotation_label();
        if lhs.is_some() && rhs.is_some() && analysis.has(&rhs.clone().unwrap().into()) {
            if merge {
                analysis.merge(&lhs.unwrap().into(), &rhs.unwrap().into());
            } else {
                analysis.move_to(&lhs.unwrap().into(), &rhs.clone().unwrap().into());
            }
        } else if lhs.is_some() {
            analysis.move_to_separate(&lhs.unwrap().into());
        }
    }
}
pub fn recursive_merge(lhs: AST, rhs: AST, analysis: PartitionState<AST>) {
    _recursive_update(lhs, rhs, analysis, true);
}

pub fn recursive_assign(lhs: AST, rhs: AST, analysis: PartitionState<AST>) {
    _recursive_update(lhs, rhs, analysis, false);
}
