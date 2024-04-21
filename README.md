# Nicer HTML

To get started. Clone this repository and install with cargo

Requires Rust and Cargo to be installed

```sh
git clone https://github.com/Zugeo0/nhtml
cd nhtml
cargo install --path .
```

Then you are able to use the nhtml command to convert nhtml files to html

```sh
nhtml convert input.nhtml output.html
# or
nhtml convert src/ out/
```

You are also able to watch files and directories for changes

```sh
nhtml watch input.nhtml output.nhtml
# or
nhtml watch src/ out/
```

Example nhtml file

```
<!DOCTYPE html>

html lang='en' {
  head {
    // Meta Info
    meta charset='UTF-8';
    meta name='viewport' content='width=device-width, initial-scale=1.0';
    meta http-equiv='X-UA-Compatible' content='ie=edge';

    // Site info
    title 'HTML 5 Boilerplate';
  }

  body {
    /* Add Body Here */
  }
}

```
