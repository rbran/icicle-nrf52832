use icicle_vm::cpu::mem::MemResult;

use super::event;
#[derive(Default)]
#[doc = "RTC0: Real time counter 0<br><br>Instances:<br>0x4000b000: RTC0<br>0x40011000: RTC1<br>0x40024000: RTC2<br>"]
pub struct Rtc0 {
    /// 12 bit prescaler for COUNTER frequency (32768/(PRESCALER+1)).
    /// Must be written when RTC is stopped
    prescaler: u16,
    event_tick: event::Event,
    event_overflow: event::Event,
    event_compare: [event::Event; 4],
}
impl Rtc0 {
    pub(crate) fn page_to_index(page: u64) -> usize {
        match page {
            262155u64 => 0usize,
            262161u64 => 1usize,
            262180u64 => 2usize,
            _ => unreachable!(),
        }
    }
    #[doc = "TASKS_START: Start RTC COUNTER<br>"]
    pub(crate) fn rtc0_tasks_start0_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        // TODO start the counter
        //todo ! ("write rtc0_tasks_start0 mwrite None write None rac None reset value 0x00 mask 0x00")
        Ok(())
    }
    #[doc = "TASKS_STOP: Stop RTC COUNTER<br>"]
    pub(crate) fn rtc0_tasks_stop4_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write rtc0_tasks_stop4 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_CLEAR: Clear RTC COUNTER<br>"]
    pub(crate) fn rtc0_tasks_clear8_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        // TODO clear the counter of rtc
        //todo ! ("write rtc0_tasks_clear8 mwrite None write None rac None reset value 0x00 mask 0x00");
        Ok(())
    }
    #[doc = "TASKS_TRIGOVRFLW: Set COUNTER to 0xFFFFF0<br>"]
    pub(crate) fn rtc0_tasks_trigovrflwc_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write rtc0_tasks_trigovrflwc mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_TICK: Event on COUNTER increment<br>"]
    pub(crate) fn rtc0_events_tick100_read(&self) -> MemResult<u32> {
        Ok(self.event_tick.triggered as u32)
    }
    #[doc = "EVENTS_TICK: Event on COUNTER increment<br>"]
    pub(crate) fn rtc0_events_tick100_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.event_tick.clean_on_write(_value))
    }
    #[doc = "EVENTS_OVRFLW: Event on COUNTER overflow<br>"]
    pub(crate) fn rtc0_events_ovrflw104_read(&self) -> MemResult<u32> {
        Ok(self.event_overflow.triggered as u32)
    }
    #[doc = "EVENTS_OVRFLW: Event on COUNTER overflow<br>"]
    pub(crate) fn rtc0_events_ovrflw104_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.event_overflow.clean_on_write(_value))
    }
    #[doc = "EVENTS_COMPARE\\[%s\\]: Description collection\\[0\\]:  Compare event on CC\\[0\\] match<br>"]
    pub(crate) fn rtc0_events_comparen140_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<u32> {
        Ok(self.event_compare[_reg_array].triggered as u32)
    }
    #[doc = "EVENTS_COMPARE\\[%s\\]: Description collection\\[0\\]:  Compare event on CC\\[0\\] match<br>"]
    pub(crate) fn rtc0_events_comparen140_write(
        &mut self,
        _reg_array: usize,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.event_compare[_reg_array].clean_on_write(_value))
    }
    #[doc = "TICK: Write '1' to Enable interrupt for TICK event<br>"]
    pub(crate) fn rtc0_intenset304_tick_read(&self) -> MemResult<bool> {
        Ok(self.event_tick.on)
    }
    #[doc = "TICK: Write '1' to Enable interrupt for TICK event<br>"]
    pub(crate) fn rtc0_intenset304_tick_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.event_tick.on = _value)
    }
    #[doc = "OVRFLW: Write '1' to Enable interrupt for OVRFLW event<br>"]
    pub(crate) fn rtc0_intenset304_ovrflw_read(&self) -> MemResult<bool> {
        Ok(self.event_overflow.on)
    }
    #[doc = "OVRFLW: Write '1' to Enable interrupt for OVRFLW event<br>"]
    pub(crate) fn rtc0_intenset304_ovrflw_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.event_overflow.on = _value)
    }
    #[doc = "COMPARE0: Write '1' to Enable interrupt for COMPARE\\[0\\] event<br>"]
    pub(crate) fn rtc0_intenset304_compare0_read(&self) -> MemResult<bool> {
        Ok(self.event_compare[0].on)
    }
    #[doc = "COMPARE0: Write '1' to Enable interrupt for COMPARE\\[0\\] event<br>"]
    pub(crate) fn rtc0_intenset304_compare0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.event_compare[0].on = _value)
    }
    #[doc = "COMPARE1: Write '1' to Enable interrupt for COMPARE\\[1\\] event<br>"]
    pub(crate) fn rtc0_intenset304_compare1_read(&self) -> MemResult<bool> {
        Ok(self.event_compare[1].on)
    }
    #[doc = "COMPARE1: Write '1' to Enable interrupt for COMPARE\\[1\\] event<br>"]
    pub(crate) fn rtc0_intenset304_compare1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.event_compare[1].on = _value)
    }
    #[doc = "COMPARE2: Write '1' to Enable interrupt for COMPARE\\[2\\] event<br>"]
    pub(crate) fn rtc0_intenset304_compare2_read(&self) -> MemResult<bool> {
        Ok(self.event_compare[2].on)
    }
    #[doc = "COMPARE2: Write '1' to Enable interrupt for COMPARE\\[2\\] event<br>"]
    pub(crate) fn rtc0_intenset304_compare2_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.event_compare[2].on = _value)
    }
    #[doc = "COMPARE3: Write '1' to Enable interrupt for COMPARE\\[3\\] event<br>"]
    pub(crate) fn rtc0_intenset304_compare3_read(&self) -> MemResult<bool> {
        Ok(self.event_compare[3].on)
    }
    #[doc = "COMPARE3: Write '1' to Enable interrupt for COMPARE\\[3\\] event<br>"]
    pub(crate) fn rtc0_intenset304_compare3_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.event_compare[3].on = _value)
    }
    #[doc = "TICK: Write '1' to Disable interrupt for TICK event<br>"]
    pub(crate) fn rtc0_intenclr308_tick_read(&self) -> MemResult<bool> {
        todo!("read TICK mwrite None write None rac None reset value false")
    }
    #[doc = "TICK: Write '1' to Disable interrupt for TICK event<br>"]
    pub(crate) fn rtc0_intenclr308_tick_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write TICK mwrite None write None rac None reset value false")
    }
    #[doc = "OVRFLW: Write '1' to Disable interrupt for OVRFLW event<br>"]
    pub(crate) fn rtc0_intenclr308_ovrflw_read(&self) -> MemResult<bool> {
        todo!("read OVRFLW mwrite None write None rac None reset value false")
    }
    #[doc = "OVRFLW: Write '1' to Disable interrupt for OVRFLW event<br>"]
    pub(crate) fn rtc0_intenclr308_ovrflw_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write OVRFLW mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE0: Write '1' to Disable interrupt for COMPARE\\[0\\] event<br>"]
    pub(crate) fn rtc0_intenclr308_compare0_read(&self) -> MemResult<bool> {
        todo!("read COMPARE0 mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE0: Write '1' to Disable interrupt for COMPARE\\[0\\] event<br>"]
    pub(crate) fn rtc0_intenclr308_compare0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write COMPARE0 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "COMPARE1: Write '1' to Disable interrupt for COMPARE\\[1\\] event<br>"]
    pub(crate) fn rtc0_intenclr308_compare1_read(&self) -> MemResult<bool> {
        todo!("read COMPARE1 mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE1: Write '1' to Disable interrupt for COMPARE\\[1\\] event<br>"]
    pub(crate) fn rtc0_intenclr308_compare1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write COMPARE1 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "COMPARE2: Write '1' to Disable interrupt for COMPARE\\[2\\] event<br>"]
    pub(crate) fn rtc0_intenclr308_compare2_read(&self) -> MemResult<bool> {
        todo!("read COMPARE2 mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE2: Write '1' to Disable interrupt for COMPARE\\[2\\] event<br>"]
    pub(crate) fn rtc0_intenclr308_compare2_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write COMPARE2 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "COMPARE3: Write '1' to Disable interrupt for COMPARE\\[3\\] event<br>"]
    pub(crate) fn rtc0_intenclr308_compare3_read(&self) -> MemResult<bool> {
        todo!("read COMPARE3 mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE3: Write '1' to Disable interrupt for COMPARE\\[3\\] event<br>"]
    pub(crate) fn rtc0_intenclr308_compare3_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write COMPARE3 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "TICK: Enable or disable event routing for TICK event<br>"]
    pub(crate) fn rtc0_evten340_tick_read(&self) -> MemResult<bool> {
        todo!("read TICK mwrite None write None rac None reset value false")
    }
    #[doc = "TICK: Enable or disable event routing for TICK event<br>"]
    pub(crate) fn rtc0_evten340_tick_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write TICK mwrite None write None rac None reset value false")
    }
    #[doc = "OVRFLW: Enable or disable event routing for OVRFLW event<br>"]
    pub(crate) fn rtc0_evten340_ovrflw_read(&self) -> MemResult<bool> {
        todo!("read OVRFLW mwrite None write None rac None reset value false")
    }
    #[doc = "OVRFLW: Enable or disable event routing for OVRFLW event<br>"]
    pub(crate) fn rtc0_evten340_ovrflw_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write OVRFLW mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE0: Enable or disable event routing for COMPARE\\[0\\] event<br>"]
    pub(crate) fn rtc0_evten340_compare0_read(&self) -> MemResult<bool> {
        todo!("read COMPARE0 mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE0: Enable or disable event routing for COMPARE\\[0\\] event<br>"]
    pub(crate) fn rtc0_evten340_compare0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write COMPARE0 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "COMPARE1: Enable or disable event routing for COMPARE\\[1\\] event<br>"]
    pub(crate) fn rtc0_evten340_compare1_read(&self) -> MemResult<bool> {
        todo!("read COMPARE1 mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE1: Enable or disable event routing for COMPARE\\[1\\] event<br>"]
    pub(crate) fn rtc0_evten340_compare1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write COMPARE1 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "COMPARE2: Enable or disable event routing for COMPARE\\[2\\] event<br>"]
    pub(crate) fn rtc0_evten340_compare2_read(&self) -> MemResult<bool> {
        todo!("read COMPARE2 mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE2: Enable or disable event routing for COMPARE\\[2\\] event<br>"]
    pub(crate) fn rtc0_evten340_compare2_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write COMPARE2 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "COMPARE3: Enable or disable event routing for COMPARE\\[3\\] event<br>"]
    pub(crate) fn rtc0_evten340_compare3_read(&self) -> MemResult<bool> {
        todo!("read COMPARE3 mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE3: Enable or disable event routing for COMPARE\\[3\\] event<br>"]
    pub(crate) fn rtc0_evten340_compare3_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write COMPARE3 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "TICK: Write '1' to Enable event routing for TICK event<br>"]
    pub(crate) fn rtc0_evtenset344_tick_read(&self) -> MemResult<bool> {
        todo!("read TICK mwrite None write None rac None reset value false")
    }
    #[doc = "TICK: Write '1' to Enable event routing for TICK event<br>"]
    pub(crate) fn rtc0_evtenset344_tick_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write TICK mwrite None write None rac None reset value false")
    }
    #[doc = "OVRFLW: Write '1' to Enable event routing for OVRFLW event<br>"]
    pub(crate) fn rtc0_evtenset344_ovrflw_read(&self) -> MemResult<bool> {
        todo!("read OVRFLW mwrite None write None rac None reset value false")
    }
    #[doc = "OVRFLW: Write '1' to Enable event routing for OVRFLW event<br>"]
    pub(crate) fn rtc0_evtenset344_ovrflw_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write OVRFLW mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE0: Write '1' to Enable event routing for COMPARE\\[0\\] event<br>"]
    pub(crate) fn rtc0_evtenset344_compare0_read(&self) -> MemResult<bool> {
        todo!("read COMPARE0 mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE0: Write '1' to Enable event routing for COMPARE\\[0\\] event<br>"]
    pub(crate) fn rtc0_evtenset344_compare0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write COMPARE0 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "COMPARE1: Write '1' to Enable event routing for COMPARE\\[1\\] event<br>"]
    pub(crate) fn rtc0_evtenset344_compare1_read(&self) -> MemResult<bool> {
        todo!("read COMPARE1 mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE1: Write '1' to Enable event routing for COMPARE\\[1\\] event<br>"]
    pub(crate) fn rtc0_evtenset344_compare1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write COMPARE1 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "COMPARE2: Write '1' to Enable event routing for COMPARE\\[2\\] event<br>"]
    pub(crate) fn rtc0_evtenset344_compare2_read(&self) -> MemResult<bool> {
        todo!("read COMPARE2 mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE2: Write '1' to Enable event routing for COMPARE\\[2\\] event<br>"]
    pub(crate) fn rtc0_evtenset344_compare2_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write COMPARE2 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "COMPARE3: Write '1' to Enable event routing for COMPARE\\[3\\] event<br>"]
    pub(crate) fn rtc0_evtenset344_compare3_read(&self) -> MemResult<bool> {
        todo!("read COMPARE3 mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE3: Write '1' to Enable event routing for COMPARE\\[3\\] event<br>"]
    pub(crate) fn rtc0_evtenset344_compare3_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write COMPARE3 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "TICK: Write '1' to Disable event routing for TICK event<br>"]
    pub(crate) fn rtc0_evtenclr348_tick_read(&self) -> MemResult<bool> {
        todo!("read TICK mwrite None write None rac None reset value false")
    }
    #[doc = "TICK: Write '1' to Disable event routing for TICK event<br>"]
    pub(crate) fn rtc0_evtenclr348_tick_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write TICK mwrite None write None rac None reset value false")
    }
    #[doc = "OVRFLW: Write '1' to Disable event routing for OVRFLW event<br>"]
    pub(crate) fn rtc0_evtenclr348_ovrflw_read(&self) -> MemResult<bool> {
        todo!("read OVRFLW mwrite None write None rac None reset value false")
    }
    #[doc = "OVRFLW: Write '1' to Disable event routing for OVRFLW event<br>"]
    pub(crate) fn rtc0_evtenclr348_ovrflw_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write OVRFLW mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE0: Write '1' to Disable event routing for COMPARE\\[0\\] event<br>"]
    pub(crate) fn rtc0_evtenclr348_compare0_read(&self) -> MemResult<bool> {
        todo!("read COMPARE0 mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE0: Write '1' to Disable event routing for COMPARE\\[0\\] event<br>"]
    pub(crate) fn rtc0_evtenclr348_compare0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write COMPARE0 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "COMPARE1: Write '1' to Disable event routing for COMPARE\\[1\\] event<br>"]
    pub(crate) fn rtc0_evtenclr348_compare1_read(&self) -> MemResult<bool> {
        todo!("read COMPARE1 mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE1: Write '1' to Disable event routing for COMPARE\\[1\\] event<br>"]
    pub(crate) fn rtc0_evtenclr348_compare1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write COMPARE1 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "COMPARE2: Write '1' to Disable event routing for COMPARE\\[2\\] event<br>"]
    pub(crate) fn rtc0_evtenclr348_compare2_read(&self) -> MemResult<bool> {
        todo!("read COMPARE2 mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE2: Write '1' to Disable event routing for COMPARE\\[2\\] event<br>"]
    pub(crate) fn rtc0_evtenclr348_compare2_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write COMPARE2 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "COMPARE3: Write '1' to Disable event routing for COMPARE\\[3\\] event<br>"]
    pub(crate) fn rtc0_evtenclr348_compare3_read(&self) -> MemResult<bool> {
        todo!("read COMPARE3 mwrite None write None rac None reset value false")
    }
    #[doc = "COMPARE3: Write '1' to Disable event routing for COMPARE\\[3\\] event<br>"]
    pub(crate) fn rtc0_evtenclr348_compare3_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write COMPARE3 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "COUNTER: Counter value<br>"]
    pub(crate) fn rtc0_counter504_counter_read(&self) -> MemResult<u32> {
        todo ! ("read COUNTER mwrite None write None rac None reset value 0x00 mask 0xffffff")
    }
    #[doc = "PRESCALER: Prescaler value<br>"]
    pub(crate) fn rtc0_prescaler508_prescaler_read(&self) -> MemResult<u16> {
        Ok(self.prescaler)
    }
    #[doc = "PRESCALER: Prescaler value<br>"]
    pub(crate) fn rtc0_prescaler508_prescaler_write(
        &mut self,
        _value: u16,
    ) -> MemResult<()> {
        Ok(self.prescaler = _value)
    }
    #[doc = "COMPARE: Compare value<br>"]
    pub(crate) fn rtc0_ccn540_compare_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<u32> {
        todo ! ("read COMPARE mwrite None write None rac None reset value 0x00 mask 0xffffff")
    }
    #[doc = "COMPARE: Compare value<br>"]
    pub(crate) fn rtc0_ccn540_compare_write(
        &mut self,
        _reg_array: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write COMPARE mwrite None write None rac None reset value 0x00 mask 0xffffff")
    }
}
