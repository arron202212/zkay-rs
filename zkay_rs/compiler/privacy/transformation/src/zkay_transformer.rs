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
use fancy_regex::Regex as Regexf;
use rccell::{RcCell, WeakCell};
use regex::Regex;
use regex::RegexSetBuilder;
use solidity::fake_solidity_generator::{ID_PATTERN, WS_PATTERN};
use zkay_ast::analysis::contains_private_checker::contains_private_expr;
use zkay_ast::ast::{
    is_instance, ASTBaseProperty, ASTBaseRef, ASTChildren, ASTFlatten, ASTInstanceOf, ASTType,
    AnnotatedTypeName, AssignmentStatement, AssignmentStatementBaseMutRef,
    AssignmentStatementBaseProperty, BlankLine, Block, BooleanLiteralExpr, BooleanLiteralType,
    BreakStatement, BuiltinFunction, ChildListBuilder, Comment, CommentBase, ContinueStatement,
    DoWhileStatement, ElementaryTypeName, EncryptionExpression, EnumDefinition, ExprUnion,
    Expression, ExpressionASType, ExpressionBaseMutRef, ExpressionBaseProperty, ForStatement,
    FunctionCallExpr, FunctionCallExprBase, FunctionCallExprBaseMutRef,
    FunctionCallExprBaseProperty, HybridArgType, HybridArgumentIdf, Identifier, IdentifierBase,
    IdentifierBaseMutRef, IdentifierBaseProperty, IdentifierDeclaration,
    IdentifierDeclarationBaseProperty, IdentifierExpr, IdentifierExprUnion, IdentifierUnion,
    IfStatement, IndexExpr, IntoAST, IntoExpression, IntoStatement, LiteralExpr, LocationExpr,
    Mapping, MeExpr, MemberAccessExpr, NamespaceDefinition, NumberLiteralExpr, NumberLiteralType,
    NumberTypeName, Parameter, PrimitiveCastExpr, ReclassifyExpr, ReclassifyExprBaseMutRef,
    ReclassifyExprBaseProperty, ReclassifyExprBaseRef, ReturnStatement, SimpleStatement,
    StateVariableDeclaration, Statement, StatementBaseMutRef, StatementBaseProperty, StatementList,
    StatementListBaseMutRef, StatementListBaseProperty, TupleExpr, TypeName, VariableDeclaration,
    VariableDeclarationStatement, WhileStatement, AST,
};
use zkay_ast::global_defs::GlobalVars;
use zkay_ast::homomorphism::Homomorphism;
use zkay_ast::visitors::deep_copy::replace_expr;
use zkay_ast::visitors::transformer_visitor::{
    AstTransformerVisitor, AstTransformerVisitorBase, AstTransformerVisitorBaseRef,
    TransformerVisitorEx,
};
use zkay_config::config::CFG;
use zkay_derive::AstTransformerVisitorBaseRefImpl;

// class ZkayVarDeclTransformer(AstTransformerVisitor)
// """
// Transformer for types, which was left out in the paper.

// This removes all privacy labels and converts the types of non-public variables (not @all)
// to cipher_type.
// """

// pub fn __init__(self)
//     super().__init__()
//     self.expr_trafo = ZkayExpressionTransformer(None)
#[derive(Clone, AstTransformerVisitorBaseRefImpl)]
pub struct ZkayVarDeclTransformer {
    ast_transformer_visitor_base: AstTransformerVisitorBase,
    expr_trafo: Option<ZkayExpressionTransformer>,
    global_vars: RcCell<GlobalVars>,
}

impl AstTransformerVisitor for ZkayVarDeclTransformer {
    // fn default() -> Self {
    //     Self::new()
    // }
    // type Return = Option<ASTFlatten>;
    // fn temper_result(&self) -> Option<ASTFlatten> {None}

    fn has_attr(&self, name: &ASTType, _ast: &AST) -> bool {
        matches!(
            name,
            ASTType::AnnotatedTypeName
                | ASTType::VariableDeclaration
                | ASTType::Parameter
                | ASTType::StateVariableDeclaration
                | ASTType::Mapping
                | ASTType::ASTBase
        )
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        match name {
            ASTType::AnnotatedTypeName => self.visitAnnotatedTypeName(ast),
            ASTType::VariableDeclaration => self.visitVariableDeclaration(ast),
            ASTType::Parameter => self.visitParameter(ast),
            ASTType::StateVariableDeclaration => self.visitStateVariableDeclaration(ast),
            ASTType::Mapping => self.visitMapping(ast),
            ASTType::ASTBase => <Self as AstTransformerVisitor>::visitAST(self, ast),
            _ => Err(eyre::eyre!("unreach")),
        }
    }
}
impl ZkayVarDeclTransformer {
    pub fn new(global_vars: RcCell<GlobalVars>) -> Self {
        Self {
            ast_transformer_visitor_base: AstTransformerVisitorBase::new(true),
            expr_trafo: Some(ZkayExpressionTransformer::new(None, global_vars.clone())),
            global_vars,
        }
    }

    pub fn visitAnnotatedTypeName(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        let t = if ast
            .try_as_annotated_type_name_ref()
            .unwrap()
            .borrow()
            .is_private()
        {
            // println!(
            //     "=====ZkayVarDeclTransformer================visitAnnotatedTypeName====={}====",
            //     ast
            // );

            Some(
                RcCell::new(TypeName::cipher_type(
                    ast.try_as_annotated_type_name_ref().unwrap().clone(),
                    ast.try_as_annotated_type_name_ref()
                        .unwrap()
                        .borrow()
                        .homomorphism
                        .clone(),
                ))
                .into(),
            )
        } else {
            // println!("=====ZkayVarDeclTransformer================visitAnnotatedTypeName====else====typename=={:?}======",ast.try_as_annotated_type_name_ref()
            //         .unwrap()
            //         .borrow()
            //         .type_name
            //         .as_ref()
            //         .unwrap().get_ast_type());
            if ast
                .try_as_annotated_type_name_ref()
                .unwrap()
                .borrow()
                .type_name
                .as_ref()
                .unwrap()
                .get_ast_type()
                == ASTType::Mapping
            {
                self.visit(
                    ast.try_as_annotated_type_name_ref()
                        .unwrap()
                        .borrow()
                        .type_name
                        .as_ref()
                        .unwrap()
                        .to_ast()
                        .try_as_type_name()
                        .unwrap()
                        .try_as_mapping_ref()
                        .unwrap()
                        .clone_owned(self.global_vars.clone())
                        .as_ref()
                        .unwrap(),
                )
            } else {
                self.visit(
                    ast.try_as_annotated_type_name_ref()
                        .unwrap()
                        .borrow()
                        .type_name
                        .as_ref()
                        .unwrap(),
                )
            }
        };
        if t.is_some() {
            *ast.try_as_annotated_type_name_ref().unwrap().borrow_mut() =
                AnnotatedTypeName::new(t.clone(), None, Homomorphism::non_homomorphic());
        } else {
            println!(
                "=======none==={:?}==",
                ast.try_as_annotated_type_name_ref()
                    .unwrap()
                    .borrow()
                    .type_name
                    .as_ref()
                    .unwrap()
                    .get_ast_type()
            );
        }

        // println!("=======tT==========={},{}====",t.is_some(),t.clone().and_then(|t| t.try_as_type_name()).is_some());
        Ok(RcCell::new(AnnotatedTypeName::new(
            t,
            None,
            Homomorphism::non_homomorphic(),
        ))
        .into())
    }

    pub fn visitVariableDeclaration(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        if ast
            .ast_base_ref()
            .unwrap()
            .borrow()
            .annotated_type
            .as_ref()
            .unwrap()
            .borrow()
            .is_private()
        {
            ast.try_as_variable_declaration_ref()
                .unwrap()
                .borrow_mut()
                .identifier_declaration_base
                .storage_location = Some(String::from("memory"));
        }
        self.visit_children(ast)
    }

    pub fn visitParameter(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        let ast = self.visit_children(ast);
        if ast.is_err() || !is_instance(ast.as_ref().unwrap(), ASTType::Parameter) {
            eyre::bail!("ast is none or ast is not parameter")
        }
        if !ast
            .as_ref()
            .unwrap()
            .ast_base_ref()
            .unwrap()
            .borrow()
            .annotated_type
            .as_ref()
            .unwrap()
            .borrow()
            .type_name
            .as_ref()
            .unwrap()
            .to_ast()
            .try_as_type_name()
            .unwrap()
            .is_primitive_type()
        {
            ast.as_ref()
                .unwrap()
                .try_as_parameter_ref()
                .unwrap()
                .borrow_mut()
                .identifier_declaration_base
                .storage_location = Some(String::from("memory"));
        }
        ast
    }

    pub fn visitStateVariableDeclaration(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        let keywords: Vec<_> = ast
            .try_as_state_variable_declaration_ref()
            .unwrap()
            .borrow()
            .identifier_declaration_base
            .keywords
            .iter()
            .filter(|&k| k != "public")
            .cloned()
            .collect();
        ast.try_as_state_variable_declaration_ref()
            .unwrap()
            .borrow_mut()
            .identifier_declaration_base
            .keywords = keywords;
        //make sure every state var gets a public getter (required for simulation)
        ast.try_as_state_variable_declaration_ref()
            .unwrap()
            .borrow_mut()
            .identifier_declaration_base
            .keywords
            .push(String::from("public"));
        if let Some(expr) = ast
            .try_as_state_variable_declaration_ref()
            .unwrap()
            .borrow_mut()
            .expr
            .as_mut()
        {
            let mut exp = self.expr_trafo.as_ref().unwrap().visit(expr).unwrap();
            *expr = exp;
        }
        self.visit_children(ast)
    }

