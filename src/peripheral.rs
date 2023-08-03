use icicle_vm::cpu::mem::{MemError, MemResult};

mod gpio;
use gpio::*;
mod interrupt;
use interrupt::*;
mod clock;
use clock::*;
mod watchdog;
use watchdog::*;
mod control;
use control::*;

#[derive(Default)]
pub struct Peripherals {
    pub gpio: [Gpio; 32],
    pub interrupts: Interrupts,
    pub clock: Clock,
    pub wdt: Wdt,
    pub control: Control,
}
impl Peripherals {
    pub fn ram_is_on(&self, _block: u8) -> bool {
        todo!()
    }
    pub fn ram_is_keep_on(&self, _block: u8) -> bool {
        todo!()
    }
    pub fn ram_set_keep_on(&mut self, _block: u8, _on: bool) {
        todo!()
    }
    pub fn ram_is_retain_on(&self, _block: u8) -> bool {
        todo!()
    }
    pub fn ram_set_retain_on(&mut self, _block: u8, _on: bool) {
        todo!()
    }

    pub fn mpu_is_protected(&self, _region: u8) -> bool {
        todo!()
    }
    pub fn mpu_enable_protection(&mut self, _region: u8) {
        todo!()
    }

    pub fn radio_ap(&self, _ap: u8) -> u8 {
        todo!()
    }
    pub fn radio_set_ap(&mut self, _ap: u8, _value: u8) {
        todo!()
    }
    pub fn radio_is_recv_on_ap(&self, _ap: u8) -> bool {
        todo!()
    }
    pub fn radio_set_recv_on_ap(&mut self, _ap: u8, _on: bool) {
        todo!()
    }
    pub fn radio_is_ena(&self, _da: u8) -> bool {
        todo!()
    }
    pub fn radio_set_ena(&mut self, _da: u8, _on: bool) {
        todo!()
    }
    pub fn radio_is_txadd(&self, _da: u8) -> bool {
        todo!()
    }
    pub fn radio_set_txadd(&mut self, _da: u8, _on: bool) {
        todo!()
    }

