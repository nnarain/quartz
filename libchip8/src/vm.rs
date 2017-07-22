extern crate rand;

/// Representation of Chip8 Virtual Machine
pub struct VirtualMachine {
    memory: [u8; 4096], // 4096 bytes of memory
    stack:  [u16; 16],  // 16 bytes of stack
    pc:     u16,        // program counter
    sp:     u8,         // stack pointer
    i:      u16,        // index register
    v:     [u8; 16],    // general purpose registers
    dt:    u8,          // delay timer
    st:    u8           // sound timer
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

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            memory: [0; 4096],
            stack:  [0; 16],
            pc:     0x200,
            sp:     0x0,
            i:      0x0,
            v:     [0; 16],
            dt:    0,
            st:    0
        }
    }

    /// Run `steps` number of instructions from memory
    pub fn step(&mut self, steps: u32) {
        for _ in 0..steps {
            let opcode = self.fetch();
            let instr = self.decode(opcode);

            self.execute(instr);
        }
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

    fn decode(&self, opcode: u16) -> Instruction {
        match opcode & 0xF000 {
            0x0000 => {
                match opcode {
                    0x00E0 => return Instruction::CLS(),
                    0x00EE => return Instruction::RET(),
                    _ => panic!("Invalid Opcode")
                }
            },
            0x1000 => return Instruction::JP(opcode & 0x0FFF),
            0x2000 => return Instruction::CALL(opcode & 0x0FFF),
            0x3000 => return Instruction::SEVXB(nybble(opcode, 2) as usize, (opcode & 0x00FF) as u8),
            0x4000 => return Instruction::SNEVXB(nybble(opcode, 2) as usize, (opcode & 0x00FF) as u8),
            0x5000 => return Instruction::SEVXY(nybble(opcode, 2) as usize, nybble(opcode, 1) as usize),
            0x6000 => return Instruction::LDVXB(nybble(opcode, 2) as usize, (opcode & 0x00FF) as u8),
            0x7000 => return Instruction::ADDVXB(nybble(opcode, 2) as usize, (opcode & 0x00FF) as u8),
            0x8000 => {
                match opcode & 0x000F {
                    0x0000 => return Instruction::LDVXY(nybble(opcode, 2) as usize, nybble(opcode, 1) as usize),
                    0x0001 => return Instruction::ORVXY(nybble(opcode, 2) as usize, nybble(opcode, 1) as usize),
                    0x0002 => return Instruction::ANDVXY(nybble(opcode, 2) as usize, nybble(opcode, 1) as usize),
                    0x0003 => return Instruction::XORVXY(nybble(opcode, 2) as usize, nybble(opcode, 1) as usize),
                    0x0004 => return Instruction::ADDVXY(nybble(opcode, 2) as usize, nybble(opcode, 1) as usize),
                    0x0005 => return Instruction::SUBVXY(nybble(opcode, 2) as usize, nybble(opcode, 1) as usize),
                    0x0006 => return Instruction::SHR(nybble(opcode, 2) as usize),
                    0x0007 => return Instruction::SUBNVXY(nybble(opcode, 2) as usize, nybble(opcode, 1) as usize),
                    0x000E => return Instruction::SHL(nybble(opcode, 2) as usize),
                    _ => panic!("Invalid Opcode!!!")
                }
            },
            0x9000 => return Instruction::SNEVXY(nybble(opcode, 2) as usize, nybble(opcode, 1) as usize),
            0xA000 => return Instruction::LDI(opcode & 0x0FFF),
            0xB000 => return Instruction::JR(opcode & 0x0FFF),
            0xC000 => return Instruction::RND(nybble(opcode, 2) as usize, (opcode & 0x00FF) as u8),
            0xD000 => return Instruction::DRAW(nybble(opcode, 2) as usize, nybble(opcode, 1) as usize, (opcode & 0x000F) as u8),
            0xE000 => {
                match opcode & 0x00FF {
                    0x009E => return Instruction::SKP(nybble(opcode, 2) as usize),
                    0x00A1 => return Instruction::SKNP(nybble(opcode, 2) as usize),
                    _ => panic!("Invalid Opcode")
                }
            },
            0xF000 => {
                match opcode & 0x00FF {
                    0x000A => return Instruction::LDVXK(nybble(opcode, 2) as usize),
                    0x0015 => return Instruction::LDDTVX(nybble(opcode, 2) as usize),
                    0x0018 => return Instruction::LDSTVX(nybble(opcode, 2) as usize),
                    0x001E => return Instruction::ADDIVX(nybble(opcode, 2) as usize),
                    0x0029 => return Instruction::LDFVX(nybble(opcode, 2) as usize),
                    0x0033 => return Instruction::LDB(nybble(opcode, 2) as usize),
                    0x0055 => return Instruction::LDIVX(nybble(opcode, 2) as usize),
                    0x0065 => return Instruction::LDVXI(nybble(opcode, 2) as usize),
                    _ => panic!("Invalid Opcode")
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
    #[test]
    fn vm_works() {
    }
}