    pub fn visitMapping(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        // println!("==visitMapping==========={ast:?}");
        if ast.is_mapping() {
            if let Some(key_label) = ast
                .try_as_mapping_ref()
                .unwrap()
                .borrow_mut()
                .key_label
                .as_mut()
            {
                let kl = key_label.borrow().name().clone();
                let mut idfbase = IdentifierBase::new(kl);
                idfbase.is_string = true;
                *key_label = RcCell::new(Identifier::Identifier(idfbase));
            }
        } else if ast.is_type_name() {
            if let Some(key_label) = ast
                .try_as_type_name_ref()
                .unwrap()
                .borrow_mut()
                .try_as_mapping_mut()
                .unwrap()
                .key_label
                .as_mut()
            {
                let kl = key_label.borrow().name().clone();
                let mut idfbase = IdentifierBase::new(kl);
                idfbase.is_string = true;
                *key_label = RcCell::new(Identifier::Identifier(idfbase));
            }
        } else {
            panic!("====else====={ast:?}===");
        }
        self.visit_children(ast)
    }
}
// class ZkayStatementTransformer(AstTransformerVisitor)
// """Corresponds to T from paper, (with additional handling of return statement and loops)."""
#[derive(Clone, AstTransformerVisitorBaseRefImpl)]
pub struct ZkayStatementTransformer {
    ast_transformer_visitor_base: AstTransformerVisitorBase,
    generator: Option<RcCell<CircuitHelper>>,
    expr_trafo: ZkayExpressionTransformer,
    var_decl_trafo: ZkayVarDeclTransformer,
}
impl AstTransformerVisitor for ZkayStatementTransformer {
    // fn default() -> Self {
    //     Self::new(None)
    // }

    // type Return = Option<ASTFlatten>;
    // fn temper_result(&self) -> Option<ASTFlatten> {
    //     None
    // }

    fn has_attr(&self, name: &ASTType, ast: &AST) -> bool {
        matches!(
            name,
            ASTType::StatementListBase
                | ASTType::StatementBase
                | ASTType::IfStatement
                | ASTType::WhileStatement
                | ASTType::DoWhileStatement
                | ASTType::ForStatement
                | ASTType::ContinueStatement
                | ASTType::BreakStatement
                | ASTType::ReturnStatement
                | ASTType::ASTBase
        ) || matches!(ast, AST::Expression(_))
            || matches!(ast, AST::Statement(Statement::StatementList(_)))
            || matches!(
                ast,
                AST::Statement(Statement::SimpleStatement(
                    SimpleStatement::AssignmentStatement(_)
                ))
            )
            || matches!(ast, AST::Statement(_))
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        match name {
            ASTType::IfStatement => self.visitIfStatement(ast),
            ASTType::WhileStatement => self.visitWhileStatement(ast),
            ASTType::DoWhileStatement => self.visitDoWhileStatement(ast),
            ASTType::ForStatement => self.visitForStatement(ast),
            ASTType::ContinueStatement => self.visitContinueStatement(ast),
            ASTType::BreakStatement => self.visitBreakStatement(ast),
            ASTType::ReturnStatement => self.visitReturnStatement(ast),
            _ if matches!(ast.to_ast(), AST::Expression(_)) => self.visitExpression(ast),
            _ if matches!(ast.to_ast(), AST::Statement(Statement::StatementList(_))) => {
                self.visitStatementList(ast)
            }
            _ if matches!(
                ast.to_ast(),
                AST::Statement(Statement::SimpleStatement(
                    SimpleStatement::AssignmentStatement(_)
                ))
            ) =>
            {
                self.visitAssignmentStatement(ast)
            }
            _ if matches!(ast.to_ast(), AST::Statement(_)) => self.visitStatement(ast),
            ASTType::ASTBase => <Self as AstTransformerVisitor>::visitAST(self, ast),
            _ => Err(eyre::eyre!("unreach")),
        }
    }
}
impl ZkayStatementTransformer {
    // pub fn __init__(&self, current_gen: CircuitHelper)
    //     super().__init__()
    //     self.generator.unwrap() = current_gen
    //     self.expr_trafo = ZkayExpressionTransformer(self.generator.unwrap())
    //     self.var_decl_trafo = ZkayVarDeclTransformer()
    pub fn new(
        current_gen: Option<WeakCell<CircuitHelper>>,
        global_vars: RcCell<GlobalVars>,
    ) -> Self {
        Self {
            ast_transformer_visitor_base: AstTransformerVisitorBase::new(false),
            generator: current_gen.clone().and_then(|g| g.upgrade()),
            expr_trafo: ZkayExpressionTransformer::new(current_gen, global_vars.clone()),
            var_decl_trafo: ZkayVarDeclTransformer::new(global_vars),
        }
    }
    // """
    // Rule (1)

    // All statements are transformed individually.
    // Whenever the transformation of a statement requires the introduction of additional statements
    // (the CircuitHelper indicates this by storing them in the statement"s pre_statements list), they are prepended to the transformed
    // statement in the list.

