use std::fmt::Display;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::environment::*;
use crate::interpreter::*;
use crate::statement::*;
use crate::token::LiteralValue;

#[derive(Clone)]
pub enum CallableRet {
    Value(LiteralValue),
    Callable(Callable),
}

#[derive(Clone)]
pub enum Callable {
    Function(FunctionInner),
    Native(Clock),
}

#[derive(Clone)]
pub struct Clock;
#[derive(Clone)]
pub struct FunctionInner {
    pub declaration: FunctionStmtInner,
}

impl FunctionInner {
    pub fn new(declaration: &FunctionStmtInner) -> Self {
        Self {
            declaration: declaration.clone(),
        }
    }
}

impl Callable {
    pub fn arity(&self) -> usize {
        match self {
            Callable::Native(_) => 0,
            Callable::Function(func) => func.declaration.params.len(),
        }
    }
    pub fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: &Vec<CallableRet>,
    ) -> Result<CallableRet, RuntimeException> {
        match self {
            Callable::Native(_) => {
                let now = SystemTime::now();
                let duration_since_epoch = now
                    .duration_since(UNIX_EPOCH)
                    .expect("system time earlier than unix epoch");
                Ok(CallableRet::Value(LiteralValue::NumberLiteral(
                    duration_since_epoch.as_secs_f64()
                        + duration_since_epoch.subsec_nanos() as f64 * 1e-9,
                )))
            }
            Callable::Function(func) => {
                let mut func_env = Environment::new_with_enclosing(&interpreter.environment);
                for i in 0..func.declaration.params.len() {
                    func_env.define(
                        &func.declaration.params[i].lexeme,
                        Some(arguments[i].clone()),
                    );
                }
                match func.declaration.body.as_ref() {
                    Stmt::BlockStmt(func_block) => {
                        match interpreter.execute_block(func_block, func_env) {
                            Ok(_) => Ok(CallableRet::Value(LiteralValue::NilLiteral)),
                            Err(e) => match e {
                                RuntimeException::FunctionReturn(value) => match value {
                                    Some(value) => Ok(value),
                                    None => Ok(CallableRet::Value(LiteralValue::NilLiteral)),
                                },
                                _ => Err(e),
                            },
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}

impl Display for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Callable::Native(_) => write!(f, "<native fn>"),
            Callable::Function(func) => write!(f, "<fn {}>", func.declaration.name.lexeme),
        }
    }
}

impl Display for CallableRet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CallableRet::Value(val) => write!(f, "{}", val),
            CallableRet::Callable(func) => write!(f, "{}", func),
        }
    }
}
