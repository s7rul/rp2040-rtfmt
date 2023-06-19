#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use rp2040_rtfmt as _;

#[rtic::app(
    device = rp_pico::hal::pac,
    dispatchers = [TIMER_IRQ_1]
)]
mod app {
    use rp_pico::hal::{
        clocks, gpio,
        gpio::pin::bank0::{Gpio25},
        gpio::pin::PushPullOutput,
        sio::Sio,
        watchdog::Watchdog,
    };
    use rp_pico::XOSC_CRYSTAL_FREQ;

    use embedded_hal::digital::v2::{OutputPin, ToggleableOutputPin};
    use rtic_monotonics::rp2040::*;


    use panic_probe as _;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: gpio::Pin<Gpio25, PushPullOutput>,
    }

    #[init(local=[
        // Task local initialized resources are static
        // Here we use MaybeUninit to allow for initialization in init()
        // This enables its usage in driver initialization
    ])]
    fn init(mut ctx: init::Context) -> (Shared, Local) {
        defmt::info!("init");
        // Initialize the interrupt for the RP2040 timer and obtain the token
        // proving that we have.
        let rp2040_timer_token = rtic_monotonics::create_rp2040_monotonic_token!();
        // Configure the clocks, watchdog - The default is to generate a 125 MHz system clock
        Timer::start(ctx.device.TIMER, &mut ctx.device.RESETS, rp2040_timer_token); // default rp2040 clock-rate is 125MHz
        let mut watchdog = Watchdog::new(ctx.device.WATCHDOG);
        let clocks = clocks::init_clocks_and_plls(
            XOSC_CRYSTAL_FREQ,
            ctx.device.XOSC,
            ctx.device.CLOCKS,
            ctx.device.PLL_SYS,
            ctx.device.PLL_USB,
            &mut ctx.device.RESETS,
            &mut watchdog,
        )
        .ok()
        .unwrap();

        // Init LED pin
        let sio = Sio::new(ctx.device.SIO);
        let gpioa = rp_pico::Pins::new(
            ctx.device.IO_BANK0,
            ctx.device.PADS_BANK0,
            sio.gpio_bank0,
            &mut ctx.device.RESETS,
        );
        let mut led = gpioa.led.into_push_pull_output();
        led.set_low().unwrap();

        // Init I2C pins

        // Spawn heartbeat task
        heartbeat::spawn().ok();

        // Return resources and timer
        (Shared {}, Local { led})
    }

    #[task(local = [led])]
    async fn heartbeat(ctx: heartbeat::Context) {
        // Loop forever.
        //
        // It is important to remember that tasks that loop
        // forever should have an `await` somewhere in that loop.
        //
        // Without the await, the task will never yield back to
        // the async executor, which means that no other lower or
        // equal  priority task will be able to run.
        loop {
            // Flicker the built-in LED
            _ = ctx.local.led.toggle();

            // Congrats, you can use your i2c and have access to it here,
            // now to do something with it!

            // Delay for 1 second
            Timer::delay(500.millis()).await;
        }
    }
}