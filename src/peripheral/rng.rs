use icicle_vm::cpu::mem::MemResult;
#[derive(Default)]
#[doc = "RNG: Random Number Generator<br><br>Instances:<br>0x4000d000: RNG<br>"]
pub struct Rng {
    #[doc = "TODO: implement things here"]
    _todo: (),
}
impl Rng {
    pub(crate) fn page_to_index(page: u64) -> usize {
        match page {
            262157u64 => 0usize,
            _ => unreachable!(),
        }
    }
    #[doc = "TASKS_START: Task starting the random number generator<br>"]
    pub(crate) fn rng_tasks_start0_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write rng_tasks_start0 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_STOP: Task stopping the random number generator<br>"]
    pub(crate) fn rng_tasks_stop4_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write rng_tasks_stop4 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_VALRDY: Event being generated for every new random number written to the VALUE register<br>"]
    pub(crate) fn rng_events_valrdy100_read(&self) -> MemResult<u32> {
        todo ! ("read rng_events_valrdy100 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_VALRDY: Event being generated for every new random number written to the VALUE register<br>"]
    pub(crate) fn rng_events_valrdy100_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write rng_events_valrdy100 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "VALRDY_STOP: Shortcut between VALRDY event and STOP task<br>"]
    pub(crate) fn rng_shorts200_valrdy_stop_read(&self) -> MemResult<bool> {
        todo ! ("read VALRDY_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "VALRDY_STOP: Shortcut between VALRDY event and STOP task<br>"]
    pub(crate) fn rng_shorts200_valrdy_stop_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write VALRDY_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "VALRDY: Write '1' to Enable interrupt for VALRDY event<br>"]
    pub(crate) fn rng_intenset304_valrdy_read(&self) -> MemResult<bool> {
        todo!("read VALRDY mwrite None write None rac None reset value false")
    }
    #[doc = "VALRDY: Write '1' to Enable interrupt for VALRDY event<br>"]
    pub(crate) fn rng_intenset304_valrdy_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write VALRDY mwrite None write None rac None reset value false")
    }
    #[doc = "VALRDY: Write '1' to Disable interrupt for VALRDY event<br>"]
    pub(crate) fn rng_intenclr308_valrdy_read(&self) -> MemResult<bool> {
        todo!("read VALRDY mwrite None write None rac None reset value false")
    }
    #[doc = "VALRDY: Write '1' to Disable interrupt for VALRDY event<br>"]
    pub(crate) fn rng_intenclr308_valrdy_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write VALRDY mwrite None write None rac None reset value false")
    }
    #[doc = "DERCEN: Bias correction<br>"]
    pub(crate) fn rng_config504_dercen_read(&self) -> MemResult<bool> {
        todo!("read DERCEN mwrite None write None rac None reset value false")
    }
    #[doc = "DERCEN: Bias correction<br>"]
    pub(crate) fn rng_config504_dercen_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write DERCEN mwrite None write None rac None reset value false")
    }
    #[doc = "VALUE: Generated random number<br>"]
    pub(crate) fn rng_value508_value_read(&self) -> MemResult<u8> {
        todo ! ("read VALUE mwrite None write None rac None reset value 0x00 mask 0xff")
    }
}
