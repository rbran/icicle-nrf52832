use icicle_vm::cpu::mem::MemResult;

use super::enums::{PinDrive, PinPull, PinSense};

#[derive(Default)]
#[doc = "P0: GPIO Port 1<br><br>Instances:<br>0x50000000: P0<br>"]
pub struct P0 {
    pub pins: [Pin; 32],
}

#[derive(Default)]
pub struct Pin {
    pub pull: PinPull,
    pub drive: PinDrive,
    pub sense: PinSense,
    pub input_buffer: bool,
    pub output_mode: bool,
    pub output_high: bool,
}

/// internals here
impl P0 {
    /// read the pin input state, returning High (true) or Low (false)
    pub fn read_input(&self, pin: usize) -> bool {
        todo!(
            "Read input pin {pin} in {} mode",
            if self.pins[pin].output_mode {
                "output"
            } else {
                "input"
            }
        );
    }

    /// pull mode
    pub fn get_pull(&self, pin: usize) -> PinPull {
        self.pins[pin].pull
    }

    /// set pull mode
    pub fn set_pull(&mut self, pin: usize, pull: PinPull) {
        self.pins[pin].pull = pull
    }

    /// drive mode
    pub fn get_drive(&self, pin: usize) -> PinDrive {
        self.pins[pin].drive
    }

    /// set drive mode
    pub fn set_drive(&mut self, pin: usize, drive: PinDrive) {
        self.pins[pin].drive = drive
    }

    /// sense mode
    pub fn get_sense(&self, pin: usize) -> PinSense {
        self.pins[pin].sense
    }

    /// set input_buffer on/off
    pub fn set_input_buffer(&mut self, pin: usize, on: bool) {
        self.pins[pin].input_buffer = on
    }

    /// input_buffer on/off
    pub fn get_input_buffer(&self, pin: usize) -> bool {
        self.pins[pin].input_buffer
    }

    /// set sense mode
    pub fn set_sense(&mut self, pin: usize, sense: PinSense) {
        self.pins[pin].sense = sense
    }


    /// if the pin is output (true) or input (false) mode
    pub fn get_output_mode(&self, pin: usize) -> bool {
        self.pins[pin].output_mode
    }

    /// set the pin output (true) or input (false) mode
    pub fn set_output_mode(&mut self, pin: usize, mode: bool) {
        self.pins[pin].output_mode = mode;
    }

    /// get the output pin state, returning High (true) or Low (false)
    /// NOTE not the same as [P0::read_input], this don't read the input value
    /// just read if the output is configured to output High/Low.
    pub fn get_output(&self, pin: usize) -> bool {
        self.pins[pin].output_high
    }

    /// set the output pin state, returning High (true) or Low (false)
    pub fn set_output(&mut self, pin: usize, state: bool) {
        //if !self.pins[pin].output_mode {
        //    todo!("change output mode of {pin} while in input mode");
        //}
        self.pins[pin].output_high = state;
    }
}

