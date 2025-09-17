use crate::state::State;

pub fn write_stack(state: &State) -> String {
    state
        .stack
        .read_all()
        .enumerate()
        .map(|(i, value)| format!("{:02} 0x{:04X}", i, value))
        .reduce(|acc, item| format!("{}\n{}", acc, item))
        .unwrap_or(String::from(" "))
}
