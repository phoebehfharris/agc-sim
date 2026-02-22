use crate::register_types::*;

use ouroboros::self_referencing;

#[self_referencing]
pub struct RegisterFile {
    // "A"
    pub accumulator: AccumulatorInteger,
    // NOTE: Everything below is actually 15-bits
    // "L"
    lower_product: MemoryInteger,
    // "Q"
    ret_address: AccumulatorInteger,
    // "EB"
    pub erasable_bank: ErasableBank,
    // "FB"
    fixed_bank: FixedBank,
    // "Z"
    pub program_counter: ProgramCounter,
    // "BB"
    #[borrows(mut erasable_bank, mut fixed_bank)]
    #[not_covariant]
    pub both_banks: BothBanks<'this>,
    // No name
    zero: Zero,
    // "ARUPT"
    accumulator_interrupt: MemoryInteger,
    // "LRUPT"
    lower_product_interrupt: MemoryInteger,
    // "QRUPT"
    ret_address_interrupt: AccumulatorInteger,
    // "SAMPTIME"
    sample_time_1: MemoryInteger,
    sample_time_2: MemoryInteger,
    // "ZRUPT"
    program_counter_interrupt: ProgramCounter,
    // TODO Do the rest of the registers
}

impl Default for RegisterFile {
    fn default() -> Self {
        let b = RegisterFileBuilder {
            accumulator: AccumulatorInteger::default(),
            lower_product: MemoryInteger::default(),
            ret_address: AccumulatorInteger::default(),
            erasable_bank: ErasableBank::default(),
            fixed_bank: FixedBank::default(),
            program_counter: ProgramCounter::default(),
            both_banks_builder: |a: &mut ErasableBank, b: &mut FixedBank| BothBanks {
                erasable: a,
                fixed: b,
            },
            zero: Zero::default(),
            accumulator_interrupt: MemoryInteger::default(),
            lower_product_interrupt: MemoryInteger::default(),
            ret_address_interrupt: AccumulatorInteger::default(),
            sample_time_1: MemoryInteger::default(),
            sample_time_2: MemoryInteger::default(),
            program_counter_interrupt: ProgramCounter::default(),
        };
        b.build()
    }
}

impl RegisterFile {
    pub fn eb_number(&self) -> usize {
        let eb_mask = 0b0_000_011_100_000_000;
        let eb = self.with_both_banks(|bb| bb.erasable.to_u15()) & eb_mask;
        eb as usize >> 8
    }
    pub fn fb_number(&self) -> usize {
        let fb_mask = 0b0_111_110_100_000_000;
        let eb = self.with_both_banks(|bb| bb.fixed.to_u15()) & fb_mask;

        let eb = eb as usize >> 10;
        let io_7 = 0;

        if io_7 == 0 {
            return eb;
        }

        if eb <= 27 {
            return eb;
        }

        eb + 12
    }
}

// TODO Create a proper new function
