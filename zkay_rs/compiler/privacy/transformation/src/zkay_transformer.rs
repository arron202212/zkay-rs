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
use rccell::{RcCell, WeakCell};
use regex::Regex;
use regex::RegexSetBuilder;
use solidity::fake_solidity_generator::{ID_PATTERN, WS_PATTERN};
use zkay_ast::analysis::contains_private_checker::contains_private_expr;
use zkay_ast::ast::{
    is_instance, ASTBaseProperty, ASTBaseRef, ASTChildren, ASTFlatten, ASTType, AnnotatedTypeName,
    AssignmentStatement, AssignmentStatementBaseMutRef, AssignmentStatementBaseProperty, BlankLine,
    Block, BooleanLiteralExpr, BooleanLiteralType, BreakStatement, BuiltinFunction,
    ChildListBuilder, Comment, CommentBase, ContinueStatement, DoWhileStatement,
    ElementaryTypeName, EncryptionExpression, EnumDefinition, ExprUnion, Expression,
    ExpressionBaseMutRef, ExpressionBaseProperty, ForStatement, FunctionCallExpr,
    FunctionCallExprBase, FunctionCallExprBaseMutRef, FunctionCallExprBaseProperty, HybridArgType,
    HybridArgumentIdf, Identifier, IdentifierBase, IdentifierBaseMutRef, IdentifierBaseProperty,
    IdentifierDeclaration, IdentifierDeclarationBaseProperty, IdentifierExpr, IdentifierExprUnion,
    IfStatement, IndexExpr, IntoAST, IntoExpression, IntoStatement, LiteralExpr, LocationExpr,
    LocationExprBaseProperty, Mapping, MeExpr, MemberAccessExpr, NamespaceDefinition,
    NumberLiteralExpr, NumberLiteralType, NumberTypeName, Parameter, PrimitiveCastExpr,
    ReclassifyExpr, ReclassifyExprBaseMutRef, ReclassifyExprBaseProperty, ReturnStatement,
    SimpleStatement, StateVariableDeclaration, Statement, StatementBaseMutRef,
    StatementBaseProperty, StatementList, StatementListBaseMutRef, StatementListBaseProperty,
    TupleExpr, TypeName, VariableDeclaration, VariableDeclarationStatement, WhileStatement, AST,
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
    ast_transformer_visitor_base: AstTransformerVisitorBase,
    expr_trafo: Option<ZkayExpressionTransformer>,
}
impl AstTransformerVisitor for ZkayVarDeclTransformer {
    fn default() -> Self {
        Self::new()
    }
    type Return = Option<ASTFlatten>;
    fn temper_result(&self) -> Self::Return {}

    fn has_attr(&self, name: &ASTType) -> bool {
        matches!(
            name,
            ASTType::AnnotatedTypeName
                | ASTType::VariableDeclaration
                | ASTType::Parameter
                | ASTType::StateVariableDeclaration
                | ASTType::Mapping
        )
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> Self::Return {
        match name {
            ASTType::AnnotatedTypeName => self.visitAnnotatedTypeName(ast),
            ASTType::VariableDeclaration => self.visitVariableDeclaration(ast),
            ASTType::Parameter => self.visitParameter(ast),
            ASTType::StateVariableDeclaration => self.visitStateVariableDeclaration(ast),
            ASTType::Mapping => self.visitMapping(ast),
            _ => {}
        }
    }
}
impl ZkayVarDeclTransformer {
    pub fn new() -> Self {
        Self {
            ast_transformer_visitor_base: AstTransformerVisitorBase::new(false),
            expr_trafo: None,
        }
    }

    pub fn visitAnnotatedTypeName(
        &self,
        ast: &ASTFlatten,
    ) -> <Self as AstTransformerVisitor>::Return {
        let t = if ast
            .try_as_annotated_type_name_ref()
            .unwrap()
            .borrow()
            .is_private()
        {
            Some(TypeName::cipher_type(
                ast.try_as_annotated_type_name_ref().unwrap().clone(),
                ast.try_as_annotated_type_name_ref()
                    .unwrap()
                    .borrow()
                    .homomorphism
                    .clone(),
            ))
        } else {
            self.visit(
                &ast.try_as_annotated_type_name_ref()
                    .unwrap()
                    .borrow()
                    .type_name
                    .clone()
                    .into(),
            )
        };
        RcCell::new(AnnotatedTypeName::new(
            t,
            None,
            String::from("NON_HOMOMORPHISM"),
        ))
        .into()
    }

    pub fn visitVariableDeclaration(
        &self,
        ast: &ASTFlatten,
    ) -> <Self as AstTransformerVisitor>::Return {
        if ast
            .try_as_variable_declaration_ref()
            .unwrap()
            .borrow()
            .identifier_declaration_base
            .annotated_type
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

    pub fn visitParameter(&self, ast: &ASTFlatten) -> <Self as AstTransformerVisitor>::Return {
        let ast = self.visit_children(ast);
        if is_instance(&ast, ASTType::Parameter) {
            if !ast
                .try_as_parameter_ref()
                .unwrap()
                .borrow()
                .identifier_declaration_base
                .annotated_type
                .type_name
                .as_ref()
                .unwrap()
                .is_primitive_type()
            {
                ast.try_as_parameter_ref()
                    .unwrap()
                    .borrow_mut()
                    .identifier_declaration_base
                    .storage_location = Some(String::from("memory"));
            }
            Some(ast)
        } else {
            None
        }
    }

    pub fn visitStateVariableDeclaration(
        &self,
        ast: &ASTFlatten,
    ) -> <Self as AstTransformerVisitor>::Return {
        ast.try_as_state_variable_declaration_ref()
            .unwrap()
            .borrow_mut()
            .identifier_declaration_base
            .keywords = ast
            .try_as_state_variable_declaration_ref()
            .unwrap()
            .borrow()
            .identifier_declaration_base
            .keywords
            .iter()
            .filter_map(|k| if k != "public" { Some(k.clone()) } else { None })
            .collect();
        //make sure every state var gets a public getter (required for simulation)
        ast.try_as_state_variable_declaration_ref()
            .unwrap()
            .borrow_mut()
            .identifier_declaration_base
            .keywords
            .push(String::from("public"));
        ast.try_as_state_variable_declaration_ref()
            .unwrap()
            .borrow_mut()
            .expr = self.expr_trafo.as_ref().unwrap().visit(
            &ast.try_as_state_variable_declaration_ref()
                .unwrap()
                .borrow_mut()
                .expr
                .clone()
                .into(),
        );
        self.visit_children(ast)
    }

    pub fn visitMapping(&self, ast: &ASTFlatten) -> <Self as AstTransformerVisitor>::Return {
        if ast
            .try_as_mapping_ref()
            .unwrap()
            .borrow()
            .key_label
            .is_some()
        {
            let kl = ast
                .try_as_mapping_ref()
                .unwrap()
                .borrow()
                .key_label
                .as_ref()
                .unwrap()
                .borrow()
                .name()
                .clone();
            ast.try_as_mapping_ref().unwrap().borrow_mut().key_label =
                Some(RcCell::new(Identifier::Identifier(IdentifierBase::new(kl))));
        }
        self.visit_children(ast)
    }
}
// class ZkayStatementTransformer(AstTransformerVisitor)
// """Corresponds to T from paper, (with additional handling of return statement and loops)."""
#[derive(Clone)]
pub struct ZkayStatementTransformer {
    ast_transformer_visitor_base: AstTransformerVisitorBase,
    gen: Option<RcCell<CircuitHelper>>,
    expr_trafo: ZkayExpressionTransformer,
    var_decl_trafo: ZkayVarDeclTransformer,
}
impl AstTransformerVisitor for ZkayStatementTransformer {
    fn default() -> Self {
        Self::new(None)
    }

