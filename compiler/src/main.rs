pub mod lexer;

fn main() {
    // let mut cur_tok : lexer::Token;
    let mut lex = lexer::Lexer::new();
    lex.gettok();
    println!("ready>");
}
