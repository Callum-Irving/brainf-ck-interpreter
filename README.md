# Brainf*ck Interpreter

This is just a normal Brainf\*ck interpreter written in Rust. If you don't know
what Brainf*ck is, you can check out the [wikipedia page](https://en.wikipedia.org/wiki/Brainfuck#Language_design). It's just a really
simple programming language. The syntax is really weird to look at, hence the
name.

Here's a simple hello world program:

```
>++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<+
+.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-
]<+.
>>>>>>>>>>>>>>>>>++++++++++.
```

If you run this program, the output is:

```
Hello, world!
```

## Requirements

- To run this you need Rust and Cargo installed on your system. The recommended
way to install them is through [rustup](https://rustup.rs/).

## Usage

```
Usage: cargo run -- [-ih] <file>

OPTIONS:
 -i --interactive       run in interactive mode
 -h --help              print this help message
```