    type Return = Option<ASTFlatten>;
    fn temper_result(&self) -> Self::Return {
        None
    }

    fn has_attr(&self, name: &ASTType) -> bool {
        matches!(
            name,
            ASTType::StatementList
                | ASTType::Statement
                | ASTType::AssignmentStatement
                | ASTType::IfStatement
                | ASTType::WhileStatement
                | ASTType::DoWhileStatement
                | ASTType::ForStatement
                | ASTType::ContinueStatement
                | ASTType::BreakStatement
                | ASTType::ReturnStatement
                | ASTType::Expression
        )
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> Self::Return {
        match name {
            ASTType::StatementList => self.visitStatementList(ast),
            ASTType::Statement => self.visitStatement(ast),
            ASTType::AssignmentStatement => self.visitAssignmentStatement(ast),
            ASTType::IfStatement => self.visitIfStatement(ast),
            ASTType::WhileStatement => self.visitWhileStatement(ast),
            ASTType::DoWhileStatement => self.visitDoWhileStatement(ast),
            ASTType::ForStatement => self.visitForStatement(ast),
            ASTType::ContinueStatement => self.visitContinueStatement(ast),
            ASTType::BreakStatement => self.visitBreakStatement(ast),
            ASTType::ReturnStatement => self.visitReturnStatement(ast),
            ASTType::Expression => self.visitExpression(ast),
            _ => {}
        }
    }
}
impl ZkayStatementTransformer {
    // pub fn __init__(&self, current_gen: CircuitHelper)
    //     super().__init__()
    //     self.gen.unwrap() = current_gen
    //     self.expr_trafo = ZkayExpressionTransformer(self.gen.unwrap())
    //     self.var_decl_trafo = ZkayVarDeclTransformer()
    pub fn new(current_gen: Option<RcCell<CircuitHelper>>) -> Self {
        Self {
            ast_transformer_visitor_base: AstTransformerVisitorBase::new(false),
            gen: current_gen.clone(),
            expr_trafo: ZkayExpressionTransformer::new(current_gen),
            var_decl_trafo: ZkayVarDeclTransformer::new(),
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
    pub fn visitStatementList(&self, ast: &ASTFlatten) -> <Self as AstTransformerVisitor>::Return {
        let mut new_statements = vec![];
        for (_idx, stmt) in ast
            .try_as_statement_list_ref()
            .unwrap()
            .borrow()
            .statements()
            .iter()
            .enumerate()
        {
            let old_code = stmt.clone().into().code();
            let transformed_stmt = self.visit(&stmt.clone().into());
            if transformed_stmt.is_none() {
                continue;
            }
            let r = Regex::new(&format!(r"@{WS_PATTERN}*{ID_PATTERN}"))
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
                        .try_as_statement_ref()
                        .unwrap()
                        .statement_base_ref()
                        .unwrap()
                        .pre_statements
                        .iter()
                        .cloned()
                        .chain([transformed_stmt.as_ref().unwrap().clone()])
                        .collect(),
                ));
            }
        }
        if !new_statements.is_empty()
            && is_instance(new_statements.last().unwrap(), ASTType::BlankLine)
        {
            new_statements.pop();
        }
        ast.try_as_statement_list_ref()
            .unwrap()
            .borrow_mut()
            .statement_list_base_mut_ref()
            .statements = new_statements;
        Some(ast.clone())
    }
    // """Default statement child handling. Expressions and declarations are visited by the corresponding transformers."""
    pub fn process_statement_child(
        &self,
        child: &ASTFlatten,
    ) -> <Self as AstTransformerVisitor>::Return {
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
    pub fn visitStatement(&self, ast: &ASTFlatten) -> <Self as AstTransformerVisitor>::Return {
        assert!(
            is_instance(ast, ASTType::SimpleStatementBase)
                || is_instance(ast, ASTType::VariableDeclarationStatement)
        );
        let mut cb = ChildListBuilder::new();
        ast.try_as_statement_ref()
            .borrow_mut()
            .process_children(&mut cb);
        cb.children.iter().for_each(|c| {
            self.process_statement_child(c);
        });
        Some(ast.clone())
    }
    // """Rule (2)"""
    pub fn visitAssignmentStatement(
        &self,
        ast: &ASTFlatten,
    ) -> <Self as AstTransformerVisitor>::Return {
        let a = self.expr_trafo.visit(
            &ast.try_as_assignment_statement_ref()
                .unwrap()
                .borrow_mut()
                .lhs()
                .clone()
                .into(),
        );
        ast.try_as_assignment_statement_ref()
            .unwrap()
            .borrow_mut()
            .assignment_statement_base_mut_ref()
            .lhs = Some(Box::new(a));
        ast.try_as_assignment_statement_ref()
            .unwrap()
            .borrow_mut()
            .assignment_statement_base_mut_ref()
            .rhs = self
            .expr_trafo
            .visit(&ast.rhs().clone().into())
            .unwrap()
            .try_as_expression();
        let mut modvals = ast
            .try_as_assignment_statement_ref()
            .unwrap()
            .borrow_mut()
            .modified_values()
            .clone();
        if CFG.lock().unwrap().user_config.opt_cache_circuit_outputs()
            && is_instance(
                ast.try_as_assignment_statement_ref()
                    .unwrap()
                    .borrow_mut()
                    .lhs()
                    .as_ref()
                    .unwrap(),
                ASTType::IdentifierExpr,
            )
            && is_instance(
                ast.try_as_assignment_statement_ref()
                    .unwrap()
                    .borrow_mut()
                    .rhs()
                    .as_ref()
                    .unwrap(),
                ASTType::MemberAccessExpr,
            )
        {
            //Skip invalidation if rhs is circuit output
            if is_instance(
                &ast.try_as_assignment_statement_ref()
                    .unwrap()
                    .borrow()
                    .rhs()
                    .as_ref()
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
                .try_as_assignment_statement_ref()
                .unwrap()
                .borrow()
                .rhs()
                .as_ref()
                .unwrap()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .try_as_member_access_expr_ref()
                .unwrap()
                .member
                .try_as_hybrid_argument_idf_ref()
                .unwrap()
                .arg_type
                == HybridArgType::PubCircuitArg
            {
                modvals = modvals
                    .iter()
                    .filter_map(|mv| {
                        if mv.target().map(|t| *t.clone())
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
                    ast.try_as_assignment_statement_ref()
                        .unwrap()
                        .borrow()
                        .rhs()
                        .as_ref()
                        .unwrap()
                        .try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .try_as_location_expr_ref()
                        .unwrap()
                        .try_as_member_access_expr_ref()
                        .unwrap()
                        .member
                        .try_as_hybrid_argument_idf_ref()
                        .unwrap()
                        .corresponding_priv_expression
                        .as_ref()
                        .unwrap(),
                    ASTType::EncryptionExpression,
                ) {
                    (*ast
                        .try_as_assignment_statement_ref()
                        .unwrap()
                        .borrow()
                        .rhs()
                        .as_ref()
                        .unwrap()
                        .try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .try_as_location_expr_ref()
                        .unwrap()
                        .try_as_member_access_expr_ref()
                        .unwrap()
                        .member
                        .try_as_hybrid_argument_idf_ref()
                        .unwrap()
                        .corresponding_priv_expression
                        .as_ref()
                        .unwrap()
                        .try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .try_as_location_expr_ref()
                        .unwrap()
                        .try_as_identifier_expr_ref()
                        .unwrap()
                        .idf
                        .as_ref()
                        .unwrap()
                        .borrow())
                    .clone()
                } else {
                    (*ast
                        .try_as_assignment_statement_ref()
                        .unwrap()
                        .borrow()
                        .rhs()
                        .as_ref()
                        .unwrap()
                        .try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .try_as_location_expr_ref()
                        .unwrap()
                        .try_as_member_access_expr_ref()
                        .unwrap()
                        .member
                        .try_as_hybrid_argument_idf_ref()
                        .unwrap()
                        .corresponding_priv_expression
                        .as_ref()
                        .unwrap()
                        .try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .try_as_location_expr_ref()
                        .unwrap()
                        .try_as_identifier_expr_ref()
                        .unwrap()
                        .idf
                        .as_ref()
                        .unwrap()
                        .borrow())
                    .clone()
                };
                assert!(is_instance(&ridf, ASTType::HybridArgumentIdf));
                if let Identifier::HybridArgumentIdf(ridf) = ridf {
                    self.gen.as_ref().unwrap().borrow_mut()._remapper.0.remap(
                        (*ast
                            .try_as_assignment_statement_ref()
                            .unwrap()
                            .borrow()
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
                            .as_ref()
                            .unwrap()
                            .try_as_identifier_declaration_ref()
                            .unwrap()
                            .idf()
                            .upgrade()
                            .unwrap()
                            .borrow())
                        .clone(),
                        ridf,
                    );
                }
            }
        }
        //Invalidate circuit value for assignment targets
        if self.gen.is_some() {
            for val in modvals {
                if val.key().is_none() {
                    self.gen.as_ref().unwrap().borrow_mut().invalidate_idf(
                        &(*val
                            .target()
                            .as_ref()
                            .unwrap()
                            .try_as_identifier_declaration_ref()
                            .unwrap()
                            .idf()
                            .upgrade()
                            .unwrap()
                            .borrow())
                        .clone(),
                    );
                }
            }
        }
        Some(ast.clone())
    }
    // """
    // Rule (6) + additional support for private conditions

    // If the condition is public, guard conditions are introduced for both branches if any of the branches contains private expressions.

    // If the condition is private, the whole if statement is inlined into the circuit. The only side-effects which are allowed
    // inside the branch bodies are assignment statements with an lhs@me. (anything else would leak private information).
    // The if statement will be replaced by an assignment statement where the lhs is a tuple of all locations which are written
    // in either branch and rhs is a tuple of the corresponding circuit outputs.
    // """
    pub fn visitIfStatement(&self, ast: &ASTFlatten) -> <Self as AstTransformerVisitor>::Return {
        if ast
            .try_as_if_statement_ref()
            .unwrap()
            .borrow()
            .condition
            .annotated_type()
            .as_ref()
            .unwrap()
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
                    .clone()
                    .into(),
            ) {
                let before_if_state = self.gen.as_ref().unwrap()._remapper.0.get_state();
                let guard_var = self
                    .gen
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .add_to_circuit_inputs(
                        &ast.try_as_if_statement_ref().unwrap().borrow().condition,
                    );
                ast.try_as_if_statement_ref()
                    .unwrap()
                    .borrow_mut()
                    .condition = guard_var.get_loc_expr(ast).try_as_expression().unwrap();
                self.gen
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
                        .try_as_statement()
                        .unwrap()
                        .try_as_statement_list()
                        .unwrap()
                        .try_as_block()
                        .unwrap();
                    self.gen
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
                    self.gen
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
                                .clone()
                                .into(),
                        )
                        .unwrap()
                        .try_as_statement()
                        .unwrap()
                        .try_as_statement_list()
                        .unwrap()
                        .try_as_block();
                    self.gen
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
                    .modified_values
                {
                    if val.key().is_none() {
                        self.gen.as_ref().unwrap().borrow_mut().invalidate_idf(
                            &(*val
                                .target()
                                .as_ref()
                                .unwrap()
                                .try_as_identifier_declaration_ref()
                                .unwrap()
                                .idf()
                                .upgrade()
                                .unwrap()
                                .borrow())
                            .clone(),
                        );
                    }
                }
            } else {
                ast.try_as_if_statement_ref()
                    .unwrap()
                    .borrow_mut()
                    .condition = self
                    .expr_trafo
                    .visit(
                        &ast.try_as_if_statement_ref()
                            .unwrap()
                            .borrow()
                            .condition
                            .clone()
                            .into(),
                    )
                    .unwrap()
                    .try_as_expression()
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
                    .try_as_statement_ref()
                    .unwrap()
                    .try_as_statement_list_ref()
                    .unwrap()
                    .try_as_block_ref()
                    .unwrap()
                    .clone();
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
                                .clone()
                                .into(),
                        )
                        .unwrap()
                        .try_as_statement()
                        .unwrap()
                        .try_as_statement_list()
                        .unwrap()
                        .try_as_block();
                }
            }
            Some(ast.clone())
        } else {
            self.gen
                .as_ref()
                .unwrap()
                .borrow_mut()
                .evaluate_stmt_in_circuit(ast)
        }
    }
    pub fn visitWhileStatement(&self, ast: &ASTFlatten) -> <Self as AstTransformerVisitor>::Return
//Loops must always be purely public
    {
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
        Some(ast.clone())
    }
    //Loops must always be purely public
    pub fn visitDoWhileStatement(
        &self,
        ast: &ASTFlatten,
    ) -> <Self as AstTransformerVisitor>::Return {
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
        Some(ast.clone())
    }

