// use antlr_rust::token::{Token,CommonToken};
use antlr_rust::common_token_stream::CommonTokenStream;
// use  semantic_version::{NpmSpec, Version};
use crate::zkay_ast::ast::{
    self, AddressPayableTypeName, AddressTypeName, AnnotatedTypeName, AssignmentStatement,
    AssignmentStatementBase, AssignmentStatementUnion, Block, BoolTypeName, BooleanLiteralExpr,
    BuiltinFunction, ConstructorOrFunctionDefinition, ContractDefinition, DoWhileStatement,
    ElementaryTypeName, EnumDefinition, EnumValue, Expression, ExpressionStatement, ForStatement,
    FunctionCallExpr, Identifier, IdentifierBase, IdentifierDeclaration, IdentifierExpr,
    IfStatement, IndexExpr, IntTypeName, LiteralExpr, LocationExpr, NamespaceDefinition,
    NumberLiteralExpr, NumberTypeName, Parameter, ReclassifyExpr, ReclassifyExprBase, RehomExpr,
    RequireStatement, SimpleStatement, Statement, StatementList, StringLiteralExpr, TupleExpr,
    TupleOrLocationExpr, TypeName, UintTypeName, UserDefinedTypeName, WhileStatement, AST,FunctionCallExprBase,
};
// use antlr_rust::TokenSource;
// use  crate::config::cfg;
// use  crate::solidity_parser::parse::SyntaxException;
use crate::solidity_parser::{
    emit::Emitter,
    generated::{
        soliditylexer::SolidityLexer,
        solidityparser::{
            AndExprContext, AnnotatedTypeNameContext, AssignmentExprContext, BitShiftExprContext,
            BitwiseAndExprContext, BitwiseNotExprContext, BitwiseOrExprContext,
            BitwiseXorExprContext, BooleanLiteralExprContext, CompExprContext,
            ConstructorDefinitionContext, ContractDefinitionContext, ContractPartContextAttrs,
            DoWhileStatementContext, ElementaryTypeNameContext, EnumDefinitionContext,
            EnumValueContext, EqExprContext, ExpressionStatementContext, ForStatementContext,
            FunctionCallExprContext, FunctionDefinitionContext, HomomorphismAnnotationContext,
            IdentifierContext, IfStatementContext, IndexExprContext, IteExprContext,
            ModifierContext, MultDivModExprContext, NotExprContext, NumberLiteralExprContext,
            OrExprContext, ParenthesisExprContext, PlusMinusExprContext, PostCrementExprContext,
            PowExprContext, PragmaDirectiveContext, PragmaDirectiveContextAttrs,
            PreCrementExprContext, SignExprContext, SolidityParser, SolidityParserContextType,
            StringLiteralExprContext, TupleExprContext, VersionPragmaContext,
            WhileStatementContext,
        },
    },
};

use antlr_rust::tree::ParseTree;
// use  crate::solidity_parser::generated::solidityvisitor::SolidityVisitor;
// use crate::solidity_parser::parse::MyParser;
// use  crate::zkay_ast::ast::StateVariableDeclaration, ContractDefinition, NumberLiteralExpr,
//     BooleanLiteralExpr, FunctionCallExpr, ExpressionStatement, IdentifierExpr,
//     ReclassifyExpr, RehomExpr, BuiltinFunction, IndexExpr;
use crate::zkay_ast::homomorphism::{HOMOMORPHISM_STORE, REHOM_EXPRESSIONS};
// use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::input_stream::InputStream;
// use crate::solidity_parser::generated::solidityparser::{SourceUnitContextAll} ;
// use antlr_rust::parser_rule_context::ParserRuleContext;
use crate::solidity_parser::parse::MyErrorListener;
// use antlr_rust::tree::ParseTreeVisitor;
use antlr_rust::tree::Visitable;
#[macro_export]
macro_rules! _visit_binary_expr {
    ($ctx: expr,$self: expr) => {{
        let mut f = BuiltinFunction::new("");
        if let Some(op) = &($ctx).op {
            f = BuiltinFunction::new(&op.text);
            f.expression_base.ast_base.line = op.line as i32;
            f.expression_base.ast_base.column = op.column as i32;
        }
        let lhs = if let Some(expr) = &($ctx).lhs {
            expr.accept($self);
            if let ast::AST::Expression(expr) = ($self).temp_result().clone() {
                expr
            } else {
                Expression::None
            }
        } else {
            Expression::None
        };
        let rhs = if let Some(expr) = &($ctx).rhs {
            expr.accept($self);
            if let ast::AST::Expression(expr) = ($self).temp_result().clone() {
                expr
            } else {
                Expression::None
            }
        } else {
            Expression::None
        };
        ast::AST::Expression(Expression::FunctionCallExpr(FunctionCallExpr::FunctionCallExpr(
            FunctionCallExprBase::new(Expression::BuiltinFunction(f),
            vec![lhs, rhs],
            Some(0),
        ))))
    }};
}

#[macro_export]
macro_rules! _visit_bool_expr {
    ($ctx: expr,$self: expr) => {
        _visit_binary_expr!($ctx, $self)
    };
}

pub fn build_ast_from_parse_tree(code: &str) -> ast::AST {
    let mut lexer = SolidityLexer::new(InputStream::new(code));
    lexer.add_error_listener(Box::new(MyErrorListener {
        code: code.to_string(),
    }));
    let tokens = CommonTokenStream::new(lexer);
    let mut parser = SolidityParser::new(tokens);
    let root = parser.sourceUnit().expect("");
    // parser.add_error_listener(MyErrorListener{code:code.to_string()}));
    let mut v = BuildASTVisitor::new(code.to_string());
    root.accept(&mut v);
    ast::AST::None
}

pub fn build_ast(code: &str) -> ast::AST {
    let mut full_ast = build_ast_from_parse_tree(code);
    // assert isinstance(full_ast, ast.SourceUnit)
    // let full_ast.original_code = str(code).split("\n");
    //  full_ast
    ast::AST::None
}

struct BuildASTVisitor {
    pub emitter: Emitter,
    pub code: String,
    pub asts: ast::AST,
}
impl BuildASTVisitor {
    pub fn new(code: String) -> Self {
        Self {
            emitter: Emitter::new(Some(code.clone())),
            code,
            asts: ast::AST::None,
        }
    }
}
use std::any::{Any, TypeId};
pub fn is_instance<S: ?Sized + Any, T: ?Sized + Any>(_s: &T) -> bool {
    TypeId::of::<T>() == TypeId::of::<S>()
}
// pub fn is_instance<'t,T:'t>(s: &'t dyn Any) -> bool {
//     TypeId::of::<T>() == s.type_id()
// }
pub fn print_type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
use crate::solidity_parser::generated::solidityvisitor::SolidityVisitorCompat;
use antlr_rust::parser::ParserNodeType;
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::tree::ParseTreeVisitorCompat;
// impl<'input> SolidityVisitor<'input> for BuildASTVisitor {
// }
impl<'input> ParseTreeVisitorCompat<'input> for BuildASTVisitor {
    type Node = SolidityParserContextType;
    type Return = ast::AST;
    fn temp_result(&mut self) -> &mut <Self as ParseTreeVisitorCompat<'input>>::Return {
        &mut self.asts
    }
}
//  pub fn  handle_field(self, field){
//         if field.is_none(){None}
//         else if isinstance(field, list)
//             { [handle_field(element) for element in field]}
//         else if isinstance(field, CommonToken)
//             // text
//             { field.text}
//         else
//             // other
//            {  self.visit(field)}
//         }
impl<'input> SolidityVisitorCompat<'input> for BuildASTVisitor {
    // type Node = SolidityParserContextType;
    //             type Return = ast::AST;
    // fn visit(self, tree:ParserNodeType){
    //     let mut sub_ast = self.visit(tree);
    //     if is_instance::<ast::AST>(&sub_ast){
    //         // sub_ast.line = tree.start.line;
    //         // sub_ast.column = tree.start.column + 1;
    //     }
    //     //  sub_ast
    //     }

