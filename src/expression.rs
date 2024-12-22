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

pub mod interpreter {
    use super::*;
    pub struct Interpreter;
    impl Visitor<LiteralValue> for Interpreter {
        fn visit_binary(&mut self, binary: &Binary) -> LiteralValue {
            let left_val = self.evaluate(&binary.left);
            let right_val = self.evaluate(&binary.right);
            match binary.operator.ttype {
                TokenType::MINUS => {
                    let l = match left_val {
                        LiteralValue::NumberLiteral(l) => l,
                        _ => panic!("runtime error: expected numeric literal for \"-\" operand"),
                    };
                    let r = match right_val {
                        LiteralValue::NumberLiteral(r) => r,
                        _ => panic!("runtime error: expected numeric literal for \"-\" operand"),
                    };
                    LiteralValue::NumberLiteral(l - r)
                }
                TokenType::PLUS => match left_val {
                    LiteralValue::NumberLiteral(l) => {
                        match right_val {
                            LiteralValue::NumberLiteral(r) => {
                                return LiteralValue::NumberLiteral(l + r)
                            }
                            _ => panic!(
                                "runtime error: expected numeric rhs literal for \"+\" operand"
                            ),
                        };
                    }
                    LiteralValue::StringLiteral(l) => {
                        match right_val {
                            LiteralValue::StringLiteral(r) => {
                                return LiteralValue::StringLiteral(l + &r)
                            }
                            _ => panic!(
                                "runtime error: expected string rhs literal for \"+\" operand"
                            ),
                        };
                    }
                    _ => panic!("runtime error: expected numeric/string literal for \"+\" operand"),
                },
                TokenType::STAR => {
                    let l = match left_val {
                        LiteralValue::NumberLiteral(l) => l,
                        _ => panic!("runtime error: expected numeric literal for \"*\" operand"),
                    };
                    let r = match right_val {
                        LiteralValue::NumberLiteral(r) => r,
                        _ => panic!("runtime error: expected numeric literal for \"*\" operand"),
                    };
                    LiteralValue::NumberLiteral(l * r)
                }
                TokenType::SLASH => {
                    let l = match left_val {
                        LiteralValue::NumberLiteral(l) => l,
                        _ => panic!("runtime error: expected numeric literal for \"/\" operand"),
                    };
                    let r = match right_val {
                        LiteralValue::NumberLiteral(r) => r,
                        _ => panic!("runtime error: expected numeric literal for \"/\" operand"),
                    };
                    LiteralValue::NumberLiteral(l / r)
                }
                TokenType::GREATER => {
                    let l = match left_val {
                        LiteralValue::NumberLiteral(l) => l,
                        _ => panic!("runtime error: expected numeric literal for \"-\" operand"),
                    };
                    let r = match right_val {
                        LiteralValue::NumberLiteral(r) => r,
                        _ => panic!("runtime error: expected numeric literal for \"-\" operand"),
                    };
                    LiteralValue::BoolLiteral(l > r)
                }
                TokenType::GREATER_EQUAL => {
                    let l = match left_val {
                        LiteralValue::NumberLiteral(l) => l,
                        _ => panic!("runtime error: expected numeric literal for \"-\" operand"),
                    };
                    let r = match right_val {
                        LiteralValue::NumberLiteral(r) => r,
                        _ => panic!("runtime error: expected numeric literal for \"-\" operand"),
                    };
                    LiteralValue::BoolLiteral(l >= r)
                }
                TokenType::LESS => {
                    let l = match left_val {
                        LiteralValue::NumberLiteral(l) => l,
                        _ => panic!("runtime error: expected numeric literal for \"-\" operand"),
                    };
                    let r = match right_val {
                        LiteralValue::NumberLiteral(r) => r,
                        _ => panic!("runtime error: expected numeric literal for \"-\" operand"),
                    };
                    LiteralValue::BoolLiteral(l < r)
                }
                TokenType::LESS_EQUAL => {
                    let l = match left_val {
                        LiteralValue::NumberLiteral(l) => l,
                        _ => panic!("runtime error: expected numeric literal for \"-\" operand"),
                    };
                    let r = match right_val {
                        LiteralValue::NumberLiteral(r) => r,
                        _ => panic!("runtime error: expected numeric literal for \"-\" operand"),
                    };
                    LiteralValue::BoolLiteral(l <= r)
                }
                TokenType::BANG_EQUAL => {
                    LiteralValue::BoolLiteral(self.is_equal(&left_val, &right_val))
                }
                TokenType::EQUAL_EQUAL => {
                    LiteralValue::BoolLiteral(!self.is_equal(&left_val, &right_val))
                }
                _ => unimplemented!(),
            }
        }

        fn visit_unary(&mut self, unary: &Unary) -> LiteralValue {
            let right_val = self.evaluate(&unary.right);
            match unary.operator.ttype {
                TokenType::MINUS => match right_val {
                    LiteralValue::NumberLiteral(f) => LiteralValue::NumberLiteral(-f),
                    _ => panic!("runtime error: expected numeric literal for \"-\" operand"),
                },
                TokenType::BANG => self.is_true(&right_val),
                _ => unimplemented!(),
            }
        }

        fn visit_literal(&mut self, literal: &Literal) -> LiteralValue {
            literal.value.clone()
        }

        fn visit_grouping(&mut self, grouping: &Grouping) -> LiteralValue {
            self.evaluate(&grouping.expression)
        }
    }

    impl Interpreter {
        fn evaluate(&mut self, expr: &Expr) -> LiteralValue {
            expr.accept(self)
        }

        fn is_true(&mut self, literal_value: &LiteralValue) -> LiteralValue {
            LiteralValue::BoolLiteral(match literal_value {
                LiteralValue::NumberLiteral(_) | LiteralValue::StringLiteral(_) => true,
                LiteralValue::BoolLiteral(b) => *b,
                LiteralValue::NilLiteral => false,
            })
        }

        fn is_equal(&mut self, l: &LiteralValue, r: &LiteralValue) -> bool {
            if matches!(l, LiteralValue::NilLiteral) {
                if matches!(r, LiteralValue::NilLiteral) {
                    true
                } else {
                    false
                }
            } else {
                match l {
                    LiteralValue::NumberLiteral(l) => match r {
                        LiteralValue::NumberLiteral(r) => l == r,
                        _ => unimplemented!(),
                    },
                    LiteralValue::BoolLiteral(l) => match r {
                        LiteralValue::BoolLiteral(r) => l == r,
                        _ => unimplemented!(),
                    },
                    LiteralValue::StringLiteral(l) => match r {
                        LiteralValue::StringLiteral(r) => l == r,
                        _ => unimplemented!(),
                    },
                    _ => unimplemented!(),
                }
            }
        }
    }
}