    pub fn visitForStatement(&self, ast: &ASTFlatten) -> <Self as AstTransformerVisitor>::Return {
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
                        .into(),
                )
                .unwrap()
                .try_as_statement_ref()
                .unwrap()
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
                        .pre_statements()
                        .clone(),
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
                    &ast.try_as_for_statement_ref().unwrap().borrow().update..clone().into()
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
        Some(ast.clone())
    }

    pub fn visitContinueStatement(
        &self,
        ast: &ASTFlatten,
    ) -> <Self as AstTransformerVisitor>::Return {
        Some(ast.clone())
    }

    pub fn visitBreakStatement(&self, ast: &ASTFlatten) -> <Self as AstTransformerVisitor>::Return {
        Some(ast.clone())
    }
    // """
    // Handle return statement.

    // If the function requires verification, the return statement is replaced by an assignment to a return variable.
    // (which will be returned at the very end of the function body, after any verification wrapper code).
    // Otherwise only the expression is transformed.
    // """
    pub fn visitReturnStatement(
        &self,
        ast: &ASTFlatten,
    ) -> <Self as AstTransformerVisitor>::Return {
        if ast
            .try_as_return_statement_ref()
            .unwrap()
            .borrow()
            .statement_base
            .function
            .as_ref()
            .unwrap()
            .requires_verification
        {
            if ast
                .try_as_return_statement_ref()
                .unwrap()
                .borrow()
                .expr
                .is_none()
            {
                return None;
            }
            assert!(!self.gen.as_ref().unwrap().has_return_var);
            self.gen.as_ref().unwrap().borrow_mut().has_return_var = true;
            let expr = self.expr_trafo.visit(Some(
                ast.try_as_return_statement_ref()
                    .unwrap()
                    .borrow()
                    .expr
                    .as_ref()
                    .unwrap()
                    .clone()
                    .into(),
            ));
            let ret_args = ast
                .try_as_return_statement_ref()
                .unwrap()
                .borrow()
                .statement_base
                .function
                .as_ref()
                .unwrap()
                .return_var_decls
                .iter()
                .map(|vd| {
                    let mut idf = IdentifierExpr::new(
                        IdentifierExprUnion::Identifier(
                            (*vd.identifier_declaration_base
                                .idf
                                .as_ref()
                                .unwrap()
                                .borrow())
                            .clone(),
                        ),
                        None,
                    );
                    idf.location_expr_base.target = Some(RcCell::new(vd.clone().into()));
                    RcCell::new(idf.to_expr())
                })
                .collect();
            let mut te =
                TupleExpr::new(ret_args).assign(expr.unwrap().try_as_expression().unwrap());
            te.statement_base_mut_ref().pre_statements = ast
                .try_as_return_statement_ref()
                .unwrap()
                .borrow()
                .statement_base
                .pre_statements
                .clone();
            Some(RcCell::new(te).into())
        } else {
            ast.try_as_return_statement_ref().unwrap().borrow_mut().expr = self.expr_trafo.visit(
                &ast.try_as_return_statement_ref()
                    .unwrap()
                    .borrow()
                    .expr
                    .clone()
                    .into(),
            );
            Some(ast.clone().into())
        }
    }
    // """Fail if there are any untransformed expressions left."""
    pub fn visitExpression(&self, ast: &ASTFlatten) -> <Self as AstTransformerVisitor>::Return {
        assert!(false, "Missed an expression of type {:?}", ast);
        Some(ast.clone())
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
    ast_transformer_visitor_base: AstTransformerVisitorBase,
    gen: Option<RcCell<CircuitHelper>>,
}
impl TransformerVisitorEx for ZkayExpressionTransformer {}
impl AstTransformerVisitor for ZkayExpressionTransformer {
    fn default() -> Self {
        Self::new(None)
    }

