use std::collections::HashMap;
use std::io::{self, Read};
use std::num::Wrapping;


#[derive(Debug)]
pub struct ExecutionContext {
    text: Vec<u8>,
    data: Vec<i8>,
    data_size: usize,
    inst_ptr: usize,
    data_ptr: usize,
    bracket_map: HashMap<usize, usize>,
}

impl ExecutionContext {
    pub fn new(text: Vec<u8>) -> ExecutionContext {
        let bracket_map = ExecutionContext::get_bracket_map(&text);
        ExecutionContext {
            text,
            data: vec![0; 30000],  // As per language standard, program data is always at least 30K cells
            data_size: 30000,
            inst_ptr: 0,
            data_ptr: 0,
            bracket_map,
        }
    }

    fn get_bracket_map(text: &Vec<u8>) -> HashMap<usize, usize> {
        let mut bracket_map = HashMap::new();
        let mut stack = Vec::new();
        for (idx, c) in (0..text.len()).zip(text.iter()) {
            match *c as char {
                '[' => stack.push(idx),
                ']' => {
                    if let Some(start_idx) = stack.pop() {
                        bracket_map.insert(start_idx, idx);
                        bracket_map.insert(idx, start_idx);
                    } else {
                        panic!("Unbalanced bracket! Left bracket has no right bracket.");
                    }
                },
                _ => ()
            }
        }
        // TODO: Maybe add more useful debugging output
        assert!(stack.len() == 0, "Unbalanced bracket(s)! One or more right brackets has no matching left bracket.");
        bracket_map
    }

    pub fn execute_all(&mut self) {
        while self.inst_ptr < self.text.len() {
            self.execute_instruction();
        }
    }

    fn execute_instruction(&mut self) {
        match self.text[self.inst_ptr] as char { 
            '>' => {
                self.data_ptr += 1;
                self.inst_ptr += 1;
                if self.data_ptr == self.data_size {
                    self.data.push(0);
                    self.data_size += 1;
                }
            },

            '<' => {
                self.data_ptr = if self.data_ptr == 0 {
                    0
                } else {
                    self.data_ptr - 1
                };
                self.inst_ptr += 1;
            },

            '+' => {
                self.data[self.data_ptr] = (Wrapping(self.data[self.data_ptr]) + Wrapping(1)).0;
                self.inst_ptr += 1;
            },

            '-' => {
                self.data[self.data_ptr] = (Wrapping(self.data[self.data_ptr]) - Wrapping(1)).0;
                self.inst_ptr += 1;
            },

            '.' => {
                print!("{}", self.data[self.data_ptr] as u8 as char);
                self.inst_ptr += 1;
            },

            ',' => {
                let mut input: [u8; 1] = [0; 1];
                io::stdin().read(&mut input).unwrap();
                self.data[self.data_ptr] = input[0] as i8;
            },

            '[' => {
                match self.data[self.data_ptr] {
                    0 => self.inst_ptr = *self.bracket_map.get(&self.inst_ptr).unwrap() + 1,

                    _ => self.inst_ptr += 1
                }
            },

            ']' => self.inst_ptr = *self.bracket_map.get(&self.inst_ptr).unwrap(),

            _ => self.inst_ptr += 1
        }
    }
}