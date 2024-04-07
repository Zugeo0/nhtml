mod position;
mod scanner;
mod parser;
mod source;
mod token;
mod emitter;

use std::{ffi::OsStr, path::{Path, PathBuf}};

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

fn check_extension(path: &Path, ext: &str) -> bool {
    return path.extension().and_then(OsStr::to_str) == Some(ext);
}

fn transpile_file(path: &Path, output: &Path) -> Result<()> {
    let input = std::fs::read_to_string(path)
        .context("Failed to read input file")?;
    let res = transpile(&input)?;
    let parent = output.parent()
        .context(format!("Failed to get parent directory of file {output:?}"))?;
    std::fs::create_dir_all(parent)?;
    std::fs::write(output, res).context("Failed to write transpiled code to file")?;
    Ok(())
}

fn transpile_dir(input: &Path, dir: &Path, output: &Path) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let base_path: PathBuf = if input.is_absolute() { input.to_path_buf() } else { cwd.join(input) };

    for dir_entry in dir.read_dir()? {
        let path_buf = dir_entry?.path();
        let path = Path::new(&path_buf);

        if !path.is_dir() && !check_extension(path, "nhtml") {
            continue;
        }

        if path.is_dir() {
            transpile_dir(input, path, output)?;
        } else {
            let relative_out = if path.is_absolute() { path.strip_prefix(&base_path)? } else { path.strip_prefix(input)? };
            let out = output.join(relative_out)
                .with_extension("html");

            transpile_file(
                path,
                &out
            )?;
        }
    }

    Ok(())
}

fn transpile_from_to(path: &Path, output: &Path) -> Result<()> {
    if path.is_dir() {
        return transpile_dir(path, path, output);
    }

    if output.is_dir() {
        let file_name = path.file_name()
            .context("Failed to get file name")?;
        let out_file = output.join(file_name)
            .with_extension("html");
        return transpile_file(path, &out_file);
    }

    transpile_file(path, output)
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Convert { path, output } => {
            if path.is_dir() && output.is_file() {
                eprintln!("Cannot output directory to a file");
                return Ok(());
            }
            transpile_from_to(&path, &output)?;
        },
        Commands::Watch { path, output } => {
            if path.is_dir() && output.is_file() {
                eprintln!("Cannot output directory to a file");
                return Ok(());
            }
            transpile_from_to(&path, &output)?;
            println!("Watching '{}'. Press CTRL-C to quit", path.to_str().unwrap());
            watch(&path, &output)?;
        },
    }

    Ok(())
}

fn watch(input: &Path, output: &Path) -> Result<()> {
    use notify::{Watcher, RecursiveMode, RecommendedWatcher, Config};

    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())
        .context("Failed to create watcher")?;

    watcher.watch(&input, RecursiveMode::Recursive)
        .context("Failed to start watcher on path")?;

    for res in rx {
        match res {
            Ok(event) => watch_event(&event, input, output)?,
            Err(err) => println!("err: {err:?}"),
        }
    }

    Ok(())
}

fn watch_event(event: &notify::Event, input: &Path, output: &Path) -> Result<()> {
    if event.paths[0].is_dir() {
        return Ok(());
    }

    if !matches!(event.kind, notify::EventKind::Modify(_)) {
        return Ok(());
    }

    if !check_extension(&event.paths[0], "nhtml") {
        return Ok(());
    }

    let cwd = std::env::current_dir()?;
    let base_path: PathBuf = if input.is_absolute() { input.to_path_buf() } else { cwd.join(input) };

    if output.is_file() {
        transpile_file(
            &event.paths[0],
            &output
        )?;
    } else {
        let relative_out = event.paths[0].strip_prefix(base_path)?;
        let out = output.join(relative_out)
            .with_extension("html");

        transpile_file(
            &event.paths[0],
            &out
        )?;
    }
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