    type Return = Option<ASTFlatten>;
    fn temper_result(&self) -> Self::Return {
        None
    }

    fn has_attr(&self, name: &ASTType) -> bool {
        matches!(
            name,
            ASTType::MeExpr
                | ASTType::LiteralExpr
                | ASTType::IdentifierExpr
                | ASTType::IndexExpr
                | ASTType::MemberAccessExpr
                | ASTType::TupleExpr
                | ASTType::ReclassifyExpr
                | ASTType::BuiltinFunction
                | ASTType::FunctionCallExpr
                | ASTType::PrimitiveCastExpr
                | ASTType::Expression
        )
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> Self::Return {
        match name {
            ASTType::MeExpr => self.visitMeExpr(ast),
            ASTType::LiteralExpr => self.visitLiteralExpr(ast),
            ASTType::IdentifierExpr => self.visitIdentifierExpr(ast),
            ASTType::IndexExpr => self.visitIndexExpr(ast),
            ASTType::MemberAccessExpr => self.visitMemberAccessExpr(ast),
            ASTType::TupleExpr => self.visitTupleExpr(ast),
            ASTType::ReclassifyExpr => self.visitReclassifyExpr(ast),
            ASTType::BuiltinFunction => self.visitBuiltinFunction(ast),
            ASTType::FunctionCallExpr => self.visitFunctionCallExpr(ast),
            ASTType::PrimitiveCastExpr => self.visitPrimitiveCastExpr(ast),
            ASTType::Expression => self.visitExpression(ast),

            _ => {}
        }
    }
}
impl ZkayExpressionTransformer {
    pub fn new(current_generator: Option<RcCell<CircuitHelper>>) -> Self
// super().__init__()
        // self.gen.unwrap() = current_generator
    {
        Self {
            ast_transformer_visitor_base: AstTransformerVisitorBase::new(false),
            gen: current_generator,
        }
    }

