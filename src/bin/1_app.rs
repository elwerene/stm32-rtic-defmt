//! Example for the app macro.

#![no_main]
#![no_std]

use rtic::app;
use stm32_rtic_defmt as _; // global logger + panicking-behavior + memory layout

#[app(device = stm32f1xx_hal::pac)]
const APP: () = {
    // RTIC app is written in here!
};
