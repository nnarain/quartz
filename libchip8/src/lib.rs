
extern crate rand;
use std::fmt;
use std::num::Wrapping;
use std::time::Instant;

const MEMORY_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const NUM_REGISTERS: usize = 16;
const NUM_KEYS: usize = 16;
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const FRAMEBUFFER_SIZE: usize = 3 * DISPLAY_WIDTH * DISPLAY_HEIGHT;

const PROGRAM_START_ADDRESS: u16 = 0x200;

/// Representation of Chip8 Virtual Machine
pub struct Chip8<'a> {
    memory: [u8; MEMORY_SIZE],     // 4096 bytes of memory
    stack:  [u16; STACK_SIZE],     // 16 bytes of stack
    pc:     u16,                   // program counter
    sp:     u8,                    // stack pointer
    i:      u16,                   // index register
    v:      [u8; NUM_REGISTERS],   // general purpose registers
    dt:     u8,                    // delay timer
    st:     u8,                    // sound timer

    keys: [bool; NUM_KEYS],              // key values
    key_wait: Option<Box<FnMut() -> u8 + 'a>>, // function that waits for key press and returns value

    display_memory: [u8; FRAMEBUFFER_SIZE], // display memory
    on_display_update: Option<Box<FnMut() + 'a>>,

    time: Instant
}

/// Chip8 instructions
#[derive(Debug)]
enum Instruction {
//    SYS(),
    CLS(),
    RET(),
    JP(u16),
    CALL(u16),
    SEVXB(usize, u8),
    SNEVXB(usize, u8),
    SEVXY(usize, usize),
    LDVXB(usize, u8),
    ADDVXB(usize, u8),
    LDVXY(usize, usize),
    ORVXY(usize, usize),
    ANDVXY(usize, usize),
    XORVXY(usize, usize),
    ADDVXY(usize, usize),
    SUBVXY(usize, usize),
    SHR(usize),
    SUBNVXY(usize, usize),
    SHL(usize),
    SNEVXY(usize, usize),
    LDI(u16),
    JR(u16),
    RND(usize, u8),
    DRAW(usize, usize, u8),
    SKP(usize),
    SKNP(usize),
    LDVXDT(usize),
    LDVXK(usize),
    LDDTVX(usize),
    LDSTVX(usize),
    ADDIVX(usize),
    LDFVX(usize),
    LDB(usize),
    LDIVX(usize),
    LDVXI(usize)
}

pub struct DecodeError {
    opcode: u16
}

impl<'a> Chip8<'a> {
    pub fn new() -> Self {
        let mut vm = Chip8 {
            memory: [0; MEMORY_SIZE],
            stack:  [0; STACK_SIZE],
            pc:     PROGRAM_START_ADDRESS,
            sp:     0x0,
            i:      0x0,
            v:      [0; NUM_REGISTERS],
            dt:     0,
            st:     0,

            keys:   [false; NUM_KEYS],
            key_wait: Some(Box::new(||{0})),

            display_memory: [0; FRAMEBUFFER_SIZE],
            on_display_update: None,

            time: Instant::now()
        };

        vm.load_font();

        vm
    }

    pub fn update(&mut self, steps: u32) -> Result<(), DecodeError> {
        self.step(steps)?;

        Ok(())
    }

    /// Run `steps` number of instructions from memory
    pub fn step(&mut self, steps: u32) -> Result<(), DecodeError> {
        for _ in 0..steps {
            self.update_timers();

            let opcode = self.fetch();

            match self.decode(opcode) {
                Ok(instr) => self.execute(instr),
                Err(e) => return Err(e)
            }
        }

        Ok(())
    }

    fn fetch(&mut self) -> u16 {
        // fetch most significant byte and least significant byte from memory
        let msb = self.memory[(self.pc) as usize];
        let lsb = self.memory[(self.pc + 1) as usize];

        // advance the program counter.
        self.pc += 2;

        let mut opcode: u16 = 0;
        opcode |= (msb as u16) << 8;
        opcode |= lsb as u16;

        opcode
    }

