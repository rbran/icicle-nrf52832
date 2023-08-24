use icicle_vm::cpu::mem::MemResult;
#[derive(Default)]
#[doc = "PWM0: Pulse Width Modulation Unit 0<br><br>Instances:<br>0x4001c000: PWM0<br>0x40021000: PWM1<br>0x40022000: PWM2<br>"]
pub struct Pwm0 {
    #[doc = "TODO: implement things here"]
    _todo: (),
}
impl Pwm0 {
    pub(crate) fn page_to_index(page: u64) -> usize {
        match page {
            262172u64 => 0usize,
            262177u64 => 1usize,
            262178u64 => 2usize,
            _ => unreachable!(),
        }
    }
    #[doc = "TASKS_STOP: Stops PWM pulse generation on all channels at the end of current PWM period, and stops sequence playback<br>"]
    pub(crate) fn pwm0_tasks_stop4_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write pwm0_tasks_stop4 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_SEQSTART\\[%s\\]: Description collection\\[0\\]:  Loads the first PWM value on all enabled channels from sequence 0, and starts playing that sequence at the rate defined in SEQ\\[0\\]REFRESH and/or DECODER.MODE. Causes PWM generation to start it was not running.<br>"]
    pub(crate) fn pwm0_tasks_seqstartn8_write(
        &mut self,
        _reg_array: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write pwm0_tasks_seqstartn8 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_NEXTSTEP: Steps by one value in the current sequence on all enabled channels if DECODER.MODE=NextStep. Does not cause PWM generation to start it was not running.<br>"]
    pub(crate) fn pwm0_tasks_nextstep10_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write pwm0_tasks_nextstep10 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_STOPPED: Response to STOP task, emitted when PWM pulses are no longer generated<br>"]
    pub(crate) fn pwm0_events_stopped104_read(&self) -> MemResult<u32> {
        todo ! ("read pwm0_events_stopped104 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_STOPPED: Response to STOP task, emitted when PWM pulses are no longer generated<br>"]
    pub(crate) fn pwm0_events_stopped104_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write pwm0_events_stopped104 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_SEQSTARTED\\[%s\\]: Description collection\\[0\\]:  First PWM period started on sequence 0<br>"]
    pub(crate) fn pwm0_events_seqstartedn108_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<u32> {
        todo ! ("read pwm0_events_seqstartedn108 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_SEQSTARTED\\[%s\\]: Description collection\\[0\\]:  First PWM period started on sequence 0<br>"]
    pub(crate) fn pwm0_events_seqstartedn108_write(
        &mut self,
        _reg_array: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write pwm0_events_seqstartedn108 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_SEQEND\\[%s\\]: Description collection\\[0\\]:  Emitted at end of every sequence 0, when last value from RAM has been applied to wave counter<br>"]
    pub(crate) fn pwm0_events_seqendn110_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<u32> {
        todo ! ("read pwm0_events_seqendn110 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_SEQEND\\[%s\\]: Description collection\\[0\\]:  Emitted at end of every sequence 0, when last value from RAM has been applied to wave counter<br>"]
    pub(crate) fn pwm0_events_seqendn110_write(
        &mut self,
        _reg_array: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write pwm0_events_seqendn110 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_PWMPERIODEND: Emitted at the end of each PWM period<br>"]
    pub(crate) fn pwm0_events_pwmperiodend118_read(&self) -> MemResult<u32> {
        todo ! ("read pwm0_events_pwmperiodend118 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_PWMPERIODEND: Emitted at the end of each PWM period<br>"]
    pub(crate) fn pwm0_events_pwmperiodend118_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write pwm0_events_pwmperiodend118 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_LOOPSDONE: Concatenated sequences have been played the amount of times defined in LOOP.CNT<br>"]
    pub(crate) fn pwm0_events_loopsdone11c_read(&self) -> MemResult<u32> {
        todo ! ("read pwm0_events_loopsdone11c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_LOOPSDONE: Concatenated sequences have been played the amount of times defined in LOOP.CNT<br>"]
    pub(crate) fn pwm0_events_loopsdone11c_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write pwm0_events_loopsdone11c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "SEQEND0_STOP: Shortcut between SEQEND\\[0\\] event and STOP task<br>"]
    pub(crate) fn pwm0_shorts200_seqend0_stop_read(&self) -> MemResult<bool> {
        todo ! ("read SEQEND0_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "SEQEND0_STOP: Shortcut between SEQEND\\[0\\] event and STOP task<br>"]
    pub(crate) fn pwm0_shorts200_seqend0_stop_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write SEQEND0_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "SEQEND1_STOP: Shortcut between SEQEND\\[1\\] event and STOP task<br>"]
    pub(crate) fn pwm0_shorts200_seqend1_stop_read(&self) -> MemResult<bool> {
        todo ! ("read SEQEND1_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "SEQEND1_STOP: Shortcut between SEQEND\\[1\\] event and STOP task<br>"]
    pub(crate) fn pwm0_shorts200_seqend1_stop_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write SEQEND1_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "LOOPSDONE_SEQSTART0: Shortcut between LOOPSDONE event and SEQSTART\\[0\\] task<br>"]
    pub(crate) fn pwm0_shorts200_loopsdone_seqstart0_read(
        &self,
    ) -> MemResult<bool> {
        todo ! ("read LOOPSDONE_SEQSTART0 mwrite None write None rac None reset value false")
    }
    #[doc = "LOOPSDONE_SEQSTART0: Shortcut between LOOPSDONE event and SEQSTART\\[0\\] task<br>"]
    pub(crate) fn pwm0_shorts200_loopsdone_seqstart0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write LOOPSDONE_SEQSTART0 mwrite None write None rac None reset value false")
    }
    #[doc = "LOOPSDONE_SEQSTART1: Shortcut between LOOPSDONE event and SEQSTART\\[1\\] task<br>"]
    pub(crate) fn pwm0_shorts200_loopsdone_seqstart1_read(
        &self,
    ) -> MemResult<bool> {
        todo ! ("read LOOPSDONE_SEQSTART1 mwrite None write None rac None reset value false")
    }
    #[doc = "LOOPSDONE_SEQSTART1: Shortcut between LOOPSDONE event and SEQSTART\\[1\\] task<br>"]
    pub(crate) fn pwm0_shorts200_loopsdone_seqstart1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write LOOPSDONE_SEQSTART1 mwrite None write None rac None reset value false")
    }
    #[doc = "LOOPSDONE_STOP: Shortcut between LOOPSDONE event and STOP task<br>"]
    pub(crate) fn pwm0_shorts200_loopsdone_stop_read(&self) -> MemResult<bool> {
        todo ! ("read LOOPSDONE_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "LOOPSDONE_STOP: Shortcut between LOOPSDONE event and STOP task<br>"]
    pub(crate) fn pwm0_shorts200_loopsdone_stop_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write LOOPSDONE_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "STOPPED: Enable or disable interrupt for STOPPED event<br>"]
    pub(crate) fn pwm0_inten300_stopped_read(&self) -> MemResult<bool> {
        todo!("read STOPPED mwrite None write None rac None reset value false")
    }
    #[doc = "STOPPED: Enable or disable interrupt for STOPPED event<br>"]
    pub(crate) fn pwm0_inten300_stopped_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write STOPPED mwrite None write None rac None reset value false")
    }
    #[doc = "SEQSTARTED0: Enable or disable interrupt for SEQSTARTED\\[0\\] event<br>"]
    pub(crate) fn pwm0_inten300_seqstarted0_read(&self) -> MemResult<bool> {
        todo ! ("read SEQSTARTED0 mwrite None write None rac None reset value false")
    }
    #[doc = "SEQSTARTED0: Enable or disable interrupt for SEQSTARTED\\[0\\] event<br>"]
    pub(crate) fn pwm0_inten300_seqstarted0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write SEQSTARTED0 mwrite None write None rac None reset value false")
    }
    #[doc = "SEQSTARTED1: Enable or disable interrupt for SEQSTARTED\\[1\\] event<br>"]
    pub(crate) fn pwm0_inten300_seqstarted1_read(&self) -> MemResult<bool> {
        todo ! ("read SEQSTARTED1 mwrite None write None rac None reset value false")
    }
    #[doc = "SEQSTARTED1: Enable or disable interrupt for SEQSTARTED\\[1\\] event<br>"]
    pub(crate) fn pwm0_inten300_seqstarted1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write SEQSTARTED1 mwrite None write None rac None reset value false")
    }
    #[doc = "SEQEND0: Enable or disable interrupt for SEQEND\\[0\\] event<br>"]
    pub(crate) fn pwm0_inten300_seqend0_read(&self) -> MemResult<bool> {
        todo!("read SEQEND0 mwrite None write None rac None reset value false")
    }
    #[doc = "SEQEND0: Enable or disable interrupt for SEQEND\\[0\\] event<br>"]
    pub(crate) fn pwm0_inten300_seqend0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SEQEND0 mwrite None write None rac None reset value false")
    }
    #[doc = "SEQEND1: Enable or disable interrupt for SEQEND\\[1\\] event<br>"]
    pub(crate) fn pwm0_inten300_seqend1_read(&self) -> MemResult<bool> {
        todo!("read SEQEND1 mwrite None write None rac None reset value false")
    }
    #[doc = "SEQEND1: Enable or disable interrupt for SEQEND\\[1\\] event<br>"]
    pub(crate) fn pwm0_inten300_seqend1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SEQEND1 mwrite None write None rac None reset value false")
    }
    #[doc = "PWMPERIODEND: Enable or disable interrupt for PWMPERIODEND event<br>"]
    pub(crate) fn pwm0_inten300_pwmperiodend_read(&self) -> MemResult<bool> {
        todo ! ("read PWMPERIODEND mwrite None write None rac None reset value false")
    }
    #[doc = "PWMPERIODEND: Enable or disable interrupt for PWMPERIODEND event<br>"]
    pub(crate) fn pwm0_inten300_pwmperiodend_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write PWMPERIODEND mwrite None write None rac None reset value false")
    }
    #[doc = "LOOPSDONE: Enable or disable interrupt for LOOPSDONE event<br>"]
    pub(crate) fn pwm0_inten300_loopsdone_read(&self) -> MemResult<bool> {
        todo!(
            "read LOOPSDONE mwrite None write None rac None reset value false"
        )
    }
    #[doc = "LOOPSDONE: Enable or disable interrupt for LOOPSDONE event<br>"]
    pub(crate) fn pwm0_inten300_loopsdone_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write LOOPSDONE mwrite None write None rac None reset value false"
        )
    }
    #[doc = "STOPPED: Write '1' to Enable interrupt for STOPPED event<br>"]
    pub(crate) fn pwm0_intenset304_stopped_read(&self) -> MemResult<bool> {
        todo!("read STOPPED mwrite None write None rac None reset value false")
    }
    #[doc = "STOPPED: Write '1' to Enable interrupt for STOPPED event<br>"]
    pub(crate) fn pwm0_intenset304_stopped_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write STOPPED mwrite None write None rac None reset value false")
    }
    #[doc = "SEQSTARTED0: Write '1' to Enable interrupt for SEQSTARTED\\[0\\] event<br>"]
    pub(crate) fn pwm0_intenset304_seqstarted0_read(&self) -> MemResult<bool> {
        todo ! ("read SEQSTARTED0 mwrite None write None rac None reset value false")
    }
    #[doc = "SEQSTARTED0: Write '1' to Enable interrupt for SEQSTARTED\\[0\\] event<br>"]
    pub(crate) fn pwm0_intenset304_seqstarted0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write SEQSTARTED0 mwrite None write None rac None reset value false")
    }
    #[doc = "SEQSTARTED1: Write '1' to Enable interrupt for SEQSTARTED\\[1\\] event<br>"]
    pub(crate) fn pwm0_intenset304_seqstarted1_read(&self) -> MemResult<bool> {
        todo ! ("read SEQSTARTED1 mwrite None write None rac None reset value false")
    }
    #[doc = "SEQSTARTED1: Write '1' to Enable interrupt for SEQSTARTED\\[1\\] event<br>"]
    pub(crate) fn pwm0_intenset304_seqstarted1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write SEQSTARTED1 mwrite None write None rac None reset value false")
    }
    #[doc = "SEQEND0: Write '1' to Enable interrupt for SEQEND\\[0\\] event<br>"]
    pub(crate) fn pwm0_intenset304_seqend0_read(&self) -> MemResult<bool> {
        todo!("read SEQEND0 mwrite None write None rac None reset value false")
    }
    #[doc = "SEQEND0: Write '1' to Enable interrupt for SEQEND\\[0\\] event<br>"]
    pub(crate) fn pwm0_intenset304_seqend0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SEQEND0 mwrite None write None rac None reset value false")
    }
    #[doc = "SEQEND1: Write '1' to Enable interrupt for SEQEND\\[1\\] event<br>"]
    pub(crate) fn pwm0_intenset304_seqend1_read(&self) -> MemResult<bool> {
        todo!("read SEQEND1 mwrite None write None rac None reset value false")
    }
    #[doc = "SEQEND1: Write '1' to Enable interrupt for SEQEND\\[1\\] event<br>"]
    pub(crate) fn pwm0_intenset304_seqend1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SEQEND1 mwrite None write None rac None reset value false")
    }
    #[doc = "PWMPERIODEND: Write '1' to Enable interrupt for PWMPERIODEND event<br>"]
    pub(crate) fn pwm0_intenset304_pwmperiodend_read(&self) -> MemResult<bool> {
        todo ! ("read PWMPERIODEND mwrite None write None rac None reset value false")
    }
    #[doc = "PWMPERIODEND: Write '1' to Enable interrupt for PWMPERIODEND event<br>"]
    pub(crate) fn pwm0_intenset304_pwmperiodend_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write PWMPERIODEND mwrite None write None rac None reset value false")
    }
    #[doc = "LOOPSDONE: Write '1' to Enable interrupt for LOOPSDONE event<br>"]
    pub(crate) fn pwm0_intenset304_loopsdone_read(&self) -> MemResult<bool> {
        todo!(
            "read LOOPSDONE mwrite None write None rac None reset value false"
        )
    }
    #[doc = "LOOPSDONE: Write '1' to Enable interrupt for LOOPSDONE event<br>"]
    pub(crate) fn pwm0_intenset304_loopsdone_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write LOOPSDONE mwrite None write None rac None reset value false"
        )
    }
    #[doc = "STOPPED: Write '1' to Disable interrupt for STOPPED event<br>"]
    pub(crate) fn pwm0_intenclr308_stopped_read(&self) -> MemResult<bool> {
        todo!("read STOPPED mwrite None write None rac None reset value false")
    }
    #[doc = "STOPPED: Write '1' to Disable interrupt for STOPPED event<br>"]
    pub(crate) fn pwm0_intenclr308_stopped_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write STOPPED mwrite None write None rac None reset value false")
    }
    #[doc = "SEQSTARTED0: Write '1' to Disable interrupt for SEQSTARTED\\[0\\] event<br>"]
    pub(crate) fn pwm0_intenclr308_seqstarted0_read(&self) -> MemResult<bool> {
        todo ! ("read SEQSTARTED0 mwrite None write None rac None reset value false")
    }
    #[doc = "SEQSTARTED0: Write '1' to Disable interrupt for SEQSTARTED\\[0\\] event<br>"]
    pub(crate) fn pwm0_intenclr308_seqstarted0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write SEQSTARTED0 mwrite None write None rac None reset value false")
    }
    #[doc = "SEQSTARTED1: Write '1' to Disable interrupt for SEQSTARTED\\[1\\] event<br>"]
    pub(crate) fn pwm0_intenclr308_seqstarted1_read(&self) -> MemResult<bool> {
        todo ! ("read SEQSTARTED1 mwrite None write None rac None reset value false")
    }
    #[doc = "SEQSTARTED1: Write '1' to Disable interrupt for SEQSTARTED\\[1\\] event<br>"]
    pub(crate) fn pwm0_intenclr308_seqstarted1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write SEQSTARTED1 mwrite None write None rac None reset value false")
    }
    #[doc = "SEQEND0: Write '1' to Disable interrupt for SEQEND\\[0\\] event<br>"]
    pub(crate) fn pwm0_intenclr308_seqend0_read(&self) -> MemResult<bool> {
        todo!("read SEQEND0 mwrite None write None rac None reset value false")
    }
    #[doc = "SEQEND0: Write '1' to Disable interrupt for SEQEND\\[0\\] event<br>"]
    pub(crate) fn pwm0_intenclr308_seqend0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SEQEND0 mwrite None write None rac None reset value false")
    }
    #[doc = "SEQEND1: Write '1' to Disable interrupt for SEQEND\\[1\\] event<br>"]
    pub(crate) fn pwm0_intenclr308_seqend1_read(&self) -> MemResult<bool> {
        todo!("read SEQEND1 mwrite None write None rac None reset value false")
    }
    #[doc = "SEQEND1: Write '1' to Disable interrupt for SEQEND\\[1\\] event<br>"]
    pub(crate) fn pwm0_intenclr308_seqend1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SEQEND1 mwrite None write None rac None reset value false")
    }
    #[doc = "PWMPERIODEND: Write '1' to Disable interrupt for PWMPERIODEND event<br>"]
    pub(crate) fn pwm0_intenclr308_pwmperiodend_read(&self) -> MemResult<bool> {
        todo ! ("read PWMPERIODEND mwrite None write None rac None reset value false")
    }
    #[doc = "PWMPERIODEND: Write '1' to Disable interrupt for PWMPERIODEND event<br>"]
    pub(crate) fn pwm0_intenclr308_pwmperiodend_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write PWMPERIODEND mwrite None write None rac None reset value false")
    }
    #[doc = "LOOPSDONE: Write '1' to Disable interrupt for LOOPSDONE event<br>"]
    pub(crate) fn pwm0_intenclr308_loopsdone_read(&self) -> MemResult<bool> {
        todo!(
            "read LOOPSDONE mwrite None write None rac None reset value false"
        )
    }
    #[doc = "LOOPSDONE: Write '1' to Disable interrupt for LOOPSDONE event<br>"]
    pub(crate) fn pwm0_intenclr308_loopsdone_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write LOOPSDONE mwrite None write None rac None reset value false"
        )
    }
    #[doc = "ENABLE: Enable or disable PWM module<br>"]
    pub(crate) fn pwm0_enable500_enable_read(&self) -> MemResult<bool> {
        todo!("read ENABLE mwrite None write None rac None reset value false")
    }
    #[doc = "ENABLE: Enable or disable PWM module<br>"]
    pub(crate) fn pwm0_enable500_enable_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ENABLE mwrite None write None rac None reset value false")
    }
    #[doc = "UPDOWN: Selects up or up and down as wave counter mode<br>"]
    pub(crate) fn pwm0_mode504_updown_read(&self) -> MemResult<bool> {
        todo!("read UPDOWN mwrite None write None rac None reset value false")
    }
    #[doc = "UPDOWN: Selects up or up and down as wave counter mode<br>"]
    pub(crate) fn pwm0_mode504_updown_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write UPDOWN mwrite None write None rac None reset value false")
    }
    #[doc = "COUNTERTOP: Value up to which the pulse generator counter counts. This register is ignored when DECODER.MODE=WaveForm and only values from RAM will be used.<br>"]
    pub(crate) fn pwm0_countertop508_countertop_read(&self) -> MemResult<u16> {
        todo ! ("read COUNTERTOP mwrite None write None rac None reset value 0x3ff mask 0x7fff")
    }
    #[doc = "COUNTERTOP: Value up to which the pulse generator counter counts. This register is ignored when DECODER.MODE=WaveForm and only values from RAM will be used.<br>"]
    pub(crate) fn pwm0_countertop508_countertop_write(
        &mut self,
        _value: u16,
    ) -> MemResult<()> {
        todo ! ("write COUNTERTOP mwrite None write None rac None reset value 0x3ff mask 0x7fff")
    }
    #[doc = "PRESCALER: Pre-scaler of PWM_CLK<br>"]
    pub(crate) fn pwm0_prescaler50c_prescaler_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E52Pwm0Prescaler50cPrescaler> {
        todo ! ("read PRESCALER mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "PRESCALER: Pre-scaler of PWM_CLK<br>"]
    pub(crate) fn pwm0_prescaler50c_prescaler_write(
        &mut self,
        _value: crate::peripheral::enums::E52Pwm0Prescaler50cPrescaler,
    ) -> MemResult<()> {
        todo ! ("write PRESCALER mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "LOAD: How a sequence is read from RAM and spread to the compare register<br>"]
    pub(crate) fn pwm0_decoder510_load_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E53Pwm0Decoder510Load> {
        todo ! ("read LOAD mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "LOAD: How a sequence is read from RAM and spread to the compare register<br>"]
    pub(crate) fn pwm0_decoder510_load_write(
        &mut self,
        _value: crate::peripheral::enums::E53Pwm0Decoder510Load,
    ) -> MemResult<()> {
        todo ! ("write LOAD mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "MODE: Selects source for advancing the active sequence<br>"]
    pub(crate) fn pwm0_decoder510_mode_read(&self) -> MemResult<bool> {
        todo!("read MODE mwrite None write None rac None reset value false")
    }
    #[doc = "MODE: Selects source for advancing the active sequence<br>"]
    pub(crate) fn pwm0_decoder510_mode_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MODE mwrite None write None rac None reset value false")
    }
    #[doc = "CNT: Amount of playback of pattern cycles<br>"]
    pub(crate) fn pwm0_loop514_cnt_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E54Pwm0Loop514Cnt> {
        todo ! ("read CNT mwrite None write None rac None reset value 0x00 mask 0xffff")
    }
    #[doc = "CNT: Amount of playback of pattern cycles<br>"]
    pub(crate) fn pwm0_loop514_cnt_write(
        &mut self,
        _value: crate::peripheral::enums::E54Pwm0Loop514Cnt,
    ) -> MemResult<()> {
        todo ! ("write CNT mwrite None write None rac None reset value 0x00 mask 0xffff")
    }
    #[doc = "PTR: Beginning address in Data RAM of this sequence<br>"]
    pub(crate) fn pwm0_seqn_ptr0_ptr_read(
        &self,
        _seqn: usize,
    ) -> MemResult<u32> {
        todo ! ("read PTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "PTR: Beginning address in Data RAM of this sequence<br>"]
    pub(crate) fn pwm0_seqn_ptr0_ptr_write(
        &mut self,
        _seqn: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write PTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "CNT: Amount of values (duty cycles) in this sequence<br>"]
    pub(crate) fn pwm0_seqn_cnt4_cnt_read(
        &self,
        _seqn: usize,
    ) -> MemResult<crate::peripheral::enums::E55Pwm0SeqnCnt4Cnt> {
        todo ! ("read CNT mwrite None write None rac None reset value 0x00 mask 0x7fff")
    }
    #[doc = "CNT: Amount of values (duty cycles) in this sequence<br>"]
    pub(crate) fn pwm0_seqn_cnt4_cnt_write(
        &mut self,
        _seqn: usize,
        _value: crate::peripheral::enums::E55Pwm0SeqnCnt4Cnt,
    ) -> MemResult<()> {
        todo ! ("write CNT mwrite None write None rac None reset value 0x00 mask 0x7fff")
    }
    #[doc = "CNT: Amount of additional PWM periods between samples loaded into compare register (load every REFRESH.CNT+1 PWM periods)<br>"]
    pub(crate) fn pwm0_seqn_refresh8_cnt_read(
        &self,
        _seqn: usize,
    ) -> MemResult<crate::peripheral::enums::E56Pwm0SeqnRefresh8Cnt> {
        todo ! ("read CNT mwrite None write None rac None reset value 0x01 mask 0xffffff")
    }
    #[doc = "CNT: Amount of additional PWM periods between samples loaded into compare register (load every REFRESH.CNT+1 PWM periods)<br>"]
    pub(crate) fn pwm0_seqn_refresh8_cnt_write(
        &mut self,
        _seqn: usize,
        _value: crate::peripheral::enums::E56Pwm0SeqnRefresh8Cnt,
    ) -> MemResult<()> {
        todo ! ("write CNT mwrite None write None rac None reset value 0x01 mask 0xffffff")
    }
    #[doc = "CNT: Time added after the sequence in PWM periods<br>"]
    pub(crate) fn pwm0_seqn_enddelayc_cnt_read(
        &self,
        _seqn: usize,
    ) -> MemResult<u32> {
        todo ! ("read CNT mwrite None write None rac None reset value 0x00 mask 0xffffff")
    }
    #[doc = "CNT: Time added after the sequence in PWM periods<br>"]
    pub(crate) fn pwm0_seqn_enddelayc_cnt_write(
        &mut self,
        _seqn: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write CNT mwrite None write None rac None reset value 0x00 mask 0xffffff")
    }
    #[doc = "PIN: Pin number<br>"]
    pub(crate) fn pwm0_psel_outn0_pin_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<u8> {
        todo ! ("read PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "PIN: Pin number<br>"]
    pub(crate) fn pwm0_psel_outn0_pin_write(
        &mut self,
        _reg_array: usize,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn pwm0_psel_outn0_connect_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn pwm0_psel_outn0_connect_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CONNECT mwrite None write None rac None reset value true")
    }
}
