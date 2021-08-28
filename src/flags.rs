/// This struct contains the CPU condition bits.
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

    /// Converts condition bits to a byte.
    pub fn as_byte(&self) -> u8 {
        let s = if self.s { 1 << 7 } else { 0 };
        let z = if self.z { 1 << 6 } else { 0 };
        let a = if self.a { 1 << 4 } else { 0 };
        let p = if self.p { 1 << 2 } else { 0 };
        let c = if self.c { 1 } else { 0  };
        s | z | a | p | c | 2
    }

    /// Retrieves condition bits from a byte.
    pub fn from_byte(&mut self, bflags: u8) {
        self.s = (bflags & 0x80) != 0;
        self.z = (bflags & 0x40) != 0;
        self.a = (bflags & 0x10) != 0;
        self.p = (bflags & 0x04) != 0;
        self.c = (bflags & 0x01) != 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn flags_from_byte() {
        let mut f = Flags::new();
        f.from_byte(0xC3);
        assert_eq!(f.s, true);
        assert_eq!(f.z, true);
        assert_eq!(f.c, true);
        assert_eq!(f.a, false);
        assert_eq!(f.p, false);
    }

    #[test]
    fn flags_as_byte() {
        let mut f = Flags::new();
        f.from_byte(0xC3);
        f.s = true;
        f.z = true;
        f.c = true;
        f.a = false;
        f.p = false;
        assert_eq!(f.as_byte(), 0xC3);
    }
}