/// Representation of Chip8 Virtual Machine
///
pub struct VirtualMachine {
    memory: [u8; 4096], ///
    stack:  [u16; 16],
    pc:     u16,
    sp:     u8,
    vx:     [u8; 16]
}

enum Instruction {
    I1,
    I2
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            memory: [0; 4096],
            stack:  [0; 16],
            pc:     0x200,
            sp:     0xF,
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
        0
    }

    fn decode(&self, opcode: u16) -> Instruction {
        Instruction::I1
    }

    fn execute(&mut self, instr: Instruction) {
        match instr {
            _ => println!("matched"),
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
