use icicle_vm::cpu::mem::{MemError, MemResult};

mod gpio;
use gpio::*;

#[derive(Default)]
pub struct Peripherals {
    #[doc = "TODO: implement the peripherals data here"]
    pub _todo: (),
    pub gpio: [Gpio; 32],
}
impl Peripherals {
    pub fn ram_is_on(&mut self, _block: u8) -> bool {
        todo!()
    }
    pub fn ram_is_keep_on(&mut self, _block: u8) -> bool {
        todo!()
    }
    pub fn ram_set_keep_on(&mut self, _block: u8, _on: bool) {
        todo!()
    }
    pub fn ram_is_retain_on(&mut self, _block: u8) -> bool {
        todo!()
    }
    pub fn ram_set_retain_on(&mut self, _block: u8, _on: bool) {
        todo!()
    }

    pub fn mpu_is_protected(&mut self, _region: u8) -> bool {
        todo!()
    }
    pub fn mpu_enable_protection(&mut self, _region: u8) {
        todo!()
    }

    pub fn radio_ap(&mut self, _ap: u8) -> u8 {
        todo!()
    }
    pub fn radio_set_ap(&mut self, _ap: u8, _value: u8) {
        todo!()
    }
    pub fn radio_is_recv_on_ap(&mut self, _ap: u8) -> bool {
        todo!()
    }
    pub fn radio_set_recv_on_ap(&mut self, _ap: u8, _on: bool) {
        todo!()
    }
    pub fn radio_is_ena(&mut self, _da: u8) -> bool {
        todo!()
    }
    pub fn radio_set_ena(&mut self, _da: u8, _on: bool) {
        todo!()
    }
    pub fn radio_is_txadd(&mut self, _da: u8) -> bool {
        todo!()
    }
    pub fn radio_set_txadd(&mut self, _da: u8, _on: bool) {
        todo!()
    }

    pub fn ppi_is_on(&mut self, _channel: u8) -> bool {
        todo!()
    }
    pub fn ppi_set_on(&mut self, _channel: u8, _on: bool) {
        todo!()
    }
    pub fn ppi_is_included(&mut self, _dim: usize, _channel: u8) -> bool {
        todo!()
    }
    pub fn ppi_set_included(
        &mut self,
        _dim: usize,
        _channel: u8,
        _value: bool,
    ) {
        todo!()
    }

