use antlr_rust::common_token_stream::CommonTokenStream;
// use antlr_rust::input_stream::InputStream;
use antlr_rust::error_listener::ErrorListener;
use antlr_rust::errors::ANTLRError;
use antlr_rust::recognizer::Recognizer;
use antlr_rust::token_factory::TokenFactory;
// use antlr_rust::BaseParser;
// use crate::solidity_parser::generated::solidityparser::SolidityParserExt;
// from zkay.errors.exceptions import ZkaySyntaxError
// use  crate::solidity_parser::generated::soliditylexer::SolidityLexer ;
// use crate::solidity_parser::generated::solidityparser::{SolidityParser,LocalTokenFactory,SourceUnitContextAll} ;
//  use std::rc::Rc;
// use crate::solidity_parser::generated::solidityparser::SolidityParserContextType;
// use crate::solidity_parser::generated::soliditylistener::SolidityListener;
// use antlr_rust::TidAble;
// class SyntaxException(ZkaySyntaxError):
//     """
//     Error during parsing"
//     """

//     def __init__(self, msg: str, ctx=None, code=None) -> None:
//         if ctx is not None:
//             assert code is not None
//             from zkay.zkay_ast.ast import get_code_error_msg
//             msg = f'{get_code_error_msg(ctx.start.line, ctx.start.column + 1, str(code).splitlines())}\n{msg}'
//         super().__init__(msg)
#[derive(Debug)]
pub struct MyErrorListener {
    pub code: String,
}

impl<'a, T: Recognizer<'a>> ErrorListener<'a, T> for MyErrorListener {
    fn syntax_error(
        &self,
        _recognizer: &T,
        _offending_symbol: Option<&<T::TF as TokenFactory<'a>>::Inner>,
        line: isize,
        column: isize,
        msg: &str,
        _e: Option<&ANTLRError>,
    ) {
        eprintln!("line {}:{} {}", line, column, msg);
        //import get_code_error_msg
        // let  _report = format!("{}\n{msg}",crate::zkay_ast::ast::get_code_error_msg(line, column + 1, str(self.code).splitlines()));
    }
}
// class MyErrorListener(ErrorListener):

//     def __init__(self, code):
//         super(MyErrorListener, self).__init__()
//         self.code = code

//     def syntaxError(self, recognizer, offending_symbol, line, column, msg, e):
//         from zkay.zkay_ast.ast import get_code_error_msg
//         report = f'{get_code_error_msg(line, column + 1, str(self.code).splitlines())}\n{msg}'
//         raise SyntaxException(report)

// use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
// use antlr_rust::token_stream::TokenStream;
// type BaseParserType<'input, I> =
// 	BaseParser<'input,SolidityParserExt<'input>, I, SolidityParserContextType , dyn SolidityListener<'input> + 'input >;

// pub struct MyParser<'input,I,H>
// where
//     I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
//     H: ErrorStrategy<'input,BaseParserType<'input,I>>{
// // pub stream : InputStream,
// // pub lexer : SolidityLexer<'input>,,
// // pub tokens:CommonTokenStream<'input>,,
// pub parser:SolidityParser<'input,I,H>,
// }

// impl<'input,I,H> MyParser<'input,I,H>
// where
//     I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
//     H: ErrorStrategy<'input,BaseParserType<'input,I>> {

//     fn new(code:&str){
//     // {  if isinstance(code, str):
//         let stream = InputStream::new(code);
//         // else:
//         //     self.stream = code
//         let lexer = SolidityLexer::new(InputStream::new(code));
//         lexer.add_error_listener(Box::new(MyErrorListener{code:code.to_string()}));
//         let tokens = CommonTokenStream::new(lexer);
//         let parser_lexer = SolidityLexer::new(InputStream::new(code));
//         parser_lexer.add_error_listener(Box::new(MyErrorListener{code:code.to_string()}));
//         let parser_tokens = CommonTokenStream::new(parser_lexer);
//         let parser = SolidityParser::new(parser_tokens);
//         parser.add_error_listener(Box::new(MyErrorListener{code:code.to_string()}));
//         // self.tree = self.parser.sourceUnit()
//         Self{stream,lexer,tokens,parser}
//     }
// }

// pub fn get_parse_tree<'input>(code:&str)-> Result<Rc<SourceUnitContextAll<'input>>,ANTLRError>
//    {
// // let p = MyParser::new(code);
// //     p.parser.sourceUnit()
//     }
