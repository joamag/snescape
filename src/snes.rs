struct Snes {}

impl Snes {
    pub fn new() -> Snes {
        Snes {}
    }

    pub fn clock(&mut self) -> u16 {
        0
    }
}
