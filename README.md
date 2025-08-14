# minigrep

A simple implementation of the `grep` command-line tool, written in Rust.

## Usage

```
minigrep <query> <file_path>
```

- `<query>`: The string to search for.
- `<file_path>`: The path to the file to search in.

## Examples

### Basic Search

To search for the word "nobody" in a file named `poem.txt`:

```
$ cat poem.txt
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!

$ minigrep nobody poem.txt
Found I'm nobody! Who are you?
Found Are you nobody, too?
```

### Case-Insensitive Search

To perform a case-insensitive search, set the `CASE_MODE` environment variable to `insensitive`.

```
$ export CASE_MODE=insensitive
$ minigrep rUsT poem.txt
Found Rust:
```

## Building

To build the project from source, you will need to have Rust and Cargo installed. You can find instructions on how to install them [here](https://www.rust-lang.org/tools/install).

Once you have Rust and Cargo installed, you can build the project by running the following command in the project's root directory:

```
cargo build --release
```

The compiled binary will be located at `target/release/minigrep`.
