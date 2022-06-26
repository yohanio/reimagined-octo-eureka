use rustyline::Editor;
use rustyline::error::ReadlineError;
use interpreter_in_rs::lexer::lexer::Lexer;
use interpreter_in_rs::token::token::Token;

fn main() {
    let mut rl = Editor::<()>::new();

    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                rl.add_history_entry(&line);

                let mut lexer = Lexer::new(&line);

                loop {
                    let tok = lexer.next_token();
                    if tok == Token::EOF {
                        break;
                    }
                    println!("{:?}", tok);
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("REPL : Bye!");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!();
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }
}
