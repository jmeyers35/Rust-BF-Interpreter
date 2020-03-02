use std::io::{self, Read};

pub struct ExecutionContext {
    text: Vec<u8>,
    data: Vec<u8>,
    data_size: usize,
    inst_ptr: usize,
    data_ptr: usize,
    open_bracket_stack: Vec<usize>,  // Stack of right bracket indices 
}

impl ExecutionContext {
    pub fn new(text: Vec<u8>) -> ExecutionContext {
        ExecutionContext {
            text: text,
            data: vec![0; 30000],  // As per language specifications, program data is always at least 30KB
            data_size: 30000,
            inst_ptr: 0,
            data_ptr: 0,
            open_bracket_stack: Vec::new(),
        }
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
                self.data[self.data_ptr] += 1;
                self.inst_ptr += 1;
            },

            '-' => {
                self.data[self.data_ptr] -= 1;
                self.inst_ptr += 1;
            },

            '.' => {
                print!("{}", self.data[self.data_ptr]);
            },

            ',' => {
                let mut input: [u8; 1] = [0; 1];
                io::stdin().read(&mut input).unwrap();
                self.data[self.data_ptr] = input[0];
            },

            '[' => {
                match self.data[self.data_ptr] {
                    0 => {
                        // TODO: this is inefficient, could do pre-processing of text and map opening to closing bracket indices
                        // However, for sufficiently small programs/[ ] blocks, this might be faster than the overhead of a map 

                        // TODO: Probably add useful debug messages for malformed brackets. Right now, if there is an unmatched [, the program will 
                        // essentially halt by incrementing the instruction pointer all the way to the end of the program.
                        while self.inst_ptr < self.text.len() && self.text[self.inst_ptr] as char != ']' {
                            self.inst_ptr += 1;
                        }  
                    },

                    _ => {
                        self.open_bracket_stack.push(self.inst_ptr);
                        self.inst_ptr += 1;
                    }
                }
            },

            ']' => {
                if let Some(open_bracket_idx) = self.open_bracket_stack.pop() {
                    self.inst_ptr = open_bracket_idx;
                } else {
                    // TODO: Malformed brackets error handling
                }
            },

            _ => {
                self.inst_ptr += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {

}