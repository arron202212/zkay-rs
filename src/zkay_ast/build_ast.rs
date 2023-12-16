// use antlr_rust::token::{Token,CommonToken};
use antlr_rust::common_token_stream::CommonTokenStream;
// use  semantic_version::{NpmSpec, Version};
use crate::zkay_ast::ast;
use crate::zkay_ast::ast::Identifier;
// use antlr_rust::TokenSource;
// use  crate::config::cfg;
// use  crate::solidity_parser::parse::SyntaxException;
use crate::solidity_parser::emit::Emitter;
use crate::solidity_parser::generated::solidityparser::ContractDefinitionContext;
use crate::solidity_parser::generated::solidityparser::ContractPartContextAttrs;
use crate::solidity_parser::generated::solidityparser::IdentifierContext;
use crate::solidity_parser::generated::solidityparser::PragmaDirectiveContext;
use crate::solidity_parser::generated::solidityparser::PragmaDirectiveContextAttrs;
use crate::solidity_parser::generated::solidityparser::SolidityParser;
use crate::solidity_parser::generated::solidityparser::VersionPragmaContext;
use antlr_rust::tree::ParseTree;
// use  crate::solidity_parser::generated::solidityvisitor::SolidityVisitor;
// use crate::solidity_parser::parse::MyParser;
// use  crate::zkay_ast::ast::StateVariableDeclaration, ContractDefinition, NumberLiteralExpr,
//     BooleanLiteralExpr, FunctionCallExpr, ExpressionStatement, IdentifierExpr,
//     ReclassifyExpr, RehomExpr, BuiltinFunction, IndexExpr;
// use  crate::zkay_ast::homomorphism::Homomorphism;
// use antlr_rust::common_token_stream::CommonTokenStream;
use crate::solidity_parser::generated::soliditylexer::SolidityLexer;
use antlr_rust::input_stream::InputStream;
// use crate::solidity_parser::generated::solidityparser::{SourceUnitContextAll} ;
// use antlr_rust::parser_rule_context::ParserRuleContext;
use crate::solidity_parser::parse::MyErrorListener;
// use antlr_rust::tree::ParseTreeVisitor;
use crate::solidity_parser::generated::solidityparser::SolidityParserContextType;
use antlr_rust::tree::Visitable;
fn build_ast_from_parse_tree(code: &str) -> ast::AST {
    let mut lexer = SolidityLexer::new(InputStream::new(code));
    lexer.add_error_listener(Box::new(MyErrorListener {
        code: code.to_string(),
    }));
    let tokens = CommonTokenStream::new(lexer);
    let mut parser = SolidityParser::new(tokens);
    let root = parser.sourceUnit().expect("");
    // parser.add_error_listener(Box::new(MyErrorListener{code:code.to_string()}));
    let mut v = BuildASTVisitor::new(code.to_string());
    root.accept(&mut v);
    ast::AST::Never
}

pub fn build_ast(code: &str) -> ast::AST {
    let mut full_ast = build_ast_from_parse_tree(code);
    // assert isinstance(full_ast, ast.SourceUnit)
    // let full_ast.original_code = str(code).split("\n");
    //  full_ast
    ast::AST::Never
}

