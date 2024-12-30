use crate::environment::*;
use crate::expression::ExprAccept;
use crate::expression::*;
use crate::statement::StmtAccept;
use crate::statement::*;
use crate::token::*;
use std::fmt;

pub struct Interpreter {
    environment: Environment,
}

pub enum RuntimeError {
    InvalidOperand(TokenType, String, usize),
    UndefinedVar(Token),
}
impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidOperand(_, desc, line) => {
                write!(f, "{}\n[line {}]", desc, line)
            }
            Self::UndefinedVar(v) => {
                write!(f, "Undefined variable '{}'.", v.lexeme)
            }
        }
    }
}
impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
        }
    }
    pub fn evaluate(&mut self, expr: &Expr) -> Result<LiteralValue, RuntimeError> {
        expr.accept(self)
    }
    pub fn interprete(&mut self, stmts: &Vec<Stmt>) -> Result<(), RuntimeError> {
        for stmt in stmts {
            self.execute(stmt)?
        }
        Ok(())
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        stmt.accept(self)
    }

    fn is_true(&mut self, literal_value: &LiteralValue) -> bool {
        match literal_value {
            LiteralValue::NumberLiteral(_) | LiteralValue::StringLiteral(_) => true,
            LiteralValue::BoolLiteral(b) => *b,
            LiteralValue::NilLiteral => false,
        }
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
                    _ => false,
                },
                LiteralValue::BoolLiteral(l) => match r {
                    LiteralValue::BoolLiteral(r) => l == r,
                    _ => false,
                },
                LiteralValue::StringLiteral(l) => match r {
                    LiteralValue::StringLiteral(r) => l == r,
                    _ => false,
                },
                _ => unreachable!(),
            }
        }
    }
}
impl ExprVisitor<Result<LiteralValue, RuntimeError>> for Interpreter {
    fn visit_binary(&mut self, binary: &Binary) -> Result<LiteralValue, RuntimeError> {
        let left_val = self.evaluate(&binary.left)?;
        let right_val = self.evaluate(&binary.right)?;
        match binary.operator.ttype {
            TokenType::MINUS => {
                let l = match left_val {
                    LiteralValue::NumberLiteral(l) => l,
                    _ => {
                        return Err(RuntimeError::InvalidOperand(
                            TokenType::MINUS,
                            String::from("Operands must be a number."),
                            binary.operator.line,
                        ))
                    }
                };
                let r = match right_val {
                    LiteralValue::NumberLiteral(r) => r,
                    _ => {
                        return Err(RuntimeError::InvalidOperand(
                            TokenType::MINUS,
                            String::from("Operands must be a number."),
                            binary.operator.line,
                        ))
                    }
                };
                Ok(LiteralValue::NumberLiteral(l - r))
            }
            TokenType::PLUS => match left_val {
                LiteralValue::NumberLiteral(l) => {
                    match right_val {
                        LiteralValue::NumberLiteral(r) => {
                            return Ok(LiteralValue::NumberLiteral(l + r))
                        }
                        _ => {
                            return Err(RuntimeError::InvalidOperand(
                                TokenType::MINUS,
                                String::from("Operands must be two numbers or two strings."),
                                binary.operator.line,
                            ))
                        }
                    };
                }
                LiteralValue::StringLiteral(l) => {
                    match right_val {
                        LiteralValue::StringLiteral(r) => {
                            return Ok(LiteralValue::StringLiteral(l + &r))
                        }
                        _ => {
                            return Err(RuntimeError::InvalidOperand(
                                TokenType::MINUS,
                                String::from("Operands must be two numbers or two strings."),
                                binary.operator.line,
                            ))
                        }
                    };
                }
                _ => {
                    return Err(RuntimeError::InvalidOperand(
                        TokenType::MINUS,
                        String::from("Operands must be two numbers or two strings."),
                        binary.operator.line,
                    ))
                }
            },
            TokenType::STAR => {
                let l = match left_val {
                    LiteralValue::NumberLiteral(l) => l,
                    _ => {
                        return Err(RuntimeError::InvalidOperand(
                            TokenType::MINUS,
                            String::from("Operands must be a number."),
                            binary.operator.line,
                        ))
                    }
                };
                let r = match right_val {
                    LiteralValue::NumberLiteral(r) => r,
                    _ => {
                        return Err(RuntimeError::InvalidOperand(
                            TokenType::MINUS,
                            String::from("Operands must be a number."),
                            binary.operator.line,
                        ))
                    }
                };
                Ok(LiteralValue::NumberLiteral(l * r))
            }
            TokenType::SLASH => {
                let l = match left_val {
                    LiteralValue::NumberLiteral(l) => l,
                    _ => {
                        return Err(RuntimeError::InvalidOperand(
                            TokenType::MINUS,
                            String::from("Operands must be a number."),
                            binary.operator.line,
                        ))
                    }
                };
                let r = match right_val {
                    LiteralValue::NumberLiteral(r) => r,
                    _ => {
                        return Err(RuntimeError::InvalidOperand(
                            TokenType::MINUS,
                            String::from("Operands must be a number."),
                            binary.operator.line,
                        ))
                    }
                };
                Ok(LiteralValue::NumberLiteral(l / r))
            }
            TokenType::GREATER => {
                let l = match left_val {
                    LiteralValue::NumberLiteral(l) => l,
                    _ => {
                        return Err(RuntimeError::InvalidOperand(
                            TokenType::MINUS,
                            String::from("Operands must be a number."),
                            binary.operator.line,
                        ))
                    }
                };
                let r = match right_val {
                    LiteralValue::NumberLiteral(r) => r,
                    _ => {
                        return Err(RuntimeError::InvalidOperand(
                            TokenType::MINUS,
                            String::from("Operands must be a number."),
                            binary.operator.line,
                        ))
                    }
                };
                Ok(LiteralValue::BoolLiteral(l > r))
            }
            TokenType::GREATER_EQUAL => {
                let l = match left_val {
                    LiteralValue::NumberLiteral(l) => l,
                    _ => {
                        return Err(RuntimeError::InvalidOperand(
                            TokenType::MINUS,
                            String::from("Operands must be a number."),
                            binary.operator.line,
                        ))
                    }
                };
                let r = match right_val {
                    LiteralValue::NumberLiteral(r) => r,
                    _ => {
                        return Err(RuntimeError::InvalidOperand(
                            TokenType::MINUS,
                            String::from("Operands must be a number."),
                            binary.operator.line,
                        ))
                    }
                };
                Ok(LiteralValue::BoolLiteral(l >= r))
            }
            TokenType::LESS => {
                let l = match left_val {
                    LiteralValue::NumberLiteral(l) => l,
                    _ => {
                        return Err(RuntimeError::InvalidOperand(
                            TokenType::MINUS,
                            String::from("Operands must be a number."),
                            binary.operator.line,
                        ))
                    }
                };
                let r = match right_val {
                    LiteralValue::NumberLiteral(r) => r,
                    _ => {
                        return Err(RuntimeError::InvalidOperand(
                            TokenType::MINUS,
                            String::from("Operands must be a number."),
                            binary.operator.line,
                        ))
                    }
                };
                Ok(LiteralValue::BoolLiteral(l < r))
            }
            TokenType::LESS_EQUAL => {
                let l = match left_val {
                    LiteralValue::NumberLiteral(l) => l,
                    _ => {
                        return Err(RuntimeError::InvalidOperand(
                            TokenType::MINUS,
                            String::from("Operands must be a number."),
                            binary.operator.line,
                        ))
                    }
                };
                let r = match right_val {
                    LiteralValue::NumberLiteral(r) => r,
                    _ => {
                        return Err(RuntimeError::InvalidOperand(
                            TokenType::MINUS,
                            String::from("Operands must be a number."),
                            binary.operator.line,
                        ))
                    }
                };
                Ok(LiteralValue::BoolLiteral(l <= r))
            }
            TokenType::BANG_EQUAL => Ok(LiteralValue::BoolLiteral(
                !self.is_equal(&left_val, &right_val),
            )),
            TokenType::EQUAL_EQUAL => Ok(LiteralValue::BoolLiteral(
                self.is_equal(&left_val, &right_val),
            )),
            _ => unimplemented!(),
        }
    }

