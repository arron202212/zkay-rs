use crate::zkay_ast::analysis::partition_state::PartitionState;
use crate::zkay_ast::analysis::side_effects::has_side_effects;
use crate::zkay_ast::ast::{
    AllExpr, AssignmentStatement, Block, BreakStatement, BuiltinFunction,
    ConstructorOrFunctionDefinition, ContinueStatement, DoWhileStatement, ExpressionStatement,
    ForStatement, FunctionCallExpr, IfStatement, LocationExpr, MeExpr, PrivacyLabelExpr,
    RequireStatement, ReturnStatement, StatementList, TupleExpr, VariableDeclarationStatement,
    WhileStatement,
};
use crate::zkay_ast::visitor::visitor::AstVisitor;

pub fn alias_analysis(ast: AST) {
    v = AliasAnalysisVisitor();
    v.visit(ast);
}

struct AliasAnalysisVisitor {
    cond_analyzer: GuardConditionAnalyzer,
}
// class AliasAnalysisVisitor(AstVisitor)
impl AliasAnalysisVisitor {
    // pub fn __init__(&self, log=False)
    //     super().__init__("node-or-children", log)
    //     self.cond_analyzer = GuardConditionAnalyzer()
    pub fn new() -> Self {
        Self {
            cond_analyzer: GuardConditionAnalyzer::new(),
        }
    }
    pub fn visitConstructorOrFunctionDefinition(&self, ast: ConstructorOrFunctionDefinition) {
        let s: PartitionState<PrivacyLabelExpr> = PartitionState::new();
        s.insert(MeExpr::new().privacy_annotation_label());
        s.insert(AllExpr().privacy_annotation_label());
        for d in ast.parent.state_variable_declarations {
            s.insert(d.idf);
        }
        for p in ast.parameters {
            s.insert(p.idf);
        }
        ast.body.before_analysis = s;
        return self.visit(ast.body);
    }

    pub fn propagate(
        &self,
        statements: Vec<Statement>,
        before_analysis: PartitionState<PrivacyLabelExpr>,
    ) -> PartitionState<PrivacyLabelExpr> {
        last = before_analysis.copy();
        // push state through each statement
        for statement in statements {
            statement.before_analysis = last;
            // print("before", statement, last)
            self.visit(statement);
            last = statement.after_analysis;
        }
        // print("after", statement, last)
        return last.copy();
    }
    pub fn visitStatementList(&self, ast: StatementList) {
        ast.after_analysis = self.propagate(ast.statements, ast.before_analysis);
    }
    pub fn visitBlock(&self, ast: Block) {
        last = ast.before_analysis.copy();

        // add fresh names from this block
        for name in ast.names.values() {
            last.insert(name);
        }

        ast.after_analysis = self.propagate(ast.statements, last);

        // remove names falling out of scope
        for name in ast.names.values() {
            ast.after_analysis.remove(name);
        }
    }
    pub fn visitIfStatement(&self, ast: IfStatement) {
        // condition
        before_then = self
            .cond_analyzer
            .analyze(ast.condition, ast.before_analysis);

        // then
        ast.then_branch.before_analysis = before_then;
        self.visit(ast.then_branch);
        after_then = ast.then_branch.after_analysis;

        // else
        let after_else = if ast.else_branch.is_some() {
            ast.else_branch.before_analysis = self
                .cond_analyzer
                .analyze(ast.condition.unop("!"), ast.before_analysis);
            self.visit(ast.else_branch);
            ast.else_branch.after_analysis()
        } else {
            ast.before_analysis.clone()
        };

        // join branches
        ast.after_analysis = after_then.join(after_else);
    }
    pub fn visitWhileStatement(&self, ast: &mut WhileStatement)
    // Body always executes after the condition, but it is also possible that it is not executed at all
    // Condition is true before the body
    // After the loop, the condition is false
    {
        if has_side_effects(ast.condition) || has_side_effects(ast.body) {
            ast.before_analysis = ast.before_analysis.separate_all();
        }

        ast.body.before_analysis = self
            .cond_analyzer
            .analyze(ast.condition, ast.before_analysis);
        self.visit(ast.body);

        // Either no loop iteration or at least one loop iteration
        let skip_loop = self
            .cond_analyzer
            .analyze(ast.condition.unop("!"), ast.before_analysis);
        let did_loop = self
            .cond_analyzer
            .analyze(ast.condition.unop("!"), ast.body.after_analysis);

        // join
        ast.after_analysis = skip_loop.join(did_loop);
    }
    pub fn visitDoWhileStatement(&self, ast: DoWhileStatement)
    // Body either executes with or without condition, but it is also possible that it is not executed at all
    // No information about condition before the body
    // After the loop, the condition is false

