use std::env;

mod fpscr {
    pub const FPSCR_NBITS: i8 = 64;

    pub struct BitNumber(u8);

    impl BitNumber {
        pub const fn new(bit_number: u8) -> Self {
            if bit_number > 63 {
                panic!("Bit range between [0, 63]");
            }

            BitNumber { 0: bit_number }
        }
    }

    impl From<u8> for BitNumber {
        fn from(value: u8) -> Self {
            BitNumber::new(value)
        }
    }

    pub struct FpscrBit {
        name: String,
        is_set: bool,
        reserved: bool,
    }

    impl FpscrBit {
        pub const fn dummy() -> Self {
            FpscrBit {
                name: String::new(),
                is_set: false,
                reserved: true,
            }
        }

        pub fn new(name: &str, is_set: bool, reserved: bool) -> Self {
            FpscrBit {
                name: name.to_string(),
                is_set,
                reserved,
            }
        }
    }

    pub struct Fpscr {
        bits: [FpscrBit; 64],
    }

    impl Fpscr {
        pub fn new(value: u64) -> Self {
            const RESERVED_FPSCR_BIT: FpscrBit = FpscrBit::dummy();
            let mut bits: [FpscrBit; 64] = [RESERVED_FPSCR_BIT; 64];

            for bit_number in 0..32 {
                bits[bit_number] = FpscrBit::new(
                    "Reserved",
                    Fpscr::is_bit_set(value, bit_number as u8),
                    true,
                );
            }

            bits[32] =
                FpscrBit::new("FX", Fpscr::is_bit_set(value, 32 as u8), false);
            bits[33] =
                FpscrBit::new("FEX", Fpscr::is_bit_set(value, 33 as u8), false);
            bits[34] =
                FpscrBit::new("VX", Fpscr::is_bit_set(value, 34 as u8), false);
            bits[35] =
                FpscrBit::new("OX", Fpscr::is_bit_set(value, 35 as u8), false);
            bits[36] =
                FpscrBit::new("UX", Fpscr::is_bit_set(value, 36 as u8), false);
            bits[37] =
                FpscrBit::new("ZX", Fpscr::is_bit_set(value, 37 as u8), false);
            bits[38] =
                FpscrBit::new("XX", Fpscr::is_bit_set(value, 38 as u8), false);
            bits[39] = FpscrBit::new(
                "VXSNAN",
                Fpscr::is_bit_set(value, 32 as u8),
                false,
            );
            bits[40] = FpscrBit::new(
                "VXSISI",
                Fpscr::is_bit_set(value, 32 as u8),
                false,
            );
            bits[41] = FpscrBit::new(
                "VXIDI",
                Fpscr::is_bit_set(value, 41 as u8),
                false,
            );
            bits[42] = FpscrBit::new(
                "VXZDZ",
                Fpscr::is_bit_set(value, 42 as u8),
                false,
            );
            bits[43] = FpscrBit::new(
                "VXIMZ",
                Fpscr::is_bit_set(value, 43 as u8),
                false,
            );
            bits[44] = FpscrBit::new(
                "VXVC",
                Fpscr::is_bit_set(value, 44 as u8),
                false,
            );
            bits[45] =
                FpscrBit::new("FR", Fpscr::is_bit_set(value, 45 as u8), false);
            bits[46] =
                FpscrBit::new("FI", Fpscr::is_bit_set(value, 46 as u8), false);
            bits[47] = FpscrBit::new(
                "FPRF:C",
                Fpscr::is_bit_set(value, 47 as u8),
                false,
            );
            bits[48] = FpscrBit::new(
                "FPRF:FPCC:FL",
                Fpscr::is_bit_set(value, 48 as u8),
                false,
            );
            bits[49] = FpscrBit::new(
                "FPRF:FPCC:FG",
                Fpscr::is_bit_set(value, 49 as u8),
                false,
            );
            bits[50] = FpscrBit::new(
                "FPRF:FPCC:FE",
                Fpscr::is_bit_set(value, 50 as u8),
                false,
            );
            bits[51] = FpscrBit::new(
                "FPRF:FPCC:FU",
                Fpscr::is_bit_set(value, 51 as u8),
                false,
            );
            bits[52] = FpscrBit::new(
                "Reserved",
                Fpscr::is_bit_set(value, 52 as u8),
                true,
            );
            bits[53] = FpscrBit::new(
                "VXSOFT",
                Fpscr::is_bit_set(value, 53 as u8),
                false,
            );
            bits[54] = FpscrBit::new(
                "VXSQRT",
                Fpscr::is_bit_set(value, 54 as u8),
                false,
            );
            bits[55] = FpscrBit::new(
                "VXCVI",
                Fpscr::is_bit_set(value, 55 as u8),
                false,
            );
            bits[56] =
                FpscrBit::new("VE", Fpscr::is_bit_set(value, 56 as u8), false);
            bits[57] =
                FpscrBit::new("OE", Fpscr::is_bit_set(value, 57 as u8), false);
            bits[58] =
                FpscrBit::new("UE", Fpscr::is_bit_set(value, 58 as u8), false);
            bits[59] =
                FpscrBit::new("ZE", Fpscr::is_bit_set(value, 59 as u8), false);
            bits[60] =
                FpscrBit::new("XE", Fpscr::is_bit_set(value, 60 as u8), false);
            bits[61] =
                FpscrBit::new("NI", Fpscr::is_bit_set(value, 61 as u8), false);
            bits[62] =
                FpscrBit::new("RN0", Fpscr::is_bit_set(value, 62 as u8), false);
            bits[63] =
                FpscrBit::new("RN1", Fpscr::is_bit_set(value, 63 as u8), false);

            Fpscr { bits }
        }

        pub fn is_set(&self, bit: u8) -> bool {
            return self.bits[bit as usize].is_set;
        }

        pub fn get_name(&self, bit: u8) -> String {
            return self.bits[bit as usize].name.clone();
        }

        fn is_bit_set(value: u64, bit: u8) -> bool {
            return (value & (1 << (63 - bit))) != 0;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut value = args[1].as_str();
    if value.starts_with("0x") {
        value = value.strip_prefix("0x").unwrap();
    }
    let flags = fpscr::Fpscr::new(u64::from_str_radix(value, 16).unwrap());

    for bit_number in 0..fpscr::FPSCR_NBITS {
        if flags.is_set(bit_number as u8) {
            println!("{}: Set", flags.get_name(bit_number as u8));
        }
    }
}
