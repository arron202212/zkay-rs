use crate::zkay_ast::ast::{Block, HybridArgumentIdf, AST};
use dyn_clone::DynClone;
// T = TypeVar("T")
pub trait TransformerVisitorEx: DynClone + std::marker::Sync + AstTransformerVisitor {}
dyn_clone::clone_trait_object!(TransformerVisitorEx);
pub struct AstTransformerVisitorBase {
    log: bool,
}
pub trait AstTransformerVisitor {
    // type Return ;
    // type AST;
    fn default() -> Self
    where
        Self: Sized;
    fn visit(&self, ast: AST) -> AST;
    fn visitBlock(
        &self,
        ast: AST,
        guard_cond: Option<HybridArgumentIdf>,
        guard_val: Option<bool>,
    ) -> AST;
    fn visit_list(&self, ast_list: Vec<AST>) -> Vec<AST> {
        ast_list
            .iter()
            .filter_map(|a| Some(self.visit(a.clone())))
            .collect()
    }
    fn visit_children(&self, mut ast: AST) -> AST {
        // ast.process_children(self.visit);
        // ast
        AST::None
    }

    fn _visit_internal(&self, ast: AST) -> AST {
        AST::None
    }
}
// class AstTransformerVisitor
// """
// Visitor which replaces visited AST elements by the corresponding visit functions return value

// The default action when no matching visit function is defined, is to replace the node with itself and to visit
// the children. If a matching visit function is defined, children are not automatically visited.
// (Corresponds to node-or-children traversal order from AstVisitor)
// """

impl AstTransformerVisitorBase {
    pub fn new(log: bool) -> Self {
        Self { log }
    }

    pub fn visit_children<T>(&self, mut ast: T) -> T {
        // ast.process_children(self.visit);
        ast
    }

    pub fn _visit_internal(&self, ast: AST) -> AST {
        if ast == AST::None {
            return ast;
        }

        if self.log {
            println!("Visiting {:?}", ast);
        }
        self.get_visit_function(ast)
    }

    pub fn get_visit_function(&self, c: AST) -> AST {
        // let visitor_function = "visit" + c.name();
        // if hasattr(&self, visitor_function) {
        //     return getattr(&self, visitor_function);
        // } else {
        //     for base in c.bases() {
        //         let f = self.get_visit_function(base);
        //         if f.is_some() {
        //             return f;
        //         }
        //     }
        // }
        // assert!(false);
        c
    }

    pub fn visitAST(&self, ast: AST) -> AST {
        self.visit_children(ast)
    }
}
impl AstTransformerVisitor for AstTransformerVisitorBase {
    fn default() -> Self {
        Self::new(false)
    }

    fn visit(&self, ast: AST) -> AST {
        self._visit_internal(ast)
    }
    fn visitBlock(
        &self,
        ast: AST,
        guard_cond: Option<HybridArgumentIdf>,
        guard_val: Option<bool>,
    ) -> AST {
        self.visit_children(ast)
    }
}
