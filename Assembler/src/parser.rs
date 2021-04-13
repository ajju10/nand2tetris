use std::collections::HashMap;

use crate::lexer::{Instruction, Token};

#[derive(Debug)]
pub struct Parser<'a> {
    comp_bits: HashMap<String, &'a str>,
    jump_bits: HashMap<String, &'a str>,
    dest_bits: HashMap<String, &'a str>,
    symbol_table: HashMap<String, i32>,
}

impl Parser<'_> {
    pub fn new<'a>() -> Parser<'a> {
        let mut comp_bits = HashMap::new();
        comp_bits.insert("0".to_string(), "0101010");
        comp_bits.insert("1".to_string(), "0111111");
        comp_bits.insert("-1".to_string(), "0111010");
        comp_bits.insert("D".to_string(), "0001100");
        comp_bits.insert("A".to_string(), "0110000");
        comp_bits.insert("!D".to_string(), "0001101");
        comp_bits.insert("!A".to_string(), "0110001");
        comp_bits.insert("-D".to_string(), "0001111");
        comp_bits.insert("-A".to_string(), "0110011");
        comp_bits.insert("D+1".to_string(), "0011111");
        comp_bits.insert("A+1".to_string(), "0110111");
        comp_bits.insert("D-1".to_string(), "0001110");
        comp_bits.insert("A-1".to_string(), "0110010");
        comp_bits.insert("D+A".to_string(), "0000010");
        comp_bits.insert("D-A".to_string(), "0010011");
        comp_bits.insert("A-D".to_string(), "0000111");
        comp_bits.insert("D&A".to_string(), "0000000");
        comp_bits.insert("D|A".to_string(), "0010101");
        comp_bits.insert("M".to_string(), "1110000");
        comp_bits.insert("!M".to_string(), "1110001");
        comp_bits.insert("-M".to_string(), "1110011");
        comp_bits.insert("M+1".to_string(), "1110111");
        comp_bits.insert("M-1".to_string(), "1110010");
        comp_bits.insert("D+M".to_string(), "1000010");
        comp_bits.insert("D-M".to_string(), "1010011");
        comp_bits.insert("M-D".to_string(), "1000111");
        comp_bits.insert("D&M".to_string(), "1000000");
        comp_bits.insert("D|M".to_string(), "1010101");


        let mut jump_bits = HashMap::new();
        jump_bits.insert("null".to_string(), "000");
        jump_bits.insert("JGT".to_string(), "001");
        jump_bits.insert("JEQ".to_string(), "010");
        jump_bits.insert("JGE".to_string(), "011");
        jump_bits.insert("JLT".to_string(), "100");
        jump_bits.insert("JNE".to_string(), "101");
        jump_bits.insert("JLE".to_string(), "110");
        jump_bits.insert("JMP".to_string(), "111");

        let mut dest_bits = HashMap::new();
        dest_bits.insert("null".to_string(), "000");
        dest_bits.insert("M".to_string(), "001");
        dest_bits.insert("D".to_string(), "010");
        dest_bits.insert("MD".to_string(), "011");
        dest_bits.insert("A".to_string(), "100");
        dest_bits.insert("AM".to_string(), "101");
        dest_bits.insert("AD".to_string(), "110");
        dest_bits.insert("AMD".to_string(), "111");

        let mut symbol_table = HashMap::new();
        symbol_table.insert("SP".to_string(), 0);
        symbol_table.insert("LCL".to_string(), 1);
        symbol_table.insert("ARG".to_string(), 2);
        symbol_table.insert("THIS".to_string(), 3);
        symbol_table.insert("THAT".to_string(), 4);
        symbol_table.insert("R0".to_string(), 0);
        symbol_table.insert("R1".to_string(), 1);
        symbol_table.insert("R2".to_string(), 2);
        symbol_table.insert("R3".to_string(), 3);
        symbol_table.insert("R4".to_string(), 4);
        symbol_table.insert("R5".to_string(), 5);
        symbol_table.insert("R6".to_string(), 6);
        symbol_table.insert("R7".to_string(), 7);
        symbol_table.insert("R8".to_string(), 8);
        symbol_table.insert("R9".to_string(), 9);
        symbol_table.insert("R10".to_string(), 10);
        symbol_table.insert("R11".to_string(), 11);
        symbol_table.insert("R12".to_string(), 12);
        symbol_table.insert("R13".to_string(), 13);
        symbol_table.insert("R14".to_string(), 14);
        symbol_table.insert("R15".to_string(), 15);
        symbol_table.insert("SCREEN".to_string(), 16384);
        symbol_table.insert("KBD".to_string(), 24576);

        Parser {
            comp_bits,
            jump_bits,
            dest_bits,
            symbol_table,
        }
    }

    pub fn parse(&mut self, tokens: &Vec<Token>) -> Vec<String> {
        let mut ram_address = 16;
        let mut bitcode = Vec::new();
        let mut opcode = String::new();
        for token in tokens.into_iter() {
            match token.get_token() {
                Instruction::LInstruction(_) => {}
                Instruction::AInstruction(a_token) => {
                    if a_token.chars().nth(1).unwrap().is_numeric() {
                        let mut addr = format!("{:b}", a_token.replace("@", "")
                            .trim().parse::<u16>().unwrap());
                        for _ in 0..(16 - addr.len()) {
                            addr.insert(0, '0');
                        }
                        opcode.push_str(&addr);
                    } else {
                        // The tokens is a variable
                        opcode.push('0');
                        let var = a_token.replace("@", "").trim().to_string();
                        if self.contains_symbol(&var) {
                            let bits = format!("{:0>15b}", self.get_address(&var));
                            opcode.push_str(&bits)
                        } else {
                            self.insert_symbol(&var, ram_address);
                            let bits = format!("{:0>15b}", self.get_address(&var));
                            opcode.push_str(&bits);
                            ram_address += 1;
                        }
                    }
                }
                Instruction::CInstruction(c_token) => {
                    opcode.push_str("111");
                    let comp_bit = self.get_comp_bits(c_token);
                    let dest_bit = self.get_dest_bits(c_token);
                    let jump_bit = self.get_jump_bits(c_token);
                    opcode = opcode + &comp_bit + &dest_bit + &jump_bit;
                }
            }
            if opcode != "" {
                opcode.push_str("\n");
                bitcode.push(opcode.to_string());
            }
            opcode.clear()
        }
        bitcode
    }

    pub fn first_pass(&mut self, tokens: &Vec<Token>) {
        let mut program_counter = 0;
        for token in tokens {
            match token.get_token() {
                Instruction::LInstruction(label) => {
                    let label = label.strip_prefix("(").unwrap()
                        .strip_suffix(")").unwrap();
                    self.insert_symbol(label, program_counter);
                }
                _ => program_counter += 1,
            }
        }
    }

    fn get_comp_bits(&self, token: &String) -> String {
        if token.contains(";") {
            let mut bit = token.split(";");
            return String::from(*self.comp_bits.get(bit.nth(0).unwrap()).unwrap());
        }

        let mut bit = token.split("=");
        String::from(*self.comp_bits.get(bit.nth(1).unwrap()).unwrap())
    }

    fn get_dest_bits(&self, token: &String) -> String {
        if token.contains(";") {
            return String::from(*self.dest_bits.get("null").unwrap());
        }

        let mut bit = token.split("=");
        String::from(*self.dest_bits.get(bit.nth(0).unwrap()).unwrap())
    }

    fn get_jump_bits(&self, token: &String) -> String {
        if token.contains("=") {
            return String::from(*self.jump_bits.get("null").unwrap());
        }

        let mut bit = token.split(";");
        String::from(*self.jump_bits.get(bit.nth(1).unwrap()).unwrap())
    }

    fn get_address(&self, symbol: &str) -> &i32 {
        let addr = self.symbol_table.get(symbol).unwrap();
        addr
    }

    fn contains_symbol(&self, symbol: &str) -> bool {
        self.symbol_table.contains_key(symbol)
    }

    fn insert_symbol(&mut self, symbol: &str, address: i32) {
        self.symbol_table.insert(symbol.to_string(), address);
    }
}