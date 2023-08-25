use icicle_vm::cpu::mem::MemResult;

use super::event;

#[derive(Default)]
#[doc = "PPI: Programmable Peripheral Interconnect<br><br>Instances:<br>0x4001f000: PPI<br>"]
pub struct Ppi {
    pub group: [PPIChannelGroup; 4],
    pub soft_channels: [PpiSoftChannel; 20],
    pub hard_channels: [PpiHardChannel; 12],
}

#[derive(Default)]
pub struct PPIChannelGroup {
    pub on: bool,
    pub included: [bool; 32],
}

#[derive(Default)]
pub struct PpiSoftChannel {
    pub on: bool,
    pub event_register: u32,
    pub task_register: u32,
}

#[derive(Default)]
pub struct PpiHardChannel {
    pub on: bool,
}

/// implementation of PPI internals
impl Ppi {
    pub fn is_on(&self, channel: usize) -> bool {
        // FUTURE use std::mem::variant_count
        match channel {
            0..=19 => self.soft_channels[channel].on,
            20..=31 => self.hard_channels[channel - 20].on,
            _ => unreachable!(),
        }
    }
    pub fn set_on(&mut self, channel: usize, on: bool) {
        // FUTURE use std::mem::variant_count
        match channel {
            0..=19 => self.soft_channels[channel].on = on,
            20..=31 => self.hard_channels[channel - 20].on = on,
            _ => unreachable!(),
        }
    }
}

