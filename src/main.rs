//! Traffic Lights (Semaphore) with Logging
//!
//! This example simulates a traffic light system using LEDs. The LEDs connected to GPIO pins will
//! turn on and off in the sequence: red -> orange -> green. The sequence will repeat in a loop.
//! Each time the color changes, a log message with the color name will be printed.

#![no_std]
#![no_main]

use hal::{
    clock::ClockControl,
    gpio::IO,
    peripherals::Peripherals,
    prelude::*,
    timer::TimerGroup,
    Delay,
    Rtc,
};
use esp_backtrace as _;
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(
        peripherals.TIMG0,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(
        peripherals.TIMG1,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt1 = timer_group1.wdt;

    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    // Set up GPIO pins for LEDs
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut green_led = io.pins.gpio5.into_push_pull_output(); // Green LED, connected to GPIO5
    let mut orange_led = io.pins.gpio6.into_push_pull_output(); // Orange LED, connected to GPIO6
    let mut red_led = io.pins.gpio7.into_push_pull_output(); // Red LED, connected to GPIO7

    // Initialize the Delay peripheral
    let mut delay = Delay::new(&clocks);

    loop {
        // Red light (LED)
        red_led.set_high().unwrap();
        orange_led.set_low().unwrap();
        green_led.set_low().unwrap();
        log_color("Red");
        delay.delay_ms(2000u32); // Red light duration: 2 seconds

        // Orange light (LED)
        red_led.set_low().unwrap();
        orange_led.set_high().unwrap();
        log_color("Orange");
        delay.delay_ms(1000u32); // Orange light duration: 1 second

        // Green light (LED)
        orange_led.set_low().unwrap();
        green_led.set_high().unwrap();
        log_color("Green");
        delay.delay_ms(3000u32); // Green light duration: 3 seconds

        // Orange light (LED) again
        green_led.set_low().unwrap();
        orange_led.set_high().unwrap();
        log_color("Orange");
        delay.delay_ms(1000u32); // Orange light duration: 1 second
    }
}

fn log_color(color: &str) {
    // Use the appropriate logging mechanism here (e.g., println!, log::info!, etc.)
    println!("Color: {}", color);
}
