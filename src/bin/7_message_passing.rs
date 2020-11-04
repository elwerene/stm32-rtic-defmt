//! Example of message passing, two tasks with different
//! message passing queue sizes.

#![no_main]
#![no_std]

use my_app as _; // global logger + panicking-behavior + memory layout
use rtic::app;

#[app(device = stm32f1xx_hal::pac)]
const APP: () = {
    #[init(spawn = [printer1, printer2])]
    fn init(cx: init::Context) {
        // Print the value via message passing!
        cx.spawn.printer1(42).ok();

        // This will fail as printer1 has default capacity of 1!
        if cx.spawn.printer1(43).is_err() {
            defmt::info!("Second spawn failed!");
        }

        // Print to the printer that can take multiple!
        cx.spawn.printer2(1).ok();
        cx.spawn.printer2(2).ok();
        cx.spawn.printer2(3).ok();
        cx.spawn.printer2(4).ok();

        defmt::info!("Hello from init!");
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        defmt::info!("Hello from idle!");

        loop {
            continue;
        }
    }

    // By adding an input parameter to the task we enable message passing!
    // Note that there is no `capacity` defined, so it will default to 1.
    #[task]
    fn printer1(_cx: printer1::Context, val: u32) {
        defmt::info!("Printer 1 says: {:?}", val);
    }

    // With capacity we can take multiple messages!
    #[task(capacity = 4)]
    fn printer2(_cx: printer2::Context, val: u32) {
        defmt::info!("Printer 2 says: {:?}", val);
    }

    // Here we list unused interrupt vectors that can be used to dispatch software tasks
    //
    // One needs one free interrupt per priority level used in software tasks.
    extern "C" {
        fn TIM2();
        fn TIM3();
    }
};
