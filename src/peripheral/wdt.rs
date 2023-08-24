use icicle_vm::cpu::mem::MemResult;
#[derive(Default)]
#[doc = "WDT: Watchdog Timer<br><br>Instances:<br>0x40010000: WDT<br>"]
pub struct Wdt {
    #[doc = "TODO: implement things here"]
    _todo: (),
}
impl Wdt {
    pub(crate) fn page_to_index(page: u64) -> usize {
        match page {
            262160u64 => 0usize,
            _ => unreachable!(),
        }
    }
    #[doc = "TASKS_START: Start the watchdog<br>"]
    pub(crate) fn wdt_tasks_start0_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write wdt_tasks_start0 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_TIMEOUT: Watchdog timeout<br>"]
    pub(crate) fn wdt_events_timeout100_read(&self) -> MemResult<u32> {
        todo ! ("read wdt_events_timeout100 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_TIMEOUT: Watchdog timeout<br>"]
    pub(crate) fn wdt_events_timeout100_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write wdt_events_timeout100 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TIMEOUT: Write '1' to Enable interrupt for TIMEOUT event<br>"]
    pub(crate) fn wdt_intenset304_timeout_read(&self) -> MemResult<bool> {
        todo!("read TIMEOUT mwrite None write None rac None reset value false")
    }
    #[doc = "TIMEOUT: Write '1' to Enable interrupt for TIMEOUT event<br>"]
    pub(crate) fn wdt_intenset304_timeout_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write TIMEOUT mwrite None write None rac None reset value false")
    }
    #[doc = "TIMEOUT: Write '1' to Disable interrupt for TIMEOUT event<br>"]
    pub(crate) fn wdt_intenclr308_timeout_read(&self) -> MemResult<bool> {
        todo!("read TIMEOUT mwrite None write None rac None reset value false")
    }
    #[doc = "TIMEOUT: Write '1' to Disable interrupt for TIMEOUT event<br>"]
    pub(crate) fn wdt_intenclr308_timeout_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write TIMEOUT mwrite None write None rac None reset value false")
    }
    #[doc = "RUNSTATUS: Indicates whether or not the watchdog is running<br>"]
    pub(crate) fn wdt_runstatus400_runstatus_read(&self) -> MemResult<bool> {
        todo!(
            "read RUNSTATUS mwrite None write None rac None reset value false"
        )
    }
    #[doc = "RR0: Request status for RR\\[0\\] register<br>"]
    pub(crate) fn wdt_reqstatus404_rr0_read(&self) -> MemResult<bool> {
        todo!("read RR0 mwrite None write None rac None reset value true")
    }
    #[doc = "RR1: Request status for RR\\[1\\] register<br>"]
    pub(crate) fn wdt_reqstatus404_rr1_read(&self) -> MemResult<bool> {
        todo!("read RR1 mwrite None write None rac None reset value false")
    }
    #[doc = "RR2: Request status for RR\\[2\\] register<br>"]
    pub(crate) fn wdt_reqstatus404_rr2_read(&self) -> MemResult<bool> {
        todo!("read RR2 mwrite None write None rac None reset value false")
    }
    #[doc = "RR3: Request status for RR\\[3\\] register<br>"]
    pub(crate) fn wdt_reqstatus404_rr3_read(&self) -> MemResult<bool> {
        todo!("read RR3 mwrite None write None rac None reset value false")
    }
    #[doc = "RR4: Request status for RR\\[4\\] register<br>"]
    pub(crate) fn wdt_reqstatus404_rr4_read(&self) -> MemResult<bool> {
        todo!("read RR4 mwrite None write None rac None reset value false")
    }
    #[doc = "RR5: Request status for RR\\[5\\] register<br>"]
    pub(crate) fn wdt_reqstatus404_rr5_read(&self) -> MemResult<bool> {
        todo!("read RR5 mwrite None write None rac None reset value false")
    }
    #[doc = "RR6: Request status for RR\\[6\\] register<br>"]
    pub(crate) fn wdt_reqstatus404_rr6_read(&self) -> MemResult<bool> {
        todo!("read RR6 mwrite None write None rac None reset value false")
    }
    #[doc = "RR7: Request status for RR\\[7\\] register<br>"]
    pub(crate) fn wdt_reqstatus404_rr7_read(&self) -> MemResult<bool> {
        todo!("read RR7 mwrite None write None rac None reset value false")
    }
    #[doc = "CRV: Counter reload value in number of cycles of the 32.768 kHz clock<br>"]
    pub(crate) fn wdt_crv504_crv_read(&self) -> MemResult<u32> {
        todo ! ("read CRV mwrite None write None rac None reset value 0xffffffff mask 0xffffffff")
    }
    #[doc = "CRV: Counter reload value in number of cycles of the 32.768 kHz clock<br>"]
    pub(crate) fn wdt_crv504_crv_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write CRV mwrite None write None rac None reset value 0xffffffff mask 0xffffffff")
    }
    #[doc = "RR0: Enable or disable RR\\[0\\] register<br>"]
    pub(crate) fn wdt_rren508_rr0_read(&self) -> MemResult<bool> {
        todo!("read RR0 mwrite None write None rac None reset value true")
    }
    #[doc = "RR0: Enable or disable RR\\[0\\] register<br>"]
    pub(crate) fn wdt_rren508_rr0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RR0 mwrite None write None rac None reset value true")
    }
    #[doc = "RR1: Enable or disable RR\\[1\\] register<br>"]
    pub(crate) fn wdt_rren508_rr1_read(&self) -> MemResult<bool> {
        todo!("read RR1 mwrite None write None rac None reset value false")
    }
    #[doc = "RR1: Enable or disable RR\\[1\\] register<br>"]
    pub(crate) fn wdt_rren508_rr1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RR1 mwrite None write None rac None reset value false")
    }
    #[doc = "RR2: Enable or disable RR\\[2\\] register<br>"]
    pub(crate) fn wdt_rren508_rr2_read(&self) -> MemResult<bool> {
        todo!("read RR2 mwrite None write None rac None reset value false")
    }
    #[doc = "RR2: Enable or disable RR\\[2\\] register<br>"]
    pub(crate) fn wdt_rren508_rr2_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RR2 mwrite None write None rac None reset value false")
    }
    #[doc = "RR3: Enable or disable RR\\[3\\] register<br>"]
    pub(crate) fn wdt_rren508_rr3_read(&self) -> MemResult<bool> {
        todo!("read RR3 mwrite None write None rac None reset value false")
    }
    #[doc = "RR3: Enable or disable RR\\[3\\] register<br>"]
    pub(crate) fn wdt_rren508_rr3_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RR3 mwrite None write None rac None reset value false")
    }
    #[doc = "RR4: Enable or disable RR\\[4\\] register<br>"]
    pub(crate) fn wdt_rren508_rr4_read(&self) -> MemResult<bool> {
        todo!("read RR4 mwrite None write None rac None reset value false")
    }
    #[doc = "RR4: Enable or disable RR\\[4\\] register<br>"]
    pub(crate) fn wdt_rren508_rr4_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RR4 mwrite None write None rac None reset value false")
    }
    #[doc = "RR5: Enable or disable RR\\[5\\] register<br>"]
    pub(crate) fn wdt_rren508_rr5_read(&self) -> MemResult<bool> {
        todo!("read RR5 mwrite None write None rac None reset value false")
    }
    #[doc = "RR5: Enable or disable RR\\[5\\] register<br>"]
    pub(crate) fn wdt_rren508_rr5_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RR5 mwrite None write None rac None reset value false")
    }
    #[doc = "RR6: Enable or disable RR\\[6\\] register<br>"]
    pub(crate) fn wdt_rren508_rr6_read(&self) -> MemResult<bool> {
        todo!("read RR6 mwrite None write None rac None reset value false")
    }
    #[doc = "RR6: Enable or disable RR\\[6\\] register<br>"]
    pub(crate) fn wdt_rren508_rr6_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RR6 mwrite None write None rac None reset value false")
    }
    #[doc = "RR7: Enable or disable RR\\[7\\] register<br>"]
    pub(crate) fn wdt_rren508_rr7_read(&self) -> MemResult<bool> {
        todo!("read RR7 mwrite None write None rac None reset value false")
    }
    #[doc = "RR7: Enable or disable RR\\[7\\] register<br>"]
    pub(crate) fn wdt_rren508_rr7_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RR7 mwrite None write None rac None reset value false")
    }
    #[doc = "SLEEP: Configure the watchdog to either be paused, or kept running, while the CPU is sleeping<br>"]
    pub(crate) fn wdt_config50c_sleep_read(&self) -> MemResult<bool> {
        todo!("read SLEEP mwrite None write None rac None reset value true")
    }
    #[doc = "SLEEP: Configure the watchdog to either be paused, or kept running, while the CPU is sleeping<br>"]
    pub(crate) fn wdt_config50c_sleep_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SLEEP mwrite None write None rac None reset value true")
    }
    #[doc = "HALT: Configure the watchdog to either be paused, or kept running, while the CPU is halted by the debugger<br>"]
    pub(crate) fn wdt_config50c_halt_read(&self) -> MemResult<bool> {
        todo!("read HALT mwrite None write None rac None reset value false")
    }
    #[doc = "HALT: Configure the watchdog to either be paused, or kept running, while the CPU is halted by the debugger<br>"]
    pub(crate) fn wdt_config50c_halt_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write HALT mwrite None write None rac None reset value false")
    }
    #[doc = "RR: Reload request register<br>"]
    pub(crate) fn wdt_rrn600_rr_write(
        &mut self,
        _reg_array: usize,
        _value: crate::peripheral::enums::E41WdtRrn600Rr,
    ) -> MemResult<()> {
        todo ! ("write RR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
}
