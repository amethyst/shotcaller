use crate::*;

/// Updates InputState resource based on most recent player's input.
pub fn update_input_state_system(
    gamemode: &GameMode,
    events: &Vec<InputEvent>,
    // mouse_events: &Vec<MouseEvent>,
    input_state: &mut InputState,
) -> SystemResult {
    // Only relevant for micro-input game mode
    match gamemode {
        GameMode::Shotcaller => return Ok(()),
        GameMode::MicroInput => {}
    }

    for k in events.iter() {
        match k {
            InputEvent::ResetInputState => {
                *input_state = InputState::Default;
            }
            InputEvent::MMove => {
                *input_state = InputState::MMove;
            }
            InputEvent::AMove => {
                *input_state = InputState::AMove;
            }
            _ => {}
        }
    }
    Ok(())
}
