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
    AST, ASTBaseProperty, ASTFlatten, ASTInstanceOf, ASTType, DeepClone, IntoAST, IntoExpression,
    expression::{
        AllExpr, BuiltinFunction, Expression, FunctionCallExpr, FunctionCallExprBaseProperty,
        LocationExpr, MeExpr, TupleExpr,
    },
    identifier_declaration::IdentifierDeclarationBaseProperty,
    is_instance,
    namespace_definition::ConstructorOrFunctionDefinition,
    statement::{
        AssignmentStatement, AssignmentStatementBaseProperty, Block, BreakStatement,
        ContinueStatement, DoWhileStatement, ExpressionStatement, ForStatement, IfStatement,
        RequireStatement, ReturnStatement, SimpleStatement, Statement, StatementBaseMutRef,
        StatementBaseProperty, StatementBaseRef, StatementList, StatementListBaseProperty,
        VariableDeclarationStatement, WhileStatement,
    },
};
use crate::global_defs::{GlobalDefs, GlobalVars, array_length_member, global_defs, global_vars};
use crate::visitors::visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef};
use rccell::RcCell;
use zkay_config::with_context_block;
use zkay_derive::ASTVisitorBaseRefImpl;
pub fn alias_analysis(ast: &ASTFlatten, global_vars: RcCell<GlobalVars>) {
    let v = AliasAnalysisVisitor::new(false, global_vars);
    let _ = v.visit(ast);
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
    fn has_attr(&self, name: &ASTType, ast: &AST) -> bool {
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

            ASTType::Block => self.visitBlock(ast),
            ASTType::IfStatement => self.visitIfStatement(ast),

            ASTType::WhileStatement => self.visitWhileStatement(ast),
            ASTType::DoWhileStatement => self.visitDoWhileStatement(ast),
            ASTType::ForStatement => self.visitForStatement(ast),
            ASTType::VariableDeclarationStatement => self.visitVariableDeclarationStatement(ast),
            ASTType::RequireStatement => self.visitRequireStatement(ast),
            ASTType::ExpressionStatement => self.visitExpressionStatement(ast),
            ASTType::ReturnStatement => self.visitReturnStatement(ast),
            ASTType::ContinueStatement => self.visitContinueStatement(ast),
            ASTType::BreakStatement => self.visitBreakStatement(ast),
            _ if matches!(
                ast.to_ast(),
                AST::Statement(Statement::SimpleStatement(
                    SimpleStatement::AssignmentStatement(_)
                ))
            ) =>
            {
                self.visitAssignmentStatement(ast)
            }
            _ if matches!(ast.to_ast(), AST::Statement(Statement::StatementList(_))) => {
                self.visitStatementList(ast)
            }
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
                    .as_ref()
                    .unwrap()
                    .clone_inner()
                    .to_ast(),
            );
        }
        for p in &ast
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .borrow()
            .parameters
        {
            //   println!("=====p========{:?}",p);
            s.insert(
                p.borrow()
                    .identifier_declaration_base
                    .idf()
                    .as_ref()
                    .unwrap()
                    .clone_inner()
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
        let _ = self.visit(
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
            // println!("===propagate===={:?}=={:?}======{:?}",file!(),line!(),last);
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
            let _ = self.visit(statement);
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
        // println!("===visitBlock========={:?}",ast);
        let mut last = ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .statement_base_ref()
            .unwrap()
            .before_analysis
            .clone()
            .unwrap();
        // add fresh names from this block
        let names: Vec<_> = ast
            .ast_base_ref()
            .unwrap()
            .borrow()
            .names()
            .values()
            .map(|name| name.upgrade().unwrap().to_ast())
            .collect();
        for name in names {
            // println!("{name:?}");
            last.insert(name);
        }
        let aa = Some(
            self.propagate(
                ast.to_ast()
                    .try_as_statement_ref()
                    .unwrap()
                    .try_as_statement_list_ref()
                    .unwrap()
                    .statements(),
                &last,
            ),
        );
        //   println!("{:?},===,{:?}",last.codes() ,aa.as_ref().unwrap().codes());
        if ast.is_block() {
            ast.try_as_block_ref()
                .unwrap()
                .borrow_mut()
                .statement_list_base
                .statement_base
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
                .statement_list_base
                .statement_base
                .after_analysis = aa;
        }
        let names: Vec<_> = ast
            .ast_base_ref()
            .unwrap()
            .borrow()
            .names()
            .values()
            .map(|name| name.upgrade().unwrap().to_ast())
            .collect();
        // remove names falling out of scope
        for name in names {
            //   println!("rm===={:?}",name);
            if ast.is_block() {
                ast.try_as_block_ref()
                    .unwrap()
                    .borrow_mut()
                    .statement_list_base
                    .statement_base
                    .after_analysis
                    .as_mut()
                    .unwrap()
                    .remove(&name);
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
                    .statement_list_base
                    .statement_base
                    .after_analysis
                    .as_mut()
                    .unwrap()
                    .remove(&name);
            }
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
                .condition,
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
        let _ = self.visit(
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
            let _ = self.visit(
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
        if has_side_effects(&ws.condition) || has_side_effects(&ws.body.clone().into()) {
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
            &ws.condition,
            ws.statement_base.before_analysis.as_ref().unwrap(),
        ));
        let _ = self.visit(&ws.body.clone().into());

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
        let cond_se = has_side_effects(&dws.condition);
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
        let _ = self.visit(&dws.body.clone().into());

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
        // println!("==visitForStatement========={:?}==",ast.to_string());
        let mut last = ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_for_statement_ref()
            .unwrap()
            .statement_base
            .before_analysis
            .clone()
            .unwrap();

        // add names introduced in init
        for name in ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_for_statement_ref()
            .unwrap()
            .statement_base
            .ast_base
            .borrow()
            .names()
            .values()
        {
            // println!("==={:?}",name.upgrade().unwrap().clone().to_ast().to_string());
            last.insert(name.upgrade().unwrap().clone().to_ast());
        }

        if ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_for_statement_ref()
            .unwrap()
            .init
            .is_some()
        {
            // println!("===fs.init====={:?}",ast
            // .to_ast()
            // .try_as_statement_ref()
            // .unwrap()
            // .try_as_for_statement_ref()
            // .unwrap().init.as_ref().unwrap().borrow().get_ast_type());
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
            let _ = self.visit(
                &ast.to_ast()
                    .try_as_statement_ref()
                    .unwrap()
                    .try_as_for_statement_ref()
                    .unwrap()
                    .init
                    .clone()
                    .unwrap()
                    .into(),
            );
            // println!("===init===after====={:?}",ast
            // .to_ast()
            // .try_as_statement_ref()
            // .unwrap()
            // .try_as_for_statement_ref()
            // .unwrap().init.as_ref().unwrap().borrow().after_analysis().clone().unwrap().codes());
            ast.try_as_ast_ref()
                .unwrap()
                .borrow_mut()
                .try_as_statement_mut()
                .unwrap()
                .try_as_for_statement_mut()
                .unwrap()
                .statement_base
                .before_analysis = ast
                .to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_for_statement_ref()
                .unwrap()
                .init
                .as_ref()
                .unwrap()
                .borrow()
                .after_analysis()
                .clone();
            // init should be taken into account when looking up things in the condition
        }
        if has_side_effects(
            &ast.to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_for_statement_ref()
                .unwrap()
                .condition,
        ) || has_side_effects(
            &ast.to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_for_statement_ref()
                .unwrap()
                .body
                .clone()
                .into(),
        ) || (ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_for_statement_ref()
            .unwrap()
            .update
            .is_some()
            && has_side_effects(
                &ast.to_ast()
                    .try_as_statement_ref()
                    .unwrap()
                    .try_as_for_statement_ref()
                    .unwrap()
                    .update
                    .clone()
                    .unwrap()
                    .into(),
            ))
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
            // println!("====has_side_effects=========={:?}==",ast.try_as_ast_ref()
            //                 .unwrap()
            //                 .borrow_mut()
            //                 .try_as_statement_mut()
            //                 .unwrap()
            //                 .try_as_for_statement_mut()
            //                 .unwrap()
            //                 .statement_base
            //                 .before_analysis.as_ref().unwrap().codes() );
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
            .before_analysis = Some(
            self.cond_analyzer.analyze(
                &ast.to_ast()
                    .try_as_statement_ref()
                    .unwrap()
                    .try_as_for_statement_ref()
                    .unwrap()
                    .condition,
                ast.to_ast()
                    .try_as_statement_ref()
                    .unwrap()
                    .try_as_for_statement_ref()
                    .unwrap()
                    .statement_base
                    .before_analysis
                    .as_ref()
                    .unwrap(),
            ),
        );
        let _ = self.visit(
            &ast.to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_for_statement_ref()
                .unwrap()
                .body
                .clone()
                .into(),
        );
        // Update is always executed after the body (if it is executed)
        if let Some(update) = &ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_for_statement_ref()
            .unwrap()
            .update
        {
            update.borrow_mut().statement_base_mut_ref().before_analysis = ast
                .to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_for_statement_ref()
                .unwrap()
                .body
                .borrow()
                .statement_list_base
                .statement_base
                .after_analysis
                .clone();
            let _ = self.visit(&update.clone().into());
        }

        let skip_loop = self.cond_analyzer.analyze(
            &RcCell::new(
                ast.to_ast()
                    .try_as_statement_ref()
                    .unwrap()
                    .try_as_for_statement_ref()
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
                .try_as_for_statement_ref()
                .unwrap()
                .init
                .as_ref()
                .unwrap()
                .borrow()
                .after_analysis()
                .as_ref()
                .unwrap(),
        );
        let after_analysis = if let Some(update) = &ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_for_statement_ref()
            .unwrap()
            .update
        {
            update.borrow().after_analysis().clone().unwrap()
        } else {
            ast.to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_for_statement_ref()
                .unwrap()
                .body
                .borrow()
                .statement_list_base
                .statement_base
                .after_analysis
                .clone()
                .unwrap()
        };
        let did_loop = self.cond_analyzer.analyze(
            &RcCell::new(
                ast.to_ast()
                    .try_as_statement_ref()
                    .unwrap()
                    .try_as_for_statement_ref()
                    .unwrap()
                    .condition
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .unop(String::from("!")),
            )
            .into(),
            &after_analysis,
        );
        // println!("{:?},{:?},{:?}",fs.update.is_some() ,fs.condition
        //             .try_as_expression_ref()
        //             .unwrap()
        //             .borrow()
        //             .unop(String::from("!")).code(),after_analysis            ._partitions
        //     .values()
        //     .map(|subset| subset.iter().map(|x|x.to_string()).collect::<Vec<_>>())
        //     .flatten()
        //     .collect::<Vec<_>>());
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
        for name in ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_for_statement_ref()
            .unwrap()
            .statement_base
            .ast_base
            .borrow()
            .names()
            .values()
        {
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
        // println!("=======visitVariableDeclarationStatement=========s=={:?}======{:?}====",ast.to_string(),1);
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
        if e.is_some() && has_side_effects(&e.clone().unwrap()) {
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
            let _ = self.visit(e);
        }

        // state after declaration
        let mut after = ast
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
            .idf()
            .clone_inner();
        //   println!("{:?},=============={:?}",after,name);
        assert!(
            after
                .as_ref()
                .unwrap()
                .has(&name.as_ref().unwrap().to_ast())
        );

        // make state more precise
        if let Some(e) = e {
            if let Some(pal) = e
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .privacy_annotation_label()
            {
                after
                    .as_mut()
                    .unwrap()
                    .merge(&name.as_ref().unwrap().clone().to_ast(), &pal.to_ast());
            }
        }
        // println!("==after====={:?}", after.as_ref().unwrap().codes());
        if ast.is_ast() {
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
        } else if ast.is_simple_statement() {
            ast.try_as_simple_statement_ref()
                .unwrap()
                .borrow_mut()
                .try_as_variable_declaration_statement_mut()
                .unwrap()
                .simple_statement_base
                .statement_base
                .after_analysis = after;
        } else {
            eyre::bail!("====visitVariableDeclarationStatement===================else=========")
        }
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
                .condition,
        ) {
            ast.try_as_ast_ref()
                .unwrap()
                .borrow_mut()
                .try_as_statement_mut()
                .unwrap()
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

        let _ = self.visit(
            &ast.try_as_ast_ref()
                .unwrap()
                .borrow()
                .try_as_statement_ref()
                .unwrap()
                .try_as_simple_statement_ref()
                .unwrap()
                .try_as_require_statement_ref()
                .unwrap()
                .condition,
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
            ast.to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_simple_statement_ref()
                .unwrap()
                .try_as_assignment_statement_ref()
                .unwrap()
                .lhs()
                .as_ref()
                .unwrap(),
        ) || has_side_effects(
            ast.to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_simple_statement_ref()
                .unwrap()
                .try_as_assignment_statement_ref()
                .unwrap()
                .rhs()
                .as_ref()
                .unwrap(),
        ) {
            let ba = Some(
                ast.to_ast()
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
            if ast.is_ast() {
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
            } else if ast.is_simple_statement() {
                ast.try_as_simple_statement_ref()
                    .unwrap()
                    .borrow_mut()
                    .try_as_assignment_statement_mut()
                    .unwrap()
                    .statement_base_mut_ref()
                    .before_analysis = ba;
            } else {
                eyre::bail!("====visitAssignmentStatement===================else=========")
            }
        }
        let lhs = ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_simple_statement_ref()
            .unwrap()
            .try_as_assignment_statement_ref()
            .unwrap()
            .lhs()
            .clone();
        let rhs = ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_simple_statement_ref()
            .unwrap()
            .try_as_assignment_statement_ref()
            .unwrap()
            .rhs()
            .clone();
        // visit expression
        let _ = self.visit(lhs.as_ref().unwrap());
        let _ = self.visit(rhs.as_ref().unwrap());

        // state after assignment
        let after = ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_simple_statement_ref()
            .unwrap()
            .try_as_assignment_statement_ref()
            .unwrap()
            .before_analysis()
            .clone();
        let after = RcCell::new(after);
        recursive_assign(lhs.as_ref().unwrap(), rhs.as_ref().unwrap(), &after);

        // save state
        if ast.is_ast() {
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
                .after_analysis = after.borrow().clone();
        } else if ast.is_simple_statement() {
            ast.try_as_simple_statement_ref()
                .unwrap()
                .borrow_mut()
                .try_as_assignment_statement_mut()
                .unwrap()
                .statement_base_mut_ref()
                .after_analysis = after.borrow().clone();
        } else {
            eyre::bail!("====visitAssignmentStatement===================else=========")
        }
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
                .expr,
        ) {
            let ba = Some(
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
            if ast.is_ast() {
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
                    .before_analysis = ba;
            } else if ast.is_simple_statement() {
                ast.try_as_simple_statement_ref()
                    .unwrap()
                    .borrow_mut()
                    .try_as_expression_statement_mut()
                    .unwrap()
                    .simple_statement_base
                    .statement_base
                    .before_analysis = ba;
            } else {
                eyre::bail!("===========else==============");
            }
        }

        // visit expression
        let _ = self.visit(
            &ast.to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_simple_statement_ref()
                .unwrap()
                .try_as_expression_statement_ref()
                .unwrap()
                .expr,
        );

        // if expression has effect, we are already at TOP
        let aa = ast
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
        if ast.is_ast() {
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
                .after_analysis = aa;
        } else if ast.is_simple_statement() {
            ast.try_as_simple_statement_ref()
                .unwrap()
                .borrow_mut()
                .try_as_expression_statement_mut()
                .unwrap()
                .simple_statement_base
                .statement_base
                .after_analysis = aa;
        }
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

pub struct WithNegated {
    _neg: RcCell<bool>,
}
impl WithNegated {
    pub fn new(_neg: RcCell<bool>) -> Self {
        let v = !*_neg.borrow();
        *_neg.borrow_mut() = v;
        Self { _neg }
    }
}

impl Drop for WithNegated {
    fn drop(&mut self) {
        let v = !*self._neg.borrow();
        *self._neg.borrow_mut() = v;
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

    fn has_attr(&self, _name: &ASTType, ast: &AST) -> bool {
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
            let _ = self.visit(cond);
            self._analysis.borrow().clone().unwrap()
        }
    }

    pub fn _negated(&self) -> WithNegated {
        // let v = !*self._neg.borrow();
        // *self._neg.borrow_mut() = v;
        // yield   MY TODO
        // self._neg = ! self._neg
        WithNegated::new(self._neg.clone())
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
                with_context_block!(var _a = self._negated()=>{
                let _= self.visit(&args[0]);});
            } else if (op == "&&" && !*self._neg.borrow()) || (op == "||" && *self._neg.borrow()) {
                let _ = self.visit(&args[0]);
                let _ = self.visit(&args[1]);
            } else if op == "parenthesis" {
                let _ = self.visit(&args[0]);
            } else if (op == "==" && !*self._neg.borrow()) || (op == "!=" && *self._neg.borrow()) {
                recursive_merge(&args[0], &args[1], &self._analysis);
            }
        }
        Ok(())
    }
}
pub fn _recursive_update(
    lhs: &ASTFlatten,
    rhs: &ASTFlatten,
    analysis: &RcCell<Option<PartitionState<AST>>>,
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
            _recursive_update(l, r, analysis, merge);
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
        match (lhs.map(|l| l.to_ast()), rhs.map(|r| r.to_ast())) {
            (Some(l), Some(r)) if analysis.borrow().as_ref().unwrap().has(&r) => {
                if merge {
                    analysis.borrow_mut().as_mut().unwrap().merge(&l, &r);
                } else {
                    analysis.borrow_mut().as_mut().unwrap().move_to(&l, &r);
                }
            }
            (Some(l), _) => {
                analysis.borrow_mut().as_mut().unwrap().move_to_separate(&l);
            }
            _ => {}
        }
    }
}
pub fn recursive_merge(
    lhs: &ASTFlatten,
    rhs: &ASTFlatten,
    analysis: &RcCell<Option<PartitionState<AST>>>,
) {
    _recursive_update(lhs, rhs, analysis, true);
}

pub fn recursive_assign(
    lhs: &ASTFlatten,
    rhs: &ASTFlatten,
    analysis: &RcCell<Option<PartitionState<AST>>>,
) {
    _recursive_update(lhs, rhs, analysis, false);
}
