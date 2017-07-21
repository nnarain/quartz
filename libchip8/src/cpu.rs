pub struct Cpu {
    pc: u16
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu{pc: 0}
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn cpu_works() {

    }
}
