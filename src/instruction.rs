#[derive(Debug)]
pub enum Instruction {
    ClearScreen,
    Jump(u16),
    SubroutineCall(u16),
    SubroutineReturn,
    SkipEqualValue(usize, u8, bool),
    SkipEqualRegister(usize, usize, bool),
    SetRegister(usize, u8),
    AddValueRegister(usize, u8),
    Arithmetic(usize, usize, Operator),
    SetIndex(u16),
    JumpOffset(usize, u16),
    Random(usize, u8),
    Draw(usize, usize, u8),
    SkipIfKey(usize, bool),
    ReadDelayTimer(usize),
    SetDelayTimer(usize),
    SetSoundTimer(usize),
    AddToIndex(usize),
    GetKey(usize),
    FontCharacter(usize),
    DecimalConversion(usize),
    StoreRegisters(usize),
    LoadRegisters(usize),
    System,
    Unknown(u8, u16),
}

#[derive(Debug)]
pub enum Operator {
    Set,
    BinaryOr,
    BinaryAnd,
    BinaryXor,
    Add,
    Subtract,
    SubtractInverse,
    ShiftL,
    ShiftR,
    Unknown(u8),
}

impl From<(u8, u8)> for Instruction {
    fn from((byte1, byte2): (u8, u8)) -> Self {
        let opcode = (byte1 & 0xF0) >> 4;
        let x = get_register_index(byte1 & 0x0F);
        let y = get_register_index(byte2 & 0xF0);
        let n = byte2 & 0x0F;
        let nn = byte2;
        let nnn = (byte1 as u16 & 0x0F) << 8 | byte2 as u16;

        match opcode {
            0x0 => opcode_0(opcode, nn, nnn),
            0x1 => Instruction::Jump(nnn),
            0x2 => Instruction::SubroutineCall(nnn),
            0x3 => Instruction::SkipEqualValue(x, nn, true),
            0x4 => Instruction::SkipEqualValue(x, nn, false),
            0x5 => Instruction::SkipEqualRegister(x, y, true),
            0x6 => Instruction::SetRegister(x, nn),
            0x7 => Instruction::AddValueRegister(x, nn),
            0x8 => Instruction::Arithmetic(x, y, arithmetic_operator(nn)),
            0x9 => Instruction::SkipEqualRegister(x, y, false),
            0xA => Instruction::SetIndex(nnn),
            0xB => Instruction::JumpOffset(x, nnn),
            0xC => Instruction::Random(x, nn),
            0xD => Instruction::Draw(x, y, n),
            0xE => opcode_e(opcode, x, nn, nnn),
            0xF => opcode_f(opcode, x, nn, nnn),
            _ => Instruction::Unknown(opcode, nnn),
        }
    }
}

fn opcode_0(opcode: u8, nn: u8, nnn: u16) -> Instruction {
    match nn {
        0xE0 => Instruction::ClearScreen,
        0xEE => Instruction::SubroutineReturn,
        _ => Instruction::System,
    }
}

fn arithmetic_operator(nn: u8) -> Operator {
    match nn & 0x0F {
        0x0 => Operator::Set,
        0x1 => Operator::BinaryOr,
        0x2 => Operator::BinaryAnd,
        0x3 => Operator::BinaryXor,
        0x4 => Operator::Add,
        0x5 => Operator::Subtract,
        0x6 => Operator::ShiftR,
        0x7 => Operator::SubtractInverse,
        0xE => Operator::ShiftL,
        _ => Operator::Unknown(nn & 0x0F),
    }
}

fn opcode_e(opcode: u8, x: usize, nn: u8, nnn: u16) -> Instruction {
    match nn {
        0x9E => Instruction::SkipIfKey(x, true),
        0xA1 => Instruction::SkipIfKey(x, false),
        _ => Instruction::Unknown(opcode, nnn),
    }
}

fn opcode_f(opcode: u8, x: usize, nn: u8, nnn: u16) -> Instruction {
    match nn {
        0x07 => Instruction::ReadDelayTimer(x),
        0x15 => Instruction::SetDelayTimer(x),
        0x18 => Instruction::SetSoundTimer(x), // TODO : Set sound timer to make beeping sound as long as above 0
        0x1E => Instruction::AddToIndex(x),
        0x0A => Instruction::GetKey(x),
        0x29 => Instruction::FontCharacter(x),
        0x33 => Instruction::DecimalConversion(x),
        0x55 => Instruction::StoreRegisters(x),
        0x65 => Instruction::LoadRegisters(x),
        _ => Instruction::Unknown(opcode, nnn),
    }
}
fn get_register_index(nibble: u8) -> usize {
    let mut index = nibble;
    if index >= 16 {
        index = index >> 4;
    }
    index as usize
}