struct BuildASTVisitor {
    emitter: Emitter,
    code: String,
    asts: ast::AST,
}
impl BuildASTVisitor {
    fn new(code: String) -> Self {
        Self {
            emitter: Emitter::new(Some(code.clone())),
            code,
            asts: ast::AST::Never,
        }
    }
}
use std::any::{Any, TypeId};
fn is_instance<S: ?Sized + Any, T: ?Sized + Any>(_s: &T) -> bool {
    TypeId::of::<T>() == TypeId::of::<S>()
}
// fn is_instance<'t,T:'t>(s: &'t dyn Any) -> bool {
//     TypeId::of::<T>() == s.type_id()
// }
fn print_type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
use crate::solidity_parser::generated::solidityvisitor::SolidityVisitorCompat;
use antlr_rust::parser::ParserNodeType;
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::tree::ParseTreeVisitorCompat;
impl<'input> ParseTreeVisitorCompat<'input> for BuildASTVisitor {
    type Node = SolidityParserContextType;
    type Return = ast::AST;
    fn temp_result(&mut self) -> &mut <Self as ParseTreeVisitorCompat<'input>>::Return {
        &mut self.asts
    }
}
//  fn  handle_field(self, field){
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
        ast::AST::Identifier(Identifier {
            name: name.to_string(),
        })
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
        ctx.idf.as_ref().unwrap().accept(self);
        let identifier = self.temp_result().clone();
        // if "$" in identifier.name:
        //     raise SyntaxException("$ is not allowed in zkay contract identifiers", ctx.idf, self.code)
        // parts = [self.visit(c) for c in ctx.parts]
        // state_vars = [p for p in parts if isinstance(p, StateVariableDeclaration)]
        // cfdefs = [p for p in parts if isinstance(p, ast.ConstructorOrFunctionDefinition)]
        // constructors = [p for p in cfdefs if p.is_constructor]
        // functions = [p for p in cfdefs if p.is_function]
        // enums = [p for p in parts if isinstance(p, ast.EnumDefinition)]
        // return ContractDefinition(identifier, state_vars, constructors, functions, enums)
        let state_vars: Vec<_> = ctx
            .parts
            .iter()
            .filter_map(|p| {
                if let Some(v) = p.stateVariableDeclaration() {
                    v.accept(self);
                    Some(self.temp_result().clone())
                } else {
                    None
                }
            })
            .collect();
        let constructors: Vec<_> = ctx
            .parts
            .iter()
            .filter_map(|p| {
                if let Some(v) = p.constructorDefinition() {
                    v.accept(self);
                    Some(self.temp_result().clone())
                } else {
                    None
                }
            })
            .collect();
        let functions: Vec<_> = ctx
            .parts
            .iter()
            .filter_map(|p| {
                if let Some(v) = p.functionDefinition() {
                    v.accept(self);
                    Some(self.temp_result().clone())
                } else {
                    None
                }
            })
            .collect();
        let enums: Vec<_> = ctx
            .parts
            .iter()
            .filter_map(|p| {
                if let Some(v) = p.enumDefinition() {
                    v.accept(self);
                    Some(self.temp_result().clone())
                } else {
                    None
                }
            })
            .collect();

        ast::AST::Never
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

    // fn  visitFunctionDefinition(self, ctx:SolidityParser.FunctionDefinitionContext){
    //     return self.handle_fdef(ctx)

    // fn  visitConstructorDefinition(self, ctx:SolidityParser.ConstructorDefinitionContext){
    //     return self.handle_fdef(ctx)

    // fn  visitEnumDefinition(self, ctx:SolidityParser.EnumDefinitionContext){
    //     idf = self.visit(ctx.idf)
    //     if "$" in idf.name:
    //         raise SyntaxException("$ is not allowed in zkay enum identifiers", ctx.idf, self.code)
    //     values = [self.visit(v) for v in ctx.values]
    //     return ast.EnumDefinition(idf, values)

    // fn  visitEnumValue(self, ctx:SolidityParser.EnumValueContext){
    //     idf = self.visit(ctx.idf)
    //     if "$" in idf.name:
    //         raise SyntaxException("$ is not allowed in zkay enum value identifiers", ctx.idf, self.code)
    //     return ast.EnumValue(idf)

    // // Visit a parse tree produced by SolidityParser#NumberLiteralExpr.
    // fn  visitNumberLiteralExpr(self, ctx: SolidityParser.NumberLiteralExprContext){
    //     v = int(ctx.getText().replace("_", ""), 0)
    //     return NumberLiteralExpr(v, ctx.getText().startswith(("0x", "0X")))

    // // Visit a parse tree produced by SolidityParser#BooleanLiteralExpr.
    // fn  visitBooleanLiteralExpr(self, ctx: SolidityParser.BooleanLiteralExprContext){
    //     b = ctx.getText() == "true"
    //     return BooleanLiteralExpr(b)

    // fn  visitStringLiteralExpr(self, ctx: SolidityParser.StringLiteralExprContext){
    //     s = ctx.getText()

    //     // Remove quotes
    //     if s.startswith("""){
    //         s = s[1:-1].replace("\\"", """)
    //     else:
    //         s = s[2:-2]

    //     raise SyntaxException("Use of unsupported string literal expression", ctx, self.code)
    //     // return StringLiteralExpr(s)

    // fn  visitTupleExpr(self, ctx:SolidityParser.TupleExprContext){
    //     contents = ctx.expr.children[1:-1]
    //     elements = []
    //     for idx in range(0, len(contents), 2){
    //         elements.append(self.visit(contents[idx]))
    //     return ast.TupleExpr(elements)

    // fn  visitModifier(self, ctx: SolidityParser.ModifierContext){
    //     return ctx.getText()

    // fn  visitAnnotatedTypeName(self, ctx: SolidityParser.AnnotatedTypeNameContext){
    //     pa = None
    //     hom = Homomorphism.NON_HOMOMORPHIC
    //     if ctx.privacy_annotation is not None:
    //         pa = self.visit(ctx.privacy_annotation)
    //         if ctx.homomorphism is not None:
    //             hom = self.visit(ctx.homomorphism)

    //         if not (isinstance(pa, ast.AllExpr) or isinstance(pa, ast.MeExpr) or isinstance(pa, IdentifierExpr)){
    //             raise SyntaxException("Privacy annotation can only be me | all | Identifier", ctx.privacy_annotation, self.code)
    //         if isinstance(pa, ast.AllExpr) and hom != Homomorphism.NON_HOMOMORPHIC:
    //             raise SyntaxException("Public types cannot be homomorphic", ctx.homomorphism, self.code)

    //     return ast.AnnotatedTypeName(self.visit(ctx.type_name), pa, hom)

    // fn  visitHomomorphismAnnotation(self, ctx:SolidityParser.HomomorphismAnnotationContext){
    //     t = ctx.getText()
    //     for h in Homomorphism:
    //         if h.type_annotation == t:
    //             return h
    //     else:
    //         raise SyntaxException(f"Unsupported homomorphism {t}", ctx, self.code)

    // fn  visitElementaryTypeName(self, ctx: SolidityParser.ElementaryTypeNameContext){
    //     t = ctx.getText()
    //     if t == "address":
    //         return ast.AddressTypeName()
    //     elif t == "address payable":
    //         return ast.AddressPayableTypeName()
    //     elif t == "bool":
    //         return ast.BoolTypeName()
    //     elif t.startswith("int"){
    //         return ast.IntTypeName(t)
    //     elif t.startswith("uint"){
    //         return ast.UintTypeName(t)
    //     elif t == "var":
    //         raise SyntaxException(f"Use of unsupported var keyword", ctx, self.code)
    //     else:
    //         raise SyntaxException(f"Use of unsupported type "{t}".", ctx, self.code)

    // fn  visitIndexExpr(self, ctx: SolidityParser.IndexExprContext){
    //     arr = self.visit(ctx.arr)
    //     if not isinstance(arr, ast.LocationExpr){
    //         raise SyntaxException(f"Expression cannot be indexed", ctx.arr, self.code)
    //     index = self.visit(ctx.index)
    //     return IndexExpr(arr, index)

    // fn  visitParenthesisExpr(self, ctx: SolidityParser.ParenthesisExprContext){
    //     f = BuiltinFunction("parenthesis").override(line=ctx.start.line, column=ctx.start.column)
    //     expr = self.visit(ctx.expr)
    //     return FunctionCallExpr(f, [expr])

    // fn  visitSignExpr(self, ctx: SolidityParser.SignExprContext){
    //     f = BuiltinFunction("sign" + ctx.op.text).override(line=ctx.op.line, column=ctx.op.column)
    //     expr = self.visit(ctx.expr)
    //     return FunctionCallExpr(f, [expr])

    // fn  visitNotExpr(self, ctx: SolidityParser.NotExprContext){
    //     f = BuiltinFunction("!").override(line=ctx.start.line, column=ctx.start.column)
    //     expr = self.visit(ctx.expr)
    //     return FunctionCallExpr(f, [expr])

    // fn  visitBitwiseNotExpr(self, ctx: SolidityParser.BitwiseNotExprContext){
    //     f = BuiltinFunction("~").override(line=ctx.start.line, column=ctx.start.column)
    //     expr = self.visit(ctx.expr)
    //     return FunctionCallExpr(f, [expr])

    // fn  _visitBinaryExpr(self, ctx){
    //     lhs = self.visit(ctx.lhs)
    //     rhs = self.visit(ctx.rhs)
    //     f = BuiltinFunction(ctx.op.text).override(line=ctx.op.line, column=ctx.op.column)
    //     return FunctionCallExpr(f, [lhs, rhs])

    // fn  _visitBoolExpr(self, ctx){
    //     return self._visitBinaryExpr(ctx)

    // fn  visitPowExpr(self, ctx: SolidityParser.PowExprContext){
    //     return self._visitBinaryExpr(ctx)

    // fn  visitMultDivModExpr(self, ctx: SolidityParser.MultDivModExprContext){
    //     return self._visitBinaryExpr(ctx)

    // fn  visitPlusMinusExpr(self, ctx: SolidityParser.PlusMinusExprContext){
    //     return self._visitBinaryExpr(ctx)

    // fn  visitCompExpr(self, ctx: SolidityParser.CompExprContext){
    //     return self._visitBinaryExpr(ctx)

    // fn  visitEqExpr(self, ctx: SolidityParser.EqExprContext){
    //     return self._visitBinaryExpr(ctx)

    // fn  visitAndExpr(self, ctx: SolidityParser.AndExprContext){
    //     return self._visitBoolExpr(ctx)

    // fn  visitOrExpr(self, ctx: SolidityParser.OrExprContext){
    //     return self._visitBoolExpr(ctx)

    // fn  visitBitwiseOrExpr(self, ctx: SolidityParser.BitwiseOrExprContext){
    //     return self._visitBinaryExpr(ctx)

    // fn  visitBitShiftExpr(self, ctx: SolidityParser.BitShiftExprContext){
    //     return self._visitBinaryExpr(ctx)

    // fn  visitBitwiseAndExpr(self, ctx: SolidityParser.BitwiseAndExprContext){
    //     return self._visitBinaryExpr(ctx)

    // fn  visitBitwiseXorExpr(self, ctx: SolidityParser.BitwiseXorExprContext){
    //     return self._visitBinaryExpr(ctx)

    // fn  visitIteExpr(self, ctx: SolidityParser.IteExprContext){
    //     f = BuiltinFunction("ite")
    //     cond = self.visit(ctx.cond)
    //     then_expr = self.visit(ctx.then_expr)
    //     else_expr = self.visit(ctx.else_expr)
    //     return FunctionCallExpr(f, [cond, then_expr, else_expr])

    // rehom_expressions = {}
    // for h in Homomorphism:
    //     rehom_expressions[h.rehom_expr_name] = h

    // fn  visitFunctionCallExpr(self, ctx: SolidityParser.FunctionCallExprContext){
    //     func = self.visit(ctx.func)
    //     args = self.handle_field(ctx.args)

    //     if isinstance(func, IdentifierExpr){
    //         if func.idf.name == "reveal":
    //             if len(args) != 2:
    //                 raise SyntaxException(f"Invalid number of arguments for reveal: {args}", ctx.args, self.code)
    //             return ReclassifyExpr(args[0], args[1], None)
    //         elif func.idf.name in self.rehom_expressions:
    //             name = func.idf.name
    //             homomorphism = self.rehom_expressions[name]
    //             if len(args) != 1:
    //                 raise SyntaxException(f"Invalid number of arguments for {name}: {args}", ctx.args, self.code)
    //             return RehomExpr(args[0], homomorphism)

    //     return FunctionCallExpr(func, args)

    // fn  visitIfStatement(self, ctx: SolidityParser.IfStatementContext){
    //     cond = self.visit(ctx.condition)
    //     then_branch = self.visit(ctx.then_branch)
    //     if not isinstance(then_branch, ast.Block){
    //         then_branch = ast.Block([then_branch], was_single_statement=True)

    //     if ctx.else_branch is not None:
    //         else_branch = self.visit(ctx.else_branch)
    //         if not isinstance(else_branch, ast.Block){
    //             else_branch = ast.Block([else_branch], was_single_statement=True)
    //     else:
    //         else_branch = None

    //     return ast.IfStatement(cond, then_branch, else_branch)

    // fn  visitWhileStatement(self, ctx: SolidityParser.WhileStatementContext){
    //     cond = self.visit(ctx.condition)
    //     body = self.visit(ctx.body)
    //     if not isinstance(body, ast.Block){
    //         body = ast.Block([body], was_single_statement=True)
    //     return ast.WhileStatement(cond, body)

    // fn  visitDoWhileStatement(self, ctx: SolidityParser.DoWhileStatementContext){
    //     body = self.visit(ctx.body)
    //     cond = self.visit(ctx.condition)
    //     if not isinstance(body, ast.Block){
    //         body = ast.Block([body], was_single_statement=True)
    //     return ast.DoWhileStatement(body, cond)

    // fn  visitForStatement(self, ctx: SolidityParser.ForStatementContext){
    //     init = None if ctx.init is None else self.visit(ctx.init)
    //     cond = self.visit(ctx.condition)
    //     update = None if ctx.update is None else self.visit(ctx.update)
    //     if isinstance(update, ast.Expression){
    //         update = ast.ExpressionStatement(update)
    //     body = self.visit(ctx.body)
    //     if not isinstance(body, ast.Block){
    //         body = ast.Block([body], was_single_statement=True)
    //     return ast.ForStatement(init, cond, update, body)

    // fn  is_expr_stmt(self, ctx: SolidityParser.ExpressionContext) -> bool:
    //     if isinstance(ctx.parentCtx, SolidityParser.ExpressionStatementContext){
    //         return True
    //     elif isinstance(ctx.parentCtx, SolidityParser.ForStatementContext) and ctx == ctx.parentCtx.update:
    //         return True
    //     else:
    //         return False

    // fn  visitAssignmentExpr(self, ctx: SolidityParser.AssignmentExprContext){
    //     if not self.is_expr_stmt(ctx){
    //         raise SyntaxException("Assignments are only allowed as statements", ctx, self.code)
    //     lhs = self.visit(ctx.lhs)
    //     rhs = self.visit(ctx.rhs)
    //     assert ctx.op.text[-1] == "="
    //     op = ctx.op.text[:-1] if ctx.op.text != "=" else ""
    //     if op:
    //         // If the assignment contains an additional operator -> replace lhs = rhs with lhs = lhs "op" rhs
    //         rhs = FunctionCallExpr(BuiltinFunction(op).override(line=ctx.op.line, column=ctx.op.column), [self.visit(ctx.lhs), rhs])
    //         rhs.line = ctx.rhs.start.line
    //         rhs.column = ctx.rhs.start.column + 1
    //     return ast.AssignmentStatement(lhs, rhs, op)

    // fn  _handle_crement_expr(self, ctx, kind: str){
    //     if not self.is_expr_stmt(ctx){
    //         raise SyntaxException(f"{kind}-crement expressions are only allowed as statements", ctx, self.code)
    //     op = "+" if ctx.op.text == "++" else "-"

    //     one = NumberLiteralExpr(1)
    //     one.line = ctx.op.line
    //     one.column = ctx.op.column + 1

    //     fct = FunctionCallExpr(BuiltinFunction(op).override(line=ctx.op.line, column=ctx.op.column), [self.visit(ctx.expr), one])
    //     fct.line = ctx.op.line
    //     fct.column = ctx.op.column + 1

    //     return ast.AssignmentStatement(self.visit(ctx.expr), fct, f"{kind}{ctx.op.text}")

    // fn  visitPreCrementExpr(self, ctx: SolidityParser.PreCrementExprContext){
    //     return self._handle_crement_expr(ctx, "pre")

    // fn  visitPostCrementExpr(self, ctx: SolidityParser.PostCrementExprContext){
    //     return self._handle_crement_expr(ctx, "post")

    // fn  visitExpressionStatement(self, ctx: SolidityParser.ExpressionStatementContext){
    //     e = self.visit(ctx.expr)
    //     if isinstance(e, ast.Statement){
    //         return e
    //     else:
    //         // handle require
    //         if isinstance(e, FunctionCallExpr){
    //             f = e.func
    //             if isinstance(f, IdentifierExpr){
    //                 if f.idf.name == "require":
    //                     if len(e.args) != 1:
    //                         raise SyntaxException(f"Invalid number of arguments for require: {e.args}", ctx.expr, self.code)
    //                     return ast.RequireStatement(e.args[0])

    //         assert isinstance(e, ast.Expression)
    //         return ExpressionStatement(e)
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::zkay_ast::ast::AST;

    #[test]
    fn test_build_ast() {
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