    // fn  visit_children(self, ctx: ParserRuleContext){
    //     // determine corresponding class name
    //     let mut t = print_type_of(ctx);
    //     t = t.replace("Context", "");
    //     // ast::AST::Identifier
    //     // // may be able to return the result for a SINGLE, UNNAMED CHILD without wrapping it in an object
    //     // let direct_unnamed = ["TypeName", "ContractPart", "StateMutability", "Statement", "SimpleStatement"]
    //     // if direct_unnamed.contains(&t)
    //     //    { if ctx.get_child_count() != 1
    //     //        { raise TypeError(t + " does not have a single, unnamed child");}
    //     //     ret = self.handle_field(ctx.get_child(0));
    //     //     return ret
    //     //     }

    //     // // HANDLE ALL FIELDS of ctx
    //     // d = ctx.__dict__

    //     // // extract fields
    //     // fields = d.keys()
    //     // fields = [f for f in fields if not f.startswith("_")]
    //     // ignore = ["parentCtx", "invokingState", "children", "start", "stop", "exception", "parser"]
    //     // fields = [f for f in fields if f not in ignore]

    //     // // visit fields
    //     // visited_fields = {}
    //     // for f in fields:
    //     //     visited_fields[f] = self.handle_field(d[f])

    //     // // may be able to return the result for a SINGLE, NAMED CHILD without wrapping it in an object
    //     // direct = ["ModifierList", "ParameterList", "ReturnParameters", "FunctionCallArguments"]
    //     // if t in direct:
    //     //     if len(visited_fields) != 1:
    //     //         raise TypeError(t + " does not have a single, named child")
    //     //     key = list(visited_fields.keys())[0]
    //     //     return visited_fields[key]

    //     // // CONSTRUCT AST FROM FIELDS
    //     // if hasattr(ast, t){
    //     //     c = getattr(ast, t)
    //     //     // call initializer
    //     //     try:
    //     //         return c(**visited_fields)
    //     //     except TypeError as e:
    //     //         raise TypeError("Could not call initializer for " + t) from e
    //     // else:
    //     //     // abort if not constructor found for this node type
    //     //     raise ValueError(t)
    //     }

    fn visit_identifier(&mut self, ctx: &IdentifierContext<'input>) -> Self::Return {
        let name = ctx.name.clone().expect("visit_identifier").text;
        // if name.startswith(cfg.reserved_name_prefix) or name.startswith(f"_{cfg.reserved_name_prefix}"){
        //     raise SyntaxException(f"Identifiers must not start with reserved prefix _?{cfg.reserved_name_prefix}", ctx, self.code)
        // elif name.endswith(cfg.reserved_conflict_resolution_suffix){
        //     raise SyntaxException(f"Identifiers must not end with reserved suffix {cfg.reserved_name_prefix}", ctx, self.code)
        // return ast.Identifier(name)
        ast::AST::Identifier(Identifier::Identifier(IdentifierBase::new(
            name.to_string(),
        )))
    }

    fn visit_pragmaDirective(&mut self, ctx: &PragmaDirectiveContext<'input>) -> Self::Return {
        // ctx.pragma().expect("visit_pragmaDirective").accept(self);
        // let pragmas=self.visit();
        let s = format!("pragma ;");
        ast::AST::Pragma(s)
    }

    fn visit_VersionPragma(&mut self, ctx: &VersionPragmaContext<'input>) -> Self::Return {
        let version = ctx.ver.as_ref().unwrap().get_text();
        let version = version.trim();
        // spec = NpmSpec(version)
        let name = ctx.name.as_ref().unwrap();
        // if name == "zkay" and Version(cfg.zkay_version) not in spec:
        //     raise SyntaxException(f"Contract requires a different zkay version.\n"
        //                           f"Current version is {cfg.zkay_version} but pragma zkay mandates {version}.",
        //                           ctx.ver, self.code)
        // elif name != "zkay" and spec != cfg.zkay_solc_version_compatibility:
        //     // For backwards compatibility with older zkay versions
        //     assert name == "solidity"
        //     raise SyntaxException(f"Contract requires solidity version {spec}, which is not compatible "
        //                           f"with the current zkay version (requires {cfg.zkay_solc_version_compatibility}).",
        //                           ctx.ver, self.code)

        ast::AST::VersionPragma(format!("{name} {version}"))
    }

