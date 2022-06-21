use std::fs::read_to_string;
use std::path::Path;

mod error;
mod lexer;
mod token;

fn main() {
    let args = std::env::args().into_iter();

    if args.len() < 2 {
        panic!("Expected at least one argument: <source file>")
    }

    let file = args.last().unwrap();
    let file_path = Path::new(&file);
    let source_code = read_to_string(file_path);

    if source_code.is_ok() {
        let mut lexer = lexer::Lexer::new(source_code.unwrap().clone());

        let (errs, tokens) = lexer.scan_tokens();

        if lexer.has_error {
            let errors = errs.unwrap();
            println!("Compiled with {} errors:", errors.len());

            for error in errors.iter() {
                println!("Error - {:?}", error);
            }
        }

        for token in tokens.iter() {
            println!("Token: {:?}", token);
        }

        println!("Tokens lenght: {}", tokens.len());
    } else {
        println!("{:?}", source_code.err().unwrap().to_string())
    }
}
