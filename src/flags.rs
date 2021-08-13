pub struct Flags {
    pub sign: bool,                 // bit 7
    pub zero: bool,                 // bit 6
    pub auxiliary_carry: bool,      // bit 4
    pub parity: bool,               // bit 2
    pub carry: bool                 // bit 0
}

impl Flags {
    pub fn new() -> Flags {
        Flags {
            sign: false,
            zero: false,
            auxiliary_carry: false,
            parity: false,
            carry: false
        }
    }
}