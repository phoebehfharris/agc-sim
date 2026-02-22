use crate::vm::VM;

mod instructions;
mod memory;
mod register_types;
mod registerfile;
mod vm;

fn main() {
    let mut vm = VM::default();

    // let eerom = 0o2000;
    let start_location = 0o4000;
    let instruction_ad = 0o60000;
    // let instruction_ads = 0o26000;
    let instruction_ext = 0o00006;
    let instruction_aug = 0o24000;
    let instruction_bzf = 0o10000;
    // vm.write_memory(start_location, instruction_ad + eerom);
    vm.write_memory(start_location + 0, instruction_ad);
    vm.write_memory(start_location + 1, instruction_ext);
    vm.write_memory(start_location + 2, instruction_aug);
    vm.write_memory(start_location + 3, instruction_ext);
    vm.write_memory(
        start_location + 4,
        instruction_bzf + start_location as u16 + 0,
    );
    vm.write_memory(0, 0o00000);
    // vm.write_memory(eerom as usize, 0o00000);
    vm.set_start_location(start_location as u16);
    vm.execute_instruction();
    println!("{}", vm);
    // vm.read_memory(start_location;

    // TODO Actually run the code

    // TODO Add a file reader
}