    // If transformation changes the appearance of a statement (apart from type changes),
    // the statement is wrapped in a comment block which displays the original statement"s code.
    // """
    pub fn visitStatementList(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        let mut new_statements = vec![];
        for (_idx, stmt) in ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_statement_list_ref()
            .unwrap()
            .statements()
            .iter()
            .enumerate()
        {
            let old_code = ASTFlatten::from(stmt.clone()).code();
            let transformed_stmt = self.visit(&stmt.clone().into());
            if transformed_stmt.is_none() {
                continue;
            }
            let r = Regex::new(&format!(r"@{WS_PATTERN}*{ID_PATTERN}"))
                .unwrap()
                .replace_all(&old_code, "");
            let old_code_wo_annotations = Regexf::new(r"(?=\b)me(?=\b)")
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
                        .to_ast()
                        .try_as_statement_ref()
                        .unwrap()
                        .statement_base_ref()
                        .unwrap()
                        .pre_statements
                        .iter()
                        .map(|ps| ps.clone_inner())
                        .chain([transformed_stmt.as_ref().unwrap().clone_inner()])
                        .collect(),
                ));
            }
        }
        if !new_statements.is_empty()
            && is_instance(new_statements.last().unwrap(), ASTType::BlankLine)
        {
            new_statements.pop();
        }
        // if ast.get_ast_type() == ASTType::StatementListBase {
        //     if new_statements
        //         .iter()
        //         .any(|s| s.get_ast_type() == ASTType::StatementListBase)
        //     {
        //         println!(
        //             "==StatementListBase=========tt========StatementListBase===={}=",
        //             line!()
        //         );
        //     }
        // }
        if ast.is_statement_list() {
            ast.try_as_statement_list_ref()
                .unwrap()
                .borrow_mut()
                .statement_list_base_mut_ref()
                .statements = new_statements;
        } else if ast.is_block() {
            ast.try_as_block_ref()
                .unwrap()
                .borrow_mut()
                .statement_list_base_mut_ref()
                .statements = new_statements;
        } else {
            panic!("==============else======={ast:?}");
        }
        Ok(ast.clone())
    }
    // """Default statement child handling. Expressions and declarations are visited by the corresponding transformers."""
    pub fn process_statement_child(&self, child: &ASTFlatten) -> Option<ASTFlatten> {
        if is_instance(child, ASTType::ExpressionBase) {
            self.expr_trafo.visit(child)
        } else {
            assert!(is_instance(child, ASTType::VariableDeclaration));
            self.var_decl_trafo.visit(child)
        }
    }
    // """
    // Rules (3), (4)

    // This is for all the statements where the statements themselves remain untouched and only the children are altered.
    // """
    pub fn visitStatement(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        assert!(
            is_instance(ast, ASTType::SimpleStatementBase)
                || is_instance(ast, ASTType::VariableDeclarationStatement)
        );
        let mut cb = ChildListBuilder::new();
        ast.to_ast()
            .try_as_statement_ref()
            .unwrap()
            .process_children(&mut cb);
        cb.children.iter().for_each(|c| {
            self.process_statement_child(c);
        });
        Ok(ast.clone())
    }
    // """Rule (2)"""
    pub fn visitAssignmentStatement(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        //  println!("==visitAssignmentStatement======================={ast:?}");
        let a = self.expr_trafo.visit(
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
        );
        if ast.is_assignment_statement() {
            ast.try_as_assignment_statement_ref()
                .unwrap()
                .borrow_mut()
                .assignment_statement_base_mut_ref()
                .lhs = a;
        } else if ast.is_ast() {
            ast.try_as_ast_ref()
                .unwrap()
                .borrow_mut()
                .try_as_statement_mut()
                .unwrap()
                .try_as_simple_statement_mut()
                .unwrap()
                .try_as_assignment_statement_mut()
                .unwrap()
                .assignment_statement_base_mut_ref()
                .lhs = a;
        } else {
            panic!("=============else============{ast:?}");
        }
        let rhs = self.expr_trafo.visit(
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
        );
        if ast.is_assignment_statement() {
            ast.try_as_assignment_statement_ref()
                .unwrap()
                .borrow_mut()
                .assignment_statement_base_mut_ref()
                .rhs = rhs;
        } else if ast.is_ast() {
            ast.try_as_ast_ref()
                .unwrap()
                .borrow_mut()
                .try_as_statement_mut()
                .unwrap()
                .try_as_simple_statement_mut()
                .unwrap()
                .try_as_assignment_statement_mut()
                .unwrap()
                .assignment_statement_base_mut_ref()
                .rhs = rhs;
        } else {
            panic!("=============else============{ast:?}");
        }

        let mut modvals = ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_simple_statement_ref()
            .unwrap()
            .try_as_assignment_statement_ref()
            .unwrap()
            .modified_values()
            .clone();
        if CFG.lock().unwrap().user_config.opt_cache_circuit_outputs()
            && is_instance(
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
                ASTType::IdentifierExpr,
            )
            && is_instance(
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
                ASTType::MemberAccessExpr,
            )
        {
            //Skip invalidation if rhs is circuit output
            if is_instance(
                &ast.to_ast()
                    .try_as_statement_ref()
                    .unwrap()
                    .try_as_simple_statement_ref()
                    .unwrap()
                    .try_as_assignment_statement_ref()
                    .unwrap()
                    .rhs()
                    .as_ref()
                    .unwrap()
                    .to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .try_as_member_access_expr_ref()
                    .unwrap()
                    .member,
                ASTType::HybridArgumentIdf,
            ) && ast
                .to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_simple_statement_ref()
                .unwrap()
                .try_as_assignment_statement_ref()
                .unwrap()
                .rhs()
                .as_ref()
                .unwrap()
                .to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .try_as_member_access_expr_ref()
                .unwrap()
                .member
                .borrow()
                .try_as_hybrid_argument_idf_ref()
                .unwrap()
                .arg_type
                == HybridArgType::PubCircuitArg
            {
                modvals = modvals
                    .iter()
                    .filter(|mv| {
                        mv.target()
                            != ast
                                .to_ast()
                                .try_as_statement_ref()
                                .unwrap()
                                .try_as_simple_statement_ref()
                                .unwrap()
                                .try_as_assignment_statement_ref()
                                .unwrap()
                                .lhs()
                                .as_ref()
                                .unwrap()
                                .ast_base_ref()
                                .unwrap()
                                .borrow()
                                .target
                                .as_ref()
                                .and_then(|t| t.clone().upgrade())
                    })
                    .cloned()
                    .collect();
                let ridf = if is_instance(
                    ast.to_ast()
                        .try_as_statement_ref()
                        .unwrap()
                        .try_as_simple_statement_ref()
                        .unwrap()
                        .try_as_assignment_statement_ref()
                        .unwrap()
                        .rhs()
                        .as_ref()
                        .unwrap()
                        .to_ast()
                        .try_as_expression_ref()
                        .unwrap()
                        .try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .try_as_location_expr_ref()
                        .unwrap()
                        .try_as_member_access_expr_ref()
                        .unwrap()
                        .member
                        .borrow()
                        .try_as_hybrid_argument_idf_ref()
                        .unwrap()
                        .corresponding_priv_expression
                        .as_ref()
                        .unwrap(),
                    ASTType::EncryptionExpression,
                ) {
                    ast.to_ast()
                        .try_as_statement_ref()
                        .unwrap()
                        .try_as_simple_statement_ref()
                        .unwrap()
                        .try_as_assignment_statement_ref()
                        .unwrap()
                        .rhs()
                        .as_ref()
                        .unwrap()
                        .try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .borrow()
                        .try_as_location_expr_ref()
                        .unwrap()
                        .try_as_member_access_expr_ref()
                        .unwrap()
                        .member
                        .borrow()
                        .try_as_hybrid_argument_idf_ref()
                        .unwrap()
                        .corresponding_priv_expression
                        .as_ref()
                        .unwrap()
                        .ast_base_ref()
                        .unwrap()
                        .borrow()
                        .idf
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .clone()
                } else {
                    ast.to_ast()
                        .try_as_statement_ref()
                        .unwrap()
                        .try_as_simple_statement_ref()
                        .unwrap()
                        .try_as_assignment_statement_ref()
                        .unwrap()
                        .rhs()
                        .as_ref()
                        .unwrap()
                        .to_ast()
                        .try_as_expression_ref()
                        .unwrap()
                        .try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .try_as_location_expr_ref()
                        .unwrap()
                        .try_as_member_access_expr_ref()
                        .unwrap()
                        .member
                        .borrow()
                        .try_as_hybrid_argument_idf_ref()
                        .unwrap()
                        .corresponding_priv_expression
                        .as_ref()
                        .unwrap()
                        .ast_base_ref()
                        .unwrap()
                        .borrow()
                        .idf
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .clone()
                };
                assert!(is_instance(&ridf, ASTType::HybridArgumentIdf));
                if let Identifier::HybridArgumentIdf(ridf) = ridf {
                    self.generator
                        .as_ref()
                        .unwrap()
                        .borrow_mut()
                        ._remapper
                        .0
                        .remap(
                            &ast.to_ast()
                                .try_as_statement_ref()
                                .unwrap()
                                .try_as_simple_statement_ref()
                                .unwrap()
                                .try_as_assignment_statement_ref()
                                .unwrap()
                                .lhs()
                                .as_ref()
                                .unwrap()
                                .ast_base_ref()
                                .unwrap()
                                .borrow()
                                .target
                                .clone()
                                .unwrap()
                                .upgrade()
                                .unwrap()
                                .ast_base_ref()
                                .unwrap()
                                .borrow()
                                .idf()
                                .unwrap(),
                            ridf,
                        );
                }
            }
        }
        //Invalidate circuit value for assignment targets
        if self.generator.is_some() {
            for val in modvals {
                if val.key().is_none() {
                    self.generator
                        .as_ref()
                        .unwrap()
                        .borrow_mut()
                        .invalidate_idf(
                            &val.target()
                                .as_ref()
                                .unwrap()
                                .ast_base_ref()
                                .unwrap()
                                .borrow()
                                .idf()
                                .unwrap(),
                        );
                }
            }
        }
        Ok(ast.clone())
    }
    // """
    // Rule (6) + additional support for private conditions

    // If the condition is public, guard conditions are introduced for both branches if any of the branches contains private expressions.

    // If the condition is private, the whole if statement is inlined into the circuit. The only side-effects which are allowed
    // inside the branch bodies are assignment statements with an lhs@me. (anything else would leak private information).
    // The if statement will be replaced by an assignment statement where the lhs is a tuple of all locations which are written
    // in either branch and rhs is a tuple of the corresponding circuit outputs.
    // """
    pub fn visitIfStatement(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        if ast
            .try_as_if_statement_ref()
            .unwrap()
            .borrow()
            .condition
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .annotated_type()
            .as_ref()
            .unwrap()
            .borrow()
            .is_public()
        {
            if contains_private_expr(
                &ast.try_as_if_statement_ref()
                    .unwrap()
                    .borrow()
                    .then_branch
                    .clone()
                    .into(),
            ) || contains_private_expr(
                &ast.try_as_if_statement_ref()
                    .unwrap()
                    .borrow()
                    .else_branch
                    .as_ref()
                    .unwrap()
                    .clone()
                    .into(),
            ) {
                let before_if_state = self
                    .generator
                    .as_ref()
                    .unwrap()
                    .borrow()
                    ._remapper
                    .0
                    .get_state();
                println!(
                    "====guard_var==========condition========={:?}",
                    ast.try_as_if_statement_ref()
                        .unwrap()
                        .borrow()
                        .condition
                        .to_string()
                );
                let guard_var = self
                    .generator
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .add_to_circuit_inputs(
                        &ast.try_as_if_statement_ref().unwrap().borrow().condition,
                    );
                ast.try_as_if_statement_ref()
                    .unwrap()
                    .borrow_mut()
                    .condition = guard_var.get_loc_expr(Some(ast));
                self.generator
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .guarded(guard_var.clone(), true);
                {
                    ast.try_as_if_statement_ref()
                        .unwrap()
                        .borrow_mut()
                        .then_branch = self
                        .visit(
                            &ast.try_as_if_statement_ref()
                                .unwrap()
                                .borrow()
                                .then_branch
                                .clone()
                                .into(),
                        )
                        .unwrap()
                        .try_as_block()
                        .unwrap();
                    self.generator
                        .as_ref()
                        .unwrap()
                        .borrow_mut()
                        ._remapper
                        .0
                        .set_state(&before_if_state);
                }
                if ast
                    .try_as_if_statement_ref()
                    .unwrap()
                    .borrow()
                    .else_branch
                    .is_some()
                {
                    self.generator
                        .as_ref()
                        .unwrap()
                        .borrow_mut()
                        .guarded(guard_var, false);

                    ast.try_as_if_statement_ref()
                        .unwrap()
                        .borrow_mut()
                        .else_branch = self
                        .visit(
                            &ast.try_as_if_statement_ref()
                                .unwrap()
                                .borrow()
                                .else_branch
                                .as_ref()
                                .unwrap()
                                .clone()
                                .into(),
                        )
                        .unwrap()
                        .try_as_block();
                    self.generator
                        .as_ref()
                        .unwrap()
                        .borrow_mut()
                        ._remapper
                        .0
                        .set_state(&before_if_state);
                }

                //Invalidate values modified in either branch
                for val in &ast
                    .try_as_if_statement_ref()
                    .unwrap()
                    .borrow()
                    .statement_base
                    .ast_base
                    .borrow()
                    .modified_values
                {
                    if val.key().is_none() {
                        self.generator
                            .as_ref()
                            .unwrap()
                            .borrow_mut()
                            .invalidate_idf(
                                &val.target()
                                    .as_ref()
                                    .unwrap()
                                    .try_as_identifier_declaration_ref()
                                    .unwrap()
                                    .borrow()
                                    .idf()
                                    .unwrap(),
                            );
                    }
                }
            } else {
                ast.try_as_if_statement_ref()
                    .unwrap()
                    .borrow_mut()
                    .condition = self
                    .expr_trafo
                    .visit(&ast.try_as_if_statement_ref().unwrap().borrow().condition)
                    .unwrap();
                ast.try_as_if_statement_ref()
                    .unwrap()
                    .borrow_mut()
                    .then_branch = self
                    .visit(
                        &ast.try_as_if_statement_ref()
                            .unwrap()
                            .borrow()
                            .then_branch
                            .clone()
                            .into(),
                    )
                    .unwrap()
                    .try_as_block()
                    .unwrap();
                if ast
                    .try_as_if_statement_ref()
                    .unwrap()
                    .borrow()
                    .else_branch
                    .is_some()
                {
                    ast.try_as_if_statement_ref()
                        .unwrap()
                        .borrow_mut()
                        .else_branch = self
                        .visit(
                            &ast.try_as_if_statement_ref()
                                .unwrap()
                                .borrow()
                                .else_branch
                                .as_ref()
                                .unwrap()
                                .clone()
                                .into(),
                        )
                        .unwrap()
                        .try_as_block();
                }
            }
            Ok(ast.clone())
        } else {
            self.generator
                .as_ref()
                .unwrap()
                .borrow_mut()
                .evaluate_stmt_in_circuit(ast)
                .ok_or(eyre::eyre!("unexpected"))
        }
    }
    //Loops must always be purely public
    pub fn visitWhileStatement(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        assert!(!contains_private_expr(
            &ast.try_as_while_statement_ref()
                .unwrap()
                .borrow()
                .condition
                .clone()
                .into()
        ));
        assert!(!contains_private_expr(
            &ast.try_as_while_statement_ref()
                .unwrap()
                .borrow()
                .body
                .clone()
                .into()
        ));
        Ok(ast.clone())
    }
    //Loops must always be purely public
    pub fn visitDoWhileStatement(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        assert!(!contains_private_expr(
            &ast.try_as_do_while_statement_ref()
                .unwrap()
                .borrow()
                .condition
                .clone()
                .into()
        ));
        assert!(!contains_private_expr(
            &ast.try_as_do_while_statement_ref()
                .unwrap()
                .borrow()
                .body
                .clone()
                .into()
        ));
        Ok(ast.clone())
    }

    pub fn visitForStatement(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        if ast
            .try_as_for_statement_ref()
            .unwrap()
            .borrow()
            .init
            .is_some()
        {
            //Init is the only part of a for loop which may contain private expressions
            ast.try_as_for_statement_ref().unwrap().borrow_mut().init = self
                .visit(
                    &ast.try_as_for_statement_ref()
                        .unwrap()
                        .borrow()
                        .init
                        .clone()
                        .unwrap()
                        .into(),
                )
                .unwrap()
                .try_as_statement_ref()
                .unwrap()
                .borrow()
                .try_as_for_statement_ref()
                .unwrap()
                .init
                .as_ref()
                .map(|i| i.clone());
            ast.try_as_for_statement_ref()
                .unwrap()
                .borrow_mut()
                .statement_base
                .pre_statements
                .extend(
                    ast.try_as_for_statement_ref()
                        .unwrap()
                        .borrow()
                        .init
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .pre_statements()
                        .iter()
                        .map(|ps| ps.clone_inner())
                        .collect::<Vec<_>>(),
                );
        }
        assert!(!contains_private_expr(
            &ast.try_as_for_statement_ref()
                .unwrap()
                .borrow()
                .condition
                .clone()
                .into()
        ));
        assert!(
            !ast.try_as_for_statement_ref()
                .unwrap()
                .borrow()
                .update
                .is_some()
                || !contains_private_expr(
                    &ast.try_as_for_statement_ref()
                        .unwrap()
                        .borrow()
                        .update
                        .clone()
                        .unwrap()
                        .into()
                )
        );
        assert!(!contains_private_expr(
            &ast.try_as_for_statement_ref()
                .unwrap()
                .borrow()
                .body
                .clone()
                .into()
        )); //OR fixed size loop -> static analysis can prove that loop terminates in fixed //iterations
        Ok(ast.clone())
    }

    pub fn visitContinueStatement(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        Ok(ast.clone())
    }

    pub fn visitBreakStatement(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        Ok(ast.clone())
    }
    // """
    // Handle return statement.

    // If the function requires verification, the return statement is replaced by an assignment to a return variable.
    // (which will be returned at the very end of the function body, after any verification wrapper code).
    // Otherwise only the expression is transformed.
    // """
    pub fn visitReturnStatement(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        if ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .statement_base_ref()
            .unwrap()
            .function
            .clone()
            .unwrap()
            .upgrade()
            .unwrap()
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .borrow()
            .requires_verification
        {
            if ast
                .to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_return_statement_ref()
                .unwrap()
                .expr
                .is_none()
            {
                eyre::bail!("expr is_none ")
            }
            assert!(!self.generator.as_ref().unwrap().borrow().has_return_var);
            self.generator.as_ref().unwrap().borrow_mut().has_return_var = true;
            let expr = self.expr_trafo.visit(
                ast.to_ast()
                    .try_as_statement_ref()
                    .unwrap()
                    .try_as_return_statement_ref()
                    .unwrap()
                    .expr
                    .as_ref()
                    .unwrap(),
            );
            let ret_args = ast
                .to_ast()
                .try_as_statement_ref()
                .unwrap()
                .statement_base_ref()
                .unwrap()
                .function
                .clone()
                .unwrap()
                .upgrade()
                .unwrap()
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .borrow()
                .return_var_decls
                .iter()
                .map(|vd| {
                    let mut idf = IdentifierExpr::new(
                        IdentifierExprUnion::Identifier(vd.borrow().idf().clone().unwrap()),
                        None,
                    );
                    idf.ast_base_ref().borrow_mut().target =
                        Some(ASTFlatten::from(vd.clone()).downgrade());
                    RcCell::new(idf).into()
                })
                .collect();
            let mut te = TupleExpr::new(ret_args).assign(expr.unwrap());
            te.statement_base_mut_ref().pre_statements = ast
                .to_ast()
                .try_as_statement_ref()
                .unwrap()
                .statement_base_ref()
                .unwrap()
                .pre_statements
                .iter()
                .map(|ps| ps.clone_inner())
                .collect();
            Ok(RcCell::new(te).into())
        } else {
            let expr = self.expr_trafo.visit(
                ast.try_as_return_statement_ref()
                    .unwrap()
                    .borrow()
                    .expr
                    .as_ref()
                    .unwrap(),
            );
            if ast.is_return_statement() {
                ast.try_as_return_statement_ref().unwrap().borrow_mut().expr = expr;
            } else {
                panic!("==============else========{ast:?}");
            }
            Ok(ast.clone().into())
        }
    }
    // """Fail if there are any untransformed expressions left."""
    pub fn visitExpression(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        eyre::bail!("Missed an expression of type {:?}", ast);
    }
}
// class ZkayExpressionTransformer(AstTransformerVisitor)
// """
// Roughly corresponds to T_L / T_e from paper.

