// 16-bit register type with an overflow bit
pub struct AccumulatorInteger {
    inner: u16,
}

impl MemoryLocation for AccumulatorInteger {
    fn to_u16(&self) -> u16 {
        return self.inner;
    }

    fn mov_u16(&mut self, other: u16) {
        // TODO: Change this to MSB
        let second_last_bit = (other >> 1) & 0b0000000000000001;

        if second_last_bit > 0 {
            self.inner = other | 0b0000000000000001;
        } else {
            self.inner = other & 0b1111111111111110;
        }
    }

    fn mov(&mut self, _other: &dyn MemoryLocation) {
        // TODO: Implement this

    }

    fn add(&mut self, other: &dyn MemoryLocation) {
        let mask = 0b0111_1111_1111_1111;
        let overflow = (self.to_u16() + other.to_u16()) & !mask != 0;

        let add = if overflow { 1 } else { 0 };

        self.mov_u16((self.to_u16() + other.to_u16() + add) & mask);

        // TODO: Add overflow handling
    }
}

pub struct MemoryInteger {
    inner: u16
}

// TODO Implement memory
//
pub struct ErasableBank {
    inner: u16,
}

// TODO Implement memory

pub struct FixedBank {
    inner: u16,
}

// TODO Implement memory

pub struct BothBanks<'a> {
    erasable: &'a mut ErasableBank,
    fixed: &'a mut FixedBank,
}

// TODO Implement memory (fun!)

pub struct ProgramCounter {
    inner: u16
}

impl MemoryLocation for ProgramCounter {
    fn to_u16(&self) -> u16 {
        return self.inner;
    }
    fn mov_u16(&mut self, other: u16) {
        // TODO Implement
        todo!()
    }

    fn mov(&mut self, _other: &dyn MemoryLocation) {
        // TODO Implement
        todo!()
    }
}

pub struct Zero{}

// TODO Implement memory

pub trait MemoryLocation {
    fn to_u16(&self) -> u16;
    fn mov_u16(&mut self, other: u16);
    // TODO Provide a default implementation of this
    fn mov(&mut self, _other: &dyn MemoryLocation);
    fn add(&mut self, other: &dyn MemoryLocation) {
        let mask = 0b0111_1111_1111_1111;
        let overflow = (self.to_u16() + other.to_u16()) & !mask != 0;

        let add = if overflow { 1 } else { 0 };

        self.mov_u16((self.to_u16() + other.to_u16() + add) & mask);
    }

}

// TODO Readd some unit tests

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
