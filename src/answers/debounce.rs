#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_futures::select::select;
use embassy_stm32::{
    exti::ExtiInput,
    gpio::{Level, Output, Pull, Speed},
};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello world");

    // button labeled "key"
    // pressed down -> high; released -> low
    let mut button = ExtiInput::new(p.PA0, p.EXTI0, Pull::Up);

    // blue led
    // high -> led off; low -> led on
    let mut led = Output::new(p.PC13, Level::High, Speed::Low);

    loop {
        // debouncing button released -> pressed (led off -> on)
        loop {
            button.wait_for_low().await;

            let wait_for_release_fut = button.wait_for_high();
            let timeout_fut = Timer::after_millis(1000);
            let select_result = select(wait_for_release_fut, timeout_fut).await;

            if select_result.is_first() {
                warn!("button released before 1s");
            } else {
                info!("button stayed pressed for 1s");
                led.set_low();
                break;
            }
        }

        // debouncing button pressed -> released (led on -> off)
        loop {
            button.wait_for_high().await;

            let wait_for_press_fut = button.wait_for_low();
            let timeout_fut = Timer::after_millis(1000);
            let select_result = select(wait_for_press_fut, timeout_fut).await;

            if select_result.is_first() {
                warn!("button pressed before 1s");
            } else {
                info!("button stayed released for 1s");
                led.set_high();
                break;
            }
        }
    }
}
