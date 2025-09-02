use embedded_hal::blocking::i2c;

const SENSOR_ADDR: u8 = 0x48; // Address for TMP102

/// A generic driver for a TMP102 temperature sensor.
pub struct Tmp102<I2C> {
    i2c: I2C,
}

impl<I2C, E> Tmp102<I2C>
where
    I2C: i2c::WriteRead<Error = E>,
{
    /// Creates a new driver.
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    /// Reads the temperature in Celsius.
    pub fn read_temperature(&mut self) -> Result<f32, E> {
        let mut buffer = [0u8; 2];
        // The TMP102 gives us the temperature when we read from it.
        self.i2c.write_read(SENSOR_ADDR, &[], &mut buffer)?;

        // Convert the two bytes to a temperature value.
        let temp_raw = ((buffer[0] as i16) << 4) | (buffer[1] >> 4);
        
        // Each LSB is 0.0625 degrees Celsius.
        Ok(temp_raw as f32 * 0.0625)
    }
}
