#[derive(Default)]
pub struct Peripherals {
    #[doc = "TODO: implement the peripherals data here"]
    _todo: (),
}
impl Peripherals {
    pub fn write_power_tasks_constlat(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_power_tasks_lowpwr(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_power_events_pofwarn(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_power_events_pofwarn(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_powerclock_intenset_pofwarn(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_powerclock_intenset_pofwarn(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_powerclock_intenset_hfclkstarted(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_powerclock_intenset_hfclkstarted(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_powerclock_intenset_lfclkstarted(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_powerclock_intenset_lfclkstarted(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_powerclock_intenset_done(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_powerclock_intenset_done(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_powerclock_intenset_ctto(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_powerclock_intenset_ctto(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_powerclock_intenclr_pofwarn(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_powerclock_intenclr_pofwarn(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_powerclock_intenclr_hfclkstarted(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_powerclock_intenclr_hfclkstarted(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_powerclock_intenclr_lfclkstarted(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_powerclock_intenclr_lfclkstarted(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_powerclock_intenclr_done(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_powerclock_intenclr_done(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_powerclock_intenclr_ctto(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_powerclock_intenclr_ctto(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_power_resetreas_resetpin(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_resetreas_resetpin(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_power_resetreas_dog(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_resetreas_dog(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_power_resetreas_sreq(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_resetreas_sreq(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_power_resetreas_lockup(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_resetreas_lockup(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_power_resetreas_off(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_resetreas_off(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_power_resetreas_lpcomp(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_resetreas_lpcomp(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_power_resetreas_dif(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_resetreas_dif(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_power_ramstatus_ramblock0(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_power_ramstatus_ramblock1(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_power_ramstatus_ramblock2(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_power_ramstatus_ramblock3(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_systemoff_systemoff(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_power_pofcon_pof(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_pofcon_pof(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_power_pofcon_threshold(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_power_pofcon_threshold(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_power_gpregret_gpregret(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_power_gpregret_gpregret(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_power_ramon_onram0(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 1u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_ramon_onram0(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_power_ramon_onram1(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 1u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_ramon_onram1(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_power_ramon_offram0(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_ramon_offram0(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_power_ramon_offram1(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_ramon_offram1(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_power_reset_reset(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_reset_reset(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_power_ramonb_onram2(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 1u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_ramonb_onram2(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_power_ramonb_onram3(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 1u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_ramonb_onram3(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_power_ramonb_offram2(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_ramonb_offram2(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_power_ramonb_offram3(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_ramonb_offram3(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_power_dcdcen_dcdcen(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_dcdcen_dcdcen(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_power_dcdcforce_forceoff(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_dcdcforce_forceoff(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_power_dcdcforce_forceon(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_dcdcforce_forceon(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn write_clock_tasks_hfclkstart(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_clock_tasks_hfclkstop(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_clock_tasks_lfclkstart(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_clock_tasks_lfclkstop(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_clock_tasks_cal(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_clock_tasks_ctstart(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_clock_tasks_ctstop(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_clock_events_hfclkstarted(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_clock_events_hfclkstarted(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_clock_events_lfclkstarted(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_clock_events_lfclkstarted(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_clock_events_done(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_clock_events_done(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_clock_events_ctto(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_clock_events_ctto(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_clock_hfclkrun_status(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_clock_hfclkstat_src(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_clock_hfclkstat_state(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_clock_lfclkrun_status(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_clock_lfclkstat_src(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn read_clock_lfclkstat_state(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_clock_lfclksrccopy_src(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn read_clock_lfclksrc_src(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_clock_lfclksrc_src(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_clock_ctiv_ctiv(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 127u64;
        todo!()
    }
    pub fn write_clock_ctiv_ctiv(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_clock_xtalfreq_xtalfreq(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 255u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_clock_xtalfreq_xtalfreq(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_power_clock(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_power_clock(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_radio(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_radio(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_uart0(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_uart0(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_spi0_twi0(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_spi0_twi0(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_spi1_twi1(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_spi1_twi1(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_gpiote(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_gpiote(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_adc(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_adc(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_timer0(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_timer0(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_timer1(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_timer1(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_timer2(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_timer2(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_rtc0(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_rtc0(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_temp(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_temp(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_rng(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_rng(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_ecb(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_ecb(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_ccm_aar(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_ccm_aar(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_wdt(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_wdt(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_rtc1(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_rtc1(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_qdec(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_qdec(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_lpcomp(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_lpcomp(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_nvmc(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_nvmc(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_ppi(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_ppi(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_rlenr0(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_mpu_rlenr0(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_mpu_protenset0_protreg0(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset0_protreg0(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset0_protreg1(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset0_protreg1(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset0_protreg2(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset0_protreg2(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset0_protreg3(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset0_protreg3(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset0_protreg4(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset0_protreg4(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset0_protreg5(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset0_protreg5(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset0_protreg6(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset0_protreg6(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset0_protreg7(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset0_protreg7(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset0_protreg8(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset0_protreg8(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset0_protreg9(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset0_protreg9(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset0_protreg10(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset0_protreg10(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset0_protreg11(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset0_protreg11(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset0_protreg12(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset0_protreg12(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset0_protreg13(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset0_protreg13(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset0_protreg14(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset0_protreg14(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset0_protreg15(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset0_protreg15(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset0_protreg16(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset0_protreg16(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset0_protreg17(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset0_protreg17(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset0_protreg18(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset0_protreg18(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset0_protreg19(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset0_protreg19(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset0_protreg20(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset0_protreg20(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset0_protreg21(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset0_protreg21(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset0_protreg22(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset0_protreg22(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset0_protreg23(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset0_protreg23(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset0_protreg24(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset0_protreg24(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset0_protreg25(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset0_protreg25(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset0_protreg26(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset0_protreg26(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset0_protreg27(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset0_protreg27(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset0_protreg28(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset0_protreg28(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset0_protreg29(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset0_protreg29(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset0_protreg30(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset0_protreg30(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset0_protreg31(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset0_protreg31(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset1_protreg32(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset1_protreg32(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset1_protreg33(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset1_protreg33(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset1_protreg34(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset1_protreg34(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset1_protreg35(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset1_protreg35(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset1_protreg36(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset1_protreg36(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset1_protreg37(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset1_protreg37(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset1_protreg38(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset1_protreg38(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset1_protreg39(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset1_protreg39(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset1_protreg40(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset1_protreg40(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset1_protreg41(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset1_protreg41(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset1_protreg42(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset1_protreg42(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset1_protreg43(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset1_protreg43(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset1_protreg44(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset1_protreg44(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset1_protreg45(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset1_protreg45(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset1_protreg46(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset1_protreg46(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset1_protreg47(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset1_protreg47(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset1_protreg48(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset1_protreg48(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset1_protreg49(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset1_protreg49(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset1_protreg50(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset1_protreg50(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset1_protreg51(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset1_protreg51(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset1_protreg52(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset1_protreg52(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset1_protreg53(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset1_protreg53(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset1_protreg54(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset1_protreg54(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset1_protreg55(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset1_protreg55(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset1_protreg56(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset1_protreg56(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset1_protreg57(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset1_protreg57(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset1_protreg58(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset1_protreg58(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset1_protreg59(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset1_protreg59(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset1_protreg60(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset1_protreg60(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset1_protreg61(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset1_protreg61(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset1_protreg62(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset1_protreg62(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protenset1_protreg63(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_protenset1_protreg63(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_disableindebug_disableindebug(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 1u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_disableindebug_disableindebug(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protblocksize_protblocksize(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_mpu_protblocksize_protblocksize(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn write_radio_tasks_txen(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_radio_tasks_rxen(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_radio_tasks_start(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_radio_tasks_stop(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_radio_tasks_disable(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_radio_tasks_rssistart(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_radio_tasks_rssistop(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_radio_tasks_bcstart(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_radio_tasks_bcstop(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_radio_events_ready(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_radio_events_ready(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_radio_events_address(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_radio_events_address(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_radio_events_payload(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_radio_events_payload(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_radio_events_end(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_radio_events_end(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_radio_events_disabled(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_radio_events_disabled(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_radio_events_devmatch(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_radio_events_devmatch(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_radio_events_devmiss(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_radio_events_devmiss(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_radio_events_rssiend(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_radio_events_rssiend(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_radio_events_bcmatch(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_radio_events_bcmatch(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_radio_shorts_ready_start(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_shorts_ready_start(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_shorts_end_disable(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_shorts_end_disable(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_shorts_disabled_txen(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_shorts_disabled_txen(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_shorts_disabled_rxen(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_shorts_disabled_rxen(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_shorts_address_rssistart(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_shorts_address_rssistart(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_shorts_end_start(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_shorts_end_start(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_shorts_address_bcstart(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_shorts_address_bcstart(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_shorts_disabled_rssistop(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_shorts_disabled_rssistop(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenset_ready(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenset_ready(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenset_address(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenset_address(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenset_payload(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenset_payload(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenset_end(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenset_end(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenset_disabled(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenset_disabled(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenset_devmatch(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenset_devmatch(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenset_devmiss(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenset_devmiss(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenset_rssiend(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenset_rssiend(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenset_bcmatch(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenset_bcmatch(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenclr_ready(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenclr_ready(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenclr_address(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenclr_address(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenclr_payload(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenclr_payload(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenclr_end(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenclr_end(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenclr_disabled(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenclr_disabled(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenclr_devmatch(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenclr_devmatch(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenclr_devmiss(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenclr_devmiss(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenclr_rssiend(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenclr_rssiend(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenclr_bcmatch(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenclr_bcmatch(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_crcstatus_crcstatus(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_radio_rxmatch_rxmatch(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn read_radio_rxcrc_rxcrc(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 16777215u64;
        match (_byte_0, _byte_1, _byte_2) {
            (None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_radio_dai_dai(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn read_radio_packetptr(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_radio_packetptr(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_radio_frequency_frequency(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 2u64;
        const _RESET_MASK: u64 = 127u64;
        todo!()
    }
    pub fn write_radio_frequency_frequency(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_txpower_txpower(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_radio_txpower_txpower(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_mode_mode(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_radio_mode_mode(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_pcnf0_lflen(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn write_radio_pcnf0_lflen(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_pcnf0_s0len(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_pcnf0_s0len(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_pcnf0_s1len(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn write_radio_pcnf0_s1len(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_pcnf1_maxlen(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_radio_pcnf1_maxlen(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_pcnf1_statlen(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_radio_pcnf1_statlen(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_pcnf1_balen(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_radio_pcnf1_balen(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_pcnf1_endian(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_pcnf1_endian(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_pcnf1_whiteen(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_pcnf1_whiteen(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_base0(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_radio_base0(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_radio_base1(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_radio_base1(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_radio_prefix0_ap0(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_radio_prefix0_ap0(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_prefix0_ap1(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_radio_prefix0_ap1(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_prefix0_ap2(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_radio_prefix0_ap2(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_prefix0_ap3(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_radio_prefix0_ap3(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_prefix1_ap4(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_radio_prefix1_ap4(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_prefix1_ap5(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_radio_prefix1_ap5(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_prefix1_ap6(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_radio_prefix1_ap6(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_prefix1_ap7(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_radio_prefix1_ap7(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_txaddress_txaddress(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_radio_txaddress_txaddress(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_rxaddresses_addr0(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_rxaddresses_addr0(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_rxaddresses_addr1(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_rxaddresses_addr1(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_rxaddresses_addr2(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_rxaddresses_addr2(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_rxaddresses_addr3(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_rxaddresses_addr3(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_rxaddresses_addr4(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_rxaddresses_addr4(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_rxaddresses_addr5(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_rxaddresses_addr5(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_rxaddresses_addr6(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_rxaddresses_addr6(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_rxaddresses_addr7(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_rxaddresses_addr7(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_crccnf_len(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_radio_crccnf_len(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_crccnf_skipaddr(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_crccnf_skipaddr(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_crcpoly_crcpoly(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 16777215u64;
        match (_byte_0, _byte_1, _byte_2) {
            (None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_radio_crcpoly_crcpoly(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2) {
            (None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_radio_crcinit_crcinit(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 16777215u64;
        match (_byte_0, _byte_1, _byte_2) {
            (None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_radio_crcinit_crcinit(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2) {
            (None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_radio_test_constcarrier(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_test_constcarrier(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_test_plllock(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_test_plllock(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_tifs_tifs(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_radio_tifs_tifs(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_rssisample_rssisample(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 127u64;
        todo!()
    }
    pub fn read_radio_state_state(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_radio_datawhiteiv_datawhiteiv(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 64u64;
        const _RESET_MASK: u64 = 127u64;
        todo!()
    }
    pub fn write_radio_datawhiteiv_datawhiteiv(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_bcc(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_radio_bcc(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_radio_dabn(
        &mut self,
        _dim: usize,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_radio_dabn(
        &mut self,
        _dim: usize,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_radio_dapn_dap(
        &mut self,
        _dim: usize,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 65535u64;
        match (_byte_0, _byte_1) {
            (None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_radio_dapn_dap(
        &mut self,
        _dim: usize,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1) {
            (None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_radio_dacnf_ena0(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_dacnf_ena0(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_dacnf_ena1(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_dacnf_ena1(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_dacnf_ena2(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_dacnf_ena2(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_dacnf_ena3(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_dacnf_ena3(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_dacnf_ena4(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_dacnf_ena4(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_dacnf_ena5(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_dacnf_ena5(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_dacnf_ena6(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_dacnf_ena6(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_dacnf_ena7(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_dacnf_ena7(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_dacnf_txadd0(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_dacnf_txadd0(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_dacnf_txadd1(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_dacnf_txadd1(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_dacnf_txadd2(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_dacnf_txadd2(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_dacnf_txadd3(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_dacnf_txadd3(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_dacnf_txadd4(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_dacnf_txadd4(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_dacnf_txadd5(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_dacnf_txadd5(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_dacnf_txadd6(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_dacnf_txadd6(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_dacnf_txadd7(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_dacnf_txadd7(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_override0_override0(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 4294967295u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_radio_override0_override0(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_radio_override1_override1(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 4294967295u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_radio_override1_override1(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_radio_override2_override2(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 4294967295u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_radio_override2_override2(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_radio_override3_override3(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 4294967295u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_radio_override3_override3(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_radio_override4_override4(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 268435455u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_radio_override4_override4(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_radio_override4_enable(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_override4_enable(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_radio_power_power(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_power_power(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn write_uart0_tasks_startrx(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_uart0_tasks_stoprx(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_uart0_tasks_starttx(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_uart0_tasks_stoptx(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_uart0_tasks_suspend(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_uart0_events_cts(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_uart0_events_cts(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_uart0_events_ncts(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_uart0_events_ncts(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_uart0_events_rxdrdy(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_uart0_events_rxdrdy(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_uart0_events_txdrdy(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_uart0_events_txdrdy(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_uart0_events_error(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_uart0_events_error(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_uart0_events_rxto(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_uart0_events_rxto(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_uart0_shorts_cts_startrx(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_shorts_cts_startrx(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_uart0_shorts_ncts_stoprx(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_shorts_ncts_stoprx(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenset_cts(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenset_cts(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenset_ncts(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenset_ncts(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenset_rxdrdy(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenset_rxdrdy(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenset_txdrdy(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenset_txdrdy(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenset_error(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenset_error(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenset_rxto(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenset_rxto(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenclr_cts(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenclr_cts(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenclr_ncts(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenclr_ncts(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenclr_rxdrdy(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenclr_rxdrdy(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenclr_txdrdy(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenclr_txdrdy(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenclr_error(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenclr_error(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenclr_rxto(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenclr_rxto(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_uart0_errorsrc_overrun(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_errorsrc_overrun(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_uart0_errorsrc_parity(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_errorsrc_parity(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_uart0_errorsrc_framing(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_errorsrc_framing(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_uart0_errorsrc_break(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_errorsrc_break(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_uart0_enable_enable(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_uart0_enable_enable(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_uart0_pselrts(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 4294967295u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_uart0_pselrts(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_uart0_pseltxd(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 4294967295u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_uart0_pseltxd(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_uart0_pselcts(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 4294967295u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_uart0_pselcts(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_uart0_pselrxd(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 4294967295u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_uart0_pselrxd(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_uart0_rxd_rxd(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_uart0_txd_txd(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_uart0_baudrate_baudrate(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 4294967295u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_uart0_baudrate_baudrate(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_uart0_config_hwfc(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_config_hwfc(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_uart0_config_parity(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_uart0_config_parity(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_uart0_power_power(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_power_power(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spi0twi0_events_ready(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_spi0twi0_events_ready(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_spi0twi0_intenset_ready(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spi0twi0_intenset_ready(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spi0twi0_intenset_stopped(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spi0twi0_intenset_stopped(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spi0twi0_intenset_txdsent(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spi0twi0_intenset_txdsent(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spi0twi0_intenset_error(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spi0twi0_intenset_error(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spi0twi0_intenset_bb(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spi0twi0_intenset_bb(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spi0twi0_intenset_suspended(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spi0twi0_intenset_suspended(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spi0twi0_intenclr_ready(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spi0twi0_intenclr_ready(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spi0twi0_intenclr_stopped(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spi0twi0_intenclr_stopped(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spi0twi0_intenclr_txdsent(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spi0twi0_intenclr_txdsent(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spi0twi0_intenclr_error(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spi0twi0_intenclr_error(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spi0twi0_intenclr_bb(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spi0twi0_intenclr_bb(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spi0twi0_intenclr_suspended(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spi0twi0_intenclr_suspended(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spi0twi0_enable_enable(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_spi0twi0_enable_enable(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spi0twi0_pselsck(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 4294967295u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_spi0twi0_pselsck(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_spi0twi0_pselmosi(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 4294967295u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_spi0twi0_pselmosi(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_spi0_pselmiso(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 4294967295u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_spi0_pselmiso(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_spi0twi0_rxd_rxd(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn read_spi0twi0_txd_txd(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_spi0twi0_txd_txd(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spi0twi0_frequency_frequency(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 67108864u64;
        const _RESET_MASK: u64 = 4294967295u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_spi0twi0_frequency_frequency(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_spi0_config_order(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spi0_config_order(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spi0_config_cpha(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spi0_config_cpha(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spi0_config_cpol(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spi0_config_cpol(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spi0twi0_power_power(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spi0twi0_power_power(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn write_twi0_tasks_startrx(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_twi0_tasks_starttx(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_twi0_tasks_stop(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_twi0_tasks_suspend(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_twi0_tasks_resume(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_twi0_events_stopped(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_twi0_events_stopped(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_twi0_events_txdsent(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_twi0_events_txdsent(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_twi0_events_error(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_twi0_events_error(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_twi0_events_bb(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_twi0_events_bb(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_twi0_events_suspended(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_twi0_events_suspended(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_twi0_shorts_bb_suspend(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_twi0_shorts_bb_suspend(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_twi0_shorts_bb_stop(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_twi0_shorts_bb_stop(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_twi0_errorsrc_overrun(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_twi0_errorsrc_overrun(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_twi0_errorsrc_anack(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_twi0_errorsrc_anack(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_twi0_errorsrc_dnack(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_twi0_errorsrc_dnack(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_twi0_address_address(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 127u64;
        todo!()
    }
    pub fn write_twi0_address_address(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn write_spis1_tasks_acquire(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_spis1_tasks_release(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_spis1_events_end(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_spis1_events_end(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_spis1_events_endrx(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_spis1_events_endrx(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_spis1_events_acquired(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_spis1_events_acquired(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_spis1_shorts_end_acquire(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spis1_shorts_end_acquire(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spis1_intenset_end(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spis1_intenset_end(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spis1_intenset_endrx(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spis1_intenset_endrx(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spis1_intenset_acquired(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spis1_intenset_acquired(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spis1_intenclr_end(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spis1_intenclr_end(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spis1_intenclr_endrx(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spis1_intenclr_endrx(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spis1_intenclr_acquired(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spis1_intenclr_acquired(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spis1_semstat_semstat(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 1u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn read_spis1_status_overread(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spis1_status_overread(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spis1_status_overflow(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spis1_status_overflow(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spis1_enable_enable(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_spis1_enable_enable(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spis1_pselsck(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 4294967295u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_spis1_pselsck(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_spis1_pselmiso(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 4294967295u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_spis1_pselmiso(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_spis1_pselmosi(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 4294967295u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_spis1_pselmosi(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_spis1_pselcsn(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 4294967295u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_spis1_pselcsn(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_spis1_rxdptr(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_spis1_rxdptr(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_spis1_maxrx_maxrx(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_spis1_maxrx_maxrx(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spis1_amountrx_amountrx(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn read_spis1_txdptr(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_spis1_txdptr(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_spis1_maxtx_maxtx(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_spis1_maxtx_maxtx(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spis1_amounttx_amounttx(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn read_spis1_config_order(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spis1_config_order(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spis1_config_cpha(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spis1_config_cpha(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spis1_config_cpol(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spis1_config_cpol(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spis1_def_def(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_spis1_def_def(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spis1_orc_orc(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_spis1_orc_orc(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_spis1_power_power(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spis1_power_power(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn write_gpiote_tasks_outn(
        &mut self,
        _dim: usize,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_gpiote_events_inn(
        &mut self,
        _dim: usize,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_gpiote_events_inn(
        &mut self,
        _dim: usize,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_gpiote_events_port(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_gpiote_events_port(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_gpiote_intenset_in0(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpiote_intenset_in0(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_intenset_in1(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpiote_intenset_in1(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_intenset_in2(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpiote_intenset_in2(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_intenset_in3(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpiote_intenset_in3(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_intenset_port(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpiote_intenset_port(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_intenclr_in0(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpiote_intenclr_in0(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_intenclr_in1(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpiote_intenclr_in1(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_intenclr_in2(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpiote_intenclr_in2(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_intenclr_in3(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpiote_intenclr_in3(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_intenclr_port(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpiote_intenclr_port(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_confign_mode(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_gpiote_confign_mode(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_confign_psel(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 31u64;
        todo!()
    }
    pub fn write_gpiote_confign_psel(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_confign_polarity(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_gpiote_confign_polarity(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_confign_outinit(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpiote_confign_outinit(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_power_power(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpiote_power_power(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn write_adc_tasks_start(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_adc_tasks_stop(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_adc_events_end(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_adc_events_end(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_adc_intenset_end(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_adc_intenset_end(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_adc_intenclr_end(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_adc_intenclr_end(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_adc_busy_busy(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_adc_enable_enable(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_adc_enable_enable(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_adc_config_res(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_adc_config_res(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_adc_config_inpsel(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 6u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_adc_config_inpsel(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_adc_config_refsel(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_adc_config_refsel(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_adc_config_psel(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_adc_config_psel(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_adc_config_extrefsel(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_adc_config_extrefsel(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_adc_result_result(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1023u64;
        match (_byte_0, _byte_1) {
            (None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_adc_power_power(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_adc_power_power(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn write_timer0_tasks_start(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_timer0_tasks_stop(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_timer0_tasks_count(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_timer0_tasks_clear(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_timer0_tasks_shutdown(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_timer0_tasks_capturen(
        &mut self,
        _dim: usize,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_timer0_events_comparen(
        &mut self,
        _dim: usize,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_timer0_events_comparen(
        &mut self,
        _dim: usize,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_timer0_shorts_compare0_clear(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_shorts_compare0_clear(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_timer0_shorts_compare1_clear(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_shorts_compare1_clear(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_timer0_shorts_compare2_clear(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_shorts_compare2_clear(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_timer0_shorts_compare3_clear(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_shorts_compare3_clear(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_timer0_shorts_compare0_stop(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_shorts_compare0_stop(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_timer0_shorts_compare1_stop(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_shorts_compare1_stop(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_timer0_shorts_compare2_stop(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_shorts_compare2_stop(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_timer0_shorts_compare3_stop(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_shorts_compare3_stop(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_timer0_intenset_compare0(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_intenset_compare0(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_timer0_intenset_compare1(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_intenset_compare1(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_timer0_intenset_compare2(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_intenset_compare2(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_timer0_intenset_compare3(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_intenset_compare3(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_timer0_intenclr_compare0(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_intenclr_compare0(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_timer0_intenclr_compare1(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_intenclr_compare1(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_timer0_intenclr_compare2(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_intenclr_compare2(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_timer0_intenclr_compare3(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_intenclr_compare3(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_timer0_mode_mode(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_mode_mode(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_timer0_bitmode_bitmode(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_timer0_bitmode_bitmode(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_timer0_prescaler_prescaler(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 4u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn write_timer0_prescaler_prescaler(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_timer0_ccn(
        &mut self,
        _dim: usize,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_timer0_ccn(
        &mut self,
        _dim: usize,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_timer0_power_power(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_power_power(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn write_rtc0_tasks_start(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_rtc0_tasks_stop(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_rtc0_tasks_clear(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_rtc0_tasks_trigovrflw(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_rtc0_events_tick(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_rtc0_events_tick(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_rtc0_events_ovrflw(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_rtc0_events_ovrflw(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_rtc0_events_comparen(
        &mut self,
        _dim: usize,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_rtc0_events_comparen(
        &mut self,
        _dim: usize,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_rtc0_intenset_tick(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_intenset_tick(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_intenset_ovrflw(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_intenset_ovrflw(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_intenset_compare0(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_intenset_compare0(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_intenset_compare1(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_intenset_compare1(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_intenset_compare2(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_intenset_compare2(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_intenset_compare3(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_intenset_compare3(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_intenclr_tick(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_intenclr_tick(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_intenclr_ovrflw(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_intenclr_ovrflw(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_intenclr_compare0(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_intenclr_compare0(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_intenclr_compare1(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_intenclr_compare1(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_intenclr_compare2(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_intenclr_compare2(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_intenclr_compare3(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_intenclr_compare3(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evten_tick(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evten_tick(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evten_ovrflw(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evten_ovrflw(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evten_compare0(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evten_compare0(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evten_compare1(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evten_compare1(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evten_compare2(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evten_compare2(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evten_compare3(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evten_compare3(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evtenset_tick(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evtenset_tick(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evtenset_ovrflw(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evtenset_ovrflw(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evtenset_compare0(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evtenset_compare0(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evtenset_compare1(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evtenset_compare1(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evtenset_compare2(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evtenset_compare2(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evtenset_compare3(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evtenset_compare3(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evtenclr_tick(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evtenclr_tick(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evtenclr_ovrflw(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evtenclr_ovrflw(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evtenclr_compare0(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evtenclr_compare0(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evtenclr_compare1(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evtenclr_compare1(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evtenclr_compare2(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evtenclr_compare2(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evtenclr_compare3(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evtenclr_compare3(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_counter_counter(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 16777215u64;
        match (_byte_0, _byte_1, _byte_2) {
            (None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_rtc0_prescaler_prescaler(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 4095u64;
        match (_byte_0, _byte_1) {
            (None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_rtc0_prescaler_prescaler(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1) {
            (None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_rtc0_ccn_compare(
        &mut self,
        _dim: usize,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 16777215u64;
        match (_byte_0, _byte_1, _byte_2) {
            (None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_rtc0_ccn_compare(
        &mut self,
        _dim: usize,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2) {
            (None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_rtc0_power_power(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_power_power(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn write_temp_tasks_start(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_temp_tasks_stop(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_temp_events_datardy(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_temp_events_datardy(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_temp_intenset_datardy(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_temp_intenset_datardy(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_temp_intenclr_datardy(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_temp_intenclr_datardy(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_temp_temp(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_temp_power_power(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_temp_power_power(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn write_rng_tasks_start(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_rng_tasks_stop(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_rng_events_valrdy(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_rng_events_valrdy(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_rng_shorts_valrdy_stop(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rng_shorts_valrdy_stop(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_rng_intenset_valrdy(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rng_intenset_valrdy(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_rng_intenclr_valrdy(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rng_intenclr_valrdy(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_rng_config_dercen(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rng_config_dercen(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_rng_value_value(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn read_rng_power_power(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rng_power_power(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn write_ecb_tasks_startecb(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_ecb_tasks_stopecb(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_ecb_events_endecb(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_ecb_events_endecb(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_ecb_events_errorecb(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_ecb_events_errorecb(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_ecb_intenset_endecb(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ecb_intenset_endecb(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ecb_intenset_errorecb(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ecb_intenset_errorecb(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ecb_intenclr_endecb(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ecb_intenclr_endecb(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ecb_intenclr_errorecb(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ecb_intenclr_errorecb(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ecb_ecbdataptr(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_ecb_ecbdataptr(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_ecb_power_power(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ecb_power_power(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn write_aarccm_tasks_start(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_aarccm_tasks_stop(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_aarccm_events_end(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_aarccm_events_end(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_aarccm_events_resolved(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_aarccm_events_resolved(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_aarccm_events_notresolved(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_aarccm_events_notresolved(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_aarccm_intenset_end(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_aarccm_intenset_end(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_aarccm_intenset_resolved(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_aarccm_intenset_resolved(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_aarccm_intenset_notresolved(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_aarccm_intenset_notresolved(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_aarccm_intenclr_end(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_aarccm_intenclr_end(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_aarccm_intenclr_resolved(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_aarccm_intenclr_resolved(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_aarccm_intenclr_notresolved(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_aarccm_intenclr_notresolved(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_aarccm_status_status(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_aarccm_enable_enable(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_aarccm_enable_enable(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_aarccm_nirk_nirk(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 1u64;
        const _RESET_MASK: u64 = 31u64;
        todo!()
    }
    pub fn write_aarccm_nirk_nirk(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_aarccm_irkptr(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_aarccm_irkptr(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_aarccm_addrptr(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_aarccm_addrptr(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_aarccm_scratchptr(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_aarccm_scratchptr(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_aarccm_power_power(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_aarccm_power_power(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn write_ccm_tasks_crypt(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_ccm_shorts_endksgen_crypt(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ccm_shorts_endksgen_crypt(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ccm_inptr(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_ccm_inptr(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_wdt_tasks_start(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_wdt_events_timeout(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_wdt_events_timeout(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_wdt_intenset_timeout(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_wdt_intenset_timeout(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_wdt_intenclr_timeout(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_wdt_intenclr_timeout(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_wdt_runstatus_runstatus(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_wdt_reqstatus_rr0(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 1u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_wdt_reqstatus_rr1(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_wdt_reqstatus_rr2(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_wdt_reqstatus_rr3(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_wdt_reqstatus_rr4(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_wdt_reqstatus_rr5(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_wdt_reqstatus_rr6(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_wdt_reqstatus_rr7(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_wdt_crv(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 4294967295u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_wdt_crv(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_wdt_rren_rr0(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 1u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_wdt_rren_rr0(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_wdt_rren_rr1(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_wdt_rren_rr1(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_wdt_rren_rr2(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_wdt_rren_rr2(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_wdt_rren_rr3(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_wdt_rren_rr3(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_wdt_rren_rr4(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_wdt_rren_rr4(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_wdt_rren_rr5(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_wdt_rren_rr5(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_wdt_rren_rr6(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_wdt_rren_rr6(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_wdt_rren_rr7(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_wdt_rren_rr7(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_wdt_config_sleep(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 1u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_wdt_config_sleep(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_wdt_config_halt(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_wdt_config_halt(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn write_wdt_rrn_rr(
        &mut self,
        _dim: usize,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_wdt_power_power(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_wdt_power_power(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn write_qdec_tasks_start(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_qdec_tasks_stop(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_qdec_tasks_readclracc(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_qdec_events_samplerdy(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_qdec_events_samplerdy(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_qdec_events_reportrdy(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_qdec_events_reportrdy(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_qdec_events_accof(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_qdec_events_accof(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_qdec_shorts_reportrdy_readclracc(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_qdec_shorts_reportrdy_readclracc(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_qdec_shorts_samplerdy_stop(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_qdec_shorts_samplerdy_stop(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_qdec_intenset_samplerdy(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_qdec_intenset_samplerdy(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_qdec_intenset_reportrdy(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_qdec_intenset_reportrdy(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_qdec_intenset_accof(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_qdec_intenset_accof(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_qdec_intenclr_samplerdy(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_qdec_intenclr_samplerdy(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_qdec_intenclr_reportrdy(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_qdec_intenclr_reportrdy(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_qdec_intenclr_accof(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_qdec_intenclr_accof(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_qdec_enable_enable(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_qdec_enable_enable(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_qdec_ledpol_ledpol(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_qdec_ledpol_ledpol(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_qdec_sampleper_sampleper(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_qdec_sampleper_sampleper(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_qdec_sample_sample(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 4294967295u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_qdec_reportper_reportper(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_qdec_reportper_reportper(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_qdec_acc(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_qdec_accread(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_qdec_pselled(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 4294967295u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_qdec_pselled(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_qdec_psela(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 4294967295u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_qdec_psela(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_qdec_pselb(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 4294967295u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_qdec_pselb(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_qdec_dbfen_dbfen(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_qdec_dbfen_dbfen(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_qdec_ledpre_ledpre(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 16u64;
        const _RESET_MASK: u64 = 511u64;
        match (_byte_0, _byte_1) {
            (None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_qdec_ledpre_ledpre(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1) {
            (None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_qdec_accdbl_accdbl(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_qdec_accdblread_accdblread(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_qdec_power_power(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_qdec_power_power(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn write_lpcomp_tasks_start(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_lpcomp_tasks_stop(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_lpcomp_tasks_sample(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_lpcomp_events_ready(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_lpcomp_events_ready(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_lpcomp_events_down(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_lpcomp_events_down(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_lpcomp_events_up(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_lpcomp_events_up(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_lpcomp_events_cross(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_lpcomp_events_cross(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_lpcomp_shorts_ready_sample(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_shorts_ready_sample(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_shorts_ready_stop(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_shorts_ready_stop(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_shorts_down_stop(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_shorts_down_stop(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_shorts_up_stop(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_shorts_up_stop(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_shorts_cross_stop(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_shorts_cross_stop(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_intenset_ready(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_intenset_ready(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_intenset_down(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_intenset_down(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_intenset_up(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_intenset_up(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_intenset_cross(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_intenset_cross(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_intenclr_ready(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_intenclr_ready(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_intenclr_down(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_intenclr_down(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_intenclr_up(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_intenclr_up(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_intenclr_cross(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_intenclr_cross(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_result_result(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_lpcomp_enable_enable(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_lpcomp_enable_enable(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_psel_psel(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_lpcomp_psel_psel(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_refsel_refsel(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_lpcomp_refsel_refsel(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_extrefsel_extrefsel(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_extrefsel_extrefsel(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_anadetect_anadetect(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_lpcomp_anadetect_anadetect(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_power_power(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_power_power(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_swi_unused(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_nvmc_ready_ready(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_nvmc_config_wen(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_nvmc_config_wen(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_nvmc_erasepage(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_nvmc_erasepage(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_nvmc_eraseall_eraseall(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_nvmc_eraseall_eraseall(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_nvmc_erasepcr0(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_nvmc_erasepcr0(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_nvmc_eraseuicr_eraseuicr(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_nvmc_eraseuicr_eraseuicr(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chen_ch0(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chen_ch0(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chen_ch1(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chen_ch1(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chen_ch2(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chen_ch2(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chen_ch3(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chen_ch3(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chen_ch4(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chen_ch4(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chen_ch5(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chen_ch5(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chen_ch6(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chen_ch6(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chen_ch7(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chen_ch7(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chen_ch8(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chen_ch8(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chen_ch9(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chen_ch9(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chen_ch10(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chen_ch10(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chen_ch11(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chen_ch11(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chen_ch12(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chen_ch12(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chen_ch13(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chen_ch13(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chen_ch14(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chen_ch14(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chen_ch15(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chen_ch15(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chen_ch20(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chen_ch20(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chen_ch21(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chen_ch21(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chen_ch22(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chen_ch22(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chen_ch23(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chen_ch23(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chen_ch24(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chen_ch24(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chen_ch25(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chen_ch25(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chen_ch26(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chen_ch26(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chen_ch27(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chen_ch27(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chen_ch28(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chen_ch28(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chen_ch29(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chen_ch29(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chen_ch30(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chen_ch30(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chen_ch31(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chen_ch31(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenset_ch0(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenset_ch0(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenset_ch1(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenset_ch1(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenset_ch2(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenset_ch2(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenset_ch3(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenset_ch3(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenset_ch4(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenset_ch4(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenset_ch5(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenset_ch5(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenset_ch6(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenset_ch6(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenset_ch7(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenset_ch7(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenset_ch8(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenset_ch8(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenset_ch9(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenset_ch9(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenset_ch10(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenset_ch10(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenset_ch11(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenset_ch11(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenset_ch12(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenset_ch12(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenset_ch13(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenset_ch13(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenset_ch14(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenset_ch14(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenset_ch15(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenset_ch15(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenset_ch20(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenset_ch20(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenset_ch21(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenset_ch21(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenset_ch22(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenset_ch22(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenset_ch23(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenset_ch23(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenset_ch24(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenset_ch24(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenset_ch25(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenset_ch25(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenset_ch26(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenset_ch26(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenset_ch27(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenset_ch27(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenset_ch28(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenset_ch28(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenset_ch29(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenset_ch29(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenset_ch30(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenset_ch30(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenset_ch31(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenset_ch31(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenclr_ch0(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenclr_ch0(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenclr_ch1(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenclr_ch1(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenclr_ch2(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenclr_ch2(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenclr_ch3(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenclr_ch3(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenclr_ch4(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenclr_ch4(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenclr_ch5(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenclr_ch5(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenclr_ch6(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenclr_ch6(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenclr_ch7(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenclr_ch7(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenclr_ch8(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenclr_ch8(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenclr_ch9(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenclr_ch9(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenclr_ch10(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenclr_ch10(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenclr_ch11(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenclr_ch11(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenclr_ch12(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenclr_ch12(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenclr_ch13(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenclr_ch13(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenclr_ch14(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenclr_ch14(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenclr_ch15(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenclr_ch15(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenclr_ch20(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenclr_ch20(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenclr_ch21(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenclr_ch21(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenclr_ch22(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenclr_ch22(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenclr_ch23(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenclr_ch23(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenclr_ch24(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenclr_ch24(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenclr_ch25(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenclr_ch25(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenclr_ch26(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenclr_ch26(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenclr_ch27(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenclr_ch27(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenclr_ch28(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenclr_ch28(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenclr_ch29(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenclr_ch29(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenclr_ch30(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenclr_ch30(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chenclr_ch31(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chenclr_ch31(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chgn_ch0(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chgn_ch0(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chgn_ch1(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chgn_ch1(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chgn_ch2(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chgn_ch2(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chgn_ch3(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chgn_ch3(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chgn_ch4(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chgn_ch4(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chgn_ch5(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chgn_ch5(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chgn_ch6(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chgn_ch6(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chgn_ch7(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chgn_ch7(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chgn_ch8(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chgn_ch8(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chgn_ch9(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chgn_ch9(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chgn_ch10(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chgn_ch10(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chgn_ch11(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chgn_ch11(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chgn_ch12(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chgn_ch12(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chgn_ch13(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chgn_ch13(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chgn_ch14(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chgn_ch14(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chgn_ch15(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chgn_ch15(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chgn_ch20(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chgn_ch20(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chgn_ch21(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chgn_ch21(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chgn_ch22(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chgn_ch22(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chgn_ch23(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chgn_ch23(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chgn_ch24(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chgn_ch24(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chgn_ch25(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chgn_ch25(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chgn_ch26(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chgn_ch26(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chgn_ch27(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chgn_ch27(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chgn_ch28(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chgn_ch28(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chgn_ch29(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chgn_ch29(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chgn_ch30(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chgn_ch30(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ppi_chgn_ch31(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ppi_chgn_ch31(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_ficr_codepagesize(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_ficr_codesize(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_ficr_clenr0(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_ficr_ppfc_ppfc(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn read_ficr_numramblock(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_ficr_sizeramblocks(
        &mut self,
        _dim: usize,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_ficr_configid_hwid(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 65535u64;
        match (_byte_0, _byte_1) {
            (None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_ficr_configid_fwid(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 65535u64;
        match (_byte_0, _byte_1) {
            (None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_ficr_deviceidn(
        &mut self,
        _dim: usize,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_ficr_ern(
        &mut self,
        _dim: usize,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_ficr_irn(
        &mut self,
        _dim: usize,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_ficr_deviceaddrtype_deviceaddrtype(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_ficr_deviceaddrn(
        &mut self,
        _dim: usize,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_ficr_overrideen_nrf_1mbit(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_ficr_overrideen_ble_1mbit(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_ficr_nrf_1mbitn(
        &mut self,
        _dim: usize,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_ficr_ble_1mbitn(
        &mut self,
        _dim: usize,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_uicr_clenr0(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_uicr_clenr0(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_uicr_rbpconf_pr0(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_uicr_rbpconf_pr0(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_uicr_rbpconf_pall(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_uicr_rbpconf_pall(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_uicr_xtalfreq_xtalfreq(
        &self,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_uicr_xtalfreq_xtalfreq(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_uicr_fwid_fwid(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 65535u64;
        match (_byte_0, _byte_1) {
            (None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_uicr_bootloaderaddr(
        &mut self,
        _dim: usize,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_uicr_bootloaderaddr(
        &mut self,
        _dim: usize,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_uicr_nrfhwn(
        &mut self,
        _dim: usize,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_uicr_nrfhwn(
        &mut self,
        _dim: usize,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_uicr_customern(
        &mut self,
        _dim: usize,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_uicr_customern(
        &mut self,
        _dim: usize,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_gpio_out_pin0(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_out_pin0(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_out_pin1(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_out_pin1(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_out_pin2(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_out_pin2(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_out_pin3(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_out_pin3(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_out_pin4(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_out_pin4(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_out_pin5(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_out_pin5(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_out_pin6(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_out_pin6(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_out_pin7(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_out_pin7(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_out_pin8(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_out_pin8(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_out_pin9(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_out_pin9(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_out_pin10(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_out_pin10(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_out_pin11(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_out_pin11(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_out_pin12(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_out_pin12(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_out_pin13(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_out_pin13(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_out_pin14(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_out_pin14(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_out_pin15(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_out_pin15(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_out_pin16(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_out_pin16(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_out_pin17(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_out_pin17(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_out_pin18(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_out_pin18(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_out_pin19(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_out_pin19(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_out_pin20(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_out_pin20(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_out_pin21(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_out_pin21(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_out_pin22(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_out_pin22(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_out_pin23(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_out_pin23(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_out_pin24(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_out_pin24(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_out_pin25(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_out_pin25(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_out_pin26(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_out_pin26(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_out_pin27(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_out_pin27(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_out_pin28(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_out_pin28(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_out_pin29(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_out_pin29(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_out_pin30(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_out_pin30(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_out_pin31(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_out_pin31(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outset_pin0(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outset_pin0(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outset_pin1(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outset_pin1(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outset_pin2(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outset_pin2(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outset_pin3(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outset_pin3(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outset_pin4(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outset_pin4(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outset_pin5(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outset_pin5(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outset_pin6(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outset_pin6(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outset_pin7(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outset_pin7(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outset_pin8(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outset_pin8(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outset_pin9(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outset_pin9(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outset_pin10(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outset_pin10(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outset_pin11(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outset_pin11(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outset_pin12(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outset_pin12(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outset_pin13(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outset_pin13(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outset_pin14(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outset_pin14(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outset_pin15(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outset_pin15(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outset_pin16(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outset_pin16(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outset_pin17(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outset_pin17(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outset_pin18(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outset_pin18(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outset_pin19(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outset_pin19(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outset_pin20(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outset_pin20(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outset_pin21(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outset_pin21(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outset_pin22(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outset_pin22(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outset_pin23(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outset_pin23(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outset_pin24(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outset_pin24(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outset_pin25(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outset_pin25(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outset_pin26(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outset_pin26(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outset_pin27(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outset_pin27(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outset_pin28(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outset_pin28(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outset_pin29(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outset_pin29(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outset_pin30(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outset_pin30(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outset_pin31(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outset_pin31(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outclr_pin0(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outclr_pin0(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outclr_pin1(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outclr_pin1(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outclr_pin2(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outclr_pin2(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outclr_pin3(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outclr_pin3(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outclr_pin4(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outclr_pin4(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outclr_pin5(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outclr_pin5(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outclr_pin6(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outclr_pin6(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outclr_pin7(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outclr_pin7(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outclr_pin8(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outclr_pin8(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outclr_pin9(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outclr_pin9(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outclr_pin10(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outclr_pin10(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outclr_pin11(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outclr_pin11(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outclr_pin12(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outclr_pin12(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outclr_pin13(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outclr_pin13(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outclr_pin14(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outclr_pin14(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outclr_pin15(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outclr_pin15(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outclr_pin16(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outclr_pin16(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outclr_pin17(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outclr_pin17(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outclr_pin18(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outclr_pin18(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outclr_pin19(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outclr_pin19(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outclr_pin20(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outclr_pin20(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outclr_pin21(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outclr_pin21(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outclr_pin22(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outclr_pin22(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outclr_pin23(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outclr_pin23(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outclr_pin24(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outclr_pin24(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outclr_pin25(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outclr_pin25(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outclr_pin26(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outclr_pin26(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outclr_pin27(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outclr_pin27(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outclr_pin28(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outclr_pin28(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outclr_pin29(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outclr_pin29(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outclr_pin30(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outclr_pin30(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_outclr_pin31(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_outclr_pin31(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_in_pin0(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_gpio_in_pin1(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_gpio_in_pin2(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_gpio_in_pin3(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_gpio_in_pin4(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_gpio_in_pin5(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_gpio_in_pin6(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_gpio_in_pin7(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_gpio_in_pin8(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_gpio_in_pin9(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_gpio_in_pin10(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_gpio_in_pin11(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_gpio_in_pin12(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_gpio_in_pin13(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_gpio_in_pin14(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_gpio_in_pin15(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_gpio_in_pin16(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_gpio_in_pin17(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_gpio_in_pin18(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_gpio_in_pin19(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_gpio_in_pin20(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_gpio_in_pin21(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_gpio_in_pin22(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_gpio_in_pin23(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_gpio_in_pin24(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_gpio_in_pin25(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_gpio_in_pin26(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_gpio_in_pin27(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_gpio_in_pin28(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_gpio_in_pin29(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_gpio_in_pin30(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_gpio_in_pin31(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_gpio_dir_pin0(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dir_pin0(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dir_pin1(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dir_pin1(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dir_pin2(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dir_pin2(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dir_pin3(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dir_pin3(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dir_pin4(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dir_pin4(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dir_pin5(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dir_pin5(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dir_pin6(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dir_pin6(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dir_pin7(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dir_pin7(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dir_pin8(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dir_pin8(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dir_pin9(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dir_pin9(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dir_pin10(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dir_pin10(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dir_pin11(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dir_pin11(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dir_pin12(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dir_pin12(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dir_pin13(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dir_pin13(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dir_pin14(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dir_pin14(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dir_pin15(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dir_pin15(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dir_pin16(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dir_pin16(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dir_pin17(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dir_pin17(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dir_pin18(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dir_pin18(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dir_pin19(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dir_pin19(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dir_pin20(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dir_pin20(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dir_pin21(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dir_pin21(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dir_pin22(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dir_pin22(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dir_pin23(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dir_pin23(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dir_pin24(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dir_pin24(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dir_pin25(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dir_pin25(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dir_pin26(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dir_pin26(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dir_pin27(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dir_pin27(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dir_pin28(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dir_pin28(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dir_pin29(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dir_pin29(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dir_pin30(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dir_pin30(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dir_pin31(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dir_pin31(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirset_pin0(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirset_pin0(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirset_pin1(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirset_pin1(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirset_pin2(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirset_pin2(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirset_pin3(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirset_pin3(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirset_pin4(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirset_pin4(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirset_pin5(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirset_pin5(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirset_pin6(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirset_pin6(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirset_pin7(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirset_pin7(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirset_pin8(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirset_pin8(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirset_pin9(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirset_pin9(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirset_pin10(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirset_pin10(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirset_pin11(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirset_pin11(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirset_pin12(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirset_pin12(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirset_pin13(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirset_pin13(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirset_pin14(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirset_pin14(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirset_pin15(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirset_pin15(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirset_pin16(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirset_pin16(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirset_pin17(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirset_pin17(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirset_pin18(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirset_pin18(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirset_pin19(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirset_pin19(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirset_pin20(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirset_pin20(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirset_pin21(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirset_pin21(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirset_pin22(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirset_pin22(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirset_pin23(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirset_pin23(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirset_pin24(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirset_pin24(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirset_pin25(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirset_pin25(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirset_pin26(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirset_pin26(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirset_pin27(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirset_pin27(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirset_pin28(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirset_pin28(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirset_pin29(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirset_pin29(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirset_pin30(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirset_pin30(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirset_pin31(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirset_pin31(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirclr_pin0(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirclr_pin0(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirclr_pin1(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirclr_pin1(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirclr_pin2(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirclr_pin2(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirclr_pin3(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirclr_pin3(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirclr_pin4(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirclr_pin4(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirclr_pin5(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirclr_pin5(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirclr_pin6(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirclr_pin6(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirclr_pin7(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirclr_pin7(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirclr_pin8(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirclr_pin8(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirclr_pin9(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirclr_pin9(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirclr_pin10(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirclr_pin10(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirclr_pin11(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirclr_pin11(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirclr_pin12(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirclr_pin12(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirclr_pin13(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirclr_pin13(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirclr_pin14(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirclr_pin14(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirclr_pin15(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirclr_pin15(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirclr_pin16(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirclr_pin16(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirclr_pin17(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirclr_pin17(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirclr_pin18(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirclr_pin18(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirclr_pin19(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirclr_pin19(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirclr_pin20(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirclr_pin20(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirclr_pin21(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirclr_pin21(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirclr_pin22(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirclr_pin22(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirclr_pin23(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirclr_pin23(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirclr_pin24(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirclr_pin24(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirclr_pin25(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirclr_pin25(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirclr_pin26(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirclr_pin26(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirclr_pin27(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirclr_pin27(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirclr_pin28(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirclr_pin28(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirclr_pin29(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirclr_pin29(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirclr_pin30(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirclr_pin30(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_dirclr_pin31(&self) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_dirclr_pin31(
        &self,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_pin_cnfn_dir(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_pin_cnfn_dir(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_pin_cnfn_input(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 1u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpio_pin_cnfn_input(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_pin_cnfn_pull(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_gpio_pin_cnfn_pull(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_pin_cnfn_drive(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_gpio_pin_cnfn_drive(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
    pub fn read_gpio_pin_cnfn_sense(
        &self,
        _dim: usize,
    ) -> icicle_vm::cpu::mem::MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_gpio_pin_cnfn_sense(
        &self,
        _dim: usize,
        _value: u8,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        todo!()
    }
}
