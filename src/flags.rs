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
}