// T_L and T_e are equivalent here, because parameter encryption checks are handled in the verification wrapper of the function body.
// In addition to the features described in the paper, this transformer also supports primitive type casting,
// tuples (multiple return values), operations with short-circuiting and function calls.
// """
#[derive(Clone, AstTransformerVisitorBaseRefImpl)]
pub struct ZkayExpressionTransformer {
    ast_transformer_visitor_base: AstTransformerVisitorBase,
    generator: Option<RcCell<CircuitHelper>>,
    global_vars: RcCell<GlobalVars>,
}
impl TransformerVisitorEx for ZkayExpressionTransformer {
    fn visitBlock(
        &self,
        _ast: &ASTFlatten,
        _guard_cond: Option<HybridArgumentIdf>,
        _guard_val: Option<bool>,
    ) -> eyre::Result<ASTFlatten> {
        eyre::bail!("")
    }
}
impl AstTransformerVisitor for ZkayExpressionTransformer {
    // fn default() -> Self {
    //     Self::new(None)
    // }

    // type Return = Option<ASTFlatten>;
    // fn temper_result(&self) -> Option<ASTFlatten> {
    //     None
    // }

    fn has_attr(&self, name: &ASTType, ast: &AST) -> bool {
        matches!(
            name,
            ASTType::MeExpr
                | ASTType::IdentifierExpr
                | ASTType::IndexExpr
                | ASTType::MemberAccessExpr
                | ASTType::TupleExpr
                | ASTType::BuiltinFunction
                | ASTType::PrimitiveCastExpr
                | ASTType::ASTBase
        ) || matches!(ast, AST::Expression(Expression::ReclassifyExpr(_)))
            || matches!(ast, AST::Expression(Expression::LiteralExpr(_)))
            || matches!(ast, AST::Expression(Expression::FunctionCallExpr(_)))
            || matches!(ast, AST::Expression(_))
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        match name {
            ASTType::MeExpr => self.visitMeExpr(ast),
            ASTType::IdentifierExpr => Self::visitIdentifierExpr(ast),
            ASTType::IndexExpr => self.visitIndexExpr(ast),
            ASTType::MemberAccessExpr => self.visitMemberAccessExpr(ast),
            ASTType::TupleExpr => self.visitTupleExpr(ast),
            ASTType::BuiltinFunction => Self::visitBuiltinFunction(ast),
            ASTType::PrimitiveCastExpr => self.visitPrimitiveCastExpr(ast),
            _ if matches!(ast.to_ast(), AST::Expression(Expression::ReclassifyExpr(_))) => {
                self.visitReclassifyExpr(ast)
            }
            _ if matches!(ast.to_ast(), AST::Expression(Expression::LiteralExpr(_))) => {
                Self::visitLiteralExpr(ast)
            }
            _ if matches!(
                ast.to_ast(),
                AST::Expression(Expression::FunctionCallExpr(_))
            ) =>
            {
                self.visitFunctionCallExpr(ast)
            }
            _ if matches!(ast.to_ast(), AST::Expression(_)) => self.visitExpression(ast),
            ASTType::ASTBase => <Self as AstTransformerVisitor>::visitAST(self, ast),
            _ => Err(eyre::eyre!("unreach")),
        }
    }
}
impl ZkayExpressionTransformer {
    pub fn new(
        current_generator: Option<WeakCell<CircuitHelper>>,
        global_vars: RcCell<GlobalVars>,
    ) -> Self {
        // super().__init__()
        // self.generator.unwrap() = current_generator
        Self {
            ast_transformer_visitor_base: AstTransformerVisitorBase::new(false),
            generator: current_generator.and_then(|g| g.upgrade()),
            global_vars,
        }
    }

