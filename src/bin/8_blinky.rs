//! Example on using HAL and blinking a LED.
//! The LED is a resource.
//!

#![no_main]
#![no_std]

use embedded_hal::digital::v2::OutputPin as _;
use rtic::{app, cyccnt::U32Ext};
use stm32_rtic_defmt as _; // global logger + panicking-behavior + memory layout
use stm32f1xx_hal::{
    gpio::{gpioc::PC13, GpioExt, Output, PushPull},
    rcc::RccExt,
};

#[app(device = stm32f1xx_hal::pac, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        led: PC13<Output<PushPull>>,
    }

    #[init(spawn = [blinky])]
    fn init(cx: init::Context) -> init::LateResources {
        // When using schedule and a monotonic TIM, remember to start the TIM!

        // This is the `cortex_m::Peripherals` struct without the SysTick which RTIC has taken ownership of.
        let mut cp = cx.core;

        // Initialize (enable) the monotonic TIM (CYCCNT)
        cp.DCB.enable_trace();
        cp.DWT.enable_cycle_counter();

        defmt::info!("Hello from init!");

        // Set up a LED
        let periph = stm32f1xx_hal::pac::Peripherals::take().unwrap();
        let mut rcc = periph.RCC.constrain();
        let mut gpioc = periph.GPIOC.split(&mut rcc.apb2);
        let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

        let _ = led.set_high();

        // Start the blinky task!
        cx.spawn.blinky().ok();

        init::LateResources {
            // Move the LED to the resources.
            led,
        }
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        defmt::info!("Hello from idle!");

        loop {
            continue;
        }
    }

    #[task(schedule = [blinky], resources = [led])]
    fn blinky(cx: blinky::Context) {
        // RTIC's safe static muts!
        static mut FLAG: bool = false;

        // Extract the LED
        let led = cx.resources.led;

        if !(*FLAG) {
            let _ = led.set_low();
            defmt::info!("LED Off");
        } else {
            let _ = led.set_high();
            defmt::info!("LED On");
        }

        cx.schedule.blinky(cx.scheduled + 32_000_000.cycles()).ok();

        *FLAG = !*FLAG;
    }

    // Here we list unused interrupt vectors that can be used to dispatch software tasks
    //
    // One needs one free interrupt per priority level used in software tasks.
    extern "C" {
        fn TIM2();
        fn TIM3();
    }
};
