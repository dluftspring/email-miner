# Email Miner
A rust cli program that parses email messages and writes them into files with the following naming convention

`<SEND-DATE(YYYY-MM-DD)>_<FROM>_<SUBJECT>`. This is useful for anyone who has had their work emails dumped to them with
the metadata compromised. It's also just me learning a little about rust

DISCLAIMER: this is my attempt at learning the basic ergonomics of rust in public and I do not know what i'm doing. For reference on how/where I learned to code please consult this [video](https://www.youtube.com/watch?v=YnL9vAFphmE)

## Quick start

First install [rust](https://www.rust-lang.org/tools/install) for your OS.

Then you can build the dependencies and run the script with

```bash
~$ cargo run -- "<glob_pattern>" "<directory>"
```

For example we might use `*.eml` as the globbing pattern and `emails_2020` as our local directory

## Compilation

To compile the binary we use

```bash
~$ rustc src/main.rs
```

This will give us a portable executable that can be run from the command line