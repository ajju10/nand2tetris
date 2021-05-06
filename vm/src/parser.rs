use std::fs::File;

use crate::code_writer::CodeWriter;

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    CArithmetic(String),
    CPush(String),
    CPop(String),
    CLabel(String),
    CGoto(String),
    CIfGoTo(String),
    CFunction(String),
    CReturn(String),
    CCall(String),
    Error,
}

pub struct Parser {
    writer: CodeWriter
}

impl Parser {
    pub fn new(output_file: File, file_name: &str) -> Self {
        Self {
            writer: CodeWriter::new(output_file, file_name)
        }
    }

    pub fn write_bootstrap_code(&mut self) {
        self.writer.write_init();
    }

    pub fn parse(&mut self, token: &Token) {
        match token {
            Token::CArithmetic(command) => {
                self.writer.write_arithmetic(command);
            }
            Token::CPush(command) => {
                let segment = arg1(token).unwrap();
                let value = arg2(token).unwrap();
                self.writer.write_push_pop(command, segment, value.parse().unwrap());
            }
            Token::CPop(command) => {
                let segment = arg1(token).unwrap();
                let value = arg2(token).unwrap();
                self.writer.write_push_pop(command, segment, value.parse().unwrap());
            }
            Token::CLabel(_) => {
                let label = arg1(token).unwrap();
                self.writer.write_label(label);
            }
            Token::CGoto(_) => {
                let label = arg1(token).unwrap();
                self.writer.write_goto(label)
            }
            Token::CIfGoTo(_) => {
                let label = arg1(token).unwrap();
                self.writer.write_if_goto(label)
            }
            Token::CFunction(_) => {
                let function_name = arg1(token).unwrap();
                let num_locals = arg2(token).unwrap();
                self.writer.write_function(
                    function_name,
                    num_locals.parse().unwrap()
                )
            }
            Token::CReturn(_) => {
                self.writer.write_return()
            }
            Token::CCall(_) => {
                let function_name = arg1(token).unwrap();
                let args = arg2(token).unwrap();
                self.writer.write_call(
                    function_name,
                    args.parse().unwrap()
                )
            }
            _ => panic!("Can't recognize command!")
        }
    }
}

fn arg1(token: &Token) -> Option<&str> {
    match token {
        Token::CArithmetic(token) => Some(token),
        Token::CPush(token) => {
            let arg = token.split_whitespace().nth(1).unwrap();
            Some(arg)
        }
        Token::CPop(token) => {
            let arg = token.split_whitespace().nth(1).unwrap();
            Some(arg)
        }
        Token::CLabel(token) => {
            let arg = token.split_whitespace().nth(1).unwrap();
            Some(arg)
        }
        Token::CGoto(token) => {
            let arg = token.split_whitespace().nth(1).unwrap();
            Some(arg)
        }
        Token::CIfGoTo(token) => {
            let arg = token.split_whitespace().nth(1).unwrap();
            Some(arg)
        }
        Token::CFunction(token) => {
            let arg = token.split_whitespace().nth(1).unwrap();
            Some(arg)
        }
        _ => panic!("Can't recognize command!"),
    }
}


fn arg2(token: &Token) -> Option<&str> {
    match token {
        Token::CArithmetic(_) => None,
        Token::CPush(token) => {
            let arg = token.split_whitespace().nth(2).unwrap();
            Some(arg)
        }
        Token::CPop(token) => {
            let arg = token.split_whitespace().nth(2).unwrap();
            Some(arg)
        }
        Token::CFunction(token) => {
            let arg = token.split_whitespace().nth(2).unwrap();
            Some(arg)
        }
        Token::CCall(token) => {
            let arg = token.split_whitespace().nth(2).unwrap();
            Some(arg)
        }
        _ => panic!("Can't recognize command!")
    }
}
