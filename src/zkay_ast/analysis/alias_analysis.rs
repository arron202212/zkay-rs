use crate::zkay_ast::analysis::partition_state::PartitionState;
use crate::zkay_ast::analysis::side_effects::has_side_effects;
use crate::zkay_ast::ast::{
    is_instance, ASTCode, ASTType, AllExpr, AssignmentStatement, Block, BreakStatement,
    BuiltinFunction, ConstructorOrFunctionDefinition, ContinueStatement, DoWhileStatement,
    ExpressionStatement, ForStatement, FunctionCallExpr, IfStatement, LocationExpr, MeExpr,
    PrivacyLabelExpr, RequireStatement, ReturnStatement, Statement, StatementList, TupleExpr,
    VariableDeclarationStatement, WhileStatement, AST,
};
use crate::zkay_ast::visitor::visitor::AstVisitor;

pub fn alias_analysis(ast: &AST) {
    let v = AliasAnalysisVisitor::new();
    v.cond_analyzer.visit(ast.clone());
}

struct AliasAnalysisVisitor {
    cond_analyzer: GuardConditionAnalyzer,
}
// class AliasAnalysisVisitor(AstVisitor)

impl AstVisitor for AliasAnalysisVisitor {
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
impl AliasAnalysisVisitor {
    // pub fn __init__(&self, log=False)
    //     super().__init__("node-or-children", log)
    //     self.cond_analyzer = GuardConditionAnalyzer()
    pub fn new() -> Self {
        Self {
            cond_analyzer: GuardConditionAnalyzer::new(),
        }
    }
    pub fn visitConstructorOrFunctionDefinition(&self, ast: &mut ConstructorOrFunctionDefinition) {
        let mut s: PartitionState<PrivacyLabelExpr> = PartitionState::new();
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
            s.insert(PrivacyLabelExpr::Identifier(d.idf()));
        }
        for p in &ast.parameters {
            s.insert(PrivacyLabelExpr::Identifier(
                *p.identifier_declaration_base.idf.clone(),
            ));
        }
        ast.body.as_mut().unwrap().set_before_analysis(Some(s));
        self.visit(ast.body.as_ref().unwrap().get_ast());
    }

