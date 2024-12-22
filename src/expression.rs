use crate::token::*;
use std::boxed::Box;

pub trait Visitor<R> {
    fn visit_binary(&mut self, binary: &Binary) -> R;
    fn visit_unary(&mut self, unary: &Unary) -> R;
    fn visit_literal(&mut self, literal: &Literal) -> R;
    fn visit_grouping(&mut self, grouping: &Grouping) -> R;
}

pub trait Accept<R> {
    fn accept<V: Visitor<R>>(&self, visitor: &mut V) -> R;
}

pub enum Expr {
    BinaryExpr(Binary),
    UnaryExpr(Unary),
    LiteralExpr(Literal),
    GroupingExpr(Grouping),
}

pub struct Binary {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

pub struct Unary {
    operator: Token,
    right: Box<Expr>,
}

pub struct Literal {
    value: LiteralValue,
}

pub struct Grouping {
    expression: Box<Expr>,
}

impl Binary {
    pub fn new(left: Box<Expr>, operator: Token, right: Box<Expr>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

impl Unary {
    pub fn new(operator: Token, right: Box<Expr>) -> Self {
        Self { operator, right }
    }
}

impl Literal {
    pub fn new(value: LiteralValue) -> Self {
        Self { value }
    }
}

impl Grouping {
    pub fn new(expression: Box<Expr>) -> Self {
        Self { expression }
    }
}

impl<R> Accept<R> for Expr {
    fn accept<V: Visitor<R>>(&self, visitor: &mut V) -> R {
        match self {
            Expr::BinaryExpr(b) => visitor.visit_binary(b),
            Expr::UnaryExpr(u) => visitor.visit_unary(u),
            Expr::LiteralExpr(l) => visitor.visit_literal(l),
            Expr::GroupingExpr(g) => visitor.visit_grouping(g),
        }
    }
}

impl<R> Accept<R> for Binary {
    fn accept<V: Visitor<R>>(&self, visitor: &mut V) -> R {
        visitor.visit_binary(self)
    }
}
impl<R> Accept<R> for Unary {
    fn accept<V: Visitor<R>>(&self, visitor: &mut V) -> R {
        visitor.visit_unary(self)
    }
}
impl<R> Accept<R> for Literal {
    fn accept<V: Visitor<R>>(&self, visitor: &mut V) -> R {
        visitor.visit_literal(self)
    }
}
impl<R> Accept<R> for Grouping {
    fn accept<V: Visitor<R>>(&self, visitor: &mut V) -> R {
        visitor.visit_grouping(self)
    }
}

pub mod ast_printer {
    use super::*;
    pub struct AstPrinter;
    impl Visitor<String> for AstPrinter {
        fn visit_binary(&mut self, binary: &Binary) -> String {
            self.parenthesize(
                &binary.operator.lexeme,
                vec![binary.left.as_ref(), binary.right.as_ref()],
            )
        }
        fn visit_unary(&mut self, unary: &Unary) -> String {
            self.parenthesize(&unary.operator.lexeme, vec![unary.right.as_ref()])
        }
        fn visit_literal(&mut self, literal: &Literal) -> String {
            String::from(format!("{}", literal.value))
        }
        fn visit_grouping(&mut self, grouping: &Grouping) -> String {
            self.parenthesize("group", vec![grouping.expression.as_ref()])
        }
    }
    impl AstPrinter {
        fn parenthesize(&mut self, name: &str, exprs: Vec<&Expr>) -> String {
            let mut expr_s = String::new();
            expr_s.push_str("(");
            expr_s.push_str(name);
            for expr in exprs {
                expr_s.push_str(" ");
                expr_s.push_str(&expr.accept(self));
            }
            expr_s.push_str(")");
            expr_s
        }
    }
}
