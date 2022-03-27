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
        while(next_char.is_whitespace()) {
            let mut lookahead = self.lookahead();

                if (lookahead.is_none()) {
                    println!("eof!");
                    break;
                }
                
            let mut next_char : char = lookahead.unwrap() as char;

            if (next_char.to_string() == "/n") {
                self.line_number += 1;
            }

            if (next_char.is_numeric() || next_char == '.') {
                let mut number_str: String = "".to_string();
                while (next_char.is_numeric() || next_char == '.') {
                    number_str += &next_char.to_string();
                    let temp = self.lookahead();
                    if (temp.is_none()) {
                        println!("eof!");
                        return Token::TokEof;
                    }
                    next_char = temp.unwrap() as char;
                }
                if (number_str.parse::<f64>().is_ok()) {
                    print!("{}\n", number_str.parse::<f64>().unwrap());
                } else {
                    print!("Failed to parse numeric literal {}, line {}\n", number_str, self.line_number);
                }
            }
            
            if (next_char.is_alphabetic()) {
                let mut identifier_str : String = "".to_string();
                while(next_char.is_alphanumeric()) {
                    identifier_str += &next_char.to_string();
                    let temp = self.lookahead();
                    if (temp.is_none()) {
                        println!("eof!");
                        return Token::TokEof;
                    } else {
                        next_char = temp.unwrap() as char;
                    }
                }
                print!("{}\n", identifier_str);
            }
        }
        return Token::TokEof;
    }

    fn lookahead(&mut self) -> Option<u8> {
        let mut lookahead: Option<u8> = std::io::stdin()
            .bytes() 
            .next()
            .and_then(|result| result.ok())
            .map(|byte| byte as u8);
    
        return lookahead;
    }
}

