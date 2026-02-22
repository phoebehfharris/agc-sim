use crate::register_types::*;

use ouroboros::self_referencing;


#[self_referencing]
pub struct RegisterFile {
    // "A"
    accumulator: AccumulatorInteger,
    // NOTE: Everything below is actually 15-bits
    // "L"
    lower_product: MemoryInteger,
    // "Q"
    ret_address: AccumulatorInteger,
    // "EB"
    erasable_bank: ErasableBank,
    // "FB"
    fixed_bank: FixedBank,
    // "Z"
    pub program_counter: ProgramCounter,
    // "BB"
    #[borrows(mut erasable_bank, mut fixed_bank)]
    #[not_covariant]
    both_banks: BothBanks<'this>,
    // No name
    zero: Zero,
    // "ARUPT"
    accumulator_interrupt : MemoryInteger,
    // "LRUPT"
    lower_product_interrupt: MemoryInteger,
    // "QRUPT"
    ret_address_interrupt: AccumulatorInteger,
    // "SAMPTIME"
    sample_time_1: MemoryInteger,
    sample_time_2: MemoryInteger,
    // "ZRUPT"
    program_counter_interrupt: ProgramCounter,
    // TODO: Do the rest of the registers
}

impl RegisterFile {
}
