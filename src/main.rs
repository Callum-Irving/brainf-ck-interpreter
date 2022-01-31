use std::env;
use std::fs;
use std::path::Path;

use std::io::{stdin, stdout, ErrorKind, Read, Write};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

// Helper function for '[' and ']' commands
fn get_corresponding(
    program: &Vec<u8>,
    instruction_ptr: &mut usize,
    dir: i8,
    normal: char,
    reverse: char,
) {
    let mut depth = 1;
    while depth != 0 {
        *instruction_ptr = instruction_ptr.wrapping_add(dir as usize);
        if program[*instruction_ptr] as char == normal {
            depth += 1;
        } else if program[*instruction_ptr] as char == reverse {
            depth -= 1;
        }
    }
}

fn interpret(program: Vec<u8>) {
    let mut instruction_ptr: usize = 0;
    let mut data_ptr: usize = 0;
    let mut data: [u8; 30_000] = [0; 30_000];

    loop {
        match program[instruction_ptr] as char {
            '>' => data_ptr += 1,
            '<' => data_ptr -= 1,
            '+' => data[data_ptr] = data[data_ptr].wrapping_add(1),
            '-' => data[data_ptr] = data[data_ptr].wrapping_sub(1),
            '.' => print!("{}", data[data_ptr] as char),
            ',' => {
                // Don't write to memory if byte is EOF
                let byte = stdin().bytes().next().and_then(|r| r.ok());
                if !byte.is_none() {
                    data[data_ptr] = byte.unwrap();
                }
            }
            '[' => {
                if data[data_ptr] == 0 {
                    get_corresponding(&program, &mut instruction_ptr, 1, '[', ']');
                }
            }
            ']' => {
                if data[data_ptr] != 0 {
                    get_corresponding(&program, &mut instruction_ptr, -1, ']', '[');
                }
            }
            _ => (),
        }

        instruction_ptr += 1;

        if instruction_ptr >= program.len() {
            return;
        }
    }
}

fn interpret_interactive(program: Vec<u8>, quiet: bool) {
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut instruction_ptr: usize = 0;
    let mut data_ptr: usize = 0;
    let mut data: [u8; 30_000] = [0; 30_000];

    loop {
        match program[instruction_ptr] as char {
            '>' => data_ptr += 1,
            '<' => data_ptr -= 1,
            '+' => data[data_ptr] = data[data_ptr].wrapping_add(1),
            '-' => data[data_ptr] = data[data_ptr].wrapping_sub(1),
            '.' => {
                print!("{}", data[data_ptr] as char);
                // Workaround for raw mode terminal
                if data[data_ptr] as char == '\n' {
                    print!("\r");
                }
                stdout.flush().unwrap();
            }
            ',' => {
                // Get first key press that represents a rust char
                for c in stdin().keys() {
                    match c.unwrap() {
                        Key::Char(c) => {
                            data[data_ptr] = c as u8;
                            if !quiet {
                                print!("{}", c);
                                if c == '\n' {
                                    print!("\r");
                                }
                                stdout.flush().unwrap();
                            }
                            break;
                        }
                        Key::Ctrl('c') => {
                            // Drop stdout to put terminal back in normal mode
                            drop(stdout);
                            print!("^C");
                            std::process::exit(130);
                        }
                        _ => {}
                    }
                }
            }
            '[' => {
                if data[data_ptr] == 0 {
                    get_corresponding(&program, &mut instruction_ptr, 1, '[', ']');
                }
            }
            ']' => {
                if data[data_ptr] != 0 {
                    get_corresponding(&program, &mut instruction_ptr, -1, ']', '[');
                }
            }
            _ => (),
        }

        instruction_ptr += 1;

        if instruction_ptr >= program.len() {
            return;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut interactive = false;
    let mut filename = "".to_string();
    let mut quiet = false;

    let usage = || {
        println!("Usage: {} [-ih] <file>\n", &args[0]);
        println!("OPTIONS:");
        println!(" -i --interactive\trun in interactive mode");
        println!(" -q --quiet\t\tdon't show input in interactive mode");
        println!(" -h --help\t\tprint this help message");
    };

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "-i" | "--interactive" => interactive = true,
            "-h" | "--help" => {
                usage();
                std::process::exit(0);
            }
            "-q" | "--quiet" => quiet = true,
            _ => {
                if arg.chars().next() == Some('-') {
                    println!("ERROR: Unknown argument '{}'\n", arg);
                    usage();
                    std::process::exit(1);
                }
                if filename == "" {
                    filename = arg.clone();
                } else {
                    println!("ERROR: Too many files specified\n");
                    usage();
                    std::process::exit(1);
                }
            }
        }
    }

    if filename == "" {
        println!("ERROR: No input file specified\n");
        usage();
        std::process::exit(1);
    }

    if !Path::new(&filename).exists() {
        println!("ERROR: File specified does not exist\n");
        usage();
        std::process::exit(1);
    }

    let result = fs::read_to_string(filename);
    let program = match result {
        Ok(contents) => contents.into_bytes(),
        Err(error) => match error.kind() {
            ErrorKind::InvalidData => {
                println!("ERROR: File specified has bad data\n");
                usage();
                std::process::exit(1);
            }
            _ => {
                println!("ERROR: Unknown error when reading file\n");
                usage();
                std::process::exit(1);
            }
        },
    };

    if interactive {
        interpret_interactive(program, quiet);
    } else {
        interpret(program);
    }
}
