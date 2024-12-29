use crate::expression::*;

pub enum Stmt {
    ExprStmt(Box<Expr>),
    PrintStmt(Box<Expr>),
}

pub trait StmtVisitor<R> {
    fn visit_expr(&mut self, expr: &Expr) -> R;
    fn visit_print(&mut self, expr: &Expr) -> R;
}

pub trait StmtAccept<R> {
    fn accept<V: StmtVisitor<R>>(&self, visitor: &mut V) -> R;
}

impl<R> StmtAccept<R> for Stmt {
    fn accept<V: StmtVisitor<R>>(&self, visitor: &mut V) -> R {
        match self {
            Stmt::ExprStmt(expr) => visitor.visit_expr(expr),
            Stmt::PrintStmt(expr) => visitor.visit_print(expr),
        }
    }
}
