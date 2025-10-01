use raylib::prelude::*;

use std::io::Read;

mod chip8;
mod keyboard;

use chip8::*;

fn run(rom: &[u8]) {
    let mut chip8_state = Chip8State::new();
    chip8_state.load_rom(rom);

    let (mut rl, thread) = raylib::init()
        .size(1000, 500)
        .title("chip8-emulator")
        .build();

    let mut blank_image = Image::gen_image_color(64, 32, Color::BLACK);
    blank_image.set_format(PixelFormat::PIXELFORMAT_UNCOMPRESSED_GRAYSCALE);

    let mut screen_texture = rl.load_texture_from_image(&thread, &blank_image).unwrap();

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let mut draw_handle = rl.begin_drawing(&thread);

        while chip8_state.cycle_available() {
            match chip8_state.emulate_cycle() {
                Some(Chip8Event::UpdateDisplay(video_mem)) => {
                    let fixed_mem: Vec<_> = video_mem.iter().map(|v| v * 255).collect();

                    screen_texture.update_texture(&fixed_mem);
                    draw_handle.draw_texture_pro(
                        &screen_texture,
                        Rectangle::new(
                            0.0,
                            0.0,
                            screen_texture.width as f32,
                            screen_texture.height as f32,
                        ),
                        Rectangle::new(0.0, 0.0, 1000.0, 500.0),
                        Vector2::new(0.0, 0.0),
                        0.0,
                        Color::WHITE,
                    );
                    draw_handle.draw_fps(0, 0);
                }
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
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        println!("Usage: chip8-emulator <ROM_FILE>");
    } else {
        let rom = read_rom(&args[1]).unwrap();
        run(rom.as_slice());
    }
}
