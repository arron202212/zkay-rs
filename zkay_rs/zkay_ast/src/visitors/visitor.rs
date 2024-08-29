#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

use crate::ast::{
    ASTChildren, ASTFlatten, ASTInstanceOf, ASTType, IntoAST, StatementBaseProperty, AST,
};

pub struct AstVisitorBase {
    pub traversal: String,
    pub log: bool,
}

impl AstVisitorBase {
    pub fn new(traversal: &str, log: bool) -> Self {
        Self {
            traversal: String::from(if traversal.is_empty() {
                "post"
            } else {
                traversal
            }),
            log,
        }
    }
}
pub trait AstVisitorBaseRef {
    fn ast_visitor_base_ref(&self) -> &AstVisitorBase;
}
pub trait AstVisitorBaseProperty {
    fn traversal(&self) -> &String;
    fn log(&self) -> bool;
}
impl<T: AstVisitorBaseRef> AstVisitorBaseProperty for T {
    fn traversal(&self) -> &String {
        &self.ast_visitor_base_ref().traversal
    }
    fn log(&self) -> bool {
        self.ast_visitor_base_ref().log
    }
}
pub trait AstVisitor: AstVisitorBaseProperty {
    type Return;
    fn visit(&self, ast: &ASTFlatten) -> Self::Return {
        //     if ast.get_ast_type()==ASTType::SourceUnit
        //    { // println!("==AstVisitor=====visit======{:?}",ast.get_ast_type());}
        let res = self._visit_internal(ast).unwrap();
        // println!("==AstVisitor=====visit======{:?}",ast.get_ast_type());
        //   // println!("==AstVisitor=====visit===return=={:?}={:?}",ast.to_string(),ast.get_ast_type());
        res
        // self._visit_internal(ast).unwrap()
    }
    fn has_attr(&self, name: &ASTType, ast: &AST) -> bool;
    fn get_attr(&self, name: &ASTType, ast: &ASTFlatten) -> eyre::Result<Self::Return>;
    fn temper_result(&self) -> Self::Return;
    fn _visit_internal(&self, ast: &ASTFlatten) -> Option<Self::Return> {
        if self.log() {
            // std::any::type_name::<Option<String>>(),
            // print!("Visiting {:?}", ast);
        }
        let mut ret = None;
        let mut ret_children = None;

        if self.traversal() == "post" {
            // println!("===post={:?}==",ast.get_ast_type());
            ret_children = self.visit_children(ast).ok();
        }
        // println!("==before=get_visit_function={:?}==",ast.get_ast_type());
        let f = self.get_visit_function(&ast.get_ast_type(), ast);
        // println!("===get_visit_function={:?}==",ast.get_ast_type());
        if f.is_some() {
            ret = f;
        } else if self.traversal() == "node-or-children" {
            ret_children = self.visit_children(ast).ok();
        }
        // println!("=555==get_visit_function={:?}==",ast.get_ast_type());
        if self.traversal() == "pre" {
            ret_children = self.visit_children(ast).ok();
        }
        // println!("=555=666=get_visit_function={:?}==",ast.get_ast_type());
        if ret.is_some() {
            // println!("=555=666 ======1===get_visit_function={:?}==",ast.get_ast_type());
            // Some(ret)
            ret
        } else if ret_children.is_some() {
            // println!("=555=666 =======2==get_visit_function={:?}==",ast.get_ast_type());
            ret_children
        } else {
            // println!("=555=666 ======3===get_visit_function={:?}==",ast.get_ast_type());
            // panic!("--_visit_internal---");
            None
        }
    }

    fn get_visit_function(&self, c: &ASTType, ast: &ASTFlatten) -> Option<Self::Return> {
        // if ast.get_ast_type() == ASTType::StatementListBase {
        //     if ast.is_statement_list_base() {
        //         println!(
        //             "==========is_statement_list_base========{:?}==",
        //             ast.try_as_statement_list_base_ref()
        //                 .unwrap()
        //                 .borrow()
        //                 .statements
        //                 .len()
        //         );
        //         let _cc = ast
        //             .try_as_statement_list_base_ref()
        //             .unwrap()
        //             .borrow()
        //             .clone();
        //         // let _a=_cc.into_ast();
        //         ast.try_as_statement_list_base_ref()
        //             .unwrap()
        //             .borrow()
        //             .statements
        //             .iter()
        //             .for_each(|statement| {
        //                 println!(
        //                     "==statement.get_ast_type====={:?}",
        //                     statement.get_ast_type()
        //                 );
        //                 if statement.get_ast_type() == ASTType::ExpressionStatement {
        //                     if statement.is_expression_statement() {
        //                         println!(
        //                             "==pre_statements.len====={:?}",
        //                             statement
        //                                 .try_as_expression_statement_ref()
        //                                 .unwrap()
        //                                 .borrow()
        //                                 .pre_statements()
        //                                 .len()
        //                         );
        //                         // println!("==expr.====={:?}",statement.try_as_expression_statement_ref().unwrap().borrow().expr);
        //                     }
        //                 }
        //             });
        //         // println!(
        //         //     "==========aaaaaa======begin========{:?},{:?}",
        //         //     ast.get_ast_type(),
        //         //     c
        //         // );
        //         // let _a=ast.to_ast();
        //         // println!(
        //         //     "==========aaaaaa======begin====after===={:?},{:?}",
        //         //     ast.get_ast_type(),
        //         //     c
        //         // );
        //     }
        // }

        if self.has_attr(c, &ast.to_ast()) {
            // println!("==========aaaaaa=============={:?},{:?}",ast.get_ast_type(),c);
            return self.get_attr(c, ast).ok();
        } else if let Some(_c) = AST::bases(c) {
            // println!("======bbbbb=================={:?},{:?}",ast.get_ast_type(),c);
            return self.get_visit_function(&_c, ast);
        }
        // println!("==========none=====end========={:?},{:?}",ast.get_ast_type(),c);
        None
    }
    fn visit_children(&self, ast: &ASTFlatten) -> eyre::Result<Self::Return> {
        // println!("====={:?}=========visit_children=========begin====",ast.get_ast_type());
        let mut ret = self.temper_result();
        for c in ast.children() {
            // if self.log() {
            // println!("=========={:?}====visit_children====ddddd==={:?}======",ast.get_ast_type(),c.get_ast_type());
            // }
            ret = self.visit(&c);
        }
        // println!("====={:?}=========visit_children=========end====",ast.get_ast_type());
        Ok(ret)
    }
}
