use dotenv::dotenv;
use wgpu::util::DeviceExt;

mod chip8;
mod graphics;
mod utility;
mod window;

use chip8::*;
use graphics::*;
use window::*;

async fn run() {
    let mut window_state = create_window();
    let mut chip8_state = Chip8State::new();
    let mut wgpu_state = WgpuState::new(&window_state.window).await;

    chip8_state.load_rom(include_bytes!("assets/picture.ch8"));

    while !window_state.window.should_close() {
        window_state.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&window_state.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    wgpu_state.resize((width as u32, height as u32));
                }
                _ => {
                    println!("{:?}", event);
                }
            }
        }

        match wgpu_state.render() {
            Ok(_) => {}
            Err(wgpu::SurfaceError::Lost) => wgpu_state.resize(wgpu_state.size),
            Err(wgpu::SurfaceError::OutOfMemory) => break,
            Err(e) => println!("{:?}", e),
        }

        chip8_state.emulate_cycle();
    }
}

fn main() {
    dotenv().ok();
    env_logger::init();

    pollster::block_on(run());
}
