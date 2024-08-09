use crate::instruction::{Opcode, decode_instruction};

#[derive(Debug)]
pub struct Silly32 {
    pub registers: [i32; 8],
    pub memory: Vec<i32>,
    pub pc: usize,
}

impl Silly32 {
    pub fn new(memory_size: usize) -> Self {
        Self {
            registers: [0; 8],
            memory: vec![0; memory_size],
            pc: 0,
        }
    }

    pub fn load_program(&mut self, program: &[i32]) {
        self.memory[..program.len()].copy_from_slice(program);
    }

    pub fn run(&mut self) {
        while self.pc < self.memory.len() {
            let instruction = self.memory[self.pc];
            println!("Executing instruction at pc {}: {:#X}", self.pc, instruction);
            self.execute_instruction(instruction);
        }
    }

    fn execute_instruction(&mut self, instruction: i32) {
        let (opcode, r1, r2, imm) = decode_instruction(instruction);

        match opcode {
            Opcode::Load => {
                self.registers[r1] = imm;
                println!("LOAD: R{} = {}", r1, imm);
            }
            Opcode::Move => {
                self.registers[r1] = self.registers[r2];
                println!("MOVE: R{} = R{}", r1, r2);
            }
            Opcode::Add => {
                self.registers[r1] += self.registers[r2];
                println!("ADD: R{} = R{} + R{}", r1, r1, r2);
            }
            Opcode::Sub => {
                self.registers[r1] -= self.registers[r2];
                println!("SUB: R{} = R{} - R{}", r1, r1, r2);
            }
            Opcode::Jump => {
                self.pc = self.registers[r1] as usize;
                println!("JUMP: PC = R{}", r1);
                return;
            }
            Opcode::JumpIfZero => {
                if self.registers[r1] == 0 {
                    self.pc = self.registers[r2] as usize;
                    println!("JZ: PC = R{} because R{} == 0", r2, r1);
                    return;
                }
            }
            Opcode::LoadMem => {
                self.registers[r1] = self.memory[self.registers[r2] as usize];
                println!("LOADMEM: R{} = MEM[R{}]", r1, r2);
            }
            Opcode::StoreMem => {
                self.memory[self.registers[r2] as usize] = self.registers[r1];
                println!("STOREMEM: MEM[R{}] = R{}", r2, r1);
            }
            Opcode::Sex => {
                println!("SEX");
            }
            Opcode::Unknown => panic!("Unknown instruction: {:#X}", instruction),
        }

        self.pc += 1;
    }
}
