use rand::Rng;

use crate::keyboard::*;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
const ROM_START_ADDRESS: usize = 0x200;
const VF: usize = 15;

const FONTSET: &[u8] = &[
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E 0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub fn nibbles(value: u16) -> (u8, u8, u8, u8) {
    (
        ((value & 0xf000) >> 12) as u8,
        ((value & 0x0f00) >> 8) as u8,
        ((value & 0x00f0) >> 4) as u8,
        (value & 0x000f) as u8,
    )
}

pub type VideoMemory = [u8; SCREEN_WIDTH * SCREEN_HEIGHT];

pub enum Chip8Event<'a> {
    UpdateDisplay(&'a VideoMemory),
    StartBeep,
    StopBeep,
}

pub struct Chip8State {
    pub keyboard: Keyboard,
    video_mem: VideoMemory,
    memory: [u8; 4096],
    stack: Vec<u16>,
    regs: [u8; 16],
    pc: usize,
    i: u16,
    sound_timer: u8,
    delay_timer: u8,
    timer_clock: fixedstep::FixedStep,
    cycle_clock: fixedstep::FixedStep,
    beeping: bool,
}

impl Chip8State {
    pub fn new() -> Self {
        let mut memory = [0x00; 4096];
        memory[0..FONTSET.len()].clone_from_slice(FONTSET);

        Self {
            keyboard: Keyboard::new(),
            memory,
            stack: vec![],
            video_mem: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
            regs: [0x00; 16],
            pc: 0x0000,
            i: 0x0000,
            sound_timer: 0x00,
            delay_timer: 0x00,
            timer_clock: fixedstep::FixedStep::start(60.0),
            cycle_clock: fixedstep::FixedStep::start(500.0),
            beeping: false,
        }
    }

    pub fn load_rom(&mut self, rom_bytes: &[u8]) {
        self.memory[ROM_START_ADDRESS..ROM_START_ADDRESS + rom_bytes.len()]
            .clone_from_slice(rom_bytes);
        self.pc = ROM_START_ADDRESS;
    }

    pub fn fetch_next_opcode(&self) -> u16 {
        ((self.memory[self.pc] as u16) << 8) | (self.memory[self.pc + 1] as u16)
    }

    pub fn cycle_available(&mut self) -> bool {
        self.cycle_clock.update()
    }

    pub fn emulate_cycle(&mut self) -> Option<Chip8Event> {
        let mut event = None;

        let opcode = self.fetch_next_opcode();
        let nnn = opcode & 0x0fff;
        let nn = opcode as u8;

        match nibbles(opcode) {
            (0x0, 0x0, 0xE, 0x0) => {
                self.video_mem.iter_mut().for_each(|v| *v = 0);
            }
            (0x0, 0x0, 0xE, 0xE) => {
                let return_address = self.stack.pop().unwrap();
                self.pc = return_address as usize;
                return None;
            }
            (0x1, _, _, _) => {
                self.pc = nnn as usize;
                return None;
            }
            (0x2, _, _, _) => {
                self.stack.push((self.pc + 2) as u16);
                self.pc = nnn as usize;
                return None;
            }
            (0x3, vx, _, _) => {
                if self.regs[vx as usize] == nn {
                    self.pc += 2;
                }
            }
            (0x4, vx, _, _) => {
                if self.regs[vx as usize] != nn {
                    self.pc += 2;
                }
            }
            (0x5, vx, vy, _) => {
                if self.regs[vx as usize] == self.regs[vy as usize] {
                    self.pc += 2;
                }
            }
            (0x6, vx, _, _) => {
                self.regs[vx as usize] = nn;
            }
            (0x7, vx, _, _) => {
                self.regs[vx as usize] = self.regs[vx as usize].wrapping_add(nn);
            }
            (0x8, vx, vy, 0x0) => {
                self.regs[vx as usize] = self.regs[vy as usize];
            }
            (0x8, vx, vy, 0x1) => {
                self.regs[vx as usize] |= self.regs[vy as usize];
            }
            (0x8, vx, vy, 0x2) => {
                self.regs[vx as usize] &= self.regs[vy as usize];
            }
            (0x8, vx, vy, 0x3) => {
                self.regs[vx as usize] ^= self.regs[vy as usize];
            }
            (0x8, vx, vy, 0x4) => {
                let (result, carry) =
                    self.regs[vx as usize].overflowing_add(self.regs[vy as usize]);
                self.regs[vx as usize] = result;
                self.regs[VF] = carry as u8;
            }
            (0x8, vx, vy, 0x5) => {
                let (result, borrow) =
                    self.regs[vx as usize].overflowing_sub(self.regs[vy as usize]);
                self.regs[vx as usize] = result;
                self.regs[VF] = !borrow as u8;
            }
            (0x8, vx, _, 0x6) => {
                let bit = self.regs[vx as usize] & 0x01;
                self.regs[VF] = bit;
                self.regs[vx as usize] >>= 1;
            }
            (0x8, vx, vy, 0x7) => {
                let (result, borrow) =
                    self.regs[vy as usize].overflowing_sub(self.regs[vx as usize]);
                self.regs[vx as usize] = result;
                self.regs[VF] = !borrow as u8;
            }
            (0x8, vx, _, 0xE) => {
                let most_significant_bit = self.regs[vx as usize] & 0x80;
                self.regs[VF] = most_significant_bit;
                self.regs[vx as usize] <<= 1;
            }
            (0x9, vx, vy, 0x0) => {
                if self.regs[vx as usize] != self.regs[vy as usize] {
                    self.pc += 2;
                }
            }
            (0xA, _, _, _) => {
                self.i = nnn;
            }
            (0xB, _, _, _) => {
                self.pc = (self.regs[0] as u16 + nnn) as usize;
                return None;
            }
            (0xC, vx, _, _) => {
                let random: u8 = rand::thread_rng().gen();
                self.regs[vx as usize] = random & nn;
            }
            (0xD, vx, vy, n) => {
                let x = self.regs[vx as usize] as usize % SCREEN_WIDTH;
                let y = self.regs[vy as usize] as usize % SCREEN_HEIGHT;
                let height = n as usize;

                self.regs[VF] = 0;

                for y_offset in 0..height {
                    let sprite_byte = self.memory[(self.i + y_offset as u16) as usize];

                    for x_offset in 0..8usize {
                        if sprite_byte & (0x80 >> x_offset) != 0 {
                            let dst_pixel = (y + y_offset) * SCREEN_WIDTH + x + x_offset;
                            if self.video_mem[dst_pixel] == 1 {
                                self.regs[VF] = 1;
                            }
                            self.video_mem[dst_pixel] ^= 1;
                        }
                    }
                }

                event = Some(Chip8Event::UpdateDisplay(&self.video_mem));
            }
            (0xE, vx, 0x9, 0xE) => {
                let key_index = self.regs[vx as usize] as usize;
                if self.keyboard.get_key_state(key_index) {
                    self.pc += 2;
                }
            }
            (0xE, vx, 0xA, 0x1) => {
                let key_index = self.regs[vx as usize] as usize;
                if !self.keyboard.get_key_state(key_index) {
                    self.pc += 2;
                }
            }
            (0xF, vx, 0x0, 0x7) => {
                self.regs[vx as usize] = self.delay_timer;
            }
            (0xF, vx, 0x0, 0xA) => {
                if let Some(key_index) = self.keyboard.wait_for_key() {
                    self.regs[vx as usize] = key_index as u8;
                } else {
                    return None;
                }
            }
            (0xF, vx, 0x1, 0x5) => {
                self.delay_timer = self.regs[vx as usize];
            }
            (0xF, vx, 0x1, 0x8) => {
                self.sound_timer = self.regs[vx as usize];
            }
            (0xF, vx, 0x1, 0xE) => {
                self.i += self.regs[vx as usize] as u16;
            }
            (0xF, vx, 0x2, 0x9) => {
                self.i = (self.regs[vx as usize] as u16) * 5;
            }
            (0xF, vx, 0x3, 0x3) => {
                self.memory[self.i as usize] = self.regs[vx as usize] / 100;
                self.memory[self.i as usize + 1] = (self.regs[vx as usize] / 10) % 10;
                self.memory[self.i as usize + 2] = (self.regs[vx as usize] % 100) % 10;
            }
            (0xF, vx, 0x5, 0x5) => {
                let start = self.i as usize;
                let end = start + (vx as usize) + 1;
                self.memory[start..end].clone_from_slice(&self.regs[..(vx + 1) as usize]);
            }
            (0xF, vx, 0x6, 0x5) => {
                let start = self.i as usize;
                let end = start + (vx as usize) + 1;
                self.regs[..(vx + 1) as usize].clone_from_slice(&self.memory[start..end]);
            }
            _ => {
                unimplemented!("0x{:04x}: 0x{:04x}", self.pc, opcode);
            }
        }

        while self.timer_clock.update() {
            self.delay_timer = self.delay_timer.saturating_sub(1);
            self.sound_timer = self.sound_timer.saturating_sub(1);
        }

        if self.sound_timer > 0 && !self.beeping {
            event = Some(Chip8Event::StartBeep);
            self.beeping = true;
        } else if self.sound_timer == 0 && self.beeping {
            event = Some(Chip8Event::StopBeep);
            self.beeping = false;
        }

        self.pc += 2;
        event
    }
}