    pub fn write_power_tasks_constlat(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_powerclock_intenset_pofwarn(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_powerclock_intenset_pofwarn(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_powerclock_intenset_hfclkstarted(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_powerclock_intenset_hfclkstarted(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_powerclock_intenset_lfclkstarted(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_powerclock_intenset_lfclkstarted(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_powerclock_intenset_done(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_powerclock_intenset_done(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_powerclock_intenset_ctto(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_powerclock_intenset_ctto(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_powerclock_intenclr_pofwarn(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_powerclock_intenclr_pofwarn(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_powerclock_intenclr_hfclkstarted(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_powerclock_intenclr_hfclkstarted(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_powerclock_intenclr_lfclkstarted(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_powerclock_intenclr_lfclkstarted(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_powerclock_intenclr_done(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_powerclock_intenclr_done(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_powerclock_intenclr_ctto(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_powerclock_intenclr_ctto(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_power_resetreas_resetpin(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_resetreas_resetpin(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_power_resetreas_dog(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_resetreas_dog(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_power_resetreas_sreq(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_resetreas_sreq(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_power_resetreas_lockup(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_resetreas_lockup(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_power_resetreas_off(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_resetreas_off(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_power_resetreas_lpcomp(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_resetreas_lpcomp(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_power_resetreas_dif(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_resetreas_dif(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    #[inline]
    pub fn read_power_ramstatus_ramblock0(&mut self) -> MemResult<u8> {
        Ok(self.ram_is_on(0) as u8)
    }
    #[inline]
    pub fn read_power_ramstatus_ramblock1(&mut self) -> MemResult<u8> {
        Ok(self.ram_is_on(1) as u8)
    }
    #[inline]
    pub fn read_power_ramstatus_ramblock2(&mut self) -> MemResult<u8> {
        Ok(self.ram_is_on(2) as u8)
    }
    #[inline]
    pub fn read_power_ramstatus_ramblock3(&mut self) -> MemResult<u8> {
        Ok(self.ram_is_on(3) as u8)
    }
    pub fn write_power_systemoff_systemoff(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_power_pofcon_pof(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_pofcon_pof(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_power_pofcon_threshold(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_power_pofcon_threshold(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_power_gpregret_gpregret(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_power_gpregret_gpregret(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    #[inline]
    pub fn read_power_ramon_onram0(&mut self) -> MemResult<u8> {
        Ok(self.ram_is_keep_on(0) as u8)
    }
    #[inline]
    pub fn write_power_ramon_onram0(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ram_set_keep_on(0, _value != 0))
    }
    #[inline]
    pub fn read_power_ramon_onram1(&mut self) -> MemResult<u8> {
        Ok(self.ram_is_keep_on(1) as u8)
    }
    #[inline]
    pub fn write_power_ramon_onram1(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ram_set_keep_on(1, _value != 0))
    }
    #[inline]
    pub fn read_power_ramon_offram0(&mut self) -> MemResult<u8> {
        Ok(self.ram_is_retain_on(0) as u8)
    }
    #[inline]
    pub fn write_power_ramon_offram0(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ram_set_retain_on(0, _value != 0))
    }
    #[inline]
    pub fn read_power_ramon_offram1(&mut self) -> MemResult<u8> {
        Ok(self.ram_is_retain_on(1) as u8)
    }
    #[inline]
    pub fn write_power_ramon_offram1(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ram_set_retain_on(1, _value != 0))
    }
    pub fn read_power_reset_reset(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_reset_reset(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    #[inline]
    pub fn read_power_ramonb_onram2(&mut self) -> MemResult<u8> {
        Ok(self.ram_is_keep_on(2) as u8)
    }
    #[inline]
    pub fn write_power_ramonb_onram2(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ram_set_keep_on(2, _value != 0))
    }
    #[inline]
    pub fn read_power_ramonb_onram3(&mut self) -> MemResult<u8> {
        Ok(self.ram_is_keep_on(3) as u8)
    }
    #[inline]
    pub fn write_power_ramonb_onram3(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ram_set_keep_on(3, _value != 0))
    }
    #[inline]
    pub fn read_power_ramonb_offram2(&mut self) -> MemResult<u8> {
        Ok(self.ram_is_retain_on(2) as u8)
    }
    #[inline]
    pub fn write_power_ramonb_offram2(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ram_set_retain_on(2, _value != 0))
    }
    #[inline]
    pub fn read_power_ramonb_offram3(&mut self) -> MemResult<u8> {
        Ok(self.ram_is_retain_on(3) as u8)
    }
    #[inline]
    pub fn write_power_ramonb_offram3(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ram_set_retain_on(3, _value != 0))
    }
    pub fn read_power_dcdcen_dcdcen(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_dcdcen_dcdcen(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_power_dcdcforce_forceoff(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_dcdcforce_forceoff(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_power_dcdcforce_forceon(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_dcdcforce_forceon(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn write_clock_tasks_hfclkstart(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_clock_hfclkrun_status(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_clock_hfclkstat_src(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_clock_hfclkstat_state(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_clock_lfclkrun_status(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_clock_lfclkstat_src(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn read_clock_lfclkstat_state(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_clock_lfclksrccopy_src(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn read_clock_lfclksrc_src(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_clock_lfclksrc_src(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_clock_ctiv_ctiv(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 127u64;
        todo!()
    }
    pub fn write_clock_ctiv_ctiv(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_clock_xtalfreq_xtalfreq(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 255u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_clock_xtalfreq_xtalfreq(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_power_clock(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_power_clock(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_radio(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_radio(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_uart0(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_uart0(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_spi0_twi0(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_spi0_twi0(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_spi1_twi1(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_spi1_twi1(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_gpiote(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_gpiote(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_adc(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_adc(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_timer0(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_timer0(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_timer1(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_timer1(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_timer2(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_timer2(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_rtc0(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_rtc0(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_temp(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_temp(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_rng(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_rng(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_ecb(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_ecb(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_ccm_aar(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_ccm_aar(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_wdt(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_wdt(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_rtc1(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_rtc1(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_qdec(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_qdec(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_lpcomp(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_lpcomp(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_nvmc(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_nvmc(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_ppi(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_ppi(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_rlenr0(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    #[inline]
    pub fn read_mpu_protenset0_protreg0(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(0) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset0_protreg0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(0)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset0_protreg1(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(1) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset0_protreg1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(1)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset0_protreg2(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(2) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset0_protreg2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(2)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset0_protreg3(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(3) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset0_protreg3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(3)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset0_protreg4(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(4) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset0_protreg4(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(4)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset0_protreg5(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(5) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset0_protreg5(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(5)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset0_protreg6(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(6) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset0_protreg6(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(6)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset0_protreg7(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(7) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset0_protreg7(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(7)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset0_protreg8(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(8) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset0_protreg8(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(8)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset0_protreg9(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(9) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset0_protreg9(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(9)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset0_protreg10(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(10) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset0_protreg10(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(10)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset0_protreg11(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(11) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset0_protreg11(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(11)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset0_protreg12(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(12) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset0_protreg12(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(12)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset0_protreg13(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(13) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset0_protreg13(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(13)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset0_protreg14(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(14) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset0_protreg14(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(14)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset0_protreg15(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(15) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset0_protreg15(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(15)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset0_protreg16(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(16) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset0_protreg16(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(16)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset0_protreg17(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(17) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset0_protreg17(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(17)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset0_protreg18(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(18) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset0_protreg18(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(18)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset0_protreg19(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(19) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset0_protreg19(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(19)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset0_protreg20(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(20) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset0_protreg20(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(20)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset0_protreg21(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(21) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset0_protreg21(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(21)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset0_protreg22(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(22) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset0_protreg22(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(22)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset0_protreg23(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(23) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset0_protreg23(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(23)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset0_protreg24(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(24) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset0_protreg24(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(24)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset0_protreg25(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(25) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset0_protreg25(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(25)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset0_protreg26(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(26) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset0_protreg26(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(26)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset0_protreg27(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(27) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset0_protreg27(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(27)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset0_protreg28(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(28) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset0_protreg28(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(28)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset0_protreg29(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(29) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset0_protreg29(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(29)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset0_protreg30(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(30) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset0_protreg30(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(30)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset0_protreg31(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(31) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset0_protreg31(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(31)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset1_protreg32(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(32) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset1_protreg32(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(32)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset1_protreg33(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(33) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset1_protreg33(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(33)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset1_protreg34(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(34) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset1_protreg34(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(34)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset1_protreg35(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(35) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset1_protreg35(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(35)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset1_protreg36(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(36) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset1_protreg36(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(36)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset1_protreg37(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(37) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset1_protreg37(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(37)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset1_protreg38(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(38) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset1_protreg38(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(38)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset1_protreg39(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(39) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset1_protreg39(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(39)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset1_protreg40(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(40) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset1_protreg40(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(40)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset1_protreg41(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(41) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset1_protreg41(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(41)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset1_protreg42(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(42) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset1_protreg42(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(42)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset1_protreg43(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(43) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset1_protreg43(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(43)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset1_protreg44(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(44) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset1_protreg44(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(44)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset1_protreg45(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(45) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset1_protreg45(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(45)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset1_protreg46(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(46) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset1_protreg46(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(46)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset1_protreg47(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(47) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset1_protreg47(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(47)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset1_protreg48(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(48) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset1_protreg48(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(48)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset1_protreg49(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(49) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset1_protreg49(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(49)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset1_protreg50(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(50) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset1_protreg50(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(50)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset1_protreg51(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(51) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset1_protreg51(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(51)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset1_protreg52(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(52) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset1_protreg52(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(52)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset1_protreg53(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(53) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset1_protreg53(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(53)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset1_protreg54(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(54) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset1_protreg54(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(54)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset1_protreg55(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(55) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset1_protreg55(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(55)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset1_protreg56(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(56) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset1_protreg56(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(56)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset1_protreg57(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(57) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset1_protreg57(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(57)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset1_protreg58(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(58) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset1_protreg58(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(58)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset1_protreg59(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(59) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset1_protreg59(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(59)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset1_protreg60(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(60) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset1_protreg60(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(60)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset1_protreg61(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(61) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset1_protreg61(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(61)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset1_protreg62(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(62) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset1_protreg62(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(62)
        }
        Ok(())
    }
    #[inline]
    pub fn read_mpu_protenset1_protreg63(&mut self) -> MemResult<u8> {
        Ok(self.mpu_is_protected(63) as u8)
    }
    #[inline]
    pub fn write_mpu_protenset1_protreg63(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        if _value != 0 {
            self.mpu_enable_protection(63)
        }
        Ok(())
    }
    pub fn read_mpu_disableindebug_disableindebug(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 1u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_disableindebug_disableindebug(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_protblocksize_protblocksize(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_mpu_protblocksize_protblocksize(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn write_radio_tasks_txen(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_radio_shorts_ready_start(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_shorts_ready_start(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_shorts_end_disable(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_shorts_end_disable(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_shorts_disabled_txen(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_shorts_disabled_txen(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_shorts_disabled_rxen(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_shorts_disabled_rxen(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_shorts_address_rssistart(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_shorts_address_rssistart(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_shorts_end_start(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_shorts_end_start(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_shorts_address_bcstart(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_shorts_address_bcstart(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_shorts_disabled_rssistop(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_shorts_disabled_rssistop(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenset_ready(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenset_ready(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenset_address(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenset_address(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenset_payload(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenset_payload(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenset_end(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenset_end(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenset_disabled(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenset_disabled(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenset_devmatch(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenset_devmatch(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenset_devmiss(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenset_devmiss(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenset_rssiend(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenset_rssiend(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenset_bcmatch(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenset_bcmatch(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenclr_ready(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenclr_ready(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenclr_address(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenclr_address(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenclr_payload(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenclr_payload(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenclr_end(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenclr_end(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenclr_disabled(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenclr_disabled(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenclr_devmatch(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenclr_devmatch(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenclr_devmiss(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenclr_devmiss(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenclr_rssiend(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenclr_rssiend(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenclr_bcmatch(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenclr_bcmatch(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_crcstatus_crcstatus(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_radio_rxmatch_rxmatch(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn read_radio_rxcrc_rxcrc(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
    ) -> MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 16777215u64;
        match (_byte_0, _byte_1, _byte_2) {
            (None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_radio_dai_dai(&self) -> MemResult<u8> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_radio_frequency_frequency(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 2u64;
        const _RESET_MASK: u64 = 127u64;
        todo!()
    }
    pub fn write_radio_frequency_frequency(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_txpower_txpower(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_radio_txpower_txpower(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_mode_mode(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_radio_mode_mode(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_pcnf0_lflen(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn write_radio_pcnf0_lflen(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_pcnf0_s0len(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_pcnf0_s0len(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_pcnf0_s1len(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn write_radio_pcnf0_s1len(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_pcnf1_maxlen(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_radio_pcnf1_maxlen(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_pcnf1_statlen(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_radio_pcnf1_statlen(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_pcnf1_balen(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_radio_pcnf1_balen(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_pcnf1_endian(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_pcnf1_endian(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_pcnf1_whiteen(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_pcnf1_whiteen(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_base0(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    #[inline]
    pub fn read_radio_prefix0_ap0(&mut self) -> MemResult<u8> {
        Ok(self.radio_ap(0))
    }
    #[inline]
    pub fn write_radio_prefix0_ap0(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ap(0, _value))
    }
    #[inline]
    pub fn read_radio_prefix0_ap1(&mut self) -> MemResult<u8> {
        Ok(self.radio_ap(1))
    }
    #[inline]
    pub fn write_radio_prefix0_ap1(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ap(1, _value))
    }
    #[inline]
    pub fn read_radio_prefix0_ap2(&mut self) -> MemResult<u8> {
        Ok(self.radio_ap(2))
    }
    #[inline]
    pub fn write_radio_prefix0_ap2(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ap(2, _value))
    }
    #[inline]
    pub fn read_radio_prefix0_ap3(&mut self) -> MemResult<u8> {
        Ok(self.radio_ap(3))
    }
    #[inline]
    pub fn write_radio_prefix0_ap3(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ap(3, _value))
    }
    #[inline]
    pub fn read_radio_prefix1_ap4(&mut self) -> MemResult<u8> {
        Ok(self.radio_ap(4))
    }
    #[inline]
    pub fn write_radio_prefix1_ap4(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ap(4, _value))
    }
    #[inline]
    pub fn read_radio_prefix1_ap5(&mut self) -> MemResult<u8> {
        Ok(self.radio_ap(5))
    }
    #[inline]
    pub fn write_radio_prefix1_ap5(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ap(5, _value))
    }
    #[inline]
    pub fn read_radio_prefix1_ap6(&mut self) -> MemResult<u8> {
        Ok(self.radio_ap(6))
    }
    #[inline]
    pub fn write_radio_prefix1_ap6(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ap(6, _value))
    }
    #[inline]
    pub fn read_radio_prefix1_ap7(&mut self) -> MemResult<u8> {
        Ok(self.radio_ap(7))
    }
    #[inline]
    pub fn write_radio_prefix1_ap7(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ap(7, _value))
    }
    pub fn read_radio_txaddress_txaddress(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_radio_txaddress_txaddress(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    #[inline]
    pub fn read_radio_rxaddresses_addr0(&mut self) -> MemResult<u8> {
        Ok(self.radio_is_recv_on_ap(0) as u8)
    }
    #[inline]
    pub fn write_radio_rxaddresses_addr0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.radio_set_recv_on_ap(0, _value != 0))
    }
    #[inline]
    pub fn read_radio_rxaddresses_addr1(&mut self) -> MemResult<u8> {
        Ok(self.radio_is_recv_on_ap(1) as u8)
    }
    #[inline]
    pub fn write_radio_rxaddresses_addr1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.radio_set_recv_on_ap(1, _value != 0))
    }
    #[inline]
    pub fn read_radio_rxaddresses_addr2(&mut self) -> MemResult<u8> {
        Ok(self.radio_is_recv_on_ap(2) as u8)
    }
    #[inline]
    pub fn write_radio_rxaddresses_addr2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.radio_set_recv_on_ap(2, _value != 0))
    }
    #[inline]
    pub fn read_radio_rxaddresses_addr3(&mut self) -> MemResult<u8> {
        Ok(self.radio_is_recv_on_ap(3) as u8)
    }
    #[inline]
    pub fn write_radio_rxaddresses_addr3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.radio_set_recv_on_ap(3, _value != 0))
    }
    #[inline]
    pub fn read_radio_rxaddresses_addr4(&mut self) -> MemResult<u8> {
        Ok(self.radio_is_recv_on_ap(4) as u8)
    }
    #[inline]
    pub fn write_radio_rxaddresses_addr4(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.radio_set_recv_on_ap(4, _value != 0))
    }
    #[inline]
    pub fn read_radio_rxaddresses_addr5(&mut self) -> MemResult<u8> {
        Ok(self.radio_is_recv_on_ap(5) as u8)
    }
    #[inline]
    pub fn write_radio_rxaddresses_addr5(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.radio_set_recv_on_ap(5, _value != 0))
    }
    #[inline]
    pub fn read_radio_rxaddresses_addr6(&mut self) -> MemResult<u8> {
        Ok(self.radio_is_recv_on_ap(6) as u8)
    }
    #[inline]
    pub fn write_radio_rxaddresses_addr6(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.radio_set_recv_on_ap(6, _value != 0))
    }
    #[inline]
    pub fn read_radio_rxaddresses_addr7(&mut self) -> MemResult<u8> {
        Ok(self.radio_is_recv_on_ap(7) as u8)
    }
    #[inline]
    pub fn write_radio_rxaddresses_addr7(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.radio_set_recv_on_ap(7, _value != 0))
    }
    pub fn read_radio_crccnf_len(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_radio_crccnf_len(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_crccnf_skipaddr(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_crccnf_skipaddr(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_crcpoly_crcpoly(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2) {
            (None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_radio_test_constcarrier(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_test_constcarrier(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_test_plllock(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_test_plllock(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_tifs_tifs(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_radio_tifs_tifs(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_rssisample_rssisample(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 127u64;
        todo!()
    }
    pub fn read_radio_state_state(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_radio_datawhiteiv_datawhiteiv(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 64u64;
        const _RESET_MASK: u64 = 127u64;
        todo!()
    }
    pub fn write_radio_datawhiteiv_datawhiteiv(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_bcc(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1) {
            (None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    #[inline]
    pub fn read_radio_dacnf_ena0(&mut self) -> MemResult<u8> {
        Ok(self.radio_is_ena(0) as u8)
    }
    #[inline]
    pub fn write_radio_dacnf_ena0(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ena(0, _value != 0))
    }
    #[inline]
    pub fn read_radio_dacnf_ena1(&mut self) -> MemResult<u8> {
        Ok(self.radio_is_ena(1) as u8)
    }
    #[inline]
    pub fn write_radio_dacnf_ena1(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ena(1, _value != 0))
    }
    #[inline]
    pub fn read_radio_dacnf_ena2(&mut self) -> MemResult<u8> {
        Ok(self.radio_is_ena(2) as u8)
    }
    #[inline]
    pub fn write_radio_dacnf_ena2(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ena(2, _value != 0))
    }
    #[inline]
    pub fn read_radio_dacnf_ena3(&mut self) -> MemResult<u8> {
        Ok(self.radio_is_ena(3) as u8)
    }
    #[inline]
    pub fn write_radio_dacnf_ena3(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ena(3, _value != 0))
    }
    #[inline]
    pub fn read_radio_dacnf_ena4(&mut self) -> MemResult<u8> {
        Ok(self.radio_is_ena(4) as u8)
    }
    #[inline]
    pub fn write_radio_dacnf_ena4(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ena(4, _value != 0))
    }
    #[inline]
    pub fn read_radio_dacnf_ena5(&mut self) -> MemResult<u8> {
        Ok(self.radio_is_ena(5) as u8)
    }
    #[inline]
    pub fn write_radio_dacnf_ena5(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ena(5, _value != 0))
    }
    #[inline]
    pub fn read_radio_dacnf_ena6(&mut self) -> MemResult<u8> {
        Ok(self.radio_is_ena(6) as u8)
    }
    #[inline]
    pub fn write_radio_dacnf_ena6(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ena(6, _value != 0))
    }
    #[inline]
    pub fn read_radio_dacnf_ena7(&mut self) -> MemResult<u8> {
        Ok(self.radio_is_ena(7) as u8)
    }
    #[inline]
    pub fn write_radio_dacnf_ena7(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ena(7, _value != 0))
    }
    #[inline]
    pub fn read_radio_dacnf_txadd0(&mut self) -> MemResult<u8> {
        Ok(self.radio_is_txadd(0) as u8)
    }
    #[inline]
    pub fn write_radio_dacnf_txadd0(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ena(0, _value != 0))
    }
    #[inline]
    pub fn read_radio_dacnf_txadd1(&mut self) -> MemResult<u8> {
        Ok(self.radio_is_txadd(1) as u8)
    }
    #[inline]
    pub fn write_radio_dacnf_txadd1(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ena(1, _value != 0))
    }
    #[inline]
    pub fn read_radio_dacnf_txadd2(&mut self) -> MemResult<u8> {
        Ok(self.radio_is_txadd(2) as u8)
    }
    #[inline]
    pub fn write_radio_dacnf_txadd2(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ena(2, _value != 0))
    }
    #[inline]
    pub fn read_radio_dacnf_txadd3(&mut self) -> MemResult<u8> {
        Ok(self.radio_is_txadd(3) as u8)
    }
    #[inline]
    pub fn write_radio_dacnf_txadd3(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ena(3, _value != 0))
    }
    #[inline]
    pub fn read_radio_dacnf_txadd4(&mut self) -> MemResult<u8> {
        Ok(self.radio_is_txadd(4) as u8)
    }
    #[inline]
    pub fn write_radio_dacnf_txadd4(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ena(4, _value != 0))
    }
    #[inline]
    pub fn read_radio_dacnf_txadd5(&mut self) -> MemResult<u8> {
        Ok(self.radio_is_txadd(5) as u8)
    }
    #[inline]
    pub fn write_radio_dacnf_txadd5(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ena(5, _value != 0))
    }
    #[inline]
    pub fn read_radio_dacnf_txadd6(&mut self) -> MemResult<u8> {
        Ok(self.radio_is_txadd(6) as u8)
    }
    #[inline]
    pub fn write_radio_dacnf_txadd6(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ena(6, _value != 0))
    }
    #[inline]
    pub fn read_radio_dacnf_txadd7(&mut self) -> MemResult<u8> {
        Ok(self.radio_is_txadd(7) as u8)
    }
    #[inline]
    pub fn write_radio_dacnf_txadd7(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ena(7, _value != 0))
    }
    pub fn read_radio_override0_override0(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_radio_override4_enable(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_override4_enable(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_power_power(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_power_power(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn write_uart0_tasks_startrx(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_uart0_shorts_cts_startrx(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_shorts_cts_startrx(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_shorts_ncts_stoprx(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_shorts_ncts_stoprx(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenset_cts(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenset_cts(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenset_ncts(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenset_ncts(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenset_rxdrdy(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenset_rxdrdy(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenset_txdrdy(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenset_txdrdy(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenset_error(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenset_error(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenset_rxto(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenset_rxto(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenclr_cts(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenclr_cts(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenclr_ncts(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenclr_ncts(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenclr_rxdrdy(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenclr_rxdrdy(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenclr_txdrdy(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenclr_txdrdy(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenclr_error(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenclr_error(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenclr_rxto(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenclr_rxto(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_errorsrc_overrun(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_errorsrc_overrun(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_errorsrc_parity(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_errorsrc_parity(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_errorsrc_framing(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_errorsrc_framing(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_errorsrc_break(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_errorsrc_break(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_enable_enable(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_uart0_enable_enable(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_pselrts(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_uart0_rxd_rxd(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_uart0_txd_txd(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_baudrate_baudrate(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_uart0_config_hwfc(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_config_hwfc(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_config_parity(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_uart0_config_parity(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_power_power(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_power_power(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_spi0twi0_events_ready(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_spi0twi0_intenset_ready(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spi0twi0_intenset_ready(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_spi0twi0_intenset_stopped(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spi0twi0_intenset_stopped(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_spi0twi0_intenset_txdsent(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spi0twi0_intenset_txdsent(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_spi0twi0_intenset_error(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spi0twi0_intenset_error(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_spi0twi0_intenset_bb(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spi0twi0_intenset_bb(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_spi0twi0_intenset_suspended(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spi0twi0_intenset_suspended(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_spi0twi0_intenclr_ready(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spi0twi0_intenclr_ready(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_spi0twi0_intenclr_stopped(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spi0twi0_intenclr_stopped(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_spi0twi0_intenclr_txdsent(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spi0twi0_intenclr_txdsent(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_spi0twi0_intenclr_error(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spi0twi0_intenclr_error(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_spi0twi0_intenclr_bb(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spi0twi0_intenclr_bb(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_spi0twi0_intenclr_suspended(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spi0twi0_intenclr_suspended(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_spi0twi0_enable_enable(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_spi0twi0_enable_enable(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_spi0twi0_pselsck(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_spi0twi0_rxd_rxd(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn read_spi0twi0_txd_txd(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_spi0twi0_txd_txd(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_spi0twi0_frequency_frequency(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_spi0_config_order(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spi0_config_order(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_spi0_config_cpha(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spi0_config_cpha(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_spi0_config_cpol(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spi0_config_cpol(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_spi0twi0_power_power(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spi0twi0_power_power(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn write_twi0_tasks_startrx(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_twi0_shorts_bb_suspend(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_twi0_shorts_bb_suspend(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_twi0_shorts_bb_stop(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_twi0_shorts_bb_stop(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_twi0_errorsrc_overrun(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_twi0_errorsrc_overrun(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_twi0_errorsrc_anack(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_twi0_errorsrc_anack(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_twi0_errorsrc_dnack(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_twi0_errorsrc_dnack(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_twi0_address_address(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 127u64;
        todo!()
    }
    pub fn write_twi0_address_address(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn write_spis1_tasks_acquire(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_spis1_shorts_end_acquire(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spis1_shorts_end_acquire(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_spis1_intenset_end(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spis1_intenset_end(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_spis1_intenset_endrx(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spis1_intenset_endrx(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_spis1_intenset_acquired(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spis1_intenset_acquired(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_spis1_intenclr_end(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spis1_intenclr_end(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_spis1_intenclr_endrx(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spis1_intenclr_endrx(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_spis1_intenclr_acquired(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spis1_intenclr_acquired(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_spis1_semstat_semstat(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 1u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn read_spis1_status_overread(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spis1_status_overread(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_spis1_status_overflow(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spis1_status_overflow(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_spis1_enable_enable(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_spis1_enable_enable(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_spis1_pselsck(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_spis1_maxrx_maxrx(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_spis1_maxrx_maxrx(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_spis1_amountrx_amountrx(&mut self) -> MemResult<u8> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_spis1_maxtx_maxtx(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_spis1_maxtx_maxtx(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_spis1_amounttx_amounttx(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn read_spis1_config_order(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spis1_config_order(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_spis1_config_cpha(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spis1_config_cpha(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_spis1_config_cpol(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spis1_config_cpol(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_spis1_def_def(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_spis1_def_def(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_spis1_orc_orc(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_spis1_orc_orc(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_spis1_power_power(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spis1_power_power(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn write_gpiote_tasks_outn(
        &mut self,
        _dim: usize,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_gpiote_intenset_in0(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpiote_intenset_in0(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_intenset_in1(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpiote_intenset_in1(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_intenset_in2(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpiote_intenset_in2(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_intenset_in3(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpiote_intenset_in3(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_intenset_port(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpiote_intenset_port(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_intenclr_in0(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpiote_intenclr_in0(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_intenclr_in1(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpiote_intenclr_in1(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_intenclr_in2(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpiote_intenclr_in2(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_intenclr_in3(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpiote_intenclr_in3(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_intenclr_port(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpiote_intenclr_port(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_confign_mode(&mut self, _dim: usize) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_gpiote_confign_mode(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_confign_psel(&mut self, _dim: usize) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 31u64;
        todo!()
    }
    pub fn write_gpiote_confign_psel(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_confign_polarity(
        &mut self,
        _dim: usize,
    ) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_gpiote_confign_polarity(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_confign_outinit(
        &mut self,
        _dim: usize,
    ) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpiote_confign_outinit(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_power_power(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpiote_power_power(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn write_adc_tasks_start(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_adc_intenset_end(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_adc_intenset_end(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_adc_intenclr_end(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_adc_intenclr_end(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_adc_busy_busy(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_adc_enable_enable(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_adc_enable_enable(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_adc_config_res(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_adc_config_res(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_adc_config_inpsel(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 6u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_adc_config_inpsel(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_adc_config_refsel(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_adc_config_refsel(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_adc_config_psel(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_adc_config_psel(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_adc_config_extrefsel(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_adc_config_extrefsel(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_adc_result_result(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
    ) -> MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1023u64;
        match (_byte_0, _byte_1) {
            (None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_adc_power_power(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_adc_power_power(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn write_timer0_tasks_start(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_timer0_shorts_compare0_clear(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_shorts_compare0_clear(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_timer0_shorts_compare1_clear(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_shorts_compare1_clear(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_timer0_shorts_compare2_clear(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_shorts_compare2_clear(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_timer0_shorts_compare3_clear(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_shorts_compare3_clear(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_timer0_shorts_compare0_stop(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_shorts_compare0_stop(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_timer0_shorts_compare1_stop(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_shorts_compare1_stop(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_timer0_shorts_compare2_stop(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_shorts_compare2_stop(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_timer0_shorts_compare3_stop(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_shorts_compare3_stop(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_timer0_intenset_compare0(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_intenset_compare0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_timer0_intenset_compare1(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_intenset_compare1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_timer0_intenset_compare2(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_intenset_compare2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_timer0_intenset_compare3(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_intenset_compare3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_timer0_intenclr_compare0(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_intenclr_compare0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_timer0_intenclr_compare1(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_intenclr_compare1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_timer0_intenclr_compare2(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_intenclr_compare2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_timer0_intenclr_compare3(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_intenclr_compare3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_timer0_mode_mode(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_mode_mode(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_timer0_bitmode_bitmode(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_timer0_bitmode_bitmode(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_timer0_prescaler_prescaler(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 4u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn write_timer0_prescaler_prescaler(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_timer0_ccn(
        &mut self,
        _dim: usize,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_timer0_power_power(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_timer0_power_power(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn write_rtc0_tasks_start(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_rtc0_intenset_tick(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_intenset_tick(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_intenset_ovrflw(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_intenset_ovrflw(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_intenset_compare0(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_intenset_compare0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_intenset_compare1(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_intenset_compare1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_intenset_compare2(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_intenset_compare2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_intenset_compare3(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_intenset_compare3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_intenclr_tick(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_intenclr_tick(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_intenclr_ovrflw(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_intenclr_ovrflw(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_intenclr_compare0(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_intenclr_compare0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_intenclr_compare1(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_intenclr_compare1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_intenclr_compare2(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_intenclr_compare2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_intenclr_compare3(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_intenclr_compare3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evten_tick(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evten_tick(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evten_ovrflw(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evten_ovrflw(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evten_compare0(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evten_compare0(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evten_compare1(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evten_compare1(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evten_compare2(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evten_compare2(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evten_compare3(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evten_compare3(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evtenset_tick(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evtenset_tick(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evtenset_ovrflw(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evtenset_ovrflw(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evtenset_compare0(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evtenset_compare0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evtenset_compare1(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evtenset_compare1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evtenset_compare2(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evtenset_compare2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evtenset_compare3(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evtenset_compare3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evtenclr_tick(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evtenclr_tick(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evtenclr_ovrflw(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evtenclr_ovrflw(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evtenclr_compare0(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evtenclr_compare0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evtenclr_compare1(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evtenclr_compare1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evtenclr_compare2(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evtenclr_compare2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evtenclr_compare3(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evtenclr_compare3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_counter_counter(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2) {
            (None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_rtc0_power_power(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_power_power(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn write_temp_tasks_start(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_temp_intenset_datardy(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_temp_intenset_datardy(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_temp_intenclr_datardy(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_temp_intenclr_datardy(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_temp_temp(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_temp_power_power(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_temp_power_power(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn write_rng_tasks_start(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_rng_shorts_valrdy_stop(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rng_shorts_valrdy_stop(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_rng_intenset_valrdy(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rng_intenset_valrdy(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_rng_intenclr_valrdy(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rng_intenclr_valrdy(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_rng_config_dercen(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rng_config_dercen(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_rng_value_value(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn read_rng_power_power(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rng_power_power(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn write_ecb_tasks_startecb(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_ecb_intenset_endecb(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ecb_intenset_endecb(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_ecb_intenset_errorecb(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ecb_intenset_errorecb(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_ecb_intenclr_endecb(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ecb_intenclr_endecb(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_ecb_intenclr_errorecb(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ecb_intenclr_errorecb(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_ecb_ecbdataptr(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_ecb_power_power(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ecb_power_power(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn write_aarccm_tasks_start(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_aarccm_intenset_end(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_aarccm_intenset_end(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_aarccm_intenset_resolved(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_aarccm_intenset_resolved(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_aarccm_intenset_notresolved(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_aarccm_intenset_notresolved(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_aarccm_intenclr_end(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_aarccm_intenclr_end(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_aarccm_intenclr_resolved(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_aarccm_intenclr_resolved(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_aarccm_intenclr_notresolved(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_aarccm_intenclr_notresolved(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_aarccm_status_status(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_aarccm_enable_enable(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_aarccm_enable_enable(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_aarccm_nirk_nirk(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 1u64;
        const _RESET_MASK: u64 = 31u64;
        todo!()
    }
    pub fn write_aarccm_nirk_nirk(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_aarccm_irkptr(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_aarccm_power_power(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_aarccm_power_power(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn write_ccm_tasks_crypt(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_ccm_shorts_endksgen_crypt(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ccm_shorts_endksgen_crypt(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_ccm_inptr(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_wdt_intenset_timeout(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_wdt_intenset_timeout(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_wdt_intenclr_timeout(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_wdt_intenclr_timeout(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_wdt_runstatus_runstatus(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_wdt_reqstatus_rr0(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 1u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_wdt_reqstatus_rr1(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_wdt_reqstatus_rr2(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_wdt_reqstatus_rr3(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_wdt_reqstatus_rr4(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_wdt_reqstatus_rr5(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_wdt_reqstatus_rr6(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_wdt_reqstatus_rr7(&self) -> MemResult<u8> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_wdt_rren_rr0(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 1u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_wdt_rren_rr0(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_wdt_rren_rr1(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_wdt_rren_rr1(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_wdt_rren_rr2(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_wdt_rren_rr2(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_wdt_rren_rr3(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_wdt_rren_rr3(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_wdt_rren_rr4(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_wdt_rren_rr4(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_wdt_rren_rr5(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_wdt_rren_rr5(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_wdt_rren_rr6(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_wdt_rren_rr6(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_wdt_rren_rr7(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_wdt_rren_rr7(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_wdt_config_sleep(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 1u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_wdt_config_sleep(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_wdt_config_halt(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_wdt_config_halt(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn write_wdt_rrn_rr(
        &mut self,
        _dim: usize,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_wdt_power_power(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_wdt_power_power(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn write_qdec_tasks_start(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_qdec_shorts_reportrdy_readclracc(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_qdec_shorts_reportrdy_readclracc(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_qdec_shorts_samplerdy_stop(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_qdec_shorts_samplerdy_stop(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_qdec_intenset_samplerdy(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_qdec_intenset_samplerdy(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_qdec_intenset_reportrdy(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_qdec_intenset_reportrdy(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_qdec_intenset_accof(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_qdec_intenset_accof(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_qdec_intenclr_samplerdy(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_qdec_intenclr_samplerdy(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_qdec_intenclr_reportrdy(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_qdec_intenclr_reportrdy(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_qdec_intenclr_accof(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_qdec_intenclr_accof(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_qdec_enable_enable(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_qdec_enable_enable(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_qdec_ledpol_ledpol(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_qdec_ledpol_ledpol(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_qdec_sampleper_sampleper(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_qdec_sampleper_sampleper(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_qdec_sample_sample(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 4294967295u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_qdec_reportper_reportper(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_qdec_reportper_reportper(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_qdec_acc(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_qdec_dbfen_dbfen(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_qdec_dbfen_dbfen(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_qdec_ledpre_ledpre(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1) {
            (None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_qdec_accdbl_accdbl(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_qdec_accdblread_accdblread(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_qdec_power_power(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_qdec_power_power(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn write_lpcomp_tasks_start(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_lpcomp_shorts_ready_sample(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_shorts_ready_sample(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_shorts_ready_stop(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_shorts_ready_stop(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_shorts_down_stop(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_shorts_down_stop(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_shorts_up_stop(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_shorts_up_stop(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_shorts_cross_stop(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_shorts_cross_stop(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_intenset_ready(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_intenset_ready(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_intenset_down(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_intenset_down(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_intenset_up(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_intenset_up(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_intenset_cross(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_intenset_cross(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_intenclr_ready(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_intenclr_ready(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_intenclr_down(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_intenclr_down(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_intenclr_up(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_intenclr_up(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_intenclr_cross(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_intenclr_cross(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_result_result(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_lpcomp_enable_enable(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_lpcomp_enable_enable(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_psel_psel(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_lpcomp_psel_psel(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_refsel_refsel(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_lpcomp_refsel_refsel(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_extrefsel_extrefsel(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_extrefsel_extrefsel(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_anadetect_anadetect(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_lpcomp_anadetect_anadetect(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_power_power(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_power_power(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_swi_unused(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_nvmc_ready_ready(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_nvmc_config_wen(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_nvmc_config_wen(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_nvmc_erasepage(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_nvmc_eraseall_eraseall(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_nvmc_eraseall_eraseall(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_nvmc_erasepcr0(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_nvmc_eraseuicr_eraseuicr(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_nvmc_eraseuicr_eraseuicr(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    #[inline]
    pub fn read_ppi_chen_ch0(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(0) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch0(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(0, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch1(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(1) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch1(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(1, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch2(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(2) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch2(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(2, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch3(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(3) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch3(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(3, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch4(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(4) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch4(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(4, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch5(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(5) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch5(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(5, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch6(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(6) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch6(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(6, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch7(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(7) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch7(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(7, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch8(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(8) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch8(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(8, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch9(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(9) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch9(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(9, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch10(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(10) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch10(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(10, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch11(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(11) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch11(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(11, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch12(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(12) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch12(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(12, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch13(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(13) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch13(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(13, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch14(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(14) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch14(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(14, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch15(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(15) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch15(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(15, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch20(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(20) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch20(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(20, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch21(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(21) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch21(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(21, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch22(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(22) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch22(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(22, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch23(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(23) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch23(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(23, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch24(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(24) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch24(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(24, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch25(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(25) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch25(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(25, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch26(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(26) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch26(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(26, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch27(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(27) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch27(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(27, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch28(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(28) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch28(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(28, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch29(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(29) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch29(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(29, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch30(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(30) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch30(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(30, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch31(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(31) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch31(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(31, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chenset_ch0(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(0) as u8)
    }
    #[inline]
    pub fn write_ppi_chenset_ch0(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(0, true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenset_ch1(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(1) as u8)
    }
    #[inline]
    pub fn write_ppi_chenset_ch1(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(1, true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenset_ch2(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(2) as u8)
    }
    #[inline]
    pub fn write_ppi_chenset_ch2(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(2, true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenset_ch3(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(3) as u8)
    }
    #[inline]
    pub fn write_ppi_chenset_ch3(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(3, true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenset_ch4(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(4) as u8)
    }
    #[inline]
    pub fn write_ppi_chenset_ch4(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(4, true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenset_ch5(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(5) as u8)
    }
    #[inline]
    pub fn write_ppi_chenset_ch5(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(5, true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenset_ch6(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(6) as u8)
    }
    #[inline]
    pub fn write_ppi_chenset_ch6(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(6, true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenset_ch7(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(7) as u8)
    }
    #[inline]
    pub fn write_ppi_chenset_ch7(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(7, true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenset_ch8(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(8) as u8)
    }
    #[inline]
    pub fn write_ppi_chenset_ch8(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(8, true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenset_ch9(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(9) as u8)
    }
    #[inline]
    pub fn write_ppi_chenset_ch9(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(9, true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenset_ch10(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(10) as u8)
    }
    #[inline]
    pub fn write_ppi_chenset_ch10(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(10, true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenset_ch11(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(11) as u8)
    }
    #[inline]
    pub fn write_ppi_chenset_ch11(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(11, true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenset_ch12(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(12) as u8)
    }
    #[inline]
    pub fn write_ppi_chenset_ch12(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(12, true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenset_ch13(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(13) as u8)
    }
    #[inline]
    pub fn write_ppi_chenset_ch13(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(13, true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenset_ch14(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(14) as u8)
    }
    #[inline]
    pub fn write_ppi_chenset_ch14(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(14, true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenset_ch15(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(15) as u8)
    }
    #[inline]
    pub fn write_ppi_chenset_ch15(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(15, true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenset_ch20(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(20) as u8)
    }
    #[inline]
    pub fn write_ppi_chenset_ch20(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(20, true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenset_ch21(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(21) as u8)
    }
    #[inline]
    pub fn write_ppi_chenset_ch21(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(21, true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenset_ch22(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(22) as u8)
    }
    #[inline]
    pub fn write_ppi_chenset_ch22(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(22, true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenset_ch23(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(23) as u8)
    }
    #[inline]
    pub fn write_ppi_chenset_ch23(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(23, true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenset_ch24(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(24) as u8)
    }
    #[inline]
    pub fn write_ppi_chenset_ch24(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(24, true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenset_ch25(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(25) as u8)
    }
    #[inline]
    pub fn write_ppi_chenset_ch25(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(25, true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenset_ch26(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(26) as u8)
    }
    #[inline]
    pub fn write_ppi_chenset_ch26(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(26, true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenset_ch27(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(27) as u8)
    }
    #[inline]
    pub fn write_ppi_chenset_ch27(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(27, true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenset_ch28(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(28) as u8)
    }
    #[inline]
    pub fn write_ppi_chenset_ch28(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(28, true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenset_ch29(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(29) as u8)
    }
    #[inline]
    pub fn write_ppi_chenset_ch29(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(29, true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenset_ch30(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(30) as u8)
    }
    #[inline]
    pub fn write_ppi_chenset_ch30(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(30, true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenset_ch31(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(31) as u8)
    }
    #[inline]
    pub fn write_ppi_chenset_ch31(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(31, true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenclr_ch0(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(0) as u8)
    }
    #[inline]
    pub fn write_ppi_chenclr_ch0(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(0, false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenclr_ch1(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(1) as u8)
    }
    #[inline]
    pub fn write_ppi_chenclr_ch1(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(1, false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenclr_ch2(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(2) as u8)
    }
    #[inline]
    pub fn write_ppi_chenclr_ch2(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(2, false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenclr_ch3(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(3) as u8)
    }
    #[inline]
    pub fn write_ppi_chenclr_ch3(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(3, false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenclr_ch4(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(4) as u8)
    }
    #[inline]
    pub fn write_ppi_chenclr_ch4(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(4, false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenclr_ch5(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(5) as u8)
    }
    #[inline]
    pub fn write_ppi_chenclr_ch5(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(5, false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenclr_ch6(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(6) as u8)
    }
    #[inline]
    pub fn write_ppi_chenclr_ch6(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(6, false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenclr_ch7(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(7) as u8)
    }
    #[inline]
    pub fn write_ppi_chenclr_ch7(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(7, false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenclr_ch8(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(8) as u8)
    }
    #[inline]
    pub fn write_ppi_chenclr_ch8(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(8, false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenclr_ch9(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(9) as u8)
    }
    #[inline]
    pub fn write_ppi_chenclr_ch9(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(9, false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenclr_ch10(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(10) as u8)
    }
    #[inline]
    pub fn write_ppi_chenclr_ch10(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(10, false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenclr_ch11(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(11) as u8)
    }
    #[inline]
    pub fn write_ppi_chenclr_ch11(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(11, false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenclr_ch12(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(12) as u8)
    }
    #[inline]
    pub fn write_ppi_chenclr_ch12(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(12, false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenclr_ch13(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(13) as u8)
    }
    #[inline]
    pub fn write_ppi_chenclr_ch13(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(13, false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenclr_ch14(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(14) as u8)
    }
    #[inline]
    pub fn write_ppi_chenclr_ch14(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(14, false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenclr_ch15(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(15) as u8)
    }
    #[inline]
    pub fn write_ppi_chenclr_ch15(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(15, false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenclr_ch20(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(20) as u8)
    }
    #[inline]
    pub fn write_ppi_chenclr_ch20(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(20, false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenclr_ch21(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(21) as u8)
    }
    #[inline]
    pub fn write_ppi_chenclr_ch21(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(21, false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenclr_ch22(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(22) as u8)
    }
    #[inline]
    pub fn write_ppi_chenclr_ch22(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(22, false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenclr_ch23(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(23) as u8)
    }
    #[inline]
    pub fn write_ppi_chenclr_ch23(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(23, false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenclr_ch24(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(24) as u8)
    }
    #[inline]
    pub fn write_ppi_chenclr_ch24(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(24, false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenclr_ch25(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(25) as u8)
    }
    #[inline]
    pub fn write_ppi_chenclr_ch25(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(25, false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenclr_ch26(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(26) as u8)
    }
    #[inline]
    pub fn write_ppi_chenclr_ch26(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(26, false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenclr_ch27(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(27) as u8)
    }
    #[inline]
    pub fn write_ppi_chenclr_ch27(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(27, false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenclr_ch28(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(28) as u8)
    }
    #[inline]
    pub fn write_ppi_chenclr_ch28(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(28, false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenclr_ch29(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(29) as u8)
    }
    #[inline]
    pub fn write_ppi_chenclr_ch29(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(29, false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenclr_ch30(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(30) as u8)
    }
    #[inline]
    pub fn write_ppi_chenclr_ch30(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(30, false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chenclr_ch31(&mut self) -> MemResult<u8> {
        Ok(self.ppi_is_on(31) as u8)
    }
    #[inline]
    pub fn write_ppi_chenclr_ch31(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.ppi_set_on(31, false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_ppi_chgn_ch0(&mut self, _dim: usize) -> MemResult<u8> {
        Ok(self.ppi_is_included(_dim, 0) as u8)
    }
    #[inline]
    pub fn write_ppi_chgn_ch0(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.ppi_set_included(_dim, 0, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chgn_ch1(&mut self, _dim: usize) -> MemResult<u8> {
        Ok(self.ppi_is_included(_dim, 1) as u8)
    }
    #[inline]
    pub fn write_ppi_chgn_ch1(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.ppi_set_included(_dim, 1, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chgn_ch2(&mut self, _dim: usize) -> MemResult<u8> {
        Ok(self.ppi_is_included(_dim, 2) as u8)
    }
    #[inline]
    pub fn write_ppi_chgn_ch2(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.ppi_set_included(_dim, 2, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chgn_ch3(&mut self, _dim: usize) -> MemResult<u8> {
        Ok(self.ppi_is_included(_dim, 3) as u8)
    }
    #[inline]
    pub fn write_ppi_chgn_ch3(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.ppi_set_included(_dim, 3, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chgn_ch4(&mut self, _dim: usize) -> MemResult<u8> {
        Ok(self.ppi_is_included(_dim, 4) as u8)
    }
    #[inline]
    pub fn write_ppi_chgn_ch4(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.ppi_set_included(_dim, 4, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chgn_ch5(&mut self, _dim: usize) -> MemResult<u8> {
        Ok(self.ppi_is_included(_dim, 5) as u8)
    }
    #[inline]
    pub fn write_ppi_chgn_ch5(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.ppi_set_included(_dim, 5, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chgn_ch6(&mut self, _dim: usize) -> MemResult<u8> {
        Ok(self.ppi_is_included(_dim, 6) as u8)
    }
    #[inline]
    pub fn write_ppi_chgn_ch6(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.ppi_set_included(_dim, 6, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chgn_ch7(&mut self, _dim: usize) -> MemResult<u8> {
        Ok(self.ppi_is_included(_dim, 7) as u8)
    }
    #[inline]
    pub fn write_ppi_chgn_ch7(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.ppi_set_included(_dim, 7, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chgn_ch8(&mut self, _dim: usize) -> MemResult<u8> {
        Ok(self.ppi_is_included(_dim, 8) as u8)
    }
    #[inline]
    pub fn write_ppi_chgn_ch8(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.ppi_set_included(_dim, 8, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chgn_ch9(&mut self, _dim: usize) -> MemResult<u8> {
        Ok(self.ppi_is_included(_dim, 9) as u8)
    }
    #[inline]
    pub fn write_ppi_chgn_ch9(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.ppi_set_included(_dim, 9, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chgn_ch10(&mut self, _dim: usize) -> MemResult<u8> {
        Ok(self.ppi_is_included(_dim, 10) as u8)
    }
    #[inline]
    pub fn write_ppi_chgn_ch10(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.ppi_set_included(_dim, 10, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chgn_ch11(&mut self, _dim: usize) -> MemResult<u8> {
        Ok(self.ppi_is_included(_dim, 11) as u8)
    }
    #[inline]
    pub fn write_ppi_chgn_ch11(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.ppi_set_included(_dim, 11, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chgn_ch12(&mut self, _dim: usize) -> MemResult<u8> {
        Ok(self.ppi_is_included(_dim, 12) as u8)
    }
    #[inline]
    pub fn write_ppi_chgn_ch12(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.ppi_set_included(_dim, 12, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chgn_ch13(&mut self, _dim: usize) -> MemResult<u8> {
        Ok(self.ppi_is_included(_dim, 13) as u8)
    }
    #[inline]
    pub fn write_ppi_chgn_ch13(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.ppi_set_included(_dim, 13, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chgn_ch14(&mut self, _dim: usize) -> MemResult<u8> {
        Ok(self.ppi_is_included(_dim, 14) as u8)
    }
    #[inline]
    pub fn write_ppi_chgn_ch14(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.ppi_set_included(_dim, 14, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chgn_ch15(&mut self, _dim: usize) -> MemResult<u8> {
        Ok(self.ppi_is_included(_dim, 15) as u8)
    }
    #[inline]
    pub fn write_ppi_chgn_ch15(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.ppi_set_included(_dim, 15, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chgn_ch20(&mut self, _dim: usize) -> MemResult<u8> {
        Ok(self.ppi_is_included(_dim, 20) as u8)
    }
    #[inline]
    pub fn write_ppi_chgn_ch20(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.ppi_set_included(_dim, 20, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chgn_ch21(&mut self, _dim: usize) -> MemResult<u8> {
        Ok(self.ppi_is_included(_dim, 21) as u8)
    }
    #[inline]
    pub fn write_ppi_chgn_ch21(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.ppi_set_included(_dim, 21, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chgn_ch22(&mut self, _dim: usize) -> MemResult<u8> {
        Ok(self.ppi_is_included(_dim, 22) as u8)
    }
    #[inline]
    pub fn write_ppi_chgn_ch22(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.ppi_set_included(_dim, 22, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chgn_ch23(&mut self, _dim: usize) -> MemResult<u8> {
        Ok(self.ppi_is_included(_dim, 23) as u8)
    }
    #[inline]
    pub fn write_ppi_chgn_ch23(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.ppi_set_included(_dim, 23, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chgn_ch24(&mut self, _dim: usize) -> MemResult<u8> {
        Ok(self.ppi_is_included(_dim, 24) as u8)
    }
    #[inline]
    pub fn write_ppi_chgn_ch24(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.ppi_set_included(_dim, 24, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chgn_ch25(&mut self, _dim: usize) -> MemResult<u8> {
        Ok(self.ppi_is_included(_dim, 25) as u8)
    }
    #[inline]
    pub fn write_ppi_chgn_ch25(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.ppi_set_included(_dim, 25, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chgn_ch26(&mut self, _dim: usize) -> MemResult<u8> {
        Ok(self.ppi_is_included(_dim, 26) as u8)
    }
    #[inline]
    pub fn write_ppi_chgn_ch26(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.ppi_set_included(_dim, 26, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chgn_ch27(&mut self, _dim: usize) -> MemResult<u8> {
        Ok(self.ppi_is_included(_dim, 27) as u8)
    }
    #[inline]
    pub fn write_ppi_chgn_ch27(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.ppi_set_included(_dim, 27, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chgn_ch28(&mut self, _dim: usize) -> MemResult<u8> {
        Ok(self.ppi_is_included(_dim, 28) as u8)
    }
    #[inline]
    pub fn write_ppi_chgn_ch28(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.ppi_set_included(_dim, 28, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chgn_ch29(&mut self, _dim: usize) -> MemResult<u8> {
        Ok(self.ppi_is_included(_dim, 29) as u8)
    }
    #[inline]
    pub fn write_ppi_chgn_ch29(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.ppi_set_included(_dim, 29, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chgn_ch30(&mut self, _dim: usize) -> MemResult<u8> {
        Ok(self.ppi_is_included(_dim, 30) as u8)
    }
    #[inline]
    pub fn write_ppi_chgn_ch30(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.ppi_set_included(_dim, 30, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chgn_ch31(&mut self, _dim: usize) -> MemResult<u8> {
        Ok(self.ppi_is_included(_dim, 31) as u8)
    }
    #[inline]
    pub fn write_ppi_chgn_ch31(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.ppi_set_included(_dim, 31, _value != 0))
    }
    pub fn read_ficr_codepagesize(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_ficr_ppfc_ppfc(&self) -> MemResult<u8> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_ficr_deviceaddrtype_deviceaddrtype(&mut self) -> MemResult<u8> {
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
    ) -> MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_ficr_overrideen_nrf_1mbit(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_ficr_overrideen_ble_1mbit(&mut self) -> MemResult<u8> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn read_uicr_rbpconf_pr0(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_uicr_rbpconf_pr0(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uicr_rbpconf_pall(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_uicr_rbpconf_pall(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uicr_xtalfreq_xtalfreq(&mut self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_uicr_xtalfreq_xtalfreq(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_uicr_fwid_fwid(
        &mut self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
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
    ) -> MemResult<()> {
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    #[inline]
    pub fn read_gpio_out_pin0(&mut self) -> MemResult<u8> {
        Ok(self.gpio[0].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin0(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[0].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin1(&mut self) -> MemResult<u8> {
        Ok(self.gpio[1].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin1(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[1].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin2(&mut self) -> MemResult<u8> {
        Ok(self.gpio[2].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin2(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[2].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin3(&mut self) -> MemResult<u8> {
        Ok(self.gpio[3].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin3(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[3].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin4(&mut self) -> MemResult<u8> {
        Ok(self.gpio[4].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin4(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[4].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin5(&mut self) -> MemResult<u8> {
        Ok(self.gpio[5].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin5(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[5].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin6(&mut self) -> MemResult<u8> {
        Ok(self.gpio[6].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin6(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[6].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin7(&mut self) -> MemResult<u8> {
        Ok(self.gpio[7].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin7(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[7].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin8(&mut self) -> MemResult<u8> {
        Ok(self.gpio[8].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin8(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[8].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin9(&mut self) -> MemResult<u8> {
        Ok(self.gpio[9].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin9(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[9].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin10(&mut self) -> MemResult<u8> {
        Ok(self.gpio[10].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin10(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[10].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin11(&mut self) -> MemResult<u8> {
        Ok(self.gpio[11].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin11(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[11].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin12(&mut self) -> MemResult<u8> {
        Ok(self.gpio[12].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin12(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[12].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin13(&mut self) -> MemResult<u8> {
        Ok(self.gpio[13].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin13(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[13].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin14(&mut self) -> MemResult<u8> {
        Ok(self.gpio[14].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin14(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[14].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin15(&mut self) -> MemResult<u8> {
        Ok(self.gpio[15].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin15(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[15].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin16(&mut self) -> MemResult<u8> {
        Ok(self.gpio[16].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin16(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[16].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin17(&mut self) -> MemResult<u8> {
        Ok(self.gpio[17].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin17(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[17].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin18(&mut self) -> MemResult<u8> {
        Ok(self.gpio[18].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin18(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[18].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin19(&mut self) -> MemResult<u8> {
        Ok(self.gpio[19].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin19(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[19].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin20(&mut self) -> MemResult<u8> {
        Ok(self.gpio[20].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin20(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[20].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin21(&mut self) -> MemResult<u8> {
        Ok(self.gpio[21].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin21(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[21].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin22(&mut self) -> MemResult<u8> {
        Ok(self.gpio[22].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin22(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[22].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin23(&mut self) -> MemResult<u8> {
        Ok(self.gpio[23].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin23(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[23].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin24(&mut self) -> MemResult<u8> {
        Ok(self.gpio[24].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin24(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[24].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin25(&mut self) -> MemResult<u8> {
        Ok(self.gpio[25].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin25(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[25].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin26(&mut self) -> MemResult<u8> {
        Ok(self.gpio[26].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin26(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[26].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin27(&mut self) -> MemResult<u8> {
        Ok(self.gpio[27].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin27(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[27].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin28(&mut self) -> MemResult<u8> {
        Ok(self.gpio[28].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin28(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[28].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin29(&mut self) -> MemResult<u8> {
        Ok(self.gpio[29].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin29(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[29].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin30(&mut self) -> MemResult<u8> {
        Ok(self.gpio[30].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin30(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[30].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin31(&mut self) -> MemResult<u8> {
        Ok(self.gpio[31].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin31(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[31].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_outset_pin0(&mut self) -> MemResult<u8> {
        Ok(self.gpio[0].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outset_pin0(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[0].set_out_high(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outset_pin1(&mut self) -> MemResult<u8> {
        Ok(self.gpio[1].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outset_pin1(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[1].set_out_high(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outset_pin2(&mut self) -> MemResult<u8> {
        Ok(self.gpio[2].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outset_pin2(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[2].set_out_high(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outset_pin3(&mut self) -> MemResult<u8> {
        Ok(self.gpio[3].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outset_pin3(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[3].set_out_high(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outset_pin4(&mut self) -> MemResult<u8> {
        Ok(self.gpio[4].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outset_pin4(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[4].set_out_high(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outset_pin5(&mut self) -> MemResult<u8> {
        Ok(self.gpio[5].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outset_pin5(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[5].set_out_high(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outset_pin6(&mut self) -> MemResult<u8> {
        Ok(self.gpio[6].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outset_pin6(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[6].set_out_high(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outset_pin7(&mut self) -> MemResult<u8> {
        Ok(self.gpio[7].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outset_pin7(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[7].set_out_high(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outset_pin8(&mut self) -> MemResult<u8> {
        Ok(self.gpio[8].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outset_pin8(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[8].set_out_high(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outset_pin9(&mut self) -> MemResult<u8> {
        Ok(self.gpio[9].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outset_pin9(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[9].set_out_high(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outset_pin10(&mut self) -> MemResult<u8> {
        Ok(self.gpio[10].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outset_pin10(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[10].set_out_high(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outset_pin11(&mut self) -> MemResult<u8> {
        Ok(self.gpio[11].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outset_pin11(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[11].set_out_high(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outset_pin12(&mut self) -> MemResult<u8> {
        Ok(self.gpio[12].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outset_pin12(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[12].set_out_high(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outset_pin13(&mut self) -> MemResult<u8> {
        Ok(self.gpio[13].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outset_pin13(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[13].set_out_high(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outset_pin14(&mut self) -> MemResult<u8> {
        Ok(self.gpio[14].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outset_pin14(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[14].set_out_high(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outset_pin15(&mut self) -> MemResult<u8> {
        Ok(self.gpio[15].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outset_pin15(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[15].set_out_high(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outset_pin16(&mut self) -> MemResult<u8> {
        Ok(self.gpio[16].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outset_pin16(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[16].set_out_high(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outset_pin17(&mut self) -> MemResult<u8> {
        Ok(self.gpio[17].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outset_pin17(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[17].set_out_high(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outset_pin18(&mut self) -> MemResult<u8> {
        Ok(self.gpio[18].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outset_pin18(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[18].set_out_high(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outset_pin19(&mut self) -> MemResult<u8> {
        Ok(self.gpio[19].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outset_pin19(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[19].set_out_high(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outset_pin20(&mut self) -> MemResult<u8> {
        Ok(self.gpio[20].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outset_pin20(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[20].set_out_high(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outset_pin21(&mut self) -> MemResult<u8> {
        Ok(self.gpio[21].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outset_pin21(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[21].set_out_high(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outset_pin22(&mut self) -> MemResult<u8> {
        Ok(self.gpio[22].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outset_pin22(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[22].set_out_high(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outset_pin23(&mut self) -> MemResult<u8> {
        Ok(self.gpio[23].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outset_pin23(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[23].set_out_high(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outset_pin24(&mut self) -> MemResult<u8> {
        Ok(self.gpio[24].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outset_pin24(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[24].set_out_high(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outset_pin25(&mut self) -> MemResult<u8> {
        Ok(self.gpio[25].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outset_pin25(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[25].set_out_high(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outset_pin26(&mut self) -> MemResult<u8> {
        Ok(self.gpio[26].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outset_pin26(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[26].set_out_high(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outset_pin27(&mut self) -> MemResult<u8> {
        Ok(self.gpio[27].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outset_pin27(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[27].set_out_high(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outset_pin28(&mut self) -> MemResult<u8> {
        Ok(self.gpio[28].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outset_pin28(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[28].set_out_high(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outset_pin29(&mut self) -> MemResult<u8> {
        Ok(self.gpio[29].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outset_pin29(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[29].set_out_high(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outset_pin30(&mut self) -> MemResult<u8> {
        Ok(self.gpio[30].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outset_pin30(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[30].set_out_high(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outset_pin31(&mut self) -> MemResult<u8> {
        Ok(self.gpio[31].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outset_pin31(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[31].set_out_high(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outclr_pin0(&mut self) -> MemResult<u8> {
        Ok(self.gpio[0].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outclr_pin0(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[0].set_out_high(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outclr_pin1(&mut self) -> MemResult<u8> {
        Ok(self.gpio[1].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outclr_pin1(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[1].set_out_high(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outclr_pin2(&mut self) -> MemResult<u8> {
        Ok(self.gpio[2].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outclr_pin2(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[2].set_out_high(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outclr_pin3(&mut self) -> MemResult<u8> {
        Ok(self.gpio[3].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outclr_pin3(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[3].set_out_high(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outclr_pin4(&mut self) -> MemResult<u8> {
        Ok(self.gpio[4].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outclr_pin4(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[4].set_out_high(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outclr_pin5(&mut self) -> MemResult<u8> {
        Ok(self.gpio[5].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outclr_pin5(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[5].set_out_high(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outclr_pin6(&mut self) -> MemResult<u8> {
        Ok(self.gpio[6].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outclr_pin6(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[6].set_out_high(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outclr_pin7(&mut self) -> MemResult<u8> {
        Ok(self.gpio[7].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outclr_pin7(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[7].set_out_high(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outclr_pin8(&mut self) -> MemResult<u8> {
        Ok(self.gpio[8].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outclr_pin8(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[8].set_out_high(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outclr_pin9(&mut self) -> MemResult<u8> {
        Ok(self.gpio[9].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outclr_pin9(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[9].set_out_high(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outclr_pin10(&mut self) -> MemResult<u8> {
        Ok(self.gpio[10].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outclr_pin10(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[10].set_out_high(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outclr_pin11(&mut self) -> MemResult<u8> {
        Ok(self.gpio[11].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outclr_pin11(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[11].set_out_high(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outclr_pin12(&mut self) -> MemResult<u8> {
        Ok(self.gpio[12].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outclr_pin12(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[12].set_out_high(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outclr_pin13(&mut self) -> MemResult<u8> {
        Ok(self.gpio[13].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outclr_pin13(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[13].set_out_high(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outclr_pin14(&mut self) -> MemResult<u8> {
        Ok(self.gpio[14].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outclr_pin14(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[14].set_out_high(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outclr_pin15(&mut self) -> MemResult<u8> {
        Ok(self.gpio[15].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outclr_pin15(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[15].set_out_high(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outclr_pin16(&mut self) -> MemResult<u8> {
        Ok(self.gpio[16].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outclr_pin16(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[16].set_out_high(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outclr_pin17(&mut self) -> MemResult<u8> {
        Ok(self.gpio[17].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outclr_pin17(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[17].set_out_high(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outclr_pin18(&mut self) -> MemResult<u8> {
        Ok(self.gpio[18].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outclr_pin18(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[18].set_out_high(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outclr_pin19(&mut self) -> MemResult<u8> {
        Ok(self.gpio[19].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outclr_pin19(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[19].set_out_high(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outclr_pin20(&mut self) -> MemResult<u8> {
        Ok(self.gpio[20].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outclr_pin20(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[20].set_out_high(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outclr_pin21(&mut self) -> MemResult<u8> {
        Ok(self.gpio[21].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outclr_pin21(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[21].set_out_high(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outclr_pin22(&mut self) -> MemResult<u8> {
        Ok(self.gpio[22].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outclr_pin22(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[22].set_out_high(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outclr_pin23(&mut self) -> MemResult<u8> {
        Ok(self.gpio[23].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outclr_pin23(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[23].set_out_high(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outclr_pin24(&mut self) -> MemResult<u8> {
        Ok(self.gpio[24].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outclr_pin24(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[24].set_out_high(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outclr_pin25(&mut self) -> MemResult<u8> {
        Ok(self.gpio[25].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outclr_pin25(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[25].set_out_high(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outclr_pin26(&mut self) -> MemResult<u8> {
        Ok(self.gpio[26].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outclr_pin26(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[26].set_out_high(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outclr_pin27(&mut self) -> MemResult<u8> {
        Ok(self.gpio[27].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outclr_pin27(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[27].set_out_high(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outclr_pin28(&mut self) -> MemResult<u8> {
        Ok(self.gpio[28].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outclr_pin28(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[28].set_out_high(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outclr_pin29(&mut self) -> MemResult<u8> {
        Ok(self.gpio[29].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outclr_pin29(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[29].set_out_high(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outclr_pin30(&mut self) -> MemResult<u8> {
        Ok(self.gpio[30].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outclr_pin30(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[30].set_out_high(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_outclr_pin31(&mut self) -> MemResult<u8> {
        Ok(self.gpio[31].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_outclr_pin31(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[31].set_out_high(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_in_pin0(&mut self) -> MemResult<u8> {
        Ok(self.gpio[0].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin1(&mut self) -> MemResult<u8> {
        Ok(self.gpio[1].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin2(&mut self) -> MemResult<u8> {
        Ok(self.gpio[2].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin3(&mut self) -> MemResult<u8> {
        Ok(self.gpio[3].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin4(&mut self) -> MemResult<u8> {
        Ok(self.gpio[4].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin5(&mut self) -> MemResult<u8> {
        Ok(self.gpio[5].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin6(&mut self) -> MemResult<u8> {
        Ok(self.gpio[6].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin7(&mut self) -> MemResult<u8> {
        Ok(self.gpio[7].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin8(&mut self) -> MemResult<u8> {
        Ok(self.gpio[8].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin9(&mut self) -> MemResult<u8> {
        Ok(self.gpio[9].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin10(&mut self) -> MemResult<u8> {
        Ok(self.gpio[10].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin11(&mut self) -> MemResult<u8> {
        Ok(self.gpio[11].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin12(&mut self) -> MemResult<u8> {
        Ok(self.gpio[12].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin13(&mut self) -> MemResult<u8> {
        Ok(self.gpio[13].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin14(&mut self) -> MemResult<u8> {
        Ok(self.gpio[14].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin15(&mut self) -> MemResult<u8> {
        Ok(self.gpio[15].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin16(&mut self) -> MemResult<u8> {
        Ok(self.gpio[16].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin17(&mut self) -> MemResult<u8> {
        Ok(self.gpio[17].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin18(&mut self) -> MemResult<u8> {
        Ok(self.gpio[18].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin19(&mut self) -> MemResult<u8> {
        Ok(self.gpio[19].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin20(&mut self) -> MemResult<u8> {
        Ok(self.gpio[20].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin21(&mut self) -> MemResult<u8> {
        Ok(self.gpio[21].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin22(&mut self) -> MemResult<u8> {
        Ok(self.gpio[22].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin23(&mut self) -> MemResult<u8> {
        Ok(self.gpio[23].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin24(&mut self) -> MemResult<u8> {
        Ok(self.gpio[24].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin25(&mut self) -> MemResult<u8> {
        Ok(self.gpio[25].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin26(&mut self) -> MemResult<u8> {
        Ok(self.gpio[26].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin27(&mut self) -> MemResult<u8> {
        Ok(self.gpio[27].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin28(&mut self) -> MemResult<u8> {
        Ok(self.gpio[28].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin29(&mut self) -> MemResult<u8> {
        Ok(self.gpio[29].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin30(&mut self) -> MemResult<u8> {
        Ok(self.gpio[30].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin31(&mut self) -> MemResult<u8> {
        Ok(self.gpio[31].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_dir_pin0(&mut self) -> MemResult<u8> {
        Ok(self.gpio[0].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin0(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[0].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin1(&mut self) -> MemResult<u8> {
        Ok(self.gpio[1].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin1(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[1].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin2(&mut self) -> MemResult<u8> {
        Ok(self.gpio[2].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin2(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[2].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin3(&mut self) -> MemResult<u8> {
        Ok(self.gpio[3].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin3(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[3].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin4(&mut self) -> MemResult<u8> {
        Ok(self.gpio[4].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin4(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[4].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin5(&mut self) -> MemResult<u8> {
        Ok(self.gpio[5].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin5(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[5].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin6(&mut self) -> MemResult<u8> {
        Ok(self.gpio[6].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin6(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[6].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin7(&mut self) -> MemResult<u8> {
        Ok(self.gpio[7].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin7(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[7].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin8(&mut self) -> MemResult<u8> {
        Ok(self.gpio[8].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin8(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[8].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin9(&mut self) -> MemResult<u8> {
        Ok(self.gpio[9].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin9(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[9].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin10(&mut self) -> MemResult<u8> {
        Ok(self.gpio[10].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin10(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[10].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin11(&mut self) -> MemResult<u8> {
        Ok(self.gpio[11].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin11(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[11].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin12(&mut self) -> MemResult<u8> {
        Ok(self.gpio[12].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin12(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[12].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin13(&mut self) -> MemResult<u8> {
        Ok(self.gpio[13].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin13(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[13].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin14(&mut self) -> MemResult<u8> {
        Ok(self.gpio[14].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin14(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[14].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin15(&mut self) -> MemResult<u8> {
        Ok(self.gpio[15].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin15(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[15].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin16(&mut self) -> MemResult<u8> {
        Ok(self.gpio[16].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin16(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[16].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin17(&mut self) -> MemResult<u8> {
        Ok(self.gpio[17].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin17(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[17].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin18(&mut self) -> MemResult<u8> {
        Ok(self.gpio[18].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin18(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[18].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin19(&mut self) -> MemResult<u8> {
        Ok(self.gpio[19].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin19(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[19].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin20(&mut self) -> MemResult<u8> {
        Ok(self.gpio[20].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin20(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[20].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin21(&mut self) -> MemResult<u8> {
        Ok(self.gpio[21].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin21(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[21].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin22(&mut self) -> MemResult<u8> {
        Ok(self.gpio[22].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin22(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[22].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin23(&mut self) -> MemResult<u8> {
        Ok(self.gpio[23].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin23(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[23].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin24(&mut self) -> MemResult<u8> {
        Ok(self.gpio[24].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin24(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[24].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin25(&mut self) -> MemResult<u8> {
        Ok(self.gpio[25].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin25(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[25].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin26(&mut self) -> MemResult<u8> {
        Ok(self.gpio[26].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin26(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[26].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin27(&mut self) -> MemResult<u8> {
        Ok(self.gpio[27].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin27(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[27].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin28(&mut self) -> MemResult<u8> {
        Ok(self.gpio[28].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin28(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[28].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin29(&mut self) -> MemResult<u8> {
        Ok(self.gpio[29].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin29(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[29].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin30(&mut self) -> MemResult<u8> {
        Ok(self.gpio[30].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin30(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[30].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin31(&mut self) -> MemResult<u8> {
        Ok(self.gpio[31].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin31(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[31].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dirset_pin0(&mut self) -> MemResult<u8> {
        Ok(self.gpio[0].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirset_pin0(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[0].set_direction(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirset_pin1(&mut self) -> MemResult<u8> {
        Ok(self.gpio[1].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirset_pin1(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[1].set_direction(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirset_pin2(&mut self) -> MemResult<u8> {
        Ok(self.gpio[2].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirset_pin2(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[2].set_direction(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirset_pin3(&mut self) -> MemResult<u8> {
        Ok(self.gpio[3].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirset_pin3(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[3].set_direction(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirset_pin4(&mut self) -> MemResult<u8> {
        Ok(self.gpio[4].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirset_pin4(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[4].set_direction(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirset_pin5(&mut self) -> MemResult<u8> {
        Ok(self.gpio[5].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirset_pin5(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[5].set_direction(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirset_pin6(&mut self) -> MemResult<u8> {
        Ok(self.gpio[6].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirset_pin6(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[6].set_direction(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirset_pin7(&mut self) -> MemResult<u8> {
        Ok(self.gpio[7].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirset_pin7(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[7].set_direction(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirset_pin8(&mut self) -> MemResult<u8> {
        Ok(self.gpio[8].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirset_pin8(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[8].set_direction(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirset_pin9(&mut self) -> MemResult<u8> {
        Ok(self.gpio[9].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirset_pin9(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[9].set_direction(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirset_pin10(&mut self) -> MemResult<u8> {
        Ok(self.gpio[10].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirset_pin10(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[10].set_direction(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirset_pin11(&mut self) -> MemResult<u8> {
        Ok(self.gpio[11].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirset_pin11(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[11].set_direction(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirset_pin12(&mut self) -> MemResult<u8> {
        Ok(self.gpio[12].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirset_pin12(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[12].set_direction(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirset_pin13(&mut self) -> MemResult<u8> {
        Ok(self.gpio[13].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirset_pin13(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[13].set_direction(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirset_pin14(&mut self) -> MemResult<u8> {
        Ok(self.gpio[14].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirset_pin14(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[14].set_direction(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirset_pin15(&mut self) -> MemResult<u8> {
        Ok(self.gpio[15].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirset_pin15(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[15].set_direction(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirset_pin16(&mut self) -> MemResult<u8> {
        Ok(self.gpio[16].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirset_pin16(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[16].set_direction(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirset_pin17(&mut self) -> MemResult<u8> {
        Ok(self.gpio[17].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirset_pin17(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[17].set_direction(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirset_pin18(&mut self) -> MemResult<u8> {
        Ok(self.gpio[18].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirset_pin18(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[18].set_direction(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirset_pin19(&mut self) -> MemResult<u8> {
        Ok(self.gpio[19].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirset_pin19(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[19].set_direction(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirset_pin20(&mut self) -> MemResult<u8> {
        Ok(self.gpio[20].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirset_pin20(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[20].set_direction(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirset_pin21(&mut self) -> MemResult<u8> {
        Ok(self.gpio[21].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirset_pin21(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[21].set_direction(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirset_pin22(&mut self) -> MemResult<u8> {
        Ok(self.gpio[22].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirset_pin22(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[22].set_direction(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirset_pin23(&mut self) -> MemResult<u8> {
        Ok(self.gpio[23].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirset_pin23(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[23].set_direction(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirset_pin24(&mut self) -> MemResult<u8> {
        Ok(self.gpio[24].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirset_pin24(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[24].set_direction(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirset_pin25(&mut self) -> MemResult<u8> {
        Ok(self.gpio[25].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirset_pin25(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[25].set_direction(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirset_pin26(&mut self) -> MemResult<u8> {
        Ok(self.gpio[26].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirset_pin26(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[26].set_direction(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirset_pin27(&mut self) -> MemResult<u8> {
        Ok(self.gpio[27].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirset_pin27(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[27].set_direction(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirset_pin28(&mut self) -> MemResult<u8> {
        Ok(self.gpio[28].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirset_pin28(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[28].set_direction(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirset_pin29(&mut self) -> MemResult<u8> {
        Ok(self.gpio[29].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirset_pin29(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[29].set_direction(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirset_pin30(&mut self) -> MemResult<u8> {
        Ok(self.gpio[30].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirset_pin30(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[30].set_direction(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirset_pin31(&mut self) -> MemResult<u8> {
        Ok(self.gpio[31].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirset_pin31(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[31].set_direction(true);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirclr_pin0(&mut self) -> MemResult<u8> {
        Ok(self.gpio[0].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirclr_pin0(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[0].set_direction(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirclr_pin1(&mut self) -> MemResult<u8> {
        Ok(self.gpio[1].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirclr_pin1(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[1].set_direction(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirclr_pin2(&mut self) -> MemResult<u8> {
        Ok(self.gpio[2].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirclr_pin2(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[2].set_direction(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirclr_pin3(&mut self) -> MemResult<u8> {
        Ok(self.gpio[3].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirclr_pin3(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[3].set_direction(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirclr_pin4(&mut self) -> MemResult<u8> {
        Ok(self.gpio[4].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirclr_pin4(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[4].set_direction(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirclr_pin5(&mut self) -> MemResult<u8> {
        Ok(self.gpio[5].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirclr_pin5(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[5].set_direction(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirclr_pin6(&mut self) -> MemResult<u8> {
        Ok(self.gpio[6].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirclr_pin6(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[6].set_direction(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirclr_pin7(&mut self) -> MemResult<u8> {
        Ok(self.gpio[7].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirclr_pin7(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[7].set_direction(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirclr_pin8(&mut self) -> MemResult<u8> {
        Ok(self.gpio[8].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirclr_pin8(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[8].set_direction(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirclr_pin9(&mut self) -> MemResult<u8> {
        Ok(self.gpio[9].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirclr_pin9(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[9].set_direction(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirclr_pin10(&mut self) -> MemResult<u8> {
        Ok(self.gpio[10].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirclr_pin10(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[10].set_direction(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirclr_pin11(&mut self) -> MemResult<u8> {
        Ok(self.gpio[11].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirclr_pin11(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[11].set_direction(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirclr_pin12(&mut self) -> MemResult<u8> {
        Ok(self.gpio[12].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirclr_pin12(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[12].set_direction(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirclr_pin13(&mut self) -> MemResult<u8> {
        Ok(self.gpio[13].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirclr_pin13(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[13].set_direction(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirclr_pin14(&mut self) -> MemResult<u8> {
        Ok(self.gpio[14].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirclr_pin14(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[14].set_direction(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirclr_pin15(&mut self) -> MemResult<u8> {
        Ok(self.gpio[15].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirclr_pin15(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[15].set_direction(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirclr_pin16(&mut self) -> MemResult<u8> {
        Ok(self.gpio[16].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirclr_pin16(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[16].set_direction(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirclr_pin17(&mut self) -> MemResult<u8> {
        Ok(self.gpio[17].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirclr_pin17(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[17].set_direction(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirclr_pin18(&mut self) -> MemResult<u8> {
        Ok(self.gpio[18].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirclr_pin18(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[18].set_direction(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirclr_pin19(&mut self) -> MemResult<u8> {
        Ok(self.gpio[19].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirclr_pin19(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[19].set_direction(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirclr_pin20(&mut self) -> MemResult<u8> {
        Ok(self.gpio[20].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirclr_pin20(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[20].set_direction(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirclr_pin21(&mut self) -> MemResult<u8> {
        Ok(self.gpio[21].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirclr_pin21(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[21].set_direction(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirclr_pin22(&mut self) -> MemResult<u8> {
        Ok(self.gpio[22].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirclr_pin22(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[22].set_direction(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirclr_pin23(&mut self) -> MemResult<u8> {
        Ok(self.gpio[23].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirclr_pin23(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[23].set_direction(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirclr_pin24(&mut self) -> MemResult<u8> {
        Ok(self.gpio[24].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirclr_pin24(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[24].set_direction(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirclr_pin25(&mut self) -> MemResult<u8> {
        Ok(self.gpio[25].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirclr_pin25(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[25].set_direction(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirclr_pin26(&mut self) -> MemResult<u8> {
        Ok(self.gpio[26].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirclr_pin26(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[26].set_direction(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirclr_pin27(&mut self) -> MemResult<u8> {
        Ok(self.gpio[27].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirclr_pin27(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[27].set_direction(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirclr_pin28(&mut self) -> MemResult<u8> {
        Ok(self.gpio[28].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirclr_pin28(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[28].set_direction(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirclr_pin29(&mut self) -> MemResult<u8> {
        Ok(self.gpio[29].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirclr_pin29(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[29].set_direction(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirclr_pin30(&mut self) -> MemResult<u8> {
        Ok(self.gpio[30].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirclr_pin30(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[30].set_direction(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_dirclr_pin31(&mut self) -> MemResult<u8> {
        Ok(self.gpio[31].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dirclr_pin31(&mut self, _value: u8) -> MemResult<()> {
        if _value != 0 {
            self.gpio[31].set_direction(false);
        }
        Ok(())
    }
    #[inline]
    pub fn read_gpio_pin_cnfn_dir(&mut self, _dim: usize) -> MemResult<u8> {
        Ok(self.gpio[_dim].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_pin_cnfn_dir(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.gpio[_dim].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_pin_cnfn_input(&mut self, _dim: usize) -> MemResult<u8> {
        Ok(self.gpio[_dim].is_connected_buffer() as u8)
    }
    #[inline]
    pub fn write_gpio_pin_cnfn_input(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.gpio[_dim].set_connected_buffer(_value != 0))
    }
    #[inline]
    pub fn read_gpio_pin_cnfn_pull(&mut self, _dim: usize) -> MemResult<u8> {
        Ok(self.gpio[_dim].get_pull() as u8)
    }
    #[inline]
    pub fn write_gpio_pin_cnfn_pull(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        // TODO allow invalid values in GpioPull??
        Ok(self.gpio[_dim]
            .set_pull(_value.try_into().map_err(|_| MemError::WriteViolation)?))
    }
    #[inline]
    pub fn read_gpio_pin_cnfn_drive(&mut self, _dim: usize) -> MemResult<u8> {
        Ok(self.gpio[_dim].get_sense() as u8)
    }
    #[inline]
    pub fn write_gpio_pin_cnfn_drive(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.gpio[_dim].set_sense(_value.try_into().unwrap()))
    }
    #[inline]
    pub fn read_gpio_pin_cnfn_sense(&mut self, _dim: usize) -> MemResult<u8> {
        Ok(self.gpio[_dim].get_sense() as u8)
    }
    #[inline]
    pub fn write_gpio_pin_cnfn_sense(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        // TODO allow invalid values in GpioSense??
        Ok(self.gpio[_dim].set_sense(
            _value.try_into().map_err(|_| MemError::WriteViolation)?,
        ))
    }
}
