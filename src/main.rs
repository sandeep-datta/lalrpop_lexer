use anyhow::{Result, bail};
use test_tokens::TestToken;
use std::{fs};

#[macro_use]
extern crate lalrpop_util;

mod test_tokens;
lalrpop_mod!(pub test_lexer); // synthesized by LALRPOP

fn lex<'a>(input: &'a str) -> Result<Vec<TestToken<'a>>> {
    let stream_parser = test_lexer::StreamParser::new();
    let stream = stream_parser.parse(input)?;

    // for tok in stream {
    //     println!("{:?}", tok);
    // }

    Ok(stream)
}

fn main() -> Result<()> {
    let file_name = "test_input/Test.input";
    eprintln!("Lexing: {}\n", file_name);
    let input = fs::read_to_string(file_name)?;
    lex(&input)?;
    Ok(())
}
