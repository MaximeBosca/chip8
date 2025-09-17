use crate::state::State;

pub fn write_timer(state: &State) -> String {
    format!(
        "DELAY: {:>3}   |  SOUND: {:>3}",
        state.delay_timer, state.sound_timer
    )
}
