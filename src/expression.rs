use crate::token::*;
use std::boxed::Box;

pub trait Expr {
    fn print(&self) -> String;
}

pub struct BinaryExpr {
    left: Box<dyn Expr>,
    operator: Token,
    right: Box<dyn Expr>,
}

pub struct UnaryExpr {
    operator: Token,
    right: Box<dyn Expr>,
}

pub struct LiteralExpr {
    value: Literal,
}

pub struct GroupingExpr {
    expression: Box<dyn Expr>,
}

fn parenthesize(name: &str, exprs: Vec<&dyn Expr>) -> String {
    let mut expr_s = String::new();
    expr_s.push_str("(");
    expr_s.push_str(name);
    for expr in exprs {
        expr_s.push_str(" ");
        expr_s.push_str(&expr.print());
    }
    expr_s.push_str(")");
    expr_s
}

impl Expr for BinaryExpr {
    fn print(&self) -> String {
        parenthesize(
            &self.operator.lexeme,
            vec![self.left.as_ref(), self.right.as_ref()],
        )
    }
}

impl Expr for UnaryExpr {
    fn print(&self) -> String {
        parenthesize(&self.operator.lexeme, vec![self.right.as_ref()])
    }
}

impl Expr for LiteralExpr {
    fn print(&self) -> String {
        String::from(format!("{}", self.value))
    }
}

impl Expr for GroupingExpr {
    fn print(&self) -> String {
        parenthesize("group", vec![self.expression.as_ref()])
    }
}

impl BinaryExpr {
    pub fn new(left: Box<dyn Expr>, operator: Token, right: Box<dyn Expr>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

impl UnaryExpr {
    pub fn new(operator: Token, right: Box<dyn Expr>) -> Self {
        Self { operator, right }
    }
}

impl LiteralExpr {
    pub fn new(value: Literal) -> Self {
        Self { value }
    }
}

impl GroupingExpr {
    pub fn new(expression: Box<dyn Expr>) -> Self {
        Self { expression }
    }
}
