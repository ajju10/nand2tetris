pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(code: String) -> Lexer {
        let mut tokens = Vec::new();
        for line in code.lines() {
            match preprocess_code(String::from(line)) {
                Some(clean_line) => tokens.push(Token::new(clean_line)),
                None => continue,
            };
        }
        Lexer { tokens }
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
}

#[derive(Debug)]
pub struct Token {
    token: Instruction,
}

#[derive(Debug)]
pub enum Instruction {
    LInstruction(String),
    AInstruction(String),
    CInstruction(String),
}


impl Token {
    fn new(clean_line: String) -> Token {
        if clean_line.starts_with("@") {
            Token {
                token: Instruction::AInstruction(String::from(clean_line.trim())),
            }
        } else if clean_line.starts_with("(") {
            Token {
                token: Instruction::LInstruction(String::from(clean_line.trim()))
            }
        } else {
            Token {
                token: Instruction::CInstruction(String::from(clean_line.trim()))
            }
        }
    }

    pub fn get_token(&self) -> &Instruction {
        &self.token
    }
}


fn preprocess_code(line: String) -> Option<String> {
    let trimmed_line = line.trim();

    return if trimmed_line.contains("//") {
        if trimmed_line.starts_with("//") {
            None
        } else {
            let comment_index = trimmed_line
                .chars()
                .position(|x| x == '/')
                .unwrap();

            let line_no_comment = trimmed_line
                .chars()
                .take(comment_index - 1)
                .collect::<String>();

            Some(line_no_comment)
        }
    } else {
        if trimmed_line.is_empty() {
            None
        } else { Some(String::from(trimmed_line)) }
    };
}