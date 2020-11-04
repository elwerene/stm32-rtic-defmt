//! Examples of Init, Idle and Software Tasks.

#![no_main]
#![no_std]

use rtic::app;
use stm32_rtic_defmt as _; // global logger + panicking-behavior + memory layout

#[app(device = stm32f1xx_hal::pac)]
const APP: () = {
    struct Resources {
        // Resources go here!
    }

    #[init]
    fn init(_cx: init::Context) {
        // Initialization code goes here
        //
        // ...

        defmt::info!("Hello from init!");
    }

    // Optional idle task, if left out idle will be a WFI.
    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        defmt::info!("Hello from idle!");

        loop {
            // Do some work or WFI.
            continue;
        }
    }

    #[task]
    fn my_first_software_task(_cx: my_first_software_task::Context) {
        // A software task, i.e. it is NOT bound to any specific interrupt vector.
    }

    #[task(binds = USART1)]
    fn my_first_hardware_task(_cx: my_first_hardware_task::Context) {
        // A hardware task, i.e. it IS bound to a specific interrupt vector.
        // In this case it is bound to the interrupt of USART1.
        //
        // Note that this does NOT set USART1 up, this needs to be done in init.
    }

    // Here we list unused interrupt vectors that can be used to dispatch software tasks
    //
    // One needs one free interrupt per priority level used in software tasks.
    extern "C" {
        fn TIM2();
    }
};
