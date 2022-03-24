use std::env;

use powerpc_fpscr::fpscr::{Fpscr, FPSCR_NBITS};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut value = args[1].as_str();
    if value.starts_with("0x") {
        value = value.strip_prefix("0x").unwrap();
    }
    let flags = Fpscr::new(u64::from_str_radix(value, 16).unwrap());

    for bit_number in 0..FPSCR_NBITS {
        if flags.is_set(bit_number as u8) {
            println!("{}: Set", flags.get_name(bit_number as u8));
        }
    }
}
