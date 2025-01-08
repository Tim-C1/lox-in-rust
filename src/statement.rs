use crate::expression::*;
use crate::token::Token;

#[derive(Clone)]
pub enum Stmt {
    ExprStmt(ExprStmtInner),
    PrintStmt(PrintStmtInner),
    VarStmt(VarStmtInner),
    BlockStmt(BlockStmtInner),
    IfStmt(IfStmtInner),
    WhileStmt(WhileStmtInner),
    FunctionStmt(FunctionStmtInner),
    ReturnStmt(ReturnStmtInner),
}

#[derive(Clone)]
pub struct ExprStmtInner(pub Box<Expr>);
#[derive(Clone)]
pub struct PrintStmtInner(pub Box<Expr>);
#[derive(Clone)]
pub struct VarStmtInner(pub Token, pub Option<Box<Expr>>);
#[derive(Clone)]
pub struct BlockStmtInner(pub Vec<Box<Stmt>>);
#[derive(Clone)]
pub struct IfStmtInner {
    pub condition: Box<Expr>,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}
#[derive(Clone)]
pub struct WhileStmtInner {
    pub condition: Box<Expr>,
    pub body: Box<Stmt>,
}
#[derive(Clone)]
pub struct FunctionStmtInner {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Box<Stmt>,
}
#[derive(Clone)]
pub struct ReturnStmtInner {
    pub keyword: Token,
    pub value: Option<Box<Expr>>,
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

impl WhileStmtInner {
    pub fn new(condition: Box<Expr>, body: Box<Stmt>) -> Self {
        Self { condition, body }
    }
}

impl FunctionStmtInner {
    pub fn new(name: Token, params: Vec<Token>, body: Box<Stmt>) -> Self {
        Self { name, params, body }
    }
}

impl ReturnStmtInner {
    pub fn new(keyword: Token, value: Option<Box<Expr>>) -> Self {
        Self { keyword, value }
    }
}

pub trait StmtVisitor<R> {
    fn visit_expr(&mut self, expr: &ExprStmtInner) -> R;
    fn visit_print(&mut self, expr: &PrintStmtInner) -> R;
    fn visit_var(&mut self, expr: &VarStmtInner) -> R;
    fn visit_block(&mut self, stmts: &BlockStmtInner) -> R;
    fn visit_if(&mut self, branch: &IfStmtInner) -> R;
    fn visit_while(&mut self, while_stmt: &WhileStmtInner) -> R;
    fn visit_function(&mut self, func_stmt: &FunctionStmtInner) -> R;
    fn visit_return(&mut self, return_stmt: &ReturnStmtInner) -> R;
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
            Stmt::WhileStmt(while_stmt) => visitor.visit_while(while_stmt),
            Stmt::FunctionStmt(func_stmt) => visitor.visit_function(func_stmt),
            Stmt::ReturnStmt(return_stmt_inner) => visitor.visit_return(return_stmt_inner),
        }
    }
}