    // @staticmethod
    // """Replace me with msg.sender."""
    pub fn visitMeExpr(ast: &ASTFlatten) -> <Self as AstTransformerVisitor>::Return {
        replace_expr(
            ast,
            &RcCell::new(LocationExpr::IdentifierExpr(IdentifierExpr::new(
                IdentifierExprUnion::String(String::from("msg")),
                None,
            )))
            .into(),
            false,
        )
        .try_as_tuple_or_location_expr_ref()
        .unwrap()
        .try_as_location_expr_ref()
        .unwrap()
        .dot(IdentifierExprUnion::String(String::from("sender")))
        .as_type(&AnnotatedTypeName::address_all().into())
    }
    // """Rule (7), don"t modify constants."""

    pub fn visitLiteralExpr(&self, _ast: &ASTFlatten) -> <Self as AstTransformerVisitor>::Return {
        Some(ast.clone())
    }
    // """Rule (8), don"t modify identifiers."""

    pub fn visitIdentifierExpr(
        &self,
        _ast: &ASTFlatten,
    ) -> <Self as AstTransformerVisitor>::Return {
        Some(ast.clone())
    }
    // """Rule (9), transform location and index expressions separately."""

    pub fn visitIndexExpr(&self, ast: &ASTFlatten) -> <Self as AstTransformerVisitor>::Return {
        replace_expr(
            ast,
            &self
                .visit(
                    &ast.try_as_index_expr_ref()
                        .unwrap()
                        .borrow()
                        .arr
                        .clone()
                        .into(),
                )
                .unwrap()
                .try_as_expression_ref()
                .unwrap()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .index(ExprUnion::Expression(
                    self.visit(
                        &ast.try_as_index_expr_ref()
                            .unwrap()
                            .borrow()
                            .key
                            .clone()
                            .into(),
                    )
                    .unwrap()
                    .try_as_expression()
                    .unwrap(),
                )),
            false,
        )
    }

    pub fn visitMemberAccessExpr(
        &self,
        ast: &ASTFlatten,
    ) -> <Self as AstTransformerVisitor>::Return {
        self.visit_children(ast)
    }

    pub fn visitTupleExpr(&self, ast: &ASTFlatten) -> <Self as AstTransformerVisitor>::Return {
        self.visit_children(ast)
    }
    // """
    // Rule (11), trigger a boundary crossing.

    // The reclassified expression is evaluated in the circuit and its result is made available in solidity.
    // """
    pub fn visitReclassifyExpr(&self, ast: &ASTFlatten) -> <Self as AstTransformerVisitor>::Return {
        let mut expr = ast
            .try_as_reclassify_expr_ref()
            .unwrap()
            .borrow()
            .reclassify_expr_base_mut_ref()
            .expr
            .clone()
            .into();
        self.gen
            .as_ref()
            .unwrap()
            .borrow_mut()
            .evaluate_expr_in_circuit(
                &expr,
                &ast.try_as_reclassify_expr_ref()
                    .unwrap()
                    .borrow()
                    .privacy()
                    .privacy_annotation_label()
                    .unwrap()
                    .into(),
                &ast.try_as_reclassify_expr_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .homomorphism,
            )
    }

    pub fn visitBuiltinFunction(
        &self,
        ast: &ASTFlatten,
    ) -> <Self as AstTransformerVisitor>::Return {
        Some(ast.clone())
    }

