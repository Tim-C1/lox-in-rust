pub mod scanner;
pub mod token;
pub mod expression;
pub mod parser;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;
use scanner::{Scanner, ScannerStatus};
use parser::*;

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
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

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
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });
            let mut scanner = Scanner::new(file_contents.trim_end());
            scanner.scan_tokens();
            // scanner.print_tokens();
            match &scanner.status {
                ScannerStatus::ScanSuccess => {},
                ScannerStatus::UnknowCharErr | ScannerStatus::NonTerminatedStringErr => exit(65),
            }
            let mut parser = Parser::new(scanner.tokens);
            let expression_rst = parser.parse();
            match expression_rst {
                Ok(expr) => {
                    let ast = expr.print();
                    println!("{}", ast);
                }
                Err(_) => exit(65),
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
