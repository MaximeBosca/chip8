pub(crate) fn write_controls() -> String {
    "    F1: PLAY/PAUSE | F2: STEP | F3: RESET | F4: EXIT
    -----------     -----------
    | 1 2 3 C |     | 1 2 3 4 |
    | 4 5 6 D |  _  | Q W E R |
    | 7 8 9 E |  _  | A S D F |
    | A 0 B F |     | Z X C V |
    -----------     -----------"
    .to_string()
}