    fn decode(&self, opcode: u16) -> Result<Instruction, DecodeError> {
        match opcode & 0xF000 {
            0x0000 => {
                match opcode {
                    0x00E0 => return Ok(Instruction::CLS()),
                    0x00EE => return Ok(Instruction::RET()),
                    _ => Err(DecodeError{opcode: opcode})
                }
            },
            0x1000 => return Ok(Instruction::JP(opcode & 0x0FFF)),
            0x2000 => return Ok(Instruction::CALL(opcode & 0x0FFF)),
            0x3000 => return Ok(Instruction::SEVXB(nybble(opcode, 2) as usize, (opcode & 0x00FF) as u8)),
            0x4000 => return Ok(Instruction::SNEVXB(nybble(opcode, 2) as usize, (opcode & 0x00FF) as u8)),
            0x5000 => return Ok(Instruction::SEVXY(nybble(opcode, 2) as usize, nybble(opcode, 1) as usize)),
            0x6000 => return Ok(Instruction::LDVXB(nybble(opcode, 2) as usize, (opcode & 0x00FF) as u8)),
            0x7000 => return Ok(Instruction::ADDVXB(nybble(opcode, 2) as usize, (opcode & 0x00FF) as u8)),
            0x8000 => {
                match opcode & 0x000F {
                    0x0000 => return Ok(Instruction::LDVXY(nybble(opcode, 2) as usize, nybble(opcode, 1) as usize)),
                    0x0001 => return Ok(Instruction::ORVXY(nybble(opcode, 2) as usize, nybble(opcode, 1) as usize)),
                    0x0002 => return Ok(Instruction::ANDVXY(nybble(opcode, 2) as usize, nybble(opcode, 1) as usize)),
                    0x0003 => return Ok(Instruction::XORVXY(nybble(opcode, 2) as usize, nybble(opcode, 1) as usize)),
                    0x0004 => return Ok(Instruction::ADDVXY(nybble(opcode, 2) as usize, nybble(opcode, 1) as usize)),
                    0x0005 => return Ok(Instruction::SUBVXY(nybble(opcode, 2) as usize, nybble(opcode, 1) as usize)),
                    0x0006 => return Ok(Instruction::SHR(nybble(opcode, 2) as usize)),
                    0x0007 => return Ok(Instruction::SUBNVXY(nybble(opcode, 2) as usize, nybble(opcode, 1) as usize)),
                    0x000E => return Ok(Instruction::SHL(nybble(opcode, 2) as usize)),
                    _ => Err(DecodeError{opcode: opcode})
                }
            },
            0x9000 => return Ok(Instruction::SNEVXY(nybble(opcode, 2) as usize, nybble(opcode, 1) as usize)),
            0xA000 => return Ok(Instruction::LDI(opcode & 0x0FFF)),
            0xB000 => return Ok(Instruction::JR(opcode & 0x0FFF)),
            0xC000 => return Ok(Instruction::RND(nybble(opcode, 2) as usize, (opcode & 0x00FF) as u8)),
            0xD000 => return Ok(Instruction::DRAW(nybble(opcode, 2) as usize, nybble(opcode, 1) as usize, (opcode & 0x000F) as u8)),
            0xE000 => {
                match opcode & 0x00FF {
                    0x009E => return Ok(Instruction::SKP(nybble(opcode, 2) as usize)),
                    0x00A1 => return Ok(Instruction::SKNP(nybble(opcode, 2) as usize)),
                    _ => Err(DecodeError{opcode: opcode})
                }
            },
            0xF000 => {
                match opcode & 0x00FF {
                    0x0007 => return Ok(Instruction::LDVXDT(nybble(opcode, 2) as usize)),
                    0x000A => return Ok(Instruction::LDVXK(nybble(opcode, 2) as usize)),
                    0x0015 => return Ok(Instruction::LDDTVX(nybble(opcode, 2) as usize)),
                    0x0018 => return Ok(Instruction::LDSTVX(nybble(opcode, 2) as usize)),
                    0x001E => return Ok(Instruction::ADDIVX(nybble(opcode, 2) as usize)),
                    0x0029 => return Ok(Instruction::LDFVX(nybble(opcode, 2) as usize)),
                    0x0033 => return Ok(Instruction::LDB(nybble(opcode, 2) as usize)),
                    0x0055 => return Ok(Instruction::LDIVX(nybble(opcode, 2) as usize)),
                    0x0065 => return Ok(Instruction::LDVXI(nybble(opcode, 2) as usize)),
                    _ => Err(DecodeError{opcode: opcode})
                }
            },
            _ => {
                panic!("Something went impossible");
            }
        }
    }

