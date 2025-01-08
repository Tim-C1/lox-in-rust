use crate::callable::*;
use crate::statement::*;
use crate::token::*;
use crate::{environment::*, expression::ExprAccept};
use crate::{expression::*, statement::StmtAccept};
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

pub struct Interpreter {
    pub environment: Rc<RefCell<Environment>>,
}

pub enum RuntimeException {
    InvalidOperand(TokenType, String, usize),
    UndefinedVar(Token),
    InvalidCallable(Token, String),
    UnmatchedArity(usize, usize),
    FunctionReturn(Option<CallableRet>),
}
impl fmt::Display for RuntimeException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidOperand(_, desc, line) => {
                write!(f, "{}\n[line {}]", desc, line)
            }
            Self::UndefinedVar(v) => {
                write!(f, "Undefined variable '{}'.", v.lexeme)
            }
            Self::InvalidCallable(_, _) => {
                write!(f, "Can only call functions and classes.")
            }
            Self::UnmatchedArity(expected, got) => {
                write!(f, "Expected {expected} arguments but got {got}.")
            }
            Self::FunctionReturn(_) => {
                todo!()
            }
        }
    }
}
impl Interpreter {
    pub fn new() -> Self {
        let globals = Environment::new();
        globals.borrow_mut().define(
            "clock",
            Some(CallableRet::Callable(Callable::Native(Clock))),
        );
        Interpreter {
            environment: globals,
        }
    }
    pub fn evaluate(&mut self, expr: &Expr) -> Result<CallableRet, RuntimeException> {
        expr.accept(self)
    }
    pub fn interprete(&mut self, stmts: &Vec<Stmt>) -> Result<(), RuntimeException> {
        for stmt in stmts {
            self.execute(stmt)?
        }
        Ok(())
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeException> {
        stmt.accept(self)
    }

    pub fn execute_block(
        &mut self,
        stmts: &BlockStmtInner,
        block_env: Environment,
    ) -> Result<(), RuntimeException> {
        let prev_env = self.environment.clone();
        self.environment = Rc::new(RefCell::new(block_env));
        for stmt in &stmts.0 {
            match self.execute(stmt.as_ref()) {
                Ok(_) => continue,
                Err(e) => {
                    self.environment = prev_env;
                    return Err(e);
                }
            }
        }
        self.environment = prev_env;
        Ok(())
    }

    fn is_true(&self, literal_value: &CallableRet) -> bool {
        match literal_value {
            CallableRet::Value(LiteralValue::NumberLiteral(_))
            | CallableRet::Value(LiteralValue::StringLiteral(_)) => true,
            CallableRet::Value(LiteralValue::BoolLiteral(b)) => *b,
            CallableRet::Value(LiteralValue::NilLiteral) => false,
            CallableRet::Callable(_) => unimplemented!("trusty of callable unimplemented!"),
        }
    }

    fn is_equal(&mut self, l: &CallableRet, r: &CallableRet) -> bool {
        if matches!(l, CallableRet::Value(LiteralValue::NilLiteral)) {
            if matches!(r, CallableRet::Value(LiteralValue::NilLiteral)) {
                true
            } else {
                false
            }
        } else {
            match l {
                CallableRet::Value(LiteralValue::NumberLiteral(l)) => match r {
                    CallableRet::Value(LiteralValue::NumberLiteral(r)) => l == r,
                    _ => false,
                },
                CallableRet::Value(LiteralValue::BoolLiteral(l)) => match r {
                    CallableRet::Value(LiteralValue::BoolLiteral(r)) => l == r,
                    _ => false,
                },
                CallableRet::Value(LiteralValue::StringLiteral(l)) => match r {
                    CallableRet::Value(LiteralValue::StringLiteral(r)) => l == r,
                    _ => false,
                },
                _ => unreachable!(),
            }
        }
    }
}
impl ExprVisitor<Result<CallableRet, RuntimeException>> for Interpreter {
    fn visit_binary(&mut self, binary: &Binary) -> Result<CallableRet, RuntimeException> {
        let left_val = self.evaluate(&binary.left)?;
        let right_val = self.evaluate(&binary.right)?;
        match binary.operator.ttype {
            TokenType::MINUS => {
                let l = match left_val {
                    CallableRet::Value(LiteralValue::NumberLiteral(l)) => l,
                    _ => {
                        return Err(RuntimeException::InvalidOperand(
                            TokenType::MINUS,
                            String::from("Operands must be a number."),
                            binary.operator.line,
                        ))
                    }
                };
                let r = match right_val {
                    CallableRet::Value(LiteralValue::NumberLiteral(r)) => r,
                    _ => {
                        return Err(RuntimeException::InvalidOperand(
                            TokenType::MINUS,
                            String::from("Operands must be a number."),
                            binary.operator.line,
                        ))
                    }
                };
                Ok(CallableRet::Value(LiteralValue::NumberLiteral(l - r)))
            }
            TokenType::PLUS => match left_val {
                CallableRet::Value(LiteralValue::NumberLiteral(l)) => {
                    match right_val {
                        CallableRet::Value(LiteralValue::NumberLiteral(r)) => {
                            return Ok(CallableRet::Value(LiteralValue::NumberLiteral(l + r)))
                        }
                        _ => {
                            return Err(RuntimeException::InvalidOperand(
                                TokenType::MINUS,
                                String::from("Operands must be two numbers or two strings."),
                                binary.operator.line,
                            ))
                        }
                    };
                }
                CallableRet::Value(LiteralValue::StringLiteral(l)) => {
                    match right_val {
                        CallableRet::Value(LiteralValue::StringLiteral(r)) => {
                            return Ok(CallableRet::Value(LiteralValue::StringLiteral(l + &r)))
                        }
                        _ => {
                            return Err(RuntimeException::InvalidOperand(
                                TokenType::MINUS,
                                String::from("Operands must be two numbers or two strings."),
                                binary.operator.line,
                            ))
                        }
                    };
                }
                _ => {
                    return Err(RuntimeException::InvalidOperand(
                        TokenType::MINUS,
                        String::from("Operands must be two numbers or two strings."),
                        binary.operator.line,
                    ))
                }
            },
            TokenType::STAR => {
                let l = match left_val {
                    CallableRet::Value(LiteralValue::NumberLiteral(l)) => l,
                    _ => {
                        return Err(RuntimeException::InvalidOperand(
                            TokenType::MINUS,
                            String::from("Operands must be a number."),
                            binary.operator.line,
                        ))
                    }
                };
                let r = match right_val {
                    CallableRet::Value(LiteralValue::NumberLiteral(r)) => r,
                    _ => {
                        return Err(RuntimeException::InvalidOperand(
                            TokenType::MINUS,
                            String::from("Operands must be a number."),
                            binary.operator.line,
                        ))
                    }
                };
                Ok(CallableRet::Value(LiteralValue::NumberLiteral(l * r)))
            }
            TokenType::SLASH => {
                let l = match left_val {
                    CallableRet::Value(LiteralValue::NumberLiteral(l)) => l,
                    _ => {
                        return Err(RuntimeException::InvalidOperand(
                            TokenType::MINUS,
                            String::from("Operands must be a number."),
                            binary.operator.line,
                        ))
                    }
                };
                let r = match right_val {
                    CallableRet::Value(LiteralValue::NumberLiteral(r)) => r,
                    _ => {
                        return Err(RuntimeException::InvalidOperand(
                            TokenType::MINUS,
                            String::from("Operands must be a number."),
                            binary.operator.line,
                        ))
                    }
                };
                Ok(CallableRet::Value(LiteralValue::NumberLiteral(l / r)))
            }
            TokenType::GREATER => {
                let l = match left_val {
                    CallableRet::Value(LiteralValue::NumberLiteral(l)) => l,
                    _ => {
                        return Err(RuntimeException::InvalidOperand(
                            TokenType::MINUS,
                            String::from("Operands must be a number."),
                            binary.operator.line,
                        ))
                    }
                };
                let r = match right_val {
                    CallableRet::Value(LiteralValue::NumberLiteral(r)) => r,
                    _ => {
                        return Err(RuntimeException::InvalidOperand(
                            TokenType::MINUS,
                            String::from("Operands must be a number."),
                            binary.operator.line,
                        ))
                    }
                };
                Ok(CallableRet::Value(LiteralValue::BoolLiteral(l > r)))
            }
            TokenType::GREATER_EQUAL => {
                let l = match left_val {
                    CallableRet::Value(LiteralValue::NumberLiteral(l)) => l,
                    _ => {
                        return Err(RuntimeException::InvalidOperand(
                            TokenType::MINUS,
                            String::from("Operands must be a number."),
                            binary.operator.line,
                        ))
                    }
                };
                let r = match right_val {
                    CallableRet::Value(LiteralValue::NumberLiteral(r)) => r,
                    _ => {
                        return Err(RuntimeException::InvalidOperand(
                            TokenType::MINUS,
                            String::from("Operands must be a number."),
                            binary.operator.line,
                        ))
                    }
                };
                Ok(CallableRet::Value(LiteralValue::BoolLiteral(l >= r)))
            }
            TokenType::LESS => {
                let l = match left_val {
                    CallableRet::Value(LiteralValue::NumberLiteral(l)) => l,
                    _ => {
                        return Err(RuntimeException::InvalidOperand(
                            TokenType::MINUS,
                            String::from("Operands must be a number."),
                            binary.operator.line,
                        ))
                    }
                };
                let r = match right_val {
                    CallableRet::Value(LiteralValue::NumberLiteral(r)) => r,
                    _ => {
                        return Err(RuntimeException::InvalidOperand(
                            TokenType::MINUS,
                            String::from("Operands must be a number."),
                            binary.operator.line,
                        ))
                    }
                };
                Ok(CallableRet::Value(LiteralValue::BoolLiteral(l < r)))
            }
            TokenType::LESS_EQUAL => {
                let l = match left_val {
                    CallableRet::Value(LiteralValue::NumberLiteral(l)) => l,
                    _ => {
                        return Err(RuntimeException::InvalidOperand(
                            TokenType::MINUS,
                            String::from("Operands must be a number."),
                            binary.operator.line,
                        ))
                    }
                };
                let r = match right_val {
                    CallableRet::Value(LiteralValue::NumberLiteral(r)) => r,
                    _ => {
                        return Err(RuntimeException::InvalidOperand(
                            TokenType::MINUS,
                            String::from("Operands must be a number."),
                            binary.operator.line,
                        ))
                    }
                };
                Ok(CallableRet::Value(LiteralValue::BoolLiteral(l <= r)))
            }
            TokenType::BANG_EQUAL => Ok(CallableRet::Value(LiteralValue::BoolLiteral(
                !self.is_equal(&left_val, &right_val),
            ))),
            TokenType::EQUAL_EQUAL => Ok(CallableRet::Value(LiteralValue::BoolLiteral(
                self.is_equal(&left_val, &right_val),
            ))),
            _ => unimplemented!(),
        }
    }

