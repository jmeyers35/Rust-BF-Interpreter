pub enum IRInstr {
    Add(i8),
    Sub(i8),
    RShift(i8),
    LShift(i8),
    StartBracket,
    EndBracket,
    SetValue(i8),
    ByteIn,
    ByteOut,
}

pub fn convert(text: &Vec<u8>) -> Vec<IRInstr> {
    // Optimize time!
    let mut cursor = 0;
    let mut ir = Vec::new();
    while cursor < text.len() {
        match text[cursor] as char {
            // easy cases first, no optimization
            '.' => {
                ir.push(IRInstr::ByteOut);
                cursor += 1;
            },
            ',' => {
                ir.push(IRInstr::ByteIn);
                cursor += 1;
            },
            '+' => {
                let increment_amount = count_consecutive_occurances(&text[cursor..]);
                ir.push(IRInstr::Add(increment_amount));
                cursor += (increment_amount + 1) as usize;
            },
            '-' => {
                let decrement_amount = count_consecutive_occurances(&text[cursor..]);
                ir.push(IRInstr::Sub(decrement_amount));
                cursor += (decrement_amount + 1) as usize;
            }
            '>' => {
                let shift_amount = count_consecutive_occurances(&text[cursor..]);
                ir.push(IRInstr::RShift(shift_amount));
                cursor += (shift_amount + 1) as usize;
            },
            '<' => {
                let shift_amount = count_consecutive_occurances(&text[cursor..]);
                ir.push(IRInstr::LShift(shift_amount));
                cursor += (shift_amount + 1) as usize;
            },
            '[' => {
                // TODO optimize loops
                cursor += 1;
                ir.push(IRInstr::StartBracket);
            },
            ']' => {
                cursor += 1;
                ir.push(IRInstr::EndBracket);
            },
            _ => {
                cursor += 1;
            }
        }
    }

    ir
}

fn count_consecutive_occurances(text: &[u8]) -> i8 {
    let c = text[0] as char;
    let mut count = 1;
    let mut cursor = 1;
    while cursor < text.len() && text[cursor] as char == c {
        count += 1;
        cursor += 1;
    }

    count
}