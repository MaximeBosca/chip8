use crate::state::State;

pub fn write_registers(state: &State) -> String {
    (0..state.register_numbers())
        .map(|register_index| {write_register(register_index, state)})
        .reduce(|a, b| format!("{}    {}", a, b))
        .unwrap()
}

fn write_register(index: usize, state: &State) -> String {
    format!("V{:X}: {:#04X}", index, state.register(index))
}