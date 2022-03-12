#![allow(unused)]

use std::io::Read;


pub enum Token {
    TokEof = -1,
    TokDef = -2,
    TokExtern = -3,
    TokIdentifier = -4,
    TokNumber = -5
  }

pub struct Lexer;

// let IdentifierStr: string = "";

impl Lexer {
    pub fn gettok() -> Token {
        let mut last_char: Option<u8> = Some(' '  as u8);
        while (char::is_whitespace(last_char.unwrap() as char)) {
            last_char = std::io::stdin()
                .bytes() 
                .next()
                .and_then(|result| result.ok())
                .map(|byte| byte as u8);
            println!("{:?}", last_char);
        }
        return Token::TokEof;
    }
}
