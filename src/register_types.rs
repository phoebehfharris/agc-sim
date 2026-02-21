// 16-bit register type with an overflow bit
struct AccumulatorInteger {
    inner: i16,
}

impl Add for AccumulatorInteger {
    type Output = Self;

    fn add(self, other: self) {
        unimplemented!();
}

impl From<MemoryInteger> for AccumulatorInteger {
    fn from(mem_int: MemoryInteger) -> Self {
        let mut tmp = mem_int;

        unimplemented!();
    }
}

struct MemoryInteger {
    inner: i16
}

impl From<AccumulatorInteger> for MemoryInteger {
    fn from(acc_int: AccumulatorInteger) -> Self {
        return acc_int & 0b1111111111111110;
    }
}
