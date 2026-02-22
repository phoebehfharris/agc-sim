use crate::vm::VM;

mod instructions;
mod memory;
mod register_types;
mod registerfile;
mod vm;

fn main() {
    let mut vm = VM::default();

    let start_location = 0o2000;
    let instruction = 0o60000;
    vm.write_memory(start_location, instruction);
    vm.set_start_location(start_location as u16);
    vm.execute_instruction();

    // TODO Actually run the code

    // TODO Add a file reader
}
