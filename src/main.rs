use dotenv::dotenv;
use rodio::OutputStream;
use rodio::Sink;

mod chip8;
mod keyboard;
mod render;
mod sound;
mod window;

use chip8::*;
use render::*;
use sound::*;
use window::*;

async fn run() {
    let mut window_state = create_window();
    let mut chip8_state = Chip8State::new();
    let mut renderer = Renderer::new(&window_state.window).await;
    let mut beeper = Beeper::new();

    chip8_state.load_rom(include_bytes!(r"assets/space_invaders.ch8"));

    let mut cycle_clock = fixedstep::FixedStep::start(500.0);
    let mut timer_clock = fixedstep::FixedStep::start(60.0);

    while !window_state.window.should_close() {
        window_state.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&window_state.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    renderer.resize((width as u32, height as u32));
                }
                glfw::WindowEvent::Key(key, _, action, _) => {
                    chip8_state.keyboard.update(key, action);
                }
                _ => {
                    println!("{:?}", event);
                }
            }
        }

        while cycle_clock.update() {
            match chip8_state.emulate_cycle() {
                Some(Chip8Event::UpdateDisplay) => renderer.update_display(&chip8_state),
                Some(Chip8Event::StartBeep) => beeper.start(),
                Some(Chip8Event::StopBeep) => beeper.stop(),
                None => {}
            }
        }

        if timer_clock.update() {
            chip8_state.update_timers();
        }
    }
}

fn main() {
    dotenv().ok();
    env_logger::init();

    pollster::block_on(run());
}
