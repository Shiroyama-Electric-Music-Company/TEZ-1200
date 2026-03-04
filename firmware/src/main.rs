#![no_std]
#![no_main]

use esp_hal::peripherals::Peripherals;
use log::info;

#[esp_rtos::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let _peripherals = Peripherals::take();
    esp_println::logger::init_logger_from_env();

    info!("TEZ-1200 firmware starting");

    loop {
        embassy_time::Timer::after(embassy_time::Duration::from_secs(1)).await;
    }
}
