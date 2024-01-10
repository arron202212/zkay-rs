
use crate::zkay_ast::ast::{Expression, AST, FunctionCallExpr, LocationExpr};
use crate::zkay_ast::visitor::visitor::AstVisitor;


pub fn contains_private_expr(ast: Option<AST>)
    {if ast is None
        {return False}
    v = ContainsPrivVisitor();
    v.visit(ast);
    return v.contains_private}


// class ContainsPrivVisitor(AstVisitor)
    // pub fn __init__(self)
    //     super().__init__('node-or-children')
    //     self.contains_private = False
pub struct ContainsPrivVisitor{
contains_private:bool
}
impl ContainsPrivVisitor{
    pub fn new() -> Self {
        Self {
            contains_private: false,
        }
    }
    pub fn visitFunctionCallExpr(self, ast: FunctionCallExpr)
       { if isinstance(ast.func, LocationExpr) and not ast.is_cast
            {self.contains_private |= ast.func.target.requires_verification;}
        self.visitExpression(ast)}

    pub fn visitExpression(self, ast: Expression)
       { if ast.evaluate_privately
            {self.contains_private = True;}
        self.visitAST(ast)}

    pub fn visitAST(self, ast)
       { if self.contains_private
            {return}
        self.visitChildren(ast)}
}