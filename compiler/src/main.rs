mod lexer;

fn main() {
    let mut cur_tok : lexer::Token = lexer::Token::TokDef;
    cur_tok = lexer::Lexer::gettok();
    println!("ready>")
}
