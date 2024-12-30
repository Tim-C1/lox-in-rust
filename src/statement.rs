use crate::expression::*;
use crate::token::Token;

pub enum Stmt {
    ExprStmt(ExprStmtInner),
    PrintStmt(PrintStmtInner),
    VarStmt(VarStmtInner),
}

pub struct ExprStmtInner(pub Box<Expr>);
pub struct PrintStmtInner(pub Box<Expr>);
pub struct VarStmtInner(pub Token, pub Option<Box<Expr>>);

pub trait StmtVisitor<R> {
    fn visit_expr(&mut self, expr: &ExprStmtInner) -> R;
    fn visit_print(&mut self, expr: &PrintStmtInner) -> R;
    fn visit_var(&mut self, expr: &VarStmtInner) -> R;
}

pub trait StmtAccept<R> {
    fn accept<V: StmtVisitor<R>>(&self, visitor: &mut V) -> R;
}

impl<R> StmtAccept<R> for Stmt {
    fn accept<V: StmtVisitor<R>>(&self, visitor: &mut V) -> R {
        match self {
            Stmt::ExprStmt(expr) => visitor.visit_expr(expr),
            Stmt::PrintStmt(print) => visitor.visit_print(print),
            Stmt::VarStmt(var) => visitor.visit_var(var),
        }
    }
}
