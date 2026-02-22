use crate::memory::Memory;
use crate::registerfile::RegisterFile;
use crate::register_types::MemoryLocation;

pub struct VM {
    registers: RegisterFile,
    memory: Memory,
}

impl VM {
    pub fn new() -> Self {
        todo!()
        // return Self {
        //     registers: RegisterFile::new(),
        //     memory: Memory::new()
        // }
    }

    pub fn execute_instruction(&mut self) {
        loop {
            // self.memory.read_location(location, eb_select, bank_select)
            let old_pc = self.registers.borrow_program_counter().to_u16();
            // Hahaha this could have been pc += 1
            self.registers.with_program_counter_mut(|pc| {
                pc.mov_u16(old_pc + 1);
            });

            let instruction_location = self.access_memory(old_pc as usize);
        }
    }

    fn access_memory(&mut self, _address: usize) -> &mut dyn MemoryLocation {
        todo!()
    }
}