    pub fn visitFunctionCallExpr(
        &self,
        ast: &ASTFlatten,
    ) -> <Self as AstTransformerVisitor>::Return {
        if is_instance(
            &ast.try_as_function_call_expr_ref().unwrap().borrow().func(),
            ASTType::BuiltinFunction,
        ) {
            // """
            // Modified Rule (12) builtin functions with private operands and homomorphic operations on ciphertexts
            // are evaluated inside the circuit.

            // A private expression on its own (like an IdentifierExpr referring to a private variable) is not enough to trigger a
            // boundary crossing (assignment of private variables is a public operation).
            // """
            if ast
                .try_as_function_call_expr_ref()
                .unwrap()
                .borrow()
                .func()
                .func()
                .try_as_builtin_function_ref()
                .unwrap()
                .is_private
            {
                let privacy_label = ast
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .privacy_annotation
                    .as_ref()
                    .unwrap()
                    .try_as_expression_ref()
                    .unwrap()
                    .privacy_annotation_label();
                self.gen
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .evaluate_expr_in_circuit(
                        &mut ast.to_expr(),
                        &(privacy_label.unwrap().into()),
                        &(ast
                            .try_as_function_call_expr_ref()
                            .unwrap()
                            .borrow()
                            .func()
                            .try_as_builtin_function_ref()
                            .unwrap()
                            .homomorphism
                            .clone()
                            .into()),
                    )
                    .clone()
                    .into()
            } else {
                // """
                // Rule (10) with additional short-circuit handling.

                // Builtin operations on public operands are normally left untransformed, but if the builtin function has
                // short-circuiting semantics, guard conditions must be added if any of the public operands contains
                // nested private expressions.
                // """
                //handle short-circuiting
                let mut args = ast
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .borrow()
                    .args()
                    .clone();
                if ast
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .borrow()
                    .func()
                    .try_as_builtin_function_ref()
                    .unwrap()
                    .has_shortcircuiting()
                    && args[1..]
                        .iter()
                        .any(|arg| contains_private_expr(&mut Some(arg.clone().into())))
                {
                    let op = &ast
                        .try_as_function_call_expr_ref()
                        .unwrap()
                        .borrow()
                        .func()
                        .try_as_builtin_function_ref()
                        .unwrap()
                        .op;
                    let guard_var = self
                        .gen
                        .as_ref()
                        .unwrap()
                        .borrow_mut()
                        .add_to_circuit_inputs(&mut args[0]);
                    args[0] = guard_var.get_loc_expr(ast).try_as_expression().unwrap();
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
                    ast.try_as_function_call_expr_ref()
                        .unwrap()
                        .borrow_mut()
                        .function_call_expr_base_mut_ref()
                        .args = args;
                }

                self.visit_children(ast)
            }
        } else if ast
            .try_as_function_call_expr_ref()
            .unwrap()
            .borrow()
            .is_cast()
        {
            // """Casts are handled either in public or inside the circuit depending on the privacy of the casted expression."""

            assert!(is_instance(
                ast.try_as_function_call_expr_ref()
                    .unwrap()
                    .borrow()
                    .func()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_location_expr_ref()
                    .unwrap()
                    .target()
                    .as_ref()
                    .unwrap(),
                ASTType::EnumDefinition
            ));
            if ast.try_as_function_call_expr_ref().unwrap().borrow().args()[0].evaluate_privately()
            {
                let privacy_label = ast
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .privacy_annotation
                    .as_ref()
                    .unwrap()
                    .try_as_expression_ref()
                    .unwrap()
                    .privacy_annotation_label();
                self.gen
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .evaluate_expr_in_circuit(
                        ast,
                        &privacy_label.unwrap().into(),
                        &ast.annotated_type().as_ref().unwrap().homomorphism,
                    )
                    .clone()
                    .into()
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
                &ast.try_as_function_call_expr_ref().unwrap().borrow().func(),
                ASTType::LocationExprBase
            ));
            let mut ast = self.visit_children(ast);
            if ast
                .try_as_function_call_expr_ref()
                .unwrap()
                .borrow()
                .func()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .target()
                .as_ref()
                .unwrap()
                .try_as_namespace_definition_ref()
                .unwrap()
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .requires_verification_when_external
            {
                //Reroute the function call to the corresponding internal function if the called function was split into external/internal.

                if !is_instance(
                    &ast.try_as_function_call_expr_ref().unwrap().borrow().func(),
                    ASTType::IdentifierExpr,
                ) {
                    unimplemented!();
                }
                ast.try_as_function_call_expr_ref()
                    .unwrap()
                    .borrow_mut()
                    .function_call_expr_base_mut_ref()
                    .func
                    .try_as_tuple_or_location_expr_mut()
                    .unwrap()
                    .try_as_location_expr_mut()
                    .unwrap()
                    .try_as_identifier_expr_mut()
                    .unwrap()
                    .idf
                    .as_mut()
                    .unwrap()
                    .borrow_mut()
                    .identifier_base_mut_ref()
                    .name = CFG.lock().unwrap().get_internal_name(
                    ast.try_as_function_call_expr_ref()
                        .unwrap()
                        .borrow_mut()
                        .function_call_expr_base_mut_ref()
                        .func
                        .try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .try_as_location_expr_ref()
                        .unwrap()
                        .target()
                        .as_ref()
                        .unwrap(),
                );
            }

            if ast
                .try_as_function_call_expr_ref()
                .unwrap()
                .borrow()
                .func()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .target()
                .as_ref()
                .unwrap()
                .try_as_namespace_definition_ref()
                .unwrap()
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .requires_verification
            {
                //If the target function has an associated circuit, make this function"s circuit aware of the call.
                // let cf = if let AST::Expression(Expression::FunctionCallExpr(fce)) = &ast {
                //     Some(fce.clone())
                // } else {
                //     None
                // };
                self.gen.as_ref().unwrap().borrow_mut().call_function(&ast);
            } else if ast
                .try_as_function_call_expr_ref()
                .unwrap()
                .borrow()
                .func()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .target()
                .as_ref()
                .unwrap()
                .try_as_namespace_definition_ref()
                .unwrap()
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .has_side_effects()
                && self.gen.is_some()
            //Invalidate modified state variables for the current circuit
            {
                for val in &ast
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .borrow()
                    .ast_base_ref()
                    .modified_values
                {
                    if val.key().is_none()
                        && is_instance(
                            &val.target().map(|t| *t).unwrap(),
                            ASTType::StateVariableDeclaration,
                        )
                    {
                        self.gen.as_ref().unwrap().borrow_mut().invalidate_idf(
                            &(*val
                                .target()
                                .as_ref()
                                .unwrap()
                                .try_as_identifier_declaration_ref()
                                .unwrap()
                                .idf()
                                .upgrade()
                                .unwrap()
                                .borrow())
                            .clone(),
                        );
                    }
                }
            }

            //The call will be present as a normal function call in the output solidity code.
            Some(ast)
        }
    }
    pub fn visit_guarded_expression(
        &self,
        guard_var: HybridArgumentIdf,
        if_true: bool,
        expr: &ASTFlatten,
    ) -> <Self as AstTransformerVisitor>::Return {
        let prelen = expr
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .statement()
            .as_ref()
            .unwrap()
            .statement_base_ref()
            .unwrap()
            .pre_statements
            .len();

        //Transform expression with guard condition in effect
        self.gen
            .as_ref()
            .unwrap()
            .borrow_mut()
            .guarded(guard_var.clone(), if_true);
        let ret = self.visit(expr);

        //If new pre statements were added, they must be guarded using an if statement in the public solidity code
        let new_pre_stmts = expr
            .try_as_expression_ref()
            .unwrap()
            .borrow()
            .statement()
            .as_ref()
            .unwrap()
            .statement_base_ref()
            .unwrap()
            .pre_statements[prelen..]
            .to_vec();
        if !new_pre_stmts.is_empty() {
            let mut cond_expr = guard_var.get_loc_expr(None);
            if is_instance(cond_expr, ASTType::BooleanLiteralExpr) {
                *cond_expr
                    .try_as_boolean_literal_type_ref()
                    .unwrap()
                    .borrow_mut() =
                    RcCell::new(BooleanLiteralExpr::new(cond_expr.value == if_true));
            } else if !if_true {
                cond_expr = cond_expr
                    .try_as_expression()
                    .unwrap()
                    .unop(String::from("!"));
            }
            let ps = expr
                .try_as_expression_ref()
                .unwrap()
                .borrow()
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
                .clone()
                .into()])
                .collect();
            expr.try_as_expression_ref()
                .unwrap()
                .borrow_mut()
                .expression_base_mut_ref()
                .statement
                .as_mut()
                .unwrap()
                .statement_base_mut_ref()
                .unwrap()
                .pre_statements = ps;
        }
        ret
    }
    // """Casts are handled either in public or inside the circuit depending on the privacy of the casted expression."""
    pub fn visitPrimitiveCastExpr(
        &self,
        ast: &ASTFlatten,
    ) -> <Self as AstTransformerVisitor>::Return {
        if ast
            .try_as_primitive_cast_expr_ref()
            .unwrap()
            .borrow()
            .expression_base
            .evaluate_privately
        {
            let privacy_label = ast
                .try_as_primitive_cast_expr_ref()
                .unwrap()
                .borrow()
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
                .as_ref()
                .unwrap()
                .borrow_mut()
                .evaluate_expr_in_circuit(
                    ast,
                    &(privacy_label.unwrap().into()),
                    &ast.expression_base
                        .annotated_type
                        .as_ref()
                        .unwrap()
                        .homomorphism,
                )
                .clone()
                .into()
        } else {
            self.visit_children(ast)
        }
    }

    pub fn visitExpression(&self, ast: &ASTFlatten) -> <Self as AstTransformerVisitor>::Return {
        // raise NotImplementedError()
        unimplemented!();
        Some(ast.clone())
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
    ast_transformer_visitor_base: AstTransformerVisitorBase,
    gen: Option<RcCell<CircuitHelper>>,
}

