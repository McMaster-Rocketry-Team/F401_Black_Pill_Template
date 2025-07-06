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
    // pressed down -> low; released -> high
    let mut button = ExtiInput::new(p.PA0, p.EXTI0, Pull::Up);
    
    // blue led
    // high -> led off; low -> led on
    let mut led = Output::new(p.PC13, Level::High, Speed::Low);

    loop {
        button.wait_for_high().await; // wait for button to be released

        let blink_led_fut = blink_led(&mut led);
        let wait_button_pressed_fut = button.wait_for_low(); // wait for the button to be pressed

        select(blink_led_fut, wait_button_pressed_fut).await;
        led.set_high(); // turns off led
    }
}

async fn blink_led<'a>(led: &mut Output<'a>) {
    led.set_low(); // turns on led
    loop {
        Timer::after_millis(1000).await;
        led.toggle();
    }
}
