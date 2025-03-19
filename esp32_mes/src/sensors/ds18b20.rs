use ds18b20::Ds18b20;
use esp_idf_hal;
use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio::{InputOutput, InputPin, OutputPin};
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_sys::EspError;
pub use one_wire_bus::OneWire;
use one_wire_bus::OneWireResult;

pub struct TemperatureProvider<'a, T>
where
    T: InputPin + OutputPin,
{
    one_wire_bus: OneWire<PinDriver<'a, T, InputOutput>>,
}

impl<'a, T: InputPin + OutputPin> TemperatureProvider<'a, T> {
    pub fn new(one_wire_bus: OneWire<PinDriver<'a, T, InputOutput>>) -> Self {
        TemperatureProvider { one_wire_bus }
    }

    pub fn get_ds_sensors<E: std::fmt::Debug>(
        &mut self,
    ) -> OneWireResult<Vec<ds18b20::Ds18b20>, EspError> {
        let one_wire_bus = &mut self.one_wire_bus;
        let mut search_state = None;

        let mut delay = Ets {};
        let mut sensors: Vec<ds18b20::Ds18b20> = vec![];
        while let Some((device_address, state)) =
            one_wire_bus.device_search(search_state.as_ref(), false, &mut delay)?
        {
            search_state = Some(state);

            if device_address.family_code() != ds18b20::FAMILY_CODE {
                continue;
            }

            log::trace!("Found ds18b20: {:?}", device_address);
            let sensor = ds18b20::Ds18b20::new::<EspError>(device_address)?;
            sensors.push(sensor);
        }

        Ok(sensors)
    }

    pub fn measure_temp(&mut self, sensor: &Ds18b20) -> OneWireResult<f32, EspError> {
        let mut delay = Ets {};

        ds18b20::start_simultaneous_temp_measurement(&mut self.one_wire_bus, &mut delay).unwrap();
        ds18b20::Resolution::Bits12.delay_for_measurement_time(&mut delay);

        let sensor_data = sensor.read_data(&mut self.one_wire_bus, &mut delay)?;
        Ok(sensor_data.temperature)
    }
}
