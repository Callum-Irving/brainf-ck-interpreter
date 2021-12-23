use std::env;
use std::fs;

use std::io::{stdin, stdout, Stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

fn interpret(program: Vec<u8>, stdout: &mut RawTerminal<Stdout>) {
    // All state for the interpreter
    let mut instruction_pointer: usize = 0;
    let mut data_pointer: usize = 0;
    let mut data: [u8; 30_000] = [0; 30_000];

    loop {
        match program[instruction_pointer] as char {
            '>' => data_pointer += 1,
            '<' => data_pointer -= 1,
            '+' => data[data_pointer] = data[data_pointer].wrapping_add(1),
            '-' => data[data_pointer] = data[data_pointer].wrapping_sub(1),
            '.' => {
                print!("{}", data[data_pointer] as char);
                // Workaround for raw mode terminal
                if data[data_pointer] as char == '\n' {
                    print!("\r");
                }
                stdout.flush().unwrap();
            }
            ',' => {
                for c in stdin().keys() {
                    match c.unwrap() {
                        Key::Char(c) => {
                            data[data_pointer] = c as u8;
                            break;
                        }
                        Key::Esc => return,
                        _ => {}
                    }
                }
            }
            // For the following 2 instructions, 'n' represents stack depth
            '[' => {
                if data[data_pointer] == 0 {
                    let mut n = 1;
                    while n != 0 {
                        instruction_pointer += 1;
                        if program[instruction_pointer] as char == '[' {
                            n += 1;
                        } else if program[instruction_pointer] as char == ']' {
                            n -= 1;
                        }
                    }
                }
            }
            ']' => {
                if data[data_pointer] != 0 {
                    let mut n = 1;
                    while n != 0 {
                        instruction_pointer -= 1;
                        if program[instruction_pointer] as char == ']' {
                            n += 1;
                        } else if program[instruction_pointer] as char == '[' {
                            n -= 1;
                        }
                    }
                }
            }
            _ => (),
        }

        instruction_pointer += 1;

        if instruction_pointer >= program.len() {
            return;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let program = fs::read_to_string(filename).unwrap().into_bytes();
    let mut stdout = stdout().into_raw_mode().unwrap();
    interpret(program, &mut stdout);
}