    pub fn ppi_is_on(&self, _channel: u8) -> bool {
        todo!()
    }
    pub fn ppi_set_on(&mut self, _channel: u8, _on: bool) {
        todo!()
    }
    pub fn ppi_is_included(&self, _dim: usize, _channel: u8) -> bool {
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

    pub fn read_control_actlr_disoofp(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_actlr_disoofp(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_actlr_disfpca(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_actlr_disfpca(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_actlr_disfold(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_actlr_disfold(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_actlr_disdefwbuf(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_actlr_disdefwbuf(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_actlr_dismcycint(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_actlr_dismcycint(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_icsr_nmipendset(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_icsr_nmipendset(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_icsr_pendsvset(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_icsr_pendsvset(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_icsr_pendsvclr(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_icsr_pendsvclr(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_icsr_pendstset(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_icsr_pendstset(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_icsr_pendstclr(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_icsr_pendstclr(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_icsr_isrpreempt(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_icsr_isrpreempt(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_icsr_isrpending(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_icsr_isrpending(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_icsr_vectpending(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
    ) -> MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 511u64;
        match (_byte_0, _byte_1) {
            (None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_control_icsr_vectpending(
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
    pub fn read_control_icsr_rettobase(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_icsr_rettobase(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_icsr_vectactive(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
    ) -> MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 511u64;
        match (_byte_0, _byte_1) {
            (None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_control_icsr_vectactive(
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
    pub fn read_control_vtor_tbloff(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 33554431u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_control_vtor_tbloff(
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
    pub fn read_control_vtor_tblbase(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_vtor_tblbase(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_demcr_mon_en(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_control_demcr_mon_pend(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_control_aircr_vectkey(
        &self,
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
    pub fn write_control_aircr_vectkey(
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
    pub fn read_control_aircr_endianness(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_aircr_endianness(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_aircr_prigroup(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_control_aircr_prigroup(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_aircr_sysresetreq(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_aircr_sysresetreq(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_aircr_vectclractive(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_aircr_vectclractive(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    #[inline]
    pub fn read_control_scr_sevonpend(&self) -> MemResult<u8> {
        Ok(self.control.event_on_pending() as u8)
    }
    #[inline]
    pub fn write_control_scr_sevonpend(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.control.set_event_on_pending(_value != 0))
    }
    #[inline]
    pub fn read_control_scr_sleepdeep(&self) -> MemResult<u8> {
        Ok(self.control.sleep_deep() as u8)
    }
    #[inline]
    pub fn write_control_scr_sleepdeep(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.control.set_sleep_deep(_value != 0))
    }
    #[inline]
    pub fn read_control_scr_sleeponexit(&self) -> MemResult<u8> {
        Ok(self.control.sleep_on_exit() as u8)
    }
    #[inline]
    pub fn write_control_scr_sleeponexit(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.control.set_sleep_on_exit(_value != 0))
    }
    pub fn read_control_ccr_stkalign(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_ccr_stkalign(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_ccr_bfhfnmign(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_ccr_bfhfnmign(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_ccr_div_0_trp(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_ccr_div_0_trp(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_ccr_unalign_trp(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_ccr_unalign_trp(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_ccr_usersetmpend(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_ccr_usersetmpend(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_ccr_nonbasethrdena(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_ccr_nonbasethrdena(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    #[inline]
    pub fn read_control_shpr1_pri_7(&self) -> MemResult<u8> {
        Ok(self.control.priority(SysHandlerPriority::Pri7))
    }
    #[inline]
    pub fn write_control_shpr1_pri_7(
        &mut self,
        _priority: u8,
    ) -> MemResult<()> {
        self.control
            .set_priority(SysHandlerPriority::Pri7, _priority)
    }
    #[inline]
    pub fn read_control_shpr1_pri_6(&self) -> MemResult<u8> {
        Ok(self.control.priority(SysHandlerPriority::Pri6))
    }
    #[inline]
    pub fn write_control_shpr1_pri_6(
        &mut self,
        _priority: u8,
    ) -> MemResult<()> {
        self.control
            .set_priority(SysHandlerPriority::Pri6, _priority)
    }
    #[inline]
    pub fn read_control_shpr1_pri_5(&self) -> MemResult<u8> {
        Ok(self.control.priority(SysHandlerPriority::Pri5))
    }
    #[inline]
    pub fn write_control_shpr1_pri_5(
        &mut self,
        _priority: u8,
    ) -> MemResult<()> {
        self.control
            .set_priority(SysHandlerPriority::Pri5, _priority)
    }
    #[inline]
    pub fn read_control_shpr1_pri_4(&self) -> MemResult<u8> {
        Ok(self.control.priority(SysHandlerPriority::Pri4))
    }
    #[inline]
    pub fn write_control_shpr1_pri_4(
        &mut self,
        _priority: u8,
    ) -> MemResult<()> {
        self.control
            .set_priority(SysHandlerPriority::Pri4, _priority)
    }
    #[inline]
    pub fn read_control_shpr2_pri_11(&self) -> MemResult<u8> {
        Ok(self.control.priority(SysHandlerPriority::Pri11))
    }
    #[inline]
    pub fn write_control_shpr2_pri_11(
        &mut self,
        _priority: u8,
    ) -> MemResult<()> {
        self.control
            .set_priority(SysHandlerPriority::Pri11, _priority)
    }
    #[inline]
    pub fn read_control_shpr2_pri_10(&self) -> MemResult<u8> {
        Ok(self.control.priority(SysHandlerPriority::Pri10))
    }
    #[inline]
    pub fn write_control_shpr2_pri_10(
        &mut self,
        _priority: u8,
    ) -> MemResult<()> {
        self.control
            .set_priority(SysHandlerPriority::Pri10, _priority)
    }
    #[inline]
    pub fn read_control_shpr2_pri_9(&self) -> MemResult<u8> {
        Ok(self.control.priority(SysHandlerPriority::Pri9))
    }
    #[inline]
    pub fn write_control_shpr2_pri_9(
        &mut self,
        _priority: u8,
    ) -> MemResult<()> {
        self.control
            .set_priority(SysHandlerPriority::Pri9, _priority)
    }
    #[inline]
    pub fn read_control_shpr2_pri_8(&self) -> MemResult<u8> {
        Ok(self.control.priority(SysHandlerPriority::Pri8))
    }
    #[inline]
    pub fn write_control_shpr2_pri_8(
        &mut self,
        _priority: u8,
    ) -> MemResult<()> {
        self.control
            .set_priority(SysHandlerPriority::Pri8, _priority)
    }
    #[inline]
    pub fn read_control_shpr3_pri_15(&self) -> MemResult<u8> {
        Ok(self.control.priority(SysHandlerPriority::Pri15))
    }
    #[inline]
    pub fn write_control_shpr3_pri_15(
        &mut self,
        _priority: u8,
    ) -> MemResult<()> {
        self.control
            .set_priority(SysHandlerPriority::Pri15, _priority)
    }
    #[inline]
    pub fn read_control_shpr3_pri_14(&self) -> MemResult<u8> {
        Ok(self.control.priority(SysHandlerPriority::Pri14))
    }
    #[inline]
    pub fn write_control_shpr3_pri_14(
        &mut self,
        _priority: u8,
    ) -> MemResult<()> {
        self.control
            .set_priority(SysHandlerPriority::Pri14, _priority)
    }
    #[inline]
    pub fn read_control_shpr3_pri_13(&self) -> MemResult<u8> {
        Ok(self.control.priority(SysHandlerPriority::Pri13))
    }
    #[inline]
    pub fn write_control_shpr3_pri_13(
        &mut self,
        _priority: u8,
    ) -> MemResult<()> {
        self.control
            .set_priority(SysHandlerPriority::Pri13, _priority)
    }
    #[inline]
    pub fn read_control_shpr3_pri_12(&self) -> MemResult<u8> {
        Ok(self.control.priority(SysHandlerPriority::Pri12))
    }
    #[inline]
    pub fn write_control_shpr3_pri_12(
        &mut self,
        _priority: u8,
    ) -> MemResult<()> {
        self.control
            .set_priority(SysHandlerPriority::Pri12, _priority)
    }
    pub fn read_control_shcsr_usgfaultena(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_shcsr_usgfaultena(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_shcsr_busfaultena(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_shcsr_busfaultena(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_shcsr_memfaultena(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_shcsr_memfaultena(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_shcsr_svcallpended(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_shcsr_svcallpended(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_shcsr_busfaultpended(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_shcsr_busfaultpended(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_shcsr_memfaultpended(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_shcsr_memfaultpended(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_shcsr_usgfaultpended(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_shcsr_usgfaultpended(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_shcsr_systickact(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_shcsr_systickact(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_shcsr_pendsvact(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_shcsr_pendsvact(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_shcsr_monitoract(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_shcsr_monitoract(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_shcsr_svcallact(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_shcsr_svcallact(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_shcsr_usgfaultact(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_shcsr_usgfaultact(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_shcsr_busfaultact(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_shcsr_busfaultact(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_shcsr_memfaultact(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_shcsr_memfaultact(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_cfsr_divbyzero(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_cfsr_divbyzero(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_cfsr_unaligned(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_cfsr_unaligned(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_cfsr_nocp(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_cfsr_nocp(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_cfsr_invpc(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_cfsr_invpc(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_cfsr_invstate(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_cfsr_invstate(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_cfsr_undefinstr(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_cfsr_undefinstr(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_cfsr_bfarvalid(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_cfsr_bfarvalid(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_cfsr_lsperr(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_cfsr_lsperr(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_cfsr_stkerr(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_cfsr_stkerr(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_cfsr_unstkerr(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_cfsr_unstkerr(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_cfsr_impreciserr(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_cfsr_impreciserr(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_cfsr_preciserr(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_cfsr_preciserr(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_cfsr_ibuserr(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_cfsr_ibuserr(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_cfsr_mmarvalid(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_cfsr_mmarvalid(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_cfsr_mlsperr(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_cfsr_mlsperr(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_cfsr_mstkerr(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_cfsr_mstkerr(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_cfsr_munstkerr(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_cfsr_munstkerr(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_cfsr_daccviol(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_cfsr_daccviol(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_cfsr_iaccviol(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_cfsr_iaccviol(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_hfsr_debugevt(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_hfsr_debugevt(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_hfsr_forced(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_hfsr_forced(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_hfsr_vecttbl(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_hfsr_vecttbl(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_dfsr_external(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_dfsr_external(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_dfsr_vcatch(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_dfsr_vcatch(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_dfsr_dwttrap(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_dfsr_dwttrap(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_dfsr_bkpt(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_dfsr_bkpt(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_dfsr_halted(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_control_dfsr_halted(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_mmfar(
        &self,
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
    pub fn write_control_mmfar(
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
    pub fn read_control_bfar(
        &self,
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
    pub fn write_control_bfar(
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
    pub fn read_control_afsr(
        &self,
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
    pub fn write_control_afsr(
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
    pub fn read_control_cpacr_cp11(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_control_cpacr_cp11(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_cpacr_cp10(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_control_cpacr_cp10(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_cpacr_cp7(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_control_cpacr_cp7(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_cpacr_cp6(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_control_cpacr_cp6(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_cpacr_cp5(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_control_cpacr_cp5(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_cpacr_cp4(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_control_cpacr_cp4(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_cpacr_cp3(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_control_cpacr_cp3(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_cpacr_cp2(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_control_cpacr_cp2(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_cpacr_cp1(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_control_cpacr_cp1(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_control_cpacr_cp0(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_control_cpacr_cp0(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn write_control_stir_intid(
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
    pub fn read_id_cpuid_implementer(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn read_id_cpuid_variant(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_cpuid_constant(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_cpuid_partno(
        &self,
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
    pub fn read_id_cpuid_revision(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_pfr0_state1(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_pfr1_m_profile(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_dfr0_m_profile(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_afr0(
        &self,
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
    pub fn read_id_id_mmfr0_auxiliary_registers(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_mmfr0_shareability_levels(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_mmfr0_outermost_shareability(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_mmfr0_pmsa(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_mmfr1(
        &self,
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
    pub fn read_id_id_mmfr2_wfi(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_mmfr3(
        &self,
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
    pub fn read_id_id_isar0_divide_instrs(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_isar0_debug_instrs(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_isar0_coproc_instrs(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_isar0_cmpbranch_instrs(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_isar0_bitfield_instrs(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_isar0_bitcount_instrs(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_isar1_interwork_instrs(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_isar1_immediate_instrs(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_isar1_ifthen_instrs(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_isar1_extend_instrs(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_isar2_reversal_instrs(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_isar2_multu_instrs(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_isar2_mults_instrs(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_isar2_mult_instrs(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_isar2_multiaccessint_instrs(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_isar2_loadstore_instrs(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_isar3_truenop_instrs(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_isar3_thumbcopy_instrs(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_isar3_tabbranch_instrs(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_isar3_synchprim_instrs(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_isar3_svc_instrs(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_isar3_simd_instrs(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_isar3_saturate_instrs(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_isar4_psr_m_instrs(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_isar4_synchprim_instrs_frac(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_isar4_barrier_instrs(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_isar4_writeback_instrs(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_isar4_withshifts_instrs(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_id_id_isar4_unpriv_instrs(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_fpe_fpccr_aspen(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_fpe_fpccr_aspen(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_fpe_fpccr_lspen(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_fpe_fpccr_lspen(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_fpe_fpccr_monrdy(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_fpe_fpccr_monrdy(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_fpe_fpccr_bfrdy(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_fpe_fpccr_bfrdy(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_fpe_fpccr_mmrdy(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_fpe_fpccr_mmrdy(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_fpe_fpccr_hfrdy(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_fpe_fpccr_hfrdy(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_fpe_fpccr_thread(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_fpe_fpccr_thread(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_fpe_fpccr_user(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_fpe_fpccr_user(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_fpe_fpccr_lspact(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_fpe_fpccr_lspact(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_fpe_fpcar(
        &self,
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
    pub fn write_fpe_fpcar(
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
    pub fn read_fpe_fpdscr_ahp(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_fpe_fpdscr_ahp(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_fpe_fpdscr_dn(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_fpe_fpdscr_dn(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_fpe_fpdscr_fz(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_fpe_fpdscr_fz(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_fpe_fpdscr_rmode(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_fpe_fpdscr_rmode(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_fpe_mvfr0_fp_rounding_modes(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_fpe_mvfr0_short_vectors(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_fpe_mvfr0_square_root(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_fpe_mvfr0_divide(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_fpe_mvfr0_fp_exception_trapping(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_fpe_mvfr0_double_precision(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_fpe_mvfr0_single_precision(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_fpe_mvfr0_a_simd(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_fpe_mvfr1_fp_fused_mac(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_fpe_mvfr1_fp_hpfp(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_fpe_mvfr1_d_nan(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_fpe_mvfr1_ftz(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_systick_stcsr_countflag(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_systick_stcsr_countflag(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_systick_stcsr_clksource(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_systick_stcsr_clksource(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_systick_stcsr_tickint(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_systick_stcsr_tickint(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_systick_stcsr_enable(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_systick_stcsr_enable(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_systick_strvr_reload(
        &self,
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
    pub fn write_systick_strvr_reload(
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
    pub fn read_systick_stcvr_current(
        &self,
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
    pub fn write_systick_stcvr_current(
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
    pub fn read_systick_stcr_noref(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_systick_stcr_skew(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_systick_stcr_tenms(
        &self,
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
    pub fn read_nvic_ictr_intlinesnum(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    #[inline]
    pub fn read_nvic_nvic_iser0<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_enable(0, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn write_nvic_nvic_iser0(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_set_enable(0, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn read_nvic_nvic_iser1<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_enable(1, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn write_nvic_nvic_iser1(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_set_enable(1, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn read_nvic_nvic_iser2<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_enable(2, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn write_nvic_nvic_iser2(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_set_enable(2, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn read_nvic_nvic_iser3<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_enable(3, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn write_nvic_nvic_iser3(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_set_enable(3, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn read_nvic_nvic_iser4<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_enable(4, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn write_nvic_nvic_iser4(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_set_enable(4, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn read_nvic_nvic_iser5<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_enable(5, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn write_nvic_nvic_iser5(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_set_enable(5, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn read_nvic_nvic_iser6<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_enable(6, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn write_nvic_nvic_iser6(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_set_enable(6, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn read_nvic_nvic_iser7<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_enable(7, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn write_nvic_nvic_iser7(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_set_enable(7, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn read_nvic_nvic_icer0<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_enable(0, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn write_nvic_nvic_icer0(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_clr_enable(0, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn read_nvic_nvic_icer1<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_enable(1, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn write_nvic_nvic_icer1(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_clr_enable(1, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn read_nvic_nvic_icer2<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_enable(2, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn write_nvic_nvic_icer2(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_clr_enable(2, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn read_nvic_nvic_icer3<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_enable(3, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn write_nvic_nvic_icer3(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_clr_enable(3, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn read_nvic_nvic_icer4<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_enable(4, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn write_nvic_nvic_icer4(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_clr_enable(4, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn read_nvic_nvic_icer5<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_enable(5, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn write_nvic_nvic_icer5(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_clr_enable(5, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn read_nvic_nvic_icer6<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_enable(6, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn write_nvic_nvic_icer6(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_clr_enable(6, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn read_nvic_nvic_icer7<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 18446744073709551615u64;
        self.interrupts
            .nvic_enable(7, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn write_nvic_nvic_icer7(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_clr_enable(7, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn read_nvic_nvic_ispr0<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_pending(0, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn write_nvic_nvic_ispr0(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_set_pending(0, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn read_nvic_nvic_ispr1<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_pending(1, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn write_nvic_nvic_ispr1(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_set_pending(1, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn read_nvic_nvic_ispr2<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_pending(2, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn write_nvic_nvic_ispr2(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_set_pending(2, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn read_nvic_nvic_ispr3<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_pending(3, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn write_nvic_nvic_ispr3(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_set_pending(3, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn read_nvic_nvic_ispr4<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_pending(4, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn write_nvic_nvic_ispr4(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_set_pending(4, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn read_nvic_nvic_ispr5<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_pending(5, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn write_nvic_nvic_ispr5(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_set_pending(5, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn read_nvic_nvic_ispr6<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_pending(6, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn write_nvic_nvic_ispr6(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_set_pending(6, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn read_nvic_nvic_ispr7<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_pending(7, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn write_nvic_nvic_ispr7(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_set_pending(7, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn read_nvic_nvic_icpr0<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_pending(0, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn write_nvic_nvic_icpr0(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_clr_pending(0, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn read_nvic_nvic_icpr1<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_pending(1, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn write_nvic_nvic_icpr1(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_clr_pending(1, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn read_nvic_nvic_icpr2<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_pending(2, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn write_nvic_nvic_icpr2(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_clr_pending(2, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn read_nvic_nvic_icpr3<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_pending(3, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn write_nvic_nvic_icpr3(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_clr_pending(3, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn read_nvic_nvic_icpr4<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_pending(4, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn write_nvic_nvic_icpr4(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_clr_pending(4, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn read_nvic_nvic_icpr5<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_pending(5, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn write_nvic_nvic_icpr5(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_clr_pending(5, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn read_nvic_nvic_icpr6<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_pending(6, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn write_nvic_nvic_icpr6(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_clr_pending(6, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn read_nvic_nvic_icpr7<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_pending(7, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn write_nvic_nvic_icpr7(
        &mut self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_clr_pending(7, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    #[inline]
    pub fn read_nvic_nvic_iabr0<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_active(0, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    pub fn write_nvic_nvic_iabr0(
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
    pub fn read_nvic_nvic_iabr1<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_active(1, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    pub fn write_nvic_nvic_iabr1(
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
    pub fn read_nvic_nvic_iabr2<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_active(2, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    pub fn write_nvic_nvic_iabr2(
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
    pub fn read_nvic_nvic_iabr3<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_active(3, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    pub fn write_nvic_nvic_iabr3(
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
    pub fn read_nvic_nvic_iabr4<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_active(4, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    pub fn write_nvic_nvic_iabr4(
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
    pub fn read_nvic_nvic_iabr5<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_active(5, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    pub fn write_nvic_nvic_iabr5(
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
    pub fn read_nvic_nvic_iabr6<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_active(6, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    pub fn write_nvic_nvic_iabr6(
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
    pub fn read_nvic_nvic_iabr7<'a>(
        &self,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        self.interrupts
            .nvic_active(7, _byte_0, _byte_1, _byte_2, _byte_3)
    }
    pub fn write_nvic_nvic_iabr7(
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
    pub fn read_nvic_nvic_ipr0_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 0) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr0_pri_n3(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 0) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr0_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 0) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr0_pri_n2(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 0) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr0_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 0) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr0_pri_n1(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 0) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr0_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 0) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr0_pri_n0(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 0) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr1_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 1) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr1_pri_n3(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 1) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr1_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 1) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr1_pri_n2(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 1) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr1_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 1) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr1_pri_n1(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 1) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr1_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 1) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr1_pri_n0(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 1) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr2_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 2) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr2_pri_n3(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 2) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr2_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 2) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr2_pri_n2(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 2) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr2_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 2) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr2_pri_n1(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 2) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr2_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 2) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr2_pri_n0(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 2) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr3_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 3) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr3_pri_n3(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 3) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr3_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 3) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr3_pri_n2(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 3) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr3_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 3) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr3_pri_n1(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 3) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr3_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 3) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr3_pri_n0(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 3) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr4_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 4) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr4_pri_n3(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 4) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr4_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 4) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr4_pri_n2(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 4) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr4_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 4) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr4_pri_n1(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 4) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr4_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 4) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr4_pri_n0(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 4) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr5_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 5) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr5_pri_n3(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 5) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr5_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 5) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr5_pri_n2(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 5) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr5_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 5) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr5_pri_n1(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 5) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr5_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 5) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr5_pri_n0(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 5) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr6_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 6) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr6_pri_n3(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 6) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr6_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 6) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr6_pri_n2(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 6) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr6_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 6) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr6_pri_n1(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 6) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr6_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 6) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr6_pri_n0(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 6) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr7_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 7) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr7_pri_n3(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 7) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr7_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 7) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr7_pri_n2(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 7) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr7_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 7) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr7_pri_n1(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 7) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr7_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 7) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr7_pri_n0(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 7) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr8_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 8) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr8_pri_n3(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 8) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr8_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 8) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr8_pri_n2(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 8) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr8_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 8) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr8_pri_n1(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 8) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr8_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 8) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr8_pri_n0(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 8) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr9_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 9) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr9_pri_n3(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 9) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr9_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 9) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr9_pri_n2(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 9) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr9_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 9) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr9_pri_n1(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 9) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr9_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 9) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr9_pri_n0(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 9) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr10_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 10) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr10_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 10) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr10_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 10) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr10_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 10) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr10_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 10) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr10_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 10) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr10_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 10) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr10_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 10) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr11_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 11) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr11_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 11) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr11_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 11) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr11_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 11) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr11_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 11) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr11_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 11) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr11_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 11) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr11_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 11) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr12_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 12) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr12_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 12) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr12_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 12) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr12_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 12) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr12_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 12) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr12_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 12) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr12_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 12) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr12_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 12) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr13_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 13) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr13_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 13) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr13_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 13) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr13_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 13) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr13_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 13) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr13_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 13) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr13_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 13) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr13_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 13) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr14_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 14) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr14_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 14) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr14_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 14) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr14_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 14) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr14_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 14) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr14_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 14) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr14_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 14) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr14_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 14) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr15_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 15) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr15_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 15) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr15_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 15) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr15_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 15) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr15_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 15) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr15_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 15) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr15_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 15) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr15_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 15) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr16_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 16) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr16_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 16) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr16_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 16) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr16_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 16) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr16_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 16) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr16_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 16) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr16_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 16) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr16_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 16) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr17_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 17) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr17_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 17) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr17_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 17) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr17_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 17) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr17_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 17) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr17_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 17) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr17_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 17) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr17_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 17) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr18_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 18) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr18_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 18) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr18_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 18) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr18_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 18) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr18_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 18) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr18_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 18) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr18_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 18) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr18_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 18) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr19_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 19) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr19_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 19) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr19_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 19) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr19_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 19) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr19_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 19) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr19_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 19) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr19_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 19) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr19_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 19) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr20_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 20) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr20_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 20) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr20_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 20) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr20_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 20) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr20_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 20) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr20_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 20) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr20_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 20) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr20_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 20) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr21_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 21) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr21_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 21) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr21_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 21) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr21_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 21) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr21_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 21) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr21_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 21) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr21_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 21) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr21_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 21) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr22_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 22) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr22_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 22) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr22_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 22) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr22_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 22) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr22_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 22) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr22_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 22) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr22_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 22) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr22_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 22) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr23_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 23) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr23_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 23) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr23_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 23) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr23_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 23) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr23_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 23) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr23_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 23) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr23_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 23) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr23_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 23) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr24_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 24) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr24_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 24) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr24_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 24) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr24_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 24) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr24_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 24) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr24_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 24) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr24_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 24) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr24_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 24) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr25_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 25) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr25_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 25) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr25_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 25) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr25_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 25) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr25_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 25) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr25_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 25) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr25_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 25) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr25_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 25) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr26_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 26) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr26_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 26) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr26_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 26) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr26_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 26) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr26_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 26) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr26_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 26) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr26_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 26) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr26_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 26) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr27_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 27) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr27_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 27) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr27_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 27) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr27_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 27) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr27_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 27) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr27_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 27) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr27_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 27) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr27_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 27) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr28_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 28) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr28_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 28) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr28_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 28) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr28_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 28) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr28_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 28) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr28_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 28) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr28_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 28) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr28_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 28) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr29_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 29) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr29_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 29) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr29_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 29) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr29_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 29) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr29_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 29) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr29_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 29) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr29_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 29) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr29_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 29) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr30_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 30) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr30_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 30) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr30_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 30) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr30_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 30) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr30_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 30) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr30_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 30) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr30_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 30) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr30_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 30) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr31_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 31) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr31_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 31) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr31_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 31) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr31_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 31) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr31_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 31) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr31_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 31) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr31_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 31) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr31_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 31) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr32_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 32) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr32_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 32) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr32_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 32) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr32_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 32) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr32_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 32) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr32_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 32) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr32_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 32) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr32_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 32) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr33_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 33) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr33_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 33) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr33_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 33) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr33_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 33) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr33_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 33) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr33_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 33) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr33_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 33) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr33_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 33) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr34_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 34) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr34_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 34) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr34_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 34) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr34_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 34) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr34_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 34) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr34_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 34) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr34_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 34) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr34_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 34) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr35_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 35) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr35_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 35) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr35_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 35) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr35_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 35) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr35_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 35) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr35_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 35) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr35_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 35) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr35_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 35) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr36_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 36) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr36_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 36) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr36_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 36) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr36_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 36) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr36_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 36) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr36_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 36) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr36_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 36) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr36_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 36) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr37_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 37) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr37_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 37) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr37_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 37) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr37_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 37) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr37_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 37) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr37_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 37) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr37_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 37) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr37_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 37) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr38_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 38) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr38_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 38) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr38_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 38) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr38_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 38) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr38_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 38) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr38_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 38) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr38_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 38) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr38_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 38) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr39_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 39) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr39_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 39) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr39_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 39) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr39_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 39) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr39_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 39) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr39_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 39) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr39_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 39) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr39_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 39) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr40_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 40) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr40_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 40) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr40_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 40) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr40_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 40) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr40_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 40) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr40_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 40) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr40_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 40) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr40_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 40) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr41_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 41) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr41_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 41) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr41_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 41) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr41_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 41) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr41_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 41) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr41_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 41) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr41_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 41) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr41_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 41) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr42_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 42) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr42_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 42) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr42_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 42) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr42_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 42) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr42_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 42) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr42_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 42) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr42_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 42) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr42_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 42) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr43_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 43) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr43_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 43) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr43_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 43) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr43_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 43) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr43_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 43) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr43_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 43) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr43_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 43) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr43_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 43) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr44_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 44) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr44_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 44) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr44_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 44) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr44_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 44) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr44_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 44) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr44_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 44) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr44_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 44) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr44_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 44) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr45_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 45) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr45_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 45) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr45_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 45) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr45_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 45) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr45_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 45) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr45_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 45) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr45_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 45) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr45_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 45) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr46_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 46) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr46_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 46) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr46_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 46) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr46_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 46) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr46_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 46) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr46_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 46) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr46_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 46) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr46_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 46) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr47_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 47) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr47_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 47) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr47_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 47) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr47_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 47) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr47_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 47) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr47_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 47) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr47_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 47) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr47_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 47) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr48_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 48) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr48_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 48) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr48_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 48) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr48_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 48) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr48_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 48) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr48_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 48) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr48_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 48) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr48_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 48) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr49_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 49) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr49_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 49) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr49_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 49) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr49_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 49) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr49_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 49) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr49_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 49) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr49_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 49) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr49_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 49) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr50_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 50) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr50_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 50) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr50_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 50) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr50_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 50) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr50_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 50) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr50_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 50) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr50_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 50) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr50_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 50) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr51_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 51) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr51_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 51) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr51_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 51) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr51_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 51) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr51_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 51) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr51_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 51) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr51_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 51) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr51_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 51) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr52_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 52) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr52_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 52) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr52_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 52) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr52_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 52) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr52_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 52) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr52_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 52) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr52_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 52) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr52_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 52) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr53_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 53) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr53_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 53) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr53_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 53) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr53_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 53) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr53_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 53) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr53_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 53) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr53_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 53) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr53_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 53) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr54_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 54) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr54_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 54) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr54_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 54) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr54_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 54) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr54_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 54) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr54_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 54) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr54_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 54) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr54_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 54) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr55_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 55) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr55_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 55) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr55_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 55) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr55_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 55) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr55_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 55) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr55_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 55) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr55_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 55) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr55_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 55) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr56_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 56) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr56_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 56) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr56_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 56) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr56_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 56) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr56_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 56) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr56_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 56) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr56_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 56) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr56_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 56) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr57_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 57) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr57_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 57) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr57_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 57) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr57_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 57) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr57_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 57) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr57_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 57) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr57_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 57) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr57_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 57) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr58_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 58) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr58_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 58) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr58_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 58) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr58_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 58) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr58_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 58) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr58_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 58) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr58_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 58) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr58_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 58) + 0, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr59_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 59) + 3))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr59_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 59) + 3, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr59_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 59) + 2))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr59_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 59) + 2, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr59_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 59) + 1))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr59_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 59) + 1, _value))
    }
    #[inline]
    pub fn read_nvic_nvic_ipr59_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 59) + 0))
    }
    #[inline]
    pub fn write_nvic_nvic_ipr59_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 59) + 0, _value))
    }
    pub fn read_mpu_mpu_type_separate(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_mpu_mpu_type_dregion(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn read_mpu_mpu_type_iregion(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn read_mpu_mpu_ctrl_enable(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_mpu_ctrl_enable(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_ctrl_hfnmiena(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_mpu_ctrl_hfnmiena(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_ctrl_privdefena(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_mpu_ctrl_privdefena(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rnr_region(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_mpu_mpu_rnr_region(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rbar_region(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn write_mpu_mpu_rbar_region(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rbar_valid(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_mpu_rbar_valid(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rbar_addr(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 134217727u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_mpu_mpu_rbar_addr(
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
    pub fn read_mpu_mpu_rasr_enable(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_enable(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rasr_size(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 31u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_size(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rasr_srd(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_srd(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rasr_b(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_b(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rasr_c(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_c(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rasr_s(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_s(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rasr_tex(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_tex(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rasr_ap(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_ap(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rasr_xn(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_xn(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rbar_a1_region(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn write_mpu_mpu_rbar_a1_region(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rbar_a1_valid(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_mpu_rbar_a1_valid(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rbar_a1_addr(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 134217727u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_mpu_mpu_rbar_a1_addr(
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
    pub fn read_mpu_mpu_rasr_a1_enable(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_a1_enable(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rasr_a1_size(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 31u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_a1_size(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rasr_a1_srd(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_a1_srd(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rasr_a1_b(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_a1_b(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rasr_a1_c(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_a1_c(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rasr_a1_s(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_a1_s(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rasr_a1_tex(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_a1_tex(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rasr_a1_ap(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_a1_ap(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rasr_a1_xn(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_a1_xn(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rbar_a2_region(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn write_mpu_mpu_rbar_a2_region(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rbar_a2_valid(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_mpu_rbar_a2_valid(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rbar_a2_addr(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 134217727u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_mpu_mpu_rbar_a2_addr(
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
    pub fn read_mpu_mpu_rbar_a2_size(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 31u64;
        todo!()
    }
    pub fn write_mpu_mpu_rbar_a2_size(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rbar_a2_srd(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_mpu_mpu_rbar_a2_srd(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rbar_a2_b(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_mpu_rbar_a2_b(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rbar_a2_c(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_mpu_rbar_a2_c(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rbar_a2_s(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_mpu_rbar_a2_s(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rbar_a2_tex(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_mpu_mpu_rbar_a2_tex(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rbar_a2_ap(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_mpu_mpu_rbar_a2_ap(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rbar_a2_xn(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_mpu_rbar_a2_xn(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rasr_a2_enable(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_a2_enable(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rasr_a2_size(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 31u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_a2_size(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rasr_a2_srd(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_a2_srd(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rasr_a2_b(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_a2_b(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rasr_a2_c(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_a2_c(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rasr_a2_s(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_a2_s(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rasr_a2_tex(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_a2_tex(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rasr_a2_ap(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_a2_ap(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rasr_a2_xn(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_a2_xn(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rbar_a3_region(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn write_mpu_mpu_rbar_a3_region(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rbar_a3_valid(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_mpu_rbar_a3_valid(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rbar_a3_addr(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> MemResult<()> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 134217727u64;
        match (_byte_0, _byte_1, _byte_2, _byte_3) {
            (None, None, None, None) => unreachable!(),
            _ => {}
        }
        todo!();
    }
    pub fn write_mpu_mpu_rbar_a3_addr(
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
    pub fn read_mpu_mpu_rasr_a3_enable(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_a3_enable(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rasr_a3_size(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 31u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_a3_size(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rasr_a3_srd(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_a3_srd(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rasr_a3_b(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_a3_b(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rasr_a3_c(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_a3_c(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rasr_a3_s(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_a3_s(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rasr_a3_tex(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_a3_tex(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rasr_a3_ap(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_a3_ap(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_mpu_rasr_a3_xn(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_mpu_rasr_a3_xn(&mut self, _value: u8) -> MemResult<()> {
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
        &self,
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
    pub fn read_powerclock_intenset_pofwarn(&self) -> MemResult<u8> {
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
    pub fn read_powerclock_intenset_hfclkstarted(&self) -> MemResult<u8> {
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
    pub fn read_powerclock_intenset_lfclkstarted(&self) -> MemResult<u8> {
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
    pub fn read_powerclock_intenset_done(&self) -> MemResult<u8> {
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
    pub fn read_powerclock_intenset_ctto(&self) -> MemResult<u8> {
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
    pub fn read_powerclock_intenclr_pofwarn(&self) -> MemResult<u8> {
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
    pub fn read_powerclock_intenclr_hfclkstarted(&self) -> MemResult<u8> {
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
    pub fn read_powerclock_intenclr_lfclkstarted(&self) -> MemResult<u8> {
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
    pub fn read_powerclock_intenclr_done(&self) -> MemResult<u8> {
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
    pub fn read_powerclock_intenclr_ctto(&self) -> MemResult<u8> {
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
    pub fn read_power_resetreas_resetpin(&self) -> MemResult<u8> {
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
    pub fn read_power_resetreas_dog(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_resetreas_dog(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_power_resetreas_sreq(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_resetreas_sreq(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_power_resetreas_lockup(&self) -> MemResult<u8> {
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
    pub fn read_power_resetreas_off(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_resetreas_off(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_power_resetreas_lpcomp(&self) -> MemResult<u8> {
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
    pub fn read_power_resetreas_dif(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_resetreas_dif(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    #[inline]
    pub fn read_power_ramstatus_ramblock0(&self) -> MemResult<u8> {
        Ok(self.ram_is_on(0) as u8)
    }
    #[inline]
    pub fn read_power_ramstatus_ramblock1(&self) -> MemResult<u8> {
        Ok(self.ram_is_on(1) as u8)
    }
    #[inline]
    pub fn read_power_ramstatus_ramblock2(&self) -> MemResult<u8> {
        Ok(self.ram_is_on(2) as u8)
    }
    #[inline]
    pub fn read_power_ramstatus_ramblock3(&self) -> MemResult<u8> {
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
    pub fn read_power_pofcon_threshold(&self) -> MemResult<u8> {
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
    pub fn read_power_gpregret_gpregret(&self) -> MemResult<u8> {
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
    pub fn read_power_ramon_onram0(&self) -> MemResult<u8> {
        Ok(self.ram_is_keep_on(0) as u8)
    }
    #[inline]
    pub fn write_power_ramon_onram0(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ram_set_keep_on(0, _value != 0))
    }
    #[inline]
    pub fn read_power_ramon_onram1(&self) -> MemResult<u8> {
        Ok(self.ram_is_keep_on(1) as u8)
    }
    #[inline]
    pub fn write_power_ramon_onram1(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ram_set_keep_on(1, _value != 0))
    }
    #[inline]
    pub fn read_power_ramon_offram0(&self) -> MemResult<u8> {
        Ok(self.ram_is_retain_on(0) as u8)
    }
    #[inline]
    pub fn write_power_ramon_offram0(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ram_set_retain_on(0, _value != 0))
    }
    #[inline]
    pub fn read_power_ramon_offram1(&self) -> MemResult<u8> {
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
    pub fn read_power_ramonb_onram2(&self) -> MemResult<u8> {
        Ok(self.ram_is_keep_on(2) as u8)
    }
    #[inline]
    pub fn write_power_ramonb_onram2(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ram_set_keep_on(2, _value != 0))
    }
    #[inline]
    pub fn read_power_ramonb_onram3(&self) -> MemResult<u8> {
        Ok(self.ram_is_keep_on(3) as u8)
    }
    #[inline]
    pub fn write_power_ramonb_onram3(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ram_set_keep_on(3, _value != 0))
    }
    #[inline]
    pub fn read_power_ramonb_offram2(&self) -> MemResult<u8> {
        Ok(self.ram_is_retain_on(2) as u8)
    }
    #[inline]
    pub fn write_power_ramonb_offram2(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ram_set_retain_on(2, _value != 0))
    }
    #[inline]
    pub fn read_power_ramonb_offram3(&self) -> MemResult<u8> {
        Ok(self.ram_is_retain_on(3) as u8)
    }
    #[inline]
    pub fn write_power_ramonb_offram3(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ram_set_retain_on(3, _value != 0))
    }
    pub fn read_power_dcdcen_dcdcen(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_power_dcdcen_dcdcen(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_power_dcdcforce_forceoff(&self) -> MemResult<u8> {
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
    pub fn read_power_dcdcforce_forceon(&self) -> MemResult<u8> {
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
    pub fn read_clock_hfclkrun_status(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_clock_hfclkstat_src(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_clock_hfclkstat_state(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_clock_lfclkrun_status(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_clock_lfclkstat_src(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn read_clock_lfclkstat_state(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_clock_lfclksrccopy_src(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn read_clock_lfclksrc_src(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_clock_lfclksrc_src(&mut self, _value: u8) -> MemResult<()> {
        let _value =
            Source::try_from(_value).map_err(|_| MemError::WriteViolation)?;
        Ok(self.clock.set_source(_value))
    }
    pub fn read_clock_ctiv_ctiv(&self) -> MemResult<u8> {
        Ok(self.clock.source() as u8)
    }
    pub fn write_clock_ctiv_ctiv(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_clock_xtalfreq_xtalfreq(&self) -> MemResult<u8> {
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
    pub fn read_mpu_perr0_power_clock(&self) -> MemResult<u8> {
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
    pub fn read_mpu_perr0_spi0_twi0(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_mpu_perr0_spi0_twi0(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_mpu_perr0_spi1_twi1(&self) -> MemResult<u8> {
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
        &self,
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
    pub fn read_mpu_protenset0_protreg0(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset0_protreg1(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset0_protreg2(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset0_protreg3(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset0_protreg4(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset0_protreg5(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset0_protreg6(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset0_protreg7(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset0_protreg8(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset0_protreg9(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset0_protreg10(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset0_protreg11(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset0_protreg12(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset0_protreg13(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset0_protreg14(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset0_protreg15(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset0_protreg16(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset0_protreg17(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset0_protreg18(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset0_protreg19(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset0_protreg20(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset0_protreg21(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset0_protreg22(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset0_protreg23(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset0_protreg24(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset0_protreg25(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset0_protreg26(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset0_protreg27(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset0_protreg28(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset0_protreg29(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset0_protreg30(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset0_protreg31(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset1_protreg32(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset1_protreg33(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset1_protreg34(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset1_protreg35(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset1_protreg36(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset1_protreg37(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset1_protreg38(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset1_protreg39(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset1_protreg40(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset1_protreg41(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset1_protreg42(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset1_protreg43(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset1_protreg44(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset1_protreg45(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset1_protreg46(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset1_protreg47(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset1_protreg48(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset1_protreg49(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset1_protreg50(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset1_protreg51(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset1_protreg52(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset1_protreg53(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset1_protreg54(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset1_protreg55(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset1_protreg56(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset1_protreg57(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset1_protreg58(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset1_protreg59(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset1_protreg60(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset1_protreg61(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset1_protreg62(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protenset1_protreg63(&self) -> MemResult<u8> {
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
    pub fn read_mpu_disableindebug_disableindebug(&self) -> MemResult<u8> {
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
    pub fn read_mpu_protblocksize_protblocksize(&self) -> MemResult<u8> {
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
    pub fn read_radio_shorts_ready_start(&self) -> MemResult<u8> {
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
    pub fn read_radio_shorts_end_disable(&self) -> MemResult<u8> {
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
    pub fn read_radio_shorts_disabled_txen(&self) -> MemResult<u8> {
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
    pub fn read_radio_shorts_disabled_rxen(&self) -> MemResult<u8> {
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
    pub fn read_radio_shorts_address_rssistart(&self) -> MemResult<u8> {
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
    pub fn read_radio_shorts_end_start(&self) -> MemResult<u8> {
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
    pub fn read_radio_shorts_address_bcstart(&self) -> MemResult<u8> {
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
    pub fn read_radio_shorts_disabled_rssistop(&self) -> MemResult<u8> {
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
    pub fn read_radio_intenset_ready(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenset_ready(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenset_address(&self) -> MemResult<u8> {
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
    pub fn read_radio_intenset_payload(&self) -> MemResult<u8> {
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
    pub fn read_radio_intenset_end(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenset_end(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenset_disabled(&self) -> MemResult<u8> {
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
    pub fn read_radio_intenset_devmatch(&self) -> MemResult<u8> {
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
    pub fn read_radio_intenset_devmiss(&self) -> MemResult<u8> {
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
    pub fn read_radio_intenset_rssiend(&self) -> MemResult<u8> {
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
    pub fn read_radio_intenset_bcmatch(&self) -> MemResult<u8> {
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
    pub fn read_radio_intenclr_ready(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenclr_ready(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenclr_address(&self) -> MemResult<u8> {
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
    pub fn read_radio_intenclr_payload(&self) -> MemResult<u8> {
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
    pub fn read_radio_intenclr_end(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_intenclr_end(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_intenclr_disabled(&self) -> MemResult<u8> {
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
    pub fn read_radio_intenclr_devmatch(&self) -> MemResult<u8> {
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
    pub fn read_radio_intenclr_devmiss(&self) -> MemResult<u8> {
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
    pub fn read_radio_intenclr_rssiend(&self) -> MemResult<u8> {
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
    pub fn read_radio_intenclr_bcmatch(&self) -> MemResult<u8> {
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
    pub fn read_radio_crcstatus_crcstatus(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_radio_rxmatch_rxmatch(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn read_radio_rxcrc_rxcrc(
        &self,
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
        &self,
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
    pub fn read_radio_frequency_frequency(&self) -> MemResult<u8> {
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
    pub fn read_radio_txpower_txpower(&self) -> MemResult<u8> {
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
    pub fn read_radio_pcnf1_maxlen(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn write_radio_pcnf1_maxlen(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_pcnf1_statlen(&self) -> MemResult<u8> {
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
    pub fn read_radio_pcnf1_endian(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_pcnf1_endian(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_pcnf1_whiteen(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_pcnf1_whiteen(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_base0(
        &self,
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
        &self,
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
    pub fn read_radio_prefix0_ap0(&self) -> MemResult<u8> {
        Ok(self.radio_ap(0))
    }
    #[inline]
    pub fn write_radio_prefix0_ap0(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ap(0, _value))
    }
    #[inline]
    pub fn read_radio_prefix0_ap1(&self) -> MemResult<u8> {
        Ok(self.radio_ap(1))
    }
    #[inline]
    pub fn write_radio_prefix0_ap1(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ap(1, _value))
    }
    #[inline]
    pub fn read_radio_prefix0_ap2(&self) -> MemResult<u8> {
        Ok(self.radio_ap(2))
    }
    #[inline]
    pub fn write_radio_prefix0_ap2(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ap(2, _value))
    }
    #[inline]
    pub fn read_radio_prefix0_ap3(&self) -> MemResult<u8> {
        Ok(self.radio_ap(3))
    }
    #[inline]
    pub fn write_radio_prefix0_ap3(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ap(3, _value))
    }
    #[inline]
    pub fn read_radio_prefix1_ap4(&self) -> MemResult<u8> {
        Ok(self.radio_ap(4))
    }
    #[inline]
    pub fn write_radio_prefix1_ap4(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ap(4, _value))
    }
    #[inline]
    pub fn read_radio_prefix1_ap5(&self) -> MemResult<u8> {
        Ok(self.radio_ap(5))
    }
    #[inline]
    pub fn write_radio_prefix1_ap5(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ap(5, _value))
    }
    #[inline]
    pub fn read_radio_prefix1_ap6(&self) -> MemResult<u8> {
        Ok(self.radio_ap(6))
    }
    #[inline]
    pub fn write_radio_prefix1_ap6(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ap(6, _value))
    }
    #[inline]
    pub fn read_radio_prefix1_ap7(&self) -> MemResult<u8> {
        Ok(self.radio_ap(7))
    }
    #[inline]
    pub fn write_radio_prefix1_ap7(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ap(7, _value))
    }
    pub fn read_radio_txaddress_txaddress(&self) -> MemResult<u8> {
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
    pub fn read_radio_rxaddresses_addr0(&self) -> MemResult<u8> {
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
    pub fn read_radio_rxaddresses_addr1(&self) -> MemResult<u8> {
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
    pub fn read_radio_rxaddresses_addr2(&self) -> MemResult<u8> {
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
    pub fn read_radio_rxaddresses_addr3(&self) -> MemResult<u8> {
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
    pub fn read_radio_rxaddresses_addr4(&self) -> MemResult<u8> {
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
    pub fn read_radio_rxaddresses_addr5(&self) -> MemResult<u8> {
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
    pub fn read_radio_rxaddresses_addr6(&self) -> MemResult<u8> {
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
    pub fn read_radio_rxaddresses_addr7(&self) -> MemResult<u8> {
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
    pub fn read_radio_crccnf_skipaddr(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_radio_crccnf_skipaddr(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_radio_crcpoly_crcpoly(
        &self,
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
        &self,
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
    pub fn read_radio_test_constcarrier(&self) -> MemResult<u8> {
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
    pub fn read_radio_test_plllock(&self) -> MemResult<u8> {
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
    pub fn read_radio_rssisample_rssisample(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 127u64;
        todo!()
    }
    pub fn read_radio_state_state(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_radio_datawhiteiv_datawhiteiv(&self) -> MemResult<u8> {
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
        &self,
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
        &self,
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
        &self,
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
    pub fn read_radio_dacnf_ena0(&self) -> MemResult<u8> {
        Ok(self.radio_is_ena(0) as u8)
    }
    #[inline]
    pub fn write_radio_dacnf_ena0(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ena(0, _value != 0))
    }
    #[inline]
    pub fn read_radio_dacnf_ena1(&self) -> MemResult<u8> {
        Ok(self.radio_is_ena(1) as u8)
    }
    #[inline]
    pub fn write_radio_dacnf_ena1(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ena(1, _value != 0))
    }
    #[inline]
    pub fn read_radio_dacnf_ena2(&self) -> MemResult<u8> {
        Ok(self.radio_is_ena(2) as u8)
    }
    #[inline]
    pub fn write_radio_dacnf_ena2(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ena(2, _value != 0))
    }
    #[inline]
    pub fn read_radio_dacnf_ena3(&self) -> MemResult<u8> {
        Ok(self.radio_is_ena(3) as u8)
    }
    #[inline]
    pub fn write_radio_dacnf_ena3(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ena(3, _value != 0))
    }
    #[inline]
    pub fn read_radio_dacnf_ena4(&self) -> MemResult<u8> {
        Ok(self.radio_is_ena(4) as u8)
    }
    #[inline]
    pub fn write_radio_dacnf_ena4(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ena(4, _value != 0))
    }
    #[inline]
    pub fn read_radio_dacnf_ena5(&self) -> MemResult<u8> {
        Ok(self.radio_is_ena(5) as u8)
    }
    #[inline]
    pub fn write_radio_dacnf_ena5(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ena(5, _value != 0))
    }
    #[inline]
    pub fn read_radio_dacnf_ena6(&self) -> MemResult<u8> {
        Ok(self.radio_is_ena(6) as u8)
    }
    #[inline]
    pub fn write_radio_dacnf_ena6(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ena(6, _value != 0))
    }
    #[inline]
    pub fn read_radio_dacnf_ena7(&self) -> MemResult<u8> {
        Ok(self.radio_is_ena(7) as u8)
    }
    #[inline]
    pub fn write_radio_dacnf_ena7(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ena(7, _value != 0))
    }
    #[inline]
    pub fn read_radio_dacnf_txadd0(&self) -> MemResult<u8> {
        Ok(self.radio_is_txadd(0) as u8)
    }
    #[inline]
    pub fn write_radio_dacnf_txadd0(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ena(0, _value != 0))
    }
    #[inline]
    pub fn read_radio_dacnf_txadd1(&self) -> MemResult<u8> {
        Ok(self.radio_is_txadd(1) as u8)
    }
    #[inline]
    pub fn write_radio_dacnf_txadd1(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ena(1, _value != 0))
    }
    #[inline]
    pub fn read_radio_dacnf_txadd2(&self) -> MemResult<u8> {
        Ok(self.radio_is_txadd(2) as u8)
    }
    #[inline]
    pub fn write_radio_dacnf_txadd2(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ena(2, _value != 0))
    }
    #[inline]
    pub fn read_radio_dacnf_txadd3(&self) -> MemResult<u8> {
        Ok(self.radio_is_txadd(3) as u8)
    }
    #[inline]
    pub fn write_radio_dacnf_txadd3(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ena(3, _value != 0))
    }
    #[inline]
    pub fn read_radio_dacnf_txadd4(&self) -> MemResult<u8> {
        Ok(self.radio_is_txadd(4) as u8)
    }
    #[inline]
    pub fn write_radio_dacnf_txadd4(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ena(4, _value != 0))
    }
    #[inline]
    pub fn read_radio_dacnf_txadd5(&self) -> MemResult<u8> {
        Ok(self.radio_is_txadd(5) as u8)
    }
    #[inline]
    pub fn write_radio_dacnf_txadd5(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ena(5, _value != 0))
    }
    #[inline]
    pub fn read_radio_dacnf_txadd6(&self) -> MemResult<u8> {
        Ok(self.radio_is_txadd(6) as u8)
    }
    #[inline]
    pub fn write_radio_dacnf_txadd6(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ena(6, _value != 0))
    }
    #[inline]
    pub fn read_radio_dacnf_txadd7(&self) -> MemResult<u8> {
        Ok(self.radio_is_txadd(7) as u8)
    }
    #[inline]
    pub fn write_radio_dacnf_txadd7(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.radio_set_ena(7, _value != 0))
    }
    pub fn read_radio_override0_override0(
        &self,
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
    pub fn read_radio_override4_enable(&self) -> MemResult<u8> {
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
    pub fn read_uart0_shorts_cts_startrx(&self) -> MemResult<u8> {
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
    pub fn read_uart0_shorts_ncts_stoprx(&self) -> MemResult<u8> {
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
    pub fn read_uart0_intenset_cts(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenset_cts(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenset_ncts(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenset_ncts(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenset_rxdrdy(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenset_rxdrdy(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenset_txdrdy(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenset_txdrdy(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenset_error(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenset_error(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenset_rxto(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenset_rxto(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenclr_cts(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenclr_cts(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenclr_ncts(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenclr_ncts(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenclr_rxdrdy(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenclr_rxdrdy(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenclr_txdrdy(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenclr_txdrdy(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenclr_error(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenclr_error(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_intenclr_rxto(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_intenclr_rxto(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_errorsrc_overrun(&self) -> MemResult<u8> {
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
    pub fn read_uart0_errorsrc_parity(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_errorsrc_parity(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_errorsrc_framing(&self) -> MemResult<u8> {
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
    pub fn read_uart0_errorsrc_break(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_uart0_errorsrc_break(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_enable_enable(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_uart0_enable_enable(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_uart0_pselrts(
        &self,
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
    pub fn read_uart0_config_parity(&self) -> MemResult<u8> {
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
        &self,
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
    pub fn read_spi0twi0_intenset_ready(&self) -> MemResult<u8> {
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
    pub fn read_spi0twi0_intenset_stopped(&self) -> MemResult<u8> {
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
    pub fn read_spi0twi0_intenset_txdsent(&self) -> MemResult<u8> {
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
    pub fn read_spi0twi0_intenset_error(&self) -> MemResult<u8> {
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
    pub fn read_spi0twi0_intenset_bb(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spi0twi0_intenset_bb(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_spi0twi0_intenset_suspended(&self) -> MemResult<u8> {
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
    pub fn read_spi0twi0_intenclr_ready(&self) -> MemResult<u8> {
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
    pub fn read_spi0twi0_intenclr_stopped(&self) -> MemResult<u8> {
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
    pub fn read_spi0twi0_intenclr_txdsent(&self) -> MemResult<u8> {
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
    pub fn read_spi0twi0_intenclr_error(&self) -> MemResult<u8> {
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
    pub fn read_spi0twi0_intenclr_bb(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spi0twi0_intenclr_bb(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_spi0twi0_intenclr_suspended(&self) -> MemResult<u8> {
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
    pub fn read_spi0twi0_enable_enable(&self) -> MemResult<u8> {
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
    pub fn read_spi0twi0_power_power(&self) -> MemResult<u8> {
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
    pub fn read_twi0_shorts_bb_suspend(&self) -> MemResult<u8> {
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
    pub fn read_twi0_shorts_bb_stop(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_twi0_shorts_bb_stop(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_twi0_errorsrc_overrun(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_twi0_errorsrc_overrun(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_twi0_errorsrc_anack(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_twi0_errorsrc_anack(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_twi0_errorsrc_dnack(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_twi0_errorsrc_dnack(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_twi0_address_address(&self) -> MemResult<u8> {
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
        &self,
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
        &self,
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
        &self,
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
    pub fn read_spis1_shorts_end_acquire(&self) -> MemResult<u8> {
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
    pub fn read_spis1_intenset_end(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spis1_intenset_end(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_spis1_intenset_endrx(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spis1_intenset_endrx(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_spis1_intenset_acquired(&self) -> MemResult<u8> {
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
    pub fn read_spis1_intenclr_end(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spis1_intenclr_end(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_spis1_intenclr_endrx(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spis1_intenclr_endrx(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_spis1_intenclr_acquired(&self) -> MemResult<u8> {
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
    pub fn read_spis1_semstat_semstat(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 1u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn read_spis1_status_overread(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spis1_status_overread(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_spis1_status_overflow(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_spis1_status_overflow(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_spis1_enable_enable(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_spis1_enable_enable(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_spis1_pselsck(
        &self,
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
    pub fn read_spis1_amountrx_amountrx(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn read_spis1_txdptr(
        &self,
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
    pub fn read_spis1_amounttx_amounttx(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 255u64;
        todo!()
    }
    pub fn read_spis1_config_order(&self) -> MemResult<u8> {
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
        &self,
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
        &self,
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
    pub fn read_gpiote_intenset_in0(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpiote_intenset_in0(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_intenset_in1(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpiote_intenset_in1(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_intenset_in2(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpiote_intenset_in2(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_intenset_in3(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpiote_intenset_in3(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_intenset_port(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpiote_intenset_port(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_intenclr_in0(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpiote_intenclr_in0(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_intenclr_in1(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpiote_intenclr_in1(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_intenclr_in2(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpiote_intenclr_in2(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_intenclr_in3(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpiote_intenclr_in3(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_intenclr_port(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_gpiote_intenclr_port(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_gpiote_confign_mode(&self, _dim: usize) -> MemResult<u8> {
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
    pub fn read_gpiote_confign_psel(&self, _dim: usize) -> MemResult<u8> {
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
    pub fn read_gpiote_confign_polarity(&self, _dim: usize) -> MemResult<u8> {
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
    pub fn read_gpiote_confign_outinit(&self, _dim: usize) -> MemResult<u8> {
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
    pub fn read_gpiote_power_power(&self) -> MemResult<u8> {
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
        &self,
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
    pub fn read_adc_config_extrefsel(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 3u64;
        todo!()
    }
    pub fn write_adc_config_extrefsel(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_adc_result_result(
        &self,
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
        &self,
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
    pub fn read_timer0_shorts_compare0_clear(&self) -> MemResult<u8> {
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
    pub fn read_timer0_shorts_compare1_clear(&self) -> MemResult<u8> {
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
    pub fn read_timer0_shorts_compare2_clear(&self) -> MemResult<u8> {
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
    pub fn read_timer0_shorts_compare3_clear(&self) -> MemResult<u8> {
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
    pub fn read_timer0_shorts_compare0_stop(&self) -> MemResult<u8> {
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
    pub fn read_timer0_shorts_compare1_stop(&self) -> MemResult<u8> {
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
    pub fn read_timer0_shorts_compare2_stop(&self) -> MemResult<u8> {
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
    pub fn read_timer0_shorts_compare3_stop(&self) -> MemResult<u8> {
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
    pub fn read_timer0_intenset_compare0(&self) -> MemResult<u8> {
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
    pub fn read_timer0_intenset_compare1(&self) -> MemResult<u8> {
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
    pub fn read_timer0_intenset_compare2(&self) -> MemResult<u8> {
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
    pub fn read_timer0_intenset_compare3(&self) -> MemResult<u8> {
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
    pub fn read_timer0_intenclr_compare0(&self) -> MemResult<u8> {
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
    pub fn read_timer0_intenclr_compare1(&self) -> MemResult<u8> {
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
    pub fn read_timer0_intenclr_compare2(&self) -> MemResult<u8> {
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
    pub fn read_timer0_intenclr_compare3(&self) -> MemResult<u8> {
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
    pub fn read_timer0_bitmode_bitmode(&self) -> MemResult<u8> {
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
    pub fn read_timer0_prescaler_prescaler(&self) -> MemResult<u8> {
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
        &self,
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
    pub fn read_timer0_power_power(&self) -> MemResult<u8> {
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
        &self,
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
        &self,
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
        &self,
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
    pub fn read_rtc0_intenset_tick(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_intenset_tick(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_intenset_ovrflw(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_intenset_ovrflw(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_intenset_compare0(&self) -> MemResult<u8> {
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
    pub fn read_rtc0_intenset_compare1(&self) -> MemResult<u8> {
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
    pub fn read_rtc0_intenset_compare2(&self) -> MemResult<u8> {
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
    pub fn read_rtc0_intenset_compare3(&self) -> MemResult<u8> {
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
    pub fn read_rtc0_intenclr_tick(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_intenclr_tick(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_intenclr_ovrflw(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_intenclr_ovrflw(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_intenclr_compare0(&self) -> MemResult<u8> {
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
    pub fn read_rtc0_intenclr_compare1(&self) -> MemResult<u8> {
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
    pub fn read_rtc0_intenclr_compare2(&self) -> MemResult<u8> {
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
    pub fn read_rtc0_intenclr_compare3(&self) -> MemResult<u8> {
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
    pub fn read_rtc0_evten_compare0(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evten_compare0(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evten_compare1(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evten_compare1(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evten_compare2(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evten_compare2(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evten_compare3(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evten_compare3(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evtenset_tick(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evtenset_tick(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evtenset_ovrflw(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evtenset_ovrflw(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evtenset_compare0(&self) -> MemResult<u8> {
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
    pub fn read_rtc0_evtenset_compare1(&self) -> MemResult<u8> {
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
    pub fn read_rtc0_evtenset_compare2(&self) -> MemResult<u8> {
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
    pub fn read_rtc0_evtenset_compare3(&self) -> MemResult<u8> {
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
    pub fn read_rtc0_evtenclr_tick(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evtenclr_tick(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evtenclr_ovrflw(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rtc0_evtenclr_ovrflw(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_rtc0_evtenclr_compare0(&self) -> MemResult<u8> {
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
    pub fn read_rtc0_evtenclr_compare1(&self) -> MemResult<u8> {
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
    pub fn read_rtc0_evtenclr_compare2(&self) -> MemResult<u8> {
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
    pub fn read_rtc0_evtenclr_compare3(&self) -> MemResult<u8> {
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
    pub fn read_temp_intenset_datardy(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_temp_intenset_datardy(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_temp_intenclr_datardy(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_temp_intenclr_datardy(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_temp_temp(
        &self,
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
        &self,
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
    pub fn read_rng_shorts_valrdy_stop(&self) -> MemResult<u8> {
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
    pub fn read_rng_intenset_valrdy(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_rng_intenset_valrdy(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_rng_intenclr_valrdy(&self) -> MemResult<u8> {
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
        &self,
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
        &self,
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
    pub fn read_ecb_intenset_endecb(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ecb_intenset_endecb(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_ecb_intenset_errorecb(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ecb_intenset_errorecb(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_ecb_intenclr_endecb(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ecb_intenclr_endecb(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_ecb_intenclr_errorecb(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_ecb_intenclr_errorecb(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_ecb_ecbdataptr(
        &self,
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
        &self,
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
        &self,
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
        &self,
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
    pub fn read_aarccm_intenset_end(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_aarccm_intenset_end(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_aarccm_intenset_resolved(&self) -> MemResult<u8> {
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
    pub fn read_aarccm_intenset_notresolved(&self) -> MemResult<u8> {
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
    pub fn read_aarccm_intenclr_end(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_aarccm_intenclr_end(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_aarccm_intenclr_resolved(&self) -> MemResult<u8> {
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
    pub fn read_aarccm_intenclr_notresolved(&self) -> MemResult<u8> {
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
    pub fn read_aarccm_status_status(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_aarccm_enable_enable(&self) -> MemResult<u8> {
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
        &self,
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
        &self,
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
        &self,
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
    pub fn read_aarccm_power_power(&self) -> MemResult<u8> {
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
    pub fn read_ccm_shorts_endksgen_crypt(&self) -> MemResult<u8> {
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
        &self,
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
        &self,
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
    pub fn read_wdt_intenset_timeout(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_wdt_intenset_timeout(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_wdt_intenclr_timeout(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_wdt_intenclr_timeout(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_wdt_runstatus_runstatus(&self) -> MemResult<u8> {
        Ok(self.wdt.run_status() as u8)
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
    pub fn read_qdec_shorts_reportrdy_readclracc(&self) -> MemResult<u8> {
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
    pub fn read_qdec_shorts_samplerdy_stop(&self) -> MemResult<u8> {
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
    pub fn read_qdec_intenset_samplerdy(&self) -> MemResult<u8> {
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
    pub fn read_qdec_intenset_reportrdy(&self) -> MemResult<u8> {
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
    pub fn read_qdec_intenset_accof(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_qdec_intenset_accof(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_qdec_intenclr_samplerdy(&self) -> MemResult<u8> {
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
    pub fn read_qdec_intenclr_reportrdy(&self) -> MemResult<u8> {
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
    pub fn read_qdec_intenclr_accof(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_qdec_intenclr_accof(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_qdec_enable_enable(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_qdec_enable_enable(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_qdec_ledpol_ledpol(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_qdec_ledpol_ledpol(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_qdec_sampleper_sampleper(&self) -> MemResult<u8> {
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
        &self,
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
    pub fn read_qdec_reportper_reportper(&self) -> MemResult<u8> {
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
    pub fn read_qdec_accdbl_accdbl(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 15u64;
        todo!()
    }
    pub fn read_qdec_accdblread_accdblread(&self) -> MemResult<u8> {
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
    pub fn read_lpcomp_shorts_ready_sample(&self) -> MemResult<u8> {
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
    pub fn read_lpcomp_shorts_ready_stop(&self) -> MemResult<u8> {
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
    pub fn read_lpcomp_shorts_down_stop(&self) -> MemResult<u8> {
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
    pub fn read_lpcomp_shorts_up_stop(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_shorts_up_stop(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_shorts_cross_stop(&self) -> MemResult<u8> {
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
    pub fn read_lpcomp_intenset_ready(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_intenset_ready(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_intenset_down(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_intenset_down(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_intenset_up(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_intenset_up(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_intenset_cross(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_intenset_cross(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_intenclr_ready(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_intenclr_ready(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_intenclr_down(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_intenclr_down(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_intenclr_up(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_intenclr_up(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_intenclr_cross(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_intenclr_cross(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_result_result(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_lpcomp_enable_enable(&self) -> MemResult<u8> {
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
    pub fn read_lpcomp_refsel_refsel(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 7u64;
        todo!()
    }
    pub fn write_lpcomp_refsel_refsel(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_lpcomp_extrefsel_extrefsel(&self) -> MemResult<u8> {
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
    pub fn read_lpcomp_anadetect_anadetect(&self) -> MemResult<u8> {
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
    pub fn read_lpcomp_power_power(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn write_lpcomp_power_power(&mut self, _value: u8) -> MemResult<()> {
        todo!()
    }
    pub fn read_swi_unused(
        &self,
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
        &self,
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
    pub fn read_nvmc_eraseall_eraseall(&self) -> MemResult<u8> {
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
        &self,
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
    pub fn read_nvmc_eraseuicr_eraseuicr(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chen_ch0(&self) -> MemResult<u8> {
        Ok(self.ppi_is_on(0) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch0(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(0, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch1(&self) -> MemResult<u8> {
        Ok(self.ppi_is_on(1) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch1(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(1, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch2(&self) -> MemResult<u8> {
        Ok(self.ppi_is_on(2) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch2(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(2, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch3(&self) -> MemResult<u8> {
        Ok(self.ppi_is_on(3) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch3(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(3, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch4(&self) -> MemResult<u8> {
        Ok(self.ppi_is_on(4) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch4(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(4, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch5(&self) -> MemResult<u8> {
        Ok(self.ppi_is_on(5) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch5(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(5, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch6(&self) -> MemResult<u8> {
        Ok(self.ppi_is_on(6) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch6(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(6, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch7(&self) -> MemResult<u8> {
        Ok(self.ppi_is_on(7) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch7(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(7, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch8(&self) -> MemResult<u8> {
        Ok(self.ppi_is_on(8) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch8(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(8, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch9(&self) -> MemResult<u8> {
        Ok(self.ppi_is_on(9) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch9(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(9, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch10(&self) -> MemResult<u8> {
        Ok(self.ppi_is_on(10) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch10(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(10, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch11(&self) -> MemResult<u8> {
        Ok(self.ppi_is_on(11) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch11(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(11, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch12(&self) -> MemResult<u8> {
        Ok(self.ppi_is_on(12) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch12(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(12, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch13(&self) -> MemResult<u8> {
        Ok(self.ppi_is_on(13) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch13(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(13, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch14(&self) -> MemResult<u8> {
        Ok(self.ppi_is_on(14) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch14(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(14, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch15(&self) -> MemResult<u8> {
        Ok(self.ppi_is_on(15) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch15(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(15, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch20(&self) -> MemResult<u8> {
        Ok(self.ppi_is_on(20) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch20(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(20, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch21(&self) -> MemResult<u8> {
        Ok(self.ppi_is_on(21) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch21(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(21, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch22(&self) -> MemResult<u8> {
        Ok(self.ppi_is_on(22) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch22(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(22, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch23(&self) -> MemResult<u8> {
        Ok(self.ppi_is_on(23) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch23(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(23, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch24(&self) -> MemResult<u8> {
        Ok(self.ppi_is_on(24) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch24(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(24, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch25(&self) -> MemResult<u8> {
        Ok(self.ppi_is_on(25) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch25(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(25, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch26(&self) -> MemResult<u8> {
        Ok(self.ppi_is_on(26) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch26(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(26, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch27(&self) -> MemResult<u8> {
        Ok(self.ppi_is_on(27) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch27(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(27, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch28(&self) -> MemResult<u8> {
        Ok(self.ppi_is_on(28) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch28(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(28, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch29(&self) -> MemResult<u8> {
        Ok(self.ppi_is_on(29) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch29(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(29, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch30(&self) -> MemResult<u8> {
        Ok(self.ppi_is_on(30) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch30(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(30, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chen_ch31(&self) -> MemResult<u8> {
        Ok(self.ppi_is_on(31) as u8)
    }
    #[inline]
    pub fn write_ppi_chen_ch31(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.ppi_set_on(31, _value != 0))
    }
    #[inline]
    pub fn read_ppi_chenset_ch0(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenset_ch1(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenset_ch2(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenset_ch3(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenset_ch4(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenset_ch5(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenset_ch6(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenset_ch7(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenset_ch8(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenset_ch9(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenset_ch10(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenset_ch11(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenset_ch12(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenset_ch13(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenset_ch14(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenset_ch15(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenset_ch20(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenset_ch21(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenset_ch22(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenset_ch23(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenset_ch24(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenset_ch25(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenset_ch26(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenset_ch27(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenset_ch28(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenset_ch29(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenset_ch30(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenset_ch31(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenclr_ch0(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenclr_ch1(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenclr_ch2(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenclr_ch3(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenclr_ch4(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenclr_ch5(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenclr_ch6(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenclr_ch7(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenclr_ch8(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenclr_ch9(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenclr_ch10(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenclr_ch11(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenclr_ch12(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenclr_ch13(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenclr_ch14(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenclr_ch15(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenclr_ch20(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenclr_ch21(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenclr_ch22(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenclr_ch23(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenclr_ch24(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenclr_ch25(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenclr_ch26(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenclr_ch27(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenclr_ch28(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenclr_ch29(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenclr_ch30(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chenclr_ch31(&self) -> MemResult<u8> {
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
    pub fn read_ppi_chgn_ch0(&self, _dim: usize) -> MemResult<u8> {
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
    pub fn read_ppi_chgn_ch1(&self, _dim: usize) -> MemResult<u8> {
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
    pub fn read_ppi_chgn_ch2(&self, _dim: usize) -> MemResult<u8> {
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
    pub fn read_ppi_chgn_ch3(&self, _dim: usize) -> MemResult<u8> {
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
    pub fn read_ppi_chgn_ch4(&self, _dim: usize) -> MemResult<u8> {
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
    pub fn read_ppi_chgn_ch5(&self, _dim: usize) -> MemResult<u8> {
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
    pub fn read_ppi_chgn_ch6(&self, _dim: usize) -> MemResult<u8> {
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
    pub fn read_ppi_chgn_ch7(&self, _dim: usize) -> MemResult<u8> {
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
    pub fn read_ppi_chgn_ch8(&self, _dim: usize) -> MemResult<u8> {
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
    pub fn read_ppi_chgn_ch9(&self, _dim: usize) -> MemResult<u8> {
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
    pub fn read_ppi_chgn_ch10(&self, _dim: usize) -> MemResult<u8> {
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
    pub fn read_ppi_chgn_ch11(&self, _dim: usize) -> MemResult<u8> {
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
    pub fn read_ppi_chgn_ch12(&self, _dim: usize) -> MemResult<u8> {
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
    pub fn read_ppi_chgn_ch13(&self, _dim: usize) -> MemResult<u8> {
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
    pub fn read_ppi_chgn_ch14(&self, _dim: usize) -> MemResult<u8> {
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
    pub fn read_ppi_chgn_ch15(&self, _dim: usize) -> MemResult<u8> {
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
    pub fn read_ppi_chgn_ch20(&self, _dim: usize) -> MemResult<u8> {
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
    pub fn read_ppi_chgn_ch21(&self, _dim: usize) -> MemResult<u8> {
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
    pub fn read_ppi_chgn_ch22(&self, _dim: usize) -> MemResult<u8> {
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
    pub fn read_ppi_chgn_ch23(&self, _dim: usize) -> MemResult<u8> {
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
    pub fn read_ppi_chgn_ch24(&self, _dim: usize) -> MemResult<u8> {
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
    pub fn read_ppi_chgn_ch25(&self, _dim: usize) -> MemResult<u8> {
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
    pub fn read_ppi_chgn_ch26(&self, _dim: usize) -> MemResult<u8> {
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
    pub fn read_ppi_chgn_ch27(&self, _dim: usize) -> MemResult<u8> {
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
    pub fn read_ppi_chgn_ch28(&self, _dim: usize) -> MemResult<u8> {
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
    pub fn read_ppi_chgn_ch29(&self, _dim: usize) -> MemResult<u8> {
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
    pub fn read_ppi_chgn_ch30(&self, _dim: usize) -> MemResult<u8> {
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
    pub fn read_ppi_chgn_ch31(&self, _dim: usize) -> MemResult<u8> {
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
    pub fn read_ficr_deviceaddrtype_deviceaddrtype(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_ficr_deviceaddrn(
        &self,
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
    pub fn read_ficr_overrideen_nrf_1mbit(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_ficr_overrideen_ble_1mbit(&self) -> MemResult<u8> {
        const _RESET_VALUE: u64 = 0u64;
        const _RESET_MASK: u64 = 1u64;
        todo!()
    }
    pub fn read_ficr_nrf_1mbitn(
        &self,
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
        &self,
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
        &self,
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
    pub fn read_uicr_xtalfreq_xtalfreq(&self) -> MemResult<u8> {
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
    pub fn read_gpio_out_pin0(&self) -> MemResult<u8> {
        Ok(self.gpio[0].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin0(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[0].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin1(&self) -> MemResult<u8> {
        Ok(self.gpio[1].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin1(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[1].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin2(&self) -> MemResult<u8> {
        Ok(self.gpio[2].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin2(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[2].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin3(&self) -> MemResult<u8> {
        Ok(self.gpio[3].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin3(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[3].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin4(&self) -> MemResult<u8> {
        Ok(self.gpio[4].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin4(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[4].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin5(&self) -> MemResult<u8> {
        Ok(self.gpio[5].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin5(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[5].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin6(&self) -> MemResult<u8> {
        Ok(self.gpio[6].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin6(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[6].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin7(&self) -> MemResult<u8> {
        Ok(self.gpio[7].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin7(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[7].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin8(&self) -> MemResult<u8> {
        Ok(self.gpio[8].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin8(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[8].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin9(&self) -> MemResult<u8> {
        Ok(self.gpio[9].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin9(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[9].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin10(&self) -> MemResult<u8> {
        Ok(self.gpio[10].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin10(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[10].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin11(&self) -> MemResult<u8> {
        Ok(self.gpio[11].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin11(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[11].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin12(&self) -> MemResult<u8> {
        Ok(self.gpio[12].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin12(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[12].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin13(&self) -> MemResult<u8> {
        Ok(self.gpio[13].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin13(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[13].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin14(&self) -> MemResult<u8> {
        Ok(self.gpio[14].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin14(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[14].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin15(&self) -> MemResult<u8> {
        Ok(self.gpio[15].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin15(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[15].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin16(&self) -> MemResult<u8> {
        Ok(self.gpio[16].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin16(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[16].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin17(&self) -> MemResult<u8> {
        Ok(self.gpio[17].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin17(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[17].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin18(&self) -> MemResult<u8> {
        Ok(self.gpio[18].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin18(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[18].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin19(&self) -> MemResult<u8> {
        Ok(self.gpio[19].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin19(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[19].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin20(&self) -> MemResult<u8> {
        Ok(self.gpio[20].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin20(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[20].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin21(&self) -> MemResult<u8> {
        Ok(self.gpio[21].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin21(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[21].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin22(&self) -> MemResult<u8> {
        Ok(self.gpio[22].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin22(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[22].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin23(&self) -> MemResult<u8> {
        Ok(self.gpio[23].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin23(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[23].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin24(&self) -> MemResult<u8> {
        Ok(self.gpio[24].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin24(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[24].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin25(&self) -> MemResult<u8> {
        Ok(self.gpio[25].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin25(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[25].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin26(&self) -> MemResult<u8> {
        Ok(self.gpio[26].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin26(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[26].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin27(&self) -> MemResult<u8> {
        Ok(self.gpio[27].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin27(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[27].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin28(&self) -> MemResult<u8> {
        Ok(self.gpio[28].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin28(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[28].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin29(&self) -> MemResult<u8> {
        Ok(self.gpio[29].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin29(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[29].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin30(&self) -> MemResult<u8> {
        Ok(self.gpio[30].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin30(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[30].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_out_pin31(&self) -> MemResult<u8> {
        Ok(self.gpio[31].is_out_high() as u8)
    }
    #[inline]
    pub fn write_gpio_out_pin31(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[31].set_out_high(_value != 0))
    }
    #[inline]
    pub fn read_gpio_outset_pin0(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outset_pin1(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outset_pin2(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outset_pin3(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outset_pin4(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outset_pin5(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outset_pin6(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outset_pin7(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outset_pin8(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outset_pin9(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outset_pin10(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outset_pin11(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outset_pin12(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outset_pin13(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outset_pin14(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outset_pin15(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outset_pin16(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outset_pin17(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outset_pin18(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outset_pin19(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outset_pin20(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outset_pin21(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outset_pin22(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outset_pin23(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outset_pin24(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outset_pin25(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outset_pin26(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outset_pin27(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outset_pin28(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outset_pin29(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outset_pin30(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outset_pin31(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outclr_pin0(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outclr_pin1(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outclr_pin2(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outclr_pin3(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outclr_pin4(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outclr_pin5(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outclr_pin6(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outclr_pin7(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outclr_pin8(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outclr_pin9(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outclr_pin10(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outclr_pin11(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outclr_pin12(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outclr_pin13(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outclr_pin14(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outclr_pin15(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outclr_pin16(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outclr_pin17(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outclr_pin18(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outclr_pin19(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outclr_pin20(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outclr_pin21(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outclr_pin22(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outclr_pin23(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outclr_pin24(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outclr_pin25(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outclr_pin26(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outclr_pin27(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outclr_pin28(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outclr_pin29(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outclr_pin30(&self) -> MemResult<u8> {
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
    pub fn read_gpio_outclr_pin31(&self) -> MemResult<u8> {
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
    pub fn read_gpio_in_pin0(&self) -> MemResult<u8> {
        Ok(self.gpio[0].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin1(&self) -> MemResult<u8> {
        Ok(self.gpio[1].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin2(&self) -> MemResult<u8> {
        Ok(self.gpio[2].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin3(&self) -> MemResult<u8> {
        Ok(self.gpio[3].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin4(&self) -> MemResult<u8> {
        Ok(self.gpio[4].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin5(&self) -> MemResult<u8> {
        Ok(self.gpio[5].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin6(&self) -> MemResult<u8> {
        Ok(self.gpio[6].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin7(&self) -> MemResult<u8> {
        Ok(self.gpio[7].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin8(&self) -> MemResult<u8> {
        Ok(self.gpio[8].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin9(&self) -> MemResult<u8> {
        Ok(self.gpio[9].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin10(&self) -> MemResult<u8> {
        Ok(self.gpio[10].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin11(&self) -> MemResult<u8> {
        Ok(self.gpio[11].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin12(&self) -> MemResult<u8> {
        Ok(self.gpio[12].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin13(&self) -> MemResult<u8> {
        Ok(self.gpio[13].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin14(&self) -> MemResult<u8> {
        Ok(self.gpio[14].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin15(&self) -> MemResult<u8> {
        Ok(self.gpio[15].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin16(&self) -> MemResult<u8> {
        Ok(self.gpio[16].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin17(&self) -> MemResult<u8> {
        Ok(self.gpio[17].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin18(&self) -> MemResult<u8> {
        Ok(self.gpio[18].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin19(&self) -> MemResult<u8> {
        Ok(self.gpio[19].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin20(&self) -> MemResult<u8> {
        Ok(self.gpio[20].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin21(&self) -> MemResult<u8> {
        Ok(self.gpio[21].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin22(&self) -> MemResult<u8> {
        Ok(self.gpio[22].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin23(&self) -> MemResult<u8> {
        Ok(self.gpio[23].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin24(&self) -> MemResult<u8> {
        Ok(self.gpio[24].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin25(&self) -> MemResult<u8> {
        Ok(self.gpio[25].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin26(&self) -> MemResult<u8> {
        Ok(self.gpio[26].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin27(&self) -> MemResult<u8> {
        Ok(self.gpio[27].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin28(&self) -> MemResult<u8> {
        Ok(self.gpio[28].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin29(&self) -> MemResult<u8> {
        Ok(self.gpio[29].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin30(&self) -> MemResult<u8> {
        Ok(self.gpio[30].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_in_pin31(&self) -> MemResult<u8> {
        Ok(self.gpio[31].read_input() as u8)
    }
    #[inline]
    pub fn read_gpio_dir_pin0(&self) -> MemResult<u8> {
        Ok(self.gpio[0].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin0(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[0].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin1(&self) -> MemResult<u8> {
        Ok(self.gpio[1].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin1(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[1].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin2(&self) -> MemResult<u8> {
        Ok(self.gpio[2].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin2(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[2].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin3(&self) -> MemResult<u8> {
        Ok(self.gpio[3].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin3(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[3].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin4(&self) -> MemResult<u8> {
        Ok(self.gpio[4].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin4(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[4].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin5(&self) -> MemResult<u8> {
        Ok(self.gpio[5].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin5(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[5].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin6(&self) -> MemResult<u8> {
        Ok(self.gpio[6].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin6(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[6].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin7(&self) -> MemResult<u8> {
        Ok(self.gpio[7].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin7(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[7].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin8(&self) -> MemResult<u8> {
        Ok(self.gpio[8].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin8(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[8].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin9(&self) -> MemResult<u8> {
        Ok(self.gpio[9].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin9(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[9].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin10(&self) -> MemResult<u8> {
        Ok(self.gpio[10].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin10(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[10].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin11(&self) -> MemResult<u8> {
        Ok(self.gpio[11].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin11(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[11].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin12(&self) -> MemResult<u8> {
        Ok(self.gpio[12].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin12(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[12].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin13(&self) -> MemResult<u8> {
        Ok(self.gpio[13].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin13(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[13].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin14(&self) -> MemResult<u8> {
        Ok(self.gpio[14].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin14(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[14].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin15(&self) -> MemResult<u8> {
        Ok(self.gpio[15].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin15(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[15].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin16(&self) -> MemResult<u8> {
        Ok(self.gpio[16].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin16(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[16].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin17(&self) -> MemResult<u8> {
        Ok(self.gpio[17].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin17(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[17].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin18(&self) -> MemResult<u8> {
        Ok(self.gpio[18].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin18(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[18].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin19(&self) -> MemResult<u8> {
        Ok(self.gpio[19].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin19(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[19].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin20(&self) -> MemResult<u8> {
        Ok(self.gpio[20].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin20(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[20].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin21(&self) -> MemResult<u8> {
        Ok(self.gpio[21].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin21(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[21].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin22(&self) -> MemResult<u8> {
        Ok(self.gpio[22].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin22(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[22].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin23(&self) -> MemResult<u8> {
        Ok(self.gpio[23].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin23(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[23].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin24(&self) -> MemResult<u8> {
        Ok(self.gpio[24].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin24(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[24].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin25(&self) -> MemResult<u8> {
        Ok(self.gpio[25].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin25(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[25].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin26(&self) -> MemResult<u8> {
        Ok(self.gpio[26].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin26(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[26].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin27(&self) -> MemResult<u8> {
        Ok(self.gpio[27].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin27(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[27].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin28(&self) -> MemResult<u8> {
        Ok(self.gpio[28].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin28(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[28].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin29(&self) -> MemResult<u8> {
        Ok(self.gpio[29].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin29(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[29].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin30(&self) -> MemResult<u8> {
        Ok(self.gpio[30].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin30(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[30].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dir_pin31(&self) -> MemResult<u8> {
        Ok(self.gpio[31].get_direction() as u8)
    }
    #[inline]
    pub fn write_gpio_dir_pin31(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.gpio[31].set_direction(_value != 0))
    }
    #[inline]
    pub fn read_gpio_dirset_pin0(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirset_pin1(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirset_pin2(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirset_pin3(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirset_pin4(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirset_pin5(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirset_pin6(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirset_pin7(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirset_pin8(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirset_pin9(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirset_pin10(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirset_pin11(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirset_pin12(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirset_pin13(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirset_pin14(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirset_pin15(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirset_pin16(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirset_pin17(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirset_pin18(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirset_pin19(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirset_pin20(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirset_pin21(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirset_pin22(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirset_pin23(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirset_pin24(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirset_pin25(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirset_pin26(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirset_pin27(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirset_pin28(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirset_pin29(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirset_pin30(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirset_pin31(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirclr_pin0(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirclr_pin1(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirclr_pin2(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirclr_pin3(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirclr_pin4(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirclr_pin5(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirclr_pin6(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirclr_pin7(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirclr_pin8(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirclr_pin9(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirclr_pin10(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirclr_pin11(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirclr_pin12(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirclr_pin13(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirclr_pin14(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirclr_pin15(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirclr_pin16(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirclr_pin17(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirclr_pin18(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirclr_pin19(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirclr_pin20(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirclr_pin21(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirclr_pin22(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirclr_pin23(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirclr_pin24(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirclr_pin25(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirclr_pin26(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirclr_pin27(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirclr_pin28(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirclr_pin29(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirclr_pin30(&self) -> MemResult<u8> {
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
    pub fn read_gpio_dirclr_pin31(&self) -> MemResult<u8> {
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
    pub fn read_gpio_pin_cnfn_dir(&self, _dim: usize) -> MemResult<u8> {
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
    pub fn read_gpio_pin_cnfn_input(&self, _dim: usize) -> MemResult<u8> {
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
    pub fn read_gpio_pin_cnfn_pull(&self, _dim: usize) -> MemResult<u8> {
        Ok(self.gpio[_dim].get_pull() as u8)
    }
    #[inline]
    pub fn write_gpio_pin_cnfn_pull(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.gpio[_dim]
            .set_pull(_value.try_into().map_err(|_| MemError::WriteViolation)?))
    }
    #[inline]
    pub fn read_gpio_pin_cnfn_drive(&self, _dim: usize) -> MemResult<u8> {
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
    pub fn read_gpio_pin_cnfn_sense(&self, _dim: usize) -> MemResult<u8> {
        Ok(self.gpio[_dim].get_sense() as u8)
    }
    #[inline]
    pub fn write_gpio_pin_cnfn_sense(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.gpio[_dim].set_sense(
            _value.try_into().map_err(|_| MemError::WriteViolation)?,
        ))
    }
}
