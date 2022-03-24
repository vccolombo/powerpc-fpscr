pub mod fpscr {
    pub const FPSCR_NBITS: i8 = 64;

    pub struct FpscrBit {
        name: String,
        reserved: bool,
    }

    impl FpscrBit {
        pub const fn reserved() -> Self {
            FpscrBit {
                name: String::new(),
                reserved: true,
            }
        }

        pub fn new(name: &str) -> Self {
            FpscrBit {
                name: name.to_string(),
                reserved: false,
            }
        }
    }

    pub struct Fpscr {
        value: u64,
        bits: [FpscrBit; 64],
    }

    impl Fpscr {
        /// # Examples
        ///
        /// ```
        /// let flags = Fpscr::new(0x00060000); // FI and FR are set
        /// ```
        pub fn new(value: u64) -> Self {
            const RESERVED_FPSCR_BIT: FpscrBit = FpscrBit::reserved();
            let mut bits: [FpscrBit; 64] = [RESERVED_FPSCR_BIT; 64];

            // TODO: this should be like a const independent of the object
            bits[32] = FpscrBit::new("FX");
            bits[33] = FpscrBit::new("FEX");
            bits[34] = FpscrBit::new("VX");
            bits[35] = FpscrBit::new("OX");
            bits[36] = FpscrBit::new("UX");
            bits[37] = FpscrBit::new("ZX");
            bits[38] = FpscrBit::new("XX");
            bits[39] = FpscrBit::new("VXSNAN");
            bits[40] = FpscrBit::new("VXSISI");
            bits[41] = FpscrBit::new("VXIDI");
            bits[42] = FpscrBit::new("VXZDZ");
            bits[43] = FpscrBit::new("VXIMZ");
            bits[44] = FpscrBit::new("VXVC");
            bits[45] = FpscrBit::new("FR");
            bits[46] = FpscrBit::new("FI");
            bits[47] = FpscrBit::new("FPRF:C");
            bits[48] = FpscrBit::new("FPRF:FPCC:FL");
            bits[49] = FpscrBit::new("FPRF:FPCC:FG");
            bits[50] = FpscrBit::new("FPRF:FPCC:FE");
            bits[51] = FpscrBit::new("FPRF:FPCC:FU");
            bits[53] = FpscrBit::new("VXSOFT");
            bits[54] = FpscrBit::new("VXSQRT");
            bits[55] = FpscrBit::new("VXCVI");
            bits[56] = FpscrBit::new("VE");
            bits[57] = FpscrBit::new("OE");
            bits[58] = FpscrBit::new("UE");
            bits[59] = FpscrBit::new("ZE");
            bits[60] = FpscrBit::new("XE");
            bits[61] = FpscrBit::new("NI");
            bits[62] = FpscrBit::new("RN0");
            bits[63] = FpscrBit::new("RN1");

            Fpscr { value, bits }
        }

        /// # Examples
        ///
        /// ```
        /// let flags = Fpscr::new(0x00020000); // FI is set
        /// assert_eq!(flags.is_set(46), true); // FI
        /// assert_eq!(flags.is_set(32), false); // FX
        /// ```
        pub fn is_set(&self, bit: u8) -> bool {
            return (self.value & (1 << (63 - bit))) != 0;
        }

        /// # Examples
        ///
        /// ```
        /// let flags = Fpscr::new(0x00020000);
        /// assert_eq!(flags.get_name(46), "FI");
        /// assert_eq!(flags.get_name(32), "FX");
        /// ```
        pub fn get_name(&self, bit: u8) -> String {
            return self.bits[bit as usize].name.clone();
        }
    }
}