    // Visit a parse tree produced by SolidityParser#contractDefinition.
    fn visit_contractDefinition(
        &mut self,
        ctx: &ContractDefinitionContext<'input>,
    ) -> Self::Return {
        let idf = if let Some(idf) = &ctx.idf {
            idf.accept(self);
            if let ast::AST::Identifier(a) = self.temp_result().clone() {
                Some(a)
            } else {
                None
            }
        } else {
            None
        };
        // if "$" in identifier.name:
        //     raise SyntaxException("$ is not allowed in zkay contract identifiers", ctx.idf, self.code)
        // parts = [self.visit(c) for c in ctx.parts]
        // state_vars = [p for p in parts if isinstance(p, StateVariableDeclaration)]
        // cfdefs = [p for p in parts if isinstance(p, ast.ConstructorOrFunctionDefinition)]
        // constructors = [p for p in cfdefs if p.is_constructor]
        // functions = [p for p in cfdefs if p.is_function]
        // enums = [p for p in parts if isinstance(p, ast.EnumDefinition)]
        // return ContractDefinition(identifier, state_vars, constructors, functions, enums)
        let state_variable_declarations: Vec<_> = ctx
            .parts
            .iter()
            .filter_map(|p| {
                if let Some(v) = p.stateVariableDeclaration() {
                    v.accept(self);
                    if let ast::AST::IdentifierDeclaration(
                        IdentifierDeclaration::StateVariableDeclaration(a),
                    ) = self.temp_result().clone()
                    {
                        Some(a)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        let constructor_definitions: Vec<_> = ctx
            .parts
            .iter()
            .filter_map(|p| {
                if let Some(v) = p.constructorDefinition() {
                    v.accept(self);
                    if let ast::AST::NamespaceDefinition(
                        NamespaceDefinition::ConstructorOrFunctionDefinition(a),
                    ) = self.temp_result().clone()
                    {
                        Some(a)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        let function_definitions: Vec<_> = ctx
            .parts
            .iter()
            .filter_map(|p| {
                if let Some(v) = p.functionDefinition() {
                    v.accept(self);
                    if let ast::AST::NamespaceDefinition(
                        NamespaceDefinition::ConstructorOrFunctionDefinition(a),
                    ) = self.temp_result().clone()
                    {
                        Some(a)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        let enum_definitions: Vec<_> = ctx
            .parts
            .iter()
            .filter_map(|p| {
                if let Some(v) = p.enumDefinition() {
                    v.accept(self);
                    if let ast::AST::NamespaceDefinition(NamespaceDefinition::EnumDefinition(a)) =
                        self.temp_result().clone()
                    {
                        Some(a)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        ast::AST::NamespaceDefinition(NamespaceDefinition::ContractDefinition(
            ContractDefinition::new (
                idf,
                state_variable_declarations,
                constructor_definitions,
                function_definitions,
                enum_definitions,
                None,
                None,
            ),
        ))
    }

    // fn  handle_fdef(self, ctx){
    //     if isinstance(ctx, SolidityParser.ConstructorDefinitionContext){
    //         idf, ret_params = None, None
    //     else:
    //         idf, ret_params = self.visit(ctx.idf), self.handle_field(ctx.return_parameters)
    //         if "$" in idf.name:
    //             raise SyntaxException("$ is not allowed in zkay function identifiers", ctx.idf, self.code)
    //     params, mods, body = self.handle_field(ctx.parameters), self.handle_field(ctx.modifiers), self.visit(ctx.body)
    //     return ast.ConstructorOrFunctionDefinition(idf, params, mods, ret_params, body)

    fn visit_functionDefinition(
        &mut self,
        ctx: &FunctionDefinitionContext<'input>,
    ) -> Self::Return {
        // self.handle_fdef(ctx)
        let idf = if let Some(idf) = &ctx.idf {
            idf.accept(self);
            if let ast::AST::Identifier(a) = self.temp_result().clone() {
                Some(a)
            } else {
                None
            }
        } else {
            None
        };
        let return_parameters = Some(if let Some(rp) = &ctx.return_parameters {
            if let Some(rp) = &rp.return_parameters {
                rp.params
                    .iter()
                    .map(|param| {
                        param.accept(self);
                        if let ast::AST::IdentifierDeclaration(IdentifierDeclaration::Parameter(
                            a,
                        )) = self.temp_result().clone()
                        {
                            a
                        } else {
                            Parameter::default()
                        }
                    })
                    .collect()
            } else {
                vec![]
            }
        } else {
            vec![]
        });
        let parameters = Some(if let Some(p) = &ctx.parameters {
            p.params
                .iter()
                .map(|param| {
                    param.accept(self);
                    if let ast::AST::IdentifierDeclaration(IdentifierDeclaration::Parameter(a)) =
                        self.temp_result().clone()
                    {
                        a
                    } else {
                        Parameter::default()
                    }
                })
                .collect()
        } else {
            vec![]
        });
        let modifiers = Some(if let Some(p) = &ctx.modifiers {
            p.modifiers
                .iter()
                .filter_map(|modifier| {
                    modifier.accept(self);
                    if let ast::AST::Modifier(a) = self.temp_result().clone() {
                        Some(a)
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            vec![]
        });
        let body = if let Some(p) = &ctx.body {
            p.accept(self);
            if let ast::AST::Statement(Statement::StatementList(block)) = self.temp_result().clone()
            {
                if let StatementList::Block(block) = block {
                    block
                } else {
                    Block::default()
                }
            } else {
                Block::default()
            }
        } else {
            Block::default()
        };
        ast::AST::NamespaceDefinition(NamespaceDefinition::ConstructorOrFunctionDefinition(
            ConstructorOrFunctionDefinition::new(
                idf,
                parameters,
                modifiers,
                return_parameters,
                Some(body),
            ),
        ))
    }

    fn visit_constructorDefinition(
        &mut self,
        ctx: &ConstructorDefinitionContext<'input>,
    ) -> Self::Return {
        // self.handle_fdef(ctx)
        let idf = None;
        let return_parameters = None;
        let parameters = if let Some(p) = &ctx.parameters {
            p.params
                .iter()
                .map(|param| {
                    param.accept(self);
                    if let ast::AST::IdentifierDeclaration(IdentifierDeclaration::Parameter(a)) =
                        self.temp_result().clone()
                    {
                        Some(a)
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            None
        };
        let modifiers = if let Some(p) = &ctx.modifiers {
            p.modifiers
                .iter()
                .map(|modifier| {
                    modifier.accept(self);
                    if let ast::AST::Modifier(a) = self.temp_result().clone() {
                        Some(a)
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            None
        };
        let body = if let Some(p) = &ctx.body {
            p.accept(self);
            if let ast::AST::Statement(Statement::StatementList(block)) = self.temp_result().clone()
            {
                if let StatementList::Block(block) = block {
                    block
                } else {
                    Block::default()
                }
            } else {
                Block::default()
            }
        } else {
            Block::default()
        };
        ast::AST::NamespaceDefinition(NamespaceDefinition::ConstructorOrFunctionDefinition(
            ConstructorOrFunctionDefinition::new(
                idf,
                parameters,
                modifiers,
                return_parameters,
                Some(body),
            ),
        ))
    }

    fn visit_enumDefinition(&mut self, ctx: &EnumDefinitionContext<'input>) -> Self::Return {
        // idf = self.visit(ctx.idf)
        // if "$" in idf.name:
        //     raise SyntaxException("$ is not allowed in zkay enum identifiers", ctx.idf, self.code)
        // values = [self.visit(v) for v in ctx.values]
        // return ast.EnumDefinition(idf, values)
        let idf = if let Some(idf) = &ctx.idf {
            idf.accept(self);
            if let ast::AST::Identifier(a) = self.temp_result().clone() {
                Some(a)
            } else {
                None
            }
        } else {
            None
        };
        let values: Vec<_> = ctx
            .values
            .iter()
            .map(|v| {
                v.accept(self);
                if let ast::AST::EnumValue(a) = self.temp_result().clone() {
                    a
                } else {
                    EnumValue::new(None)
                }
            })
            .collect();
        ast::AST::NamespaceDefinition(NamespaceDefinition::EnumDefinition(EnumDefinition::new(
            idf, values,
        )))
    }

    fn visit_enumValue(&mut self, ctx: &EnumValueContext<'input>) -> Self::Return {
        // idf = self.visit(ctx.idf)
        // if "$" in idf.name:
        //     raise SyntaxException("$ is not allowed in zkay enum value identifiers", ctx.idf, self.code)
        // return ast.EnumValue(idf)
        let idf = if let Some(idf) = &ctx.idf {
            idf.accept(self);
            if let ast::AST::Identifier(a) = self.temp_result().clone() {
                Some(a)
            } else {
                None
            }
        } else {
            None
        };
        ast::AST::EnumValue(EnumValue::new(idf))
    }

    // Visit a parse tree produced by SolidityParser#NumberLiteralExpr.
    fn visit_NumberLiteralExpr(&mut self, ctx: &NumberLiteralExprContext<'input>) -> Self::Return {
        // v = int(ctx.getText().replace("_", ""), 0)
        // return NumberLiteralExpr(v, ctx.getText().startswith(("0x", "0X")))
        let s = ctx.get_text();
        let v = s.replace("_", "").parse().unwrap_or(0);
        ast::AST::Expression(Expression::LiteralExpr(LiteralExpr::NumberLiteralExpr(
            NumberLiteralExpr::new(v, s.starts_with("0x") || s.starts_with("0X")),
        )))
    }

    // Visit a parse tree produced by SolidityParser#BooleanLiteralExpr.
    fn visit_BooleanLiteralExpr(
        &mut self,
        ctx: &BooleanLiteralExprContext<'input>,
    ) -> Self::Return {
        //   b = ctx.getText() == "true"
        // return BooleanLiteralExpr(b)
        ast::AST::Expression(Expression::LiteralExpr(LiteralExpr::BooleanLiteralExpr(
            BooleanLiteralExpr::new(ctx.get_text() == String::from("true")),
        )))
    }

    fn visit_StringLiteralExpr(&mut self, ctx: &StringLiteralExprContext<'input>) -> Self::Return {
        let mut s = ctx.get_text();
        let n = s.len();
        // // Remove quotes
        s = if s.starts_with("\"") {
            s[1..n - 1].replace("\"", "")
        } else {
            s[2..n - 2].to_string()
        };
        assert!(
            false,
            "Use of unsupported string literal expression {:?},{:?}",
            ctx, self.code
        );
        // // raise SyntaxException("Use of unsupported string literal expression", ctx, self.code)
        // return StringLiteralExpr(s)
        ast::AST::Expression(Expression::LiteralExpr(LiteralExpr::StringLiteralExpr(
            StringLiteralExpr::new(s),
        )))
    }

    fn visit_TupleExpr(&mut self, ctx: &TupleExprContext<'input>) -> Self::Return {
        let mut contents = if let Some(e) = &ctx.expr {
            use crate::solidity_parser::generated::solidityparser::TupleExpressionContextAttrs;
            e.expression_all()
        } else {
            vec![]
        };
        if contents.len() > 1 {
            contents = contents[1..contents.len() - 1].to_vec();
        }
        let mut elements = vec![];
        for idx in (0..contents.len()).step_by(2) {
            contents[idx].accept(self);
            if let ast::AST::Expression(expr) = self.temp_result().clone() {
                elements.push(expr);
            }
        }
        // contents = ctx.expr.children[1:-1]
        // elements = []
        // for idx in range(0, len(contents), 2){
        //     elements.append(self.visit(contents[idx]))
        // return ast.TupleExpr(elements)
        ast::AST::Expression(Expression::TupleOrLocationExpr(
            TupleOrLocationExpr::TupleExpr(TupleExpr::new(elements)),
        ))
    }

    fn visit_modifier(&mut self, ctx: &ModifierContext<'input>) -> Self::Return {
        //  ctx.getText()
        ast::AST::Modifier(ctx.get_text())
    }

    fn visit_annotatedTypeName(&mut self, ctx: &AnnotatedTypeNameContext<'input>) -> Self::Return {
        // pa = None
        // hom = Homomorphism.NonHomomorphic
        // if ctx.privacy_annotation is not None:
        //     pa = self.visit(ctx.privacy_annotation)
        //     if ctx.homomorphism is not None:
        //         hom = self.visit(ctx.homomorphism)

        //     if not (isinstance(pa, ast.AllExpr) or isinstance(pa, ast.MeExpr) or isinstance(pa, IdentifierExpr)){
        //         raise SyntaxException("Privacy annotation can only be me | all | Identifier", ctx.privacy_annotation, self.code)
        //     if isinstance(pa, ast.AllExpr) and hom != Homomorphism.NonHomomorphic:
        //         raise SyntaxException("Public types cannot be homomorphic", ctx.homomorphism, self.code)

        // return ast.AnnotatedTypeName(self.visit(ctx.type_name), pa, hom)
        let mut privacy_annotation = None;
        let mut homomorphism = String::from("NON_HOMOMORPHIC");
        if let Some(pa) = &ctx.privacy_annotation {
            pa.accept(self);
            privacy_annotation = if let ast::AST::Expression(expr) = self.temp_result().clone() {
                Some(expr)
            } else {
                None
            };
            if let Some(hom) = &ctx.homomorphism {
                hom.accept(self);
                if let ast::AST::Homomorphism(hom) = self.temp_result().clone() {
                    homomorphism = hom;
                }
            }
        }
        let type_name = if let Some(tn) = &ctx.type_name {
            tn.accept(self);
            if let ast::AST::TypeName(tn) = self.temp_result().clone() {
                tn
            } else {
                TypeName::None
            }
        } else {
            TypeName::None
        };
        ast::AST::AnnotatedTypeName(AnnotatedTypeName::new(
            type_name,
            privacy_annotation,
            homomorphism,
        ))
    }

    fn visit_homomorphismAnnotation(
        &mut self,
        ctx: &HomomorphismAnnotationContext<'input>,
    ) -> Self::Return {
        let t = ctx.get_text();
        // for h in Homomorphism
        //     if h.type_annotation == t
        //         return h
        // else:
        //     raise SyntaxException(f"Unsupported homomorphism {t}", ctx, self.code)
        if let Some(v) = HOMOMORPHISM_STORE
            .lock()
            .unwrap()
            .values()
            .filter(|h| h.type_annotation == t)
            .next()
        {
            ast::AST::Homomorphism(v.value.clone())
        } else {
            assert!(
                false,
                "Unsupported homomorphism {t},{:?},{:?}",
                ctx, self.code
            );
            ast::AST::None
        }
    }

    fn visit_elementaryTypeName(
        &mut self,
        ctx: &ElementaryTypeNameContext<'input>,
    ) -> Self::Return {
        let t = ctx.get_text();
        match t.as_str() {
            "address" => ast::AST::TypeName(TypeName::UserDefinedTypeName(
                UserDefinedTypeName::AddressTypeName(AddressTypeName::new()),
            )),
            "address payable" => ast::AST::TypeName(TypeName::UserDefinedTypeName(
                UserDefinedTypeName::AddressPayableTypeName(AddressPayableTypeName::new()),
            )),
            "bool" => ast::AST::TypeName(TypeName::ElementaryTypeName(
                ElementaryTypeName::BoolTypeName(BoolTypeName::new()),
            )),
            ts if t.starts_with("int") => ast::AST::TypeName(TypeName::ElementaryTypeName(
                ElementaryTypeName::NumberTypeName(NumberTypeName::IntTypeName(IntTypeName::new(
                    t,
                ))),
            )),
            ts if t.starts_with("uint") => ast::AST::TypeName(TypeName::ElementaryTypeName(
                ElementaryTypeName::NumberTypeName(NumberTypeName::UintTypeName(
                    UintTypeName::new(t),
                )),
            )),
            "var" => {
                assert!(
                    false,
                    "Use of unsupported var keyword,{:?},{:?}",
                    ctx, self.code
                );
                ast::AST::None
            }
            _ => {
                assert!(
                    false,
                    "Use of unsupported type {t},{:?},{:?}",
                    ctx, self.code
                );
                ast::AST::None
            }
        }
    }

    fn visit_IndexExpr(&mut self, ctx: &IndexExprContext<'input>) -> Self::Return {
        // arr = self.visit(ctx.arr)
        // if not isinstance(arr, ast.LocationExpr){
        //     raise SyntaxException(f"Expression cannot be indexed", ctx.arr, self.code)
        // index = self.visit(ctx.index)
        // return IndexExpr(arr, index)
        let arr = if let Some(arr) = &ctx.arr {
            arr.accept(self);
            if let ast::AST::Expression(expr) = self.temp_result().clone() {
                if let Expression::TupleOrLocationExpr(expr) = expr {
                    if let TupleOrLocationExpr::LocationExpr(expr) = expr {
                        expr
                    } else {
                        LocationExpr::default()
                    }
                } else {
                    LocationExpr::default()
                }
            } else {
                LocationExpr::default()
            }
        } else {
            LocationExpr::default()
        };
        let index = if let Some(index) = &ctx.index {
            index.accept(self);
            if let ast::AST::Expression(expr) = self.temp_result().clone() {
                expr
            } else {
                Expression::None
            }
        } else {
            Expression::None
        };

        ast::AST::Expression(Expression::TupleOrLocationExpr(
            TupleOrLocationExpr::LocationExpr(LocationExpr::IndexExpr(IndexExpr::new(arr, index))),
        ))
    }

    fn visit_ParenthesisExpr(&mut self, ctx: &ParenthesisExprContext<'input>) -> Self::Return {
        // f = BuiltinFunction("parenthesis").override(line=ctx.start.line, column=ctx.start.column)
        // expr = self.visit(ctx.expr)
        // return FunctionCallExpr(f, [expr])
        let mut f = BuiltinFunction::new("parenthesis");
        f.expression_base.ast_base.line = ctx.start().line as i32;
        f.expression_base.ast_base.column = ctx.start().column as i32;
        let expr = if let Some(expr) = &ctx.expr {
            expr.accept(self);
            if let ast::AST::Expression(expr) = self.temp_result().clone() {
                expr
            } else {
                Expression::None
            }
        } else {
            Expression::None
        };
        ast::AST::Expression(Expression::FunctionCallExpr(FunctionCallExpr::FunctionCallExpr(
            FunctionCallExprBase::new(Expression::BuiltinFunction(f),
            vec![expr],
            Some(0),
        ))))
    }

    fn visit_SignExpr(&mut self, ctx: &SignExprContext<'input>) -> Self::Return {
        // f = BuiltinFunction("sign" + ctx.op.text).override(line=ctx.op.line, column=ctx.op.column)
        // expr = self.visit(ctx.expr)
        // return FunctionCallExpr(f, [expr])
        let mut f = BuiltinFunction::new("sign");
        if let Some(op) = &ctx.op {
            f = BuiltinFunction::new(("sign".to_string() + &op.text).as_str());
            f.expression_base.ast_base.line = op.line as i32;
            f.expression_base.ast_base.column = op.column as i32;
        }
        let expr = if let Some(expr) = &ctx.expr {
            expr.accept(self);
            if let ast::AST::Expression(expr) = self.temp_result().clone() {
                expr
            } else {
                Expression::None
            }
        } else {
            Expression::None
        };
        ast::AST::Expression(Expression::FunctionCallExpr(FunctionCallExpr::FunctionCallExpr(
            FunctionCallExprBase::new(Expression::BuiltinFunction(f),
            vec![expr],
            Some(0),
        ))))
    }

    fn visit_NotExpr(&mut self, ctx: &NotExprContext<'input>) -> Self::Return {
        // f = BuiltinFunction("!").override(line=ctx.start.line, column=ctx.start.column)
        // expr = self.visit(ctx.expr)
        // return FunctionCallExpr(f, [expr])
        let mut f = BuiltinFunction::new("!");
        f.expression_base.ast_base.line = ctx.start().line as i32;
        f.expression_base.ast_base.column = ctx.start().column as i32;
        let expr = if let Some(expr) = &ctx.expr {
            expr.accept(self);
            if let ast::AST::Expression(expr) = self.temp_result().clone() {
                expr
            } else {
                Expression::None
            }
        } else {
            Expression::None
        };
        ast::AST::Expression(Expression::FunctionCallExpr(FunctionCallExpr::FunctionCallExpr(
            FunctionCallExprBase::new(Expression::BuiltinFunction(f),
            vec![expr],
            Some(0),
        ))))
    }

    fn visit_BitwiseNotExpr(&mut self, ctx: &BitwiseNotExprContext<'input>) -> Self::Return {
        // f = BuiltinFunction("~").override(line=ctx.start.line, column=ctx.start.column)
        // expr = self.visit(ctx.expr)
        // return FunctionCallExpr(f, [expr])
        let mut f = BuiltinFunction::new("~");
        f.expression_base.ast_base.line = ctx.start().line as i32;
        f.expression_base.ast_base.column = ctx.start().column as i32;
        let expr = if let Some(expr) = &ctx.expr {
            expr.accept(self);
            if let ast::AST::Expression(expr) = self.temp_result().clone() {
                expr
            } else {
                Expression::None
            }
        } else {
            Expression::None
        };
        ast::AST::Expression(Expression::FunctionCallExpr(FunctionCallExpr::FunctionCallExpr(
            FunctionCallExprBase::new(Expression::BuiltinFunction(f),
            vec![expr],
            Some(0),
        ))))
    }

    //     fn  _visitBinaryExpr(self, ctx){
    //         // lhs = self.visit(ctx.lhs)
    //         // rhs = self.visit(ctx.rhs)
    //         // f = BuiltinFunction(ctx.op.text).override(line=ctx.op.line, column=ctx.op.column)
    //         // return FunctionCallExpr(f, [lhs, rhs])
    //    ast::AST::None
    //  }

    //     fn  _visitBoolExpr(self, ctx){
    //         // return self._visitBinaryExpr(ctx)
    //         ast::AST::None
    //     }

    fn visit_PowExpr(&mut self, ctx: &PowExprContext<'input>) -> Self::Return {
        // return self._visitBinaryExpr(ctx)
        _visit_binary_expr!(ctx, self)
    }

    fn visit_MultDivModExpr(&mut self, ctx: &MultDivModExprContext<'input>) -> Self::Return {
        // return self._visitBinaryExpr(ctx)
        _visit_binary_expr!(ctx, self)
    }

    fn visit_PlusMinusExpr(&mut self, ctx: &PlusMinusExprContext<'input>) -> Self::Return {
        // return self._visitBinaryExpr(ctx)
        _visit_binary_expr!(ctx, self)
    }

    fn visit_CompExpr(&mut self, ctx: &CompExprContext<'input>) -> Self::Return {
        // return self._visitBinaryExpr(ctx)
        _visit_binary_expr!(ctx, self)
    }

    fn visit_EqExpr(&mut self, ctx: &EqExprContext<'input>) -> Self::Return {
        // return self._visitBinaryExpr(ctx)
        _visit_binary_expr!(ctx, self)
    }

    fn visit_AndExpr(&mut self, ctx: &AndExprContext<'input>) -> Self::Return {
        // return self._visitBoolExpr(ctx)
        _visit_bool_expr!(ctx, self)
    }

    fn visit_OrExpr(&mut self, ctx: &OrExprContext<'input>) -> Self::Return {
        // return self._visitBoolExpr(ctx)
        _visit_bool_expr!(ctx, self)
    }

    fn visit_BitwiseOrExpr(&mut self, ctx: &BitwiseOrExprContext<'input>) -> Self::Return {
        // return self._visitBinaryExpr(ctx)
        _visit_binary_expr!(ctx, self)
    }

    fn visit_BitShiftExpr(&mut self, ctx: &BitShiftExprContext<'input>) -> Self::Return {
        // return self._visitBinaryExpr(ctx)
        _visit_binary_expr!(ctx, self)
    }

    fn visit_BitwiseAndExpr(&mut self, ctx: &BitwiseAndExprContext<'input>) -> Self::Return {
        // return self._visitBinaryExpr(ctx)
        _visit_binary_expr!(ctx, self)
    }
    fn visit_BitwiseXorExpr(&mut self, ctx: &BitwiseXorExprContext<'input>) -> Self::Return {
        // return self._visitBinaryExpr(ctx)
        _visit_binary_expr!(ctx, self)
    }

    fn visit_IteExpr(&mut self, ctx: &IteExprContext<'input>) -> Self::Return {
        // f = BuiltinFunction("ite")
        // cond = self.visit(ctx.cond)
        // then_expr = self.visit(ctx.then_expr)
        // else_expr = self.visit(ctx.else_expr)
        // return FunctionCallExpr(f, [cond, then_expr, else_expr])
        let mut f = BuiltinFunction::new("ite");
        let cond = if let Some(expr) = &ctx.cond {
            expr.accept(self);
            if let ast::AST::Expression(expr) = self.temp_result().clone() {
                expr
            } else {
                Expression::None
            }
        } else {
            Expression::None
        };
        let then_expr = if let Some(expr) = &ctx.then_expr {
            expr.accept(self);
            if let ast::AST::Expression(expr) = self.temp_result().clone() {
                expr
            } else {
                Expression::None
            }
        } else {
            Expression::None
        };
        let else_expr = if let Some(expr) = &ctx.else_expr {
            expr.accept(self);
            if let ast::AST::Expression(expr) = self.temp_result().clone() {
                expr
            } else {
                Expression::None
            }
        } else {
            Expression::None
        };
        ast::AST::Expression(Expression::FunctionCallExpr(FunctionCallExpr::FunctionCallExpr(
            FunctionCallExprBase::new(Expression::BuiltinFunction(f),
            vec![cond, then_expr, else_expr],
            Some(0),
        ))))
    }

    // rehom_expressions = {}
    // for h in Homomorphism:
    //     rehom_expressions[h.rehom_expr_name] = h

    fn visit_FunctionCallExpr(&mut self, ctx: &FunctionCallExprContext<'input>) -> Self::Return {
        // func = self.visit(ctx.func)
        // args = self.handle_field(ctx.args)

        // if isinstance(func, IdentifierExpr){
        //     if func.idf.name == "reveal":
        //         if len(args) != 2:
        //             raise SyntaxException(f"Invalid number of arguments for reveal: {args}", ctx.args, self.code)
        //         return ReclassifyExpr(args[0], args[1], None)
        //     elif func.idf.name in self.rehom_expressions:
        //         name = func.idf.name
        //         homomorphism = self.rehom_expressions[name]
        //         if len(args) != 1:
        //             raise SyntaxException(f"Invalid number of arguments for {name}: {args}", ctx.args, self.code)
        //         return RehomExpr(args[0], homomorphism)

        // return FunctionCallExpr(func, args)
        let mut func = if let Some(expr) = &ctx.func {
            expr.accept(self);
            if let ast::AST::Expression(expr) = self.temp_result().clone() {
                expr
            } else {
                Expression::None
            }
        } else {
            Expression::None
        };
        let args = if let Some(args) = &ctx.args {
            args.exprs
                .iter()
                .map(|expr| {
                    expr.accept(self);
                    if let ast::AST::Expression(expr) = self.temp_result().clone() {
                        expr
                    } else {
                        Expression::None
                    }
                })
                .collect()
        } else {
            vec![]
        };
        func = if let Expression::TupleOrLocationExpr(func) = &func {
            if let TupleOrLocationExpr::LocationExpr(func) = func {
                if let LocationExpr::IdentifierExpr(func) = func {
                    if func.idf.name() == String::from("reveal") {
                        assert!(
                            args.len() == 2,
                            "Invalid number of arguments for reveal: {args:?},{:?},{:?}",
                            ctx.args,
                            self.code
                        );
                        // raise SyntaxException(f"Invalid number of arguments for reveal: {args}", ctx.args, self.code)
                        Expression::ReclassifyExpr(ReclassifyExpr::ReclassifyExpr(
                            ReclassifyExprBase::new(args[0].clone(), args[1].clone(), None),
                        ))
                    } else if let Some(homomorphism) =
                        REHOM_EXPRESSIONS.lock().unwrap().get(&func.idf.name())
                    {
                        assert!(
                            args.len() == 1,
                            "Invalid number of arguments for {:?}: {args:?},{:?},{:?}",
                            func,
                            ctx.args,
                            self.code
                        );
                        // raise SyntaxException(f"Invalid number of arguments for {name}: {args}", ctx.args, self.code)
                        Expression::ReclassifyExpr(ReclassifyExpr::RehomExpr(RehomExpr::new(
                            args[0].clone(),
                            Some(homomorphism.value.clone()),
                        )))
                    } else {
                        Expression::None
                    }
                } else {
                    Expression::default()
                }
            } else {
                Expression::None
            }
        } else {
            Expression::None
        };
        ast::AST::Expression(Expression::FunctionCallExpr(FunctionCallExpr::FunctionCallExpr(
            FunctionCallExprBase::new(func,
            args,
            Some(0),
        ))))
    }

    fn visit_ifStatement(&mut self, ctx: &IfStatementContext<'input>) -> Self::Return {
        // cond = self.visit(ctx.condition)
        // then_branch = self.visit(ctx.then_branch)
        // if not isinstance(then_branch, ast.Block){
        //     then_branch = ast.Block([then_branch], was_single_statement=True)

        // if ctx.else_branch is not None:
        //     else_branch = self.visit(ctx.else_branch)
        //     if not isinstance(else_branch, ast.Block){
        //         else_branch = ast.Block([else_branch], was_single_statement=True)
        // else:
        //     else_branch = None

        // return ast.IfStatement(cond, then_branch, else_branch)
        let cond = if let Some(expr) = &ctx.condition {
            expr.accept(self);
            if let ast::AST::Expression(expr) = self.temp_result().clone() {
                expr
            } else {
                Expression::None
            }
        } else {
            Expression::None
        };
        let then_branch = if let Some(expr) = &ctx.then_branch {
            expr.accept(self);
            if let ast::AST::Statement(Statement::StatementList(expr)) = self.temp_result().clone()
            {
                if let StatementList::Block(expr) = expr {
                    expr
                } else {
                    Block::default()
                }
            } else if let ast::AST::Statement(expr) = self.temp_result().clone() {
                Block::new(vec![expr], true)
            } else {
                Block::default()
            }
        } else {
            Block::default()
        };
        let else_branch = if let Some(expr) = &ctx.else_branch {
            expr.accept(self);
            Some(
                if let ast::AST::Statement(Statement::StatementList(expr)) =
                    self.temp_result().clone()
                {
                    if let StatementList::Block(expr) = expr {
                        expr
                    } else {
                        Block::default()
                    }
                } else if let ast::AST::Statement(expr) = self.temp_result().clone() {
                    Block::new(vec![expr], true)
                } else {
                    Block::default()
                },
            )
        } else {
            None
        };
        ast::AST::Statement(Statement::IfStatement(IfStatement::new(
            cond,
            then_branch,
            else_branch,
        )))
    }

    fn visit_whileStatement(&mut self, ctx: &WhileStatementContext<'input>) -> Self::Return {
        // cond = self.visit(ctx.condition)
        // body = self.visit(ctx.body)
        // if not isinstance(body, ast.Block){
        //     body = ast.Block([body], was_single_statement=True)
        // return ast.WhileStatement(cond, body)
        let cond = if let Some(expr) = &ctx.condition {
            expr.accept(self);
            if let ast::AST::Expression(expr) = self.temp_result().clone() {
                expr
            } else {
                Expression::None
            }
        } else {
            Expression::None
        };
        let body = if let Some(expr) = &ctx.body {
            expr.accept(self);
            if let ast::AST::Statement(Statement::StatementList(expr)) = self.temp_result().clone()
            {
                if let StatementList::Block(expr) = expr {
                    expr
                } else {
                    Block::default()
                }
            } else if let ast::AST::Statement(expr) = self.temp_result().clone() {
                Block::new(vec![expr], true)
            } else {
                Block::default()
            }
        } else {
            Block::default()
        };

        ast::AST::Statement(Statement::WhileStatement(WhileStatement::new(cond, body)))
    }

    fn visit_doWhileStatement(&mut self, ctx: &DoWhileStatementContext<'input>) -> Self::Return {
        // body = self.visit(ctx.body)
        // cond = self.visit(ctx.condition)
        // if not isinstance(body, ast.Block){
        //     body = ast.Block([body], was_single_statement=True)
        // return ast.DoWhileStatement(body, cond)
        let body = if let Some(expr) = &ctx.body {
            expr.accept(self);
            if let ast::AST::Statement(Statement::StatementList(expr)) = self.temp_result().clone()
            {
                if let StatementList::Block(expr) = expr {
                    expr
                } else {
                    Block::default()
                }
            } else if let ast::AST::Statement(expr) = self.temp_result().clone() {
                Block::new(vec![expr], true)
            } else {
                Block::default()
            }
        } else {
            Block::default()
        };
        let cond = if let Some(expr) = &ctx.condition {
            expr.accept(self);
            if let ast::AST::Expression(expr) = self.temp_result().clone() {
                expr
            } else {
                Expression::None
            }
        } else {
            Expression::None
        };

        ast::AST::Statement(Statement::DoWhileStatement(DoWhileStatement::new(
            body, cond,
        )))
    }

    fn visit_forStatement(&mut self, ctx: &ForStatementContext<'input>) -> Self::Return {
        // init = None if ctx.init is None else self.visit(ctx.init)
        // cond = self.visit(ctx.condition)
        // update = None if ctx.update is None else self.visit(ctx.update)
        // if isinstance(update, ast.Expression){
        //     update = ast.ExpressionStatement(update)
        // body = self.visit(ctx.body)
        // if not isinstance(body, ast.Block){
        //     body = ast.Block([body], was_single_statement=True)
        // return ast.ForStatement(init, cond, update, body)
        let init = if let Some(expr) = &ctx.init {
            expr.accept(self);
            Some(
                if let ast::AST::Statement(Statement::SimpleStatement(expr)) =
                    self.temp_result().clone()
                {
                    expr
                } else {
                    SimpleStatement::None
                },
            )
        } else {
            None
        };
        let cond = if let Some(expr) = &ctx.condition {
            expr.accept(self);
            if let ast::AST::Expression(expr) = self.temp_result().clone() {
                expr
            } else {
                Expression::None
            }
        } else {
            Expression::None
        };
        let update = if let Some(expr) = &ctx.update {
            expr.accept(self);
            Some(
                if let ast::AST::Expression(expr) = self.temp_result().clone() {
                    SimpleStatement::ExpressionStatement(ExpressionStatement::new(expr))
                } else {
                    SimpleStatement::None
                },
            )
        } else {
            None
        };
        let body = if let Some(expr) = &ctx.body {
            expr.accept(self);
            if let ast::AST::Statement(Statement::StatementList(expr)) = self.temp_result().clone()
            {
                if let StatementList::Block(expr) = expr {
                    expr
                } else {
                    Block::default()
                }
            } else if let ast::AST::Statement(expr) = self.temp_result().clone() {
                Block::new(vec![expr], true)
            } else {
                Block::default()
            }
        } else {
            Block::default()
        };

        ast::AST::Statement(Statement::ForStatement(ForStatement::new(
            init, cond, update, body,
        )))
    }

    // fn  is_expr_stmt(self, ctx: SolidityParser.ExpressionContext) -> bool
    //     {// if isinstance(ctx.parentCtx, SolidityParser.ExpressionStatementContext){
    //     //     return True
    //     // elif isinstance(ctx.parentCtx, SolidityParser.ForStatementContext) and ctx == ctx.parentCtx.update:
    //     //     return True
    //     // else:
    //     //     return False
    //     false
    // }

    fn visit_AssignmentExpr(&mut self, ctx: &AssignmentExprContext<'input>) -> Self::Return {
        // if not self.is_expr_stmt(ctx){
        //     raise SyntaxException("Assignments are only allowed as statements", ctx, self.code)
        // lhs = self.visit(ctx.lhs)
        // rhs = self.visit(ctx.rhs)
        // assert ctx.op.text[-1] == "="
        // op = ctx.op.text[:-1] if ctx.op.text != "=" else ""
        // if op:
        //     // If the assignment contains an additional operator -> replace lhs = rhs with lhs = lhs "op" rhs
        //     rhs = FunctionCallExpr(BuiltinFunction(op).override(line=ctx.op.line, column=ctx.op.column), [self.visit(ctx.lhs), rhs])
        //     rhs.line = ctx.rhs.start.line
        //     rhs.column = ctx.rhs.start.column + 1
        // return ast.AssignmentStatement(lhs, rhs, op)
        let lhs = if let Some(expr) = &ctx.lhs {
            expr.accept(self);
            if let ast::AST::Expression(expr) = self.temp_result().clone() {
                expr
            } else {
                Expression::None
            }
        } else {
            Expression::None
        };
        let mut rhs = if let Some(expr) = &ctx.rhs {
            expr.accept(self);
            if let ast::AST::Expression(expr) = self.temp_result().clone() {
                expr
            } else {
                Expression::None
            }
        } else {
            Expression::None
        };
        let op = if let Some(op) = &ctx.op {
            let (line, column) = (op.line as i32, op.column as i32);
            assert!(!op.text.is_empty() && op.text.chars().last().unwrap() == '=');
            let op = if op.text != String::from("=") {
                op.text[..op.text.len() - 1].to_string()
            } else {
                String::new()
            };
            if !op.is_empty() {
                // If the assignment contains an additional operator -> replace lhs = rhs with lhs = lhs "op" rhs
                let mut f = BuiltinFunction::new(&op);
                f.expression_base.ast_base.line = line;
                f.expression_base.ast_base.column = column;
                let mut fce = FunctionCallExprBase::new(
                    Expression::BuiltinFunction(f),
                    vec![lhs.clone(), rhs],
                    Some(0),
                );
                fce.expression_base.ast_base.line = line;
                fce.expression_base.ast_base.column = column + 1;
                rhs = Expression::FunctionCallExpr(FunctionCallExpr::FunctionCallExpr(fce));
            }
            op
        } else {
            // assert!(false);
            String::new()
        };
        let lhs = if let Expression::TupleOrLocationExpr(expr) = lhs {
            match expr {
                TupleOrLocationExpr::TupleExpr(expr) => AssignmentStatementUnion::TupleExpr(expr),
                TupleOrLocationExpr::LocationExpr(expr) => {
                    AssignmentStatementUnion::LocationExpr(expr)
                }
                _ => AssignmentStatementUnion::None,
            }
        } else {
            AssignmentStatementUnion::None
        };

        ast::AST::Statement(Statement::SimpleStatement(
            SimpleStatement::AssignmentStatement(AssignmentStatement::AssignmentStatement(
                AssignmentStatementBase::new(lhs, rhs, Some(op)),
            )),
        ))
    }

    //     fn  _handle_crement_expr(self, ctx, kind: str){
    //         // if not self.is_expr_stmt(ctx){
    //         //     raise SyntaxException(f"{kind}-crement expressions are only allowed as statements", ctx, self.code)
    //         // op = "+" if ctx.op.text == "++" else "-"

    //         // one = NumberLiteralExpr(1)
    //         // one.line = ctx.op.line
    //         // one.column = ctx.op.column + 1

    //         // fct = FunctionCallExpr(BuiltinFunction(op).override(line=ctx.op.line, column=ctx.op.column), [self.visit(ctx.expr), one])
    //         // fct.line = ctx.op.line
    //         // fct.column = ctx.op.column + 1

    //         // return ast.AssignmentStatement(self.visit(ctx.expr), fct, f"{kind}{ctx.op.text}")
    //    ast::AST::None
    //      }

    fn visit_PreCrementExpr(&mut self, ctx: &PreCrementExprContext<'input>) -> Self::Return {
        // return self._handle_crement_expr(ctx, "pre")
        let kind = "pre";
        let mut expr = if let Some(expr) = &ctx.expr {
            expr.accept(self);
            if let ast::AST::Expression(expr) = self.temp_result().clone() {
                expr
            } else {
                Expression::None
            }
        } else {
            Expression::None
        };
        if let Some(op) = &ctx.op {
            let (line, column) = (op.line as i32, op.column as i32);
            let optext = String::from(if op.text == String::from("++") {
                "+"
            } else {
                "-"
            });
            let mut one = NumberLiteralExpr::new(1, false);
            one.literal_expr_base.expression_base.ast_base.line = line;
            one.literal_expr_base.expression_base.ast_base.column = column + 1;
            let mut f = BuiltinFunction::new(&optext);
            f.expression_base.ast_base.line = line;
            f.expression_base.ast_base.column = column;
            let mut fce = FunctionCallExprBase::new(
                Expression::BuiltinFunction(f),
                vec![
                    expr.clone(),
                    Expression::LiteralExpr(LiteralExpr::NumberLiteralExpr(one)),
                ],
                Some(0),
            );
            fce.expression_base.ast_base.line = line;
            fce.expression_base.ast_base.column = column + 1;

            let expr = if let Expression::TupleOrLocationExpr(expr) = expr {
                match expr {
                    TupleOrLocationExpr::TupleExpr(expr) => {
                        AssignmentStatementUnion::TupleExpr(expr)
                    }
                    TupleOrLocationExpr::LocationExpr(expr) => {
                        AssignmentStatementUnion::LocationExpr(expr)
                    }
                    _ => AssignmentStatementUnion::None,
                }
            } else {
                AssignmentStatementUnion::None
            };
            ast::AST::Statement(Statement::SimpleStatement(
                SimpleStatement::AssignmentStatement(AssignmentStatement::AssignmentStatement(
                    AssignmentStatementBase::new(expr, Expression::FunctionCallExpr(FunctionCallExpr::FunctionCallExpr(fce)),
                    Some(format!("{kind}{}", op.text)),
                ))),)
            )
        } else {
            ast::AST::None
        }
    }

    fn visit_PostCrementExpr(&mut self, ctx: &PostCrementExprContext<'input>) -> Self::Return {
        // return self._handle_crement_expr(ctx, "post")
        let kind = "post";
        let mut expr = if let Some(expr) = &ctx.expr {
            expr.accept(self);
            if let ast::AST::Expression(expr) = self.temp_result().clone() {
                expr
            } else {
                Expression::None
            }
        } else {
            Expression::None
        };
        if let Some(op) = &ctx.op {
            let (line, column) = (op.line as i32, op.column as i32);
            let optext = String::from(if op.text == String::from("++") {
                "+"
            } else {
                "-"
            });
            let mut one = NumberLiteralExpr::new(1, false);
            one.literal_expr_base.expression_base.ast_base.line = line;
            one.literal_expr_base.expression_base.ast_base.column = column + 1;
            let mut f = BuiltinFunction::new(&optext);
            f.expression_base.ast_base.line = line;
            f.expression_base.ast_base.column = column;
            let mut fce = FunctionCallExprBase::new(
                Expression::BuiltinFunction(f),
                vec![
                    expr.clone(),
                    Expression::LiteralExpr(LiteralExpr::NumberLiteralExpr(one)),
                ],
                Some(0),
            );
            fce.expression_base.ast_base.line = line;
            fce.expression_base.ast_base.column = column + 1;

            let expr = if let Expression::TupleOrLocationExpr(expr) = expr {
                match expr {
                    TupleOrLocationExpr::TupleExpr(expr) => {
                        AssignmentStatementUnion::TupleExpr(expr)
                    }
                    TupleOrLocationExpr::LocationExpr(expr) => {
                        AssignmentStatementUnion::LocationExpr(expr)
                    }
                    _ => AssignmentStatementUnion::None,
                }
            } else {
                AssignmentStatementUnion::None
            };
            ast::AST::Statement(Statement::SimpleStatement(
                SimpleStatement::AssignmentStatement(AssignmentStatement::AssignmentStatement(
                    AssignmentStatementBase::new(expr, Expression::FunctionCallExpr(FunctionCallExpr::FunctionCallExpr(fce)),
                    Some(format!("{kind}{}", op.text)),
                )),)
            ))
        } else {
            ast::AST::None
        }
    }

    fn visit_expressionStatement(
        &mut self,
        ctx: &ExpressionStatementContext<'input>,
    ) -> Self::Return {
        // e = self.visit(ctx.expr)
        // if isinstance(e, ast.Statement){
        //     return e
        // }
        // else
        //     {// handle require
        //     if isinstance(e, FunctionCallExpr){
        //         f = e.func
        //         if isinstance(f, IdentifierExpr){
        //             if f.idf.name == "require"
        //                { if len(e.args) != 1:
        //                     {raise SyntaxException(f"Invalid number of arguments for require: {e.args}", ctx.expr, self.code)}
        //                 return ast.RequireStatement(e.args[0])}
        //         }
        //     }
        //     assert isinstance(e, ast.Expression)
        //     return ExpressionStatement(e)}
        let expr = if let Some(expr) = &ctx.expr {
            expr.accept(self);
            if let ast::AST::Expression(expr) = self.temp_result().clone() {
                expr
            } else {
                Expression::None
            }
        } else {
            Expression::None
        };
        // if let ast::AST::Statement(_) = &expr {
        //     return self.temp_result().clone();
        // }
        if let Expression::FunctionCallExpr(e) = &expr {
            let f = e.func().unwrap_or_default();
            if let Expression::TupleOrLocationExpr(f) = f {
                if let TupleOrLocationExpr::LocationExpr(f) = f {
                    if let LocationExpr::IdentifierExpr(f) = f {
                        if f.idf.name() == String::from("require") {
                            assert!(
                                e.args().len() == 1,
                                "Invalid number of arguments for require: {:?},{:?},{:?}",
                                e.args(),
                                ctx.expr,
                                self.code
                            );
                            // raise SyntaxException(f"Invalid number of arguments for require: {e.args}", ctx.expr, self.code)}
                            return ast::AST::Statement(Statement::SimpleStatement(
                                SimpleStatement::RequireStatement(RequireStatement::new(
                                    e.args()[0].clone(),
                                    None,
                                )),
                            ));
                        }
                    }
                }
            }
        }

        ast::AST::Statement(Statement::SimpleStatement(
            SimpleStatement::ExpressionStatement(ExpressionStatement::new(expr)),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::zkay_ast::ast::AST;

    #[test]
    pub fn test_build_ast() {
        let _ = build_ast("");
        assert!(true);
    }
}

// def test_build_ast(self):
//     ast = build_ast(self.example.code())
//     self.assertIsNotNone(ast)

// def test_to_ast_and_back(self):
//     # ast
//     ast = build_ast(self.example.code())
//     # back to string
//     new_code = str(ast)
//     self.assertIn(self.example.name(), new_code)
//     new_code = normalize_code(new_code)
//     # reference
//     reference = normalize_code(self.example.code())
//     # check
//     self.assertEqual(reference, new_code)
