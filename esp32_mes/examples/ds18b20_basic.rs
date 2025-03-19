use esp32_mes::sensors::ds18b20::TemperatureProvider;
use esp_idf_hal::{delay::FreeRtos, gpio::PinDriver, prelude::Peripherals};

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let ds_pin = PinDriver::input_output_od(peripherals.pins.gpio16).unwrap();
    let one_wire_bus = one_wire_bus::OneWire::new(ds_pin).unwrap();

    let mut temp_provider = TemperatureProvider::new(one_wire_bus);
    let sensor = temp_provider
        .get_ds_sensors::<&str>()
        .unwrap()
        .into_iter()
        .next()
        .unwrap();

    loop {
        let res = temp_provider.measure_temp(&sensor);

        match res {
            Ok(temp) => {
                log::info!("Temperature: {}", temp)
            }
            Err(e) => log::error!("Error while getting temperature: {:?}", e),
        }

        FreeRtos::delay_ms(1000);
    }
}
