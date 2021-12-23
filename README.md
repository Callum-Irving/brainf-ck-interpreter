# Brainf*ck Interpreter

Thisis just a normal Brainf\*ck interpreter written in Rust. If you don't know
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

## Running

### Requirements

- To run this you need Rust and Cargo installed on your system. The recommended
way to install them is through [rustup](https://rustup.rs/).

### Usage

To run this project, clone this repository and then use

```
cargo run -- <filename>
```

where `<filename>` is the source file.
