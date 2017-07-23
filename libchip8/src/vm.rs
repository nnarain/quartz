extern crate rand;
use std::fmt;
use std::error::Error;

const MEMORY_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const NUM_REGISTERS: usize = 16;

const PROGRAM_START_ADDRESS: u16 = 0x200;

/// Representation of Chip8 Virtual Machine
pub struct VirtualMachine {
    memory: [u8; MEMORY_SIZE],     // 4096 bytes of memory
    stack:  [u16; STACK_SIZE],     // 16 bytes of stack
    pc:     u16,                   // program counter
    sp:     u8,                    // stack pointer
    i:      u16,                   // index register
    v:      [u8; NUM_REGISTERS],   // general purpose registers
    dt:     u8,                    // delay timer
    st:     u8                     // sound timer
}

/// Chip8 instructions
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

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        let mut vm = VirtualMachine {
            memory: [0; MEMORY_SIZE],
            stack:  [0; STACK_SIZE],
            pc:     PROGRAM_START_ADDRESS,
            sp:     0x0,
            i:      0x0,
            v:      [0; NUM_REGISTERS],
            dt:     0,
            st:     0
        };

        vm.load_font();

        vm
    }

    /// Run `steps` number of instructions from memory
    pub fn step(&mut self, steps: u32) -> Result<(), DecodeError> {
        for _ in 0..steps {
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
            Instruction::CLS() => {},
            Instruction::RET() => {
                self.pc = self.stack[self.sp as usize];
                self.sp -= 1;
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
                self.v[x] += b;
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
                let r: u16 = (self.v[x] + self.v[y]) as u16;

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

                self.v[x] -= self.v[y];
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

                self.v[x] = self.v[y] - self.v[x];
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

            },
            Instruction::SKP(x) => {

            },
            Instruction::SKNP(x) => {

            },
            Instruction::LDVXDT(x) => {

            },
            Instruction::LDVXK(x) => {

            },
            Instruction::LDDTVX(x) => {
                self.dt = self.v[x];
            },
            Instruction::LDSTVX(x) => {
                self.st = self.v[x];
            },
            Instruction::ADDIVX(x) => {
                self.i = self.i + (self.v[x] as u16);
            },
            Instruction::LDFVX(x) => {

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

impl fmt::Debug for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to decode opcode: {}", self.opcode)
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
    let h = dec % 10;
    dec /= 10;
    let t = dec % 10;
    dec /= 10;
    let o = dec % 10;

    (h, t, o)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(vm: &mut VirtualMachine, memory: Vec<u8>, should_panic: bool) {
        vm.load_memory(memory);

        loop {
            match vm.step(1) {
                Ok(_) => continue,
                Err(e) => {
                    if should_panic {
                        panic!("{:?}", e)
                    }
                    else {
                        break;
                    }
                }
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_invalid_opcode() {
        let mut vm = VirtualMachine::new();

        let program = vec![
            0xFF,
            0xFF
        ];

        run(&mut vm, program, true);
    }

    #[test]
    fn test_jump() {
        let mut vm = VirtualMachine::new();

        let program = vec![
            0x14, 0x50,
            0xFF,
            0xFF  // stop
        ];

        run(&mut vm, program, false);

        assert_eq!(vm.get_pc(), 0x452u16);
    }

    #[test]
    fn test_call() {
        let mut vm = VirtualMachine::new();

        let program = vec![
            0x22, 0x50,
            0xFF,
            0xFF  // stop
        ];

        run(&mut vm, program, false);

        assert_eq!(vm.get_pc(), 0x252u16);
        assert_eq!(vm.get_sp(), 1);
        assert_eq!(vm.get_stack(0), 0x0202);
    }
}
