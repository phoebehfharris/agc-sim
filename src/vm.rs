use std::fmt::Display;
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
                    let mem = self.read_memory(rhs_addr);
                    self.registers.with_accumulator_mut(|acc| {
                        acc.add(mem);
                    })
                }
                Instruction::ADS(rhs) => {
                    let mem = self.read_memory(rhs);
                    let mut new = 0;
                    self.registers.with_accumulator_mut(|acc| {
                        acc.add(mem);
                        new = acc.to_u16();
                    });
                    self.write_memory(rhs, new);
                }
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

impl Display for VM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "VM ===================")?;
        writeln!(
            f,
            "Acc {:#018b}",
            self.registers.borrow_accumulator().to_u16()
        )?;
        writeln!(
            f,
            "LoP {:#018b}",
            self.registers.borrow_lower_product().to_u16()
        )?;
        writeln!(
            f,
            "Ret {:#018b}",
            self.registers.borrow_ret_address().to_u16()
        )?;
        writeln!(
            f,
            "Pc  {:#018b}",
            self.registers.borrow_program_counter().to_u16()
        )?;
        writeln!(
            f,
            "AcI {:#018b}",
            self.registers.borrow_accumulator_interrupt().to_u16()
        )?;
        writeln!(
            f,
            "LpI {:#018b}",
            self.registers.borrow_lower_product_interrupt().to_u16()
        )?;
        writeln!(
            f,
            "ReI {:#018b}",
            self.registers.borrow_ret_address_interrupt().to_u16()
        )?;
        writeln!(
            f,
            "ST1 {:#018b}",
            self.registers.borrow_sample_time_1().to_u16()
        )?;
        writeln!(
            f,
            "ST2 {:#018b}",
            self.registers.borrow_sample_time_2().to_u16()
        )?;
        writeln!(
            f,
            "PcI {:#018b}",
            self.registers.borrow_program_counter_interrupt().to_u16()
        )?;
        Ok(())
    }
}
