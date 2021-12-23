use std::env;
use std::fs;

use std::io::{stdin, stdout, Stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

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

fn interpret(program: Vec<u8>, stdout: &mut RawTerminal<Stdout>) {
    // All state for the interpreter
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
                            break;
                        }
                        Key::Esc => return,
                        _ => {}
                    }
                }
            }
            // For the following 2 instructions, 'n' represents stack depth
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
    let filename = &args[1];
    let program = fs::read_to_string(filename).unwrap().into_bytes();
    let mut stdout = stdout().into_raw_mode().unwrap();
    interpret(program, &mut stdout);
}
