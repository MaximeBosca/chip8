#[derive(Debug)]
pub enum Instruction {
    ClearScreen,
    Jump(u16),
    SetRegister(usize, u8),
    AddValueRegister(usize, u8),
    SetIndex(u16),
    Draw(usize, usize, u8),
    Unknown(u8, u16)
}
impl From<(u8, u8)> for Instruction {
    fn from((byte1, byte2): (u8, u8)) -> Self {
        let opcode = byte1 & 0xF0;
        let x = get_register_index(byte1 & 0x0F);
        let y = get_register_index(byte2 & 0xF0);
        let n = byte2 & 0x0F;
        let nn = byte2;
        let nnn = (byte1 as u16 & 0x0F) << 8 | byte2 as u16;

        match opcode {
            0x00 => Instruction::ClearScreen,
            0x10 => Instruction::Jump(nnn),
            0x60 => Instruction::SetRegister(x, nn),
            0x70 => Instruction::AddValueRegister(x, nn),
            0xA0 => Instruction::SetIndex(nnn),
            0xD0 => Instruction::Draw(x, y, n),
            _ => {
                Instruction::Unknown(opcode, nnn)
            }
        }
    }
}

fn get_register_index(nibble: u8) -> usize {
    let mut index = nibble;
    if index >= 16 {
        index = index >> 4;
    }
    index as usize
}