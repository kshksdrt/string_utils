# String utilities

String utils(written in Rust) is a successor to my previous [text-utils, which was written in go](https://github.com/kshksdrt/text-utils/).

Disclaimer: These are simply few things I wanted. Do not be surprised if you do not find them not useful.

## Features:

- String to Hex Color Generator

It takes a string as input and generates a unique and reproducible hexadecimal color code (e.g., `#RRGGBB`). It's useful for assigning consistent colors to tags, usernames, categories, etc., based on their names. However, I presonally created this for file names.

## Prerequisites

- **Rust and Cargo:** You need the Rust toolchain, which includes `rustc` (the compiler) and `cargo` (the build tool and package manager). See [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

## Compilation

To create a **debug build** for a faster compilation, less optimized executable,

```bash
cargo build
```
