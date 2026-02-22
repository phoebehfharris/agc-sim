use crate::vm::VM;

mod instructions;
mod memory;
mod register_types;
mod registerfile;
mod vm;

fn main() {
    let mut vm = VM::default();

    let start_location = 0o2000;
    let instruction_ad = 0o60000;
    let instruction_ads = 0o26000;
    vm.write_memory(start_location, instruction_ad);
    vm.write_memory(start_location + 1, instruction_ads);
    vm.write_memory(0, 0o00001);
    vm.set_start_location(start_location as u16);
    vm.execute_instruction();
    println!("{}", vm);
    // vm.read_memory(start_location;

    // TODO Actually run the code

    // TODO Add a file reader
}