    fn execute(&mut self, instr: Instruction) {
        match instr {
            Instruction::CLS() => {
                self.clear_display();
            },
            Instruction::RET() => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            },
            Instruction::JP(addr) => {
                self.pc = addr;
            },
            Instruction::CALL(addr) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;

                self.pc = addr;
            },
            Instruction::SEVXB(x, b) => {
                if self.v[x] == b {
                    self.pc += 2;
                }
            },
            Instruction::SNEVXB(x, b) => {
                if self.v[x] != b {
                    self.pc += 2;
                }
            },
            Instruction::SEVXY(x, y) => {
                if self.v[x] == self.v[y] {
                    self.pc += 2;
                }
            },
            Instruction::LDVXB(x, b) => {
                self.v[x] = b;
            },
            Instruction::ADDVXB(x, b) => {
                self.v[x] = (Wrapping(self.v[x]) + Wrapping(b)).0;
            },
            Instruction::LDVXY(x, y) => {
                self.v[x] = self.v[y];
            },
            Instruction::ORVXY(x, y) => {
                self.v[x] |= self.v[y];
            },
            Instruction::ANDVXY(x, y) => {
                self.v[x] &= self.v[y];
            },
            Instruction::XORVXY(x, y) => {
                self.v[x] ^= self.v[y];
            },
            Instruction::ADDVXY(x, y) => {
                let r: u16 = (self.v[x] as u16) + (self.v[y] as u16);

                // check for overflow
                if (r & 0x0100) != 0 {
                    self.v[0xF] = 1;
                }
                else {
                    self.v[0xF] = 0;
                }

                self.v[x] = r as u8;
            },
            Instruction::SUBVXY(x, y) => {
                if self.v[x] > self.v[y] {
                    self.v[0xF] = 1;
                }
                else {
                    self.v[0xF] = 0;
                }

                let wx = Wrapping(self.v[x]);
                let wy = Wrapping(self.v[y]);
                self.v[x] = (wx - wy).0;
            },
            Instruction::SHR(x) => {
                self.v[0xF] = self.v[x] & 0x01;
                self.v[x] >>= 1;
            },
            Instruction::SUBNVXY(x, y) => {
                if self.v[y] > self.v[x] {
                    self.v[0xF] = 1;
                }
                else {
                    self.v[0xF] = 0;
                }

                self.v[x] = (Wrapping(self.v[y]) - Wrapping(self.v[x])).0;
            },
            Instruction::SHL(x) => {
                self.v[0xF] = (self.v[x] & 0x80) >> 7;
                self.v[x] <<= 1;
            },
            Instruction::SNEVXY(x, y) => {
                if self.v[x] != self.v[y] {
                    self.pc += 2;
                }
            },
            Instruction::LDI(n) => {
                self.i = n;
            },
            Instruction::JR(n) => {
                self.pc = n + (self.v[0] as u16);
            },
            Instruction::RND(x, b) => {
                self.v[x] = b & rand::random::<u8>();
            },
            Instruction::DRAW(x, y, n) => {
                self.draw(x, y, n as usize);
            },
            Instruction::SKP(x) => {
                let k = self.v[x];
                if self.keys[k as usize] {
                    self.pc += 2;
                }
            },
            Instruction::SKNP(x) => {
                let k = self.v[x];
                if !self.keys[k as usize] {
                    self.pc += 2;
                }
            },
            Instruction::LDVXDT(x) => {
                self.v[x] = self.dt;
            },
            Instruction::LDVXK(x) => {
                match self.key_wait {
                    Some(ref mut key_wait) => {
                        self.v[x] = key_wait();
                    },
                    None => {
                        panic!("Keypad instruction used without the key_wait being set");
                    }
                }
            },
            Instruction::LDDTVX(x) => {
                self.dt = self.v[x];
            },
            Instruction::LDSTVX(x) => {
                self.st = self.v[x];
            },
            Instruction::ADDIVX(x) => {
                self.i = (Wrapping(self.i) + Wrapping(self.v[x] as u16)).0;
            },
            Instruction::LDFVX(x) => {
                self.i = (self.v[x] as u16 * 5) as u16;
            },
            Instruction::LDB(x) => {
                let (h, t, o) = bcd(self.v[x]);
                self.memory[self.i as usize]       = h;
                self.memory[(self.i + 1) as usize] = t;
                self.memory[(self.i + 2) as usize] = o;
            },
            Instruction::LDIVX(x) => {
                for i in 0..x {
                    let addr = (self.i + i as u16) as usize;
                    self.memory[addr] = self.v[i];
                }
            },
            Instruction::LDVXI(x) => {
                for i in 0..x {
                    let addr = (self.i + i as u16) as usize;
                    self.v[i] = self.memory[addr];
                }
            }
        }
    }

    pub fn load_memory(&mut self, memory: Vec<u8>) {
        if memory.len() > MEMORY_SIZE - (PROGRAM_START_ADDRESS as usize) {
            panic!("provided memory will not fit in vm");
        }

        let program_start_offset = PROGRAM_START_ADDRESS as usize;

        for (i, byte) in memory.iter().enumerate() {
            self.memory[program_start_offset + i] = *byte;
        }
    }

    pub fn key(&mut self, k: u8, val: bool) {
        self.keys[k as usize] = val;
    }

    pub fn set_key_wait(&mut self, key_wait: Box<FnMut() -> u8 + 'a>) {
        self.key_wait = Some(key_wait);
    }

    pub fn set_on_display_update(&mut self, on_display_update: Box<FnMut() + 'a>) {
        self.on_display_update = Some(on_display_update);
    }

    pub fn get_register(&self, x: usize) -> u8 {
        self.v[x]
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn get_sp(&self) -> u8 {
        self.sp
    }

    pub fn get_stack(&self, i: usize) -> u16 {
        self.stack[i]
    }

    pub fn get_i(&self) -> u16 {
        self.i
    }

    pub fn get_dt(&self) -> u8 {
        self.dt
    }

    pub fn get_st(&self) -> u8 {
        self.st
    }

    pub fn get_display_memory(&self) -> &[u8] {
        &self.display_memory
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> (u8, u8, u8) {
        let index = self.pixel_index(x, y);

        let r = self.display_memory[index + 0];
        let g = self.display_memory[index + 1];
        let b = self.display_memory[index + 2];

        (r, g, b)
    }

    fn draw(&mut self, x: usize, y: usize, n: usize) {
        let start_address = self.i as usize;

        let x = self.v[x] as usize;
        let y = self.v[y] as usize;

        // for bytes in sprite
        for i in 0..n {
            let byte = self.memory[start_address + i];

            let pixel_y = (y + i) % DISPLAY_HEIGHT;
            // pixels on/off state is encoded in the bits
            for (c, bit) in (0..8).rev().enumerate() {
                let pixel_x = (x + c) % DISPLAY_WIDTH;
                // state of current pixel
                let state = byte & (1 << bit) != 0;
                let prev_state = self.is_pixel_set(pixel_x, pixel_y);

                let current_state = state ^ prev_state;
                self.v[0xF] = current_state as u8;

                self.set_pixel(pixel_x, pixel_y, current_state);
            }
        }

        match self.on_display_update {
            Some(ref mut on_display_update) => on_display_update(),
            None => {}
        }
    }

    fn clear_display(&mut self) {
        for x in 0..DISPLAY_WIDTH {
            for y in 0..DISPLAY_HEIGHT {
                self.set_pixel(x, y, false);
            }
        }

        match self.on_display_update {
            Some(ref mut on_display_update) => on_display_update(),
            None => {}
        }
    }

    fn set_pixel(&mut self, x: usize, y: usize, is_on: bool) {
        let index = self.pixel_index(x, y);

        let value = if is_on { 255u8 } else { 0u8 };

        // write value into framebuffer
        self.display_memory[index + 0] = value;
        self.display_memory[index + 1] = value;
        self.display_memory[index + 2] = value;
    }

    fn is_pixel_set(&self, x: usize, y: usize) -> bool {
        let index = self.pixel_index(x, y);

        // check if the specified pixel is on
        self.display_memory[index] == 255
    }

    fn pixel_index(&self, x: usize, y: usize) -> usize {
        (y * (DISPLAY_WIDTH * 3)) + (x * 3)
    }

    fn update_timers(&mut self) {
        // Timers are updated at 60Hz
        let now = Instant::now();
        let elapsed = now.duration_since(self.time);
        let elapsed = elapsed.subsec_nanos() as f64 * 1e-9;

        if elapsed >= (1.0/60.0) {
            self.update_delay_timer();
            self.time = now;
        }
    }

    fn update_delay_timer(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
    }

    fn load_font(&mut self) {
        let fonts: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0,
            0x20, 0x60, 0x20, 0x20, 0x70,
            0xF0, 0x10, 0xF0, 0x80, 0xF0,
            0xF0, 0x10, 0xF0, 0x10, 0xF0,
            0x90, 0x90, 0xF0, 0x10, 0x10,
            0xF0, 0x80, 0xF0, 0x10, 0xF0,
            0xF0, 0x80, 0xF0, 0x90, 0xF0,
            0xF0, 0x10, 0x20, 0x40, 0x40,
            0xF0, 0x90, 0xF0, 0x90, 0xF0,
            0xF0, 0x90, 0xF0, 0x10, 0xF0,
            0xF0, 0x90, 0xF0, 0x90, 0x90,
            0xE0, 0x90, 0xE0, 0x90, 0xE0,
            0xF0, 0x80, 0x80, 0x80, 0xF0,
            0xE0, 0x90, 0x90, 0x90, 0xE0,
            0xF0, 0x80, 0xF0, 0x80, 0xF0,
            0xF0, 0x80, 0xF0, 0x80, 0x80
        ];

        for (i, item) in fonts.iter().enumerate() {
            self.memory[i] = *item;
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Debug for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to decode opcode: {:X}", self.opcode)
    }
}

/// Get the nybble `n` from `value`
fn nybble(value: u16, n: u8) -> u8 {
    let shift = 4 * n;
    let mask: u16 = 0x0F << shift;
    ((value & mask) >> shift) as u8
}

fn bcd(value: u8) -> (u8, u8, u8) {
    let mut dec = value;
    let o = dec % 10;
    dec /= 10;
    let t = dec % 10;
    dec /= 10;
    let h = dec % 10;

    (h, t, o)
}


pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bcd() {
        let (h, t, o) = bcd(255);

        assert_eq!(h, 2);
        assert_eq!(t, 5);
        assert_eq!(o, 5);
    }

    #[test]
    fn test_nybble() {
        let value = 0xDEADu16;

        assert_eq!(nybble(value, 3), 0xD);
        assert_eq!(nybble(value, 2), 0xE);
        assert_eq!(nybble(value, 1), 0xA);
        assert_eq!(nybble(value, 0), 0xD);
    }
}
