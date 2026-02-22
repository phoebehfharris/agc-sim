use std::io::Read;

use crate::instructions::{Instruction, parse_bytes};
use crate::memory::Memory;
use crate::register_types::MemoryLocation;
use crate::registerfile::RegisterFile;

#[derive(Default)]
pub struct VM {
    registers: RegisterFile,
    memory: Memory,
}

impl VM {
    pub fn execute_instruction(&mut self) {
        loop {
            // self.memory.read_location(location, eb_select, bank_select)
            let old_pc = self.registers.borrow_program_counter().to_u16();
            // Hahaha this could have been pc += 1
            self.registers.with_program_counter_mut(|pc| {
                pc.mov_u16(old_pc + 1);
            });

            let i1 = self.read_memory(old_pc as usize);
            let i2 = self.read_memory((old_pc + 1) as usize);

            let (opcode, _taken) = parse_bytes(i1, i2);

            match opcode {
                Instruction::AD(rhs_addr) => {
                    println!("Here");
                    let mem = self.read_memory(rhs_addr);
                    self.registers.with_accumulator_mut(|acc| {
                        acc.add(mem);
                    })
                }
                Instruction::ADS(_rhs) => self.registers.with_accumulator_mut(|_acc| {}),
                // TODO Add the rest of the instructions
                _ => break,
            }
        }
    }

    pub fn set_start_location(&mut self, start: u16) {
        self.registers.with_program_counter_mut(|pc| {
            pc.mov_u16(start);
        })
    }
    pub fn read_memory(&mut self, address: usize) -> u16 {
        let eb = self.registers.eb_number();
        let fb = self.registers.fb_number();

        self.memory.read_location(address, eb, fb).unwrap_or(0)
    }
    pub fn write_memory(&mut self, address: usize, data: u16) {
        let eb = self.registers.eb_number();
        let fb = self.registers.fb_number();

        self.memory.write_location(address, eb, fb, data)
    }
}