    fn visit_unary(&mut self, unary: &Unary) -> Result<CallableRet, RuntimeException> {
        let right_val = self.evaluate(&unary.right)?;
        match unary.operator.ttype {
            TokenType::MINUS => match right_val {
                CallableRet::Value(LiteralValue::NumberLiteral(f)) => {
                    Ok(CallableRet::Value(LiteralValue::NumberLiteral(-f)))
                }
                _ => Err(RuntimeException::InvalidOperand(
                    TokenType::MINUS,
                    String::from("Operand must be a number."),
                    unary.operator.line,
                )),
            },
            TokenType::BANG => Ok(CallableRet::Value(LiteralValue::BoolLiteral(
                !self.is_true(&right_val),
            ))),
            _ => unimplemented!(),
        }
    }

    fn visit_literal(&mut self, literal: &Literal) -> Result<CallableRet, RuntimeException> {
        Ok(CallableRet::Value(literal.value.clone()))
    }

    fn visit_grouping(&mut self, grouping: &Grouping) -> Result<CallableRet, RuntimeException> {
        self.evaluate(&grouping.expression)
    }

    fn visit_var(&mut self, var: &Var) -> Result<CallableRet, RuntimeException> {
        Ok(self.environment.borrow().get(&var.name)?)
    }

    fn visit_assignment(
        &mut self,
        assignment: &Assignment,
    ) -> Result<CallableRet, RuntimeException> {
        let value = self.evaluate(assignment.value.as_ref())?;
        RefCell::borrow_mut(&self.environment).assign(&assignment.name, value.clone())?;
        Ok(value)
    }

