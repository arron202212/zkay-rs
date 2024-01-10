
use crate::zkay_ast::ast::AST;

// T = TypeVar('T')

pub struct AstTransformerVisitor{
log:bool}
// class AstTransformerVisitor
    // """
    // Visitor which replaces visited AST elements by the corresponding visit functions return value

    // The default action when no matching visit function is defined, is to replace the node with itself and to visit
    // the children. If a matching visit function is defined, children are not automatically visited.
    // (Corresponds to node-or-children traversal order from AstVisitor)
    // """

    pub fn (self, log:bool)->Self
       { Self{log}}

   pub fn visit(self, ast)
         {self._visit_internal(ast)}

   pub fn visit_list(self, ast_list: Vec<AST>)
         {list(filter(None.__ne__, map(self.visit, ast_list)))}

   pub fn visit_children(self, ast: T) -> T
        {ast.process_children(self.visit);
        return ast}

   pub fn _visit_internal(self, ast)
       { if ast is None
            {return None}

        if self.log
           { println!('Visiting', type(ast));}
        return self.get_visit_function(ast.__class__)(ast)}

   pub fn get_visit_function(self, c)
        {visitor_function = 'visit' + c.__name__;
        if hasattr(self, visitor_function)
            {return getattr(self, visitor_function)}
        else
            {for base in c.__bases__
               { let f = self.get_visit_function(base);
                if f.is_some()
                    {return f}}}
        assert!(false);
        }

   pub fn visitAST(self, ast: AST)
        {self.visit_children(ast)}
