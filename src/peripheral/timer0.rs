use icicle_vm::cpu::mem::MemResult;
#[derive(Default)]
#[doc = "TIMER0: Timer/Counter 0<br><br>Instances:<br>0x40008000: TIMER0<br>0x40009000: TIMER1<br>0x4000a000: TIMER2<br>"]
pub struct Timer0 {
    #[doc = "TODO: implement things here"]
    _todo: (),
}
impl Timer0 {
    pub(crate) fn page_to_index(page: u64) -> usize {
        match page {
            262152u64 => 0usize,
            262153u64 => 1usize,
            262154u64 => 2usize,
            _ => unreachable!(),
        }
    }
    #[doc = "TASKS_START: Start Timer<br>"]
    pub(crate) fn timer0_tasks_start0_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write timer0_tasks_start0 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_STOP: Stop Timer<br>"]
    pub(crate) fn timer0_tasks_stop4_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write timer0_tasks_stop4 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_COUNT: Increment Timer (Counter mode only)<br>"]
    pub(crate) fn timer0_tasks_count8_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write timer0_tasks_count8 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_CLEAR: Clear time<br>"]
    pub(crate) fn timer0_tasks_clearc_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write timer0_tasks_clearc mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_SHUTDOWN: Deprecated register -  Shut down timer<br>"]
    pub(crate) fn timer0_tasks_shutdown10_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write timer0_tasks_shutdown10 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_CAPTURE\\[%s\\]: Description collection\\[0\\]:  Capture Timer value to CC\\[0\\] register<br>"]
    pub(crate) fn timer0_tasks_capturen40_write(
        &mut self,
        _reg_array: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write timer0_tasks_capturen40 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_COMPARE\\[%s\\]: Description collection\\[0\\]:  Compare event on CC\\[0\\] match<br>"]
    pub(crate) fn timer0_events_comparen140_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<u32> {
        todo ! ("read timer0_events_comparen140 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_COMPARE\\[%s\\]: Description collection\\[0\\]:  Compare event on CC\\[0\\] match<br>"]
    pub(crate) fn timer0_events_comparen140_write(
        &mut self,
        _reg_array: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write timer0_events_comparen140 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "COMPARE0_CLEAR: Shortcut between COMPARE\\[0\\] event and CLEAR task<br>"]
    pub(crate) fn timer0_shorts200_compare0_clear_read(
        &self,
    ) -> MemResult<bool> {
        todo ! ("read COMPARE0_CLEAR mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE0_CLEAR: Shortcut between COMPARE\\[0\\] event and CLEAR task<br>"]
    pub(crate) fn timer0_shorts200_compare0_clear_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write COMPARE0_CLEAR mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE1_CLEAR: Shortcut between COMPARE\\[1\\] event and CLEAR task<br>"]
    pub(crate) fn timer0_shorts200_compare1_clear_read(
        &self,
    ) -> MemResult<bool> {
        todo ! ("read COMPARE1_CLEAR mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE1_CLEAR: Shortcut between COMPARE\\[1\\] event and CLEAR task<br>"]
    pub(crate) fn timer0_shorts200_compare1_clear_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write COMPARE1_CLEAR mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE2_CLEAR: Shortcut between COMPARE\\[2\\] event and CLEAR task<br>"]
    pub(crate) fn timer0_shorts200_compare2_clear_read(
        &self,
    ) -> MemResult<bool> {
        todo ! ("read COMPARE2_CLEAR mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE2_CLEAR: Shortcut between COMPARE\\[2\\] event and CLEAR task<br>"]
    pub(crate) fn timer0_shorts200_compare2_clear_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write COMPARE2_CLEAR mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE3_CLEAR: Shortcut between COMPARE\\[3\\] event and CLEAR task<br>"]
    pub(crate) fn timer0_shorts200_compare3_clear_read(
        &self,
    ) -> MemResult<bool> {
        todo ! ("read COMPARE3_CLEAR mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE3_CLEAR: Shortcut between COMPARE\\[3\\] event and CLEAR task<br>"]
    pub(crate) fn timer0_shorts200_compare3_clear_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write COMPARE3_CLEAR mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE0_STOP: Shortcut between COMPARE\\[0\\] event and STOP task<br>"]
    pub(crate) fn timer0_shorts200_compare0_stop_read(
        &self,
    ) -> MemResult<bool> {
        todo ! ("read COMPARE0_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE0_STOP: Shortcut between COMPARE\\[0\\] event and STOP task<br>"]
    pub(crate) fn timer0_shorts200_compare0_stop_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write COMPARE0_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE1_STOP: Shortcut between COMPARE\\[1\\] event and STOP task<br>"]
    pub(crate) fn timer0_shorts200_compare1_stop_read(
        &self,
    ) -> MemResult<bool> {
        todo ! ("read COMPARE1_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE1_STOP: Shortcut between COMPARE\\[1\\] event and STOP task<br>"]
    pub(crate) fn timer0_shorts200_compare1_stop_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write COMPARE1_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE2_STOP: Shortcut between COMPARE\\[2\\] event and STOP task<br>"]
    pub(crate) fn timer0_shorts200_compare2_stop_read(
        &self,
    ) -> MemResult<bool> {
        todo ! ("read COMPARE2_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE2_STOP: Shortcut between COMPARE\\[2\\] event and STOP task<br>"]
    pub(crate) fn timer0_shorts200_compare2_stop_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write COMPARE2_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE3_STOP: Shortcut between COMPARE\\[3\\] event and STOP task<br>"]
    pub(crate) fn timer0_shorts200_compare3_stop_read(
        &self,
    ) -> MemResult<bool> {
        todo ! ("read COMPARE3_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE3_STOP: Shortcut between COMPARE\\[3\\] event and STOP task<br>"]
    pub(crate) fn timer0_shorts200_compare3_stop_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write COMPARE3_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE0: Write '1' to Enable interrupt for COMPARE\\[0\\] event<br>"]
    pub(crate) fn timer0_intenset304_compare0_read(&self) -> MemResult<bool> {
        todo!("read COMPARE0 mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE0: Write '1' to Enable interrupt for COMPARE\\[0\\] event<br>"]
    pub(crate) fn timer0_intenset304_compare0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write COMPARE0 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "COMPARE1: Write '1' to Enable interrupt for COMPARE\\[1\\] event<br>"]
    pub(crate) fn timer0_intenset304_compare1_read(&self) -> MemResult<bool> {
        todo!("read COMPARE1 mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE1: Write '1' to Enable interrupt for COMPARE\\[1\\] event<br>"]
    pub(crate) fn timer0_intenset304_compare1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write COMPARE1 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "COMPARE2: Write '1' to Enable interrupt for COMPARE\\[2\\] event<br>"]
    pub(crate) fn timer0_intenset304_compare2_read(&self) -> MemResult<bool> {
        todo!("read COMPARE2 mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE2: Write '1' to Enable interrupt for COMPARE\\[2\\] event<br>"]
    pub(crate) fn timer0_intenset304_compare2_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write COMPARE2 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "COMPARE3: Write '1' to Enable interrupt for COMPARE\\[3\\] event<br>"]
    pub(crate) fn timer0_intenset304_compare3_read(&self) -> MemResult<bool> {
        todo!("read COMPARE3 mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE3: Write '1' to Enable interrupt for COMPARE\\[3\\] event<br>"]
    pub(crate) fn timer0_intenset304_compare3_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write COMPARE3 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "COMPARE0: Write '1' to Disable interrupt for COMPARE\\[0\\] event<br>"]
    pub(crate) fn timer0_intenclr308_compare0_read(&self) -> MemResult<bool> {
        todo!("read COMPARE0 mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE0: Write '1' to Disable interrupt for COMPARE\\[0\\] event<br>"]
    pub(crate) fn timer0_intenclr308_compare0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write COMPARE0 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "COMPARE1: Write '1' to Disable interrupt for COMPARE\\[1\\] event<br>"]
    pub(crate) fn timer0_intenclr308_compare1_read(&self) -> MemResult<bool> {
        todo!("read COMPARE1 mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE1: Write '1' to Disable interrupt for COMPARE\\[1\\] event<br>"]
    pub(crate) fn timer0_intenclr308_compare1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write COMPARE1 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "COMPARE2: Write '1' to Disable interrupt for COMPARE\\[2\\] event<br>"]
    pub(crate) fn timer0_intenclr308_compare2_read(&self) -> MemResult<bool> {
        todo!("read COMPARE2 mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE2: Write '1' to Disable interrupt for COMPARE\\[2\\] event<br>"]
    pub(crate) fn timer0_intenclr308_compare2_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write COMPARE2 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "COMPARE3: Write '1' to Disable interrupt for COMPARE\\[3\\] event<br>"]
    pub(crate) fn timer0_intenclr308_compare3_read(&self) -> MemResult<bool> {
        todo!("read COMPARE3 mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE3: Write '1' to Disable interrupt for COMPARE\\[3\\] event<br>"]
    pub(crate) fn timer0_intenclr308_compare3_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write COMPARE3 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "MODE: Timer mode<br>"]
    pub(crate) fn timer0_mode504_mode_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E38Timer0Mode504Mode> {
        todo ! ("read MODE mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "MODE: Timer mode<br>"]
    pub(crate) fn timer0_mode504_mode_write(
        &mut self,
        _value: crate::peripheral::enums::E38Timer0Mode504Mode,
    ) -> MemResult<()> {
        todo ! ("write MODE mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "BITMODE: Timer bit width<br>"]
    pub(crate) fn timer0_bitmode508_bitmode_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E39Timer0Bitmode508Bitmode> {
        todo ! ("read BITMODE mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "BITMODE: Timer bit width<br>"]
    pub(crate) fn timer0_bitmode508_bitmode_write(
        &mut self,
        _value: crate::peripheral::enums::E39Timer0Bitmode508Bitmode,
    ) -> MemResult<()> {
        todo ! ("write BITMODE mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "PRESCALER: Prescaler value<br>"]
    pub(crate) fn timer0_prescaler510_prescaler_read(&self) -> MemResult<u8> {
        todo ! ("read PRESCALER mwrite None write None rac None reset value 0x04 mask 0x0f")
    }
    #[doc = "PRESCALER: Prescaler value<br>"]
    pub(crate) fn timer0_prescaler510_prescaler_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write PRESCALER mwrite None write None rac None reset value 0x04 mask 0x0f")
    }
    #[doc = "CC: Capture/Compare value<br>"]
    pub(crate) fn timer0_ccn540_cc_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<u32> {
        todo ! ("read CC mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "CC: Capture/Compare value<br>"]
    pub(crate) fn timer0_ccn540_cc_write(
        &mut self,
        _reg_array: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write CC mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
}
