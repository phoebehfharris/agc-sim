pub enum Instruction {
    AD(usize),
    ADS(usize),
    AUG(usize),
    BZF(usize),
    BZMF(usize),
    CA(usize),
    CCS(usize),
    // COM, // WHAT DO YOU MEAN: compiles to CS A, what do I do :sob:
    CS(usize),
    DAS(usize),
    DCA(usize),
    DCOM,
    DCS(usize),
    DDOUBL,
    DIM(usize),
    // DOUBLE,
    // DTCB,
    // DTCF,
    DV(usize),
    DXCH(usize),
    EXTEND,
    INCR(usize),
    INDEX(usize),
    INHINT,
    LXCH(usize),
    MASK(usize),
    MP(usize),
    MSU(usize),
    // NOOP,
    // OVSK,
    QXCH(usize),
    RAND(usize),
    READ(usize),
    RELINT,
    RESUME,
    // RETURN,
    ROR(usize),
    RXOR(usize),
    SQUARE,
    SU(usize),
    TC(usize),
    // TCAA,
    TCF(usize),
    TS(usize),
    WAND(usize),
    WOR(usize),
    WRITE(usize),
    XCH(usize),
    // XLQ,
    // XXALQ,
    ZL,
    ZQ,
    ERR, // This is not a real instruction, just something to test for parse errors
}

pub fn parse_bytes(b1: u16, b2: u16) -> (Instruction, usize) {
    let mut next_instruction_offset = 1;
    let mut extend = false;
    if b1 == 0o00006 {
        next_instruction_offset = 2;
        extend = true;
    }

    let instruction = match extend {
        true => parse_extended(b2),
        false => parse_normal(b1),
    };
    (instruction, next_instruction_offset)
}

fn parse_normal(b: u16) -> Instruction {
    let operator_mask = 0b0_111_000_000_000_000;
    let operand_mask = !operator_mask;
    let qq_mask = 0b0_000_110_000_000_000;
    let short_operand_mask = !(operator_mask | qq_mask);
    // let ad_mask      = 0b0_000_110_000_000_000;
    let instruction = match b & operator_mask {
        // TC esque instructions
        0o00000 => match b & short_operand_mask {
            // 0o00002 => Instruction::RETURN, // Can be removed,
            0o00003 => Instruction::RELINT, // Can be reomved
            0o00004 => Instruction::INHINT, // Can be removed
            0o00006 => Instruction::ERR,    // This is Extend and is expected
            _ => Instruction::TC((b & operand_mask) as usize),
        },
        0o10000 => {
            let addr = (b & operand_mask) as usize;
            if addr < 0o2000 {
                Instruction::CCS(addr)
            } else {
                Instruction::TCF(addr)
            }
        }
        0o20000 => match b & qq_mask {
            0o00000 => Instruction::DAS(((b - 1) & short_operand_mask) as usize),
            0o02000 => match b & short_operand_mask {
                0o00007 => Instruction::ZL,
                _ => Instruction::LXCH((b & short_operand_mask) as usize),
            },
            0o04000 => Instruction::INCR((b & short_operand_mask) as usize),
            0o06000 => Instruction::ADS((b & short_operand_mask) as usize),
            _ => Instruction::ERR,
        },
        0o30000 => Instruction::CA((b & operand_mask) as usize),
        0o40000 => Instruction::CS((b & operand_mask) as usize),
        0o50000 => match b & qq_mask {
            0o00000 => match b & short_operand_mask {
                0o00017 => Instruction::RESUME,
                _ => Instruction::INDEX((b & short_operand_mask) as usize),
            },
            0o02000 => Instruction::DXCH(((b - 1) & short_operand_mask) as usize),
            0o04000 => Instruction::TS((b & short_operand_mask) as usize),
            0o06000 => Instruction::XCH((b & short_operand_mask) as usize),
            _ => Instruction::ERR,
        },
        0o60000 => Instruction::AD((b & operand_mask) as usize),
        0o70000 => Instruction::MASK((b & operand_mask) as usize),
        _ => Instruction::ERR,
    };
    instruction
}
fn parse_extended(b: u16) -> Instruction {
    todo!()
}