impl TransformerVisitorEx for ZkayCircuitTransformer {}
impl AstTransformerVisitor for ZkayCircuitTransformer {
    fn default() -> Self {
        Self::new(None)
    }

    type Return = Option<ASTFlatten>;
    fn temper_result(&self) -> Self::Return {}

    fn has_attr(&self, name: &ASTType) -> bool {
        matches!(
            name,
            ASTType::LiteralExpr
                | ASTType::IndexExpr
                | ASTType::IdentifierExpr
                | ASTType::ReclassifyExpr
                | ASTType::Expression
                | ASTType::FunctionCallExpr
                | ASTType::ReturnStatement
                | ASTType::AssignmentStatement
                | ASTType::VariableDeclarationStatement
                | ASTType::IfStatement
                | ASTType::Block
                | ASTType::Statement
        )
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> Self::Return {
        match name {
            ASTType::LiteralExpr => self.visitLiteralExpr(ast),
            ASTType::IndexExpr => self.visitIndexExpr(ast),
            ASTType::IdentifierExpr => self.visitIdentifierExpr(ast),
            ASTType::ReclassifyExpr => self.visitReclassifyExpr(ast),
            ASTType::Expression => self.visitExpression(ast),
            ASTType::FunctionCallExpr => self.visitFunctionCallExpr(ast),
            ASTType::ReturnStatement => self.visitReturnStatement(ast),
            ASTType::AssignmentStatement => self.visitAssignmentStatement(ast),
            ASTType::VariableDeclarationStatement => self.visitVariableDeclarationStatement(ast),
            ASTType::IfStatement => self.visitIfStatement(ast),
            ASTType::Block => self.visitBlock(ast),
            ASTType::Statement => self.visitStatement(ast),
            _ => {}
        }
    }
}
impl ZkayCircuitTransformer {
    // super().__init__()
    // self.gen.unwrap() = current_generator
    pub fn new(current_generator: Option<RcCell<CircuitHelper>>) -> Self {
        Self {
            ast_transformer_visitor_base: AstTransformerVisitorBase::new(false),
            gen: current_generator,
        }
    }
    // """Rule (13), don"t modify constants."""
    pub fn visitLiteralExpr(&self, ast: &ASTFlatten) -> <Self as AstTransformerVisitor>::Return {
        Some(ast.clone())
    }

    pub fn visitIndexExpr(&self, ast: &ASTFlatten) -> <Self as AstTransformerVisitor>::Return {
        self.transform_location(ast)
    }

