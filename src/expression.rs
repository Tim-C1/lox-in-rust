use crate::token::*;
use std::boxed::Box;

pub trait ExprVisitor<R> {
    fn visit_binary(&mut self, binary: &Binary) -> R;
    fn visit_unary(&mut self, unary: &Unary) -> R;
    fn visit_literal(&mut self, literal: &Literal) -> R;
    fn visit_grouping(&mut self, grouping: &Grouping) -> R;
    fn visit_var(&mut self, var: &Var) -> R;
    fn visit_assignment(&mut self, assignment: &Assignment) -> R;
}

pub trait ExprAccept<R> {
    fn accept<V: ExprVisitor<R>>(&self, visitor: &mut V) -> R;
}

pub enum Expr {
    BinaryExpr(Binary),
    UnaryExpr(Unary),
    LiteralExpr(Literal),
    GroupingExpr(Grouping),
    VarExpr(Var),
    AssignmentExpr(Assignment),
}

pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct Literal {
    pub value: LiteralValue,
}

pub struct Grouping {
    pub expression: Box<Expr>,
}

pub struct Var {
    pub name: Token,
}

pub struct Assignment {
    pub name: Token,
    pub value: Box<Expr>,
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

impl Var {
    pub fn new(name: Token) -> Self {
        Self { name }
    }
}

impl Assignment {
    pub fn new(name: Token, value: Box<Expr>) -> Self {
        Self { name, value }
    }
}

impl<R> ExprAccept<R> for Expr {
    fn accept<V: ExprVisitor<R>>(&self, visitor: &mut V) -> R {
        match self {
            Expr::BinaryExpr(b) => visitor.visit_binary(b),
            Expr::UnaryExpr(u) => visitor.visit_unary(u),
            Expr::LiteralExpr(l) => visitor.visit_literal(l),
            Expr::GroupingExpr(g) => visitor.visit_grouping(g),
            Expr::VarExpr(v) => visitor.visit_var(v),
            Expr::AssignmentExpr(a) => visitor.visit_assignment(a),
        }
    }
}

impl<R> ExprAccept<R> for Binary {
    fn accept<V: ExprVisitor<R>>(&self, visitor: &mut V) -> R {
        visitor.visit_binary(self)
    }
}
impl<R> ExprAccept<R> for Unary {
    fn accept<V: ExprVisitor<R>>(&self, visitor: &mut V) -> R {
        visitor.visit_unary(self)
    }
}
impl<R> ExprAccept<R> for Literal {
    fn accept<V: ExprVisitor<R>>(&self, visitor: &mut V) -> R {
        visitor.visit_literal(self)
    }
}
impl<R> ExprAccept<R> for Grouping {
    fn accept<V: ExprVisitor<R>>(&self, visitor: &mut V) -> R {
        visitor.visit_grouping(self)
    }
}

pub mod ast_printer {
    use super::*;
    pub struct AstPrinter;
    impl ExprVisitor<String> for AstPrinter {
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
            String::from(format!("{:?}", literal.value))
        }
        fn visit_grouping(&mut self, grouping: &Grouping) -> String {
            self.parenthesize("group", vec![grouping.expression.as_ref()])
        }
        fn visit_var(&mut self, _var: &Var) -> String {
            // String::from(format!("{:?}", var.name.lexeme))
            todo!()
        }

        fn visit_assignment(&mut self, _assignment: &Assignment) -> String {
            todo!()
        }
    }
    impl AstPrinter {
        pub fn print(&mut self, expr: &Expr) {
            let s = expr.accept(self);
            println!("{s}");
        }
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
