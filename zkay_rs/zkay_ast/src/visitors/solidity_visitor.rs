#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

use crate::ast::{
    indent, is_instance, ASTBaseProperty, ASTFlatten, ASTInstanceOf, ASTType, AnnotatedTypeName,
    ArrayBaseProperty, ArrayLiteralExprBaseProperty, AssignmentStatementBaseProperty, CodeVisitor,
    CodeVisitorBase, CommentBaseProperty, DeepClone, ElementaryTypeNameBaseProperty, Expression,
    FunctionCallExprBaseProperty, Identifier, IdentifierBaseProperty, IntoAST, IntoStatement,
    ListUnion, LiteralExpr, MeExpr, Parameter, ParameterUnion, ReclassifyExprBaseProperty,
    SimpleStatement, SingleOrListUnion, Statement, StatementList, StatementListBaseProperty,
    TypeName, UserDefinedTypeNameBaseRef, AST, LINE_ENDING,
};
use crate::homomorphism::HOMOMORPHISM_STORE;
use crate::visitors::visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef};
use eyre::{eyre, Result};
use rccell::RcCell;
use std::cmp::Ordering;
use zkay_config::config::CFG;
pub fn to_solidity(ast: &ASTFlatten) -> String {
    SolidityVisitor::new().visit(ast)
}