    fn visit_logical(&mut self, logical: &Logical) -> Result<CallableRet, RuntimeException> {
        let left = self.evaluate(&logical.left)?;
        if logical.operator.ttype == TokenType::OR {
            if self.is_true(&left) {
                return Ok(left);
            }
        } else {
            if !self.is_true(&left) {
                return Ok(left);
            }
        }
        self.evaluate(&logical.right)
    }

    fn visit_call(&mut self, call: &Call) -> Result<CallableRet, RuntimeException> {
        let callee = self.evaluate(&call.callee)?;
        let mut arguments = Vec::new();
        for arg in &call.arguments {
            arguments.push(self.evaluate(arg.as_ref())?);
        }
        match callee {
            CallableRet::Callable(mut function) => {
                if arguments.len() != function.arity() {
                    Err(RuntimeException::UnmatchedArity(
                        function.arity(),
                        arguments.len(),
                    ))
                } else {
                    function.call(self, &arguments)
                }
            }
            CallableRet::Value(_) => Err(RuntimeException::InvalidCallable(
                call.paren.clone(),
                String::from("Can only call functions and classes"),
            )),
        }
    }
}

impl StmtVisitor<Result<(), RuntimeException>> for Interpreter {
    fn visit_expr(&mut self, expr: &ExprStmtInner) -> Result<(), RuntimeException> {
        self.evaluate(expr.0.as_ref())?;
        Ok(())
    }

