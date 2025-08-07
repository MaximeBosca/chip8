use crate::state::State;

//TODO : Once step by step mode available, check and fix stack display
pub fn write_stack(state: &State) -> String {
    state.stack.read_all()
        .enumerate()
        .map(|(i, value) | {
            format!("{:02} 0x{:04X}", i, value)
        })
        .reduce(|acc, item| { format!("{}\n{}", acc, item)})
        .unwrap_or(String::from(" "))
}