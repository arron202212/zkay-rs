#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr_rust::{
    common_token_stream::CommonTokenStream,
    input_stream::InputStream,
    token::{CommonToken, Token},
    tree::{ParseTree, Visitable},
};
use rccell::RcCell;
use solidity_parser::{
    emit::Emitter,
    generated::{
        soliditylexer::SolidityLexer,
        solidityparser::{
            AllExprContext, AllExprContextAttrs, AndExprContext, AnnotatedTypeNameContext,
            AssignmentExprContext, BitShiftExprContext, BitwiseAndExprContext,
            BitwiseNotExprContext, BitwiseOrExprContext, BitwiseXorExprContext, BlockContext,
            BooleanLiteralExprContext, BreakStatementContext, BreakStatementContextAttrs,
            CompExprContext, ConstructorDefinitionContext, ContinueStatementContext,
            ContinueStatementContextAttrs, ContractDefinitionContext, ContractPartContext,
            ContractPartContextAttrs, DoWhileStatementContext, ElementaryTypeNameContext,
            ElementaryTypeNameExpressionContext, ElementaryTypeNameExpressionContextAttrs,
            EnumDefinitionContext, EnumValueContext, EqExprContext, ExpressionStatementContext,
            ForStatementContext, FunctionCallArgumentsContext, FunctionCallExprContext,
            FunctionDefinitionContext, HomomorphismAnnotationContext, IdentifierContext,
            IdentifierExprContext, IfStatementContext, IndexExprContext, IteExprContext,
            MappingContext, MeExprContext, MeExprContextAttrs, MemberAccessExprContext,
            ModifierContext, ModifierListContext, MultDivModExprContext, NotExprContext,
            NumberLiteralContext, NumberLiteralContextAttrs, NumberLiteralExprContext,
            OrExprContext, ParameterListContext, ParenthesisExprContext, PlusMinusExprContext,
            PostCrementExprContext, PowExprContext, PragmaDirectiveContext,
            PragmaDirectiveContextAttrs, PreCrementExprContext, PrimitiveCastExprContext,
            ReturnParametersContext, ReturnStatementContext, SignExprContext,
            SimpleStatementContext, SimpleStatementContextAttrs, SolidityParser,
            SolidityParserContextType, SourceUnitContext, StateMutabilityContext,
            StateMutabilityContextAttrs, StateVariableDeclarationContext, StatementContext,
            StatementContextAttrs, StringLiteralExprContext, TupleExprContext,
            TupleExpressionContext, TupleExpressionContextAttrs, TypeNameContext,
            TypeNameContextAttrs, UserDefinedTypeNameContext, VariableDeclarationContext,
            VariableDeclarationStatementContext, VersionConstraintContext, VersionContext,
            VersionOperatorContext, VersionPragmaContext, WhileStatementContext,
        },
    },
    parse::MyErrorListener,
};
use zkay_ast::{
    ast::{
        self, is_instance, is_instances, ASTFlatten, ASTType, AddressPayableTypeName,
        AddressTypeName, AllExpr, AnnotatedTypeName, AssignmentStatement, AssignmentStatementBase,
        Block, BoolTypeName, BooleanLiteralExpr, BreakStatement, BuiltinFunction,
        ConstructorOrFunctionDefinition, ContinueStatement, ContractDefinition, ContractTypeName,
        DoWhileStatement, ElementaryTypeName, EnumDefinition, EnumTypeName, EnumValue,
        EnumValueTypeName, Expression, ExpressionStatement, ForStatement, FunctionCallExpr,
        FunctionCallExprBase, FunctionCallExprBaseProperty, Identifier, IdentifierBase,
        IdentifierBaseProperty, IdentifierDeclaration, IdentifierExpr, IdentifierExprUnion,
        IfStatement, IndexExpr, IntTypeName, IntoAST, IntoExpression, LiteralExpr, LocationExpr,
        Mapping, MeExpr, MemberAccessExpr, NamespaceDefinition, NumberLiteralExpr, NumberTypeName,
        Parameter, PrimitiveCastExpr, ReclassifyExpr, ReclassifyExprBase, RehomExpr,
        RequireStatement, ReturnStatement, SimpleStatement, SourceUnit, StateVariableDeclaration,
        Statement, StatementList, StringLiteralExpr, StructTypeName, TupleExpr,
        TupleOrLocationExpr, TypeName, UintTypeName, UserDefinedTypeName, VariableDeclaration,
        VariableDeclarationStatement, WhileStatement, AST,
    },
    homomorphism::{HOMOMORPHISM_STORE, REHOM_EXPRESSIONS},
};

#[macro_export]
macro_rules! _visit_binary_expr {
    ($ctx: expr,$self: expr) => {{
        let op = ($ctx).op.as_ref().unwrap();
        let mut f = BuiltinFunction::new(&op.text);
        f.expression_base.ast_base.borrow_mut().line = op.line as i32;
        f.expression_base.ast_base.borrow_mut().column = op.column as i32;
        let lhs = if let Some(expr) = &($ctx).lhs {
            expr.accept($self);
            if let Some(AST::Expression(expr)) = ($self).temp_result().clone() {
                Some(expr)
            } else {
                None
            }
        } else {
            None
        };
        let rhs = if let Some(expr) = &($ctx).rhs {
            expr.accept($self);
            if let Some(AST::Expression(expr)) = ($self).temp_result().clone() {
                Some(expr)
            } else {
                None
            }
        } else {
            None
        };
        Some(
            FunctionCallExprBase::new(
                RcCell::new(Expression::BuiltinFunction(f)).into(),
                [lhs.unwrap(), rhs.unwrap()]
                    .into_iter()
                    .map(RcCell::new)
                    .map(Into::<ASTFlatten>::into)
                    .collect(),
                Some(0),
            )
            .into_ast(),
        )
    }};
}

#[macro_export]
macro_rules! _visit_bool_expr {
    ($ctx: expr,$self: expr) => {
        _visit_binary_expr!($ctx, $self)
    };
}

pub fn build_ast_from_parse_tree(code: &str) -> Option<ASTFlatten> {
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
    v.temp_result()
        .clone()
        .map(RcCell::new)
        .map(Into::<ASTFlatten>::into)
}

pub fn build_ast(code: &str) -> ASTFlatten {
    let mut full_ast = build_ast_from_parse_tree(code);
    assert!(full_ast.is_some());
    // assert isinstance(full_ast, ast.SourceUnit)
    full_ast
        .as_mut()
        .unwrap()
        .try_as_ast_mut()
        .unwrap()
        .borrow_mut()
        .try_as_source_unit_mut()
        .unwrap()
        .original_code = code.split("\n").map(String::from).collect();
    full_ast.unwrap()
}

