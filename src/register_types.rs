// 16-bit register type with an overflow bit
#[derive(Default)]
pub struct AccumulatorInteger {
    inner: u16,
}

impl MemoryLocation for AccumulatorInteger {
    fn to_u16(&self) -> u16 {
        self.inner
    }
    fn to_u15(&self) -> u16 {
        return self.inner & 0b0111111111111111;
    }

    fn mov_u16(&mut self, other: u16) {
        // TODO: Change this to MSB
        let second_bit = other & 0b0100000000000000;

        if second_bit > 0 {
            self.inner = other | 0b1000000000000000;
        } else {
            self.inner = other & 0b0111111111111111;
        }
    }

    fn add(&mut self, other: u16) {
        let value = sp15_add(self.to_u15(), other);
        self.mov_u16(value);

        // TODO: Add overflow handling
    }
}

#[derive(Default)]
pub struct MemoryInteger {
    inner: u16,
}

impl MemoryLocation for MemoryInteger {
    fn to_u16(&self) -> u16 {
        self.inner
    }
    fn to_u15(&self) -> u16 {
        return self.inner & 0b0111111111111111;
    }

    fn mov_u16(&mut self, other: u16) {
        // TODO: Change this to MSB
        let second_bit = other & 0b0100000000000000;

        if second_bit > 0 {
            self.inner = other | 0b1000000000000000;
        } else {
            self.inner = other & 0b0111111111111111;
        }
    }

    fn add(&mut self, other: u16) {
        let value = sp15_add(self.to_u15(), other);
        self.mov_u16(value);

        // TODO: Add overflow handling
    }
}

// TODO Implement memory
//
#[derive(Default)]
pub struct ErasableBank {
    inner: u16,
}

impl MemoryLocation for ErasableBank {
    fn to_u16(&self) -> u16 {
        self.inner
    }
    fn to_u15(&self) -> u16 {
        return self.inner & 0b0111111111111111;
    }

    fn mov_u16(&mut self, other: u16) {
        // TODO: Change this to MSB
        let second_bit = other & 0b0100000000000000;

        if second_bit > 0 {
            self.inner = other | 0b1000000000000000;
        } else {
            self.inner = other & 0b0111111111111111;
        }
    }

    fn add(&mut self, other: u16) {
        let value = sp15_add(self.to_u15(), other);
        self.mov_u16(value);

        // TODO: Add overflow handling
    }
}

#[derive(Default)]
pub struct FixedBank {
    inner: u16,
}

impl MemoryLocation for FixedBank {
    fn to_u16(&self) -> u16 {
        self.inner
    }
    fn to_u15(&self) -> u16 {
        return self.inner & 0b0111111111111111;
    }

    fn mov_u16(&mut self, other: u16) {
        // TODO: Change this to MSB
        let second_bit = other & 0b0100000000000000;

        if second_bit > 0 {
            self.inner = other | 0b1000000000000000;
        } else {
            self.inner = other & 0b0111111111111111;
        }
    }

    fn add(&mut self, other: u16) {
        let value = sp15_add(self.to_u15(), other);
        self.mov_u16(value);

        // TODO: Add overflow handling
    }
}

// #[derive(Default)]
pub struct BothBanks<'a> {
    pub erasable: &'a mut ErasableBank,
    pub fixed: &'a mut FixedBank,
}

// impl BothBanks

// TODO Implement memory (fun!)

#[derive(Default)]
pub struct ProgramCounter {
    inner: u16,
}

impl MemoryLocation for ProgramCounter {
    fn to_u16(&self) -> u16 {
        return self.inner;
    }
    fn to_u15(&self) -> u16 {
        return self.inner & 0b0111111111111111;
    }
    fn mov_u16(&mut self, other: u16) {
        // TODO: Change this to MSB
        let second_bit = other & 0b0100000000000000;

        if second_bit > 0 {
            self.inner = other | 0b1000000000000000;
        } else {
            self.inner = other & 0b0111111111111111;
        }
    }

    fn add(&mut self, other: u16) {
        let value = sp15_add(self.to_u15(), other);
        self.mov_u16(value);

        // TODO: Add overflow handling
    }
}

#[derive(Default)]
pub struct Zero {}

// impl MemoryLocation for Zero {
//     fn to_u16(&self) -> u16 {
//         return 0;
//     }

//     fn mov_u16(&mut self, other: u16) {
//         panic!("Attempted to set the zero register!")
//     }
// }

// TODO Implement memory

pub trait MemoryLocation {
    fn to_u16(&self) -> u16;
    fn to_u15(&self) -> u16;
    fn mov_u16(&mut self, other: u16);

    fn add(&mut self, other: u16) {
        self.mov_u16(sp15_add(self.to_u15(), other));
    }
}

pub fn sp15_add(a: u16, b: u16) -> u16 {
    let mask = 0b0111_1111_1111_1111;
    let overflow = (a + b) & !mask != 0;

    let add = if overflow { 1 } else { 0 };

    return (a + b + add) & mask;
}

// TODO Readd some unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acc_convert() {
        let acc = AccumulatorInteger {
            inner: 0b0111_1111_0000_1110,
        };

        assert_eq!(acc.to_u15(), 0b0111111100001110);
    }

    #[test]
    fn test_acc_u15_mask() {
        let acc = AccumulatorInteger {
            inner: 0b1111_1111_0000_1110,
        };

        assert_eq!(acc.to_u15(), 0b0111111100001110);
    }

    #[test]
    fn test_acc_add_neg() {
        let mut acc = AccumulatorInteger {
            inner: 0b000000000000000,
        };
        acc.add(0b111111111111111);

        assert_eq!(acc.to_u15(), 0b111111111111111);
    }

    #[test]
    fn test_acc_add_pos() {
        let mut acc = AccumulatorInteger {
            inner: 0b110111111111111,
        };
        acc.add(0b010000000000000);

        assert_eq!(acc.to_u15(), 0b001000000000000);
    }

    // #[test]
    // fn test_acc_to_mem_same_bit() {
    //     let acc = AccumulatorInteger::from(0b1111111100001110);
    //     let mem = MemoryInteger::from(acc);

    //     assert_eq!(mem.inner, 0b1111111100001110);
    // }

    // #[test]
    // fn test_acc_to_mem_discard_bit() {
    //     let acc = AccumulatorInteger::from(0b1111111100001111);
    //     let mem = MemoryInteger::from(acc);

    //     assert_eq!(mem.inner, 0b1111111100001110);
    // }

    // #[test]
    // fn test_mem_to_acc_same_bit() {
    //     let mem = MemoryInteger::from(0b1111111111111111);
    //     let acc = AccumulatorInteger::from(mem);

    //     assert_eq!(acc.inner, 0b1111111111111111)
    // }

    // #[test]
    // fn test_mem_to_acc_discard_bit() {
    //     let mem = MemoryInteger::from(0b1111111111111110);
    //     let acc = AccumulatorInteger::from(mem);

    //     assert_eq!(acc.inner, 0b1111111111111111)
    // }
}
