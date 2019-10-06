//! cap1xxx [`embedded-hal`] I2C driver for Microchip cap1xxx capacitive touch buttons
//!
//! This crate is primarily targeting cap1666 6 buttons - 6 led hardware found
//! on the Pimoroni GFX HAT
#![allow(dead_code)]
use embedded_hal::blocking::i2c::{Write, WriteRead};

mod consts;

use consts::*;
use std::cmp::min;
use std::time::Duration;

// Read Error
pub struct ReadError<T>(T);

//impl<R: Read> !Read for ReadError<R>;

impl<T> From<T> for ReadError<T> {
    fn from(read_error: T) -> Self {
        ReadError(read_error)
    }
}

// Write Error
pub struct WriteError<W>(W);

impl<W> From<W> for WriteError<W> {
    fn from(write_error: W) -> Self {
        WriteError(write_error)
    }
}

// Combined Read or Write Error
pub enum Error<R, W> {
    ReadError(ReadError<R>),
    WriteError(WriteError<W>),
    LedNumberOverflowError,
}

impl<R, W> From<ReadError<R>> for Error<R, W> {
    fn from(e: ReadError<R>) -> Self {
        Error::ReadError(e)
    }
}
impl<R, W> From<WriteError<W>> for Error<R, W> {
    fn from(e: WriteError<W>) -> Self {
        Error::WriteError(e)
    }
}

pub type RWResult<T, R> = Result<R, Error<<T as WriteRead>::Error, <T as Write>::Error>>;

struct CAP1XXX<T>
where
    T: WriteRead + Write,
{
    i2c: T,
    number_of_leds: u8,
}