/// implementation of registers/fields read/write
impl Ppi {
    pub(crate) fn page_to_index(page: u64) -> usize {
        match page {
            262175u64 => 0usize,
            _ => unreachable!(),
        }
    }
    #[doc = "EN: Description cluster\\[0\\]:  Enable channel group 0<br>"]
    pub fn ppi_tasks_chgn_en0_write(
        &mut self,
        _tasks_chgn: usize,
        _value: u32,
    ) -> MemResult<()> {
        if event::trigger_task(_value) {
            self.group[_tasks_chgn].on = true;
        }
        Ok(())
    }
    #[doc = "DIS: Description cluster\\[0\\]:  Disable channel group 0<br>"]
    pub(crate) fn ppi_tasks_chgn_dis4_write(
        &mut self,
        _tasks_chgn: usize,
        _value: u32,
    ) -> MemResult<()> {
        if event::trigger_task(_value) {
            self.group[_tasks_chgn].on = false;
        }
        Ok(())
    }
    #[doc = "CH0: Enable or disable channel 0<br>"]
    pub(crate) fn ppi_chen500_ch0_read(&self) -> MemResult<bool> {
        Ok(self.is_on(0))
    }
    #[doc = "CH0: Enable or disable channel 0<br>"]
    pub fn ppi_chen500_ch0_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_on(0, _value))
    }
    #[doc = "CH1: Enable or disable channel 1<br>"]
    pub(crate) fn ppi_chen500_ch1_read(&self) -> MemResult<bool> {
        Ok(self.is_on(1))
    }
    #[doc = "CH1: Enable or disable channel 1<br>"]
    pub fn ppi_chen500_ch1_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_on(1, _value))
    }
    #[doc = "CH2: Enable or disable channel 2<br>"]
    pub(crate) fn ppi_chen500_ch2_read(&self) -> MemResult<bool> {
        Ok(self.is_on(2))
    }
    #[doc = "CH2: Enable or disable channel 2<br>"]
    pub fn ppi_chen500_ch2_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_on(2, _value))
    }
    #[doc = "CH3: Enable or disable channel 3<br>"]
    pub(crate) fn ppi_chen500_ch3_read(&self) -> MemResult<bool> {
        Ok(self.is_on(3))
    }
    #[doc = "CH3: Enable or disable channel 3<br>"]
    pub fn ppi_chen500_ch3_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_on(3, _value))
    }
    #[doc = "CH4: Enable or disable channel 4<br>"]
    pub(crate) fn ppi_chen500_ch4_read(&self) -> MemResult<bool> {
        Ok(self.is_on(4))
    }
    #[doc = "CH4: Enable or disable channel 4<br>"]
    pub fn ppi_chen500_ch4_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_on(4, _value))
    }
    #[doc = "CH5: Enable or disable channel 5<br>"]
    pub(crate) fn ppi_chen500_ch5_read(&self) -> MemResult<bool> {
        Ok(self.is_on(5))
    }
    #[doc = "CH5: Enable or disable channel 5<br>"]
    pub fn ppi_chen500_ch5_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_on(5, _value))
    }
    #[doc = "CH6: Enable or disable channel 6<br>"]
    pub(crate) fn ppi_chen500_ch6_read(&self) -> MemResult<bool> {
        Ok(self.is_on(6))
    }
    #[doc = "CH6: Enable or disable channel 6<br>"]
    pub fn ppi_chen500_ch6_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_on(6, _value))
    }
    #[doc = "CH7: Enable or disable channel 7<br>"]
    pub(crate) fn ppi_chen500_ch7_read(&self) -> MemResult<bool> {
        Ok(self.is_on(7))
    }
    #[doc = "CH7: Enable or disable channel 7<br>"]
    pub fn ppi_chen500_ch7_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_on(7, _value))
    }
    #[doc = "CH8: Enable or disable channel 8<br>"]
    pub(crate) fn ppi_chen500_ch8_read(&self) -> MemResult<bool> {
        Ok(self.is_on(8))
    }
    #[doc = "CH8: Enable or disable channel 8<br>"]
    pub fn ppi_chen500_ch8_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_on(8, _value))
    }
    #[doc = "CH9: Enable or disable channel 9<br>"]
    pub(crate) fn ppi_chen500_ch9_read(&self) -> MemResult<bool> {
        Ok(self.is_on(9))
    }
    #[doc = "CH9: Enable or disable channel 9<br>"]
    pub fn ppi_chen500_ch9_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_on(9, _value))
    }
    #[doc = "CH10: Enable or disable channel 10<br>"]
    pub(crate) fn ppi_chen500_ch10_read(&self) -> MemResult<bool> {
        Ok(self.is_on(10))
    }
    #[doc = "CH10: Enable or disable channel 10<br>"]
    pub fn ppi_chen500_ch10_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_on(10, _value))
    }
    #[doc = "CH11: Enable or disable channel 11<br>"]
    pub(crate) fn ppi_chen500_ch11_read(&self) -> MemResult<bool> {
        Ok(self.is_on(11))
    }
    #[doc = "CH11: Enable or disable channel 11<br>"]
    pub fn ppi_chen500_ch11_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_on(11, _value))
    }
    #[doc = "CH12: Enable or disable channel 12<br>"]
    pub(crate) fn ppi_chen500_ch12_read(&self) -> MemResult<bool> {
        Ok(self.is_on(12))
    }
    #[doc = "CH12: Enable or disable channel 12<br>"]
    pub fn ppi_chen500_ch12_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_on(12, _value))
    }
    #[doc = "CH13: Enable or disable channel 13<br>"]
    pub(crate) fn ppi_chen500_ch13_read(&self) -> MemResult<bool> {
        Ok(self.is_on(13))
    }
    #[doc = "CH13: Enable or disable channel 13<br>"]
    pub fn ppi_chen500_ch13_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_on(13, _value))
    }
    #[doc = "CH14: Enable or disable channel 14<br>"]
    pub(crate) fn ppi_chen500_ch14_read(&self) -> MemResult<bool> {
        Ok(self.is_on(14))
    }
    #[doc = "CH14: Enable or disable channel 14<br>"]
    pub fn ppi_chen500_ch14_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_on(14, _value))
    }
    #[doc = "CH15: Enable or disable channel 15<br>"]
    pub(crate) fn ppi_chen500_ch15_read(&self) -> MemResult<bool> {
        Ok(self.is_on(15))
    }
    #[doc = "CH15: Enable or disable channel 15<br>"]
    pub fn ppi_chen500_ch15_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_on(15, _value))
    }
    #[doc = "CH16: Enable or disable channel 16<br>"]
    pub(crate) fn ppi_chen500_ch16_read(&self) -> MemResult<bool> {
        Ok(self.is_on(16))
    }
    #[doc = "CH16: Enable or disable channel 16<br>"]
    pub fn ppi_chen500_ch16_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_on(16, _value))
    }
    #[doc = "CH17: Enable or disable channel 17<br>"]
    pub(crate) fn ppi_chen500_ch17_read(&self) -> MemResult<bool> {
        Ok(self.is_on(17))
    }
    #[doc = "CH17: Enable or disable channel 17<br>"]
    pub fn ppi_chen500_ch17_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_on(17, _value))
    }
    #[doc = "CH18: Enable or disable channel 18<br>"]
    pub(crate) fn ppi_chen500_ch18_read(&self) -> MemResult<bool> {
        Ok(self.is_on(18))
    }
    #[doc = "CH18: Enable or disable channel 18<br>"]
    pub fn ppi_chen500_ch18_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_on(18, _value))
    }
    #[doc = "CH19: Enable or disable channel 19<br>"]
    pub(crate) fn ppi_chen500_ch19_read(&self) -> MemResult<bool> {
        Ok(self.is_on(19))
    }
    #[doc = "CH19: Enable or disable channel 19<br>"]
    pub fn ppi_chen500_ch19_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_on(19, _value))
    }
    #[doc = "CH20: Enable or disable channel 20<br>"]
    pub(crate) fn ppi_chen500_ch20_read(&self) -> MemResult<bool> {
        Ok(self.is_on(20))
    }
    #[doc = "CH20: Enable or disable channel 20<br>"]
    pub fn ppi_chen500_ch20_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_on(20, _value))
    }
    #[doc = "CH21: Enable or disable channel 21<br>"]
    pub(crate) fn ppi_chen500_ch21_read(&self) -> MemResult<bool> {
        Ok(self.is_on(21))
    }
    #[doc = "CH21: Enable or disable channel 21<br>"]
    pub fn ppi_chen500_ch21_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_on(21, _value))
    }
    #[doc = "CH22: Enable or disable channel 22<br>"]
    pub(crate) fn ppi_chen500_ch22_read(&self) -> MemResult<bool> {
        Ok(self.is_on(22))
    }
    #[doc = "CH22: Enable or disable channel 22<br>"]
    pub fn ppi_chen500_ch22_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_on(22, _value))
    }
    #[doc = "CH23: Enable or disable channel 23<br>"]
    pub(crate) fn ppi_chen500_ch23_read(&self) -> MemResult<bool> {
        Ok(self.is_on(23))
    }
    #[doc = "CH23: Enable or disable channel 23<br>"]
    pub fn ppi_chen500_ch23_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_on(23, _value))
    }
    #[doc = "CH24: Enable or disable channel 24<br>"]
    pub(crate) fn ppi_chen500_ch24_read(&self) -> MemResult<bool> {
        Ok(self.is_on(24))
    }
    #[doc = "CH24: Enable or disable channel 24<br>"]
    pub fn ppi_chen500_ch24_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_on(24, _value))
    }
    #[doc = "CH25: Enable or disable channel 25<br>"]
    pub(crate) fn ppi_chen500_ch25_read(&self) -> MemResult<bool> {
        Ok(self.is_on(25))
    }
    #[doc = "CH25: Enable or disable channel 25<br>"]
    pub fn ppi_chen500_ch25_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_on(25, _value))
    }
    #[doc = "CH26: Enable or disable channel 26<br>"]
    pub(crate) fn ppi_chen500_ch26_read(&self) -> MemResult<bool> {
        Ok(self.is_on(26))
    }
    #[doc = "CH26: Enable or disable channel 26<br>"]
    pub fn ppi_chen500_ch26_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_on(26, _value))
    }
    #[doc = "CH27: Enable or disable channel 27<br>"]
    pub(crate) fn ppi_chen500_ch27_read(&self) -> MemResult<bool> {
        Ok(self.is_on(27))
    }
    #[doc = "CH27: Enable or disable channel 27<br>"]
    pub fn ppi_chen500_ch27_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_on(27, _value))
    }
    #[doc = "CH28: Enable or disable channel 28<br>"]
    pub(crate) fn ppi_chen500_ch28_read(&self) -> MemResult<bool> {
        Ok(self.is_on(28))
    }
    #[doc = "CH28: Enable or disable channel 28<br>"]
    pub fn ppi_chen500_ch28_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_on(28, _value))
    }
    #[doc = "CH29: Enable or disable channel 29<br>"]
    pub(crate) fn ppi_chen500_ch29_read(&self) -> MemResult<bool> {
        Ok(self.is_on(29))
    }
    #[doc = "CH29: Enable or disable channel 29<br>"]
    pub fn ppi_chen500_ch29_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_on(29, _value))
    }
    #[doc = "CH30: Enable or disable channel 30<br>"]
    pub(crate) fn ppi_chen500_ch30_read(&self) -> MemResult<bool> {
        Ok(self.is_on(30))
    }
    #[doc = "CH30: Enable or disable channel 30<br>"]
    pub fn ppi_chen500_ch30_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_on(30, _value))
    }
    #[doc = "CH31: Enable or disable channel 31<br>"]
    pub(crate) fn ppi_chen500_ch31_read(&self) -> MemResult<bool> {
        Ok(self.is_on(31))
    }
    #[doc = "CH31: Enable or disable channel 31<br>"]
    pub fn ppi_chen500_ch31_write(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.set_on(31, _value))
    }
    #[doc = "CH0: Channel 0 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch0_read(&self) -> MemResult<bool> {
        Ok(self.is_on(0))
    }
    #[doc = "CH0: Channel 0 enable set register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenset504_ch0_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(0, true);
        }
        Ok(())
    }
    #[doc = "CH1: Channel 1 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch1_read(&self) -> MemResult<bool> {
        Ok(self.is_on(1))
    }
    #[doc = "CH1: Channel 1 enable set register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenset504_ch1_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(1, true);
        }
        Ok(())
    }
    #[doc = "CH2: Channel 2 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch2_read(&self) -> MemResult<bool> {
        Ok(self.is_on(2))
    }
    #[doc = "CH2: Channel 2 enable set register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenset504_ch2_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(2, true);
        }
        Ok(())
    }
    #[doc = "CH3: Channel 3 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch3_read(&self) -> MemResult<bool> {
        Ok(self.is_on(3))
    }
    #[doc = "CH3: Channel 3 enable set register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenset504_ch3_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(3, true);
        }
        Ok(())
    }
    #[doc = "CH4: Channel 4 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch4_read(&self) -> MemResult<bool> {
        Ok(self.is_on(4))
    }
    #[doc = "CH4: Channel 4 enable set register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenset504_ch4_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(4, true);
        }
        Ok(())
    }
    #[doc = "CH5: Channel 5 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch5_read(&self) -> MemResult<bool> {
        Ok(self.is_on(5))
    }
    #[doc = "CH5: Channel 5 enable set register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenset504_ch5_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(5, true);
        }
        Ok(())
    }
    #[doc = "CH6: Channel 6 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch6_read(&self) -> MemResult<bool> {
        Ok(self.is_on(6))
    }
    #[doc = "CH6: Channel 6 enable set register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenset504_ch6_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(6, true);
        }
        Ok(())
    }
    #[doc = "CH7: Channel 7 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch7_read(&self) -> MemResult<bool> {
        Ok(self.is_on(7))
    }
    #[doc = "CH7: Channel 7 enable set register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenset504_ch7_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(7, true);
        }
        Ok(())
    }
    #[doc = "CH8: Channel 8 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch8_read(&self) -> MemResult<bool> {
        Ok(self.is_on(8))
    }
    #[doc = "CH8: Channel 8 enable set register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenset504_ch8_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(8, true);
        }
        Ok(())
    }
    #[doc = "CH9: Channel 9 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch9_read(&self) -> MemResult<bool> {
        Ok(self.is_on(9))
    }
    #[doc = "CH9: Channel 9 enable set register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenset504_ch9_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(9, true);
        }
        Ok(())
    }
    #[doc = "CH10: Channel 10 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch10_read(&self) -> MemResult<bool> {
        Ok(self.is_on(10))
    }
    #[doc = "CH10: Channel 10 enable set register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenset504_ch10_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(10, true);
        }
        Ok(())
    }
    #[doc = "CH11: Channel 11 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch11_read(&self) -> MemResult<bool> {
        Ok(self.is_on(11))
    }
    #[doc = "CH11: Channel 11 enable set register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenset504_ch11_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(11, true);
        }
        Ok(())
    }
    #[doc = "CH12: Channel 12 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch12_read(&self) -> MemResult<bool> {
        Ok(self.is_on(12))
    }
    #[doc = "CH12: Channel 12 enable set register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenset504_ch12_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(12, true);
        }
        Ok(())
    }
    #[doc = "CH13: Channel 13 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch13_read(&self) -> MemResult<bool> {
        Ok(self.is_on(13))
    }
    #[doc = "CH13: Channel 13 enable set register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenset504_ch13_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(13, true);
        }
        Ok(())
    }
    #[doc = "CH14: Channel 14 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch14_read(&self) -> MemResult<bool> {
        Ok(self.is_on(14))
    }
    #[doc = "CH14: Channel 14 enable set register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenset504_ch14_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(14, true);
        }
        Ok(())
    }
    #[doc = "CH15: Channel 15 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch15_read(&self) -> MemResult<bool> {
        Ok(self.is_on(15))
    }
    #[doc = "CH15: Channel 15 enable set register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenset504_ch15_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(15, true);
        }
        Ok(())
    }
    #[doc = "CH16: Channel 16 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch16_read(&self) -> MemResult<bool> {
        Ok(self.is_on(16))
    }
    #[doc = "CH16: Channel 16 enable set register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenset504_ch16_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(16, true);
        }
        Ok(())
    }
    #[doc = "CH17: Channel 17 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch17_read(&self) -> MemResult<bool> {
        Ok(self.is_on(17))
    }
    #[doc = "CH17: Channel 17 enable set register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenset504_ch17_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(17, true);
        }
        Ok(())
    }
    #[doc = "CH18: Channel 18 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch18_read(&self) -> MemResult<bool> {
        Ok(self.is_on(18))
    }
    #[doc = "CH18: Channel 18 enable set register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenset504_ch18_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(18, true);
        }
        Ok(())
    }
    #[doc = "CH19: Channel 19 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch19_read(&self) -> MemResult<bool> {
        Ok(self.is_on(19))
    }
    #[doc = "CH19: Channel 19 enable set register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenset504_ch19_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(19, true);
        }
        Ok(())
    }
    #[doc = "CH20: Channel 20 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch20_read(&self) -> MemResult<bool> {
        Ok(self.is_on(20))
    }
    #[doc = "CH20: Channel 20 enable set register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenset504_ch20_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(20, true);
        }
        Ok(())
    }
    #[doc = "CH21: Channel 21 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch21_read(&self) -> MemResult<bool> {
        Ok(self.is_on(21))
    }
    #[doc = "CH21: Channel 21 enable set register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenset504_ch21_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(21, true);
        }
        Ok(())
    }
    #[doc = "CH22: Channel 22 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch22_read(&self) -> MemResult<bool> {
        Ok(self.is_on(22))
    }
    #[doc = "CH22: Channel 22 enable set register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenset504_ch22_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(22, true);
        }
        Ok(())
    }
    #[doc = "CH23: Channel 23 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch23_read(&self) -> MemResult<bool> {
        Ok(self.is_on(23))
    }
    #[doc = "CH23: Channel 23 enable set register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenset504_ch23_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(23, true);
        }
        Ok(())
    }
    #[doc = "CH24: Channel 24 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch24_read(&self) -> MemResult<bool> {
        Ok(self.is_on(24))
    }
    #[doc = "CH24: Channel 24 enable set register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenset504_ch24_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(24, true);
        }
        Ok(())
    }
    #[doc = "CH25: Channel 25 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch25_read(&self) -> MemResult<bool> {
        Ok(self.is_on(25))
    }
    #[doc = "CH25: Channel 25 enable set register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenset504_ch25_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(25, true);
        }
        Ok(())
    }
    #[doc = "CH26: Channel 26 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch26_read(&self) -> MemResult<bool> {
        Ok(self.is_on(26))
    }
    #[doc = "CH26: Channel 26 enable set register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenset504_ch26_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(26, true);
        }
        Ok(())
    }
    #[doc = "CH27: Channel 27 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch27_read(&self) -> MemResult<bool> {
        Ok(self.is_on(27))
    }
    #[doc = "CH27: Channel 27 enable set register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenset504_ch27_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(27, true);
        }
        Ok(())
    }
    #[doc = "CH28: Channel 28 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch28_read(&self) -> MemResult<bool> {
        Ok(self.is_on(28))
    }
    #[doc = "CH28: Channel 28 enable set register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenset504_ch28_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(28, true);
        }
        Ok(())
    }
    #[doc = "CH29: Channel 29 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch29_read(&self) -> MemResult<bool> {
        Ok(self.is_on(29))
    }
    #[doc = "CH29: Channel 29 enable set register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenset504_ch29_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(29, true);
        }
        Ok(())
    }
    #[doc = "CH30: Channel 30 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch30_read(&self) -> MemResult<bool> {
        Ok(self.is_on(30))
    }
    #[doc = "CH30: Channel 30 enable set register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenset504_ch30_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(30, true);
        }
        Ok(())
    }
    #[doc = "CH31: Channel 31 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch31_read(&self) -> MemResult<bool> {
        Ok(self.is_on(31))
    }
    #[doc = "CH31: Channel 31 enable set register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenset504_ch31_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(31, true);
        }
        Ok(())
    }
    #[doc = "CH0: Channel 0 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch0_read(&self) -> MemResult<bool> {
        Ok(self.is_on(0))
    }
    #[doc = "CH0: Channel 0 enable clear register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenclr508_ch0_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(0, false);
        }
        Ok(())
    }
    #[doc = "CH1: Channel 1 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch1_read(&self) -> MemResult<bool> {
        Ok(self.is_on(1))
    }
    #[doc = "CH1: Channel 1 enable clear register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenclr508_ch1_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(1, false);
        }
        Ok(())
    }
    #[doc = "CH2: Channel 2 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch2_read(&self) -> MemResult<bool> {
        Ok(self.is_on(2))
    }
    #[doc = "CH2: Channel 2 enable clear register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenclr508_ch2_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(2, false);
        }
        Ok(())
    }
    #[doc = "CH3: Channel 3 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch3_read(&self) -> MemResult<bool> {
        Ok(self.is_on(3))
    }
    #[doc = "CH3: Channel 3 enable clear register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenclr508_ch3_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(3, false);
        }
        Ok(())
    }
    #[doc = "CH4: Channel 4 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch4_read(&self) -> MemResult<bool> {
        Ok(self.is_on(4))
    }
    #[doc = "CH4: Channel 4 enable clear register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenclr508_ch4_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(4, false);
        }
        Ok(())
    }
    #[doc = "CH5: Channel 5 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch5_read(&self) -> MemResult<bool> {
        Ok(self.is_on(5))
    }
    #[doc = "CH5: Channel 5 enable clear register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenclr508_ch5_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(5, false);
        }
        Ok(())
    }
    #[doc = "CH6: Channel 6 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch6_read(&self) -> MemResult<bool> {
        Ok(self.is_on(6))
    }
    #[doc = "CH6: Channel 6 enable clear register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenclr508_ch6_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(6, false);
        }
        Ok(())
    }
    #[doc = "CH7: Channel 7 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch7_read(&self) -> MemResult<bool> {
        Ok(self.is_on(7))
    }
    #[doc = "CH7: Channel 7 enable clear register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenclr508_ch7_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(7, false);
        }
        Ok(())
    }
    #[doc = "CH8: Channel 8 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch8_read(&self) -> MemResult<bool> {
        Ok(self.is_on(8))
    }
    #[doc = "CH8: Channel 8 enable clear register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenclr508_ch8_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(8, false);
        }
        Ok(())
    }
    #[doc = "CH9: Channel 9 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch9_read(&self) -> MemResult<bool> {
        Ok(self.is_on(9))
    }
    #[doc = "CH9: Channel 9 enable clear register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenclr508_ch9_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(9, false);
        }
        Ok(())
    }
    #[doc = "CH10: Channel 10 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch10_read(&self) -> MemResult<bool> {
        Ok(self.is_on(10))
    }
    #[doc = "CH10: Channel 10 enable clear register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenclr508_ch10_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(10, false);
        }
        Ok(())
    }
    #[doc = "CH11: Channel 11 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch11_read(&self) -> MemResult<bool> {
        Ok(self.is_on(11))
    }
    #[doc = "CH11: Channel 11 enable clear register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenclr508_ch11_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(11, false);
        }
        Ok(())
    }
    #[doc = "CH12: Channel 12 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch12_read(&self) -> MemResult<bool> {
        Ok(self.is_on(12))
    }
    #[doc = "CH12: Channel 12 enable clear register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenclr508_ch12_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(12, false);
        }
        Ok(())
    }
    #[doc = "CH13: Channel 13 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch13_read(&self) -> MemResult<bool> {
        Ok(self.is_on(13))
    }
    #[doc = "CH13: Channel 13 enable clear register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenclr508_ch13_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(13, false);
        }
        Ok(())
    }
    #[doc = "CH14: Channel 14 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch14_read(&self) -> MemResult<bool> {
        Ok(self.is_on(14))
    }
    #[doc = "CH14: Channel 14 enable clear register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenclr508_ch14_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(14, false);
        }
        Ok(())
    }
    #[doc = "CH15: Channel 15 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch15_read(&self) -> MemResult<bool> {
        Ok(self.is_on(15))
    }
    #[doc = "CH15: Channel 15 enable clear register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenclr508_ch15_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(15, false);
        }
        Ok(())
    }
    #[doc = "CH16: Channel 16 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch16_read(&self) -> MemResult<bool> {
        Ok(self.is_on(16))
    }
    #[doc = "CH16: Channel 16 enable clear register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenclr508_ch16_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(16, false);
        }
        Ok(())
    }
    #[doc = "CH17: Channel 17 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch17_read(&self) -> MemResult<bool> {
        Ok(self.is_on(17))
    }
    #[doc = "CH17: Channel 17 enable clear register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenclr508_ch17_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(17, false);
        }
        Ok(())
    }
    #[doc = "CH18: Channel 18 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch18_read(&self) -> MemResult<bool> {
        Ok(self.is_on(18))
    }
    #[doc = "CH18: Channel 18 enable clear register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenclr508_ch18_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(18, false);
        }
        Ok(())
    }
    #[doc = "CH19: Channel 19 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch19_read(&self) -> MemResult<bool> {
        Ok(self.is_on(19))
    }
    #[doc = "CH19: Channel 19 enable clear register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenclr508_ch19_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(19, false);
        }
        Ok(())
    }
    #[doc = "CH20: Channel 20 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch20_read(&self) -> MemResult<bool> {
        Ok(self.is_on(20))
    }
    #[doc = "CH20: Channel 20 enable clear register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenclr508_ch20_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(20, false);
        }
        Ok(())
    }
    #[doc = "CH21: Channel 21 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch21_read(&self) -> MemResult<bool> {
        Ok(self.is_on(21))
    }
    #[doc = "CH21: Channel 21 enable clear register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenclr508_ch21_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(21, false);
        }
        Ok(())
    }
    #[doc = "CH22: Channel 22 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch22_read(&self) -> MemResult<bool> {
        Ok(self.is_on(22))
    }
    #[doc = "CH22: Channel 22 enable clear register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenclr508_ch22_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(22, false);
        }
        Ok(())
    }
    #[doc = "CH23: Channel 23 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch23_read(&self) -> MemResult<bool> {
        Ok(self.is_on(23))
    }
    #[doc = "CH23: Channel 23 enable clear register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenclr508_ch23_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(23, false);
        }
        Ok(())
    }
    #[doc = "CH24: Channel 24 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch24_read(&self) -> MemResult<bool> {
        Ok(self.is_on(24))
    }
    #[doc = "CH24: Channel 24 enable clear register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenclr508_ch24_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(24, false);
        }
        Ok(())
    }
    #[doc = "CH25: Channel 25 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch25_read(&self) -> MemResult<bool> {
        Ok(self.is_on(25))
    }
    #[doc = "CH25: Channel 25 enable clear register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenclr508_ch25_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(25, false);
        }
        Ok(())
    }
    #[doc = "CH26: Channel 26 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch26_read(&self) -> MemResult<bool> {
        Ok(self.is_on(26))
    }
    #[doc = "CH26: Channel 26 enable clear register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenclr508_ch26_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(26, false);
        }
        Ok(())
    }
    #[doc = "CH27: Channel 27 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch27_read(&self) -> MemResult<bool> {
        Ok(self.is_on(27))
    }
    #[doc = "CH27: Channel 27 enable clear register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenclr508_ch27_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(27, false);
        }
        Ok(())
    }
    #[doc = "CH28: Channel 28 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch28_read(&self) -> MemResult<bool> {
        Ok(self.is_on(28))
    }
    #[doc = "CH28: Channel 28 enable clear register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenclr508_ch28_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(28, false);
        }
        Ok(())
    }
    #[doc = "CH29: Channel 29 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch29_read(&self) -> MemResult<bool> {
        Ok(self.is_on(29))
    }
    #[doc = "CH29: Channel 29 enable clear register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenclr508_ch29_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(29, false);
        }
        Ok(())
    }
    #[doc = "CH30: Channel 30 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch30_read(&self) -> MemResult<bool> {
        Ok(self.is_on(30))
    }
    #[doc = "CH30: Channel 30 enable clear register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenclr508_ch30_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(30, false);
        }
        Ok(())
    }
    #[doc = "CH31: Channel 31 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch31_read(&self) -> MemResult<bool> {
        Ok(self.is_on(31))
    }
    #[doc = "CH31: Channel 31 enable clear register.  Writing '0' has no effect<br>"]
    pub fn ppi_chenclr508_ch31_write(&mut self, _value: bool) -> MemResult<()> {
        if _value {
            self.set_on(31, false);
        }
        Ok(())
    }
    #[doc = "EEP: Pointer to event register. Accepts only addresses to registers from the Event group.<br>"]
    pub(crate) fn ppi_chn_eep0_eep_read(&self, _chn: usize) -> MemResult<u32> {
        Ok(self.soft_channels[_chn].event_register)
    }
    #[doc = "EEP: Pointer to event register. Accepts only addresses to registers from the Event group.<br>"]
    pub(crate) fn ppi_chn_eep0_eep_write(
        &mut self,
        _chn: usize,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.soft_channels[_chn].event_register = _value)
    }
    #[doc = "TEP: Pointer to task register. Accepts only addresses to registers from the Task group.<br>"]
    pub(crate) fn ppi_chn_tep4_tep_read(&self, _chn: usize) -> MemResult<u32> {
        Ok(self.soft_channels[_chn].task_register)
    }
    #[doc = "TEP: Pointer to task register. Accepts only addresses to registers from the Task group.<br>"]
    pub(crate) fn ppi_chn_tep4_tep_write(
        &mut self,
        _chn: usize,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.soft_channels[_chn].task_register = _value)
    }
    #[doc = "CH0: Include or exclude channel 0<br>"]
    pub fn ppi_chgn800_ch0_read(
        &mut self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        Ok(self.group[_reg_array].included[0])
    }
    #[doc = "CH0: Include or exclude channel 0<br>"]
    pub fn ppi_chgn800_ch0_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.group[_reg_array].included[0] = _value)
    }
    #[doc = "CH1: Include or exclude channel 1<br>"]
    pub fn ppi_chgn800_ch1_read(
        &mut self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        Ok(self.group[_reg_array].included[1])
    }
    #[doc = "CH1: Include or exclude channel 1<br>"]
    pub fn ppi_chgn800_ch1_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.group[_reg_array].included[1] = _value)
    }
    #[doc = "CH2: Include or exclude channel 2<br>"]
    pub fn ppi_chgn800_ch2_read(
        &mut self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        Ok(self.group[_reg_array].included[2])
    }
    #[doc = "CH2: Include or exclude channel 2<br>"]
    pub fn ppi_chgn800_ch2_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.group[_reg_array].included[2] = _value)
    }
    #[doc = "CH3: Include or exclude channel 3<br>"]
    pub fn ppi_chgn800_ch3_read(
        &mut self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        Ok(self.group[_reg_array].included[3])
    }
    #[doc = "CH3: Include or exclude channel 3<br>"]
    pub fn ppi_chgn800_ch3_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.group[_reg_array].included[3] = _value)
    }
    #[doc = "CH4: Include or exclude channel 4<br>"]
    pub fn ppi_chgn800_ch4_read(
        &mut self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        Ok(self.group[_reg_array].included[4])
    }
    #[doc = "CH4: Include or exclude channel 4<br>"]
    pub fn ppi_chgn800_ch4_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.group[_reg_array].included[4] = _value)
    }
    #[doc = "CH5: Include or exclude channel 5<br>"]
    pub fn ppi_chgn800_ch5_read(
        &mut self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        Ok(self.group[_reg_array].included[5])
    }
    #[doc = "CH5: Include or exclude channel 5<br>"]
    pub fn ppi_chgn800_ch5_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.group[_reg_array].included[5] = _value)
    }
    #[doc = "CH6: Include or exclude channel 6<br>"]
    pub fn ppi_chgn800_ch6_read(
        &mut self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        Ok(self.group[_reg_array].included[6])
    }
    #[doc = "CH6: Include or exclude channel 6<br>"]
    pub fn ppi_chgn800_ch6_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.group[_reg_array].included[6] = _value)
    }
    #[doc = "CH7: Include or exclude channel 7<br>"]
    pub fn ppi_chgn800_ch7_read(
        &mut self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        Ok(self.group[_reg_array].included[7])
    }
    #[doc = "CH7: Include or exclude channel 7<br>"]
    pub fn ppi_chgn800_ch7_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.group[_reg_array].included[7] = _value)
    }
    #[doc = "CH8: Include or exclude channel 8<br>"]
    pub fn ppi_chgn800_ch8_read(
        &mut self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        Ok(self.group[_reg_array].included[8])
    }
    #[doc = "CH8: Include or exclude channel 8<br>"]
    pub fn ppi_chgn800_ch8_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.group[_reg_array].included[8] = _value)
    }
    #[doc = "CH9: Include or exclude channel 9<br>"]
    pub fn ppi_chgn800_ch9_read(
        &mut self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        Ok(self.group[_reg_array].included[9])
    }
    #[doc = "CH9: Include or exclude channel 9<br>"]
    pub fn ppi_chgn800_ch9_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.group[_reg_array].included[9] = _value)
    }
    #[doc = "CH10: Include or exclude channel 10<br>"]
    pub fn ppi_chgn800_ch10_read(
        &mut self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        Ok(self.group[_reg_array].included[10])
    }
    #[doc = "CH10: Include or exclude channel 10<br>"]
    pub fn ppi_chgn800_ch10_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.group[_reg_array].included[10] = _value)
    }
    #[doc = "CH11: Include or exclude channel 11<br>"]
    pub fn ppi_chgn800_ch11_read(
        &mut self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        Ok(self.group[_reg_array].included[11])
    }
    #[doc = "CH11: Include or exclude channel 11<br>"]
    pub fn ppi_chgn800_ch11_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.group[_reg_array].included[11] = _value)
    }
    #[doc = "CH12: Include or exclude channel 12<br>"]
    pub fn ppi_chgn800_ch12_read(
        &mut self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        Ok(self.group[_reg_array].included[12])
    }
    #[doc = "CH12: Include or exclude channel 12<br>"]
    pub fn ppi_chgn800_ch12_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.group[_reg_array].included[12] = _value)
    }
    #[doc = "CH13: Include or exclude channel 13<br>"]
    pub fn ppi_chgn800_ch13_read(
        &mut self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        Ok(self.group[_reg_array].included[13])
    }
    #[doc = "CH13: Include or exclude channel 13<br>"]
    pub fn ppi_chgn800_ch13_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.group[_reg_array].included[13] = _value)
    }
    #[doc = "CH14: Include or exclude channel 14<br>"]
    pub fn ppi_chgn800_ch14_read(
        &mut self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        Ok(self.group[_reg_array].included[14])
    }
    #[doc = "CH14: Include or exclude channel 14<br>"]
    pub fn ppi_chgn800_ch14_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.group[_reg_array].included[14] = _value)
    }
    #[doc = "CH15: Include or exclude channel 15<br>"]
    pub fn ppi_chgn800_ch15_read(
        &mut self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        Ok(self.group[_reg_array].included[15])
    }
    #[doc = "CH15: Include or exclude channel 15<br>"]
    pub fn ppi_chgn800_ch15_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.group[_reg_array].included[15] = _value)
    }
    #[doc = "CH16: Include or exclude channel 16<br>"]
    pub fn ppi_chgn800_ch16_read(
        &mut self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        Ok(self.group[_reg_array].included[16])
    }
    #[doc = "CH16: Include or exclude channel 16<br>"]
    pub fn ppi_chgn800_ch16_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.group[_reg_array].included[16] = _value)
    }
    #[doc = "CH17: Include or exclude channel 17<br>"]
    pub fn ppi_chgn800_ch17_read(
        &mut self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        Ok(self.group[_reg_array].included[17])
    }
    #[doc = "CH17: Include or exclude channel 17<br>"]
    pub fn ppi_chgn800_ch17_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.group[_reg_array].included[17] = _value)
    }
    #[doc = "CH18: Include or exclude channel 18<br>"]
    pub fn ppi_chgn800_ch18_read(
        &mut self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        Ok(self.group[_reg_array].included[18])
    }
    #[doc = "CH18: Include or exclude channel 18<br>"]
    pub fn ppi_chgn800_ch18_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.group[_reg_array].included[18] = _value)
    }
    #[doc = "CH19: Include or exclude channel 19<br>"]
    pub fn ppi_chgn800_ch19_read(
        &mut self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        Ok(self.group[_reg_array].included[19])
    }
    #[doc = "CH19: Include or exclude channel 19<br>"]
    pub fn ppi_chgn800_ch19_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.group[_reg_array].included[19] = _value)
    }
    #[doc = "CH20: Include or exclude channel 20<br>"]
    pub fn ppi_chgn800_ch20_read(
        &mut self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        Ok(self.group[_reg_array].included[20])
    }
    #[doc = "CH20: Include or exclude channel 20<br>"]
    pub fn ppi_chgn800_ch20_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.group[_reg_array].included[20] = _value)
    }
    #[doc = "CH21: Include or exclude channel 21<br>"]
    pub fn ppi_chgn800_ch21_read(
        &mut self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        Ok(self.group[_reg_array].included[21])
    }
    #[doc = "CH21: Include or exclude channel 21<br>"]
    pub fn ppi_chgn800_ch21_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.group[_reg_array].included[21] = _value)
    }
    #[doc = "CH22: Include or exclude channel 22<br>"]
    pub fn ppi_chgn800_ch22_read(
        &mut self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        Ok(self.group[_reg_array].included[22])
    }
    #[doc = "CH22: Include or exclude channel 22<br>"]
    pub fn ppi_chgn800_ch22_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.group[_reg_array].included[22] = _value)
    }
    #[doc = "CH23: Include or exclude channel 23<br>"]
    pub fn ppi_chgn800_ch23_read(
        &mut self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        Ok(self.group[_reg_array].included[23])
    }
    #[doc = "CH23: Include or exclude channel 23<br>"]
    pub fn ppi_chgn800_ch23_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.group[_reg_array].included[23] = _value)
    }
    #[doc = "CH24: Include or exclude channel 24<br>"]
    pub fn ppi_chgn800_ch24_read(
        &mut self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        Ok(self.group[_reg_array].included[24])
    }
    #[doc = "CH24: Include or exclude channel 24<br>"]
    pub fn ppi_chgn800_ch24_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.group[_reg_array].included[24] = _value)
    }
    #[doc = "CH25: Include or exclude channel 25<br>"]
    pub fn ppi_chgn800_ch25_read(
        &mut self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        Ok(self.group[_reg_array].included[25])
    }
    #[doc = "CH25: Include or exclude channel 25<br>"]
    pub fn ppi_chgn800_ch25_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.group[_reg_array].included[25] = _value)
    }
    #[doc = "CH26: Include or exclude channel 26<br>"]
    pub fn ppi_chgn800_ch26_read(
        &mut self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        Ok(self.group[_reg_array].included[26])
    }
    #[doc = "CH26: Include or exclude channel 26<br>"]
    pub fn ppi_chgn800_ch26_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.group[_reg_array].included[26] = _value)
    }
    #[doc = "CH27: Include or exclude channel 27<br>"]
    pub fn ppi_chgn800_ch27_read(
        &mut self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        Ok(self.group[_reg_array].included[27])
    }
    #[doc = "CH27: Include or exclude channel 27<br>"]
    pub fn ppi_chgn800_ch27_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.group[_reg_array].included[27] = _value)
    }
    #[doc = "CH28: Include or exclude channel 28<br>"]
    pub fn ppi_chgn800_ch28_read(
        &mut self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        Ok(self.group[_reg_array].included[28])
    }
    #[doc = "CH28: Include or exclude channel 28<br>"]
    pub fn ppi_chgn800_ch28_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.group[_reg_array].included[28] = _value)
    }
    #[doc = "CH29: Include or exclude channel 29<br>"]
    pub fn ppi_chgn800_ch29_read(
        &mut self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        Ok(self.group[_reg_array].included[29])
    }
    #[doc = "CH29: Include or exclude channel 29<br>"]
    pub fn ppi_chgn800_ch29_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.group[_reg_array].included[29] = _value)
    }
    #[doc = "CH30: Include or exclude channel 30<br>"]
    pub fn ppi_chgn800_ch30_read(
        &mut self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        Ok(self.group[_reg_array].included[30])
    }
    #[doc = "CH30: Include or exclude channel 30<br>"]
    pub fn ppi_chgn800_ch30_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.group[_reg_array].included[30] = _value)
    }
    #[doc = "CH31: Include or exclude channel 31<br>"]
    pub fn ppi_chgn800_ch31_read(
        &mut self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        Ok(self.group[_reg_array].included[31])
    }
    #[doc = "CH31: Include or exclude channel 31<br>"]
    pub fn ppi_chgn800_ch31_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.group[_reg_array].included[31] = _value)
    }
    #[doc = "TEP: Pointer to task register<br>"]
    pub(crate) fn ppi_forkn_tep0_tep_read(
        &self,
        _forkn: usize,
    ) -> MemResult<u32> {
        todo ! ("read TEP mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "TEP: Pointer to task register<br>"]
    pub(crate) fn ppi_forkn_tep0_tep_write(
        &mut self,
        _forkn: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write TEP mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
}
