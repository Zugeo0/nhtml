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
    let mut scanner = Scanner::new(input);
    let elems = Parser::parse(&mut scanner)?;
    let res = emitter::emit_html(elems);

    Ok(res)
}

fn main() -> anyhow::Result<()> {
    let res = transpile(include_str!("../test.nhtml"))?;

    std::fs::write("test.html", res)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::transpile;

    #[test]
    fn test_html_template() {
        let src = r#"
            <!DOCTYPE html>

            html lang="en" {
              head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                meta http-equiv="X-UA-Compatible" content="ie=edge";
                title "HTML 5 Boilerplate"
                link rel="stylesheet" href="style.css";
              }
              body {
            	script src="index.js";
            
                p "Hello App!"
              }
            }
        "#;

        let expect = r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <meta http-equiv="X-UA-Compatible" content="ie=edge">
        <title>
            HTML 5 Boilerplate
        </title>
        <link rel="stylesheet" href="style.css">
    </head>
    <body>
        <script src="index.js"></script>
        <p>
            Hello App!
        </p>
    </body>
</html>
"#;

        let res = transpile(src).unwrap();
    
        assert_eq!(&res, expect)
    }
}
