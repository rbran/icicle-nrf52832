use icicle_vm::cpu::mem::MemResult;
#[derive(Default)]
#[doc = "GPIOTE: GPIO Tasks and Events<br><br>Instances:<br>0x40006000: GPIOTE<br>"]
pub struct Gpiote {
    #[doc = "TODO: implement things here"]
    _todo: (),
}
impl Gpiote {
    pub(crate) fn page_to_index(page: u64) -> usize {
        match page {
            262150u64 => 0usize,
            _ => unreachable!(),
        }
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
