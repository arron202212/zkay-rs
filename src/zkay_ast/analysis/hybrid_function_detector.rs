// use crate::type_check::type_exceptions::TypeException
use crate::zkay_ast::ast::{
    is_instance, ASTCode, ASTType, AllExpr, BuiltinFunction, ConstructorOrFunctionDefinition,
    FunctionCallExpr, LocationExpr, PrimitiveCastExpr, ReclassifyExpr, AST,
};
use crate::zkay_ast::visitor::{function_visitor::FunctionVisitor, visitor::AstVisitor};

pub fn detect_hybrid_functions(ast: AST)
// """
// :param ast
// :return: marks all functions which will require verification
// """
{
    let v = DirectHybridFunctionDetectionVisitor;
    v.visit(ast);

    let v = IndirectHybridFunctionDetectionVisitor;
    v.visit(ast);

    let v = NonInlineableCallDetector;
    v.visit(ast);
}

// class DirectHybridFunctionDetectionVisitor(FunctionVisitor)
pub struct DirectHybridFunctionDetectionVisitor;

impl FunctionVisitor for DirectHybridFunctionDetectionVisitor {}
impl AstVisitor for DirectHybridFunctionDetectionVisitor {
    type Return = Option<String>;
    fn temper_result(&self) -> Option<Self::Return> {
        None
    }
    fn log(&self) -> bool {
        false
    }
    fn traversal(&self) -> &'static str {
        "node-or-children"
    }
    fn has_attr(&self, name: &String) -> bool {
        self.get_attr(name).is_some()
    }
    fn get_attr(&self, name: &String) -> Option<String> {
        None
    }
    fn call_visit_function(&self, ast: &AST) -> Option<Self::Return> {
        None
    }
}
impl DirectHybridFunctionDetectionVisitor {
    pub fn visitReclassifyExpr(self, ast: ReclassifyExpr) {
        if let ReclassifyExpr::ReclassifyExpr(re) = ast {
            re.expression_base
                .statement
                .unwrap()
                .function
                .requires_verification = true;
        }
    }

    pub fn visitPrimitiveCastExpr(self, ast: PrimitiveCastExpr) {
        if ast.expr.evaluate_privately() {
            ast.statement.function.requires_verification = true;
        } else {
            self.visit_children(&ast.get_ast());
        }
    }

    pub fn visitAllExpr(self, ast: AllExpr)
    // pass
    {
    }
    pub fn visitFunctionCallExpr(self, ast: &mut FunctionCallExpr) {
        if is_instance(&ast.func().unwrap(), ASTType::BuiltinFunction)
            && ast.func().unwrap().is_private()
        {
            ast.statement()
                .unwrap()
                .statement_mut()
                .function
                .requires_verification = true;
        } else if ast.is_cast() && ast.evaluate_privately() {
            ast.statement()
                .unwrap()
                .statement_mut()
                .function
                .requires_verification = true;
        } else {
            self.visit_children(&ast.get_ast());
        }
    }
    pub fn visitConstructorOrFunctionDefinition(self, ast: ConstructorOrFunctionDefinition) {
        self.visit(ast.body.unwrap().get_ast());

        if ast.can_be_external() {
            if ast.requires_verification {
                ast.requires_verification_when_external = true;
            } else {
                for param in ast.parameters {
                    if param
                        .identifier_declaration_base
                        .annotated_type
                        .is_private()
                    {
                        ast.requires_verification_when_external = true;
                        break;
                    }
                }
            }
        }
    }
}
// class IndirectHybridFunctionDetectionVisitor(FunctionVisitor)
pub struct IndirectHybridFunctionDetectionVisitor;

impl FunctionVisitor for IndirectHybridFunctionDetectionVisitor {}
impl AstVisitor for IndirectHybridFunctionDetectionVisitor {
    type Return = Option<String>;
    fn temper_result(&self) -> Option<Self::Return> {
        None
    }
    fn log(&self) -> bool {
        false
    }
    fn traversal(&self) -> &'static str {
        "node-or-children"
    }
    fn has_attr(&self, name: &String) -> bool {
        self.get_attr(name).is_some()
    }
    fn get_attr(&self, name: &String) -> Option<String> {
        None
    }
    fn call_visit_function(&self, ast: &AST) -> Option<Self::Return> {
        None
    }
}
impl IndirectHybridFunctionDetectionVisitor {
    pub fn visitConstructorOrFunctionDefinition(self, ast: ConstructorOrFunctionDefinition) {
        if !ast.requires_verification {
            for fct in ast.called_functions {
                if fct.requires_verification {
                    ast.requires_verification = true;
                    if ast.can_be_external() {
                        ast.requires_verification_when_external = true;
                    }
                    break;
                }
            }
        }
    }
}
// class NonInlineableCallDetector(FunctionVisitor)
pub struct NonInlineableCallDetector;

impl FunctionVisitor for NonInlineableCallDetector {}
impl AstVisitor for NonInlineableCallDetector {
    type Return = Option<String>;
    fn temper_result(&self) -> Option<Self::Return> {
        None
    }
    fn log(&self) -> bool {
        false
    }
    fn traversal(&self) -> &'static str {
        "node-or-children"
    }
    fn has_attr(&self, name: &String) -> bool {
        self.get_attr(name).is_some()
    }
    fn get_attr(&self, name: &String) -> Option<String> {
        None
    }
    fn call_visit_function(&self, ast: &AST) -> Option<Self::Return> {
        None
    }
}
impl NonInlineableCallDetector {
    pub fn visitFunctionCallExpr(self, ast: FunctionCallExpr) {
        if !ast.is_cast() && is_instance(&ast.func().unwrap(), ASTType::LocationExpr) {
            if ast.func().unwrap().target().unwrap().requires_verification
                && ast.func().unwrap().target().unwrap().is_recursive()
            {
                assert!(
                    false,
                    "Non-inlineable call to recursive private function {:?}",
                    ast.func()
                )
            }
        }
        self.visit_children(&ast.get_ast());
    }
}
