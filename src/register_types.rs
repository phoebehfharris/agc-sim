// 16-bit register type with an overflow bit
pub struct AccumulatorInteger {
    inner: u16,
}

pub struct MemoryInteger {
    inner: u16
}
pub struct ErasableBank {
    inner: u16,
}

pub struct FixedBank {
    inner: u16,
}

pub struct BothBanks<'a> {
    erasable: &'a mut ErasableBank,
    fixed: &'a mut FixedBank,
}

pub struct ProgramCounter {
    inner: u16
}

pub struct Zero{}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_acc_to_mem_same_bit() {
//         let acc = AccumulatorInteger::from(0b1111111100001110);
//         let mem = MemoryInteger::from(acc);
//
//         assert_eq!(mem.inner, 0b1111111100001110);
//     }
//
//     #[test]
//     fn test_acc_to_mem_discard_bit() {
//         let acc = AccumulatorInteger::from(0b1111111100001111);
//         let mem = MemoryInteger::from(acc);
//
//         assert_eq!(mem.inner, 0b1111111100001110);
//     }
//
//     #[test]
//     fn test_mem_to_acc_same_bit() {
//         let mem = MemoryInteger::from(0b1111111111111111);
//         let acc = AccumulatorInteger::from(mem);
//
//         assert_eq!(acc.inner, 0b1111111111111111)
//     }
//
//     #[test]
//     fn test_mem_to_acc_discard_bit() {
//         let mem = MemoryInteger::from(0b1111111111111110);
//         let acc = AccumulatorInteger::from(mem);
//
//         assert_eq!(acc.inner, 0b1111111111111111)
//     }
//
// }
