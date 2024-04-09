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
    is_instance, ASTBaseProperty, ASTFlatten, ASTType, AllExpr, AssignmentStatement,
    AssignmentStatementBaseProperty, Block, BreakStatement, BuiltinFunction,
    ConstructorOrFunctionDefinition, ContinueStatement, DoWhileStatement, ExpressionStatement,
    ForStatement, FunctionCallExpr, FunctionCallExprBaseProperty,
    IdentifierDeclarationBaseProperty, IfStatement, IntoAST, IntoExpression, LocationExpr, MeExpr,
    RequireStatement, ReturnStatement, Statement, StatementBaseMutRef, StatementBaseProperty,
    StatementBaseRef, StatementList, StatementListBaseProperty, TupleExpr,
    VariableDeclarationStatement, WhileStatement, AST,
};
use crate::visitor::visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef};
use rccell::RcCell;
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
// class AliasAnalysisVisitor(AstVisitor)

impl AstVisitor for AliasAnalysisVisitor {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}
    fn has_attr(&self, name: &ASTType) -> bool {
        matches!(
            name,
            ASTType::SourceUnit
                | ASTType::ContractDefinition
                | ASTType::ConstructorOrFunctionDefinition
                | ASTType::StructDefinition
                | ASTType::EnumDefinition
                | ASTType::VariableDeclaration
                | ASTType::StatementListBase
                | ASTType::SimpleStatementBase
                | ASTType::ForStatement
                | ASTType::Mapping
        )
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> Self::Return {
        match name {
            ASTType::ConstructorOrFunctionDefinition => {
                self.visitConstructorOrFunctionDefinition(ast)
            }
            ASTType::StatementListBase => self.visitStatementList(ast),
            ASTType::Block => self.visitBlock(ast),
            ASTType::IfStatement => self.visitIfStatement(ast),

            ASTType::WhileStatement => self.visitWhileStatement(ast),
            ASTType::DoWhileStatement => self.visitDoWhileStatement(ast),
            ASTType::ForStatement => self.visitForStatement(ast),
            ASTType::VariableDeclarationStatement => self.visitVariableDeclarationStatement(ast),
            ASTType::RequireStatement => self.visitRequireStatement(ast),

            ASTType::AssignmentStatementBase => self.visitAssignmentStatement(ast),

            ASTType::ExpressionStatement => self.visitExpressionStatement(ast),
            ASTType::ReturnStatement => self.visitReturnStatement(ast),
            ASTType::ContinueStatement => self.visitContinueStatement(ast),
            ASTType::BreakStatement => self.visitBreakStatement(ast),
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
    pub fn visitConstructorOrFunctionDefinition(&self, ast: &ASTFlatten) {
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
                d.borrow()
                    .try_as_identifier_declaration_ref()
                    .unwrap()
                    .try_as_state_variable_declaration_ref()
                    .unwrap()
                    .idf()
                    .upgrade()
                    .unwrap()
                    .borrow()
                    .to_ast(),
            );
        }
        for p in &ast.parameters {
            s.insert(
                p.borrow()
                    .identifier_declaration_base
                    .idf
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .to_ast(),
            );
        }
        ast.body
            .as_mut()
            .unwrap()
            .borrow()
            .statement_base_mut_ref()
            .before_analysis = Some(s);
        self.visit(ast.body.clone().into());
    }

    pub fn propagate(
        &self,
        statements: &Vec<RcCell<AST>>,
        before_analysis: &PartitionState<AST>,
    ) -> PartitionState<AST> {
        let mut last = before_analysis.clone();
        // push state through each statement
        for statement in statements.iter_mut() {
            statement
                .borrow_mut()
                .try_as_statement_mut()
                .unwrap()
                .statement_base_mut_ref()
                .unwrap()
                .before_analysis = Some(last.clone());
            print!("before  {:?},{:?}", statement, last);
            self.visit(statement.clone().into());
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
    pub fn visitStatementList(&self, ast: &ASTFlatten) {
        ast.statement_base_mut_ref().after_analysis = Some(self.propagate(
            &ast.statements().clone(),
            ast.before_analysis().as_ref().unwrap(),
        ));
    }
    pub fn visitBlock(&self, ast: &ASTFlatten) {
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
            last.insert(name.upgrade().unwrap().borrow().to_ast());
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
                .remove(&name.upgrade().unwrap().borrow().to_ast());
        }
    }
    pub fn visitIfStatement(&self, ast: &ASTFlatten) {
        // condition
        let before_then = self.cond_analyzer.analyze(
            ast.condition.clone().into(),
            ast.statement_base.before_analysis.as_ref().unwrap(),
        );

        // then
        ast.then_branch
            .borrow_mut()
            .statement_list_base
            .statement_base
            .before_analysis = Some(before_then);
        self.visit(ast.then_branch.clone().into());
        let after_then = ast
            .then_branch
            .borrow()
            .statement_list_base
            .statement_base
            .after_analysis
            .clone();

        // else
        let after_else = if ast.else_branch.is_some() {
            ast.else_branch
                .as_mut()
                .unwrap()
                .borrow_mut()
                .statement_base_mut_ref()
                .before_analysis = Some(self.cond_analyzer.analyze(
                ast.condition.borrow().unop(String::from("!")).to_ast(),
                ast.statement_base.before_analysis.as_ref().unwrap(),
            ));
            self.visit(ast.else_branch.clone().into());
            ast.else_branch
                .as_ref()
                .unwrap()
                .borrow()
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
    pub fn visitWhileStatement(&self, ast: &ASTFlatten)
    // Body always executes after the condition, but it is also possible that it is not executed at all
    // Condition is true before the body
    // After the loop, the condition is false
    {
        if has_side_effects(ast.condition.borrow().to_ast())
            || has_side_effects(ast.body.borrow().to_ast())
        {
            ast.statement_base.before_analysis = Some(
                ast.statement_base
                    .before_analysis
                    .clone()
                    .unwrap()
                    .separate_all(),
            );
        }

        ast.body
            .borrow_mut()
            .statement_list_base
            .statement_base
            .before_analysis = Some(self.cond_analyzer.analyze(
            ast.condition.clone().into(),
            ast.statement_base.before_analysis.as_ref().unwrap(),
        ));
        self.visit(ast.body.clone().into());

        // Either no loop iteration or at least one loop iteration
        let skip_loop = self.cond_analyzer.analyze(
            ast.condition.borrow().unop(String::from("!")).to_ast(),
            ast.statement_base.before_analysis.as_ref().unwrap(),
        );
        let did_loop = self.cond_analyzer.analyze(
            ast.condition.borrow().unop(String::from("!")).to_ast(),
            ast.body
                .borrow()
                .statement_list_base
                .statement_base
                .after_analysis
                .as_ref()
                .unwrap(),
        );

        // join
        ast.statement_base.after_analysis = Some(skip_loop.join(&did_loop));
    }
    pub fn visitDoWhileStatement(&self, ast: &ASTFlatten)
    // Body either executes with or without condition, but it is also possible that it is not executed at all
    // No information about condition before the body
    // After the loop, the condition is false

    // Could be subsequent loop iteration after condition with side effect
    {
        let cond_se = has_side_effects(ast.condition.borrow().to_ast());
        if cond_se || has_side_effects(ast.body.borrow().to_ast()) {
            ast.statement_base.before_analysis = Some(
                ast.statement_base
                    .before_analysis
                    .as_ref()
                    .unwrap()
                    .separate_all(),
            );
        }

        ast.body
            .borrow_mut()
            .statement_list_base
            .statement_base
            .before_analysis = ast.statement_base.before_analysis.clone();
        self.visit(ast.body.clone().into());

        // ast.before_analysis is only used by expressions inside condition -> body has already happened at that point
        ast.statement_base.before_analysis = ast
            .body
            .borrow()
            .statement_list_base
            .statement_base
            .after_analysis
            .clone();
        ast.statement_base.after_analysis = Some(self.cond_analyzer.analyze(
            ast.condition.borrow().unop(String::from("!")).to_ast(),
            ast.statement_base.before_analysis.as_ref().unwrap(),
        ));
    }
    pub fn visitForStatement(&self, ast: &ASTFlatten) {
        let mut last = ast.statement_base.before_analysis.clone().unwrap();

        // add names introduced in init
        for name in ast.statement_base.ast_base.names().values() {
            last.insert(name.upgrade().unwrap().borrow().to_ast());
        }

        if ast.init.is_some() {
            ast.init
                .as_mut()
                .unwrap()
                .borrow()
                .statement_base_mut_ref()
                .before_analysis = Some(last.clone());
            self.visit(&mut ast.init.as_ref().unwrap().borrow().to_ast());
            ast.statement_base.before_analysis =
                ast.init.as_ref().unwrap().borrow().after_analysis().clone();
            // init should be taken into account when looking up things in the condition
        }
        if has_side_effects(ast.condition.borrow().to_ast())
            || has_side_effects(ast.body.borrow().to_ast())
            || (ast.update.is_some()
                && has_side_effects(ast.update.as_ref().unwrap().borrow().to_ast()))
        {
            ast.statement_base.before_analysis = Some(last.separate_all());
        }
        ast.body
            .borrow_mut()
            .statement_list_base
            .statement_base
            .before_analysis = Some(self.cond_analyzer.analyze(
            ast.condition.borrow().to_ast(),
            ast.statement_base.before_analysis.as_ref().unwrap(),
        ));
        self.visit(ast.body.clone().into());
        if let Some(update) = &mut ast.update
        // Update is always executed after the body (if it is executed)
        {
            update.borrow_mut().statement_base_mut_ref().before_analysis = ast
                .body
                .borrow()
                .statement_list_base
                .statement_base
                .after_analysis
                .clone();
            self.visit(update.clone().into());
        }

        let skip_loop = self.cond_analyzer.analyze(
            ast.condition.unop(String::from("!")).to_ast(),
            ast.init
                .as_ref()
                .unwrap()
                .borrow()
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
                    .borrow()
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
                .remove(&name.upgrade().unwrap().borrow().to_ast());
        }
    }
    pub fn visitVariableDeclarationStatement(&self, ast: &ASTFlatten) {
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
            self.visit(e.clone().into());
        }

        // state after declaration
        let after = ast
            .simple_statement_base
            .statement_base
            .before_analysis
            .clone();

        // name of variable is already in list
        let name = &ast.variable_declaration.identifier_declaration_base.idf;
        assert!(after
            .as_ref()
            .unwrap()
            .has(&name.as_ref().unwrap().borrow().to_ast()));

        // make state more precise
        if let Some(e) = e {
            if let Some(pal) = e.privacy_annotation_label() {
                after
                    .clone()
                    .unwrap()
                    .merge(&name.as_ref().unwrap().borrow().to_ast(), &pal.into());
            }
        }

        ast.simple_statement_base.statement_base.after_analysis = after;
    }
    pub fn visitRequireStatement(&self, ast: &ASTFlatten) {
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

        self.visit(ast.condition.clone().into());

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
    pub fn visitAssignmentStatement(&self, ast: &ASTFlatten) {
        if has_side_effects(*ast.lhs().as_ref().unwrap().clone())
            || has_side_effects(ast.rhs().as_ref().unwrap().to_ast())
        {
            ast.statement_base_mut_ref().before_analysis =
                Some(ast.before_analysis().as_ref().unwrap().separate_all());
        }
        let lhs = ast.lhs();
        let rhs = ast.rhs();
        // visit expression
        self.visit(lhs.clone().into());
        self.visit(rhs.clone().into());

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
    pub fn visitExpressionStatement(&self, ast: &ASTFlatten) {
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
        self.visit(ast.expr.clone().into());

        // if expression has effect, we are already at TOP
        ast.simple_statement_base.statement_base.after_analysis = ast
            .simple_statement_base
            .statement_base
            .before_analysis
            .clone();
    }
    pub fn visitReturnStatement(&self, ast: &ASTFlatten) {
        ast.statement_base.after_analysis = ast.statement_base.before_analysis.clone();
    }

    pub fn visitContinueStatement(&self, ast: &ASTFlatten) {
        ast.statement_base.after_analysis = ast.statement_base.before_analysis.clone();
    }

    pub fn visitBreakStatement(&self, ast: &ASTFlatten) {
        ast.statement_base.after_analysis = ast.statement_base.before_analysis.clone();
    }

    pub fn visitStatement(&self, _: &ASTFlatten) {
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
impl AstVisitor for GuardConditionAnalyzer {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}

    fn has_attr(&self, name: &ASTType) -> bool {
        &ASTType::FunctionCallExprBase == name
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> Self::Return {
        match name {
            ASTType::FunctionCallExprBase => self.visitFunctionCallExpr(ast),

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
    pub fn analyze(&self, cond: AST, before_analysis: &PartitionState<AST>) -> PartitionState<AST> {
        if has_side_effects(cond.clone()) {
            before_analysis.separate_all()
        } else {
            self._neg = false;
            self._analysis = Some(before_analysis.clone());
            self.visit(cond.clone().into());
            self._analysis.clone().unwrap()
        }
    }

    pub fn _negated(&mut self) {
        self._neg = !self._neg;
        // yield
        // self._neg = ! self._neg
    }

    pub fn visitFunctionCallExpr(&self, ast: &ASTFlatten) {
        if is_instance(&**ast.func(), ASTType::BuiltinFunction) {
            let args = ast.args();
            let op = &ast.func().try_as_builtin_function_ref().unwrap().op;
            if op == "!" {
                self._negated();
                self.visit(args[0].clone().into());
                self._negated();
            } else if (op == "&&" && !self._neg) || (op == "||" && self._neg) {
                self.visit(args[0].clone().into());
                self.visit(args[1].clone().into());
            } else if op == "parenthesis" {
                self.visit(args[0].clone().into());
            } else if (op == "==" && !self._neg) || (op == "!=" && self._neg) {
                recursive_merge(
                    args[0].clone().into(),
                    args[1].clone().into(),
                    self._analysis.clone().unwrap(),
                );
            }
        }
    }
}
pub fn _recursive_update(
    lhs: &ASTFlatten,
    rhs: &ASTFlatten,
    mut analysis: PartitionState<AST>,
    merge: bool,
) {
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
pub fn recursive_merge(lhs: &ASTFlatten, rhs: &ASTFlatten, analysis: PartitionState<AST>) {
    _recursive_update(lhs, rhs, analysis, true);
}

pub fn recursive_assign(lhs: &ASTFlatten, rhs: &ASTFlatten, analysis: PartitionState<AST>) {
    _recursive_update(lhs, rhs, analysis, false);
}
