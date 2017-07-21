/// Representation of Chip8 Virtual Machine
pub struct VirtualMachine {
    memory: [u8; 4096], // 4096 bytes of memory
    stack:  [u16; 16],  // 16 bytes of stack
    pc:     u16,        // program counter
    sp:     u8,         // stack pointer
    vx:     [u8; 16],   // general purpose registers
    I:      u16         // index register
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
            vx:     [0; 16],
            I:      0x0
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
        Instruction::CLS()
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vm_works() {
    }
}