    fn visit_unary(&mut self, unary: &Unary) -> Result<LiteralValue, RuntimeError> {
        let right_val = self.evaluate(&unary.right)?;
        match unary.operator.ttype {
            TokenType::MINUS => match right_val {
                LiteralValue::NumberLiteral(f) => Ok(LiteralValue::NumberLiteral(-f)),
                _ => Err(RuntimeError::InvalidOperand(
                    TokenType::MINUS,
                    String::from("Operand must be a number."),
                    unary.operator.line,
                )),
            },
            TokenType::BANG => Ok(LiteralValue::BoolLiteral(!self.is_true(&right_val))),
            _ => unimplemented!(),
        }
    }

    fn visit_literal(&mut self, literal: &Literal) -> Result<LiteralValue, RuntimeError> {
        Ok(literal.value.clone())
    }

    fn visit_grouping(&mut self, grouping: &Grouping) -> Result<LiteralValue, RuntimeError> {
        self.evaluate(&grouping.expression)
    }

    fn visit_var(&mut self, var: &Var) -> Result<LiteralValue, RuntimeError> {
        Ok(self.environment.get(&var.name)?)
    }
}

impl StmtVisitor<Result<(), RuntimeError>> for Interpreter {
    fn visit_expr(&mut self, expr: &ExprStmtInner) -> Result<(), RuntimeError> {
        self.evaluate(expr.0.as_ref())?;
        Ok(())
    }

    fn visit_print(&mut self, print: &PrintStmtInner) -> Result<(), RuntimeError> {
        let rst = self.evaluate(print.0.as_ref())?;
        println!("{rst}");
        Ok(())
    }

    fn visit_var(&mut self, var: &VarStmtInner) -> Result<(), RuntimeError> {
        let val = match &var.1 {
            Some(expr) => Some(self.evaluate(expr.as_ref())?),
            None => None,
        };
        self.environment.define(&var.0.lexeme, val);
        Ok(())
    }
}
