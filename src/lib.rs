//! cap1xxx [`embedded-hal`] I2C driver for Microchip cap1xxx capacitive touch buttons
//!
//! This crate is primarily targeting cap1666 6 buttons - 6 led hardware found
//! on the Pimoroni GFX HAT
#![allow(dead_code)]
use embedded_hal::blocking::i2c::{Write, WriteRead};

mod consts;
pub mod error;
use consts::*;
use error::*;
use std::cmp::{max, min};
use std::time::Duration;

pub type RWResult<T, R> = Result<R, Error<<T as WriteRead>::Error, <T as Write>::Error>>;

pub struct CAP1XXX<T>
where
    T: WriteRead + Write,
{
    i2c: T,
    number_of_leds: u8,
    i2c_address: u8,
}

impl<T> CAP1XXX<T>
where
    T: WriteRead + Write,
{
    pub fn new(i2c: T, i2c_address: u8, number_of_leds: u8) -> Self {
        Self {
            i2c,
            number_of_leds,
            i2c_address,
        }
    }

    pub fn init(&mut self) -> RWResult<T, ()> {
        //self.repeat_enabled    = 0b00000000
        //self.release_enabled   = 0b11111111

        //self.product_id = self._get_product_id()?;

        //if not self.product_id in self.supported:
        //    raise Exception("Product ID {} not supported!".format(self.product_id))

        // Enable all inputs with interrupt by default
        self.enable_inputs(0b11111111)?;
        self.enable_interrupts(0b11111111)?;

        // Disable repeat for all channels, but give
        // it sane defaults anyway
        self.enable_repeat(0b00000000)?;
        self.enable_multitouch(true)?;

        self.set_hold_delay(Duration::from_millis(210))?;
        self.set_repeat_rate(Duration::from_millis(210))?;

        // Tested sane defaults for various configurations
        self.write_byte(R_SAMPLING_CONFIG, 0b00001000)?; // 1sample per measure, 1.28ms time, 35ms cycle
        self.write_byte(R_CALIBRATION, 0b00111111)?; // recalibrate
        self.write_byte(R_SENSITIVITY, 0b01100000)?; // 2x sensitivity
        self.write_byte(R_GENERAL_CONFIG, 0b00111000)?;
        self.write_byte(R_CONFIGURATION2, 0b01100000)?;

        Ok(())
    }

    fn write_byte(
        &mut self,
        register: u8,
        value: u8,
    ) -> Result<(), WriteError<<T as Write>::Error>> {
        Ok(self.i2c.cmd_write(self.i2c_address, register, &[value])?)
    }

    fn read_byte(&mut self, register: u8) -> Result<u8, ReadError<<T as WriteRead>::Error>> {
        let mut buf = [0u8];
        self.i2c
            .write_read(self.i2c_address, &[register], &mut buf)?;
        Ok(buf[0])
    }

    fn read_block(
        &mut self,
        register: u8,
        len: usize,
    ) -> Result<Vec<u8>, ReadError<<T as WriteRead>::Error>> {
        let mut buf = vec![0u8; len];
        self.i2c
            .write_read(self.i2c_address, &[register], &mut buf)?;
        Ok(buf)
    }

    fn change_value<F>(&mut self, register: u8, op: F) -> RWResult<T, ()>
    where
        F: Fn(u8) -> u8,
    {
        let new_value = op(self.read_byte(register)?);
        Ok(self.write_byte(register, new_value)?)
    }

    fn set_bit(&mut self, register: u8, bit: u8) -> RWResult<T, ()> {
        self.change_value(register, |value| value | (1 << bit))
    }

    fn clear_bit(&mut self, register: u8, bit: u8) -> RWResult<T, ()> {
        self.change_value(register, |value| value & !(1 << bit))
    }

    fn change_bit(&mut self, register: u8, bit: u8, state: bool) -> RWResult<T, ()> {
        if state {
            self.set_bit(register, bit)
        } else {
            self.clear_bit(register, bit)
        }
    }

    fn change_bits(&mut self, register: u8, offset: u8, size: u8, bits: u8) -> RWResult<T, ()> {
        self.change_value(register, |value| {
            let mut ret = value;
            for x in 0..size {
                ret &= !(1 << (offset + x));
            }
            ret |= bits << offset;
            ret
        })
    }

    // ----------------------------------------------------------------------------
    // Buttons handling

    /// Clear the interrupt flag, bit 0, of the
    //  main control register
    pub fn clear_interrupt(&mut self) -> RWResult<T, ()> {
        self.clear_bit(R_MAIN_CONTROL, 0)
    }
    pub fn is_interrupted(&mut self) -> RWResult<T, bool> {
        Ok(self
            .read_byte(R_MAIN_CONTROL)
            .map(|value| (value & 1) > 0)?)
    }
    pub fn auto_recalibrate(&mut self, value: bool) -> RWResult<T, ()> {
        self.change_bit(R_GENERAL_CONFIG, 3, value)
    }
    pub fn filter_analog_noise(&mut self, value: bool) -> RWResult<T, ()> {
        self.change_bit(R_GENERAL_CONFIG, 4, !value)
    }
    pub fn filter_digital_noise(&mut self, value: bool) -> RWResult<T, ()> {
        self.change_bit(R_GENERAL_CONFIG, 5, !value)
    }
    /// Set time before a press and hold is detected,
    /// Clamps to multiples of 35 from 35 to 560
    pub fn set_hold_delay(&mut self, delay: Duration) -> RWResult<T, ()> {
        self.change_value(R_INPUT_CONFIG2, |v| {
            (v & !0b1111) | Self::duration_to_rate_scale(delay)
        })
    }
    /// Set repeat rate in milliseconds,
    //  Clamps to multiples of 35 from 35 to 560
    pub fn set_repeat_rate(&mut self, delay: Duration) -> RWResult<T, ()> {
        self.change_value(R_INPUT_CONFIG, |v| {
            (v & !0b1111) | Self::duration_to_rate_scale(delay)
        })
    }

    pub fn duration_to_rate_scale(duration: Duration) -> u8 {
        let ms = duration.as_millis();
        let ms = max(35, ms);
        let ms = min(560, ms);
        ((ms - ms % 35 - 35) / 35) as u8
    }
    fn get_product_id(&mut self) -> RWResult<T, u8> {
        Ok(self.read_byte(R_PRODUCT_ID)?)
    }

    /// Toggles multi-touch by toggling the multi-touch block bit in the config register
    pub fn enable_multitouch(&mut self, enable: bool) -> RWResult<T, ()> {
        self.change_value(R_MTOUCH_CONFIG, |value| {
            if enable {
                value & !0x80
            } else {
                value | 0x80
            }
        })
    }
    pub fn enable_repeat(&mut self, inputs: u8) -> RWResult<T, ()> {
        Ok(self.write_byte(R_REPEAT_EN, inputs)?)
    }
    pub fn enable_interrupts(&mut self, inputs: u8) -> RWResult<T, ()> {
        Ok(self.write_byte(R_INTERRUPT_EN, inputs)?)
    }
    pub fn enable_inputs(&mut self, inputs: u8) -> RWResult<T, ()> {
        Ok(self.write_byte(R_INPUT_ENABLE, inputs)?)
    }

    pub fn read_input_status(&mut self) -> Result<u8, ReadError<<T as WriteRead>::Error>> {
        self.read_byte(R_INPUT_STATUS)
    }

    pub fn read_threshold_values(&mut self) -> Result<Vec<u8>, ReadError<<T as WriteRead>::Error>> {
        self.read_block(R_INPUT_1_THRESH, self.number_of_leds as usize)
    }
    pub fn read_delta_values(&mut self) -> Result<Vec<u8>, ReadError<<T as WriteRead>::Error>> {
        self.read_block(R_INPUT_1_THRESH, self.number_of_leds as usize)
    }

    // ----------------------------------------------------------------------------
    // LEDS handling
    pub fn set_led_linking(&mut self, led_index: u8, state: bool) -> RWResult<T, ()> {
        if led_index >= self.number_of_leds {
            Err(Error::LedNumberOverflowError)
        } else {
            self.change_bit(R_LED_LINKING, led_index, state)
        }
    }
    pub fn set_led_output_type(&mut self, led_index: u8, state: bool) -> RWResult<T, ()> {
        if led_index >= self.number_of_leds {
            Err(Error::LedNumberOverflowError)
        } else {
            self.change_bit(R_LED_OUTPUT_TYPE, led_index, state)
        }
    }
    pub fn set_led_state(&mut self, led_index: u8, state: bool) -> RWResult<T, ()> {
        if led_index >= self.number_of_leds {
            Err(Error::LedNumberOverflowError)
        } else {
            self.change_bit(R_LED_OUTPUT_CON, led_index, state)
        }
    }
    pub fn set_led_polarity(&mut self, led_index: u8, state: bool) -> RWResult<T, ()> {
        if led_index >= self.number_of_leds {
            Err(Error::LedNumberOverflowError)
        } else {
            self.change_bit(R_LED_POLARITY, led_index, state)
        }
    }
    /// Set the behaviour of a LED
    pub fn set_led_behaviour(&mut self, led_index: u8, value: u8) -> RWResult<T, ()> {
        if led_index >= self.number_of_leds {
            Err(Error::LedNumberOverflowError)
        } else {
            let offset = led_index * 2 % 8;
            let register = led_index / 4;
            let value = value & 3;
            self.change_bits(R_LED_BEHAVIOUR_1 + register, offset, 2, value)
        }
    }
    pub fn convert_duration_to_period_value(period: Duration) -> u8 {
        ((min(4064, period.as_millis()) / 32) & 127) as u8
    }

    /// Set the overall period of a pulse from 32ms to 4.064 seconds
    pub fn set_led_pulse1_period(&mut self, period: Duration) -> RWResult<T, ()> {
        self.change_bits(
            R_LED_PULSE_1_PER,
            0,
            7,
            Self::convert_duration_to_period_value(period),
        )
    }
    /// Set the overall period of a pulse from 32ms to 4.064 seconds
    pub fn set_led_pulse2_period(&mut self, period: Duration) -> RWResult<T, ()> {
        self.change_bits(
            R_LED_PULSE_2_PER,
            0,
            7,
            Self::convert_duration_to_period_value(period),
        )
    }
    pub fn set_led_breathe_period(&mut self, period: Duration) -> RWResult<T, ()> {
        self.change_bits(
            R_LED_BREATHE_PER,
            0,
            7,
            Self::convert_duration_to_period_value(period),
        )
    }
    pub fn set_led_pulse1_count(&mut self, count: u8) -> RWResult<T, ()> {
        self.change_bits(R_LED_CONFIG, 0, 3, (count - 1) & 7)
    }
    pub fn set_led_pulse2_count(&mut self, count: u8) -> RWResult<T, ()> {
        self.change_bits(R_LED_CONFIG, 3, 3, (count - 1) & 7)
    }
    pub fn set_led_ramp_alert(&mut self, value: bool) -> RWResult<T, ()> {
        self.change_bit(R_LED_CONFIG, 6, value)
    }

    /// Set the rise/fall rate in ms, max 2000.
    //
    //  Rounds input to the nearest valid value.
    //
    //  Valid values are 0, 250, 500, 750, 1000, 1250, 1500, 2000
    pub fn set_led_direct_ramp_rate(&mut self, rise_rate: u16, fall_rate: u16) -> RWResult<T, ()> {
        let rise_rate = rise_rate / 250;
        let fall_rate = fall_rate / 250;
        let rise_rate = min(7, rise_rate);
        let fall_rate = min(7, fall_rate);
        let rate = rise_rate << 4 | fall_rate;
        Ok(self.write_byte(R_LED_DIRECT_RAMP, rate as u8)?)
    }
    pub fn set_led_direct_duty(&mut self, duty_min: u8, duty_max: u8) -> RWResult<T, ()> {
        let value = duty_max << 4 | duty_min;
        Ok(self.write_byte(R_LED_DIRECT_DUT, value)?)
    }
    pub fn set_led_pulse1_duty(&mut self, duty_min: u8, duty_max: u8) -> RWResult<T, ()> {
        let value = duty_max << 4 | duty_min;
        Ok(self.write_byte(R_LED_PULSE_1_DUT, value)?)
    }
    pub fn set_led_pulse2_duty(&mut self, duty_min: u8, duty_max: u8) -> RWResult<T, ()> {
        let value = duty_max << 4 | duty_min;
        Ok(self.write_byte(R_LED_PULSE_2_DUT, value)?)
    }
    pub fn set_led_breathe_duty(&mut self, duty_min: u8, duty_max: u8) -> RWResult<T, ()> {
        let value = duty_max << 4 | duty_min;
        Ok(self.write_byte(R_LED_BREATHE_DUT, value)?)
    }
    pub fn set_led_direct_min_duty(&mut self, value: u8) -> RWResult<T, ()> {
        self.change_bits(R_LED_DIRECT_DUT, 0, 4, value)
    }
    pub fn set_led_direct_max_duty(&mut self, value: u8) -> RWResult<T, ()> {
        self.change_bits(R_LED_DIRECT_DUT, 4, 4, value)
    }
    pub fn set_led_breathe_min_duty(&mut self, value: u8) -> RWResult<T, ()> {
        self.change_bits(R_LED_BREATHE_DUT, 0, 4, value)
    }
    pub fn set_led_breathe_max_duty(&mut self, value: u8) -> RWResult<T, ()> {
        self.change_bits(R_LED_BREATHE_DUT, 4, 4, value)
    }
    pub fn set_led_pulse1_min_duty(&mut self, value: u8) -> RWResult<T, ()> {
        self.change_bits(R_LED_PULSE_1_DUT, 0, 4, value)
    }
    pub fn set_led_pulse1_max_duty(&mut self, value: u8) -> RWResult<T, ()> {
        self.change_bits(R_LED_PULSE_1_DUT, 4, 4, value)
    }
    pub fn set_led_pulse2_min_duty(&mut self, value: u8) -> RWResult<T, ()> {
        self.change_bits(R_LED_PULSE_2_DUT, 0, 4, value)
    }
    pub fn set_led_pulse2_max_duty(&mut self, value: u8) -> RWResult<T, ()> {
        self.change_bits(R_LED_PULSE_2_DUT, 4, 4, value)
    }
}

// internal trait to help sending commands to our device
trait CmdWrite<T: Write> {
    fn cmd_write(&mut self, i2c_address: u8, command: u8, buf: &[u8]) -> Result<(), T::Error>;
}

impl<T: Write> CmdWrite<T> for T {
    fn cmd_write(&mut self, i2c_address: u8, command: u8, buffer: &[u8]) -> Result<(), T::Error> {
        // *r is really shitty
        let to_send: Vec<u8> = [command].iter().chain(buffer).map(|r| *r).collect();
        self.write(i2c_address, &to_send)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
