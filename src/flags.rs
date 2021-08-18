pub struct Flags {
    pub s: bool,                // sign             : bit 7
    pub z: bool,                // zero             : bit 6
    pub a: bool,                // auxiliary carry  : bit 4
    pub p: bool,                // parity           : bit 2
    pub c: bool                 // carry            : bit 0
}

impl Flags {
    pub fn new() -> Flags {
        Flags {
            s: false,
            z: false,
            a: false,
            p: false,
            c: false
        }
    }

    pub fn as_byte(&self) -> u8 {
        let s = if self.s { 1 << 7 } else { 0  };
        let z = if self.z { 1 << 6 } else { 0  };
        let a = if self.a { 1 << 4 } else { 0  };
        let p = if self.p { 1 << 2 } else { 0  };
        let c = if self.c { 1 } else { 0  };
        s | z | a | p | c | 2
    }
}