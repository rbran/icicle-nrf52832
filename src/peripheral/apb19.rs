use icicle_vm::cpu::mem::MemResult;
#[derive(Default)]
#[doc = "COMP: Comparator<br>LPCOMP: Low Power Comparator<br><br>Instances:<br>0x40013000: COMP, LPCOMP<br>"]
pub struct Apb19 {
    #[doc = "TODO: implement things here"]
    _todo: (),
}
impl Apb19 {
    pub(crate) fn page_to_index(page: u64) -> usize {
        match page {
            262163u64 => 0usize,
            _ => unreachable!(),
        }
    }
    #[doc = "TASKS_START: Start comparator<br>TASKS_START: Start comparator<br>"]
    pub(crate) fn apb19_tasks_start0_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb19_tasks_start0 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_STOP: Stop comparator<br>TASKS_STOP: Stop comparator<br>"]
    pub(crate) fn apb19_tasks_stop4_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb19_tasks_stop4 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_SAMPLE: Sample comparator value<br>TASKS_SAMPLE: Sample comparator value<br>"]
    pub(crate) fn apb19_tasks_sample8_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb19_tasks_sample8 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_READY: COMP is ready and output is valid<br>EVENTS_READY: LPCOMP is ready and output is valid<br>"]
    pub(crate) fn apb19_events_ready100_read(&self) -> MemResult<u32> {
        todo ! ("read apb19_events_ready100 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_READY: COMP is ready and output is valid<br>EVENTS_READY: LPCOMP is ready and output is valid<br>"]
    pub(crate) fn apb19_events_ready100_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb19_events_ready100 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_DOWN: Downward crossing<br>EVENTS_DOWN: Downward crossing<br>"]
    pub(crate) fn apb19_events_down104_read(&self) -> MemResult<u32> {
        todo ! ("read apb19_events_down104 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_DOWN: Downward crossing<br>EVENTS_DOWN: Downward crossing<br>"]
    pub(crate) fn apb19_events_down104_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb19_events_down104 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_UP: Upward crossing<br>EVENTS_UP: Upward crossing<br>"]
    pub(crate) fn apb19_events_up108_read(&self) -> MemResult<u32> {
        todo ! ("read apb19_events_up108 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_UP: Upward crossing<br>EVENTS_UP: Upward crossing<br>"]
    pub(crate) fn apb19_events_up108_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb19_events_up108 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_CROSS: Downward or upward crossing<br>EVENTS_CROSS: Downward or upward crossing<br>"]
    pub(crate) fn apb19_events_cross10c_read(&self) -> MemResult<u32> {
        todo ! ("read apb19_events_cross10c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_CROSS: Downward or upward crossing<br>EVENTS_CROSS: Downward or upward crossing<br>"]
    pub(crate) fn apb19_events_cross10c_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb19_events_cross10c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "READY_SAMPLE: Shortcut between READY event and SAMPLE task<br>READY_SAMPLE: Shortcut between READY event and SAMPLE task<br>"]
    pub(crate) fn apb19_shorts200_ready_sample_read(&self) -> MemResult<bool> {
        todo ! ("read READY_SAMPLE, READY_SAMPLE mwrite None write None rac None reset value false")
    }
    #[doc = "READY_SAMPLE: Shortcut between READY event and SAMPLE task<br>READY_SAMPLE: Shortcut between READY event and SAMPLE task<br>"]
    pub(crate) fn apb19_shorts200_ready_sample_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write READY_SAMPLE, READY_SAMPLE mwrite None write None rac None reset value false")
    }
    #[doc = "READY_STOP: Shortcut between READY event and STOP task<br>READY_STOP: Shortcut between READY event and STOP task<br>"]
    pub(crate) fn apb19_shorts200_ready_stop_read(&self) -> MemResult<bool> {
        todo ! ("read READY_STOP, READY_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "READY_STOP: Shortcut between READY event and STOP task<br>READY_STOP: Shortcut between READY event and STOP task<br>"]
    pub(crate) fn apb19_shorts200_ready_stop_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write READY_STOP, READY_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "DOWN_STOP: Shortcut between DOWN event and STOP task<br>DOWN_STOP: Shortcut between DOWN event and STOP task<br>"]
    pub(crate) fn apb19_shorts200_down_stop_read(&self) -> MemResult<bool> {
        todo ! ("read DOWN_STOP, DOWN_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "DOWN_STOP: Shortcut between DOWN event and STOP task<br>DOWN_STOP: Shortcut between DOWN event and STOP task<br>"]
    pub(crate) fn apb19_shorts200_down_stop_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write DOWN_STOP, DOWN_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "UP_STOP: Shortcut between UP event and STOP task<br>UP_STOP: Shortcut between UP event and STOP task<br>"]
    pub(crate) fn apb19_shorts200_up_stop_read(&self) -> MemResult<bool> {
        todo ! ("read UP_STOP, UP_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "UP_STOP: Shortcut between UP event and STOP task<br>UP_STOP: Shortcut between UP event and STOP task<br>"]
    pub(crate) fn apb19_shorts200_up_stop_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write UP_STOP, UP_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "CROSS_STOP: Shortcut between CROSS event and STOP task<br>CROSS_STOP: Shortcut between CROSS event and STOP task<br>"]
    pub(crate) fn apb19_shorts200_cross_stop_read(&self) -> MemResult<bool> {
        todo ! ("read CROSS_STOP, CROSS_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "CROSS_STOP: Shortcut between CROSS event and STOP task<br>CROSS_STOP: Shortcut between CROSS event and STOP task<br>"]
    pub(crate) fn apb19_shorts200_cross_stop_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write CROSS_STOP, CROSS_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "READY: Enable or disable interrupt for READY event<br>"]
    pub(crate) fn apb19_inten300_ready_read(&self) -> MemResult<bool> {
        todo!("read READY mwrite None write None rac None reset value false")
    }
    #[doc = "READY: Enable or disable interrupt for READY event<br>"]
    pub(crate) fn apb19_inten300_ready_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write READY mwrite None write None rac None reset value false")
    }
    #[doc = "DOWN: Enable or disable interrupt for DOWN event<br>"]
    pub(crate) fn apb19_inten300_down_read(&self) -> MemResult<bool> {
        todo!("read DOWN mwrite None write None rac None reset value false")
    }
    #[doc = "DOWN: Enable or disable interrupt for DOWN event<br>"]
    pub(crate) fn apb19_inten300_down_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write DOWN mwrite None write None rac None reset value false")
    }
    #[doc = "UP: Enable or disable interrupt for UP event<br>"]
    pub(crate) fn apb19_inten300_up_read(&self) -> MemResult<bool> {
        todo!("read UP mwrite None write None rac None reset value false")
    }
    #[doc = "UP: Enable or disable interrupt for UP event<br>"]
    pub(crate) fn apb19_inten300_up_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write UP mwrite None write None rac None reset value false")
    }
    #[doc = "CROSS: Enable or disable interrupt for CROSS event<br>"]
    pub(crate) fn apb19_inten300_cross_read(&self) -> MemResult<bool> {
        todo!("read CROSS mwrite None write None rac None reset value false")
    }
    #[doc = "CROSS: Enable or disable interrupt for CROSS event<br>"]
    pub(crate) fn apb19_inten300_cross_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CROSS mwrite None write None rac None reset value false")
    }
    #[doc = "READY: Write '1' to Enable interrupt for READY event<br>READY: Write '1' to Enable interrupt for READY event<br>"]
    pub(crate) fn apb19_intenset304_ready_read(&self) -> MemResult<bool> {
        todo ! ("read READY, READY mwrite None write None rac None reset value false")
    }
    #[doc = "READY: Write '1' to Enable interrupt for READY event<br>READY: Write '1' to Enable interrupt for READY event<br>"]
    pub(crate) fn apb19_intenset304_ready_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write READY, READY mwrite None write None rac None reset value false")
    }
    #[doc = "DOWN: Write '1' to Enable interrupt for DOWN event<br>DOWN: Write '1' to Enable interrupt for DOWN event<br>"]
    pub(crate) fn apb19_intenset304_down_read(&self) -> MemResult<bool> {
        todo!(
            "read DOWN, DOWN mwrite None write None rac None reset value false"
        )
    }
    #[doc = "DOWN: Write '1' to Enable interrupt for DOWN event<br>DOWN: Write '1' to Enable interrupt for DOWN event<br>"]
    pub(crate) fn apb19_intenset304_down_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write DOWN, DOWN mwrite None write None rac None reset value false")
    }
    #[doc = "UP: Write '1' to Enable interrupt for UP event<br>UP: Write '1' to Enable interrupt for UP event<br>"]
    pub(crate) fn apb19_intenset304_up_read(&self) -> MemResult<bool> {
        todo!("read UP, UP mwrite None write None rac None reset value false")
    }
    #[doc = "UP: Write '1' to Enable interrupt for UP event<br>UP: Write '1' to Enable interrupt for UP event<br>"]
    pub(crate) fn apb19_intenset304_up_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write UP, UP mwrite None write None rac None reset value false")
    }
    #[doc = "CROSS: Write '1' to Enable interrupt for CROSS event<br>CROSS: Write '1' to Enable interrupt for CROSS event<br>"]
    pub(crate) fn apb19_intenset304_cross_read(&self) -> MemResult<bool> {
        todo ! ("read CROSS, CROSS mwrite None write None rac None reset value false")
    }
    #[doc = "CROSS: Write '1' to Enable interrupt for CROSS event<br>CROSS: Write '1' to Enable interrupt for CROSS event<br>"]
    pub(crate) fn apb19_intenset304_cross_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write CROSS, CROSS mwrite None write None rac None reset value false")
    }
    #[doc = "READY: Write '1' to Disable interrupt for READY event<br>READY: Write '1' to Disable interrupt for READY event<br>"]
    pub(crate) fn apb19_intenclr308_ready_read(&self) -> MemResult<bool> {
        todo ! ("read READY, READY mwrite None write None rac None reset value false")
    }
    #[doc = "READY: Write '1' to Disable interrupt for READY event<br>READY: Write '1' to Disable interrupt for READY event<br>"]
    pub(crate) fn apb19_intenclr308_ready_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write READY, READY mwrite None write None rac None reset value false")
    }
    #[doc = "DOWN: Write '1' to Disable interrupt for DOWN event<br>DOWN: Write '1' to Disable interrupt for DOWN event<br>"]
    pub(crate) fn apb19_intenclr308_down_read(&self) -> MemResult<bool> {
        todo!(
            "read DOWN, DOWN mwrite None write None rac None reset value false"
        )
    }
    #[doc = "DOWN: Write '1' to Disable interrupt for DOWN event<br>DOWN: Write '1' to Disable interrupt for DOWN event<br>"]
    pub(crate) fn apb19_intenclr308_down_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write DOWN, DOWN mwrite None write None rac None reset value false")
    }
    #[doc = "UP: Write '1' to Disable interrupt for UP event<br>UP: Write '1' to Disable interrupt for UP event<br>"]
    pub(crate) fn apb19_intenclr308_up_read(&self) -> MemResult<bool> {
        todo!("read UP, UP mwrite None write None rac None reset value false")
    }
    #[doc = "UP: Write '1' to Disable interrupt for UP event<br>UP: Write '1' to Disable interrupt for UP event<br>"]
    pub(crate) fn apb19_intenclr308_up_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write UP, UP mwrite None write None rac None reset value false")
    }
    #[doc = "CROSS: Write '1' to Disable interrupt for CROSS event<br>CROSS: Write '1' to Disable interrupt for CROSS event<br>"]
    pub(crate) fn apb19_intenclr308_cross_read(&self) -> MemResult<bool> {
        todo ! ("read CROSS, CROSS mwrite None write None rac None reset value false")
    }
    #[doc = "CROSS: Write '1' to Disable interrupt for CROSS event<br>CROSS: Write '1' to Disable interrupt for CROSS event<br>"]
    pub(crate) fn apb19_intenclr308_cross_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write CROSS, CROSS mwrite None write None rac None reset value false")
    }
    #[doc = "RESULT: Result of last compare. Decision point SAMPLE task.<br>RESULT: Result of last compare. Decision point SAMPLE task.<br>"]
    pub(crate) fn apb19_result400_result_read(&self) -> MemResult<bool> {
        todo ! ("read RESULT, RESULT mwrite None write None rac None reset value false")
    }
    #[doc = "ENABLE: Enable or disable COMP<br>ENABLE: Enable or disable LPCOMP<br>"]
    pub(crate) fn apb19_enable500_enable_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E44Apb19Enable500Enable> {
        todo ! ("read ENABLE, ENABLE mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "ENABLE: Enable or disable COMP<br>ENABLE: Enable or disable LPCOMP<br>"]
    pub(crate) fn apb19_enable500_enable_write(
        &mut self,
        _value: crate::peripheral::enums::E44Apb19Enable500Enable,
    ) -> MemResult<()> {
        todo ! ("write ENABLE, ENABLE mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "PSEL: Analog pin select<br>PSEL: Analog pin select<br>"]
    pub(crate) fn apb19_psel504_psel_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E45Apb19Psel504Psel> {
        todo ! ("read PSEL, PSEL mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "PSEL: Analog pin select<br>PSEL: Analog pin select<br>"]
    pub(crate) fn apb19_psel504_psel_write(
        &mut self,
        _value: crate::peripheral::enums::E45Apb19Psel504Psel,
    ) -> MemResult<()> {
        todo ! ("write PSEL, PSEL mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "REFSEL: Reference select<br>"]
    pub(crate) fn apb19_refsel508_refsel_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E46Apb19Refsel508Refsel> {
        todo ! ("read REFSEL mwrite None write None rac None reset value 0x04 mask 0x07")
    }
    #[doc = "REFSEL: Reference select<br>"]
    pub(crate) fn apb19_refsel508_refsel_write(
        &mut self,
        _value: crate::peripheral::enums::E46Apb19Refsel508Refsel,
    ) -> MemResult<()> {
        todo ! ("write REFSEL mwrite None write None rac None reset value 0x04 mask 0x07")
    }
    #[doc = "EXTREFSEL: External analog reference select<br>"]
    pub(crate) fn apb19_extrefsel50c_extrefsel_read(&self) -> MemResult<bool> {
        todo!(
            "read EXTREFSEL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "EXTREFSEL: External analog reference select<br>"]
    pub(crate) fn apb19_extrefsel50c_extrefsel_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write EXTREFSEL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "ANADETECT: Analog detect configuration<br>"]
    pub(crate) fn apb19_anadetect520_anadetect_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E47Apb19Anadetect520Anadetect>
    {
        todo ! ("read ANADETECT mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "ANADETECT: Analog detect configuration<br>"]
    pub(crate) fn apb19_anadetect520_anadetect_write(
        &mut self,
        _value: crate::peripheral::enums::E47Apb19Anadetect520Anadetect,
    ) -> MemResult<()> {
        todo ! ("write ANADETECT mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "THDOWN: VDOWN = (THDOWN+1)/64*VREF<br>"]
    pub(crate) fn apb19_th530_thdown_read(&self) -> MemResult<u8> {
        todo ! ("read THDOWN mwrite None write None rac None reset value 0x00 mask 0x3f")
    }
    #[doc = "THDOWN: VDOWN = (THDOWN+1)/64*VREF<br>"]
    pub(crate) fn apb19_th530_thdown_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write THDOWN mwrite None write None rac None reset value 0x00 mask 0x3f")
    }
    #[doc = "THUP: VUP = (THUP+1)/64*VREF<br>"]
    pub(crate) fn apb19_th530_thup_read(&self) -> MemResult<u8> {
        todo ! ("read THUP mwrite None write None rac None reset value 0x00 mask 0x3f")
    }
    #[doc = "THUP: VUP = (THUP+1)/64*VREF<br>"]
    pub(crate) fn apb19_th530_thup_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write THUP mwrite None write None rac None reset value 0x00 mask 0x3f")
    }
    #[doc = "SP: Speed and power modes<br>"]
    pub(crate) fn apb19_mode534_sp_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E48Apb19Mode534Sp> {
        todo ! ("read SP mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "SP: Speed and power modes<br>"]
    pub(crate) fn apb19_mode534_sp_write(
        &mut self,
        _value: crate::peripheral::enums::E48Apb19Mode534Sp,
    ) -> MemResult<()> {
        todo ! ("write SP mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "MAIN: Main operation modes<br>"]
    pub(crate) fn apb19_mode534_main_read(&self) -> MemResult<bool> {
        todo!("read MAIN mwrite None write None rac None reset value false")
    }
    #[doc = "MAIN: Main operation modes<br>"]
    pub(crate) fn apb19_mode534_main_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MAIN mwrite None write None rac None reset value false")
    }
    #[doc = "HYST: Comparator hysteresis<br>HYST: Comparator hysteresis enable<br>"]
    pub(crate) fn apb19_hyst538_hyst_read(&self) -> MemResult<bool> {
        todo!(
            "read HYST, HYST mwrite None write None rac None reset value false"
        )
    }
    #[doc = "HYST: Comparator hysteresis<br>HYST: Comparator hysteresis enable<br>"]
    pub(crate) fn apb19_hyst538_hyst_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write HYST, HYST mwrite None write None rac None reset value false")
    }
    #[doc = "ISOURCE: Comparator hysteresis<br>"]
    pub(crate) fn apb19_isource53c_isource_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E49Apb19Isource53cIsource> {
        todo ! ("read ISOURCE mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "ISOURCE: Comparator hysteresis<br>"]
    pub(crate) fn apb19_isource53c_isource_write(
        &mut self,
        _value: crate::peripheral::enums::E49Apb19Isource53cIsource,
    ) -> MemResult<()> {
        todo ! ("write ISOURCE mwrite None write None rac None reset value 0x00 mask 0x03")
    }
}
