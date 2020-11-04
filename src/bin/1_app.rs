//! Example for the app macro.

#![no_main]
#![no_std]

use my_app as _; // global logger + panicking-behavior + memory layout
use rtic::app;

#[app(device = stm32f1xx_hal::pac)]
const APP: () = {
    // RTIC app is written in here!
};
