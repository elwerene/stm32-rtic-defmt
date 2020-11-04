//! Example on spawning a software/hardware task.

#![no_main]
#![no_std]

use rtic::app;
use stm32_rtic_defmt as _; // global logger + panicking-behavior + memory layout

#[app(device = stm32f1xx_hal::pac)]
const APP: () = {
    struct Resources {
        // Resources go here!
    }

    #[init(spawn = [hello_world_task])]
    fn init(cx: init::Context) {
        // Any spawn in init will run after init finishes.
        cx.spawn.hello_world_task().ok();

        defmt::info!("Hello from init!");
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        defmt::info!("Hello from idle!");

        loop {
            continue;
        }
    }

    #[task]
    fn hello_world_task(_cx: hello_world_task::Context) {
        defmt::info!("Hello world from task!");
    }

    // Here we list unused interrupt vectors that can be used to dispatch software tasks

    //
    // One needs one free interrupt per priority level used in software tasks.
    extern "C" {
        fn TIM2();
    }
};
