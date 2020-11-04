#![no_std]
#![no_main]

use cortex_m_rt::entry;
use stm32-rtic-defmt as _; // memory layout + panic handler

#[entry]
fn main() -> ! {
    assert!(false, "TODO: Write actual tests");

    stm32-rtic-defmt::exit();
}