// class SolidityVisitor(CodeVisitor)
impl AstVisitorBaseRef for SolidityVisitor {
    fn ast_visitor_base_ref(&self) -> &AstVisitorBase {
        &self.code_visitor_base.ast_visitor_base
    }
}
pub struct SolidityVisitor {
    pub code_visitor_base: CodeVisitorBase,
}
impl SolidityVisitor {
    // pub fn __init__(self)
    //     // do not display `final` keywords (`final` is not in Solidity fragment)
    //     super().__init__(False)
    pub fn new() -> Self {
        Self {
            code_visitor_base: CodeVisitorBase::new(false),
        }
    }
}
impl AstVisitor for SolidityVisitor {
    type Return = String;
    fn temper_result(&self) -> Self::Return {
        String::new()
    }
    fn has_attr(&self, name: &ASTType, ast: &AST) -> bool {
        // println!("======has_attr========{:?}======",ast.get_ast_type());
        matches!(
            name,
            ASTType::ASTBase
                | ASTType::CommentBase
                | ASTType::IdentifierBase
                | ASTType::FunctionCallExprBase
                | ASTType::PrimitiveCastExpr
                | ASTType::BooleanLiteralExpr
                | ASTType::NumberLiteralExpr
                | ASTType::StringLiteralExpr
                | ASTType::ArrayLiteralExprBase
                | ASTType::TupleExpr
                | ASTType::IdentifierExpr
                | ASTType::MemberAccessExpr
                | ASTType::IndexExpr
                | ASTType::MeExpr
                | ASTType::AllExpr
                | ASTType::ReclassifyExprBase
                | ASTType::RehomExpr
                | ASTType::IfStatement
                | ASTType::WhileStatement
                | ASTType::DoWhileStatement
                | ASTType::ForStatement
                | ASTType::BreakStatement
                | ASTType::ContinueStatement
                | ASTType::ReturnStatement
                | ASTType::ExpressionStatement
                | ASTType::RequireStatement
                | ASTType::AssignmentStatementBase
                | ASTType::CircuitDirectiveStatementBase
                | ASTType::StatementListBase
                | ASTType::Block
                | ASTType::IndentBlock
                | ASTType::ElementaryTypeNameBase
                | ASTType::UserDefinedTypeNameBase
                | ASTType::AddressTypeName
                | ASTType::AddressPayableTypeName
                | ASTType::AnnotatedTypeName
                | ASTType::Mapping
                | ASTType::ArrayBase
                | ASTType::CipherText
                | ASTType::TupleType
                | ASTType::VariableDeclaration
                | ASTType::VariableDeclarationStatement
                | ASTType::Parameter
                | ASTType::ConstructorOrFunctionDefinition
                | ASTType::EnumValue
                | ASTType::EnumDefinition
                | ASTType::StructDefinition
                | ASTType::StateVariableDeclaration
                | ASTType::ContractDefinition
                | ASTType::SourceUnit
        ) || matches!(ast, AST::Comment(_))
            || matches!(ast, AST::Identifier(_))
            || matches!(ast, AST::Expression(Expression::FunctionCallExpr(_)))
            || matches!(
                ast,
                AST::Expression(Expression::LiteralExpr(LiteralExpr::ArrayLiteralExpr(_)))
            )
            || matches!(
                ast,
                AST::Statement(Statement::SimpleStatement(
                    SimpleStatement::AssignmentStatement(_)
                ))
            )
            || matches!(ast, AST::Statement(Statement::CircuitDirectiveStatement(_)))
            || matches!(ast, AST::Statement(Statement::StatementList(_)))
            || matches!(ast, AST::TypeName(TypeName::ElementaryTypeName(_)))
            || matches!(ast, AST::TypeName(TypeName::UserDefinedTypeName(_)))
            || matches!(ast, AST::TypeName(TypeName::Array(_)))
    }
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> eyre::Result<Self::Return> {
        // println!("====get_attr======code========={:?}",name);
        match name {
            ASTType::ASTBase => self.visit_AST(ast),
            ASTType::PrimitiveCastExpr => self.visit_PrimitiveCastExpr(ast),
            ASTType::BooleanLiteralExpr => self.visit_BooleanLiteralExpr(ast),
            ASTType::NumberLiteralExpr => self.visit_NumberLiteralExpr(ast),
            ASTType::StringLiteralExpr => self.visit_StringLiteralExpr(ast),
            ASTType::TupleExpr => self.visit_TupleExpr(ast),
            ASTType::IdentifierExpr => self.visit_IdentifierExpr(ast),
            ASTType::MemberAccessExpr => self.visit_MemberAccessExpr(ast),
            ASTType::IndexExpr => self.visit_IndexExpr(ast),
            ASTType::MeExpr => self.visit_MeExpr(ast),
            ASTType::AllExpr => self.visit_AllExpr(ast),
            ASTType::ReclassifyExprBase => self.visit_ReclassifyExpr(ast),
            ASTType::RehomExpr => self.visit_RehomExpr(ast),
            ASTType::IfStatement => self.visit_IfStatement(ast),
            ASTType::WhileStatement => self.visit_WhileStatement(ast),
            ASTType::DoWhileStatement => self.visit_DoWhileStatement(ast),
            ASTType::ForStatement => self.visit_ForStatement(ast),
            ASTType::BreakStatement => self.visit_BreakStatement(ast),
            ASTType::ContinueStatement => self.visit_ContinueStatement(ast),
            ASTType::ReturnStatement => self.visit_ReturnStatement(ast),
            ASTType::ExpressionStatement => self.visit_ExpressionStatement(ast),
            ASTType::RequireStatement => self.visit_RequireStatement(ast),
            ASTType::Block => self.visit_Block(ast),
            ASTType::IndentBlock => self.visit_IndentBlock(ast),
            ASTType::AddressTypeName => self.visit_AddressTypeName(ast),
            ASTType::AddressPayableTypeName => self.visit_AddressPayableTypeName(ast),
            ASTType::AnnotatedTypeName => self.visit_AnnotatedTypeName(ast),
            ASTType::Mapping => self.visit_Mapping(ast),
            ASTType::CipherText => self.visit_CipherText(ast),
            ASTType::TupleType => self.visit_TupleType(ast),
            ASTType::VariableDeclaration => self.visit_VariableDeclaration(ast),
            ASTType::VariableDeclarationStatement => self.visit_VariableDeclarationStatement(ast),
            ASTType::Parameter => self.visit_Parameter(ast),
            ASTType::ConstructorOrFunctionDefinition => {
                self.visit_ConstructorOrFunctionDefinition(ast)
            }
            ASTType::EnumValue => self.visit_EnumValue(ast),
            ASTType::EnumDefinition => self.visit_EnumDefinition(ast),
            ASTType::StructDefinition => self.visit_StructDefinition(ast),
            ASTType::StateVariableDeclaration => self.visit_StateVariableDeclaration(ast),
            ASTType::ContractDefinition => self.visit_ContractDefinition(ast),
            ASTType::SourceUnit => self.visit_SourceUnit(ast),
            _ if matches!(ast.to_ast(), AST::TypeName(TypeName::ElementaryTypeName(_))) => {
                self.visit_ElementaryTypeName(ast)
            }
            _ if matches!(
                ast.to_ast(),
                AST::TypeName(TypeName::UserDefinedTypeName(_))
            ) =>
            {
                self.visit_UserDefinedTypeName(ast)
            }
            _ if matches!(ast.to_ast(), AST::TypeName(TypeName::Array(_))) => self.visit_Array(ast),

            _ if matches!(
                ast.to_ast(),
                AST::Expression(Expression::LiteralExpr(LiteralExpr::ArrayLiteralExpr(_)))
            ) =>
            {
                self.visit_ArrayLiteralExpr(ast)
            }
            _ if matches!(ast.to_ast(), AST::Comment(_)) => self.visit_Comment(ast),
            _ if matches!(ast.to_ast(), AST::Identifier(_)) => self.visit_Identifier(ast),
            _ if matches!(
                ast.to_ast(),
                AST::Expression(Expression::FunctionCallExpr(_))
            ) =>
            {
                self.visit_FunctionCallExpr(ast)
            }
            _ if matches!(
                ast.to_ast(),
                AST::Statement(Statement::SimpleStatement(
                    SimpleStatement::AssignmentStatement(_)
                ))
            ) =>
            {
                self.visit_AssignmentStatement(ast)
            }
            _ if matches!(
                ast.to_ast(),
                AST::Statement(Statement::CircuitDirectiveStatement(_))
            ) =>
            {
                self.visit_CircuitDirectiveStatement(ast)
            }
            _ if matches!(ast.to_ast(), AST::Statement(Statement::StatementList(_))) => {
                self.visit_StatementList(ast)
            }
            _ => panic!(""), //Ok(String::new()),
        }
    }
}
impl CodeVisitor for SolidityVisitor {
    fn visit_AnnotatedTypeName(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("===visit_AnnotatedTypeName============{:?}=",ast.get_ast_type());
        let t = ast
            .to_ast()
            .try_as_annotated_type_name_ref()
            .unwrap()
            .type_name
            .as_ref()
            .map_or(String::new(), |type_name| {
                // print!(
                //     "=(==visit_AnnotatedTypeName==#======{:?}=====",
                //     type_name.get_ast_type()
                // );
                self.visit(&type_name.clone().into())
            });
        // print!(
        //                     "=(==visit_AnnotatedTypeName==#======{:?}=====",
        //                     ast
        //             .to_ast()
        //             .try_as_annotated_type_name_ref()
        //             .unwrap()
        //             .type_name.as_ref().unwrap().get_ast_type()
        //                 );
        //  println!("=*==visit_AnnotatedTypeName============{t}======)");
        let p = ast
            .to_ast()
            .try_as_annotated_type_name_ref()
            .unwrap()
            .privacy_annotation
            .as_ref()
            .map_or(String::new(), |privacy_annotation| {
                self.visit(privacy_annotation)
            });

        Ok(
            if ast
                .to_ast()
                .try_as_annotated_type_name_ref()
                .unwrap()
                .had_privacy_annotation
            {
                format!(
                    "{t}@{p}{}",
                    HOMOMORPHISM_STORE
                        .lock()
                        .unwrap()
                        .get(
                            &ast.to_ast()
                                .try_as_annotated_type_name_ref()
                                .unwrap()
                                .homomorphism
                        )
                        .unwrap()
                )
            } else {
                t
            },
        )
    }
    // fn visit_AnnotatedTypeName(
    //     &self,
    //     ast: &ASTFlatten,
    // ) -> eyre::Result<<Self as AstVisitor>::Return> {
    //     //only display data type, not privacy annotation
    //     // println!(
    //     //     "====visit_AnnotatedTypeName========================={:?}",
    //     //     ast.to_ast()
    //     //         .try_as_annotated_type_name_ref()
    //     //         .unwrap()
    //     //         .type_name
    //     //         .as_ref()
    //     //         .unwrap()
    //     //         .get_ast_type()
    //     // );
    //     let res = self.visit(
    //         &ast.to_ast()
    //             .try_as_annotated_type_name_ref()
    //             .unwrap()
    //             .type_name
    //             .clone()
    //             .unwrap()
    //             .into(),
    //     );
    //     // println!("===visit_AnnotatedTypeName===res===={res}=========");
    //     Ok(res)
    // }

    fn visit_MeExpr(&self, _: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("===visit_MeExpr===SOL========");
        Ok(String::from("msg.sender"))
    }

    fn handle_pragma(&self, _pragma: String) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(format!(
            "pragma solidity {};",
            CFG.lock().unwrap().zkay_solc_version_compatibility()
        ))
    }
}

