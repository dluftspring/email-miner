# Email Miner
A rust cli program that parses email messages and writes them into the following naming convention

`<SEND-DATE(YYYY-MM-DD)>_<FROM>_<SUBJECT>`. This is useful for anyone who has had their work emails dumped to them with
the metadata compromised. It's also just me learning a little about rust

## Quick start

First install [rust](https://www.rust-lang.org/tools/install) for your OS.

Then you can build the dependencies and run the script with

```bash
~$ cargo run
```

## Compilation

To compile the binary we use

```bash
~$ rustc src/main.rs
```

This will give us a portable executable that can be run from the command line