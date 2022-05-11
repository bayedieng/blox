use blox::lexer::{Lexer, TokenKind};

fn main() {
    let src = include_str!("../test.lox");
    let mut lexer = Lexer::new(src);
    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        if token.kind == TokenKind::Eof {
            break;
        }
    }
}
