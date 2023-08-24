use icicle_vm::cpu::mem::MemResult;
#[derive(Default)]
#[doc = "PPI: Programmable Peripheral Interconnect<br><br>Instances:<br>0x4001f000: PPI<br>"]
pub struct Ppi {
    #[doc = "TODO: implement things here"]
    _todo: (),
}
impl Ppi {
    pub(crate) fn page_to_index(page: u64) -> usize {
        match page {
            262175u64 => 0usize,
            _ => unreachable!(),
        }
    }
    #[doc = "EN: Description cluster\\[0\\]:  Enable channel group 0<br>"]
    pub(crate) fn ppi_tasks_chgn_en0_write(
        &mut self,
        _tasks_chgn: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write ppi_tasks_chgn_en0 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DIS: Description cluster\\[0\\]:  Disable channel group 0<br>"]
    pub(crate) fn ppi_tasks_chgn_dis4_write(
        &mut self,
        _tasks_chgn: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write ppi_tasks_chgn_dis4 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "CH0: Enable or disable channel 0<br>"]
    pub(crate) fn ppi_chen500_ch0_read(&self) -> MemResult<bool> {
        todo!("read CH0 mwrite None write None rac None reset value false")
    }
    #[doc = "CH0: Enable or disable channel 0<br>"]
    pub(crate) fn ppi_chen500_ch0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH0 mwrite None write None rac None reset value false")
    }
    #[doc = "CH1: Enable or disable channel 1<br>"]
    pub(crate) fn ppi_chen500_ch1_read(&self) -> MemResult<bool> {
        todo!("read CH1 mwrite None write None rac None reset value false")
    }
    #[doc = "CH1: Enable or disable channel 1<br>"]
    pub(crate) fn ppi_chen500_ch1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH1 mwrite None write None rac None reset value false")
    }
    #[doc = "CH2: Enable or disable channel 2<br>"]
    pub(crate) fn ppi_chen500_ch2_read(&self) -> MemResult<bool> {
        todo!("read CH2 mwrite None write None rac None reset value false")
    }
    #[doc = "CH2: Enable or disable channel 2<br>"]
    pub(crate) fn ppi_chen500_ch2_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH2 mwrite None write None rac None reset value false")
    }
    #[doc = "CH3: Enable or disable channel 3<br>"]
    pub(crate) fn ppi_chen500_ch3_read(&self) -> MemResult<bool> {
        todo!("read CH3 mwrite None write None rac None reset value false")
    }
    #[doc = "CH3: Enable or disable channel 3<br>"]
    pub(crate) fn ppi_chen500_ch3_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH3 mwrite None write None rac None reset value false")
    }
    #[doc = "CH4: Enable or disable channel 4<br>"]
    pub(crate) fn ppi_chen500_ch4_read(&self) -> MemResult<bool> {
        todo!("read CH4 mwrite None write None rac None reset value false")
    }
    #[doc = "CH4: Enable or disable channel 4<br>"]
    pub(crate) fn ppi_chen500_ch4_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH4 mwrite None write None rac None reset value false")
    }
    #[doc = "CH5: Enable or disable channel 5<br>"]
    pub(crate) fn ppi_chen500_ch5_read(&self) -> MemResult<bool> {
        todo!("read CH5 mwrite None write None rac None reset value false")
    }
    #[doc = "CH5: Enable or disable channel 5<br>"]
    pub(crate) fn ppi_chen500_ch5_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH5 mwrite None write None rac None reset value false")
    }
    #[doc = "CH6: Enable or disable channel 6<br>"]
    pub(crate) fn ppi_chen500_ch6_read(&self) -> MemResult<bool> {
        todo!("read CH6 mwrite None write None rac None reset value false")
    }
    #[doc = "CH6: Enable or disable channel 6<br>"]
    pub(crate) fn ppi_chen500_ch6_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH6 mwrite None write None rac None reset value false")
    }
    #[doc = "CH7: Enable or disable channel 7<br>"]
    pub(crate) fn ppi_chen500_ch7_read(&self) -> MemResult<bool> {
        todo!("read CH7 mwrite None write None rac None reset value false")
    }
    #[doc = "CH7: Enable or disable channel 7<br>"]
    pub(crate) fn ppi_chen500_ch7_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH7 mwrite None write None rac None reset value false")
    }
    #[doc = "CH8: Enable or disable channel 8<br>"]
    pub(crate) fn ppi_chen500_ch8_read(&self) -> MemResult<bool> {
        todo!("read CH8 mwrite None write None rac None reset value false")
    }
    #[doc = "CH8: Enable or disable channel 8<br>"]
    pub(crate) fn ppi_chen500_ch8_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH8 mwrite None write None rac None reset value false")
    }
    #[doc = "CH9: Enable or disable channel 9<br>"]
    pub(crate) fn ppi_chen500_ch9_read(&self) -> MemResult<bool> {
        todo!("read CH9 mwrite None write None rac None reset value false")
    }
    #[doc = "CH9: Enable or disable channel 9<br>"]
    pub(crate) fn ppi_chen500_ch9_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH9 mwrite None write None rac None reset value false")
    }
    #[doc = "CH10: Enable or disable channel 10<br>"]
    pub(crate) fn ppi_chen500_ch10_read(&self) -> MemResult<bool> {
        todo!("read CH10 mwrite None write None rac None reset value false")
    }
    #[doc = "CH10: Enable or disable channel 10<br>"]
    pub(crate) fn ppi_chen500_ch10_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH10 mwrite None write None rac None reset value false")
    }
    #[doc = "CH11: Enable or disable channel 11<br>"]
    pub(crate) fn ppi_chen500_ch11_read(&self) -> MemResult<bool> {
        todo!("read CH11 mwrite None write None rac None reset value false")
    }
    #[doc = "CH11: Enable or disable channel 11<br>"]
    pub(crate) fn ppi_chen500_ch11_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH11 mwrite None write None rac None reset value false")
    }
    #[doc = "CH12: Enable or disable channel 12<br>"]
    pub(crate) fn ppi_chen500_ch12_read(&self) -> MemResult<bool> {
        todo!("read CH12 mwrite None write None rac None reset value false")
    }
    #[doc = "CH12: Enable or disable channel 12<br>"]
    pub(crate) fn ppi_chen500_ch12_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH12 mwrite None write None rac None reset value false")
    }
    #[doc = "CH13: Enable or disable channel 13<br>"]
    pub(crate) fn ppi_chen500_ch13_read(&self) -> MemResult<bool> {
        todo!("read CH13 mwrite None write None rac None reset value false")
    }
    #[doc = "CH13: Enable or disable channel 13<br>"]
    pub(crate) fn ppi_chen500_ch13_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH13 mwrite None write None rac None reset value false")
    }
    #[doc = "CH14: Enable or disable channel 14<br>"]
    pub(crate) fn ppi_chen500_ch14_read(&self) -> MemResult<bool> {
        todo!("read CH14 mwrite None write None rac None reset value false")
    }
    #[doc = "CH14: Enable or disable channel 14<br>"]
    pub(crate) fn ppi_chen500_ch14_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH14 mwrite None write None rac None reset value false")
    }
    #[doc = "CH15: Enable or disable channel 15<br>"]
    pub(crate) fn ppi_chen500_ch15_read(&self) -> MemResult<bool> {
        todo!("read CH15 mwrite None write None rac None reset value false")
    }
    #[doc = "CH15: Enable or disable channel 15<br>"]
    pub(crate) fn ppi_chen500_ch15_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH15 mwrite None write None rac None reset value false")
    }
    #[doc = "CH16: Enable or disable channel 16<br>"]
    pub(crate) fn ppi_chen500_ch16_read(&self) -> MemResult<bool> {
        todo!("read CH16 mwrite None write None rac None reset value false")
    }
    #[doc = "CH16: Enable or disable channel 16<br>"]
    pub(crate) fn ppi_chen500_ch16_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH16 mwrite None write None rac None reset value false")
    }
    #[doc = "CH17: Enable or disable channel 17<br>"]
    pub(crate) fn ppi_chen500_ch17_read(&self) -> MemResult<bool> {
        todo!("read CH17 mwrite None write None rac None reset value false")
    }
    #[doc = "CH17: Enable or disable channel 17<br>"]
    pub(crate) fn ppi_chen500_ch17_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH17 mwrite None write None rac None reset value false")
    }
    #[doc = "CH18: Enable or disable channel 18<br>"]
    pub(crate) fn ppi_chen500_ch18_read(&self) -> MemResult<bool> {
        todo!("read CH18 mwrite None write None rac None reset value false")
    }
    #[doc = "CH18: Enable or disable channel 18<br>"]
    pub(crate) fn ppi_chen500_ch18_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH18 mwrite None write None rac None reset value false")
    }
    #[doc = "CH19: Enable or disable channel 19<br>"]
    pub(crate) fn ppi_chen500_ch19_read(&self) -> MemResult<bool> {
        todo!("read CH19 mwrite None write None rac None reset value false")
    }
    #[doc = "CH19: Enable or disable channel 19<br>"]
    pub(crate) fn ppi_chen500_ch19_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH19 mwrite None write None rac None reset value false")
    }
    #[doc = "CH20: Enable or disable channel 20<br>"]
    pub(crate) fn ppi_chen500_ch20_read(&self) -> MemResult<bool> {
        todo!("read CH20 mwrite None write None rac None reset value false")
    }
    #[doc = "CH20: Enable or disable channel 20<br>"]
    pub(crate) fn ppi_chen500_ch20_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH20 mwrite None write None rac None reset value false")
    }
    #[doc = "CH21: Enable or disable channel 21<br>"]
    pub(crate) fn ppi_chen500_ch21_read(&self) -> MemResult<bool> {
        todo!("read CH21 mwrite None write None rac None reset value false")
    }
    #[doc = "CH21: Enable or disable channel 21<br>"]
    pub(crate) fn ppi_chen500_ch21_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH21 mwrite None write None rac None reset value false")
    }
    #[doc = "CH22: Enable or disable channel 22<br>"]
    pub(crate) fn ppi_chen500_ch22_read(&self) -> MemResult<bool> {
        todo!("read CH22 mwrite None write None rac None reset value false")
    }
    #[doc = "CH22: Enable or disable channel 22<br>"]
    pub(crate) fn ppi_chen500_ch22_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH22 mwrite None write None rac None reset value false")
    }
    #[doc = "CH23: Enable or disable channel 23<br>"]
    pub(crate) fn ppi_chen500_ch23_read(&self) -> MemResult<bool> {
        todo!("read CH23 mwrite None write None rac None reset value false")
    }
    #[doc = "CH23: Enable or disable channel 23<br>"]
    pub(crate) fn ppi_chen500_ch23_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH23 mwrite None write None rac None reset value false")
    }
    #[doc = "CH24: Enable or disable channel 24<br>"]
    pub(crate) fn ppi_chen500_ch24_read(&self) -> MemResult<bool> {
        todo!("read CH24 mwrite None write None rac None reset value false")
    }
    #[doc = "CH24: Enable or disable channel 24<br>"]
    pub(crate) fn ppi_chen500_ch24_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH24 mwrite None write None rac None reset value false")
    }
    #[doc = "CH25: Enable or disable channel 25<br>"]
    pub(crate) fn ppi_chen500_ch25_read(&self) -> MemResult<bool> {
        todo!("read CH25 mwrite None write None rac None reset value false")
    }
    #[doc = "CH25: Enable or disable channel 25<br>"]
    pub(crate) fn ppi_chen500_ch25_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH25 mwrite None write None rac None reset value false")
    }
    #[doc = "CH26: Enable or disable channel 26<br>"]
    pub(crate) fn ppi_chen500_ch26_read(&self) -> MemResult<bool> {
        todo!("read CH26 mwrite None write None rac None reset value false")
    }
    #[doc = "CH26: Enable or disable channel 26<br>"]
    pub(crate) fn ppi_chen500_ch26_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH26 mwrite None write None rac None reset value false")
    }
    #[doc = "CH27: Enable or disable channel 27<br>"]
    pub(crate) fn ppi_chen500_ch27_read(&self) -> MemResult<bool> {
        todo!("read CH27 mwrite None write None rac None reset value false")
    }
    #[doc = "CH27: Enable or disable channel 27<br>"]
    pub(crate) fn ppi_chen500_ch27_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH27 mwrite None write None rac None reset value false")
    }
    #[doc = "CH28: Enable or disable channel 28<br>"]
    pub(crate) fn ppi_chen500_ch28_read(&self) -> MemResult<bool> {
        todo!("read CH28 mwrite None write None rac None reset value false")
    }
    #[doc = "CH28: Enable or disable channel 28<br>"]
    pub(crate) fn ppi_chen500_ch28_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH28 mwrite None write None rac None reset value false")
    }
    #[doc = "CH29: Enable or disable channel 29<br>"]
    pub(crate) fn ppi_chen500_ch29_read(&self) -> MemResult<bool> {
        todo!("read CH29 mwrite None write None rac None reset value false")
    }
    #[doc = "CH29: Enable or disable channel 29<br>"]
    pub(crate) fn ppi_chen500_ch29_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH29 mwrite None write None rac None reset value false")
    }
    #[doc = "CH30: Enable or disable channel 30<br>"]
    pub(crate) fn ppi_chen500_ch30_read(&self) -> MemResult<bool> {
        todo!("read CH30 mwrite None write None rac None reset value false")
    }
    #[doc = "CH30: Enable or disable channel 30<br>"]
    pub(crate) fn ppi_chen500_ch30_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH30 mwrite None write None rac None reset value false")
    }
    #[doc = "CH31: Enable or disable channel 31<br>"]
    pub(crate) fn ppi_chen500_ch31_read(&self) -> MemResult<bool> {
        todo!("read CH31 mwrite None write None rac None reset value false")
    }
    #[doc = "CH31: Enable or disable channel 31<br>"]
    pub(crate) fn ppi_chen500_ch31_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH31 mwrite None write None rac None reset value false")
    }
    #[doc = "CH0: Channel 0 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch0_read(&self) -> MemResult<bool> {
        todo!("read CH0 mwrite None write None rac None reset value false")
    }
    #[doc = "CH0: Channel 0 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH0 mwrite None write None rac None reset value false")
    }
    #[doc = "CH1: Channel 1 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch1_read(&self) -> MemResult<bool> {
        todo!("read CH1 mwrite None write None rac None reset value false")
    }
    #[doc = "CH1: Channel 1 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH1 mwrite None write None rac None reset value false")
    }
    #[doc = "CH2: Channel 2 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch2_read(&self) -> MemResult<bool> {
        todo!("read CH2 mwrite None write None rac None reset value false")
    }
    #[doc = "CH2: Channel 2 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch2_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH2 mwrite None write None rac None reset value false")
    }
    #[doc = "CH3: Channel 3 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch3_read(&self) -> MemResult<bool> {
        todo!("read CH3 mwrite None write None rac None reset value false")
    }
    #[doc = "CH3: Channel 3 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch3_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH3 mwrite None write None rac None reset value false")
    }
    #[doc = "CH4: Channel 4 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch4_read(&self) -> MemResult<bool> {
        todo!("read CH4 mwrite None write None rac None reset value false")
    }
    #[doc = "CH4: Channel 4 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch4_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH4 mwrite None write None rac None reset value false")
    }
    #[doc = "CH5: Channel 5 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch5_read(&self) -> MemResult<bool> {
        todo!("read CH5 mwrite None write None rac None reset value false")
    }
    #[doc = "CH5: Channel 5 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch5_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH5 mwrite None write None rac None reset value false")
    }
    #[doc = "CH6: Channel 6 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch6_read(&self) -> MemResult<bool> {
        todo!("read CH6 mwrite None write None rac None reset value false")
    }
    #[doc = "CH6: Channel 6 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch6_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH6 mwrite None write None rac None reset value false")
    }
    #[doc = "CH7: Channel 7 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch7_read(&self) -> MemResult<bool> {
        todo!("read CH7 mwrite None write None rac None reset value false")
    }
    #[doc = "CH7: Channel 7 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch7_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH7 mwrite None write None rac None reset value false")
    }
    #[doc = "CH8: Channel 8 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch8_read(&self) -> MemResult<bool> {
        todo!("read CH8 mwrite None write None rac None reset value false")
    }
    #[doc = "CH8: Channel 8 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch8_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH8 mwrite None write None rac None reset value false")
    }
    #[doc = "CH9: Channel 9 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch9_read(&self) -> MemResult<bool> {
        todo!("read CH9 mwrite None write None rac None reset value false")
    }
    #[doc = "CH9: Channel 9 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch9_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH9 mwrite None write None rac None reset value false")
    }
    #[doc = "CH10: Channel 10 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch10_read(&self) -> MemResult<bool> {
        todo!("read CH10 mwrite None write None rac None reset value false")
    }
    #[doc = "CH10: Channel 10 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch10_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH10 mwrite None write None rac None reset value false")
    }
    #[doc = "CH11: Channel 11 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch11_read(&self) -> MemResult<bool> {
        todo!("read CH11 mwrite None write None rac None reset value false")
    }
    #[doc = "CH11: Channel 11 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch11_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH11 mwrite None write None rac None reset value false")
    }
    #[doc = "CH12: Channel 12 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch12_read(&self) -> MemResult<bool> {
        todo!("read CH12 mwrite None write None rac None reset value false")
    }
    #[doc = "CH12: Channel 12 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch12_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH12 mwrite None write None rac None reset value false")
    }
    #[doc = "CH13: Channel 13 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch13_read(&self) -> MemResult<bool> {
        todo!("read CH13 mwrite None write None rac None reset value false")
    }
    #[doc = "CH13: Channel 13 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch13_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH13 mwrite None write None rac None reset value false")
    }
    #[doc = "CH14: Channel 14 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch14_read(&self) -> MemResult<bool> {
        todo!("read CH14 mwrite None write None rac None reset value false")
    }
    #[doc = "CH14: Channel 14 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch14_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH14 mwrite None write None rac None reset value false")
    }
    #[doc = "CH15: Channel 15 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch15_read(&self) -> MemResult<bool> {
        todo!("read CH15 mwrite None write None rac None reset value false")
    }
    #[doc = "CH15: Channel 15 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch15_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH15 mwrite None write None rac None reset value false")
    }
    #[doc = "CH16: Channel 16 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch16_read(&self) -> MemResult<bool> {
        todo!("read CH16 mwrite None write None rac None reset value false")
    }
    #[doc = "CH16: Channel 16 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch16_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH16 mwrite None write None rac None reset value false")
    }
    #[doc = "CH17: Channel 17 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch17_read(&self) -> MemResult<bool> {
        todo!("read CH17 mwrite None write None rac None reset value false")
    }
    #[doc = "CH17: Channel 17 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch17_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH17 mwrite None write None rac None reset value false")
    }
    #[doc = "CH18: Channel 18 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch18_read(&self) -> MemResult<bool> {
        todo!("read CH18 mwrite None write None rac None reset value false")
    }
    #[doc = "CH18: Channel 18 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch18_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH18 mwrite None write None rac None reset value false")
    }
    #[doc = "CH19: Channel 19 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch19_read(&self) -> MemResult<bool> {
        todo!("read CH19 mwrite None write None rac None reset value false")
    }
    #[doc = "CH19: Channel 19 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch19_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH19 mwrite None write None rac None reset value false")
    }
    #[doc = "CH20: Channel 20 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch20_read(&self) -> MemResult<bool> {
        todo!("read CH20 mwrite None write None rac None reset value false")
    }
    #[doc = "CH20: Channel 20 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch20_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH20 mwrite None write None rac None reset value false")
    }
    #[doc = "CH21: Channel 21 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch21_read(&self) -> MemResult<bool> {
        todo!("read CH21 mwrite None write None rac None reset value false")
    }
    #[doc = "CH21: Channel 21 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch21_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH21 mwrite None write None rac None reset value false")
    }
    #[doc = "CH22: Channel 22 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch22_read(&self) -> MemResult<bool> {
        todo!("read CH22 mwrite None write None rac None reset value false")
    }
    #[doc = "CH22: Channel 22 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch22_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH22 mwrite None write None rac None reset value false")
    }
    #[doc = "CH23: Channel 23 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch23_read(&self) -> MemResult<bool> {
        todo!("read CH23 mwrite None write None rac None reset value false")
    }
    #[doc = "CH23: Channel 23 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch23_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH23 mwrite None write None rac None reset value false")
    }
    #[doc = "CH24: Channel 24 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch24_read(&self) -> MemResult<bool> {
        todo!("read CH24 mwrite None write None rac None reset value false")
    }
    #[doc = "CH24: Channel 24 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch24_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH24 mwrite None write None rac None reset value false")
    }
    #[doc = "CH25: Channel 25 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch25_read(&self) -> MemResult<bool> {
        todo!("read CH25 mwrite None write None rac None reset value false")
    }
    #[doc = "CH25: Channel 25 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch25_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH25 mwrite None write None rac None reset value false")
    }
    #[doc = "CH26: Channel 26 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch26_read(&self) -> MemResult<bool> {
        todo!("read CH26 mwrite None write None rac None reset value false")
    }
    #[doc = "CH26: Channel 26 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch26_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH26 mwrite None write None rac None reset value false")
    }
    #[doc = "CH27: Channel 27 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch27_read(&self) -> MemResult<bool> {
        todo!("read CH27 mwrite None write None rac None reset value false")
    }
    #[doc = "CH27: Channel 27 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch27_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH27 mwrite None write None rac None reset value false")
    }
    #[doc = "CH28: Channel 28 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch28_read(&self) -> MemResult<bool> {
        todo!("read CH28 mwrite None write None rac None reset value false")
    }
    #[doc = "CH28: Channel 28 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch28_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH28 mwrite None write None rac None reset value false")
    }
    #[doc = "CH29: Channel 29 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch29_read(&self) -> MemResult<bool> {
        todo!("read CH29 mwrite None write None rac None reset value false")
    }
    #[doc = "CH29: Channel 29 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch29_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH29 mwrite None write None rac None reset value false")
    }
    #[doc = "CH30: Channel 30 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch30_read(&self) -> MemResult<bool> {
        todo!("read CH30 mwrite None write None rac None reset value false")
    }
    #[doc = "CH30: Channel 30 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch30_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH30 mwrite None write None rac None reset value false")
    }
    #[doc = "CH31: Channel 31 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch31_read(&self) -> MemResult<bool> {
        todo!("read CH31 mwrite None write None rac None reset value false")
    }
    #[doc = "CH31: Channel 31 enable set register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenset504_ch31_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH31 mwrite None write None rac None reset value false")
    }
    #[doc = "CH0: Channel 0 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch0_read(&self) -> MemResult<bool> {
        todo!("read CH0 mwrite None write None rac None reset value false")
    }
    #[doc = "CH0: Channel 0 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH0 mwrite None write None rac None reset value false")
    }
    #[doc = "CH1: Channel 1 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch1_read(&self) -> MemResult<bool> {
        todo!("read CH1 mwrite None write None rac None reset value false")
    }
    #[doc = "CH1: Channel 1 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH1 mwrite None write None rac None reset value false")
    }
    #[doc = "CH2: Channel 2 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch2_read(&self) -> MemResult<bool> {
        todo!("read CH2 mwrite None write None rac None reset value false")
    }
    #[doc = "CH2: Channel 2 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch2_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH2 mwrite None write None rac None reset value false")
    }
    #[doc = "CH3: Channel 3 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch3_read(&self) -> MemResult<bool> {
        todo!("read CH3 mwrite None write None rac None reset value false")
    }
    #[doc = "CH3: Channel 3 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch3_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH3 mwrite None write None rac None reset value false")
    }
    #[doc = "CH4: Channel 4 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch4_read(&self) -> MemResult<bool> {
        todo!("read CH4 mwrite None write None rac None reset value false")
    }
    #[doc = "CH4: Channel 4 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch4_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH4 mwrite None write None rac None reset value false")
    }
    #[doc = "CH5: Channel 5 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch5_read(&self) -> MemResult<bool> {
        todo!("read CH5 mwrite None write None rac None reset value false")
    }
    #[doc = "CH5: Channel 5 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch5_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH5 mwrite None write None rac None reset value false")
    }
    #[doc = "CH6: Channel 6 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch6_read(&self) -> MemResult<bool> {
        todo!("read CH6 mwrite None write None rac None reset value false")
    }
    #[doc = "CH6: Channel 6 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch6_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH6 mwrite None write None rac None reset value false")
    }
    #[doc = "CH7: Channel 7 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch7_read(&self) -> MemResult<bool> {
        todo!("read CH7 mwrite None write None rac None reset value false")
    }
    #[doc = "CH7: Channel 7 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch7_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH7 mwrite None write None rac None reset value false")
    }
    #[doc = "CH8: Channel 8 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch8_read(&self) -> MemResult<bool> {
        todo!("read CH8 mwrite None write None rac None reset value false")
    }
    #[doc = "CH8: Channel 8 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch8_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH8 mwrite None write None rac None reset value false")
    }
    #[doc = "CH9: Channel 9 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch9_read(&self) -> MemResult<bool> {
        todo!("read CH9 mwrite None write None rac None reset value false")
    }
    #[doc = "CH9: Channel 9 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch9_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH9 mwrite None write None rac None reset value false")
    }
    #[doc = "CH10: Channel 10 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch10_read(&self) -> MemResult<bool> {
        todo!("read CH10 mwrite None write None rac None reset value false")
    }
    #[doc = "CH10: Channel 10 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch10_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH10 mwrite None write None rac None reset value false")
    }
    #[doc = "CH11: Channel 11 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch11_read(&self) -> MemResult<bool> {
        todo!("read CH11 mwrite None write None rac None reset value false")
    }
    #[doc = "CH11: Channel 11 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch11_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH11 mwrite None write None rac None reset value false")
    }
    #[doc = "CH12: Channel 12 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch12_read(&self) -> MemResult<bool> {
        todo!("read CH12 mwrite None write None rac None reset value false")
    }
    #[doc = "CH12: Channel 12 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch12_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH12 mwrite None write None rac None reset value false")
    }
    #[doc = "CH13: Channel 13 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch13_read(&self) -> MemResult<bool> {
        todo!("read CH13 mwrite None write None rac None reset value false")
    }
    #[doc = "CH13: Channel 13 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch13_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH13 mwrite None write None rac None reset value false")
    }
    #[doc = "CH14: Channel 14 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch14_read(&self) -> MemResult<bool> {
        todo!("read CH14 mwrite None write None rac None reset value false")
    }
    #[doc = "CH14: Channel 14 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch14_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH14 mwrite None write None rac None reset value false")
    }
    #[doc = "CH15: Channel 15 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch15_read(&self) -> MemResult<bool> {
        todo!("read CH15 mwrite None write None rac None reset value false")
    }
    #[doc = "CH15: Channel 15 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch15_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH15 mwrite None write None rac None reset value false")
    }
    #[doc = "CH16: Channel 16 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch16_read(&self) -> MemResult<bool> {
        todo!("read CH16 mwrite None write None rac None reset value false")
    }
    #[doc = "CH16: Channel 16 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch16_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH16 mwrite None write None rac None reset value false")
    }
    #[doc = "CH17: Channel 17 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch17_read(&self) -> MemResult<bool> {
        todo!("read CH17 mwrite None write None rac None reset value false")
    }
    #[doc = "CH17: Channel 17 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch17_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH17 mwrite None write None rac None reset value false")
    }
    #[doc = "CH18: Channel 18 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch18_read(&self) -> MemResult<bool> {
        todo!("read CH18 mwrite None write None rac None reset value false")
    }
    #[doc = "CH18: Channel 18 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch18_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH18 mwrite None write None rac None reset value false")
    }
    #[doc = "CH19: Channel 19 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch19_read(&self) -> MemResult<bool> {
        todo!("read CH19 mwrite None write None rac None reset value false")
    }
    #[doc = "CH19: Channel 19 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch19_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH19 mwrite None write None rac None reset value false")
    }
    #[doc = "CH20: Channel 20 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch20_read(&self) -> MemResult<bool> {
        todo!("read CH20 mwrite None write None rac None reset value false")
    }
    #[doc = "CH20: Channel 20 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch20_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH20 mwrite None write None rac None reset value false")
    }
    #[doc = "CH21: Channel 21 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch21_read(&self) -> MemResult<bool> {
        todo!("read CH21 mwrite None write None rac None reset value false")
    }
    #[doc = "CH21: Channel 21 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch21_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH21 mwrite None write None rac None reset value false")
    }
    #[doc = "CH22: Channel 22 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch22_read(&self) -> MemResult<bool> {
        todo!("read CH22 mwrite None write None rac None reset value false")
    }
    #[doc = "CH22: Channel 22 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch22_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH22 mwrite None write None rac None reset value false")
    }
    #[doc = "CH23: Channel 23 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch23_read(&self) -> MemResult<bool> {
        todo!("read CH23 mwrite None write None rac None reset value false")
    }
    #[doc = "CH23: Channel 23 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch23_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH23 mwrite None write None rac None reset value false")
    }
    #[doc = "CH24: Channel 24 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch24_read(&self) -> MemResult<bool> {
        todo!("read CH24 mwrite None write None rac None reset value false")
    }
    #[doc = "CH24: Channel 24 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch24_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH24 mwrite None write None rac None reset value false")
    }
    #[doc = "CH25: Channel 25 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch25_read(&self) -> MemResult<bool> {
        todo!("read CH25 mwrite None write None rac None reset value false")
    }
    #[doc = "CH25: Channel 25 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch25_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH25 mwrite None write None rac None reset value false")
    }
    #[doc = "CH26: Channel 26 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch26_read(&self) -> MemResult<bool> {
        todo!("read CH26 mwrite None write None rac None reset value false")
    }
    #[doc = "CH26: Channel 26 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch26_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH26 mwrite None write None rac None reset value false")
    }
    #[doc = "CH27: Channel 27 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch27_read(&self) -> MemResult<bool> {
        todo!("read CH27 mwrite None write None rac None reset value false")
    }
    #[doc = "CH27: Channel 27 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch27_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH27 mwrite None write None rac None reset value false")
    }
    #[doc = "CH28: Channel 28 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch28_read(&self) -> MemResult<bool> {
        todo!("read CH28 mwrite None write None rac None reset value false")
    }
    #[doc = "CH28: Channel 28 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch28_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH28 mwrite None write None rac None reset value false")
    }
    #[doc = "CH29: Channel 29 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch29_read(&self) -> MemResult<bool> {
        todo!("read CH29 mwrite None write None rac None reset value false")
    }
    #[doc = "CH29: Channel 29 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch29_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH29 mwrite None write None rac None reset value false")
    }
    #[doc = "CH30: Channel 30 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch30_read(&self) -> MemResult<bool> {
        todo!("read CH30 mwrite None write None rac None reset value false")
    }
    #[doc = "CH30: Channel 30 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch30_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH30 mwrite None write None rac None reset value false")
    }
    #[doc = "CH31: Channel 31 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch31_read(&self) -> MemResult<bool> {
        todo!("read CH31 mwrite None write None rac None reset value false")
    }
    #[doc = "CH31: Channel 31 enable clear register.  Writing '0' has no effect<br>"]
    pub(crate) fn ppi_chenclr508_ch31_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH31 mwrite None write None rac None reset value false")
    }
    #[doc = "EEP: Pointer to event register. Accepts only addresses to registers from the Event group.<br>"]
    pub(crate) fn ppi_chn_eep0_eep_read(&self, _chn: usize) -> MemResult<u32> {
        todo ! ("read EEP mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "EEP: Pointer to event register. Accepts only addresses to registers from the Event group.<br>"]
    pub(crate) fn ppi_chn_eep0_eep_write(
        &mut self,
        _chn: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write EEP mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "TEP: Pointer to task register. Accepts only addresses to registers from the Task group.<br>"]
    pub(crate) fn ppi_chn_tep4_tep_read(&self, _chn: usize) -> MemResult<u32> {
        todo ! ("read TEP mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "TEP: Pointer to task register. Accepts only addresses to registers from the Task group.<br>"]
    pub(crate) fn ppi_chn_tep4_tep_write(
        &mut self,
        _chn: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write TEP mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "CH0: Include or exclude channel 0<br>"]
    pub(crate) fn ppi_chgn800_ch0_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read CH0 mwrite None write None rac None reset value false")
    }
    #[doc = "CH0: Include or exclude channel 0<br>"]
    pub(crate) fn ppi_chgn800_ch0_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH0 mwrite None write None rac None reset value false")
    }
    #[doc = "CH1: Include or exclude channel 1<br>"]
    pub(crate) fn ppi_chgn800_ch1_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read CH1 mwrite None write None rac None reset value false")
    }
    #[doc = "CH1: Include or exclude channel 1<br>"]
    pub(crate) fn ppi_chgn800_ch1_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH1 mwrite None write None rac None reset value false")
    }
    #[doc = "CH2: Include or exclude channel 2<br>"]
    pub(crate) fn ppi_chgn800_ch2_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read CH2 mwrite None write None rac None reset value false")
    }
    #[doc = "CH2: Include or exclude channel 2<br>"]
    pub(crate) fn ppi_chgn800_ch2_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH2 mwrite None write None rac None reset value false")
    }
    #[doc = "CH3: Include or exclude channel 3<br>"]
    pub(crate) fn ppi_chgn800_ch3_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read CH3 mwrite None write None rac None reset value false")
    }
    #[doc = "CH3: Include or exclude channel 3<br>"]
    pub(crate) fn ppi_chgn800_ch3_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH3 mwrite None write None rac None reset value false")
    }
    #[doc = "CH4: Include or exclude channel 4<br>"]
    pub(crate) fn ppi_chgn800_ch4_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read CH4 mwrite None write None rac None reset value false")
    }
    #[doc = "CH4: Include or exclude channel 4<br>"]
    pub(crate) fn ppi_chgn800_ch4_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH4 mwrite None write None rac None reset value false")
    }
    #[doc = "CH5: Include or exclude channel 5<br>"]
    pub(crate) fn ppi_chgn800_ch5_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read CH5 mwrite None write None rac None reset value false")
    }
    #[doc = "CH5: Include or exclude channel 5<br>"]
    pub(crate) fn ppi_chgn800_ch5_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH5 mwrite None write None rac None reset value false")
    }
    #[doc = "CH6: Include or exclude channel 6<br>"]
    pub(crate) fn ppi_chgn800_ch6_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read CH6 mwrite None write None rac None reset value false")
    }
    #[doc = "CH6: Include or exclude channel 6<br>"]
    pub(crate) fn ppi_chgn800_ch6_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH6 mwrite None write None rac None reset value false")
    }
    #[doc = "CH7: Include or exclude channel 7<br>"]
    pub(crate) fn ppi_chgn800_ch7_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read CH7 mwrite None write None rac None reset value false")
    }
    #[doc = "CH7: Include or exclude channel 7<br>"]
    pub(crate) fn ppi_chgn800_ch7_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH7 mwrite None write None rac None reset value false")
    }
    #[doc = "CH8: Include or exclude channel 8<br>"]
    pub(crate) fn ppi_chgn800_ch8_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read CH8 mwrite None write None rac None reset value false")
    }
    #[doc = "CH8: Include or exclude channel 8<br>"]
    pub(crate) fn ppi_chgn800_ch8_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH8 mwrite None write None rac None reset value false")
    }
    #[doc = "CH9: Include or exclude channel 9<br>"]
    pub(crate) fn ppi_chgn800_ch9_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read CH9 mwrite None write None rac None reset value false")
    }
    #[doc = "CH9: Include or exclude channel 9<br>"]
    pub(crate) fn ppi_chgn800_ch9_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH9 mwrite None write None rac None reset value false")
    }
    #[doc = "CH10: Include or exclude channel 10<br>"]
    pub(crate) fn ppi_chgn800_ch10_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read CH10 mwrite None write None rac None reset value false")
    }
    #[doc = "CH10: Include or exclude channel 10<br>"]
    pub(crate) fn ppi_chgn800_ch10_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH10 mwrite None write None rac None reset value false")
    }
    #[doc = "CH11: Include or exclude channel 11<br>"]
    pub(crate) fn ppi_chgn800_ch11_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read CH11 mwrite None write None rac None reset value false")
    }
    #[doc = "CH11: Include or exclude channel 11<br>"]
    pub(crate) fn ppi_chgn800_ch11_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH11 mwrite None write None rac None reset value false")
    }
    #[doc = "CH12: Include or exclude channel 12<br>"]
    pub(crate) fn ppi_chgn800_ch12_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read CH12 mwrite None write None rac None reset value false")
    }
    #[doc = "CH12: Include or exclude channel 12<br>"]
    pub(crate) fn ppi_chgn800_ch12_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH12 mwrite None write None rac None reset value false")
    }
    #[doc = "CH13: Include or exclude channel 13<br>"]
    pub(crate) fn ppi_chgn800_ch13_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read CH13 mwrite None write None rac None reset value false")
    }
    #[doc = "CH13: Include or exclude channel 13<br>"]
    pub(crate) fn ppi_chgn800_ch13_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH13 mwrite None write None rac None reset value false")
    }
    #[doc = "CH14: Include or exclude channel 14<br>"]
    pub(crate) fn ppi_chgn800_ch14_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read CH14 mwrite None write None rac None reset value false")
    }
    #[doc = "CH14: Include or exclude channel 14<br>"]
    pub(crate) fn ppi_chgn800_ch14_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH14 mwrite None write None rac None reset value false")
    }
    #[doc = "CH15: Include or exclude channel 15<br>"]
    pub(crate) fn ppi_chgn800_ch15_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read CH15 mwrite None write None rac None reset value false")
    }
    #[doc = "CH15: Include or exclude channel 15<br>"]
    pub(crate) fn ppi_chgn800_ch15_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH15 mwrite None write None rac None reset value false")
    }
    #[doc = "CH16: Include or exclude channel 16<br>"]
    pub(crate) fn ppi_chgn800_ch16_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read CH16 mwrite None write None rac None reset value false")
    }
    #[doc = "CH16: Include or exclude channel 16<br>"]
    pub(crate) fn ppi_chgn800_ch16_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH16 mwrite None write None rac None reset value false")
    }
    #[doc = "CH17: Include or exclude channel 17<br>"]
    pub(crate) fn ppi_chgn800_ch17_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read CH17 mwrite None write None rac None reset value false")
    }
    #[doc = "CH17: Include or exclude channel 17<br>"]
    pub(crate) fn ppi_chgn800_ch17_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH17 mwrite None write None rac None reset value false")
    }
    #[doc = "CH18: Include or exclude channel 18<br>"]
    pub(crate) fn ppi_chgn800_ch18_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read CH18 mwrite None write None rac None reset value false")
    }
    #[doc = "CH18: Include or exclude channel 18<br>"]
    pub(crate) fn ppi_chgn800_ch18_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH18 mwrite None write None rac None reset value false")
    }
    #[doc = "CH19: Include or exclude channel 19<br>"]
    pub(crate) fn ppi_chgn800_ch19_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read CH19 mwrite None write None rac None reset value false")
    }
    #[doc = "CH19: Include or exclude channel 19<br>"]
    pub(crate) fn ppi_chgn800_ch19_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH19 mwrite None write None rac None reset value false")
    }
    #[doc = "CH20: Include or exclude channel 20<br>"]
    pub(crate) fn ppi_chgn800_ch20_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read CH20 mwrite None write None rac None reset value false")
    }
    #[doc = "CH20: Include or exclude channel 20<br>"]
    pub(crate) fn ppi_chgn800_ch20_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH20 mwrite None write None rac None reset value false")
    }
    #[doc = "CH21: Include or exclude channel 21<br>"]
    pub(crate) fn ppi_chgn800_ch21_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read CH21 mwrite None write None rac None reset value false")
    }
    #[doc = "CH21: Include or exclude channel 21<br>"]
    pub(crate) fn ppi_chgn800_ch21_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH21 mwrite None write None rac None reset value false")
    }
    #[doc = "CH22: Include or exclude channel 22<br>"]
    pub(crate) fn ppi_chgn800_ch22_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read CH22 mwrite None write None rac None reset value false")
    }
    #[doc = "CH22: Include or exclude channel 22<br>"]
    pub(crate) fn ppi_chgn800_ch22_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH22 mwrite None write None rac None reset value false")
    }
    #[doc = "CH23: Include or exclude channel 23<br>"]
    pub(crate) fn ppi_chgn800_ch23_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read CH23 mwrite None write None rac None reset value false")
    }
    #[doc = "CH23: Include or exclude channel 23<br>"]
    pub(crate) fn ppi_chgn800_ch23_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH23 mwrite None write None rac None reset value false")
    }
    #[doc = "CH24: Include or exclude channel 24<br>"]
    pub(crate) fn ppi_chgn800_ch24_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read CH24 mwrite None write None rac None reset value false")
    }
    #[doc = "CH24: Include or exclude channel 24<br>"]
    pub(crate) fn ppi_chgn800_ch24_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH24 mwrite None write None rac None reset value false")
    }
    #[doc = "CH25: Include or exclude channel 25<br>"]
    pub(crate) fn ppi_chgn800_ch25_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read CH25 mwrite None write None rac None reset value false")
    }
    #[doc = "CH25: Include or exclude channel 25<br>"]
    pub(crate) fn ppi_chgn800_ch25_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH25 mwrite None write None rac None reset value false")
    }
    #[doc = "CH26: Include or exclude channel 26<br>"]
    pub(crate) fn ppi_chgn800_ch26_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read CH26 mwrite None write None rac None reset value false")
    }
    #[doc = "CH26: Include or exclude channel 26<br>"]
    pub(crate) fn ppi_chgn800_ch26_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH26 mwrite None write None rac None reset value false")
    }
    #[doc = "CH27: Include or exclude channel 27<br>"]
    pub(crate) fn ppi_chgn800_ch27_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read CH27 mwrite None write None rac None reset value false")
    }
    #[doc = "CH27: Include or exclude channel 27<br>"]
    pub(crate) fn ppi_chgn800_ch27_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH27 mwrite None write None rac None reset value false")
    }
    #[doc = "CH28: Include or exclude channel 28<br>"]
    pub(crate) fn ppi_chgn800_ch28_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read CH28 mwrite None write None rac None reset value false")
    }
    #[doc = "CH28: Include or exclude channel 28<br>"]
    pub(crate) fn ppi_chgn800_ch28_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH28 mwrite None write None rac None reset value false")
    }
    #[doc = "CH29: Include or exclude channel 29<br>"]
    pub(crate) fn ppi_chgn800_ch29_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read CH29 mwrite None write None rac None reset value false")
    }
    #[doc = "CH29: Include or exclude channel 29<br>"]
    pub(crate) fn ppi_chgn800_ch29_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH29 mwrite None write None rac None reset value false")
    }
    #[doc = "CH30: Include or exclude channel 30<br>"]
    pub(crate) fn ppi_chgn800_ch30_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read CH30 mwrite None write None rac None reset value false")
    }
    #[doc = "CH30: Include or exclude channel 30<br>"]
    pub(crate) fn ppi_chgn800_ch30_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH30 mwrite None write None rac None reset value false")
    }
    #[doc = "CH31: Include or exclude channel 31<br>"]
    pub(crate) fn ppi_chgn800_ch31_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read CH31 mwrite None write None rac None reset value false")
    }
    #[doc = "CH31: Include or exclude channel 31<br>"]
    pub(crate) fn ppi_chgn800_ch31_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CH31 mwrite None write None rac None reset value false")
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
