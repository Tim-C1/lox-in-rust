use crate::expression::*;
use crate::token::Token;

pub enum Stmt {
    ExprStmt(ExprStmtInner),
    PrintStmt(PrintStmtInner),
    VarStmt(VarStmtInner),
    BlockStmt(BlockStmtInner),
    IfStmt(IfStmtInner),
}

pub struct ExprStmtInner(pub Box<Expr>);
pub struct PrintStmtInner(pub Box<Expr>);
pub struct VarStmtInner(pub Token, pub Option<Box<Expr>>);
pub struct BlockStmtInner(pub Vec<Box<Stmt>>);
pub struct IfStmtInner {
    pub condition: Box<Expr>,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

impl IfStmtInner {
    pub fn new(
        condition: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    ) -> Self {
        Self {
            condition,
            then_branch,
            else_branch,
        }
    }
}

pub trait StmtVisitor<R> {
    fn visit_expr(&mut self, expr: &ExprStmtInner) -> R;
    fn visit_print(&mut self, expr: &PrintStmtInner) -> R;
    fn visit_var(&mut self, expr: &VarStmtInner) -> R;
    fn visit_block(&mut self, stmts: &BlockStmtInner) -> R;
    fn visit_if(&mut self, branch: &IfStmtInner) -> R;
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
            Stmt::BlockStmt(block) => visitor.visit_block(block),
            Stmt::IfStmt(branch) => visitor.visit_if(branch),
        }
    }
}
