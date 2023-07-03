pub struct Keyboard {
    key_state: [bool; 16],
    wait_for_key: bool,
    last_press: Option<usize>,
}

fn key_mapping(key: glfw::Key) -> Option<usize> {
    match key {
        glfw::Key::Num1 => Some(0),
        glfw::Key::Num2 => Some(1),
        glfw::Key::Num3 => Some(2),
        glfw::Key::Num4 => Some(3),
        glfw::Key::Q => Some(4),
        glfw::Key::W => Some(5),
        glfw::Key::E => Some(6),
        glfw::Key::R => Some(7),
        glfw::Key::A => Some(8),
        glfw::Key::S => Some(9),
        glfw::Key::D => Some(10),
        glfw::Key::F => Some(11),
        glfw::Key::Z => Some(12),
        glfw::Key::X => Some(13),
        glfw::Key::C => Some(14),
        glfw::Key::V => Some(15),
        _ => None,
    }
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            key_state: [false; 16],
            wait_for_key: false,
            last_press: None,
        }
    }

    pub fn update(&mut self, key: glfw::Key, action: glfw::Action) {
        if let Some(key_index) = key_mapping(key) {
            let key_pressed = action != glfw::Action::Release;

            self.key_state[key_index] = key_pressed;
            if key_pressed {
                self.last_press = Some(key_index);
            }
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
