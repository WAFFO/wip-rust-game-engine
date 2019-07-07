use engine::Engine;
use javascript;
use engine::input::MouseButton::{Left, Middle, Right, M4, M5};

#[derive(Copy, Clone, PartialEq)]
pub enum EventType {
    Press,
    Release,
    Move,
    Scroll,
}

#[derive(Copy, Clone, Default)]
pub struct Mouse {
    left: bool,
    right: bool,
    middle: bool,
    m4: bool,
    m5: bool,
    x: f32,
    y: f32,
    move_x: f32,
    move_y: f32,
    move_s: f32,
}

impl Mouse {
    pub(super) fn new() -> Self {
        Mouse{
            left: false,
            right: false,
            middle: false,
            m4: false,
            m5: false,
            x: 0.0,
            y: 0.0,
            move_x: 0.0,
            move_y: 0.0,
            move_s: 0.0,
        }
    }
    pub fn left(&self) -> bool { self.left }
    pub fn right(&self) -> bool { self.right }
    pub fn middle(&self) -> bool { self.middle }
    pub fn m4(&self) -> bool { self.m4 }
    pub fn m5(&self) -> bool { self.m5 }
    pub fn x(&self) -> f32 { self.x }
    pub fn y(&self) -> f32 { self.y }
    pub fn move_x(&self) -> f32 { self.move_x }
    pub fn move_y(&self) -> f32 { self.move_y }
    pub fn move_s(&self) -> f32 { self.move_s }
}

#[derive(Copy, Clone, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    M4,
    M5,
}

#[derive(Copy, Clone)]
pub struct KeyBoard {
    board: [bool; 256]
}

impl KeyBoard {
    pub(super) fn new() -> Self { KeyBoard{ board: [false; 256] } }
    pub fn get(&self, key: usize) -> bool {
        if key > 255 {
            javascript::log_1("ERROR: key_up: {}", &(key as i32).into());
            false
        }
        else {
            self.board[key]
        }
    }
}

impl std::ops::Index<usize> for KeyBoard {
    type Output = bool;

    fn index(&self, index: usize) -> &bool {
        if index > 255 {
            javascript::log_1("ERROR: key_up: {}", &(index as i32).into());
            &false
        }
        else {
            &self.board[index]
        }
    }
}

impl Engine {
    pub fn js_mouse_click(&mut self, button: u32, buttons: u32, x: f32, y: f32) -> (MouseButton, Mouse) {
        self.mouse.left = (buttons & 1) > 0;
        self.mouse.right = (buttons & 2) > 0;
        self.mouse.middle = (buttons & 4) > 0;
        self.mouse.m4 = (buttons & 8) > 0;
        self.mouse.m5 = (buttons & 16) > 0;
        self.mouse.x = x;
        self.mouse.y = y;

        let current = match button {
            1 => Middle,
            2 => Right,
            3 => M4,
            4 => M5,
            _ => Left,

        };

        (current, self.mouse.clone())
    }

    pub fn js_mouse_press(&mut self, button: u32, buttons: u32, x: f32, y: f32) -> (MouseButton, Mouse) {
        self.mouse.left = (buttons & 1) > 0;
        self.mouse.right = (buttons & 2) > 0;
        self.mouse.middle = (buttons & 4) > 0;
        self.mouse.m4 = (buttons & 8) > 0;
        self.mouse.m5 = (buttons & 16) > 0;
        self.mouse.x = x;
        self.mouse.y = y;

        let current = match button {
            1 => Middle,
            2 => Right,
            3 => M4,
            4 => M5,
            _ => Left,

        };

        (current, self.mouse.clone())
    }

    pub fn js_mouse_release(&mut self, button: u32, buttons: u32, x: f32, y: f32) -> (MouseButton, Mouse) {
        self.mouse.left = (buttons & 1) > 0;
        self.mouse.right = (buttons & 2) > 0;
        self.mouse.middle = (buttons & 4) > 0;
        self.mouse.m4 = (buttons & 8) > 0;
        self.mouse.m5 = (buttons & 16) > 0;
        self.mouse.x = x;
        self.mouse.y = y;

        let current = match button {
            1 => Middle,
            2 => Right,
            3 => M4,
            4 => M5,
            _ => Left,

        };

        (current, self.mouse.clone())
    }

    pub fn js_mouse_move(&mut self, x: f32, y: f32, move_x: f32, move_y: f32) -> Mouse {
        self.mouse.x = x;
        self.mouse.y = y;
        self.mouse.move_x = move_x;
        self.mouse.move_y = move_y;

        self.mouse.clone()
    }

    pub fn js_mouse_scroll(&mut self, s: f32) -> Mouse {
        self.mouse.move_s = s; // maybe: s/s.abs();

        self.mouse.clone()
    }

    pub fn js_key_down(&mut self, key: usize) -> KeyBoard {
        // probably clean engine.input here
        if key > 255 {
            javascript::log_1("ERROR: key_down: {}", &(key as i32).into());
        }
        else {
            self.key_board.board[key] = true;
        }

        self.key_board.clone()
    }

    pub fn js_key_up(&mut self, key: usize) -> KeyBoard {
        // probably clean engine.input here
        if key > 255 {
            javascript::log_1("ERROR: key_up: {}", &(key as i32).into());
        }
        else {
            self.key_board.board[key] = false;
        }

        self.key_board.clone()
    }


}