struct BuildASTVisitor {
    pub emitter: Emitter,
    pub code: String,
    pub asts: Option<AST>,
}
impl BuildASTVisitor {
    pub fn new(code: String) -> Self {
        Self {
            emitter: Emitter::new(Some(code.clone())),
            code,
            asts: None,
        }
    }
}
// use std::any::{Any, TypeId};
// pub fn is_instance<S: ?Sized + Any, T: ?Sized + Any>(_s: &T) -> bool {
//     TypeId::of::<T>() == TypeId::of::<S>()
// }
// pub fn is_instance<'t,T:'t>(s: &'t dyn Any) -> bool {
//     TypeId::of::<T>() == s.type_id()
// }
pub fn print_type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
use antlr_rust::parser::ParserNodeType;
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::tree::ParseTreeVisitorCompat;
use solidity_parser::generated::solidityvisitor::SolidityVisitorCompat;
// impl<'input> SolidityVisitor<'input> for BuildASTVisitor {
// }
impl<'input> ParseTreeVisitorCompat<'input> for BuildASTVisitor {
    type Node = SolidityParserContextType;
    type Return = Option<AST>;
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
    //             type Return = AST;
    // fn visit(self, tree:ParserNodeType){
    //     let mut sub_ast = self.visit(tree);
    //     if is_instance::<AST>(&sub_ast){
    //         // sub_ast.line = tree.start.line;
    //         // sub_ast.column = tree.start.column + 1;
    //     }
    //     //  sub_ast
    //     }

