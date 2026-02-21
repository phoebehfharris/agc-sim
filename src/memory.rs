trait BankOps {
    fn write(&mut self, location: usize, value: u16);
    fn read(&self, location: usize) -> u16;
}

struct Bank {
    mem: [u16; 0o2000],
}

struct EBank {
    mem: [u16; 0o400],
}

struct NullBank {}

impl BankOps for Bank {
    fn write(&mut self, location: usize, value: u16) {
        self.mem[location] = value;
    }
    fn read(&self, location: usize) -> u16 {
        self.mem[location]
    }
}

impl BankOps for EBank {
    fn write(&mut self, location: usize, value: u16) {
        self.mem[location] = value;
    }
    fn read(&self, location: usize) -> u16 {
        self.mem[location]
    }
}

impl BankOps for NullBank {
    fn write(&mut self, _location: usize, _value: u16) {}
    fn read(&self, _location: usize) -> u16 {
        0
    }
}

struct Memory {
    ebanks: [EBank; 8],
    banks: [Bank; 43],
    null: NullBank,
}

impl Memory {
    fn map_bank(&mut self, bank: usize) -> &mut Bank {
        return &mut self.banks[bank];
    }
    fn map_ebank(&mut self, ebank: usize) -> &mut EBank {
        return &mut self.ebanks[ebank];
    }

    fn select_location(
        &mut self,
        location: usize,
        eb_select: usize,
        bank_select: usize,
    ) -> Option<(&mut dyn BankOps, usize)> {
        if location > 0o7777 {
            return None;
        }
        let bank: &mut dyn BankOps = match location {
            0..=0o0377 => &mut self.ebanks[0],
            0o400..=0o777 => &mut self.ebanks[1],
            0o1000..=0o1377 => &mut self.ebanks[2],
            0o1400..=0o1777 => &mut self.ebanks[eb_select],
            0o2000..=0o3777 => &mut self.banks[bank_select],
            0o4000..=0o5777 => &mut self.banks[02],
            0o6000..=0o7777 => &mut self.banks[03],

            _ => &mut self.null,
        };
        let bank_base_offset: usize = match location {
            0..=0o0377 => 0,
            0o400..=0o777 => 0o400,
            0o1000..=0o1377 => 0o1000,
            0o1400..=0o1777 => 0o1400,
            0o2000..=0o3777 => 0o2000,
            0o4000..=0o5777 => 0o4000,
            0o6000..=0o7777 => 0o6000,

            _ => 0,
        };
        return Some((bank, bank_base_offset));
    }

    pub fn read_location(
        &mut self,
        location: usize,
        eb_select: usize,
        bank_select: usize,
    ) -> Option<u16> {
        let (bank, offset) = self.select_location(location, eb_select, bank_select)?;

        Some(bank.read(location - offset))
    }

    pub fn write_location(
        &mut self,
        location: usize,
        eb_select: usize,
        bank_select: usize,
        value: u16,
    ) {
        let (bank, offset) = match self.select_location(location, eb_select, bank_select) {
            Some((b, o)) => (b, o),
            None => return,
        };

        bank.write(location - offset, value);
    }
}