    // @staticmethod
    // """Replace me with msg.sender."""
    pub fn visitMeExpr(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        replace_expr(
            ast,
            &RcCell::new(LocationExpr::IdentifierExpr(IdentifierExpr::new(
                IdentifierExprUnion::String(String::from("msg")),
                None,
            )))
            .into(),
            false,
            self.global_vars.clone(),
        )
        .map(|_expr| {
            _expr
                .to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .dot(IdentifierExprUnion::String(String::from("sender")))
                .as_type(&AnnotatedTypeName::address_all().into())
        })
        .ok_or(eyre::eyre!("unexpected"))
    }
    // """Rule (7), don"t modify constants."""
    pub fn visitLiteralExpr(ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        Ok(ast.clone())
    }
    // """Rule (8), don"t modify identifiers."""
    pub fn visitIdentifierExpr(ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        Ok(ast.clone())
    }
    // """Rule (9), transform location and index expressions separately."""
    pub fn visitIndexExpr(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        replace_expr(
            ast,
            &self
                .visit(
                    &ast.to_ast()
                        .try_as_expression_ref()
                        .unwrap()
                        .try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .try_as_location_expr_ref()
                        .unwrap()
                        .try_as_index_expr_ref()
                        .unwrap()
                        .arr
                        .clone()
                        .unwrap()
                        .into(),
                )
                .unwrap()
                .to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .index(ExprUnion::Expression(
                    self.visit(
                        &ast.to_ast()
                            .try_as_expression_ref()
                            .unwrap()
                            .try_as_tuple_or_location_expr_ref()
                            .unwrap()
                            .try_as_location_expr_ref()
                            .unwrap()
                            .try_as_index_expr_ref()
                            .unwrap()
                            .key
                            .clone()
                            .into(),
                    )
                    .unwrap(),
                )),
            false,
            self.global_vars.clone(),
        )
        .ok_or(eyre::eyre!("unexpected"))
    }

    pub fn visitMemberAccessExpr(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        self.visit_children(ast)
    }

    pub fn visitTupleExpr(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        self.visit_children(ast)
    }
    // """
    // Rule (11), trigger a boundary crossing.
    // The reclassified expression is evaluated in the circuit and its result is made available in solidity.
    // """
    pub fn visitReclassifyExpr(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        // println!(
        //     "===ZkayExpressionTransformer====visitReclassifyExpr======={}=====",
        //     ast
        // );
        let mut expr: ASTFlatten = ast
            .to_ast()
            .try_as_expression_ref()
            .unwrap()
            .try_as_reclassify_expr_ref()
            .unwrap()
            .expr()
            .clone()
            .into();
        //   println!(
        //             "====expression==transform=======visitReclassifyExpr==expr====={:?}=====",
        //             expr.get_ast_type()
        //         );
        self.generator
            .as_ref()
            .unwrap()
            .borrow()
            .evaluate_expr_in_circuit(
                &expr,
                &ast.to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_reclassify_expr_ref()
                    .unwrap()
                    .privacy()
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .privacy_annotation_label()
                    .unwrap()
                    .into(),
                &ast.ast_base_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .homomorphism,
            )
            .ok_or(eyre::eyre!("unexpected"))
    }

    pub fn visitBuiltinFunction(ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        Ok(ast.clone())
    }