    fn visit_print(&mut self, print: &PrintStmtInner) -> Result<(), RuntimeException> {
        let rst = self.evaluate(print.0.as_ref())?;
        match rst {
            CallableRet::Value(val) => Ok(println!("{val}")),
            CallableRet::Callable(func) => Ok(println!("{func}")),
        }
    }

    fn visit_var(&mut self, var: &VarStmtInner) -> Result<(), RuntimeException> {
        let val = match &var.1 {
            Some(expr) => Some(self.evaluate(expr.as_ref())?),
            None => None,
        };
        RefCell::borrow_mut(&self.environment).define(&var.0.lexeme, val);
        Ok(())
    }

    fn visit_block(&mut self, stmts: &BlockStmtInner) -> Result<(), RuntimeException> {
        let block_env = Environment::new_with_enclosing(&self.environment);
        self.execute_block(stmts, block_env)
    }

    fn visit_if(&mut self, branch: &IfStmtInner) -> Result<(), RuntimeException> {
        let condition = self.evaluate(&branch.condition)?;
        if self.is_true(&condition) {
            self.execute(&branch.then_branch)
        } else {
            match branch.else_branch {
                Some(ref else_branch) => self.execute(else_branch),
                None => Ok(()),
            }
        }
    }

    fn visit_while(&mut self, while_stmt: &WhileStmtInner) -> Result<(), RuntimeException> {
        let mut condition = self.evaluate(while_stmt.condition.as_ref())?;
        while self.is_true(&condition) {
            self.execute(while_stmt.body.as_ref())?;
            condition = self.evaluate(while_stmt.condition.as_ref())?;
        }
        Ok(())
    }

    fn visit_function(&mut self, func_stmt: &FunctionStmtInner) -> Result<(), RuntimeException> {
        let func = FunctionInner::new(func_stmt, self.environment.clone());
        Ok(self.environment.borrow_mut().define(
            &func_stmt.name.lexeme,
            Some(CallableRet::Callable(Callable::Function(func))),
        ))
    }

    fn visit_return(&mut self, return_stmt: &ReturnStmtInner) -> Result<(), RuntimeException> {
        match &return_stmt.value {
            Some(value) => Err(RuntimeException::FunctionReturn(Some(
                self.evaluate(&value)?,
            ))),
            None => Err(RuntimeException::FunctionReturn(None)),
        }
    }
}
