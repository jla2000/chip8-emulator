use std::sync::mpsc::Receiver;

const WINDOW_WIDTH: u32 = 1000;
const WINDOW_HEIGHT: u32 = 500;
const WINDOW_TITLE: &str = "Chip8 Emulator";

pub struct WindowState {
    pub glfw: glfw::Glfw,
    pub window: glfw::Window,
    pub events: Receiver<(f64, glfw::WindowEvent)>,
}

pub fn create_window() -> WindowState {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::SRgbCapable(true));
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
    glfw.window_hint(glfw::WindowHint::Resizable(false));

    let (mut window, events) = glfw
        .create_window(
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            WINDOW_TITLE,
            glfw::WindowMode::Windowed,
        )
        .unwrap();

    window.set_framebuffer_size_polling(true);

    WindowState {
        glfw,
        window,
        events,
    }
}
