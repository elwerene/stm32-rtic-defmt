//! Example on scheduling tasks in the future.

#![no_main]
#![no_std]

use rtic::{app, cyccnt::U32Ext};
use stm32_rtic_defmt as _; // global logger + panicking-behavior + memory layout

#[app(device = stm32f1xx_hal::pac, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    #[init(schedule = [hello_world_task])]
    fn init(cx: init::Context) {
        // This is the `cortex_m::Peripherals` struct without the SysTick which RTIC has taken ownership of.
        let mut cp = cx.core;

        // Initialize (enable) the monotonic TIM (CYCCNT)
        cp.DCB.enable_trace();
        cp.DWT.enable_cycle_counter();

        // Schedule the task 3s into the future (the MCU runs at 64 MHz).
        cx.schedule
            .hello_world_task(cx.start + 192_000_000.cycles())
            .ok();

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
    fn hello_world_task(cx: hello_world_task::Context) {
        defmt::info!(
            "Hello world from the future! (late: {:u32}",
            cx.scheduled.elapsed().as_cycles(),
        );
    }

    // Here we list unused interrupt vectors that can be used to dispatch software tasks
    //
    // One needs one free interrupt per priority level used in software tasks.
    extern "C" {
        fn TIM2();
    }
};