    pub fn visitFunctionCallExpr(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        if is_instance(
            ast.to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_function_call_expr_ref()
                .unwrap()
                .func(),
            ASTType::BuiltinFunction,
        ) {
            // """
            // Modified Rule (12) builtin functions with private operands and homomorphic operations on ciphertexts
            // are evaluated inside the circuit.

            // A private expression on its own (like an IdentifierExpr referring to a private variable) is not enough to trigger a
            // boundary crossing (assignment of private variables is a public operation).
            // """
            if ast
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
                .is_private
            {
                let privacy_label = ast
                    .ast_base_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .privacy_annotation
                    .as_ref()
                    .unwrap()
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .privacy_annotation_label();
                self.generator
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .evaluate_expr_in_circuit(
                        ast,
                        &(privacy_label.unwrap().into()),
                        &(ast
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
                            .homomorphism
                            .clone()
                            .into()),
                    )
                    .ok_or(eyre::eyre!("unexpected"))
            } else {
                // """
                // Rule (10) with additional short-circuit handling.

                // Builtin operations on public operands are normally left untransformed, but if the builtin function has
                // short-circuiting semantics, guard conditions must be added if any of the public operands contains
                // nested private expressions.
                // """
                //handle short-circuiting
                let mut args = ast
                    .to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .args()
                    .clone();
                if ast
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
                    .has_shortcircuiting()
                    && args[1..].iter().any(|arg| contains_private_expr(arg))
                {
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
                    // println!("======guard_var============args[0]===========");
                    let guard_var = self
                        .generator
                        .as_ref()
                        .unwrap()
                        .borrow_mut()
                        .add_to_circuit_inputs(&mut args[0]);
                    args[0] = guard_var.get_loc_expr(Some(ast));
                    if op == "ite" {
                        args[1] = self
                            .visit_guarded_expression(guard_var.clone(), true, &mut args[1].clone())
                            .unwrap();
                        args[2] = self
                            .visit_guarded_expression(guard_var.clone(), false, &args[2])
                            .unwrap();
                    } else if op == "||" {
                        args[1] = self
                            .visit_guarded_expression(guard_var.clone(), false, &args[1])
                            .unwrap();
                    } else if op == "&&" {
                        args[1] = self
                            .visit_guarded_expression(guard_var.clone(), true, &mut args[1].clone())
                            .unwrap();
                    }
                    // println!("=============={ast:?}");
                    ast.try_as_function_call_expr_ref()
                        .unwrap()
                        .borrow_mut()
                        .function_call_expr_base_mut_ref()
                        .args = args;
                }

                self.visit_children(ast)
            }
        } else if ast
            .to_ast()
            .try_as_expression_ref()
            .unwrap()
            .try_as_function_call_expr_ref()
            .unwrap()
            .is_cast()
        {
            // """Casts are handled either in public or inside the circuit depending on the privacy of the casted expression."""
            assert!(is_instance(
                ast.to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .func()
                    .ast_base_ref()
                    .unwrap()
                    .borrow()
                    .target
                    .clone()
                    .unwrap()
                    .upgrade()
                    .as_ref()
                    .unwrap(),
                ASTType::EnumDefinition
            ));
            if ast
                .to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_function_call_expr_ref()
                .unwrap()
                .args()[0]
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .evaluate_privately()
            {
                let privacy_label = ast
                    .ast_base_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .privacy_annotation
                    .as_ref()
                    .unwrap()
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .privacy_annotation_label();
                self.generator
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .evaluate_expr_in_circuit(
                        ast,
                        &privacy_label.unwrap().into(),
                        &ast.ast_base_ref()
                            .unwrap()
                            .borrow()
                            .annotated_type()
                            .as_ref()
                            .unwrap()
                            .borrow()
                            .homomorphism,
                    )
                    .ok_or(eyre::eyre!("unexpected"))
            } else {
                self.visit_children(ast)
            }
        } else {
            // """
            // Handle normal function calls (outside private expression case).

            // The called functions are allowed to have side effects,
            // if the function does not require verification it can even be recursive.
            // """
            assert!(is_instance(
                ast.to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .func(),
                ASTType::LocationExprBase
            ));
            let mut ast = self.visit_children(ast).unwrap();
            // println!("=={}==={}====={ast:?}",file!(),line!());
            if ast
                .to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_function_call_expr_ref()
                .unwrap()
                .func()
                .ast_base_ref()
                .unwrap()
                .borrow()
                .target
                .clone()
                .unwrap()
                .upgrade()
                .unwrap()
                .to_ast()
                .try_as_namespace_definition_ref()
                .unwrap()
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .requires_verification_when_external
            {
                //Reroute the function call to the corresponding internal function if the called function was split into external/internal.
                if !is_instance(
                    ast.to_ast()
                        .try_as_expression_ref()
                        .unwrap()
                        .try_as_function_call_expr_ref()
                        .unwrap()
                        .func(),
                    ASTType::IdentifierExpr,
                ) {
                    unimplemented!();
                }
                let name = CFG.lock().unwrap().get_internal_name(
                    &ast.try_as_function_call_expr_ref()
                        .unwrap()
                        .borrow_mut()
                        .function_call_expr_base_mut_ref()
                        .func
                        .ast_base_ref()
                        .unwrap()
                        .borrow()
                        .target
                        .clone()
                        .unwrap()
                        .upgrade()
                        .unwrap(),
                );
                ast.to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .func()
                    .ast_base_ref()
                    .unwrap()
                    .borrow()
                    .idf
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .identifier_base_mut_ref()
                    .name = name;
            }

            if ast
                .to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_function_call_expr_ref()
                .unwrap()
                .func()
                .ast_base_ref()
                .unwrap()
                .borrow()
                .target
                .clone()
                .unwrap()
                .upgrade()
                .unwrap()
                .to_ast()
                .try_as_namespace_definition_ref()
                .unwrap()
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .requires_verification
            {
                //If the target function has an associated circuit, make this function"s circuit aware of the call.
                // let cf = if let AST::Expression(Expression::FunctionCallExpr(fce)) = &ast {
                //     Ok(fce.clone())
                // } else {
                //     None
                // };
                self.generator
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .call_function(&ast);
            } else if ast
                .to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_function_call_expr_ref()
                .unwrap()
                .func()
                .ast_base_ref()
                .unwrap()
                .borrow()
                .target
                .clone()
                .unwrap()
                .upgrade()
                .unwrap()
                .to_ast()
                .try_as_namespace_definition_ref()
                .unwrap()
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .has_side_effects()
                && self.generator.is_some()
            {
                //Invalidate modified state variables for the current circuit

                for val in &ast.ast_base_ref().unwrap().borrow().modified_values {
                    if val.key().is_none()
                        && is_instance(
                            &val.target().clone().unwrap(),
                            ASTType::StateVariableDeclaration,
                        )
                    {
                        self.generator
                            .as_ref()
                            .unwrap()
                            .borrow_mut()
                            .invalidate_idf(
                                &val.target()
                                    .as_ref()
                                    .unwrap()
                                    .try_as_identifier_declaration_ref()
                                    .unwrap()
                                    .borrow()
                                    .idf()
                                    .unwrap(),
                            );
                    }
                }
            }

            //The call will be present as a normal function call in the output solidity code.
            Ok(ast)
        }
    }
    pub fn visit_guarded_expression(
        &self,
        guard_var: HybridArgumentIdf,
        if_true: bool,
        expr: &ASTFlatten,
    ) -> Option<ASTFlatten> {
        let prelen = expr
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .statement()
            .clone()
            .unwrap()
            .upgrade()
            .unwrap()
            .try_as_statement_ref()
            .unwrap()
            .borrow()
            .statement_base_ref()
            .unwrap()
            .pre_statements
            .len();

        //Transform expression with guard condition in effect
        self.generator
            .as_ref()
            .unwrap()
            .borrow_mut()
            .guarded(guard_var.clone(), if_true);
        let ret = self.visit(expr);

        //If new pre statements were added, they must be guarded using an if statement in the public solidity code
        let new_pre_stmts: Vec<_> = expr
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .statement()
            .clone()
            .unwrap()
            .upgrade()
            .unwrap()
            .try_as_statement_ref()
            .unwrap()
            .borrow()
            .statement_base_ref()
            .unwrap()
            .pre_statements[prelen..]
            .iter()
            .map(|ps| ps.clone_inner())
            .collect();
        if !new_pre_stmts.is_empty() {
            let mut cond_expr = guard_var.get_loc_expr(None);
            if is_instance(&cond_expr, ASTType::BooleanLiteralExpr) {
                let v = (cond_expr
                    .try_as_boolean_literal_type_ref()
                    .unwrap()
                    .borrow()
                    .value()
                    == "true")
                    == if_true;
                cond_expr = RcCell::new(BooleanLiteralExpr::new(v)).into();
            } else if !if_true {
                cond_expr = RcCell::new(
                    cond_expr
                        .try_as_expression()
                        .unwrap()
                        .borrow()
                        .unop(String::from("!")),
                )
                .into();
            }
            let ps = expr
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .statement()
                .clone()
                .unwrap()
                .upgrade()
                .unwrap()
                .try_as_statement_ref()
                .unwrap()
                .borrow()
                .statement_base_ref()
                .unwrap()
                .pre_statements[..prelen]
                .iter()
                .map(|ps| ps.clone_inner())
                .chain([RcCell::new(IfStatement::new(
                    cond_expr.clone(),
                    RcCell::new(Block::new(new_pre_stmts, false)),
                    None,
                ))
                .clone()
                .into()])
                .collect();
            expr.try_as_expression_ref()
                .unwrap()
                .borrow_mut()
                .expression_base_mut_ref()
                .statement
                .clone()
                .unwrap()
                .upgrade()
                .unwrap()
                .try_as_statement_ref()
                .unwrap()
                .borrow_mut()
                .statement_base_mut_ref()
                .unwrap()
                .pre_statements = ps;
        }
        ret
    }
    // """Casts are handled either in public or inside the circuit depending on the privacy of the casted expression."""
    pub fn visitPrimitiveCastExpr(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        if ast
            .try_as_primitive_cast_expr_ref()
            .unwrap()
            .borrow()
            .expression_base
            .evaluate_privately
        {
            let privacy_label = ast
                .ast_base_ref()
                .unwrap()
                .borrow()
                .annotated_type
                .as_ref()
                .unwrap()
                .borrow()
                .privacy_annotation
                .as_ref()
                .unwrap()
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .privacy_annotation_label();
            self.generator
                .as_ref()
                .unwrap()
                .borrow_mut()
                .evaluate_expr_in_circuit(
                    ast,
                    &(privacy_label.unwrap().into()),
                    &ast.ast_base_ref()
                        .unwrap()
                        .borrow()
                        .annotated_type
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .homomorphism,
                )
                .ok_or(eyre::eyre!("unexpected"))
        } else {
            self.visit_children(ast)
        }
    }
    #[allow(unreachable_code)]
    pub fn visitExpression(&self, _ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        // panic!("=======visitExpression================NotImplementedError==========={_ast:?}");
        // raise NotImplementedError()
        // unimplemented!();
        eyre::bail!("NotImplementedError")
    }
}
// class ZkayCircuitTransformer(AstTransformerVisitor)
// """
// Corresponds to T_phi from paper.

// This extends the abstract circuit representation while transforming private expressions and statements.
// Private expressions can never have side effects.
// Private statements may contain assignment statements with lhs@me (no other types of side effects are allowed).
// """
#[derive(Clone, AstTransformerVisitorBaseRefImpl)]
pub struct ZkayCircuitTransformer {
    ast_transformer_visitor_base: AstTransformerVisitorBase,
    generator: Option<RcCell<CircuitHelper>>,
    global_vars: RcCell<GlobalVars>,
}

impl TransformerVisitorEx for ZkayCircuitTransformer {
    fn visitBlock(
        &self,
        ast: &ASTFlatten,
        guard_cond: Option<HybridArgumentIdf>,
        guard_val: Option<bool>,
    ) -> eyre::Result<ASTFlatten> {
        self.generator
            .as_ref()
            .unwrap()
            .borrow_mut()
            .add_block_to_circuit(ast, guard_cond, guard_val)
            .ok_or(eyre::eyre!("unexpected"))
    }
}
impl AstTransformerVisitor for ZkayCircuitTransformer {
    // fn default() -> Self {
    //     Self::new(None)
    // }

    // type Return = Option<ASTFlatten>;
    // fn temper_result(&self) -> Option<ASTFlatten> {None}

    fn has_attr(&self, name: &ASTType, ast: &AST) -> bool {
        matches!(
            name,
            ASTType::LiteralExprBase
                | ASTType::IndexExpr
                | ASTType::IdentifierExpr
                | ASTType::ExpressionBase
                | ASTType::FunctionCallExprBase
                | ASTType::ReturnStatement
                | ASTType::AssignmentStatementBase
                | ASTType::VariableDeclarationStatement
                | ASTType::IfStatement
                | ASTType::Block
                | ASTType::StatementBase
                | ASTType::ASTBase
        ) || matches!(ast, AST::Expression(Expression::ReclassifyExpr(_)))
            || matches!(ast, AST::Expression(Expression::LiteralExpr(_)))
            || matches!(ast, AST::Expression(Expression::FunctionCallExpr(_)))
            || matches!(ast, AST::Expression(_))
            || matches!(
                ast,
                AST::Statement(Statement::SimpleStatement(
                    SimpleStatement::AssignmentStatement(_)
                ))
            )
            || matches!(ast, AST::Statement(_))
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        match name {
            ASTType::IndexExpr => self.visitIndexExpr(ast),
            ASTType::IdentifierExpr => self.visitIdentifierExpr(ast),
            ASTType::ReturnStatement => self.visitReturnStatement(ast),
            ASTType::VariableDeclarationStatement => self.visitVariableDeclarationStatement(ast),
            ASTType::IfStatement => self.visitIfStatement(ast),
            ASTType::Block => self.visitBlock(ast, None, None),
            _ if matches!(ast.to_ast(), AST::Expression(Expression::ReclassifyExpr(_))) => {
                self.visitReclassifyExpr(ast)
            }
            _ if matches!(ast.to_ast(), AST::Expression(Expression::LiteralExpr(_))) => {
                self.visitLiteralExpr(ast)
            }
            _ if matches!(
                ast.to_ast(),
                AST::Expression(Expression::FunctionCallExpr(_))
            ) =>
            {
                self.visitFunctionCallExpr(ast)
            }
            _ if matches!(
                ast.to_ast(),
                AST::Statement(Statement::SimpleStatement(
                    SimpleStatement::AssignmentStatement(_)
                ))
            ) =>
            {
                self.visitAssignmentStatement(ast)
            }
            _ if matches!(ast.to_ast(), AST::Expression(_)) => self.visitExpression(ast),
            _ if matches!(ast.to_ast(), AST::Statement(_)) => self.visitStatement(ast),
            ASTType::ASTBase => <Self as AstTransformerVisitor>::visitAST(self, ast),
            _ => Err(eyre::eyre!("unreach")),
        }
    }
}
impl ZkayCircuitTransformer {
    // super().__init__()
    // self.generator.unwrap() = current_generator
    pub fn new(
        current_generator: Option<WeakCell<CircuitHelper>>,
        global_vars: RcCell<GlobalVars>,
    ) -> Self {
        // println!("===ZkayCircuitTransformer===========current_generator========={:?},{:?}",current_generator.as_ref().is_some(),current_generator.clone().and_then(|g| g.upgrade()).is_some());
        Self {
            ast_transformer_visitor_base: AstTransformerVisitorBase::new(false),
            generator: current_generator.and_then(|g| g.upgrade()),
            global_vars,
        }
    }
    // """Rule (13), don"t modify constants."""
    pub fn visitLiteralExpr(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        Ok(ast.clone())
    }

