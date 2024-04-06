mod position;
mod scanner;
mod parser;
mod source;
mod token;
mod emitter;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use scanner::Scanner;
use anyhow::{Result, Context};

pub fn transpile(input: &str) -> Result<String> {
    let mut scanner = Scanner::new(input);
    let elems = parser::Parser::parse(&mut scanner)?;
    let res = emitter::emit_html(elems);

    Ok(res)
}


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Convert {
        path: PathBuf,
        output: PathBuf,
    },
    Watch {
        path: PathBuf,
        output: PathBuf,
    }
}

fn transpile_from_to(path: PathBuf, output: PathBuf) -> Result<()> {
    let input = std::fs::read_to_string(path).context("Failed to read input file")?;
    let res = transpile(&input)?;
    std::fs::write(output, res).context("Failed to write transpiled code to file")?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Convert { path, output } => {
            transpile_from_to(path, output)?;
        },
        Commands::Watch { path, output } => {
            transpile_from_to(path.clone(), output.clone())?;
            println!("Watching '{}'. Press CTRL-C to quit", path.to_str().unwrap());
            watch(path, output)?;
        },
    }

    Ok(())
}

fn watch(input: PathBuf, output: PathBuf) -> Result<()> {
    use notify::{Watcher, RecursiveMode, RecommendedWatcher, Config};

    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default()).context("Failed to create watcher")?;
    watcher.watch(input.as_ref(), RecursiveMode::Recursive).context("Failed to start watcher on path")?;

    loop {
        let _ = rx.recv().context("Watcher failed")?;
        transpile_from_to(input.clone(), output.clone())?;
    }
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
                title "HTML 5 Boilerplate";
                link rel="stylesheet" href="style.css";
              }

              // Main body
              body {
                script src="index.js";
                a class="test" "Hello";
                a class='test' 'Hello';
            
                p "Hello App!";
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
        <a class="test">
            Hello
        </a>
        <a class='test'>
            Hello
        </a>
        <p>
            Hello App!
        </p>
    </body>
</html>
"#;

        let res = transpile(src).unwrap();
        let mut expect_lines = expect.lines();
        
        for (i, line) in res.lines().enumerate() {
            assert_eq!(Some(line), expect_lines.next(), "Mismatch on line {}", i + 1)
        }
    }
}
