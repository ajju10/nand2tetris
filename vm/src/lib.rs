use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use crate::parser::{Parser, Token};

pub mod parser;
pub mod code_writer;

pub fn init() {
    let args: Vec<_> = env::args().collect();
    let file_name = &args[1];
    let output_file = File::create("main.asm").unwrap();
    let tokens = read_file(file_name);
    let mut parser = Parser::new(output_file, file_name);
    parser.write_bootstrap_code();
    for token in tokens.into_iter() {
        parser.parse(&token)
    }
}


fn read_file(filename: &str) -> Vec<Token> {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();

    let clean_lines: String = contents.lines()
        .filter_map(|line| {
            if line.contains("//") {
                if line.starts_with("//") {
                    None
                } else {
                    let comment_index = line
                        .trim()
                        .chars()
                        .position(|x| x == '/')
                        .unwrap();

                    let command: String = line
                        .chars()
                        .take(comment_index)
                        .collect();

                    Some(command + "\n")
                }
            } else {
                if line.is_empty() {
                    None
                } else {
                    Some(line.to_string() + "\n")
                }
            }
        })
        .collect();

    let tokens = clean_lines.trim().lines()
        .filter_map(|line| {
            if line.contains("push") {
                Some(Token::CPush(String::from(line.trim())))
            } else if line.contains("pop") {
                Some(Token::CPop(String::from(line.trim())))
            } else if line.contains("add") ||
                line.contains("sub") ||
                line.contains("eq") ||
                line.contains("lt") ||
                line.contains("gt") {
                Some(Token::CArithmetic(String::from(line.trim())))
            } else if line.contains("label") {
                Some(Token::CLabel(String::from(line.trim())))
            } else if line.contains("goto") {
                if line.contains("if") {
                    Some(Token::CIfGoTo(String::from(line.trim())))
                } else {
                    Some(Token::CGoto(String::from(line.trim())))
                }
            } else if line.contains("function") {
                Some(Token::CFunction(String::from(line.trim())))
            } else if line.contains("return") {
                Some(Token::CReturn(String::from(line.trim())))
            } else if line.contains("call") {
                Some(Token::CCall(String::from(line.trim())))
            } else {
                None
            }
        })
        .collect();

    tokens
}
