pub struct Keypad {
    keys: [bool; 16] // 0 to F
}

impl Keypad {
    pub fn new() -> Self {
        Keypad { keys: [false;16] }
    }

    pub fn reset(&mut self) {
        self.keys = [false;16];
    }

    pub fn key_down(&mut self, key: usize) {
        self.keys[key] = true
    }

    pub fn is_key_down(&mut self, key:usize) -> bool {
        if self.keys[key] {return true}
        false
    }

    pub fn is_key_up(&mut self, key:usize) -> bool {
        if !self.keys[key] {return true}
        false
    }
}