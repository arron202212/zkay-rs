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
    is_instance, ASTBaseProperty, ASTFlatten, ASTInstanceOf, ASTType, AllExpr, AssignmentStatement,
    AssignmentStatementBaseProperty, Block, BreakStatement, BuiltinFunction,
    ConstructorOrFunctionDefinition, ContinueStatement, DoWhileStatement, Expression,
    ExpressionStatement, ForStatement, FunctionCallExpr, FunctionCallExprBaseProperty,
    IdentifierDeclarationBaseProperty, IfStatement, IntoAST, IntoExpression, LocationExpr, MeExpr,
    RequireStatement, ReturnStatement, SimpleStatement, Statement, StatementBaseMutRef,
    StatementBaseProperty, StatementBaseRef, StatementList, StatementListBaseProperty, TupleExpr,
    VariableDeclarationStatement, WhileStatement, AST,
};
use crate::global_defs::{array_length_member, global_defs, global_vars, GlobalDefs, GlobalVars};
use crate::visitor::visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef};
use rccell::RcCell;
use zkay_derive::ASTVisitorBaseRefImpl;
pub fn alias_analysis(ast: &ASTFlatten, global_vars: RcCell<GlobalVars>) {
    let mut v = AliasAnalysisVisitor::new(false, global_vars);
    v.visit(ast);
}
#[derive(ASTVisitorBaseRefImpl)]
struct AliasAnalysisVisitor {
    pub ast_visitor_base: AstVisitorBase,
    pub cond_analyzer: GuardConditionAnalyzer,
    global_vars: RcCell<GlobalVars>,
}
// class AliasAnalysisVisitor(AstVisitor)

