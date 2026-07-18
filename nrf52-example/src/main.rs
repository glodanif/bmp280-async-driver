#![no_std]
#![no_main]

use bmp280_driver::{Address, Bmp280Device, FilterCoefficient, NormalConfig};
use embassy_executor::Spawner;
use embassy_nrf::twim::{Config, InterruptHandler, Twim};
use embassy_nrf::{bind_interrupts, peripherals};
use embassy_time::{Delay, Timer};
#[allow(unused)]
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let peri = embassy_nrf::init(Default::default());

    let mut rx_ram_buffer: [u8; 16] = [0u8; 16];
    let i2c = Twim::new(
        peri.TWISPI0,
        Irqs,
        peri.P0_26,
        peri.P0_27,
        Config::default(),
        &mut rx_ram_buffer,
    );

    let new_bmp280 = Bmp280Device::new(i2c, Address::Secondary, Delay);
    let mut bmp280 = new_bmp280
        .prepare()
        .await
        .expect("Failed to prepare bmp280")
        .into_normal(NormalConfig {
            filter_coefficient: FilterCoefficient::K16,
            ..Default::default()
        })
        .await
        .expect("Failed to prepare bmp280");

    Timer::after_millis(500).await;

    let chip_id = bmp280.chip_id().await;
    match chip_id {
        Ok(id) => {
            if id != bmp280_driver::EXPECTED_CHIP_ID {
                defmt::error!("Invalid chip id: {}", id);
            } else {
                defmt::info!("Chip ID: {}", id);
            }
        }
        Err(e) => {
            defmt::error!("Failed to read chip id: {}", e);
        }
    };

    loop {
        Timer::after_secs(1).await;
        let sensor_data = bmp280.read().await;
        match sensor_data {
            Ok(data) => {
                defmt::info!(
                    "BMP280 read: T {}, P: {}",
                    data.temperature_celsius(),
                    data.pressure_hpa()
                );
            }
            Err(e) => {
                defmt::error!("Failed to read BMP280: {}", e);
            }
        }
    }
}

bind_interrupts!(struct Irqs {
      TWISPI0 => InterruptHandler<peripherals::TWISPI0>;
});
