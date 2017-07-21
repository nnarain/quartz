
mod vm;
use vm::VirtualMachine;

pub struct Chip8 {
    vm: VirtualMachine
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            vm: VirtualMachine::new()
        }
    }

    pub fn update(&mut self, steps: u32) {
        self.vm.step(steps);
    }
}

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

    }
}