impl AstVisitor for AliasAnalysisVisitor {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}
    fn has_attr(&self, ast: &AST) -> bool {
        matches!(
            ast.get_ast_type(),
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
        ) || matches!(ast.to_ast(), AST::Statement(Statement::StatementList(_)))
            || matches!(
                ast.to_ast(),
                AST::Statement(Statement::SimpleStatement(
                    SimpleStatement::AssignmentStatement(_)
                ))
            )
            || matches!(ast.to_ast(), AST::Statement(_))
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> eyre::Result<Self::Return> {
        match name {
            ASTType::ConstructorOrFunctionDefinition => {
                self.visitConstructorOrFunctionDefinition(ast)
            }
            _ if matches!(ast.to_ast(), AST::Statement(Statement::StatementList(_))) => {
                self.visitStatementList(ast)
            }
            ASTType::Block => self.visitBlock(ast),
            ASTType::IfStatement => self.visitIfStatement(ast),

            ASTType::WhileStatement => self.visitWhileStatement(ast),
            ASTType::DoWhileStatement => self.visitDoWhileStatement(ast),
            ASTType::ForStatement => self.visitForStatement(ast),
            ASTType::VariableDeclarationStatement => self.visitVariableDeclarationStatement(ast),
            ASTType::RequireStatement => self.visitRequireStatement(ast),

            _ if matches!(
                ast.to_ast(),
                AST::Statement(Statement::SimpleStatement(
                    SimpleStatement::AssignmentStatement(_)
                ))
            ) =>
            {
                self.visitAssignmentStatement(ast)
            }

            ASTType::ExpressionStatement => self.visitExpressionStatement(ast),
            ASTType::ReturnStatement => self.visitReturnStatement(ast),
            ASTType::ContinueStatement => self.visitContinueStatement(ast),
            ASTType::BreakStatement => self.visitBreakStatement(ast),
            _ if matches!(ast.to_ast(), AST::Statement(_)) => self.visitStatement(ast),
            _ => Err(eyre::eyre!("unreach")),
        }
    }
}
impl AliasAnalysisVisitor {
    // pub fn __init__(&self, log=False)
    //     super().__init__("node-or-children", log)
    //     self.cond_analyzer = GuardConditionAnalyzer()
    pub fn new(log: bool, global_vars: RcCell<GlobalVars>) -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", log),
            cond_analyzer: GuardConditionAnalyzer::new(log),
            global_vars,
        }
    }
    pub fn visitConstructorOrFunctionDefinition(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("{:?},{:?}",ast,ast.to_string());
        let mut s: PartitionState<AST> = PartitionState::new();
        s.insert(
            MeExpr::new()
                .into_expr()
                .privacy_annotation_label()
                .unwrap()
                .to_ast(),
        );
        s.insert(
            AllExpr::new()
                .into_expr()
                .privacy_annotation_label()
                .unwrap()
                .to_ast(),
        );
        // println!("==ast.to_ast().ast_base_ref().borrow().parant====================={:?}",ast.to_ast().ast_base_ref().unwrap().borrow().parent.clone().unwrap().upgrade().unwrap().get_ast_type());
        for d in &ast
            .to_ast()
            .ast_base_ref()
            .unwrap()
            .borrow()
            .parent
            .clone()
            .unwrap()
            .upgrade()
            .clone()
            .unwrap()
            .try_as_contract_definition_ref()
            .unwrap()
            .borrow()
            .state_variable_declarations
        {
            // println!("=====d========{:?}",d);
            s.insert(
                d.try_as_state_variable_declaration_ref()
                    .unwrap()
                    .borrow()
                    .idf()
                    .upgrade()
                    .unwrap()
                    .to_ast(),
            );
        }
        for p in &ast
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .borrow()
            .parameters
        {
            s.insert(
                p.borrow()
                    .identifier_declaration_base
                    .idf
                    .as_ref()
                    .unwrap()
                    .clone()
                    .to_ast(),
            );
        }
        ast.try_as_constructor_or_function_definition_ref()
            .unwrap()
            .borrow_mut()
            .body
            .as_mut()
            .unwrap()
            .borrow_mut()
            .statement_base_mut_ref()
            .before_analysis = Some(s);
        self.visit(
            &ast.try_as_constructor_or_function_definition_ref()
                .unwrap()
                .borrow()
                .body
                .clone()
                .unwrap()
                .into(),
        );
        Ok(())
    }

    pub fn propagate(
        &self,
        statements: &Vec<ASTFlatten>,
        before_analysis: &PartitionState<AST>,
    ) -> PartitionState<AST> {
        let mut last = before_analysis.clone();
        // push state through each statement
        for statement in statements {
            // println!("{:?}",statement);
            statement
                .try_as_ast_ref()
                .unwrap()
                .borrow_mut()
                .try_as_statement_mut()
                .unwrap()
                .statement_base_mut_ref()
                .unwrap()
                .before_analysis = Some(last.clone());
            // print!("before  {:?},{:?}", statement, last);
            self.visit(&statement);
            last = statement
                .to_ast()
                .try_as_statement_ref()
                .unwrap()
                .statement_base_ref()
                .unwrap()
                .after_analysis
                .as_ref()
                .unwrap()
                .clone();
            // print!("after {:?},{:?}", statement, last);
        }

        last
    }
    pub fn visitStatementList(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("============={:?}=================",ast);
        let aa = Some(
            self.propagate(
                &ast.to_ast()
                    .try_as_statement_ref()
                    .unwrap()
                    .try_as_statement_list_ref()
                    .unwrap()
                    .statements()
                    .clone(),
                ast.to_ast()
                    .try_as_statement_ref()
                    .unwrap()
                    .try_as_statement_list_ref()
                    .unwrap()
                    .before_analysis()
                    .as_ref()
                    .unwrap(),
            ),
        );
        if ast.is_block() {
            ast.try_as_block_ref()
                .unwrap()
                .borrow_mut()
                .statement_base_mut_ref()
                .after_analysis = aa;
        } else if ast.is_ast() {
            ast.try_as_ast_ref()
                .unwrap()
                .borrow_mut()
                .try_as_statement_mut()
                .unwrap()
                .try_as_statement_list_mut()
                .unwrap()
                .try_as_block_mut()
                .unwrap()
                .statement_base_mut_ref()
                .after_analysis = aa;
        }

        Ok(())
    }
    pub fn visitBlock(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        // add fresh names from this block
        for name in ast
            .try_as_block_ref()
            .unwrap()
            .borrow()
            .statement_list_base
            .statement_base
            .ast_base
            .borrow()
            .names()
            .values()
        {
            ast.try_as_block_ref()
                .unwrap()
                .borrow_mut()
                .statement_list_base
                .statement_base
                .before_analysis
                .as_mut()
                .unwrap()
                .insert(name.upgrade().unwrap().to_ast());
        }

        ast.try_as_block_ref()
            .unwrap()
            .borrow_mut()
            .statement_list_base
            .statement_base
            .after_analysis = Some(
            self.propagate(
                &ast.try_as_block_ref()
                    .unwrap()
                    .borrow()
                    .statement_list_base
                    .statements,
                ast.try_as_block_ref()
                    .unwrap()
                    .borrow_mut()
                    .statement_list_base
                    .statement_base
                    .before_analysis
                    .as_ref()
                    .unwrap(),
            ),
        );

        // remove names falling out of scope
        for name in ast
            .try_as_block_ref()
            .unwrap()
            .borrow()
            .statement_list_base
            .statement_base
            .ast_base
            .borrow()
            .names()
            .values()
        {
            ast.try_as_block_ref()
                .unwrap()
                .borrow_mut()
                .statement_list_base
                .statement_base
                .after_analysis
                .as_mut()
                .unwrap()
                .remove(&name.upgrade().unwrap().to_ast());
        }
        Ok(())
    }
    pub fn visitIfStatement(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("=======visitIfStatement================={:?}====",ast);
        // condition
        let before_then = self.cond_analyzer.analyze(
            &ast.to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_if_statement_ref()
                .unwrap()
                .condition
                .clone()
                .into(),
            ast.to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_if_statement_ref()
                .unwrap()
                .statement_base
                .before_analysis
                .as_ref()
                .unwrap(),
        );

        // then
        ast.try_as_ast_ref()
            .unwrap()
            .borrow_mut()
            .try_as_statement_mut()
            .unwrap()
            .try_as_if_statement_mut()
            .unwrap()
            .then_branch
            .borrow_mut()
            .statement_list_base
            .statement_base
            .before_analysis = Some(before_then);
        self.visit(
            &ast.to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_if_statement_ref()
                .unwrap()
                .then_branch
                .clone()
                .into(),
        );
        let after_then = ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_if_statement_ref()
            .unwrap()
            .then_branch
            .borrow()
            .statement_list_base
            .statement_base
            .after_analysis
            .clone();

        // else
        let after_else = if ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_if_statement_ref()
            .unwrap()
            .else_branch
            .is_some()
        {
            ast.try_as_ast_ref()
                .unwrap()
                .borrow_mut()
                .try_as_statement_mut()
                .unwrap()
                .try_as_if_statement_mut()
                .unwrap()
                .else_branch
                .as_mut()
                .unwrap()
                .borrow_mut()
                .statement_base_mut_ref()
                .before_analysis = Some(
                self.cond_analyzer.analyze(
                    &RcCell::new(
                        ast.to_ast()
                            .try_as_statement_ref()
                            .unwrap()
                            .try_as_if_statement_ref()
                            .unwrap()
                            .condition
                            .try_as_expression_ref()
                            .unwrap()
                            .borrow()
                            .unop(String::from("!")),
                    )
                    .into(),
                    ast.to_ast()
                        .try_as_statement_ref()
                        .unwrap()
                        .try_as_if_statement_ref()
                        .unwrap()
                        .statement_base
                        .before_analysis
                        .as_ref()
                        .unwrap(),
                ),
            );
            self.visit(
                &ast.to_ast()
                    .try_as_statement_ref()
                    .unwrap()
                    .try_as_if_statement_ref()
                    .unwrap()
                    .else_branch
                    .clone()
                    .unwrap()
                    .into(),
            );
            ast.to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_if_statement_ref()
                .unwrap()
                .else_branch
                .as_ref()
                .unwrap()
                .borrow()
                .statement_list_base
                .statement_base
                .after_analysis
                .clone()
        } else {
            ast.to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_if_statement_ref()
                .unwrap()
                .statement_base
                .before_analysis
                .clone()
        };

        // join branches
        ast.try_as_ast_ref()
            .unwrap()
            .borrow_mut()
            .try_as_statement_mut()
            .unwrap()
            .try_as_if_statement_mut()
            .unwrap()
            .statement_base
            .after_analysis = Some(after_then.unwrap().join(&after_else.unwrap()));
        Ok(())
    }
    // Body always executes after the condition, but it is also possible that it is not executed at all
    // Condition is true before the body
    // After the loop, the condition is false
    pub fn visitWhileStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("===visitWhileStatement================{:?}",ast);
        let ws = ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_while_statement_ref()
            .unwrap()
            .clone();
        if has_side_effects(&ws.condition.clone().into())
            || has_side_effects(&ws.body.clone().into())
        {
            ast.try_as_ast_ref()
                .unwrap()
                .borrow_mut()
                .try_as_statement_mut()
                .unwrap()
                .try_as_while_statement_mut()
                .unwrap()
                .statement_base
                .before_analysis = Some(
                ws.statement_base
                    .before_analysis
                    .clone()
                    .unwrap()
                    .separate_all(),
            );
        }

        ast.try_as_ast_ref()
            .unwrap()
            .borrow_mut()
            .try_as_statement_mut()
            .unwrap()
            .try_as_while_statement_mut()
            .unwrap()
            .body
            .borrow_mut()
            .statement_list_base
            .statement_base
            .before_analysis = Some(self.cond_analyzer.analyze(
            &ws.condition.clone().into(),
            ws.statement_base.before_analysis.as_ref().unwrap(),
        ));
        self.visit(&ws.body.clone().into());

        // Either no loop iteration or at least one loop iteration
        let skip_loop = self.cond_analyzer.analyze(
            &RcCell::new(
                ws.condition
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .unop(String::from("!")),
            )
            .into(),
            ws.statement_base.before_analysis.as_ref().unwrap(),
        );
        let did_loop = self.cond_analyzer.analyze(
            &RcCell::new(
                ws.condition
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .unop(String::from("!")),
            )
            .into(),
            ws.body
                .borrow()
                .statement_list_base
                .statement_base
                .after_analysis
                .as_ref()
                .unwrap(),
        );

        // join
        ast.try_as_ast_ref()
            .unwrap()
            .borrow_mut()
            .try_as_statement_mut()
            .unwrap()
            .try_as_while_statement_mut()
            .unwrap()
            .statement_base
            .after_analysis = Some(skip_loop.join(&did_loop));
        Ok(())
    }
    // Body either executes with or without condition, but it is also possible that it is not executed at all
    // No information about condition before the body
    // After the loop, the condition is false

    // Could be subsequent loop iteration after condition with side effect
    pub fn visitDoWhileStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let dws = ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_do_while_statement_ref()
            .unwrap()
            .clone();
        let cond_se = has_side_effects(&dws.condition.clone().into());
        if cond_se || has_side_effects(&dws.body.clone().into()) {
            ast.try_as_ast_ref()
                .unwrap()
                .borrow_mut()
                .try_as_statement_mut()
                .unwrap()
                .try_as_do_while_statement_mut()
                .unwrap()
                .statement_base
                .before_analysis = Some(
                dws.statement_base
                    .before_analysis
                    .as_ref()
                    .unwrap()
                    .separate_all(),
            );
        }

        ast.try_as_ast_ref()
            .unwrap()
            .borrow_mut()
            .try_as_statement_mut()
            .unwrap()
            .try_as_do_while_statement_mut()
            .unwrap()
            .body
            .borrow_mut()
            .statement_list_base
            .statement_base
            .before_analysis = dws.statement_base.before_analysis.clone();
        self.visit(&dws.body.clone().into());

        // ast.before_analysis is only used by expressions inside condition -> body has already happened at that point
        ast.try_as_ast_ref()
            .unwrap()
            .borrow_mut()
            .try_as_statement_mut()
            .unwrap()
            .try_as_do_while_statement_mut()
            .unwrap()
            .statement_base
            .before_analysis = dws
            .body
            .borrow()
            .statement_list_base
            .statement_base
            .after_analysis
            .clone();
        ast.try_as_ast_ref()
            .unwrap()
            .borrow_mut()
            .try_as_statement_mut()
            .unwrap()
            .try_as_do_while_statement_mut()
            .unwrap()
            .statement_base
            .after_analysis = Some(
            self.cond_analyzer.analyze(
                &RcCell::new(
                    dws.condition
                        .try_as_expression_ref()
                        .unwrap()
                        .borrow()
                        .unop(String::from("!")),
                )
                .into(),
                dws.statement_base.before_analysis.as_ref().unwrap(),
            ),
        );
        Ok(())
    }
    pub fn visitForStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let fs = ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_for_statement_ref()
            .unwrap()
            .clone();
        let mut last = fs.statement_base.before_analysis.clone().unwrap();

        // add names introduced in init
        for name in fs.statement_base.ast_base.borrow().names().values() {
            last.insert(name.upgrade().unwrap().clone().to_ast());
        }

        if fs.init.is_some() {
            ast.try_as_ast_ref()
                .unwrap()
                .borrow_mut()
                .try_as_statement_mut()
                .unwrap()
                .try_as_for_statement_mut()
                .unwrap()
                .init
                .as_mut()
                .unwrap()
                .borrow_mut()
                .statement_base_mut_ref()
                .before_analysis = Some(last.clone());
            self.visit(&fs.init.clone().unwrap().into());
            ast.try_as_ast_ref()
                .unwrap()
                .borrow_mut()
                .try_as_statement_mut()
                .unwrap()
                .try_as_for_statement_mut()
                .unwrap()
                .statement_base
                .before_analysis = fs.init.as_ref().unwrap().borrow().after_analysis().clone();
            // init should be taken into account when looking up things in the condition
        }
        if has_side_effects(&fs.condition.clone().into())
            || has_side_effects(&fs.body.clone().into())
            || (fs.update.is_some()
                && has_side_effects(&fs.update.as_ref().unwrap().clone().into()))
        {
            ast.try_as_ast_ref()
                .unwrap()
                .borrow_mut()
                .try_as_statement_mut()
                .unwrap()
                .try_as_for_statement_mut()
                .unwrap()
                .statement_base
                .before_analysis = Some(last.separate_all());
        }
        ast.try_as_ast_ref()
            .unwrap()
            .borrow_mut()
            .try_as_statement_mut()
            .unwrap()
            .try_as_for_statement_mut()
            .unwrap()
            .body
            .borrow_mut()
            .statement_list_base
            .statement_base
            .before_analysis = Some(self.cond_analyzer.analyze(
            &fs.condition.clone().into(),
            fs.statement_base.before_analysis.as_ref().unwrap(),
        ));
        self.visit(&fs.body.clone().into());
        // Update is always executed after the body (if it is executed)
        if let Some(update) = &fs.update {
            update.borrow_mut().statement_base_mut_ref().before_analysis = fs
                .body
                .borrow()
                .statement_list_base
                .statement_base
                .after_analysis
                .clone();
            self.visit(&update.clone().into());
        }

        let skip_loop = self.cond_analyzer.analyze(
            &RcCell::new(
                fs.condition
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .unop(String::from("!")),
            )
            .into(),
            fs.init
                .as_ref()
                .unwrap()
                .borrow()
                .after_analysis()
                .as_ref()
                .unwrap(),
        );
        let after_analysis = if let Some(update) = &fs.update {
            update.borrow().after_analysis().clone().unwrap()
        } else {
            fs.body
                .borrow()
                .statement_list_base
                .statement_base
                .after_analysis
                .clone()
                .unwrap()
        };
        let did_loop = self.cond_analyzer.analyze(
            &RcCell::new(
                fs.condition
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .unop(String::from("!")),
            )
            .into(),
            &after_analysis,
        );

        // join
        ast.try_as_ast_ref()
            .unwrap()
            .borrow_mut()
            .try_as_statement_mut()
            .unwrap()
            .try_as_for_statement_mut()
            .unwrap()
            .statement_base
            .after_analysis = Some(skip_loop.join(&did_loop));

        // drop names introduced in init
        for name in fs.statement_base.ast_base.borrow().names().values() {
            ast.try_as_ast_ref()
                .unwrap()
                .borrow_mut()
                .try_as_statement_mut()
                .unwrap()
                .try_as_for_statement_mut()
                .unwrap()
                .statement_base
                .after_analysis
                .as_mut()
                .unwrap()
                .remove(&name.upgrade().unwrap().clone().to_ast());
        }
        Ok(())
    }
    pub fn visitVariableDeclarationStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("=======visitVariableDeclarationStatement================={:?}====",ast);
        let e = &ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_simple_statement_ref()
            .unwrap()
            .try_as_variable_declaration_statement_ref()
            .unwrap()
            .expr
            .clone();
        if e.is_some() && has_side_effects(&e.as_ref().unwrap().clone().into()) {
            ast.try_as_ast_ref()
                .unwrap()
                .borrow_mut()
                .try_as_statement_mut()
                .unwrap()
                .try_as_simple_statement_mut()
                .unwrap()
                .try_as_variable_declaration_statement_mut()
                .unwrap()
                .simple_statement_base
                .statement_base
                .before_analysis = Some(
                ast.to_ast()
                    .try_as_statement_ref()
                    .unwrap()
                    .try_as_simple_statement_ref()
                    .unwrap()
                    .try_as_variable_declaration_statement_ref()
                    .unwrap()
                    .simple_statement_base
                    .statement_base
                    .before_analysis
                    .as_ref()
                    .unwrap()
                    .separate_all(),
            );
        }

        // visit expression
        if let Some(e) = e {
            self.visit(&e.clone().into());
        }

        // state after declaration
        let after = ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_simple_statement_ref()
            .unwrap()
            .try_as_variable_declaration_statement_ref()
            .unwrap()
            .simple_statement_base
            .statement_base
            .before_analysis
            .clone();

        // name of variable is already in list
        let name = ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_simple_statement_ref()
            .unwrap()
            .try_as_variable_declaration_statement_ref()
            .unwrap()
            .variable_declaration
            .borrow()
            .identifier_declaration_base
            .idf
            .clone();
        assert!(after
            .as_ref()
            .unwrap()
            .has(&name.as_ref().unwrap().clone().to_ast()));

        // make state more precise
        if let Some(e) = e {
            if let Some(pal) = e
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .privacy_annotation_label()
            {
                after
                    .clone()
                    .unwrap()
                    .merge(&name.as_ref().unwrap().clone().to_ast(), &pal.to_ast());
            }
        }

        ast.try_as_ast_ref()
            .unwrap()
            .borrow_mut()
            .try_as_statement_mut()
            .unwrap()
            .try_as_simple_statement_mut()
            .unwrap()
            .try_as_variable_declaration_statement_mut()
            .unwrap()
            .simple_statement_base
            .statement_base
            .after_analysis = after;
        Ok(())
    }
    pub fn visitRequireStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        if has_side_effects(
            &ast.try_as_ast_ref()
                .unwrap()
                .borrow()
                .try_as_statement_ref()
                .unwrap()
                .try_as_simple_statement_ref()
                .unwrap()
                .try_as_require_statement_ref()
                .unwrap()
                .condition
                .clone()
                .into(),
        ) {
            ast.try_as_ast_ref()
                .unwrap()
                .borrow_mut()
                .try_as_statement_mut()
                .unwrap()
                // .try_as_simple_statement_ref()
                // .unwrap().try_as_require_statement_ref()
                // .unwrap()
                // .simple_statement_base
                .statement_base_mut_ref()
                .unwrap()
                .before_analysis = Some(
                ast.try_as_ast_ref()
                    .unwrap()
                    .borrow()
                    .try_as_statement_ref()
                    .unwrap()
                    .try_as_simple_statement_ref()
                    .unwrap()
                    .try_as_require_statement_ref()
                    .unwrap()
                    .simple_statement_base
                    .statement_base
                    .before_analysis
                    .as_ref()
                    .unwrap()
                    .separate_all(),
            );
        }

        self.visit(
            &ast.try_as_ast_ref()
                .unwrap()
                .borrow()
                .try_as_statement_ref()
                .unwrap()
                .try_as_simple_statement_ref()
                .unwrap()
                .try_as_require_statement_ref()
                .unwrap()
                .condition
                .clone()
                .into(),
        );

        // state after require
        let mut after = ast
            .try_as_ast_ref()
            .unwrap()
            .borrow()
            .try_as_statement_ref()
            .unwrap()
            .try_as_simple_statement_ref()
            .unwrap()
            .try_as_require_statement_ref()
            .unwrap()
            .simple_statement_base
            .statement_base
            .before_analysis
            .clone();

        // make state more precise
        let c = &ast
            .try_as_ast_ref()
            .unwrap()
            .borrow()
            .try_as_statement_ref()
            .unwrap()
            .try_as_simple_statement_ref()
            .unwrap()
            .try_as_require_statement_ref()
            .unwrap()
            .condition
            .clone();
        if is_instance(c, ASTType::FunctionCallExprBase)
            && is_instance(
                c.try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .func(),
                ASTType::BuiltinFunction,
            )
            && &c
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .try_as_function_call_expr_ref()
                .unwrap()
                .func()
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .try_as_builtin_function_ref()
                .unwrap()
                .op
                == "=="
        {
            let lhs = c
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .try_as_function_call_expr_ref()
                .unwrap()
                .args()[0]
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .privacy_annotation_label();
            let rhs = c
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .try_as_function_call_expr_ref()
                .unwrap()
                .args()[1]
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .privacy_annotation_label();
            if lhs.is_some() && rhs.is_some() {
                after.as_mut().unwrap().merge(
                    &lhs.clone().unwrap().to_ast(),
                    &rhs.clone().unwrap().to_ast(),
                );
            }
        }

        ast.try_as_ast_ref()
            .unwrap()
            .borrow_mut()
            .try_as_statement_mut()
            .unwrap()
            .try_as_simple_statement_mut()
            .unwrap()
            .try_as_require_statement_mut()
            .unwrap()
            .simple_statement_base
            .statement_base
            .after_analysis = after;
        Ok(())
    }
    pub fn visitAssignmentStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("====visitAssignmentStatement====================={:?}",ast);
        if has_side_effects(
            &ast.try_as_ast_ref()
                .unwrap()
                .borrow()
                .try_as_statement_ref()
                .unwrap()
                .try_as_simple_statement_ref()
                .unwrap()
                .try_as_assignment_statement_ref()
                .unwrap()
                .lhs()
                .clone()
                .unwrap()
                .into(),
        ) || has_side_effects(
            &ast.try_as_ast_ref()
                .unwrap()
                .borrow()
                .try_as_statement_ref()
                .unwrap()
                .try_as_simple_statement_ref()
                .unwrap()
                .try_as_assignment_statement_ref()
                .unwrap()
                .rhs()
                .clone()
                .unwrap()
                .into(),
        ) {
            let ba = Some(
                ast.try_as_ast_ref()
                    .unwrap()
                    .borrow()
                    .try_as_statement_ref()
                    .unwrap()
                    .try_as_simple_statement_ref()
                    .unwrap()
                    .try_as_assignment_statement_ref()
                    .unwrap()
                    .before_analysis()
                    .as_ref()
                    .unwrap()
                    .separate_all(),
            );
            ast.try_as_ast_ref()
                .unwrap()
                .borrow_mut()
                .try_as_statement_mut()
                .unwrap()
                .try_as_simple_statement_mut()
                .unwrap()
                .try_as_assignment_statement_mut()
                .unwrap()
                .statement_base_mut_ref()
                .before_analysis = ba;
        }
        let lhs = ast
            .try_as_ast_ref()
            .unwrap()
            .borrow()
            .try_as_statement_ref()
            .unwrap()
            .try_as_simple_statement_ref()
            .unwrap()
            .try_as_assignment_statement_ref()
            .unwrap()
            .lhs()
            .clone();
        let rhs = ast
            .try_as_ast_ref()
            .unwrap()
            .borrow()
            .try_as_statement_ref()
            .unwrap()
            .try_as_simple_statement_ref()
            .unwrap()
            .try_as_assignment_statement_ref()
            .unwrap()
            .rhs()
            .clone();
        // visit expression
        self.visit(&lhs.clone().unwrap().into());
        self.visit(&rhs.clone().unwrap().into());

        // state after assignment
        let after = ast
            .try_as_ast_ref()
            .unwrap()
            .borrow()
            .try_as_statement_ref()
            .unwrap()
            .try_as_simple_statement_ref()
            .unwrap()
            .try_as_assignment_statement_ref()
            .unwrap()
            .before_analysis()
            .clone();
        recursive_assign(
            &lhs.as_ref().unwrap().clone().into(),
            &rhs.as_ref().unwrap().clone().into(),
            after.clone().unwrap(),
        );

        // save state
        ast.try_as_ast_ref()
            .unwrap()
            .borrow_mut()
            .try_as_statement_mut()
            .unwrap()
            .try_as_simple_statement_mut()
            .unwrap()
            .try_as_assignment_statement_mut()
            .unwrap()
            .statement_base_mut_ref()
            .after_analysis = after.clone();
        Ok(())
    }
    pub fn visitExpressionStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("======visitExpressionStatement======================={:?}",ast);
        if has_side_effects(
            &ast.to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_simple_statement_ref()
                .unwrap()
                .try_as_expression_statement_ref()
                .unwrap()
                .expr
                .clone()
                .into(),
        ) {
            ast.try_as_ast_ref()
                .unwrap()
                .borrow_mut()
                .try_as_statement_mut()
                .unwrap()
                .try_as_simple_statement_mut()
                .unwrap()
                .try_as_expression_statement_mut()
                .unwrap()
                .simple_statement_base
                .statement_base
                .before_analysis = Some(
                ast.to_ast()
                    .try_as_statement_ref()
                    .unwrap()
                    .try_as_simple_statement_ref()
                    .unwrap()
                    .try_as_expression_statement_ref()
                    .unwrap()
                    .simple_statement_base
                    .statement_base
                    .before_analysis
                    .as_ref()
                    .unwrap()
                    .separate_all(),
            );
        }

        // visit expression
        self.visit(
            &ast.to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_simple_statement_ref()
                .unwrap()
                .try_as_expression_statement_ref()
                .unwrap()
                .expr
                .clone()
                .into(),
        );

        // if expression has effect, we are already at TOP
        ast.try_as_ast_ref()
            .unwrap()
            .borrow_mut()
            .try_as_statement_mut()
            .unwrap()
            .try_as_simple_statement_mut()
            .unwrap()
            .try_as_expression_statement_mut()
            .unwrap()
            .simple_statement_base
            .statement_base
            .after_analysis = ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_simple_statement_ref()
            .unwrap()
            .try_as_expression_statement_ref()
            .unwrap()
            .simple_statement_base
            .statement_base
            .before_analysis
            .clone();
        Ok(())
    }
    pub fn visitReturnStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("========visitReturnStatement============={:?}",ast);
        let ba = ast
            .try_as_ast_ref()
            .unwrap()
            .borrow()
            .try_as_statement_ref()
            .unwrap()
            .try_as_return_statement_ref()
            .unwrap()
            .statement_base
            .before_analysis
            .clone();
        ast.try_as_ast_ref()
            .unwrap()
            .borrow_mut()
            .try_as_statement_mut()
            .unwrap()
            .try_as_return_statement_mut()
            .unwrap()
            .statement_base
            .after_analysis = ba;
        Ok(())
    }

    pub fn visitContinueStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        ast.try_as_ast_ref()
            .unwrap()
            .borrow_mut()
            .try_as_statement_mut()
            .unwrap()
            .try_as_continue_statement_mut()
            .unwrap()
            .statement_base
            .after_analysis = ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_continue_statement_ref()
            .unwrap()
            .statement_base
            .before_analysis
            .clone();
        Ok(())
    }

    pub fn visitBreakStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        ast.try_as_ast_ref()
            .unwrap()
            .borrow_mut()
            .try_as_statement_mut()
            .unwrap()
            .try_as_break_statement_mut()
            .unwrap()
            .statement_base
            .after_analysis = ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_break_statement_ref()
            .unwrap()
            .statement_base
            .before_analysis
            .clone();
        Ok(())
    }

    pub fn visitStatement(&self, _: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        // raise NotImplementedError();
        // unimplemented!();
        Err(eyre::eyre!("unimplemented"))
    }
}
#[derive(ASTVisitorBaseRefImpl)]
pub struct GuardConditionAnalyzer {
    pub ast_visitor_base: AstVisitorBase,
    _neg: RcCell<bool>,
    _analysis: RcCell<Option<PartitionState<AST>>>,
}
impl AstVisitor for GuardConditionAnalyzer {
    type Return = ();
    fn temper_result(&self) -> Self::Return {}

