use enigo::{Enigo, KeyboardControllable, MouseButton, MouseControllable};

#[derive(Debug, Default)]
pub struct VirtualMouseKeyboard {
    engine: Enigo,
}

impl VirtualMouseKeyboard {
    pub fn mouse_move(&mut self, x: i32, y: i32) {
        self.engine.mouse_move_relative(x, y);
    }

    pub fn mouse_left_click(&mut self) {
        self.engine.mouse_click(MouseButton::Left);
    }

    pub fn mouse_right_click(&mut self) {
        self.engine.mouse_click(MouseButton::Right);
    }

    pub fn add_character(&mut self, c: char) {
        self.engine.key_sequence(&c.to_string());
    }

    pub fn backspace_click(&mut self) {
        self.engine.key_click(enigo::Key::Backspace);
    }

    pub fn spacebar_click(&mut self) {
        self.engine.key_click(enigo::Key::Space);
    }

    pub fn return_click(&mut self) {
        self.engine.key_click(enigo::Key::Return);
    }

    pub fn cursor_left_move(&mut self) {
        self.engine.key_click(enigo::Key::LeftArrow);
    }

    pub fn cursor_right_move(&mut self) {
        self.engine.key_click(enigo::Key::RightArrow);
    }

    pub fn volume_up(&mut self) {
        self.engine.key_click(enigo::Key::UpArrow);
    }

    pub fn volume_down(&mut self) {
        self.engine.key_click(enigo::Key::DownArrow);
    }
}
