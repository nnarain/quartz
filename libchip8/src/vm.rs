use std::ops;

/// Representation of Chip8 Virtual Machine
pub struct VirtualMachine {
    memory: [u8; 4096], // 4096 bytes of memory
    stack:  [u16; 16],  // 16 bytes of stack
    pc:     u16,        // program counter
    sp:     u8,         // stack pointer
    I:      u16,        // index register
    vx:     [u8; 16]    // general purpose registers
}

/// Chip8 instructions
enum Instruction {
    SYS(),
    CLS(),
    RET(),
    JP(u16),
    CALL(u16),
    SEVXB(u8, u8),
    SNEVXB(u8, u8),
    SEVXY(u8, u8),
    LDVXB(u8, u8),
    ADDVXB(u8, u8),
    LDVXY(u8, u8),
    ORVXY(u8, u8),
    ANDVXY(u8, u8),
    XORVXY(u8, u8),
    ADDVXY(u8, u8),
    SUBVXY(u8, u8),
    SHRVXY(u8, u8),
    SUBNVXY(u8, u8),
    SHLVXY(u8, u8),
    SNEVXY(u8, u8),
    LDI(u16),
    JR(u16),
    RND(u8, u8),
    DRAW(u8, u8, u8),
    SKP(u8),
    SKNP(u8),
    LDVXDT(u8),
    LDVXK(u8),
    LDDTVX(u8),
    LDSTVS(u8),
    ADDIVX(u8),
    LDFVX(u8),
    LDB(u8),
    LDIVX(u8),
    LDVXI(u8)
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            memory: [0; 4096],
            stack:  [0; 16],
            pc:     0x200,
            sp:     0xF,
            I:      0x0,
            vx:     [0; 16]
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
            0x3000 => return Instruction::SEVXB(nybble(opcode, 2), (opcode & 0x00FF) as u8),
            0x4000 => return Instruction::SNEVXB(nybble(opcode, 2), (opcode & 0x00FF) as u8),
            0x5000 => return Instruction::SEVXY(nybble(opcode, 2), nybble(opcode, 1)),
            0x6000 => return Instruction::LDVXB(nybble(opcode, 2), (opcode & 0x00FF) as u8),
            0x7000 => return Instruction::ADDVXB(nybble(opcode, 2), (opcode & 0x00FF) as u8),
            0x8000 => {
                match opcode & 0x000F {
                    0x0000 => return Instruction::LDVXY(nybble(opcode, 2), nybble(opcode, 1)),
                    0x0001 => return Instruction::ORVXY(nybble(opcode, 2), nybble(opcode, 1)),
                    0x0002 => return Instruction::ANDVXY(nybble(opcode, 2), nybble(opcode, 1)),
                    0x0003 => return Instruction::XORVXY(nybble(opcode, 2), nybble(opcode, 1)),
                    0x0004 => return Instruction::ADDVXY(nybble(opcode, 2), nybble(opcode, 1)),
                    0x0005 => return Instruction::SUBVXY(nybble(opcode, 2), nybble(opcode, 1)),
                    0x0006 => return Instruction::SHRVXY(nybble(opcode, 2), nybble(opcode, 1)),
                    0x0007 => return Instruction::SUBNVXY(nybble(opcode, 2), nybble(opcode, 1)),
                    0x000E => return Instruction::SHLVXY(nybble(opcode, 2), nybble(opcode, 1)),
                    _ => panic!("Invalid Opcode!!!")
                }
            },
            0x9000 => return Instruction::SNEVXY(nybble(opcode, 2), nybble(opcode, 1)),
            0xA000 => return Instruction::LDI(opcode & 0x0FFF),
            0xB000 => return Instruction::JR(opcode & 0x0FFF),
            0xC000 => return Instruction::RND(nybble(opcode, 2), (opcode & 0x00FF) as u8),
            0xD000 => return Instruction::DRAW(nybble(opcode, 2), nybble(opcode, 1), (opcode & 0x000F) as u8),
            0xE000 => {
                match opcode & 0x00FF {
                    0x009E => return Instruction::SKP(nybble(opcode, 2)),
                    0x00A1 => return Instruction::SKNP(nybble(opcode, 2)),
                    _ => panic!("Invalid Opcode")
                }
            },
            0xF000 => {
                match opcode & 0x00FF {
                    0x000A => return Instruction::LDVXK(nybble(opcode, 2)),
                    0x0015 => return Instruction::LDDTVX(nybble(opcode, 2)),
                    0x0018 => return Instruction::LDSTVS(nybble(opcode, 2)),
                    0x001E => return Instruction::ADDIVX(nybble(opcode, 2)),
                    0x0029 => return Instruction::LDFVX(nybble(opcode, 2)),
                    0x0033 => return Instruction::LDB(nybble(opcode, 2)),
                    0x0055 => return Instruction::LDIVX(nybble(opcode, 2)),
                    0x0065 => return Instruction::LDVXI(nybble(opcode, 2)),
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
            Instruction::SYS() => println!(""),
            Instruction::CLS() => println!(""),
            Instruction::RET() => println!(""),
            Instruction::JP(addr) => println!(""),
            Instruction::CALL(addr) => println!(""),
            Instruction::SEVXB(x, b) => println!(""),
            Instruction::SNEVXB(x, b) => println!(""),
            Instruction::SEVXY(x, y) => println!(""),
            Instruction::LDVXB(x, b) => println!(""),
            Instruction::ADDVXB(x, b) => println!(""),
            Instruction::LDVXY(x, y) => println!(""),
            Instruction::ORVXY(x, y) => println!(""),
            Instruction::ANDVXY(x, y) => println!(""),
            Instruction::XORVXY(x, y) => println!(""),
            Instruction::ADDVXY(x, y) => println!(""),
            Instruction::SUBVXY(x, y) => println!(""),
            Instruction::SHRVXY(x, y) => println!(""),
            Instruction::SUBNVXY(x, y) => println!(""),
            Instruction::SHLVXY(x, y) => println!(""),
            Instruction::SNEVXY(x, y) => println!(""),
            Instruction::LDI(n) => println!(""),
            Instruction::JR(n) => println!(""),
            Instruction::RND(x, b) => println!(""),
            Instruction::DRAW(x, y, n) => println!(""),
            Instruction::SKP(x) => println!(""),
            Instruction::SKNP(x) => println!(""),
            Instruction::LDVXDT(x) => println!(""),
            Instruction::LDVXK(x) => println!(""),
            Instruction::LDDTVX(x) => println!(""),
            Instruction::LDSTVS(x) => println!(""),
            Instruction::ADDIVX(x) => println!(""),
            Instruction::LDFVX(x) => println!(""),
            Instruction::LDB(x) => println!(""),
            Instruction::LDIVX(x) => println!(""),
            Instruction::LDVXI(x) => println!("")
        }
    }
}

/// Get the nybble `n` from `value`
fn nybble(value: u16, n: u8) -> u8 {
    let shift = 4 * n;
    let mask: u16 = 0x0F << shift;
    ((value & mask) >> shift) as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vm_works() {
    }
}
