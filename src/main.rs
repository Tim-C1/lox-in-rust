use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

use codecrafters_interpreter::callable::*;
use codecrafters_interpreter::expression::ast_printer::AstPrinter;
use codecrafters_interpreter::interpreter::*;
use codecrafters_interpreter::parser::*;
use codecrafters_interpreter::scanner::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            let mut scanner = Scanner::new(file_contents.trim_end());
            scanner.scan_tokens();
            scanner.print_tokens();
            match &scanner.status {
                ScannerStatus::ScanSuccess => exit(0),
                ScannerStatus::UnknowCharErr | ScannerStatus::NonTerminatedStringErr => exit(65),
            }
        }
        "parse" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });
            let mut scanner = Scanner::new(file_contents.trim_end());
            scanner.scan_tokens();
            match &scanner.status {
                ScannerStatus::ScanSuccess => {}
                ScannerStatus::UnknowCharErr | ScannerStatus::NonTerminatedStringErr => exit(65),
            }
            let mut parser = Parser::new(scanner.tokens);
            let expr = parser.parse_expr();
            match expr {
                Ok(expr) => {
                    let mut printer = AstPrinter;
                    printer.print(expr.as_ref());
                }
                Err(_) => exit(65),
            }
        }
        "evaluate" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });
            let mut scanner = Scanner::new(file_contents.trim_end());
            scanner.scan_tokens();
            match &scanner.status {
                ScannerStatus::ScanSuccess => {}
                ScannerStatus::UnknowCharErr | ScannerStatus::NonTerminatedStringErr => exit(65),
            }
            let mut parser = Parser::new(scanner.tokens);
            let expr = parser.parse_expr();
            match expr {
                Ok(expr) => {
                    let mut evaluator = Interpreter::new();
                    match evaluator.evaluate(&expr) {
                        Ok(ret) => match ret {
                            CallableRet::Value(val) => println!("{val}"),
                            CallableRet::Callable(_) => unimplemented!(),
                        },
                        Err(e) => {
                            eprintln!("{e}");
                            exit(70);
                        }
                    }
                }
                Err(_) => exit(65),
            }
        }
        "run" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });
            let mut scanner = Scanner::new(file_contents.trim_end());
            scanner.scan_tokens();
            match &scanner.status {
                ScannerStatus::ScanSuccess => {}
                _ => exit(65),
            }
            let mut parser = Parser::new(scanner.tokens);
            let stmts = parser.parse();
            match parser.status {
                ParserStatus::Success => {
                    let mut interpreter = Interpreter::new();
                    match interpreter.interprete(&stmts) {
                        Ok(()) => exit(0),
                        Err(e) => {
                            eprintln!("{e}");
                            exit(70);
                        }
                    }
                }
                ParserStatus::Panic => {
                    exit(65);
                }
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