impl<T> CAP1XXX<T>
where
    T: WriteRead + Write,
{
    pub fn new(i2c: T, number_of_leds: u8) -> Self {
        Self {
            i2c,
            number_of_leds,
        }
    }
    fn write_byte(
        &mut self,
        register: u8,
        value: u8,
    ) -> Result<(), WriteError<<T as Write>::Error>> {
        Ok(self.i2c.cmd_write(DEFAULT_ADDR, register, &[value])?)
    }

    fn read_byte(&mut self, register: u8) -> Result<u8, ReadError<<T as WriteRead>::Error>> {
        let mut buf = [0u8];
        self.i2c.write_read(DEFAULT_ADDR, &[register], &mut buf)?;
        Ok(buf[0])
    }

    fn read_block(
        &mut self,
        register: u8,
        len: usize,
    ) -> Result<Vec<u8>, ReadError<<T as WriteRead>::Error>> {
        let mut buf = vec![0u8; len];
        self.i2c.write_read(DEFAULT_ADDR, &[register], &mut buf)?;
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

    // LEDS handling
    fn set_led_linking(&mut self, led_index: u8, state: bool) -> RWResult<T, ()> {
        if led_index >= self.number_of_leds {
            Err(Error::LedNumberOverflowError)
        } else {
            self.change_bit(R_LED_LINKING, led_index, state)
        }
    }
    fn set_led_output_type(&mut self, led_index: u8, state: bool) -> RWResult<T, ()> {
        if led_index >= self.number_of_leds {
            Err(Error::LedNumberOverflowError)
        } else {
            self.change_bit(R_LED_OUTPUT_TYPE, led_index, state)
        }
    }
    fn set_led_state(&mut self, led_index: u8, state: bool) -> RWResult<T, ()> {
        if led_index >= self.number_of_leds {
            Err(Error::LedNumberOverflowError)
        } else {
            self.change_bit(R_LED_OUTPUT_CON, led_index, state)
        }
    }
    fn set_led_polarity(&mut self, led_index: u8, state: bool) -> RWResult<T, ()> {
        if led_index >= self.number_of_leds {
            Err(Error::LedNumberOverflowError)
        } else {
            self.change_bit(R_LED_POLARITY, led_index, state)
        }
    }
    /// Set the behaviour of a LED
    fn set_led_behaviour(&mut self, led_index: u8, value: u8) -> RWResult<T, ()> {
        if led_index >= self.number_of_leds {
            Err(Error::LedNumberOverflowError)
        } else {
            let offset = led_index * 2 % 8;
            let register = led_index / 4;
            let value = value & 3;
            self.change_bits(R_LED_BEHAVIOUR_1 + register, offset, 2, value)
        }
    }
    fn convert_duration_to_period_value(period: Duration) -> u8 {
        ((period.as_millis() / 32) & 127) as u8
    }

    /// Set the overall period of a pulse from 32ms to 4.064 seconds
    fn set_led_pulse1_period(&mut self, period: Duration) -> RWResult<T, ()> {
        self.change_bits(
            R_LED_PULSE_1_PER,
            0,
            7,
            Self::convert_duration_to_period_value(period),
        )
    }
    /// Set the overall period of a pulse from 32ms to 4.064 seconds
    fn set_led_pulse2_period(&mut self, period: Duration) -> RWResult<T, ()> {
        self.change_bits(
            R_LED_PULSE_2_PER,
            0,
            7,
            Self::convert_duration_to_period_value(period),
        )
    }
    fn set_led_breathe_period(&mut self, period: Duration) -> RWResult<T, ()> {
        self.change_bits(
            R_LED_BREATHE_PER,
            0,
            7,
            Self::convert_duration_to_period_value(period),
        )
    }
    fn set_led_pulse1_count(&mut self, count: u8) -> RWResult<T, ()> {
        self.change_bits(R_LED_CONFIG, 0, 3, (count - 1) & 7)
    }
    fn set_led_pulse2_count(&mut self, count: u8) -> RWResult<T, ()> {
        self.change_bits(R_LED_CONFIG, 3, 3, (count - 1) & 7)
    }
    fn set_led_ramp_alert(&mut self, value: bool) -> RWResult<T, ()> {
        self.change_bit(R_LED_CONFIG, 6, value)
    }

    /// Set the rise/fall rate in ms, max 2000.
    //
    //  Rounds input to the nearest valid value.
    //
    //  Valid values are 0, 250, 500, 750, 1000, 1250, 1500, 2000
    fn set_led_direct_ramp_rate(&mut self, rise_rate: u16, fall_rate: u16) -> RWResult<T, ()> {
        let rise_rate = rise_rate / 250;
        let fall_rate = fall_rate / 250;
        let rise_rate = min(7, rise_rate);
        let fall_rate = min(7, fall_rate);
        let rate = rise_rate << 4 | fall_rate;
        Ok(self.write_byte(R_LED_DIRECT_RAMP, rate as u8)?)
    }
    fn set_led_direct_duty(&mut self, duty_min: u8, duty_max: u8) -> RWResult<T, ()> {
        let value = duty_max << 4 | duty_min;
        Ok(self.write_byte(R_LED_DIRECT_DUT, value)?)
    }
    fn set_led_pulse1_duty(&mut self, duty_min: u8, duty_max: u8) -> RWResult<T, ()> {
        let value = duty_max << 4 | duty_min;
        Ok(self.write_byte(R_LED_PULSE_1_DUT, value)?)
    }
    fn set_led_pulse2_duty(&mut self, duty_min: u8, duty_max: u8) -> RWResult<T, ()> {
        let value = duty_max << 4 | duty_min;
        Ok(self.write_byte(R_LED_PULSE_2_DUT, value)?)
    }
    fn set_led_breathe_duty(&mut self, duty_min: u8, duty_max: u8) -> RWResult<T, ()> {
        let value = duty_max << 4 | duty_min;
        Ok(self.write_byte(R_LED_BREATHE_DUT, value)?)
    }
    fn set_led_direct_min_duty(&mut self, value: u8) -> RWResult<T, ()> {
        self.change_bits(R_LED_DIRECT_DUT, 0, 4, value)
    }
    fn set_led_direct_max_duty(&mut self, value: u8) -> RWResult<T, ()> {
        self.change_bits(R_LED_DIRECT_DUT, 4, 4, value)
    }
    fn set_led_breathe_min_duty(&mut self, value: u8) -> RWResult<T, ()> {
        self.change_bits(R_LED_BREATHE_DUT, 0, 4, value)
    }
    fn set_led_breathe_max_duty(&mut self, value: u8) -> RWResult<T, ()> {
        self.change_bits(R_LED_BREATHE_DUT, 4, 4, value)
    }
    fn set_led_pulse1_min_duty(&mut self, value: u8) -> RWResult<T, ()> {
        self.change_bits(R_LED_PULSE_1_DUT, 0, 4, value)
    }
    fn set_led_pulse1_max_duty(&mut self, value: u8) -> RWResult<T, ()> {
        self.change_bits(R_LED_PULSE_1_DUT, 4, 4, value)
    }
    fn set_led_pulse2_min_duty(&mut self, value: u8) -> RWResult<T, ()> {
        self.change_bits(R_LED_PULSE_2_DUT, 0, 4, value)
    }
    fn set_led_pulse2_max_duty(&mut self, value: u8) -> RWResult<T, ()> {
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
