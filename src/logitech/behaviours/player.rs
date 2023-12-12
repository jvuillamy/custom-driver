use super::{DriverBehaviour, KeyDirection, PressType, SharedState};

/**
 * This mode can be used to control a media player applications.
 * - Left/Right: Rewind/Seek.
 * - Play: Return followed by Spacebar.
 * - Toggle: Backspace
 */
#[derive(Default)]
pub struct PlayerMode;

impl DriverBehaviour for PlayerMode {
    fn handle_motion(&mut self, state: &mut impl SharedState, dir: KeyDirection) {
        let has_modifier = state.get_toggle_flag() == PressType::LONG;

        use KeyDirection as KD;
        match (has_modifier, dir) {
            (false, KD::LEFT) => state.get_controller().cursor_left_move(),
            (false, KD::RIGHT) => state.get_controller().cursor_right_move(),
            (true, KD::LEFT) => state.get_controller().volume_down(),
            (true, KD::RIGHT) => state.get_controller().volume_up(),
        }
    }

    fn handle_toggle(&mut self, state: &mut impl SharedState) {
        if state.get_toggle_flag() == PressType::SHORT {
            state.get_controller().backspace_click();
        }
    }

    fn handle_play(&mut self, state: &mut impl SharedState) {
        // Often more useful to have the return key, but play/pause is
        // controlled by the spacebar: both are therefore triggered.
        state.get_controller().return_click();
        state.get_controller().spacebar_click();
    }
}