    // fn  visit_children(self, ctx: ParserRuleContext){
    //     // determine corresponding class name
    //     let mut t = print_type_of(ctx);
    //     t = t.replace("Context", "");
    //     // AST::Identifier
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
        Some(IdentifierBase::new(name.to_string()).into_ast())
    }

    fn visit_pragmaDirective(&mut self, ctx: &PragmaDirectiveContext<'input>) -> Self::Return {
        ctx.pragma().expect("visit_pragmaDirective").accept(self);
        let pragmas = if let Some(AST::VersionPragma(p)) = self.temp_result().clone() {
            p
        } else {
            String::new()
        };
        let s = format!("pragma {pragmas};");
        Some(AST::Pragma(s))
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

        Some(AST::VersionPragma(format!("{name} {version}")))
    }

    // Visit a parse tree produced by SolidityParser#contractDefinition.
    fn visit_contractDefinition(
        &mut self,
        ctx: &ContractDefinitionContext<'input>,
    ) -> Self::Return {
        let idf = if let Some(idf) = &ctx.idf {
            idf.accept(self);
            if let Some(AST::Identifier(a)) = self.temp_result().clone() {
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
                    ////println!("====stateVariableDeclaration======={:?}",self.temp_result().clone());
                    if let Some(AST::IdentifierDeclaration(
                        IdentifierDeclaration::StateVariableDeclaration(_),
                    )) = self.temp_result().clone()
                    {
                        self.temp_result().clone()
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
                    ////println!("====constructorDefinition======={:?}",self.temp_result().clone());

                    if let Some(AST::NamespaceDefinition(
                        NamespaceDefinition::ConstructorOrFunctionDefinition(a),
                    )) = self.temp_result().clone()
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
                    ////println!("==functionDefinition=={:?}==",self.temp_result().clone());
                    if let Some(AST::NamespaceDefinition(
                        NamespaceDefinition::ConstructorOrFunctionDefinition(a),
                    )) = self.temp_result().clone()
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
                    ////println!("==enumDefinition=={:?}==",self.temp_result().clone());
                    if let Some(AST::NamespaceDefinition(NamespaceDefinition::EnumDefinition(a))) =
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

        Some(
            ContractDefinition::new(
                idf.map(RcCell::new),
                state_variable_declarations
                    .into_iter()
                    .map(RcCell::new)
                    .map(Into::<ASTFlatten>::into)
                    .collect(),
                constructor_definitions,
                function_definitions,
                enum_definitions,
                None,
                None,
            )
            .into_ast(),
        )
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
            if let Some(AST::Identifier(a)) = self.temp_result().clone() {
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
                    .filter_map(|param| {
                        param.accept(self);
                        if let Some(AST::IdentifierDeclaration(IdentifierDeclaration::Parameter(
                            a,
                        ))) = self.temp_result().clone()
                        {
                            Some(a)
                        } else {
                            None
                        }
                    })
                    .map(RcCell::new)
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
                    // param.accept(self);
                    // //println!("=====parameter==== self.temp_result()====={:?}", self.temp_result());
                    let annotated_type = if let Some(at) = &param.annotated_type {
                        at.accept(self);
                        self.temp_result()
                            .clone()
                            .unwrap()
                            .try_as_annotated_type_name()
                    } else {
                        None
                    };
                    let idf = if let Some(idf) = &param.idf {
                        idf.accept(self);
                        self.temp_result().clone().unwrap().try_as_identifier()
                    } else {
                        None
                    };
                    let keywords: Vec<_> = param
                        .keywords
                        .iter()
                        .map(|kw| kw.get_text().to_string())
                        .collect();
                    // //println!("===functdefini===Parameter==========={:?},{:?},{:?}",keywords,annotated_type,idf);
                    println!(
                        "==functdefini==keywords.len()==={:?}=={:?}==={}=====",
                        annotated_type,
                        idf,
                        keywords.len()
                    );
                    Parameter::new(
                        keywords,
                        RcCell::new(annotated_type.unwrap()),
                        idf.map(RcCell::new),
                        None,
                    )
                    // if let Some(AST::IdentifierDeclaration(IdentifierDeclaration::Parameter(a))) =
                    //     self.temp_result().clone()
                    // {
                    //     Some(a)
                    // } else {
                    // None
                    // }
                })
                .map(RcCell::new)
                .collect()
        } else {
            vec![]
        });
        // println!("=====parameters=====len===={:?}",parameters.as_ref().unwrap().len());
        let modifiers = Some(if let Some(p) = &ctx.modifiers {
            p.modifiers
                .iter()
                .filter_map(|modifier| {
                    modifier.accept(self);
                    if let Some(AST::Modifier(a)) = self.temp_result().clone() {
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
            if let Some(AST::Statement(Statement::StatementList(StatementList::Block(block)))) =
                self.temp_result().clone()
            {
                Some(block)
            } else {
                None
            }
        } else {
            None
        };
        Some(
            ConstructorOrFunctionDefinition::new(
                idf.map(RcCell::new),
                parameters,
                modifiers,
                return_parameters,
                body,
            )
            .into_ast(),
        )
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
                    if let Some(AST::IdentifierDeclaration(IdentifierDeclaration::Parameter(a))) =
                        self.temp_result().clone()
                    {
                        Some(RcCell::new(a))
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            None
        };
        //  //println!("==visit_constructorDefinition===parameters========={:?}",parameters);
        let modifiers = if let Some(p) = &ctx.modifiers {
            p.modifiers
                .iter()
                .map(|modifier| {
                    modifier.accept(self);
                    if let Some(AST::Modifier(a)) = self.temp_result().clone() {
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
            if let Some(AST::Statement(Statement::StatementList(StatementList::Block(block)))) =
                self.temp_result().clone()
            {
                Some(block)
            } else {
                None
            }
        } else {
            None
        };
        Some(
            ConstructorOrFunctionDefinition::new(
                idf,
                parameters,
                modifiers,
                return_parameters,
                body,
            )
            .into_ast(),
        )
    }

    fn visit_enumDefinition(&mut self, ctx: &EnumDefinitionContext<'input>) -> Self::Return {
        // idf = self.visit(ctx.idf)
        // if "$" in idf.name:
        //     raise SyntaxException("$ is not allowed in zkay enum identifiers", ctx.idf, self.code)
        // values = [self.visit(v) for v in ctx.values]
        // return ast.EnumDefinition(idf, values)
        let idf = if let Some(idf) = &ctx.idf {
            idf.accept(self);
            if let Some(AST::Identifier(a)) = self.temp_result().clone() {
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
                if let Some(AST::EnumValue(a)) = self.temp_result().clone() {
                    a
                } else {
                    EnumValue::new(None)
                }
            })
            .collect();
        Some(EnumDefinition::new(idf.map(RcCell::new), values).into_ast())
    }

    fn visit_enumValue(&mut self, ctx: &EnumValueContext<'input>) -> Self::Return {
        // idf = self.visit(ctx.idf)
        // if "$" in idf.name:
        //     raise SyntaxException("$ is not allowed in zkay enum value identifiers", ctx.idf, self.code)
        // return ast.EnumValue(idf)
        let idf = if let Some(idf) = &ctx.idf {
            idf.accept(self);
            if let Some(AST::Identifier(a)) = self.temp_result().clone() {
                Some(a)
            } else {
                None
            }
        } else {
            None
        };
        Some(EnumValue::new(idf).into_ast())
    }

    // Visit a parse tree produced by SolidityParser#NumberLiteralExpr.
    fn visit_NumberLiteralExpr(&mut self, ctx: &NumberLiteralExprContext<'input>) -> Self::Return {
        // v = int(ctx.getText().replace("_", ""), 0)
        // return NumberLiteralExpr(v, ctx.getText().startswith(("0x", "0X")))
        let s = ctx.get_text();
        let v = s.replace("_", "").parse().unwrap_or(0);
        Some(NumberLiteralExpr::new(v, s.starts_with("0x") || s.starts_with("0X")).into_ast())
    }

    // Visit a parse tree produced by SolidityParser#BooleanLiteralExpr.
    fn visit_BooleanLiteralExpr(
        &mut self,
        ctx: &BooleanLiteralExprContext<'input>,
    ) -> Self::Return {
        //   b = ctx.getText() == "true"
        // return BooleanLiteralExpr(b)
        Some(BooleanLiteralExpr::new(ctx.get_text() == String::from("true")).into_ast())
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
        Some(StringLiteralExpr::new(s).into_ast())
    }

    fn visit_TupleExpr(&mut self, ctx: &TupleExprContext<'input>) -> Self::Return {
        let mut contents = if let Some(e) = &ctx.expr {
            use solidity_parser::generated::solidityparser::TupleExpressionContextAttrs;
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
            if let Some(AST::Expression(expr)) = self.temp_result().clone() {
                elements.push(expr);
            }
        }
        // contents = ctx.expr.children[1:-1]
        // elements = []
        // for idx in range(0, len(contents), 2){
        //     elements.append(self.visit(contents[idx]))
        // return ast.TupleExpr(elements)
        Some(
            TupleExpr::new(
                elements
                    .into_iter()
                    .map(RcCell::new)
                    .map(Into::<ASTFlatten>::into)
                    .collect(),
            )
            .into_ast(),
        )
    }

    fn visit_modifier(&mut self, ctx: &ModifierContext<'input>) -> Self::Return {
        //  ctx.getText()
        Some(AST::Modifier(ctx.get_text()))
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
            // ////println!("privacy_annotation==={:?},===={:?}", pa, self.temp_result().clone());
            privacy_annotation = if let Some(AST::Expression(expr)) = self.temp_result().clone() {
                Some(expr)
            } else {
                None
            };
            if let Some(hom) = &ctx.homomorphism {
                hom.accept(self);
                if let Some(AST::Homomorphism(hom)) = self.temp_result().clone() {
                    homomorphism = hom;
                }
            }
            assert!(
                privacy_annotation.is_some()
                    && is_instances(
                        privacy_annotation.as_ref().unwrap(),
                        vec![ASTType::AllExpr, ASTType::MeExpr, ASTType::IdentifierExpr]
                    ),
                "Privacy annotation can only be me | all | Identifier,{:?},{:?}",
                privacy_annotation,
                self.code
            );
            assert!(
                !(is_instance(privacy_annotation.as_ref().unwrap(), ASTType::AllExpr)
                    && homomorphism != String::from("NON_HOMOMORPHIC")),
                "Public types cannot be homomorphic,{:?},{:?}",
                homomorphism,
                self.code
            );
        }
        // ////println!("======{:?},{:?}",ctx,ctx.type_name);
        let type_name = if let Some(tn) = &ctx.type_name {
            tn.accept(self);
            // ////println!("=type_name=={:?},{:?}",tn,self.temp_result().clone());
            if let Some(AST::TypeName(tn)) = self.temp_result().clone() {
                Some(tn)
            } else {
                None
            }
        } else {
            None
        };
        Some(
            AnnotatedTypeName::new(
                type_name.map(RcCell::new),
                privacy_annotation
                    .map(RcCell::new)
                    .map(Into::<ASTFlatten>::into),
                homomorphism,
            )
            .into_ast(),
        )
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
            Some(AST::Homomorphism(v.value.clone()))
        } else {
            assert!(
                false,
                "Unsupported homomorphism {t},{:?},{:?}",
                ctx, self.code
            );
            None
        }
    }

    fn visit_elementaryTypeName(
        &mut self,
        ctx: &ElementaryTypeNameContext<'input>,
    ) -> Self::Return {
        // ////println!("===========ctx.get_text()==={}==========",ctx.get_text());
        let t = ctx.get_text();
        match t.as_str() {
            "address" => Some(AddressTypeName::new().into_ast()),
            "address payable" => Some(AddressPayableTypeName::new().into_ast()),
            "bool" => Some(BoolTypeName::new().into_ast()),
            _ts if t.starts_with("int") => Some(IntTypeName::new(t).into_ast()),
            _ts if t.starts_with("uint") => Some(UintTypeName::new(t).into_ast()),
            "var" => {
                assert!(
                    false,
                    "Use of unsupported var keyword,{:?},{:?}",
                    ctx, self.code
                );
                None
            }
            _ => {
                assert!(
                    false,
                    "Use of unsupported type {t},{:?},{:?}",
                    ctx, self.code
                );
                None
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
            if let Some(AST::Expression(Expression::TupleOrLocationExpr(
                TupleOrLocationExpr::LocationExpr(expr),
            ))) = self.temp_result().clone()
            {
                Some(expr)
            } else {
                None
            }
        } else {
            None
        };
        let index = if let Some(index) = &ctx.index {
            index.accept(self);
            if let Some(AST::Expression(expr)) = self.temp_result().clone() {
                Some(RcCell::new(expr))
            } else {
                None
            }
        } else {
            None
        };

        Some(IndexExpr::new(arr, index.unwrap().into()).into_ast())
    }

    fn visit_ParenthesisExpr(&mut self, ctx: &ParenthesisExprContext<'input>) -> Self::Return {
        // f = BuiltinFunction("parenthesis").override(line=ctx.start.line, column=ctx.start.column)
        // expr = self.visit(ctx.expr)
        // return FunctionCallExpr(f, [expr])
        let mut f = BuiltinFunction::new("parenthesis");
        f.expression_base.ast_base.borrow_mut().line = ctx.start().line as i32;
        f.expression_base.ast_base.borrow_mut().column = ctx.start().column as i32;
        let expr = if let Some(expr) = &ctx.expr {
            expr.accept(self);
            if let Some(AST::Expression(expr)) = self.temp_result().clone() {
                Some(expr)
            } else {
                None
            }
        } else {
            None
        };
        Some(
            FunctionCallExprBase::new(
                RcCell::new(Expression::BuiltinFunction(f)).into(),
                vec![RcCell::new(expr.unwrap()).into()],
                Some(0),
            )
            .into_ast(),
        )
    }

    fn visit_SignExpr(&mut self, ctx: &SignExprContext<'input>) -> Self::Return {
        // f = BuiltinFunction("sign" + ctx.op.text).override(line=ctx.op.line, column=ctx.op.column)
        // expr = self.visit(ctx.expr)
        // return FunctionCallExpr(f, [expr])
        // ////println!("{:?},========{:?}",1,ctx.op.as_ref().unwrap());
        let op = ctx.op.as_ref().unwrap();
        // ////println!("{:?},========{:?}",2,op.text);
        let mut f = BuiltinFunction::new(("sign".to_string() + &op.text).as_str());
        f.expression_base.ast_base.borrow_mut().line = op.line as i32;
        f.expression_base.ast_base.borrow_mut().column = op.column as i32;
        let expr = if let Some(expr) = &ctx.expr {
            expr.accept(self);
            if let Some(AST::Expression(expr)) = self.temp_result().clone() {
                Some(expr)
            } else {
                None
            }
        } else {
            None
        };
        Some(
            FunctionCallExprBase::new(
                RcCell::new(Expression::BuiltinFunction(f)).into(),
                vec![RcCell::new(expr.unwrap()).into()],
                Some(0),
            )
            .into_ast(),
        )
    }

    fn visit_NotExpr(&mut self, ctx: &NotExprContext<'input>) -> Self::Return {
        // f = BuiltinFunction("!").override(line=ctx.start.line, column=ctx.start.column)
        // expr = self.visit(ctx.expr)
        // return FunctionCallExpr(f, [expr])
        let mut f = BuiltinFunction::new("!");
        f.expression_base.ast_base.borrow_mut().line = ctx.start().line as i32;
        f.expression_base.ast_base.borrow_mut().column = ctx.start().column as i32;
        let expr = if let Some(expr) = &ctx.expr {
            expr.accept(self);
            if let Some(AST::Expression(expr)) = self.temp_result().clone() {
                Some(expr)
            } else {
                None
            }
        } else {
            None
        };
        Some(
            FunctionCallExprBase::new(
                RcCell::new(Expression::BuiltinFunction(f)).into(),
                vec![RcCell::new(expr.unwrap()).into()],
                Some(0),
            )
            .into_ast(),
        )
    }

    fn visit_BitwiseNotExpr(&mut self, ctx: &BitwiseNotExprContext<'input>) -> Self::Return {
        // f = BuiltinFunction("~").override(line=ctx.start.line, column=ctx.start.column)
        // expr = self.visit(ctx.expr)
        // return FunctionCallExpr(f, [expr])
        let mut f = BuiltinFunction::new("~");
        f.expression_base.ast_base.borrow_mut().line = ctx.start().line as i32;
        f.expression_base.ast_base.borrow_mut().column = ctx.start().column as i32;
        let expr = if let Some(expr) = &ctx.expr {
            expr.accept(self);
            if let Some(AST::Expression(expr)) = self.temp_result().clone() {
                Some(expr)
            } else {
                None
            }
        } else {
            None
        };
        Some(
            FunctionCallExprBase::new(
                RcCell::new(Expression::BuiltinFunction(f)).into(),
                vec![RcCell::new(expr.unwrap()).into()],
                Some(0),
            )
            .into_ast(),
        )
    }

    //     fn  _visitBinaryExpr(self, ctx){
    //         // lhs = self.visit(ctx.lhs)
    //         // rhs = self.visit(ctx.rhs)
    //         // f = BuiltinFunction(ctx.op.text).override(line=ctx.op.line, column=ctx.op.column)
    //         // return FunctionCallExpr(f, [lhs, rhs])
    //    AST::None
    //  }

    //     fn  _visitBoolExpr(self, ctx){
    //         // return self._visitBinaryExpr(ctx)
    //         AST::None
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
            if let Some(AST::Expression(expr)) = self.temp_result().clone() {
                Some(expr)
            } else {
                None
            }
        } else {
            None
        };
        let then_expr = if let Some(expr) = &ctx.then_expr {
            expr.accept(self);
            if let Some(AST::Expression(expr)) = self.temp_result().clone() {
                Some(expr)
            } else {
                None
            }
        } else {
            None
        };
        let else_expr = if let Some(expr) = &ctx.else_expr {
            expr.accept(self);
            if let Some(AST::Expression(expr)) = self.temp_result().clone() {
                Some(expr)
            } else {
                None
            }
        } else {
            None
        };
        Some(
            FunctionCallExprBase::new(
                RcCell::new(Expression::BuiltinFunction(f)).into(),
                [cond.unwrap(), then_expr.unwrap(), else_expr.unwrap()]
                    .into_iter()
                    .map(RcCell::new)
                    .map(Into::<ASTFlatten>::into)
                    .collect(),
                Some(0),
            )
            .into_ast(),
        )
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
            if let Some(AST::Expression(expr)) = self.temp_result().clone() {
                // ////println!("{:?},==={:?}",expr,self.temp_result().clone()  );
                Some(expr)
            } else {
                None
            }
        } else {
            None
        };
        let args = if let Some(args) = &ctx.args {
            args.exprs
                .iter()
                .filter_map(|expr| {
                    expr.accept(self);
                    if let Some(AST::Expression(expr)) = self.temp_result().clone() {
                        Some(expr)
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            vec![]
        };
        if let Some(Expression::TupleOrLocationExpr(TupleOrLocationExpr::LocationExpr(
            LocationExpr::IdentifierExpr(func),
        ))) = &func
        {
            //    ////println!("{:?},==0000={:?}",func.idf.name(),REHOM_EXPRESSIONS.lock().unwrap() );
            if func.idf.as_ref().unwrap().borrow().name() == "reveal" {
                assert!(
                    args.len() == 2,
                    "Invalid number of arguments for reveal: {args:?},{:?},{:?}",
                    ctx.args,
                    self.code
                );
                // raise SyntaxException(f"Invalid number of arguments for reveal: {args}", ctx.args, self.code)
                return Some(
                    ReclassifyExprBase::new(
                        RcCell::new(args[0].clone()).into(),
                        RcCell::new(args[1].clone()).into(),
                        None,
                    )
                    .into_ast(),
                );
            } else if let Some(homomorphism) = REHOM_EXPRESSIONS
                .lock()
                .unwrap()
                .get(&func.idf.as_ref().unwrap().borrow().name())
            {
                assert!(
                    args.len() == 1,
                    "Invalid number of arguments for {:?}: {args:?},{:?},{:?}",
                    func,
                    ctx.args,
                    self.code
                );
                // raise SyntaxException(f"Invalid number of arguments for {name}: {args}", ctx.args, self.code)
                return Some(
                    RehomExpr::new(
                        RcCell::new(args[0].clone()).into(),
                        Some(homomorphism.value.clone()),
                    )
                    .into_ast(),
                );
            }
        }
        Some(
            FunctionCallExprBase::new(
                RcCell::new(func.unwrap()).into(),
                args.into_iter()
                    .map(RcCell::new)
                    .map(Into::<ASTFlatten>::into)
                    .collect(),
                Some(0),
            )
            .into_ast(),
        )
    }

    fn visit_ifStatement(&mut self, ctx: &IfStatementContext<'input>) -> Self::Return {
        let cond = if let Some(condition) = &ctx.condition {
            condition.accept(self);
            if let Some(AST::Expression(expr)) = self.temp_result().clone() {
                Some(expr)
            } else {
                None
            }
        } else {
            None
        };
        // ////println!("={:?}=then_branch=={:?}", ctx.then_branch, 1);

        let then_branch = ctx.then_branch.as_ref().unwrap();
        then_branch.accept(self);
        // ////println!(
        //     "={:?}=then_branch=={:?}",
        //     ctx.then_branch,
        //     self.temp_result().clone()
        // );
        let then_branch = if !is_instance(self.temp_result().as_ref().unwrap(), ASTType::Block) {
            Some(Block::new(
                vec![RcCell::new(self.temp_result().clone().unwrap()).into()],
                true,
            ))
        } else {
            self.temp_result()
                .clone()
                .unwrap()
                .try_as_statement()
                .unwrap()
                .try_as_statement_list()
                .unwrap()
                .try_as_block()
        };

        let else_branch = if let Some(expr) = &ctx.else_branch {
            expr.accept(self);
            if let Some(AST::Statement(Statement::StatementList(StatementList::Block(expr)))) =
                self.temp_result().clone()
            {
                Some(expr)
            } else if let Some(AST::Statement(expr)) = self.temp_result().clone() {
                Some(Block::new(vec![RcCell::new(expr.to_ast()).into()], true))
            } else {
                None
            }
        } else {
            None
        };
        Some(
            IfStatement::new(
                RcCell::new(cond.unwrap()).into(),
                then_branch.unwrap(),
                else_branch,
            )
            .into_ast(),
        )
    }

    fn visit_whileStatement(&mut self, ctx: &WhileStatementContext<'input>) -> Self::Return {
        // cond = self.visit(ctx.condition)
        // body = self.visit(ctx.body)
        // if not isinstance(body, ast.Block){
        //     body = ast.Block([body], was_single_statement=True)
        // return ast.WhileStatement(cond, body)
        let cond = if let Some(expr) = &ctx.condition {
            expr.accept(self);
            if let Some(AST::Expression(expr)) = self.temp_result().clone() {
                Some(expr)
            } else {
                None
            }
        } else {
            None
        };
        let body = if let Some(expr) = &ctx.body {
            expr.accept(self);
            if let Some(AST::Statement(Statement::StatementList(StatementList::Block(expr)))) =
                self.temp_result().clone()
            {
                Some(expr)
            } else if let Some(AST::Statement(expr)) = self.temp_result().clone() {
                Some(Block::new(vec![RcCell::new(expr.to_ast()).into()], true))
            } else {
                None
            }
        } else {
            None
        };

        Some(WhileStatement::new(RcCell::new(cond.unwrap()).into(), body.unwrap()).into_ast())
    }

    fn visit_doWhileStatement(&mut self, ctx: &DoWhileStatementContext<'input>) -> Self::Return {
        // body = self.visit(ctx.body)
        // cond = self.visit(ctx.condition)
        // if not isinstance(body, ast.Block){
        //     body = ast.Block([body], was_single_statement=True)
        // return ast.DoWhileStatement(body, cond)
        let body = if let Some(expr) = &ctx.body {
            expr.accept(self);
            if let Some(AST::Statement(Statement::StatementList(StatementList::Block(expr)))) =
                self.temp_result().clone()
            {
                Some(expr)
            } else if let Some(AST::Statement(expr)) = self.temp_result().clone() {
                Some(Block::new(vec![RcCell::new(expr.to_ast()).into()], true))
            } else {
                None
            }
        } else {
            None
        };
        let cond = if let Some(expr) = &ctx.condition {
            expr.accept(self);
            if let Some(AST::Expression(expr)) = self.temp_result().clone() {
                Some(expr)
            } else {
                None
            }
        } else {
            None
        };

        Some(DoWhileStatement::new(body.unwrap(), RcCell::new(cond.unwrap()).into()).into_ast())
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
            if let Some(AST::Statement(Statement::SimpleStatement(expr))) =
                self.temp_result().clone()
            {
                Some(expr)
            } else {
                None
            }
        } else {
            None
        };
        let cond = if let Some(expr) = &ctx.condition {
            expr.accept(self);
            if let Some(AST::Expression(expr)) = self.temp_result().clone() {
                Some(expr)
            } else {
                None
            }
        } else {
            None
        };
        let update = if let Some(expr) = &ctx.update {
            expr.accept(self);
            if let Some(AST::Expression(expr)) = self.temp_result().clone() {
                Some(SimpleStatement::ExpressionStatement(
                    ExpressionStatement::new(RcCell::new(expr).into()),
                ))
            } else {
                None
            }
        } else {
            None
        };
        let body = if let Some(expr) = &ctx.body {
            expr.accept(self);
            if let Some(AST::Statement(Statement::StatementList(StatementList::Block(expr)))) =
                self.temp_result().clone()
            {
                Some(expr)
            } else if let Some(AST::Statement(expr)) = self.temp_result().clone() {
                Some(Block::new(vec![RcCell::new(expr.to_ast()).into()], true))
            } else {
                None
            }
        } else {
            None
        };

        Some(AST::Statement(Statement::ForStatement(ForStatement::new(
            init,
            RcCell::new(cond.unwrap()).into(),
            update,
            body.unwrap(),
        ))))
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
            if let Some(AST::Expression(expr)) = self.temp_result().clone() {
                Some(expr)
            } else {
                None
            }
        } else {
            None
        };
        let mut rhs = if let Some(expr) = &ctx.rhs {
            expr.accept(self);
            if let Some(AST::Expression(expr)) = self.temp_result().clone() {
                Some(expr)
            } else {
                None
            }
        } else {
            None
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
                f.expression_base.ast_base.borrow_mut().line = line;
                f.expression_base.ast_base.borrow_mut().column = column;
                let mut fce = FunctionCallExprBase::new(
                    RcCell::new(Expression::BuiltinFunction(f)).into(),
                    [lhs.clone().unwrap(), rhs.unwrap()]
                        .into_iter()
                        .map(RcCell::new)
                        .map(Into::<ASTFlatten>::into)
                        .collect(),
                    Some(0),
                );
                fce.expression_base.ast_base.borrow_mut().line = line;
                fce.expression_base.ast_base.borrow_mut().column = column + 1;
                rhs = Some(Expression::FunctionCallExpr(
                    FunctionCallExpr::FunctionCallExpr(fce),
                ));
            }
            op
        } else {
            // assert!(false);
            String::new()
        };
        let lhs = lhs.map(|l| l.into_ast());

        Some(
            AssignmentStatementBase::new(
                lhs.map(RcCell::new).map(Into::<ASTFlatten>::into),
                rhs.map(RcCell::new).map(Into::<ASTFlatten>::into),
                Some(op),
            )
            .into_ast(),
        )
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
    //    AST::None
    //      }

    fn visit_PreCrementExpr(&mut self, ctx: &PreCrementExprContext<'input>) -> Self::Return {
        // return self._handle_crement_expr(ctx, "pre")
        let kind = "pre";
        let mut expr = if let Some(expr) = &ctx.expr {
            expr.accept(self);
            if let Some(AST::Expression(expr)) = self.temp_result().clone() {
                Some(expr)
            } else {
                None
            }
        } else {
            None
        };
        if let Some(op) = &ctx.op {
            let (line, column) = (op.line as i32, op.column as i32);
            let optext = String::from(if op.text == String::from("++") {
                "+"
            } else {
                "-"
            });
            let mut one = NumberLiteralExpr::new(1, false);
            one.literal_expr_base
                .expression_base
                .ast_base
                .borrow_mut()
                .line = line;
            one.literal_expr_base
                .expression_base
                .ast_base
                .borrow_mut()
                .column = column + 1;
            let mut f = BuiltinFunction::new(&optext);
            f.expression_base.ast_base.borrow_mut().line = line;
            f.expression_base.ast_base.borrow_mut().column = column;
            let mut fce = FunctionCallExprBase::new(
                RcCell::new(Expression::BuiltinFunction(f)).into(),
                [
                    expr.clone().unwrap(),
                    Expression::LiteralExpr(LiteralExpr::NumberLiteralExpr(one)),
                ]
                .into_iter()
                .map(RcCell::new)
                .map(Into::<ASTFlatten>::into)
                .collect(),
                Some(0),
            );
            fce.expression_base.ast_base.borrow_mut().line = line;
            fce.expression_base.ast_base.borrow_mut().column = column + 1;

            let expr = expr.map(|e| e.to_ast());

            Some(
                AssignmentStatementBase::new(
                    expr.map(RcCell::new).map(Into::<ASTFlatten>::into),
                    Some(
                        RcCell::new(Expression::FunctionCallExpr(
                            FunctionCallExpr::FunctionCallExpr(fce),
                        ))
                        .into(),
                    ),
                    Some(format!("{kind}{}", op.text)),
                )
                .into_ast(),
            )
        } else {
            None
        }
    }

    fn visit_PostCrementExpr(&mut self, ctx: &PostCrementExprContext<'input>) -> Self::Return {
        // return self._handle_crement_expr(ctx, "post")
        let kind = "post";
        let mut expr = if let Some(expr) = &ctx.expr {
            expr.accept(self);
            if let Some(AST::Expression(expr)) = self.temp_result().clone() {
                Some(expr)
            } else {
                None
            }
        } else {
            None
        };
        if let Some(op) = &ctx.op {
            let (line, column) = (op.line as i32, op.column as i32);
            let optext = String::from(if op.text == String::from("++") {
                "+"
            } else {
                "-"
            });
            let mut one = NumberLiteralExpr::new(1, false);
            one.literal_expr_base
                .expression_base
                .ast_base
                .borrow_mut()
                .line = line;
            one.literal_expr_base
                .expression_base
                .ast_base
                .borrow_mut()
                .column = column + 1;
            let mut f = BuiltinFunction::new(&optext);
            f.expression_base.ast_base.borrow_mut().line = line;
            f.expression_base.ast_base.borrow_mut().column = column;
            let mut fce = FunctionCallExprBase::new(
                RcCell::new(Expression::BuiltinFunction(f)).into(),
                [
                    expr.clone().unwrap(),
                    Expression::LiteralExpr(LiteralExpr::NumberLiteralExpr(one)),
                ]
                .into_iter()
                .map(RcCell::new)
                .map(Into::<ASTFlatten>::into)
                .collect(),
                Some(0),
            );
            fce.expression_base.ast_base.borrow_mut().line = line;
            fce.expression_base.ast_base.borrow_mut().column = column + 1;

            let expr = expr.map(|e| e.into_ast());
            Some(
                AssignmentStatementBase::new(
                    expr.map(RcCell::new).map(Into::<ASTFlatten>::into),
                    Some(RcCell::new(fce.to_expr()).into()),
                    Some(format!("{kind}{}", op.text)),
                )
                .into_ast(),
            )
        } else {
            None
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
        // ////println!("{:?}",ctx.expr );
        let mut expression = None;
        if let Some(expr) = &ctx.expr {
            expr.accept(self);
            if let Some(AST::Statement(_)) = self.temp_result() {
                //  ////println!("==self.temp_result().clone()======={:?}",self.temp_result().clone());
                return self.temp_result().clone();
            }
            if let Some(AST::Expression(expr)) = self.temp_result().clone() {
                expression = Some(expr);
            }
        }
        // if let AST::Statement(_) = &expr {
        //     return self.temp_result().clone();
        // }
        if let Some(Expression::FunctionCallExpr(e)) = &expression {
            if let Some(f) = e
                .func()
                .clone()
                .try_as_expression_ref()
                .unwrap()
                .borrow()
                .try_as_tuple_or_location_expr_ref()
                .unwrap()
                .try_as_location_expr_ref()
                .unwrap()
                .try_as_identifier_expr_ref()
            {
                if f.idf.as_ref().unwrap().borrow().name() == "require" {
                    assert!(
                        e.args().len() == 1,
                        "Invalid number of arguments for require: {:?},{:?},{:?}",
                        e.args(),
                        ctx.expr,
                        self.code
                    );
                    // raise SyntaxException(f"Invalid number of arguments for require: {e.args}", ctx.expr, self.code)}
                    return Some(RequireStatement::new(e.args()[0].clone(), None).to_ast());
                }
            }
        }

        Some(ExpressionStatement::new(RcCell::new(expression.unwrap()).into()).into_ast())
    }

    fn visit_sourceUnit(&mut self, ctx: &SourceUnitContext<'input>) -> Self::Return {
        let pragma_directive = if let Some(pd) = &ctx.pragma_directive {
            pd.accept(self);
            if let Some(AST::Pragma(s)) = self.temp_result().clone() {
                s
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        let contracts = ctx
            .contracts
            .iter()
            .filter_map(|contract| {
                contract.accept(self);
                self.temp_result().clone().map(|c| {
                    c.try_as_namespace_definition()
                        .unwrap()
                        .try_as_contract_definition()
                        .unwrap()
                })
            })
            .collect();
        Some(SourceUnit::new(pragma_directive, contracts, None).into_ast())
    }

    fn visit_version(&mut self, ctx: &VersionContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_versionOperator(&mut self, ctx: &VersionOperatorContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_versionConstraint(&mut self, ctx: &VersionConstraintContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_contractPart(&mut self, ctx: &ContractPartContext<'input>) -> Self::Return {
        if let Some(statement) = ctx.stateVariableDeclaration() {
            statement.accept(self);

            return self.temp_result().clone();
        }
        if let Some(statement) = ctx.constructorDefinition() {
            statement.accept(self);
            return self.temp_result().clone();
        }
        if let Some(statement) = ctx.functionDefinition() {
            statement.accept(self);
            return self.temp_result().clone();
        }
        if let Some(statement) = ctx.enumDefinition() {
            statement.accept(self);
            return self.temp_result().clone();
        }

        None
    }

    fn visit_stateVariableDeclaration(
        &mut self,
        ctx: &StateVariableDeclarationContext<'input>,
    ) -> Self::Return {
        let annotated_type = if let Some(at) = &ctx.annotated_type {
            at.accept(self);
            self.temp_result()
                .clone()
                .unwrap()
                .try_as_annotated_type_name()
        } else {
            None
        };
        let idf = if let Some(idf) = &ctx.idf {
            idf.accept(self);
            self.temp_result().clone().unwrap().try_as_identifier()
        } else {
            None
        };
        let expr = if let Some(expr) = &ctx.expr {
            expr.accept(self);
            if let Some(AST::Expression(e)) = self.temp_result() {
                Some(e.clone())
            } else {
                None
            }
        } else {
            None
        };
        let keywords: Vec<_> = ctx.keywords.iter().map(|kw| kw.to_string()).collect();
        Some(
            StateVariableDeclaration::new(
                RcCell::new(annotated_type.unwrap()),
                keywords,
                idf.map(RcCell::new),
                expr.map(RcCell::new).map(Into::<ASTFlatten>::into),
            )
            .into_ast(),
        )
    }

    fn visit_returnParameters(&mut self, ctx: &ReturnParametersContext<'input>) -> Self::Return {
        //TODO vec
        let rp: Vec<_> = ctx
            .return_parameters
            .iter()
            .map(|p| {
                p.accept(self);
                self.temp_result().clone()
            })
            .collect();
        if rp.is_empty() {
            None
        } else {
            rp[0].clone()
        }
    }

    fn visit_modifierList(&mut self, ctx: &ModifierListContext<'input>) -> Self::Return {
        //TODO vec
        let rp: Vec<_> = ctx
            .modifiers
            .iter()
            .map(|p| {
                p.accept(self);
                self.temp_result().clone()
            })
            .collect();
        if rp.is_empty() {
            None
        } else {
            rp[0].clone()
        }
    }

    fn visit_parameterList(&mut self, ctx: &ParameterListContext<'input>) -> Self::Return {
        //TODO vec
        let rp: Vec<_> = ctx
            .params
            .iter()
            .map(|p| {
                let annotated_type = if let Some(at) = &p.annotated_type {
                    at.accept(self);
                    self.temp_result()
                        .clone()
                        .unwrap()
                        .try_as_annotated_type_name()
                } else {
                    None
                };
                let idf = if let Some(idf) = &p.idf {
                    idf.accept(self);
                    self.temp_result().clone().unwrap().try_as_identifier()
                } else {
                    None
                };
                let keywords: Vec<_> = p.keywords.iter().map(|kw| kw.to_string()).collect();
                // //println!("{:?},{:?},{:?}",keywords,annotated_type,idf);

                (keywords, annotated_type, idf)
            })
            .collect();
        // //println!("=={:?}==ctx
        //     .params======={:?}============",ctx.parameter,ctx
        //     .params);
        // if rp.is_empty() {
        None
        // } else {
        //     rp[0].clone()
        // }
    }
    fn visit_variableDeclaration(
        &mut self,
        ctx: &VariableDeclarationContext<'input>,
    ) -> Self::Return {
        let annotated_type = if let Some(at) = &ctx.annotated_type {
            at.accept(self);
            self.temp_result()
                .clone()
                .unwrap()
                .try_as_annotated_type_name()
        } else {
            None
        };
        let idf = if let Some(idf) = &ctx.idf {
            idf.accept(self);
            self.temp_result().clone().unwrap().try_as_identifier()
        } else {
            None
        };
        let keywords: Vec<_> = ctx.keywords.iter().map(|kw| kw.to_string()).collect();
        Some(
            VariableDeclaration::new(
                keywords,
                RcCell::new(annotated_type.unwrap()),
                idf.map(RcCell::new),
                None,
            )
            .into_ast(),
        )
    }
    fn visit_typeName(&mut self, ctx: &TypeNameContext<'input>) -> Self::Return {
        if let Some(statement) = ctx.elementaryTypeName() {
            statement.accept(self);

            return self.temp_result().clone();
        }
        if let Some(statement) = ctx.userDefinedTypeName() {
            statement.accept(self);
            return self.temp_result().clone();
        }
        if let Some(statement) = ctx.mapping() {
            statement.accept(self);
            return self.temp_result().clone();
        }
        None
    }

    fn visit_userDefinedTypeName(
        &mut self,
        ctx: &UserDefinedTypeNameContext<'input>,
    ) -> Self::Return {
        let identifier = if let Some(identifier) = &ctx.identifier {
            identifier.accept(self);
            self.temp_result().clone().unwrap().try_as_identifier()
        } else {
            None
        };
        let names: Vec<_> = ctx
            .names
            .iter()
            .map(|name| {
                name.accept(self);
                self.temp_result()
                    .clone()
                    .unwrap()
                    .try_as_identifier()
                    .unwrap()
            })
            .collect();
        match identifier.unwrap().name().as_str() {
            "enum" => Some(EnumTypeName::new(names, None).into_ast()),
            "enum value" => Some(EnumValueTypeName::new(names, None).into_ast()),
            "struct" => Some(StructTypeName::new(names, None).into_ast()),
            "constract" => Some(ContractTypeName::new(names, None).into_ast()),
            "address" => Some(AddressTypeName::new().into_ast()),
            "address payable" => Some(AddressPayableTypeName::new().into_ast()),
            _ => None,
        }
    }

    fn visit_mapping(&mut self, ctx: &MappingContext<'input>) -> Self::Return {
        let key_type = if let Some(key_type) = &ctx.key_type {
            key_type.accept(self);
            // ////println!("======{:?}===ctx.key_type========{:?}", self.temp_result(),ctx.key_type);
            self.temp_result().clone().unwrap().try_as_type_name()
        } else {
            None
        };
        assert!(key_type.is_some(), "key_type is none");
        let key_label = if let Some(key_label) = &ctx.key_label {
            key_label.accept(self);
            self.temp_result().clone().unwrap().try_as_identifier()
        } else {
            None
        };
        let value_type = if let Some(value_type) = &ctx.value_type {
            value_type.accept(self);
            self.temp_result()
                .clone()
                .unwrap()
                .try_as_annotated_type_name()
        } else {
            None
        };
        assert!(value_type.is_some(), "value_type is none");
        Some(Mapping::new(key_type.unwrap(), key_label, value_type.unwrap()).into_ast())
    }

    fn visit_stateMutability(&mut self, ctx: &StateMutabilityContext<'input>) -> Self::Return {
        if ctx.PayableKeyword().is_some() {
            return None;
        }
        if ctx.PureKeyword().is_some() {
            return None;
        }
        if ctx.ViewKeyword().is_some() {
            return None;
        }
        None
    }

    fn visit_block(&mut self, ctx: &BlockContext<'input>) -> Self::Return {
        // if let Some(statement) = &ctx.statement {
        //     statement.accept(self);
        //     ////println!("==========statement==========BlockContext==={:?}",self.temp_result().clone());
        //     Some(Block::new(vec![self.temp_result().clone().unwrap()], true).into_ast())
        // } else {
        let statements: Vec<_> = ctx
            .statements
            .iter()
            .filter_map(|statement| {
                statement.accept(self);
                // ////println!("====================BlockContext==={:?}",self.temp_result().clone());
                self.temp_result().clone()
            })
            .collect();
        let was_single_statement = statements.len() == 1;
        Some(
            Block::new(
                statements
                    .into_iter()
                    .map(RcCell::new)
                    .map(Into::<ASTFlatten>::into)
                    .collect(),
                was_single_statement,
            )
            .into_ast(),
        )
        // }
    }

    fn visit_statement(&mut self, ctx: &StatementContext<'input>) -> Self::Return {
        if let Some(statement) = ctx.ifStatement() {
            statement.accept(self);

            return self.temp_result().clone();
        }
        if let Some(statement) = ctx.whileStatement() {
            statement.accept(self);
            return self.temp_result().clone();
        }
        if let Some(statement) = ctx.forStatement() {
            statement.accept(self);
            return self.temp_result().clone();
        }
        if let Some(statement) = ctx.block() {
            statement.accept(self);
            return self.temp_result().clone();
        }
        if let Some(statement) = ctx.doWhileStatement() {
            statement.accept(self);
            return self.temp_result().clone();
        }
        if let Some(statement) = ctx.continueStatement() {
            statement.accept(self);
            return self.temp_result().clone();
        }
        if let Some(statement) = ctx.breakStatement() {
            statement.accept(self);
            return self.temp_result().clone();
        }
        if let Some(statement) = ctx.returnStatement() {
            statement.accept(self);
            return self.temp_result().clone();
        }
        if let Some(statement) = ctx.simpleStatement() {
            statement.accept(self);
            return self.temp_result().clone();
        }
        None
    }

    fn visit_simpleStatement(&mut self, ctx: &SimpleStatementContext<'input>) -> Self::Return {
        if let Some(statement) = ctx.variableDeclarationStatement() {
            statement.accept(self);
            ////println!("==variableDeclarationStatement======={:?}===========",self.temp_result().clone());
            return self.temp_result().clone();
        }
        if let Some(statement) = ctx.expressionStatement() {
            statement.accept(self);
            ////println!("==expressionStatement======={:?}===========",self.temp_result().clone());
            return self.temp_result().clone();
        }
        None
    }

    fn visit_continueStatement(&mut self, ctx: &ContinueStatementContext<'input>) -> Self::Return {
        if ctx.ContinueKeyword().is_some() {
            Some(ContinueStatement::new().into_ast())
        } else {
            None
        }
    }

    fn visit_breakStatement(&mut self, ctx: &BreakStatementContext<'input>) -> Self::Return {
        if ctx.BreakKeyword().is_some() {
            Some(BreakStatement::new().into_ast())
        } else {
            None
        }
    }

    fn visit_returnStatement(&mut self, ctx: &ReturnStatementContext<'input>) -> Self::Return {
        let expr = if let Some(expr) = &ctx.expr {
            expr.accept(self);
            if let Some(AST::Expression(e)) = self.temp_result() {
                Some(e.clone())
            } else {
                None
            }
        } else {
            None
        };
        Some(ReturnStatement::new(expr.map(RcCell::new).map(Into::<ASTFlatten>::into)).into_ast())
    }

    fn visit_variableDeclarationStatement(
        &mut self,
        ctx: &VariableDeclarationStatementContext<'input>,
    ) -> Self::Return {
        ctx.variable_declaration.as_ref().unwrap().accept(self);
        // ////println!("{:?}====={:?}", self
        //     .temp_result(), ctx.variable_declaration);
        let variable_declaration = self
            .temp_result()
            .clone()
            .unwrap()
            .try_as_identifier_declaration()
            .unwrap()
            .try_as_variable_declaration();
        let expr = if let Some(expr) = &ctx.expr {
            expr.accept(self);
            if let Some(AST::Expression(e)) = self.temp_result() {
                Some(e.clone())
            } else {
                None
            }
        } else {
            None
        };
        Some(
            VariableDeclarationStatement::new(
                RcCell::new(variable_declaration.unwrap()),
                expr.map(RcCell::new).map(Into::<ASTFlatten>::into),
            )
            .into_ast(),
        )
    }

    fn visit_AllExpr(&mut self, ctx: &AllExprContext<'input>) -> Self::Return {
        if ctx.AllKeyword().is_some() {
            Some(AllExpr::new().into_ast())
        } else {
            None
        }
    }

    fn visit_IdentifierExpr(&mut self, ctx: &IdentifierExprContext<'input>) -> Self::Return {
        if let Some(idf) = &ctx.idf {
            idf.accept(self);
            if let Some(AST::Identifier(v)) = self.temp_result().clone() {
                Some(
                    IdentifierExpr::new(IdentifierExprUnion::Identifier(RcCell::new(v)), None)
                        .into_ast(),
                )
            } else {
                None
            }
        } else {
            None
        }
    }

    fn visit_MeExpr(&mut self, ctx: &MeExprContext<'input>) -> Self::Return {
        if ctx.MeKeyword().is_some() {
            Some(MeExpr::new().into_ast())
        } else {
            None
        }
    }

    fn visit_PrimitiveCastExpr(&mut self, ctx: &PrimitiveCastExprContext<'input>) -> Self::Return {
        let elem_type = if let Some(elem_type) = &ctx.elem_type {
            elem_type.accept(self);
            self.temp_result().clone().unwrap().try_as_type_name()
        } else {
            None
        };
        let expr = if let Some(expr) = &ctx.expr {
            expr.accept(self);
            self.temp_result().clone().unwrap().try_as_expression()
        } else {
            None
        };
        Some(
            PrimitiveCastExpr::new(
                elem_type.map(RcCell::new).unwrap(),
                expr.map(RcCell::new).map(Into::<ASTFlatten>::into).unwrap(),
                false,
            )
            .into_ast(),
        )
    }

    fn visit_MemberAccessExpr(&mut self, ctx: &MemberAccessExprContext<'input>) -> Self::Return {
        let member = if let Some(member) = &ctx.member {
            member.accept(self);
            self.temp_result().clone().unwrap().try_as_identifier()
        } else {
            None
        };
        let expr = if let Some(expr) = &ctx.expr {
            expr.accept(self);
            self.temp_result().clone().unwrap().try_as_expression()
        } else {
            None
        };
        Some(
            MemberAccessExpr::new(
                expr.map(|e| {
                    e.try_as_tuple_or_location_expr()
                        .unwrap()
                        .try_as_location_expr()
                        .unwrap()
                })
                .map(RcCell::new),
                member.map(RcCell::new).unwrap(),
            )
            .into_ast(),
        )
    }

    fn visit_functionCallArguments(
        &mut self,
        ctx: &FunctionCallArgumentsContext<'input>,
    ) -> Self::Return {
        //TODO vec
        let rp: Vec<_> = ctx
            .exprs
            .iter()
            .map(|p| {
                p.accept(self);
                self.temp_result().clone()
            })
            .collect();
        if rp.is_empty() {
            None
        } else {
            rp[0].clone()
        }
    }

    fn visit_tupleExpression(&mut self, ctx: &TupleExpressionContext<'input>) -> Self::Return {
        //TODO vec
        let rp: Vec<_> = ctx
            .expression_all()
            .iter()
            .map(|p| {
                p.accept(self);
                self.temp_result().clone()
            })
            .collect();
        if rp.is_empty() {
            None
        } else {
            rp[0].clone()
        }
    }

    fn visit_elementaryTypeNameExpression(
        &mut self,
        ctx: &ElementaryTypeNameExpressionContext<'input>,
    ) -> Self::Return {
        if let Some(etn) = ctx.elementaryTypeName() {
            etn.accept(self);
            self.temp_result().clone()
        } else {
            None
        }
    }

    fn visit_numberLiteral(&mut self, ctx: &NumberLiteralContext<'input>) -> Self::Return {
        if let Some(dn) = ctx.DecimalNumber() {
            return Some(
                NumberLiteralExpr::new(dn.symbol.get_text().parse::<i32>().unwrap(), false)
                    .into_ast(),
            );
        }
        if let Some(dn) = ctx.HexNumber() {
            return Some(NumberLiteralExpr::new_string(dn.symbol.get_text().to_owned()).into_ast());
        }
        None
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    // use zkay_ast::AST;

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
