use super::{DriverBehaviour, DriverState, KeyDirection, PressType};

/**
 * This mode controls the mouse with the left/right buttons. A toggle long press
 * is used as a modifier and maps left/right keys to up/down. Short press on:
 *  - play button clicks the left mouse button.
 *  - toggle button clicks the right mouse button.
 */
#[derive(Default)]
pub struct MouseMode;

impl DriverBehaviour for MouseMode {
    fn handle_motion(&mut self, state: &mut DriverState, dir: KeyDirection) {
        const MOVE_SPEED: i32 = 20;
        let sign: i32 = if dir == KeyDirection::LEFT { -1 } else { 1 };

        let has_modifier = state.toggle_flag == PressType::LONG;
        let (x, y) = if has_modifier {
            (0, sign * MOVE_SPEED)
        } else {
            (sign * MOVE_SPEED, 0)
        };

        state.controller.mouse_move(x, y);
    }

    fn handle_toggle(&mut self, state: &mut DriverState) {
        if state.toggle_flag == PressType::SHORT {
            state.controller.mouse_right_click();
        }
    }

    fn handle_play(&mut self, state: &mut DriverState) {
        state.controller.mouse_left_click();
    }
}