    pub fn propagate(
        &self,
        mut statements: Vec<AST>,
        before_analysis: PartitionState<PrivacyLabelExpr>,
    ) -> PartitionState<PrivacyLabelExpr> {
        let mut last = before_analysis.clone();
        // push state through each statement
        for statement in statements.iter_mut() {
            statement.set_before_analysis(Some(last.clone()));
            print!("before  {:?},{:?}", statement, last);
            self.visit(statement.get_ast());
            last = statement.after_analysis().unwrap();
            print!("after {:?},{:?}", statement, last);
        }

        last
    }
    pub fn visitStatementList(&self, mut ast: StatementList) {
        ast.set_after_analysis(Some(
            self.propagate(ast.statements(), ast.before_analysis().unwrap()),
        ));
    }
    pub fn visitBlock(&self,mut  ast: Block) {
        let mut last = ast
            .statement_list_base
            .statement_base
            .before_analysis
            .unwrap();

        // add fresh names from this block
        for name in ast
            .statement_list_base
            .statement_base
            .ast_base
            .names
            .values()
        {
            last.insert(PrivacyLabelExpr::Identifier(name.clone()));
        }

        ast.statement_list_base.statement_base.after_analysis =
            Some(self.propagate(ast.statement_list_base.statements, last));

        // remove names falling out of scope
        for name in ast
            .statement_list_base
            .statement_base
            .ast_base
            .names
            .values()
        {
            ast.statement_list_base
                .statement_base
                .after_analysis.as_mut()
                .unwrap()
                .remove(&PrivacyLabelExpr::Identifier(name.clone()));
        }
    }
    pub fn visitIfStatement(&mut self, mut ast: IfStatement) {
        // condition
        let before_then = self.cond_analyzer.analyze(
            ast.condition.get_ast(),
            ast.statement_base.before_analysis.clone().unwrap(),
        );

        // then
        ast.then_branch
            .statement_list_base
            .statement_base
            .before_analysis = Some(before_then);
        self.visit(ast.then_branch.get_ast());
        let after_then = ast
            .then_branch
            .statement_list_base
            .statement_base
            .after_analysis;

        // else
        let after_else = if ast.else_branch.is_some() {
            ast.else_branch.as_mut()
                .unwrap()
                .set_before_analysis(Some(self.cond_analyzer.analyze(
                    ast.condition.unop(String::from("!")).get_ast(),
                    ast.statement_base.before_analysis.unwrap(),
                )));
            self.visit(ast.else_branch.as_ref().unwrap().get_ast());
            ast.else_branch.as_ref()
                .unwrap()
                .statement_list_base
                .statement_base
                .after_analysis.clone()
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
        if has_side_effects(ast.condition.get_ast()) || has_side_effects(ast.body.get_ast()) {
            ast.statement_base.before_analysis =
                Some(ast.statement_base.before_analysis.clone().unwrap().separate_all());
        }

        ast.body.statement_list_base.statement_base.before_analysis =
            Some(self.cond_analyzer.analyze(
                ast.condition.get_ast(),
                ast.statement_base.before_analysis.clone().unwrap(),
            ));
        self.visit(ast.body.get_ast());

        // Either no loop iteration or at least one loop iteration
        let skip_loop = self.cond_analyzer.analyze(
            ast.condition.unop(String::from("!")).get_ast(),
            ast.statement_base.before_analysis.clone().unwrap(),
        );
        let did_loop = self.cond_analyzer.analyze(
            ast.condition.unop(String::from("!")).get_ast(),
            ast.body
                .statement_list_base
                .statement_base
                .after_analysis.clone()
                .unwrap(),
        );

        // join
        ast.statement_base.after_analysis = Some(skip_loop.join(&did_loop));
    }
    pub fn visitDoWhileStatement(&mut self, mut ast: DoWhileStatement)
    // Body either executes with or without condition, but it is also possible that it is not executed at all
    // No information about condition before the body
    // After the loop, the condition is false

    // Could be subsequent loop iteration after condition with side effect
    {
        let cond_se = has_side_effects(ast.condition.get_ast());
        if cond_se || has_side_effects(ast.body.get_ast()) {
            ast.statement_base.before_analysis =
                Some(ast.statement_base.before_analysis.unwrap().separate_all());
        }

        ast.body.statement_list_base.statement_base.before_analysis =
            ast.statement_base.before_analysis.clone();
        self.visit(ast.body.get_ast());

        // ast.before_analysis is only used by expressions inside condition -> body has already happened at that point
        ast.statement_base.before_analysis = ast
            .body
            .statement_list_base
            .statement_base
            .after_analysis
            .clone();
        ast.statement_base.after_analysis = Some(self.cond_analyzer.analyze(
            ast.condition.unop(String::from("!")).get_ast(),
            ast.statement_base.before_analysis.clone().unwrap(),
        ));
    }
    pub fn visitForStatement(&mut self, mut ast: ForStatement) {
        let mut last = ast.statement_base.before_analysis.clone().unwrap();

        // add names introduced in init
        for name in ast.statement_base.ast_base.names.values() {
            last.insert(PrivacyLabelExpr::Identifier(name.clone()));
        }

        if ast.init.is_some() {
            ast.init.as_mut().unwrap().set_before_analysis(Some(last.clone()));
            self.visit(ast.init.as_ref().unwrap().get_ast());
            ast.statement_base.before_analysis = ast.init.as_ref().unwrap().after_analysis();
            // init should be taken into account when looking up things in the condition
        }
        if has_side_effects(ast.condition.get_ast())
            || has_side_effects(ast.body.get_ast())
            || (ast.update.is_some() && has_side_effects(ast.update.as_ref().unwrap().get_ast()))
        {
            ast.statement_base.before_analysis = Some(last.separate_all());
        }
        ast.body.statement_list_base.statement_base.before_analysis =
            Some(self.cond_analyzer.analyze(
                ast.condition.get_ast(),
                ast.statement_base.before_analysis.clone().unwrap(),
            ));
        self.visit(ast.body.get_ast());
        if let Some(update) = &mut ast.update
        // Update is always executed after the body (if it is executed)
        {
            update.set_before_analysis(
                ast.body
                    .statement_list_base
                    .statement_base
                    .after_analysis
                    .clone(),
            );
            self.visit(update.get_ast());
        }

        let skip_loop = self.cond_analyzer.analyze(
            ast.condition.unop(String::from("!")).get_ast(),
            ast.init.unwrap().after_analysis().unwrap(),
        );
        let did_loop = self.cond_analyzer.analyze(
            ast.condition.unop(String::from("!")).get_ast(),
            if let Some(update) = &ast.update {
                update.after_analysis().unwrap()
            } else {
                ast.body
                    .statement_list_base
                    .statement_base
                    .after_analysis.clone()
                    .unwrap()
            },
        );

        // join
        ast.statement_base.after_analysis = Some(skip_loop.join(&did_loop));

        // drop names introduced in init
        for name in ast.statement_base.ast_base.names.values() {
            ast.statement_base
                .after_analysis.as_mut()
                .unwrap()
                .remove(&PrivacyLabelExpr::Identifier(name.clone()));
        }
    }
    pub fn visitVariableDeclarationStatement(&mut self,mut  ast: VariableDeclarationStatement) {
        let e = &ast.expr;
        if e.is_some() && has_side_effects(e.as_ref().unwrap().get_ast()) {
            ast.simple_statement_base.statement_base.before_analysis = Some(
                ast.simple_statement_base
                    .statement_base
                    .before_analysis
                    .unwrap()
                    .separate_all(),
            );
        }

        // visit expression
        if let Some(e) = e {
            self.visit(e.get_ast());
        }

        // state after declaration
        let after = ast
            .simple_statement_base
            .statement_base
            .before_analysis
            .clone();

        // name of variable is already in list
        let name = ast.variable_declaration.identifier_declaration_base.idf;
        assert!(after.as_ref().unwrap().has(&PrivacyLabelExpr::Identifier(*name.clone())));

        // make state more precise
        if let Some(e) = e {
            if let Some(pal) = e.privacy_annotation_label() {
                after.clone()
                    .unwrap()
                    .merge(&PrivacyLabelExpr::Identifier(*name.clone()), &pal.into());
            }
        }

        ast.simple_statement_base.statement_base.after_analysis = after;
    }
    pub fn visitRequireStatement(&mut self, mut ast: RequireStatement) {
        if has_side_effects(ast.condition.get_ast()) {
            ast.simple_statement_base.statement_base.before_analysis = Some(
                ast.simple_statement_base
                    .statement_base
                    .before_analysis.as_ref()
                    .unwrap()
                    .separate_all(),
            );
        }

        self.visit(ast.condition.get_ast());

        // state after require
        let mut after = ast
            .simple_statement_base
            .statement_base
            .before_analysis
            .clone();

        // make state more precise
        let c = ast.condition;
        if is_instance(&c, ASTType::FunctionCallExpr)
            && is_instance(&c.func().unwrap(), ASTType::BuiltinFunction)
            && &c.func().unwrap().op().unwrap() == "=="
        {
            let lhs = c.args()[0].privacy_annotation_label();
            let rhs = c.args()[1].privacy_annotation_label();
            if lhs.is_some() && rhs.is_some() {
                after.as_mut()
                    .unwrap()
                    .merge(&lhs.clone().unwrap().into(), &rhs.clone().unwrap().into());
            }
        }

        ast.simple_statement_base.statement_base.after_analysis = after;
    }
    pub fn visitAssignmentStatement(&mut self, mut ast: AssignmentStatement) {
        let lhs = ast.lhs();
        let rhs = ast.rhs();
        if has_side_effects(lhs.clone().unwrap().into()) || has_side_effects(rhs.clone().unwrap().get_ast()) {
            ast.set_before_analysis(Some(ast.before_analysis().unwrap().separate_all()));
        }

        // visit expression
        self.visit(lhs.clone().unwrap().into());
        self.visit(rhs.as_ref().unwrap().get_ast());

        // state after assignment
        let after = ast.before_analysis();
        recursive_assign(lhs.unwrap().into(), rhs.unwrap().get_ast(), after.clone().unwrap());

        // save state
        ast.set_after_analysis(after);
    }
    pub fn visitExpressionStatement(&mut self, mut ast: ExpressionStatement) {
        if has_side_effects(ast.expr.get_ast()) {
            ast.simple_statement_base.statement_base.before_analysis = Some(
                ast.simple_statement_base
                    .statement_base
                    .before_analysis
                    .unwrap()
                    .separate_all(),
            );
        }

        // visit expression
        self.visit(ast.expr.get_ast());

        // if expression has effect, we are already at TOP
        ast.simple_statement_base.statement_base.after_analysis = ast
            .simple_statement_base
            .statement_base
            .before_analysis
            .clone();
    }
    pub fn visitReturnStatement(&self,mut ast: ReturnStatement) {
        ast.statement_base.after_analysis = ast.statement_base.before_analysis;
    }

    pub fn visitContinueStatement(&self,mut  ast: ContinueStatement) {
        ast.statement_base.after_analysis = ast.statement_base.before_analysis;
    }

    pub fn visitBreakStatement(&self, mut ast: BreakStatement) {
        ast.statement_base.after_analysis = ast.statement_base.before_analysis;
    }

    pub fn visitStatement(&self, _: AST) {
        // raise NotImplementedError();
        unimplemented!();
    }
}
pub struct GuardConditionAnalyzer {
    _neg: bool,
    _analysis: Option<PartitionState<PrivacyLabelExpr>>,
}
impl AstVisitor for GuardConditionAnalyzer {
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
// class GuardConditionAnalyzer(AstVisitor)
// pub fn __init__(&self, log=False)
//     super().__init__("node-or-children", log)
//     self._neg = False
//     self._analysis = None
impl GuardConditionAnalyzer {
    pub fn new() -> Self {
        Self {
            _neg: false,
            _analysis: None,
        }
    }
    pub fn analyze(
        &mut self,
        cond: AST,
        before_analysis: PartitionState<PrivacyLabelExpr>,
    ) -> PartitionState<PrivacyLabelExpr> {
        if has_side_effects(cond.clone()) {
            before_analysis.separate_all()
        } else {
            self._neg = false;
            self._analysis = Some(before_analysis);
            self.visit(cond.clone());
            self._analysis.clone().unwrap()
        }
    }

