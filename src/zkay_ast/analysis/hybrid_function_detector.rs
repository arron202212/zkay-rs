// use crate::type_check::type_exceptions::TypeException
use crate::zkay_ast::ast::{
    AllExpr, BuiltinFunction, ConstructorOrFunctionDefinition, FunctionCallExpr, LocationExpr,
    PrimitiveCastExpr, ReclassifyExpr,
};
use crate::zkay_ast::visitor::function_visitor::FunctionVisitor;

pub fn detect_hybrid_functions(ast: AST)
// """
// :param ast
// :return: marks all functions which will require verification
// """
{
    let v = DirectHybridFunctionDetectionVisitor::new();
    v.visit(ast);

    let v = IndirectHybridFunctionDetectionVisitor::new();
    v.visit(ast);

    let v = NonInlineableCallDetector::new();
    v.visit(ast);
}

// class DirectHybridFunctionDetectionVisitor(FunctionVisitor)
pub struct DirectHybridFunctionDetectionVisitor;
impl DirectHybridFunctionDetectionVisitor {
    pub fn visitReclassifyExpr(self, ast: ReclassifyExpr) {
        ast.statement.function.requires_verification = True;
    }

    pub fn visitPrimitiveCastExpr(self, ast: PrimitiveCastExpr) {
        if ast.expr.evaluate_privately {
            ast.statement.function.requires_verification = True;
        } else {
            self.visitChildren(ast);
        }
    }

    pub fn visitAllExpr(self, ast: AllExpr)
    // pass
    {
    }
    pub fn visitFunctionCallExpr(self, ast: FunctionCallExpr) {
        if isinstance(ast.func, BuiltinFunction) && ast.func.is_private {
            ast.statement.function.requires_verification = True;
        } else if ast.is_cast && ast.evaluate_privately {
            ast.statement.function.requires_verification = True;
        } else {
            self.visitChildren(ast);
        }
    }
    pub fn visitConstructorOrFunctionDefinition(self, ast: ConstructorOrFunctionDefinition) {
        self.visit(ast.body);

        if ast.can_be_external {
            if ast.requires_verification {
                ast.requires_verification_when_external = True;
            } else {
                for param in ast.parameters {
                    if param.annotated_type.is_private() {
                        ast.requires_verification_when_external = True;
                        break;
                    }
                }
            }
        }
    }
}
// class IndirectHybridFunctionDetectionVisitor(FunctionVisitor)
pub struct IndirectHybridFunctionDetectionVisitor;
impl IndirectHybridFunctionDetectionVisitor {
    pub fn visitConstructorOrFunctionDefinition(self, ast: ConstructorOrFunctionDefinition) {
        if !ast.requires_verification {
            for fct in ast.called_functions {
                if fct.requires_verification {
                    ast.requires_verification = True;
                    if ast.can_be_external {
                        ast.requires_verification_when_external = True;
                    }
                    break;
                }
            }
        }
    }
}
// class NonInlineableCallDetector(FunctionVisitor)
pub struct NonInlineableCallDetector;
impl NonInlineableCallDetector {
    pub fn visitFunctionCallExpr(self, ast: FunctionCallExpr) {
        if !ast.is_cast && isinstance(ast.func, LocationExpr) {
            if ast.func.target.requires_verification && ast.func.target.is_recursive {
                assert!(
                    false,
                    "Non-inlineable call to recursive private function {:?}",
                    ast.func
                )
            }
        }
        self.visitChildren(ast);
    }
}
