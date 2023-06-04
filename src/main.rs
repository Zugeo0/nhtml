mod position;
mod scanner;
mod parser;
mod source;
mod token;
mod emitter;

use parser::Parser;
use scanner::Scanner;
use anyhow::Result;

pub fn transpile(input: &str) -> Result<String> {
    let mut scanner = Scanner::new(&input);
    let tags = Parser::parse(&mut scanner)?;
    let res = emitter::emit_html(tags);

    Ok(res)
}

fn main() -> anyhow::Result<()> {
    let res = transpile(&include_str!("../test.nhtml"))?;

    std::fs::write("test.html", res)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{scanner::Scanner, parser::Parser, emitter};

    #[test]
    fn main_test() {
        let mut scanner = Scanner::new(&include_str!("../test.nhtml"));
        let tags = Parser::parse(&mut scanner).unwrap();

        let res = emitter::emit_html(tags);
    
        assert_eq!(&res, include_str!("../test.html"))
    }
}
