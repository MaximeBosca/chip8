const LAYOUT: [[u8; 4]; 4] = [
    [0x1, 0x2, 0x3, 0xC],
    [0x4, 0x5, 0x6, 0xD],
    [0x7, 0x8, 0x9, 0xE],
    [0xA, 0x0, 0xB, 0xF],
];
const HYPHEN_LINE: &str = "----------";
pub(crate) fn write_fn_controls() -> String {
    "    F1: PLAY/PAUSE | F2: STEP | F3: RESET | F4: EXIT".to_string()
}

pub(crate) fn write_game_controls() -> String {
    "   -----------
   | 1 2 3 4 |
_  | Q W E R |
_  | A S D F |
   | Z X C V |
   -----------"
        .to_string()
}

pub fn game_pad() -> [[u8; 4]; 4] {
    LAYOUT
}

pub(crate) fn write_hyphen_line() -> String {
    HYPHEN_LINE.to_string()
}
