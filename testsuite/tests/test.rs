#![no_std]
#![no_main]

use cortex_m_rt::entry;
use stm32_rtic_defmt as _; // memory layout + panic handler

#[entry]
fn main() -> ! {
    assert!(false, "TODO: Write actual tests");

    stm32_rtic_defmt::exit();
}
