
mod cpu;
use cpu::Cpu;

pub struct Chip8 {
    cpu: Cpu
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8{cpu: Cpu::new()}
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