    pub fn visitIdentifierExpr(&self, ast: &ASTFlatten) -> <Self as AstTransformerVisitor>::Return {
        let mut ast = ast.clone();
        if !is_instance(
            ast.try_as_identifier_expr_ref()
                .unwrap()
                .borrow()
                .idf
                .as_ref()
                .unwrap(),
            ASTType::HybridArgumentIdf,
        ) {
            //If ast is not already transformed, get current SSA version
            ast = self.gen.as_ref().unwrap().get_remapped_idf_expr(ast);
        }
        if is_instance(&ast, ASTType::IdentifierExpr)
            && is_instance(
                ast.try_as_identifier_expr_ref()
                    .unwrap()
                    .borrow()
                    .idf
                    .as_ref()
                    .unwrap(),
                ASTType::HybridArgumentIdf,
            )
        {
            //The current version of ast.idf is already in the circuit

            assert!(
                ast.try_as_identifier_expr_ref()
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
            ast
        } else {
            //ast is not yet in the circuit -> move it in
            self.transform_location(ast)
        }
    }
    // """Rule (14), move location into the circuit."""
    pub fn transform_location(&self, loc: LocationExpr) -> <Self as AstTransformerVisitor>::Return {
        self.gen
            .as_ref()
            .unwrap()
            .borrow_mut()
            .add_to_circuit_inputs(&mut loc.to_expr())
            .get_idf_expr(&None)
    }
    // """Rule (15), boundary crossing if analysis determined that it is """

    pub fn visitReclassifyExpr(&self, ast: &ASTFlatten) -> <Self as AstTransformerVisitor>::Return {
        if ast
            .try_as_reclassify_expr_ref()
            .unwrap()
            .borrow()
            .annotated_type()
            .as_ref()
            .unwrap()
            .is_cipher()
        {
            //We need a homomorphic ciphertext -> make sure the correct encryption of the value is available

            let orig_type = ast
                .try_as_reclassify_expr_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .zkay_type();
            let orig_privacy = orig_type
                .privacy_annotation
                .unwrap()
                .try_as_expression_ref()
                .unwrap()
                .privacy_annotation_label();
            let orig_homomorphism = orig_type.homomorphism;
            self.gen
                .as_ref()
                .unwrap()
                .borrow_mut()
                .evaluate_expr_in_circuit(
                    &ast.try_as_reclassify_expr_ref()
                        .unwrap()
                        .borrow()
                        .reclassify_expr_base_mut_ref()
                        .expr,
                    &orig_privacy.unwrap().into(),
                    &orig_homomorphism,
                )
                .clone()
                .into()
        } else if ast
            .try_as_reclassify_expr_ref()
            .unwrap()
            .borrow()
            .expr()
            .evaluate_privately()
        {
            self.visit(
                &ast.try_as_reclassify_expr_ref()
                    .unwrap()
                    .borrow()
                    .expr()
                    .into(),
            )
            .unwrap()
        } else {
            assert!(ast
                .try_as_reclassify_expr_ref()
                .unwrap()
                .borrow()
                .expr()
                .annotated_type()
                .as_ref()
                .unwrap()
                .is_public());
            self.gen
                .as_ref()
                .unwrap()
                .borrow_mut()
                .add_to_circuit_inputs(
                    &ast.try_as_reclassify_expr_ref()
                        .unwrap()
                        .borrow()
                        .reclassify_expr_base_mut_ref()
                        .expr,
                )
                .get_idf_expr(&None)
                .clone()
                .into()
        }
    }
    // """Rule (16), other expressions don"t need special treatment."""
    pub fn visitExpression(&self, ast: &ASTFlatten) -> <Self as AstTransformerVisitor>::Return {
        self.visit_children(ast)
    }

    pub fn visitFunctionCallExpr(
        &self,
        ast: &ASTFlatten,
    ) -> <Self as AstTransformerVisitor>::Return {
        let t = ast
            .try_as_function_call_expr_ref()
            .unwrap()
            .borrow()
            .annotated_type()
            .as_ref()
            .unwrap()
            .type_name
            .clone()
            .unwrap();

        //Constant folding for literal types
        if let TypeName::ElementaryTypeName(ElementaryTypeName::BooleanLiteralType(t)) = &t {
            return replace_expr(
                ast,
                &RcCell::newBooleanLiteralExpr::new(t.value()).into(),
                false,
            );
        } else if let TypeName::ElementaryTypeName(ElementaryTypeName::NumberTypeName(
            NumberTypeName::NumberLiteralType(t),
        )) = &t
        {
            return replace_expr(
                &ast,
                &RcCell::new(NumberLiteralExpr::new(t.value(), false)).into(),
                false,
            );
        }

        if is_instance(
            &**ast.try_as_function_call_expr_ref().unwrap().borrow().func(),
            ASTType::BuiltinFunction,
        ) {
            if ast
                .try_as_function_call_expr_ref()
                .unwrap()
                .borrow()
                .func()
                .try_as_builtin_function_ref()
                .unwrap()
                .homomorphism
                != Homomorphism::non_homomorphic()
            {
                //To perform homomorphic operations, we require the recipient"s public key

                let crypto_params = CFG.lock().unwrap().user_config.get_crypto_params(
                    &ast.try_as_function_call_expr_ref()
                        .unwrap()
                        .borrow()
                        .func()
                        .try_as_builtin_function_ref()
                        .unwrap()
                        .homomorphism,
                );
                let recipient = ast
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .borrow()
                    .annotated_type()
                    .as_ref()
                    .unwrap()
                    .zkay_type()
                    .privacy_annotation
                    .unwrap()
                    .try_as_expression_ref()
                    .unwrap()
                    .privacy_annotation_label();
                let mut s = ast
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .borrow()
                    .statement();
                ast.function_call_expr_base_mut_ref().public_key = Some(Box::new(
                    self.gen
                        .as_ref()
                        .unwrap()
                        .borrow_mut()
                        ._require_public_key_for_label_at(
                            Some(&mut s),
                            &recipient.unwrap().into(),
                            &crypto_params,
                        ),
                ));
                let mut args = ast
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .borrow()
                    .args()
                    .clone();
                if &ast
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .borrow()
                    .func()
                    .try_as_builtin_function_ref()
                    .unwrap()
                    .op
                    == "*"
                //special case: private scalar multiplication using additive homomorphism
                //TODO ugly hack below removes ReclassifyExpr
                {
                    let mut new_args = vec![];
                    for arg in args {
                        let mut arg = arg.clone();
                        if is_instance(&arg, ASTType::ReclassifyExpr) {
                            arg = *arg.try_as_reclassify_expr_ref().unwrap().expr().clone();
                            ast.try_as_function_call_expr_ref()
                                .unwrap()
                                .borrow_mut()
                                .function_call_expr_base_mut_ref()
                                .func
                                .try_as_builtin_function_mut()
                                .unwrap()
                                .rerand_using = Some(Box::new(
                                self.gen
                                    .as_ref()
                                    .unwrap()
                                    .borrow_mut()
                                    .get_randomness_for_rerand(ast),
                            ));
                        //result requires re-randomization
                        } else if arg.annotated_type().as_ref().unwrap().is_private() {
                            arg.expression_base_mut_ref().annotated_type =
                                Some(AnnotatedTypeName::cipher_type(
                                    arg.annotated_type().as_ref().unwrap().clone(),
                                    Some(
                                        ast.try_as_function_call_expr_ref()
                                            .unwrap()
                                            .borrow()
                                            .func()
                                            .try_as_builtin_function_ref()
                                            .unwrap()
                                            .homomorphism
                                            .clone(),
                                    ),
                                ));
                        }
                        new_args.push(arg);
                    }
                    ast.try_as_function_call_expr_ref()
                        .unwrap()
                        .borrow_mut()
                        .function_call_expr_base_mut_ref()
                        .args = new_args;
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
            return self.visit_children(ast);
        }

        let fdef = ast
            .func()
            .try_as_tuple_or_location_expr_ref()
            .unwrap()
            .try_as_location_expr_ref()
            .unwrap()
            .target()
            .as_ref()
            .unwrap()
            .clone();
        assert!(fdef
            .try_as_namespace_definition_ref()
            .unwrap()
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .is_function());
        assert!(!fdef
            .try_as_namespace_definition_ref()
            .unwrap()
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .return_parameters
            .is_empty());
        assert!(
            fdef.try_as_namespace_definition_ref()
                .unwrap()
                .try_as_constructor_or_function_definition_ref()
                .unwrap()
                .has_static_body
        );

        //Function call inside private expression -> entire body will be inlined into circuit.
        //Function must not have side-effects (only pure and view is allowed) and cannot have a nonstatic body (i.e. recursion)
        self.gen
            .as_ref()
            .unwrap()
            .borrow_mut()
            .inline_function_call_into_circuit(ast);
    }

    pub fn visitReturnStatement(
        &self,
        ast: &ASTFlatten,
    ) -> <Self as AstTransformerVisitor>::Return {
        self.gen
            .as_ref()
            .unwrap()
            .borrow_mut()
            .add_return_stmt_to_circuit(ast)
    }

    pub fn visitAssignmentStatement(
        &self,
        ast: &ASTFlatten,
    ) -> <Self as AstTransformerVisitor>::Return {
        self.gen
            .as_ref()
            .unwrap()
            .borrow_mut()
            .add_assignment_to_circuit(ast)
    }

    pub fn visitVariableDeclarationStatement(
        &self,
        ast: &ASTFlatten,
    ) -> <Self as AstTransformerVisitor>::Return {
        self.gen
            .as_ref()
            .unwrap()
            .borrow_mut()
            .add_var_decl_to_circuit(ast)
    }

    pub fn visitIfStatement(&self, ast: &ASTFlatten) -> <Self as AstTransformerVisitor>::Return {
        self.gen
            .as_ref()
            .unwrap()
            .borrow_mut()
            .add_if_statement_to_circuit(ast)
    }

    pub fn visitBlock(
        &self,
        ast: &ASTFlatten,
        guard_cond: Option<HybridArgumentIdf>,
        guard_val: Option<bool>,
    ) -> <Self as AstTransformerVisitor>::Return {
        self.gen
            .as_ref()
            .unwrap()
            .borrow_mut()
            .add_block_to_circuit(ast, guard_cond, guard_val)
    }
    // """Fail if statement type was not handled."""
    // raise NotImplementedError("Unsupported statement")
    pub fn visitStatement(&self, ast: &ASTFlatten) -> <Self as AstTransformerVisitor>::Return {
        unimplemented!("Unsupported statement");
        Some(ast.clone())
    }
}
