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
    let mut ram_address = 16;
    for token in lexer.get_tokens() {
        let bits = parser.parse(token, ram_address);
        binary_writer.write(bits.as_bytes()).unwrap();
        ram_address += 1
    }
}


#[cfg(test)]
mod test {
    use crate::lexer::Lexer;

    #[test]
    fn create_tokens() {
        let code = String::from(
            "// Hello this is a sample comment in the file

                @100 // Load 100
                D=D-A

                D;JGT
                "
        );
        let lexer = Lexer::new(code);

        assert_eq!(
            lexer.get_tokens(),
            &vec!["@100".to_string(), "D=D-A".to_string(), "D;JGT".to_string()]
        )
    }
}