    pub fn visitIndexExpr(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        // println!("======visitIndexExpr===============transform_location======");
        // panic!("==");
        self.transform_location(ast)
            .ok_or(eyre::eyre!("unexpected"))
    }

    pub fn visitIdentifierExpr(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        // println!("======visitIdentifierExpr=======beg=========={:?}====",ast.get_ast_type());
        let mut ast = ast.clone();
        if !is_instance(
            ast.ast_base_ref().unwrap().borrow().idf.as_ref().unwrap(),
            ASTType::HybridArgumentIdf,
        ) {
            // println!("======visitIdentifierExpr=======!=========={:?}====",ast.get_ast_type());
            //If ast is not already transformed, get current SSA version
            ast = self
                .generator
                .as_ref()
                .unwrap()
                .borrow()
                .get_remapped_idf_expr(ast);
        }
        // println!("======visitIdentifierExpr=======pp=========={:?}====",ast.get_ast_type());
        if is_instance(&ast, ASTType::IdentifierExpr)
            && is_instance(
                ast.ast_base_ref().unwrap().borrow().idf.as_ref().unwrap(),
                ASTType::HybridArgumentIdf,
            )
        {
            // println!("======visitIdentifierExpr=========h========{:?}====",ast.get_ast_type());
            //The current version of ast.idf is already in the circuit
            assert!(
                ast.ast_base_ref()
                    .unwrap()
                    .borrow()
                    .idf
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .try_as_hybrid_argument_idf_ref()
                    .unwrap()
                    .arg_type
                    != HybridArgType::PubContractVal
            );
            Ok(ast)
        } else {
            //ast is not yet in the circuit -> move it in
            // println!("======visitIdentifierExpr===============transform_location=={:?}====",ast.get_ast_type());
            self.transform_location(&ast)
                .ok_or(eyre::eyre!("unexpected"))
        }
    }
    // """Rule (14), move location into the circuit."""
    pub fn transform_location(&self, loc: &ASTFlatten) -> Option<ASTFlatten> {
        // println!("===========loc=====================");
        self.generator
            .as_ref()
            .unwrap()
            .borrow()
            .add_to_circuit_inputs(loc)
            .get_idf_expr(None)
    }
    // """Rule (15), boundary crossing if analysis determined that it is """
    pub fn visitReclassifyExpr(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        // println!(
        //     "=====visitReclassifyExpr====15====begin==={:?}==={}===={}===={:?}========={:?}=",
        //     ast.get_ast_type(),
        //     ast,
        //     ast.to_ast()
        //         .try_as_expression_ref()
        //         .unwrap()
        //         .try_as_reclassify_expr_ref()
        //         .unwrap()
        //         .expr(),
        //     ast.to_ast()
        //         .try_as_expression_ref()
        //         .unwrap()
        //         .try_as_reclassify_expr_ref()
        //         .unwrap()
        //         .expr()
        //         .get_ast_type(),ast
        //     .ast_base_ref()
        //     .unwrap()
        //     .borrow()
        //     .annotated_type()
        //     .as_ref()
        //     .unwrap()
        //     .borrow().type_name.as_ref().unwrap().get_ast_type()
        // );
        //  println!("=====0===is_cipher={:?}={}=",ast
        //     .ast_base_ref()
        //     .unwrap()
        //     .borrow()
        //     .annotated_type() .as_ref()
        //     .unwrap()
        //     .borrow().get_ast_type(),ast
        //     .ast_base_ref()
        //     .unwrap()
        //     .borrow()
        //     .annotated_type()
        //     .as_ref()
        //     .unwrap()
        //     .borrow()
        //     .is_cipher());
        if ast
            .ast_base_ref()
            .unwrap()
            .borrow()
            .annotated_type()
            .as_ref()
            .unwrap()
            .borrow()
            .is_cipher()
        {
            // println!("=====1===is_cipher===");
            //We need a homomorphic ciphertext -> make sure the correct encryption of the value is available
            let orig_type = ast
                .ast_base_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .zkay_type();
            let orig_privacy = orig_type
                .privacy_annotation
                .unwrap()
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .privacy_annotation_label();
            let orig_homomorphism = orig_type.homomorphism;
            self.generator
                .as_ref()
                .unwrap()
                .borrow()
                .evaluate_expr_in_circuit(
                    ast.to_ast()
                        .try_as_expression_ref()
                        .unwrap()
                        .try_as_reclassify_expr_ref()
                        .unwrap()
                        .expr(),
                    &orig_privacy.unwrap().into(),
                    &orig_homomorphism,
                )
                .ok_or(eyre::eyre!("unexpected"))
        } else if ast
            .to_ast()
            .try_as_expression_ref()
            .unwrap()
            .try_as_reclassify_expr_ref()
            .unwrap()
            .expr()
            .to_ast()
            .try_as_expression_ref()
            .unwrap()
            .evaluate_privately()
        {
            // println!("=====1==2====");
            self.visit(
                ast.to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_reclassify_expr_ref()
                    .unwrap()
                    .expr(),
            )
            .ok_or(eyre::eyre!("unexpected"))
        } else {
            assert!(ast
                .to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_reclassify_expr_ref()
                .unwrap()
                .expr()
                .ast_base_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .is_public());
            // println!(
            //     "===visitReclassifyExpr===add_to_circuit_inputs=====ast=={}====={}=",
            //     ast,
            //     ast.to_ast()
            //         .try_as_expression_ref()
            //         .unwrap()
            //         .try_as_reclassify_expr_ref()
            //         .unwrap()
            //         .expr()
            // );
            // if ast.to_ast()
            //         .try_as_expression_ref()
            //         .unwrap()
            //         .try_as_reclassify_expr_ref()
            //         .unwrap()
            //         .expr()
            //         .get_ast_type()==ASTType::PrimitiveCastExpr{
            //         //TODO
            //         eyre::bail!("")
            // }
            self.generator
                .as_ref()
                .unwrap()
                .borrow()
                .add_to_circuit_inputs(
                    &ast.to_ast()
                        .try_as_expression_ref()
                        .unwrap()
                        .try_as_reclassify_expr_ref()
                        .unwrap()
                        .expr(),
                )
                .get_idf_expr(None)
                .ok_or(eyre::eyre!("unexpected"))
        }
    }
    // """Rule (16), other expressions don"t need special treatment."""
    pub fn visitExpression(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        // println!(
        //     "===visitExpression==========get_ast_type============={:?}",
        //     ast.get_ast_type()
        // );
        self.visit_children(ast)
    }

