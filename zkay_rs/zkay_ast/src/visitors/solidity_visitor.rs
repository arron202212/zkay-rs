#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

use crate::ast::{
    ASTFlatten, ASTInstanceOf, ASTType, AnnotatedTypeName, CodeVisitor, CodeVisitorBase,
    Expression, IntoAST, ListUnion, LiteralExpr, MeExpr, SimpleStatement, Statement, TypeName, AST,
    LINE_ENDING,
};
use crate::visitors::visitor::{AstVisitor, AstVisitorBase, AstVisitorBaseRef};
use eyre::{eyre, Result};
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
    fn has_attr(&self, ast: &AST) -> bool {
        // println!("======has_attr========{:?}======",ast.get_ast_type());
        matches!(
            ast.get_ast_type(),
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
            ASTType::ASTBase => self.code_visitor_base.visit_AST(ast),
            ASTType::PrimitiveCastExpr => self.code_visitor_base.visit_PrimitiveCastExpr(ast),
            ASTType::BooleanLiteralExpr => self.code_visitor_base.visit_BooleanLiteralExpr(ast),
            ASTType::NumberLiteralExpr => self.code_visitor_base.visit_NumberLiteralExpr(ast),
            ASTType::StringLiteralExpr => self.code_visitor_base.visit_StringLiteralExpr(ast),
            ASTType::TupleExpr => self.code_visitor_base.visit_TupleExpr(ast),
            ASTType::IdentifierExpr => self.code_visitor_base.visit_IdentifierExpr(ast),
            ASTType::MemberAccessExpr => self.code_visitor_base.visit_MemberAccessExpr(ast),
            ASTType::IndexExpr => self.code_visitor_base.visit_IndexExpr(ast),
            ASTType::MeExpr => self.visit_MeExpr(ast),
            ASTType::AllExpr => self.code_visitor_base.visit_AllExpr(ast),
            ASTType::ReclassifyExprBase => self.code_visitor_base.visit_ReclassifyExpr(ast),
            ASTType::RehomExpr => self.code_visitor_base.visit_RehomExpr(ast),
            ASTType::IfStatement => self.code_visitor_base.visit_IfStatement(ast),
            ASTType::WhileStatement => self.code_visitor_base.visit_WhileStatement(ast),
            ASTType::DoWhileStatement => self.code_visitor_base.visit_DoWhileStatement(ast),
            ASTType::ForStatement => self.code_visitor_base.visit_ForStatement(ast),
            ASTType::BreakStatement => self.code_visitor_base.visit_BreakStatement(ast),
            ASTType::ContinueStatement => self.code_visitor_base.visit_ContinueStatement(ast),
            ASTType::ReturnStatement => self.code_visitor_base.visit_ReturnStatement(ast),
            ASTType::ExpressionStatement => self.code_visitor_base.visit_ExpressionStatement(ast),
            ASTType::RequireStatement => self.code_visitor_base.visit_RequireStatement(ast),
            ASTType::Block => self.code_visitor_base.visit_Block(ast),
            ASTType::IndentBlock => self.code_visitor_base.visit_IndentBlock(ast),
            ASTType::AddressTypeName => self.code_visitor_base.visit_AddressTypeName(ast),
            ASTType::AddressPayableTypeName => {
                self.code_visitor_base.visit_AddressPayableTypeName(ast)
            }
            ASTType::AnnotatedTypeName => self.visit_AnnotatedTypeName(ast),
            ASTType::Mapping => self.code_visitor_base.visit_Mapping(ast),
            ASTType::CipherText => self.code_visitor_base.visit_CipherText(ast),
            ASTType::TupleType => self.code_visitor_base.visit_TupleType(ast),
            ASTType::VariableDeclaration => self.code_visitor_base.visit_VariableDeclaration(ast),
            ASTType::VariableDeclarationStatement => self
                .code_visitor_base
                .visit_VariableDeclarationStatement(ast),
            ASTType::Parameter => self.code_visitor_base.visit_Parameter(ast),
            ASTType::ConstructorOrFunctionDefinition => self
                .code_visitor_base
                .visit_ConstructorOrFunctionDefinition(ast),
            ASTType::EnumValue => self.code_visitor_base.visit_EnumValue(ast),
            ASTType::EnumDefinition => self.code_visitor_base.visit_EnumDefinition(ast),
            ASTType::StructDefinition => self.code_visitor_base.visit_StructDefinition(ast),
            ASTType::StateVariableDeclaration => {
                self.code_visitor_base.visit_StateVariableDeclaration(ast)
            }
            ASTType::ContractDefinition => self.code_visitor_base.visit_ContractDefinition(ast),
            ASTType::SourceUnit => self.visit_SourceUnit(ast),
            _ if matches!(ast.to_ast(), AST::TypeName(TypeName::ElementaryTypeName(_))) => {
                self.code_visitor_base.visit_ElementaryTypeName(ast)
            }
            _ if matches!(
                ast.to_ast(),
                AST::TypeName(TypeName::UserDefinedTypeName(_))
            ) =>
            {
                self.code_visitor_base.visit_UserDefinedTypeName(ast)
            }
            _ if matches!(ast.to_ast(), AST::TypeName(TypeName::Array(_))) => {
                self.code_visitor_base.visit_Array(ast)
            }

            _ if matches!(
                ast.to_ast(),
                AST::Expression(Expression::LiteralExpr(LiteralExpr::ArrayLiteralExpr(_)))
            ) =>
            {
                self.code_visitor_base.visit_ArrayLiteralExpr(ast)
            }
            _ if matches!(ast.to_ast(), AST::Comment(_)) => {
                self.code_visitor_base.visit_Comment(ast)
            }
            _ if matches!(ast.to_ast(), AST::Identifier(_)) => {
                self.code_visitor_base.visit_Identifier(ast)
            }
            _ if matches!(
                ast.to_ast(),
                AST::Expression(Expression::FunctionCallExpr(_))
            ) =>
            {
                self.code_visitor_base.visit_FunctionCallExpr(ast)
            }
            _ if matches!(
                ast.to_ast(),
                AST::Statement(Statement::SimpleStatement(
                    SimpleStatement::AssignmentStatement(_)
                ))
            ) =>
            {
                self.code_visitor_base.visit_AssignmentStatement(ast)
            }
            _ if matches!(
                ast.to_ast(),
                AST::Statement(Statement::CircuitDirectiveStatement(_))
            ) =>
            {
                self.code_visitor_base.visit_CircuitDirectiveStatement(ast)
            }
            _ if matches!(ast.to_ast(), AST::Statement(Statement::StatementList(_))) => {
                self.code_visitor_base.visit_StatementList(ast)
            }
            _ => Ok(String::new()),
        }
    }
}
impl CodeVisitor for SolidityVisitor {
    fn visit_AnnotatedTypeName(
        &self,
        ast: &ASTFlatten,
    ) -> eyre::Result<<Self as AstVisitor>::Return> {
        //only display data type, not privacy annotation
        Ok(self.code_visitor_base.visit(
            &ast.to_ast()
                .try_as_annotated_type_name_ref()
                .unwrap()
                .type_name
                .clone()
                .unwrap()
                .into(),
        ))
    }

    fn visit_MeExpr(&self, _: &ASTFlatten) -> eyre::Result<<Self as AstVisitor>::Return> {
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

        let s: Vec<_> = l.iter().map(|e| self.handle(e).unwrap()).collect();
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
}
