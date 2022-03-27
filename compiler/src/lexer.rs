#![allow(unused)]

use std::io::Read;

pub enum Token {
    TokEof = -1,
    TokDef = -2,
    TokExtern = -3,
    TokIdentifier = -4,
    TokNumber = -5
}

pub struct Lexer {
    line_number : u32
}

//Lookahead lexer: character byte stream -> tokens
// Will return tokens one-by-one to the parser
// Peeks one character ahead in the byte stream and determines token type,
// then: consumes input until next whitespace
// rejects if it does not pattern match, and accepts if it does.
impl Lexer {

    pub fn new() -> Lexer {
        Lexer {
            line_number : 1
        }
    }

    pub fn gettok(&mut self) -> Token {
        let mut next_char: char = ' ';
        while(next_char == ' ' || next_char == '\n') {
            next_char = self.lookahead();

            if (next_char as u8 == 0) {
                return Token::TokEof;
            }

            if (next_char.is_numeric() || next_char == '.') {
                let mut number_str: String = "".to_string();
                while (next_char.is_numeric() || next_char == '.') {
                    number_str += &next_char.to_string();
                    next_char = self.lookahead();
                }
                if (number_str.parse::<f64>().is_ok()) {
                    print!("{}\n", number_str.parse::<f64>().unwrap());
                } else {
                    print!("Failed to parse numeric literal {}, line {}\n", number_str, self.line_number);
                }
            }
            
            if (next_char.is_alphabetic()) {
                let mut identifier_str : String = "".to_string();
                while(next_char.is_alphabetic()) {
                    identifier_str += &next_char.to_string();
                    next_char = self.lookahead();
                }
                print!("{}\n", identifier_str);
            }
        }
        return Token::TokEof;
    }

    fn lookahead(&mut self) -> char {
        let mut lookahead: Option<u8> = Some(' '  as u8);
        lookahead = std::io::stdin()
            .bytes() 
            .next()
            .and_then(|result| result.ok())
            .map(|byte| byte as u8);
            

        if (lookahead == None) { panic!("Failed to read byte stream."); }
        
            let mut lookahead_char : char = lookahead.unwrap() as char;
    
            if (lookahead_char.to_string() == "/n") {
                self.line_number += 1;
            }
        
            return lookahead_char;
    }
}

