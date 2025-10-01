pub struct Keyboard {
    key_state: [bool; 16],
    wait_for_key: bool,
    last_press: Option<usize>,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            key_state: [false; 16],
            wait_for_key: false,
            last_press: None,
        }
    }

    pub fn update(&mut self, index: usize, pressed: bool) {
        self.key_state[index] = pressed;
        if pressed {
            self.last_press = Some(index);
        }
    }

    pub fn wait_for_key(&mut self) -> Option<usize> {
        if !self.wait_for_key {
            self.wait_for_key = true;
            self.last_press = None;
        } else if self.last_press.is_some() {
            self.wait_for_key = false;
        }

        self.last_press
    }

    pub fn is_key_pressed(&mut self, index: usize) -> bool {
        self.key_state[index]
    }
}