    pub fn visitFunctionCallExpr(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        // println!(
        //     "===visitFunctionCallExpr==========code============={:?}",
        //     ast.to_string()
        // );
        let t = ast
            .ast_base_ref()
            .unwrap()
            .borrow()
            .annotated_type()
            .as_ref()
            .unwrap()
            .borrow()
            .type_name
            .clone()
            .unwrap();

        //Constant folding for literal types
        if let TypeName::ElementaryTypeName(ElementaryTypeName::BooleanLiteralType(t)) =
            &t.to_ast().try_as_type_name().unwrap().clone()
        {
            return replace_expr(
                ast,
                &RcCell::new(BooleanLiteralExpr::new(t.value() == "true")).into(),
                false,
                self.global_vars.clone(),
            )
            .ok_or(eyre::eyre!("unexpected"));
        } else if let TypeName::ElementaryTypeName(ElementaryTypeName::NumberTypeName(
            NumberTypeName::NumberLiteralType(t),
        )) = t.to_ast().try_as_type_name().unwrap()
        {
            return replace_expr(
                &ast,
                &RcCell::new(NumberLiteralExpr::new(
                    t.value().parse::<i32>().unwrap(),
                    false,
                ))
                .into(),
                false,
                self.global_vars.clone(),
            )
            .ok_or(eyre::eyre!("unexpected"));
        }
        // println!(
        //     "=BuiltinFunction===={:?}===={:?}=={:?}======{:?}=",
        //     ast.to_string(),
        //     ast.get_ast_type(),
        //     ast.to_ast()
        //         .try_as_expression_ref()
        //         .unwrap()
        //         .try_as_function_call_expr_ref()
        //         .unwrap()
        //         .func()
        //         .get_ast_type(),
        //     ast.to_ast()
        //         .try_as_expression_ref()
        //         .unwrap()
        //         .try_as_function_call_expr_ref()
        //         .unwrap()
        //         .func()
        //         .to_string()
        // );
        if is_instance(
            ast.to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_function_call_expr_ref()
                .unwrap()
                .func(),
            ASTType::BuiltinFunction,
        ) {
            // println!(
            //     "=BuiltinFunction=in==={:?}===={:?}=={:?}======{:?}=",
            //     ast.to_string(),
            //     ast.get_ast_type(),
            //     ast.to_ast()
            //         .try_as_expression_ref()
            //         .unwrap()
            //         .try_as_function_call_expr_ref()
            //         .unwrap()
            //         .func()
            //         .get_ast_type(),
            //     ast.to_ast()
            //         .try_as_expression_ref()
            //         .unwrap()
            //         .try_as_function_call_expr_ref()
            //         .unwrap()
            //         .func()
            //         .to_string()
            // );
            if ast
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
                .homomorphism
                != Homomorphism::non_homomorphic()
            {
                //To perform homomorphic operations, we require the recipient"s public key

                let crypto_params = CFG.lock().unwrap().user_config.get_crypto_params(
                    &ast.to_ast()
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
                        .homomorphism,
                );
                let recipient = ast
                    .ast_base_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .zkay_type()
                    .privacy_annotation
                    .unwrap()
                    .try_as_expression_ref()
                    .unwrap()
                    .borrow()
                    .privacy_annotation_label();
                let mut s = ast
                    .to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .statement()
                    .as_ref()
                    .and_then(|t| t.clone().upgrade());
                let public_key = Some(RcCell::new(
                    self.generator
                        .as_ref()
                        .unwrap()
                        .borrow()
                        ._require_public_key_for_label_at(
                            s.as_ref(),
                            &recipient.unwrap().into(),
                            &crypto_params,
                        ),
                ));
                if ast.is_function_call_expr() {
                    ast.try_as_function_call_expr_ref()
                        .unwrap()
                        .borrow_mut()
                        .function_call_expr_base_mut_ref()
                        .public_key = public_key;
                } else if ast.is_expression() {
                    ast.try_as_expression_ref()
                        .unwrap()
                        .borrow_mut()
                        .try_as_function_call_expr_mut()
                        .unwrap()
                        .function_call_expr_base_mut_ref()
                        .public_key = public_key;
                } else {
                    panic!("============else============{ast:?}");
                }
                let mut args = ast
                    .to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .args()
                    .clone();
                if &ast
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
                    == "*"
                {
                    //special case: private scalar multiplication using additive homomorphism
                    //TODO ugly hack below removes ReclassifyExpr
                    let mut new_args = vec![];
                    for arg in args {
                        let mut arg = arg.clone();
                        if is_instance(&arg, ASTType::ReclassifyExpr) {
                            let a = arg
                                .try_as_reclassify_expr_ref()
                                .unwrap()
                                .borrow()
                                .expr()
                                .clone();
                            arg = a;
                            let rerand_using = Some(
                                self.generator
                                    .as_ref()
                                    .unwrap()
                                    .borrow_mut()
                                    .get_randomness_for_rerand(ast),
                            );
                            let mut func = ast
                                .to_ast()
                                .try_as_expression_ref()
                                .unwrap()
                                .try_as_function_call_expr_ref()
                                .unwrap()
                                .func()
                                .clone();
                            if func.is_builtin_function() {
                                ast.to_ast()
                                    .try_as_expression_ref()
                                    .unwrap()
                                    .try_as_function_call_expr_ref()
                                    .unwrap()
                                    .func()
                                    .try_as_builtin_function_ref()
                                    .unwrap()
                                    .borrow_mut()
                                    .rerand_using = rerand_using;
                            } else {
                                panic!("=============else=========={func:?}");
                            }
                        //result requires re-randomization
                        } else if arg
                            .ast_base_ref()
                            .unwrap()
                            .borrow()
                            .annotated_type()
                            .as_ref()
                            .unwrap()
                            .borrow()
                            .is_private()
                        {
                            // println!("===visitFunctionCallExpr=======cipher_type========assign==========annotated_type=========");
                            arg.ast_base_ref().unwrap().borrow_mut().annotated_type =
                                Some(AnnotatedTypeName::cipher_type(
                                    arg.ast_base_ref()
                                        .unwrap()
                                        .borrow()
                                        .annotated_type
                                        .as_ref()
                                        .unwrap()
                                        .clone(),
                                    Some(
                                        ast.try_as_function_call_expr_ref()
                                            .unwrap()
                                            .borrow()
                                            .func()
                                            .try_as_builtin_function_ref()
                                            .unwrap()
                                            .borrow()
                                            .homomorphism
                                            .clone(),
                                    ),
                                ));
                        }
                        new_args.push(arg);
                    }

                    if ast.is_function_call_expr() {
                        ast.try_as_function_call_expr_ref()
                            .unwrap()
                            .borrow_mut()
                            .function_call_expr_base_mut_ref()
                            .args = new_args;
                    } else if ast.is_expression() {
                        ast.try_as_expression_ref()
                            .unwrap()
                            .borrow_mut()
                            .try_as_function_call_expr_mut()
                            .unwrap()
                            .function_call_expr_base_mut_ref()
                            .args = new_args;
                    } else {
                        panic!("============else============{ast:?}");
                    }
                } else {
                    //We require all non-public arguments to be present as ciphertexts
                    for arg in &args {
                        if arg
                            .ast_base_ref()
                            .unwrap()
                            .borrow()
                            .annotated_type()
                            .as_ref()
                            .unwrap()
                            .borrow()
                            .is_private()
                        {
                            // println!("===visitFunctionCallExpr==is_private=====cipher_type========assign====*****======annotated_type======={}======={:?}=",arg,arg.get_ast_type());
                            let at = Some(AnnotatedTypeName::cipher_type(
                                arg.ast_base_ref()
                                    .unwrap()
                                    .borrow()
                                    .annotated_type()
                                    .as_ref()
                                    .unwrap()
                                    .clone(),
                                Some(
                                    ast.to_ast()
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
                                        .homomorphism
                                        .clone(),
                                ),
                            ));
                            arg.ast_base_ref().unwrap().borrow_mut().annotated_type = at;
                            // println!(
                            //     "=========annotated_type======typename======={:?}==========",
                            //     arg.ast_base_ref()
                            //         .unwrap()
                            //         .borrow()
                            //         .annotated_type
                            //         .as_ref()
                            //         .unwrap()
                            //         .borrow()
                            //         .type_name
                            //         .as_ref()
                            //         .unwrap()
                            //         .get_ast_type()
                            // );
                        }
                    }

                    if ast.is_function_call_expr() {
                        ast.try_as_function_call_expr_ref()
                            .unwrap()
                            .borrow_mut()
                            .function_call_expr_base_mut_ref()
                            .args = args;
                    } else if ast.is_expression() {
                        ast.try_as_expression_ref()
                            .unwrap()
                            .borrow_mut()
                            .try_as_function_call_expr_mut()
                            .unwrap()
                            .function_call_expr_base_mut_ref()
                            .args = args;
                    } else {
                        panic!("============else============{ast:?}");
                    }
                }
            }

            //Builtin functions are supported natively by the circuit
            // println!(
            //     "===visitFunctionCallExpr===visit_children=================={}",
            //     ast
            // );
            return self.visit_children(ast);
        }

        let fdef = ast
            .try_as_function_call_expr_ref()
            .unwrap()
            .borrow()
            .func()
            .ast_base_ref()
            .unwrap()
            .borrow()
            .target
            .clone()
            .unwrap()
            .upgrade()
            .unwrap();
        assert!(fdef
            .try_as_namespace_definition_ref()
            .unwrap()
            .borrow()
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .is_function());
        assert!(!fdef
            .try_as_namespace_definition_ref()
            .unwrap()
            .borrow()
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .return_parameters
            .is_empty());
        assert!(
            fdef.try_as_namespace_definition_ref()
                .unwrap()
                .borrow()
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .has_static_body
        );

        //Function call inside private expression -> entire body will be inlined into circuit.
        //Function must not have side-effects (only pure and view is allowed) and cannot have a nonstatic body (i.e. recursion)
        self.generator
            .as_ref()
            .unwrap()
            .borrow_mut()
            .inline_function_call_into_circuit(ast)
            .ok_or(eyre::eyre!("unexpected"))
    }

    pub fn visitReturnStatement(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        self.generator
            .as_ref()
            .unwrap()
            .borrow_mut()
            .add_return_stmt_to_circuit(ast)
            .ok_or(eyre::eyre!("unexpected"))
    }

    pub fn visitAssignmentStatement(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        self.generator
            .as_ref()
            .unwrap()
            .borrow_mut()
            .add_assignment_to_circuit(ast)
            .ok_or(eyre::eyre!("unexpected"))
    }

    pub fn visitVariableDeclarationStatement(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        self.generator
            .as_ref()
            .unwrap()
            .borrow_mut()
            .add_var_decl_to_circuit(ast)
            .ok_or(eyre::eyre!("unexpected"))
    }

    pub fn visitIfStatement(&self, ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        self.generator
            .as_ref()
            .unwrap()
            .borrow_mut()
            .add_if_statement_to_circuit(ast)
            .ok_or(eyre::eyre!("unexpected"))
    }

    // """Fail if statement type was not handled."""
    // raise NotImplementedError("Unsupported statement")
    #[allow(unreachable_code)]
    pub fn visitStatement(&self, _ast: &ASTFlatten) -> eyre::Result<ASTFlatten> {
        unimplemented!("Unsupported statement");
        Ok(_ast.clone())
    }
}
