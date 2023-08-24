use icicle_vm::cpu::mem::MemResult;
#[derive(Default)]
#[doc = "PDM: Pulse Density Modulation (Digital Microphone) Interface<br><br>Instances:<br>0x4001d000: PDM<br>"]
pub struct Pdm {
    #[doc = "TODO: implement things here"]
    _todo: (),
}
impl Pdm {
    pub(crate) fn page_to_index(page: u64) -> usize {
        match page {
            262173u64 => 0usize,
            _ => unreachable!(),
        }
    }
    #[doc = "TASKS_START: Starts continuous PDM transfer<br>"]
    pub(crate) fn pdm_tasks_start0_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write pdm_tasks_start0 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_STOP: Stops PDM transfer<br>"]
    pub(crate) fn pdm_tasks_stop4_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write pdm_tasks_stop4 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_STARTED: PDM transfer has started<br>"]
    pub(crate) fn pdm_events_started100_read(&self) -> MemResult<u32> {
        todo ! ("read pdm_events_started100 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_STARTED: PDM transfer has started<br>"]
    pub(crate) fn pdm_events_started100_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write pdm_events_started100 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_STOPPED: PDM transfer has finished<br>"]
    pub(crate) fn pdm_events_stopped104_read(&self) -> MemResult<u32> {
        todo ! ("read pdm_events_stopped104 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_STOPPED: PDM transfer has finished<br>"]
    pub(crate) fn pdm_events_stopped104_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write pdm_events_stopped104 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_END: The PDM has written the last sample specified by SAMPLE.MAXCNT (or the last sample after a STOP task has been received) to Data RAM<br>"]
    pub(crate) fn pdm_events_end108_read(&self) -> MemResult<u32> {
        todo ! ("read pdm_events_end108 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_END: The PDM has written the last sample specified by SAMPLE.MAXCNT (or the last sample after a STOP task has been received) to Data RAM<br>"]
    pub(crate) fn pdm_events_end108_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write pdm_events_end108 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "STARTED: Enable or disable interrupt for STARTED event<br>"]
    pub(crate) fn pdm_inten300_started_read(&self) -> MemResult<bool> {
        todo!("read STARTED mwrite None write None rac None reset value false")
    }
    #[doc = "STARTED: Enable or disable interrupt for STARTED event<br>"]
    pub(crate) fn pdm_inten300_started_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write STARTED mwrite None write None rac None reset value false")
    }
    #[doc = "STOPPED: Enable or disable interrupt for STOPPED event<br>"]
    pub(crate) fn pdm_inten300_stopped_read(&self) -> MemResult<bool> {
        todo!("read STOPPED mwrite None write None rac None reset value false")
    }
    #[doc = "STOPPED: Enable or disable interrupt for STOPPED event<br>"]
    pub(crate) fn pdm_inten300_stopped_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write STOPPED mwrite None write None rac None reset value false")
    }
    #[doc = "END: Enable or disable interrupt for END event<br>"]
    pub(crate) fn pdm_inten300_end_read(&self) -> MemResult<bool> {
        todo!("read END mwrite None write None rac None reset value false")
    }
    #[doc = "END: Enable or disable interrupt for END event<br>"]
    pub(crate) fn pdm_inten300_end_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write END mwrite None write None rac None reset value false")
    }
    #[doc = "STARTED: Write '1' to Enable interrupt for STARTED event<br>"]
    pub(crate) fn pdm_intenset304_started_read(&self) -> MemResult<bool> {
        todo!("read STARTED mwrite None write None rac None reset value false")
    }
    #[doc = "STARTED: Write '1' to Enable interrupt for STARTED event<br>"]
    pub(crate) fn pdm_intenset304_started_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write STARTED mwrite None write None rac None reset value false")
    }
    #[doc = "STOPPED: Write '1' to Enable interrupt for STOPPED event<br>"]
    pub(crate) fn pdm_intenset304_stopped_read(&self) -> MemResult<bool> {
        todo!("read STOPPED mwrite None write None rac None reset value false")
    }
    #[doc = "STOPPED: Write '1' to Enable interrupt for STOPPED event<br>"]
    pub(crate) fn pdm_intenset304_stopped_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write STOPPED mwrite None write None rac None reset value false")
    }
    #[doc = "END: Write '1' to Enable interrupt for END event<br>"]
    pub(crate) fn pdm_intenset304_end_read(&self) -> MemResult<bool> {
        todo!("read END mwrite None write None rac None reset value false")
    }
    #[doc = "END: Write '1' to Enable interrupt for END event<br>"]
    pub(crate) fn pdm_intenset304_end_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write END mwrite None write None rac None reset value false")
    }
    #[doc = "STARTED: Write '1' to Disable interrupt for STARTED event<br>"]
    pub(crate) fn pdm_intenclr308_started_read(&self) -> MemResult<bool> {
        todo!("read STARTED mwrite None write None rac None reset value false")
    }
    #[doc = "STARTED: Write '1' to Disable interrupt for STARTED event<br>"]
    pub(crate) fn pdm_intenclr308_started_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write STARTED mwrite None write None rac None reset value false")
    }
    #[doc = "STOPPED: Write '1' to Disable interrupt for STOPPED event<br>"]
    pub(crate) fn pdm_intenclr308_stopped_read(&self) -> MemResult<bool> {
        todo!("read STOPPED mwrite None write None rac None reset value false")
    }
    #[doc = "STOPPED: Write '1' to Disable interrupt for STOPPED event<br>"]
    pub(crate) fn pdm_intenclr308_stopped_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write STOPPED mwrite None write None rac None reset value false")
    }
    #[doc = "END: Write '1' to Disable interrupt for END event<br>"]
    pub(crate) fn pdm_intenclr308_end_read(&self) -> MemResult<bool> {
        todo!("read END mwrite None write None rac None reset value false")
    }
    #[doc = "END: Write '1' to Disable interrupt for END event<br>"]
    pub(crate) fn pdm_intenclr308_end_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write END mwrite None write None rac None reset value false")
    }
    #[doc = "ENABLE: Enable or disable PDM module<br>"]
    pub(crate) fn pdm_enable500_enable_read(&self) -> MemResult<bool> {
        todo!("read ENABLE mwrite None write None rac None reset value false")
    }
    #[doc = "ENABLE: Enable or disable PDM module<br>"]
    pub(crate) fn pdm_enable500_enable_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ENABLE mwrite None write None rac None reset value false")
    }
    #[doc = "FREQ: PDM_CLK frequency<br>"]
    pub(crate) fn pdm_pdmclkctrl504_freq_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E57PdmPdmclkctrl504Freq> {
        todo ! ("read FREQ mwrite None write None rac None reset value 0x8400000 mask 0xffffffff")
    }
    #[doc = "FREQ: PDM_CLK frequency<br>"]
    pub(crate) fn pdm_pdmclkctrl504_freq_write(
        &mut self,
        _value: crate::peripheral::enums::E57PdmPdmclkctrl504Freq,
    ) -> MemResult<()> {
        todo ! ("write FREQ mwrite None write None rac None reset value 0x8400000 mask 0xffffffff")
    }
    #[doc = "OPERATION: Mono or stereo operation<br>"]
    pub(crate) fn pdm_mode508_operation_read(&self) -> MemResult<bool> {
        todo!(
            "read OPERATION mwrite None write None rac None reset value false"
        )
    }
    #[doc = "OPERATION: Mono or stereo operation<br>"]
    pub(crate) fn pdm_mode508_operation_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write OPERATION mwrite None write None rac None reset value false"
        )
    }
    #[doc = "EDGE: Defines on which PDM_CLK edge Left (or mono) is sampled<br>"]
    pub(crate) fn pdm_mode508_edge_read(&self) -> MemResult<bool> {
        todo!("read EDGE mwrite None write None rac None reset value false")
    }
    #[doc = "EDGE: Defines on which PDM_CLK edge Left (or mono) is sampled<br>"]
    pub(crate) fn pdm_mode508_edge_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write EDGE mwrite None write None rac None reset value false")
    }
    #[doc = "GAINL: Left output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters) 0x00    -20 dB gain adjust 0x01  -19.5 dB gain adjust (...) 0x27   -0.5 dB gain adjust 0x28      0 dB gain adjust 0x29   +0.5 dB gain adjust (...) 0x4F  +19.5 dB gain adjust 0x50    +20 dB gain adjust<br>"]
    pub(crate) fn pdm_gainl518_gainl_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E58PdmGainl518Gainl> {
        todo ! ("read GAINL mwrite None write None rac None reset value 0x28 mask 0x7f")
    }
    #[doc = "GAINL: Left output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters) 0x00    -20 dB gain adjust 0x01  -19.5 dB gain adjust (...) 0x27   -0.5 dB gain adjust 0x28      0 dB gain adjust 0x29   +0.5 dB gain adjust (...) 0x4F  +19.5 dB gain adjust 0x50    +20 dB gain adjust<br>"]
    pub(crate) fn pdm_gainl518_gainl_write(
        &mut self,
        _value: crate::peripheral::enums::E58PdmGainl518Gainl,
    ) -> MemResult<()> {
        todo ! ("write GAINL mwrite None write None rac None reset value 0x28 mask 0x7f")
    }
    #[doc = "GAINR: Right output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters)<br>"]
    pub(crate) fn pdm_gainr51c_gainr_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E59PdmGainr51cGainr> {
        todo ! ("read GAINR mwrite None write None rac None reset value 0x28 mask 0xff")
    }
    #[doc = "GAINR: Right output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters)<br>"]
    pub(crate) fn pdm_gainr51c_gainr_write(
        &mut self,
        _value: crate::peripheral::enums::E59PdmGainr51cGainr,
    ) -> MemResult<()> {
        todo ! ("write GAINR mwrite None write None rac None reset value 0x28 mask 0xff")
    }
    #[doc = "PIN: Pin number<br>"]
    pub(crate) fn pdm_psel_clk0_pin_read(&self) -> MemResult<u8> {
        todo ! ("read PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "PIN: Pin number<br>"]
    pub(crate) fn pdm_psel_clk0_pin_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn pdm_psel_clk0_connect_read(&self) -> MemResult<bool> {
        todo!("read CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn pdm_psel_clk0_connect_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "PIN: Pin number<br>"]
    pub(crate) fn pdm_psel_din4_pin_read(&self) -> MemResult<u8> {
        todo ! ("read PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "PIN: Pin number<br>"]
    pub(crate) fn pdm_psel_din4_pin_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn pdm_psel_din4_connect_read(&self) -> MemResult<bool> {
        todo!("read CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn pdm_psel_din4_connect_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "SAMPLEPTR: Address to write PDM samples to over DMA<br>"]
    pub(crate) fn pdm_sample_ptr0_sampleptr_read(&self) -> MemResult<u32> {
        todo ! ("read SAMPLEPTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "SAMPLEPTR: Address to write PDM samples to over DMA<br>"]
    pub(crate) fn pdm_sample_ptr0_sampleptr_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write SAMPLEPTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "BUFFSIZE: Length of DMA RAM allocation in number of samples<br>"]
    pub(crate) fn pdm_sample_maxcnt4_buffsize_read(&self) -> MemResult<u16> {
        todo ! ("read BUFFSIZE mwrite None write None rac None reset value 0x00 mask 0x7fff")
    }
    #[doc = "BUFFSIZE: Length of DMA RAM allocation in number of samples<br>"]
    pub(crate) fn pdm_sample_maxcnt4_buffsize_write(
        &mut self,
        _value: u16,
    ) -> MemResult<()> {
        todo ! ("write BUFFSIZE mwrite None write None rac None reset value 0x00 mask 0x7fff")
    }
}
