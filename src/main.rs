use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::ledc::*;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::prelude::*;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");
    led().expect("error");
}

fn led() -> anyhow::Result<()>  {
    println!("Configuring output channel");

    let peripherals = Peripherals::take()?;
    let mut channel = LedcDriver::new(
        peripherals.ledc.channel0,
        LedcTimerDriver::new(
            peripherals.ledc.timer0,
            &config::TimerConfig::new().frequency(25.kHz().into()),
        )?,
        peripherals.pins.gpio2,
    )?;

    println!("Starting duty-cycle loop");

    let max_duty = channel.get_max_duty();
    for numerator in [0, 1, 2, 3, 4, 5].iter().cycle() {
        println!("Duty {numerator}/5");
        channel.set_duty(max_duty * numerator / 5)?;
        FreeRtos::delay_ms(2000);
    }

    loop {
        FreeRtos::delay_ms(1000);
    }
}