    pub fn _negated(&mut self) {
        self._neg = !self._neg;
        // yield
        // self._neg = ! self._neg
    }

    pub fn visitFunctionCallExpr(&mut self, ast: FunctionCallExpr) {
        if is_instance(&ast.func().unwrap(), ASTType::BuiltinFunction) {
            let args = ast.args();
            let op = &ast.func().unwrap().op().unwrap();
            if op == "!" {
                self._negated();
                self.visit(args[0].get_ast());
                self._negated();
            } else if (op == "&&" && !self._neg) || (op == "||" && self._neg) {
                self.visit(args[0].get_ast());
                self.visit(args[1].get_ast());
            } else if op == "parenthesis" {
                self.visit(args[0].get_ast());
            } else if (op == "==" && !self._neg) || (op == "!=" && self._neg) {
                recursive_merge(
                    args[0].get_ast(),
                    args[1].get_ast(),
                    self._analysis.clone().unwrap(),
                );
            }
        }
    }
}
pub fn _recursive_update(
    lhs: AST,
    rhs: AST,
    mut analysis: PartitionState<PrivacyLabelExpr>,
    merge: bool,
) {
    if is_instance(&lhs, ASTType::TupleExpr) && is_instance(&rhs, ASTType::TupleExpr) {
        for (l, r) in lhs.elements().iter().zip(rhs.elements()) {
            _recursive_update(l.get_ast(), r.get_ast(), analysis.clone(), merge);
        }
    } else {
        let lhs = lhs.privacy_annotation_label();
        let rhs = rhs.privacy_annotation_label();
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
pub fn recursive_merge(lhs: AST, rhs: AST, analysis: PartitionState<PrivacyLabelExpr>) {
    _recursive_update(lhs, rhs, analysis, true);
}

pub fn recursive_assign(lhs: AST, rhs: AST, analysis: PartitionState<PrivacyLabelExpr>) {
    _recursive_update(lhs, rhs, analysis, false);
}