    // Could be subsequent loop iteration after condition with side effect
    {
        let cond_se = has_side_effects(ast.condition);
        if cond_se || has_side_effects(ast.body) {
            ast.before_analysis = ast.before_analysis.separate_all();
        }

        ast.body.before_analysis = ast.before_analysis.copy();
        self.visit(ast.body);

        // ast.before_analysis is only used by expressions inside condition -> body has already happened at that point
        ast.before_analysis = ast.body.after_analysis.copy();
        ast.after_analysis = self
            .cond_analyzer
            .analyze(ast.condition.unop("!"), ast.before_analysis);
    }
    pub fn visitForStatement(&self, ast: ForStatement) {
        last = ast.before_analysis.copy();

        // add names introduced in init
        for name in ast.names.values() {
            last.insert(name);
        }

        if ast.init.is_some() {
            ast.init.before_analysis = last.copy();
            self.visit(ast.init);
            ast.before_analysis = ast.init.after_analysis.copy(); // init should be taken into account when looking up things in the condition
        }
        if has_side_effects(ast.condition)
            || has_side_effects(ast.body)
            || (ast.update.is_some() && has_side_effects(ast.update))
        {
            ast.before_analysis = last.separate_all();
        }
        ast.body.before_analysis = self
            .cond_analyzer
            .analyze(ast.condition, ast.before_analysis);
        self.visit(ast.body);
        if ast.update.is_some()
        // Update is always executed after the body (if it is executed)
        {
            ast.update.before_analysis = ast.body.after_analysis.copy();
            self.visit(ast.update);
        }

        skip_loop = self
            .cond_analyzer
            .analyze(ast.condition.unop("!"), ast.init.after_analysis);
        did_loop = self.cond_analyzer.analyze(
            ast.condition.unop("!"),
            if ast.update {
                ast.update.after_analysis
            } else {
                ast.body.after_analysis
            },
        );

        // join
        ast.after_analysis = skip_loop.join(did_loop);

        // drop names introduced in init
        for name in ast.names.values() {
            ast.after_analysis.remove(name);
        }
    }
    pub fn visitVariableDeclarationStatement(&self, ast: VariableDeclarationStatement) {
        e = ast.expr;
        if e && has_side_effects(e) {
            ast.before_analysis = ast.before_analysis.separate_all();
        }

        // visit expression
        if e {
            self.visit(e);
        }

        // state after declaration
        after = ast.before_analysis.copy();

        // name of variable is already in list
        name = ast.variable_declaration.idf;
        assert(after.has(name));

        // make state more precise
        if e && e.privacy_annotation_label() {
            after.merge(name, e.privacy_annotation_label());
        }

        ast.after_analysis = after;
    }
    pub fn visitRequireStatement(&self, ast: RequireStatement) {
        if has_side_effects(ast.condition) {
            ast.before_analysis = ast.before_analysis.separate_all();
        }

        self.visit(ast.condition);

        // state after require
        after = ast.before_analysis.copy();

        // make state more precise
        c = ast.condition;
        if isinstance(c, FunctionCallExpr)
            && isinstance(c.func, BuiltinFunction)
            && c.func.op == "=="
        {
            let lhs = c.args[0].privacy_annotation_label();
            let rhs = c.args[1].privacy_annotation_label();
            if lhs && rhs {
                after.merge(lhs, rhs);
            }
        }

        ast.after_analysis = after;
    }
    pub fn visitAssignmentStatement(&self, ast: AssignmentStatement) {
        lhs = ast.lhs;
        rhs = ast.rhs;
        if has_side_effects(lhs) || has_side_effects(rhs) {
            ast.before_analysis = ast.before_analysis.separate_all();
        }

        // visit expression
        self.visit(ast.lhs);
        self.visit(ast.rhs);

        // state after assignment
        after = ast.before_analysis.copy();
        recursive_assign(lhs, rhs, after);

        // save state
        ast.after_analysis = after;
    }
    pub fn visitExpressionStatement(&self, ast: ExpressionStatement) {
        if has_side_effects(ast.expr) {
            ast.before_analysis = ast.before_analysis.separate_all();
        }

        // visit expression
        self.visit(ast.expr);

        // if expression has effect, we are already at TOP
        ast.after_analysis = ast.before_analysis.copy();
    }
    pub fn visitReturnStatement(&self, ast: ReturnStatement) {
        ast.after_analysis = ast.before_analysis;
    }

    pub fn visitContinueStatement(&self, ast: ContinueStatement) {
        ast.after_analysis = ast.before_analysis;
    }

    pub fn visitBreakStatement(&self, ast: BreakStatement) {
        ast.after_analysis = ast.before_analysis;
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
    pub fn analyze(&mut self, cond: AST, before_analysis: PartitionState) -> PartitionState {
        if has_side_effects(cond) {
            before_analysis.copy().separate_all()
        } else {
            self._neg = false;
            self._analysis = before_analysis.copy();
            self.visit(cond);
            self._analysis.clone()
        }
    }

    pub fn _negated(&mut self) {
        self._neg = !self._neg;
        // yield
        // self._neg = ! self._neg
    }

    pub fn visitFunctionCallExpr(&mut self, ast: FunctionCallExpr) {
        if isinstance(ast.func, BuiltinFunction) {
            let op = ast.func.op;
            if op == "!" {
                self._negated();
                self.visit(ast.args[0]);
                self._negated();
            } else if (op == "&&" && !self._neg) || (op == "||" && self._neg) {
                self.visit(ast.args[0]);
                self.visit(ast.args[1]);
            } else if op == "parenthesis" {
                self.visit(ast.args[0])
            } else if (op == "==" && !self._neg) || (op == "!=" && self._neg) {
                recursive_merge(ast.args[0], ast.args[1], self._analysis)
            }
        }
    }
}
pub fn _recursive_update(lhs: AST, rhs: AST, analysis: PartitionState, merge: bool) {
    if isinstance(lhs, TupleExpr) && isinstance(rhs, TupleExpr) {
        for (l, r) in lhs.elements.iter().zip(rhs.elements) {
            _recursive_update(l, r, analysis, merge);
        }
    } else {
        lhs = lhs.privacy_annotation_label();
        rhs = rhs.privacy_annotation_label();
        if lhs && rhs && analysis.has(rhs) {
            if merge {
                analysis.merge(lhs, rhs);
            } else {
                analysis.move_to(lhs, rhs);
            }
        } else if lhs {
            analysis.move_to_separate(lhs);
        }
    }
}
pub fn recursive_merge(lhs: AST, rhs: AST, analysis: PartitionState) {
    _recursive_update(lhs, rhs, analysis, true);
}

pub fn recursive_assign(lhs: AST, rhs: AST, analysis: PartitionState) {
    _recursive_update(lhs, rhs, analysis, false);
}