/// registers read/write here
impl P0 {
    pub(crate) fn page_to_index(page: u64) -> usize {
        match page {
            327680u64 => 0usize,
            _ => unreachable!(),
        }
    }
    #[doc = "PIN0: Pin 0<br>"]
    pub(crate) fn p0_out504_pin0_read(&self) -> MemResult<bool> {
        Ok(self.get_output(0))
    }
    #[doc = "PIN0: Pin 0<br>"]
    pub fn p0_out504_pin0_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output(0, _value))
    }
    #[doc = "PIN1: Pin 1<br>"]
    pub(crate) fn p0_out504_pin1_read(&self) -> MemResult<bool> {
        Ok(self.get_output(1))
    }
    #[doc = "PIN1: Pin 1<br>"]
    pub fn p0_out504_pin1_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output(1, _value))
    }
    #[doc = "PIN2: Pin 2<br>"]
    pub(crate) fn p0_out504_pin2_read(&self) -> MemResult<bool> {
        Ok(self.get_output(2))
    }
    #[doc = "PIN2: Pin 2<br>"]
    pub fn p0_out504_pin2_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output(2, _value))
    }
    #[doc = "PIN3: Pin 3<br>"]
    pub(crate) fn p0_out504_pin3_read(&self) -> MemResult<bool> {
        Ok(self.get_output(3))
    }
    #[doc = "PIN3: Pin 3<br>"]
    pub fn p0_out504_pin3_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output(3, _value))
    }
    #[doc = "PIN4: Pin 4<br>"]
    pub(crate) fn p0_out504_pin4_read(&self) -> MemResult<bool> {
        Ok(self.get_output(4))
    }
    #[doc = "PIN4: Pin 4<br>"]
    pub fn p0_out504_pin4_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output(4, _value))
    }
    #[doc = "PIN5: Pin 5<br>"]
    pub(crate) fn p0_out504_pin5_read(&self) -> MemResult<bool> {
        Ok(self.get_output(5))
    }
    #[doc = "PIN5: Pin 5<br>"]
    pub fn p0_out504_pin5_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output(5, _value))
    }
    #[doc = "PIN6: Pin 6<br>"]
    pub(crate) fn p0_out504_pin6_read(&self) -> MemResult<bool> {
        Ok(self.get_output(6))
    }
    #[doc = "PIN6: Pin 6<br>"]
    pub fn p0_out504_pin6_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output(6, _value))
    }
    #[doc = "PIN7: Pin 7<br>"]
    pub(crate) fn p0_out504_pin7_read(&self) -> MemResult<bool> {
        Ok(self.get_output(7))
    }
    #[doc = "PIN7: Pin 7<br>"]
    pub fn p0_out504_pin7_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output(7, _value))
    }
    #[doc = "PIN8: Pin 8<br>"]
    pub(crate) fn p0_out504_pin8_read(&self) -> MemResult<bool> {
        Ok(self.get_output(8))
    }
    #[doc = "PIN8: Pin 8<br>"]
    pub fn p0_out504_pin8_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output(8, _value))
    }
    #[doc = "PIN9: Pin 9<br>"]
    pub(crate) fn p0_out504_pin9_read(&self) -> MemResult<bool> {
        Ok(self.get_output(9))
    }
    #[doc = "PIN9: Pin 9<br>"]
    pub fn p0_out504_pin9_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output(9, _value))
    }
    #[doc = "PIN10: Pin 10<br>"]
    pub(crate) fn p0_out504_pin10_read(&self) -> MemResult<bool> {
        Ok(self.get_output(10))
    }
    #[doc = "PIN10: Pin 10<br>"]
    pub fn p0_out504_pin10_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output(10, _value))
    }
    #[doc = "PIN11: Pin 11<br>"]
    pub(crate) fn p0_out504_pin11_read(&self) -> MemResult<bool> {
        Ok(self.get_output(11))
    }
    #[doc = "PIN11: Pin 11<br>"]
    pub fn p0_out504_pin11_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output(11, _value))
    }
    #[doc = "PIN12: Pin 12<br>"]
    pub(crate) fn p0_out504_pin12_read(&self) -> MemResult<bool> {
        Ok(self.get_output(12))
    }
    #[doc = "PIN12: Pin 12<br>"]
    pub fn p0_out504_pin12_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output(12, _value))
    }
    #[doc = "PIN13: Pin 13<br>"]
    pub(crate) fn p0_out504_pin13_read(&self) -> MemResult<bool> {
        Ok(self.get_output(13))
    }
    #[doc = "PIN13: Pin 13<br>"]
    pub fn p0_out504_pin13_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output(13, _value))
    }
    #[doc = "PIN14: Pin 14<br>"]
    pub(crate) fn p0_out504_pin14_read(&self) -> MemResult<bool> {
        Ok(self.get_output(14))
    }
    #[doc = "PIN14: Pin 14<br>"]
    pub fn p0_out504_pin14_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output(14, _value))
    }
    #[doc = "PIN15: Pin 15<br>"]
    pub(crate) fn p0_out504_pin15_read(&self) -> MemResult<bool> {
        Ok(self.get_output(15))
    }
    #[doc = "PIN15: Pin 15<br>"]
    pub fn p0_out504_pin15_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output(15, _value))
    }
    #[doc = "PIN16: Pin 16<br>"]
    pub(crate) fn p0_out504_pin16_read(&self) -> MemResult<bool> {
        Ok(self.get_output(16))
    }
    #[doc = "PIN16: Pin 16<br>"]
    pub fn p0_out504_pin16_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output(16, _value))
    }
    #[doc = "PIN17: Pin 17<br>"]
    pub(crate) fn p0_out504_pin17_read(&self) -> MemResult<bool> {
        Ok(self.get_output(17))
    }
    #[doc = "PIN17: Pin 17<br>"]
    pub fn p0_out504_pin17_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output(17, _value))
    }
    #[doc = "PIN18: Pin 18<br>"]
    pub(crate) fn p0_out504_pin18_read(&self) -> MemResult<bool> {
        Ok(self.get_output(18))
    }
    #[doc = "PIN18: Pin 18<br>"]
    pub fn p0_out504_pin18_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output(18, _value))
    }
    #[doc = "PIN19: Pin 19<br>"]
    pub(crate) fn p0_out504_pin19_read(&self) -> MemResult<bool> {
        Ok(self.get_output(19))
    }
    #[doc = "PIN19: Pin 19<br>"]
    pub fn p0_out504_pin19_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output(19, _value))
    }
    #[doc = "PIN20: Pin 20<br>"]
    pub(crate) fn p0_out504_pin20_read(&self) -> MemResult<bool> {
        Ok(self.get_output(20))
    }
    #[doc = "PIN20: Pin 20<br>"]
    pub fn p0_out504_pin20_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output(20, _value))
    }
    #[doc = "PIN21: Pin 21<br>"]
    pub(crate) fn p0_out504_pin21_read(&self) -> MemResult<bool> {
        Ok(self.get_output(21))
    }
    #[doc = "PIN21: Pin 21<br>"]
    pub fn p0_out504_pin21_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output(21, _value))
    }
    #[doc = "PIN22: Pin 22<br>"]
    pub(crate) fn p0_out504_pin22_read(&self) -> MemResult<bool> {
        Ok(self.get_output(22))
    }
    #[doc = "PIN22: Pin 22<br>"]
    pub fn p0_out504_pin22_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output(22, _value))
    }
    #[doc = "PIN23: Pin 23<br>"]
    pub(crate) fn p0_out504_pin23_read(&self) -> MemResult<bool> {
        Ok(self.get_output(23))
    }
    #[doc = "PIN23: Pin 23<br>"]
    pub fn p0_out504_pin23_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output(23, _value))
    }
    #[doc = "PIN24: Pin 24<br>"]
    pub(crate) fn p0_out504_pin24_read(&self) -> MemResult<bool> {
        Ok(self.get_output(24))
    }
    #[doc = "PIN24: Pin 24<br>"]
    pub fn p0_out504_pin24_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output(24, _value))
    }
    #[doc = "PIN25: Pin 25<br>"]
    pub(crate) fn p0_out504_pin25_read(&self) -> MemResult<bool> {
        Ok(self.get_output(25))
    }
    #[doc = "PIN25: Pin 25<br>"]
    pub fn p0_out504_pin25_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output(25, _value))
    }
    #[doc = "PIN26: Pin 26<br>"]
    pub(crate) fn p0_out504_pin26_read(&self) -> MemResult<bool> {
        Ok(self.get_output(26))
    }
    #[doc = "PIN26: Pin 26<br>"]
    pub fn p0_out504_pin26_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output(26, _value))
    }
    #[doc = "PIN27: Pin 27<br>"]
    pub(crate) fn p0_out504_pin27_read(&self) -> MemResult<bool> {
        Ok(self.get_output(27))
    }
    #[doc = "PIN27: Pin 27<br>"]
    pub fn p0_out504_pin27_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output(27, _value))
    }
    #[doc = "PIN28: Pin 28<br>"]
    pub(crate) fn p0_out504_pin28_read(&self) -> MemResult<bool> {
        Ok(self.get_output(28))
    }
    #[doc = "PIN28: Pin 28<br>"]
    pub fn p0_out504_pin28_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output(28, _value))
    }
    #[doc = "PIN29: Pin 29<br>"]
    pub(crate) fn p0_out504_pin29_read(&self) -> MemResult<bool> {
        Ok(self.get_output(29))
    }
    #[doc = "PIN29: Pin 29<br>"]
    pub fn p0_out504_pin29_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output(29, _value))
    }
    #[doc = "PIN30: Pin 30<br>"]
    pub(crate) fn p0_out504_pin30_read(&self) -> MemResult<bool> {
        Ok(self.get_output(30))
    }
    #[doc = "PIN30: Pin 30<br>"]
    pub fn p0_out504_pin30_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output(30, _value))
    }
    #[doc = "PIN31: Pin 31<br>"]
    pub(crate) fn p0_out504_pin31_read(&self) -> MemResult<bool> {
        Ok(self.get_output(31))
    }
    #[doc = "PIN31: Pin 31<br>"]
    pub fn p0_out504_pin31_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output(31, _value))
    }
    #[doc = "PIN0: Pin 0<br>"]
    pub(crate) fn p0_outset508_pin0_read(&self) -> MemResult<bool> {
        Ok(self.get_output(0))
    }
    #[doc = "PIN0: Pin 0<br>"]
    pub fn p0_outset508_pin0_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(0, true);
        }
        Ok(())
    }
    #[doc = "PIN1: Pin 1<br>"]
    pub(crate) fn p0_outset508_pin1_read(&self) -> MemResult<bool> {
        Ok(self.get_output(1))
    }
    #[doc = "PIN1: Pin 1<br>"]
    pub fn p0_outset508_pin1_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(1, true);
        }
        Ok(())
    }
    #[doc = "PIN2: Pin 2<br>"]
    pub(crate) fn p0_outset508_pin2_read(&self) -> MemResult<bool> {
        Ok(self.get_output(2))
    }
    #[doc = "PIN2: Pin 2<br>"]
    pub fn p0_outset508_pin2_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(2, true);
        }
        Ok(())
    }
    #[doc = "PIN3: Pin 3<br>"]
    pub(crate) fn p0_outset508_pin3_read(&self) -> MemResult<bool> {
        Ok(self.get_output(3))
    }
    #[doc = "PIN3: Pin 3<br>"]
    pub fn p0_outset508_pin3_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(3, true);
        }
        Ok(())
    }
    #[doc = "PIN4: Pin 4<br>"]
    pub(crate) fn p0_outset508_pin4_read(&self) -> MemResult<bool> {
        Ok(self.get_output(4))
    }
    #[doc = "PIN4: Pin 4<br>"]
    pub fn p0_outset508_pin4_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(4, true);
        }
        Ok(())
    }
    #[doc = "PIN5: Pin 5<br>"]
    pub(crate) fn p0_outset508_pin5_read(&self) -> MemResult<bool> {
        Ok(self.get_output(5))
    }
    #[doc = "PIN5: Pin 5<br>"]
    pub fn p0_outset508_pin5_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(5, true);
        }
        Ok(())
    }
    #[doc = "PIN6: Pin 6<br>"]
    pub(crate) fn p0_outset508_pin6_read(&self) -> MemResult<bool> {
        Ok(self.get_output(6))
    }
    #[doc = "PIN6: Pin 6<br>"]
    pub fn p0_outset508_pin6_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(6, true);
        }
        Ok(())
    }
    #[doc = "PIN7: Pin 7<br>"]
    pub(crate) fn p0_outset508_pin7_read(&self) -> MemResult<bool> {
        Ok(self.get_output(7))
    }
    #[doc = "PIN7: Pin 7<br>"]
    pub fn p0_outset508_pin7_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(7, true);
        }
        Ok(())
    }
    #[doc = "PIN8: Pin 8<br>"]
    pub(crate) fn p0_outset508_pin8_read(&self) -> MemResult<bool> {
        Ok(self.get_output(8))
    }
    #[doc = "PIN8: Pin 8<br>"]
    pub fn p0_outset508_pin8_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(8, true);
        }
        Ok(())
    }
    #[doc = "PIN9: Pin 9<br>"]
    pub(crate) fn p0_outset508_pin9_read(&self) -> MemResult<bool> {
        Ok(self.get_output(9))
    }
    #[doc = "PIN9: Pin 9<br>"]
    pub fn p0_outset508_pin9_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(9, true);
        }
        Ok(())
    }
    #[doc = "PIN10: Pin 10<br>"]
    pub(crate) fn p0_outset508_pin10_read(&self) -> MemResult<bool> {
        Ok(self.get_output(10))
    }
    #[doc = "PIN10: Pin 10<br>"]
    pub fn p0_outset508_pin10_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(10, true);
        }
        Ok(())
    }
    #[doc = "PIN11: Pin 11<br>"]
    pub(crate) fn p0_outset508_pin11_read(&self) -> MemResult<bool> {
        Ok(self.get_output(11))
    }
    #[doc = "PIN11: Pin 11<br>"]
    pub fn p0_outset508_pin11_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(11, true);
        }
        Ok(())
    }
    #[doc = "PIN12: Pin 12<br>"]
    pub(crate) fn p0_outset508_pin12_read(&self) -> MemResult<bool> {
        Ok(self.get_output(12))
    }
    #[doc = "PIN12: Pin 12<br>"]
    pub fn p0_outset508_pin12_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(12, true);
        }
        Ok(())
    }
    #[doc = "PIN13: Pin 13<br>"]
    pub(crate) fn p0_outset508_pin13_read(&self) -> MemResult<bool> {
        Ok(self.get_output(13))
    }
    #[doc = "PIN13: Pin 13<br>"]
    pub fn p0_outset508_pin13_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(13, true);
        }
        Ok(())
    }
    #[doc = "PIN14: Pin 14<br>"]
    pub(crate) fn p0_outset508_pin14_read(&self) -> MemResult<bool> {
        Ok(self.get_output(14))
    }
    #[doc = "PIN14: Pin 14<br>"]
    pub fn p0_outset508_pin14_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(14, true);
        }
        Ok(())
    }
    #[doc = "PIN15: Pin 15<br>"]
    pub(crate) fn p0_outset508_pin15_read(&self) -> MemResult<bool> {
        Ok(self.get_output(15))
    }
    #[doc = "PIN15: Pin 15<br>"]
    pub fn p0_outset508_pin15_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(15, true);
        }
        Ok(())
    }
    #[doc = "PIN16: Pin 16<br>"]
    pub(crate) fn p0_outset508_pin16_read(&self) -> MemResult<bool> {
        Ok(self.get_output(16))
    }
    #[doc = "PIN16: Pin 16<br>"]
    pub fn p0_outset508_pin16_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(16, true);
        }
        Ok(())
    }
    #[doc = "PIN17: Pin 17<br>"]
    pub(crate) fn p0_outset508_pin17_read(&self) -> MemResult<bool> {
        Ok(self.get_output(17))
    }
    #[doc = "PIN17: Pin 17<br>"]
    pub fn p0_outset508_pin17_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(17, true);
        }
        Ok(())
    }
    #[doc = "PIN18: Pin 18<br>"]
    pub(crate) fn p0_outset508_pin18_read(&self) -> MemResult<bool> {
        Ok(self.get_output(18))
    }
    #[doc = "PIN18: Pin 18<br>"]
    pub fn p0_outset508_pin18_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(18, true);
        }
        Ok(())
    }
    #[doc = "PIN19: Pin 19<br>"]
    pub(crate) fn p0_outset508_pin19_read(&self) -> MemResult<bool> {
        Ok(self.get_output(19))
    }
    #[doc = "PIN19: Pin 19<br>"]
    pub fn p0_outset508_pin19_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(19, true);
        }
        Ok(())
    }
    #[doc = "PIN20: Pin 20<br>"]
    pub(crate) fn p0_outset508_pin20_read(&self) -> MemResult<bool> {
        Ok(self.get_output(20))
    }
    #[doc = "PIN20: Pin 20<br>"]
    pub fn p0_outset508_pin20_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(20, true);
        }
        Ok(())
    }
    #[doc = "PIN21: Pin 21<br>"]
    pub(crate) fn p0_outset508_pin21_read(&self) -> MemResult<bool> {
        Ok(self.get_output(21))
    }
    #[doc = "PIN21: Pin 21<br>"]
    pub fn p0_outset508_pin21_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(21, true);
        }
        Ok(())
    }
    #[doc = "PIN22: Pin 22<br>"]
    pub(crate) fn p0_outset508_pin22_read(&self) -> MemResult<bool> {
        Ok(self.get_output(22))
    }
    #[doc = "PIN22: Pin 22<br>"]
    pub fn p0_outset508_pin22_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(22, true);
        }
        Ok(())
    }
    #[doc = "PIN23: Pin 23<br>"]
    pub(crate) fn p0_outset508_pin23_read(&self) -> MemResult<bool> {
        Ok(self.get_output(23))
    }
    #[doc = "PIN23: Pin 23<br>"]
    pub fn p0_outset508_pin23_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(23, true);
        }
        Ok(())
    }
    #[doc = "PIN24: Pin 24<br>"]
    pub(crate) fn p0_outset508_pin24_read(&self) -> MemResult<bool> {
        Ok(self.get_output(24))
    }
    #[doc = "PIN24: Pin 24<br>"]
    pub fn p0_outset508_pin24_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(24, true);
        }
        Ok(())
    }
    #[doc = "PIN25: Pin 25<br>"]
    pub(crate) fn p0_outset508_pin25_read(&self) -> MemResult<bool> {
        Ok(self.get_output(25))
    }
    #[doc = "PIN25: Pin 25<br>"]
    pub fn p0_outset508_pin25_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(25, true);
        }
        Ok(())
    }
    #[doc = "PIN26: Pin 26<br>"]
    pub(crate) fn p0_outset508_pin26_read(&self) -> MemResult<bool> {
        Ok(self.get_output(26))
    }
    #[doc = "PIN26: Pin 26<br>"]
    pub fn p0_outset508_pin26_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(26, true);
        }
        Ok(())
    }
    #[doc = "PIN27: Pin 27<br>"]
    pub(crate) fn p0_outset508_pin27_read(&self) -> MemResult<bool> {
        Ok(self.get_output(27))
    }
    #[doc = "PIN27: Pin 27<br>"]
    pub fn p0_outset508_pin27_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(27, true);
        }
        Ok(())
    }
    #[doc = "PIN28: Pin 28<br>"]
    pub(crate) fn p0_outset508_pin28_read(&self) -> MemResult<bool> {
        Ok(self.get_output(28))
    }
    #[doc = "PIN28: Pin 28<br>"]
    pub fn p0_outset508_pin28_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(28, true);
        }
        Ok(())
    }
    #[doc = "PIN29: Pin 29<br>"]
    pub(crate) fn p0_outset508_pin29_read(&self) -> MemResult<bool> {
        Ok(self.get_output(29))
    }
    #[doc = "PIN29: Pin 29<br>"]
    pub fn p0_outset508_pin29_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(29, true);
        }
        Ok(())
    }
    #[doc = "PIN30: Pin 30<br>"]
    pub(crate) fn p0_outset508_pin30_read(&self) -> MemResult<bool> {
        Ok(self.get_output(30))
    }
    #[doc = "PIN30: Pin 30<br>"]
    pub fn p0_outset508_pin30_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(30, true);
        }
        Ok(())
    }
    #[doc = "PIN31: Pin 31<br>"]
    pub(crate) fn p0_outset508_pin31_read(&self) -> MemResult<bool> {
        Ok(self.get_output(31))
    }
    #[doc = "PIN31: Pin 31<br>"]
    pub fn p0_outset508_pin31_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(31, true);
        }
        Ok(())
    }
    #[doc = "PIN0: Pin 0<br>"]
    pub(crate) fn p0_outclr50c_pin0_read(&self) -> MemResult<bool> {
        Ok(self.get_output(0))
    }
    #[doc = "PIN0: Pin 0<br>"]
    pub fn p0_outclr50c_pin0_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(0, false);
        }
        Ok(())
    }
    #[doc = "PIN1: Pin 1<br>"]
    pub(crate) fn p0_outclr50c_pin1_read(&self) -> MemResult<bool> {
        Ok(self.get_output(1))
    }
    #[doc = "PIN1: Pin 1<br>"]
    pub fn p0_outclr50c_pin1_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(1, false);
        }
        Ok(())
    }
    #[doc = "PIN2: Pin 2<br>"]
    pub(crate) fn p0_outclr50c_pin2_read(&self) -> MemResult<bool> {
        Ok(self.get_output(2))
    }
    #[doc = "PIN2: Pin 2<br>"]
    pub fn p0_outclr50c_pin2_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(2, false);
        }
        Ok(())
    }
    #[doc = "PIN3: Pin 3<br>"]
    pub(crate) fn p0_outclr50c_pin3_read(&self) -> MemResult<bool> {
        Ok(self.get_output(3))
    }
    #[doc = "PIN3: Pin 3<br>"]
    pub fn p0_outclr50c_pin3_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(3, false);
        }
        Ok(())
    }
    #[doc = "PIN4: Pin 4<br>"]
    pub(crate) fn p0_outclr50c_pin4_read(&self) -> MemResult<bool> {
        Ok(self.get_output(4))
    }
    #[doc = "PIN4: Pin 4<br>"]
    pub fn p0_outclr50c_pin4_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(4, false);
        }
        Ok(())
    }
    #[doc = "PIN5: Pin 5<br>"]
    pub(crate) fn p0_outclr50c_pin5_read(&self) -> MemResult<bool> {
        Ok(self.get_output(5))
    }
    #[doc = "PIN5: Pin 5<br>"]
    pub fn p0_outclr50c_pin5_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(5, false);
        }
        Ok(())
    }
    #[doc = "PIN6: Pin 6<br>"]
    pub(crate) fn p0_outclr50c_pin6_read(&self) -> MemResult<bool> {
        Ok(self.get_output(6))
    }
    #[doc = "PIN6: Pin 6<br>"]
    pub fn p0_outclr50c_pin6_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(6, false);
        }
        Ok(())
    }
    #[doc = "PIN7: Pin 7<br>"]
    pub(crate) fn p0_outclr50c_pin7_read(&self) -> MemResult<bool> {
        Ok(self.get_output(7))
    }
    #[doc = "PIN7: Pin 7<br>"]
    pub fn p0_outclr50c_pin7_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(7, false);
        }
        Ok(())
    }
    #[doc = "PIN8: Pin 8<br>"]
    pub(crate) fn p0_outclr50c_pin8_read(&self) -> MemResult<bool> {
        Ok(self.get_output(8))
    }
    #[doc = "PIN8: Pin 8<br>"]
    pub fn p0_outclr50c_pin8_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(8, false);
        }
        Ok(())
    }
    #[doc = "PIN9: Pin 9<br>"]
    pub(crate) fn p0_outclr50c_pin9_read(&self) -> MemResult<bool> {
        Ok(self.get_output(9))
    }
    #[doc = "PIN9: Pin 9<br>"]
    pub fn p0_outclr50c_pin9_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(9, false);
        }
        Ok(())
    }
    #[doc = "PIN10: Pin 10<br>"]
    pub(crate) fn p0_outclr50c_pin10_read(&self) -> MemResult<bool> {
        Ok(self.get_output(10))
    }
    #[doc = "PIN10: Pin 10<br>"]
    pub fn p0_outclr50c_pin10_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(10, false);
        }
        Ok(())
    }
    #[doc = "PIN11: Pin 11<br>"]
    pub(crate) fn p0_outclr50c_pin11_read(&self) -> MemResult<bool> {
        Ok(self.get_output(11))
    }
    #[doc = "PIN11: Pin 11<br>"]
    pub fn p0_outclr50c_pin11_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(11, false);
        }
        Ok(())
    }
    #[doc = "PIN12: Pin 12<br>"]
    pub(crate) fn p0_outclr50c_pin12_read(&self) -> MemResult<bool> {
        Ok(self.get_output(12))
    }
    #[doc = "PIN12: Pin 12<br>"]
    pub fn p0_outclr50c_pin12_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(12, false);
        }
        Ok(())
    }
    #[doc = "PIN13: Pin 13<br>"]
    pub(crate) fn p0_outclr50c_pin13_read(&self) -> MemResult<bool> {
        Ok(self.get_output(13))
    }
    #[doc = "PIN13: Pin 13<br>"]
    pub fn p0_outclr50c_pin13_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(13, false);
        }
        Ok(())
    }
    #[doc = "PIN14: Pin 14<br>"]
    pub(crate) fn p0_outclr50c_pin14_read(&self) -> MemResult<bool> {
        Ok(self.get_output(14))
    }
    #[doc = "PIN14: Pin 14<br>"]
    pub fn p0_outclr50c_pin14_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(14, false);
        }
        Ok(())
    }
    #[doc = "PIN15: Pin 15<br>"]
    pub(crate) fn p0_outclr50c_pin15_read(&self) -> MemResult<bool> {
        Ok(self.get_output(15))
    }
    #[doc = "PIN15: Pin 15<br>"]
    pub fn p0_outclr50c_pin15_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(15, false);
        }
        Ok(())
    }
    #[doc = "PIN16: Pin 16<br>"]
    pub(crate) fn p0_outclr50c_pin16_read(&self) -> MemResult<bool> {
        Ok(self.get_output(16))
    }
    #[doc = "PIN16: Pin 16<br>"]
    pub fn p0_outclr50c_pin16_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(16, false);
        }
        Ok(())
    }
    #[doc = "PIN17: Pin 17<br>"]
    pub(crate) fn p0_outclr50c_pin17_read(&self) -> MemResult<bool> {
        Ok(self.get_output(17))
    }
    #[doc = "PIN17: Pin 17<br>"]
    pub fn p0_outclr50c_pin17_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(17, false);
        }
        Ok(())
    }
    #[doc = "PIN18: Pin 18<br>"]
    pub(crate) fn p0_outclr50c_pin18_read(&self) -> MemResult<bool> {
        Ok(self.get_output(18))
    }
    #[doc = "PIN18: Pin 18<br>"]
    pub fn p0_outclr50c_pin18_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(18, false);
        }
        Ok(())
    }
    #[doc = "PIN19: Pin 19<br>"]
    pub(crate) fn p0_outclr50c_pin19_read(&self) -> MemResult<bool> {
        Ok(self.get_output(19))
    }
    #[doc = "PIN19: Pin 19<br>"]
    pub fn p0_outclr50c_pin19_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(19, false);
        }
        Ok(())
    }
    #[doc = "PIN20: Pin 20<br>"]
    pub(crate) fn p0_outclr50c_pin20_read(&self) -> MemResult<bool> {
        Ok(self.get_output(20))
    }
    #[doc = "PIN20: Pin 20<br>"]
    pub fn p0_outclr50c_pin20_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(20, false);
        }
        Ok(())
    }
    #[doc = "PIN21: Pin 21<br>"]
    pub(crate) fn p0_outclr50c_pin21_read(&self) -> MemResult<bool> {
        Ok(self.get_output(21))
    }
    #[doc = "PIN21: Pin 21<br>"]
    pub fn p0_outclr50c_pin21_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(21, false);
        }
        Ok(())
    }
    #[doc = "PIN22: Pin 22<br>"]
    pub(crate) fn p0_outclr50c_pin22_read(&self) -> MemResult<bool> {
        Ok(self.get_output(22))
    }
    #[doc = "PIN22: Pin 22<br>"]
    pub fn p0_outclr50c_pin22_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(22, false);
        }
        Ok(())
    }
    #[doc = "PIN23: Pin 23<br>"]
    pub(crate) fn p0_outclr50c_pin23_read(&self) -> MemResult<bool> {
        Ok(self.get_output(23))
    }
    #[doc = "PIN23: Pin 23<br>"]
    pub fn p0_outclr50c_pin23_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(23, false);
        }
        Ok(())
    }
    #[doc = "PIN24: Pin 24<br>"]
    pub(crate) fn p0_outclr50c_pin24_read(&self) -> MemResult<bool> {
        Ok(self.get_output(24))
    }
    #[doc = "PIN24: Pin 24<br>"]
    pub fn p0_outclr50c_pin24_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(24, false);
        }
        Ok(())
    }
    #[doc = "PIN25: Pin 25<br>"]
    pub(crate) fn p0_outclr50c_pin25_read(&self) -> MemResult<bool> {
        Ok(self.get_output(25))
    }
    #[doc = "PIN25: Pin 25<br>"]
    pub fn p0_outclr50c_pin25_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(25, false);
        }
        Ok(())
    }
    #[doc = "PIN26: Pin 26<br>"]
    pub(crate) fn p0_outclr50c_pin26_read(&self) -> MemResult<bool> {
        Ok(self.get_output(26))
    }
    #[doc = "PIN26: Pin 26<br>"]
    pub fn p0_outclr50c_pin26_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(26, false);
        }
        Ok(())
    }
    #[doc = "PIN27: Pin 27<br>"]
    pub(crate) fn p0_outclr50c_pin27_read(&self) -> MemResult<bool> {
        Ok(self.get_output(27))
    }
    #[doc = "PIN27: Pin 27<br>"]
    pub fn p0_outclr50c_pin27_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(27, false);
        }
        Ok(())
    }
    #[doc = "PIN28: Pin 28<br>"]
    pub(crate) fn p0_outclr50c_pin28_read(&self) -> MemResult<bool> {
        Ok(self.get_output(28))
    }
    #[doc = "PIN28: Pin 28<br>"]
    pub fn p0_outclr50c_pin28_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(28, false);
        }
        Ok(())
    }
    #[doc = "PIN29: Pin 29<br>"]
    pub(crate) fn p0_outclr50c_pin29_read(&self) -> MemResult<bool> {
        Ok(self.get_output(29))
    }
    #[doc = "PIN29: Pin 29<br>"]
    pub fn p0_outclr50c_pin29_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(29, false);
        }
        Ok(())
    }
    #[doc = "PIN30: Pin 30<br>"]
    pub(crate) fn p0_outclr50c_pin30_read(&self) -> MemResult<bool> {
        Ok(self.get_output(30))
    }
    #[doc = "PIN30: Pin 30<br>"]
    pub fn p0_outclr50c_pin30_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(30, false);
        }
        Ok(())
    }
    #[doc = "PIN31: Pin 31<br>"]
    pub(crate) fn p0_outclr50c_pin31_read(&self) -> MemResult<bool> {
        Ok(self.get_output(31))
    }
    #[doc = "PIN31: Pin 31<br>"]
    pub fn p0_outclr50c_pin31_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output(31, false);
        }
        Ok(())
    }
    #[doc = "PIN0: Pin 0<br>"]
    pub(crate) fn p0_in510_pin0_read(&self) -> MemResult<bool> {
        Ok(self.read_input(0))
    }
    #[doc = "PIN1: Pin 1<br>"]
    pub(crate) fn p0_in510_pin1_read(&self) -> MemResult<bool> {
        Ok(self.read_input(1))
    }
    #[doc = "PIN2: Pin 2<br>"]
    pub(crate) fn p0_in510_pin2_read(&self) -> MemResult<bool> {
        Ok(self.read_input(2))
    }
    #[doc = "PIN3: Pin 3<br>"]
    pub(crate) fn p0_in510_pin3_read(&self) -> MemResult<bool> {
        Ok(self.read_input(3))
    }
    #[doc = "PIN4: Pin 4<br>"]
    pub(crate) fn p0_in510_pin4_read(&self) -> MemResult<bool> {
        Ok(self.read_input(4))
    }
    #[doc = "PIN5: Pin 5<br>"]
    pub(crate) fn p0_in510_pin5_read(&self) -> MemResult<bool> {
        Ok(self.read_input(5))
    }
    #[doc = "PIN6: Pin 6<br>"]
    pub(crate) fn p0_in510_pin6_read(&self) -> MemResult<bool> {
        Ok(self.read_input(6))
    }
    #[doc = "PIN7: Pin 7<br>"]
    pub(crate) fn p0_in510_pin7_read(&self) -> MemResult<bool> {
        Ok(self.read_input(7))
    }
    #[doc = "PIN8: Pin 8<br>"]
    pub(crate) fn p0_in510_pin8_read(&self) -> MemResult<bool> {
        Ok(self.read_input(8))
    }
    #[doc = "PIN9: Pin 9<br>"]
    pub(crate) fn p0_in510_pin9_read(&self) -> MemResult<bool> {
        Ok(self.read_input(9))
    }
    #[doc = "PIN10: Pin 10<br>"]
    pub(crate) fn p0_in510_pin10_read(&self) -> MemResult<bool> {
        Ok(self.read_input(10))
    }
    #[doc = "PIN11: Pin 11<br>"]
    pub(crate) fn p0_in510_pin11_read(&self) -> MemResult<bool> {
        Ok(self.read_input(11))
    }
    #[doc = "PIN12: Pin 12<br>"]
    pub(crate) fn p0_in510_pin12_read(&self) -> MemResult<bool> {
        Ok(self.read_input(12))
    }
    #[doc = "PIN13: Pin 13<br>"]
    pub(crate) fn p0_in510_pin13_read(&self) -> MemResult<bool> {
        Ok(self.read_input(13))
    }
    #[doc = "PIN14: Pin 14<br>"]
    pub(crate) fn p0_in510_pin14_read(&self) -> MemResult<bool> {
        Ok(self.read_input(14))
    }
    #[doc = "PIN15: Pin 15<br>"]
    pub(crate) fn p0_in510_pin15_read(&self) -> MemResult<bool> {
        Ok(self.read_input(15))
    }
    #[doc = "PIN16: Pin 16<br>"]
    pub(crate) fn p0_in510_pin16_read(&self) -> MemResult<bool> {
        Ok(self.read_input(16))
    }
    #[doc = "PIN17: Pin 17<br>"]
    pub(crate) fn p0_in510_pin17_read(&self) -> MemResult<bool> {
        Ok(self.read_input(17))
    }
    #[doc = "PIN18: Pin 18<br>"]
    pub(crate) fn p0_in510_pin18_read(&self) -> MemResult<bool> {
        Ok(self.read_input(18))
    }
    #[doc = "PIN19: Pin 19<br>"]
    pub(crate) fn p0_in510_pin19_read(&self) -> MemResult<bool> {
        Ok(self.read_input(19))
    }
    #[doc = "PIN20: Pin 20<br>"]
    pub(crate) fn p0_in510_pin20_read(&self) -> MemResult<bool> {
        Ok(self.read_input(20))
    }
    #[doc = "PIN21: Pin 21<br>"]
    pub(crate) fn p0_in510_pin21_read(&self) -> MemResult<bool> {
        Ok(self.read_input(21))
    }
    #[doc = "PIN22: Pin 22<br>"]
    pub(crate) fn p0_in510_pin22_read(&self) -> MemResult<bool> {
        Ok(self.read_input(22))
    }
    #[doc = "PIN23: Pin 23<br>"]
    pub(crate) fn p0_in510_pin23_read(&self) -> MemResult<bool> {
        Ok(self.read_input(23))
    }
    #[doc = "PIN24: Pin 24<br>"]
    pub(crate) fn p0_in510_pin24_read(&self) -> MemResult<bool> {
        Ok(self.read_input(24))
    }
    #[doc = "PIN25: Pin 25<br>"]
    pub(crate) fn p0_in510_pin25_read(&self) -> MemResult<bool> {
        Ok(self.read_input(25))
    }
    #[doc = "PIN26: Pin 26<br>"]
    pub(crate) fn p0_in510_pin26_read(&self) -> MemResult<bool> {
        Ok(self.read_input(26))
    }
    #[doc = "PIN27: Pin 27<br>"]
    pub(crate) fn p0_in510_pin27_read(&self) -> MemResult<bool> {
        Ok(self.read_input(27))
    }
    #[doc = "PIN28: Pin 28<br>"]
    pub(crate) fn p0_in510_pin28_read(&self) -> MemResult<bool> {
        Ok(self.read_input(28))
    }
    #[doc = "PIN29: Pin 29<br>"]
    pub(crate) fn p0_in510_pin29_read(&self) -> MemResult<bool> {
        Ok(self.read_input(29))
    }
    #[doc = "PIN30: Pin 30<br>"]
    pub(crate) fn p0_in510_pin30_read(&self) -> MemResult<bool> {
        Ok(self.read_input(30))
    }
    #[doc = "PIN31: Pin 31<br>"]
    pub(crate) fn p0_in510_pin31_read(&self) -> MemResult<bool> {
        Ok(self.read_input(31))
    }
    #[doc = "PIN0: Pin 0<br>"]
    pub(crate) fn p0_dir514_pin0_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(0))
    }
    #[doc = "PIN0: Pin 0<br>"]
    pub fn p0_dir514_pin0_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output_mode(0, _value))
    }
    #[doc = "PIN1: Pin 1<br>"]
    pub(crate) fn p0_dir514_pin1_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(1))
    }
    #[doc = "PIN1: Pin 1<br>"]
    pub fn p0_dir514_pin1_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output_mode(1, _value))
    }
    #[doc = "PIN2: Pin 2<br>"]
    pub(crate) fn p0_dir514_pin2_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(2))
    }
    #[doc = "PIN2: Pin 2<br>"]
    pub fn p0_dir514_pin2_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output_mode(2, _value))
    }
    #[doc = "PIN3: Pin 3<br>"]
    pub(crate) fn p0_dir514_pin3_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(3))
    }
    #[doc = "PIN3: Pin 3<br>"]
    pub fn p0_dir514_pin3_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output_mode(3, _value))
    }
    #[doc = "PIN4: Pin 4<br>"]
    pub(crate) fn p0_dir514_pin4_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(4))
    }
    #[doc = "PIN4: Pin 4<br>"]
    pub fn p0_dir514_pin4_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output_mode(4, _value))
    }
    #[doc = "PIN5: Pin 5<br>"]
    pub(crate) fn p0_dir514_pin5_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(5))
    }
    #[doc = "PIN5: Pin 5<br>"]
    pub fn p0_dir514_pin5_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output_mode(5, _value))
    }
    #[doc = "PIN6: Pin 6<br>"]
    pub(crate) fn p0_dir514_pin6_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(6))
    }
    #[doc = "PIN6: Pin 6<br>"]
    pub fn p0_dir514_pin6_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output_mode(6, _value))
    }
    #[doc = "PIN7: Pin 7<br>"]
    pub(crate) fn p0_dir514_pin7_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(7))
    }
    #[doc = "PIN7: Pin 7<br>"]
    pub fn p0_dir514_pin7_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output_mode(7, _value))
    }
    #[doc = "PIN8: Pin 8<br>"]
    pub(crate) fn p0_dir514_pin8_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(8))
    }
    #[doc = "PIN8: Pin 8<br>"]
    pub fn p0_dir514_pin8_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output_mode(8, _value))
    }
    #[doc = "PIN9: Pin 9<br>"]
    pub(crate) fn p0_dir514_pin9_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(9))
    }
    #[doc = "PIN9: Pin 9<br>"]
    pub fn p0_dir514_pin9_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output_mode(9, _value))
    }
    #[doc = "PIN10: Pin 10<br>"]
    pub(crate) fn p0_dir514_pin10_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(10))
    }
    #[doc = "PIN10: Pin 10<br>"]
    pub fn p0_dir514_pin10_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output_mode(10, _value))
    }
    #[doc = "PIN11: Pin 11<br>"]
    pub(crate) fn p0_dir514_pin11_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(11))
    }
    #[doc = "PIN11: Pin 11<br>"]
    pub fn p0_dir514_pin11_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output_mode(11, _value))
    }
    #[doc = "PIN12: Pin 12<br>"]
    pub(crate) fn p0_dir514_pin12_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(12))
    }
    #[doc = "PIN12: Pin 12<br>"]
    pub fn p0_dir514_pin12_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output_mode(12, _value))
    }
    #[doc = "PIN13: Pin 13<br>"]
    pub(crate) fn p0_dir514_pin13_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(13))
    }
    #[doc = "PIN13: Pin 13<br>"]
    pub fn p0_dir514_pin13_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output_mode(13, _value))
    }
    #[doc = "PIN14: Pin 14<br>"]
    pub(crate) fn p0_dir514_pin14_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(14))
    }
    #[doc = "PIN14: Pin 14<br>"]
    pub fn p0_dir514_pin14_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output_mode(14, _value))
    }
    #[doc = "PIN15: Pin 15<br>"]
    pub(crate) fn p0_dir514_pin15_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(15))
    }
    #[doc = "PIN15: Pin 15<br>"]
    pub fn p0_dir514_pin15_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output_mode(15, _value))
    }
    #[doc = "PIN16: Pin 16<br>"]
    pub(crate) fn p0_dir514_pin16_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(16))
    }
    #[doc = "PIN16: Pin 16<br>"]
    pub fn p0_dir514_pin16_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output_mode(16, _value))
    }
    #[doc = "PIN17: Pin 17<br>"]
    pub(crate) fn p0_dir514_pin17_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(17))
    }
    #[doc = "PIN17: Pin 17<br>"]
    pub fn p0_dir514_pin17_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output_mode(17, _value))
    }
    #[doc = "PIN18: Pin 18<br>"]
    pub(crate) fn p0_dir514_pin18_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(18))
    }
    #[doc = "PIN18: Pin 18<br>"]
    pub fn p0_dir514_pin18_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output_mode(18, _value))
    }
    #[doc = "PIN19: Pin 19<br>"]
    pub(crate) fn p0_dir514_pin19_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(19))
    }
    #[doc = "PIN19: Pin 19<br>"]
    pub fn p0_dir514_pin19_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output_mode(19, _value))
    }
    #[doc = "PIN20: Pin 20<br>"]
    pub(crate) fn p0_dir514_pin20_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(20))
    }
    #[doc = "PIN20: Pin 20<br>"]
    pub fn p0_dir514_pin20_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output_mode(20, _value))
    }
    #[doc = "PIN21: Pin 21<br>"]
    pub(crate) fn p0_dir514_pin21_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(21))
    }
    #[doc = "PIN21: Pin 21<br>"]
    pub fn p0_dir514_pin21_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output_mode(21, _value))
    }
    #[doc = "PIN22: Pin 22<br>"]
    pub(crate) fn p0_dir514_pin22_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(22))
    }
    #[doc = "PIN22: Pin 22<br>"]
    pub fn p0_dir514_pin22_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output_mode(22, _value))
    }
    #[doc = "PIN23: Pin 23<br>"]
    pub(crate) fn p0_dir514_pin23_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(23))
    }
    #[doc = "PIN23: Pin 23<br>"]
    pub fn p0_dir514_pin23_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output_mode(23, _value))
    }
    #[doc = "PIN24: Pin 24<br>"]
    pub(crate) fn p0_dir514_pin24_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(24))
    }
    #[doc = "PIN24: Pin 24<br>"]
    pub fn p0_dir514_pin24_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output_mode(24, _value))
    }
    #[doc = "PIN25: Pin 25<br>"]
    pub(crate) fn p0_dir514_pin25_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(25))
    }
    #[doc = "PIN25: Pin 25<br>"]
    pub fn p0_dir514_pin25_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output_mode(25, _value))
    }
    #[doc = "PIN26: Pin 26<br>"]
    pub(crate) fn p0_dir514_pin26_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(26))
    }
    #[doc = "PIN26: Pin 26<br>"]
    pub fn p0_dir514_pin26_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output_mode(26, _value))
    }
    #[doc = "PIN27: Pin 27<br>"]
    pub(crate) fn p0_dir514_pin27_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(27))
    }
    #[doc = "PIN27: Pin 27<br>"]
    pub fn p0_dir514_pin27_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output_mode(27, _value))
    }
    #[doc = "PIN28: Pin 28<br>"]
    pub(crate) fn p0_dir514_pin28_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(28))
    }
    #[doc = "PIN28: Pin 28<br>"]
    pub fn p0_dir514_pin28_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output_mode(28, _value))
    }
    #[doc = "PIN29: Pin 29<br>"]
    pub(crate) fn p0_dir514_pin29_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(29))
    }
    #[doc = "PIN29: Pin 29<br>"]
    pub fn p0_dir514_pin29_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output_mode(29, _value))
    }
    #[doc = "PIN30: Pin 30<br>"]
    pub(crate) fn p0_dir514_pin30_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(30))
    }
    #[doc = "PIN30: Pin 30<br>"]
    pub fn p0_dir514_pin30_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output_mode(30, _value))
    }
    #[doc = "PIN31: Pin 31<br>"]
    pub(crate) fn p0_dir514_pin31_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(31))
    }
    #[doc = "PIN31: Pin 31<br>"]
    pub fn p0_dir514_pin31_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_output_mode(31, _value))
    }
    #[doc = "PIN0: Set as output pin 0<br>"]
    pub(crate) fn p0_dirset518_pin0_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(0))
    }
    #[doc = "PIN0: Set as output pin 0<br>"]
    pub fn p0_dirset518_pin0_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(0, true);
        }
        Ok(())
    }
    #[doc = "PIN1: Set as output pin 1<br>"]
    pub(crate) fn p0_dirset518_pin1_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(1))
    }
    #[doc = "PIN1: Set as output pin 1<br>"]
    pub fn p0_dirset518_pin1_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(1, true);
        }
        Ok(())
    }
    #[doc = "PIN2: Set as output pin 2<br>"]
    pub(crate) fn p0_dirset518_pin2_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(2))
    }
    #[doc = "PIN2: Set as output pin 2<br>"]
    pub fn p0_dirset518_pin2_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(2, true);
        }
        Ok(())
    }
    #[doc = "PIN3: Set as output pin 3<br>"]
    pub(crate) fn p0_dirset518_pin3_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(3))
    }
    #[doc = "PIN3: Set as output pin 3<br>"]
    pub fn p0_dirset518_pin3_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(3, true);
        }
        Ok(())
    }
    #[doc = "PIN4: Set as output pin 4<br>"]
    pub(crate) fn p0_dirset518_pin4_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(4))
    }
    #[doc = "PIN4: Set as output pin 4<br>"]
    pub fn p0_dirset518_pin4_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(4, true);
        }
        Ok(())
    }
    #[doc = "PIN5: Set as output pin 5<br>"]
    pub(crate) fn p0_dirset518_pin5_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(5))
    }
    #[doc = "PIN5: Set as output pin 5<br>"]
    pub fn p0_dirset518_pin5_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(5, true);
        }
        Ok(())
    }
    #[doc = "PIN6: Set as output pin 6<br>"]
    pub(crate) fn p0_dirset518_pin6_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(6))
    }
    #[doc = "PIN6: Set as output pin 6<br>"]
    pub fn p0_dirset518_pin6_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(6, true);
        }
        Ok(())
    }
    #[doc = "PIN7: Set as output pin 7<br>"]
    pub(crate) fn p0_dirset518_pin7_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(7))
    }
    #[doc = "PIN7: Set as output pin 7<br>"]
    pub fn p0_dirset518_pin7_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(7, true);
        }
        Ok(())
    }
    #[doc = "PIN8: Set as output pin 8<br>"]
    pub(crate) fn p0_dirset518_pin8_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(8))
    }
    #[doc = "PIN8: Set as output pin 8<br>"]
    pub fn p0_dirset518_pin8_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(8, true);
        }
        Ok(())
    }
    #[doc = "PIN9: Set as output pin 9<br>"]
    pub(crate) fn p0_dirset518_pin9_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(9))
    }
    #[doc = "PIN9: Set as output pin 9<br>"]
    pub fn p0_dirset518_pin9_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(9, true);
        }
        Ok(())
    }
    #[doc = "PIN10: Set as output pin 10<br>"]
    pub(crate) fn p0_dirset518_pin10_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(10))
    }
    #[doc = "PIN10: Set as output pin 10<br>"]
    pub fn p0_dirset518_pin10_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(10, true);
        }
        Ok(())
    }
    #[doc = "PIN11: Set as output pin 11<br>"]
    pub(crate) fn p0_dirset518_pin11_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(11))
    }
    #[doc = "PIN11: Set as output pin 11<br>"]
    pub fn p0_dirset518_pin11_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(11, true);
        }
        Ok(())
    }
    #[doc = "PIN12: Set as output pin 12<br>"]
    pub(crate) fn p0_dirset518_pin12_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(12))
    }
    #[doc = "PIN12: Set as output pin 12<br>"]
    pub fn p0_dirset518_pin12_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(12, true);
        }
        Ok(())
    }
    #[doc = "PIN13: Set as output pin 13<br>"]
    pub(crate) fn p0_dirset518_pin13_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(13))
    }
    #[doc = "PIN13: Set as output pin 13<br>"]
    pub fn p0_dirset518_pin13_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(13, true);
        }
        Ok(())
    }
    #[doc = "PIN14: Set as output pin 14<br>"]
    pub(crate) fn p0_dirset518_pin14_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(14))
    }
    #[doc = "PIN14: Set as output pin 14<br>"]
    pub fn p0_dirset518_pin14_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(14, true);
        }
        Ok(())
    }
    #[doc = "PIN15: Set as output pin 15<br>"]
    pub(crate) fn p0_dirset518_pin15_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(15))
    }
    #[doc = "PIN15: Set as output pin 15<br>"]
    pub fn p0_dirset518_pin15_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(15, true);
        }
        Ok(())
    }
    #[doc = "PIN16: Set as output pin 16<br>"]
    pub(crate) fn p0_dirset518_pin16_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(16))
    }
    #[doc = "PIN16: Set as output pin 16<br>"]
    pub fn p0_dirset518_pin16_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(16, true);
        }
        Ok(())
    }
    #[doc = "PIN17: Set as output pin 17<br>"]
    pub(crate) fn p0_dirset518_pin17_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(17))
    }
    #[doc = "PIN17: Set as output pin 17<br>"]
    pub fn p0_dirset518_pin17_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(17, true);
        }
        Ok(())
    }
    #[doc = "PIN18: Set as output pin 18<br>"]
    pub(crate) fn p0_dirset518_pin18_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(18))
    }
    #[doc = "PIN18: Set as output pin 18<br>"]
    pub fn p0_dirset518_pin18_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(18, true);
        }
        Ok(())
    }
    #[doc = "PIN19: Set as output pin 19<br>"]
    pub(crate) fn p0_dirset518_pin19_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(19))
    }
    #[doc = "PIN19: Set as output pin 19<br>"]
    pub fn p0_dirset518_pin19_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(19, true);
        }
        Ok(())
    }
    #[doc = "PIN20: Set as output pin 20<br>"]
    pub(crate) fn p0_dirset518_pin20_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(20))
    }
    #[doc = "PIN20: Set as output pin 20<br>"]
    pub fn p0_dirset518_pin20_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(20, true);
        }
        Ok(())
    }
    #[doc = "PIN21: Set as output pin 21<br>"]
    pub(crate) fn p0_dirset518_pin21_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(21))
    }
    #[doc = "PIN21: Set as output pin 21<br>"]
    pub fn p0_dirset518_pin21_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(21, true);
        }
        Ok(())
    }
    #[doc = "PIN22: Set as output pin 22<br>"]
    pub(crate) fn p0_dirset518_pin22_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(22))
    }
    #[doc = "PIN22: Set as output pin 22<br>"]
    pub fn p0_dirset518_pin22_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(22, true);
        }
        Ok(())
    }
    #[doc = "PIN23: Set as output pin 23<br>"]
    pub(crate) fn p0_dirset518_pin23_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(23))
    }
    #[doc = "PIN23: Set as output pin 23<br>"]
    pub fn p0_dirset518_pin23_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(23, true);
        }
        Ok(())
    }
    #[doc = "PIN24: Set as output pin 24<br>"]
    pub(crate) fn p0_dirset518_pin24_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(24))
    }
    #[doc = "PIN24: Set as output pin 24<br>"]
    pub fn p0_dirset518_pin24_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(24, true);
        }
        Ok(())
    }
    #[doc = "PIN25: Set as output pin 25<br>"]
    pub(crate) fn p0_dirset518_pin25_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(25))
    }
    #[doc = "PIN25: Set as output pin 25<br>"]
    pub fn p0_dirset518_pin25_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(25, true);
        }
        Ok(())
    }
    #[doc = "PIN26: Set as output pin 26<br>"]
    pub(crate) fn p0_dirset518_pin26_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(26))
    }
    #[doc = "PIN26: Set as output pin 26<br>"]
    pub fn p0_dirset518_pin26_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(26, true);
        }
        Ok(())
    }
    #[doc = "PIN27: Set as output pin 27<br>"]
    pub(crate) fn p0_dirset518_pin27_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(27))
    }
    #[doc = "PIN27: Set as output pin 27<br>"]
    pub fn p0_dirset518_pin27_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(27, true);
        }
        Ok(())
    }
    #[doc = "PIN28: Set as output pin 28<br>"]
    pub(crate) fn p0_dirset518_pin28_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(28))
    }
    #[doc = "PIN28: Set as output pin 28<br>"]
    pub fn p0_dirset518_pin28_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(28, true);
        }
        Ok(())
    }
    #[doc = "PIN29: Set as output pin 29<br>"]
    pub(crate) fn p0_dirset518_pin29_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(29))
    }
    #[doc = "PIN29: Set as output pin 29<br>"]
    pub fn p0_dirset518_pin29_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(29, true);
        }
        Ok(())
    }
    #[doc = "PIN30: Set as output pin 30<br>"]
    pub(crate) fn p0_dirset518_pin30_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(30))
    }
    #[doc = "PIN30: Set as output pin 30<br>"]
    pub fn p0_dirset518_pin30_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(30, true);
        }
        Ok(())
    }
    #[doc = "PIN31: Set as output pin 31<br>"]
    pub(crate) fn p0_dirset518_pin31_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(31))
    }
    #[doc = "PIN31: Set as output pin 31<br>"]
    pub fn p0_dirset518_pin31_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(31, true);
        }
        Ok(())
    }
    #[doc = "PIN0: Set as input pin 0<br>"]
    pub(crate) fn p0_dirclr51c_pin0_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(0))
    }
    #[doc = "PIN0: Set as input pin 0<br>"]
    pub fn p0_dirclr51c_pin0_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(0, false);
        }
        Ok(())
    }
    #[doc = "PIN1: Set as input pin 1<br>"]
    pub(crate) fn p0_dirclr51c_pin1_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(1))
    }
    #[doc = "PIN1: Set as input pin 1<br>"]
    pub fn p0_dirclr51c_pin1_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(1, false);
        }
        Ok(())
    }
    #[doc = "PIN2: Set as input pin 2<br>"]
    pub(crate) fn p0_dirclr51c_pin2_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(2))
    }
    #[doc = "PIN2: Set as input pin 2<br>"]
    pub fn p0_dirclr51c_pin2_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(2, false);
        }
        Ok(())
    }
    #[doc = "PIN3: Set as input pin 3<br>"]
    pub(crate) fn p0_dirclr51c_pin3_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(3))
    }
    #[doc = "PIN3: Set as input pin 3<br>"]
    pub fn p0_dirclr51c_pin3_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(3, false);
        }
        Ok(())
    }
    #[doc = "PIN4: Set as input pin 4<br>"]
    pub(crate) fn p0_dirclr51c_pin4_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(4))
    }
    #[doc = "PIN4: Set as input pin 4<br>"]
    pub fn p0_dirclr51c_pin4_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(4, false);
        }
        Ok(())
    }
    #[doc = "PIN5: Set as input pin 5<br>"]
    pub(crate) fn p0_dirclr51c_pin5_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(5))
    }
    #[doc = "PIN5: Set as input pin 5<br>"]
    pub fn p0_dirclr51c_pin5_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(5, false);
        }
        Ok(())
    }
    #[doc = "PIN6: Set as input pin 6<br>"]
    pub(crate) fn p0_dirclr51c_pin6_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(6))
    }
    #[doc = "PIN6: Set as input pin 6<br>"]
    pub fn p0_dirclr51c_pin6_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(6, false);
        }
        Ok(())
    }
    #[doc = "PIN7: Set as input pin 7<br>"]
    pub(crate) fn p0_dirclr51c_pin7_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(7))
    }
    #[doc = "PIN7: Set as input pin 7<br>"]
    pub fn p0_dirclr51c_pin7_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(7, false);
        }
        Ok(())
    }
    #[doc = "PIN8: Set as input pin 8<br>"]
    pub(crate) fn p0_dirclr51c_pin8_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(8))
    }
    #[doc = "PIN8: Set as input pin 8<br>"]
    pub fn p0_dirclr51c_pin8_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(8, false);
        }
        Ok(())
    }
    #[doc = "PIN9: Set as input pin 9<br>"]
    pub(crate) fn p0_dirclr51c_pin9_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(9))
    }
    #[doc = "PIN9: Set as input pin 9<br>"]
    pub fn p0_dirclr51c_pin9_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(9, false);
        }
        Ok(())
    }
    #[doc = "PIN10: Set as input pin 10<br>"]
    pub(crate) fn p0_dirclr51c_pin10_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(10))
    }
    #[doc = "PIN10: Set as input pin 10<br>"]
    pub fn p0_dirclr51c_pin10_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(10, false);
        }
        Ok(())
    }
    #[doc = "PIN11: Set as input pin 11<br>"]
    pub(crate) fn p0_dirclr51c_pin11_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(11))
    }
    #[doc = "PIN11: Set as input pin 11<br>"]
    pub fn p0_dirclr51c_pin11_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(11, false);
        }
        Ok(())
    }
    #[doc = "PIN12: Set as input pin 12<br>"]
    pub(crate) fn p0_dirclr51c_pin12_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(12))
    }
    #[doc = "PIN12: Set as input pin 12<br>"]
    pub fn p0_dirclr51c_pin12_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(12, false);
        }
        Ok(())
    }
    #[doc = "PIN13: Set as input pin 13<br>"]
    pub(crate) fn p0_dirclr51c_pin13_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(13))
    }
    #[doc = "PIN13: Set as input pin 13<br>"]
    pub fn p0_dirclr51c_pin13_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(13, false);
        }
        Ok(())
    }
    #[doc = "PIN14: Set as input pin 14<br>"]
    pub(crate) fn p0_dirclr51c_pin14_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(14))
    }
    #[doc = "PIN14: Set as input pin 14<br>"]
    pub fn p0_dirclr51c_pin14_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(14, false);
        }
        Ok(())
    }
    #[doc = "PIN15: Set as input pin 15<br>"]
    pub(crate) fn p0_dirclr51c_pin15_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(15))
    }
    #[doc = "PIN15: Set as input pin 15<br>"]
    pub fn p0_dirclr51c_pin15_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(15, false);
        }
        Ok(())
    }
    #[doc = "PIN16: Set as input pin 16<br>"]
    pub(crate) fn p0_dirclr51c_pin16_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(16))
    }
    #[doc = "PIN16: Set as input pin 16<br>"]
    pub fn p0_dirclr51c_pin16_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(16, false);
        }
        Ok(())
    }
    #[doc = "PIN17: Set as input pin 17<br>"]
    pub(crate) fn p0_dirclr51c_pin17_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(17))
    }
    #[doc = "PIN17: Set as input pin 17<br>"]
    pub fn p0_dirclr51c_pin17_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(17, false);
        }
        Ok(())
    }
    #[doc = "PIN18: Set as input pin 18<br>"]
    pub(crate) fn p0_dirclr51c_pin18_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(18))
    }
    #[doc = "PIN18: Set as input pin 18<br>"]
    pub fn p0_dirclr51c_pin18_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(18, false);
        }
        Ok(())
    }
    #[doc = "PIN19: Set as input pin 19<br>"]
    pub(crate) fn p0_dirclr51c_pin19_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(19))
    }
    #[doc = "PIN19: Set as input pin 19<br>"]
    pub fn p0_dirclr51c_pin19_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(19, false);
        }
        Ok(())
    }
    #[doc = "PIN20: Set as input pin 20<br>"]
    pub(crate) fn p0_dirclr51c_pin20_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(20))
    }
    #[doc = "PIN20: Set as input pin 20<br>"]
    pub fn p0_dirclr51c_pin20_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(20, false);
        }
        Ok(())
    }
    #[doc = "PIN21: Set as input pin 21<br>"]
    pub(crate) fn p0_dirclr51c_pin21_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(21))
    }
    #[doc = "PIN21: Set as input pin 21<br>"]
    pub fn p0_dirclr51c_pin21_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(21, false);
        }
        Ok(())
    }
    #[doc = "PIN22: Set as input pin 22<br>"]
    pub(crate) fn p0_dirclr51c_pin22_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(22))
    }
    #[doc = "PIN22: Set as input pin 22<br>"]
    pub fn p0_dirclr51c_pin22_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(22, false);
        }
        Ok(())
    }
    #[doc = "PIN23: Set as input pin 23<br>"]
    pub(crate) fn p0_dirclr51c_pin23_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(23))
    }
    #[doc = "PIN23: Set as input pin 23<br>"]
    pub fn p0_dirclr51c_pin23_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(23, false);
        }
        Ok(())
    }
    #[doc = "PIN24: Set as input pin 24<br>"]
    pub(crate) fn p0_dirclr51c_pin24_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(24))
    }
    #[doc = "PIN24: Set as input pin 24<br>"]
    pub fn p0_dirclr51c_pin24_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(24, false);
        }
        Ok(())
    }
    #[doc = "PIN25: Set as input pin 25<br>"]
    pub(crate) fn p0_dirclr51c_pin25_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(25))
    }
    #[doc = "PIN25: Set as input pin 25<br>"]
    pub fn p0_dirclr51c_pin25_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(25, false);
        }
        Ok(())
    }
    #[doc = "PIN26: Set as input pin 26<br>"]
    pub(crate) fn p0_dirclr51c_pin26_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(26))
    }
    #[doc = "PIN26: Set as input pin 26<br>"]
    pub fn p0_dirclr51c_pin26_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(26, false);
        }
        Ok(())
    }
    #[doc = "PIN27: Set as input pin 27<br>"]
    pub(crate) fn p0_dirclr51c_pin27_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(27))
    }
    #[doc = "PIN27: Set as input pin 27<br>"]
    pub fn p0_dirclr51c_pin27_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(27, false);
        }
        Ok(())
    }
    #[doc = "PIN28: Set as input pin 28<br>"]
    pub(crate) fn p0_dirclr51c_pin28_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(28))
    }
    #[doc = "PIN28: Set as input pin 28<br>"]
    pub fn p0_dirclr51c_pin28_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(28, false);
        }
        Ok(())
    }
    #[doc = "PIN29: Set as input pin 29<br>"]
    pub(crate) fn p0_dirclr51c_pin29_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(29))
    }
    #[doc = "PIN29: Set as input pin 29<br>"]
    pub fn p0_dirclr51c_pin29_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(29, false);
        }
        Ok(())
    }
    #[doc = "PIN30: Set as input pin 30<br>"]
    pub(crate) fn p0_dirclr51c_pin30_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(30))
    }
    #[doc = "PIN30: Set as input pin 30<br>"]
    pub fn p0_dirclr51c_pin30_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(30, false);
        }
        Ok(())
    }
    #[doc = "PIN31: Set as input pin 31<br>"]
    pub(crate) fn p0_dirclr51c_pin31_read(&self) -> MemResult<bool> {
        Ok(self.get_output_mode(31))
    }
    #[doc = "PIN31: Set as input pin 31<br>"]
    pub fn p0_dirclr51c_pin31_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_output_mode(31, false);
        }
        Ok(())
    }
    #[doc = "PIN0: Status on whether PIN0 has met criteria set in PIN_CNF0.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin0_read(&self) -> MemResult<bool> {
        todo!("read PIN0 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN0: Status on whether PIN0 has met criteria set in PIN_CNF0.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN0 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN1: Status on whether PIN1 has met criteria set in PIN_CNF1.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin1_read(&self) -> MemResult<bool> {
        todo!("read PIN1 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN1: Status on whether PIN1 has met criteria set in PIN_CNF1.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN1 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN2: Status on whether PIN2 has met criteria set in PIN_CNF2.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin2_read(&self) -> MemResult<bool> {
        todo!("read PIN2 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN2: Status on whether PIN2 has met criteria set in PIN_CNF2.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin2_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN2 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN3: Status on whether PIN3 has met criteria set in PIN_CNF3.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin3_read(&self) -> MemResult<bool> {
        todo!("read PIN3 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN3: Status on whether PIN3 has met criteria set in PIN_CNF3.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin3_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN3 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN4: Status on whether PIN4 has met criteria set in PIN_CNF4.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin4_read(&self) -> MemResult<bool> {
        todo!("read PIN4 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN4: Status on whether PIN4 has met criteria set in PIN_CNF4.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin4_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN4 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN5: Status on whether PIN5 has met criteria set in PIN_CNF5.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin5_read(&self) -> MemResult<bool> {
        todo!("read PIN5 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN5: Status on whether PIN5 has met criteria set in PIN_CNF5.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin5_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN5 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN6: Status on whether PIN6 has met criteria set in PIN_CNF6.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin6_read(&self) -> MemResult<bool> {
        todo!("read PIN6 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN6: Status on whether PIN6 has met criteria set in PIN_CNF6.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin6_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN6 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN7: Status on whether PIN7 has met criteria set in PIN_CNF7.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin7_read(&self) -> MemResult<bool> {
        todo!("read PIN7 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN7: Status on whether PIN7 has met criteria set in PIN_CNF7.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin7_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN7 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN8: Status on whether PIN8 has met criteria set in PIN_CNF8.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin8_read(&self) -> MemResult<bool> {
        todo!("read PIN8 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN8: Status on whether PIN8 has met criteria set in PIN_CNF8.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin8_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN8 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN9: Status on whether PIN9 has met criteria set in PIN_CNF9.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin9_read(&self) -> MemResult<bool> {
        todo!("read PIN9 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN9: Status on whether PIN9 has met criteria set in PIN_CNF9.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin9_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN9 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN10: Status on whether PIN10 has met criteria set in PIN_CNF10.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin10_read(&self) -> MemResult<bool> {
        todo!("read PIN10 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN10: Status on whether PIN10 has met criteria set in PIN_CNF10.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin10_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN10 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN11: Status on whether PIN11 has met criteria set in PIN_CNF11.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin11_read(&self) -> MemResult<bool> {
        todo!("read PIN11 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN11: Status on whether PIN11 has met criteria set in PIN_CNF11.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin11_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN11 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN12: Status on whether PIN12 has met criteria set in PIN_CNF12.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin12_read(&self) -> MemResult<bool> {
        todo!("read PIN12 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN12: Status on whether PIN12 has met criteria set in PIN_CNF12.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin12_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN12 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN13: Status on whether PIN13 has met criteria set in PIN_CNF13.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin13_read(&self) -> MemResult<bool> {
        todo!("read PIN13 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN13: Status on whether PIN13 has met criteria set in PIN_CNF13.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin13_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN13 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN14: Status on whether PIN14 has met criteria set in PIN_CNF14.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin14_read(&self) -> MemResult<bool> {
        todo!("read PIN14 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN14: Status on whether PIN14 has met criteria set in PIN_CNF14.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin14_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN14 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN15: Status on whether PIN15 has met criteria set in PIN_CNF15.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin15_read(&self) -> MemResult<bool> {
        todo!("read PIN15 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN15: Status on whether PIN15 has met criteria set in PIN_CNF15.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin15_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN15 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN16: Status on whether PIN16 has met criteria set in PIN_CNF16.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin16_read(&self) -> MemResult<bool> {
        todo!("read PIN16 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN16: Status on whether PIN16 has met criteria set in PIN_CNF16.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin16_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN16 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN17: Status on whether PIN17 has met criteria set in PIN_CNF17.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin17_read(&self) -> MemResult<bool> {
        todo!("read PIN17 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN17: Status on whether PIN17 has met criteria set in PIN_CNF17.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin17_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN17 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN18: Status on whether PIN18 has met criteria set in PIN_CNF18.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin18_read(&self) -> MemResult<bool> {
        todo!("read PIN18 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN18: Status on whether PIN18 has met criteria set in PIN_CNF18.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin18_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN18 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN19: Status on whether PIN19 has met criteria set in PIN_CNF19.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin19_read(&self) -> MemResult<bool> {
        todo!("read PIN19 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN19: Status on whether PIN19 has met criteria set in PIN_CNF19.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin19_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN19 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN20: Status on whether PIN20 has met criteria set in PIN_CNF20.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin20_read(&self) -> MemResult<bool> {
        todo!("read PIN20 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN20: Status on whether PIN20 has met criteria set in PIN_CNF20.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin20_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN20 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN21: Status on whether PIN21 has met criteria set in PIN_CNF21.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin21_read(&self) -> MemResult<bool> {
        todo!("read PIN21 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN21: Status on whether PIN21 has met criteria set in PIN_CNF21.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin21_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN21 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN22: Status on whether PIN22 has met criteria set in PIN_CNF22.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin22_read(&self) -> MemResult<bool> {
        todo!("read PIN22 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN22: Status on whether PIN22 has met criteria set in PIN_CNF22.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin22_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN22 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN23: Status on whether PIN23 has met criteria set in PIN_CNF23.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin23_read(&self) -> MemResult<bool> {
        todo!("read PIN23 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN23: Status on whether PIN23 has met criteria set in PIN_CNF23.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin23_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN23 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN24: Status on whether PIN24 has met criteria set in PIN_CNF24.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin24_read(&self) -> MemResult<bool> {
        todo!("read PIN24 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN24: Status on whether PIN24 has met criteria set in PIN_CNF24.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin24_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN24 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN25: Status on whether PIN25 has met criteria set in PIN_CNF25.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin25_read(&self) -> MemResult<bool> {
        todo!("read PIN25 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN25: Status on whether PIN25 has met criteria set in PIN_CNF25.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin25_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN25 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN26: Status on whether PIN26 has met criteria set in PIN_CNF26.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin26_read(&self) -> MemResult<bool> {
        todo!("read PIN26 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN26: Status on whether PIN26 has met criteria set in PIN_CNF26.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin26_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN26 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN27: Status on whether PIN27 has met criteria set in PIN_CNF27.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin27_read(&self) -> MemResult<bool> {
        todo!("read PIN27 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN27: Status on whether PIN27 has met criteria set in PIN_CNF27.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin27_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN27 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN28: Status on whether PIN28 has met criteria set in PIN_CNF28.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin28_read(&self) -> MemResult<bool> {
        todo!("read PIN28 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN28: Status on whether PIN28 has met criteria set in PIN_CNF28.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin28_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN28 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN29: Status on whether PIN29 has met criteria set in PIN_CNF29.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin29_read(&self) -> MemResult<bool> {
        todo!("read PIN29 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN29: Status on whether PIN29 has met criteria set in PIN_CNF29.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin29_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN29 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN30: Status on whether PIN30 has met criteria set in PIN_CNF30.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin30_read(&self) -> MemResult<bool> {
        todo!("read PIN30 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN30: Status on whether PIN30 has met criteria set in PIN_CNF30.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin30_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN30 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN31: Status on whether PIN31 has met criteria set in PIN_CNF31.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin31_read(&self) -> MemResult<bool> {
        todo!("read PIN31 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN31: Status on whether PIN31 has met criteria set in PIN_CNF31.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin31_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN31 mwrite None write None rac None reset value false")
    }
    #[doc = "DETECTMODE: Select between default DETECT signal behaviour and LDETECT mode<br>"]
    pub(crate) fn p0_detectmode524_detectmode_read(&self) -> MemResult<bool> {
        todo!(
            "read DETECTMODE mwrite None write None rac None reset value false"
        )
    }
    #[doc = "DETECTMODE: Select between default DETECT signal behaviour and LDETECT mode<br>"]
    pub(crate) fn p0_detectmode524_detectmode_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write DETECTMODE mwrite None write None rac None reset value false")
    }
    #[doc = "DIR: Pin direction. Same physical register as DIR register<br>"]
    pub(crate) fn p0_pin_cnfn700_dir_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        Ok(self.get_output_mode(_reg_array))
    }
    #[doc = "DIR: Pin direction. Same physical register as DIR register<br>"]
    pub(crate) fn p0_pin_cnfn700_dir_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.set_output_mode(_reg_array, _value))
    }
    #[doc = "INPUT: Connect or disconnect input buffer<br>"]
    pub(crate) fn p0_pin_cnfn700_input_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        Ok(self.get_input_buffer(_reg_array))
    }
    #[doc = "INPUT: Connect or disconnect input buffer<br>"]
    pub(crate) fn p0_pin_cnfn700_input_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.set_input_buffer(_reg_array, _value))
    }
    #[doc = "PULL: Pull configuration<br>"]
    pub(crate) fn p0_pin_cnfn700_pull_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<PinPull> {
        Ok(self.get_pull(_reg_array))
    }
    #[doc = "PULL: Pull configuration<br>"]
    pub(crate) fn p0_pin_cnfn700_pull_write(
        &mut self,
        _reg_array: usize,
        _value: PinPull,
    ) -> MemResult<()> {
        Ok(self.set_pull(_reg_array, _value))
    }
    #[doc = "DRIVE: Drive configuration<br>"]
    pub(crate) fn p0_pin_cnfn700_drive_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<PinDrive> {
        Ok(self.get_drive(_reg_array))
    }
    #[doc = "DRIVE: Drive configuration<br>"]
    pub(crate) fn p0_pin_cnfn700_drive_write(
        &mut self,
        _reg_array: usize,
        _value: PinDrive,
    ) -> MemResult<()> {
        Ok(self.set_drive(_reg_array, _value))
    }
    #[doc = "SENSE: Pin sensing mechanism<br>"]
    pub(crate) fn p0_pin_cnfn700_sense_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<PinSense> {
        Ok(self.get_sense(_reg_array))
    }
    #[doc = "SENSE: Pin sensing mechanism<br>"]
    pub(crate) fn p0_pin_cnfn700_sense_write(
        &mut self,
        _reg_array: usize,
        _value: PinSense,
    ) -> MemResult<()> {
        Ok(self.set_sense(_reg_array, _value))
    }

    #[doc = "TASKS_OUT\\[%s\\]: Description collection\\[0\\]:  Task for writing to pin specified in CONFIG\\[0\\].PSEL. Action on pin is configured in CONFIG\\[0\\].POLARITY.<br>"]
    pub(crate) fn gpiote_tasks_outn0_write(
        &mut self,
        _reg_array: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write gpiote_tasks_outn0 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_SET\\[%s\\]: Description collection\\[0\\]:  Task for writing to pin specified in CONFIG\\[0\\].PSEL. Action on pin is to set it high.<br>"]
    pub(crate) fn gpiote_tasks_setn30_write(
        &mut self,
        _reg_array: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write gpiote_tasks_setn30 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_CLR\\[%s\\]: Description collection\\[0\\]:  Task for writing to pin specified in CONFIG\\[0\\].PSEL. Action on pin is to set it low.<br>"]
    pub(crate) fn gpiote_tasks_clrn60_write(
        &mut self,
        _reg_array: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write gpiote_tasks_clrn60 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_IN\\[%s\\]: Description collection\\[0\\]:  Event generated from pin specified in CONFIG\\[0\\].PSEL<br>"]
    pub(crate) fn gpiote_events_inn100_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<u32> {
        todo ! ("read gpiote_events_inn100 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_IN\\[%s\\]: Description collection\\[0\\]:  Event generated from pin specified in CONFIG\\[0\\].PSEL<br>"]
    pub(crate) fn gpiote_events_inn100_write(
        &mut self,
        _reg_array: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write gpiote_events_inn100 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_PORT: Event generated from multiple input GPIO pins with SENSE mechanism enabled<br>"]
    pub(crate) fn gpiote_events_port17c_read(&self) -> MemResult<u32> {
        todo ! ("read gpiote_events_port17c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_PORT: Event generated from multiple input GPIO pins with SENSE mechanism enabled<br>"]
    pub(crate) fn gpiote_events_port17c_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write gpiote_events_port17c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "IN0: Write '1' to Enable interrupt for IN\\[0\\] event<br>"]
    pub(crate) fn gpiote_intenset304_in0_read(&self) -> MemResult<bool> {
        todo!("read IN0 mwrite None write None rac None reset value false")
    }
    #[doc = "IN0: Write '1' to Enable interrupt for IN\\[0\\] event<br>"]
    pub(crate) fn gpiote_intenset304_in0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write IN0 mwrite None write None rac None reset value false")
    }
    #[doc = "IN1: Write '1' to Enable interrupt for IN\\[1\\] event<br>"]
    pub(crate) fn gpiote_intenset304_in1_read(&self) -> MemResult<bool> {
        todo!("read IN1 mwrite None write None rac None reset value false")
    }
    #[doc = "IN1: Write '1' to Enable interrupt for IN\\[1\\] event<br>"]
    pub(crate) fn gpiote_intenset304_in1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write IN1 mwrite None write None rac None reset value false")
    }
    #[doc = "IN2: Write '1' to Enable interrupt for IN\\[2\\] event<br>"]
    pub(crate) fn gpiote_intenset304_in2_read(&self) -> MemResult<bool> {
        todo!("read IN2 mwrite None write None rac None reset value false")
    }
    #[doc = "IN2: Write '1' to Enable interrupt for IN\\[2\\] event<br>"]
    pub(crate) fn gpiote_intenset304_in2_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write IN2 mwrite None write None rac None reset value false")
    }
    #[doc = "IN3: Write '1' to Enable interrupt for IN\\[3\\] event<br>"]
    pub(crate) fn gpiote_intenset304_in3_read(&self) -> MemResult<bool> {
        todo!("read IN3 mwrite None write None rac None reset value false")
    }
    #[doc = "IN3: Write '1' to Enable interrupt for IN\\[3\\] event<br>"]
    pub(crate) fn gpiote_intenset304_in3_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write IN3 mwrite None write None rac None reset value false")
    }
    #[doc = "IN4: Write '1' to Enable interrupt for IN\\[4\\] event<br>"]
    pub(crate) fn gpiote_intenset304_in4_read(&self) -> MemResult<bool> {
        todo!("read IN4 mwrite None write None rac None reset value false")
    }
    #[doc = "IN4: Write '1' to Enable interrupt for IN\\[4\\] event<br>"]
    pub(crate) fn gpiote_intenset304_in4_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write IN4 mwrite None write None rac None reset value false")
    }
    #[doc = "IN5: Write '1' to Enable interrupt for IN\\[5\\] event<br>"]
    pub(crate) fn gpiote_intenset304_in5_read(&self) -> MemResult<bool> {
        todo!("read IN5 mwrite None write None rac None reset value false")
    }
    #[doc = "IN5: Write '1' to Enable interrupt for IN\\[5\\] event<br>"]
    pub(crate) fn gpiote_intenset304_in5_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write IN5 mwrite None write None rac None reset value false")
    }
    #[doc = "IN6: Write '1' to Enable interrupt for IN\\[6\\] event<br>"]
    pub(crate) fn gpiote_intenset304_in6_read(&self) -> MemResult<bool> {
        todo!("read IN6 mwrite None write None rac None reset value false")
    }
    #[doc = "IN6: Write '1' to Enable interrupt for IN\\[6\\] event<br>"]
    pub(crate) fn gpiote_intenset304_in6_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write IN6 mwrite None write None rac None reset value false")
    }
    #[doc = "IN7: Write '1' to Enable interrupt for IN\\[7\\] event<br>"]
    pub(crate) fn gpiote_intenset304_in7_read(&self) -> MemResult<bool> {
        todo!("read IN7 mwrite None write None rac None reset value false")
    }
    #[doc = "IN7: Write '1' to Enable interrupt for IN\\[7\\] event<br>"]
    pub(crate) fn gpiote_intenset304_in7_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write IN7 mwrite None write None rac None reset value false")
    }
    #[doc = "PORT: Write '1' to Enable interrupt for PORT event<br>"]
    pub(crate) fn gpiote_intenset304_port_read(&self) -> MemResult<bool> {
        todo!("read PORT mwrite None write None rac None reset value false")
    }
    #[doc = "PORT: Write '1' to Enable interrupt for PORT event<br>"]
    pub(crate) fn gpiote_intenset304_port_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PORT mwrite None write None rac None reset value false")
    }
    #[doc = "IN0: Write '1' to Disable interrupt for IN\\[0\\] event<br>"]
    pub(crate) fn gpiote_intenclr308_in0_read(&self) -> MemResult<bool> {
        todo!("read IN0 mwrite None write None rac None reset value false")
    }
    #[doc = "IN0: Write '1' to Disable interrupt for IN\\[0\\] event<br>"]
    pub(crate) fn gpiote_intenclr308_in0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write IN0 mwrite None write None rac None reset value false")
    }
    #[doc = "IN1: Write '1' to Disable interrupt for IN\\[1\\] event<br>"]
    pub(crate) fn gpiote_intenclr308_in1_read(&self) -> MemResult<bool> {
        todo!("read IN1 mwrite None write None rac None reset value false")
    }
    #[doc = "IN1: Write '1' to Disable interrupt for IN\\[1\\] event<br>"]
    pub(crate) fn gpiote_intenclr308_in1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write IN1 mwrite None write None rac None reset value false")
    }
    #[doc = "IN2: Write '1' to Disable interrupt for IN\\[2\\] event<br>"]
    pub(crate) fn gpiote_intenclr308_in2_read(&self) -> MemResult<bool> {
        todo!("read IN2 mwrite None write None rac None reset value false")
    }
    #[doc = "IN2: Write '1' to Disable interrupt for IN\\[2\\] event<br>"]
    pub(crate) fn gpiote_intenclr308_in2_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write IN2 mwrite None write None rac None reset value false")
    }
    #[doc = "IN3: Write '1' to Disable interrupt for IN\\[3\\] event<br>"]
    pub(crate) fn gpiote_intenclr308_in3_read(&self) -> MemResult<bool> {
        todo!("read IN3 mwrite None write None rac None reset value false")
    }
    #[doc = "IN3: Write '1' to Disable interrupt for IN\\[3\\] event<br>"]
    pub(crate) fn gpiote_intenclr308_in3_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write IN3 mwrite None write None rac None reset value false")
    }
    #[doc = "IN4: Write '1' to Disable interrupt for IN\\[4\\] event<br>"]
    pub(crate) fn gpiote_intenclr308_in4_read(&self) -> MemResult<bool> {
        todo!("read IN4 mwrite None write None rac None reset value false")
    }
    #[doc = "IN4: Write '1' to Disable interrupt for IN\\[4\\] event<br>"]
    pub(crate) fn gpiote_intenclr308_in4_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write IN4 mwrite None write None rac None reset value false")
    }
    #[doc = "IN5: Write '1' to Disable interrupt for IN\\[5\\] event<br>"]
    pub(crate) fn gpiote_intenclr308_in5_read(&self) -> MemResult<bool> {
        todo!("read IN5 mwrite None write None rac None reset value false")
    }
    #[doc = "IN5: Write '1' to Disable interrupt for IN\\[5\\] event<br>"]
    pub(crate) fn gpiote_intenclr308_in5_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write IN5 mwrite None write None rac None reset value false")
    }
    #[doc = "IN6: Write '1' to Disable interrupt for IN\\[6\\] event<br>"]
    pub(crate) fn gpiote_intenclr308_in6_read(&self) -> MemResult<bool> {
        todo!("read IN6 mwrite None write None rac None reset value false")
    }
    #[doc = "IN6: Write '1' to Disable interrupt for IN\\[6\\] event<br>"]
    pub(crate) fn gpiote_intenclr308_in6_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write IN6 mwrite None write None rac None reset value false")
    }
    #[doc = "IN7: Write '1' to Disable interrupt for IN\\[7\\] event<br>"]
    pub(crate) fn gpiote_intenclr308_in7_read(&self) -> MemResult<bool> {
        todo!("read IN7 mwrite None write None rac None reset value false")
    }
    #[doc = "IN7: Write '1' to Disable interrupt for IN\\[7\\] event<br>"]
    pub(crate) fn gpiote_intenclr308_in7_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write IN7 mwrite None write None rac None reset value false")
    }
    #[doc = "PORT: Write '1' to Disable interrupt for PORT event<br>"]
    pub(crate) fn gpiote_intenclr308_port_read(&self) -> MemResult<bool> {
        todo!("read PORT mwrite None write None rac None reset value false")
    }
    #[doc = "PORT: Write '1' to Disable interrupt for PORT event<br>"]
    pub(crate) fn gpiote_intenclr308_port_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PORT mwrite None write None rac None reset value false")
    }
    #[doc = "MODE: Mode<br>"]
    pub(crate) fn gpiote_confign510_mode_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<crate::peripheral::enums::E28GpioteConfign510Mode> {
        todo ! ("read MODE mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "MODE: Mode<br>"]
    pub(crate) fn gpiote_confign510_mode_write(
        &mut self,
        _reg_array: usize,
        _value: crate::peripheral::enums::E28GpioteConfign510Mode,
    ) -> MemResult<()> {
        todo ! ("write MODE mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "PSEL: GPIO number associated with SET\\[n\\], CLR\\[n\\] and OUT\\[n\\] tasks and IN\\[n\\] event<br>"]
    pub(crate) fn gpiote_confign510_psel_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<u8> {
        todo ! ("read PSEL mwrite None write None rac None reset value 0x00 mask 0x1f")
    }
    #[doc = "PSEL: GPIO number associated with SET\\[n\\], CLR\\[n\\] and OUT\\[n\\] tasks and IN\\[n\\] event<br>"]
    pub(crate) fn gpiote_confign510_psel_write(
        &mut self,
        _reg_array: usize,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write PSEL mwrite None write None rac None reset value 0x00 mask 0x1f")
    }
    #[doc = "POLARITY: When In task mode: Operation to be performed on output when OUT\\[n\\] task is triggered. When In event mode: Operation on input that shall trigger IN\\[n\\] event.<br>"]
    pub(crate) fn gpiote_confign510_polarity_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<crate::peripheral::enums::E29GpioteConfign510Polarity> {
        todo ! ("read POLARITY mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "POLARITY: When In task mode: Operation to be performed on output when OUT\\[n\\] task is triggered. When In event mode: Operation on input that shall trigger IN\\[n\\] event.<br>"]
    pub(crate) fn gpiote_confign510_polarity_write(
        &mut self,
        _reg_array: usize,
        _value: crate::peripheral::enums::E29GpioteConfign510Polarity,
    ) -> MemResult<()> {
        todo ! ("write POLARITY mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "OUTINIT: When in task mode: Initial value of the output when the GPIOTE channel is configured. When in event mode: No effect.<br>"]
    pub(crate) fn gpiote_confign510_outinit_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read OUTINIT mwrite None write None rac None reset value false")
    }
    #[doc = "OUTINIT: When in task mode: Initial value of the output when the GPIOTE channel is configured. When in event mode: No effect.<br>"]
    pub(crate) fn gpiote_confign510_outinit_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write OUTINIT mwrite None write None rac None reset value false")
    }
}