impl SolidityVisitor {
    fn visit_list(
        &self,
        l: Vec<ListUnion>,
        mut sep: &str,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        if sep.is_empty() {
            sep = "\n";
        }
        if l.is_empty() {
            return Ok(self.temper_result());
        }

        let s: Vec<_> = l.iter().filter_map(|e| self.handle(e).ok()).collect();
        let s = s.join(sep);
        Ok(s)
    }
    fn handle(&self, e: &ListUnion) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(match e {
            ListUnion::String(e) => e.to_owned(),
            ListUnion::AST(e) => self.visit(e),
        })
    }
    fn visit_SourceUnit(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        let p = self.handle_pragma(
            ast.try_as_source_unit_ref()
                .unwrap()
                .borrow()
                .pragma_directive
                .clone(),
        )?;
        let contracts = self.visit_list(
            ast.try_as_source_unit_ref()
                .unwrap()
                .borrow()
                .contracts
                .iter()
                .map(|contract| ListUnion::AST(contract.clone().into()))
                .collect(),
            "",
        )?;
        let lfstr = |uc| format!("import \"{}\";", uc);
        //  "\n\n".join(filter("".__ne__, [p, linesep.join([lfstr.format(uc) for uc in ast.used_contracts]), contracts]))
        Ok([
            p,
            ast.try_as_source_unit_ref()
                .unwrap()
                .borrow()
                .used_contracts
                .iter()
                .map(lfstr)
                .collect::<Vec<_>>()
                .join(LINE_ENDING),
            contracts,
        ]
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("\n\n"))
    }

    pub fn visit_single_or_list(
        &self,
        v: SingleOrListUnion,
        mut sep: &str,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        if sep.is_empty() {
            sep = "\n";
        }
        match v {
            SingleOrListUnion::Vec(v) => self.visit_list(v, sep),
            SingleOrListUnion::String(v) => Ok(v),
            SingleOrListUnion::AST(v) => Ok(self.visit(&v)),
        }
    }

    pub fn visit_AST(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        // should never be called
        // raise NotImplementedError("Did not implement code generation for " + repr(ast))
        // unimplemented!("Did not implement code generation for {:?} ", ast);
        // //println!("=======visit_AST==============");

        Err(eyre!("Did not implement code generation for {:?} ", ast))
    }
    pub fn visit_Comment(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        let text = ast.to_ast().try_as_comment_ref().unwrap().text().clone();
        // println!("==visit_Comment=============={text}=======");
        // if text=="Verify zk proof of execution"{
        // panic!("Verify zk proof of execution");
        // }
        Ok(if text.is_empty() {
            text
        } else if text.contains('\n') {
            format!("/* {} */", text)
        } else {
            format!("// {}", text)
        })
    }

    pub fn visit_Identifier(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        // print!("==visit_Identifier====================={}",ast.to_ast().try_as_identifier_ref().unwrap().name().clone());
        Ok(ast.to_ast().try_as_identifier_ref().unwrap().name().clone())
    }

    pub fn visit_FunctionCallExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(
            if is_instance(
                ast.to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .func(),
                ASTType::BuiltinFunction,
            ) {
                let args: Vec<_> = ast
                    .to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .args()
                    .iter()
                    .map(|a| self.visit(a))
                    .collect();
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
                    .format_string(&args)
            } else {
                let f = self.visit(
                    ast.to_ast()
                        .try_as_expression_ref()
                        .unwrap()
                        .try_as_function_call_expr_ref()
                        .unwrap()
                        .func(),
                );
                let a = self.visit_list(
                    ast.to_ast()
                        .try_as_expression_ref()
                        .unwrap()
                        .try_as_function_call_expr_ref()
                        .unwrap()
                        .args()
                        .iter()
                        .map(|arg| ListUnion::AST(arg.clone()))
                        .collect(),
                    ", ",
                )?;
                format!("{f}({a})")
            },
        )
    }

    pub fn visit_PrimitiveCastExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("==visit_PrimitiveCastExpr==={:?}=====",ast);
        Ok(
            if ast
                .to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_primitive_cast_expr_ref()
                .unwrap()
                .is_implicit
            {
                self.visit(
                    &ast.to_ast()
                        .try_as_expression_ref()
                        .unwrap()
                        .try_as_primitive_cast_expr_ref()
                        .unwrap()
                        .expr,
                )
            } else {
                format!(
                    "{}({})",
                    self.visit(
                        &ast.to_ast()
                            .try_as_expression_ref()
                            .unwrap()
                            .try_as_primitive_cast_expr_ref()
                            .unwrap()
                            .elem_type
                            .clone()
                            .into()
                    ),
                    self.visit(
                        &ast.to_ast()
                            .try_as_expression_ref()
                            .unwrap()
                            .try_as_primitive_cast_expr_ref()
                            .unwrap()
                            .expr
                    )
                )
            },
        )
    }

    pub fn visit_BooleanLiteralExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("===visit_BooleanLiteralExpr=========={:?}====",ast);
        Ok(ast
            .to_ast()
            .try_as_expression_ref()
            .unwrap()
            .try_as_literal_expr_ref()
            .unwrap()
            .try_as_boolean_literal_expr_ref()
            .unwrap()
            .value
            .to_string()
            .to_ascii_lowercase())
    }

    pub fn visit_NumberLiteralExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // //println!("======visit_NumberLiteralExpr==============={:?}",ast);
        Ok(
            if ast
                .to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_literal_expr_ref()
                .unwrap()
                .try_as_number_literal_expr_ref()
                .unwrap()
                .was_hex
            {
                format!(
                    "{:x}",
                    ast.to_ast()
                        .try_as_expression_ref()
                        .unwrap()
                        .try_as_literal_expr_ref()
                        .unwrap()
                        .try_as_number_literal_expr_ref()
                        .unwrap()
                        .value
                )
            } else {
                ast.to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_literal_expr_ref()
                    .unwrap()
                    .try_as_number_literal_expr_ref()
                    .unwrap()
                    .value_string
                    .clone()
                    .unwrap_or(
                        ast.to_ast()
                            .try_as_expression_ref()
                            .unwrap()
                            .try_as_literal_expr_ref()
                            .unwrap()
                            .try_as_number_literal_expr_ref()
                            .unwrap()
                            .value
                            .to_string(),
                    )
            },
        )
    }

    pub fn visit_StringLiteralExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(format!(
            "\"{}\"",
            ast.try_as_string_literal_expr_ref().unwrap().borrow().value
        ))
    }

    pub fn visit_ArrayLiteralExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("===visit_ArrayLiteralExpr===================={ast:?}");
        Ok(format!(
            "[{}]",
            self.visit_list(
                ast.to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_literal_expr_ref()
                    .unwrap()
                    .try_as_array_literal_expr_ref()
                    .unwrap()
                    .values()
                    .iter()
                    .map(|value| ListUnion::AST(value.clone()))
                    .collect(),
                ", "
            )?
        ))
    }

    pub fn visit_TupleExpr(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(format!(
            "({})",
            self.visit_list(
                ast.to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_tuple_or_location_expr_ref()
                    .unwrap()
                    .try_as_tuple_expr_ref()
                    .unwrap()
                    .elements
                    .iter()
                    .map(|element| ListUnion::AST(element.clone()))
                    .collect(),
                ", "
            )?
        ))
    }

    pub fn visit_IdentifierExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // //println!("======visit_IdentifierExpr========={:?}", ast);
        Ok(self.visit(
            &ast.ast_base_ref()
                .unwrap()
                .borrow()
                .idf()
                .as_ref()
                .unwrap()
                .clone()
                .into(),
        ))
    }

    pub fn visit_MemberAccessExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("==visit_MemberAccessExpr=code======{:?}",ast);
        let mae = ast
            .to_ast()
            .try_as_expression_ref()
            .unwrap()
            .try_as_tuple_or_location_expr_ref()
            .unwrap()
            .try_as_location_expr_ref()
            .unwrap()
            .try_as_member_access_expr_ref()
            .unwrap()
            .clone();
        Ok(format!(
            "{}.{}",
            self.visit(&mae.expr.as_ref().unwrap().clone().into()),
            self.visit(&mae.member.clone().into())
        ))
    }

    pub fn visit_IndexExpr(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        // //println!("=======visit_IndexExpr================={:?}",ast);
        Ok(format!(
            "{}[{}]",
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
                    .arr
                    .as_ref()
                    .unwrap()
                    .clone()
                    .into()
            ),
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
            )
        ))
    }

    pub fn visit_AllExpr(&self, _: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(String::from("all"))
    }

    pub fn visit_ReclassifyExpr(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let e = self.visit(
            ast.to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_reclassify_expr_ref()
                .unwrap()
                .expr(),
        );
        let p = self.visit(
            ast.to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_reclassify_expr_ref()
                .unwrap()
                .privacy(),
        );
        // println!(
        //     "===visit_ReclassifyExpr==============={:?}",
        //     ast.to_ast()
        //         .try_as_expression_ref()
        //         .unwrap()
        //         .try_as_reclassify_expr_ref()
        //         .unwrap()
        //         .homomorphism()
        // );
        let h = ast
            .to_ast()
            .try_as_expression_ref()
            .unwrap()
            .try_as_reclassify_expr_ref()
            .unwrap()
            .homomorphism()
            .as_ref()
            .map_or(String::new(), |ho| {
                HOMOMORPHISM_STORE
                    .lock()
                    .unwrap()
                    .get(ho)
                    .map_or(String::new(), |h| h.to_string())
            });
        // println!("reveal{}({e}, {p})", h);
        Ok(format!("reveal{}({e}, {p})", h))
    }

    pub fn visit_RehomExpr(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        let e = self.visit(
            &ast.to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_reclassify_expr_ref()
                .unwrap()
                .try_as_rehom_expr_ref()
                .unwrap()
                .reclassify_expr_base
                .expr,
        );
        Ok(format!(
            "{}({e})",
            ast.to_ast()
                .try_as_expression_ref()
                .unwrap()
                .try_as_reclassify_expr_ref()
                .unwrap()
                .try_as_rehom_expr_ref()
                .unwrap()
                .func_name()
        ))
    }

    pub fn visit_IfStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("===visit_IfStatement=========={:?}",ast);
        let c = self.visit(
            &ast.to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_if_statement_ref()
                .unwrap()
                .condition,
        );
        let t = self.visit_single_or_list(
            SingleOrListUnion::AST(
                ast.to_ast()
                    .try_as_statement_ref()
                    .unwrap()
                    .try_as_if_statement_ref()
                    .unwrap()
                    .then_branch
                    .clone()
                    .into(),
            ),
            "",
        )?;
        //  println!("==sol==if stmt===={t}==={:?}=",ast.to_ast()
        //             .try_as_statement_ref()
        //             .unwrap()
        //             .try_as_if_statement_ref()
        //             .unwrap()
        //             .then_branch.get_ast_type());
        let mut ret = format!("if ({c}) {t}");
        if let Some(else_branch) = &ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_if_statement_ref()
            .unwrap()
            .else_branch
        {
            let e =
                self.visit_single_or_list(SingleOrListUnion::AST(else_branch.clone().into()), "")?;
            ret += format!("\n else {e}").as_str();
        }
        Ok(ret)
    }

    pub fn visit_WhileStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("===visit_WhileStatement======={:?}===",ast);
        let c = self.visit(
            &ast.to_ast()
                .try_as_statement()
                .unwrap()
                .try_as_while_statement_ref()
                .unwrap()
                .condition,
        );
        let b = self.visit_single_or_list(
            SingleOrListUnion::AST(
                ast.to_ast()
                    .try_as_statement()
                    .unwrap()
                    .try_as_while_statement_ref()
                    .unwrap()
                    .body
                    .clone()
                    .into(),
            ),
            "",
        )?;
        Ok(format!("while ({c}) {b}"))
    }

    pub fn visit_DoWhileStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let b = self.visit_single_or_list(
            SingleOrListUnion::AST(
                ast.to_ast()
                    .try_as_statement()
                    .unwrap()
                    .try_as_do_while_statement_ref()
                    .unwrap()
                    .body
                    .clone()
                    .into(),
            ),
            "",
        )?;
        let c = self.visit(
            &ast.to_ast()
                .try_as_statement()
                .unwrap()
                .try_as_do_while_statement_ref()
                .unwrap()
                .condition,
        );
        Ok(format!("do {b} while ({c});"))
    }

    pub fn visit_ForStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("==visit_ForStatement========{:?}======",ast);
        let for_statement = ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_for_statement_ref()
            .unwrap()
            .clone();
        let i = if let Some(init) = &for_statement.init {
            self.visit_single_or_list(SingleOrListUnion::AST(init.clone().into()), "")?
        } else {
            String::from(";")
        };
        let c = self.visit(&for_statement.condition);
        let u = for_statement
            .update
            .as_ref()
            .map_or(String::new(), |update| {
                // println!("===update=====code========={:?}",update.borrow().to_string());
                format!(
                    " {}",
                    self.visit_single_or_list(SingleOrListUnion::AST(update.clone().into()), "")
                        .unwrap_or_default()
                        .replace(';', "")
                )
            });

        let b = self.visit_single_or_list(
            SingleOrListUnion::AST(for_statement.body.clone().into()),
            "",
        )?;
        Ok(format!("for ({i} {c};{u}) {b}"))
    }

    pub fn visit_BreakStatement(
        &self,
        _: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(String::from("break;"))
    }

    pub fn visit_ContinueStatement(
        &self,
        _: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(String::from("continue;"))
    }

    pub fn visit_ReturnStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("======visit_ReturnStatement==========={:?}",ast);
        Ok(
            if ast
                .to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_return_statement_ref()
                .unwrap()
                .expr
                .is_none()
            {
                String::from("return;")
            } else {
                let e = self.visit(
                    ast.to_ast()
                        .try_as_statement_ref()
                        .unwrap()
                        .try_as_return_statement_ref()
                        .unwrap()
                        .expr
                        .as_ref()
                        .unwrap(),
                );
                format!("return {e};")
            },
        )
    }

    pub fn visit_ExpressionStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        //  println!("===visit_ExpressionStatement=========={:?}",ast);
        Ok(self.visit(
            &ast.to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_simple_statement_ref()
                .unwrap()
                .try_as_expression_statement_ref()
                .unwrap()
                .expr,
        ) + ";")
    }

    pub fn visit_RequireStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // //println!("====visit_RequireStatement=========={:?}", ast);
        let c = self.visit(
            &ast.to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_simple_statement_ref()
                .unwrap()
                .try_as_require_statement_ref()
                .unwrap()
                .condition,
        );
        Ok(format!("require({c});"))
    }

    pub fn visit_AssignmentStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("====visit_AssignmentStatement=========={:?}=====", ast.is_ast());
        let ast = ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_simple_statement_ref()
            .unwrap()
            .try_as_assignment_statement_ref()
            .unwrap()
            .clone();
        let lhs = ast.lhs().clone();
        let mut op = ast.op().clone();
        if lhs
            .as_ref()
            .unwrap()
            .to_ast()
            .try_as_expression_ref()
            .as_ref()
            .map_or(false, |asu| {
                asu.annotated_type()
                    .as_ref()
                    .map_or(false, |at| at.borrow().is_private())
            })
        {
            op = String::new();
        }

        let rhs = if !op.is_empty() {
            ast.rhs().clone().map(|fce| {
                // println!("=====fce==========={:?}=====",fce);
                fce.to_ast()
                    .try_as_expression_ref()
                    .unwrap()
                    .try_as_function_call_expr_ref()
                    .unwrap()
                    .args()[1]
                    .clone()
            })
        } else {
            ast.rhs().clone()
        };
        // println!("=visit_AssignmentStatement======op=={}===={:?}======={:?}",op,lhs.as_ref().unwrap().get_ast_type(),rhs.as_ref().unwrap().get_ast_type());
        let fstr = if op.starts_with("pre") {
            op = op[3..].to_string();
            "{1}{0};"
        } else if op.starts_with("post") {
            op = op[4..].to_string();
            "{0}{1};"
        } else {
            "{} {}= {};"
        };

        let format_string = |ls, rs| match fstr {
            "{1}{0};" => format!("{op}{ls};"),
            "{0}{1};" => format!("{ls}{op};"),
            _ => format!("{ls} {op}= {rs};"),
        };
        Ok(
            if is_instance(lhs.as_ref().unwrap(), ASTType::SliceExpr)
                && is_instance(rhs.as_ref().unwrap(), ASTType::SliceExpr)
            {
                let (lhs, rhs) = (
                    lhs.as_ref()
                        .unwrap()
                        .to_ast()
                        .try_as_expression_ref()
                        .unwrap()
                        .try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .try_as_location_expr_ref()
                        .unwrap()
                        .try_as_slice_expr_ref()
                        .unwrap()
                        .clone(),
                    rhs.as_ref()
                        .unwrap()
                        .to_ast()
                        .try_as_expression_ref()
                        .unwrap()
                        .try_as_tuple_or_location_expr_ref()
                        .unwrap()
                        .try_as_location_expr_ref()
                        .unwrap()
                        .try_as_slice_expr_ref()
                        .unwrap()
                        .clone(),
                );
                assert!(lhs.size == rhs.size, "Slice ranges don't have same size");
                let mut s = String::new();
                let (lexpr, rexpr) = (
                    self.visit(&lhs.arr.as_ref().unwrap().clone().into()),
                    self.visit(&rhs.arr.as_ref().unwrap().clone().into()),
                );
                let mut lbase = if let Some(base) = &lhs.base {
                    format!("{} + ", self.visit(base))
                } else {
                    String::new()
                };
                let mut rbase = if let Some(base) = &rhs.base {
                    format!("{} + ", self.visit(base))
                } else {
                    String::new()
                };
                if lhs.size <= 3 {
                    //   println!("=visit_AssignmentStatement====44444==op={:?}====fstr========{:?}",op,fstr);
                    for i in 0..lhs.size {
                        s += &format_string(
                            format!("{lexpr}[{lbase}{}]", lhs.start_offset + i),
                            format!("{rexpr}[{rbase}{}]", rhs.start_offset + i),
                        );
                        s += "\n";
                    }
                } else {
                    // println!("=======+++++===============");
                    let i = CFG.lock().unwrap().reserved_name_prefix() + "i";
                    if lhs.start_offset != 0 {
                        lbase += &format!("{} + ", lhs.start_offset);
                    }
                    if rhs.start_offset != 0 {
                        rbase += &format!("{} + ", rhs.start_offset);
                    }
                    s += format!("for (uint {i} = 0; {i} < {}; ++{i}) {{\n", lhs.size).as_str();
                    s += &indent(format_string(
                        format!("{lexpr}[{lbase}{i}]"),
                        format!("{rexpr}[{rbase}{i}]"),
                    ));
                    s += "\n";
                    s += "}\n";
                }
                s[..s.len() - 1].to_string()
            } else {
                //   println!("=visit_AssignmentStatement===type===op={:?}============",rhs.as_ref().unwrap().get_ast_type);
                format_string(
                    self.visit(lhs.as_ref().unwrap()),
                    self.visit(rhs.as_ref().unwrap()),
                )
            },
        )
    }
    pub fn visit_CircuitDirectiveStatement(
        &self,
        _ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(String::new())
    }

    pub fn handle_block(&self, ast: &StatementList) -> eyre::Result<<Self as AstVisitor>::Return> {
        self.visit_list(
            ast.statements()
                .iter()
                .map(|statement| ListUnion::AST(statement.clone_inner()))
                .collect(),
            "",
        )
        .map(|_s| indent(_s))
    }

    pub fn visit_StatementList(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        self.visit_list(
            ast.to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_statement_list_ref()
                .unwrap()
                .statements()
                .iter()
                .map(|statement| ListUnion::AST(statement.clone_inner()))
                .collect(),
            "",
        )
    }

    pub fn visit_Block(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("==visit_Block=========={}=====", ast
        //             .try_as_block_ref()
        //             .unwrap()
        //             .borrow()
        //             .statement_list_base
        //             .statements
        //             .len());
        let b = self
            .handle_block(&StatementList::Block(
                ast.try_as_block_ref().unwrap().borrow().clone(),
            ))?
            .trim_end()
            .to_string();

        Ok(
            if ast
                .try_as_block_ref()
                .unwrap()
                .borrow()
                .was_single_statement
                && ast
                    .try_as_block_ref()
                    .unwrap()
                    .borrow()
                    .statement_list_base
                    .statements
                    .len()
                    == 1
            {
                b
            } else {
                format!("{{\n{b}\n}}")
            },
        )
    }

    pub fn visit_IndentBlock(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        self.handle_block(
            ast.try_as_indent_block_ref()
                .unwrap()
                .borrow()
                .to_statement()
                .try_as_statement_list_ref()
                .unwrap(),
        )
    }

    pub fn visit_ElementaryTypeName(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(ast
            .to_ast()
            .try_as_type_name_ref()
            .unwrap()
            .try_as_elementary_type_name_ref()
            .unwrap()
            .name()
            .clone())
    }

    pub fn visit_UserDefinedTypeName(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let names: Vec<_> = ast
            .to_ast()
            .try_as_type_name_ref()
            .unwrap()
            .try_as_user_defined_type_name_ref()
            .unwrap()
            .user_defined_type_name_base_ref()
            .names
            .iter()
            .map(|name| ListUnion::AST(name.clone().into()))
            .collect();
        self.visit_list(names, ".")
    }

    pub fn visit_AddressTypeName(
        &self,
        _ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(String::from("address"))
    }

    pub fn visit_AddressPayableTypeName(
        &self,
        _ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(String::from("address payable"))
    }

    pub fn visit_Mapping(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("=========visit_Mapping=======begin==========");
        let k = self.visit(
            &ast.to_ast()
                .try_as_type_name_ref()
                .unwrap()
                .try_as_mapping_ref()
                .unwrap()
                .key_type
                .clone()
                .into(),
        );
        let label = if let Some(idf) = &ast
            .to_ast()
            .try_as_type_name_ref()
            .unwrap()
            .try_as_mapping_ref()
            .unwrap()
            .key_label
        {
            if idf.borrow().is_identifier()
                && idf.borrow().try_as_identifier_ref().unwrap().is_string
            {
                format!(
                    "/*!{}*/",
                    idf.borrow().try_as_identifier_ref().unwrap().name
                )
            } else {
                format!("!{}", self.visit(&idf.clone().into()))
            }
        } else {
            String::new()
        };
        // println!("=========visit_Mapping=======value_type=====begin=====");
        let v = self.visit(
            &ast.to_ast()
                .try_as_type_name_ref()
                .unwrap()
                .try_as_mapping_ref()
                .unwrap()
                .value_type
                .clone()
                .into(),
        );
        // println!("=========visit_Mapping====value_type============={v}====");
        Ok(format!("mapping({k}{label} => {v})"))
    }

    pub fn visit_Array(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("===visit_Array====================={ast}");
        let value_type = ast
            .to_ast()
            .try_as_type_name_ref()
            .unwrap()
            .try_as_array_ref()
            .unwrap()
            .value_type()
            .clone();
        // println!(
        //     "===visit_Array=====value_type================{:?}",
        //     value_type.get_ast_type()
        // );

        let t = self.visit(&value_type.clone().into());
        // print!("====visit_Array===t======={}", t);
        let expr = ast
            .to_ast()
            .try_as_type_name_ref()
            .unwrap()
            .try_as_array_ref()
            .unwrap()
            .expr()
            .clone();

        let e = expr.as_ref().map_or(String::new(), |_expr| {
            // println!(
            //     "===visit_Array=====expr================{:?}",
            //     _expr.get_ast_type()
            // );
            self.visit(_expr)
        });
        Ok(format!("{t}[{e}]"))
    }

    pub fn visit_CipherText(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!(
        //     "===visit_CipherText================={}",
        //     ast.to_ast()
        //         .try_as_type_name_ref()
        //         .unwrap()
        //         .try_as_array_ref()
        //         .unwrap()
        //         .try_as_cipher_text_ref()
        //         .unwrap()
        //         .plain_type
        //         .as_ref()
        //         .unwrap()
        //         .borrow()
        //         .to_ast()
        //         .code()
        // );
        let e = self.visit_Array(ast)?;
        // println!("==visit_CipherText=====e==={e}=======");
        // println!("===visit_CipherText======e==========={}===={:?}===", ast.to_ast()
        //         .try_as_type_name_ref()
        //         .unwrap()
        //         .try_as_array_ref()
        //         .unwrap()
        //         .try_as_cipher_text_ref()
        //         .unwrap()
        //         .plain_type
        //         .as_ref()
        //         .unwrap()
        //         .borrow()
        //         .to_ast()
        //         .code(),ast.to_ast()
        //         .try_as_type_name_ref()
        //         .unwrap()
        //         .try_as_array_ref()
        //         .unwrap()
        //         .try_as_cipher_text_ref()
        //         .unwrap()
        //         .plain_type
        //         .as_ref()
        //         .unwrap()
        //         .borrow().type_name.as_ref().unwrap().get_ast_type());
        let code = ast
            .to_ast()
            .try_as_type_name_ref()
            .unwrap()
            .try_as_array_ref()
            .unwrap()
            .try_as_cipher_text_ref()
            .unwrap()
            .plain_type
            .as_ref()
            .unwrap()
            .borrow()
            .to_ast()
            .code();
        // let pt = self.visit(
        //     &ast.to_ast()
        //         .try_as_type_name_ref()
        //         .unwrap()
        //         .try_as_array_ref()
        //         .unwrap()
        //         .try_as_cipher_text_ref()
        //         .unwrap()
        //         .plain_type
        //         .clone()
        //         .unwrap()
        //         .into(),
        // );
        Ok(format!("{e}/*{}*/", code))
    }

    pub fn visit_TupleType(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        let s = self.visit_list(
            ast.try_as_tuple_type_ref()
                .unwrap()
                .borrow()
                .types
                .iter()
                .map(|typ| ListUnion::AST(typ.clone().into()))
                .collect(),
            ", ",
        )?;
        Ok(format!("({s})"))
    }

    pub fn visit_VariableDeclaration(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // println!("=====self.display_final ========={}====",self.display_final );
        let keywords: Vec<_> = ast
            .try_as_variable_declaration_ref()
            .unwrap()
            .borrow()
            .identifier_declaration_base
            .keywords
            .iter()
            .filter(|&k| self.code_visitor_base.display_final || k != "final")
            .cloned()
            .collect();

        let k = keywords.join(" ");
        let t = self.visit(
            &ast.try_as_variable_declaration_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .clone()
                .unwrap()
                .into(),
        );

        let s = ast
            .try_as_variable_declaration_ref()
            .unwrap()
            .borrow()
            .identifier_declaration_base
            .storage_location
            .as_ref()
            .map_or(String::new(), |storage_location| {
                format!(" {storage_location}")
            });

        let i = self.visit(
            &ast.ast_base_ref()
                .unwrap()
                .borrow()
                .idf()
                .clone()
                .unwrap()
                .into(),
        );
        // let ii:ASTFlatten=ast.ast_base_ref()
        //                 .unwrap()
        //                 .borrow()
        //                 .idf().clone().unwrap().into();
        //   println!("==idf==type={i}==ii={}====",ii);
        // println!("=====visit_VariableDeclaration====={k} {t}{s} {i}==========k===={k},=====t=={t},===s=={s}, =i={i},");
        // let vd=format!("{k} {t}{s} {i}");
        // println!("====visit_VariableDeclaration============{vd}=====");
        Ok(format!("{k} {t}{s} {i}").trim().to_string())
    }

    pub fn visit_VariableDeclarationStatement(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let mut s = self.visit(
            &ast.to_ast()
                .try_as_statement_ref()
                .unwrap()
                .try_as_simple_statement_ref()
                .unwrap()
                .try_as_variable_declaration_statement_ref()
                .unwrap()
                .variable_declaration
                .clone()
                .into(),
        );
        if let Some(expr) = &ast
            .to_ast()
            .try_as_statement_ref()
            .unwrap()
            .try_as_simple_statement_ref()
            .unwrap()
            .try_as_variable_declaration_statement_ref()
            .unwrap()
            .expr
        {
            s += format!(" = {}", self.visit(expr)).as_str();
        }
        s += ";";
        // s=s.trim().to_owned();
        // println!("=========s========{s}========");
        Ok(s)
    }

    pub fn visit_Parameter(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        let final_string = String::from("final ");
        let f = if !self.code_visitor_base.display_final {
            None
        } else if ast
            .try_as_parameter_ref()
            .unwrap()
            .borrow()
            .identifier_declaration_base
            .keywords
            .contains(&"final".to_owned())
        {
            Some(final_string)
        } else {
            None
        };
        let t = self.visit(
            &ast.try_as_parameter_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .clone()
                .unwrap()
                .into(),
        );
        let i = ast
            .try_as_parameter_ref()
            .unwrap()
            .borrow()
            .idf()
            .as_ref()
            .map(|idf| self.visit(&idf.clone().into()));
        let description: Vec<_> = [
            f,
            Some(t),
            ast.try_as_parameter_ref()
                .unwrap()
                .borrow()
                .identifier_declaration_base
                .storage_location
                .clone(),
            i,
        ]
        .iter()
        .filter_map(|d| d.clone())
        .collect();
        // println!("=====description================{:?}=========",description);
        Ok(description.join(" "))
    }

    pub fn visit_ConstructorOrFunctionDefinition(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        //    self.code_visitor_base.ast_visitor_base.log=if ast.try_as_constructor_or_function_definition_ref()
        //             .unwrap()
        //             .borrow()
        //             .idf()
        //             .as_ref()
        //             .unwrap().borrow().name()=="vote"{
        //         true
        //     }else{false};
        let b = if let Some(body) = &ast
            .try_as_constructor_or_function_definition_ref()
            .unwrap()
            .borrow()
            .body
        {
            //  println!("===visit_ConstructorOrFunctionDefinition========={}============{:?}====",ast.try_as_constructor_or_function_definition_ref()
            //                 .unwrap()
            //                 .borrow()
            //                 .idf()
            //                 .as_ref()
            //                 .unwrap().borrow().name(),body.get_ast_type());
            self.visit_single_or_list(SingleOrListUnion::AST(body.clone().into()), "")?
        } else {
            String::new()
        };
        self.function_definition_to_str(
            ast.try_as_constructor_or_function_definition_ref()
                .unwrap()
                .borrow()
                .idf()
                .as_ref()
                .unwrap(),
            ast.try_as_constructor_or_function_definition_ref()
                .unwrap()
                .borrow()
                .parameters
                .iter()
                .map(|parameter| ParameterUnion::Parameter(parameter.clone()))
                .collect(),
            &ast.try_as_constructor_or_function_definition_ref()
                .unwrap()
                .borrow()
                .modifiers,
            &ast.try_as_constructor_or_function_definition_ref()
                .unwrap()
                .borrow()
                .return_parameters,
            &b,
        )
    }
    pub fn function_definition_to_str(
        &self,
        idf: &RcCell<Identifier>,
        parameters: Vec<ParameterUnion>,
        modifiers: &[String],
        return_parameters: &[RcCell<Parameter>],
        body: &String,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let definition = if idf.borrow().name() != "constructor" {
            let i = self.visit(&idf.clone().into());
            format!("function {i}")
        } else {
            String::from("constructor")
        };
        // //println!("{:?}", parameters);
        let p = self.visit_list(
            parameters
                .iter()
                .map(|parameter| match parameter {
                    ParameterUnion::Parameter(p) => ListUnion::AST(p.clone().into()),
                    ParameterUnion::String(s) => ListUnion::String(s.clone()),
                })
                .collect(),
            ", ",
        )?;

        let mut m = modifiers.join(" ");
        if !m.is_empty() {
            m = format!(" {m}");
        }
        let mut r = self.visit_list(
            return_parameters
                .iter()
                .map(|p| ListUnion::AST(p.clone().into()))
                .collect(),
            ", ",
        )?;
        if !r.is_empty() {
            r = format!(" returns ({r})");
        }

        Ok(format!("{definition}({p}){m}{r} {body}"))
    }

    pub fn visit_EnumValue(&self, ast: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
        Ok(
            if let Some(idf) = &ast.try_as_enum_value_ref().unwrap().borrow().idf() {
                // println!("==visit_EnumValue=========={}====",idf.borrow().name());
                self.visit(&idf.clone().into())
            } else {
                String::new()
            },
        )
    }

    pub fn visit_EnumDefinition(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let values = indent(
            self.visit_list(
                ast.try_as_enum_definition_ref()
                    .unwrap()
                    .borrow()
                    .values
                    .iter()
                    .map(|value| ListUnion::AST(value.clone().into()))
                    .collect(),
                ", ",
            )?,
        );
        Ok(format!(
            "enum {} {{\n{values}\n}}",
            self.visit(
                &ast.try_as_enum_definition_ref()
                    .unwrap()
                    .borrow()
                    .idf()
                    .as_ref()
                    .unwrap()
                    .clone()
                    .into()
            )
        ))
    }

    // @staticmethod
    pub fn __cmp_type_size(v1: &ASTFlatten, v2: &ASTFlatten) -> Ordering {
        match (
            v1.ast_base_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .type_name
                .clone(),
            v2.ast_base_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .as_ref()
                .unwrap()
                .borrow()
                .type_name
                .clone(),
        ) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            (Some(t1), Some(t2)) => {
                let cmp = t1
                    .to_ast()
                    .try_as_type_name()
                    .unwrap()
                    .size_in_uints()
                    .cmp(&t2.to_ast().try_as_type_name().unwrap().size_in_uints());
                if cmp == Ordering::Equal {
                    t1.to_ast()
                        .try_as_type_name()
                        .unwrap()
                        .elem_bitwidth()
                        .cmp(&t2.to_ast().try_as_type_name().unwrap().elem_bitwidth())
                } else {
                    cmp
                }
            }
        }
    }

    pub fn visit_StructDefinition(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        // Define struct with members in order of descending size (to get maximum space savings through packing)
        let mut members_by_descending_size = ast
            .try_as_struct_definition_ref()
            .unwrap()
            .borrow()
            .members
            .clone();
        members_by_descending_size.sort_by(|v1, v2| Self::__cmp_type_size(v1, v2).reverse());
        let body = indent(
            members_by_descending_size
                .iter()
                .map(|member| self.visit(member) + ";")
                .collect::<Vec<_>>()
                .join("\n"),
        );
        Ok(format!(
            "struct {} {{\n{body}\n}}",
            self.visit(
                &ast.try_as_struct_definition_ref()
                    .unwrap()
                    .borrow()
                    .idf()
                    .as_ref()
                    .unwrap()
                    .clone()
                    .into()
            )
        ))
    }

    pub fn visit_StateVariableDeclaration(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let final_string = String::from("final ");

        let keywords: Vec<_> = ast
            .try_as_state_variable_declaration_ref()
            .unwrap()
            .borrow()
            .identifier_declaration_base
            .keywords
            .iter()
            .cloned()
            .filter_map(|k| (self.code_visitor_base.display_final || k != "final").then_some(k))
            .collect();
        let f = if keywords.contains(&"final".to_owned()) {
            final_string.clone()
        } else {
            String::new()
        };
        let t = self.visit(
            &ast.try_as_state_variable_declaration_ref()
                .unwrap()
                .borrow()
                .annotated_type()
                .clone()
                .unwrap()
                .into(),
        );
        let mut k = ast
            .try_as_state_variable_declaration_ref()
            .unwrap()
            .borrow()
            .identifier_declaration_base
            .keywords
            .iter()
            .filter(|&k| k != "final")
            .cloned()
            .collect::<Vec<_>>()
            .join(" ");
        if !k.is_empty() {
            k = format!("{k} ");
        }
        // println!("==visit_StateVariableDeclaration================={f} =========={k}=======");

        let i = self.visit(
            &ast.try_as_state_variable_declaration_ref()
                .unwrap()
                .borrow()
                .idf()
                .as_ref()
                .unwrap()
                .clone()
                .into(),
        );
        let mut ret = format!("{f}{t} {k}{i}").trim().to_string();
        if let Some(expr) = &ast
            .try_as_state_variable_declaration_ref()
            .unwrap()
            .borrow()
            .expr
        {
            ret += &format!(" = {}", self.visit(expr));
        }
        Ok(ret + ";")
    }

    pub fn contract_definition_to_str(
        idf: Identifier,
        state_vars: Vec<String>,
        constructors: Vec<String>,
        functions: Vec<String>,
        enums: Vec<String>,
        structs: Vec<String>,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let i = idf.to_string(); //Self::new().visit(&RcCell::new(idf).into());
        let structs = structs.join("\n\n");
        let enums = enums.join("\n\n");
        let state_vars = state_vars.join("\n");
        let constructors = constructors.join("\n\n");
        let functions = functions.join("\n\n");
        let mut body = [structs, enums, state_vars, constructors, functions]
            .into_iter()
            .filter_map(|s| if !s.is_empty() { Some(s) } else { None })
            .collect::<Vec<_>>()
            .join("\n\n");
        body = indent(body);
        Ok(format!("contract {i} {{\n{body}\n}}"))
    }

    pub fn visit_ContractDefinition(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        let state_vars = ast
            .try_as_contract_definition_ref()
            .unwrap()
            .borrow()
            .state_variable_declarations
            .iter()
            .map(|e| self.visit(e))
            .collect::<Vec<_>>(); //[ for e in ast.state_variable_declarations]
        let constructors = ast
            .try_as_contract_definition_ref()
            .unwrap()
            .borrow()
            .constructor_definitions
            .iter()
            .map(|e| self.visit(&e.clone().into()))
            .collect::<Vec<_>>(); //[self.visit(e) for e in ast.constructor_definitions]
        let functions = ast
            .try_as_contract_definition_ref()
            .unwrap()
            .borrow()
            .function_definitions
            .iter()
            .map(|e| self.visit(&e.clone().into()))
            .collect::<Vec<_>>(); //[self.visit(e) for e in ast.function_definitions]
        let enums = ast
            .try_as_contract_definition_ref()
            .unwrap()
            .borrow()
            .enum_definitions
            .iter()
            .map(|e| self.visit(&e.clone().into()))
            .collect::<Vec<_>>(); //[self.visit(e) for e in ast.enum_definitions]
        let structs = ast
            .try_as_contract_definition_ref()
            .unwrap()
            .borrow()
            .struct_definitions
            .iter()
            .map(|e| self.visit(&e.clone().into()))
            .collect::<Vec<_>>(); //[self.visit(e) for e in ast.struct_definitions]

        Self::contract_definition_to_str(
            ast.try_as_contract_definition_ref()
                .unwrap()
                .borrow()
                .idf()
                .as_ref()
                .unwrap()
                .borrow()
                .clone(),
            state_vars,
            constructors,
            functions,
            enums,
            structs,
        )
    }
}
