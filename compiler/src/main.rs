pub mod lexer;

fn main() {
    // let mut cur_tok : lexer::Token;
    println!("ready>");
    let mut lex = lexer::Lexer::new();
    lex.gettok();
}
