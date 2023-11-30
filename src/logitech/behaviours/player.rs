use super::{DriverState, DriverBehaviour, KeyDirection, PressType};

/**
 * This mode can be used to control a media player applications.
 * Left/Right: Rewind/Seek.
 * Play: Return followed by Spacebar.
 * Toggle: Backspace
 */
#[derive(Default)]
pub struct PlayerMode;

impl DriverBehaviour for PlayerMode {
    fn handle_motion(&mut self, state: &mut DriverState, dir: KeyDirection) {
        use KeyDirection::*;

        let has_modifier = state.toggle_flag == PressType::LONG;
        match (has_modifier, dir) {
            (false, LEFT) => state.controller.cursor_left_move(),
            (false, RIGHT) => state.controller.cursor_right_move(),
            (true, LEFT) => state.controller.volume_down(),
            (true, RIGHT) => state.controller.volume_up(),
        }
    }

    fn handle_toggle(&mut self, state: &mut DriverState) {
        if state.toggle_flag == PressType::SHORT {
            state.controller.backspace_click();
        }
    }

    fn handle_play(&mut self, state: &mut DriverState) {
        // Often more useful to have the return key, but play/pause is 
        // controlled by the spacebar: both are therefore triggered.
        state.controller.return_click();
        state.controller.spacebar_click();
    }
}