    fn has_attr(&self, ast: &AST) -> bool {
        matches!(ast, AST::Expression(Expression::FunctionCallExpr(_)))
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> eyre::Result<Self::Return> {
        match name {
            _ if matches!(
                ast.to_ast(),
                AST::Expression(Expression::FunctionCallExpr(_))
            ) =>
            {
                self.visitFunctionCallExpr(ast)
            }

            _ => Err(eyre::eyre!("unreach")),
        }
    }
}

impl GuardConditionAnalyzer {
    pub fn new(log: bool) -> Self {
        Self {
            ast_visitor_base: AstVisitorBase::new("node-or-children", log),
            _neg: RcCell::new(false),
            _analysis: RcCell::new(None),
        }
    }
    pub fn analyze(
        &self,
        cond: &ASTFlatten,
        before_analysis: &PartitionState<AST>,
    ) -> PartitionState<AST> {
        if has_side_effects(cond) {
            before_analysis.separate_all()
        } else {
            *self._neg.borrow_mut() = false;
            *self._analysis.borrow_mut() = Some(before_analysis.clone());
            self.visit(cond);
            self._analysis.borrow().clone().unwrap()
        }
    }

    pub fn _negated(&self) {
        let v = !*self._neg.borrow();
        *self._neg.borrow_mut() = v;
        // yield
        // self._neg = ! self._neg
    }

