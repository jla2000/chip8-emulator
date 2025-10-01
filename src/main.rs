use std::io::Read;

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

async fn run(rom: &[u8]) {
    let mut window_state = create_window();
    let mut chip8_state = Chip8State::new();
    let mut renderer = Renderer::new(&window_state.window).await;
    // let mut beeper = Beeper::new();

    chip8_state.load_rom(rom);

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
                _ => {}
            }
        }

        while chip8_state.cycle_available() {
            match chip8_state.emulate_cycle() {
                Some(Chip8Event::UpdateDisplay(video_mem)) => renderer.update_display(video_mem),
                // Some(Chip8Event::StartBeep) => beeper.start(),
                // Some(Chip8Event::StopBeep) => beeper.stop(),
                _ => {}
            }
        }
    }
}

fn read_rom(filename: &str) -> std::io::Result<Vec<u8>> {
    let f = std::fs::File::open(filename)?;
    let mut reader = std::io::BufReader::new(f);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        println!("Usage: chip8-emulator <ROM_FILE>");
    } else {
        let rom = read_rom(&args[1]).unwrap();
        pollster::block_on(run(rom.as_slice()));
    }
}
