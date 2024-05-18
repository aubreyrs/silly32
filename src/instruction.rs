#[derive(Debug)]
pub enum Opcode {
    Load,
    Move,
    Add,
    Sub,
    Jump,
    JumpIfZero,
    LoadMem,
    StoreMem,
    Unknown,
}

impl From<i32> for Opcode {
    fn from(value: i32) -> Self {
        match value {
            0x01 => Opcode::Load,
            0x02 => Opcode::Move,
            0x03 => Opcode::Add,
            0x04 => Opcode::Sub,
            0x05 => Opcode::Jump,
            0x06 => Opcode::JumpIfZero,
            0x07 => Opcode::LoadMem,
            0x08 => Opcode::StoreMem,
            _ => Opcode::Unknown,
        }
    }
}

pub fn decode_instruction(instruction: i32) -> (Opcode, usize, usize, i32) {
    let opcode = (instruction >> 24) & 0xFF;
    let r1 = ((instruction >> 16) & 0xFF) as usize;
    let r2 = ((instruction >> 8) & 0xFF) as usize;
    let imm = (instruction & 0xFF) as i32;

    (Opcode::from(opcode), r1, r2, imm)
}