    pub fn visitFunctionCallExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("======visitFunctionCallExpr=========={:?}",ast
        //         .to_ast().try_as_expression_ref().unwrap().try_as_function_call_expr_ref().unwrap()
        //         .func());

        if is_instance(
            ast.to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_function_call_expr_ref()
                .unwrap()
                .func(),
            ASTType::BuiltinFunction,
        ) {
            let args = ast
                .to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_function_call_expr_ref()
                .unwrap()
                .args()
                .clone();
            let op = &ast
                .to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_function_call_expr_ref()
                .unwrap()
                .func()
                .to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_builtin_function_ref()
                .unwrap()
                .op
                .clone();
            if op == "!" {
                self._negated();
                self.visit(&args[0].clone().into());
                self._negated();
            } else if (op == "&&" && !*self._neg.borrow()) || (op == "||" && *self._neg.borrow()) {
                self.visit(&args[0].clone().into());
                self.visit(&args[1].clone().into());
            } else if op == "parenthesis" {
                self.visit(&args[0].clone().into());
            } else if (op == "==" && !*self._neg.borrow()) || (op == "!=" && *self._neg.borrow()) {
                recursive_merge(
                    &args[0].clone().into(),
                    &args[1].clone().into(),
                    self._analysis.borrow().clone().unwrap(),
                );
            }
        }
        Ok(())
    }
}
pub fn _recursive_update(
    lhs: &ASTFlatten,
    rhs: &ASTFlatten,
    mut analysis: PartitionState<AST>,
    merge: bool,
) {
    if is_instance(lhs, ASTType::TupleExpr) && is_instance(rhs, ASTType::TupleExpr) {
        for (l, r) in lhs
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .try_as_tuple_or_location_expr_ref()
            .unwrap()
            .try_as_tuple_expr_ref()
            .unwrap()
            .elements
            .iter()
            .zip(
                &rhs.try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_tuple_expr_ref()
                    .unwrap()
                    .elements,
            )
        {
            _recursive_update(
                &l.clone().into(),
                &r.clone().into(),
                analysis.clone(),
                merge,
            );
        }
    } else {
        let lhs = lhs
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .privacy_annotation_label();
        // println!("=======rhs========={:?}",rhs);
        let rhs = rhs
            .to_ast()
            .try_as_expression_ref()
            .unwrap()
            .privacy_annotation_label();
        if lhs.is_some() && rhs.is_some() && analysis.has(&rhs.clone().unwrap().to_ast()) {
            if merge {
                analysis.merge(&lhs.unwrap().to_ast(), &rhs.unwrap().to_ast());
            } else {
                analysis.move_to(&lhs.unwrap().to_ast(), &rhs.clone().unwrap().to_ast());
            }
        } else if lhs.is_some() {
            analysis.move_to_separate(&lhs.unwrap().to_ast());
        }
    }
}
pub fn recursive_merge(lhs: &ASTFlatten, rhs: &ASTFlatten, analysis: PartitionState<AST>) {
    _recursive_update(lhs, rhs, analysis, true);
}

pub fn recursive_assign(lhs: &ASTFlatten, rhs: &ASTFlatten, analysis: PartitionState<AST>) {
    _recursive_update(lhs, rhs, analysis, false);
}
