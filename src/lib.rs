use std::{env, fs};
use std::io::BufWriter;
use std::io::prelude::*;

use lexer::Lexer;
use parser::Parser;

pub mod lexer;
pub mod parser;

pub fn assemble() {
    let args: Vec<_> = env::args().collect();
    let file_name = &args[1];
    let code = fs::read_to_string(file_name)
        .expect("Could not open file");
    let binary_file = fs::File::create(
        file_name.replace(".asm", ".hack"))
        .unwrap();
    let lexer = Lexer::new(code);
    let mut parser = Parser::new();

    parser.first_pass(lexer.get_tokens());

    let mut binary_writer = BufWriter::new(binary_file);
    let bits = parser.parse(lexer.get_tokens());
    for bit in bits.into_iter() {
        if bit == "" { continue; }
        binary_writer.write(bit.as_bytes()).unwrap();
    }
}
