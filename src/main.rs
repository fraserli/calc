mod evaluate;
mod lexer;
mod parser;

use anyhow::{Context, Result};

use std::io::Write;

fn evaluate(text: &str) -> Result<String> {
    let tokens = lexer::lex(text).context("Failed to tokenize input")?;
    let expr = parser::parse(tokens).context("Failed to parse expression")?;
    Ok(expr.eval().to_string())
}

fn main() -> Result<()> {
    loop {
        print!("> ");
        std::io::stdout().flush()?;

        let mut line = String::new();
        let bytes = std::io::stdin().read_line(&mut line)?;

        if bytes == 0 {
            println!("\nExiting...");
            break;
        } else if !line.trim().is_empty() {
            match evaluate(&line) {
                Ok(output) => {
                    println!("    = {output}");
                }
                Err(error) => println!("    {:#}", error),
            }
        }
    }

    Ok(())
}
