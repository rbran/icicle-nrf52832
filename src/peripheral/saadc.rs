use icicle_vm::cpu::mem::MemResult;
#[derive(Default)]
#[doc = "SAADC: Analog to Digital Converter<br><br>Instances:<br>0x40007000: SAADC<br>"]
pub struct Saadc {
    #[doc = "TODO: implement things here"]
    _todo: (),
}
impl Saadc {
    pub(crate) fn page_to_index(page: u64) -> usize {
        match page {
            262151u64 => 0usize,
            _ => unreachable!(),
        }
    }
    #[doc = "TASKS_START: Start the ADC and prepare the result buffer in RAM<br>"]
    pub(crate) fn saadc_tasks_start0_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write saadc_tasks_start0 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_SAMPLE: Take one ADC sample, if scan is enabled all channels are sampled<br>"]
    pub(crate) fn saadc_tasks_sample4_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write saadc_tasks_sample4 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_STOP: Stop the ADC and terminate any on-going conversion<br>"]
    pub(crate) fn saadc_tasks_stop8_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write saadc_tasks_stop8 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_CALIBRATEOFFSET: Starts offset auto-calibration<br>"]
    pub(crate) fn saadc_tasks_calibrateoffsetc_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write saadc_tasks_calibrateoffsetc mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_STARTED: The ADC has started<br>"]
    pub(crate) fn saadc_events_started100_read(&self) -> MemResult<u32> {
        todo ! ("read saadc_events_started100 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_STARTED: The ADC has started<br>"]
    pub(crate) fn saadc_events_started100_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write saadc_events_started100 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_END: The ADC has filled up the Result buffer<br>"]
    pub(crate) fn saadc_events_end104_read(&self) -> MemResult<u32> {
        todo ! ("read saadc_events_end104 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_END: The ADC has filled up the Result buffer<br>"]
    pub(crate) fn saadc_events_end104_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write saadc_events_end104 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_DONE: A conversion task has been completed. Depending on the mode, multiple conversions might be needed for a result to be transferred to RAM.<br>"]
    pub(crate) fn saadc_events_done108_read(&self) -> MemResult<u32> {
        todo ! ("read saadc_events_done108 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_DONE: A conversion task has been completed. Depending on the mode, multiple conversions might be needed for a result to be transferred to RAM.<br>"]
    pub(crate) fn saadc_events_done108_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write saadc_events_done108 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_RESULTDONE: A result is ready to get transferred to RAM.<br>"]
    pub(crate) fn saadc_events_resultdone10c_read(&self) -> MemResult<u32> {
        todo ! ("read saadc_events_resultdone10c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_RESULTDONE: A result is ready to get transferred to RAM.<br>"]
    pub(crate) fn saadc_events_resultdone10c_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write saadc_events_resultdone10c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_CALIBRATEDONE: Calibration is complete<br>"]
    pub(crate) fn saadc_events_calibratedone110_read(&self) -> MemResult<u32> {
        todo ! ("read saadc_events_calibratedone110 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_CALIBRATEDONE: Calibration is complete<br>"]
    pub(crate) fn saadc_events_calibratedone110_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write saadc_events_calibratedone110 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_STOPPED: The ADC has stopped<br>"]
    pub(crate) fn saadc_events_stopped114_read(&self) -> MemResult<u32> {
        todo ! ("read saadc_events_stopped114 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_STOPPED: The ADC has stopped<br>"]
    pub(crate) fn saadc_events_stopped114_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write saadc_events_stopped114 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "LIMITH: Description cluster\\[0\\]:  Last results is equal or above CH\\[0\\].LIMIT.HIGH<br>"]
    pub(crate) fn saadc_events_chn_limith0_read(
        &self,
        _events_chn: usize,
    ) -> MemResult<u32> {
        todo ! ("read saadc_events_chn_limith0 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "LIMITH: Description cluster\\[0\\]:  Last results is equal or above CH\\[0\\].LIMIT.HIGH<br>"]
    pub(crate) fn saadc_events_chn_limith0_write(
        &mut self,
        _events_chn: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write saadc_events_chn_limith0 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "LIMITL: Description cluster\\[0\\]:  Last results is equal or below CH\\[0\\].LIMIT.LOW<br>"]
    pub(crate) fn saadc_events_chn_limitl4_read(
        &self,
        _events_chn: usize,
    ) -> MemResult<u32> {
        todo ! ("read saadc_events_chn_limitl4 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "LIMITL: Description cluster\\[0\\]:  Last results is equal or below CH\\[0\\].LIMIT.LOW<br>"]
    pub(crate) fn saadc_events_chn_limitl4_write(
        &mut self,
        _events_chn: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write saadc_events_chn_limitl4 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "STARTED: Enable or disable interrupt for STARTED event<br>"]
    pub(crate) fn saadc_inten300_started_read(&self) -> MemResult<bool> {
        todo!("read STARTED mwrite None write None rac None reset value false")
    }
    #[doc = "STARTED: Enable or disable interrupt for STARTED event<br>"]
    pub(crate) fn saadc_inten300_started_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write STARTED mwrite None write None rac None reset value false")
    }
    #[doc = "END: Enable or disable interrupt for END event<br>"]
    pub(crate) fn saadc_inten300_end_read(&self) -> MemResult<bool> {
        todo!("read END mwrite None write None rac None reset value false")
    }
    #[doc = "END: Enable or disable interrupt for END event<br>"]
    pub(crate) fn saadc_inten300_end_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write END mwrite None write None rac None reset value false")
    }
    #[doc = "DONE: Enable or disable interrupt for DONE event<br>"]
    pub(crate) fn saadc_inten300_done_read(&self) -> MemResult<bool> {
        todo!("read DONE mwrite None write None rac None reset value false")
    }
    #[doc = "DONE: Enable or disable interrupt for DONE event<br>"]
    pub(crate) fn saadc_inten300_done_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write DONE mwrite None write None rac None reset value false")
    }
    #[doc = "RESULTDONE: Enable or disable interrupt for RESULTDONE event<br>"]
    pub(crate) fn saadc_inten300_resultdone_read(&self) -> MemResult<bool> {
        todo!(
            "read RESULTDONE mwrite None write None rac None reset value false"
        )
    }
    #[doc = "RESULTDONE: Enable or disable interrupt for RESULTDONE event<br>"]
    pub(crate) fn saadc_inten300_resultdone_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RESULTDONE mwrite None write None rac None reset value false")
    }
    #[doc = "CALIBRATEDONE: Enable or disable interrupt for CALIBRATEDONE event<br>"]
    pub(crate) fn saadc_inten300_calibratedone_read(&self) -> MemResult<bool> {
        todo ! ("read CALIBRATEDONE mwrite None write None rac None reset value false")
    }
    #[doc = "CALIBRATEDONE: Enable or disable interrupt for CALIBRATEDONE event<br>"]
    pub(crate) fn saadc_inten300_calibratedone_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write CALIBRATEDONE mwrite None write None rac None reset value false")
    }
    #[doc = "STOPPED: Enable or disable interrupt for STOPPED event<br>"]
    pub(crate) fn saadc_inten300_stopped_read(&self) -> MemResult<bool> {
        todo!("read STOPPED mwrite None write None rac None reset value false")
    }
    #[doc = "STOPPED: Enable or disable interrupt for STOPPED event<br>"]
    pub(crate) fn saadc_inten300_stopped_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write STOPPED mwrite None write None rac None reset value false")
    }
    #[doc = "CH0LIMITH: Enable or disable interrupt for CH\\[0\\].LIMITH event<br>"]
    pub(crate) fn saadc_inten300_ch0limith_read(&self) -> MemResult<bool> {
        todo!(
            "read CH0LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH0LIMITH: Enable or disable interrupt for CH\\[0\\].LIMITH event<br>"]
    pub(crate) fn saadc_inten300_ch0limith_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH0LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH0LIMITL: Enable or disable interrupt for CH\\[0\\].LIMITL event<br>"]
    pub(crate) fn saadc_inten300_ch0limitl_read(&self) -> MemResult<bool> {
        todo!(
            "read CH0LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH0LIMITL: Enable or disable interrupt for CH\\[0\\].LIMITL event<br>"]
    pub(crate) fn saadc_inten300_ch0limitl_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH0LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH1LIMITH: Enable or disable interrupt for CH\\[1\\].LIMITH event<br>"]
    pub(crate) fn saadc_inten300_ch1limith_read(&self) -> MemResult<bool> {
        todo!(
            "read CH1LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH1LIMITH: Enable or disable interrupt for CH\\[1\\].LIMITH event<br>"]
    pub(crate) fn saadc_inten300_ch1limith_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH1LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH1LIMITL: Enable or disable interrupt for CH\\[1\\].LIMITL event<br>"]
    pub(crate) fn saadc_inten300_ch1limitl_read(&self) -> MemResult<bool> {
        todo!(
            "read CH1LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH1LIMITL: Enable or disable interrupt for CH\\[1\\].LIMITL event<br>"]
    pub(crate) fn saadc_inten300_ch1limitl_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH1LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH2LIMITH: Enable or disable interrupt for CH\\[2\\].LIMITH event<br>"]
    pub(crate) fn saadc_inten300_ch2limith_read(&self) -> MemResult<bool> {
        todo!(
            "read CH2LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH2LIMITH: Enable or disable interrupt for CH\\[2\\].LIMITH event<br>"]
    pub(crate) fn saadc_inten300_ch2limith_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH2LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH2LIMITL: Enable or disable interrupt for CH\\[2\\].LIMITL event<br>"]
    pub(crate) fn saadc_inten300_ch2limitl_read(&self) -> MemResult<bool> {
        todo!(
            "read CH2LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH2LIMITL: Enable or disable interrupt for CH\\[2\\].LIMITL event<br>"]
    pub(crate) fn saadc_inten300_ch2limitl_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH2LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH3LIMITH: Enable or disable interrupt for CH\\[3\\].LIMITH event<br>"]
    pub(crate) fn saadc_inten300_ch3limith_read(&self) -> MemResult<bool> {
        todo!(
            "read CH3LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH3LIMITH: Enable or disable interrupt for CH\\[3\\].LIMITH event<br>"]
    pub(crate) fn saadc_inten300_ch3limith_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH3LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH3LIMITL: Enable or disable interrupt for CH\\[3\\].LIMITL event<br>"]
    pub(crate) fn saadc_inten300_ch3limitl_read(&self) -> MemResult<bool> {
        todo!(
            "read CH3LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH3LIMITL: Enable or disable interrupt for CH\\[3\\].LIMITL event<br>"]
    pub(crate) fn saadc_inten300_ch3limitl_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH3LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH4LIMITH: Enable or disable interrupt for CH\\[4\\].LIMITH event<br>"]
    pub(crate) fn saadc_inten300_ch4limith_read(&self) -> MemResult<bool> {
        todo!(
            "read CH4LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH4LIMITH: Enable or disable interrupt for CH\\[4\\].LIMITH event<br>"]
    pub(crate) fn saadc_inten300_ch4limith_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH4LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH4LIMITL: Enable or disable interrupt for CH\\[4\\].LIMITL event<br>"]
    pub(crate) fn saadc_inten300_ch4limitl_read(&self) -> MemResult<bool> {
        todo!(
            "read CH4LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH4LIMITL: Enable or disable interrupt for CH\\[4\\].LIMITL event<br>"]
    pub(crate) fn saadc_inten300_ch4limitl_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH4LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH5LIMITH: Enable or disable interrupt for CH\\[5\\].LIMITH event<br>"]
    pub(crate) fn saadc_inten300_ch5limith_read(&self) -> MemResult<bool> {
        todo!(
            "read CH5LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH5LIMITH: Enable or disable interrupt for CH\\[5\\].LIMITH event<br>"]
    pub(crate) fn saadc_inten300_ch5limith_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH5LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH5LIMITL: Enable or disable interrupt for CH\\[5\\].LIMITL event<br>"]
    pub(crate) fn saadc_inten300_ch5limitl_read(&self) -> MemResult<bool> {
        todo!(
            "read CH5LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH5LIMITL: Enable or disable interrupt for CH\\[5\\].LIMITL event<br>"]
    pub(crate) fn saadc_inten300_ch5limitl_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH5LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH6LIMITH: Enable or disable interrupt for CH\\[6\\].LIMITH event<br>"]
    pub(crate) fn saadc_inten300_ch6limith_read(&self) -> MemResult<bool> {
        todo!(
            "read CH6LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH6LIMITH: Enable or disable interrupt for CH\\[6\\].LIMITH event<br>"]
    pub(crate) fn saadc_inten300_ch6limith_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH6LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH6LIMITL: Enable or disable interrupt for CH\\[6\\].LIMITL event<br>"]
    pub(crate) fn saadc_inten300_ch6limitl_read(&self) -> MemResult<bool> {
        todo!(
            "read CH6LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH6LIMITL: Enable or disable interrupt for CH\\[6\\].LIMITL event<br>"]
    pub(crate) fn saadc_inten300_ch6limitl_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH6LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH7LIMITH: Enable or disable interrupt for CH\\[7\\].LIMITH event<br>"]
    pub(crate) fn saadc_inten300_ch7limith_read(&self) -> MemResult<bool> {
        todo!(
            "read CH7LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH7LIMITH: Enable or disable interrupt for CH\\[7\\].LIMITH event<br>"]
    pub(crate) fn saadc_inten300_ch7limith_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH7LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH7LIMITL: Enable or disable interrupt for CH\\[7\\].LIMITL event<br>"]
    pub(crate) fn saadc_inten300_ch7limitl_read(&self) -> MemResult<bool> {
        todo!(
            "read CH7LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH7LIMITL: Enable or disable interrupt for CH\\[7\\].LIMITL event<br>"]
    pub(crate) fn saadc_inten300_ch7limitl_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH7LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "STARTED: Write '1' to Enable interrupt for STARTED event<br>"]
    pub(crate) fn saadc_intenset304_started_read(&self) -> MemResult<bool> {
        todo!("read STARTED mwrite None write None rac None reset value false")
    }
    #[doc = "STARTED: Write '1' to Enable interrupt for STARTED event<br>"]
    pub(crate) fn saadc_intenset304_started_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write STARTED mwrite None write None rac None reset value false")
    }
    #[doc = "END: Write '1' to Enable interrupt for END event<br>"]
    pub(crate) fn saadc_intenset304_end_read(&self) -> MemResult<bool> {
        todo!("read END mwrite None write None rac None reset value false")
    }
    #[doc = "END: Write '1' to Enable interrupt for END event<br>"]
    pub(crate) fn saadc_intenset304_end_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write END mwrite None write None rac None reset value false")
    }
    #[doc = "DONE: Write '1' to Enable interrupt for DONE event<br>"]
    pub(crate) fn saadc_intenset304_done_read(&self) -> MemResult<bool> {
        todo!("read DONE mwrite None write None rac None reset value false")
    }
    #[doc = "DONE: Write '1' to Enable interrupt for DONE event<br>"]
    pub(crate) fn saadc_intenset304_done_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write DONE mwrite None write None rac None reset value false")
    }
    #[doc = "RESULTDONE: Write '1' to Enable interrupt for RESULTDONE event<br>"]
    pub(crate) fn saadc_intenset304_resultdone_read(&self) -> MemResult<bool> {
        todo!(
            "read RESULTDONE mwrite None write None rac None reset value false"
        )
    }
    #[doc = "RESULTDONE: Write '1' to Enable interrupt for RESULTDONE event<br>"]
    pub(crate) fn saadc_intenset304_resultdone_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RESULTDONE mwrite None write None rac None reset value false")
    }
    #[doc = "CALIBRATEDONE: Write '1' to Enable interrupt for CALIBRATEDONE event<br>"]
    pub(crate) fn saadc_intenset304_calibratedone_read(
        &self,
    ) -> MemResult<bool> {
        todo ! ("read CALIBRATEDONE mwrite None write None rac None reset value false")
    }
    #[doc = "CALIBRATEDONE: Write '1' to Enable interrupt for CALIBRATEDONE event<br>"]
    pub(crate) fn saadc_intenset304_calibratedone_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write CALIBRATEDONE mwrite None write None rac None reset value false")
    }
    #[doc = "STOPPED: Write '1' to Enable interrupt for STOPPED event<br>"]
    pub(crate) fn saadc_intenset304_stopped_read(&self) -> MemResult<bool> {
        todo!("read STOPPED mwrite None write None rac None reset value false")
    }
    #[doc = "STOPPED: Write '1' to Enable interrupt for STOPPED event<br>"]
    pub(crate) fn saadc_intenset304_stopped_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write STOPPED mwrite None write None rac None reset value false")
    }
    #[doc = "CH0LIMITH: Write '1' to Enable interrupt for CH\\[0\\].LIMITH event<br>"]
    pub(crate) fn saadc_intenset304_ch0limith_read(&self) -> MemResult<bool> {
        todo!(
            "read CH0LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH0LIMITH: Write '1' to Enable interrupt for CH\\[0\\].LIMITH event<br>"]
    pub(crate) fn saadc_intenset304_ch0limith_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH0LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH0LIMITL: Write '1' to Enable interrupt for CH\\[0\\].LIMITL event<br>"]
    pub(crate) fn saadc_intenset304_ch0limitl_read(&self) -> MemResult<bool> {
        todo!(
            "read CH0LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH0LIMITL: Write '1' to Enable interrupt for CH\\[0\\].LIMITL event<br>"]
    pub(crate) fn saadc_intenset304_ch0limitl_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH0LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH1LIMITH: Write '1' to Enable interrupt for CH\\[1\\].LIMITH event<br>"]
    pub(crate) fn saadc_intenset304_ch1limith_read(&self) -> MemResult<bool> {
        todo!(
            "read CH1LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH1LIMITH: Write '1' to Enable interrupt for CH\\[1\\].LIMITH event<br>"]
    pub(crate) fn saadc_intenset304_ch1limith_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH1LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH1LIMITL: Write '1' to Enable interrupt for CH\\[1\\].LIMITL event<br>"]
    pub(crate) fn saadc_intenset304_ch1limitl_read(&self) -> MemResult<bool> {
        todo!(
            "read CH1LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH1LIMITL: Write '1' to Enable interrupt for CH\\[1\\].LIMITL event<br>"]
    pub(crate) fn saadc_intenset304_ch1limitl_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH1LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH2LIMITH: Write '1' to Enable interrupt for CH\\[2\\].LIMITH event<br>"]
    pub(crate) fn saadc_intenset304_ch2limith_read(&self) -> MemResult<bool> {
        todo!(
            "read CH2LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH2LIMITH: Write '1' to Enable interrupt for CH\\[2\\].LIMITH event<br>"]
    pub(crate) fn saadc_intenset304_ch2limith_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH2LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH2LIMITL: Write '1' to Enable interrupt for CH\\[2\\].LIMITL event<br>"]
    pub(crate) fn saadc_intenset304_ch2limitl_read(&self) -> MemResult<bool> {
        todo!(
            "read CH2LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH2LIMITL: Write '1' to Enable interrupt for CH\\[2\\].LIMITL event<br>"]
    pub(crate) fn saadc_intenset304_ch2limitl_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH2LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH3LIMITH: Write '1' to Enable interrupt for CH\\[3\\].LIMITH event<br>"]
    pub(crate) fn saadc_intenset304_ch3limith_read(&self) -> MemResult<bool> {
        todo!(
            "read CH3LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH3LIMITH: Write '1' to Enable interrupt for CH\\[3\\].LIMITH event<br>"]
    pub(crate) fn saadc_intenset304_ch3limith_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH3LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH3LIMITL: Write '1' to Enable interrupt for CH\\[3\\].LIMITL event<br>"]
    pub(crate) fn saadc_intenset304_ch3limitl_read(&self) -> MemResult<bool> {
        todo!(
            "read CH3LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH3LIMITL: Write '1' to Enable interrupt for CH\\[3\\].LIMITL event<br>"]
    pub(crate) fn saadc_intenset304_ch3limitl_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH3LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH4LIMITH: Write '1' to Enable interrupt for CH\\[4\\].LIMITH event<br>"]
    pub(crate) fn saadc_intenset304_ch4limith_read(&self) -> MemResult<bool> {
        todo!(
            "read CH4LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH4LIMITH: Write '1' to Enable interrupt for CH\\[4\\].LIMITH event<br>"]
    pub(crate) fn saadc_intenset304_ch4limith_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH4LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH4LIMITL: Write '1' to Enable interrupt for CH\\[4\\].LIMITL event<br>"]
    pub(crate) fn saadc_intenset304_ch4limitl_read(&self) -> MemResult<bool> {
        todo!(
            "read CH4LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH4LIMITL: Write '1' to Enable interrupt for CH\\[4\\].LIMITL event<br>"]
    pub(crate) fn saadc_intenset304_ch4limitl_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH4LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH5LIMITH: Write '1' to Enable interrupt for CH\\[5\\].LIMITH event<br>"]
    pub(crate) fn saadc_intenset304_ch5limith_read(&self) -> MemResult<bool> {
        todo!(
            "read CH5LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH5LIMITH: Write '1' to Enable interrupt for CH\\[5\\].LIMITH event<br>"]
    pub(crate) fn saadc_intenset304_ch5limith_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH5LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH5LIMITL: Write '1' to Enable interrupt for CH\\[5\\].LIMITL event<br>"]
    pub(crate) fn saadc_intenset304_ch5limitl_read(&self) -> MemResult<bool> {
        todo!(
            "read CH5LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH5LIMITL: Write '1' to Enable interrupt for CH\\[5\\].LIMITL event<br>"]
    pub(crate) fn saadc_intenset304_ch5limitl_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH5LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH6LIMITH: Write '1' to Enable interrupt for CH\\[6\\].LIMITH event<br>"]
    pub(crate) fn saadc_intenset304_ch6limith_read(&self) -> MemResult<bool> {
        todo!(
            "read CH6LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH6LIMITH: Write '1' to Enable interrupt for CH\\[6\\].LIMITH event<br>"]
    pub(crate) fn saadc_intenset304_ch6limith_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH6LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH6LIMITL: Write '1' to Enable interrupt for CH\\[6\\].LIMITL event<br>"]
    pub(crate) fn saadc_intenset304_ch6limitl_read(&self) -> MemResult<bool> {
        todo!(
            "read CH6LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH6LIMITL: Write '1' to Enable interrupt for CH\\[6\\].LIMITL event<br>"]
    pub(crate) fn saadc_intenset304_ch6limitl_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH6LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH7LIMITH: Write '1' to Enable interrupt for CH\\[7\\].LIMITH event<br>"]
    pub(crate) fn saadc_intenset304_ch7limith_read(&self) -> MemResult<bool> {
        todo!(
            "read CH7LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH7LIMITH: Write '1' to Enable interrupt for CH\\[7\\].LIMITH event<br>"]
    pub(crate) fn saadc_intenset304_ch7limith_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH7LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH7LIMITL: Write '1' to Enable interrupt for CH\\[7\\].LIMITL event<br>"]
    pub(crate) fn saadc_intenset304_ch7limitl_read(&self) -> MemResult<bool> {
        todo!(
            "read CH7LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH7LIMITL: Write '1' to Enable interrupt for CH\\[7\\].LIMITL event<br>"]
    pub(crate) fn saadc_intenset304_ch7limitl_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH7LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "STARTED: Write '1' to Disable interrupt for STARTED event<br>"]
    pub(crate) fn saadc_intenclr308_started_read(&self) -> MemResult<bool> {
        todo!("read STARTED mwrite None write None rac None reset value false")
    }
    #[doc = "STARTED: Write '1' to Disable interrupt for STARTED event<br>"]
    pub(crate) fn saadc_intenclr308_started_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write STARTED mwrite None write None rac None reset value false")
    }
    #[doc = "END: Write '1' to Disable interrupt for END event<br>"]
    pub(crate) fn saadc_intenclr308_end_read(&self) -> MemResult<bool> {
        todo!("read END mwrite None write None rac None reset value false")
    }
    #[doc = "END: Write '1' to Disable interrupt for END event<br>"]
    pub(crate) fn saadc_intenclr308_end_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write END mwrite None write None rac None reset value false")
    }
    #[doc = "DONE: Write '1' to Disable interrupt for DONE event<br>"]
    pub(crate) fn saadc_intenclr308_done_read(&self) -> MemResult<bool> {
        todo!("read DONE mwrite None write None rac None reset value false")
    }
    #[doc = "DONE: Write '1' to Disable interrupt for DONE event<br>"]
    pub(crate) fn saadc_intenclr308_done_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write DONE mwrite None write None rac None reset value false")
    }
    #[doc = "RESULTDONE: Write '1' to Disable interrupt for RESULTDONE event<br>"]
    pub(crate) fn saadc_intenclr308_resultdone_read(&self) -> MemResult<bool> {
        todo!(
            "read RESULTDONE mwrite None write None rac None reset value false"
        )
    }
    #[doc = "RESULTDONE: Write '1' to Disable interrupt for RESULTDONE event<br>"]
    pub(crate) fn saadc_intenclr308_resultdone_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RESULTDONE mwrite None write None rac None reset value false")
    }
    #[doc = "CALIBRATEDONE: Write '1' to Disable interrupt for CALIBRATEDONE event<br>"]
    pub(crate) fn saadc_intenclr308_calibratedone_read(
        &self,
    ) -> MemResult<bool> {
        todo ! ("read CALIBRATEDONE mwrite None write None rac None reset value false")
    }
    #[doc = "CALIBRATEDONE: Write '1' to Disable interrupt for CALIBRATEDONE event<br>"]
    pub(crate) fn saadc_intenclr308_calibratedone_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write CALIBRATEDONE mwrite None write None rac None reset value false")
    }
    #[doc = "STOPPED: Write '1' to Disable interrupt for STOPPED event<br>"]
    pub(crate) fn saadc_intenclr308_stopped_read(&self) -> MemResult<bool> {
        todo!("read STOPPED mwrite None write None rac None reset value false")
    }
    #[doc = "STOPPED: Write '1' to Disable interrupt for STOPPED event<br>"]
    pub(crate) fn saadc_intenclr308_stopped_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write STOPPED mwrite None write None rac None reset value false")
    }
    #[doc = "CH0LIMITH: Write '1' to Disable interrupt for CH\\[0\\].LIMITH event<br>"]
    pub(crate) fn saadc_intenclr308_ch0limith_read(&self) -> MemResult<bool> {
        todo!(
            "read CH0LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH0LIMITH: Write '1' to Disable interrupt for CH\\[0\\].LIMITH event<br>"]
    pub(crate) fn saadc_intenclr308_ch0limith_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH0LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH0LIMITL: Write '1' to Disable interrupt for CH\\[0\\].LIMITL event<br>"]
    pub(crate) fn saadc_intenclr308_ch0limitl_read(&self) -> MemResult<bool> {
        todo!(
            "read CH0LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH0LIMITL: Write '1' to Disable interrupt for CH\\[0\\].LIMITL event<br>"]
    pub(crate) fn saadc_intenclr308_ch0limitl_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH0LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH1LIMITH: Write '1' to Disable interrupt for CH\\[1\\].LIMITH event<br>"]
    pub(crate) fn saadc_intenclr308_ch1limith_read(&self) -> MemResult<bool> {
        todo!(
            "read CH1LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH1LIMITH: Write '1' to Disable interrupt for CH\\[1\\].LIMITH event<br>"]
    pub(crate) fn saadc_intenclr308_ch1limith_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH1LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH1LIMITL: Write '1' to Disable interrupt for CH\\[1\\].LIMITL event<br>"]
    pub(crate) fn saadc_intenclr308_ch1limitl_read(&self) -> MemResult<bool> {
        todo!(
            "read CH1LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH1LIMITL: Write '1' to Disable interrupt for CH\\[1\\].LIMITL event<br>"]
    pub(crate) fn saadc_intenclr308_ch1limitl_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH1LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH2LIMITH: Write '1' to Disable interrupt for CH\\[2\\].LIMITH event<br>"]
    pub(crate) fn saadc_intenclr308_ch2limith_read(&self) -> MemResult<bool> {
        todo!(
            "read CH2LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH2LIMITH: Write '1' to Disable interrupt for CH\\[2\\].LIMITH event<br>"]
    pub(crate) fn saadc_intenclr308_ch2limith_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH2LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH2LIMITL: Write '1' to Disable interrupt for CH\\[2\\].LIMITL event<br>"]
    pub(crate) fn saadc_intenclr308_ch2limitl_read(&self) -> MemResult<bool> {
        todo!(
            "read CH2LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH2LIMITL: Write '1' to Disable interrupt for CH\\[2\\].LIMITL event<br>"]
    pub(crate) fn saadc_intenclr308_ch2limitl_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH2LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH3LIMITH: Write '1' to Disable interrupt for CH\\[3\\].LIMITH event<br>"]
    pub(crate) fn saadc_intenclr308_ch3limith_read(&self) -> MemResult<bool> {
        todo!(
            "read CH3LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH3LIMITH: Write '1' to Disable interrupt for CH\\[3\\].LIMITH event<br>"]
    pub(crate) fn saadc_intenclr308_ch3limith_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH3LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH3LIMITL: Write '1' to Disable interrupt for CH\\[3\\].LIMITL event<br>"]
    pub(crate) fn saadc_intenclr308_ch3limitl_read(&self) -> MemResult<bool> {
        todo!(
            "read CH3LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH3LIMITL: Write '1' to Disable interrupt for CH\\[3\\].LIMITL event<br>"]
    pub(crate) fn saadc_intenclr308_ch3limitl_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH3LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH4LIMITH: Write '1' to Disable interrupt for CH\\[4\\].LIMITH event<br>"]
    pub(crate) fn saadc_intenclr308_ch4limith_read(&self) -> MemResult<bool> {
        todo!(
            "read CH4LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH4LIMITH: Write '1' to Disable interrupt for CH\\[4\\].LIMITH event<br>"]
    pub(crate) fn saadc_intenclr308_ch4limith_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH4LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH4LIMITL: Write '1' to Disable interrupt for CH\\[4\\].LIMITL event<br>"]
    pub(crate) fn saadc_intenclr308_ch4limitl_read(&self) -> MemResult<bool> {
        todo!(
            "read CH4LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH4LIMITL: Write '1' to Disable interrupt for CH\\[4\\].LIMITL event<br>"]
    pub(crate) fn saadc_intenclr308_ch4limitl_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH4LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH5LIMITH: Write '1' to Disable interrupt for CH\\[5\\].LIMITH event<br>"]
    pub(crate) fn saadc_intenclr308_ch5limith_read(&self) -> MemResult<bool> {
        todo!(
            "read CH5LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH5LIMITH: Write '1' to Disable interrupt for CH\\[5\\].LIMITH event<br>"]
    pub(crate) fn saadc_intenclr308_ch5limith_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH5LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH5LIMITL: Write '1' to Disable interrupt for CH\\[5\\].LIMITL event<br>"]
    pub(crate) fn saadc_intenclr308_ch5limitl_read(&self) -> MemResult<bool> {
        todo!(
            "read CH5LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH5LIMITL: Write '1' to Disable interrupt for CH\\[5\\].LIMITL event<br>"]
    pub(crate) fn saadc_intenclr308_ch5limitl_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH5LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH6LIMITH: Write '1' to Disable interrupt for CH\\[6\\].LIMITH event<br>"]
    pub(crate) fn saadc_intenclr308_ch6limith_read(&self) -> MemResult<bool> {
        todo!(
            "read CH6LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH6LIMITH: Write '1' to Disable interrupt for CH\\[6\\].LIMITH event<br>"]
    pub(crate) fn saadc_intenclr308_ch6limith_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH6LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH6LIMITL: Write '1' to Disable interrupt for CH\\[6\\].LIMITL event<br>"]
    pub(crate) fn saadc_intenclr308_ch6limitl_read(&self) -> MemResult<bool> {
        todo!(
            "read CH6LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH6LIMITL: Write '1' to Disable interrupt for CH\\[6\\].LIMITL event<br>"]
    pub(crate) fn saadc_intenclr308_ch6limitl_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH6LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH7LIMITH: Write '1' to Disable interrupt for CH\\[7\\].LIMITH event<br>"]
    pub(crate) fn saadc_intenclr308_ch7limith_read(&self) -> MemResult<bool> {
        todo!(
            "read CH7LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH7LIMITH: Write '1' to Disable interrupt for CH\\[7\\].LIMITH event<br>"]
    pub(crate) fn saadc_intenclr308_ch7limith_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH7LIMITH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH7LIMITL: Write '1' to Disable interrupt for CH\\[7\\].LIMITL event<br>"]
    pub(crate) fn saadc_intenclr308_ch7limitl_read(&self) -> MemResult<bool> {
        todo!(
            "read CH7LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CH7LIMITL: Write '1' to Disable interrupt for CH\\[7\\].LIMITL event<br>"]
    pub(crate) fn saadc_intenclr308_ch7limitl_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CH7LIMITL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "STATUS: Status<br>"]
    pub(crate) fn saadc_status400_status_read(&self) -> MemResult<bool> {
        todo!("read STATUS mwrite None write None rac None reset value false")
    }
    #[doc = "ENABLE: Enable or disable ADC<br>"]
    pub(crate) fn saadc_enable500_enable_read(&self) -> MemResult<bool> {
        todo!("read ENABLE mwrite None write None rac None reset value false")
    }
    #[doc = "ENABLE: Enable or disable ADC<br>"]
    pub(crate) fn saadc_enable500_enable_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ENABLE mwrite None write None rac None reset value false")
    }
    #[doc = "PSELP: Analog positive input channel<br>"]
    pub(crate) fn saadc_chn_pselp0_pselp_read(
        &self,
        _chn: usize,
    ) -> MemResult<crate::peripheral::enums::E30SaadcChnPselp0Pselp> {
        todo ! ("read PSELP mwrite None write None rac None reset value 0x00 mask 0x1f")
    }
    #[doc = "PSELP: Analog positive input channel<br>"]
    pub(crate) fn saadc_chn_pselp0_pselp_write(
        &mut self,
        _chn: usize,
        _value: crate::peripheral::enums::E30SaadcChnPselp0Pselp,
    ) -> MemResult<()> {
        todo ! ("write PSELP mwrite None write None rac None reset value 0x00 mask 0x1f")
    }
    #[doc = "PSELN: Analog negative input, enables differential channel<br>"]
    pub(crate) fn saadc_chn_pseln4_pseln_read(
        &self,
        _chn: usize,
    ) -> MemResult<crate::peripheral::enums::E31SaadcChnPseln4Pseln> {
        todo ! ("read PSELN mwrite None write None rac None reset value 0x00 mask 0x1f")
    }
    #[doc = "PSELN: Analog negative input, enables differential channel<br>"]
    pub(crate) fn saadc_chn_pseln4_pseln_write(
        &mut self,
        _chn: usize,
        _value: crate::peripheral::enums::E31SaadcChnPseln4Pseln,
    ) -> MemResult<()> {
        todo ! ("write PSELN mwrite None write None rac None reset value 0x00 mask 0x1f")
    }
    #[doc = "RESP: Positive channel resistor control<br>"]
    pub(crate) fn saadc_chn_config8_resp_read(
        &self,
        _chn: usize,
    ) -> MemResult<crate::peripheral::enums::E32SaadcChnConfig8Resp> {
        todo ! ("read RESP mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "RESP: Positive channel resistor control<br>"]
    pub(crate) fn saadc_chn_config8_resp_write(
        &mut self,
        _chn: usize,
        _value: crate::peripheral::enums::E32SaadcChnConfig8Resp,
    ) -> MemResult<()> {
        todo ! ("write RESP mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "RESN: Negative channel resistor control<br>"]
    pub(crate) fn saadc_chn_config8_resn_read(
        &self,
        _chn: usize,
    ) -> MemResult<crate::peripheral::enums::E33SaadcChnConfig8Resn> {
        todo ! ("read RESN mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "RESN: Negative channel resistor control<br>"]
    pub(crate) fn saadc_chn_config8_resn_write(
        &mut self,
        _chn: usize,
        _value: crate::peripheral::enums::E33SaadcChnConfig8Resn,
    ) -> MemResult<()> {
        todo ! ("write RESN mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "GAIN: Gain control<br>"]
    pub(crate) fn saadc_chn_config8_gain_read(
        &self,
        _chn: usize,
    ) -> MemResult<crate::peripheral::enums::E34SaadcChnConfig8Gain> {
        todo ! ("read GAIN mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "GAIN: Gain control<br>"]
    pub(crate) fn saadc_chn_config8_gain_write(
        &mut self,
        _chn: usize,
        _value: crate::peripheral::enums::E34SaadcChnConfig8Gain,
    ) -> MemResult<()> {
        todo ! ("write GAIN mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "REFSEL: Reference control<br>"]
    pub(crate) fn saadc_chn_config8_refsel_read(
        &self,
        _chn: usize,
    ) -> MemResult<bool> {
        todo!("read REFSEL mwrite None write None rac None reset value false")
    }
    #[doc = "REFSEL: Reference control<br>"]
    pub(crate) fn saadc_chn_config8_refsel_write(
        &mut self,
        _chn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write REFSEL mwrite None write None rac None reset value false")
    }
    #[doc = "TACQ: Acquisition time, the time the ADC uses to sample the input voltage<br>"]
    pub(crate) fn saadc_chn_config8_tacq_read(
        &self,
        _chn: usize,
    ) -> MemResult<crate::peripheral::enums::E35SaadcChnConfig8Tacq> {
        todo ! ("read TACQ mwrite None write None rac None reset value 0x02 mask 0x07")
    }
    #[doc = "TACQ: Acquisition time, the time the ADC uses to sample the input voltage<br>"]
    pub(crate) fn saadc_chn_config8_tacq_write(
        &mut self,
        _chn: usize,
        _value: crate::peripheral::enums::E35SaadcChnConfig8Tacq,
    ) -> MemResult<()> {
        todo ! ("write TACQ mwrite None write None rac None reset value 0x02 mask 0x07")
    }
    #[doc = "MODE: Enable differential mode<br>"]
    pub(crate) fn saadc_chn_config8_mode_read(
        &self,
        _chn: usize,
    ) -> MemResult<bool> {
        todo!("read MODE mwrite None write None rac None reset value false")
    }
    #[doc = "MODE: Enable differential mode<br>"]
    pub(crate) fn saadc_chn_config8_mode_write(
        &mut self,
        _chn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MODE mwrite None write None rac None reset value false")
    }
    #[doc = "BURST: Enable burst mode<br>"]
    pub(crate) fn saadc_chn_config8_burst_read(
        &self,
        _chn: usize,
    ) -> MemResult<bool> {
        todo!("read BURST mwrite None write None rac None reset value false")
    }
    #[doc = "BURST: Enable burst mode<br>"]
    pub(crate) fn saadc_chn_config8_burst_write(
        &mut self,
        _chn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write BURST mwrite None write None rac None reset value false")
    }
    #[doc = "LOW: Low level limit<br>"]
    pub(crate) fn saadc_chn_limitc_low_read(
        &self,
        _chn: usize,
    ) -> MemResult<u16> {
        todo ! ("read LOW mwrite None write None rac None reset value 0x8000 mask 0xffff")
    }
    #[doc = "LOW: Low level limit<br>"]
    pub(crate) fn saadc_chn_limitc_low_write(
        &mut self,
        _chn: usize,
        _value: u16,
    ) -> MemResult<()> {
        todo ! ("write LOW mwrite None write None rac None reset value 0x8000 mask 0xffff")
    }
    #[doc = "HIGH: High level limit<br>"]
    pub(crate) fn saadc_chn_limitc_high_read(
        &self,
        _chn: usize,
    ) -> MemResult<u16> {
        todo ! ("read HIGH mwrite None write None rac None reset value 0x7fff mask 0xffff")
    }
    #[doc = "HIGH: High level limit<br>"]
    pub(crate) fn saadc_chn_limitc_high_write(
        &mut self,
        _chn: usize,
        _value: u16,
    ) -> MemResult<()> {
        todo ! ("write HIGH mwrite None write None rac None reset value 0x7fff mask 0xffff")
    }
    #[doc = "VAL: Set the resolution<br>"]
    pub(crate) fn saadc_resolution5f0_val_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E36SaadcResolution5f0Val> {
        todo ! ("read VAL mwrite None write None rac None reset value 0x01 mask 0x07")
    }
    #[doc = "VAL: Set the resolution<br>"]
    pub(crate) fn saadc_resolution5f0_val_write(
        &mut self,
        _value: crate::peripheral::enums::E36SaadcResolution5f0Val,
    ) -> MemResult<()> {
        todo ! ("write VAL mwrite None write None rac None reset value 0x01 mask 0x07")
    }
    #[doc = "OVERSAMPLE: Oversample control<br>"]
    pub(crate) fn saadc_oversample5f4_oversample_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E37SaadcOversample5f4Oversample>
    {
        todo ! ("read OVERSAMPLE mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "OVERSAMPLE: Oversample control<br>"]
    pub(crate) fn saadc_oversample5f4_oversample_write(
        &mut self,
        _value: crate::peripheral::enums::E37SaadcOversample5f4Oversample,
    ) -> MemResult<()> {
        todo ! ("write OVERSAMPLE mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "CC: Capture and compare value. Sample rate is 16 MHz/CC<br>"]
    pub(crate) fn saadc_samplerate5f8_cc_read(&self) -> MemResult<u16> {
        todo ! ("read CC mwrite None write None rac None reset value 0x00 mask 0x7ff")
    }
    #[doc = "CC: Capture and compare value. Sample rate is 16 MHz/CC<br>"]
    pub(crate) fn saadc_samplerate5f8_cc_write(
        &mut self,
        _value: u16,
    ) -> MemResult<()> {
        todo ! ("write CC mwrite None write None rac None reset value 0x00 mask 0x7ff")
    }
    #[doc = "MODE: Select mode for sample rate control<br>"]
    pub(crate) fn saadc_samplerate5f8_mode_read(&self) -> MemResult<bool> {
        todo!("read MODE mwrite None write None rac None reset value false")
    }
    #[doc = "MODE: Select mode for sample rate control<br>"]
    pub(crate) fn saadc_samplerate5f8_mode_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MODE mwrite None write None rac None reset value false")
    }
    #[doc = "PTR: Data pointer<br>"]
    pub(crate) fn saadc_result_ptr0_ptr_read(&self) -> MemResult<u32> {
        todo ! ("read PTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "PTR: Data pointer<br>"]
    pub(crate) fn saadc_result_ptr0_ptr_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write PTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "MAXCNT: Maximum number of buffer words to transfer<br>"]
    pub(crate) fn saadc_result_maxcnt4_maxcnt_read(&self) -> MemResult<u16> {
        todo ! ("read MAXCNT mwrite None write None rac None reset value 0x00 mask 0x7fff")
    }
    #[doc = "MAXCNT: Maximum number of buffer words to transfer<br>"]
    pub(crate) fn saadc_result_maxcnt4_maxcnt_write(
        &mut self,
        _value: u16,
    ) -> MemResult<()> {
        todo ! ("write MAXCNT mwrite None write None rac None reset value 0x00 mask 0x7fff")
    }
    #[doc = "AMOUNT: Number of buffer words transferred since last START. This register can be read after an END or STOPPED event.<br>"]
    pub(crate) fn saadc_result_amount8_amount_read(&self) -> MemResult<u16> {
        todo ! ("read AMOUNT mwrite None write None rac None reset value 0x00 mask 0x7fff")
    }
}
