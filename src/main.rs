use std::{env, error::Error, fs};

pub mod ast;

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().last().unwrap();
    let input = fs::read_to_string(path)?;
    let mut lexer = ast::lexer::Lexer::new(input);

    while let Some(next) = lexer.next() {
        println!("{next:?}")
    }

    Ok(())
}
