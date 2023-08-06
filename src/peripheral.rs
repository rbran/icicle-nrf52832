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
mod ram;
use ram::*;
mod mpu;
use mpu::*;
mod radio;
use radio::*;
mod ppi;
use ppi::*;

pub struct Peripherals {
    pub gpio: [Gpio; 32],
    pub interrupts: Interrupts,
    pub clock: Clock,
    pub wdt: Wdt,
    pub control: Control,
    pub ram: [RamBlock; 4],
    pub mpu: [MpuRegion; 64],
    pub radio: Radio,
    pub ppi: PPI,
}

impl Default for Peripherals {
    fn default() -> Self {
        Self {
            gpio: Default::default(),
            interrupts: Default::default(),
            clock: Default::default(),
            wdt: Default::default(),
            control: Default::default(),
            ram: Default::default(),
            mpu: [MpuRegion::default(); 64],
            radio: Radio::default(),
            ppi: PPI::default(),
        }
    }
}

impl Peripherals {
    #[doc = "Control ACTLR DISOOFP: Disables floating point instructions completing out of order with respect to integer instructions"]
    #[inline]
    pub(crate) fn read_control_actlr_disoofp(&self) -> MemResult<bool> {
        todo!("read Control ACTLR DISOOFP reset value false")
    }
    #[doc = "Control ACTLR DISOOFP: Disables floating point instructions completing out of order with respect to integer instructions"]
    #[inline]
    pub(crate) fn write_control_actlr_disoofp(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control ACTLR DISOOFP reset value false")
    }
    #[doc = "Control ACTLR DISFPCA: Disable automatic update of CONTROL.FPCA"]
    #[inline]
    pub(crate) fn read_control_actlr_disfpca(&self) -> MemResult<bool> {
        todo!("read Control ACTLR DISFPCA reset value false")
    }
    #[doc = "Control ACTLR DISFPCA: Disable automatic update of CONTROL.FPCA"]
    #[inline]
    pub(crate) fn write_control_actlr_disfpca(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control ACTLR DISFPCA reset value false")
    }
    #[doc = "Control ACTLR DISFOLD: Disables folding of IT instructions"]
    #[inline]
    pub(crate) fn read_control_actlr_disfold(&self) -> MemResult<bool> {
        todo!("read Control ACTLR DISFOLD reset value false")
    }
    #[doc = "Control ACTLR DISFOLD: Disables folding of IT instructions"]
    #[inline]
    pub(crate) fn write_control_actlr_disfold(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control ACTLR DISFOLD reset value false")
    }
    #[doc = "Control ACTLR DISDEFWBUF: Disables write buffer use during default memory map accesses"]
    #[inline]
    pub(crate) fn read_control_actlr_disdefwbuf(&self) -> MemResult<bool> {
        todo!("read Control ACTLR DISDEFWBUF reset value false")
    }
    #[doc = "Control ACTLR DISDEFWBUF: Disables write buffer use during default memory map accesses"]
    #[inline]
    pub(crate) fn write_control_actlr_disdefwbuf(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control ACTLR DISDEFWBUF reset value false")
    }
    #[doc = "Control ACTLR DISMCYCINT: Disables interruption of multi-cycle instructions"]
    #[inline]
    pub(crate) fn read_control_actlr_dismcycint(&self) -> MemResult<bool> {
        todo!("read Control ACTLR DISMCYCINT reset value false")
    }
    #[doc = "Control ACTLR DISMCYCINT: Disables interruption of multi-cycle instructions"]
    #[inline]
    pub(crate) fn write_control_actlr_dismcycint(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control ACTLR DISMCYCINT reset value false")
    }
    #[doc = "Control ICSR NMIPENDSET: Activates an NMI exception or reads back the current state"]
    #[inline]
    pub(crate) fn read_control_icsr_nmipendset(&self) -> MemResult<bool> {
        todo!("read Control ICSR NMIPENDSET reset value false")
    }
    #[doc = "Control ICSR NMIPENDSET: Activates an NMI exception or reads back the current state"]
    #[inline]
    pub(crate) fn write_control_icsr_nmipendset(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control ICSR NMIPENDSET reset value false")
    }
    #[doc = "Control ICSR PENDSVSET: Sets a pending PendSV interrupt or reads back the current state"]
    #[inline]
    pub(crate) fn read_control_icsr_pendsvset(&self) -> MemResult<bool> {
        todo!("read Control ICSR PENDSVSET reset value false")
    }
    #[doc = "Control ICSR PENDSVSET: Sets a pending PendSV interrupt or reads back the current state"]
    #[inline]
    pub(crate) fn write_control_icsr_pendsvset(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control ICSR PENDSVSET reset value false")
    }
    #[doc = "Control ICSR PENDSVCLR: Clears a pending PendSV interrupt"]
    #[inline]
    pub(crate) fn read_control_icsr_pendsvclr(&self) -> MemResult<bool> {
        todo!("read Control ICSR PENDSVCLR reset value false")
    }
    #[doc = "Control ICSR PENDSVCLR: Clears a pending PendSV interrupt"]
    #[inline]
    pub(crate) fn write_control_icsr_pendsvclr(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control ICSR PENDSVCLR reset value false")
    }
    #[doc = "Control ICSR PENDSTSET: Sets a pending SysTick or reads back the current state"]
    #[inline]
    pub(crate) fn read_control_icsr_pendstset(&self) -> MemResult<bool> {
        todo!("read Control ICSR PENDSTSET reset value false")
    }
    #[doc = "Control ICSR PENDSTSET: Sets a pending SysTick or reads back the current state"]
    #[inline]
    pub(crate) fn write_control_icsr_pendstset(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control ICSR PENDSTSET reset value false")
    }
    #[doc = "Control ICSR PENDSTCLR: Clears a pending SysTick, whether set here or by the timer hardware"]
    #[inline]
    pub(crate) fn read_control_icsr_pendstclr(&self) -> MemResult<bool> {
        todo!("read Control ICSR PENDSTCLR reset value false")
    }
    #[doc = "Control ICSR PENDSTCLR: Clears a pending SysTick, whether set here or by the timer hardware"]
    #[inline]
    pub(crate) fn write_control_icsr_pendstclr(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control ICSR PENDSTCLR reset value false")
    }
    #[doc = "Control ICSR ISRPREEMPT: Indicates whether a pending exception will be serviced on exit from debug halt state"]
    #[inline]
    pub(crate) fn read_control_icsr_isrpreempt(&self) -> MemResult<bool> {
        todo!("read Control ICSR ISRPREEMPT reset value false")
    }
    #[doc = "Control ICSR ISRPREEMPT: Indicates whether a pending exception will be serviced on exit from debug halt state"]
    #[inline]
    pub(crate) fn write_control_icsr_isrpreempt(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control ICSR ISRPREEMPT reset value false")
    }
    #[doc = "Control ICSR ISRPENDING: Indicates if an external configurable, NVIC generated, interrupt is pending"]
    #[inline]
    pub(crate) fn read_control_icsr_isrpending(&self) -> MemResult<bool> {
        todo!("read Control ICSR ISRPENDING reset value false")
    }
    #[doc = "Control ICSR ISRPENDING: Indicates if an external configurable, NVIC generated, interrupt is pending"]
    #[inline]
    pub(crate) fn write_control_icsr_isrpending(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control ICSR ISRPENDING reset value false")
    }
    #[doc = "Control ICSR VECTPENDING: The exception number for the highest priority pending exception. 0 indicates no pending exceptions"]
    #[inline]
    pub(crate) fn read_control_icsr_vectpending(&self) -> MemResult<u16> {
        todo!("read Control ICSR VECTPENDING reset value 0x00 mask 0x1ff")
    }
    #[doc = "Control ICSR VECTPENDING: The exception number for the highest priority pending exception. 0 indicates no pending exceptions"]
    #[inline]
    pub(crate) fn write_control_icsr_vectpending(
        &mut self,
        _value: u16,
    ) -> MemResult<()> {
        todo!("write Control ICSR VECTPENDING reset value 0x00 mask 0x1ff")
    }
    #[doc = "Control ICSR RETTOBASE: In Handler mode, indicates whether there is an active exception other than the exception indicated by the current value of the IPSR"]
    #[inline]
    pub(crate) fn read_control_icsr_rettobase(&self) -> MemResult<bool> {
        todo!("read Control ICSR RETTOBASE reset value false")
    }
    #[doc = "Control ICSR RETTOBASE: In Handler mode, indicates whether there is an active exception other than the exception indicated by the current value of the IPSR"]
    #[inline]
    pub(crate) fn write_control_icsr_rettobase(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control ICSR RETTOBASE reset value false")
    }
    #[doc = "Control ICSR VECTACTIVE: The exception number for the current executing exception"]
    #[inline]
    pub(crate) fn read_control_icsr_vectactive(&self) -> MemResult<u16> {
        todo!("read Control ICSR VECTACTIVE reset value 0x00 mask 0x1ff")
    }
    #[doc = "Control ICSR VECTACTIVE: The exception number for the current executing exception"]
    #[inline]
    pub(crate) fn write_control_icsr_vectactive(
        &mut self,
        _value: u16,
    ) -> MemResult<()> {
        todo!("write Control ICSR VECTACTIVE reset value 0x00 mask 0x1ff")
    }
    #[doc = "Control VTOR TBLOFF: Bits [31:7] of the vector table address"]
    #[inline]
    pub(crate) fn read_control_vtor_tbloff(&self) -> MemResult<u32> {
        todo!("read Control VTOR TBLOFF reset value 0x00 mask 0x1ffffff")
    }
    #[doc = "Control VTOR TBLOFF: Bits [31:7] of the vector table address"]
    #[inline]
    pub(crate) fn write_control_vtor_tbloff(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write Control VTOR TBLOFF reset value 0x00 mask 0x1ffffff")
    }
    #[doc = "Control VTOR TBLBASE: Determines whether the vector table is in the code or SRAM memory region"]
    #[inline]
    pub(crate) fn read_control_vtor_tblbase(&self) -> MemResult<bool> {
        todo!("read Control VTOR TBLBASE reset value false")
    }
    #[doc = "Control VTOR TBLBASE: Determines whether the vector table is in the code or SRAM memory region"]
    #[inline]
    pub(crate) fn write_control_vtor_tblbase(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control VTOR TBLBASE reset value false")
    }
    #[doc = "Control DEMCR MON_EN: Enable the DebugMonitor exception"]
    #[inline]
    pub(crate) fn read_control_demcr_mon_en(&self) -> MemResult<bool> {
        todo!("read Control DEMCR MON_EN reset value false")
    }
    #[doc = "Control DEMCR MON_PEND: Sets or clears the pending state of the DebugMonitor exception"]
    #[inline]
    pub(crate) fn read_control_demcr_mon_pend(&self) -> MemResult<bool> {
        todo!("read Control DEMCR MON_PEND reset value false")
    }
    #[doc = "Control AIRCR VECTKEY: Vector Key\n\nControl AIRCR VECTKEYSTAT: UNKNOWN"]
    #[inline]
    pub(crate) fn read_control_aircr_vectkey(&self) -> MemResult<u16> {
        todo ! ("read Control AIRCR VECTKEY, Control AIRCR VECTKEYSTAT reset value 0x00 mask 0xffff")
    }
    #[doc = "Control AIRCR VECTKEY: Vector Key\n\nControl AIRCR VECTKEYSTAT: UNKNOWN"]
    #[inline]
    pub(crate) fn write_control_aircr_vectkey(
        &mut self,
        _value: u16,
    ) -> MemResult<()> {
        todo ! ("write Control AIRCR VECTKEY, Control AIRCR VECTKEYSTAT reset value 0x00 mask 0xffff")
    }
    #[doc = "Control AIRCR ENDIANNESS: Indicates the memory system data endianness"]
    #[inline]
    pub(crate) fn read_control_aircr_endianness(&self) -> MemResult<bool> {
        todo!("read Control AIRCR ENDIANNESS reset value false")
    }
    #[doc = "Control AIRCR ENDIANNESS: Indicates the memory system data endianness"]
    #[inline]
    pub(crate) fn write_control_aircr_endianness(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control AIRCR ENDIANNESS reset value false")
    }
    #[doc = "Control AIRCR PRIGROUP: Priority grouping, indicates the    binary point position."]
    #[inline]
    pub(crate) fn read_control_aircr_prigroup(&self) -> MemResult<u8> {
        todo!("read Control AIRCR PRIGROUP reset value 0x00 mask 0x07")
    }
    #[doc = "Control AIRCR PRIGROUP: Priority grouping, indicates the    binary point position."]
    #[inline]
    pub(crate) fn write_control_aircr_prigroup(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write Control AIRCR PRIGROUP reset value 0x00 mask 0x07")
    }
    #[doc = "Control AIRCR SYSRESETREQ: System Reset Request"]
    #[inline]
    pub(crate) fn read_control_aircr_sysresetreq(&self) -> MemResult<bool> {
        todo!("read Control AIRCR SYSRESETREQ reset value false")
    }
    #[doc = "Control AIRCR SYSRESETREQ: System Reset Request"]
    #[inline]
    pub(crate) fn write_control_aircr_sysresetreq(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control AIRCR SYSRESETREQ reset value false")
    }
    #[doc = "Control AIRCR VECTCLRACTIVE: Clears all active state information for fixed and configurable exceptions"]
    #[inline]
    pub(crate) fn read_control_aircr_vectclractive(&self) -> MemResult<bool> {
        todo!("read Control AIRCR VECTCLRACTIVE reset value false")
    }
    #[doc = "Control AIRCR VECTCLRACTIVE: Clears all active state information for fixed and configurable exceptions"]
    #[inline]
    pub(crate) fn write_control_aircr_vectclractive(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control AIRCR VECTCLRACTIVE reset value false")
    }
    #[doc = "Control SCR SEVONPEND: Determines whether an interrupt transition from inactive state to pending state is a wakeup event"]
    #[inline]
    pub(crate) fn read_control_scr_sevonpend(&self) -> MemResult<bool> {
        Ok(self.control.event_on_pending())
    }
    #[doc = "Control SCR SEVONPEND: Determines whether an interrupt transition from inactive state to pending state is a wakeup event"]
    #[inline]
    pub(crate) fn write_control_scr_sevonpend(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.control.set_event_on_pending(_value))
    }
    #[doc = "Control SCR SLEEPDEEP: Hint indicating that waking from sleep might take longer"]
    #[inline]
    pub(crate) fn read_control_scr_sleepdeep(&self) -> MemResult<bool> {
        Ok(self.control.sleep_deep())
    }
    #[doc = "Control SCR SLEEPDEEP: Hint indicating that waking from sleep might take longer"]
    #[inline]
    pub(crate) fn write_control_scr_sleepdeep(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.control.set_sleep_deep(_value))
    }
    #[doc = "Control SCR SLEEPONEXIT: whether, on an exit from an ISR that returns to the base level of execution priority, the processor enters a sleep state"]
    #[inline]
    pub(crate) fn read_control_scr_sleeponexit(&self) -> MemResult<bool> {
        Ok(self.control.sleep_on_exit())
    }
    #[doc = "Control SCR SLEEPONEXIT: whether, on an exit from an ISR that returns to the base level of execution priority, the processor enters a sleep state"]
    #[inline]
    pub(crate) fn write_control_scr_sleeponexit(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.control.set_sleep_on_exit(_value))
    }
    #[doc = "Control CCR STKALIGN: Determines whether the exception entry sequence guarantees 8-byte stack frame alignment, adjusting the SP if necessary before saving state"]
    #[inline]
    pub(crate) fn read_control_ccr_stkalign(&self) -> MemResult<bool> {
        todo!("read Control CCR STKALIGN reset value false")
    }
    #[doc = "Control CCR STKALIGN: Determines whether the exception entry sequence guarantees 8-byte stack frame alignment, adjusting the SP if necessary before saving state"]
    #[inline]
    pub(crate) fn write_control_ccr_stkalign(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control CCR STKALIGN reset value false")
    }
    #[doc = "Control CCR BFHFNMIGN: Determines the effect of precise data access faults on handlers running at priority -1 or priority -2"]
    #[inline]
    pub(crate) fn read_control_ccr_bfhfnmign(&self) -> MemResult<bool> {
        todo!("read Control CCR BFHFNMIGN reset value false")
    }
    #[doc = "Control CCR BFHFNMIGN: Determines the effect of precise data access faults on handlers running at priority -1 or priority -2"]
    #[inline]
    pub(crate) fn write_control_ccr_bfhfnmign(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control CCR BFHFNMIGN reset value false")
    }
    #[doc = "Control CCR DIV_0_TRP: Controls the trap on divide by 0"]
    #[inline]
    pub(crate) fn read_control_ccr_div_0_trp(&self) -> MemResult<bool> {
        todo!("read Control CCR DIV_0_TRP reset value false")
    }
    #[doc = "Control CCR DIV_0_TRP: Controls the trap on divide by 0"]
    #[inline]
    pub(crate) fn write_control_ccr_div_0_trp(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control CCR DIV_0_TRP reset value false")
    }
    #[doc = "Control CCR UNALIGN_TRP: Controls the trapping of unaligned word or halfword accesses"]
    #[inline]
    pub(crate) fn read_control_ccr_unalign_trp(&self) -> MemResult<bool> {
        todo!("read Control CCR UNALIGN_TRP reset value false")
    }
    #[doc = "Control CCR UNALIGN_TRP: Controls the trapping of unaligned word or halfword accesses"]
    #[inline]
    pub(crate) fn write_control_ccr_unalign_trp(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control CCR UNALIGN_TRP reset value false")
    }
    #[doc = "Control CCR USERSETMPEND: Controls whether unprivileged software can access the STIR"]
    #[inline]
    pub(crate) fn read_control_ccr_usersetmpend(&self) -> MemResult<bool> {
        todo!("read Control CCR USERSETMPEND reset value false")
    }
    #[doc = "Control CCR USERSETMPEND: Controls whether unprivileged software can access the STIR"]
    #[inline]
    pub(crate) fn write_control_ccr_usersetmpend(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control CCR USERSETMPEND reset value false")
    }
    #[doc = "Control CCR NONBASETHRDENA: Controls whether the processor can enter Thread mode at an execution priority level other than base level"]
    #[inline]
    pub(crate) fn read_control_ccr_nonbasethrdena(&self) -> MemResult<bool> {
        todo!("read Control CCR NONBASETHRDENA reset value false")
    }
    #[doc = "Control CCR NONBASETHRDENA: Controls whether the processor can enter Thread mode at an execution priority level other than base level"]
    #[inline]
    pub(crate) fn write_control_ccr_nonbasethrdena(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control CCR NONBASETHRDENA reset value false")
    }
    #[doc = "Control SHPR1 PRI_7: Priority of system handler 7"]
    #[inline]
    pub(crate) fn read_control_shpr1_pri_7(&self) -> MemResult<u8> {
        Ok(self.control.priority(SysHandlerPriority::Pri7))
    }
    #[doc = "Control SHPR1 PRI_7: Priority of system handler 7"]
    #[inline]
    pub(crate) fn write_control_shpr1_pri_7(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        self.control.set_priority(SysHandlerPriority::Pri7, _value)
    }
    #[doc = "Control SHPR1 PRI_6: Priority of system handler 6, UsageFault"]
    #[inline]
    pub(crate) fn read_control_shpr1_pri_6(&self) -> MemResult<u8> {
        Ok(self.control.priority(SysHandlerPriority::Pri6))
    }
    #[doc = "Control SHPR1 PRI_6: Priority of system handler 6, UsageFault"]
    #[inline]
    pub(crate) fn write_control_shpr1_pri_6(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        self.control.set_priority(SysHandlerPriority::Pri6, _value)
    }
    #[doc = "Control SHPR1 PRI_5: Priority of system handler 5, BusFault"]
    #[inline]
    pub(crate) fn read_control_shpr1_pri_5(&self) -> MemResult<u8> {
        Ok(self.control.priority(SysHandlerPriority::Pri5))
    }
    #[doc = "Control SHPR1 PRI_5: Priority of system handler 5, BusFault"]
    #[inline]
    pub(crate) fn write_control_shpr1_pri_5(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        self.control.set_priority(SysHandlerPriority::Pri5, _value)
    }
    #[doc = "Control SHPR1 PRI_4: Priority of system handler 4, MemManage"]
    #[inline]
    pub(crate) fn read_control_shpr1_pri_4(&self) -> MemResult<u8> {
        Ok(self.control.priority(SysHandlerPriority::Pri4))
    }
    #[doc = "Control SHPR1 PRI_4: Priority of system handler 4, MemManage"]
    #[inline]
    pub(crate) fn write_control_shpr1_pri_4(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        self.control.set_priority(SysHandlerPriority::Pri4, _value)
    }
    #[doc = "Control SHPR2 PRI_11: Priority of system handler 11, SVCall"]
    #[inline]
    pub(crate) fn read_control_shpr2_pri_11(&self) -> MemResult<u8> {
        Ok(self.control.priority(SysHandlerPriority::Pri11))
    }
    #[doc = "Control SHPR2 PRI_11: Priority of system handler 11, SVCall"]
    #[inline]
    pub(crate) fn write_control_shpr2_pri_11(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        self.control.set_priority(SysHandlerPriority::Pri11, _value)
    }
    #[doc = "Control SHPR2 PRI_10: Priority of system handler 10"]
    #[inline]
    pub(crate) fn read_control_shpr2_pri_10(&self) -> MemResult<u8> {
        Ok(self.control.priority(SysHandlerPriority::Pri10))
    }
    #[doc = "Control SHPR2 PRI_10: Priority of system handler 10"]
    #[inline]
    pub(crate) fn write_control_shpr2_pri_10(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        self.control.set_priority(SysHandlerPriority::Pri10, _value)
    }
    #[doc = "Control SHPR2 PRI_9: Priority of system handler 9"]
    #[inline]
    pub(crate) fn read_control_shpr2_pri_9(&self) -> MemResult<u8> {
        Ok(self.control.priority(SysHandlerPriority::Pri9))
    }
    #[doc = "Control SHPR2 PRI_9: Priority of system handler 9"]
    #[inline]
    pub(crate) fn write_control_shpr2_pri_9(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        self.control.set_priority(SysHandlerPriority::Pri9, _value)
    }
    #[doc = "Control SHPR2 PRI_8: Priority of system handler 8"]
    #[inline]
    pub(crate) fn read_control_shpr2_pri_8(&self) -> MemResult<u8> {
        Ok(self.control.priority(SysHandlerPriority::Pri8))
    }
    #[doc = "Control SHPR2 PRI_8: Priority of system handler 8"]
    #[inline]
    pub(crate) fn write_control_shpr2_pri_8(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        self.control.set_priority(SysHandlerPriority::Pri8, _value)
    }
    #[doc = "Control SHPR3 PRI_15: Priority of system handler 15, SysTick"]
    #[inline]
    pub(crate) fn read_control_shpr3_pri_15(&self) -> MemResult<u8> {
        Ok(self.control.priority(SysHandlerPriority::Pri15))
    }
    #[doc = "Control SHPR3 PRI_15: Priority of system handler 15, SysTick"]
    #[inline]
    pub(crate) fn write_control_shpr3_pri_15(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        self.control.set_priority(SysHandlerPriority::Pri15, _value)
    }
    #[doc = "Control SHPR3 PRI_14: Priority of system handler 14, PendSV"]
    #[inline]
    pub(crate) fn read_control_shpr3_pri_14(&self) -> MemResult<u8> {
        Ok(self.control.priority(SysHandlerPriority::Pri14))
    }
    #[doc = "Control SHPR3 PRI_14: Priority of system handler 14, PendSV"]
    #[inline]
    pub(crate) fn write_control_shpr3_pri_14(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        self.control.set_priority(SysHandlerPriority::Pri14, _value)
    }
    #[doc = "Control SHPR3 PRI_13: Priority of system handler 13"]
    #[inline]
    pub(crate) fn read_control_shpr3_pri_13(&self) -> MemResult<u8> {
        Ok(self.control.priority(SysHandlerPriority::Pri13))
    }
    #[doc = "Control SHPR3 PRI_13: Priority of system handler 13"]
    #[inline]
    pub(crate) fn write_control_shpr3_pri_13(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        self.control.set_priority(SysHandlerPriority::Pri13, _value)
    }
    #[doc = "Control SHPR3 PRI_12: Priority of system handler 4, DebugMonitor"]
    #[inline]
    pub(crate) fn read_control_shpr3_pri_12(&self) -> MemResult<u8> {
        Ok(self.control.priority(SysHandlerPriority::Pri12))
    }
    #[doc = "Control SHPR3 PRI_12: Priority of system handler 4, DebugMonitor"]
    #[inline]
    pub(crate) fn write_control_shpr3_pri_12(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        self.control.set_priority(SysHandlerPriority::Pri12, _value)
    }
    #[doc = "Control SHCSR USGFAULTENA: Enable UsageFault"]
    #[inline]
    pub(crate) fn read_control_shcsr_usgfaultena(&self) -> MemResult<bool> {
        todo!("read Control SHCSR USGFAULTENA reset value false")
    }
    #[doc = "Control SHCSR USGFAULTENA: Enable UsageFault"]
    #[inline]
    pub(crate) fn write_control_shcsr_usgfaultena(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control SHCSR USGFAULTENA reset value false")
    }
    #[doc = "Control SHCSR BUSFAULTENA: Enable BusFault"]
    #[inline]
    pub(crate) fn read_control_shcsr_busfaultena(&self) -> MemResult<bool> {
        todo!("read Control SHCSR BUSFAULTENA reset value false")
    }
    #[doc = "Control SHCSR BUSFAULTENA: Enable BusFault"]
    #[inline]
    pub(crate) fn write_control_shcsr_busfaultena(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control SHCSR BUSFAULTENA reset value false")
    }
    #[doc = "Control SHCSR MEMFAULTENA: Enable MemManage fault"]
    #[inline]
    pub(crate) fn read_control_shcsr_memfaultena(&self) -> MemResult<bool> {
        todo!("read Control SHCSR MEMFAULTENA reset value false")
    }
    #[doc = "Control SHCSR MEMFAULTENA: Enable MemManage fault"]
    #[inline]
    pub(crate) fn write_control_shcsr_memfaultena(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control SHCSR MEMFAULTENA reset value false")
    }
    #[doc = "Control SHCSR SVCALLPENDED: SVCall pending"]
    #[inline]
    pub(crate) fn read_control_shcsr_svcallpended(&self) -> MemResult<bool> {
        todo!("read Control SHCSR SVCALLPENDED reset value false")
    }
    #[doc = "Control SHCSR SVCALLPENDED: SVCall pending"]
    #[inline]
    pub(crate) fn write_control_shcsr_svcallpended(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control SHCSR SVCALLPENDED reset value false")
    }
    #[doc = "Control SHCSR BUSFAULTPENDED: BusFault pending"]
    #[inline]
    pub(crate) fn read_control_shcsr_busfaultpended(&self) -> MemResult<bool> {
        todo!("read Control SHCSR BUSFAULTPENDED reset value false")
    }
    #[doc = "Control SHCSR BUSFAULTPENDED: BusFault pending"]
    #[inline]
    pub(crate) fn write_control_shcsr_busfaultpended(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control SHCSR BUSFAULTPENDED reset value false")
    }
    #[doc = "Control SHCSR MEMFAULTPENDED: MemManage pending"]
    #[inline]
    pub(crate) fn read_control_shcsr_memfaultpended(&self) -> MemResult<bool> {
        todo!("read Control SHCSR MEMFAULTPENDED reset value false")
    }
    #[doc = "Control SHCSR MEMFAULTPENDED: MemManage pending"]
    #[inline]
    pub(crate) fn write_control_shcsr_memfaultpended(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control SHCSR MEMFAULTPENDED reset value false")
    }
    #[doc = "Control SHCSR USGFAULTPENDED: UsageFault pending"]
    #[inline]
    pub(crate) fn read_control_shcsr_usgfaultpended(&self) -> MemResult<bool> {
        todo!("read Control SHCSR USGFAULTPENDED reset value false")
    }
    #[doc = "Control SHCSR USGFAULTPENDED: UsageFault pending"]
    #[inline]
    pub(crate) fn write_control_shcsr_usgfaultpended(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control SHCSR USGFAULTPENDED reset value false")
    }
    #[doc = "Control SHCSR SYSTICKACT: SysTick active"]
    #[inline]
    pub(crate) fn read_control_shcsr_systickact(&self) -> MemResult<bool> {
        todo!("read Control SHCSR SYSTICKACT reset value false")
    }
    #[doc = "Control SHCSR SYSTICKACT: SysTick active"]
    #[inline]
    pub(crate) fn write_control_shcsr_systickact(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control SHCSR SYSTICKACT reset value false")
    }
    #[doc = "Control SHCSR PENDSVACT: PendSV active"]
    #[inline]
    pub(crate) fn read_control_shcsr_pendsvact(&self) -> MemResult<bool> {
        todo!("read Control SHCSR PENDSVACT reset value false")
    }
    #[doc = "Control SHCSR PENDSVACT: PendSV active"]
    #[inline]
    pub(crate) fn write_control_shcsr_pendsvact(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control SHCSR PENDSVACT reset value false")
    }
    #[doc = "Control SHCSR MONITORACT: Monitor active"]
    #[inline]
    pub(crate) fn read_control_shcsr_monitoract(&self) -> MemResult<bool> {
        todo!("read Control SHCSR MONITORACT reset value false")
    }
    #[doc = "Control SHCSR MONITORACT: Monitor active"]
    #[inline]
    pub(crate) fn write_control_shcsr_monitoract(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control SHCSR MONITORACT reset value false")
    }
    #[doc = "Control SHCSR SVCALLACT: SVCall active"]
    #[inline]
    pub(crate) fn read_control_shcsr_svcallact(&self) -> MemResult<bool> {
        todo!("read Control SHCSR SVCALLACT reset value false")
    }
    #[doc = "Control SHCSR SVCALLACT: SVCall active"]
    #[inline]
    pub(crate) fn write_control_shcsr_svcallact(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control SHCSR SVCALLACT reset value false")
    }
    #[doc = "Control SHCSR USGFAULTACT: UsageFault active"]
    #[inline]
    pub(crate) fn read_control_shcsr_usgfaultact(&self) -> MemResult<bool> {
        todo!("read Control SHCSR USGFAULTACT reset value false")
    }
    #[doc = "Control SHCSR USGFAULTACT: UsageFault active"]
    #[inline]
    pub(crate) fn write_control_shcsr_usgfaultact(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control SHCSR USGFAULTACT reset value false")
    }
    #[doc = "Control SHCSR BUSFAULTACT: BusFault active"]
    #[inline]
    pub(crate) fn read_control_shcsr_busfaultact(&self) -> MemResult<bool> {
        todo!("read Control SHCSR BUSFAULTACT reset value false")
    }
    #[doc = "Control SHCSR BUSFAULTACT: BusFault active"]
    #[inline]
    pub(crate) fn write_control_shcsr_busfaultact(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control SHCSR BUSFAULTACT reset value false")
    }
    #[doc = "Control SHCSR MEMFAULTACT: MemManage active"]
    #[inline]
    pub(crate) fn read_control_shcsr_memfaultact(&self) -> MemResult<bool> {
        todo!("read Control SHCSR MEMFAULTACT reset value false")
    }
    #[doc = "Control SHCSR MEMFAULTACT: MemManage active"]
    #[inline]
    pub(crate) fn write_control_shcsr_memfaultact(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control SHCSR MEMFAULTACT reset value false")
    }
    #[doc = "Control CFSR DIVBYZERO: Divide by zero error"]
    #[inline]
    pub(crate) fn read_control_cfsr_divbyzero(&self) -> MemResult<bool> {
        todo!("read Control CFSR DIVBYZERO reset value false")
    }
    #[doc = "Control CFSR DIVBYZERO: Divide by zero error"]
    #[inline]
    pub(crate) fn write_control_cfsr_divbyzero(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control CFSR DIVBYZERO reset value false")
    }
    #[doc = "Control CFSR UNALIGNED: Unaligned access error"]
    #[inline]
    pub(crate) fn read_control_cfsr_unaligned(&self) -> MemResult<bool> {
        todo!("read Control CFSR UNALIGNED reset value false")
    }
    #[doc = "Control CFSR UNALIGNED: Unaligned access error"]
    #[inline]
    pub(crate) fn write_control_cfsr_unaligned(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control CFSR UNALIGNED reset value false")
    }
    #[doc = "Control CFSR NOCP: Coprocessor access error"]
    #[inline]
    pub(crate) fn read_control_cfsr_nocp(&self) -> MemResult<bool> {
        todo!("read Control CFSR NOCP reset value false")
    }
    #[doc = "Control CFSR NOCP: Coprocessor access error"]
    #[inline]
    pub(crate) fn write_control_cfsr_nocp(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control CFSR NOCP reset value false")
    }
    #[doc = "Control CFSR INVPC: Integrity check error on EXC_RETURN"]
    #[inline]
    pub(crate) fn read_control_cfsr_invpc(&self) -> MemResult<bool> {
        todo!("read Control CFSR INVPC reset value false")
    }
    #[doc = "Control CFSR INVPC: Integrity check error on EXC_RETURN"]
    #[inline]
    pub(crate) fn write_control_cfsr_invpc(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control CFSR INVPC reset value false")
    }
    #[doc = "Control CFSR INVSTATE: Instruction executed with invalid EPSR.T or EPSR.IT field"]
    #[inline]
    pub(crate) fn read_control_cfsr_invstate(&self) -> MemResult<bool> {
        todo!("read Control CFSR INVSTATE reset value false")
    }
    #[doc = "Control CFSR INVSTATE: Instruction executed with invalid EPSR.T or EPSR.IT field"]
    #[inline]
    pub(crate) fn write_control_cfsr_invstate(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control CFSR INVSTATE reset value false")
    }
    #[doc = "Control CFSR UNDEFINSTR: Processor has attempted to execute an undefined instruction."]
    #[inline]
    pub(crate) fn read_control_cfsr_undefinstr(&self) -> MemResult<bool> {
        todo!("read Control CFSR UNDEFINSTR reset value false")
    }
    #[doc = "Control CFSR UNDEFINSTR: Processor has attempted to execute an undefined instruction."]
    #[inline]
    pub(crate) fn write_control_cfsr_undefinstr(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control CFSR UNDEFINSTR reset value false")
    }
    #[doc = "Control CFSR BFARVALID: BFAR has valid contents"]
    #[inline]
    pub(crate) fn read_control_cfsr_bfarvalid(&self) -> MemResult<bool> {
        todo!("read Control CFSR BFARVALID reset value false")
    }
    #[doc = "Control CFSR BFARVALID: BFAR has valid contents"]
    #[inline]
    pub(crate) fn write_control_cfsr_bfarvalid(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control CFSR BFARVALID reset value false")
    }
    #[doc = "Control CFSR LSPERR: Bus fault during FP lazy state preservation"]
    #[inline]
    pub(crate) fn read_control_cfsr_lsperr(&self) -> MemResult<bool> {
        todo!("read Control CFSR LSPERR reset value false")
    }
    #[doc = "Control CFSR LSPERR: Bus fault during FP lazy state preservation"]
    #[inline]
    pub(crate) fn write_control_cfsr_lsperr(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control CFSR LSPERR reset value false")
    }
    #[doc = "Control CFSR STKERR: Derived bus fault on exception entry"]
    #[inline]
    pub(crate) fn read_control_cfsr_stkerr(&self) -> MemResult<bool> {
        todo!("read Control CFSR STKERR reset value false")
    }
    #[doc = "Control CFSR STKERR: Derived bus fault on exception entry"]
    #[inline]
    pub(crate) fn write_control_cfsr_stkerr(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control CFSR STKERR reset value false")
    }
    #[doc = "Control CFSR UNSTKERR: Derived bus fault on exception return"]
    #[inline]
    pub(crate) fn read_control_cfsr_unstkerr(&self) -> MemResult<bool> {
        todo!("read Control CFSR UNSTKERR reset value false")
    }
    #[doc = "Control CFSR UNSTKERR: Derived bus fault on exception return"]
    #[inline]
    pub(crate) fn write_control_cfsr_unstkerr(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control CFSR UNSTKERR reset value false")
    }
    #[doc = "Control CFSR IMPRECISERR: Imprecise data access error"]
    #[inline]
    pub(crate) fn read_control_cfsr_impreciserr(&self) -> MemResult<bool> {
        todo!("read Control CFSR IMPRECISERR reset value false")
    }
    #[doc = "Control CFSR IMPRECISERR: Imprecise data access error"]
    #[inline]
    pub(crate) fn write_control_cfsr_impreciserr(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control CFSR IMPRECISERR reset value false")
    }
    #[doc = "Control CFSR PRECISERR: Precise data access error"]
    #[inline]
    pub(crate) fn read_control_cfsr_preciserr(&self) -> MemResult<bool> {
        todo!("read Control CFSR PRECISERR reset value false")
    }
    #[doc = "Control CFSR PRECISERR: Precise data access error"]
    #[inline]
    pub(crate) fn write_control_cfsr_preciserr(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control CFSR PRECISERR reset value false")
    }
    #[doc = "Control CFSR IBUSERR: Bus fault on an instruction prefetch"]
    #[inline]
    pub(crate) fn read_control_cfsr_ibuserr(&self) -> MemResult<bool> {
        todo!("read Control CFSR IBUSERR reset value false")
    }
    #[doc = "Control CFSR IBUSERR: Bus fault on an instruction prefetch"]
    #[inline]
    pub(crate) fn write_control_cfsr_ibuserr(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control CFSR IBUSERR reset value false")
    }
    #[doc = "Control CFSR MMARVALID: MMAR has valid contents"]
    #[inline]
    pub(crate) fn read_control_cfsr_mmarvalid(&self) -> MemResult<bool> {
        todo!("read Control CFSR MMARVALID reset value false")
    }
    #[doc = "Control CFSR MMARVALID: MMAR has valid contents"]
    #[inline]
    pub(crate) fn write_control_cfsr_mmarvalid(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control CFSR MMARVALID reset value false")
    }
    #[doc = "Control CFSR MLSPERR: MemManage fault during FP lazy state preservation"]
    #[inline]
    pub(crate) fn read_control_cfsr_mlsperr(&self) -> MemResult<bool> {
        todo!("read Control CFSR MLSPERR reset value false")
    }
    #[doc = "Control CFSR MLSPERR: MemManage fault during FP lazy state preservation"]
    #[inline]
    pub(crate) fn write_control_cfsr_mlsperr(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control CFSR MLSPERR reset value false")
    }
    #[doc = "Control CFSR MSTKERR: Derived MemManage fault on exception entry"]
    #[inline]
    pub(crate) fn read_control_cfsr_mstkerr(&self) -> MemResult<bool> {
        todo!("read Control CFSR MSTKERR reset value false")
    }
    #[doc = "Control CFSR MSTKERR: Derived MemManage fault on exception entry"]
    #[inline]
    pub(crate) fn write_control_cfsr_mstkerr(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control CFSR MSTKERR reset value false")
    }
    #[doc = "Control CFSR MUNSTKERR: Derived MemManage fault on exception return"]
    #[inline]
    pub(crate) fn read_control_cfsr_munstkerr(&self) -> MemResult<bool> {
        todo!("read Control CFSR MUNSTKERR reset value false")
    }
    #[doc = "Control CFSR MUNSTKERR: Derived MemManage fault on exception return"]
    #[inline]
    pub(crate) fn write_control_cfsr_munstkerr(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control CFSR MUNSTKERR reset value false")
    }
    #[doc = "Control CFSR DACCVIOL: Data access violation. The MMAR shows the data address that the load or store tried to access"]
    #[inline]
    pub(crate) fn read_control_cfsr_daccviol(&self) -> MemResult<bool> {
        todo!("read Control CFSR DACCVIOL reset value false")
    }
    #[doc = "Control CFSR DACCVIOL: Data access violation. The MMAR shows the data address that the load or store tried to access"]
    #[inline]
    pub(crate) fn write_control_cfsr_daccviol(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control CFSR DACCVIOL reset value false")
    }
    #[doc = "Control CFSR IACCVIOL: MPU or Execute Never (XN) default memory map access violation on an instruction fetch"]
    #[inline]
    pub(crate) fn read_control_cfsr_iaccviol(&self) -> MemResult<bool> {
        todo!("read Control CFSR IACCVIOL reset value false")
    }
    #[doc = "Control CFSR IACCVIOL: MPU or Execute Never (XN) default memory map access violation on an instruction fetch"]
    #[inline]
    pub(crate) fn write_control_cfsr_iaccviol(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control CFSR IACCVIOL reset value false")
    }
    #[doc = "Control HFSR DEBUGEVT: Indicates a Debug event has occurred"]
    #[inline]
    pub(crate) fn read_control_hfsr_debugevt(&self) -> MemResult<bool> {
        todo!("read Control HFSR DEBUGEVT reset value false")
    }
    #[doc = "Control HFSR DEBUGEVT: Indicates a Debug event has occurred"]
    #[inline]
    pub(crate) fn write_control_hfsr_debugevt(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control HFSR DEBUGEVT reset value false")
    }
    #[doc = "Control HFSR FORCED: Indicates a fault with configurable priority has been escalated to a HardFault"]
    #[inline]
    pub(crate) fn read_control_hfsr_forced(&self) -> MemResult<bool> {
        todo!("read Control HFSR FORCED reset value false")
    }
    #[doc = "Control HFSR FORCED: Indicates a fault with configurable priority has been escalated to a HardFault"]
    #[inline]
    pub(crate) fn write_control_hfsr_forced(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control HFSR FORCED reset value false")
    }
    #[doc = "Control HFSR VECTTBL: Indicates a fault has occurred because of a vector table read error on exception processing"]
    #[inline]
    pub(crate) fn read_control_hfsr_vecttbl(&self) -> MemResult<bool> {
        todo!("read Control HFSR VECTTBL reset value false")
    }
    #[doc = "Control HFSR VECTTBL: Indicates a fault has occurred because of a vector table read error on exception processing"]
    #[inline]
    pub(crate) fn write_control_hfsr_vecttbl(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control HFSR VECTTBL reset value false")
    }
    #[doc = "Control DFSR EXTERNAL: Indicates a debug event generated because of the assertion of EDBGRQ"]
    #[inline]
    pub(crate) fn read_control_dfsr_external(&self) -> MemResult<bool> {
        todo!("read Control DFSR EXTERNAL reset value false")
    }
    #[doc = "Control DFSR EXTERNAL: Indicates a debug event generated because of the assertion of EDBGRQ"]
    #[inline]
    pub(crate) fn write_control_dfsr_external(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control DFSR EXTERNAL reset value false")
    }
    #[doc = "Control DFSR VCATCH: Indicates triggering of a Vector catch"]
    #[inline]
    pub(crate) fn read_control_dfsr_vcatch(&self) -> MemResult<bool> {
        todo!("read Control DFSR VCATCH reset value false")
    }
    #[doc = "Control DFSR VCATCH: Indicates triggering of a Vector catch"]
    #[inline]
    pub(crate) fn write_control_dfsr_vcatch(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control DFSR VCATCH reset value false")
    }
    #[doc = "Control DFSR DWTTRAP: Indicates a debug event generated by the DWT"]
    #[inline]
    pub(crate) fn read_control_dfsr_dwttrap(&self) -> MemResult<bool> {
        todo!("read Control DFSR DWTTRAP reset value false")
    }
    #[doc = "Control DFSR DWTTRAP: Indicates a debug event generated by the DWT"]
    #[inline]
    pub(crate) fn write_control_dfsr_dwttrap(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control DFSR DWTTRAP reset value false")
    }
    #[doc = "Control DFSR BKPT: Indicates a debug event generated by BKPT instruction execution or a breakpoint match in FPB"]
    #[inline]
    pub(crate) fn read_control_dfsr_bkpt(&self) -> MemResult<bool> {
        todo!("read Control DFSR BKPT reset value false")
    }
    #[doc = "Control DFSR BKPT: Indicates a debug event generated by BKPT instruction execution or a breakpoint match in FPB"]
    #[inline]
    pub(crate) fn write_control_dfsr_bkpt(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control DFSR BKPT reset value false")
    }
    #[doc = "Control DFSR HALTED: Indicates a debug event generated by C_HALT or C_STEP request or setting DEMCR.MON_STEP"]
    #[inline]
    pub(crate) fn read_control_dfsr_halted(&self) -> MemResult<bool> {
        todo!("read Control DFSR HALTED reset value false")
    }
    #[doc = "Control DFSR HALTED: Indicates a debug event generated by C_HALT or C_STEP request or setting DEMCR.MON_STEP"]
    #[inline]
    pub(crate) fn write_control_dfsr_halted(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write Control DFSR HALTED reset value false")
    }
    #[doc = "Control MMFAR: Shows the address of the memory location that caused an MMU fault"]
    #[inline]
    pub(crate) fn read_control_mmfar(&self) -> MemResult<u32> {
        todo!("read Control MMFAR reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "Control MMFAR: Shows the address of the memory location that caused an MMU fault"]
    #[inline]
    pub(crate) fn write_control_mmfar(&mut self, _value: u32) -> MemResult<()> {
        todo!("write Control MMFAR reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "Control BFAR: Shows the address associated with a precise data access fault"]
    #[inline]
    pub(crate) fn read_control_bfar(&self) -> MemResult<u32> {
        todo!("read Control BFAR reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "Control BFAR: Shows the address associated with a precise data access fault"]
    #[inline]
    pub(crate) fn write_control_bfar(&mut self, _value: u32) -> MemResult<()> {
        todo!("write Control BFAR reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "Control AFSR: Latched version of the AUXFAULT inputs"]
    #[inline]
    pub(crate) fn read_control_afsr(&self) -> MemResult<u32> {
        todo!("read Control AFSR reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "Control AFSR: Latched version of the AUXFAULT inputs"]
    #[inline]
    pub(crate) fn write_control_afsr(&mut self, _value: u32) -> MemResult<()> {
        todo!("write Control AFSR reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "Control CPACR CP11: Defines access permissions for the CP11 coprocessor."]
    #[inline]
    pub(crate) fn read_control_cpacr_cp11(&self) -> MemResult<u8> {
        todo!("read Control CPACR CP11 reset value 0x00 mask 0x03")
    }
    #[doc = "Control CPACR CP11: Defines access permissions for the CP11 coprocessor."]
    #[inline]
    pub(crate) fn write_control_cpacr_cp11(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write Control CPACR CP11 reset value 0x00 mask 0x03")
    }
    #[doc = "Control CPACR CP10: Defines access permissions for the CP10 coprocessor."]
    #[inline]
    pub(crate) fn read_control_cpacr_cp10(&self) -> MemResult<u8> {
        todo!("read Control CPACR CP10 reset value 0x00 mask 0x03")
    }
    #[doc = "Control CPACR CP10: Defines access permissions for the CP10 coprocessor."]
    #[inline]
    pub(crate) fn write_control_cpacr_cp10(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write Control CPACR CP10 reset value 0x00 mask 0x03")
    }
    #[doc = "Control CPACR CP7: Defines access permissions for the CP7 coprocessor."]
    #[inline]
    pub(crate) fn read_control_cpacr_cp7(&self) -> MemResult<u8> {
        todo!("read Control CPACR CP7 reset value 0x00 mask 0x03")
    }
    #[doc = "Control CPACR CP7: Defines access permissions for the CP7 coprocessor."]
    #[inline]
    pub(crate) fn write_control_cpacr_cp7(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write Control CPACR CP7 reset value 0x00 mask 0x03")
    }
    #[doc = "Control CPACR CP6: Defines access permissions for the CP6 coprocessor."]
    #[inline]
    pub(crate) fn read_control_cpacr_cp6(&self) -> MemResult<u8> {
        todo!("read Control CPACR CP6 reset value 0x00 mask 0x03")
    }
    #[doc = "Control CPACR CP6: Defines access permissions for the CP6 coprocessor."]
    #[inline]
    pub(crate) fn write_control_cpacr_cp6(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write Control CPACR CP6 reset value 0x00 mask 0x03")
    }
    #[doc = "Control CPACR CP5: Defines access permissions for the CP5 coprocessor."]
    #[inline]
    pub(crate) fn read_control_cpacr_cp5(&self) -> MemResult<u8> {
        todo!("read Control CPACR CP5 reset value 0x00 mask 0x03")
    }
    #[doc = "Control CPACR CP5: Defines access permissions for the CP5 coprocessor."]
    #[inline]
    pub(crate) fn write_control_cpacr_cp5(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write Control CPACR CP5 reset value 0x00 mask 0x03")
    }
    #[doc = "Control CPACR CP4: Defines access permissions for the CP4 coprocessor."]
    #[inline]
    pub(crate) fn read_control_cpacr_cp4(&self) -> MemResult<u8> {
        todo!("read Control CPACR CP4 reset value 0x00 mask 0x03")
    }
    #[doc = "Control CPACR CP4: Defines access permissions for the CP4 coprocessor."]
    #[inline]
    pub(crate) fn write_control_cpacr_cp4(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write Control CPACR CP4 reset value 0x00 mask 0x03")
    }
    #[doc = "Control CPACR CP3: Defines access permissions for the CP3 coprocessor."]
    #[inline]
    pub(crate) fn read_control_cpacr_cp3(&self) -> MemResult<u8> {
        todo!("read Control CPACR CP3 reset value 0x00 mask 0x03")
    }
    #[doc = "Control CPACR CP3: Defines access permissions for the CP3 coprocessor."]
    #[inline]
    pub(crate) fn write_control_cpacr_cp3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write Control CPACR CP3 reset value 0x00 mask 0x03")
    }
    #[doc = "Control CPACR CP2: Defines access permissions for the CP2 coprocessor."]
    #[inline]
    pub(crate) fn read_control_cpacr_cp2(&self) -> MemResult<u8> {
        todo!("read Control CPACR CP2 reset value 0x00 mask 0x03")
    }
    #[doc = "Control CPACR CP2: Defines access permissions for the CP2 coprocessor."]
    #[inline]
    pub(crate) fn write_control_cpacr_cp2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write Control CPACR CP2 reset value 0x00 mask 0x03")
    }
    #[doc = "Control CPACR CP1: Defines access permissions for the CP1 coprocessor."]
    #[inline]
    pub(crate) fn read_control_cpacr_cp1(&self) -> MemResult<u8> {
        todo!("read Control CPACR CP1 reset value 0x00 mask 0x03")
    }
    #[doc = "Control CPACR CP1: Defines access permissions for the CP1 coprocessor."]
    #[inline]
    pub(crate) fn write_control_cpacr_cp1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write Control CPACR CP1 reset value 0x00 mask 0x03")
    }
    #[doc = "Control CPACR CP0: Defines access permissions for the CP0 coprocessor."]
    #[inline]
    pub(crate) fn read_control_cpacr_cp0(&self) -> MemResult<u8> {
        todo!("read Control CPACR CP0 reset value 0x00 mask 0x03")
    }
    #[doc = "Control CPACR CP0: Defines access permissions for the CP0 coprocessor."]
    #[inline]
    pub(crate) fn write_control_cpacr_cp0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write Control CPACR CP0 reset value 0x00 mask 0x03")
    }
    #[doc = "Control STIR INTID: Indicates the interrupt to be triggered. The value written is (ExceptionNumber - 16)"]
    #[inline]
    pub(crate) fn write_control_stir_intid(
        &mut self,
        _value: u16,
    ) -> MemResult<()> {
        todo!("write Control STIR INTID reset value 0x00 mask 0x1ff")
    }
    #[doc = "ID CPUID Implementer: Implementer code"]
    #[inline]
    pub(crate) fn read_id_cpuid_implementer(&self) -> MemResult<u8> {
        todo!("read ID CPUID Implementer reset value 0x00 mask 0xff")
    }
    #[doc = "ID CPUID Variant: Implementation defined"]
    #[inline]
    pub(crate) fn read_id_cpuid_variant(&self) -> MemResult<u8> {
        todo!("read ID CPUID Variant reset value 0x00 mask 0x0f")
    }
    #[doc = "ID CPUID Constant: Indicates the architecture"]
    #[inline]
    pub(crate) fn read_id_cpuid_constant(&self) -> MemResult<u8> {
        todo!("read ID CPUID Constant reset value 0x00 mask 0x0f")
    }
    #[doc = "ID CPUID Partno: Indicates part number"]
    #[inline]
    pub(crate) fn read_id_cpuid_partno(&self) -> MemResult<u16> {
        todo!("read ID CPUID Partno reset value 0x00 mask 0xfff")
    }
    #[doc = "ID CPUID Revision: Indicates revision"]
    #[inline]
    pub(crate) fn read_id_cpuid_revision(&self) -> MemResult<u8> {
        todo!("read ID CPUID Revision reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_PFR0 State1: Thumb instruction set support"]
    #[inline]
    pub(crate) fn read_id_id_pfr0_state1(&self) -> MemResult<u8> {
        todo!("read ID ID_PFR0 State1 reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_PFR1 M_Profile: M profile programmers' model"]
    #[inline]
    pub(crate) fn read_id_id_pfr1_m_profile(&self) -> MemResult<u8> {
        todo!("read ID ID_PFR1 M_Profile reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_DFR0 M_Profile: Debug model, M profile"]
    #[inline]
    pub(crate) fn read_id_id_dfr0_m_profile(&self) -> MemResult<u8> {
        todo!("read ID ID_DFR0 M_Profile reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_AFR0: Implementation defined features"]
    #[inline]
    pub(crate) fn read_id_id_afr0(&self) -> MemResult<u32> {
        todo!("read ID ID_AFR0 reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "ID ID_MMFR0 Auxiliary_registers: Indicates support for Auxiliary registers"]
    #[inline]
    pub(crate) fn read_id_id_mmfr0_auxiliary_registers(&self) -> MemResult<u8> {
        todo!("read ID ID_MMFR0 Auxiliary_registers reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_MMFR0 Shareability_levels: Indicates the number of shareability levels implemented"]
    #[inline]
    pub(crate) fn read_id_id_mmfr0_shareability_levels(&self) -> MemResult<u8> {
        todo!("read ID ID_MMFR0 Shareability_levels reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_MMFR0 Outermost_shareability: Indicates the outermost shareability domain implemented"]
    #[inline]
    pub(crate) fn read_id_id_mmfr0_outermost_shareability(
        &self,
    ) -> MemResult<u8> {
        todo ! ("read ID ID_MMFR0 Outermost_shareability reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_MMFR0 PMSA: Indicates support for a PMSA"]
    #[inline]
    pub(crate) fn read_id_id_mmfr0_pmsa(&self) -> MemResult<u8> {
        todo!("read ID ID_MMFR0 PMSA reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_MMFR1: Memory Model Features"]
    #[inline]
    pub(crate) fn read_id_id_mmfr1(&self) -> MemResult<u32> {
        todo!("read ID ID_MMFR1 reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "ID ID_MMFR2 WFI: Indicates support for wait-for-interrupt stalling. "]
    #[inline]
    pub(crate) fn read_id_id_mmfr2_wfi(&self) -> MemResult<u8> {
        todo!("read ID ID_MMFR2 WFI reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_MMFR3: Memory Model Features"]
    #[inline]
    pub(crate) fn read_id_id_mmfr3(&self) -> MemResult<u32> {
        todo!("read ID ID_MMFR3 reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "ID ID_ISAR0 Divide_instrs: Indicates the supported divide instructions"]
    #[inline]
    pub(crate) fn read_id_id_isar0_divide_instrs(&self) -> MemResult<u8> {
        todo!("read ID ID_ISAR0 Divide_instrs reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_ISAR0 Debug_instrs: Indicates the supported debug instructions"]
    #[inline]
    pub(crate) fn read_id_id_isar0_debug_instrs(&self) -> MemResult<u8> {
        todo!("read ID ID_ISAR0 Debug_instrs reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_ISAR0 Coproc_instrs: Indicates the supported coprocessor instructions"]
    #[inline]
    pub(crate) fn read_id_id_isar0_coproc_instrs(&self) -> MemResult<u8> {
        todo!("read ID ID_ISAR0 Coproc_instrs reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_ISAR0 CmpBranch_instrs: Indicates support for combined compare and branch instructions."]
    #[inline]
    pub(crate) fn read_id_id_isar0_cmpbranch_instrs(&self) -> MemResult<u8> {
        todo!("read ID ID_ISAR0 CmpBranch_instrs reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_ISAR0 Bitfield_instrs: Indicates support for bitfield instructions. "]
    #[inline]
    pub(crate) fn read_id_id_isar0_bitfield_instrs(&self) -> MemResult<u8> {
        todo!("read ID ID_ISAR0 Bitfield_instrs reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_ISAR0 BitCount_instrs: Indicates support for bit counting instructions. "]
    #[inline]
    pub(crate) fn read_id_id_isar0_bitcount_instrs(&self) -> MemResult<u8> {
        todo!("read ID ID_ISAR0 BitCount_instrs reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_ISAR1 Interwork_instrs: Indicates support for instructions that branch between ARM and Thumb code."]
    #[inline]
    pub(crate) fn read_id_id_isar1_interwork_instrs(&self) -> MemResult<u8> {
        todo!("read ID ID_ISAR1 Interwork_instrs reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_ISAR1 Immediate_instrs: Indicates support for immediate instructions. "]
    #[inline]
    pub(crate) fn read_id_id_isar1_immediate_instrs(&self) -> MemResult<u8> {
        todo!("read ID ID_ISAR1 Immediate_instrs reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_ISAR1 IfThen_instrs: Indicates support for IfThen instructions."]
    #[inline]
    pub(crate) fn read_id_id_isar1_ifthen_instrs(&self) -> MemResult<u8> {
        todo!("read ID ID_ISAR1 IfThen_instrs reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_ISAR1 Extend_instrs: Indicates support for sign or zero extend instructions. "]
    #[inline]
    pub(crate) fn read_id_id_isar1_extend_instrs(&self) -> MemResult<u8> {
        todo!("read ID ID_ISAR1 Extend_instrs reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_ISAR2 Reversal_instrs: Indicates the supported reversal instructions. "]
    #[inline]
    pub(crate) fn read_id_id_isar2_reversal_instrs(&self) -> MemResult<u8> {
        todo!("read ID ID_ISAR2 Reversal_instrs reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_ISAR2 MultU_instrs: Indicates the supported advanced unsigned multiply instructions."]
    #[inline]
    pub(crate) fn read_id_id_isar2_multu_instrs(&self) -> MemResult<u8> {
        todo!("read ID ID_ISAR2 MultU_instrs reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_ISAR2 MultS_instrs: Indicates the supported advanced signed multiply instructions."]
    #[inline]
    pub(crate) fn read_id_id_isar2_mults_instrs(&self) -> MemResult<u8> {
        todo!("read ID ID_ISAR2 MultS_instrs reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_ISAR2 Mult_instrs: Indicates the supported additional multiply instructions. "]
    #[inline]
    pub(crate) fn read_id_id_isar2_mult_instrs(&self) -> MemResult<u8> {
        todo!("read ID ID_ISAR2 Mult_instrs reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_ISAR2 MultiAccessInt_instrs: Indicates the supported multi-access interruptible instructions."]
    #[inline]
    pub(crate) fn read_id_id_isar2_multiaccessint_instrs(
        &self,
    ) -> MemResult<u8> {
        todo!(
            "read ID ID_ISAR2 MultiAccessInt_instrs reset value 0x00 mask 0x0f"
        )
    }
    #[doc = "ID ID_ISAR2 MemHint_instrs: Indicates the implemented Memory Hint instructions."]
    #[inline]
    pub(crate) fn read_id_id_isar2_memhint_instrs(&self) -> MemResult<u8> {
        todo!("read ID ID_ISAR2 MemHint_instrs reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_ISAR2 LoadStore_instrs: Indicates the supported additional load and store instructions. "]
    #[inline]
    pub(crate) fn read_id_id_isar2_loadstore_instrs(&self) -> MemResult<u8> {
        todo!("read ID ID_ISAR2 LoadStore_instrs reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_ISAR3 TrueNOP_instrs: Indicates support for true NOP instructions."]
    #[inline]
    pub(crate) fn read_id_id_isar3_truenop_instrs(&self) -> MemResult<u8> {
        todo!("read ID ID_ISAR3 TrueNOP_instrs reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_ISAR3 ThumbCopy_instrs: Indicates support for Thumb copy instructions. "]
    #[inline]
    pub(crate) fn read_id_id_isar3_thumbcopy_instrs(&self) -> MemResult<u8> {
        todo!("read ID ID_ISAR3 ThumbCopy_instrs reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_ISAR3 TabBranch_instrs: Indicates support for table branch instructions. "]
    #[inline]
    pub(crate) fn read_id_id_isar3_tabbranch_instrs(&self) -> MemResult<u8> {
        todo!("read ID ID_ISAR3 TabBranch_instrs reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_ISAR3 SynchPrim_instrs: Indicates support for synchronization primitive instructions."]
    #[inline]
    pub(crate) fn read_id_id_isar3_synchprim_instrs(&self) -> MemResult<u8> {
        todo!("read ID ID_ISAR3 SynchPrim_instrs reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_ISAR3 SVC_instrs: Indicates support for SVC instructions."]
    #[inline]
    pub(crate) fn read_id_id_isar3_svc_instrs(&self) -> MemResult<u8> {
        todo!("read ID ID_ISAR3 SVC_instrs reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_ISAR3 SIMD_instrs: Indicates support for Single Instruction Multiple Data (SIMD) instructions."]
    #[inline]
    pub(crate) fn read_id_id_isar3_simd_instrs(&self) -> MemResult<u8> {
        todo!("read ID ID_ISAR3 SIMD_instrs reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_ISAR3 Saturate_instrs: Indicates support for saturate instructions. "]
    #[inline]
    pub(crate) fn read_id_id_isar3_saturate_instrs(&self) -> MemResult<u8> {
        todo!("read ID ID_ISAR3 Saturate_instrs reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_ISAR4 PSR_M_instrs: Indicates support for saturate instructions. "]
    #[inline]
    pub(crate) fn read_id_id_isar4_psr_m_instrs(&self) -> MemResult<u8> {
        todo!("read ID ID_ISAR4 PSR_M_instrs reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_ISAR4 SynchPrim_instrs_frac: Indicates support for Synchronization Primitives"]
    #[inline]
    pub(crate) fn read_id_id_isar4_synchprim_instrs_frac(
        &self,
    ) -> MemResult<u8> {
        todo!(
            "read ID ID_ISAR4 SynchPrim_instrs_frac reset value 0x00 mask 0x0f"
        )
    }
    #[doc = "ID ID_ISAR4 Barrier_instrs: Indicates the supported barrier instructions. "]
    #[inline]
    pub(crate) fn read_id_id_isar4_barrier_instrs(&self) -> MemResult<u8> {
        todo!("read ID ID_ISAR4 Barrier_instrs reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_ISAR4 Writeback_instrs: Indicates support for Writeback addressing modes"]
    #[inline]
    pub(crate) fn read_id_id_isar4_writeback_instrs(&self) -> MemResult<u8> {
        todo!("read ID ID_ISAR4 Writeback_instrs reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_ISAR4 WithShifts_instrs: Indicates the support for instructions with shifts:"]
    #[inline]
    pub(crate) fn read_id_id_isar4_withshifts_instrs(&self) -> MemResult<u8> {
        todo!("read ID ID_ISAR4 WithShifts_instrs reset value 0x00 mask 0x0f")
    }
    #[doc = "ID ID_ISAR4 Unpriv_instrs: Indicates the supported unprivileged instructions"]
    #[inline]
    pub(crate) fn read_id_id_isar4_unpriv_instrs(&self) -> MemResult<u8> {
        todo!("read ID ID_ISAR4 Unpriv_instrs reset value 0x00 mask 0x0f")
    }
    #[doc = "FPE FPCCR ASPEN: When this bit is set to 1, execution of a floating-point instruction sets the CONTROL.FPCA bit to 1"]
    #[inline]
    pub(crate) fn read_fpe_fpccr_aspen(&self) -> MemResult<bool> {
        todo!("read FPE FPCCR ASPEN reset value false")
    }
    #[doc = "FPE FPCCR ASPEN: When this bit is set to 1, execution of a floating-point instruction sets the CONTROL.FPCA bit to 1"]
    #[inline]
    pub(crate) fn write_fpe_fpccr_aspen(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write FPE FPCCR ASPEN reset value false")
    }
    #[doc = "FPE FPCCR LSPEN: Enables lazy context save of FP state"]
    #[inline]
    pub(crate) fn read_fpe_fpccr_lspen(&self) -> MemResult<bool> {
        todo!("read FPE FPCCR LSPEN reset value false")
    }
    #[doc = "FPE FPCCR LSPEN: Enables lazy context save of FP state"]
    #[inline]
    pub(crate) fn write_fpe_fpccr_lspen(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write FPE FPCCR LSPEN reset value false")
    }
    #[doc = "FPE FPCCR MONRDY: Indicates whether the software executing when the processor allocated the FP stack frame was able to set the DebugMonitor exception to pending"]
    #[inline]
    pub(crate) fn read_fpe_fpccr_monrdy(&self) -> MemResult<bool> {
        todo!("read FPE FPCCR MONRDY reset value false")
    }
    #[doc = "FPE FPCCR MONRDY: Indicates whether the software executing when the processor allocated the FP stack frame was able to set the DebugMonitor exception to pending"]
    #[inline]
    pub(crate) fn write_fpe_fpccr_monrdy(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write FPE FPCCR MONRDY reset value false")
    }
    #[doc = "FPE FPCCR BFRDY: Indicates whether the software executing when the processor allocated the FP stack frame was able to set the BusFault exception to pending"]
    #[inline]
    pub(crate) fn read_fpe_fpccr_bfrdy(&self) -> MemResult<bool> {
        todo!("read FPE FPCCR BFRDY reset value false")
    }
    #[doc = "FPE FPCCR BFRDY: Indicates whether the software executing when the processor allocated the FP stack frame was able to set the BusFault exception to pending"]
    #[inline]
    pub(crate) fn write_fpe_fpccr_bfrdy(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write FPE FPCCR BFRDY reset value false")
    }
    #[doc = "FPE FPCCR MMRDY: Indicates whether the software executing when the processor allocated the FP stack frame was able to set the MemManage exception to pending"]
    #[inline]
    pub(crate) fn read_fpe_fpccr_mmrdy(&self) -> MemResult<bool> {
        todo!("read FPE FPCCR MMRDY reset value false")
    }
    #[doc = "FPE FPCCR MMRDY: Indicates whether the software executing when the processor allocated the FP stack frame was able to set the MemManage exception to pending"]
    #[inline]
    pub(crate) fn write_fpe_fpccr_mmrdy(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write FPE FPCCR MMRDY reset value false")
    }
    #[doc = "FPE FPCCR HFRDY: Indicates whether the software executing when the processor allocated the FP stack frame was able to set the HardFault exception to pending"]
    #[inline]
    pub(crate) fn read_fpe_fpccr_hfrdy(&self) -> MemResult<bool> {
        todo!("read FPE FPCCR HFRDY reset value false")
    }
    #[doc = "FPE FPCCR HFRDY: Indicates whether the software executing when the processor allocated the FP stack frame was able to set the HardFault exception to pending"]
    #[inline]
    pub(crate) fn write_fpe_fpccr_hfrdy(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write FPE FPCCR HFRDY reset value false")
    }
    #[doc = "FPE FPCCR THREAD: Indicates the processor mode when it allocated the FP stack frame"]
    #[inline]
    pub(crate) fn read_fpe_fpccr_thread(&self) -> MemResult<bool> {
        todo!("read FPE FPCCR THREAD reset value false")
    }
    #[doc = "FPE FPCCR THREAD: Indicates the processor mode when it allocated the FP stack frame"]
    #[inline]
    pub(crate) fn write_fpe_fpccr_thread(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write FPE FPCCR THREAD reset value false")
    }
    #[doc = "FPE FPCCR USER: Indicates the privilege level of the software executing when the processor allocated the FP stack frame"]
    #[inline]
    pub(crate) fn read_fpe_fpccr_user(&self) -> MemResult<bool> {
        todo!("read FPE FPCCR USER reset value false")
    }
    #[doc = "FPE FPCCR USER: Indicates the privilege level of the software executing when the processor allocated the FP stack frame"]
    #[inline]
    pub(crate) fn write_fpe_fpccr_user(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write FPE FPCCR USER reset value false")
    }
    #[doc = "FPE FPCCR LSPACT: Indicates whether Lazy preservation of the FP state is active"]
    #[inline]
    pub(crate) fn read_fpe_fpccr_lspact(&self) -> MemResult<bool> {
        todo!("read FPE FPCCR LSPACT reset value false")
    }
    #[doc = "FPE FPCCR LSPACT: Indicates whether Lazy preservation of the FP state is active"]
    #[inline]
    pub(crate) fn write_fpe_fpccr_lspact(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write FPE FPCCR LSPACT reset value false")
    }
    #[doc = "FPE FPCAR: Holds the location of the unpopulated floating-point register space allocated on an exception stack frame"]
    #[inline]
    pub(crate) fn read_fpe_fpcar(&self) -> MemResult<u32> {
        todo!("read FPE FPCAR reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "FPE FPCAR: Holds the location of the unpopulated floating-point register space allocated on an exception stack frame"]
    #[inline]
    pub(crate) fn write_fpe_fpcar(&mut self, _value: u32) -> MemResult<()> {
        todo!("write FPE FPCAR reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "FPE FPDSCR AHP: Default value for FPSCR.AHP"]
    #[inline]
    pub(crate) fn read_fpe_fpdscr_ahp(&self) -> MemResult<bool> {
        todo!("read FPE FPDSCR AHP reset value false")
    }
    #[doc = "FPE FPDSCR AHP: Default value for FPSCR.AHP"]
    #[inline]
    pub(crate) fn write_fpe_fpdscr_ahp(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write FPE FPDSCR AHP reset value false")
    }
    #[doc = "FPE FPDSCR DN: Default value for FPSCR.DN"]
    #[inline]
    pub(crate) fn read_fpe_fpdscr_dn(&self) -> MemResult<bool> {
        todo!("read FPE FPDSCR DN reset value false")
    }
    #[doc = "FPE FPDSCR DN: Default value for FPSCR.DN"]
    #[inline]
    pub(crate) fn write_fpe_fpdscr_dn(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write FPE FPDSCR DN reset value false")
    }
    #[doc = "FPE FPDSCR FZ: Default value for FPSCR.FZ"]
    #[inline]
    pub(crate) fn read_fpe_fpdscr_fz(&self) -> MemResult<bool> {
        todo!("read FPE FPDSCR FZ reset value false")
    }
    #[doc = "FPE FPDSCR FZ: Default value for FPSCR.FZ"]
    #[inline]
    pub(crate) fn write_fpe_fpdscr_fz(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write FPE FPDSCR FZ reset value false")
    }
    #[doc = "FPE FPDSCR RMode: Default value for FPSCR.RMode"]
    #[inline]
    pub(crate) fn read_fpe_fpdscr_rmode(&self) -> MemResult<u8> {
        todo!("read FPE FPDSCR RMode reset value 0x00 mask 0x03")
    }
    #[doc = "FPE FPDSCR RMode: Default value for FPSCR.RMode"]
    #[inline]
    pub(crate) fn write_fpe_fpdscr_rmode(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write FPE FPDSCR RMode reset value 0x00 mask 0x03")
    }
    #[doc = "FPE MVFR0 FP_Rounding_modes: Indicates the rounding modes supported by the FP floating-point hardware"]
    #[inline]
    pub(crate) fn read_fpe_mvfr0_fp_rounding_modes(&self) -> MemResult<u8> {
        todo!("read FPE MVFR0 FP_Rounding_modes reset value 0x00 mask 0x0f")
    }
    #[doc = "FPE MVFR0 Short_vectors: Indicates the hardware support for FP short vectors"]
    #[inline]
    pub(crate) fn read_fpe_mvfr0_short_vectors(&self) -> MemResult<u8> {
        todo!("read FPE MVFR0 Short_vectors reset value 0x00 mask 0x0f")
    }
    #[doc = "FPE MVFR0 Square_root: Indicates the hardware support for FP square root operations"]
    #[inline]
    pub(crate) fn read_fpe_mvfr0_square_root(&self) -> MemResult<u8> {
        todo!("read FPE MVFR0 Square_root reset value 0x00 mask 0x0f")
    }
    #[doc = "FPE MVFR0 Divide: Indicates the hardware support for FP divide operations"]
    #[inline]
    pub(crate) fn read_fpe_mvfr0_divide(&self) -> MemResult<u8> {
        todo!("read FPE MVFR0 Divide reset value 0x00 mask 0x0f")
    }
    #[doc = "FPE MVFR0 FP_exception_trapping: Indicates whether the FP hardware implementation supports exception trapping"]
    #[inline]
    pub(crate) fn read_fpe_mvfr0_fp_exception_trapping(&self) -> MemResult<u8> {
        todo!("read FPE MVFR0 FP_exception_trapping reset value 0x00 mask 0x0f")
    }
    #[doc = "FPE MVFR0 Double_precision: Indicates the hardware support for FP double-precision operations"]
    #[inline]
    pub(crate) fn read_fpe_mvfr0_double_precision(&self) -> MemResult<u8> {
        todo!("read FPE MVFR0 Double_precision reset value 0x00 mask 0x0f")
    }
    #[doc = "FPE MVFR0 Single_precision: Indicates the hardware support for FP single-precision operations"]
    #[inline]
    pub(crate) fn read_fpe_mvfr0_single_precision(&self) -> MemResult<u8> {
        todo!("read FPE MVFR0 Single_precision reset value 0x00 mask 0x0f")
    }
    #[doc = "FPE MVFR0 A_SIMD: Indicates the size of the FP register bank"]
    #[inline]
    pub(crate) fn read_fpe_mvfr0_a_simd(&self) -> MemResult<u8> {
        todo!("read FPE MVFR0 A_SIMD reset value 0x00 mask 0x0f")
    }
    #[doc = "FPE MVFR1 FP_fused_MAC: Indicates whether the FP supports fused multiply accumulate operations"]
    #[inline]
    pub(crate) fn read_fpe_mvfr1_fp_fused_mac(&self) -> MemResult<u8> {
        todo!("read FPE MVFR1 FP_fused_MAC reset value 0x00 mask 0x0f")
    }
    #[doc = "FPE MVFR1 FP_HPFP: Indicates whether the FP supports half-precision floating-point conversion operations"]
    #[inline]
    pub(crate) fn read_fpe_mvfr1_fp_hpfp(&self) -> MemResult<u8> {
        todo!("read FPE MVFR1 FP_HPFP reset value 0x00 mask 0x0f")
    }
    #[doc = "FPE MVFR1 D_NaN: Indicates whether the FP hardware implementation supports only the Default NaN mode"]
    #[inline]
    pub(crate) fn read_fpe_mvfr1_d_nan(&self) -> MemResult<u8> {
        todo!("read FPE MVFR1 D_NaN reset value 0x00 mask 0x0f")
    }
    #[doc = "FPE MVFR1 FtZ: Indicates whether the FP hardware implementation supports only the Flush-to-Zero mode of operation"]
    #[inline]
    pub(crate) fn read_fpe_mvfr1_ftz(&self) -> MemResult<u8> {
        todo!("read FPE MVFR1 FtZ reset value 0x00 mask 0x0f")
    }
    #[doc = "SysTick STCSR COUNTFLAG: Indicates whether the counter has counted to 0 since the last read of this register"]
    #[inline]
    pub(crate) fn read_systick_stcsr_countflag(&self) -> MemResult<bool> {
        todo!("read SysTick STCSR COUNTFLAG reset value false")
    }
    #[doc = "SysTick STCSR COUNTFLAG: Indicates whether the counter has counted to 0 since the last read of this register"]
    #[inline]
    pub(crate) fn write_systick_stcsr_countflag(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SysTick STCSR COUNTFLAG reset value false")
    }
    #[doc = "SysTick STCSR CLKSOURCE: Indicates the SysTick clock source"]
    #[inline]
    pub(crate) fn read_systick_stcsr_clksource(&self) -> MemResult<bool> {
        todo!("read SysTick STCSR CLKSOURCE reset value false")
    }
    #[doc = "SysTick STCSR CLKSOURCE: Indicates the SysTick clock source"]
    #[inline]
    pub(crate) fn write_systick_stcsr_clksource(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SysTick STCSR CLKSOURCE reset value false")
    }
    #[doc = "SysTick STCSR TICKINT: Indicates whether counting to 0 causes the status of the SysTick exception to change to pending"]
    #[inline]
    pub(crate) fn read_systick_stcsr_tickint(&self) -> MemResult<bool> {
        todo!("read SysTick STCSR TICKINT reset value false")
    }
    #[doc = "SysTick STCSR TICKINT: Indicates whether counting to 0 causes the status of the SysTick exception to change to pending"]
    #[inline]
    pub(crate) fn write_systick_stcsr_tickint(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SysTick STCSR TICKINT reset value false")
    }
    #[doc = "SysTick STCSR ENABLE: Indicates the enabled status of the SysTick counter"]
    #[inline]
    pub(crate) fn read_systick_stcsr_enable(&self) -> MemResult<bool> {
        todo!("read SysTick STCSR ENABLE reset value false")
    }
    #[doc = "SysTick STCSR ENABLE: Indicates the enabled status of the SysTick counter"]
    #[inline]
    pub(crate) fn write_systick_stcsr_enable(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SysTick STCSR ENABLE reset value false")
    }
    #[doc = "SysTick STRVR RELOAD: The value to load into the SYST_CVR register when the counter reaches 0"]
    #[inline]
    pub(crate) fn read_systick_strvr_reload(&self) -> MemResult<u32> {
        todo!("read SysTick STRVR RELOAD reset value 0x00 mask 0xffffff")
    }
    #[doc = "SysTick STRVR RELOAD: The value to load into the SYST_CVR register when the counter reaches 0"]
    #[inline]
    pub(crate) fn write_systick_strvr_reload(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write SysTick STRVR RELOAD reset value 0x00 mask 0xffffff")
    }
    #[doc = "SysTick STCVR CURRENT: This is the value of the counter at the time it is sampled"]
    #[inline]
    pub(crate) fn read_systick_stcvr_current(&self) -> MemResult<u32> {
        todo!("read SysTick STCVR CURRENT reset value 0x00 mask 0xffffff")
    }
    #[doc = "SysTick STCVR CURRENT: This is the value of the counter at the time it is sampled"]
    #[inline]
    pub(crate) fn write_systick_stcvr_current(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write SysTick STCVR CURRENT reset value 0x00 mask 0xffffff")
    }
    #[doc = "SysTick STCR NOREF: Indicates whether the IMPLEMENTATION DEFINED reference clock is provided"]
    #[inline]
    pub(crate) fn read_systick_stcr_noref(&self) -> MemResult<bool> {
        todo!("read SysTick STCR NOREF reset value false")
    }
    #[doc = "SysTick STCR SKEW: Indicates whether the 10ms calibration value is exact"]
    #[inline]
    pub(crate) fn read_systick_stcr_skew(&self) -> MemResult<bool> {
        todo!("read SysTick STCR SKEW reset value false")
    }
    #[doc = "SysTick STCR TENMS: Optionally, holds a reload value to be used for 10ms (100Hz) timing, subject to system clock skew errors"]
    #[inline]
    pub(crate) fn read_systick_stcr_tenms(&self) -> MemResult<u32> {
        todo!("read SysTick STCR TENMS reset value 0x00 mask 0xffffff")
    }
    #[doc = "NVIC ICTR INTLINESNUM: The total number of interrupt lines supported n, defined in groups of\n32. That is, the total number of interrupt lines is up to (32*(INTLINESNUM+1)). However,\nthe absolute maximum number of interrupts is 496, corresponding to the INTLINESNUM\nvalue 0b1111."]
    #[inline]
    pub(crate) fn read_nvic_ictr_intlinesnum(&self) -> MemResult<u8> {
        todo!("read NVIC ICTR INTLINESNUM reset value 0x00 mask 0x0f")
    }
    #[doc = "NVIC NVIC_ISER0: Enables, or reads the enable state of a group of interrupts"]
    #[inline]
    pub(crate) fn read_nvic_nvic_iser0(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_enable(0))
    }
    #[doc = "NVIC NVIC_ISER0: Enables, or reads the enable state of a group of interrupts"]
    #[inline]
    pub(crate) fn write_nvic_nvic_iser0(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.interrupts.nvic_set_enable(0, _value))
    }
    #[doc = "NVIC NVIC_ISER1: Enables, or reads the enable state of a group of interrupts"]
    #[inline]
    pub(crate) fn read_nvic_nvic_iser1(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_enable(1))
    }
    #[doc = "NVIC NVIC_ISER1: Enables, or reads the enable state of a group of interrupts"]
    #[inline]
    pub(crate) fn write_nvic_nvic_iser1(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.interrupts.nvic_set_enable(1, _value))
    }
    #[doc = "NVIC NVIC_ISER2: Enables, or reads the enable state of a group of interrupts"]
    #[inline]
    pub(crate) fn read_nvic_nvic_iser2(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_enable(2))
    }
    #[doc = "NVIC NVIC_ISER2: Enables, or reads the enable state of a group of interrupts"]
    #[inline]
    pub(crate) fn write_nvic_nvic_iser2(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.interrupts.nvic_set_enable(2, _value))
    }
    #[doc = "NVIC NVIC_ISER3: Enables, or reads the enable state of a group of interrupts"]
    #[inline]
    pub(crate) fn read_nvic_nvic_iser3(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_enable(3))
    }
    #[doc = "NVIC NVIC_ISER3: Enables, or reads the enable state of a group of interrupts"]
    #[inline]
    pub(crate) fn write_nvic_nvic_iser3(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.interrupts.nvic_set_enable(3, _value))
    }
    #[doc = "NVIC NVIC_ISER4: Enables, or reads the enable state of a group of interrupts"]
    #[inline]
    pub(crate) fn read_nvic_nvic_iser4(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_enable(4))
    }
    #[doc = "NVIC NVIC_ISER4: Enables, or reads the enable state of a group of interrupts"]
    #[inline]
    pub(crate) fn write_nvic_nvic_iser4(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.interrupts.nvic_set_enable(4, _value))
    }
    #[doc = "NVIC NVIC_ISER5: Enables, or reads the enable state of a group of interrupts"]
    #[inline]
    pub(crate) fn read_nvic_nvic_iser5(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_enable(5))
    }
    #[doc = "NVIC NVIC_ISER5: Enables, or reads the enable state of a group of interrupts"]
    #[inline]
    pub(crate) fn write_nvic_nvic_iser5(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.interrupts.nvic_set_enable(5, _value))
    }
    #[doc = "NVIC NVIC_ISER6: Enables, or reads the enable state of a group of interrupts"]
    #[inline]
    pub(crate) fn read_nvic_nvic_iser6(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_enable(6))
    }
    #[doc = "NVIC NVIC_ISER6: Enables, or reads the enable state of a group of interrupts"]
    #[inline]
    pub(crate) fn write_nvic_nvic_iser6(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.interrupts.nvic_set_enable(6, _value))
    }
    #[doc = "NVIC NVIC_ISER7: Enables, or reads the enable state of a group of interrupts"]
    #[inline]
    pub(crate) fn read_nvic_nvic_iser7(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_enable(7))
    }
    #[doc = "NVIC NVIC_ISER7: Enables, or reads the enable state of a group of interrupts"]
    #[inline]
    pub(crate) fn write_nvic_nvic_iser7(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.interrupts.nvic_set_enable(7, _value))
    }
    #[doc = "NVIC NVIC_ICER0: Disables, or reads the enable state of a group of interrupts"]
    #[inline]
    pub(crate) fn read_nvic_nvic_icer0(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_enable(0))
    }
    #[doc = "NVIC NVIC_ICER0: Disables, or reads the enable state of a group of interrupts"]
    #[inline]
    pub(crate) fn write_nvic_nvic_icer0(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.interrupts.nvic_clr_enable(0, _value))
    }
    #[doc = "NVIC NVIC_ICER1: Disables, or reads the enable state of a group of interrupts"]
    #[inline]
    pub(crate) fn read_nvic_nvic_icer1(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_enable(1))
    }
    #[doc = "NVIC NVIC_ICER1: Disables, or reads the enable state of a group of interrupts"]
    #[inline]
    pub(crate) fn write_nvic_nvic_icer1(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.interrupts.nvic_clr_enable(1, _value))
    }
    #[doc = "NVIC NVIC_ICER2: Disables, or reads the enable state of a group of interrupts"]
    #[inline]
    pub(crate) fn read_nvic_nvic_icer2(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_enable(2))
    }
    #[doc = "NVIC NVIC_ICER2: Disables, or reads the enable state of a group of interrupts"]
    #[inline]
    pub(crate) fn write_nvic_nvic_icer2(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.interrupts.nvic_clr_enable(2, _value))
    }
    #[doc = "NVIC NVIC_ICER3: Disables, or reads the enable state of a group of interrupts"]
    #[inline]
    pub(crate) fn read_nvic_nvic_icer3(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_enable(3))
    }
    #[doc = "NVIC NVIC_ICER3: Disables, or reads the enable state of a group of interrupts"]
    #[inline]
    pub(crate) fn write_nvic_nvic_icer3(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.interrupts.nvic_clr_enable(3, _value))
    }
    #[doc = "NVIC NVIC_ICER4: Disables, or reads the enable state of a group of interrupts"]
    #[inline]
    pub(crate) fn read_nvic_nvic_icer4(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_enable(4))
    }
    #[doc = "NVIC NVIC_ICER4: Disables, or reads the enable state of a group of interrupts"]
    #[inline]
    pub(crate) fn write_nvic_nvic_icer4(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.interrupts.nvic_clr_enable(4, _value))
    }
    #[doc = "NVIC NVIC_ICER5: Disables, or reads the enable state of a group of interrupts"]
    #[inline]
    pub(crate) fn read_nvic_nvic_icer5(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_enable(5))
    }
    #[doc = "NVIC NVIC_ICER5: Disables, or reads the enable state of a group of interrupts"]
    #[inline]
    pub(crate) fn write_nvic_nvic_icer5(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.interrupts.nvic_clr_enable(5, _value))
    }
    #[doc = "NVIC NVIC_ICER6: Disables, or reads the enable state of a group of interrupts"]
    #[inline]
    pub(crate) fn read_nvic_nvic_icer6(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_enable(6))
    }
    #[doc = "NVIC NVIC_ICER6: Disables, or reads the enable state of a group of interrupts"]
    #[inline]
    pub(crate) fn write_nvic_nvic_icer6(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.interrupts.nvic_clr_enable(6, _value))
    }
    #[doc = "NVIC NVIC_ICER7: Disables, or reads the enable state of a group of interrupts"]
    #[inline]
    pub(crate) fn read_nvic_nvic_icer7(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_enable(7))
    }
    #[doc = "NVIC NVIC_ICER7: Disables, or reads the enable state of a group of interrupts"]
    #[inline]
    pub(crate) fn write_nvic_nvic_icer7(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.interrupts.nvic_clr_enable(7, _value))
    }
    #[doc = "NVIC NVIC_ISPR0: For a group of interrupts, changes interrupt status to pending, or shows the current pending\nstatus"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ispr0(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_pending(0))
    }
    #[doc = "NVIC NVIC_ISPR0: For a group of interrupts, changes interrupt status to pending, or shows the current pending\nstatus"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ispr0(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.interrupts.nvic_set_pending(0, _value))
    }
    #[doc = "NVIC NVIC_ISPR1: For a group of interrupts, changes interrupt status to pending, or shows the current pending\nstatus"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ispr1(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_pending(1))
    }
    #[doc = "NVIC NVIC_ISPR1: For a group of interrupts, changes interrupt status to pending, or shows the current pending\nstatus"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ispr1(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.interrupts.nvic_set_pending(1, _value))
    }
    #[doc = "NVIC NVIC_ISPR2: For a group of interrupts, changes interrupt status to pending, or shows the current pending\nstatus"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ispr2(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_pending(2))
    }
    #[doc = "NVIC NVIC_ISPR2: For a group of interrupts, changes interrupt status to pending, or shows the current pending\nstatus"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ispr2(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.interrupts.nvic_set_pending(2, _value))
    }
    #[doc = "NVIC NVIC_ISPR3: For a group of interrupts, changes interrupt status to pending, or shows the current pending\nstatus"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ispr3(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_pending(3))
    }
    #[doc = "NVIC NVIC_ISPR3: For a group of interrupts, changes interrupt status to pending, or shows the current pending\nstatus"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ispr3(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.interrupts.nvic_set_pending(3, _value))
    }
    #[doc = "NVIC NVIC_ISPR4: For a group of interrupts, changes interrupt status to pending, or shows the current pending\nstatus"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ispr4(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_pending(4))
    }
    #[doc = "NVIC NVIC_ISPR4: For a group of interrupts, changes interrupt status to pending, or shows the current pending\nstatus"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ispr4(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.interrupts.nvic_set_pending(4, _value))
    }
    #[doc = "NVIC NVIC_ISPR5: For a group of interrupts, changes interrupt status to pending, or shows the current pending\nstatus"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ispr5(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_pending(5))
    }
    #[doc = "NVIC NVIC_ISPR5: For a group of interrupts, changes interrupt status to pending, or shows the current pending\nstatus"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ispr5(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.interrupts.nvic_set_pending(5, _value))
    }
    #[doc = "NVIC NVIC_ISPR6: For a group of interrupts, changes interrupt status to pending, or shows the current pending\nstatus"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ispr6(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_pending(6))
    }
    #[doc = "NVIC NVIC_ISPR6: For a group of interrupts, changes interrupt status to pending, or shows the current pending\nstatus"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ispr6(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.interrupts.nvic_set_pending(6, _value))
    }
    #[doc = "NVIC NVIC_ISPR7: For a group of interrupts, changes interrupt status to pending, or shows the current pending\nstatus"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ispr7(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_pending(7))
    }
    #[doc = "NVIC NVIC_ISPR7: For a group of interrupts, changes interrupt status to pending, or shows the current pending\nstatus"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ispr7(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.interrupts.nvic_set_pending(7, _value))
    }
    #[doc = "NVIC NVIC_ICPR0: For a group of interrupts, clears the interrupt pending status, or shows the current pending\nstatus"]
    #[inline]
    pub(crate) fn read_nvic_nvic_icpr0(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_pending(0))
    }
    #[doc = "NVIC NVIC_ICPR0: For a group of interrupts, clears the interrupt pending status, or shows the current pending\nstatus"]
    #[inline]
    pub(crate) fn write_nvic_nvic_icpr0(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.interrupts.nvic_clr_pending(0, _value))
    }
    #[doc = "NVIC NVIC_ICPR1: For a group of interrupts, clears the interrupt pending status, or shows the current pending\nstatus"]
    #[inline]
    pub(crate) fn read_nvic_nvic_icpr1(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_pending(1))
    }
    #[doc = "NVIC NVIC_ICPR1: For a group of interrupts, clears the interrupt pending status, or shows the current pending\nstatus"]
    #[inline]
    pub(crate) fn write_nvic_nvic_icpr1(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.interrupts.nvic_clr_pending(1, _value))
    }
    #[doc = "NVIC NVIC_ICPR2: For a group of interrupts, clears the interrupt pending status, or shows the current pending\nstatus"]
    #[inline]
    pub(crate) fn read_nvic_nvic_icpr2(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_pending(2))
    }
    #[doc = "NVIC NVIC_ICPR2: For a group of interrupts, clears the interrupt pending status, or shows the current pending\nstatus"]
    #[inline]
    pub(crate) fn write_nvic_nvic_icpr2(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.interrupts.nvic_clr_pending(2, _value))
    }
    #[doc = "NVIC NVIC_ICPR3: For a group of interrupts, clears the interrupt pending status, or shows the current pending\nstatus"]
    #[inline]
    pub(crate) fn read_nvic_nvic_icpr3(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_pending(3))
    }
    #[doc = "NVIC NVIC_ICPR3: For a group of interrupts, clears the interrupt pending status, or shows the current pending\nstatus"]
    #[inline]
    pub(crate) fn write_nvic_nvic_icpr3(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.interrupts.nvic_clr_pending(3, _value))
    }
    #[doc = "NVIC NVIC_ICPR4: For a group of interrupts, clears the interrupt pending status, or shows the current pending\nstatus"]
    #[inline]
    pub(crate) fn read_nvic_nvic_icpr4(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_pending(4))
    }
    #[doc = "NVIC NVIC_ICPR4: For a group of interrupts, clears the interrupt pending status, or shows the current pending\nstatus"]
    #[inline]
    pub(crate) fn write_nvic_nvic_icpr4(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.interrupts.nvic_clr_pending(4, _value))
    }
    #[doc = "NVIC NVIC_ICPR5: For a group of interrupts, clears the interrupt pending status, or shows the current pending\nstatus"]
    #[inline]
    pub(crate) fn read_nvic_nvic_icpr5(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_pending(5))
    }
    #[doc = "NVIC NVIC_ICPR5: For a group of interrupts, clears the interrupt pending status, or shows the current pending\nstatus"]
    #[inline]
    pub(crate) fn write_nvic_nvic_icpr5(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.interrupts.nvic_clr_pending(5, _value))
    }
    #[doc = "NVIC NVIC_ICPR6: For a group of interrupts, clears the interrupt pending status, or shows the current pending\nstatus"]
    #[inline]
    pub(crate) fn read_nvic_nvic_icpr6(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_pending(6))
    }
    #[doc = "NVIC NVIC_ICPR6: For a group of interrupts, clears the interrupt pending status, or shows the current pending\nstatus"]
    #[inline]
    pub(crate) fn write_nvic_nvic_icpr6(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.interrupts.nvic_clr_pending(6, _value))
    }
    #[doc = "NVIC NVIC_ICPR7: For a group of interrupts, clears the interrupt pending status, or shows the current pending\nstatus"]
    #[inline]
    pub(crate) fn read_nvic_nvic_icpr7(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_pending(7))
    }
    #[doc = "NVIC NVIC_ICPR7: For a group of interrupts, clears the interrupt pending status, or shows the current pending\nstatus"]
    #[inline]
    pub(crate) fn write_nvic_nvic_icpr7(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.interrupts.nvic_clr_pending(7, _value))
    }
    #[doc = "NVIC NVIC_IABR0: For a group of 32 interrupts, shows whether each interrupt is active"]
    #[inline]
    pub(crate) fn read_nvic_nvic_iabr0(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_enable(0))
    }
    #[doc = "NVIC NVIC_IABR0: For a group of 32 interrupts, shows whether each interrupt is active"]
    #[inline]
    pub(crate) fn write_nvic_nvic_iabr0(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IABR0 reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "NVIC NVIC_IABR1: For a group of 32 interrupts, shows whether each interrupt is active"]
    #[inline]
    pub(crate) fn read_nvic_nvic_iabr1(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_enable(1))
    }
    #[doc = "NVIC NVIC_IABR1: For a group of 32 interrupts, shows whether each interrupt is active"]
    #[inline]
    pub(crate) fn write_nvic_nvic_iabr1(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IABR1 reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "NVIC NVIC_IABR2: For a group of 32 interrupts, shows whether each interrupt is active"]
    #[inline]
    pub(crate) fn read_nvic_nvic_iabr2(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_enable(2))
    }
    #[doc = "NVIC NVIC_IABR2: For a group of 32 interrupts, shows whether each interrupt is active"]
    #[inline]
    pub(crate) fn write_nvic_nvic_iabr2(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IABR2 reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "NVIC NVIC_IABR3: For a group of 32 interrupts, shows whether each interrupt is active"]
    #[inline]
    pub(crate) fn read_nvic_nvic_iabr3(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_enable(3))
    }
    #[doc = "NVIC NVIC_IABR3: For a group of 32 interrupts, shows whether each interrupt is active"]
    #[inline]
    pub(crate) fn write_nvic_nvic_iabr3(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IABR3 reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "NVIC NVIC_IABR4: For a group of 32 interrupts, shows whether each interrupt is active"]
    #[inline]
    pub(crate) fn read_nvic_nvic_iabr4(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_enable(4))
    }
    #[doc = "NVIC NVIC_IABR4: For a group of 32 interrupts, shows whether each interrupt is active"]
    #[inline]
    pub(crate) fn write_nvic_nvic_iabr4(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IABR4 reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "NVIC NVIC_IABR5: For a group of 32 interrupts, shows whether each interrupt is active"]
    #[inline]
    pub(crate) fn read_nvic_nvic_iabr5(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_enable(5))
    }
    #[doc = "NVIC NVIC_IABR5: For a group of 32 interrupts, shows whether each interrupt is active"]
    #[inline]
    pub(crate) fn write_nvic_nvic_iabr5(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IABR5 reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "NVIC NVIC_IABR6: For a group of 32 interrupts, shows whether each interrupt is active"]
    #[inline]
    pub(crate) fn read_nvic_nvic_iabr6(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_enable(6))
    }
    #[doc = "NVIC NVIC_IABR6: For a group of 32 interrupts, shows whether each interrupt is active"]
    #[inline]
    pub(crate) fn write_nvic_nvic_iabr6(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IABR6 reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "NVIC NVIC_IABR7: For a group of 32 interrupts, shows whether each interrupt is active"]
    #[inline]
    pub(crate) fn read_nvic_nvic_iabr7(&self) -> MemResult<u32> {
        Ok(self.interrupts.nvic_enable(7))
    }
    #[doc = "NVIC NVIC_IABR7: For a group of 32 interrupts, shows whether each interrupt is active"]
    #[inline]
    pub(crate) fn write_nvic_nvic_iabr7(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IABR7 reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "NVIC NVIC_IPR0 PRI_N3: Priority of interrupt 3"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr0_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 0) + 3))
    }
    #[doc = "NVIC NVIC_IPR0 PRI_N3: Priority of interrupt 3"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr0_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 0) + 3, _value))
    }
    #[doc = "NVIC NVIC_IPR0 PRI_N2: Priority of interrupt 2"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr0_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 0) + 2))
    }
    #[doc = "NVIC NVIC_IPR0 PRI_N2: Priority of interrupt 2"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr0_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 0) + 2, _value))
    }
    #[doc = "NVIC NVIC_IPR0 PRI_N1: Priority of interrupt 1"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr0_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 0) + 1))
    }
    #[doc = "NVIC NVIC_IPR0 PRI_N1: Priority of interrupt 1"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr0_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 0) + 1, _value))
    }
    #[doc = "NVIC NVIC_IPR0 PRI_N0: Priority of interrupt 0"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr0_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 0) + 0))
    }
    #[doc = "NVIC NVIC_IPR0 PRI_N0: Priority of interrupt 0"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr0_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 0) + 0, _value))
    }
    #[doc = "NVIC NVIC_IPR1 PRI_N3: Priority of interrupt 7"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr1_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 1) + 3))
    }
    #[doc = "NVIC NVIC_IPR1 PRI_N3: Priority of interrupt 7"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr1_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 1) + 3, _value))
    }
    #[doc = "NVIC NVIC_IPR1 PRI_N2: Priority of interrupt 6"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr1_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 1) + 2))
    }
    #[doc = "NVIC NVIC_IPR1 PRI_N2: Priority of interrupt 6"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr1_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 1) + 2, _value))
    }
    #[doc = "NVIC NVIC_IPR1 PRI_N1: Priority of interrupt 5"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr1_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 1) + 1))
    }
    #[doc = "NVIC NVIC_IPR1 PRI_N1: Priority of interrupt 5"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr1_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 1) + 1, _value))
    }
    #[doc = "NVIC NVIC_IPR1 PRI_N0: Priority of interrupt 4"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr1_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 1) + 0))
    }
    #[doc = "NVIC NVIC_IPR1 PRI_N0: Priority of interrupt 4"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr1_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 1) + 0, _value))
    }
    #[doc = "NVIC NVIC_IPR2 PRI_N3: Priority of interrupt 11"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr2_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 2) + 3))
    }
    #[doc = "NVIC NVIC_IPR2 PRI_N3: Priority of interrupt 11"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr2_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 2) + 3, _value))
    }
    #[doc = "NVIC NVIC_IPR2 PRI_N2: Priority of interrupt 10"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr2_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 2) + 2))
    }
    #[doc = "NVIC NVIC_IPR2 PRI_N2: Priority of interrupt 10"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr2_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 2) + 2, _value))
    }
    #[doc = "NVIC NVIC_IPR2 PRI_N1: Priority of interrupt 9"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr2_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 2) + 1))
    }
    #[doc = "NVIC NVIC_IPR2 PRI_N1: Priority of interrupt 9"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr2_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 2) + 1, _value))
    }
    #[doc = "NVIC NVIC_IPR2 PRI_N0: Priority of interrupt 8"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr2_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 2) + 0))
    }
    #[doc = "NVIC NVIC_IPR2 PRI_N0: Priority of interrupt 8"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr2_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 2) + 0, _value))
    }
    #[doc = "NVIC NVIC_IPR3 PRI_N3: Priority of interrupt 15"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr3_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 3) + 3))
    }
    #[doc = "NVIC NVIC_IPR3 PRI_N3: Priority of interrupt 15"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr3_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 3) + 3, _value))
    }
    #[doc = "NVIC NVIC_IPR3 PRI_N2: Priority of interrupt 14"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr3_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 3) + 2))
    }
    #[doc = "NVIC NVIC_IPR3 PRI_N2: Priority of interrupt 14"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr3_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 3) + 2, _value))
    }
    #[doc = "NVIC NVIC_IPR3 PRI_N1: Priority of interrupt 13"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr3_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 3) + 1))
    }
    #[doc = "NVIC NVIC_IPR3 PRI_N1: Priority of interrupt 13"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr3_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 3) + 1, _value))
    }
    #[doc = "NVIC NVIC_IPR3 PRI_N0: Priority of interrupt 12"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr3_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 3) + 0))
    }
    #[doc = "NVIC NVIC_IPR3 PRI_N0: Priority of interrupt 12"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr3_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 3) + 0, _value))
    }
    #[doc = "NVIC NVIC_IPR4 PRI_N3: Priority of interrupt 19"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr4_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 4) + 3))
    }
    #[doc = "NVIC NVIC_IPR4 PRI_N3: Priority of interrupt 19"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr4_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 4) + 3, _value))
    }
    #[doc = "NVIC NVIC_IPR4 PRI_N2: Priority of interrupt 18"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr4_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 4) + 2))
    }
    #[doc = "NVIC NVIC_IPR4 PRI_N2: Priority of interrupt 18"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr4_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 4) + 2, _value))
    }
    #[doc = "NVIC NVIC_IPR4 PRI_N1: Priority of interrupt 17"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr4_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 4) + 1))
    }
    #[doc = "NVIC NVIC_IPR4 PRI_N1: Priority of interrupt 17"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr4_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 4) + 1, _value))
    }
    #[doc = "NVIC NVIC_IPR4 PRI_N0: Priority of interrupt 16"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr4_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 4) + 0))
    }
    #[doc = "NVIC NVIC_IPR4 PRI_N0: Priority of interrupt 16"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr4_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 4) + 0, _value))
    }
    #[doc = "NVIC NVIC_IPR5 PRI_N3: Priority of interrupt 23"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr5_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 5) + 3))
    }
    #[doc = "NVIC NVIC_IPR5 PRI_N3: Priority of interrupt 23"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr5_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 5) + 3, _value))
    }
    #[doc = "NVIC NVIC_IPR5 PRI_N2: Priority of interrupt 22"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr5_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 5) + 2))
    }
    #[doc = "NVIC NVIC_IPR5 PRI_N2: Priority of interrupt 22"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr5_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 5) + 2, _value))
    }
    #[doc = "NVIC NVIC_IPR5 PRI_N1: Priority of interrupt 21"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr5_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 5) + 1))
    }
    #[doc = "NVIC NVIC_IPR5 PRI_N1: Priority of interrupt 21"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr5_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 5) + 1, _value))
    }
    #[doc = "NVIC NVIC_IPR5 PRI_N0: Priority of interrupt 20"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr5_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 5) + 0))
    }
    #[doc = "NVIC NVIC_IPR5 PRI_N0: Priority of interrupt 20"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr5_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 5) + 0, _value))
    }
    #[doc = "NVIC NVIC_IPR6 PRI_N3: Priority of interrupt 27"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr6_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 6) + 3))
    }
    #[doc = "NVIC NVIC_IPR6 PRI_N3: Priority of interrupt 27"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr6_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 6) + 3, _value))
    }
    #[doc = "NVIC NVIC_IPR6 PRI_N2: Priority of interrupt 26"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr6_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 6) + 2))
    }
    #[doc = "NVIC NVIC_IPR6 PRI_N2: Priority of interrupt 26"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr6_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 6) + 2, _value))
    }
    #[doc = "NVIC NVIC_IPR6 PRI_N1: Priority of interrupt 25"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr6_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 6) + 1))
    }
    #[doc = "NVIC NVIC_IPR6 PRI_N1: Priority of interrupt 25"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr6_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 6) + 1, _value))
    }
    #[doc = "NVIC NVIC_IPR6 PRI_N0: Priority of interrupt 24"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr6_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 6) + 0))
    }
    #[doc = "NVIC NVIC_IPR6 PRI_N0: Priority of interrupt 24"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr6_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 6) + 0, _value))
    }
    #[doc = "NVIC NVIC_IPR7 PRI_N3: Priority of interrupt 31"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr7_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 7) + 3))
    }
    #[doc = "NVIC NVIC_IPR7 PRI_N3: Priority of interrupt 31"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr7_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 7) + 3, _value))
    }
    #[doc = "NVIC NVIC_IPR7 PRI_N2: Priority of interrupt 30"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr7_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 7) + 2))
    }
    #[doc = "NVIC NVIC_IPR7 PRI_N2: Priority of interrupt 30"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr7_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 7) + 2, _value))
    }
    #[doc = "NVIC NVIC_IPR7 PRI_N1: Priority of interrupt 29"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr7_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 7) + 1))
    }
    #[doc = "NVIC NVIC_IPR7 PRI_N1: Priority of interrupt 29"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr7_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 7) + 1, _value))
    }
    #[doc = "NVIC NVIC_IPR7 PRI_N0: Priority of interrupt 28"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr7_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 7) + 0))
    }
    #[doc = "NVIC NVIC_IPR7 PRI_N0: Priority of interrupt 28"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr7_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 7) + 0, _value))
    }
    #[doc = "NVIC NVIC_IPR8 PRI_N3: Priority of interrupt 35"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr8_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 8) + 3))
    }
    #[doc = "NVIC NVIC_IPR8 PRI_N3: Priority of interrupt 35"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr8_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 8) + 3, _value))
    }
    #[doc = "NVIC NVIC_IPR8 PRI_N2: Priority of interrupt 34"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr8_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 8) + 2))
    }
    #[doc = "NVIC NVIC_IPR8 PRI_N2: Priority of interrupt 34"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr8_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 8) + 2, _value))
    }
    #[doc = "NVIC NVIC_IPR8 PRI_N1: Priority of interrupt 33"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr8_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 8) + 1))
    }
    #[doc = "NVIC NVIC_IPR8 PRI_N1: Priority of interrupt 33"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr8_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 8) + 1, _value))
    }
    #[doc = "NVIC NVIC_IPR8 PRI_N0: Priority of interrupt 32"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr8_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 8) + 0))
    }
    #[doc = "NVIC NVIC_IPR8 PRI_N0: Priority of interrupt 32"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr8_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 8) + 0, _value))
    }
    #[doc = "NVIC NVIC_IPR9 PRI_N3: Priority of interrupt 39"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr9_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 9) + 3))
    }
    #[doc = "NVIC NVIC_IPR9 PRI_N3: Priority of interrupt 39"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr9_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 9) + 3, _value))
    }
    #[doc = "NVIC NVIC_IPR9 PRI_N2: Priority of interrupt 38"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr9_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 9) + 2))
    }
    #[doc = "NVIC NVIC_IPR9 PRI_N2: Priority of interrupt 38"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr9_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 9) + 2, _value))
    }
    #[doc = "NVIC NVIC_IPR9 PRI_N1: Priority of interrupt 37"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr9_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 9) + 1))
    }
    #[doc = "NVIC NVIC_IPR9 PRI_N1: Priority of interrupt 37"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr9_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 9) + 1, _value))
    }
    #[doc = "NVIC NVIC_IPR9 PRI_N0: Priority of interrupt 36"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr9_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 9) + 0))
    }
    #[doc = "NVIC NVIC_IPR9 PRI_N0: Priority of interrupt 36"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr9_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts.set_priority((4 * 9) + 0, _value))
    }
    #[doc = "NVIC NVIC_IPR10 PRI_N3: Priority of interrupt 43"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr10_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 10) + 3))
    }
    #[doc = "NVIC NVIC_IPR10 PRI_N3: Priority of interrupt 43"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr10_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR10 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR10 PRI_N2: Priority of interrupt 42"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr10_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 10) + 2))
    }
    #[doc = "NVIC NVIC_IPR10 PRI_N2: Priority of interrupt 42"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr10_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR10 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR10 PRI_N1: Priority of interrupt 41"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr10_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 10) + 1))
    }
    #[doc = "NVIC NVIC_IPR10 PRI_N1: Priority of interrupt 41"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr10_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR10 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR10 PRI_N0: Priority of interrupt 40"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr10_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 10) + 0))
    }
    #[doc = "NVIC NVIC_IPR10 PRI_N0: Priority of interrupt 40"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr10_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR10 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR11 PRI_N3: Priority of interrupt 47"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr11_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 11) + 3))
    }
    #[doc = "NVIC NVIC_IPR11 PRI_N3: Priority of interrupt 47"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr11_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR11 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR11 PRI_N2: Priority of interrupt 46"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr11_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 11) + 2))
    }
    #[doc = "NVIC NVIC_IPR11 PRI_N2: Priority of interrupt 46"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr11_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR11 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR11 PRI_N1: Priority of interrupt 45"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr11_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 11) + 1))
    }
    #[doc = "NVIC NVIC_IPR11 PRI_N1: Priority of interrupt 45"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr11_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR11 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR11 PRI_N0: Priority of interrupt 44"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr11_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 11) + 0))
    }
    #[doc = "NVIC NVIC_IPR11 PRI_N0: Priority of interrupt 44"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr11_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR11 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR12 PRI_N3: Priority of interrupt 51"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr12_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 12) + 3))
    }
    #[doc = "NVIC NVIC_IPR12 PRI_N3: Priority of interrupt 51"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr12_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR12 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR12 PRI_N2: Priority of interrupt 50"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr12_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 12) + 2))
    }
    #[doc = "NVIC NVIC_IPR12 PRI_N2: Priority of interrupt 50"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr12_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR12 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR12 PRI_N1: Priority of interrupt 49"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr12_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 12) + 1))
    }
    #[doc = "NVIC NVIC_IPR12 PRI_N1: Priority of interrupt 49"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr12_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR12 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR12 PRI_N0: Priority of interrupt 48"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr12_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 12) + 0))
    }
    #[doc = "NVIC NVIC_IPR12 PRI_N0: Priority of interrupt 48"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr12_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR12 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR13 PRI_N3: Priority of interrupt 55"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr13_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 13) + 3))
    }
    #[doc = "NVIC NVIC_IPR13 PRI_N3: Priority of interrupt 55"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr13_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR13 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR13 PRI_N2: Priority of interrupt 54"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr13_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 13) + 2))
    }
    #[doc = "NVIC NVIC_IPR13 PRI_N2: Priority of interrupt 54"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr13_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR13 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR13 PRI_N1: Priority of interrupt 53"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr13_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 13) + 1))
    }
    #[doc = "NVIC NVIC_IPR13 PRI_N1: Priority of interrupt 53"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr13_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR13 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR13 PRI_N0: Priority of interrupt 52"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr13_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 13) + 0))
    }
    #[doc = "NVIC NVIC_IPR13 PRI_N0: Priority of interrupt 52"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr13_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR13 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR14 PRI_N3: Priority of interrupt 59"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr14_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 14) + 3))
    }
    #[doc = "NVIC NVIC_IPR14 PRI_N3: Priority of interrupt 59"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr14_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR14 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR14 PRI_N2: Priority of interrupt 58"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr14_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 14) + 2))
    }
    #[doc = "NVIC NVIC_IPR14 PRI_N2: Priority of interrupt 58"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr14_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR14 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR14 PRI_N1: Priority of interrupt 57"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr14_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 14) + 1))
    }
    #[doc = "NVIC NVIC_IPR14 PRI_N1: Priority of interrupt 57"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr14_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR14 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR14 PRI_N0: Priority of interrupt 56"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr14_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 14) + 0))
    }
    #[doc = "NVIC NVIC_IPR14 PRI_N0: Priority of interrupt 56"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr14_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR14 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR15 PRI_N3: Priority of interrupt 63"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr15_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 15) + 3))
    }
    #[doc = "NVIC NVIC_IPR15 PRI_N3: Priority of interrupt 63"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr15_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR15 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR15 PRI_N2: Priority of interrupt 62"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr15_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 15) + 2))
    }
    #[doc = "NVIC NVIC_IPR15 PRI_N2: Priority of interrupt 62"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr15_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR15 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR15 PRI_N1: Priority of interrupt 61"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr15_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 15) + 1))
    }
    #[doc = "NVIC NVIC_IPR15 PRI_N1: Priority of interrupt 61"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr15_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR15 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR15 PRI_N0: Priority of interrupt 60"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr15_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 15) + 0))
    }
    #[doc = "NVIC NVIC_IPR15 PRI_N0: Priority of interrupt 60"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr15_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR15 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR16 PRI_N3: Priority of interrupt 67"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr16_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 16) + 3))
    }
    #[doc = "NVIC NVIC_IPR16 PRI_N3: Priority of interrupt 67"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr16_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR16 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR16 PRI_N2: Priority of interrupt 66"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr16_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 16) + 2))
    }
    #[doc = "NVIC NVIC_IPR16 PRI_N2: Priority of interrupt 66"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr16_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR16 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR16 PRI_N1: Priority of interrupt 65"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr16_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 16) + 1))
    }
    #[doc = "NVIC NVIC_IPR16 PRI_N1: Priority of interrupt 65"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr16_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR16 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR16 PRI_N0: Priority of interrupt 64"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr16_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 16) + 0))
    }
    #[doc = "NVIC NVIC_IPR16 PRI_N0: Priority of interrupt 64"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr16_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR16 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR17 PRI_N3: Priority of interrupt 71"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr17_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 17) + 3))
    }
    #[doc = "NVIC NVIC_IPR17 PRI_N3: Priority of interrupt 71"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr17_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR17 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR17 PRI_N2: Priority of interrupt 70"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr17_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 17) + 2))
    }
    #[doc = "NVIC NVIC_IPR17 PRI_N2: Priority of interrupt 70"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr17_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR17 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR17 PRI_N1: Priority of interrupt 69"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr17_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 17) + 1))
    }
    #[doc = "NVIC NVIC_IPR17 PRI_N1: Priority of interrupt 69"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr17_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR17 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR17 PRI_N0: Priority of interrupt 68"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr17_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 17) + 0))
    }
    #[doc = "NVIC NVIC_IPR17 PRI_N0: Priority of interrupt 68"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr17_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR17 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR18 PRI_N3: Priority of interrupt 75"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr18_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 18) + 3))
    }
    #[doc = "NVIC NVIC_IPR18 PRI_N3: Priority of interrupt 75"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr18_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR18 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR18 PRI_N2: Priority of interrupt 74"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr18_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 18) + 2))
    }
    #[doc = "NVIC NVIC_IPR18 PRI_N2: Priority of interrupt 74"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr18_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR18 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR18 PRI_N1: Priority of interrupt 73"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr18_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 18) + 1))
    }
    #[doc = "NVIC NVIC_IPR18 PRI_N1: Priority of interrupt 73"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr18_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR18 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR18 PRI_N0: Priority of interrupt 72"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr18_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 18) + 0))
    }
    #[doc = "NVIC NVIC_IPR18 PRI_N0: Priority of interrupt 72"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr18_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR18 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR19 PRI_N3: Priority of interrupt 79"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr19_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 19) + 3))
    }
    #[doc = "NVIC NVIC_IPR19 PRI_N3: Priority of interrupt 79"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr19_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR19 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR19 PRI_N2: Priority of interrupt 78"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr19_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 19) + 2))
    }
    #[doc = "NVIC NVIC_IPR19 PRI_N2: Priority of interrupt 78"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr19_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR19 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR19 PRI_N1: Priority of interrupt 77"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr19_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 19) + 1))
    }
    #[doc = "NVIC NVIC_IPR19 PRI_N1: Priority of interrupt 77"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr19_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR19 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR19 PRI_N0: Priority of interrupt 76"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr19_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 19) + 0))
    }
    #[doc = "NVIC NVIC_IPR19 PRI_N0: Priority of interrupt 76"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr19_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR19 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR20 PRI_N3: Priority of interrupt 83"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr20_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 20) + 3))
    }
    #[doc = "NVIC NVIC_IPR20 PRI_N3: Priority of interrupt 83"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr20_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR20 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR20 PRI_N2: Priority of interrupt 82"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr20_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 20) + 2))
    }
    #[doc = "NVIC NVIC_IPR20 PRI_N2: Priority of interrupt 82"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr20_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR20 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR20 PRI_N1: Priority of interrupt 81"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr20_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 20) + 1))
    }
    #[doc = "NVIC NVIC_IPR20 PRI_N1: Priority of interrupt 81"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr20_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR20 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR20 PRI_N0: Priority of interrupt 80"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr20_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 20) + 0))
    }
    #[doc = "NVIC NVIC_IPR20 PRI_N0: Priority of interrupt 80"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr20_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR20 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR21 PRI_N3: Priority of interrupt 87"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr21_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 21) + 3))
    }
    #[doc = "NVIC NVIC_IPR21 PRI_N3: Priority of interrupt 87"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr21_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR21 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR21 PRI_N2: Priority of interrupt 86"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr21_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 21) + 2))
    }
    #[doc = "NVIC NVIC_IPR21 PRI_N2: Priority of interrupt 86"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr21_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR21 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR21 PRI_N1: Priority of interrupt 85"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr21_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 21) + 1))
    }
    #[doc = "NVIC NVIC_IPR21 PRI_N1: Priority of interrupt 85"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr21_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR21 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR21 PRI_N0: Priority of interrupt 84"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr21_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 21) + 0))
    }
    #[doc = "NVIC NVIC_IPR21 PRI_N0: Priority of interrupt 84"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr21_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR21 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR22 PRI_N3: Priority of interrupt 91"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr22_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 22) + 3))
    }
    #[doc = "NVIC NVIC_IPR22 PRI_N3: Priority of interrupt 91"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr22_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR22 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR22 PRI_N2: Priority of interrupt 90"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr22_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 22) + 2))
    }
    #[doc = "NVIC NVIC_IPR22 PRI_N2: Priority of interrupt 90"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr22_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR22 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR22 PRI_N1: Priority of interrupt 89"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr22_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 22) + 1))
    }
    #[doc = "NVIC NVIC_IPR22 PRI_N1: Priority of interrupt 89"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr22_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR22 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR22 PRI_N0: Priority of interrupt 88"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr22_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 22) + 0))
    }
    #[doc = "NVIC NVIC_IPR22 PRI_N0: Priority of interrupt 88"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr22_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR22 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR23 PRI_N3: Priority of interrupt 95"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr23_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 23) + 3))
    }
    #[doc = "NVIC NVIC_IPR23 PRI_N3: Priority of interrupt 95"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr23_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR23 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR23 PRI_N2: Priority of interrupt 94"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr23_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 23) + 2))
    }
    #[doc = "NVIC NVIC_IPR23 PRI_N2: Priority of interrupt 94"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr23_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR23 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR23 PRI_N1: Priority of interrupt 93"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr23_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 23) + 1))
    }
    #[doc = "NVIC NVIC_IPR23 PRI_N1: Priority of interrupt 93"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr23_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR23 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR23 PRI_N0: Priority of interrupt 92"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr23_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 23) + 0))
    }
    #[doc = "NVIC NVIC_IPR23 PRI_N0: Priority of interrupt 92"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr23_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR23 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR24 PRI_N3: Priority of interrupt 99"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr24_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 24) + 3))
    }
    #[doc = "NVIC NVIC_IPR24 PRI_N3: Priority of interrupt 99"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr24_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR24 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR24 PRI_N2: Priority of interrupt 98"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr24_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 24) + 2))
    }
    #[doc = "NVIC NVIC_IPR24 PRI_N2: Priority of interrupt 98"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr24_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR24 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR24 PRI_N1: Priority of interrupt 97"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr24_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 24) + 1))
    }
    #[doc = "NVIC NVIC_IPR24 PRI_N1: Priority of interrupt 97"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr24_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR24 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR24 PRI_N0: Priority of interrupt 96"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr24_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 24) + 0))
    }
    #[doc = "NVIC NVIC_IPR24 PRI_N0: Priority of interrupt 96"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr24_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR24 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR25 PRI_N3: Priority of interrupt 103"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr25_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 25) + 3))
    }
    #[doc = "NVIC NVIC_IPR25 PRI_N3: Priority of interrupt 103"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr25_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR25 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR25 PRI_N2: Priority of interrupt 102"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr25_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 25) + 2))
    }
    #[doc = "NVIC NVIC_IPR25 PRI_N2: Priority of interrupt 102"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr25_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR25 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR25 PRI_N1: Priority of interrupt 101"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr25_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 25) + 1))
    }
    #[doc = "NVIC NVIC_IPR25 PRI_N1: Priority of interrupt 101"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr25_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR25 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR25 PRI_N0: Priority of interrupt 100"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr25_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 25) + 0))
    }
    #[doc = "NVIC NVIC_IPR25 PRI_N0: Priority of interrupt 100"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr25_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR25 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR26 PRI_N3: Priority of interrupt 107"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr26_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 26) + 3))
    }
    #[doc = "NVIC NVIC_IPR26 PRI_N3: Priority of interrupt 107"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr26_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR26 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR26 PRI_N2: Priority of interrupt 106"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr26_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 26) + 2))
    }
    #[doc = "NVIC NVIC_IPR26 PRI_N2: Priority of interrupt 106"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr26_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR26 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR26 PRI_N1: Priority of interrupt 105"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr26_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 26) + 1))
    }
    #[doc = "NVIC NVIC_IPR26 PRI_N1: Priority of interrupt 105"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr26_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR26 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR26 PRI_N0: Priority of interrupt 104"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr26_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 26) + 0))
    }
    #[doc = "NVIC NVIC_IPR26 PRI_N0: Priority of interrupt 104"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr26_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR26 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR27 PRI_N3: Priority of interrupt 111"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr27_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 27) + 3))
    }
    #[doc = "NVIC NVIC_IPR27 PRI_N3: Priority of interrupt 111"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr27_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR27 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR27 PRI_N2: Priority of interrupt 110"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr27_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 27) + 2))
    }
    #[doc = "NVIC NVIC_IPR27 PRI_N2: Priority of interrupt 110"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr27_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR27 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR27 PRI_N1: Priority of interrupt 109"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr27_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 27) + 1))
    }
    #[doc = "NVIC NVIC_IPR27 PRI_N1: Priority of interrupt 109"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr27_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR27 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR27 PRI_N0: Priority of interrupt 108"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr27_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 27) + 0))
    }
    #[doc = "NVIC NVIC_IPR27 PRI_N0: Priority of interrupt 108"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr27_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR27 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR28 PRI_N3: Priority of interrupt 115"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr28_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 28) + 3))
    }
    #[doc = "NVIC NVIC_IPR28 PRI_N3: Priority of interrupt 115"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr28_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR28 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR28 PRI_N2: Priority of interrupt 114"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr28_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 28) + 2))
    }
    #[doc = "NVIC NVIC_IPR28 PRI_N2: Priority of interrupt 114"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr28_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR28 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR28 PRI_N1: Priority of interrupt 113"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr28_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 28) + 1))
    }
    #[doc = "NVIC NVIC_IPR28 PRI_N1: Priority of interrupt 113"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr28_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR28 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR28 PRI_N0: Priority of interrupt 112"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr28_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 28) + 0))
    }
    #[doc = "NVIC NVIC_IPR28 PRI_N0: Priority of interrupt 112"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr28_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR28 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR29 PRI_N3: Priority of interrupt 119"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr29_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 29) + 3))
    }
    #[doc = "NVIC NVIC_IPR29 PRI_N3: Priority of interrupt 119"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr29_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR29 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR29 PRI_N2: Priority of interrupt 118"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr29_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 29) + 2))
    }
    #[doc = "NVIC NVIC_IPR29 PRI_N2: Priority of interrupt 118"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr29_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR29 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR29 PRI_N1: Priority of interrupt 117"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr29_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 29) + 1))
    }
    #[doc = "NVIC NVIC_IPR29 PRI_N1: Priority of interrupt 117"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr29_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR29 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR29 PRI_N0: Priority of interrupt 116"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr29_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 29) + 0))
    }
    #[doc = "NVIC NVIC_IPR29 PRI_N0: Priority of interrupt 116"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr29_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR29 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR30 PRI_N3: Priority of interrupt 123"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr30_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 30) + 3))
    }
    #[doc = "NVIC NVIC_IPR30 PRI_N3: Priority of interrupt 123"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr30_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR30 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR30 PRI_N2: Priority of interrupt 122"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr30_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 30) + 2))
    }
    #[doc = "NVIC NVIC_IPR30 PRI_N2: Priority of interrupt 122"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr30_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR30 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR30 PRI_N1: Priority of interrupt 121"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr30_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 30) + 1))
    }
    #[doc = "NVIC NVIC_IPR30 PRI_N1: Priority of interrupt 121"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr30_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR30 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR30 PRI_N0: Priority of interrupt 120"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr30_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 30) + 0))
    }
    #[doc = "NVIC NVIC_IPR30 PRI_N0: Priority of interrupt 120"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr30_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR30 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR31 PRI_N3: Priority of interrupt 127"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr31_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 31) + 3))
    }
    #[doc = "NVIC NVIC_IPR31 PRI_N3: Priority of interrupt 127"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr31_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR31 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR31 PRI_N2: Priority of interrupt 126"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr31_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 31) + 2))
    }
    #[doc = "NVIC NVIC_IPR31 PRI_N2: Priority of interrupt 126"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr31_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR31 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR31 PRI_N1: Priority of interrupt 125"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr31_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 31) + 1))
    }
    #[doc = "NVIC NVIC_IPR31 PRI_N1: Priority of interrupt 125"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr31_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR31 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR31 PRI_N0: Priority of interrupt 124"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr31_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 31) + 0))
    }
    #[doc = "NVIC NVIC_IPR31 PRI_N0: Priority of interrupt 124"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr31_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR31 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR32 PRI_N3: Priority of interrupt 131"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr32_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 32) + 3))
    }
    #[doc = "NVIC NVIC_IPR32 PRI_N3: Priority of interrupt 131"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr32_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR32 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR32 PRI_N2: Priority of interrupt 130"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr32_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 32) + 2))
    }
    #[doc = "NVIC NVIC_IPR32 PRI_N2: Priority of interrupt 130"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr32_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR32 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR32 PRI_N1: Priority of interrupt 129"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr32_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 32) + 1))
    }
    #[doc = "NVIC NVIC_IPR32 PRI_N1: Priority of interrupt 129"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr32_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR32 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR32 PRI_N0: Priority of interrupt 128"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr32_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 32) + 0))
    }
    #[doc = "NVIC NVIC_IPR32 PRI_N0: Priority of interrupt 128"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr32_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR32 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR33 PRI_N3: Priority of interrupt 135"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr33_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 33) + 3))
    }
    #[doc = "NVIC NVIC_IPR33 PRI_N3: Priority of interrupt 135"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr33_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR33 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR33 PRI_N2: Priority of interrupt 134"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr33_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 33) + 2))
    }
    #[doc = "NVIC NVIC_IPR33 PRI_N2: Priority of interrupt 134"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr33_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR33 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR33 PRI_N1: Priority of interrupt 133"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr33_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 33) + 1))
    }
    #[doc = "NVIC NVIC_IPR33 PRI_N1: Priority of interrupt 133"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr33_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR33 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR33 PRI_N0: Priority of interrupt 132"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr33_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 33) + 0))
    }
    #[doc = "NVIC NVIC_IPR33 PRI_N0: Priority of interrupt 132"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr33_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR33 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR34 PRI_N3: Priority of interrupt 139"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr34_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 34) + 3))
    }
    #[doc = "NVIC NVIC_IPR34 PRI_N3: Priority of interrupt 139"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr34_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR34 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR34 PRI_N2: Priority of interrupt 138"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr34_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 34) + 2))
    }
    #[doc = "NVIC NVIC_IPR34 PRI_N2: Priority of interrupt 138"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr34_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR34 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR34 PRI_N1: Priority of interrupt 137"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr34_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 34) + 1))
    }
    #[doc = "NVIC NVIC_IPR34 PRI_N1: Priority of interrupt 137"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr34_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR34 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR34 PRI_N0: Priority of interrupt 136"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr34_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 34) + 0))
    }
    #[doc = "NVIC NVIC_IPR34 PRI_N0: Priority of interrupt 136"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr34_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR34 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR35 PRI_N3: Priority of interrupt 143"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr35_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 35) + 3))
    }
    #[doc = "NVIC NVIC_IPR35 PRI_N3: Priority of interrupt 143"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr35_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR35 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR35 PRI_N2: Priority of interrupt 142"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr35_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 35) + 2))
    }
    #[doc = "NVIC NVIC_IPR35 PRI_N2: Priority of interrupt 142"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr35_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR35 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR35 PRI_N1: Priority of interrupt 141"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr35_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 35) + 1))
    }
    #[doc = "NVIC NVIC_IPR35 PRI_N1: Priority of interrupt 141"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr35_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR35 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR35 PRI_N0: Priority of interrupt 140"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr35_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 35) + 0))
    }
    #[doc = "NVIC NVIC_IPR35 PRI_N0: Priority of interrupt 140"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr35_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR35 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR36 PRI_N3: Priority of interrupt 147"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr36_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 36) + 3))
    }
    #[doc = "NVIC NVIC_IPR36 PRI_N3: Priority of interrupt 147"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr36_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR36 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR36 PRI_N2: Priority of interrupt 146"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr36_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 36) + 2))
    }
    #[doc = "NVIC NVIC_IPR36 PRI_N2: Priority of interrupt 146"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr36_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR36 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR36 PRI_N1: Priority of interrupt 145"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr36_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 36) + 1))
    }
    #[doc = "NVIC NVIC_IPR36 PRI_N1: Priority of interrupt 145"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr36_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR36 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR36 PRI_N0: Priority of interrupt 144"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr36_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 36) + 0))
    }
    #[doc = "NVIC NVIC_IPR36 PRI_N0: Priority of interrupt 144"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr36_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR36 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR37 PRI_N3: Priority of interrupt 151"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr37_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 37) + 3))
    }
    #[doc = "NVIC NVIC_IPR37 PRI_N3: Priority of interrupt 151"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr37_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR37 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR37 PRI_N2: Priority of interrupt 150"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr37_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 37) + 2))
    }
    #[doc = "NVIC NVIC_IPR37 PRI_N2: Priority of interrupt 150"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr37_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR37 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR37 PRI_N1: Priority of interrupt 149"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr37_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 37) + 1))
    }
    #[doc = "NVIC NVIC_IPR37 PRI_N1: Priority of interrupt 149"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr37_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR37 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR37 PRI_N0: Priority of interrupt 148"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr37_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 37) + 0))
    }
    #[doc = "NVIC NVIC_IPR37 PRI_N0: Priority of interrupt 148"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr37_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR37 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR38 PRI_N3: Priority of interrupt 155"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr38_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 38) + 3))
    }
    #[doc = "NVIC NVIC_IPR38 PRI_N3: Priority of interrupt 155"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr38_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR38 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR38 PRI_N2: Priority of interrupt 154"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr38_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 38) + 2))
    }
    #[doc = "NVIC NVIC_IPR38 PRI_N2: Priority of interrupt 154"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr38_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR38 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR38 PRI_N1: Priority of interrupt 153"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr38_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 38) + 1))
    }
    #[doc = "NVIC NVIC_IPR38 PRI_N1: Priority of interrupt 153"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr38_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR38 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR38 PRI_N0: Priority of interrupt 152"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr38_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 38) + 0))
    }
    #[doc = "NVIC NVIC_IPR38 PRI_N0: Priority of interrupt 152"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr38_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR38 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR39 PRI_N3: Priority of interrupt 159"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr39_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 39) + 3))
    }
    #[doc = "NVIC NVIC_IPR39 PRI_N3: Priority of interrupt 159"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr39_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR39 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR39 PRI_N2: Priority of interrupt 158"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr39_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 39) + 2))
    }
    #[doc = "NVIC NVIC_IPR39 PRI_N2: Priority of interrupt 158"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr39_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR39 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR39 PRI_N1: Priority of interrupt 157"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr39_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 39) + 1))
    }
    #[doc = "NVIC NVIC_IPR39 PRI_N1: Priority of interrupt 157"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr39_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR39 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR39 PRI_N0: Priority of interrupt 156"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr39_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 39) + 0))
    }
    #[doc = "NVIC NVIC_IPR39 PRI_N0: Priority of interrupt 156"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr39_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR39 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR40 PRI_N3: Priority of interrupt 163"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr40_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 40) + 3))
    }
    #[doc = "NVIC NVIC_IPR40 PRI_N3: Priority of interrupt 163"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr40_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR40 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR40 PRI_N2: Priority of interrupt 162"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr40_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 40) + 2))
    }
    #[doc = "NVIC NVIC_IPR40 PRI_N2: Priority of interrupt 162"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr40_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR40 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR40 PRI_N1: Priority of interrupt 161"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr40_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 40) + 1))
    }
    #[doc = "NVIC NVIC_IPR40 PRI_N1: Priority of interrupt 161"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr40_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR40 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR40 PRI_N0: Priority of interrupt 160"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr40_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 40) + 0))
    }
    #[doc = "NVIC NVIC_IPR40 PRI_N0: Priority of interrupt 160"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr40_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR40 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR41 PRI_N3: Priority of interrupt 167"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr41_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 41) + 3))
    }
    #[doc = "NVIC NVIC_IPR41 PRI_N3: Priority of interrupt 167"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr41_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR41 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR41 PRI_N2: Priority of interrupt 166"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr41_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 41) + 2))
    }
    #[doc = "NVIC NVIC_IPR41 PRI_N2: Priority of interrupt 166"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr41_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR41 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR41 PRI_N1: Priority of interrupt 165"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr41_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 41) + 1))
    }
    #[doc = "NVIC NVIC_IPR41 PRI_N1: Priority of interrupt 165"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr41_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR41 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR41 PRI_N0: Priority of interrupt 164"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr41_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 41) + 0))
    }
    #[doc = "NVIC NVIC_IPR41 PRI_N0: Priority of interrupt 164"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr41_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR41 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR42 PRI_N3: Priority of interrupt 171"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr42_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 42) + 3))
    }
    #[doc = "NVIC NVIC_IPR42 PRI_N3: Priority of interrupt 171"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr42_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR42 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR42 PRI_N2: Priority of interrupt 170"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr42_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 42) + 2))
    }
    #[doc = "NVIC NVIC_IPR42 PRI_N2: Priority of interrupt 170"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr42_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR42 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR42 PRI_N1: Priority of interrupt 169"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr42_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 42) + 1))
    }
    #[doc = "NVIC NVIC_IPR42 PRI_N1: Priority of interrupt 169"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr42_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR42 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR42 PRI_N0: Priority of interrupt 168"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr42_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 42) + 0))
    }
    #[doc = "NVIC NVIC_IPR42 PRI_N0: Priority of interrupt 168"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr42_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR42 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR43 PRI_N3: Priority of interrupt 175"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr43_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 43) + 3))
    }
    #[doc = "NVIC NVIC_IPR43 PRI_N3: Priority of interrupt 175"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr43_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR43 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR43 PRI_N2: Priority of interrupt 174"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr43_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 43) + 2))
    }
    #[doc = "NVIC NVIC_IPR43 PRI_N2: Priority of interrupt 174"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr43_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR43 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR43 PRI_N1: Priority of interrupt 173"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr43_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 43) + 1))
    }
    #[doc = "NVIC NVIC_IPR43 PRI_N1: Priority of interrupt 173"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr43_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR43 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR43 PRI_N0: Priority of interrupt 172"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr43_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 43) + 0))
    }
    #[doc = "NVIC NVIC_IPR43 PRI_N0: Priority of interrupt 172"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr43_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR43 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR44 PRI_N3: Priority of interrupt 179"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr44_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 44) + 3))
    }
    #[doc = "NVIC NVIC_IPR44 PRI_N3: Priority of interrupt 179"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr44_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR44 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR44 PRI_N2: Priority of interrupt 178"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr44_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 44) + 2))
    }
    #[doc = "NVIC NVIC_IPR44 PRI_N2: Priority of interrupt 178"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr44_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR44 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR44 PRI_N1: Priority of interrupt 177"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr44_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 44) + 1))
    }
    #[doc = "NVIC NVIC_IPR44 PRI_N1: Priority of interrupt 177"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr44_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR44 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR44 PRI_N0: Priority of interrupt 176"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr44_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 44) + 0))
    }
    #[doc = "NVIC NVIC_IPR44 PRI_N0: Priority of interrupt 176"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr44_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR44 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR45 PRI_N3: Priority of interrupt 183"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr45_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 45) + 3))
    }
    #[doc = "NVIC NVIC_IPR45 PRI_N3: Priority of interrupt 183"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr45_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR45 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR45 PRI_N2: Priority of interrupt 182"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr45_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 45) + 2))
    }
    #[doc = "NVIC NVIC_IPR45 PRI_N2: Priority of interrupt 182"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr45_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR45 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR45 PRI_N1: Priority of interrupt 181"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr45_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 45) + 1))
    }
    #[doc = "NVIC NVIC_IPR45 PRI_N1: Priority of interrupt 181"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr45_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR45 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR45 PRI_N0: Priority of interrupt 180"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr45_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 45) + 0))
    }
    #[doc = "NVIC NVIC_IPR45 PRI_N0: Priority of interrupt 180"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr45_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR45 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR46 PRI_N3: Priority of interrupt 187"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr46_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 46) + 3))
    }
    #[doc = "NVIC NVIC_IPR46 PRI_N3: Priority of interrupt 187"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr46_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR46 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR46 PRI_N2: Priority of interrupt 186"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr46_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 46) + 2))
    }
    #[doc = "NVIC NVIC_IPR46 PRI_N2: Priority of interrupt 186"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr46_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR46 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR46 PRI_N1: Priority of interrupt 185"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr46_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 46) + 1))
    }
    #[doc = "NVIC NVIC_IPR46 PRI_N1: Priority of interrupt 185"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr46_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR46 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR46 PRI_N0: Priority of interrupt 184"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr46_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 46) + 0))
    }
    #[doc = "NVIC NVIC_IPR46 PRI_N0: Priority of interrupt 184"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr46_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR46 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR47 PRI_N3: Priority of interrupt 191"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr47_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 47) + 3))
    }
    #[doc = "NVIC NVIC_IPR47 PRI_N3: Priority of interrupt 191"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr47_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR47 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR47 PRI_N2: Priority of interrupt 190"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr47_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 47) + 2))
    }
    #[doc = "NVIC NVIC_IPR47 PRI_N2: Priority of interrupt 190"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr47_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR47 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR47 PRI_N1: Priority of interrupt 189"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr47_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 47) + 1))
    }
    #[doc = "NVIC NVIC_IPR47 PRI_N1: Priority of interrupt 189"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr47_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR47 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR47 PRI_N0: Priority of interrupt 188"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr47_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 47) + 0))
    }
    #[doc = "NVIC NVIC_IPR47 PRI_N0: Priority of interrupt 188"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr47_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR47 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR48 PRI_N3: Priority of interrupt 195"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr48_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 48) + 3))
    }
    #[doc = "NVIC NVIC_IPR48 PRI_N3: Priority of interrupt 195"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr48_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR48 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR48 PRI_N2: Priority of interrupt 194"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr48_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 48) + 2))
    }
    #[doc = "NVIC NVIC_IPR48 PRI_N2: Priority of interrupt 194"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr48_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR48 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR48 PRI_N1: Priority of interrupt 193"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr48_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 48) + 1))
    }
    #[doc = "NVIC NVIC_IPR48 PRI_N1: Priority of interrupt 193"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr48_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR48 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR48 PRI_N0: Priority of interrupt 192"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr48_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 48) + 0))
    }
    #[doc = "NVIC NVIC_IPR48 PRI_N0: Priority of interrupt 192"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr48_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR48 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR49 PRI_N3: Priority of interrupt 199"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr49_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 49) + 3))
    }
    #[doc = "NVIC NVIC_IPR49 PRI_N3: Priority of interrupt 199"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr49_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR49 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR49 PRI_N2: Priority of interrupt 198"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr49_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 49) + 2))
    }
    #[doc = "NVIC NVIC_IPR49 PRI_N2: Priority of interrupt 198"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr49_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR49 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR49 PRI_N1: Priority of interrupt 197"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr49_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 49) + 1))
    }
    #[doc = "NVIC NVIC_IPR49 PRI_N1: Priority of interrupt 197"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr49_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR49 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR49 PRI_N0: Priority of interrupt 196"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr49_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 49) + 0))
    }
    #[doc = "NVIC NVIC_IPR49 PRI_N0: Priority of interrupt 196"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr49_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR49 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR50 PRI_N3: Priority of interrupt 203"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr50_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 50) + 3))
    }
    #[doc = "NVIC NVIC_IPR50 PRI_N3: Priority of interrupt 203"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr50_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR50 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR50 PRI_N2: Priority of interrupt 202"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr50_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 50) + 2))
    }
    #[doc = "NVIC NVIC_IPR50 PRI_N2: Priority of interrupt 202"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr50_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR50 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR50 PRI_N1: Priority of interrupt 201"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr50_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 50) + 1))
    }
    #[doc = "NVIC NVIC_IPR50 PRI_N1: Priority of interrupt 201"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr50_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR50 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR50 PRI_N0: Priority of interrupt 200"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr50_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 50) + 0))
    }
    #[doc = "NVIC NVIC_IPR50 PRI_N0: Priority of interrupt 200"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr50_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR50 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR51 PRI_N3: Priority of interrupt 207"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr51_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 51) + 3))
    }
    #[doc = "NVIC NVIC_IPR51 PRI_N3: Priority of interrupt 207"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr51_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR51 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR51 PRI_N2: Priority of interrupt 206"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr51_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 51) + 2))
    }
    #[doc = "NVIC NVIC_IPR51 PRI_N2: Priority of interrupt 206"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr51_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR51 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR51 PRI_N1: Priority of interrupt 205"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr51_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 51) + 1))
    }
    #[doc = "NVIC NVIC_IPR51 PRI_N1: Priority of interrupt 205"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr51_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR51 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR51 PRI_N0: Priority of interrupt 204"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr51_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 51) + 0))
    }
    #[doc = "NVIC NVIC_IPR51 PRI_N0: Priority of interrupt 204"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr51_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR51 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR52 PRI_N3: Priority of interrupt 211"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr52_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 52) + 3))
    }
    #[doc = "NVIC NVIC_IPR52 PRI_N3: Priority of interrupt 211"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr52_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR52 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR52 PRI_N2: Priority of interrupt 210"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr52_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 52) + 2))
    }
    #[doc = "NVIC NVIC_IPR52 PRI_N2: Priority of interrupt 210"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr52_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR52 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR52 PRI_N1: Priority of interrupt 209"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr52_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 52) + 1))
    }
    #[doc = "NVIC NVIC_IPR52 PRI_N1: Priority of interrupt 209"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr52_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR52 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR52 PRI_N0: Priority of interrupt 208"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr52_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 52) + 0))
    }
    #[doc = "NVIC NVIC_IPR52 PRI_N0: Priority of interrupt 208"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr52_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR52 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR53 PRI_N3: Priority of interrupt 215"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr53_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 53) + 3))
    }
    #[doc = "NVIC NVIC_IPR53 PRI_N3: Priority of interrupt 215"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr53_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR53 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR53 PRI_N2: Priority of interrupt 214"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr53_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 53) + 2))
    }
    #[doc = "NVIC NVIC_IPR53 PRI_N2: Priority of interrupt 214"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr53_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR53 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR53 PRI_N1: Priority of interrupt 213"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr53_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 53) + 1))
    }
    #[doc = "NVIC NVIC_IPR53 PRI_N1: Priority of interrupt 213"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr53_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR53 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR53 PRI_N0: Priority of interrupt 212"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr53_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 53) + 0))
    }
    #[doc = "NVIC NVIC_IPR53 PRI_N0: Priority of interrupt 212"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr53_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR53 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR54 PRI_N3: Priority of interrupt 219"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr54_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 54) + 3))
    }
    #[doc = "NVIC NVIC_IPR54 PRI_N3: Priority of interrupt 219"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr54_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR54 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR54 PRI_N2: Priority of interrupt 218"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr54_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 54) + 2))
    }
    #[doc = "NVIC NVIC_IPR54 PRI_N2: Priority of interrupt 218"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr54_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR54 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR54 PRI_N1: Priority of interrupt 217"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr54_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 54) + 1))
    }
    #[doc = "NVIC NVIC_IPR54 PRI_N1: Priority of interrupt 217"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr54_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR54 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR54 PRI_N0: Priority of interrupt 216"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr54_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 54) + 0))
    }
    #[doc = "NVIC NVIC_IPR54 PRI_N0: Priority of interrupt 216"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr54_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR54 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR55 PRI_N3: Priority of interrupt 223"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr55_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 55) + 3))
    }
    #[doc = "NVIC NVIC_IPR55 PRI_N3: Priority of interrupt 223"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr55_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR55 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR55 PRI_N2: Priority of interrupt 222"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr55_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 55) + 2))
    }
    #[doc = "NVIC NVIC_IPR55 PRI_N2: Priority of interrupt 222"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr55_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR55 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR55 PRI_N1: Priority of interrupt 221"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr55_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 55) + 1))
    }
    #[doc = "NVIC NVIC_IPR55 PRI_N1: Priority of interrupt 221"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr55_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR55 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR55 PRI_N0: Priority of interrupt 220"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr55_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 55) + 0))
    }
    #[doc = "NVIC NVIC_IPR55 PRI_N0: Priority of interrupt 220"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr55_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR55 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR56 PRI_N3: Priority of interrupt 227"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr56_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 56) + 3))
    }
    #[doc = "NVIC NVIC_IPR56 PRI_N3: Priority of interrupt 227"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr56_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR56 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR56 PRI_N2: Priority of interrupt 226"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr56_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 56) + 2))
    }
    #[doc = "NVIC NVIC_IPR56 PRI_N2: Priority of interrupt 226"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr56_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR56 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR56 PRI_N1: Priority of interrupt 225"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr56_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 56) + 1))
    }
    #[doc = "NVIC NVIC_IPR56 PRI_N1: Priority of interrupt 225"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr56_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR56 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR56 PRI_N0: Priority of interrupt 224"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr56_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 56) + 0))
    }
    #[doc = "NVIC NVIC_IPR56 PRI_N0: Priority of interrupt 224"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr56_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR56 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR57 PRI_N3: Priority of interrupt 231"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr57_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 57) + 3))
    }
    #[doc = "NVIC NVIC_IPR57 PRI_N3: Priority of interrupt 231"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr57_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR57 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR57 PRI_N2: Priority of interrupt 230"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr57_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 57) + 2))
    }
    #[doc = "NVIC NVIC_IPR57 PRI_N2: Priority of interrupt 230"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr57_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR57 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR57 PRI_N1: Priority of interrupt 229"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr57_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 57) + 1))
    }
    #[doc = "NVIC NVIC_IPR57 PRI_N1: Priority of interrupt 229"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr57_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR57 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR57 PRI_N0: Priority of interrupt 228"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr57_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 57) + 0))
    }
    #[doc = "NVIC NVIC_IPR57 PRI_N0: Priority of interrupt 228"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr57_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR57 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR58 PRI_N3: Priority of interrupt 235"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr58_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 58) + 3))
    }
    #[doc = "NVIC NVIC_IPR58 PRI_N3: Priority of interrupt 235"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr58_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR58 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR58 PRI_N2: Priority of interrupt 234"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr58_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 58) + 2))
    }
    #[doc = "NVIC NVIC_IPR58 PRI_N2: Priority of interrupt 234"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr58_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR58 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR58 PRI_N1: Priority of interrupt 233"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr58_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 58) + 1))
    }
    #[doc = "NVIC NVIC_IPR58 PRI_N1: Priority of interrupt 233"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr58_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR58 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR58 PRI_N0: Priority of interrupt 232"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr58_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 58) + 0))
    }
    #[doc = "NVIC NVIC_IPR58 PRI_N0: Priority of interrupt 232"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr58_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR58 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR59 PRI_N3: Priority of interrupt 239"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr59_pri_n3(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 59) + 3))
    }
    #[doc = "NVIC NVIC_IPR59 PRI_N3: Priority of interrupt 239"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr59_pri_n3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR59 PRI_N3 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR59 PRI_N2: Priority of interrupt 238"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr59_pri_n2(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 59) + 2))
    }
    #[doc = "NVIC NVIC_IPR59 PRI_N2: Priority of interrupt 238"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr59_pri_n2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR59 PRI_N2 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR59 PRI_N1: Priority of interrupt 237"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr59_pri_n1(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 59) + 1))
    }
    #[doc = "NVIC NVIC_IPR59 PRI_N1: Priority of interrupt 237"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr59_pri_n1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR59 PRI_N1 reset value 0x00 mask 0xff")
    }
    #[doc = "NVIC NVIC_IPR59 PRI_N0: Priority of interrupt 236"]
    #[inline]
    pub(crate) fn read_nvic_nvic_ipr59_pri_n0(&self) -> MemResult<u8> {
        Ok(self.interrupts.priority((4 * 59) + 0))
    }
    #[doc = "NVIC NVIC_IPR59 PRI_N0: Priority of interrupt 236"]
    #[inline]
    pub(crate) fn write_nvic_nvic_ipr59_pri_n0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVIC NVIC_IPR59 PRI_N0 reset value 0x00 mask 0xff")
    }
    #[doc = "MPU MPU_TYPE SEPARATE: Indicates support for separate instruction and data address maps"]
    #[inline]
    pub(crate) fn read_mpu_mpu_type_separate(&self) -> MemResult<bool> {
        todo!("read MPU MPU_TYPE SEPARATE reset value false")
    }
    #[doc = "MPU MPU_TYPE DREGION: Number of regions supported by the MPU"]
    #[inline]
    pub(crate) fn read_mpu_mpu_type_dregion(&self) -> MemResult<u8> {
        todo!("read MPU MPU_TYPE DREGION reset value 0x00 mask 0xff")
    }
    #[doc = "MPU MPU_TYPE IREGION: Number of instruction regions supported by the MPU"]
    #[inline]
    pub(crate) fn read_mpu_mpu_type_iregion(&self) -> MemResult<u8> {
        todo!("read MPU MPU_TYPE IREGION reset value 0x00 mask 0xff")
    }
    #[doc = "MPU MPU_CTRL ENABLE: Enables the MPU"]
    #[inline]
    pub(crate) fn read_mpu_mpu_ctrl_enable(&self) -> MemResult<bool> {
        todo!("read MPU MPU_CTRL ENABLE reset value false")
    }
    #[doc = "MPU MPU_CTRL ENABLE: Enables the MPU"]
    #[inline]
    pub(crate) fn write_mpu_mpu_ctrl_enable(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU MPU_CTRL ENABLE reset value false")
    }
    #[doc = "MPU MPU_CTRL HFNMIENA: When the ENABLE bit is set to 1, controls whether handlers executing with priority less than 0 access memory with the MPU enabled or with the MPU disabled"]
    #[inline]
    pub(crate) fn read_mpu_mpu_ctrl_hfnmiena(&self) -> MemResult<bool> {
        todo!("read MPU MPU_CTRL HFNMIENA reset value false")
    }
    #[doc = "MPU MPU_CTRL HFNMIENA: When the ENABLE bit is set to 1, controls whether handlers executing with priority less than 0 access memory with the MPU enabled or with the MPU disabled"]
    #[inline]
    pub(crate) fn write_mpu_mpu_ctrl_hfnmiena(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU MPU_CTRL HFNMIENA reset value false")
    }
    #[doc = "MPU MPU_CTRL PRIVDEFENA: When the ENABLE bit is set to 1, Enables the default memory map as a background region for privileged access"]
    #[inline]
    pub(crate) fn read_mpu_mpu_ctrl_privdefena(&self) -> MemResult<bool> {
        todo!("read MPU MPU_CTRL PRIVDEFENA reset value false")
    }
    #[doc = "MPU MPU_CTRL PRIVDEFENA: When the ENABLE bit is set to 1, Enables the default memory map as a background region for privileged access"]
    #[inline]
    pub(crate) fn write_mpu_mpu_ctrl_privdefena(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU MPU_CTRL PRIVDEFENA reset value false")
    }
    #[doc = "MPU MPU_RNR REGION: Indicates the memory region accessed by MPU_RBAR and MPU_RSAR"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rnr_region(&self) -> MemResult<u8> {
        todo!("read MPU MPU_RNR REGION reset value 0x00 mask 0xff")
    }
    #[doc = "MPU MPU_RNR REGION: Indicates the memory region accessed by MPU_RBAR and MPU_RSAR"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rnr_region(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RNR REGION reset value 0x00 mask 0xff")
    }
    #[doc = "MPU MPU_RBAR REGION: On writes, can specify the number of the region to update. On reads, returns bits [3:0] of MPU_RNR"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rbar_region(&self) -> MemResult<u8> {
        todo!("read MPU MPU_RBAR REGION reset value 0x00 mask 0x0f")
    }
    #[doc = "MPU MPU_RBAR REGION: On writes, can specify the number of the region to update. On reads, returns bits [3:0] of MPU_RNR"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rbar_region(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RBAR REGION reset value 0x00 mask 0x0f")
    }
    #[doc = "MPU MPU_RBAR VALID: On writes, indicates whether the region to update is specified by MPU_RNR.REGION, or by the REGION value specified in this write. When using the REGION value specified by this write, MPU_RNR.REGION is updated to this value"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rbar_valid(&self) -> MemResult<bool> {
        todo!("read MPU MPU_RBAR VALID reset value false")
    }
    #[doc = "MPU MPU_RBAR VALID: On writes, indicates whether the region to update is specified by MPU_RNR.REGION, or by the REGION value specified in this write. When using the REGION value specified by this write, MPU_RNR.REGION is updated to this value"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rbar_valid(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RBAR VALID reset value false")
    }
    #[doc = "MPU MPU_RBAR ADDR: Base address of the region"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rbar_addr(&self) -> MemResult<u32> {
        todo!("read MPU MPU_RBAR ADDR reset value 0x00 mask 0x7ffffff")
    }
    #[doc = "MPU MPU_RBAR ADDR: Base address of the region"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rbar_addr(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RBAR ADDR reset value 0x00 mask 0x7ffffff")
    }
    #[doc = "MPU MPU_RASR ENABLE: Enables this region"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_enable(&self) -> MemResult<bool> {
        todo!("read MPU MPU_RASR ENABLE reset value false")
    }
    #[doc = "MPU MPU_RASR ENABLE: Enables this region"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_enable(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR ENABLE reset value false")
    }
    #[doc = "MPU MPU_RASR SIZE: Indicates the region size"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_size(&self) -> MemResult<u8> {
        todo!("read MPU MPU_RASR SIZE reset value 0x00 mask 0x1f")
    }
    #[doc = "MPU MPU_RASR SIZE: Indicates the region size"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_size(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR SIZE reset value 0x00 mask 0x1f")
    }
    #[doc = "MPU MPU_RASR SRD: Subregion Disable"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_srd(&self) -> MemResult<u8> {
        todo!("read MPU MPU_RASR SRD reset value 0x00 mask 0xff")
    }
    #[doc = "MPU MPU_RASR SRD: Subregion Disable"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_srd(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR SRD reset value 0x00 mask 0xff")
    }
    #[doc = "MPU MPU_RASR B: MPU Region Attribute field"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_b(&self) -> MemResult<bool> {
        todo!("read MPU MPU_RASR B reset value false")
    }
    #[doc = "MPU MPU_RASR B: MPU Region Attribute field"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_b(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR B reset value false")
    }
    #[doc = "MPU MPU_RASR C: MPU Region Attribute field"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_c(&self) -> MemResult<bool> {
        todo!("read MPU MPU_RASR C reset value false")
    }
    #[doc = "MPU MPU_RASR C: MPU Region Attribute field"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_c(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR C reset value false")
    }
    #[doc = "MPU MPU_RASR S: MPU Region Attribute field: Sharable"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_s(&self) -> MemResult<bool> {
        todo!("read MPU MPU_RASR S reset value false")
    }
    #[doc = "MPU MPU_RASR S: MPU Region Attribute field: Sharable"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_s(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR S reset value false")
    }
    #[doc = "MPU MPU_RASR TEX: MPU Region Attribute field"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_tex(&self) -> MemResult<u8> {
        todo!("read MPU MPU_RASR TEX reset value 0x00 mask 0x07")
    }
    #[doc = "MPU MPU_RASR TEX: MPU Region Attribute field"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_tex(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR TEX reset value 0x00 mask 0x07")
    }
    #[doc = "MPU MPU_RASR AP: Access permissions"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_ap(&self) -> MemResult<u8> {
        todo!("read MPU MPU_RASR AP reset value 0x00 mask 0x07")
    }
    #[doc = "MPU MPU_RASR AP: Access permissions"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_ap(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR AP reset value 0x00 mask 0x07")
    }
    #[doc = "MPU MPU_RASR XN: Execute Never"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_xn(&self) -> MemResult<bool> {
        todo!("read MPU MPU_RASR XN reset value false")
    }
    #[doc = "MPU MPU_RASR XN: Execute Never"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_xn(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR XN reset value false")
    }
    #[doc = "MPU MPU_RBAR_A1 REGION: On writes, can specify the number of the region to update. On reads, returns bits [3:0] of MPU_RNR"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rbar_a1_region(&self) -> MemResult<u8> {
        todo!("read MPU MPU_RBAR_A1 REGION reset value 0x00 mask 0x0f")
    }
    #[doc = "MPU MPU_RBAR_A1 REGION: On writes, can specify the number of the region to update. On reads, returns bits [3:0] of MPU_RNR"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rbar_a1_region(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RBAR_A1 REGION reset value 0x00 mask 0x0f")
    }
    #[doc = "MPU MPU_RBAR_A1 VALID: On writes, indicates whether the region to update is specified by MPU_RNR.REGION, or by the REGION value specified in this write. When using the REGION value specified by this write, MPU_RNR.REGION is updated to this value"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rbar_a1_valid(&self) -> MemResult<bool> {
        todo!("read MPU MPU_RBAR_A1 VALID reset value false")
    }
    #[doc = "MPU MPU_RBAR_A1 VALID: On writes, indicates whether the region to update is specified by MPU_RNR.REGION, or by the REGION value specified in this write. When using the REGION value specified by this write, MPU_RNR.REGION is updated to this value"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rbar_a1_valid(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RBAR_A1 VALID reset value false")
    }
    #[doc = "MPU MPU_RBAR_A1 ADDR: Base address of the region"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rbar_a1_addr(&self) -> MemResult<u32> {
        todo!("read MPU MPU_RBAR_A1 ADDR reset value 0x00 mask 0x7ffffff")
    }
    #[doc = "MPU MPU_RBAR_A1 ADDR: Base address of the region"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rbar_a1_addr(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RBAR_A1 ADDR reset value 0x00 mask 0x7ffffff")
    }
    #[doc = "MPU MPU_RASR_A1 ENABLE: Enables this region"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_a1_enable(&self) -> MemResult<bool> {
        todo!("read MPU MPU_RASR_A1 ENABLE reset value false")
    }
    #[doc = "MPU MPU_RASR_A1 ENABLE: Enables this region"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_a1_enable(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR_A1 ENABLE reset value false")
    }
    #[doc = "MPU MPU_RASR_A1 SIZE: Indicates the region size"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_a1_size(&self) -> MemResult<u8> {
        todo!("read MPU MPU_RASR_A1 SIZE reset value 0x00 mask 0x1f")
    }
    #[doc = "MPU MPU_RASR_A1 SIZE: Indicates the region size"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_a1_size(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR_A1 SIZE reset value 0x00 mask 0x1f")
    }
    #[doc = "MPU MPU_RASR_A1 SRD: Subregion Disable"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_a1_srd(&self) -> MemResult<u8> {
        todo!("read MPU MPU_RASR_A1 SRD reset value 0x00 mask 0xff")
    }
    #[doc = "MPU MPU_RASR_A1 SRD: Subregion Disable"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_a1_srd(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR_A1 SRD reset value 0x00 mask 0xff")
    }
    #[doc = "MPU MPU_RASR_A1 B: MPU Region Attribute field"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_a1_b(&self) -> MemResult<bool> {
        todo!("read MPU MPU_RASR_A1 B reset value false")
    }
    #[doc = "MPU MPU_RASR_A1 B: MPU Region Attribute field"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_a1_b(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR_A1 B reset value false")
    }
    #[doc = "MPU MPU_RASR_A1 C: MPU Region Attribute field"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_a1_c(&self) -> MemResult<bool> {
        todo!("read MPU MPU_RASR_A1 C reset value false")
    }
    #[doc = "MPU MPU_RASR_A1 C: MPU Region Attribute field"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_a1_c(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR_A1 C reset value false")
    }
    #[doc = "MPU MPU_RASR_A1 S: MPU Region Attribute field: Sharable"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_a1_s(&self) -> MemResult<bool> {
        todo!("read MPU MPU_RASR_A1 S reset value false")
    }
    #[doc = "MPU MPU_RASR_A1 S: MPU Region Attribute field: Sharable"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_a1_s(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR_A1 S reset value false")
    }
    #[doc = "MPU MPU_RASR_A1 TEX: MPU Region Attribute field"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_a1_tex(&self) -> MemResult<u8> {
        todo!("read MPU MPU_RASR_A1 TEX reset value 0x00 mask 0x07")
    }
    #[doc = "MPU MPU_RASR_A1 TEX: MPU Region Attribute field"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_a1_tex(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR_A1 TEX reset value 0x00 mask 0x07")
    }
    #[doc = "MPU MPU_RASR_A1 AP: Access permissions"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_a1_ap(&self) -> MemResult<u8> {
        todo!("read MPU MPU_RASR_A1 AP reset value 0x00 mask 0x07")
    }
    #[doc = "MPU MPU_RASR_A1 AP: Access permissions"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_a1_ap(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR_A1 AP reset value 0x00 mask 0x07")
    }
    #[doc = "MPU MPU_RASR_A1 XN: Execute Never"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_a1_xn(&self) -> MemResult<bool> {
        todo!("read MPU MPU_RASR_A1 XN reset value false")
    }
    #[doc = "MPU MPU_RASR_A1 XN: Execute Never"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_a1_xn(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR_A1 XN reset value false")
    }
    #[doc = "MPU MPU_RBAR_A2 REGION: On writes, can specify the number of the region to update. On reads, returns bits [3:0] of MPU_RNR"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rbar_a2_region(&self) -> MemResult<u8> {
        todo!("read MPU MPU_RBAR_A2 REGION reset value 0x00 mask 0x0f")
    }
    #[doc = "MPU MPU_RBAR_A2 REGION: On writes, can specify the number of the region to update. On reads, returns bits [3:0] of MPU_RNR"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rbar_a2_region(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RBAR_A2 REGION reset value 0x00 mask 0x0f")
    }
    #[doc = "MPU MPU_RBAR_A2 VALID: On writes, indicates whether the region to update is specified by MPU_RNR.REGION, or by the REGION value specified in this write. When using the REGION value specified by this write, MPU_RNR.REGION is updated to this value"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rbar_a2_valid(&self) -> MemResult<bool> {
        todo!("read MPU MPU_RBAR_A2 VALID reset value false")
    }
    #[doc = "MPU MPU_RBAR_A2 VALID: On writes, indicates whether the region to update is specified by MPU_RNR.REGION, or by the REGION value specified in this write. When using the REGION value specified by this write, MPU_RNR.REGION is updated to this value"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rbar_a2_valid(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RBAR_A2 VALID reset value false")
    }
    #[doc = "MPU MPU_RBAR_A2 ADDR: Base address of the region"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rbar_a2_addr(&self) -> MemResult<u32> {
        todo!("read MPU MPU_RBAR_A2 ADDR reset value 0x00 mask 0x7ffffff")
    }
    #[doc = "MPU MPU_RBAR_A2 ADDR: Base address of the region"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rbar_a2_addr(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RBAR_A2 ADDR reset value 0x00 mask 0x7ffffff")
    }
    #[doc = "MPU MPU_RASR_A2 ENABLE: Enables this region"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_a2_enable(&self) -> MemResult<bool> {
        todo!("read MPU MPU_RASR_A2 ENABLE reset value false")
    }
    #[doc = "MPU MPU_RASR_A2 ENABLE: Enables this region"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_a2_enable(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR_A2 ENABLE reset value false")
    }
    #[doc = "MPU MPU_RASR_A2 SIZE: Indicates the region size"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_a2_size(&self) -> MemResult<u8> {
        todo!("read MPU MPU_RASR_A2 SIZE reset value 0x00 mask 0x1f")
    }
    #[doc = "MPU MPU_RASR_A2 SIZE: Indicates the region size"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_a2_size(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR_A2 SIZE reset value 0x00 mask 0x1f")
    }
    #[doc = "MPU MPU_RASR_A2 SRD: Subregion Disable"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_a2_srd(&self) -> MemResult<u8> {
        todo!("read MPU MPU_RASR_A2 SRD reset value 0x00 mask 0xff")
    }
    #[doc = "MPU MPU_RASR_A2 SRD: Subregion Disable"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_a2_srd(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR_A2 SRD reset value 0x00 mask 0xff")
    }
    #[doc = "MPU MPU_RASR_A2 B: MPU Region Attribute field"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_a2_b(&self) -> MemResult<bool> {
        todo!("read MPU MPU_RASR_A2 B reset value false")
    }
    #[doc = "MPU MPU_RASR_A2 B: MPU Region Attribute field"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_a2_b(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR_A2 B reset value false")
    }
    #[doc = "MPU MPU_RASR_A2 C: MPU Region Attribute field"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_a2_c(&self) -> MemResult<bool> {
        todo!("read MPU MPU_RASR_A2 C reset value false")
    }
    #[doc = "MPU MPU_RASR_A2 C: MPU Region Attribute field"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_a2_c(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR_A2 C reset value false")
    }
    #[doc = "MPU MPU_RASR_A2 S: MPU Region Attribute field: Sharable"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_a2_s(&self) -> MemResult<bool> {
        todo!("read MPU MPU_RASR_A2 S reset value false")
    }
    #[doc = "MPU MPU_RASR_A2 S: MPU Region Attribute field: Sharable"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_a2_s(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR_A2 S reset value false")
    }
    #[doc = "MPU MPU_RASR_A2 TEX: MPU Region Attribute field"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_a2_tex(&self) -> MemResult<u8> {
        todo!("read MPU MPU_RASR_A2 TEX reset value 0x00 mask 0x07")
    }
    #[doc = "MPU MPU_RASR_A2 TEX: MPU Region Attribute field"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_a2_tex(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR_A2 TEX reset value 0x00 mask 0x07")
    }
    #[doc = "MPU MPU_RASR_A2 AP: Access permissions"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_a2_ap(&self) -> MemResult<u8> {
        todo!("read MPU MPU_RASR_A2 AP reset value 0x00 mask 0x07")
    }
    #[doc = "MPU MPU_RASR_A2 AP: Access permissions"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_a2_ap(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR_A2 AP reset value 0x00 mask 0x07")
    }
    #[doc = "MPU MPU_RASR_A2 XN: Execute Never"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_a2_xn(&self) -> MemResult<bool> {
        todo!("read MPU MPU_RASR_A2 XN reset value false")
    }
    #[doc = "MPU MPU_RASR_A2 XN: Execute Never"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_a2_xn(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR_A2 XN reset value false")
    }
    #[doc = "MPU MPU_RBAR_A3 REGION: On writes, can specify the number of the region to update. On reads, returns bits [3:0] of MPU_RNR"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rbar_a3_region(&self) -> MemResult<u8> {
        todo!("read MPU MPU_RBAR_A3 REGION reset value 0x00 mask 0x0f")
    }
    #[doc = "MPU MPU_RBAR_A3 REGION: On writes, can specify the number of the region to update. On reads, returns bits [3:0] of MPU_RNR"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rbar_a3_region(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RBAR_A3 REGION reset value 0x00 mask 0x0f")
    }
    #[doc = "MPU MPU_RBAR_A3 VALID: On writes, indicates whether the region to update is specified by MPU_RNR.REGION, or by the REGION value specified in this write. When using the REGION value specified by this write, MPU_RNR.REGION is updated to this value"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rbar_a3_valid(&self) -> MemResult<bool> {
        todo!("read MPU MPU_RBAR_A3 VALID reset value false")
    }
    #[doc = "MPU MPU_RBAR_A3 VALID: On writes, indicates whether the region to update is specified by MPU_RNR.REGION, or by the REGION value specified in this write. When using the REGION value specified by this write, MPU_RNR.REGION is updated to this value"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rbar_a3_valid(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RBAR_A3 VALID reset value false")
    }
    #[doc = "MPU MPU_RBAR_A3 ADDR: Base address of the region"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rbar_a3_addr(&self) -> MemResult<u32> {
        todo!("read MPU MPU_RBAR_A3 ADDR reset value 0x00 mask 0x7ffffff")
    }
    #[doc = "MPU MPU_RBAR_A3 ADDR: Base address of the region"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rbar_a3_addr(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RBAR_A3 ADDR reset value 0x00 mask 0x7ffffff")
    }
    #[doc = "MPU MPU_RASR_A3 ENABLE: Enables this region"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_a3_enable(&self) -> MemResult<bool> {
        todo!("read MPU MPU_RASR_A3 ENABLE reset value false")
    }
    #[doc = "MPU MPU_RASR_A3 ENABLE: Enables this region"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_a3_enable(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR_A3 ENABLE reset value false")
    }
    #[doc = "MPU MPU_RASR_A3 SIZE: Indicates the region size"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_a3_size(&self) -> MemResult<u8> {
        todo!("read MPU MPU_RASR_A3 SIZE reset value 0x00 mask 0x1f")
    }
    #[doc = "MPU MPU_RASR_A3 SIZE: Indicates the region size"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_a3_size(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR_A3 SIZE reset value 0x00 mask 0x1f")
    }
    #[doc = "MPU MPU_RASR_A3 SRD: Subregion Disable"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_a3_srd(&self) -> MemResult<u8> {
        todo!("read MPU MPU_RASR_A3 SRD reset value 0x00 mask 0xff")
    }
    #[doc = "MPU MPU_RASR_A3 SRD: Subregion Disable"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_a3_srd(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR_A3 SRD reset value 0x00 mask 0xff")
    }
    #[doc = "MPU MPU_RASR_A3 B: MPU Region Attribute field"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_a3_b(&self) -> MemResult<bool> {
        todo!("read MPU MPU_RASR_A3 B reset value false")
    }
    #[doc = "MPU MPU_RASR_A3 B: MPU Region Attribute field"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_a3_b(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR_A3 B reset value false")
    }
    #[doc = "MPU MPU_RASR_A3 C: MPU Region Attribute field"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_a3_c(&self) -> MemResult<bool> {
        todo!("read MPU MPU_RASR_A3 C reset value false")
    }
    #[doc = "MPU MPU_RASR_A3 C: MPU Region Attribute field"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_a3_c(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR_A3 C reset value false")
    }
    #[doc = "MPU MPU_RASR_A3 S: MPU Region Attribute field: Sharable"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_a3_s(&self) -> MemResult<bool> {
        todo!("read MPU MPU_RASR_A3 S reset value false")
    }
    #[doc = "MPU MPU_RASR_A3 S: MPU Region Attribute field: Sharable"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_a3_s(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR_A3 S reset value false")
    }
    #[doc = "MPU MPU_RASR_A3 TEX: MPU Region Attribute field"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_a3_tex(&self) -> MemResult<u8> {
        todo!("read MPU MPU_RASR_A3 TEX reset value 0x00 mask 0x07")
    }
    #[doc = "MPU MPU_RASR_A3 TEX: MPU Region Attribute field"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_a3_tex(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR_A3 TEX reset value 0x00 mask 0x07")
    }
    #[doc = "MPU MPU_RASR_A3 AP: Access permissions"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_a3_ap(&self) -> MemResult<u8> {
        todo!("read MPU MPU_RASR_A3 AP reset value 0x00 mask 0x07")
    }
    #[doc = "MPU MPU_RASR_A3 AP: Access permissions"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_a3_ap(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR_A3 AP reset value 0x00 mask 0x07")
    }
    #[doc = "MPU MPU_RASR_A3 XN: Execute Never"]
    #[inline]
    pub(crate) fn read_mpu_mpu_rasr_a3_xn(&self) -> MemResult<bool> {
        todo!("read MPU MPU_RASR_A3 XN reset value false")
    }
    #[doc = "MPU MPU_RASR_A3 XN: Execute Never"]
    #[inline]
    pub(crate) fn write_mpu_mpu_rasr_a3_xn(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU MPU_RASR_A3 XN reset value false")
    }
    #[doc = "POWER TASKS_CONSTLAT: Enable constant latency mode."]
    #[inline]
    pub(crate) fn write_power_tasks_constlat(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write POWER TASKS_CONSTLAT reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "POWER TASKS_LOWPWR: Enable low power mode (variable latency)."]
    #[inline]
    pub(crate) fn write_power_tasks_lowpwr(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!(
            "write POWER TASKS_LOWPWR reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "POWER EVENTS_POFWARN: Power failure warning."]
    #[inline]
    pub(crate) fn read_power_events_pofwarn(&self) -> MemResult<u32> {
        todo ! ("read POWER EVENTS_POFWARN reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "POWER EVENTS_POFWARN: Power failure warning."]
    #[inline]
    pub(crate) fn write_power_events_pofwarn(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write POWER EVENTS_POFWARN reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "POWER INTENSET POFWARN: Enable interrupt on POFWARN event."]
    #[inline]
    pub(crate) fn read_powerclock_intenset_pofwarn(&self) -> MemResult<bool> {
        todo!("read POWER INTENSET POFWARN reset value false")
    }
    #[doc = "POWER INTENSET POFWARN: Enable interrupt on POFWARN event."]
    #[inline]
    pub(crate) fn write_powerclock_intenset_pofwarn(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write POWER INTENSET POFWARN reset value false")
    }
    #[doc = "CLOCK INTENSET HFCLKSTARTED: Enable interrupt on HFCLKSTARTED event."]
    #[inline]
    pub(crate) fn read_powerclock_intenset_hfclkstarted(
        &self,
    ) -> MemResult<bool> {
        todo!("read CLOCK INTENSET HFCLKSTARTED reset value false")
    }
    #[doc = "CLOCK INTENSET HFCLKSTARTED: Enable interrupt on HFCLKSTARTED event."]
    #[inline]
    pub(crate) fn write_powerclock_intenset_hfclkstarted(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CLOCK INTENSET HFCLKSTARTED reset value false")
    }
    #[doc = "CLOCK INTENSET LFCLKSTARTED: Enable interrupt on LFCLKSTARTED event."]
    #[inline]
    pub(crate) fn read_powerclock_intenset_lfclkstarted(
        &self,
    ) -> MemResult<bool> {
        todo!("read CLOCK INTENSET LFCLKSTARTED reset value false")
    }
    #[doc = "CLOCK INTENSET LFCLKSTARTED: Enable interrupt on LFCLKSTARTED event."]
    #[inline]
    pub(crate) fn write_powerclock_intenset_lfclkstarted(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CLOCK INTENSET LFCLKSTARTED reset value false")
    }
    #[doc = "CLOCK INTENSET DONE: Enable interrupt on DONE event."]
    #[inline]
    pub(crate) fn read_powerclock_intenset_done(&self) -> MemResult<bool> {
        todo!("read CLOCK INTENSET DONE reset value false")
    }
    #[doc = "CLOCK INTENSET DONE: Enable interrupt on DONE event."]
    #[inline]
    pub(crate) fn write_powerclock_intenset_done(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CLOCK INTENSET DONE reset value false")
    }
    #[doc = "CLOCK INTENSET CTTO: Enable interrupt on CTTO event."]
    #[inline]
    pub(crate) fn read_powerclock_intenset_ctto(&self) -> MemResult<bool> {
        todo!("read CLOCK INTENSET CTTO reset value false")
    }
    #[doc = "CLOCK INTENSET CTTO: Enable interrupt on CTTO event."]
    #[inline]
    pub(crate) fn write_powerclock_intenset_ctto(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CLOCK INTENSET CTTO reset value false")
    }
    #[doc = "POWER INTENCLR POFWARN: Disable interrupt on POFWARN event."]
    #[inline]
    pub(crate) fn read_powerclock_intenclr_pofwarn(&self) -> MemResult<bool> {
        todo!("read POWER INTENCLR POFWARN reset value false")
    }
    #[doc = "POWER INTENCLR POFWARN: Disable interrupt on POFWARN event."]
    #[inline]
    pub(crate) fn write_powerclock_intenclr_pofwarn(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write POWER INTENCLR POFWARN reset value false")
    }
    #[doc = "CLOCK INTENCLR HFCLKSTARTED: Disable interrupt on HFCLKSTARTED event."]
    #[inline]
    pub(crate) fn read_powerclock_intenclr_hfclkstarted(
        &self,
    ) -> MemResult<bool> {
        todo!("read CLOCK INTENCLR HFCLKSTARTED reset value false")
    }
    #[doc = "CLOCK INTENCLR HFCLKSTARTED: Disable interrupt on HFCLKSTARTED event."]
    #[inline]
    pub(crate) fn write_powerclock_intenclr_hfclkstarted(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CLOCK INTENCLR HFCLKSTARTED reset value false")
    }
    #[doc = "CLOCK INTENCLR LFCLKSTARTED: Disable interrupt on LFCLKSTARTED event."]
    #[inline]
    pub(crate) fn read_powerclock_intenclr_lfclkstarted(
        &self,
    ) -> MemResult<bool> {
        todo!("read CLOCK INTENCLR LFCLKSTARTED reset value false")
    }
    #[doc = "CLOCK INTENCLR LFCLKSTARTED: Disable interrupt on LFCLKSTARTED event."]
    #[inline]
    pub(crate) fn write_powerclock_intenclr_lfclkstarted(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CLOCK INTENCLR LFCLKSTARTED reset value false")
    }
    #[doc = "CLOCK INTENCLR DONE: Disable interrupt on DONE event."]
    #[inline]
    pub(crate) fn read_powerclock_intenclr_done(&self) -> MemResult<bool> {
        todo!("read CLOCK INTENCLR DONE reset value false")
    }
    #[doc = "CLOCK INTENCLR DONE: Disable interrupt on DONE event."]
    #[inline]
    pub(crate) fn write_powerclock_intenclr_done(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CLOCK INTENCLR DONE reset value false")
    }
    #[doc = "CLOCK INTENCLR CTTO: Disable interrupt on CTTO event."]
    #[inline]
    pub(crate) fn read_powerclock_intenclr_ctto(&self) -> MemResult<bool> {
        todo!("read CLOCK INTENCLR CTTO reset value false")
    }
    #[doc = "CLOCK INTENCLR CTTO: Disable interrupt on CTTO event."]
    #[inline]
    pub(crate) fn write_powerclock_intenclr_ctto(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CLOCK INTENCLR CTTO reset value false")
    }
    #[doc = "POWER RESETREAS RESETPIN: Reset from pin-reset detected."]
    #[inline]
    pub(crate) fn read_power_resetreas_resetpin(&self) -> MemResult<bool> {
        todo!("read POWER RESETREAS RESETPIN reset value false")
    }
    #[doc = "POWER RESETREAS RESETPIN: Reset from pin-reset detected."]
    #[inline]
    pub(crate) fn write_power_resetreas_resetpin(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write POWER RESETREAS RESETPIN reset value false")
    }
    #[doc = "POWER RESETREAS DOG: Reset from watchdog detected."]
    #[inline]
    pub(crate) fn read_power_resetreas_dog(&self) -> MemResult<bool> {
        todo!("read POWER RESETREAS DOG reset value false")
    }
    #[doc = "POWER RESETREAS DOG: Reset from watchdog detected."]
    #[inline]
    pub(crate) fn write_power_resetreas_dog(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write POWER RESETREAS DOG reset value false")
    }
    #[doc = "POWER RESETREAS SREQ: Reset from AIRCR.SYSRESETREQ detected."]
    #[inline]
    pub(crate) fn read_power_resetreas_sreq(&self) -> MemResult<bool> {
        todo!("read POWER RESETREAS SREQ reset value false")
    }
    #[doc = "POWER RESETREAS SREQ: Reset from AIRCR.SYSRESETREQ detected."]
    #[inline]
    pub(crate) fn write_power_resetreas_sreq(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write POWER RESETREAS SREQ reset value false")
    }
    #[doc = "POWER RESETREAS LOCKUP: Reset from CPU lock-up detected."]
    #[inline]
    pub(crate) fn read_power_resetreas_lockup(&self) -> MemResult<bool> {
        todo!("read POWER RESETREAS LOCKUP reset value false")
    }
    #[doc = "POWER RESETREAS LOCKUP: Reset from CPU lock-up detected."]
    #[inline]
    pub(crate) fn write_power_resetreas_lockup(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write POWER RESETREAS LOCKUP reset value false")
    }
    #[doc = "POWER RESETREAS OFF: Reset from wake-up from OFF mode detected by the use of DETECT signal from GPIO."]
    #[inline]
    pub(crate) fn read_power_resetreas_off(&self) -> MemResult<bool> {
        todo!("read POWER RESETREAS OFF reset value false")
    }
    #[doc = "POWER RESETREAS OFF: Reset from wake-up from OFF mode detected by the use of DETECT signal from GPIO."]
    #[inline]
    pub(crate) fn write_power_resetreas_off(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write POWER RESETREAS OFF reset value false")
    }
    #[doc = "POWER RESETREAS LPCOMP: Reset from wake-up from OFF mode detected by the use of ANADETECT signal from LPCOMP."]
    #[inline]
    pub(crate) fn read_power_resetreas_lpcomp(&self) -> MemResult<bool> {
        todo!("read POWER RESETREAS LPCOMP reset value false")
    }
    #[doc = "POWER RESETREAS LPCOMP: Reset from wake-up from OFF mode detected by the use of ANADETECT signal from LPCOMP."]
    #[inline]
    pub(crate) fn write_power_resetreas_lpcomp(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write POWER RESETREAS LPCOMP reset value false")
    }
    #[doc = "POWER RESETREAS DIF: Reset from wake-up from OFF mode detected by entering into debug interface mode."]
    #[inline]
    pub(crate) fn read_power_resetreas_dif(&self) -> MemResult<bool> {
        todo!("read POWER RESETREAS DIF reset value false")
    }
    #[doc = "POWER RESETREAS DIF: Reset from wake-up from OFF mode detected by entering into debug interface mode."]
    #[inline]
    pub(crate) fn write_power_resetreas_dif(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write POWER RESETREAS DIF reset value false")
    }
    #[doc = "POWER RAMSTATUS RAMBLOCK0: RAM block 0 status."]
    #[inline]
    pub(crate) fn read_power_ramstatus_ramblock0(&self) -> MemResult<bool> {
        Ok(self.ram[0].is_on())
    }
    #[doc = "POWER RAMSTATUS RAMBLOCK1: RAM block 1 status."]
    #[inline]
    pub(crate) fn read_power_ramstatus_ramblock1(&self) -> MemResult<bool> {
        Ok(self.ram[1].is_on())
    }
    #[doc = "POWER RAMSTATUS RAMBLOCK2: RAM block 2 status."]
    #[inline]
    pub(crate) fn read_power_ramstatus_ramblock2(&self) -> MemResult<bool> {
        Ok(self.ram[2].is_on())
    }
    #[doc = "POWER RAMSTATUS RAMBLOCK3: RAM block 3 status."]
    #[inline]
    pub(crate) fn read_power_ramstatus_ramblock3(&self) -> MemResult<bool> {
        Ok(self.ram[3].is_on())
    }
    #[doc = "POWER SYSTEMOFF SYSTEMOFF: Enter system off mode."]
    #[inline]
    pub(crate) fn write_power_systemoff_systemoff(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write POWER SYSTEMOFF SYSTEMOFF reset value false")
    }
    #[doc = "POWER POFCON POF: Power failure comparator enable."]
    #[inline]
    pub(crate) fn read_power_pofcon_pof(&self) -> MemResult<bool> {
        todo!("read POWER POFCON POF reset value false")
    }
    #[doc = "POWER POFCON POF: Power failure comparator enable."]
    #[inline]
    pub(crate) fn write_power_pofcon_pof(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write POWER POFCON POF reset value false")
    }
    #[doc = "POWER POFCON THRESHOLD: Set threshold level."]
    #[inline]
    pub(crate) fn read_power_pofcon_threshold(&self) -> MemResult<u8> {
        todo!("read POWER POFCON THRESHOLD reset value 0x00 mask 0x03")
    }
    #[doc = "POWER POFCON THRESHOLD: Set threshold level."]
    #[inline]
    pub(crate) fn write_power_pofcon_threshold(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write POWER POFCON THRESHOLD reset value 0x00 mask 0x03")
    }
    #[doc = "POWER GPREGRET GPREGRET: General purpose retention register."]
    #[inline]
    pub(crate) fn read_power_gpregret_gpregret(&self) -> MemResult<u8> {
        todo!("read POWER GPREGRET GPREGRET reset value 0x00 mask 0xff")
    }
    #[doc = "POWER GPREGRET GPREGRET: General purpose retention register."]
    #[inline]
    pub(crate) fn write_power_gpregret_gpregret(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write POWER GPREGRET GPREGRET reset value 0x00 mask 0xff")
    }
    #[doc = "POWER RAMON ONRAM0: RAM block 0 behaviour in ON mode."]
    #[inline]
    pub(crate) fn read_power_ramon_onram0(&self) -> MemResult<bool> {
        Ok(self.ram[0].is_keep_on())
    }
    #[doc = "POWER RAMON ONRAM0: RAM block 0 behaviour in ON mode."]
    #[inline]
    pub(crate) fn write_power_ramon_onram0(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ram[0].set_keep_on(_value))
    }
    #[doc = "POWER RAMON ONRAM1: RAM block 1 behaviour in ON mode."]
    #[inline]
    pub(crate) fn read_power_ramon_onram1(&self) -> MemResult<bool> {
        Ok(self.ram[1].is_keep_on())
    }
    #[doc = "POWER RAMON ONRAM1: RAM block 1 behaviour in ON mode."]
    #[inline]
    pub(crate) fn write_power_ramon_onram1(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ram[1].set_keep_on(_value))
    }
    #[doc = "POWER RAMON OFFRAM0: RAM block 0 behaviour in OFF mode."]
    #[inline]
    pub(crate) fn read_power_ramon_offram0(&self) -> MemResult<bool> {
        Ok(self.ram[0].is_retain_on())
    }
    #[doc = "POWER RAMON OFFRAM0: RAM block 0 behaviour in OFF mode."]
    #[inline]
    pub(crate) fn write_power_ramon_offram0(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ram[0].set_retain_on(_value))
    }
    #[doc = "POWER RAMON OFFRAM1: RAM block 1 behaviour in OFF mode."]
    #[inline]
    pub(crate) fn read_power_ramon_offram1(&self) -> MemResult<bool> {
        Ok(self.ram[1].is_retain_on())
    }
    #[doc = "POWER RAMON OFFRAM1: RAM block 1 behaviour in OFF mode."]
    #[inline]
    pub(crate) fn write_power_ramon_offram1(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ram[1].set_retain_on(_value))
    }
    #[doc = "POWER RESET RESET: Enable or disable pin reset in debug interface mode."]
    #[inline]
    pub(crate) fn read_power_reset_reset(&self) -> MemResult<bool> {
        todo!("read POWER RESET RESET reset value false")
    }
    #[doc = "POWER RESET RESET: Enable or disable pin reset in debug interface mode."]
    #[inline]
    pub(crate) fn write_power_reset_reset(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write POWER RESET RESET reset value false")
    }
    #[doc = "POWER RAMONB ONRAM2: RAM block 2 behaviour in ON mode."]
    #[inline]
    pub(crate) fn read_power_ramonb_onram2(&self) -> MemResult<bool> {
        Ok(self.ram[2].is_keep_on())
    }
    #[doc = "POWER RAMONB ONRAM2: RAM block 2 behaviour in ON mode."]
    #[inline]
    pub(crate) fn write_power_ramonb_onram2(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ram[2].set_keep_on(_value))
    }
    #[doc = "POWER RAMONB ONRAM3: RAM block 3 behaviour in ON mode."]
    #[inline]
    pub(crate) fn read_power_ramonb_onram3(&self) -> MemResult<bool> {
        Ok(self.ram[3].is_keep_on())
    }
    #[doc = "POWER RAMONB ONRAM3: RAM block 3 behaviour in ON mode."]
    #[inline]
    pub(crate) fn write_power_ramonb_onram3(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ram[3].set_keep_on(_value))
    }
    #[doc = "POWER RAMONB OFFRAM2: RAM block 2 behaviour in OFF mode."]
    #[inline]
    pub(crate) fn read_power_ramonb_offram2(&self) -> MemResult<bool> {
        Ok(self.ram[2].is_retain_on())
    }
    #[doc = "POWER RAMONB OFFRAM2: RAM block 2 behaviour in OFF mode."]
    #[inline]
    pub(crate) fn write_power_ramonb_offram2(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ram[2].set_retain_on(_value))
    }
    #[doc = "POWER RAMONB OFFRAM3: RAM block 3 behaviour in OFF mode."]
    #[inline]
    pub(crate) fn read_power_ramonb_offram3(&self) -> MemResult<bool> {
        Ok(self.ram[3].is_retain_on())
    }
    #[doc = "POWER RAMONB OFFRAM3: RAM block 3 behaviour in OFF mode."]
    #[inline]
    pub(crate) fn write_power_ramonb_offram3(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ram[3].set_retain_on(_value))
    }
    #[doc = "POWER DCDCEN DCDCEN: Enable DCDC converter."]
    #[inline]
    pub(crate) fn read_power_dcdcen_dcdcen(&self) -> MemResult<bool> {
        todo!("read POWER DCDCEN DCDCEN reset value false")
    }
    #[doc = "POWER DCDCEN DCDCEN: Enable DCDC converter."]
    #[inline]
    pub(crate) fn write_power_dcdcen_dcdcen(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write POWER DCDCEN DCDCEN reset value false")
    }
    #[doc = "POWER DCDCFORCE FORCEOFF: DCDC power-up force off."]
    #[inline]
    pub(crate) fn read_power_dcdcforce_forceoff(&self) -> MemResult<bool> {
        todo!("read POWER DCDCFORCE FORCEOFF reset value false")
    }
    #[doc = "POWER DCDCFORCE FORCEOFF: DCDC power-up force off."]
    #[inline]
    pub(crate) fn write_power_dcdcforce_forceoff(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write POWER DCDCFORCE FORCEOFF reset value false")
    }
    #[doc = "POWER DCDCFORCE FORCEON: DCDC power-up force on."]
    #[inline]
    pub(crate) fn read_power_dcdcforce_forceon(&self) -> MemResult<bool> {
        todo!("read POWER DCDCFORCE FORCEON reset value false")
    }
    #[doc = "POWER DCDCFORCE FORCEON: DCDC power-up force on."]
    #[inline]
    pub(crate) fn write_power_dcdcforce_forceon(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write POWER DCDCFORCE FORCEON reset value false")
    }
    #[doc = "CLOCK TASKS_HFCLKSTART: Start HFCLK clock source."]
    #[inline]
    pub(crate) fn write_clock_tasks_hfclkstart(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write CLOCK TASKS_HFCLKSTART reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "CLOCK TASKS_HFCLKSTOP: Stop HFCLK clock source."]
    #[inline]
    pub(crate) fn write_clock_tasks_hfclkstop(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write CLOCK TASKS_HFCLKSTOP reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "CLOCK TASKS_LFCLKSTART: Start LFCLK clock source."]
    #[inline]
    pub(crate) fn write_clock_tasks_lfclkstart(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write CLOCK TASKS_LFCLKSTART reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "CLOCK TASKS_LFCLKSTOP: Stop LFCLK clock source."]
    #[inline]
    pub(crate) fn write_clock_tasks_lfclkstop(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write CLOCK TASKS_LFCLKSTOP reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "CLOCK TASKS_CAL: Start calibration of LFCLK RC oscillator."]
    #[inline]
    pub(crate) fn write_clock_tasks_cal(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write CLOCK TASKS_CAL reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "CLOCK TASKS_CTSTART: Start calibration timer."]
    #[inline]
    pub(crate) fn write_clock_tasks_ctstart(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write CLOCK TASKS_CTSTART reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "CLOCK TASKS_CTSTOP: Stop calibration timer."]
    #[inline]
    pub(crate) fn write_clock_tasks_ctstop(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!(
            "write CLOCK TASKS_CTSTOP reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "CLOCK EVENTS_HFCLKSTARTED: HFCLK oscillator started."]
    #[inline]
    pub(crate) fn read_clock_events_hfclkstarted(&self) -> MemResult<u32> {
        todo ! ("read CLOCK EVENTS_HFCLKSTARTED reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "CLOCK EVENTS_HFCLKSTARTED: HFCLK oscillator started."]
    #[inline]
    pub(crate) fn write_clock_events_hfclkstarted(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write CLOCK EVENTS_HFCLKSTARTED reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "CLOCK EVENTS_LFCLKSTARTED: LFCLK oscillator started."]
    #[inline]
    pub(crate) fn read_clock_events_lfclkstarted(&self) -> MemResult<u32> {
        todo ! ("read CLOCK EVENTS_LFCLKSTARTED reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "CLOCK EVENTS_LFCLKSTARTED: LFCLK oscillator started."]
    #[inline]
    pub(crate) fn write_clock_events_lfclkstarted(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write CLOCK EVENTS_LFCLKSTARTED reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "CLOCK EVENTS_DONE: Calibration of LFCLK RC oscillator completed."]
    #[inline]
    pub(crate) fn read_clock_events_done(&self) -> MemResult<u32> {
        todo!("read CLOCK EVENTS_DONE reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "CLOCK EVENTS_DONE: Calibration of LFCLK RC oscillator completed."]
    #[inline]
    pub(crate) fn write_clock_events_done(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!(
            "write CLOCK EVENTS_DONE reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "CLOCK EVENTS_CTTO: Calibration timer timeout."]
    #[inline]
    pub(crate) fn read_clock_events_ctto(&self) -> MemResult<u32> {
        todo!("read CLOCK EVENTS_CTTO reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "CLOCK EVENTS_CTTO: Calibration timer timeout."]
    #[inline]
    pub(crate) fn write_clock_events_ctto(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!(
            "write CLOCK EVENTS_CTTO reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "CLOCK HFCLKRUN STATUS: Task HFCLKSTART trigger status."]
    #[inline]
    pub(crate) fn read_clock_hfclkrun_status(&self) -> MemResult<bool> {
        todo!("read CLOCK HFCLKRUN STATUS reset value false")
    }
    #[doc = "CLOCK HFCLKSTAT SRC: Active clock source for the HF clock."]
    #[inline]
    pub(crate) fn read_clock_hfclkstat_src(&self) -> MemResult<bool> {
        todo!("read CLOCK HFCLKSTAT SRC reset value false")
    }
    #[doc = "CLOCK HFCLKSTAT STATE: State for the HFCLK."]
    #[inline]
    pub(crate) fn read_clock_hfclkstat_state(&self) -> MemResult<bool> {
        todo!("read CLOCK HFCLKSTAT STATE reset value false")
    }
    #[doc = "CLOCK LFCLKRUN STATUS: Task LFCLKSTART triggered status."]
    #[inline]
    pub(crate) fn read_clock_lfclkrun_status(&self) -> MemResult<bool> {
        todo!("read CLOCK LFCLKRUN STATUS reset value false")
    }
    #[doc = "CLOCK LFCLKSTAT SRC: Active clock source for the LF clock."]
    #[inline]
    pub(crate) fn read_clock_lfclkstat_src(&self) -> MemResult<u8> {
        todo!("read CLOCK LFCLKSTAT SRC reset value 0x00 mask 0x03")
    }
    #[doc = "CLOCK LFCLKSTAT STATE: State for the LF clock."]
    #[inline]
    pub(crate) fn read_clock_lfclkstat_state(&self) -> MemResult<bool> {
        todo!("read CLOCK LFCLKSTAT STATE reset value false")
    }
    #[doc = "CLOCK LFCLKSRCCOPY SRC: Clock source for the LFCLK clock, set when task LKCLKSTART is triggered."]
    #[inline]
    pub(crate) fn read_clock_lfclksrccopy_src(&self) -> MemResult<u8> {
        todo!("read CLOCK LFCLKSRCCOPY SRC reset value 0x00 mask 0x03")
    }
    #[doc = "CLOCK LFCLKSRC SRC: Clock source."]
    #[inline]
    pub(crate) fn read_clock_lfclksrc_src(&self) -> MemResult<u8> {
        todo!("read CLOCK LFCLKSRC SRC reset value 0x00 mask 0x03")
    }
    #[doc = "CLOCK LFCLKSRC SRC: Clock source."]
    #[inline]
    pub(crate) fn write_clock_lfclksrc_src(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write CLOCK LFCLKSRC SRC reset value 0x00 mask 0x03")
    }
    #[doc = "CLOCK CTIV CTIV: Calibration timer interval in 0.25s resolution."]
    #[inline]
    pub(crate) fn read_clock_ctiv_ctiv(&self) -> MemResult<u8> {
        todo!("read CLOCK CTIV CTIV reset value 0x00 mask 0x7f")
    }
    #[doc = "CLOCK CTIV CTIV: Calibration timer interval in 0.25s resolution."]
    #[inline]
    pub(crate) fn write_clock_ctiv_ctiv(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write CLOCK CTIV CTIV reset value 0x00 mask 0x7f")
    }
    #[doc = "CLOCK XTALFREQ XTALFREQ: External Xtal frequency selection."]
    #[inline]
    pub(crate) fn read_clock_xtalfreq_xtalfreq(&self) -> MemResult<u8> {
        todo!("read CLOCK XTALFREQ XTALFREQ reset value 0xff mask 0xff")
    }
    #[doc = "CLOCK XTALFREQ XTALFREQ: External Xtal frequency selection."]
    #[inline]
    pub(crate) fn write_clock_xtalfreq_xtalfreq(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write CLOCK XTALFREQ XTALFREQ reset value 0xff mask 0xff")
    }
    #[doc = "MPU PERR0 POWER_CLOCK: POWER_CLOCK region configuration."]
    #[inline]
    pub(crate) fn read_mpu_perr0_power_clock(&self) -> MemResult<bool> {
        todo!("read MPU PERR0 POWER_CLOCK reset value false")
    }
    #[doc = "MPU PERR0 POWER_CLOCK: POWER_CLOCK region configuration."]
    #[inline]
    pub(crate) fn write_mpu_perr0_power_clock(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU PERR0 POWER_CLOCK reset value false")
    }
    #[doc = "MPU PERR0 RADIO: RADIO region configuration."]
    #[inline]
    pub(crate) fn read_mpu_perr0_radio(&self) -> MemResult<bool> {
        todo!("read MPU PERR0 RADIO reset value false")
    }
    #[doc = "MPU PERR0 RADIO: RADIO region configuration."]
    #[inline]
    pub(crate) fn write_mpu_perr0_radio(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU PERR0 RADIO reset value false")
    }
    #[doc = "MPU PERR0 UART0: UART0 region configuration."]
    #[inline]
    pub(crate) fn read_mpu_perr0_uart0(&self) -> MemResult<bool> {
        todo!("read MPU PERR0 UART0 reset value false")
    }
    #[doc = "MPU PERR0 UART0: UART0 region configuration."]
    #[inline]
    pub(crate) fn write_mpu_perr0_uart0(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU PERR0 UART0 reset value false")
    }
    #[doc = "MPU PERR0 SPI0_TWI0: SPI0 and TWI0 region configuration."]
    #[inline]
    pub(crate) fn read_mpu_perr0_spi0_twi0(&self) -> MemResult<bool> {
        todo!("read MPU PERR0 SPI0_TWI0 reset value false")
    }
    #[doc = "MPU PERR0 SPI0_TWI0: SPI0 and TWI0 region configuration."]
    #[inline]
    pub(crate) fn write_mpu_perr0_spi0_twi0(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU PERR0 SPI0_TWI0 reset value false")
    }
    #[doc = "MPU PERR0 SPI1_TWI1: SPI1 and TWI1 region configuration."]
    #[inline]
    pub(crate) fn read_mpu_perr0_spi1_twi1(&self) -> MemResult<bool> {
        todo!("read MPU PERR0 SPI1_TWI1 reset value false")
    }
    #[doc = "MPU PERR0 SPI1_TWI1: SPI1 and TWI1 region configuration."]
    #[inline]
    pub(crate) fn write_mpu_perr0_spi1_twi1(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU PERR0 SPI1_TWI1 reset value false")
    }
    #[doc = "MPU PERR0 GPIOTE: GPIOTE region configuration."]
    #[inline]
    pub(crate) fn read_mpu_perr0_gpiote(&self) -> MemResult<bool> {
        todo!("read MPU PERR0 GPIOTE reset value false")
    }
    #[doc = "MPU PERR0 GPIOTE: GPIOTE region configuration."]
    #[inline]
    pub(crate) fn write_mpu_perr0_gpiote(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU PERR0 GPIOTE reset value false")
    }
    #[doc = "MPU PERR0 ADC: ADC region configuration."]
    #[inline]
    pub(crate) fn read_mpu_perr0_adc(&self) -> MemResult<bool> {
        todo!("read MPU PERR0 ADC reset value false")
    }
    #[doc = "MPU PERR0 ADC: ADC region configuration."]
    #[inline]
    pub(crate) fn write_mpu_perr0_adc(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU PERR0 ADC reset value false")
    }
    #[doc = "MPU PERR0 TIMER0: TIMER0 region configuration."]
    #[inline]
    pub(crate) fn read_mpu_perr0_timer0(&self) -> MemResult<bool> {
        todo!("read MPU PERR0 TIMER0 reset value false")
    }
    #[doc = "MPU PERR0 TIMER0: TIMER0 region configuration."]
    #[inline]
    pub(crate) fn write_mpu_perr0_timer0(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU PERR0 TIMER0 reset value false")
    }
    #[doc = "MPU PERR0 TIMER1: TIMER1 region configuration."]
    #[inline]
    pub(crate) fn read_mpu_perr0_timer1(&self) -> MemResult<bool> {
        todo!("read MPU PERR0 TIMER1 reset value false")
    }
    #[doc = "MPU PERR0 TIMER1: TIMER1 region configuration."]
    #[inline]
    pub(crate) fn write_mpu_perr0_timer1(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU PERR0 TIMER1 reset value false")
    }
    #[doc = "MPU PERR0 TIMER2: TIMER2 region configuration."]
    #[inline]
    pub(crate) fn read_mpu_perr0_timer2(&self) -> MemResult<bool> {
        todo!("read MPU PERR0 TIMER2 reset value false")
    }
    #[doc = "MPU PERR0 TIMER2: TIMER2 region configuration."]
    #[inline]
    pub(crate) fn write_mpu_perr0_timer2(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU PERR0 TIMER2 reset value false")
    }
    #[doc = "MPU PERR0 RTC0: RTC0 region configuration."]
    #[inline]
    pub(crate) fn read_mpu_perr0_rtc0(&self) -> MemResult<bool> {
        todo!("read MPU PERR0 RTC0 reset value false")
    }
    #[doc = "MPU PERR0 RTC0: RTC0 region configuration."]
    #[inline]
    pub(crate) fn write_mpu_perr0_rtc0(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU PERR0 RTC0 reset value false")
    }
    #[doc = "MPU PERR0 TEMP: TEMP region configuration."]
    #[inline]
    pub(crate) fn read_mpu_perr0_temp(&self) -> MemResult<bool> {
        todo!("read MPU PERR0 TEMP reset value false")
    }
    #[doc = "MPU PERR0 TEMP: TEMP region configuration."]
    #[inline]
    pub(crate) fn write_mpu_perr0_temp(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU PERR0 TEMP reset value false")
    }
    #[doc = "MPU PERR0 RNG: RNG region configuration."]
    #[inline]
    pub(crate) fn read_mpu_perr0_rng(&self) -> MemResult<bool> {
        todo!("read MPU PERR0 RNG reset value false")
    }
    #[doc = "MPU PERR0 RNG: RNG region configuration."]
    #[inline]
    pub(crate) fn write_mpu_perr0_rng(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU PERR0 RNG reset value false")
    }
    #[doc = "MPU PERR0 ECB: ECB region configuration."]
    #[inline]
    pub(crate) fn read_mpu_perr0_ecb(&self) -> MemResult<bool> {
        todo!("read MPU PERR0 ECB reset value false")
    }
    #[doc = "MPU PERR0 ECB: ECB region configuration."]
    #[inline]
    pub(crate) fn write_mpu_perr0_ecb(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU PERR0 ECB reset value false")
    }
    #[doc = "MPU PERR0 CCM_AAR: CCM and AAR region configuration."]
    #[inline]
    pub(crate) fn read_mpu_perr0_ccm_aar(&self) -> MemResult<bool> {
        todo!("read MPU PERR0 CCM_AAR reset value false")
    }
    #[doc = "MPU PERR0 CCM_AAR: CCM and AAR region configuration."]
    #[inline]
    pub(crate) fn write_mpu_perr0_ccm_aar(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU PERR0 CCM_AAR reset value false")
    }
    #[doc = "MPU PERR0 WDT: WDT region configuration."]
    #[inline]
    pub(crate) fn read_mpu_perr0_wdt(&self) -> MemResult<bool> {
        todo!("read MPU PERR0 WDT reset value false")
    }
    #[doc = "MPU PERR0 WDT: WDT region configuration."]
    #[inline]
    pub(crate) fn write_mpu_perr0_wdt(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU PERR0 WDT reset value false")
    }
    #[doc = "MPU PERR0 RTC1: RTC1 region configuration."]
    #[inline]
    pub(crate) fn read_mpu_perr0_rtc1(&self) -> MemResult<bool> {
        todo!("read MPU PERR0 RTC1 reset value false")
    }
    #[doc = "MPU PERR0 RTC1: RTC1 region configuration."]
    #[inline]
    pub(crate) fn write_mpu_perr0_rtc1(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU PERR0 RTC1 reset value false")
    }
    #[doc = "MPU PERR0 QDEC: QDEC region configuration."]
    #[inline]
    pub(crate) fn read_mpu_perr0_qdec(&self) -> MemResult<bool> {
        todo!("read MPU PERR0 QDEC reset value false")
    }
    #[doc = "MPU PERR0 QDEC: QDEC region configuration."]
    #[inline]
    pub(crate) fn write_mpu_perr0_qdec(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU PERR0 QDEC reset value false")
    }
    #[doc = "MPU PERR0 LPCOMP: LPCOMP region configuration."]
    #[inline]
    pub(crate) fn read_mpu_perr0_lpcomp(&self) -> MemResult<bool> {
        todo!("read MPU PERR0 LPCOMP reset value false")
    }
    #[doc = "MPU PERR0 LPCOMP: LPCOMP region configuration."]
    #[inline]
    pub(crate) fn write_mpu_perr0_lpcomp(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU PERR0 LPCOMP reset value false")
    }
    #[doc = "MPU PERR0 NVMC: NVMC region configuration."]
    #[inline]
    pub(crate) fn read_mpu_perr0_nvmc(&self) -> MemResult<bool> {
        todo!("read MPU PERR0 NVMC reset value false")
    }
    #[doc = "MPU PERR0 NVMC: NVMC region configuration."]
    #[inline]
    pub(crate) fn write_mpu_perr0_nvmc(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU PERR0 NVMC reset value false")
    }
    #[doc = "MPU PERR0 PPI: PPI region configuration."]
    #[inline]
    pub(crate) fn read_mpu_perr0_ppi(&self) -> MemResult<bool> {
        todo!("read MPU PERR0 PPI reset value false")
    }
    #[doc = "MPU PERR0 PPI: PPI region configuration."]
    #[inline]
    pub(crate) fn write_mpu_perr0_ppi(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU PERR0 PPI reset value false")
    }
    #[doc = "MPU RLENR0: Length of RAM region 0."]
    #[inline]
    pub(crate) fn read_mpu_rlenr0(&self) -> MemResult<u32> {
        todo!("read MPU RLENR0 reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "MPU RLENR0: Length of RAM region 0."]
    #[inline]
    pub(crate) fn write_mpu_rlenr0(&mut self, _value: u32) -> MemResult<()> {
        todo!("write MPU RLENR0 reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "MPU PROTENSET0 PROTREG0: Protection enable for region 0."]
    #[inline]
    pub(crate) fn read_mpu_protenset0_protreg0(&self) -> MemResult<bool> {
        Ok(self.mpu[0].is_protected())
    }
    #[doc = "MPU PROTENSET0 PROTREG0: Protection enable for region 0."]
    #[inline]
    pub(crate) fn write_mpu_protenset0_protreg0(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[0].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET0 PROTREG1: Protection enable for region 1."]
    #[inline]
    pub(crate) fn read_mpu_protenset0_protreg1(&self) -> MemResult<bool> {
        Ok(self.mpu[1].is_protected())
    }
    #[doc = "MPU PROTENSET0 PROTREG1: Protection enable for region 1."]
    #[inline]
    pub(crate) fn write_mpu_protenset0_protreg1(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[1].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET0 PROTREG2: Protection enable for region 2."]
    #[inline]
    pub(crate) fn read_mpu_protenset0_protreg2(&self) -> MemResult<bool> {
        Ok(self.mpu[2].is_protected())
    }
    #[doc = "MPU PROTENSET0 PROTREG2: Protection enable for region 2."]
    #[inline]
    pub(crate) fn write_mpu_protenset0_protreg2(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[2].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET0 PROTREG3: Protection enable for region 3."]
    #[inline]
    pub(crate) fn read_mpu_protenset0_protreg3(&self) -> MemResult<bool> {
        Ok(self.mpu[3].is_protected())
    }
    #[doc = "MPU PROTENSET0 PROTREG3: Protection enable for region 3."]
    #[inline]
    pub(crate) fn write_mpu_protenset0_protreg3(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[3].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET0 PROTREG4: Protection enable for region 4."]
    #[inline]
    pub(crate) fn read_mpu_protenset0_protreg4(&self) -> MemResult<bool> {
        Ok(self.mpu[4].is_protected())
    }
    #[doc = "MPU PROTENSET0 PROTREG4: Protection enable for region 4."]
    #[inline]
    pub(crate) fn write_mpu_protenset0_protreg4(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[4].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET0 PROTREG5: Protection enable for region 5."]
    #[inline]
    pub(crate) fn read_mpu_protenset0_protreg5(&self) -> MemResult<bool> {
        Ok(self.mpu[5].is_protected())
    }
    #[doc = "MPU PROTENSET0 PROTREG5: Protection enable for region 5."]
    #[inline]
    pub(crate) fn write_mpu_protenset0_protreg5(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[5].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET0 PROTREG6: Protection enable for region 6."]
    #[inline]
    pub(crate) fn read_mpu_protenset0_protreg6(&self) -> MemResult<bool> {
        Ok(self.mpu[6].is_protected())
    }
    #[doc = "MPU PROTENSET0 PROTREG6: Protection enable for region 6."]
    #[inline]
    pub(crate) fn write_mpu_protenset0_protreg6(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[6].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET0 PROTREG7: Protection enable for region 7."]
    #[inline]
    pub(crate) fn read_mpu_protenset0_protreg7(&self) -> MemResult<bool> {
        Ok(self.mpu[7].is_protected())
    }
    #[doc = "MPU PROTENSET0 PROTREG7: Protection enable for region 7."]
    #[inline]
    pub(crate) fn write_mpu_protenset0_protreg7(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[7].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET0 PROTREG8: Protection enable for region 8."]
    #[inline]
    pub(crate) fn read_mpu_protenset0_protreg8(&self) -> MemResult<bool> {
        Ok(self.mpu[8].is_protected())
    }
    #[doc = "MPU PROTENSET0 PROTREG8: Protection enable for region 8."]
    #[inline]
    pub(crate) fn write_mpu_protenset0_protreg8(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[8].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET0 PROTREG9: Protection enable for region 9."]
    #[inline]
    pub(crate) fn read_mpu_protenset0_protreg9(&self) -> MemResult<bool> {
        Ok(self.mpu[9].is_protected())
    }
    #[doc = "MPU PROTENSET0 PROTREG9: Protection enable for region 9."]
    #[inline]
    pub(crate) fn write_mpu_protenset0_protreg9(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[9].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET0 PROTREG10: Protection enable for region 10."]
    #[inline]
    pub(crate) fn read_mpu_protenset0_protreg10(&self) -> MemResult<bool> {
        Ok(self.mpu[10].is_protected())
    }
    #[doc = "MPU PROTENSET0 PROTREG10: Protection enable for region 10."]
    #[inline]
    pub(crate) fn write_mpu_protenset0_protreg10(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[10].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET0 PROTREG11: Protection enable for region 11."]
    #[inline]
    pub(crate) fn read_mpu_protenset0_protreg11(&self) -> MemResult<bool> {
        Ok(self.mpu[11].is_protected())
    }
    #[doc = "MPU PROTENSET0 PROTREG11: Protection enable for region 11."]
    #[inline]
    pub(crate) fn write_mpu_protenset0_protreg11(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[11].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET0 PROTREG12: Protection enable for region 12."]
    #[inline]
    pub(crate) fn read_mpu_protenset0_protreg12(&self) -> MemResult<bool> {
        Ok(self.mpu[12].is_protected())
    }
    #[doc = "MPU PROTENSET0 PROTREG12: Protection enable for region 12."]
    #[inline]
    pub(crate) fn write_mpu_protenset0_protreg12(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[12].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET0 PROTREG13: Protection enable for region 13."]
    #[inline]
    pub(crate) fn read_mpu_protenset0_protreg13(&self) -> MemResult<bool> {
        Ok(self.mpu[13].is_protected())
    }
    #[doc = "MPU PROTENSET0 PROTREG13: Protection enable for region 13."]
    #[inline]
    pub(crate) fn write_mpu_protenset0_protreg13(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[13].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET0 PROTREG14: Protection enable for region 14."]
    #[inline]
    pub(crate) fn read_mpu_protenset0_protreg14(&self) -> MemResult<bool> {
        Ok(self.mpu[14].is_protected())
    }
    #[doc = "MPU PROTENSET0 PROTREG14: Protection enable for region 14."]
    #[inline]
    pub(crate) fn write_mpu_protenset0_protreg14(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[14].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET0 PROTREG15: Protection enable for region 15."]
    #[inline]
    pub(crate) fn read_mpu_protenset0_protreg15(&self) -> MemResult<bool> {
        Ok(self.mpu[15].is_protected())
    }
    #[doc = "MPU PROTENSET0 PROTREG15: Protection enable for region 15."]
    #[inline]
    pub(crate) fn write_mpu_protenset0_protreg15(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[15].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET0 PROTREG16: Protection enable for region 16."]
    #[inline]
    pub(crate) fn read_mpu_protenset0_protreg16(&self) -> MemResult<bool> {
        Ok(self.mpu[16].is_protected())
    }
    #[doc = "MPU PROTENSET0 PROTREG16: Protection enable for region 16."]
    #[inline]
    pub(crate) fn write_mpu_protenset0_protreg16(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[16].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET0 PROTREG17: Protection enable for region 17."]
    #[inline]
    pub(crate) fn read_mpu_protenset0_protreg17(&self) -> MemResult<bool> {
        Ok(self.mpu[17].is_protected())
    }
    #[doc = "MPU PROTENSET0 PROTREG17: Protection enable for region 17."]
    #[inline]
    pub(crate) fn write_mpu_protenset0_protreg17(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[17].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET0 PROTREG18: Protection enable for region 18."]
    #[inline]
    pub(crate) fn read_mpu_protenset0_protreg18(&self) -> MemResult<bool> {
        Ok(self.mpu[18].is_protected())
    }
    #[doc = "MPU PROTENSET0 PROTREG18: Protection enable for region 18."]
    #[inline]
    pub(crate) fn write_mpu_protenset0_protreg18(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[18].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET0 PROTREG19: Protection enable for region 19."]
    #[inline]
    pub(crate) fn read_mpu_protenset0_protreg19(&self) -> MemResult<bool> {
        Ok(self.mpu[19].is_protected())
    }
    #[doc = "MPU PROTENSET0 PROTREG19: Protection enable for region 19."]
    #[inline]
    pub(crate) fn write_mpu_protenset0_protreg19(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[19].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET0 PROTREG20: Protection enable for region 20."]
    #[inline]
    pub(crate) fn read_mpu_protenset0_protreg20(&self) -> MemResult<bool> {
        Ok(self.mpu[20].is_protected())
    }
    #[doc = "MPU PROTENSET0 PROTREG20: Protection enable for region 20."]
    #[inline]
    pub(crate) fn write_mpu_protenset0_protreg20(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[20].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET0 PROTREG21: Protection enable for region 21."]
    #[inline]
    pub(crate) fn read_mpu_protenset0_protreg21(&self) -> MemResult<bool> {
        Ok(self.mpu[21].is_protected())
    }
    #[doc = "MPU PROTENSET0 PROTREG21: Protection enable for region 21."]
    #[inline]
    pub(crate) fn write_mpu_protenset0_protreg21(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[21].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET0 PROTREG22: Protection enable for region 22."]
    #[inline]
    pub(crate) fn read_mpu_protenset0_protreg22(&self) -> MemResult<bool> {
        Ok(self.mpu[22].is_protected())
    }
    #[doc = "MPU PROTENSET0 PROTREG22: Protection enable for region 22."]
    #[inline]
    pub(crate) fn write_mpu_protenset0_protreg22(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[22].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET0 PROTREG23: Protection enable for region 23."]
    #[inline]
    pub(crate) fn read_mpu_protenset0_protreg23(&self) -> MemResult<bool> {
        Ok(self.mpu[23].is_protected())
    }
    #[doc = "MPU PROTENSET0 PROTREG23: Protection enable for region 23."]
    #[inline]
    pub(crate) fn write_mpu_protenset0_protreg23(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[23].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET0 PROTREG24: Protection enable for region 24."]
    #[inline]
    pub(crate) fn read_mpu_protenset0_protreg24(&self) -> MemResult<bool> {
        Ok(self.mpu[24].is_protected())
    }
    #[doc = "MPU PROTENSET0 PROTREG24: Protection enable for region 24."]
    #[inline]
    pub(crate) fn write_mpu_protenset0_protreg24(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[24].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET0 PROTREG25: Protection enable for region 25."]
    #[inline]
    pub(crate) fn read_mpu_protenset0_protreg25(&self) -> MemResult<bool> {
        Ok(self.mpu[25].is_protected())
    }
    #[doc = "MPU PROTENSET0 PROTREG25: Protection enable for region 25."]
    #[inline]
    pub(crate) fn write_mpu_protenset0_protreg25(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[25].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET0 PROTREG26: Protection enable for region 26."]
    #[inline]
    pub(crate) fn read_mpu_protenset0_protreg26(&self) -> MemResult<bool> {
        Ok(self.mpu[26].is_protected())
    }
    #[doc = "MPU PROTENSET0 PROTREG26: Protection enable for region 26."]
    #[inline]
    pub(crate) fn write_mpu_protenset0_protreg26(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[26].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET0 PROTREG27: Protection enable for region 27."]
    #[inline]
    pub(crate) fn read_mpu_protenset0_protreg27(&self) -> MemResult<bool> {
        Ok(self.mpu[27].is_protected())
    }
    #[doc = "MPU PROTENSET0 PROTREG27: Protection enable for region 27."]
    #[inline]
    pub(crate) fn write_mpu_protenset0_protreg27(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[27].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET0 PROTREG28: Protection enable for region 28."]
    #[inline]
    pub(crate) fn read_mpu_protenset0_protreg28(&self) -> MemResult<bool> {
        Ok(self.mpu[28].is_protected())
    }
    #[doc = "MPU PROTENSET0 PROTREG28: Protection enable for region 28."]
    #[inline]
    pub(crate) fn write_mpu_protenset0_protreg28(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[28].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET0 PROTREG29: Protection enable for region 29."]
    #[inline]
    pub(crate) fn read_mpu_protenset0_protreg29(&self) -> MemResult<bool> {
        Ok(self.mpu[29].is_protected())
    }
    #[doc = "MPU PROTENSET0 PROTREG29: Protection enable for region 29."]
    #[inline]
    pub(crate) fn write_mpu_protenset0_protreg29(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[29].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET0 PROTREG30: Protection enable for region 30."]
    #[inline]
    pub(crate) fn read_mpu_protenset0_protreg30(&self) -> MemResult<bool> {
        Ok(self.mpu[30].is_protected())
    }
    #[doc = "MPU PROTENSET0 PROTREG30: Protection enable for region 30."]
    #[inline]
    pub(crate) fn write_mpu_protenset0_protreg30(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[30].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET0 PROTREG31: Protection enable for region 31."]
    #[inline]
    pub(crate) fn read_mpu_protenset0_protreg31(&self) -> MemResult<bool> {
        Ok(self.mpu[31].is_protected())
    }
    #[doc = "MPU PROTENSET0 PROTREG31: Protection enable for region 31."]
    #[inline]
    pub(crate) fn write_mpu_protenset0_protreg31(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[31].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET1 PROTREG32: Protection enable for region 32."]
    #[inline]
    pub(crate) fn read_mpu_protenset1_protreg32(&self) -> MemResult<bool> {
        Ok(self.mpu[32].is_protected())
    }
    #[doc = "MPU PROTENSET1 PROTREG32: Protection enable for region 32."]
    #[inline]
    pub(crate) fn write_mpu_protenset1_protreg32(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[32].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET1 PROTREG33: Protection enable for region 33."]
    #[inline]
    pub(crate) fn read_mpu_protenset1_protreg33(&self) -> MemResult<bool> {
        Ok(self.mpu[33].is_protected())
    }
    #[doc = "MPU PROTENSET1 PROTREG33: Protection enable for region 33."]
    #[inline]
    pub(crate) fn write_mpu_protenset1_protreg33(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[33].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET1 PROTREG34: Protection enable for region 34."]
    #[inline]
    pub(crate) fn read_mpu_protenset1_protreg34(&self) -> MemResult<bool> {
        Ok(self.mpu[34].is_protected())
    }
    #[doc = "MPU PROTENSET1 PROTREG34: Protection enable for region 34."]
    #[inline]
    pub(crate) fn write_mpu_protenset1_protreg34(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[34].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET1 PROTREG35: Protection enable for region 35."]
    #[inline]
    pub(crate) fn read_mpu_protenset1_protreg35(&self) -> MemResult<bool> {
        Ok(self.mpu[35].is_protected())
    }
    #[doc = "MPU PROTENSET1 PROTREG35: Protection enable for region 35."]
    #[inline]
    pub(crate) fn write_mpu_protenset1_protreg35(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[35].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET1 PROTREG36: Protection enable for region 36."]
    #[inline]
    pub(crate) fn read_mpu_protenset1_protreg36(&self) -> MemResult<bool> {
        Ok(self.mpu[36].is_protected())
    }
    #[doc = "MPU PROTENSET1 PROTREG36: Protection enable for region 36."]
    #[inline]
    pub(crate) fn write_mpu_protenset1_protreg36(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[36].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET1 PROTREG37: Protection enable for region 37."]
    #[inline]
    pub(crate) fn read_mpu_protenset1_protreg37(&self) -> MemResult<bool> {
        Ok(self.mpu[37].is_protected())
    }
    #[doc = "MPU PROTENSET1 PROTREG37: Protection enable for region 37."]
    #[inline]
    pub(crate) fn write_mpu_protenset1_protreg37(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[37].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET1 PROTREG38: Protection enable for region 38."]
    #[inline]
    pub(crate) fn read_mpu_protenset1_protreg38(&self) -> MemResult<bool> {
        Ok(self.mpu[38].is_protected())
    }
    #[doc = "MPU PROTENSET1 PROTREG38: Protection enable for region 38."]
    #[inline]
    pub(crate) fn write_mpu_protenset1_protreg38(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[38].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET1 PROTREG39: Protection enable for region 39."]
    #[inline]
    pub(crate) fn read_mpu_protenset1_protreg39(&self) -> MemResult<bool> {
        Ok(self.mpu[39].is_protected())
    }
    #[doc = "MPU PROTENSET1 PROTREG39: Protection enable for region 39."]
    #[inline]
    pub(crate) fn write_mpu_protenset1_protreg39(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[39].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET1 PROTREG40: Protection enable for region 40."]
    #[inline]
    pub(crate) fn read_mpu_protenset1_protreg40(&self) -> MemResult<bool> {
        Ok(self.mpu[40].is_protected())
    }
    #[doc = "MPU PROTENSET1 PROTREG40: Protection enable for region 40."]
    #[inline]
    pub(crate) fn write_mpu_protenset1_protreg40(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[40].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET1 PROTREG41: Protection enable for region 41."]
    #[inline]
    pub(crate) fn read_mpu_protenset1_protreg41(&self) -> MemResult<bool> {
        Ok(self.mpu[41].is_protected())
    }
    #[doc = "MPU PROTENSET1 PROTREG41: Protection enable for region 41."]
    #[inline]
    pub(crate) fn write_mpu_protenset1_protreg41(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[41].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET1 PROTREG42: Protection enable for region 42."]
    #[inline]
    pub(crate) fn read_mpu_protenset1_protreg42(&self) -> MemResult<bool> {
        Ok(self.mpu[42].is_protected())
    }
    #[doc = "MPU PROTENSET1 PROTREG42: Protection enable for region 42."]
    #[inline]
    pub(crate) fn write_mpu_protenset1_protreg42(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[42].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET1 PROTREG43: Protection enable for region 43."]
    #[inline]
    pub(crate) fn read_mpu_protenset1_protreg43(&self) -> MemResult<bool> {
        Ok(self.mpu[43].is_protected())
    }
    #[doc = "MPU PROTENSET1 PROTREG43: Protection enable for region 43."]
    #[inline]
    pub(crate) fn write_mpu_protenset1_protreg43(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[43].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET1 PROTREG44: Protection enable for region 44."]
    #[inline]
    pub(crate) fn read_mpu_protenset1_protreg44(&self) -> MemResult<bool> {
        Ok(self.mpu[44].is_protected())
    }
    #[doc = "MPU PROTENSET1 PROTREG44: Protection enable for region 44."]
    #[inline]
    pub(crate) fn write_mpu_protenset1_protreg44(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[44].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET1 PROTREG45: Protection enable for region 45."]
    #[inline]
    pub(crate) fn read_mpu_protenset1_protreg45(&self) -> MemResult<bool> {
        Ok(self.mpu[45].is_protected())
    }
    #[doc = "MPU PROTENSET1 PROTREG45: Protection enable for region 45."]
    #[inline]
    pub(crate) fn write_mpu_protenset1_protreg45(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[45].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET1 PROTREG46: Protection enable for region 46."]
    #[inline]
    pub(crate) fn read_mpu_protenset1_protreg46(&self) -> MemResult<bool> {
        Ok(self.mpu[46].is_protected())
    }
    #[doc = "MPU PROTENSET1 PROTREG46: Protection enable for region 46."]
    #[inline]
    pub(crate) fn write_mpu_protenset1_protreg46(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[46].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET1 PROTREG47: Protection enable for region 47."]
    #[inline]
    pub(crate) fn read_mpu_protenset1_protreg47(&self) -> MemResult<bool> {
        Ok(self.mpu[47].is_protected())
    }
    #[doc = "MPU PROTENSET1 PROTREG47: Protection enable for region 47."]
    #[inline]
    pub(crate) fn write_mpu_protenset1_protreg47(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[47].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET1 PROTREG48: Protection enable for region 48."]
    #[inline]
    pub(crate) fn read_mpu_protenset1_protreg48(&self) -> MemResult<bool> {
        Ok(self.mpu[48].is_protected())
    }
    #[doc = "MPU PROTENSET1 PROTREG48: Protection enable for region 48."]
    #[inline]
    pub(crate) fn write_mpu_protenset1_protreg48(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[48].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET1 PROTREG49: Protection enable for region 49."]
    #[inline]
    pub(crate) fn read_mpu_protenset1_protreg49(&self) -> MemResult<bool> {
        Ok(self.mpu[49].is_protected())
    }
    #[doc = "MPU PROTENSET1 PROTREG49: Protection enable for region 49."]
    #[inline]
    pub(crate) fn write_mpu_protenset1_protreg49(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[49].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET1 PROTREG50: Protection enable for region 50."]
    #[inline]
    pub(crate) fn read_mpu_protenset1_protreg50(&self) -> MemResult<bool> {
        Ok(self.mpu[50].is_protected())
    }
    #[doc = "MPU PROTENSET1 PROTREG50: Protection enable for region 50."]
    #[inline]
    pub(crate) fn write_mpu_protenset1_protreg50(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[50].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET1 PROTREG51: Protection enable for region 51."]
    #[inline]
    pub(crate) fn read_mpu_protenset1_protreg51(&self) -> MemResult<bool> {
        Ok(self.mpu[51].is_protected())
    }
    #[doc = "MPU PROTENSET1 PROTREG51: Protection enable for region 51."]
    #[inline]
    pub(crate) fn write_mpu_protenset1_protreg51(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[51].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET1 PROTREG52: Protection enable for region 52."]
    #[inline]
    pub(crate) fn read_mpu_protenset1_protreg52(&self) -> MemResult<bool> {
        Ok(self.mpu[52].is_protected())
    }
    #[doc = "MPU PROTENSET1 PROTREG52: Protection enable for region 52."]
    #[inline]
    pub(crate) fn write_mpu_protenset1_protreg52(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[52].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET1 PROTREG53: Protection enable for region 53."]
    #[inline]
    pub(crate) fn read_mpu_protenset1_protreg53(&self) -> MemResult<bool> {
        Ok(self.mpu[53].is_protected())
    }
    #[doc = "MPU PROTENSET1 PROTREG53: Protection enable for region 53."]
    #[inline]
    pub(crate) fn write_mpu_protenset1_protreg53(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[53].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET1 PROTREG54: Protection enable for region 54."]
    #[inline]
    pub(crate) fn read_mpu_protenset1_protreg54(&self) -> MemResult<bool> {
        Ok(self.mpu[54].is_protected())
    }
    #[doc = "MPU PROTENSET1 PROTREG54: Protection enable for region 54."]
    #[inline]
    pub(crate) fn write_mpu_protenset1_protreg54(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[54].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET1 PROTREG55: Protection enable for region 55."]
    #[inline]
    pub(crate) fn read_mpu_protenset1_protreg55(&self) -> MemResult<bool> {
        Ok(self.mpu[55].is_protected())
    }
    #[doc = "MPU PROTENSET1 PROTREG55: Protection enable for region 55."]
    #[inline]
    pub(crate) fn write_mpu_protenset1_protreg55(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[55].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET1 PROTREG56: Protection enable for region 56."]
    #[inline]
    pub(crate) fn read_mpu_protenset1_protreg56(&self) -> MemResult<bool> {
        Ok(self.mpu[56].is_protected())
    }
    #[doc = "MPU PROTENSET1 PROTREG56: Protection enable for region 56."]
    #[inline]
    pub(crate) fn write_mpu_protenset1_protreg56(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[56].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET1 PROTREG57: Protection enable for region 57."]
    #[inline]
    pub(crate) fn read_mpu_protenset1_protreg57(&self) -> MemResult<bool> {
        Ok(self.mpu[57].is_protected())
    }
    #[doc = "MPU PROTENSET1 PROTREG57: Protection enable for region 57."]
    #[inline]
    pub(crate) fn write_mpu_protenset1_protreg57(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[57].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET1 PROTREG58: Protection enable for region 58."]
    #[inline]
    pub(crate) fn read_mpu_protenset1_protreg58(&self) -> MemResult<bool> {
        Ok(self.mpu[58].is_protected())
    }
    #[doc = "MPU PROTENSET1 PROTREG58: Protection enable for region 58."]
    #[inline]
    pub(crate) fn write_mpu_protenset1_protreg58(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[58].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET1 PROTREG59: Protection enable for region 59."]
    #[inline]
    pub(crate) fn read_mpu_protenset1_protreg59(&self) -> MemResult<bool> {
        Ok(self.mpu[59].is_protected())
    }
    #[doc = "MPU PROTENSET1 PROTREG59: Protection enable for region 59."]
    #[inline]
    pub(crate) fn write_mpu_protenset1_protreg59(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[59].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET1 PROTREG60: Protection enable for region 60."]
    #[inline]
    pub(crate) fn read_mpu_protenset1_protreg60(&self) -> MemResult<bool> {
        Ok(self.mpu[60].is_protected())
    }
    #[doc = "MPU PROTENSET1 PROTREG60: Protection enable for region 60."]
    #[inline]
    pub(crate) fn write_mpu_protenset1_protreg60(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[60].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET1 PROTREG61: Protection enable for region 61."]
    #[inline]
    pub(crate) fn read_mpu_protenset1_protreg61(&self) -> MemResult<bool> {
        Ok(self.mpu[61].is_protected())
    }
    #[doc = "MPU PROTENSET1 PROTREG61: Protection enable for region 61."]
    #[inline]
    pub(crate) fn write_mpu_protenset1_protreg61(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[61].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET1 PROTREG62: Protection enable for region 62."]
    #[inline]
    pub(crate) fn read_mpu_protenset1_protreg62(&self) -> MemResult<bool> {
        Ok(self.mpu[62].is_protected())
    }
    #[doc = "MPU PROTENSET1 PROTREG62: Protection enable for region 62."]
    #[inline]
    pub(crate) fn write_mpu_protenset1_protreg62(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[62].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU PROTENSET1 PROTREG63: Protection enable for region 63."]
    #[inline]
    pub(crate) fn read_mpu_protenset1_protreg63(&self) -> MemResult<bool> {
        Ok(self.mpu[63].is_protected())
    }
    #[doc = "MPU PROTENSET1 PROTREG63: Protection enable for region 63."]
    #[inline]
    pub(crate) fn write_mpu_protenset1_protreg63(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.mpu[63].set_protected(true)
        }
        Ok(())
    }
    #[doc = "MPU DISABLEINDEBUG DISABLEINDEBUG: Disable protection mechanism in debug mode."]
    #[inline]
    pub(crate) fn read_mpu_disableindebug_disableindebug(
        &self,
    ) -> MemResult<bool> {
        todo!("read MPU DISABLEINDEBUG DISABLEINDEBUG reset value true")
    }
    #[doc = "MPU DISABLEINDEBUG DISABLEINDEBUG: Disable protection mechanism in debug mode."]
    #[inline]
    pub(crate) fn write_mpu_disableindebug_disableindebug(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MPU DISABLEINDEBUG DISABLEINDEBUG reset value true")
    }
    #[doc = "MPU PROTBLOCKSIZE PROTBLOCKSIZE: Erase and write protection block size."]
    #[inline]
    pub(crate) fn read_mpu_protblocksize_protblocksize(&self) -> MemResult<u8> {
        todo!("read MPU PROTBLOCKSIZE PROTBLOCKSIZE reset value 0x00 mask 0x03")
    }
    #[doc = "MPU PROTBLOCKSIZE PROTBLOCKSIZE: Erase and write protection block size."]
    #[inline]
    pub(crate) fn write_mpu_protblocksize_protblocksize(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!(
            "write MPU PROTBLOCKSIZE PROTBLOCKSIZE reset value 0x00 mask 0x03"
        )
    }
    #[doc = "RADIO TASKS_TXEN: Enable radio in TX mode."]
    #[inline]
    pub(crate) fn write_radio_tasks_txen(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write RADIO TASKS_TXEN reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO TASKS_RXEN: Enable radio in RX mode."]
    #[inline]
    pub(crate) fn write_radio_tasks_rxen(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write RADIO TASKS_RXEN reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO TASKS_START: Start radio."]
    #[inline]
    pub(crate) fn write_radio_tasks_start(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!(
            "write RADIO TASKS_START reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "RADIO TASKS_STOP: Stop radio."]
    #[inline]
    pub(crate) fn write_radio_tasks_stop(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write RADIO TASKS_STOP reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO TASKS_DISABLE: Disable radio."]
    #[inline]
    pub(crate) fn write_radio_tasks_disable(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write RADIO TASKS_DISABLE reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO TASKS_RSSISTART: Start the RSSI and take one sample of the receive signal strength."]
    #[inline]
    pub(crate) fn write_radio_tasks_rssistart(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write RADIO TASKS_RSSISTART reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO TASKS_RSSISTOP: Stop the RSSI measurement."]
    #[inline]
    pub(crate) fn write_radio_tasks_rssistop(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write RADIO TASKS_RSSISTOP reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO TASKS_BCSTART: Start the bit counter."]
    #[inline]
    pub(crate) fn write_radio_tasks_bcstart(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write RADIO TASKS_BCSTART reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO TASKS_BCSTOP: Stop the bit counter."]
    #[inline]
    pub(crate) fn write_radio_tasks_bcstop(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!(
            "write RADIO TASKS_BCSTOP reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "RADIO EVENTS_READY: Ready event."]
    #[inline]
    pub(crate) fn read_radio_events_ready(&self) -> MemResult<u32> {
        todo!(
            "read RADIO EVENTS_READY reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "RADIO EVENTS_READY: Ready event."]
    #[inline]
    pub(crate) fn write_radio_events_ready(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!(
            "write RADIO EVENTS_READY reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "RADIO EVENTS_ADDRESS: Address event."]
    #[inline]
    pub(crate) fn read_radio_events_address(&self) -> MemResult<u32> {
        todo ! ("read RADIO EVENTS_ADDRESS reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO EVENTS_ADDRESS: Address event."]
    #[inline]
    pub(crate) fn write_radio_events_address(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write RADIO EVENTS_ADDRESS reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO EVENTS_PAYLOAD: Payload event."]
    #[inline]
    pub(crate) fn read_radio_events_payload(&self) -> MemResult<u32> {
        todo ! ("read RADIO EVENTS_PAYLOAD reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO EVENTS_PAYLOAD: Payload event."]
    #[inline]
    pub(crate) fn write_radio_events_payload(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write RADIO EVENTS_PAYLOAD reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO EVENTS_END: End event."]
    #[inline]
    pub(crate) fn read_radio_events_end(&self) -> MemResult<u32> {
        todo!("read RADIO EVENTS_END reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO EVENTS_END: End event."]
    #[inline]
    pub(crate) fn write_radio_events_end(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write RADIO EVENTS_END reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO EVENTS_DISABLED: Disable event."]
    #[inline]
    pub(crate) fn read_radio_events_disabled(&self) -> MemResult<u32> {
        todo ! ("read RADIO EVENTS_DISABLED reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO EVENTS_DISABLED: Disable event."]
    #[inline]
    pub(crate) fn write_radio_events_disabled(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write RADIO EVENTS_DISABLED reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO EVENTS_DEVMATCH: A device address match occurred on the last received packet."]
    #[inline]
    pub(crate) fn read_radio_events_devmatch(&self) -> MemResult<u32> {
        todo ! ("read RADIO EVENTS_DEVMATCH reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO EVENTS_DEVMATCH: A device address match occurred on the last received packet."]
    #[inline]
    pub(crate) fn write_radio_events_devmatch(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write RADIO EVENTS_DEVMATCH reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO EVENTS_DEVMISS: No device address match occurred on the last received packet."]
    #[inline]
    pub(crate) fn read_radio_events_devmiss(&self) -> MemResult<u32> {
        todo ! ("read RADIO EVENTS_DEVMISS reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO EVENTS_DEVMISS: No device address match occurred on the last received packet."]
    #[inline]
    pub(crate) fn write_radio_events_devmiss(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write RADIO EVENTS_DEVMISS reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO EVENTS_RSSIEND: Sampling of the receive signal strength complete. A new RSSI sample is ready for readout at the RSSISAMPLE register."]
    #[inline]
    pub(crate) fn read_radio_events_rssiend(&self) -> MemResult<u32> {
        todo ! ("read RADIO EVENTS_RSSIEND reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO EVENTS_RSSIEND: Sampling of the receive signal strength complete. A new RSSI sample is ready for readout at the RSSISAMPLE register."]
    #[inline]
    pub(crate) fn write_radio_events_rssiend(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write RADIO EVENTS_RSSIEND reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO EVENTS_BCMATCH: Bit counter reached bit count value specified in BCC register."]
    #[inline]
    pub(crate) fn read_radio_events_bcmatch(&self) -> MemResult<u32> {
        todo ! ("read RADIO EVENTS_BCMATCH reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO EVENTS_BCMATCH: Bit counter reached bit count value specified in BCC register."]
    #[inline]
    pub(crate) fn write_radio_events_bcmatch(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write RADIO EVENTS_BCMATCH reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO SHORTS READY_START: Shortcut between READY event and START task."]
    #[inline]
    pub(crate) fn read_radio_shorts_ready_start(&self) -> MemResult<bool> {
        todo!("read RADIO SHORTS READY_START reset value false")
    }
    #[doc = "RADIO SHORTS READY_START: Shortcut between READY event and START task."]
    #[inline]
    pub(crate) fn write_radio_shorts_ready_start(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RADIO SHORTS READY_START reset value false")
    }
    #[doc = "RADIO SHORTS END_DISABLE: Shortcut between END event and DISABLE task."]
    #[inline]
    pub(crate) fn read_radio_shorts_end_disable(&self) -> MemResult<bool> {
        todo!("read RADIO SHORTS END_DISABLE reset value false")
    }
    #[doc = "RADIO SHORTS END_DISABLE: Shortcut between END event and DISABLE task."]
    #[inline]
    pub(crate) fn write_radio_shorts_end_disable(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RADIO SHORTS END_DISABLE reset value false")
    }
    #[doc = "RADIO SHORTS DISABLED_TXEN: Shortcut between DISABLED event and TXEN task. "]
    #[inline]
    pub(crate) fn read_radio_shorts_disabled_txen(&self) -> MemResult<bool> {
        todo!("read RADIO SHORTS DISABLED_TXEN reset value false")
    }
    #[doc = "RADIO SHORTS DISABLED_TXEN: Shortcut between DISABLED event and TXEN task. "]
    #[inline]
    pub(crate) fn write_radio_shorts_disabled_txen(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RADIO SHORTS DISABLED_TXEN reset value false")
    }
    #[doc = "RADIO SHORTS DISABLED_RXEN: Shortcut between DISABLED event and RXEN task."]
    #[inline]
    pub(crate) fn read_radio_shorts_disabled_rxen(&self) -> MemResult<bool> {
        todo!("read RADIO SHORTS DISABLED_RXEN reset value false")
    }
    #[doc = "RADIO SHORTS DISABLED_RXEN: Shortcut between DISABLED event and RXEN task."]
    #[inline]
    pub(crate) fn write_radio_shorts_disabled_rxen(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RADIO SHORTS DISABLED_RXEN reset value false")
    }
    #[doc = "RADIO SHORTS ADDRESS_RSSISTART: Shortcut between ADDRESS event and RSSISTART task."]
    #[inline]
    pub(crate) fn read_radio_shorts_address_rssistart(
        &self,
    ) -> MemResult<bool> {
        todo!("read RADIO SHORTS ADDRESS_RSSISTART reset value false")
    }
    #[doc = "RADIO SHORTS ADDRESS_RSSISTART: Shortcut between ADDRESS event and RSSISTART task."]
    #[inline]
    pub(crate) fn write_radio_shorts_address_rssistart(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RADIO SHORTS ADDRESS_RSSISTART reset value false")
    }
    #[doc = "RADIO SHORTS END_START: Shortcut between END event and START task."]
    #[inline]
    pub(crate) fn read_radio_shorts_end_start(&self) -> MemResult<bool> {
        todo!("read RADIO SHORTS END_START reset value false")
    }
    #[doc = "RADIO SHORTS END_START: Shortcut between END event and START task."]
    #[inline]
    pub(crate) fn write_radio_shorts_end_start(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RADIO SHORTS END_START reset value false")
    }
    #[doc = "RADIO SHORTS ADDRESS_BCSTART: Shortcut between ADDRESS event and BCSTART task."]
    #[inline]
    pub(crate) fn read_radio_shorts_address_bcstart(&self) -> MemResult<bool> {
        todo!("read RADIO SHORTS ADDRESS_BCSTART reset value false")
    }
    #[doc = "RADIO SHORTS ADDRESS_BCSTART: Shortcut between ADDRESS event and BCSTART task."]
    #[inline]
    pub(crate) fn write_radio_shorts_address_bcstart(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RADIO SHORTS ADDRESS_BCSTART reset value false")
    }
    #[doc = "RADIO SHORTS DISABLED_RSSISTOP: Shortcut between DISABLED event and RSSISTOP task."]
    #[inline]
    pub(crate) fn read_radio_shorts_disabled_rssistop(
        &self,
    ) -> MemResult<bool> {
        todo!("read RADIO SHORTS DISABLED_RSSISTOP reset value false")
    }
    #[doc = "RADIO SHORTS DISABLED_RSSISTOP: Shortcut between DISABLED event and RSSISTOP task."]
    #[inline]
    pub(crate) fn write_radio_shorts_disabled_rssistop(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RADIO SHORTS DISABLED_RSSISTOP reset value false")
    }
    #[doc = "RADIO INTENSET READY: Enable interrupt on READY event."]
    #[inline]
    pub(crate) fn read_radio_intenset_ready(&self) -> MemResult<bool> {
        todo!("read RADIO INTENSET READY reset value false")
    }
    #[doc = "RADIO INTENSET READY: Enable interrupt on READY event."]
    #[inline]
    pub(crate) fn write_radio_intenset_ready(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RADIO INTENSET READY reset value false")
    }
    #[doc = "RADIO INTENSET ADDRESS: Enable interrupt on ADDRESS event."]
    #[inline]
    pub(crate) fn read_radio_intenset_address(&self) -> MemResult<bool> {
        todo!("read RADIO INTENSET ADDRESS reset value false")
    }
    #[doc = "RADIO INTENSET ADDRESS: Enable interrupt on ADDRESS event."]
    #[inline]
    pub(crate) fn write_radio_intenset_address(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RADIO INTENSET ADDRESS reset value false")
    }
    #[doc = "RADIO INTENSET PAYLOAD: Enable interrupt on PAYLOAD event."]
    #[inline]
    pub(crate) fn read_radio_intenset_payload(&self) -> MemResult<bool> {
        todo!("read RADIO INTENSET PAYLOAD reset value false")
    }
    #[doc = "RADIO INTENSET PAYLOAD: Enable interrupt on PAYLOAD event."]
    #[inline]
    pub(crate) fn write_radio_intenset_payload(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RADIO INTENSET PAYLOAD reset value false")
    }
    #[doc = "RADIO INTENSET END: Enable interrupt on END event."]
    #[inline]
    pub(crate) fn read_radio_intenset_end(&self) -> MemResult<bool> {
        todo!("read RADIO INTENSET END reset value false")
    }
    #[doc = "RADIO INTENSET END: Enable interrupt on END event."]
    #[inline]
    pub(crate) fn write_radio_intenset_end(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RADIO INTENSET END reset value false")
    }
    #[doc = "RADIO INTENSET DISABLED: Enable interrupt on DISABLED event."]
    #[inline]
    pub(crate) fn read_radio_intenset_disabled(&self) -> MemResult<bool> {
        todo!("read RADIO INTENSET DISABLED reset value false")
    }
    #[doc = "RADIO INTENSET DISABLED: Enable interrupt on DISABLED event."]
    #[inline]
    pub(crate) fn write_radio_intenset_disabled(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RADIO INTENSET DISABLED reset value false")
    }
    #[doc = "RADIO INTENSET DEVMATCH: Enable interrupt on DEVMATCH event."]
    #[inline]
    pub(crate) fn read_radio_intenset_devmatch(&self) -> MemResult<bool> {
        todo!("read RADIO INTENSET DEVMATCH reset value false")
    }
    #[doc = "RADIO INTENSET DEVMATCH: Enable interrupt on DEVMATCH event."]
    #[inline]
    pub(crate) fn write_radio_intenset_devmatch(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RADIO INTENSET DEVMATCH reset value false")
    }
    #[doc = "RADIO INTENSET DEVMISS: Enable interrupt on DEVMISS event."]
    #[inline]
    pub(crate) fn read_radio_intenset_devmiss(&self) -> MemResult<bool> {
        todo!("read RADIO INTENSET DEVMISS reset value false")
    }
    #[doc = "RADIO INTENSET DEVMISS: Enable interrupt on DEVMISS event."]
    #[inline]
    pub(crate) fn write_radio_intenset_devmiss(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RADIO INTENSET DEVMISS reset value false")
    }
    #[doc = "RADIO INTENSET RSSIEND: Enable interrupt on RSSIEND event."]
    #[inline]
    pub(crate) fn read_radio_intenset_rssiend(&self) -> MemResult<bool> {
        todo!("read RADIO INTENSET RSSIEND reset value false")
    }
    #[doc = "RADIO INTENSET RSSIEND: Enable interrupt on RSSIEND event."]
    #[inline]
    pub(crate) fn write_radio_intenset_rssiend(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RADIO INTENSET RSSIEND reset value false")
    }
    #[doc = "RADIO INTENSET BCMATCH: Enable interrupt on BCMATCH event."]
    #[inline]
    pub(crate) fn read_radio_intenset_bcmatch(&self) -> MemResult<bool> {
        todo!("read RADIO INTENSET BCMATCH reset value false")
    }
    #[doc = "RADIO INTENSET BCMATCH: Enable interrupt on BCMATCH event."]
    #[inline]
    pub(crate) fn write_radio_intenset_bcmatch(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RADIO INTENSET BCMATCH reset value false")
    }
    #[doc = "RADIO INTENCLR READY: Disable interrupt on READY event."]
    #[inline]
    pub(crate) fn read_radio_intenclr_ready(&self) -> MemResult<bool> {
        todo!("read RADIO INTENCLR READY reset value false")
    }
    #[doc = "RADIO INTENCLR READY: Disable interrupt on READY event."]
    #[inline]
    pub(crate) fn write_radio_intenclr_ready(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RADIO INTENCLR READY reset value false")
    }
    #[doc = "RADIO INTENCLR ADDRESS: Disable interrupt on ADDRESS event."]
    #[inline]
    pub(crate) fn read_radio_intenclr_address(&self) -> MemResult<bool> {
        todo!("read RADIO INTENCLR ADDRESS reset value false")
    }
    #[doc = "RADIO INTENCLR ADDRESS: Disable interrupt on ADDRESS event."]
    #[inline]
    pub(crate) fn write_radio_intenclr_address(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RADIO INTENCLR ADDRESS reset value false")
    }
    #[doc = "RADIO INTENCLR PAYLOAD: Disable interrupt on PAYLOAD event."]
    #[inline]
    pub(crate) fn read_radio_intenclr_payload(&self) -> MemResult<bool> {
        todo!("read RADIO INTENCLR PAYLOAD reset value false")
    }
    #[doc = "RADIO INTENCLR PAYLOAD: Disable interrupt on PAYLOAD event."]
    #[inline]
    pub(crate) fn write_radio_intenclr_payload(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RADIO INTENCLR PAYLOAD reset value false")
    }
    #[doc = "RADIO INTENCLR END: Disable interrupt on END event."]
    #[inline]
    pub(crate) fn read_radio_intenclr_end(&self) -> MemResult<bool> {
        todo!("read RADIO INTENCLR END reset value false")
    }
    #[doc = "RADIO INTENCLR END: Disable interrupt on END event."]
    #[inline]
    pub(crate) fn write_radio_intenclr_end(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RADIO INTENCLR END reset value false")
    }
    #[doc = "RADIO INTENCLR DISABLED: Disable interrupt on DISABLED event."]
    #[inline]
    pub(crate) fn read_radio_intenclr_disabled(&self) -> MemResult<bool> {
        todo!("read RADIO INTENCLR DISABLED reset value false")
    }
    #[doc = "RADIO INTENCLR DISABLED: Disable interrupt on DISABLED event."]
    #[inline]
    pub(crate) fn write_radio_intenclr_disabled(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RADIO INTENCLR DISABLED reset value false")
    }
    #[doc = "RADIO INTENCLR DEVMATCH: Disable interrupt on DEVMATCH event."]
    #[inline]
    pub(crate) fn read_radio_intenclr_devmatch(&self) -> MemResult<bool> {
        todo!("read RADIO INTENCLR DEVMATCH reset value false")
    }
    #[doc = "RADIO INTENCLR DEVMATCH: Disable interrupt on DEVMATCH event."]
    #[inline]
    pub(crate) fn write_radio_intenclr_devmatch(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RADIO INTENCLR DEVMATCH reset value false")
    }
    #[doc = "RADIO INTENCLR DEVMISS: Disable interrupt on DEVMISS event."]
    #[inline]
    pub(crate) fn read_radio_intenclr_devmiss(&self) -> MemResult<bool> {
        todo!("read RADIO INTENCLR DEVMISS reset value false")
    }
    #[doc = "RADIO INTENCLR DEVMISS: Disable interrupt on DEVMISS event."]
    #[inline]
    pub(crate) fn write_radio_intenclr_devmiss(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RADIO INTENCLR DEVMISS reset value false")
    }
    #[doc = "RADIO INTENCLR RSSIEND: Disable interrupt on RSSIEND event."]
    #[inline]
    pub(crate) fn read_radio_intenclr_rssiend(&self) -> MemResult<bool> {
        todo!("read RADIO INTENCLR RSSIEND reset value false")
    }
    #[doc = "RADIO INTENCLR RSSIEND: Disable interrupt on RSSIEND event."]
    #[inline]
    pub(crate) fn write_radio_intenclr_rssiend(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RADIO INTENCLR RSSIEND reset value false")
    }
    #[doc = "RADIO INTENCLR BCMATCH: Disable interrupt on BCMATCH event."]
    #[inline]
    pub(crate) fn read_radio_intenclr_bcmatch(&self) -> MemResult<bool> {
        todo!("read RADIO INTENCLR BCMATCH reset value false")
    }
    #[doc = "RADIO INTENCLR BCMATCH: Disable interrupt on BCMATCH event."]
    #[inline]
    pub(crate) fn write_radio_intenclr_bcmatch(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RADIO INTENCLR BCMATCH reset value false")
    }
    #[doc = "RADIO CRCSTATUS CRCSTATUS: CRC status of received packet."]
    #[inline]
    pub(crate) fn read_radio_crcstatus_crcstatus(&self) -> MemResult<bool> {
        todo!("read RADIO CRCSTATUS CRCSTATUS reset value false")
    }
    #[doc = "RADIO RXMATCH RXMATCH: Logical address in which previous packet was received."]
    #[inline]
    pub(crate) fn read_radio_rxmatch_rxmatch(&self) -> MemResult<u8> {
        todo!("read RADIO RXMATCH RXMATCH reset value 0x00 mask 0x07")
    }
    #[doc = "RADIO RXCRC RXCRC: CRC field of previously received packet."]
    #[inline]
    pub(crate) fn read_radio_rxcrc_rxcrc(&self) -> MemResult<u32> {
        todo!("read RADIO RXCRC RXCRC reset value 0x00 mask 0xffffff")
    }
    #[doc = "RADIO DAI DAI: Index (n) of device address (see DAB[n] and DAP[n]) that obtained an address match."]
    #[inline]
    pub(crate) fn read_radio_dai_dai(&self) -> MemResult<u8> {
        todo!("read RADIO DAI DAI reset value 0x00 mask 0x07")
    }
    #[doc = "RADIO PACKETPTR: Packet pointer. Decision point: START task."]
    #[inline]
    pub(crate) fn read_radio_packetptr(&self) -> MemResult<u32> {
        todo!("read RADIO PACKETPTR reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO PACKETPTR: Packet pointer. Decision point: START task."]
    #[inline]
    pub(crate) fn write_radio_packetptr(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write RADIO PACKETPTR reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO FREQUENCY FREQUENCY: Radio channel frequency offset in MHz: RF Frequency = 2400 + FREQUENCY (MHz). Decision point: TXEN or RXEN task. "]
    #[inline]
    pub(crate) fn read_radio_frequency_frequency(&self) -> MemResult<u8> {
        todo!("read RADIO FREQUENCY FREQUENCY reset value 0x02 mask 0x7f")
    }
    #[doc = "RADIO FREQUENCY FREQUENCY: Radio channel frequency offset in MHz: RF Frequency = 2400 + FREQUENCY (MHz). Decision point: TXEN or RXEN task. "]
    #[inline]
    pub(crate) fn write_radio_frequency_frequency(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write RADIO FREQUENCY FREQUENCY reset value 0x02 mask 0x7f")
    }
    #[doc = "RADIO TXPOWER TXPOWER: Radio output power. Decision point: TXEN task."]
    #[inline]
    pub(crate) fn read_radio_txpower_txpower(&self) -> MemResult<u8> {
        todo!("read RADIO TXPOWER TXPOWER reset value 0x00 mask 0xff")
    }
    #[doc = "RADIO TXPOWER TXPOWER: Radio output power. Decision point: TXEN task."]
    #[inline]
    pub(crate) fn write_radio_txpower_txpower(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write RADIO TXPOWER TXPOWER reset value 0x00 mask 0xff")
    }
    #[doc = "RADIO MODE MODE: Radio data rate and modulation setting. Decision point: TXEN or RXEN task."]
    #[inline]
    pub(crate) fn read_radio_mode_mode(&self) -> MemResult<u8> {
        todo!("read RADIO MODE MODE reset value 0x00 mask 0x03")
    }
    #[doc = "RADIO MODE MODE: Radio data rate and modulation setting. Decision point: TXEN or RXEN task."]
    #[inline]
    pub(crate) fn write_radio_mode_mode(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write RADIO MODE MODE reset value 0x00 mask 0x03")
    }
    #[doc = "RADIO PCNF0 LFLEN: Length of length field in number of bits. Decision point: START task."]
    #[inline]
    pub(crate) fn read_radio_pcnf0_lflen(&self) -> MemResult<u8> {
        todo!("read RADIO PCNF0 LFLEN reset value 0x00 mask 0x0f")
    }
    #[doc = "RADIO PCNF0 LFLEN: Length of length field in number of bits. Decision point: START task."]
    #[inline]
    pub(crate) fn write_radio_pcnf0_lflen(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write RADIO PCNF0 LFLEN reset value 0x00 mask 0x0f")
    }
    #[doc = "RADIO PCNF0 S0LEN: Length of S0 field in number of bytes. Decision point: START task."]
    #[inline]
    pub(crate) fn read_radio_pcnf0_s0len(&self) -> MemResult<bool> {
        todo!("read RADIO PCNF0 S0LEN reset value false")
    }
    #[doc = "RADIO PCNF0 S0LEN: Length of S0 field in number of bytes. Decision point: START task."]
    #[inline]
    pub(crate) fn write_radio_pcnf0_s0len(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RADIO PCNF0 S0LEN reset value false")
    }
    #[doc = "RADIO PCNF0 S1LEN: Length of S1 field in number of bits. Decision point: START task."]
    #[inline]
    pub(crate) fn read_radio_pcnf0_s1len(&self) -> MemResult<u8> {
        todo!("read RADIO PCNF0 S1LEN reset value 0x00 mask 0x0f")
    }
    #[doc = "RADIO PCNF0 S1LEN: Length of S1 field in number of bits. Decision point: START task."]
    #[inline]
    pub(crate) fn write_radio_pcnf0_s1len(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write RADIO PCNF0 S1LEN reset value 0x00 mask 0x0f")
    }
    #[doc = "RADIO PCNF1 MAXLEN: Maximum length of packet payload in number of bytes."]
    #[inline]
    pub(crate) fn read_radio_pcnf1_maxlen(&self) -> MemResult<u8> {
        todo!("read RADIO PCNF1 MAXLEN reset value 0x00 mask 0xff")
    }
    #[doc = "RADIO PCNF1 MAXLEN: Maximum length of packet payload in number of bytes."]
    #[inline]
    pub(crate) fn write_radio_pcnf1_maxlen(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write RADIO PCNF1 MAXLEN reset value 0x00 mask 0xff")
    }
    #[doc = "RADIO PCNF1 STATLEN: Static length in number of bytes. Decision point: START task."]
    #[inline]
    pub(crate) fn read_radio_pcnf1_statlen(&self) -> MemResult<u8> {
        todo!("read RADIO PCNF1 STATLEN reset value 0x00 mask 0xff")
    }
    #[doc = "RADIO PCNF1 STATLEN: Static length in number of bytes. Decision point: START task."]
    #[inline]
    pub(crate) fn write_radio_pcnf1_statlen(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write RADIO PCNF1 STATLEN reset value 0x00 mask 0xff")
    }
    #[doc = "RADIO PCNF1 BALEN: Base address length in number of bytes. Decision point: START task."]
    #[inline]
    pub(crate) fn read_radio_pcnf1_balen(&self) -> MemResult<u8> {
        todo!("read RADIO PCNF1 BALEN reset value 0x00 mask 0x07")
    }
    #[doc = "RADIO PCNF1 BALEN: Base address length in number of bytes. Decision point: START task."]
    #[inline]
    pub(crate) fn write_radio_pcnf1_balen(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write RADIO PCNF1 BALEN reset value 0x00 mask 0x07")
    }
    #[doc = "RADIO PCNF1 ENDIAN: On air endianness of packet length field. Decision point: START task."]
    #[inline]
    pub(crate) fn read_radio_pcnf1_endian(&self) -> MemResult<bool> {
        todo!("read RADIO PCNF1 ENDIAN reset value false")
    }
    #[doc = "RADIO PCNF1 ENDIAN: On air endianness of packet length field. Decision point: START task."]
    #[inline]
    pub(crate) fn write_radio_pcnf1_endian(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RADIO PCNF1 ENDIAN reset value false")
    }
    #[doc = "RADIO PCNF1 WHITEEN: Packet whitening enable."]
    #[inline]
    pub(crate) fn read_radio_pcnf1_whiteen(&self) -> MemResult<bool> {
        todo!("read RADIO PCNF1 WHITEEN reset value false")
    }
    #[doc = "RADIO PCNF1 WHITEEN: Packet whitening enable."]
    #[inline]
    pub(crate) fn write_radio_pcnf1_whiteen(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RADIO PCNF1 WHITEEN reset value false")
    }
    #[doc = "RADIO BASE0: Radio base address 0. Decision point: START task."]
    #[inline]
    pub(crate) fn read_radio_base0(&self) -> MemResult<u32> {
        todo!("read RADIO BASE0 reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO BASE0: Radio base address 0. Decision point: START task."]
    #[inline]
    pub(crate) fn write_radio_base0(&mut self, _value: u32) -> MemResult<()> {
        todo!("write RADIO BASE0 reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO BASE1: Radio base address 1. Decision point: START task."]
    #[inline]
    pub(crate) fn read_radio_base1(&self) -> MemResult<u32> {
        todo!("read RADIO BASE1 reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO BASE1: Radio base address 1. Decision point: START task."]
    #[inline]
    pub(crate) fn write_radio_base1(&mut self, _value: u32) -> MemResult<()> {
        todo!("write RADIO BASE1 reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO PREFIX0 AP0: Address prefix 0. Decision point: START task."]
    #[inline]
    pub(crate) fn read_radio_prefix0_ap0(&self) -> MemResult<u8> {
        Ok(self.radio.address_prefix(0))
    }
    #[doc = "RADIO PREFIX0 AP0: Address prefix 0. Decision point: START task."]
    #[inline]
    pub(crate) fn write_radio_prefix0_ap0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.radio.set_address_prefix(0, _value))
    }
    #[doc = "RADIO PREFIX0 AP1: Address prefix 1. Decision point: START task."]
    #[inline]
    pub(crate) fn read_radio_prefix0_ap1(&self) -> MemResult<u8> {
        Ok(self.radio.address_prefix(1))
    }
    #[doc = "RADIO PREFIX0 AP1: Address prefix 1. Decision point: START task."]
    #[inline]
    pub(crate) fn write_radio_prefix0_ap1(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.radio.set_address_prefix(1, _value))
    }
    #[doc = "RADIO PREFIX0 AP2: Address prefix 2. Decision point: START task."]
    #[inline]
    pub(crate) fn read_radio_prefix0_ap2(&self) -> MemResult<u8> {
        Ok(self.radio.address_prefix(2))
    }
    #[doc = "RADIO PREFIX0 AP2: Address prefix 2. Decision point: START task."]
    #[inline]
    pub(crate) fn write_radio_prefix0_ap2(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.radio.set_address_prefix(2, _value))
    }
    #[doc = "RADIO PREFIX0 AP3: Address prefix 3. Decision point: START task."]
    #[inline]
    pub(crate) fn read_radio_prefix0_ap3(&self) -> MemResult<u8> {
        Ok(self.radio.address_prefix(3))
    }
    #[doc = "RADIO PREFIX0 AP3: Address prefix 3. Decision point: START task."]
    #[inline]
    pub(crate) fn write_radio_prefix0_ap3(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.radio.set_address_prefix(3, _value))
    }
    #[doc = "RADIO PREFIX1 AP4: Address prefix 4. Decision point: START task."]
    #[inline]
    pub(crate) fn read_radio_prefix1_ap4(&self) -> MemResult<u8> {
        Ok(self.radio.address_prefix(4))
    }
    #[doc = "RADIO PREFIX1 AP4: Address prefix 4. Decision point: START task."]
    #[inline]
    pub(crate) fn write_radio_prefix1_ap4(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.radio.set_address_prefix(4, _value))
    }
    #[doc = "RADIO PREFIX1 AP5: Address prefix 5. Decision point: START task."]
    #[inline]
    pub(crate) fn read_radio_prefix1_ap5(&self) -> MemResult<u8> {
        Ok(self.radio.address_prefix(5))
    }
    #[doc = "RADIO PREFIX1 AP5: Address prefix 5. Decision point: START task."]
    #[inline]
    pub(crate) fn write_radio_prefix1_ap5(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.radio.set_address_prefix(5, _value))
    }
    #[doc = "RADIO PREFIX1 AP6: Address prefix 6. Decision point: START task."]
    #[inline]
    pub(crate) fn read_radio_prefix1_ap6(&self) -> MemResult<u8> {
        Ok(self.radio.address_prefix(6))
    }
    #[doc = "RADIO PREFIX1 AP6: Address prefix 6. Decision point: START task."]
    #[inline]
    pub(crate) fn write_radio_prefix1_ap6(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.radio.set_address_prefix(6, _value))
    }
    #[doc = "RADIO PREFIX1 AP7: Address prefix 7. Decision point: START task."]
    #[inline]
    pub(crate) fn read_radio_prefix1_ap7(&self) -> MemResult<u8> {
        Ok(self.radio.address_prefix(7))
    }
    #[doc = "RADIO PREFIX1 AP7: Address prefix 7. Decision point: START task."]
    #[inline]
    pub(crate) fn write_radio_prefix1_ap7(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.radio.set_address_prefix(7, _value))
    }
    #[doc = "RADIO TXADDRESS TXADDRESS: Logical address to be used when transmitting a packet. Decision point: START task."]
    #[inline]
    pub(crate) fn read_radio_txaddress_txaddress(&self) -> MemResult<u8> {
        todo!("read RADIO TXADDRESS TXADDRESS reset value 0x00 mask 0x07")
    }
    #[doc = "RADIO TXADDRESS TXADDRESS: Logical address to be used when transmitting a packet. Decision point: START task."]
    #[inline]
    pub(crate) fn write_radio_txaddress_txaddress(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write RADIO TXADDRESS TXADDRESS reset value 0x00 mask 0x07")
    }
    #[doc = "RADIO RXADDRESSES ADDR0: Enable reception on logical address 0. Decision point: START task."]
    #[inline]
    pub(crate) fn read_radio_rxaddresses_addr0(&self) -> MemResult<bool> {
        Ok(self.radio.receive_on_ap(0))
    }
    #[doc = "RADIO RXADDRESSES ADDR0: Enable reception on logical address 0. Decision point: START task."]
    #[inline]
    pub(crate) fn write_radio_rxaddresses_addr0(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.radio.set_receive_on_ap(0, _value))
    }
    #[doc = "RADIO RXADDRESSES ADDR1: Enable reception on logical address 1. Decision point: START task."]
    #[inline]
    pub(crate) fn read_radio_rxaddresses_addr1(&self) -> MemResult<bool> {
        Ok(self.radio.receive_on_ap(1))
    }
    #[doc = "RADIO RXADDRESSES ADDR1: Enable reception on logical address 1. Decision point: START task."]
    #[inline]
    pub(crate) fn write_radio_rxaddresses_addr1(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.radio.set_receive_on_ap(1, _value))
    }
    #[doc = "RADIO RXADDRESSES ADDR2: Enable reception on logical address 2. Decision point: START task."]
    #[inline]
    pub(crate) fn read_radio_rxaddresses_addr2(&self) -> MemResult<bool> {
        Ok(self.radio.receive_on_ap(2))
    }
    #[doc = "RADIO RXADDRESSES ADDR2: Enable reception on logical address 2. Decision point: START task."]
    #[inline]
    pub(crate) fn write_radio_rxaddresses_addr2(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.radio.set_receive_on_ap(2, _value))
    }
    #[doc = "RADIO RXADDRESSES ADDR3: Enable reception on logical address 3. Decision point: START task."]
    #[inline]
    pub(crate) fn read_radio_rxaddresses_addr3(&self) -> MemResult<bool> {
        Ok(self.radio.receive_on_ap(3))
    }
    #[doc = "RADIO RXADDRESSES ADDR3: Enable reception on logical address 3. Decision point: START task."]
    #[inline]
    pub(crate) fn write_radio_rxaddresses_addr3(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.radio.set_receive_on_ap(3, _value))
    }
    #[doc = "RADIO RXADDRESSES ADDR4: Enable reception on logical address 4. Decision point: START task."]
    #[inline]
    pub(crate) fn read_radio_rxaddresses_addr4(&self) -> MemResult<bool> {
        Ok(self.radio.receive_on_ap(4))
    }
    #[doc = "RADIO RXADDRESSES ADDR4: Enable reception on logical address 4. Decision point: START task."]
    #[inline]
    pub(crate) fn write_radio_rxaddresses_addr4(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.radio.set_receive_on_ap(4, _value))
    }
    #[doc = "RADIO RXADDRESSES ADDR5: Enable reception on logical address 5. Decision point: START task."]
    #[inline]
    pub(crate) fn read_radio_rxaddresses_addr5(&self) -> MemResult<bool> {
        Ok(self.radio.receive_on_ap(5))
    }
    #[doc = "RADIO RXADDRESSES ADDR5: Enable reception on logical address 5. Decision point: START task."]
    #[inline]
    pub(crate) fn write_radio_rxaddresses_addr5(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.radio.set_receive_on_ap(5, _value))
    }
    #[doc = "RADIO RXADDRESSES ADDR6: Enable reception on logical address 6. Decision point: START task."]
    #[inline]
    pub(crate) fn read_radio_rxaddresses_addr6(&self) -> MemResult<bool> {
        Ok(self.radio.receive_on_ap(6))
    }
    #[doc = "RADIO RXADDRESSES ADDR6: Enable reception on logical address 6. Decision point: START task."]
    #[inline]
    pub(crate) fn write_radio_rxaddresses_addr6(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.radio.set_receive_on_ap(6, _value))
    }
    #[doc = "RADIO RXADDRESSES ADDR7: Enable reception on logical address 7. Decision point: START task."]
    #[inline]
    pub(crate) fn read_radio_rxaddresses_addr7(&self) -> MemResult<bool> {
        Ok(self.radio.receive_on_ap(7))
    }
    #[doc = "RADIO RXADDRESSES ADDR7: Enable reception on logical address 7. Decision point: START task."]
    #[inline]
    pub(crate) fn write_radio_rxaddresses_addr7(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.radio.set_receive_on_ap(7, _value))
    }
    #[doc = "RADIO CRCCNF LEN: CRC length. Decision point: START task."]
    #[inline]
    pub(crate) fn read_radio_crccnf_len(&self) -> MemResult<u8> {
        todo!("read RADIO CRCCNF LEN reset value 0x00 mask 0x03")
    }
    #[doc = "RADIO CRCCNF LEN: CRC length. Decision point: START task."]
    #[inline]
    pub(crate) fn write_radio_crccnf_len(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write RADIO CRCCNF LEN reset value 0x00 mask 0x03")
    }
    #[doc = "RADIO CRCCNF SKIPADDR: Leave packet address field out of the CRC calculation. Decision point: START task."]
    #[inline]
    pub(crate) fn read_radio_crccnf_skipaddr(&self) -> MemResult<bool> {
        todo!("read RADIO CRCCNF SKIPADDR reset value false")
    }
    #[doc = "RADIO CRCCNF SKIPADDR: Leave packet address field out of the CRC calculation. Decision point: START task."]
    #[inline]
    pub(crate) fn write_radio_crccnf_skipaddr(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RADIO CRCCNF SKIPADDR reset value false")
    }
    #[doc = "RADIO CRCPOLY CRCPOLY: CRC polynomial. Decision point: START task."]
    #[inline]
    pub(crate) fn read_radio_crcpoly_crcpoly(&self) -> MemResult<u32> {
        todo!("read RADIO CRCPOLY CRCPOLY reset value 0x00 mask 0xffffff")
    }
    #[doc = "RADIO CRCPOLY CRCPOLY: CRC polynomial. Decision point: START task."]
    #[inline]
    pub(crate) fn write_radio_crcpoly_crcpoly(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write RADIO CRCPOLY CRCPOLY reset value 0x00 mask 0xffffff")
    }
    #[doc = "RADIO CRCINIT CRCINIT: Initial value for CRC calculation. Decision point: START task."]
    #[inline]
    pub(crate) fn read_radio_crcinit_crcinit(&self) -> MemResult<u32> {
        todo!("read RADIO CRCINIT CRCINIT reset value 0x00 mask 0xffffff")
    }
    #[doc = "RADIO CRCINIT CRCINIT: Initial value for CRC calculation. Decision point: START task."]
    #[inline]
    pub(crate) fn write_radio_crcinit_crcinit(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write RADIO CRCINIT CRCINIT reset value 0x00 mask 0xffffff")
    }
    #[doc = "RADIO TEST CONSTCARRIER: Constant carrier. Decision point: TXEN task."]
    #[inline]
    pub(crate) fn read_radio_test_constcarrier(&self) -> MemResult<bool> {
        todo!("read RADIO TEST CONSTCARRIER reset value false")
    }
    #[doc = "RADIO TEST CONSTCARRIER: Constant carrier. Decision point: TXEN task."]
    #[inline]
    pub(crate) fn write_radio_test_constcarrier(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RADIO TEST CONSTCARRIER reset value false")
    }
    #[doc = "RADIO TEST PLLLOCK: PLL lock. Decision point: TXEN or RXEN task."]
    #[inline]
    pub(crate) fn read_radio_test_plllock(&self) -> MemResult<bool> {
        todo!("read RADIO TEST PLLLOCK reset value false")
    }
    #[doc = "RADIO TEST PLLLOCK: PLL lock. Decision point: TXEN or RXEN task."]
    #[inline]
    pub(crate) fn write_radio_test_plllock(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RADIO TEST PLLLOCK reset value false")
    }
    #[doc = "RADIO TIFS TIFS: Inter frame spacing in microseconds. Decision point: START rask"]
    #[inline]
    pub(crate) fn read_radio_tifs_tifs(&self) -> MemResult<u8> {
        todo!("read RADIO TIFS TIFS reset value 0x00 mask 0xff")
    }
    #[doc = "RADIO TIFS TIFS: Inter frame spacing in microseconds. Decision point: START rask"]
    #[inline]
    pub(crate) fn write_radio_tifs_tifs(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write RADIO TIFS TIFS reset value 0x00 mask 0xff")
    }
    #[doc = "RADIO RSSISAMPLE RSSISAMPLE: RSSI sample result. The result is read as a positive value so that ReceivedSignalStrength = -RSSISAMPLE dBm"]
    #[inline]
    pub(crate) fn read_radio_rssisample_rssisample(&self) -> MemResult<u8> {
        todo!("read RADIO RSSISAMPLE RSSISAMPLE reset value 0x00 mask 0x7f")
    }
    #[doc = "RADIO STATE STATE: Current radio state."]
    #[inline]
    pub(crate) fn read_radio_state_state(&self) -> MemResult<u8> {
        todo!("read RADIO STATE STATE reset value 0x00 mask 0x0f")
    }
    #[doc = "RADIO DATAWHITEIV DATAWHITEIV: Data whitening initial value. Bit 0 corresponds to Position 0 of the LSFR, Bit 1 to position 5... Decision point: TXEN or RXEN task."]
    #[inline]
    pub(crate) fn read_radio_datawhiteiv_datawhiteiv(&self) -> MemResult<u8> {
        todo!("read RADIO DATAWHITEIV DATAWHITEIV reset value 0x40 mask 0x7f")
    }
    #[doc = "RADIO DATAWHITEIV DATAWHITEIV: Data whitening initial value. Bit 0 corresponds to Position 0 of the LSFR, Bit 1 to position 5... Decision point: TXEN or RXEN task."]
    #[inline]
    pub(crate) fn write_radio_datawhiteiv_datawhiteiv(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write RADIO DATAWHITEIV DATAWHITEIV reset value 0x40 mask 0x7f")
    }
    #[doc = "RADIO BCC: Bit counter compare."]
    #[inline]
    pub(crate) fn read_radio_bcc(&self) -> MemResult<u32> {
        todo!("read RADIO BCC reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO BCC: Bit counter compare."]
    #[inline]
    pub(crate) fn write_radio_bcc(&mut self, _value: u32) -> MemResult<()> {
        todo!("write RADIO BCC reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO DAB[%s]: Device address base segment."]
    #[inline]
    pub(crate) fn read_radio_dabn(&self, _dim: usize) -> MemResult<u32> {
        todo!("read RADIO DAB[%s] reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO DAB[%s]: Device address base segment."]
    #[inline]
    pub(crate) fn write_radio_dabn(
        &mut self,
        _dim: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write RADIO DAB[%s] reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO DAP[%s] DAP: Device address prefix."]
    #[inline]
    pub(crate) fn read_radio_dapn_dap(&self, _dim: usize) -> MemResult<u16> {
        todo!("read RADIO DAP[%s] DAP reset value 0x00 mask 0xffff")
    }
    #[doc = "RADIO DAP[%s] DAP: Device address prefix."]
    #[inline]
    pub(crate) fn write_radio_dapn_dap(
        &mut self,
        _dim: usize,
        _value: u16,
    ) -> MemResult<()> {
        todo!("write RADIO DAP[%s] DAP reset value 0x00 mask 0xffff")
    }
    #[doc = "RADIO DACNF ENA0: Enable or disable device address matching using device address 0."]
    #[inline]
    pub(crate) fn read_radio_dacnf_ena0(&self) -> MemResult<bool> {
        Ok(self.radio.address_prefix_on(0))
    }
    #[doc = "RADIO DACNF ENA0: Enable or disable device address matching using device address 0."]
    #[inline]
    pub(crate) fn write_radio_dacnf_ena0(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.radio.set_address_prefix_on(0, _value))
    }
    #[doc = "RADIO DACNF ENA1: Enable or disable device address matching using device address 1."]
    #[inline]
    pub(crate) fn read_radio_dacnf_ena1(&self) -> MemResult<bool> {
        Ok(self.radio.address_prefix_on(1))
    }
    #[doc = "RADIO DACNF ENA1: Enable or disable device address matching using device address 1."]
    #[inline]
    pub(crate) fn write_radio_dacnf_ena1(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.radio.set_address_prefix_on(1, _value))
    }
    #[doc = "RADIO DACNF ENA2: Enable or disable device address matching using device address 2."]
    #[inline]
    pub(crate) fn read_radio_dacnf_ena2(&self) -> MemResult<bool> {
        Ok(self.radio.address_prefix_on(2))
    }
    #[doc = "RADIO DACNF ENA2: Enable or disable device address matching using device address 2."]
    #[inline]
    pub(crate) fn write_radio_dacnf_ena2(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.radio.set_address_prefix_on(2, _value))
    }
    #[doc = "RADIO DACNF ENA3: Enable or disable device address matching using device address 3."]
    #[inline]
    pub(crate) fn read_radio_dacnf_ena3(&self) -> MemResult<bool> {
        Ok(self.radio.address_prefix_on(3))
    }
    #[doc = "RADIO DACNF ENA3: Enable or disable device address matching using device address 3."]
    #[inline]
    pub(crate) fn write_radio_dacnf_ena3(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.radio.set_address_prefix_on(3, _value))
    }
    #[doc = "RADIO DACNF ENA4: Enable or disable device address matching using device address 4."]
    #[inline]
    pub(crate) fn read_radio_dacnf_ena4(&self) -> MemResult<bool> {
        Ok(self.radio.address_prefix_on(4))
    }
    #[doc = "RADIO DACNF ENA4: Enable or disable device address matching using device address 4."]
    #[inline]
    pub(crate) fn write_radio_dacnf_ena4(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.radio.set_address_prefix_on(4, _value))
    }
    #[doc = "RADIO DACNF ENA5: Enable or disable device address matching using device address 5."]
    #[inline]
    pub(crate) fn read_radio_dacnf_ena5(&self) -> MemResult<bool> {
        Ok(self.radio.address_prefix_on(5))
    }
    #[doc = "RADIO DACNF ENA5: Enable or disable device address matching using device address 5."]
    #[inline]
    pub(crate) fn write_radio_dacnf_ena5(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.radio.set_address_prefix_on(5, _value))
    }
    #[doc = "RADIO DACNF ENA6: Enable or disable device address matching using device address 6."]
    #[inline]
    pub(crate) fn read_radio_dacnf_ena6(&self) -> MemResult<bool> {
        Ok(self.radio.address_prefix_on(6))
    }
    #[doc = "RADIO DACNF ENA6: Enable or disable device address matching using device address 6."]
    #[inline]
    pub(crate) fn write_radio_dacnf_ena6(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.radio.set_address_prefix_on(6, _value))
    }
    #[doc = "RADIO DACNF ENA7: Enable or disable device address matching using device address 7."]
    #[inline]
    pub(crate) fn read_radio_dacnf_ena7(&self) -> MemResult<bool> {
        Ok(self.radio.address_prefix_on(7))
    }
    #[doc = "RADIO DACNF ENA7: Enable or disable device address matching using device address 7."]
    #[inline]
    pub(crate) fn write_radio_dacnf_ena7(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.radio.set_address_prefix_on(7, _value))
    }
    #[doc = "RADIO DACNF TXADD0: TxAdd for device address 0."]
    #[inline]
    pub(crate) fn read_radio_dacnf_txadd0(&self) -> MemResult<bool> {
        Ok(self.radio.tx_on_ap(0))
    }
    #[doc = "RADIO DACNF TXADD0: TxAdd for device address 0."]
    #[inline]
    pub(crate) fn write_radio_dacnf_txadd0(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.radio.set_tx_on_ap(0, _value))
    }
    #[doc = "RADIO DACNF TXADD1: TxAdd for device address 1."]
    #[inline]
    pub(crate) fn read_radio_dacnf_txadd1(&self) -> MemResult<bool> {
        Ok(self.radio.tx_on_ap(1))
    }
    #[doc = "RADIO DACNF TXADD1: TxAdd for device address 1."]
    #[inline]
    pub(crate) fn write_radio_dacnf_txadd1(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.radio.set_tx_on_ap(1, _value))
    }
    #[doc = "RADIO DACNF TXADD2: TxAdd for device address 2."]
    #[inline]
    pub(crate) fn read_radio_dacnf_txadd2(&self) -> MemResult<bool> {
        Ok(self.radio.tx_on_ap(2))
    }
    #[doc = "RADIO DACNF TXADD2: TxAdd for device address 2."]
    #[inline]
    pub(crate) fn write_radio_dacnf_txadd2(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.radio.set_tx_on_ap(2, _value))
    }
    #[doc = "RADIO DACNF TXADD3: TxAdd for device address 3."]
    #[inline]
    pub(crate) fn read_radio_dacnf_txadd3(&self) -> MemResult<bool> {
        Ok(self.radio.tx_on_ap(3))
    }
    #[doc = "RADIO DACNF TXADD3: TxAdd for device address 3."]
    #[inline]
    pub(crate) fn write_radio_dacnf_txadd3(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.radio.set_tx_on_ap(3, _value))
    }
    #[doc = "RADIO DACNF TXADD4: TxAdd for device address 4."]
    #[inline]
    pub(crate) fn read_radio_dacnf_txadd4(&self) -> MemResult<bool> {
        Ok(self.radio.tx_on_ap(4))
    }
    #[doc = "RADIO DACNF TXADD4: TxAdd for device address 4."]
    #[inline]
    pub(crate) fn write_radio_dacnf_txadd4(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.radio.set_tx_on_ap(4, _value))
    }
    #[doc = "RADIO DACNF TXADD5: TxAdd for device address 5."]
    #[inline]
    pub(crate) fn read_radio_dacnf_txadd5(&self) -> MemResult<bool> {
        Ok(self.radio.tx_on_ap(5))
    }
    #[doc = "RADIO DACNF TXADD5: TxAdd for device address 5."]
    #[inline]
    pub(crate) fn write_radio_dacnf_txadd5(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.radio.set_tx_on_ap(5, _value))
    }
    #[doc = "RADIO DACNF TXADD6: TxAdd for device address 6."]
    #[inline]
    pub(crate) fn read_radio_dacnf_txadd6(&self) -> MemResult<bool> {
        Ok(self.radio.tx_on_ap(6))
    }
    #[doc = "RADIO DACNF TXADD6: TxAdd for device address 6."]
    #[inline]
    pub(crate) fn write_radio_dacnf_txadd6(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.radio.set_tx_on_ap(6, _value))
    }
    #[doc = "RADIO DACNF TXADD7: TxAdd for device address 7."]
    #[inline]
    pub(crate) fn read_radio_dacnf_txadd7(&self) -> MemResult<bool> {
        Ok(self.radio.tx_on_ap(7))
    }
    #[doc = "RADIO DACNF TXADD7: TxAdd for device address 7."]
    #[inline]
    pub(crate) fn write_radio_dacnf_txadd7(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.radio.set_tx_on_ap(7, _value))
    }
    #[doc = "RADIO OVERRIDE0: Trim value override register 0."]
    #[inline]
    pub(crate) fn read_radio_override0(&self) -> MemResult<u32> {
        todo!("read RADIO OVERRIDE0 reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO OVERRIDE0: Trim value override register 0."]
    #[inline]
    pub(crate) fn write_radio_override0(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write RADIO OVERRIDE0 reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO OVERRIDE1: Trim value override register 1."]
    #[inline]
    pub(crate) fn read_radio_override1(&self) -> MemResult<u32> {
        todo!("read RADIO OVERRIDE1 reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO OVERRIDE1: Trim value override register 1."]
    #[inline]
    pub(crate) fn write_radio_override1(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write RADIO OVERRIDE1 reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO OVERRIDE2: Trim value override register 2."]
    #[inline]
    pub(crate) fn read_radio_override2(&self) -> MemResult<u32> {
        todo!("read RADIO OVERRIDE2 reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO OVERRIDE2: Trim value override register 2."]
    #[inline]
    pub(crate) fn write_radio_override2(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write RADIO OVERRIDE2 reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO OVERRIDE3: Trim value override register 3."]
    #[inline]
    pub(crate) fn read_radio_override3(&self) -> MemResult<u32> {
        todo!("read RADIO OVERRIDE3 reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO OVERRIDE3: Trim value override register 3."]
    #[inline]
    pub(crate) fn write_radio_override3(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write RADIO OVERRIDE3 reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RADIO OVERRIDE4 OVERRIDE4: Trim value override 4."]
    #[inline]
    pub(crate) fn read_radio_override4_override4(&self) -> MemResult<u32> {
        todo!("read RADIO OVERRIDE4 OVERRIDE4 reset value 0x00 mask 0xfffffff")
    }
    #[doc = "RADIO OVERRIDE4 OVERRIDE4: Trim value override 4."]
    #[inline]
    pub(crate) fn write_radio_override4_override4(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write RADIO OVERRIDE4 OVERRIDE4 reset value 0x00 mask 0xfffffff")
    }
    #[doc = "RADIO OVERRIDE4 ENABLE: Enable or disable override of default trim values."]
    #[inline]
    pub(crate) fn read_radio_override4_enable(&self) -> MemResult<bool> {
        todo!("read RADIO OVERRIDE4 ENABLE reset value false")
    }
    #[doc = "RADIO OVERRIDE4 ENABLE: Enable or disable override of default trim values."]
    #[inline]
    pub(crate) fn write_radio_override4_enable(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RADIO OVERRIDE4 ENABLE reset value false")
    }
    #[doc = "RADIO POWER POWER: Peripheral power control."]
    #[inline]
    pub(crate) fn read_radio_power_power(&self) -> MemResult<bool> {
        todo!("read RADIO POWER POWER reset value false")
    }
    #[doc = "RADIO POWER POWER: Peripheral power control."]
    #[inline]
    pub(crate) fn write_radio_power_power(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RADIO POWER POWER reset value false")
    }
    #[doc = "UART0 TASKS_STARTRX: Start UART receiver."]
    #[inline]
    pub(crate) fn write_uart0_tasks_startrx(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write UART0 TASKS_STARTRX reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "UART0 TASKS_STOPRX: Stop UART receiver."]
    #[inline]
    pub(crate) fn write_uart0_tasks_stoprx(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!(
            "write UART0 TASKS_STOPRX reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "UART0 TASKS_STARTTX: Start UART transmitter."]
    #[inline]
    pub(crate) fn write_uart0_tasks_starttx(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write UART0 TASKS_STARTTX reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "UART0 TASKS_STOPTX: Stop UART transmitter."]
    #[inline]
    pub(crate) fn write_uart0_tasks_stoptx(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!(
            "write UART0 TASKS_STOPTX reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "UART0 TASKS_SUSPEND: Suspend UART."]
    #[inline]
    pub(crate) fn write_uart0_tasks_suspend(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write UART0 TASKS_SUSPEND reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "UART0 EVENTS_CTS: CTS activated."]
    #[inline]
    pub(crate) fn read_uart0_events_cts(&self) -> MemResult<u32> {
        todo!("read UART0 EVENTS_CTS reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "UART0 EVENTS_CTS: CTS activated."]
    #[inline]
    pub(crate) fn write_uart0_events_cts(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write UART0 EVENTS_CTS reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "UART0 EVENTS_NCTS: CTS deactivated."]
    #[inline]
    pub(crate) fn read_uart0_events_ncts(&self) -> MemResult<u32> {
        todo!("read UART0 EVENTS_NCTS reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "UART0 EVENTS_NCTS: CTS deactivated."]
    #[inline]
    pub(crate) fn write_uart0_events_ncts(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!(
            "write UART0 EVENTS_NCTS reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "UART0 EVENTS_RXDRDY: Data received in RXD."]
    #[inline]
    pub(crate) fn read_uart0_events_rxdrdy(&self) -> MemResult<u32> {
        todo!(
            "read UART0 EVENTS_RXDRDY reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "UART0 EVENTS_RXDRDY: Data received in RXD."]
    #[inline]
    pub(crate) fn write_uart0_events_rxdrdy(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write UART0 EVENTS_RXDRDY reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "UART0 EVENTS_TXDRDY: Data sent from TXD."]
    #[inline]
    pub(crate) fn read_uart0_events_txdrdy(&self) -> MemResult<u32> {
        todo!(
            "read UART0 EVENTS_TXDRDY reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "UART0 EVENTS_TXDRDY: Data sent from TXD."]
    #[inline]
    pub(crate) fn write_uart0_events_txdrdy(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write UART0 EVENTS_TXDRDY reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "UART0 EVENTS_ERROR: Error detected."]
    #[inline]
    pub(crate) fn read_uart0_events_error(&self) -> MemResult<u32> {
        todo!(
            "read UART0 EVENTS_ERROR reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "UART0 EVENTS_ERROR: Error detected."]
    #[inline]
    pub(crate) fn write_uart0_events_error(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!(
            "write UART0 EVENTS_ERROR reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "UART0 EVENTS_RXTO: Receiver timeout."]
    #[inline]
    pub(crate) fn read_uart0_events_rxto(&self) -> MemResult<u32> {
        todo!("read UART0 EVENTS_RXTO reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "UART0 EVENTS_RXTO: Receiver timeout."]
    #[inline]
    pub(crate) fn write_uart0_events_rxto(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!(
            "write UART0 EVENTS_RXTO reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "UART0 SHORTS CTS_STARTRX: Shortcut between CTS event and STARTRX task."]
    #[inline]
    pub(crate) fn read_uart0_shorts_cts_startrx(&self) -> MemResult<bool> {
        todo!("read UART0 SHORTS CTS_STARTRX reset value false")
    }
    #[doc = "UART0 SHORTS CTS_STARTRX: Shortcut between CTS event and STARTRX task."]
    #[inline]
    pub(crate) fn write_uart0_shorts_cts_startrx(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write UART0 SHORTS CTS_STARTRX reset value false")
    }
    #[doc = "UART0 SHORTS NCTS_STOPRX: Shortcut between NCTS event and STOPRX task."]
    #[inline]
    pub(crate) fn read_uart0_shorts_ncts_stoprx(&self) -> MemResult<bool> {
        todo!("read UART0 SHORTS NCTS_STOPRX reset value false")
    }
    #[doc = "UART0 SHORTS NCTS_STOPRX: Shortcut between NCTS event and STOPRX task."]
    #[inline]
    pub(crate) fn write_uart0_shorts_ncts_stoprx(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write UART0 SHORTS NCTS_STOPRX reset value false")
    }
    #[doc = "UART0 INTENSET CTS: Enable interrupt on CTS event."]
    #[inline]
    pub(crate) fn read_uart0_intenset_cts(&self) -> MemResult<bool> {
        todo!("read UART0 INTENSET CTS reset value false")
    }
    #[doc = "UART0 INTENSET CTS: Enable interrupt on CTS event."]
    #[inline]
    pub(crate) fn write_uart0_intenset_cts(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write UART0 INTENSET CTS reset value false")
    }
    #[doc = "UART0 INTENSET NCTS: Enable interrupt on NCTS event."]
    #[inline]
    pub(crate) fn read_uart0_intenset_ncts(&self) -> MemResult<bool> {
        todo!("read UART0 INTENSET NCTS reset value false")
    }
    #[doc = "UART0 INTENSET NCTS: Enable interrupt on NCTS event."]
    #[inline]
    pub(crate) fn write_uart0_intenset_ncts(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write UART0 INTENSET NCTS reset value false")
    }
    #[doc = "UART0 INTENSET RXDRDY: Enable interrupt on RXRDY event."]
    #[inline]
    pub(crate) fn read_uart0_intenset_rxdrdy(&self) -> MemResult<bool> {
        todo!("read UART0 INTENSET RXDRDY reset value false")
    }
    #[doc = "UART0 INTENSET RXDRDY: Enable interrupt on RXRDY event."]
    #[inline]
    pub(crate) fn write_uart0_intenset_rxdrdy(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write UART0 INTENSET RXDRDY reset value false")
    }
    #[doc = "UART0 INTENSET TXDRDY: Enable interrupt on TXRDY event."]
    #[inline]
    pub(crate) fn read_uart0_intenset_txdrdy(&self) -> MemResult<bool> {
        todo!("read UART0 INTENSET TXDRDY reset value false")
    }
    #[doc = "UART0 INTENSET TXDRDY: Enable interrupt on TXRDY event."]
    #[inline]
    pub(crate) fn write_uart0_intenset_txdrdy(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write UART0 INTENSET TXDRDY reset value false")
    }
    #[doc = "UART0 INTENSET ERROR: Enable interrupt on ERROR event."]
    #[inline]
    pub(crate) fn read_uart0_intenset_error(&self) -> MemResult<bool> {
        todo!("read UART0 INTENSET ERROR reset value false")
    }
    #[doc = "UART0 INTENSET ERROR: Enable interrupt on ERROR event."]
    #[inline]
    pub(crate) fn write_uart0_intenset_error(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write UART0 INTENSET ERROR reset value false")
    }
    #[doc = "UART0 INTENSET RXTO: Enable interrupt on RXTO event."]
    #[inline]
    pub(crate) fn read_uart0_intenset_rxto(&self) -> MemResult<bool> {
        todo!("read UART0 INTENSET RXTO reset value false")
    }
    #[doc = "UART0 INTENSET RXTO: Enable interrupt on RXTO event."]
    #[inline]
    pub(crate) fn write_uart0_intenset_rxto(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write UART0 INTENSET RXTO reset value false")
    }
    #[doc = "UART0 INTENCLR CTS: Disable interrupt on CTS event."]
    #[inline]
    pub(crate) fn read_uart0_intenclr_cts(&self) -> MemResult<bool> {
        todo!("read UART0 INTENCLR CTS reset value false")
    }
    #[doc = "UART0 INTENCLR CTS: Disable interrupt on CTS event."]
    #[inline]
    pub(crate) fn write_uart0_intenclr_cts(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write UART0 INTENCLR CTS reset value false")
    }
    #[doc = "UART0 INTENCLR NCTS: Disable interrupt on NCTS event."]
    #[inline]
    pub(crate) fn read_uart0_intenclr_ncts(&self) -> MemResult<bool> {
        todo!("read UART0 INTENCLR NCTS reset value false")
    }
    #[doc = "UART0 INTENCLR NCTS: Disable interrupt on NCTS event."]
    #[inline]
    pub(crate) fn write_uart0_intenclr_ncts(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write UART0 INTENCLR NCTS reset value false")
    }
    #[doc = "UART0 INTENCLR RXDRDY: Disable interrupt on RXRDY event."]
    #[inline]
    pub(crate) fn read_uart0_intenclr_rxdrdy(&self) -> MemResult<bool> {
        todo!("read UART0 INTENCLR RXDRDY reset value false")
    }
    #[doc = "UART0 INTENCLR RXDRDY: Disable interrupt on RXRDY event."]
    #[inline]
    pub(crate) fn write_uart0_intenclr_rxdrdy(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write UART0 INTENCLR RXDRDY reset value false")
    }
    #[doc = "UART0 INTENCLR TXDRDY: Disable interrupt on TXRDY event."]
    #[inline]
    pub(crate) fn read_uart0_intenclr_txdrdy(&self) -> MemResult<bool> {
        todo!("read UART0 INTENCLR TXDRDY reset value false")
    }
    #[doc = "UART0 INTENCLR TXDRDY: Disable interrupt on TXRDY event."]
    #[inline]
    pub(crate) fn write_uart0_intenclr_txdrdy(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write UART0 INTENCLR TXDRDY reset value false")
    }
    #[doc = "UART0 INTENCLR ERROR: Disable interrupt on ERROR event."]
    #[inline]
    pub(crate) fn read_uart0_intenclr_error(&self) -> MemResult<bool> {
        todo!("read UART0 INTENCLR ERROR reset value false")
    }
    #[doc = "UART0 INTENCLR ERROR: Disable interrupt on ERROR event."]
    #[inline]
    pub(crate) fn write_uart0_intenclr_error(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write UART0 INTENCLR ERROR reset value false")
    }
    #[doc = "UART0 INTENCLR RXTO: Disable interrupt on RXTO event."]
    #[inline]
    pub(crate) fn read_uart0_intenclr_rxto(&self) -> MemResult<bool> {
        todo!("read UART0 INTENCLR RXTO reset value false")
    }
    #[doc = "UART0 INTENCLR RXTO: Disable interrupt on RXTO event."]
    #[inline]
    pub(crate) fn write_uart0_intenclr_rxto(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write UART0 INTENCLR RXTO reset value false")
    }
    #[doc = "UART0 ERRORSRC OVERRUN: A start bit is received while the previous data still lies in RXD. (Data loss)."]
    #[inline]
    pub(crate) fn read_uart0_errorsrc_overrun(&self) -> MemResult<bool> {
        todo!("read UART0 ERRORSRC OVERRUN reset value false")
    }
    #[doc = "UART0 ERRORSRC OVERRUN: A start bit is received while the previous data still lies in RXD. (Data loss)."]
    #[inline]
    pub(crate) fn write_uart0_errorsrc_overrun(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write UART0 ERRORSRC OVERRUN reset value false")
    }
    #[doc = "UART0 ERRORSRC PARITY: A character with bad parity is received. Only checked if HW parity control is enabled."]
    #[inline]
    pub(crate) fn read_uart0_errorsrc_parity(&self) -> MemResult<bool> {
        todo!("read UART0 ERRORSRC PARITY reset value false")
    }
    #[doc = "UART0 ERRORSRC PARITY: A character with bad parity is received. Only checked if HW parity control is enabled."]
    #[inline]
    pub(crate) fn write_uart0_errorsrc_parity(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write UART0 ERRORSRC PARITY reset value false")
    }
    #[doc = "UART0 ERRORSRC FRAMING: A valid stop bit is not detected on the serial data input after all bits in a character have been received."]
    #[inline]
    pub(crate) fn read_uart0_errorsrc_framing(&self) -> MemResult<bool> {
        todo!("read UART0 ERRORSRC FRAMING reset value false")
    }
    #[doc = "UART0 ERRORSRC FRAMING: A valid stop bit is not detected on the serial data input after all bits in a character have been received."]
    #[inline]
    pub(crate) fn write_uart0_errorsrc_framing(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write UART0 ERRORSRC FRAMING reset value false")
    }
    #[doc = "UART0 ERRORSRC BREAK: The serial data input is '0' for longer than the length of a data frame."]
    #[inline]
    pub(crate) fn read_uart0_errorsrc_break(&self) -> MemResult<bool> {
        todo!("read UART0 ERRORSRC BREAK reset value false")
    }
    #[doc = "UART0 ERRORSRC BREAK: The serial data input is '0' for longer than the length of a data frame."]
    #[inline]
    pub(crate) fn write_uart0_errorsrc_break(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write UART0 ERRORSRC BREAK reset value false")
    }
    #[doc = "UART0 ENABLE ENABLE: Enable or disable UART and acquire IOs."]
    #[inline]
    pub(crate) fn read_uart0_enable_enable(&self) -> MemResult<u8> {
        todo!("read UART0 ENABLE ENABLE reset value 0x00 mask 0x07")
    }
    #[doc = "UART0 ENABLE ENABLE: Enable or disable UART and acquire IOs."]
    #[inline]
    pub(crate) fn write_uart0_enable_enable(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write UART0 ENABLE ENABLE reset value 0x00 mask 0x07")
    }
    #[doc = "UART0 PSELRTS: Pin select for RTS."]
    #[inline]
    pub(crate) fn read_uart0_pselrts(&self) -> MemResult<u32> {
        todo!(
            "read UART0 PSELRTS reset value 0xffffffff mask 0xffffffffffffffff"
        )
    }
    #[doc = "UART0 PSELRTS: Pin select for RTS."]
    #[inline]
    pub(crate) fn write_uart0_pselrts(&mut self, _value: u32) -> MemResult<()> {
        todo ! ("write UART0 PSELRTS reset value 0xffffffff mask 0xffffffffffffffff")
    }
    #[doc = "UART0 PSELTXD: Pin select for TXD."]
    #[inline]
    pub(crate) fn read_uart0_pseltxd(&self) -> MemResult<u32> {
        todo!(
            "read UART0 PSELTXD reset value 0xffffffff mask 0xffffffffffffffff"
        )
    }
    #[doc = "UART0 PSELTXD: Pin select for TXD."]
    #[inline]
    pub(crate) fn write_uart0_pseltxd(&mut self, _value: u32) -> MemResult<()> {
        todo ! ("write UART0 PSELTXD reset value 0xffffffff mask 0xffffffffffffffff")
    }
    #[doc = "UART0 PSELCTS: Pin select for CTS."]
    #[inline]
    pub(crate) fn read_uart0_pselcts(&self) -> MemResult<u32> {
        todo!(
            "read UART0 PSELCTS reset value 0xffffffff mask 0xffffffffffffffff"
        )
    }
    #[doc = "UART0 PSELCTS: Pin select for CTS."]
    #[inline]
    pub(crate) fn write_uart0_pselcts(&mut self, _value: u32) -> MemResult<()> {
        todo ! ("write UART0 PSELCTS reset value 0xffffffff mask 0xffffffffffffffff")
    }
    #[doc = "UART0 PSELRXD: Pin select for RXD."]
    #[inline]
    pub(crate) fn read_uart0_pselrxd(&self) -> MemResult<u32> {
        todo!(
            "read UART0 PSELRXD reset value 0xffffffff mask 0xffffffffffffffff"
        )
    }
    #[doc = "UART0 PSELRXD: Pin select for RXD."]
    #[inline]
    pub(crate) fn write_uart0_pselrxd(&mut self, _value: u32) -> MemResult<()> {
        todo ! ("write UART0 PSELRXD reset value 0xffffffff mask 0xffffffffffffffff")
    }
    #[doc = "UART0 RXD RXD: RX data from previous transfer. Double buffered."]
    #[inline]
    pub(crate) fn read_uart0_rxd_rxd(&self) -> MemResult<u8> {
        todo!("read UART0 RXD RXD reset value 0x00 mask 0xff")
    }
    #[doc = "UART0 TXD TXD: TX data for transfer."]
    #[inline]
    pub(crate) fn write_uart0_txd_txd(&mut self, _value: u8) -> MemResult<()> {
        todo!("write UART0 TXD TXD reset value 0x00 mask 0xff")
    }
    #[doc = "UART0 BAUDRATE: UART Baudrate."]
    #[inline]
    pub(crate) fn read_uart0_baudrate(&self) -> MemResult<u32> {
        todo!("read UART0 BAUDRATE reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "UART0 BAUDRATE: UART Baudrate."]
    #[inline]
    pub(crate) fn write_uart0_baudrate(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write UART0 BAUDRATE reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "UART0 CONFIG HWFC: Hardware flow control."]
    #[inline]
    pub(crate) fn read_uart0_config_hwfc(&self) -> MemResult<bool> {
        todo!("read UART0 CONFIG HWFC reset value false")
    }
    #[doc = "UART0 CONFIG HWFC: Hardware flow control."]
    #[inline]
    pub(crate) fn write_uart0_config_hwfc(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write UART0 CONFIG HWFC reset value false")
    }
    #[doc = "UART0 CONFIG PARITY: Include parity bit."]
    #[inline]
    pub(crate) fn read_uart0_config_parity(&self) -> MemResult<u8> {
        todo!("read UART0 CONFIG PARITY reset value 0x00 mask 0x07")
    }
    #[doc = "UART0 CONFIG PARITY: Include parity bit."]
    #[inline]
    pub(crate) fn write_uart0_config_parity(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write UART0 CONFIG PARITY reset value 0x00 mask 0x07")
    }
    #[doc = "UART0 POWER POWER: Peripheral power control."]
    #[inline]
    pub(crate) fn read_uart0_power_power(&self) -> MemResult<bool> {
        todo!("read UART0 POWER POWER reset value false")
    }
    #[doc = "UART0 POWER POWER: Peripheral power control."]
    #[inline]
    pub(crate) fn write_uart0_power_power(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write UART0 POWER POWER reset value false")
    }
    #[doc = "SPI0 EVENTS_READY: TXD byte sent and RXD byte received.\n\nTWI0 EVENTS_RXDREADY: Two-wire ready to deliver new RXD byte received.\n\nSPI0 EVENTS_READY: TXD byte sent and RXD byte received.\n\nTWI0 EVENTS_RXDREADY: Two-wire ready to deliver new RXD byte received."]
    #[inline]
    pub(crate) fn read_spi0twi0_events_ready(&self) -> MemResult<u32> {
        todo ! ("read SPI0 EVENTS_READY, TWI0 EVENTS_RXDREADY, SPI0 EVENTS_READY, TWI0 EVENTS_RXDREADY reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "SPI0 EVENTS_READY: TXD byte sent and RXD byte received.\n\nTWI0 EVENTS_RXDREADY: Two-wire ready to deliver new RXD byte received.\n\nSPI0 EVENTS_READY: TXD byte sent and RXD byte received.\n\nTWI0 EVENTS_RXDREADY: Two-wire ready to deliver new RXD byte received."]
    #[inline]
    pub(crate) fn write_spi0twi0_events_ready(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write SPI0 EVENTS_READY, TWI0 EVENTS_RXDREADY, SPI0 EVENTS_READY, TWI0 EVENTS_RXDREADY reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "SPI0 INTENSET READY: Enable interrupt on READY event.\n\nTWI0 INTENSET RXDREADY: Enable interrupt on READY event.\n\nSPI0 INTENSET READY: Enable interrupt on READY event.\n\nTWI0 INTENSET RXDREADY: Enable interrupt on READY event."]
    #[inline]
    pub(crate) fn read_spi0twi0_intenset_ready(&self) -> MemResult<bool> {
        todo ! ("read SPI0 INTENSET READY, TWI0 INTENSET RXDREADY, SPI0 INTENSET READY, TWI0 INTENSET RXDREADY reset value false")
    }
    #[doc = "SPI0 INTENSET READY: Enable interrupt on READY event.\n\nTWI0 INTENSET RXDREADY: Enable interrupt on READY event.\n\nSPI0 INTENSET READY: Enable interrupt on READY event.\n\nTWI0 INTENSET RXDREADY: Enable interrupt on READY event."]
    #[inline]
    pub(crate) fn write_spi0twi0_intenset_ready(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write SPI0 INTENSET READY, TWI0 INTENSET RXDREADY, SPI0 INTENSET READY, TWI0 INTENSET RXDREADY reset value false")
    }
    #[doc = "TWI0 INTENSET STOPPED: Enable interrupt on STOPPED event.\n\nTWI0 INTENSET STOPPED: Enable interrupt on STOPPED event."]
    #[inline]
    pub(crate) fn read_spi0twi0_intenset_stopped(&self) -> MemResult<bool> {
        todo ! ("read TWI0 INTENSET STOPPED, TWI0 INTENSET STOPPED reset value false")
    }
    #[doc = "TWI0 INTENSET STOPPED: Enable interrupt on STOPPED event.\n\nTWI0 INTENSET STOPPED: Enable interrupt on STOPPED event."]
    #[inline]
    pub(crate) fn write_spi0twi0_intenset_stopped(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TWI0 INTENSET STOPPED, TWI0 INTENSET STOPPED reset value false")
    }
    #[doc = "TWI0 INTENSET TXDSENT: Enable interrupt on TXDSENT event.\n\nTWI0 INTENSET TXDSENT: Enable interrupt on TXDSENT event."]
    #[inline]
    pub(crate) fn read_spi0twi0_intenset_txdsent(&self) -> MemResult<bool> {
        todo ! ("read TWI0 INTENSET TXDSENT, TWI0 INTENSET TXDSENT reset value false")
    }
    #[doc = "TWI0 INTENSET TXDSENT: Enable interrupt on TXDSENT event.\n\nTWI0 INTENSET TXDSENT: Enable interrupt on TXDSENT event."]
    #[inline]
    pub(crate) fn write_spi0twi0_intenset_txdsent(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TWI0 INTENSET TXDSENT, TWI0 INTENSET TXDSENT reset value false")
    }
    #[doc = "TWI0 INTENSET ERROR: Enable interrupt on ERROR event.\n\nTWI0 INTENSET ERROR: Enable interrupt on ERROR event."]
    #[inline]
    pub(crate) fn read_spi0twi0_intenset_error(&self) -> MemResult<bool> {
        todo!("read TWI0 INTENSET ERROR, TWI0 INTENSET ERROR reset value false")
    }
    #[doc = "TWI0 INTENSET ERROR: Enable interrupt on ERROR event.\n\nTWI0 INTENSET ERROR: Enable interrupt on ERROR event."]
    #[inline]
    pub(crate) fn write_spi0twi0_intenset_error(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write TWI0 INTENSET ERROR, TWI0 INTENSET ERROR reset value false"
        )
    }
    #[doc = "TWI0 INTENSET BB: Enable interrupt on BB event.\n\nTWI0 INTENSET BB: Enable interrupt on BB event."]
    #[inline]
    pub(crate) fn read_spi0twi0_intenset_bb(&self) -> MemResult<bool> {
        todo!("read TWI0 INTENSET BB, TWI0 INTENSET BB reset value false")
    }
    #[doc = "TWI0 INTENSET BB: Enable interrupt on BB event.\n\nTWI0 INTENSET BB: Enable interrupt on BB event."]
    #[inline]
    pub(crate) fn write_spi0twi0_intenset_bb(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write TWI0 INTENSET BB, TWI0 INTENSET BB reset value false")
    }
    #[doc = "TWI0 INTENSET SUSPENDED: Enable interrupt on SUSPENDED event.\n\nTWI0 INTENSET SUSPENDED: Enable interrupt on SUSPENDED event."]
    #[inline]
    pub(crate) fn read_spi0twi0_intenset_suspended(&self) -> MemResult<bool> {
        todo ! ("read TWI0 INTENSET SUSPENDED, TWI0 INTENSET SUSPENDED reset value false")
    }
    #[doc = "TWI0 INTENSET SUSPENDED: Enable interrupt on SUSPENDED event.\n\nTWI0 INTENSET SUSPENDED: Enable interrupt on SUSPENDED event."]
    #[inline]
    pub(crate) fn write_spi0twi0_intenset_suspended(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TWI0 INTENSET SUSPENDED, TWI0 INTENSET SUSPENDED reset value false")
    }
    #[doc = "SPI0 INTENCLR READY: Disable interrupt on READY event.\n\nTWI0 INTENCLR RXDREADY: Disable interrupt on RXDREADY event.\n\nSPI0 INTENCLR READY: Disable interrupt on READY event.\n\nTWI0 INTENCLR RXDREADY: Disable interrupt on RXDREADY event."]
    #[inline]
    pub(crate) fn read_spi0twi0_intenclr_ready(&self) -> MemResult<bool> {
        todo ! ("read SPI0 INTENCLR READY, TWI0 INTENCLR RXDREADY, SPI0 INTENCLR READY, TWI0 INTENCLR RXDREADY reset value false")
    }
    #[doc = "SPI0 INTENCLR READY: Disable interrupt on READY event.\n\nTWI0 INTENCLR RXDREADY: Disable interrupt on RXDREADY event.\n\nSPI0 INTENCLR READY: Disable interrupt on READY event.\n\nTWI0 INTENCLR RXDREADY: Disable interrupt on RXDREADY event."]
    #[inline]
    pub(crate) fn write_spi0twi0_intenclr_ready(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write SPI0 INTENCLR READY, TWI0 INTENCLR RXDREADY, SPI0 INTENCLR READY, TWI0 INTENCLR RXDREADY reset value false")
    }
    #[doc = "TWI0 INTENCLR STOPPED: Disable interrupt on STOPPED event.\n\nTWI0 INTENCLR STOPPED: Disable interrupt on STOPPED event."]
    #[inline]
    pub(crate) fn read_spi0twi0_intenclr_stopped(&self) -> MemResult<bool> {
        todo ! ("read TWI0 INTENCLR STOPPED, TWI0 INTENCLR STOPPED reset value false")
    }
    #[doc = "TWI0 INTENCLR STOPPED: Disable interrupt on STOPPED event.\n\nTWI0 INTENCLR STOPPED: Disable interrupt on STOPPED event."]
    #[inline]
    pub(crate) fn write_spi0twi0_intenclr_stopped(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TWI0 INTENCLR STOPPED, TWI0 INTENCLR STOPPED reset value false")
    }
    #[doc = "TWI0 INTENCLR TXDSENT: Disable interrupt on TXDSENT event.\n\nTWI0 INTENCLR TXDSENT: Disable interrupt on TXDSENT event."]
    #[inline]
    pub(crate) fn read_spi0twi0_intenclr_txdsent(&self) -> MemResult<bool> {
        todo ! ("read TWI0 INTENCLR TXDSENT, TWI0 INTENCLR TXDSENT reset value false")
    }
    #[doc = "TWI0 INTENCLR TXDSENT: Disable interrupt on TXDSENT event.\n\nTWI0 INTENCLR TXDSENT: Disable interrupt on TXDSENT event."]
    #[inline]
    pub(crate) fn write_spi0twi0_intenclr_txdsent(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TWI0 INTENCLR TXDSENT, TWI0 INTENCLR TXDSENT reset value false")
    }
    #[doc = "TWI0 INTENCLR ERROR: Disable interrupt on ERROR event.\n\nTWI0 INTENCLR ERROR: Disable interrupt on ERROR event."]
    #[inline]
    pub(crate) fn read_spi0twi0_intenclr_error(&self) -> MemResult<bool> {
        todo!("read TWI0 INTENCLR ERROR, TWI0 INTENCLR ERROR reset value false")
    }
    #[doc = "TWI0 INTENCLR ERROR: Disable interrupt on ERROR event.\n\nTWI0 INTENCLR ERROR: Disable interrupt on ERROR event."]
    #[inline]
    pub(crate) fn write_spi0twi0_intenclr_error(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write TWI0 INTENCLR ERROR, TWI0 INTENCLR ERROR reset value false"
        )
    }
    #[doc = "TWI0 INTENCLR BB: Disable interrupt on BB event.\n\nTWI0 INTENCLR BB: Disable interrupt on BB event."]
    #[inline]
    pub(crate) fn read_spi0twi0_intenclr_bb(&self) -> MemResult<bool> {
        todo!("read TWI0 INTENCLR BB, TWI0 INTENCLR BB reset value false")
    }
    #[doc = "TWI0 INTENCLR BB: Disable interrupt on BB event.\n\nTWI0 INTENCLR BB: Disable interrupt on BB event."]
    #[inline]
    pub(crate) fn write_spi0twi0_intenclr_bb(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write TWI0 INTENCLR BB, TWI0 INTENCLR BB reset value false")
    }
    #[doc = "TWI0 INTENCLR SUSPENDED: Disable interrupt on SUSPENDED event.\n\nTWI0 INTENCLR SUSPENDED: Disable interrupt on SUSPENDED event."]
    #[inline]
    pub(crate) fn read_spi0twi0_intenclr_suspended(&self) -> MemResult<bool> {
        todo ! ("read TWI0 INTENCLR SUSPENDED, TWI0 INTENCLR SUSPENDED reset value false")
    }
    #[doc = "TWI0 INTENCLR SUSPENDED: Disable interrupt on SUSPENDED event.\n\nTWI0 INTENCLR SUSPENDED: Disable interrupt on SUSPENDED event."]
    #[inline]
    pub(crate) fn write_spi0twi0_intenclr_suspended(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TWI0 INTENCLR SUSPENDED, TWI0 INTENCLR SUSPENDED reset value false")
    }
    #[doc = "SPI0 ENABLE ENABLE: Enable or disable SPI.\n\nTWI0 ENABLE ENABLE: Enable or disable W2M\n\nSPI0 ENABLE ENABLE: Enable or disable SPI.\n\nTWI0 ENABLE ENABLE: Enable or disable W2M"]
    #[inline]
    pub(crate) fn read_spi0twi0_enable_enable(&self) -> MemResult<u8> {
        todo ! ("read SPI0 ENABLE ENABLE, TWI0 ENABLE ENABLE, SPI0 ENABLE ENABLE, TWI0 ENABLE ENABLE reset value 0x00 mask 0x07")
    }
    #[doc = "SPI0 ENABLE ENABLE: Enable or disable SPI.\n\nTWI0 ENABLE ENABLE: Enable or disable W2M\n\nSPI0 ENABLE ENABLE: Enable or disable SPI.\n\nTWI0 ENABLE ENABLE: Enable or disable W2M"]
    #[inline]
    pub(crate) fn write_spi0twi0_enable_enable(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write SPI0 ENABLE ENABLE, TWI0 ENABLE ENABLE, SPI0 ENABLE ENABLE, TWI0 ENABLE ENABLE reset value 0x00 mask 0x07")
    }
    #[doc = "SPI0 PSELSCK: Pin select for SCK.\n\nTWI0 PSELSCL: Pin select for SCL.\n\nSPI0 PSELSCK: Pin select for SCK.\n\nTWI0 PSELSCL: Pin select for SCL."]
    #[inline]
    pub(crate) fn read_spi0twi0_pselsck(&self) -> MemResult<u32> {
        todo ! ("read SPI0 PSELSCK, TWI0 PSELSCL, SPI0 PSELSCK, TWI0 PSELSCL reset value 0xffffffff mask 0xffffffffffffffff")
    }
    #[doc = "SPI0 PSELSCK: Pin select for SCK.\n\nTWI0 PSELSCL: Pin select for SCL.\n\nSPI0 PSELSCK: Pin select for SCK.\n\nTWI0 PSELSCL: Pin select for SCL."]
    #[inline]
    pub(crate) fn write_spi0twi0_pselsck(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write SPI0 PSELSCK, TWI0 PSELSCL, SPI0 PSELSCK, TWI0 PSELSCL reset value 0xffffffff mask 0xffffffffffffffff")
    }
    #[doc = "SPI0 PSELMOSI: Pin select for MOSI.\n\nTWI0 PSELSDA: Pin select for SDA.\n\nSPI0 PSELMOSI: Pin select for MOSI.\n\nTWI0 PSELSDA: Pin select for SDA."]
    #[inline]
    pub(crate) fn read_spi0twi0_pselmosi(&self) -> MemResult<u32> {
        todo ! ("read SPI0 PSELMOSI, TWI0 PSELSDA, SPI0 PSELMOSI, TWI0 PSELSDA reset value 0xffffffff mask 0xffffffffffffffff")
    }
    #[doc = "SPI0 PSELMOSI: Pin select for MOSI.\n\nTWI0 PSELSDA: Pin select for SDA.\n\nSPI0 PSELMOSI: Pin select for MOSI.\n\nTWI0 PSELSDA: Pin select for SDA."]
    #[inline]
    pub(crate) fn write_spi0twi0_pselmosi(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write SPI0 PSELMOSI, TWI0 PSELSDA, SPI0 PSELMOSI, TWI0 PSELSDA reset value 0xffffffff mask 0xffffffffffffffff")
    }
    #[doc = "SPI0 PSELMISO: Pin select for MISO.\n\nSPI0 PSELMISO: Pin select for MISO."]
    #[inline]
    pub(crate) fn read_spi0_pselmiso(&self) -> MemResult<u32> {
        todo ! ("read SPI0 PSELMISO, SPI0 PSELMISO reset value 0xffffffff mask 0xffffffffffffffff")
    }
    #[doc = "SPI0 PSELMISO: Pin select for MISO.\n\nSPI0 PSELMISO: Pin select for MISO."]
    #[inline]
    pub(crate) fn write_spi0_pselmiso(&mut self, _value: u32) -> MemResult<()> {
        todo ! ("write SPI0 PSELMISO, SPI0 PSELMISO reset value 0xffffffff mask 0xffffffffffffffff")
    }
    #[doc = "SPI0 RXD RXD: RX data from last transfer.\n\nTWI0 RXD RXD: RX data from last transfer.\n\nSPI0 RXD RXD: RX data from last transfer.\n\nTWI0 RXD RXD: RX data from last transfer."]
    #[inline]
    pub(crate) fn read_spi0twi0_rxd_rxd(&self) -> MemResult<u8> {
        todo ! ("read SPI0 RXD RXD, TWI0 RXD RXD, SPI0 RXD RXD, TWI0 RXD RXD reset value 0x00 mask 0xff")
    }
    #[doc = "SPI0 TXD TXD: TX data for next transfer.\n\nTWI0 TXD TXD: TX data for next transfer.\n\nSPI0 TXD TXD: TX data for next transfer.\n\nTWI0 TXD TXD: TX data for next transfer."]
    #[inline]
    pub(crate) fn read_spi0twi0_txd_txd(&self) -> MemResult<u8> {
        todo ! ("read SPI0 TXD TXD, TWI0 TXD TXD, SPI0 TXD TXD, TWI0 TXD TXD reset value 0x00 mask 0xff")
    }
    #[doc = "SPI0 TXD TXD: TX data for next transfer.\n\nTWI0 TXD TXD: TX data for next transfer.\n\nSPI0 TXD TXD: TX data for next transfer.\n\nTWI0 TXD TXD: TX data for next transfer."]
    #[inline]
    pub(crate) fn write_spi0twi0_txd_txd(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write SPI0 TXD TXD, TWI0 TXD TXD, SPI0 TXD TXD, TWI0 TXD TXD reset value 0x00 mask 0xff")
    }
    #[doc = "SPI0 FREQUENCY: SPI frequency\n\nTWI0 FREQUENCY: Two-wire frequency.\n\nSPI0 FREQUENCY: SPI frequency\n\nTWI0 FREQUENCY: Two-wire frequency."]
    #[inline]
    pub(crate) fn read_spi0twi0_frequency(&self) -> MemResult<u32> {
        todo ! ("read SPI0 FREQUENCY, TWI0 FREQUENCY, SPI0 FREQUENCY, TWI0 FREQUENCY reset value 0x4000000 mask 0xffffffffffffffff")
    }
    #[doc = "SPI0 FREQUENCY: SPI frequency\n\nTWI0 FREQUENCY: Two-wire frequency.\n\nSPI0 FREQUENCY: SPI frequency\n\nTWI0 FREQUENCY: Two-wire frequency."]
    #[inline]
    pub(crate) fn write_spi0twi0_frequency(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write SPI0 FREQUENCY, TWI0 FREQUENCY, SPI0 FREQUENCY, TWI0 FREQUENCY reset value 0x4000000 mask 0xffffffffffffffff")
    }
    #[doc = "SPI0 CONFIG ORDER: Bit order.\n\nSPI0 CONFIG ORDER: Bit order."]
    #[inline]
    pub(crate) fn read_spi0_config_order(&self) -> MemResult<bool> {
        todo!("read SPI0 CONFIG ORDER, SPI0 CONFIG ORDER reset value false")
    }
    #[doc = "SPI0 CONFIG ORDER: Bit order.\n\nSPI0 CONFIG ORDER: Bit order."]
    #[inline]
    pub(crate) fn write_spi0_config_order(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SPI0 CONFIG ORDER, SPI0 CONFIG ORDER reset value false")
    }
    #[doc = "SPI0 CONFIG CPHA: Serial clock (SCK) phase.\n\nSPI0 CONFIG CPHA: Serial clock (SCK) phase."]
    #[inline]
    pub(crate) fn read_spi0_config_cpha(&self) -> MemResult<bool> {
        todo!("read SPI0 CONFIG CPHA, SPI0 CONFIG CPHA reset value false")
    }
    #[doc = "SPI0 CONFIG CPHA: Serial clock (SCK) phase.\n\nSPI0 CONFIG CPHA: Serial clock (SCK) phase."]
    #[inline]
    pub(crate) fn write_spi0_config_cpha(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SPI0 CONFIG CPHA, SPI0 CONFIG CPHA reset value false")
    }
    #[doc = "SPI0 CONFIG CPOL: Serial clock (SCK) polarity.\n\nSPI0 CONFIG CPOL: Serial clock (SCK) polarity."]
    #[inline]
    pub(crate) fn read_spi0_config_cpol(&self) -> MemResult<bool> {
        todo!("read SPI0 CONFIG CPOL, SPI0 CONFIG CPOL reset value false")
    }
    #[doc = "SPI0 CONFIG CPOL: Serial clock (SCK) polarity.\n\nSPI0 CONFIG CPOL: Serial clock (SCK) polarity."]
    #[inline]
    pub(crate) fn write_spi0_config_cpol(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SPI0 CONFIG CPOL, SPI0 CONFIG CPOL reset value false")
    }
    #[doc = "SPI0 POWER POWER: Peripheral power control.\n\nTWI0 POWER POWER: Peripheral power control.\n\nSPI0 POWER POWER: Peripheral power control.\n\nTWI0 POWER POWER: Peripheral power control."]
    #[inline]
    pub(crate) fn read_spi0twi0_power_power(&self) -> MemResult<bool> {
        todo ! ("read SPI0 POWER POWER, TWI0 POWER POWER, SPI0 POWER POWER, TWI0 POWER POWER reset value false")
    }
    #[doc = "SPI0 POWER POWER: Peripheral power control.\n\nTWI0 POWER POWER: Peripheral power control.\n\nSPI0 POWER POWER: Peripheral power control.\n\nTWI0 POWER POWER: Peripheral power control."]
    #[inline]
    pub(crate) fn write_spi0twi0_power_power(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write SPI0 POWER POWER, TWI0 POWER POWER, SPI0 POWER POWER, TWI0 POWER POWER reset value false")
    }
    #[doc = "TWI0 TASKS_STARTRX: Start 2-Wire master receive sequence.\n\nTWI0 TASKS_STARTRX: Start 2-Wire master receive sequence."]
    #[inline]
    pub(crate) fn write_twi0_tasks_startrx(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write TWI0 TASKS_STARTRX, TWI0 TASKS_STARTRX reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "TWI0 TASKS_STARTTX: Start 2-Wire master transmit sequence.\n\nTWI0 TASKS_STARTTX: Start 2-Wire master transmit sequence."]
    #[inline]
    pub(crate) fn write_twi0_tasks_starttx(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write TWI0 TASKS_STARTTX, TWI0 TASKS_STARTTX reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "TWI0 TASKS_STOP: Stop 2-Wire transaction.\n\nTWI0 TASKS_STOP: Stop 2-Wire transaction."]
    #[inline]
    pub(crate) fn write_twi0_tasks_stop(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write TWI0 TASKS_STOP, TWI0 TASKS_STOP reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "TWI0 TASKS_SUSPEND: Suspend 2-Wire transaction.\n\nTWI0 TASKS_SUSPEND: Suspend 2-Wire transaction."]
    #[inline]
    pub(crate) fn write_twi0_tasks_suspend(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write TWI0 TASKS_SUSPEND, TWI0 TASKS_SUSPEND reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "TWI0 TASKS_RESUME: Resume 2-Wire transaction.\n\nTWI0 TASKS_RESUME: Resume 2-Wire transaction."]
    #[inline]
    pub(crate) fn write_twi0_tasks_resume(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write TWI0 TASKS_RESUME, TWI0 TASKS_RESUME reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "TWI0 EVENTS_STOPPED: Two-wire stopped.\n\nTWI0 EVENTS_STOPPED: Two-wire stopped."]
    #[inline]
    pub(crate) fn read_twi0_events_stopped(&self) -> MemResult<u32> {
        todo ! ("read TWI0 EVENTS_STOPPED, TWI0 EVENTS_STOPPED reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "TWI0 EVENTS_STOPPED: Two-wire stopped.\n\nTWI0 EVENTS_STOPPED: Two-wire stopped."]
    #[inline]
    pub(crate) fn write_twi0_events_stopped(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write TWI0 EVENTS_STOPPED, TWI0 EVENTS_STOPPED reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "TWI0 EVENTS_TXDSENT: Two-wire finished sending last TXD byte.\n\nTWI0 EVENTS_TXDSENT: Two-wire finished sending last TXD byte."]
    #[inline]
    pub(crate) fn read_twi0_events_txdsent(&self) -> MemResult<u32> {
        todo ! ("read TWI0 EVENTS_TXDSENT, TWI0 EVENTS_TXDSENT reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "TWI0 EVENTS_TXDSENT: Two-wire finished sending last TXD byte.\n\nTWI0 EVENTS_TXDSENT: Two-wire finished sending last TXD byte."]
    #[inline]
    pub(crate) fn write_twi0_events_txdsent(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write TWI0 EVENTS_TXDSENT, TWI0 EVENTS_TXDSENT reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "TWI0 EVENTS_ERROR: Two-wire error detected.\n\nTWI0 EVENTS_ERROR: Two-wire error detected."]
    #[inline]
    pub(crate) fn read_twi0_events_error(&self) -> MemResult<u32> {
        todo ! ("read TWI0 EVENTS_ERROR, TWI0 EVENTS_ERROR reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "TWI0 EVENTS_ERROR: Two-wire error detected.\n\nTWI0 EVENTS_ERROR: Two-wire error detected."]
    #[inline]
    pub(crate) fn write_twi0_events_error(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write TWI0 EVENTS_ERROR, TWI0 EVENTS_ERROR reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "TWI0 EVENTS_BB: Two-wire byte boundary.\n\nTWI0 EVENTS_BB: Two-wire byte boundary."]
    #[inline]
    pub(crate) fn read_twi0_events_bb(&self) -> MemResult<u32> {
        todo ! ("read TWI0 EVENTS_BB, TWI0 EVENTS_BB reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "TWI0 EVENTS_BB: Two-wire byte boundary.\n\nTWI0 EVENTS_BB: Two-wire byte boundary."]
    #[inline]
    pub(crate) fn write_twi0_events_bb(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write TWI0 EVENTS_BB, TWI0 EVENTS_BB reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "TWI0 EVENTS_SUSPENDED: Two-wire suspended.\n\nTWI0 EVENTS_SUSPENDED: Two-wire suspended."]
    #[inline]
    pub(crate) fn read_twi0_events_suspended(&self) -> MemResult<u32> {
        todo ! ("read TWI0 EVENTS_SUSPENDED, TWI0 EVENTS_SUSPENDED reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "TWI0 EVENTS_SUSPENDED: Two-wire suspended.\n\nTWI0 EVENTS_SUSPENDED: Two-wire suspended."]
    #[inline]
    pub(crate) fn write_twi0_events_suspended(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write TWI0 EVENTS_SUSPENDED, TWI0 EVENTS_SUSPENDED reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "TWI0 SHORTS BB_SUSPEND: Shortcut between BB event and the SUSPEND task.\n\nTWI0 SHORTS BB_SUSPEND: Shortcut between BB event and the SUSPEND task."]
    #[inline]
    pub(crate) fn read_twi0_shorts_bb_suspend(&self) -> MemResult<bool> {
        todo ! ("read TWI0 SHORTS BB_SUSPEND, TWI0 SHORTS BB_SUSPEND reset value false")
    }
    #[doc = "TWI0 SHORTS BB_SUSPEND: Shortcut between BB event and the SUSPEND task.\n\nTWI0 SHORTS BB_SUSPEND: Shortcut between BB event and the SUSPEND task."]
    #[inline]
    pub(crate) fn write_twi0_shorts_bb_suspend(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TWI0 SHORTS BB_SUSPEND, TWI0 SHORTS BB_SUSPEND reset value false")
    }
    #[doc = "TWI0 SHORTS BB_STOP: Shortcut between BB event and the STOP task.\n\nTWI0 SHORTS BB_STOP: Shortcut between BB event and the STOP task."]
    #[inline]
    pub(crate) fn read_twi0_shorts_bb_stop(&self) -> MemResult<bool> {
        todo!("read TWI0 SHORTS BB_STOP, TWI0 SHORTS BB_STOP reset value false")
    }
    #[doc = "TWI0 SHORTS BB_STOP: Shortcut between BB event and the STOP task.\n\nTWI0 SHORTS BB_STOP: Shortcut between BB event and the STOP task."]
    #[inline]
    pub(crate) fn write_twi0_shorts_bb_stop(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write TWI0 SHORTS BB_STOP, TWI0 SHORTS BB_STOP reset value false"
        )
    }
    #[doc = "TWI0 ERRORSRC OVERRUN: Byte received in RXD register before read of the last received byte (data loss).\n\nTWI0 ERRORSRC OVERRUN: Byte received in RXD register before read of the last received byte (data loss)."]
    #[inline]
    pub(crate) fn read_twi0_errorsrc_overrun(&self) -> MemResult<bool> {
        todo ! ("read TWI0 ERRORSRC OVERRUN, TWI0 ERRORSRC OVERRUN reset value false")
    }
    #[doc = "TWI0 ERRORSRC OVERRUN: Byte received in RXD register before read of the last received byte (data loss).\n\nTWI0 ERRORSRC OVERRUN: Byte received in RXD register before read of the last received byte (data loss)."]
    #[inline]
    pub(crate) fn write_twi0_errorsrc_overrun(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TWI0 ERRORSRC OVERRUN, TWI0 ERRORSRC OVERRUN reset value false")
    }
    #[doc = "TWI0 ERRORSRC ANACK: NACK received after sending the address.\n\nTWI0 ERRORSRC ANACK: NACK received after sending the address."]
    #[inline]
    pub(crate) fn read_twi0_errorsrc_anack(&self) -> MemResult<bool> {
        todo!("read TWI0 ERRORSRC ANACK, TWI0 ERRORSRC ANACK reset value false")
    }
    #[doc = "TWI0 ERRORSRC ANACK: NACK received after sending the address.\n\nTWI0 ERRORSRC ANACK: NACK received after sending the address."]
    #[inline]
    pub(crate) fn write_twi0_errorsrc_anack(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write TWI0 ERRORSRC ANACK, TWI0 ERRORSRC ANACK reset value false"
        )
    }
    #[doc = "TWI0 ERRORSRC DNACK: NACK received after sending a data byte.\n\nTWI0 ERRORSRC DNACK: NACK received after sending a data byte."]
    #[inline]
    pub(crate) fn read_twi0_errorsrc_dnack(&self) -> MemResult<bool> {
        todo!("read TWI0 ERRORSRC DNACK, TWI0 ERRORSRC DNACK reset value false")
    }
    #[doc = "TWI0 ERRORSRC DNACK: NACK received after sending a data byte.\n\nTWI0 ERRORSRC DNACK: NACK received after sending a data byte."]
    #[inline]
    pub(crate) fn write_twi0_errorsrc_dnack(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write TWI0 ERRORSRC DNACK, TWI0 ERRORSRC DNACK reset value false"
        )
    }
    #[doc = "TWI0 ADDRESS ADDRESS: Two-wire address.\n\nTWI0 ADDRESS ADDRESS: Two-wire address."]
    #[inline]
    pub(crate) fn read_twi0_address_address(&self) -> MemResult<u8> {
        todo ! ("read TWI0 ADDRESS ADDRESS, TWI0 ADDRESS ADDRESS reset value 0x00 mask 0x7f")
    }
    #[doc = "TWI0 ADDRESS ADDRESS: Two-wire address.\n\nTWI0 ADDRESS ADDRESS: Two-wire address."]
    #[inline]
    pub(crate) fn write_twi0_address_address(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write TWI0 ADDRESS ADDRESS, TWI0 ADDRESS ADDRESS reset value 0x00 mask 0x7f")
    }
    #[doc = "SPIS1 TASKS_ACQUIRE: Acquire SPI semaphore."]
    #[inline]
    pub(crate) fn write_spis1_tasks_acquire(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write SPIS1 TASKS_ACQUIRE reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "SPIS1 TASKS_RELEASE: Release SPI semaphore."]
    #[inline]
    pub(crate) fn write_spis1_tasks_release(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write SPIS1 TASKS_RELEASE reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "SPIS1 EVENTS_END: Granted transaction completed."]
    #[inline]
    pub(crate) fn read_spis1_events_end(&self) -> MemResult<u32> {
        todo!("read SPIS1 EVENTS_END reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "SPIS1 EVENTS_END: Granted transaction completed."]
    #[inline]
    pub(crate) fn write_spis1_events_end(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write SPIS1 EVENTS_END reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "SPIS1 EVENTS_ENDRX: End of RXD buffer reached"]
    #[inline]
    pub(crate) fn read_spis1_events_endrx(&self) -> MemResult<u32> {
        todo!(
            "read SPIS1 EVENTS_ENDRX reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "SPIS1 EVENTS_ENDRX: End of RXD buffer reached"]
    #[inline]
    pub(crate) fn write_spis1_events_endrx(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!(
            "write SPIS1 EVENTS_ENDRX reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "SPIS1 EVENTS_ACQUIRED: Semaphore acquired."]
    #[inline]
    pub(crate) fn read_spis1_events_acquired(&self) -> MemResult<u32> {
        todo ! ("read SPIS1 EVENTS_ACQUIRED reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "SPIS1 EVENTS_ACQUIRED: Semaphore acquired."]
    #[inline]
    pub(crate) fn write_spis1_events_acquired(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write SPIS1 EVENTS_ACQUIRED reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "SPIS1 SHORTS END_ACQUIRE: Shortcut between END event and the ACQUIRE task."]
    #[inline]
    pub(crate) fn read_spis1_shorts_end_acquire(&self) -> MemResult<bool> {
        todo!("read SPIS1 SHORTS END_ACQUIRE reset value false")
    }
    #[doc = "SPIS1 SHORTS END_ACQUIRE: Shortcut between END event and the ACQUIRE task."]
    #[inline]
    pub(crate) fn write_spis1_shorts_end_acquire(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SPIS1 SHORTS END_ACQUIRE reset value false")
    }
    #[doc = "SPIS1 INTENSET END: Enable interrupt on END event."]
    #[inline]
    pub(crate) fn read_spis1_intenset_end(&self) -> MemResult<bool> {
        todo!("read SPIS1 INTENSET END reset value false")
    }
    #[doc = "SPIS1 INTENSET END: Enable interrupt on END event."]
    #[inline]
    pub(crate) fn write_spis1_intenset_end(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SPIS1 INTENSET END reset value false")
    }
    #[doc = "SPIS1 INTENSET ENDRX: enable interrupt on ENDRX event."]
    #[inline]
    pub(crate) fn read_spis1_intenset_endrx(&self) -> MemResult<bool> {
        todo!("read SPIS1 INTENSET ENDRX reset value false")
    }
    #[doc = "SPIS1 INTENSET ENDRX: enable interrupt on ENDRX event."]
    #[inline]
    pub(crate) fn write_spis1_intenset_endrx(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SPIS1 INTENSET ENDRX reset value false")
    }
    #[doc = "SPIS1 INTENSET ACQUIRED: Enable interrupt on ACQUIRED event."]
    #[inline]
    pub(crate) fn read_spis1_intenset_acquired(&self) -> MemResult<bool> {
        todo!("read SPIS1 INTENSET ACQUIRED reset value false")
    }
    #[doc = "SPIS1 INTENSET ACQUIRED: Enable interrupt on ACQUIRED event."]
    #[inline]
    pub(crate) fn write_spis1_intenset_acquired(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SPIS1 INTENSET ACQUIRED reset value false")
    }
    #[doc = "SPIS1 INTENCLR END: Disable interrupt on END event."]
    #[inline]
    pub(crate) fn read_spis1_intenclr_end(&self) -> MemResult<bool> {
        todo!("read SPIS1 INTENCLR END reset value false")
    }
    #[doc = "SPIS1 INTENCLR END: Disable interrupt on END event."]
    #[inline]
    pub(crate) fn write_spis1_intenclr_end(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SPIS1 INTENCLR END reset value false")
    }
    #[doc = "SPIS1 INTENCLR ENDRX: Disable interrupt on ENDRX event."]
    #[inline]
    pub(crate) fn read_spis1_intenclr_endrx(&self) -> MemResult<bool> {
        todo!("read SPIS1 INTENCLR ENDRX reset value false")
    }
    #[doc = "SPIS1 INTENCLR ENDRX: Disable interrupt on ENDRX event."]
    #[inline]
    pub(crate) fn write_spis1_intenclr_endrx(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SPIS1 INTENCLR ENDRX reset value false")
    }
    #[doc = "SPIS1 INTENCLR ACQUIRED: Disable interrupt on ACQUIRED event."]
    #[inline]
    pub(crate) fn read_spis1_intenclr_acquired(&self) -> MemResult<bool> {
        todo!("read SPIS1 INTENCLR ACQUIRED reset value false")
    }
    #[doc = "SPIS1 INTENCLR ACQUIRED: Disable interrupt on ACQUIRED event."]
    #[inline]
    pub(crate) fn write_spis1_intenclr_acquired(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SPIS1 INTENCLR ACQUIRED reset value false")
    }
    #[doc = "SPIS1 SEMSTAT SEMSTAT: Semaphore status."]
    #[inline]
    pub(crate) fn read_spis1_semstat_semstat(&self) -> MemResult<u8> {
        todo!("read SPIS1 SEMSTAT SEMSTAT reset value 0x01 mask 0x03")
    }
    #[doc = "SPIS1 STATUS OVERREAD: TX buffer overread detected, and prevented."]
    #[inline]
    pub(crate) fn read_spis1_status_overread(&self) -> MemResult<bool> {
        todo!("read SPIS1 STATUS OVERREAD reset value false")
    }
    #[doc = "SPIS1 STATUS OVERREAD: TX buffer overread detected, and prevented."]
    #[inline]
    pub(crate) fn write_spis1_status_overread(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SPIS1 STATUS OVERREAD reset value false")
    }
    #[doc = "SPIS1 STATUS OVERFLOW: RX buffer overflow detected, and prevented."]
    #[inline]
    pub(crate) fn read_spis1_status_overflow(&self) -> MemResult<bool> {
        todo!("read SPIS1 STATUS OVERFLOW reset value false")
    }
    #[doc = "SPIS1 STATUS OVERFLOW: RX buffer overflow detected, and prevented."]
    #[inline]
    pub(crate) fn write_spis1_status_overflow(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SPIS1 STATUS OVERFLOW reset value false")
    }
    #[doc = "SPIS1 ENABLE ENABLE: Enable or disable SPIS."]
    #[inline]
    pub(crate) fn read_spis1_enable_enable(&self) -> MemResult<u8> {
        todo!("read SPIS1 ENABLE ENABLE reset value 0x00 mask 0x07")
    }
    #[doc = "SPIS1 ENABLE ENABLE: Enable or disable SPIS."]
    #[inline]
    pub(crate) fn write_spis1_enable_enable(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write SPIS1 ENABLE ENABLE reset value 0x00 mask 0x07")
    }
    #[doc = "SPIS1 PSELSCK: Pin select for SCK."]
    #[inline]
    pub(crate) fn read_spis1_pselsck(&self) -> MemResult<u32> {
        todo!(
            "read SPIS1 PSELSCK reset value 0xffffffff mask 0xffffffffffffffff"
        )
    }
    #[doc = "SPIS1 PSELSCK: Pin select for SCK."]
    #[inline]
    pub(crate) fn write_spis1_pselsck(&mut self, _value: u32) -> MemResult<()> {
        todo ! ("write SPIS1 PSELSCK reset value 0xffffffff mask 0xffffffffffffffff")
    }
    #[doc = "SPIS1 PSELMISO: Pin select for MISO."]
    #[inline]
    pub(crate) fn read_spis1_pselmiso(&self) -> MemResult<u32> {
        todo ! ("read SPIS1 PSELMISO reset value 0xffffffff mask 0xffffffffffffffff")
    }
    #[doc = "SPIS1 PSELMISO: Pin select for MISO."]
    #[inline]
    pub(crate) fn write_spis1_pselmiso(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write SPIS1 PSELMISO reset value 0xffffffff mask 0xffffffffffffffff")
    }
    #[doc = "SPIS1 PSELMOSI: Pin select for MOSI."]
    #[inline]
    pub(crate) fn read_spis1_pselmosi(&self) -> MemResult<u32> {
        todo ! ("read SPIS1 PSELMOSI reset value 0xffffffff mask 0xffffffffffffffff")
    }
    #[doc = "SPIS1 PSELMOSI: Pin select for MOSI."]
    #[inline]
    pub(crate) fn write_spis1_pselmosi(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write SPIS1 PSELMOSI reset value 0xffffffff mask 0xffffffffffffffff")
    }
    #[doc = "SPIS1 PSELCSN: Pin select for CSN."]
    #[inline]
    pub(crate) fn read_spis1_pselcsn(&self) -> MemResult<u32> {
        todo!(
            "read SPIS1 PSELCSN reset value 0xffffffff mask 0xffffffffffffffff"
        )
    }
    #[doc = "SPIS1 PSELCSN: Pin select for CSN."]
    #[inline]
    pub(crate) fn write_spis1_pselcsn(&mut self, _value: u32) -> MemResult<()> {
        todo ! ("write SPIS1 PSELCSN reset value 0xffffffff mask 0xffffffffffffffff")
    }
    #[doc = "SPIS1 RXDPTR: RX data pointer."]
    #[inline]
    pub(crate) fn read_spis1_rxdptr(&self) -> MemResult<u32> {
        todo!("read SPIS1 RXDPTR reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "SPIS1 RXDPTR: RX data pointer."]
    #[inline]
    pub(crate) fn write_spis1_rxdptr(&mut self, _value: u32) -> MemResult<()> {
        todo!("write SPIS1 RXDPTR reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "SPIS1 MAXRX MAXRX: Maximum number of bytes in the receive buffer."]
    #[inline]
    pub(crate) fn read_spis1_maxrx_maxrx(&self) -> MemResult<u8> {
        todo!("read SPIS1 MAXRX MAXRX reset value 0x00 mask 0xff")
    }
    #[doc = "SPIS1 MAXRX MAXRX: Maximum number of bytes in the receive buffer."]
    #[inline]
    pub(crate) fn write_spis1_maxrx_maxrx(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write SPIS1 MAXRX MAXRX reset value 0x00 mask 0xff")
    }
    #[doc = "SPIS1 AMOUNTRX AMOUNTRX: Number of bytes received in last granted transaction."]
    #[inline]
    pub(crate) fn read_spis1_amountrx_amountrx(&self) -> MemResult<u8> {
        todo!("read SPIS1 AMOUNTRX AMOUNTRX reset value 0x00 mask 0xff")
    }
    #[doc = "SPIS1 TXDPTR: TX data pointer."]
    #[inline]
    pub(crate) fn read_spis1_txdptr(&self) -> MemResult<u32> {
        todo!("read SPIS1 TXDPTR reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "SPIS1 TXDPTR: TX data pointer."]
    #[inline]
    pub(crate) fn write_spis1_txdptr(&mut self, _value: u32) -> MemResult<()> {
        todo!("write SPIS1 TXDPTR reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "SPIS1 MAXTX MAXTX: Maximum number of bytes in the transmit buffer."]
    #[inline]
    pub(crate) fn read_spis1_maxtx_maxtx(&self) -> MemResult<u8> {
        todo!("read SPIS1 MAXTX MAXTX reset value 0x00 mask 0xff")
    }
    #[doc = "SPIS1 MAXTX MAXTX: Maximum number of bytes in the transmit buffer."]
    #[inline]
    pub(crate) fn write_spis1_maxtx_maxtx(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write SPIS1 MAXTX MAXTX reset value 0x00 mask 0xff")
    }
    #[doc = "SPIS1 AMOUNTTX AMOUNTTX: Number of bytes transmitted in last granted transaction."]
    #[inline]
    pub(crate) fn read_spis1_amounttx_amounttx(&self) -> MemResult<u8> {
        todo!("read SPIS1 AMOUNTTX AMOUNTTX reset value 0x00 mask 0xff")
    }
    #[doc = "SPIS1 CONFIG ORDER: Bit order."]
    #[inline]
    pub(crate) fn read_spis1_config_order(&self) -> MemResult<bool> {
        todo!("read SPIS1 CONFIG ORDER reset value false")
    }
    #[doc = "SPIS1 CONFIG ORDER: Bit order."]
    #[inline]
    pub(crate) fn write_spis1_config_order(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SPIS1 CONFIG ORDER reset value false")
    }
    #[doc = "SPIS1 CONFIG CPHA: Serial clock (SCK) phase."]
    #[inline]
    pub(crate) fn read_spis1_config_cpha(&self) -> MemResult<bool> {
        todo!("read SPIS1 CONFIG CPHA reset value false")
    }
    #[doc = "SPIS1 CONFIG CPHA: Serial clock (SCK) phase."]
    #[inline]
    pub(crate) fn write_spis1_config_cpha(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SPIS1 CONFIG CPHA reset value false")
    }
    #[doc = "SPIS1 CONFIG CPOL: Serial clock (SCK) polarity."]
    #[inline]
    pub(crate) fn read_spis1_config_cpol(&self) -> MemResult<bool> {
        todo!("read SPIS1 CONFIG CPOL reset value false")
    }
    #[doc = "SPIS1 CONFIG CPOL: Serial clock (SCK) polarity."]
    #[inline]
    pub(crate) fn write_spis1_config_cpol(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SPIS1 CONFIG CPOL reset value false")
    }
    #[doc = "SPIS1 DEF DEF: Default character."]
    #[inline]
    pub(crate) fn read_spis1_def_def(&self) -> MemResult<u8> {
        todo!("read SPIS1 DEF DEF reset value 0x00 mask 0xff")
    }
    #[doc = "SPIS1 DEF DEF: Default character."]
    #[inline]
    pub(crate) fn write_spis1_def_def(&mut self, _value: u8) -> MemResult<()> {
        todo!("write SPIS1 DEF DEF reset value 0x00 mask 0xff")
    }
    #[doc = "SPIS1 ORC ORC: Over-read character."]
    #[inline]
    pub(crate) fn read_spis1_orc_orc(&self) -> MemResult<u8> {
        todo!("read SPIS1 ORC ORC reset value 0x00 mask 0xff")
    }
    #[doc = "SPIS1 ORC ORC: Over-read character."]
    #[inline]
    pub(crate) fn write_spis1_orc_orc(&mut self, _value: u8) -> MemResult<()> {
        todo!("write SPIS1 ORC ORC reset value 0x00 mask 0xff")
    }
    #[doc = "SPIS1 POWER POWER: Peripheral power control."]
    #[inline]
    pub(crate) fn read_spis1_power_power(&self) -> MemResult<bool> {
        todo!("read SPIS1 POWER POWER reset value false")
    }
    #[doc = "SPIS1 POWER POWER: Peripheral power control."]
    #[inline]
    pub(crate) fn write_spis1_power_power(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SPIS1 POWER POWER reset value false")
    }
    #[doc = "GPIOTE TASKS_OUT[%s]: Tasks asssociated with GPIOTE channels."]
    #[inline]
    pub(crate) fn write_gpiote_tasks_outn(
        &mut self,
        _dim: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write GPIOTE TASKS_OUT[%s] reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "GPIOTE EVENTS_IN[%s]: Tasks asssociated with GPIOTE channels."]
    #[inline]
    pub(crate) fn read_gpiote_events_inn(&self, _dim: usize) -> MemResult<u32> {
        todo ! ("read GPIOTE EVENTS_IN[%s] reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "GPIOTE EVENTS_IN[%s]: Tasks asssociated with GPIOTE channels."]
    #[inline]
    pub(crate) fn write_gpiote_events_inn(
        &mut self,
        _dim: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write GPIOTE EVENTS_IN[%s] reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "GPIOTE EVENTS_PORT: Event generated from multiple pins."]
    #[inline]
    pub(crate) fn read_gpiote_events_port(&self) -> MemResult<u32> {
        todo!(
            "read GPIOTE EVENTS_PORT reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "GPIOTE EVENTS_PORT: Event generated from multiple pins."]
    #[inline]
    pub(crate) fn write_gpiote_events_port(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!(
            "write GPIOTE EVENTS_PORT reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "GPIOTE INTENSET IN0: Enable interrupt on IN[0] event."]
    #[inline]
    pub(crate) fn read_gpiote_intenset_in0(&self) -> MemResult<bool> {
        todo!("read GPIOTE INTENSET IN0 reset value false")
    }
    #[doc = "GPIOTE INTENSET IN0: Enable interrupt on IN[0] event."]
    #[inline]
    pub(crate) fn write_gpiote_intenset_in0(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write GPIOTE INTENSET IN0 reset value false")
    }
    #[doc = "GPIOTE INTENSET IN1: Enable interrupt on IN[1] event."]
    #[inline]
    pub(crate) fn read_gpiote_intenset_in1(&self) -> MemResult<bool> {
        todo!("read GPIOTE INTENSET IN1 reset value false")
    }
    #[doc = "GPIOTE INTENSET IN1: Enable interrupt on IN[1] event."]
    #[inline]
    pub(crate) fn write_gpiote_intenset_in1(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write GPIOTE INTENSET IN1 reset value false")
    }
    #[doc = "GPIOTE INTENSET IN2: Enable interrupt on IN[2] event."]
    #[inline]
    pub(crate) fn read_gpiote_intenset_in2(&self) -> MemResult<bool> {
        todo!("read GPIOTE INTENSET IN2 reset value false")
    }
    #[doc = "GPIOTE INTENSET IN2: Enable interrupt on IN[2] event."]
    #[inline]
    pub(crate) fn write_gpiote_intenset_in2(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write GPIOTE INTENSET IN2 reset value false")
    }
    #[doc = "GPIOTE INTENSET IN3: Enable interrupt on IN[3] event."]
    #[inline]
    pub(crate) fn read_gpiote_intenset_in3(&self) -> MemResult<bool> {
        todo!("read GPIOTE INTENSET IN3 reset value false")
    }
    #[doc = "GPIOTE INTENSET IN3: Enable interrupt on IN[3] event."]
    #[inline]
    pub(crate) fn write_gpiote_intenset_in3(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write GPIOTE INTENSET IN3 reset value false")
    }
    #[doc = "GPIOTE INTENSET PORT: Enable interrupt on PORT event."]
    #[inline]
    pub(crate) fn read_gpiote_intenset_port(&self) -> MemResult<bool> {
        todo!("read GPIOTE INTENSET PORT reset value false")
    }
    #[doc = "GPIOTE INTENSET PORT: Enable interrupt on PORT event."]
    #[inline]
    pub(crate) fn write_gpiote_intenset_port(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write GPIOTE INTENSET PORT reset value false")
    }
    #[doc = "GPIOTE INTENCLR IN0: Disable interrupt on IN[0] event."]
    #[inline]
    pub(crate) fn read_gpiote_intenclr_in0(&self) -> MemResult<bool> {
        todo!("read GPIOTE INTENCLR IN0 reset value false")
    }
    #[doc = "GPIOTE INTENCLR IN0: Disable interrupt on IN[0] event."]
    #[inline]
    pub(crate) fn write_gpiote_intenclr_in0(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write GPIOTE INTENCLR IN0 reset value false")
    }
    #[doc = "GPIOTE INTENCLR IN1: Disable interrupt on IN[1] event."]
    #[inline]
    pub(crate) fn read_gpiote_intenclr_in1(&self) -> MemResult<bool> {
        todo!("read GPIOTE INTENCLR IN1 reset value false")
    }
    #[doc = "GPIOTE INTENCLR IN1: Disable interrupt on IN[1] event."]
    #[inline]
    pub(crate) fn write_gpiote_intenclr_in1(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write GPIOTE INTENCLR IN1 reset value false")
    }
    #[doc = "GPIOTE INTENCLR IN2: Disable interrupt on IN[2] event."]
    #[inline]
    pub(crate) fn read_gpiote_intenclr_in2(&self) -> MemResult<bool> {
        todo!("read GPIOTE INTENCLR IN2 reset value false")
    }
    #[doc = "GPIOTE INTENCLR IN2: Disable interrupt on IN[2] event."]
    #[inline]
    pub(crate) fn write_gpiote_intenclr_in2(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write GPIOTE INTENCLR IN2 reset value false")
    }
    #[doc = "GPIOTE INTENCLR IN3: Disable interrupt on IN[3] event."]
    #[inline]
    pub(crate) fn read_gpiote_intenclr_in3(&self) -> MemResult<bool> {
        todo!("read GPIOTE INTENCLR IN3 reset value false")
    }
    #[doc = "GPIOTE INTENCLR IN3: Disable interrupt on IN[3] event."]
    #[inline]
    pub(crate) fn write_gpiote_intenclr_in3(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write GPIOTE INTENCLR IN3 reset value false")
    }
    #[doc = "GPIOTE INTENCLR PORT: Disable interrupt on PORT event."]
    #[inline]
    pub(crate) fn read_gpiote_intenclr_port(&self) -> MemResult<bool> {
        todo!("read GPIOTE INTENCLR PORT reset value false")
    }
    #[doc = "GPIOTE INTENCLR PORT: Disable interrupt on PORT event."]
    #[inline]
    pub(crate) fn write_gpiote_intenclr_port(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write GPIOTE INTENCLR PORT reset value false")
    }
    #[doc = "GPIOTE CONFIG[%s] MODE: Mode"]
    #[inline]
    pub(crate) fn read_gpiote_confign_mode(
        &self,
        _dim: usize,
    ) -> MemResult<u8> {
        todo!("read GPIOTE CONFIG[%s] MODE reset value 0x00 mask 0x03")
    }
    #[doc = "GPIOTE CONFIG[%s] MODE: Mode"]
    #[inline]
    pub(crate) fn write_gpiote_confign_mode(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write GPIOTE CONFIG[%s] MODE reset value 0x00 mask 0x03")
    }
    #[doc = "GPIOTE CONFIG[%s] PSEL: Pin select."]
    #[inline]
    pub(crate) fn read_gpiote_confign_psel(
        &self,
        _dim: usize,
    ) -> MemResult<u8> {
        todo!("read GPIOTE CONFIG[%s] PSEL reset value 0x00 mask 0x1f")
    }
    #[doc = "GPIOTE CONFIG[%s] PSEL: Pin select."]
    #[inline]
    pub(crate) fn write_gpiote_confign_psel(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write GPIOTE CONFIG[%s] PSEL reset value 0x00 mask 0x1f")
    }
    #[doc = "GPIOTE CONFIG[%s] POLARITY: Effects on output when in Task mode, or events on input that generates an event."]
    #[inline]
    pub(crate) fn read_gpiote_confign_polarity(
        &self,
        _dim: usize,
    ) -> MemResult<u8> {
        todo!("read GPIOTE CONFIG[%s] POLARITY reset value 0x00 mask 0x03")
    }
    #[doc = "GPIOTE CONFIG[%s] POLARITY: Effects on output when in Task mode, or events on input that generates an event."]
    #[inline]
    pub(crate) fn write_gpiote_confign_polarity(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write GPIOTE CONFIG[%s] POLARITY reset value 0x00 mask 0x03")
    }
    #[doc = "GPIOTE CONFIG[%s] OUTINIT: Initial value of the output when the GPIOTE channel is configured as a Task."]
    #[inline]
    pub(crate) fn read_gpiote_confign_outinit(
        &self,
        _dim: usize,
    ) -> MemResult<bool> {
        todo!("read GPIOTE CONFIG[%s] OUTINIT reset value false")
    }
    #[doc = "GPIOTE CONFIG[%s] OUTINIT: Initial value of the output when the GPIOTE channel is configured as a Task."]
    #[inline]
    pub(crate) fn write_gpiote_confign_outinit(
        &mut self,
        _dim: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write GPIOTE CONFIG[%s] OUTINIT reset value false")
    }
    #[doc = "GPIOTE POWER POWER: Peripheral power control."]
    #[inline]
    pub(crate) fn read_gpiote_power_power(&self) -> MemResult<bool> {
        todo!("read GPIOTE POWER POWER reset value false")
    }
    #[doc = "GPIOTE POWER POWER: Peripheral power control."]
    #[inline]
    pub(crate) fn write_gpiote_power_power(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write GPIOTE POWER POWER reset value false")
    }
    #[doc = "ADC TASKS_START: Start an ADC conversion."]
    #[inline]
    pub(crate) fn write_adc_tasks_start(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write ADC TASKS_START reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "ADC TASKS_STOP: Stop ADC."]
    #[inline]
    pub(crate) fn write_adc_tasks_stop(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write ADC TASKS_STOP reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "ADC EVENTS_END: ADC conversion complete."]
    #[inline]
    pub(crate) fn read_adc_events_end(&self) -> MemResult<u32> {
        todo!("read ADC EVENTS_END reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "ADC EVENTS_END: ADC conversion complete."]
    #[inline]
    pub(crate) fn write_adc_events_end(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write ADC EVENTS_END reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "ADC INTENSET END: Enable interrupt on END event."]
    #[inline]
    pub(crate) fn read_adc_intenset_end(&self) -> MemResult<bool> {
        todo!("read ADC INTENSET END reset value false")
    }
    #[doc = "ADC INTENSET END: Enable interrupt on END event."]
    #[inline]
    pub(crate) fn write_adc_intenset_end(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ADC INTENSET END reset value false")
    }
    #[doc = "ADC INTENCLR END: Disable interrupt on END event."]
    #[inline]
    pub(crate) fn read_adc_intenclr_end(&self) -> MemResult<bool> {
        todo!("read ADC INTENCLR END reset value false")
    }
    #[doc = "ADC INTENCLR END: Disable interrupt on END event."]
    #[inline]
    pub(crate) fn write_adc_intenclr_end(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ADC INTENCLR END reset value false")
    }
    #[doc = "ADC BUSY BUSY: ADC busy register."]
    #[inline]
    pub(crate) fn read_adc_busy_busy(&self) -> MemResult<bool> {
        todo!("read ADC BUSY BUSY reset value false")
    }
    #[doc = "ADC ENABLE ENABLE: ADC enable."]
    #[inline]
    pub(crate) fn read_adc_enable_enable(&self) -> MemResult<u8> {
        todo!("read ADC ENABLE ENABLE reset value 0x00 mask 0x03")
    }
    #[doc = "ADC ENABLE ENABLE: ADC enable."]
    #[inline]
    pub(crate) fn write_adc_enable_enable(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write ADC ENABLE ENABLE reset value 0x00 mask 0x03")
    }
    #[doc = "ADC CONFIG RES: ADC resolution."]
    #[inline]
    pub(crate) fn read_adc_config_res(&self) -> MemResult<u8> {
        todo!("read ADC CONFIG RES reset value 0x00 mask 0x03")
    }
    #[doc = "ADC CONFIG RES: ADC resolution."]
    #[inline]
    pub(crate) fn write_adc_config_res(&mut self, _value: u8) -> MemResult<()> {
        todo!("write ADC CONFIG RES reset value 0x00 mask 0x03")
    }
    #[doc = "ADC CONFIG INPSEL: ADC input selection."]
    #[inline]
    pub(crate) fn read_adc_config_inpsel(&self) -> MemResult<u8> {
        todo!("read ADC CONFIG INPSEL reset value 0x06 mask 0x07")
    }
    #[doc = "ADC CONFIG INPSEL: ADC input selection."]
    #[inline]
    pub(crate) fn write_adc_config_inpsel(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write ADC CONFIG INPSEL reset value 0x06 mask 0x07")
    }
    #[doc = "ADC CONFIG REFSEL: ADC reference selection."]
    #[inline]
    pub(crate) fn read_adc_config_refsel(&self) -> MemResult<u8> {
        todo!("read ADC CONFIG REFSEL reset value 0x00 mask 0x03")
    }
    #[doc = "ADC CONFIG REFSEL: ADC reference selection."]
    #[inline]
    pub(crate) fn write_adc_config_refsel(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write ADC CONFIG REFSEL reset value 0x00 mask 0x03")
    }
    #[doc = "ADC CONFIG PSEL: ADC analog pin selection."]
    #[inline]
    pub(crate) fn read_adc_config_psel(&self) -> MemResult<u8> {
        todo!("read ADC CONFIG PSEL reset value 0x00 mask 0xff")
    }
    #[doc = "ADC CONFIG PSEL: ADC analog pin selection."]
    #[inline]
    pub(crate) fn write_adc_config_psel(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write ADC CONFIG PSEL reset value 0x00 mask 0xff")
    }
    #[doc = "ADC CONFIG EXTREFSEL: ADC external reference pin selection."]
    #[inline]
    pub(crate) fn read_adc_config_extrefsel(&self) -> MemResult<u8> {
        todo!("read ADC CONFIG EXTREFSEL reset value 0x00 mask 0x03")
    }
    #[doc = "ADC CONFIG EXTREFSEL: ADC external reference pin selection."]
    #[inline]
    pub(crate) fn write_adc_config_extrefsel(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write ADC CONFIG EXTREFSEL reset value 0x00 mask 0x03")
    }
    #[doc = "ADC RESULT RESULT: Result of ADC conversion."]
    #[inline]
    pub(crate) fn read_adc_result_result(&self) -> MemResult<u16> {
        todo!("read ADC RESULT RESULT reset value 0x00 mask 0x3ff")
    }
    #[doc = "ADC POWER POWER: Peripheral power control."]
    #[inline]
    pub(crate) fn read_adc_power_power(&self) -> MemResult<bool> {
        todo!("read ADC POWER POWER reset value false")
    }
    #[doc = "ADC POWER POWER: Peripheral power control."]
    #[inline]
    pub(crate) fn write_adc_power_power(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ADC POWER POWER reset value false")
    }
    #[doc = "TIMER0 TASKS_START: Start Timer.\n\nTIMER0 TASKS_START: Start Timer.\n\nTIMER0 TASKS_START: Start Timer."]
    #[inline]
    pub(crate) fn write_timer0_tasks_start(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write TIMER0 TASKS_START, TIMER0 TASKS_START, TIMER0 TASKS_START reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "TIMER0 TASKS_STOP: Stop Timer.\n\nTIMER0 TASKS_STOP: Stop Timer.\n\nTIMER0 TASKS_STOP: Stop Timer."]
    #[inline]
    pub(crate) fn write_timer0_tasks_stop(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write TIMER0 TASKS_STOP, TIMER0 TASKS_STOP, TIMER0 TASKS_STOP reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "TIMER0 TASKS_COUNT: Increment Timer (In counter mode).\n\nTIMER0 TASKS_COUNT: Increment Timer (In counter mode).\n\nTIMER0 TASKS_COUNT: Increment Timer (In counter mode)."]
    #[inline]
    pub(crate) fn write_timer0_tasks_count(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write TIMER0 TASKS_COUNT, TIMER0 TASKS_COUNT, TIMER0 TASKS_COUNT reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "TIMER0 TASKS_CLEAR: Clear timer.\n\nTIMER0 TASKS_CLEAR: Clear timer.\n\nTIMER0 TASKS_CLEAR: Clear timer."]
    #[inline]
    pub(crate) fn write_timer0_tasks_clear(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write TIMER0 TASKS_CLEAR, TIMER0 TASKS_CLEAR, TIMER0 TASKS_CLEAR reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "TIMER0 TASKS_SHUTDOWN: Shutdown timer.\n\nTIMER0 TASKS_SHUTDOWN: Shutdown timer.\n\nTIMER0 TASKS_SHUTDOWN: Shutdown timer."]
    #[inline]
    pub(crate) fn write_timer0_tasks_shutdown(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write TIMER0 TASKS_SHUTDOWN, TIMER0 TASKS_SHUTDOWN, TIMER0 TASKS_SHUTDOWN reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "TIMER0 TASKS_CAPTURE[%s]: Capture Timer value to CC[n] registers.\n\nTIMER0 TASKS_CAPTURE[%s]: Capture Timer value to CC[n] registers.\n\nTIMER0 TASKS_CAPTURE[%s]: Capture Timer value to CC[n] registers."]
    #[inline]
    pub(crate) fn write_timer0_tasks_capturen(
        &mut self,
        _dim: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write TIMER0 TASKS_CAPTURE[%s], TIMER0 TASKS_CAPTURE[%s], TIMER0 TASKS_CAPTURE[%s] reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "TIMER0 EVENTS_COMPARE[%s]: Compare event on CC[n] match.\n\nTIMER0 EVENTS_COMPARE[%s]: Compare event on CC[n] match.\n\nTIMER0 EVENTS_COMPARE[%s]: Compare event on CC[n] match."]
    #[inline]
    pub(crate) fn read_timer0_events_comparen(
        &self,
        _dim: usize,
    ) -> MemResult<u32> {
        todo ! ("read TIMER0 EVENTS_COMPARE[%s], TIMER0 EVENTS_COMPARE[%s], TIMER0 EVENTS_COMPARE[%s] reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "TIMER0 EVENTS_COMPARE[%s]: Compare event on CC[n] match.\n\nTIMER0 EVENTS_COMPARE[%s]: Compare event on CC[n] match.\n\nTIMER0 EVENTS_COMPARE[%s]: Compare event on CC[n] match."]
    #[inline]
    pub(crate) fn write_timer0_events_comparen(
        &mut self,
        _dim: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write TIMER0 EVENTS_COMPARE[%s], TIMER0 EVENTS_COMPARE[%s], TIMER0 EVENTS_COMPARE[%s] reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "TIMER0 SHORTS COMPARE0_CLEAR: Shortcut between CC[0] event and the CLEAR task.\n\nTIMER0 SHORTS COMPARE0_CLEAR: Shortcut between CC[0] event and the CLEAR task.\n\nTIMER0 SHORTS COMPARE0_CLEAR: Shortcut between CC[0] event and the CLEAR task."]
    #[inline]
    pub(crate) fn read_timer0_shorts_compare0_clear(&self) -> MemResult<bool> {
        todo ! ("read TIMER0 SHORTS COMPARE0_CLEAR, TIMER0 SHORTS COMPARE0_CLEAR, TIMER0 SHORTS COMPARE0_CLEAR reset value false")
    }
    #[doc = "TIMER0 SHORTS COMPARE0_CLEAR: Shortcut between CC[0] event and the CLEAR task.\n\nTIMER0 SHORTS COMPARE0_CLEAR: Shortcut between CC[0] event and the CLEAR task.\n\nTIMER0 SHORTS COMPARE0_CLEAR: Shortcut between CC[0] event and the CLEAR task."]
    #[inline]
    pub(crate) fn write_timer0_shorts_compare0_clear(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TIMER0 SHORTS COMPARE0_CLEAR, TIMER0 SHORTS COMPARE0_CLEAR, TIMER0 SHORTS COMPARE0_CLEAR reset value false")
    }
    #[doc = "TIMER0 SHORTS COMPARE1_CLEAR: Shortcut between CC[1] event and the CLEAR task.\n\nTIMER0 SHORTS COMPARE1_CLEAR: Shortcut between CC[1] event and the CLEAR task.\n\nTIMER0 SHORTS COMPARE1_CLEAR: Shortcut between CC[1] event and the CLEAR task."]
    #[inline]
    pub(crate) fn read_timer0_shorts_compare1_clear(&self) -> MemResult<bool> {
        todo ! ("read TIMER0 SHORTS COMPARE1_CLEAR, TIMER0 SHORTS COMPARE1_CLEAR, TIMER0 SHORTS COMPARE1_CLEAR reset value false")
    }
    #[doc = "TIMER0 SHORTS COMPARE1_CLEAR: Shortcut between CC[1] event and the CLEAR task.\n\nTIMER0 SHORTS COMPARE1_CLEAR: Shortcut between CC[1] event and the CLEAR task.\n\nTIMER0 SHORTS COMPARE1_CLEAR: Shortcut between CC[1] event and the CLEAR task."]
    #[inline]
    pub(crate) fn write_timer0_shorts_compare1_clear(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TIMER0 SHORTS COMPARE1_CLEAR, TIMER0 SHORTS COMPARE1_CLEAR, TIMER0 SHORTS COMPARE1_CLEAR reset value false")
    }
    #[doc = "TIMER0 SHORTS COMPARE2_CLEAR: Shortcut between CC[2] event and the CLEAR task.\n\nTIMER0 SHORTS COMPARE2_CLEAR: Shortcut between CC[2] event and the CLEAR task.\n\nTIMER0 SHORTS COMPARE2_CLEAR: Shortcut between CC[2] event and the CLEAR task."]
    #[inline]
    pub(crate) fn read_timer0_shorts_compare2_clear(&self) -> MemResult<bool> {
        todo ! ("read TIMER0 SHORTS COMPARE2_CLEAR, TIMER0 SHORTS COMPARE2_CLEAR, TIMER0 SHORTS COMPARE2_CLEAR reset value false")
    }
    #[doc = "TIMER0 SHORTS COMPARE2_CLEAR: Shortcut between CC[2] event and the CLEAR task.\n\nTIMER0 SHORTS COMPARE2_CLEAR: Shortcut between CC[2] event and the CLEAR task.\n\nTIMER0 SHORTS COMPARE2_CLEAR: Shortcut between CC[2] event and the CLEAR task."]
    #[inline]
    pub(crate) fn write_timer0_shorts_compare2_clear(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TIMER0 SHORTS COMPARE2_CLEAR, TIMER0 SHORTS COMPARE2_CLEAR, TIMER0 SHORTS COMPARE2_CLEAR reset value false")
    }
    #[doc = "TIMER0 SHORTS COMPARE3_CLEAR: Shortcut between CC[3] event and the CLEAR task.\n\nTIMER0 SHORTS COMPARE3_CLEAR: Shortcut between CC[3] event and the CLEAR task.\n\nTIMER0 SHORTS COMPARE3_CLEAR: Shortcut between CC[3] event and the CLEAR task."]
    #[inline]
    pub(crate) fn read_timer0_shorts_compare3_clear(&self) -> MemResult<bool> {
        todo ! ("read TIMER0 SHORTS COMPARE3_CLEAR, TIMER0 SHORTS COMPARE3_CLEAR, TIMER0 SHORTS COMPARE3_CLEAR reset value false")
    }
    #[doc = "TIMER0 SHORTS COMPARE3_CLEAR: Shortcut between CC[3] event and the CLEAR task.\n\nTIMER0 SHORTS COMPARE3_CLEAR: Shortcut between CC[3] event and the CLEAR task.\n\nTIMER0 SHORTS COMPARE3_CLEAR: Shortcut between CC[3] event and the CLEAR task."]
    #[inline]
    pub(crate) fn write_timer0_shorts_compare3_clear(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TIMER0 SHORTS COMPARE3_CLEAR, TIMER0 SHORTS COMPARE3_CLEAR, TIMER0 SHORTS COMPARE3_CLEAR reset value false")
    }
    #[doc = "TIMER0 SHORTS COMPARE0_STOP: Shortcut between CC[0] event and the STOP task.\n\nTIMER0 SHORTS COMPARE0_STOP: Shortcut between CC[0] event and the STOP task.\n\nTIMER0 SHORTS COMPARE0_STOP: Shortcut between CC[0] event and the STOP task."]
    #[inline]
    pub(crate) fn read_timer0_shorts_compare0_stop(&self) -> MemResult<bool> {
        todo ! ("read TIMER0 SHORTS COMPARE0_STOP, TIMER0 SHORTS COMPARE0_STOP, TIMER0 SHORTS COMPARE0_STOP reset value false")
    }
    #[doc = "TIMER0 SHORTS COMPARE0_STOP: Shortcut between CC[0] event and the STOP task.\n\nTIMER0 SHORTS COMPARE0_STOP: Shortcut between CC[0] event and the STOP task.\n\nTIMER0 SHORTS COMPARE0_STOP: Shortcut between CC[0] event and the STOP task."]
    #[inline]
    pub(crate) fn write_timer0_shorts_compare0_stop(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TIMER0 SHORTS COMPARE0_STOP, TIMER0 SHORTS COMPARE0_STOP, TIMER0 SHORTS COMPARE0_STOP reset value false")
    }
    #[doc = "TIMER0 SHORTS COMPARE1_STOP: Shortcut between CC[1] event and the STOP task.\n\nTIMER0 SHORTS COMPARE1_STOP: Shortcut between CC[1] event and the STOP task.\n\nTIMER0 SHORTS COMPARE1_STOP: Shortcut between CC[1] event and the STOP task."]
    #[inline]
    pub(crate) fn read_timer0_shorts_compare1_stop(&self) -> MemResult<bool> {
        todo ! ("read TIMER0 SHORTS COMPARE1_STOP, TIMER0 SHORTS COMPARE1_STOP, TIMER0 SHORTS COMPARE1_STOP reset value false")
    }
    #[doc = "TIMER0 SHORTS COMPARE1_STOP: Shortcut between CC[1] event and the STOP task.\n\nTIMER0 SHORTS COMPARE1_STOP: Shortcut between CC[1] event and the STOP task.\n\nTIMER0 SHORTS COMPARE1_STOP: Shortcut between CC[1] event and the STOP task."]
    #[inline]
    pub(crate) fn write_timer0_shorts_compare1_stop(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TIMER0 SHORTS COMPARE1_STOP, TIMER0 SHORTS COMPARE1_STOP, TIMER0 SHORTS COMPARE1_STOP reset value false")
    }
    #[doc = "TIMER0 SHORTS COMPARE2_STOP: Shortcut between CC[2] event and the STOP task.\n\nTIMER0 SHORTS COMPARE2_STOP: Shortcut between CC[2] event and the STOP task.\n\nTIMER0 SHORTS COMPARE2_STOP: Shortcut between CC[2] event and the STOP task."]
    #[inline]
    pub(crate) fn read_timer0_shorts_compare2_stop(&self) -> MemResult<bool> {
        todo ! ("read TIMER0 SHORTS COMPARE2_STOP, TIMER0 SHORTS COMPARE2_STOP, TIMER0 SHORTS COMPARE2_STOP reset value false")
    }
    #[doc = "TIMER0 SHORTS COMPARE2_STOP: Shortcut between CC[2] event and the STOP task.\n\nTIMER0 SHORTS COMPARE2_STOP: Shortcut between CC[2] event and the STOP task.\n\nTIMER0 SHORTS COMPARE2_STOP: Shortcut between CC[2] event and the STOP task."]
    #[inline]
    pub(crate) fn write_timer0_shorts_compare2_stop(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TIMER0 SHORTS COMPARE2_STOP, TIMER0 SHORTS COMPARE2_STOP, TIMER0 SHORTS COMPARE2_STOP reset value false")
    }
    #[doc = "TIMER0 SHORTS COMPARE3_STOP: Shortcut between CC[3] event and the STOP task.\n\nTIMER0 SHORTS COMPARE3_STOP: Shortcut between CC[3] event and the STOP task.\n\nTIMER0 SHORTS COMPARE3_STOP: Shortcut between CC[3] event and the STOP task."]
    #[inline]
    pub(crate) fn read_timer0_shorts_compare3_stop(&self) -> MemResult<bool> {
        todo ! ("read TIMER0 SHORTS COMPARE3_STOP, TIMER0 SHORTS COMPARE3_STOP, TIMER0 SHORTS COMPARE3_STOP reset value false")
    }
    #[doc = "TIMER0 SHORTS COMPARE3_STOP: Shortcut between CC[3] event and the STOP task.\n\nTIMER0 SHORTS COMPARE3_STOP: Shortcut between CC[3] event and the STOP task.\n\nTIMER0 SHORTS COMPARE3_STOP: Shortcut between CC[3] event and the STOP task."]
    #[inline]
    pub(crate) fn write_timer0_shorts_compare3_stop(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TIMER0 SHORTS COMPARE3_STOP, TIMER0 SHORTS COMPARE3_STOP, TIMER0 SHORTS COMPARE3_STOP reset value false")
    }
    #[doc = "TIMER0 INTENSET COMPARE0: Enable interrupt on COMPARE[0]\n\nTIMER0 INTENSET COMPARE0: Enable interrupt on COMPARE[0]\n\nTIMER0 INTENSET COMPARE0: Enable interrupt on COMPARE[0]"]
    #[inline]
    pub(crate) fn read_timer0_intenset_compare0(&self) -> MemResult<bool> {
        todo ! ("read TIMER0 INTENSET COMPARE0, TIMER0 INTENSET COMPARE0, TIMER0 INTENSET COMPARE0 reset value false")
    }
    #[doc = "TIMER0 INTENSET COMPARE0: Enable interrupt on COMPARE[0]\n\nTIMER0 INTENSET COMPARE0: Enable interrupt on COMPARE[0]\n\nTIMER0 INTENSET COMPARE0: Enable interrupt on COMPARE[0]"]
    #[inline]
    pub(crate) fn write_timer0_intenset_compare0(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TIMER0 INTENSET COMPARE0, TIMER0 INTENSET COMPARE0, TIMER0 INTENSET COMPARE0 reset value false")
    }
    #[doc = "TIMER0 INTENSET COMPARE1: Enable interrupt on COMPARE[1]\n\nTIMER0 INTENSET COMPARE1: Enable interrupt on COMPARE[1]\n\nTIMER0 INTENSET COMPARE1: Enable interrupt on COMPARE[1]"]
    #[inline]
    pub(crate) fn read_timer0_intenset_compare1(&self) -> MemResult<bool> {
        todo ! ("read TIMER0 INTENSET COMPARE1, TIMER0 INTENSET COMPARE1, TIMER0 INTENSET COMPARE1 reset value false")
    }
    #[doc = "TIMER0 INTENSET COMPARE1: Enable interrupt on COMPARE[1]\n\nTIMER0 INTENSET COMPARE1: Enable interrupt on COMPARE[1]\n\nTIMER0 INTENSET COMPARE1: Enable interrupt on COMPARE[1]"]
    #[inline]
    pub(crate) fn write_timer0_intenset_compare1(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TIMER0 INTENSET COMPARE1, TIMER0 INTENSET COMPARE1, TIMER0 INTENSET COMPARE1 reset value false")
    }
    #[doc = "TIMER0 INTENSET COMPARE2: Enable interrupt on COMPARE[2]\n\nTIMER0 INTENSET COMPARE2: Enable interrupt on COMPARE[2]\n\nTIMER0 INTENSET COMPARE2: Enable interrupt on COMPARE[2]"]
    #[inline]
    pub(crate) fn read_timer0_intenset_compare2(&self) -> MemResult<bool> {
        todo ! ("read TIMER0 INTENSET COMPARE2, TIMER0 INTENSET COMPARE2, TIMER0 INTENSET COMPARE2 reset value false")
    }
    #[doc = "TIMER0 INTENSET COMPARE2: Enable interrupt on COMPARE[2]\n\nTIMER0 INTENSET COMPARE2: Enable interrupt on COMPARE[2]\n\nTIMER0 INTENSET COMPARE2: Enable interrupt on COMPARE[2]"]
    #[inline]
    pub(crate) fn write_timer0_intenset_compare2(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TIMER0 INTENSET COMPARE2, TIMER0 INTENSET COMPARE2, TIMER0 INTENSET COMPARE2 reset value false")
    }
    #[doc = "TIMER0 INTENSET COMPARE3: Enable interrupt on COMPARE[3]\n\nTIMER0 INTENSET COMPARE3: Enable interrupt on COMPARE[3]\n\nTIMER0 INTENSET COMPARE3: Enable interrupt on COMPARE[3]"]
    #[inline]
    pub(crate) fn read_timer0_intenset_compare3(&self) -> MemResult<bool> {
        todo ! ("read TIMER0 INTENSET COMPARE3, TIMER0 INTENSET COMPARE3, TIMER0 INTENSET COMPARE3 reset value false")
    }
    #[doc = "TIMER0 INTENSET COMPARE3: Enable interrupt on COMPARE[3]\n\nTIMER0 INTENSET COMPARE3: Enable interrupt on COMPARE[3]\n\nTIMER0 INTENSET COMPARE3: Enable interrupt on COMPARE[3]"]
    #[inline]
    pub(crate) fn write_timer0_intenset_compare3(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TIMER0 INTENSET COMPARE3, TIMER0 INTENSET COMPARE3, TIMER0 INTENSET COMPARE3 reset value false")
    }
    #[doc = "TIMER0 INTENCLR COMPARE0: Disable interrupt on COMPARE[0]\n\nTIMER0 INTENCLR COMPARE0: Disable interrupt on COMPARE[0]\n\nTIMER0 INTENCLR COMPARE0: Disable interrupt on COMPARE[0]"]
    #[inline]
    pub(crate) fn read_timer0_intenclr_compare0(&self) -> MemResult<bool> {
        todo ! ("read TIMER0 INTENCLR COMPARE0, TIMER0 INTENCLR COMPARE0, TIMER0 INTENCLR COMPARE0 reset value false")
    }
    #[doc = "TIMER0 INTENCLR COMPARE0: Disable interrupt on COMPARE[0]\n\nTIMER0 INTENCLR COMPARE0: Disable interrupt on COMPARE[0]\n\nTIMER0 INTENCLR COMPARE0: Disable interrupt on COMPARE[0]"]
    #[inline]
    pub(crate) fn write_timer0_intenclr_compare0(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TIMER0 INTENCLR COMPARE0, TIMER0 INTENCLR COMPARE0, TIMER0 INTENCLR COMPARE0 reset value false")
    }
    #[doc = "TIMER0 INTENCLR COMPARE1: Disable interrupt on COMPARE[1]\n\nTIMER0 INTENCLR COMPARE1: Disable interrupt on COMPARE[1]\n\nTIMER0 INTENCLR COMPARE1: Disable interrupt on COMPARE[1]"]
    #[inline]
    pub(crate) fn read_timer0_intenclr_compare1(&self) -> MemResult<bool> {
        todo ! ("read TIMER0 INTENCLR COMPARE1, TIMER0 INTENCLR COMPARE1, TIMER0 INTENCLR COMPARE1 reset value false")
    }
    #[doc = "TIMER0 INTENCLR COMPARE1: Disable interrupt on COMPARE[1]\n\nTIMER0 INTENCLR COMPARE1: Disable interrupt on COMPARE[1]\n\nTIMER0 INTENCLR COMPARE1: Disable interrupt on COMPARE[1]"]
    #[inline]
    pub(crate) fn write_timer0_intenclr_compare1(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TIMER0 INTENCLR COMPARE1, TIMER0 INTENCLR COMPARE1, TIMER0 INTENCLR COMPARE1 reset value false")
    }
    #[doc = "TIMER0 INTENCLR COMPARE2: Disable interrupt on COMPARE[2]\n\nTIMER0 INTENCLR COMPARE2: Disable interrupt on COMPARE[2]\n\nTIMER0 INTENCLR COMPARE2: Disable interrupt on COMPARE[2]"]
    #[inline]
    pub(crate) fn read_timer0_intenclr_compare2(&self) -> MemResult<bool> {
        todo ! ("read TIMER0 INTENCLR COMPARE2, TIMER0 INTENCLR COMPARE2, TIMER0 INTENCLR COMPARE2 reset value false")
    }
    #[doc = "TIMER0 INTENCLR COMPARE2: Disable interrupt on COMPARE[2]\n\nTIMER0 INTENCLR COMPARE2: Disable interrupt on COMPARE[2]\n\nTIMER0 INTENCLR COMPARE2: Disable interrupt on COMPARE[2]"]
    #[inline]
    pub(crate) fn write_timer0_intenclr_compare2(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TIMER0 INTENCLR COMPARE2, TIMER0 INTENCLR COMPARE2, TIMER0 INTENCLR COMPARE2 reset value false")
    }
    #[doc = "TIMER0 INTENCLR COMPARE3: Disable interrupt on COMPARE[3]\n\nTIMER0 INTENCLR COMPARE3: Disable interrupt on COMPARE[3]\n\nTIMER0 INTENCLR COMPARE3: Disable interrupt on COMPARE[3]"]
    #[inline]
    pub(crate) fn read_timer0_intenclr_compare3(&self) -> MemResult<bool> {
        todo ! ("read TIMER0 INTENCLR COMPARE3, TIMER0 INTENCLR COMPARE3, TIMER0 INTENCLR COMPARE3 reset value false")
    }
    #[doc = "TIMER0 INTENCLR COMPARE3: Disable interrupt on COMPARE[3]\n\nTIMER0 INTENCLR COMPARE3: Disable interrupt on COMPARE[3]\n\nTIMER0 INTENCLR COMPARE3: Disable interrupt on COMPARE[3]"]
    #[inline]
    pub(crate) fn write_timer0_intenclr_compare3(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TIMER0 INTENCLR COMPARE3, TIMER0 INTENCLR COMPARE3, TIMER0 INTENCLR COMPARE3 reset value false")
    }
    #[doc = "TIMER0 MODE MODE: Select Normal or Counter mode.\n\nTIMER0 MODE MODE: Select Normal or Counter mode.\n\nTIMER0 MODE MODE: Select Normal or Counter mode."]
    #[inline]
    pub(crate) fn read_timer0_mode_mode(&self) -> MemResult<bool> {
        todo ! ("read TIMER0 MODE MODE, TIMER0 MODE MODE, TIMER0 MODE MODE reset value false")
    }
    #[doc = "TIMER0 MODE MODE: Select Normal or Counter mode.\n\nTIMER0 MODE MODE: Select Normal or Counter mode.\n\nTIMER0 MODE MODE: Select Normal or Counter mode."]
    #[inline]
    pub(crate) fn write_timer0_mode_mode(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TIMER0 MODE MODE, TIMER0 MODE MODE, TIMER0 MODE MODE reset value false")
    }
    #[doc = "TIMER0 BITMODE BITMODE: Sets timer behaviour ro be like the implementation of a timer with width as indicated.\n\nTIMER0 BITMODE BITMODE: Sets timer behaviour ro be like the implementation of a timer with width as indicated.\n\nTIMER0 BITMODE BITMODE: Sets timer behaviour ro be like the implementation of a timer with width as indicated."]
    #[inline]
    pub(crate) fn read_timer0_bitmode_bitmode(&self) -> MemResult<u8> {
        todo ! ("read TIMER0 BITMODE BITMODE, TIMER0 BITMODE BITMODE, TIMER0 BITMODE BITMODE reset value 0x00 mask 0x03")
    }
    #[doc = "TIMER0 BITMODE BITMODE: Sets timer behaviour ro be like the implementation of a timer with width as indicated.\n\nTIMER0 BITMODE BITMODE: Sets timer behaviour ro be like the implementation of a timer with width as indicated.\n\nTIMER0 BITMODE BITMODE: Sets timer behaviour ro be like the implementation of a timer with width as indicated."]
    #[inline]
    pub(crate) fn write_timer0_bitmode_bitmode(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write TIMER0 BITMODE BITMODE, TIMER0 BITMODE BITMODE, TIMER0 BITMODE BITMODE reset value 0x00 mask 0x03")
    }
    #[doc = "TIMER0 PRESCALER PRESCALER: Timer PRESCALER value. Max value is 9.\n\nTIMER0 PRESCALER PRESCALER: Timer PRESCALER value. Max value is 9.\n\nTIMER0 PRESCALER PRESCALER: Timer PRESCALER value. Max value is 9."]
    #[inline]
    pub(crate) fn read_timer0_prescaler_prescaler(&self) -> MemResult<u8> {
        todo ! ("read TIMER0 PRESCALER PRESCALER, TIMER0 PRESCALER PRESCALER, TIMER0 PRESCALER PRESCALER reset value 0x04 mask 0x0f")
    }
    #[doc = "TIMER0 PRESCALER PRESCALER: Timer PRESCALER value. Max value is 9.\n\nTIMER0 PRESCALER PRESCALER: Timer PRESCALER value. Max value is 9.\n\nTIMER0 PRESCALER PRESCALER: Timer PRESCALER value. Max value is 9."]
    #[inline]
    pub(crate) fn write_timer0_prescaler_prescaler(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write TIMER0 PRESCALER PRESCALER, TIMER0 PRESCALER PRESCALER, TIMER0 PRESCALER PRESCALER reset value 0x04 mask 0x0f")
    }
    #[doc = "TIMER0 CC[%s]: Capture/compare registers.\n\nTIMER0 CC[%s]: Capture/compare registers.\n\nTIMER0 CC[%s]: Capture/compare registers."]
    #[inline]
    pub(crate) fn read_timer0_ccn(&self, _dim: usize) -> MemResult<u32> {
        todo ! ("read TIMER0 CC[%s], TIMER0 CC[%s], TIMER0 CC[%s] reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "TIMER0 CC[%s]: Capture/compare registers.\n\nTIMER0 CC[%s]: Capture/compare registers.\n\nTIMER0 CC[%s]: Capture/compare registers."]
    #[inline]
    pub(crate) fn write_timer0_ccn(
        &mut self,
        _dim: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write TIMER0 CC[%s], TIMER0 CC[%s], TIMER0 CC[%s] reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "TIMER0 POWER POWER: Peripheral power control.\n\nTIMER0 POWER POWER: Peripheral power control.\n\nTIMER0 POWER POWER: Peripheral power control."]
    #[inline]
    pub(crate) fn read_timer0_power_power(&self) -> MemResult<bool> {
        todo ! ("read TIMER0 POWER POWER, TIMER0 POWER POWER, TIMER0 POWER POWER reset value false")
    }
    #[doc = "TIMER0 POWER POWER: Peripheral power control.\n\nTIMER0 POWER POWER: Peripheral power control.\n\nTIMER0 POWER POWER: Peripheral power control."]
    #[inline]
    pub(crate) fn write_timer0_power_power(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TIMER0 POWER POWER, TIMER0 POWER POWER, TIMER0 POWER POWER reset value false")
    }
    #[doc = "RTC0 TASKS_START: Start RTC Counter.\n\nRTC0 TASKS_START: Start RTC Counter."]
    #[inline]
    pub(crate) fn write_rtc0_tasks_start(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write RTC0 TASKS_START, RTC0 TASKS_START reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RTC0 TASKS_STOP: Stop RTC Counter.\n\nRTC0 TASKS_STOP: Stop RTC Counter."]
    #[inline]
    pub(crate) fn write_rtc0_tasks_stop(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write RTC0 TASKS_STOP, RTC0 TASKS_STOP reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RTC0 TASKS_CLEAR: Clear RTC Counter.\n\nRTC0 TASKS_CLEAR: Clear RTC Counter."]
    #[inline]
    pub(crate) fn write_rtc0_tasks_clear(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write RTC0 TASKS_CLEAR, RTC0 TASKS_CLEAR reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RTC0 TASKS_TRIGOVRFLW: Set COUNTER to 0xFFFFFFF0.\n\nRTC0 TASKS_TRIGOVRFLW: Set COUNTER to 0xFFFFFFF0."]
    #[inline]
    pub(crate) fn write_rtc0_tasks_trigovrflw(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write RTC0 TASKS_TRIGOVRFLW, RTC0 TASKS_TRIGOVRFLW reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RTC0 EVENTS_TICK: Event on COUNTER increment.\n\nRTC0 EVENTS_TICK: Event on COUNTER increment."]
    #[inline]
    pub(crate) fn read_rtc0_events_tick(&self) -> MemResult<u32> {
        todo ! ("read RTC0 EVENTS_TICK, RTC0 EVENTS_TICK reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RTC0 EVENTS_TICK: Event on COUNTER increment.\n\nRTC0 EVENTS_TICK: Event on COUNTER increment."]
    #[inline]
    pub(crate) fn write_rtc0_events_tick(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write RTC0 EVENTS_TICK, RTC0 EVENTS_TICK reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RTC0 EVENTS_OVRFLW: Event on COUNTER overflow.\n\nRTC0 EVENTS_OVRFLW: Event on COUNTER overflow."]
    #[inline]
    pub(crate) fn read_rtc0_events_ovrflw(&self) -> MemResult<u32> {
        todo ! ("read RTC0 EVENTS_OVRFLW, RTC0 EVENTS_OVRFLW reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RTC0 EVENTS_OVRFLW: Event on COUNTER overflow.\n\nRTC0 EVENTS_OVRFLW: Event on COUNTER overflow."]
    #[inline]
    pub(crate) fn write_rtc0_events_ovrflw(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write RTC0 EVENTS_OVRFLW, RTC0 EVENTS_OVRFLW reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RTC0 EVENTS_COMPARE[%s]: Compare event on CC[n] match.\n\nRTC0 EVENTS_COMPARE[%s]: Compare event on CC[n] match."]
    #[inline]
    pub(crate) fn read_rtc0_events_comparen(
        &self,
        _dim: usize,
    ) -> MemResult<u32> {
        todo ! ("read RTC0 EVENTS_COMPARE[%s], RTC0 EVENTS_COMPARE[%s] reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RTC0 EVENTS_COMPARE[%s]: Compare event on CC[n] match.\n\nRTC0 EVENTS_COMPARE[%s]: Compare event on CC[n] match."]
    #[inline]
    pub(crate) fn write_rtc0_events_comparen(
        &mut self,
        _dim: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write RTC0 EVENTS_COMPARE[%s], RTC0 EVENTS_COMPARE[%s] reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RTC0 INTENSET TICK: Enable interrupt on TICK event.\n\nRTC0 INTENSET TICK: Enable interrupt on TICK event."]
    #[inline]
    pub(crate) fn read_rtc0_intenset_tick(&self) -> MemResult<bool> {
        todo!("read RTC0 INTENSET TICK, RTC0 INTENSET TICK reset value false")
    }
    #[doc = "RTC0 INTENSET TICK: Enable interrupt on TICK event.\n\nRTC0 INTENSET TICK: Enable interrupt on TICK event."]
    #[inline]
    pub(crate) fn write_rtc0_intenset_tick(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RTC0 INTENSET TICK, RTC0 INTENSET TICK reset value false")
    }
    #[doc = "RTC0 INTENSET OVRFLW: Enable interrupt on OVRFLW event.\n\nRTC0 INTENSET OVRFLW: Enable interrupt on OVRFLW event."]
    #[inline]
    pub(crate) fn read_rtc0_intenset_ovrflw(&self) -> MemResult<bool> {
        todo!(
            "read RTC0 INTENSET OVRFLW, RTC0 INTENSET OVRFLW reset value false"
        )
    }
    #[doc = "RTC0 INTENSET OVRFLW: Enable interrupt on OVRFLW event.\n\nRTC0 INTENSET OVRFLW: Enable interrupt on OVRFLW event."]
    #[inline]
    pub(crate) fn write_rtc0_intenset_ovrflw(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RTC0 INTENSET OVRFLW, RTC0 INTENSET OVRFLW reset value false")
    }
    #[doc = "RTC0 INTENSET COMPARE0: Enable interrupt on COMPARE[0] event.\n\nRTC0 INTENSET COMPARE0: Enable interrupt on COMPARE[0] event."]
    #[inline]
    pub(crate) fn read_rtc0_intenset_compare0(&self) -> MemResult<bool> {
        todo ! ("read RTC0 INTENSET COMPARE0, RTC0 INTENSET COMPARE0 reset value false")
    }
    #[doc = "RTC0 INTENSET COMPARE0: Enable interrupt on COMPARE[0] event.\n\nRTC0 INTENSET COMPARE0: Enable interrupt on COMPARE[0] event."]
    #[inline]
    pub(crate) fn write_rtc0_intenset_compare0(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RTC0 INTENSET COMPARE0, RTC0 INTENSET COMPARE0 reset value false")
    }
    #[doc = "RTC0 INTENSET COMPARE1: Enable interrupt on COMPARE[1] event.\n\nRTC0 INTENSET COMPARE1: Enable interrupt on COMPARE[1] event."]
    #[inline]
    pub(crate) fn read_rtc0_intenset_compare1(&self) -> MemResult<bool> {
        todo ! ("read RTC0 INTENSET COMPARE1, RTC0 INTENSET COMPARE1 reset value false")
    }
    #[doc = "RTC0 INTENSET COMPARE1: Enable interrupt on COMPARE[1] event.\n\nRTC0 INTENSET COMPARE1: Enable interrupt on COMPARE[1] event."]
    #[inline]
    pub(crate) fn write_rtc0_intenset_compare1(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RTC0 INTENSET COMPARE1, RTC0 INTENSET COMPARE1 reset value false")
    }
    #[doc = "RTC0 INTENSET COMPARE2: Enable interrupt on COMPARE[2] event.\n\nRTC0 INTENSET COMPARE2: Enable interrupt on COMPARE[2] event."]
    #[inline]
    pub(crate) fn read_rtc0_intenset_compare2(&self) -> MemResult<bool> {
        todo ! ("read RTC0 INTENSET COMPARE2, RTC0 INTENSET COMPARE2 reset value false")
    }
    #[doc = "RTC0 INTENSET COMPARE2: Enable interrupt on COMPARE[2] event.\n\nRTC0 INTENSET COMPARE2: Enable interrupt on COMPARE[2] event."]
    #[inline]
    pub(crate) fn write_rtc0_intenset_compare2(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RTC0 INTENSET COMPARE2, RTC0 INTENSET COMPARE2 reset value false")
    }
    #[doc = "RTC0 INTENSET COMPARE3: Enable interrupt on COMPARE[3] event.\n\nRTC0 INTENSET COMPARE3: Enable interrupt on COMPARE[3] event."]
    #[inline]
    pub(crate) fn read_rtc0_intenset_compare3(&self) -> MemResult<bool> {
        todo ! ("read RTC0 INTENSET COMPARE3, RTC0 INTENSET COMPARE3 reset value false")
    }
    #[doc = "RTC0 INTENSET COMPARE3: Enable interrupt on COMPARE[3] event.\n\nRTC0 INTENSET COMPARE3: Enable interrupt on COMPARE[3] event."]
    #[inline]
    pub(crate) fn write_rtc0_intenset_compare3(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RTC0 INTENSET COMPARE3, RTC0 INTENSET COMPARE3 reset value false")
    }
    #[doc = "RTC0 INTENCLR TICK: Disable interrupt on TICK event.\n\nRTC0 INTENCLR TICK: Disable interrupt on TICK event."]
    #[inline]
    pub(crate) fn read_rtc0_intenclr_tick(&self) -> MemResult<bool> {
        todo!("read RTC0 INTENCLR TICK, RTC0 INTENCLR TICK reset value false")
    }
    #[doc = "RTC0 INTENCLR TICK: Disable interrupt on TICK event.\n\nRTC0 INTENCLR TICK: Disable interrupt on TICK event."]
    #[inline]
    pub(crate) fn write_rtc0_intenclr_tick(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RTC0 INTENCLR TICK, RTC0 INTENCLR TICK reset value false")
    }
    #[doc = "RTC0 INTENCLR OVRFLW: Disable interrupt on OVRFLW event.\n\nRTC0 INTENCLR OVRFLW: Disable interrupt on OVRFLW event."]
    #[inline]
    pub(crate) fn read_rtc0_intenclr_ovrflw(&self) -> MemResult<bool> {
        todo!(
            "read RTC0 INTENCLR OVRFLW, RTC0 INTENCLR OVRFLW reset value false"
        )
    }
    #[doc = "RTC0 INTENCLR OVRFLW: Disable interrupt on OVRFLW event.\n\nRTC0 INTENCLR OVRFLW: Disable interrupt on OVRFLW event."]
    #[inline]
    pub(crate) fn write_rtc0_intenclr_ovrflw(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RTC0 INTENCLR OVRFLW, RTC0 INTENCLR OVRFLW reset value false")
    }
    #[doc = "RTC0 INTENCLR COMPARE0: Disable interrupt on COMPARE[0] event.\n\nRTC0 INTENCLR COMPARE0: Disable interrupt on COMPARE[0] event."]
    #[inline]
    pub(crate) fn read_rtc0_intenclr_compare0(&self) -> MemResult<bool> {
        todo ! ("read RTC0 INTENCLR COMPARE0, RTC0 INTENCLR COMPARE0 reset value false")
    }
    #[doc = "RTC0 INTENCLR COMPARE0: Disable interrupt on COMPARE[0] event.\n\nRTC0 INTENCLR COMPARE0: Disable interrupt on COMPARE[0] event."]
    #[inline]
    pub(crate) fn write_rtc0_intenclr_compare0(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RTC0 INTENCLR COMPARE0, RTC0 INTENCLR COMPARE0 reset value false")
    }
    #[doc = "RTC0 INTENCLR COMPARE1: Disable interrupt on COMPARE[1] event.\n\nRTC0 INTENCLR COMPARE1: Disable interrupt on COMPARE[1] event."]
    #[inline]
    pub(crate) fn read_rtc0_intenclr_compare1(&self) -> MemResult<bool> {
        todo ! ("read RTC0 INTENCLR COMPARE1, RTC0 INTENCLR COMPARE1 reset value false")
    }
    #[doc = "RTC0 INTENCLR COMPARE1: Disable interrupt on COMPARE[1] event.\n\nRTC0 INTENCLR COMPARE1: Disable interrupt on COMPARE[1] event."]
    #[inline]
    pub(crate) fn write_rtc0_intenclr_compare1(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RTC0 INTENCLR COMPARE1, RTC0 INTENCLR COMPARE1 reset value false")
    }
    #[doc = "RTC0 INTENCLR COMPARE2: Disable interrupt on COMPARE[2] event.\n\nRTC0 INTENCLR COMPARE2: Disable interrupt on COMPARE[2] event."]
    #[inline]
    pub(crate) fn read_rtc0_intenclr_compare2(&self) -> MemResult<bool> {
        todo ! ("read RTC0 INTENCLR COMPARE2, RTC0 INTENCLR COMPARE2 reset value false")
    }
    #[doc = "RTC0 INTENCLR COMPARE2: Disable interrupt on COMPARE[2] event.\n\nRTC0 INTENCLR COMPARE2: Disable interrupt on COMPARE[2] event."]
    #[inline]
    pub(crate) fn write_rtc0_intenclr_compare2(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RTC0 INTENCLR COMPARE2, RTC0 INTENCLR COMPARE2 reset value false")
    }
    #[doc = "RTC0 INTENCLR COMPARE3: Disable interrupt on COMPARE[3] event.\n\nRTC0 INTENCLR COMPARE3: Disable interrupt on COMPARE[3] event."]
    #[inline]
    pub(crate) fn read_rtc0_intenclr_compare3(&self) -> MemResult<bool> {
        todo ! ("read RTC0 INTENCLR COMPARE3, RTC0 INTENCLR COMPARE3 reset value false")
    }
    #[doc = "RTC0 INTENCLR COMPARE3: Disable interrupt on COMPARE[3] event.\n\nRTC0 INTENCLR COMPARE3: Disable interrupt on COMPARE[3] event."]
    #[inline]
    pub(crate) fn write_rtc0_intenclr_compare3(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RTC0 INTENCLR COMPARE3, RTC0 INTENCLR COMPARE3 reset value false")
    }
    #[doc = "RTC0 EVTEN TICK: TICK event enable.\n\nRTC0 EVTEN TICK: TICK event enable."]
    #[inline]
    pub(crate) fn read_rtc0_evten_tick(&self) -> MemResult<bool> {
        todo!("read RTC0 EVTEN TICK, RTC0 EVTEN TICK reset value false")
    }
    #[doc = "RTC0 EVTEN TICK: TICK event enable.\n\nRTC0 EVTEN TICK: TICK event enable."]
    #[inline]
    pub(crate) fn write_rtc0_evten_tick(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RTC0 EVTEN TICK, RTC0 EVTEN TICK reset value false")
    }
    #[doc = "RTC0 EVTEN OVRFLW: OVRFLW event enable.\n\nRTC0 EVTEN OVRFLW: OVRFLW event enable."]
    #[inline]
    pub(crate) fn read_rtc0_evten_ovrflw(&self) -> MemResult<bool> {
        todo!("read RTC0 EVTEN OVRFLW, RTC0 EVTEN OVRFLW reset value false")
    }
    #[doc = "RTC0 EVTEN OVRFLW: OVRFLW event enable.\n\nRTC0 EVTEN OVRFLW: OVRFLW event enable."]
    #[inline]
    pub(crate) fn write_rtc0_evten_ovrflw(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RTC0 EVTEN OVRFLW, RTC0 EVTEN OVRFLW reset value false")
    }
    #[doc = "RTC0 EVTEN COMPARE0: COMPARE[0] event enable.\n\nRTC0 EVTEN COMPARE0: COMPARE[0] event enable."]
    #[inline]
    pub(crate) fn read_rtc0_evten_compare0(&self) -> MemResult<bool> {
        todo!("read RTC0 EVTEN COMPARE0, RTC0 EVTEN COMPARE0 reset value false")
    }
    #[doc = "RTC0 EVTEN COMPARE0: COMPARE[0] event enable.\n\nRTC0 EVTEN COMPARE0: COMPARE[0] event enable."]
    #[inline]
    pub(crate) fn write_rtc0_evten_compare0(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write RTC0 EVTEN COMPARE0, RTC0 EVTEN COMPARE0 reset value false"
        )
    }
    #[doc = "RTC0 EVTEN COMPARE1: COMPARE[1] event enable.\n\nRTC0 EVTEN COMPARE1: COMPARE[1] event enable."]
    #[inline]
    pub(crate) fn read_rtc0_evten_compare1(&self) -> MemResult<bool> {
        todo!("read RTC0 EVTEN COMPARE1, RTC0 EVTEN COMPARE1 reset value false")
    }
    #[doc = "RTC0 EVTEN COMPARE1: COMPARE[1] event enable.\n\nRTC0 EVTEN COMPARE1: COMPARE[1] event enable."]
    #[inline]
    pub(crate) fn write_rtc0_evten_compare1(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write RTC0 EVTEN COMPARE1, RTC0 EVTEN COMPARE1 reset value false"
        )
    }
    #[doc = "RTC0 EVTEN COMPARE2: COMPARE[2] event enable.\n\nRTC0 EVTEN COMPARE2: COMPARE[2] event enable."]
    #[inline]
    pub(crate) fn read_rtc0_evten_compare2(&self) -> MemResult<bool> {
        todo!("read RTC0 EVTEN COMPARE2, RTC0 EVTEN COMPARE2 reset value false")
    }
    #[doc = "RTC0 EVTEN COMPARE2: COMPARE[2] event enable.\n\nRTC0 EVTEN COMPARE2: COMPARE[2] event enable."]
    #[inline]
    pub(crate) fn write_rtc0_evten_compare2(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write RTC0 EVTEN COMPARE2, RTC0 EVTEN COMPARE2 reset value false"
        )
    }
    #[doc = "RTC0 EVTEN COMPARE3: COMPARE[3] event enable.\n\nRTC0 EVTEN COMPARE3: COMPARE[3] event enable."]
    #[inline]
    pub(crate) fn read_rtc0_evten_compare3(&self) -> MemResult<bool> {
        todo!("read RTC0 EVTEN COMPARE3, RTC0 EVTEN COMPARE3 reset value false")
    }
    #[doc = "RTC0 EVTEN COMPARE3: COMPARE[3] event enable.\n\nRTC0 EVTEN COMPARE3: COMPARE[3] event enable."]
    #[inline]
    pub(crate) fn write_rtc0_evten_compare3(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write RTC0 EVTEN COMPARE3, RTC0 EVTEN COMPARE3 reset value false"
        )
    }
    #[doc = "RTC0 EVTENSET TICK: Enable routing to PPI of TICK event.\n\nRTC0 EVTENSET TICK: Enable routing to PPI of TICK event."]
    #[inline]
    pub(crate) fn read_rtc0_evtenset_tick(&self) -> MemResult<bool> {
        todo!("read RTC0 EVTENSET TICK, RTC0 EVTENSET TICK reset value false")
    }
    #[doc = "RTC0 EVTENSET TICK: Enable routing to PPI of TICK event.\n\nRTC0 EVTENSET TICK: Enable routing to PPI of TICK event."]
    #[inline]
    pub(crate) fn write_rtc0_evtenset_tick(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RTC0 EVTENSET TICK, RTC0 EVTENSET TICK reset value false")
    }
    #[doc = "RTC0 EVTENSET OVRFLW: Enable routing to PPI of OVRFLW event.\n\nRTC0 EVTENSET OVRFLW: Enable routing to PPI of OVRFLW event."]
    #[inline]
    pub(crate) fn read_rtc0_evtenset_ovrflw(&self) -> MemResult<bool> {
        todo!(
            "read RTC0 EVTENSET OVRFLW, RTC0 EVTENSET OVRFLW reset value false"
        )
    }
    #[doc = "RTC0 EVTENSET OVRFLW: Enable routing to PPI of OVRFLW event.\n\nRTC0 EVTENSET OVRFLW: Enable routing to PPI of OVRFLW event."]
    #[inline]
    pub(crate) fn write_rtc0_evtenset_ovrflw(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RTC0 EVTENSET OVRFLW, RTC0 EVTENSET OVRFLW reset value false")
    }
    #[doc = "RTC0 EVTENSET COMPARE0: Enable routing to PPI of COMPARE[0] event.\n\nRTC0 EVTENSET COMPARE0: Enable routing to PPI of COMPARE[0] event."]
    #[inline]
    pub(crate) fn read_rtc0_evtenset_compare0(&self) -> MemResult<bool> {
        todo ! ("read RTC0 EVTENSET COMPARE0, RTC0 EVTENSET COMPARE0 reset value false")
    }
    #[doc = "RTC0 EVTENSET COMPARE0: Enable routing to PPI of COMPARE[0] event.\n\nRTC0 EVTENSET COMPARE0: Enable routing to PPI of COMPARE[0] event."]
    #[inline]
    pub(crate) fn write_rtc0_evtenset_compare0(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RTC0 EVTENSET COMPARE0, RTC0 EVTENSET COMPARE0 reset value false")
    }
    #[doc = "RTC0 EVTENSET COMPARE1: Enable routing to PPI of COMPARE[1] event.\n\nRTC0 EVTENSET COMPARE1: Enable routing to PPI of COMPARE[1] event."]
    #[inline]
    pub(crate) fn read_rtc0_evtenset_compare1(&self) -> MemResult<bool> {
        todo ! ("read RTC0 EVTENSET COMPARE1, RTC0 EVTENSET COMPARE1 reset value false")
    }
    #[doc = "RTC0 EVTENSET COMPARE1: Enable routing to PPI of COMPARE[1] event.\n\nRTC0 EVTENSET COMPARE1: Enable routing to PPI of COMPARE[1] event."]
    #[inline]
    pub(crate) fn write_rtc0_evtenset_compare1(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RTC0 EVTENSET COMPARE1, RTC0 EVTENSET COMPARE1 reset value false")
    }
    #[doc = "RTC0 EVTENSET COMPARE2: Enable routing to PPI of COMPARE[2] event.\n\nRTC0 EVTENSET COMPARE2: Enable routing to PPI of COMPARE[2] event."]
    #[inline]
    pub(crate) fn read_rtc0_evtenset_compare2(&self) -> MemResult<bool> {
        todo ! ("read RTC0 EVTENSET COMPARE2, RTC0 EVTENSET COMPARE2 reset value false")
    }
    #[doc = "RTC0 EVTENSET COMPARE2: Enable routing to PPI of COMPARE[2] event.\n\nRTC0 EVTENSET COMPARE2: Enable routing to PPI of COMPARE[2] event."]
    #[inline]
    pub(crate) fn write_rtc0_evtenset_compare2(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RTC0 EVTENSET COMPARE2, RTC0 EVTENSET COMPARE2 reset value false")
    }
    #[doc = "RTC0 EVTENSET COMPARE3: Enable routing to PPI of COMPARE[3] event.\n\nRTC0 EVTENSET COMPARE3: Enable routing to PPI of COMPARE[3] event."]
    #[inline]
    pub(crate) fn read_rtc0_evtenset_compare3(&self) -> MemResult<bool> {
        todo ! ("read RTC0 EVTENSET COMPARE3, RTC0 EVTENSET COMPARE3 reset value false")
    }
    #[doc = "RTC0 EVTENSET COMPARE3: Enable routing to PPI of COMPARE[3] event.\n\nRTC0 EVTENSET COMPARE3: Enable routing to PPI of COMPARE[3] event."]
    #[inline]
    pub(crate) fn write_rtc0_evtenset_compare3(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RTC0 EVTENSET COMPARE3, RTC0 EVTENSET COMPARE3 reset value false")
    }
    #[doc = "RTC0 EVTENCLR TICK: Disable routing to PPI of TICK event.\n\nRTC0 EVTENCLR TICK: Disable routing to PPI of TICK event."]
    #[inline]
    pub(crate) fn read_rtc0_evtenclr_tick(&self) -> MemResult<bool> {
        todo!("read RTC0 EVTENCLR TICK, RTC0 EVTENCLR TICK reset value false")
    }
    #[doc = "RTC0 EVTENCLR TICK: Disable routing to PPI of TICK event.\n\nRTC0 EVTENCLR TICK: Disable routing to PPI of TICK event."]
    #[inline]
    pub(crate) fn write_rtc0_evtenclr_tick(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RTC0 EVTENCLR TICK, RTC0 EVTENCLR TICK reset value false")
    }
    #[doc = "RTC0 EVTENCLR OVRFLW: Disable routing to PPI of OVRFLW event.\n\nRTC0 EVTENCLR OVRFLW: Disable routing to PPI of OVRFLW event."]
    #[inline]
    pub(crate) fn read_rtc0_evtenclr_ovrflw(&self) -> MemResult<bool> {
        todo!(
            "read RTC0 EVTENCLR OVRFLW, RTC0 EVTENCLR OVRFLW reset value false"
        )
    }
    #[doc = "RTC0 EVTENCLR OVRFLW: Disable routing to PPI of OVRFLW event.\n\nRTC0 EVTENCLR OVRFLW: Disable routing to PPI of OVRFLW event."]
    #[inline]
    pub(crate) fn write_rtc0_evtenclr_ovrflw(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RTC0 EVTENCLR OVRFLW, RTC0 EVTENCLR OVRFLW reset value false")
    }
    #[doc = "RTC0 EVTENCLR COMPARE0: Disable routing to PPI of COMPARE[0] event.\n\nRTC0 EVTENCLR COMPARE0: Disable routing to PPI of COMPARE[0] event."]
    #[inline]
    pub(crate) fn read_rtc0_evtenclr_compare0(&self) -> MemResult<bool> {
        todo ! ("read RTC0 EVTENCLR COMPARE0, RTC0 EVTENCLR COMPARE0 reset value false")
    }
    #[doc = "RTC0 EVTENCLR COMPARE0: Disable routing to PPI of COMPARE[0] event.\n\nRTC0 EVTENCLR COMPARE0: Disable routing to PPI of COMPARE[0] event."]
    #[inline]
    pub(crate) fn write_rtc0_evtenclr_compare0(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RTC0 EVTENCLR COMPARE0, RTC0 EVTENCLR COMPARE0 reset value false")
    }
    #[doc = "RTC0 EVTENCLR COMPARE1: Disable routing to PPI of COMPARE[1] event.\n\nRTC0 EVTENCLR COMPARE1: Disable routing to PPI of COMPARE[1] event."]
    #[inline]
    pub(crate) fn read_rtc0_evtenclr_compare1(&self) -> MemResult<bool> {
        todo ! ("read RTC0 EVTENCLR COMPARE1, RTC0 EVTENCLR COMPARE1 reset value false")
    }
    #[doc = "RTC0 EVTENCLR COMPARE1: Disable routing to PPI of COMPARE[1] event.\n\nRTC0 EVTENCLR COMPARE1: Disable routing to PPI of COMPARE[1] event."]
    #[inline]
    pub(crate) fn write_rtc0_evtenclr_compare1(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RTC0 EVTENCLR COMPARE1, RTC0 EVTENCLR COMPARE1 reset value false")
    }
    #[doc = "RTC0 EVTENCLR COMPARE2: Disable routing to PPI of COMPARE[2] event.\n\nRTC0 EVTENCLR COMPARE2: Disable routing to PPI of COMPARE[2] event."]
    #[inline]
    pub(crate) fn read_rtc0_evtenclr_compare2(&self) -> MemResult<bool> {
        todo ! ("read RTC0 EVTENCLR COMPARE2, RTC0 EVTENCLR COMPARE2 reset value false")
    }
    #[doc = "RTC0 EVTENCLR COMPARE2: Disable routing to PPI of COMPARE[2] event.\n\nRTC0 EVTENCLR COMPARE2: Disable routing to PPI of COMPARE[2] event."]
    #[inline]
    pub(crate) fn write_rtc0_evtenclr_compare2(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RTC0 EVTENCLR COMPARE2, RTC0 EVTENCLR COMPARE2 reset value false")
    }
    #[doc = "RTC0 EVTENCLR COMPARE3: Disable routing to PPI of COMPARE[3] event.\n\nRTC0 EVTENCLR COMPARE3: Disable routing to PPI of COMPARE[3] event."]
    #[inline]
    pub(crate) fn read_rtc0_evtenclr_compare3(&self) -> MemResult<bool> {
        todo ! ("read RTC0 EVTENCLR COMPARE3, RTC0 EVTENCLR COMPARE3 reset value false")
    }
    #[doc = "RTC0 EVTENCLR COMPARE3: Disable routing to PPI of COMPARE[3] event.\n\nRTC0 EVTENCLR COMPARE3: Disable routing to PPI of COMPARE[3] event."]
    #[inline]
    pub(crate) fn write_rtc0_evtenclr_compare3(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RTC0 EVTENCLR COMPARE3, RTC0 EVTENCLR COMPARE3 reset value false")
    }
    #[doc = "RTC0 COUNTER COUNTER: Counter value.\n\nRTC0 COUNTER COUNTER: Counter value."]
    #[inline]
    pub(crate) fn read_rtc0_counter_counter(&self) -> MemResult<u32> {
        todo ! ("read RTC0 COUNTER COUNTER, RTC0 COUNTER COUNTER reset value 0x00 mask 0xffffff")
    }
    #[doc = "RTC0 PRESCALER PRESCALER: RTC PRESCALER value.\n\nRTC0 PRESCALER PRESCALER: RTC PRESCALER value."]
    #[inline]
    pub(crate) fn read_rtc0_prescaler_prescaler(&self) -> MemResult<u16> {
        todo ! ("read RTC0 PRESCALER PRESCALER, RTC0 PRESCALER PRESCALER reset value 0x00 mask 0xfff")
    }
    #[doc = "RTC0 PRESCALER PRESCALER: RTC PRESCALER value.\n\nRTC0 PRESCALER PRESCALER: RTC PRESCALER value."]
    #[inline]
    pub(crate) fn write_rtc0_prescaler_prescaler(
        &mut self,
        _value: u16,
    ) -> MemResult<()> {
        todo ! ("write RTC0 PRESCALER PRESCALER, RTC0 PRESCALER PRESCALER reset value 0x00 mask 0xfff")
    }
    #[doc = "RTC0 CC[%s] COMPARE: Compare value.\n\nRTC0 CC[%s] COMPARE: Compare value."]
    #[inline]
    pub(crate) fn read_rtc0_ccn_compare(&self, _dim: usize) -> MemResult<u32> {
        todo ! ("read RTC0 CC[%s] COMPARE, RTC0 CC[%s] COMPARE reset value 0x00 mask 0xffffff")
    }
    #[doc = "RTC0 CC[%s] COMPARE: Compare value.\n\nRTC0 CC[%s] COMPARE: Compare value."]
    #[inline]
    pub(crate) fn write_rtc0_ccn_compare(
        &mut self,
        _dim: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write RTC0 CC[%s] COMPARE, RTC0 CC[%s] COMPARE reset value 0x00 mask 0xffffff")
    }
    #[doc = "RTC0 POWER POWER: Peripheral power control.\n\nRTC0 POWER POWER: Peripheral power control."]
    #[inline]
    pub(crate) fn read_rtc0_power_power(&self) -> MemResult<bool> {
        todo!("read RTC0 POWER POWER, RTC0 POWER POWER reset value false")
    }
    #[doc = "RTC0 POWER POWER: Peripheral power control.\n\nRTC0 POWER POWER: Peripheral power control."]
    #[inline]
    pub(crate) fn write_rtc0_power_power(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RTC0 POWER POWER, RTC0 POWER POWER reset value false")
    }
    #[doc = "TEMP TASKS_START: Start temperature measurement."]
    #[inline]
    pub(crate) fn write_temp_tasks_start(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write TEMP TASKS_START reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "TEMP TASKS_STOP: Stop temperature measurement."]
    #[inline]
    pub(crate) fn write_temp_tasks_stop(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write TEMP TASKS_STOP reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "TEMP EVENTS_DATARDY: Temperature measurement complete, data ready event."]
    #[inline]
    pub(crate) fn read_temp_events_datardy(&self) -> MemResult<u32> {
        todo!(
            "read TEMP EVENTS_DATARDY reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "TEMP EVENTS_DATARDY: Temperature measurement complete, data ready event."]
    #[inline]
    pub(crate) fn write_temp_events_datardy(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write TEMP EVENTS_DATARDY reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "TEMP INTENSET DATARDY: Enable interrupt on DATARDY event."]
    #[inline]
    pub(crate) fn read_temp_intenset_datardy(&self) -> MemResult<bool> {
        todo!("read TEMP INTENSET DATARDY reset value false")
    }
    #[doc = "TEMP INTENSET DATARDY: Enable interrupt on DATARDY event."]
    #[inline]
    pub(crate) fn write_temp_intenset_datardy(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write TEMP INTENSET DATARDY reset value false")
    }
    #[doc = "TEMP INTENCLR DATARDY: Disable interrupt on DATARDY event."]
    #[inline]
    pub(crate) fn read_temp_intenclr_datardy(&self) -> MemResult<bool> {
        todo!("read TEMP INTENCLR DATARDY reset value false")
    }
    #[doc = "TEMP INTENCLR DATARDY: Disable interrupt on DATARDY event."]
    #[inline]
    pub(crate) fn write_temp_intenclr_datardy(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write TEMP INTENCLR DATARDY reset value false")
    }
    #[doc = "TEMP TEMP: Die temperature in degC, 2's complement format, 0.25 degC pecision."]
    #[inline]
    pub(crate) fn read_temp_temp(&self) -> MemResult<u32> {
        todo!("read TEMP TEMP reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "TEMP POWER POWER: Peripheral power control."]
    #[inline]
    pub(crate) fn read_temp_power_power(&self) -> MemResult<bool> {
        todo!("read TEMP POWER POWER reset value false")
    }
    #[doc = "TEMP POWER POWER: Peripheral power control."]
    #[inline]
    pub(crate) fn write_temp_power_power(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write TEMP POWER POWER reset value false")
    }
    #[doc = "RNG TASKS_START: Start the random number generator."]
    #[inline]
    pub(crate) fn write_rng_tasks_start(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write RNG TASKS_START reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RNG TASKS_STOP: Stop the random number generator."]
    #[inline]
    pub(crate) fn write_rng_tasks_stop(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write RNG TASKS_STOP reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RNG EVENTS_VALRDY: New random number generated and written to VALUE register."]
    #[inline]
    pub(crate) fn read_rng_events_valrdy(&self) -> MemResult<u32> {
        todo!("read RNG EVENTS_VALRDY reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "RNG EVENTS_VALRDY: New random number generated and written to VALUE register."]
    #[inline]
    pub(crate) fn write_rng_events_valrdy(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!(
            "write RNG EVENTS_VALRDY reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "RNG SHORTS VALRDY_STOP: Shortcut between VALRDY event and STOP task."]
    #[inline]
    pub(crate) fn read_rng_shorts_valrdy_stop(&self) -> MemResult<bool> {
        todo!("read RNG SHORTS VALRDY_STOP reset value false")
    }
    #[doc = "RNG SHORTS VALRDY_STOP: Shortcut between VALRDY event and STOP task."]
    #[inline]
    pub(crate) fn write_rng_shorts_valrdy_stop(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RNG SHORTS VALRDY_STOP reset value false")
    }
    #[doc = "RNG INTENSET VALRDY: Enable interrupt on VALRDY event."]
    #[inline]
    pub(crate) fn read_rng_intenset_valrdy(&self) -> MemResult<bool> {
        todo!("read RNG INTENSET VALRDY reset value false")
    }
    #[doc = "RNG INTENSET VALRDY: Enable interrupt on VALRDY event."]
    #[inline]
    pub(crate) fn write_rng_intenset_valrdy(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RNG INTENSET VALRDY reset value false")
    }
    #[doc = "RNG INTENCLR VALRDY: Disable interrupt on VALRDY event."]
    #[inline]
    pub(crate) fn read_rng_intenclr_valrdy(&self) -> MemResult<bool> {
        todo!("read RNG INTENCLR VALRDY reset value false")
    }
    #[doc = "RNG INTENCLR VALRDY: Disable interrupt on VALRDY event."]
    #[inline]
    pub(crate) fn write_rng_intenclr_valrdy(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RNG INTENCLR VALRDY reset value false")
    }
    #[doc = "RNG CONFIG DERCEN: Digital error correction enable."]
    #[inline]
    pub(crate) fn read_rng_config_dercen(&self) -> MemResult<bool> {
        todo!("read RNG CONFIG DERCEN reset value false")
    }
    #[doc = "RNG CONFIG DERCEN: Digital error correction enable."]
    #[inline]
    pub(crate) fn write_rng_config_dercen(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RNG CONFIG DERCEN reset value false")
    }
    #[doc = "RNG VALUE VALUE: Generated random number."]
    #[inline]
    pub(crate) fn read_rng_value_value(&self) -> MemResult<u8> {
        todo!("read RNG VALUE VALUE reset value 0x00 mask 0xff")
    }
    #[doc = "RNG POWER POWER: Peripheral power control."]
    #[inline]
    pub(crate) fn read_rng_power_power(&self) -> MemResult<bool> {
        todo!("read RNG POWER POWER reset value false")
    }
    #[doc = "RNG POWER POWER: Peripheral power control."]
    #[inline]
    pub(crate) fn write_rng_power_power(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RNG POWER POWER reset value false")
    }
    #[doc = "ECB TASKS_STARTECB: Start ECB block encrypt. If a crypto operation is running, this will not initiate a new encryption and the ERRORECB event will be triggered."]
    #[inline]
    pub(crate) fn write_ecb_tasks_startecb(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!(
            "write ECB TASKS_STARTECB reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "ECB TASKS_STOPECB: Stop current ECB encryption. If a crypto operation is running, this will will trigger the ERRORECB event."]
    #[inline]
    pub(crate) fn write_ecb_tasks_stopecb(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!(
            "write ECB TASKS_STOPECB reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "ECB EVENTS_ENDECB: ECB block encrypt complete."]
    #[inline]
    pub(crate) fn read_ecb_events_endecb(&self) -> MemResult<u32> {
        todo!("read ECB EVENTS_ENDECB reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "ECB EVENTS_ENDECB: ECB block encrypt complete."]
    #[inline]
    pub(crate) fn write_ecb_events_endecb(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!(
            "write ECB EVENTS_ENDECB reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "ECB EVENTS_ERRORECB: ECB block encrypt aborted due to a STOPECB task or due to an error."]
    #[inline]
    pub(crate) fn read_ecb_events_errorecb(&self) -> MemResult<u32> {
        todo!(
            "read ECB EVENTS_ERRORECB reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "ECB EVENTS_ERRORECB: ECB block encrypt aborted due to a STOPECB task or due to an error."]
    #[inline]
    pub(crate) fn write_ecb_events_errorecb(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write ECB EVENTS_ERRORECB reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "ECB INTENSET ENDECB: Enable interrupt on ENDECB event."]
    #[inline]
    pub(crate) fn read_ecb_intenset_endecb(&self) -> MemResult<bool> {
        todo!("read ECB INTENSET ENDECB reset value false")
    }
    #[doc = "ECB INTENSET ENDECB: Enable interrupt on ENDECB event."]
    #[inline]
    pub(crate) fn write_ecb_intenset_endecb(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ECB INTENSET ENDECB reset value false")
    }
    #[doc = "ECB INTENSET ERRORECB: Enable interrupt on ERRORECB event."]
    #[inline]
    pub(crate) fn read_ecb_intenset_errorecb(&self) -> MemResult<bool> {
        todo!("read ECB INTENSET ERRORECB reset value false")
    }
    #[doc = "ECB INTENSET ERRORECB: Enable interrupt on ERRORECB event."]
    #[inline]
    pub(crate) fn write_ecb_intenset_errorecb(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ECB INTENSET ERRORECB reset value false")
    }
    #[doc = "ECB INTENCLR ENDECB: Disable interrupt on ENDECB event."]
    #[inline]
    pub(crate) fn read_ecb_intenclr_endecb(&self) -> MemResult<bool> {
        todo!("read ECB INTENCLR ENDECB reset value false")
    }
    #[doc = "ECB INTENCLR ENDECB: Disable interrupt on ENDECB event."]
    #[inline]
    pub(crate) fn write_ecb_intenclr_endecb(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ECB INTENCLR ENDECB reset value false")
    }
    #[doc = "ECB INTENCLR ERRORECB: Disable interrupt on ERRORECB event."]
    #[inline]
    pub(crate) fn read_ecb_intenclr_errorecb(&self) -> MemResult<bool> {
        todo!("read ECB INTENCLR ERRORECB reset value false")
    }
    #[doc = "ECB INTENCLR ERRORECB: Disable interrupt on ERRORECB event."]
    #[inline]
    pub(crate) fn write_ecb_intenclr_errorecb(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ECB INTENCLR ERRORECB reset value false")
    }
    #[doc = "ECB ECBDATAPTR: ECB block encrypt memory pointer."]
    #[inline]
    pub(crate) fn read_ecb_ecbdataptr(&self) -> MemResult<u32> {
        todo!("read ECB ECBDATAPTR reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "ECB ECBDATAPTR: ECB block encrypt memory pointer."]
    #[inline]
    pub(crate) fn write_ecb_ecbdataptr(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write ECB ECBDATAPTR reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "ECB POWER POWER: Peripheral power control."]
    #[inline]
    pub(crate) fn read_ecb_power_power(&self) -> MemResult<bool> {
        todo!("read ECB POWER POWER reset value false")
    }
    #[doc = "ECB POWER POWER: Peripheral power control."]
    #[inline]
    pub(crate) fn write_ecb_power_power(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ECB POWER POWER reset value false")
    }
    #[doc = "AAR TASKS_START: Start resolving addresses based on IRKs specified in the IRK data structure.\n\nCCM TASKS_KSGEN: Start generation of key-stream. This operation will stop by itself when completed."]
    #[inline]
    pub(crate) fn write_aarccm_tasks_start(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write AAR TASKS_START, CCM TASKS_KSGEN reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "AAR TASKS_STOP: Stop resolving addresses.\n\nCCM TASKS_STOP: Stop encrypt/decrypt."]
    #[inline]
    pub(crate) fn write_aarccm_tasks_stop(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write AAR TASKS_STOP, CCM TASKS_STOP reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "AAR EVENTS_END: Address resolution procedure completed.\n\nCCM EVENTS_ENDKSGEN: Keystream generation completed."]
    #[inline]
    pub(crate) fn read_aarccm_events_end(&self) -> MemResult<u32> {
        todo ! ("read AAR EVENTS_END, CCM EVENTS_ENDKSGEN reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "AAR EVENTS_END: Address resolution procedure completed.\n\nCCM EVENTS_ENDKSGEN: Keystream generation completed."]
    #[inline]
    pub(crate) fn write_aarccm_events_end(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write AAR EVENTS_END, CCM EVENTS_ENDKSGEN reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "AAR EVENTS_RESOLVED: Address resolved.\n\nCCM EVENTS_ENDCRYPT: Encrypt/decrypt completed."]
    #[inline]
    pub(crate) fn read_aarccm_events_resolved(&self) -> MemResult<u32> {
        todo ! ("read AAR EVENTS_RESOLVED, CCM EVENTS_ENDCRYPT reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "AAR EVENTS_RESOLVED: Address resolved.\n\nCCM EVENTS_ENDCRYPT: Encrypt/decrypt completed."]
    #[inline]
    pub(crate) fn write_aarccm_events_resolved(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write AAR EVENTS_RESOLVED, CCM EVENTS_ENDCRYPT reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "AAR EVENTS_NOTRESOLVED: Address not resolved.\n\nCCM EVENTS_ERROR: Error happened."]
    #[inline]
    pub(crate) fn read_aarccm_events_notresolved(&self) -> MemResult<u32> {
        todo ! ("read AAR EVENTS_NOTRESOLVED, CCM EVENTS_ERROR reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "AAR EVENTS_NOTRESOLVED: Address not resolved.\n\nCCM EVENTS_ERROR: Error happened."]
    #[inline]
    pub(crate) fn write_aarccm_events_notresolved(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write AAR EVENTS_NOTRESOLVED, CCM EVENTS_ERROR reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "AAR INTENSET END: Enable interrupt on END event.\n\nCCM INTENSET ENDKSGEN: Enable interrupt on ENDKSGEN event."]
    #[inline]
    pub(crate) fn read_aarccm_intenset_end(&self) -> MemResult<bool> {
        todo!("read AAR INTENSET END, CCM INTENSET ENDKSGEN reset value false")
    }
    #[doc = "AAR INTENSET END: Enable interrupt on END event.\n\nCCM INTENSET ENDKSGEN: Enable interrupt on ENDKSGEN event."]
    #[inline]
    pub(crate) fn write_aarccm_intenset_end(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write AAR INTENSET END, CCM INTENSET ENDKSGEN reset value false")
    }
    #[doc = "AAR INTENSET RESOLVED: Enable interrupt on RESOLVED event.\n\nCCM INTENSET ENDCRYPT: Enable interrupt on ENDCRYPT event."]
    #[inline]
    pub(crate) fn read_aarccm_intenset_resolved(&self) -> MemResult<bool> {
        todo ! ("read AAR INTENSET RESOLVED, CCM INTENSET ENDCRYPT reset value false")
    }
    #[doc = "AAR INTENSET RESOLVED: Enable interrupt on RESOLVED event.\n\nCCM INTENSET ENDCRYPT: Enable interrupt on ENDCRYPT event."]
    #[inline]
    pub(crate) fn write_aarccm_intenset_resolved(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write AAR INTENSET RESOLVED, CCM INTENSET ENDCRYPT reset value false")
    }
    #[doc = "AAR INTENSET NOTRESOLVED: Enable interrupt on NOTRESOLVED event.\n\nCCM INTENSET ERROR: Enable interrupt on ERROR event."]
    #[inline]
    pub(crate) fn read_aarccm_intenset_notresolved(&self) -> MemResult<bool> {
        todo ! ("read AAR INTENSET NOTRESOLVED, CCM INTENSET ERROR reset value false")
    }
    #[doc = "AAR INTENSET NOTRESOLVED: Enable interrupt on NOTRESOLVED event.\n\nCCM INTENSET ERROR: Enable interrupt on ERROR event."]
    #[inline]
    pub(crate) fn write_aarccm_intenset_notresolved(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write AAR INTENSET NOTRESOLVED, CCM INTENSET ERROR reset value false")
    }
    #[doc = "AAR INTENCLR END: Disable interrupt on ENDKSGEN event.\n\nCCM INTENCLR ENDKSGEN: Disable interrupt on ENDKSGEN event."]
    #[inline]
    pub(crate) fn read_aarccm_intenclr_end(&self) -> MemResult<bool> {
        todo!("read AAR INTENCLR END, CCM INTENCLR ENDKSGEN reset value false")
    }
    #[doc = "AAR INTENCLR END: Disable interrupt on ENDKSGEN event.\n\nCCM INTENCLR ENDKSGEN: Disable interrupt on ENDKSGEN event."]
    #[inline]
    pub(crate) fn write_aarccm_intenclr_end(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write AAR INTENCLR END, CCM INTENCLR ENDKSGEN reset value false")
    }
    #[doc = "AAR INTENCLR RESOLVED: Disable interrupt on RESOLVED event.\n\nCCM INTENCLR ENDCRYPT: Disable interrupt on ENDCRYPT event."]
    #[inline]
    pub(crate) fn read_aarccm_intenclr_resolved(&self) -> MemResult<bool> {
        todo ! ("read AAR INTENCLR RESOLVED, CCM INTENCLR ENDCRYPT reset value false")
    }
    #[doc = "AAR INTENCLR RESOLVED: Disable interrupt on RESOLVED event.\n\nCCM INTENCLR ENDCRYPT: Disable interrupt on ENDCRYPT event."]
    #[inline]
    pub(crate) fn write_aarccm_intenclr_resolved(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write AAR INTENCLR RESOLVED, CCM INTENCLR ENDCRYPT reset value false")
    }
    #[doc = "AAR INTENCLR NOTRESOLVED: Disable interrupt on NOTRESOLVED event.\n\nCCM INTENCLR ERROR: Disable interrupt on ERROR event."]
    #[inline]
    pub(crate) fn read_aarccm_intenclr_notresolved(&self) -> MemResult<bool> {
        todo ! ("read AAR INTENCLR NOTRESOLVED, CCM INTENCLR ERROR reset value false")
    }
    #[doc = "AAR INTENCLR NOTRESOLVED: Disable interrupt on NOTRESOLVED event.\n\nCCM INTENCLR ERROR: Disable interrupt on ERROR event."]
    #[inline]
    pub(crate) fn write_aarccm_intenclr_notresolved(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write AAR INTENCLR NOTRESOLVED, CCM INTENCLR ERROR reset value false")
    }
    #[doc = "AAR STATUS STATUS: The IRK used last time an address was resolved.\n\nCCM MICSTATUS MICSTATUS: Result of the MIC check performed during the previous CCM RX STARTCRYPT"]
    #[inline]
    pub(crate) fn read_aarccm_status_status(&self) -> MemResult<u8> {
        todo ! ("read AAR STATUS STATUS, CCM MICSTATUS MICSTATUS reset value 0x00 mask 0x0f")
    }
    #[doc = "AAR ENABLE ENABLE: Enable AAR.\n\nCCM ENABLE ENABLE: CCM enable."]
    #[inline]
    pub(crate) fn read_aarccm_enable_enable(&self) -> MemResult<u8> {
        todo ! ("read AAR ENABLE ENABLE, CCM ENABLE ENABLE reset value 0x00 mask 0x03")
    }
    #[doc = "AAR ENABLE ENABLE: Enable AAR.\n\nCCM ENABLE ENABLE: CCM enable."]
    #[inline]
    pub(crate) fn write_aarccm_enable_enable(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write AAR ENABLE ENABLE, CCM ENABLE ENABLE reset value 0x00 mask 0x03")
    }
    #[doc = "AAR NIRK NIRK: Number of Identity root Keys in the IRK data structure.\n\nCCM MODE MODE: CCM mode operation."]
    #[inline]
    pub(crate) fn read_aarccm_nirk_nirk(&self) -> MemResult<u8> {
        todo!("read AAR NIRK NIRK, CCM MODE MODE reset value 0x01 mask 0x1f")
    }
    #[doc = "AAR NIRK NIRK: Number of Identity root Keys in the IRK data structure.\n\nCCM MODE MODE: CCM mode operation."]
    #[inline]
    pub(crate) fn write_aarccm_nirk_nirk(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write AAR NIRK NIRK, CCM MODE MODE reset value 0x01 mask 0x1f")
    }
    #[doc = "AAR IRKPTR: Pointer to the IRK data structure.\n\nCCM CNFPTR: Pointer to a data structure holding AES key and NONCE vector."]
    #[inline]
    pub(crate) fn read_aarccm_irkptr(&self) -> MemResult<u32> {
        todo ! ("read AAR IRKPTR, CCM CNFPTR reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "AAR IRKPTR: Pointer to the IRK data structure.\n\nCCM CNFPTR: Pointer to a data structure holding AES key and NONCE vector."]
    #[inline]
    pub(crate) fn write_aarccm_irkptr(&mut self, _value: u32) -> MemResult<()> {
        todo ! ("write AAR IRKPTR, CCM CNFPTR reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "AAR ADDRPTR: Pointer to the resolvable address (6 bytes).\n\nCCM OUTPTR: Pointer to the output packet."]
    #[inline]
    pub(crate) fn read_aarccm_addrptr(&self) -> MemResult<u32> {
        todo ! ("read AAR ADDRPTR, CCM OUTPTR reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "AAR ADDRPTR: Pointer to the resolvable address (6 bytes).\n\nCCM OUTPTR: Pointer to the output packet."]
    #[inline]
    pub(crate) fn write_aarccm_addrptr(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write AAR ADDRPTR, CCM OUTPTR reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "AAR SCRATCHPTR: Pointer to a scratch data area used for temporary storage during resolution. A minimum of 3 bytes must be reserved.\n\nCCM SCRATCHPTR: Pointer to a scratch data area used for temporary storage during resolution. A minimum of 43 bytes must be reserved."]
    #[inline]
    pub(crate) fn read_aarccm_scratchptr(&self) -> MemResult<u32> {
        todo ! ("read AAR SCRATCHPTR, CCM SCRATCHPTR reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "AAR SCRATCHPTR: Pointer to a scratch data area used for temporary storage during resolution. A minimum of 3 bytes must be reserved.\n\nCCM SCRATCHPTR: Pointer to a scratch data area used for temporary storage during resolution. A minimum of 43 bytes must be reserved."]
    #[inline]
    pub(crate) fn write_aarccm_scratchptr(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write AAR SCRATCHPTR, CCM SCRATCHPTR reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "AAR POWER POWER: Peripheral power control.\n\nCCM POWER POWER: Peripheral power control."]
    #[inline]
    pub(crate) fn read_aarccm_power_power(&self) -> MemResult<bool> {
        todo!("read AAR POWER POWER, CCM POWER POWER reset value false")
    }
    #[doc = "AAR POWER POWER: Peripheral power control.\n\nCCM POWER POWER: Peripheral power control."]
    #[inline]
    pub(crate) fn write_aarccm_power_power(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write AAR POWER POWER, CCM POWER POWER reset value false")
    }
    #[doc = "CCM TASKS_CRYPT: Start encrypt/decrypt. This operation will stop by itself when completed."]
    #[inline]
    pub(crate) fn write_ccm_tasks_crypt(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write CCM TASKS_CRYPT reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "CCM SHORTS ENDKSGEN_CRYPT: Shortcut between ENDKSGEN event and CRYPT task."]
    #[inline]
    pub(crate) fn read_ccm_shorts_endksgen_crypt(&self) -> MemResult<bool> {
        todo!("read CCM SHORTS ENDKSGEN_CRYPT reset value false")
    }
    #[doc = "CCM SHORTS ENDKSGEN_CRYPT: Shortcut between ENDKSGEN event and CRYPT task."]
    #[inline]
    pub(crate) fn write_ccm_shorts_endksgen_crypt(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CCM SHORTS ENDKSGEN_CRYPT reset value false")
    }
    #[doc = "CCM INPTR: Pointer to the input packet."]
    #[inline]
    pub(crate) fn read_ccm_inptr(&self) -> MemResult<u32> {
        todo!("read CCM INPTR reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "CCM INPTR: Pointer to the input packet."]
    #[inline]
    pub(crate) fn write_ccm_inptr(&mut self, _value: u32) -> MemResult<()> {
        todo!("write CCM INPTR reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "WDT TASKS_START: Start the watchdog."]
    #[inline]
    pub(crate) fn write_wdt_tasks_start(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write WDT TASKS_START reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "WDT EVENTS_TIMEOUT: Watchdog timeout."]
    #[inline]
    pub(crate) fn read_wdt_events_timeout(&self) -> MemResult<u32> {
        todo!(
            "read WDT EVENTS_TIMEOUT reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "WDT EVENTS_TIMEOUT: Watchdog timeout."]
    #[inline]
    pub(crate) fn write_wdt_events_timeout(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!(
            "write WDT EVENTS_TIMEOUT reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "WDT INTENSET TIMEOUT: Enable interrupt on TIMEOUT event."]
    #[inline]
    pub(crate) fn read_wdt_intenset_timeout(&self) -> MemResult<bool> {
        todo!("read WDT INTENSET TIMEOUT reset value false")
    }
    #[doc = "WDT INTENSET TIMEOUT: Enable interrupt on TIMEOUT event."]
    #[inline]
    pub(crate) fn write_wdt_intenset_timeout(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write WDT INTENSET TIMEOUT reset value false")
    }
    #[doc = "WDT INTENCLR TIMEOUT: Disable interrupt on TIMEOUT event."]
    #[inline]
    pub(crate) fn read_wdt_intenclr_timeout(&self) -> MemResult<bool> {
        todo!("read WDT INTENCLR TIMEOUT reset value false")
    }
    #[doc = "WDT INTENCLR TIMEOUT: Disable interrupt on TIMEOUT event."]
    #[inline]
    pub(crate) fn write_wdt_intenclr_timeout(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write WDT INTENCLR TIMEOUT reset value false")
    }
    #[doc = "WDT RUNSTATUS RUNSTATUS: Watchdog running status."]
    #[inline]
    pub(crate) fn read_wdt_runstatus_runstatus(&self) -> MemResult<bool> {
        todo!("read WDT RUNSTATUS RUNSTATUS reset value false")
    }
    #[doc = "WDT REQSTATUS RR0: Request status for RR[0]."]
    #[inline]
    pub(crate) fn read_wdt_reqstatus_rr0(&self) -> MemResult<bool> {
        todo!("read WDT REQSTATUS RR0 reset value true")
    }
    #[doc = "WDT REQSTATUS RR1: Request status for RR[1]."]
    #[inline]
    pub(crate) fn read_wdt_reqstatus_rr1(&self) -> MemResult<bool> {
        todo!("read WDT REQSTATUS RR1 reset value false")
    }
    #[doc = "WDT REQSTATUS RR2: Request status for RR[2]."]
    #[inline]
    pub(crate) fn read_wdt_reqstatus_rr2(&self) -> MemResult<bool> {
        todo!("read WDT REQSTATUS RR2 reset value false")
    }
    #[doc = "WDT REQSTATUS RR3: Request status for RR[3]."]
    #[inline]
    pub(crate) fn read_wdt_reqstatus_rr3(&self) -> MemResult<bool> {
        todo!("read WDT REQSTATUS RR3 reset value false")
    }
    #[doc = "WDT REQSTATUS RR4: Request status for RR[4]."]
    #[inline]
    pub(crate) fn read_wdt_reqstatus_rr4(&self) -> MemResult<bool> {
        todo!("read WDT REQSTATUS RR4 reset value false")
    }
    #[doc = "WDT REQSTATUS RR5: Request status for RR[5]."]
    #[inline]
    pub(crate) fn read_wdt_reqstatus_rr5(&self) -> MemResult<bool> {
        todo!("read WDT REQSTATUS RR5 reset value false")
    }
    #[doc = "WDT REQSTATUS RR6: Request status for RR[6]."]
    #[inline]
    pub(crate) fn read_wdt_reqstatus_rr6(&self) -> MemResult<bool> {
        todo!("read WDT REQSTATUS RR6 reset value false")
    }
    #[doc = "WDT REQSTATUS RR7: Request status for RR[7]."]
    #[inline]
    pub(crate) fn read_wdt_reqstatus_rr7(&self) -> MemResult<bool> {
        todo!("read WDT REQSTATUS RR7 reset value false")
    }
    #[doc = "WDT CRV: Counter reload value in number of 32kiHz clock cycles."]
    #[inline]
    pub(crate) fn read_wdt_crv(&self) -> MemResult<u32> {
        todo!("read WDT CRV reset value 0xffffffff mask 0xffffffffffffffff")
    }
    #[doc = "WDT CRV: Counter reload value in number of 32kiHz clock cycles."]
    #[inline]
    pub(crate) fn write_wdt_crv(&mut self, _value: u32) -> MemResult<()> {
        todo!("write WDT CRV reset value 0xffffffff mask 0xffffffffffffffff")
    }
    #[doc = "WDT RREN RR0: Enable or disable RR[0] register."]
    #[inline]
    pub(crate) fn read_wdt_rren_rr0(&self) -> MemResult<bool> {
        todo!("read WDT RREN RR0 reset value true")
    }
    #[doc = "WDT RREN RR0: Enable or disable RR[0] register."]
    #[inline]
    pub(crate) fn write_wdt_rren_rr0(&mut self, _value: bool) -> MemResult<()> {
        todo!("write WDT RREN RR0 reset value true")
    }
    #[doc = "WDT RREN RR1: Enable or disable RR[1] register."]
    #[inline]
    pub(crate) fn read_wdt_rren_rr1(&self) -> MemResult<bool> {
        todo!("read WDT RREN RR1 reset value false")
    }
    #[doc = "WDT RREN RR1: Enable or disable RR[1] register."]
    #[inline]
    pub(crate) fn write_wdt_rren_rr1(&mut self, _value: bool) -> MemResult<()> {
        todo!("write WDT RREN RR1 reset value false")
    }
    #[doc = "WDT RREN RR2: Enable or disable RR[2] register."]
    #[inline]
    pub(crate) fn read_wdt_rren_rr2(&self) -> MemResult<bool> {
        todo!("read WDT RREN RR2 reset value false")
    }
    #[doc = "WDT RREN RR2: Enable or disable RR[2] register."]
    #[inline]
    pub(crate) fn write_wdt_rren_rr2(&mut self, _value: bool) -> MemResult<()> {
        todo!("write WDT RREN RR2 reset value false")
    }
    #[doc = "WDT RREN RR3: Enable or disable RR[3] register."]
    #[inline]
    pub(crate) fn read_wdt_rren_rr3(&self) -> MemResult<bool> {
        todo!("read WDT RREN RR3 reset value false")
    }
    #[doc = "WDT RREN RR3: Enable or disable RR[3] register."]
    #[inline]
    pub(crate) fn write_wdt_rren_rr3(&mut self, _value: bool) -> MemResult<()> {
        todo!("write WDT RREN RR3 reset value false")
    }
    #[doc = "WDT RREN RR4: Enable or disable RR[4] register."]
    #[inline]
    pub(crate) fn read_wdt_rren_rr4(&self) -> MemResult<bool> {
        todo!("read WDT RREN RR4 reset value false")
    }
    #[doc = "WDT RREN RR4: Enable or disable RR[4] register."]
    #[inline]
    pub(crate) fn write_wdt_rren_rr4(&mut self, _value: bool) -> MemResult<()> {
        todo!("write WDT RREN RR4 reset value false")
    }
    #[doc = "WDT RREN RR5: Enable or disable RR[5] register."]
    #[inline]
    pub(crate) fn read_wdt_rren_rr5(&self) -> MemResult<bool> {
        todo!("read WDT RREN RR5 reset value false")
    }
    #[doc = "WDT RREN RR5: Enable or disable RR[5] register."]
    #[inline]
    pub(crate) fn write_wdt_rren_rr5(&mut self, _value: bool) -> MemResult<()> {
        todo!("write WDT RREN RR5 reset value false")
    }
    #[doc = "WDT RREN RR6: Enable or disable RR[6] register."]
    #[inline]
    pub(crate) fn read_wdt_rren_rr6(&self) -> MemResult<bool> {
        todo!("read WDT RREN RR6 reset value false")
    }
    #[doc = "WDT RREN RR6: Enable or disable RR[6] register."]
    #[inline]
    pub(crate) fn write_wdt_rren_rr6(&mut self, _value: bool) -> MemResult<()> {
        todo!("write WDT RREN RR6 reset value false")
    }
    #[doc = "WDT RREN RR7: Enable or disable RR[7] register."]
    #[inline]
    pub(crate) fn read_wdt_rren_rr7(&self) -> MemResult<bool> {
        todo!("read WDT RREN RR7 reset value false")
    }
    #[doc = "WDT RREN RR7: Enable or disable RR[7] register."]
    #[inline]
    pub(crate) fn write_wdt_rren_rr7(&mut self, _value: bool) -> MemResult<()> {
        todo!("write WDT RREN RR7 reset value false")
    }
    #[doc = "WDT CONFIG SLEEP: Configure the watchdog to pause or not while the CPU is sleeping."]
    #[inline]
    pub(crate) fn read_wdt_config_sleep(&self) -> MemResult<bool> {
        todo!("read WDT CONFIG SLEEP reset value true")
    }
    #[doc = "WDT CONFIG SLEEP: Configure the watchdog to pause or not while the CPU is sleeping."]
    #[inline]
    pub(crate) fn write_wdt_config_sleep(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write WDT CONFIG SLEEP reset value true")
    }
    #[doc = "WDT CONFIG HALT: Configure the watchdog to pause or not while the CPU is halted by the debugger."]
    #[inline]
    pub(crate) fn read_wdt_config_halt(&self) -> MemResult<bool> {
        todo!("read WDT CONFIG HALT reset value false")
    }
    #[doc = "WDT CONFIG HALT: Configure the watchdog to pause or not while the CPU is halted by the debugger."]
    #[inline]
    pub(crate) fn write_wdt_config_halt(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write WDT CONFIG HALT reset value false")
    }
    #[doc = "WDT RR[%s]: Reload requests registers."]
    #[inline]
    pub(crate) fn write_wdt_rrn(
        &mut self,
        _dim: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write WDT RR[%s] reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "WDT POWER POWER: Peripheral power control."]
    #[inline]
    pub(crate) fn read_wdt_power_power(&self) -> MemResult<bool> {
        todo!("read WDT POWER POWER reset value false")
    }
    #[doc = "WDT POWER POWER: Peripheral power control."]
    #[inline]
    pub(crate) fn write_wdt_power_power(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write WDT POWER POWER reset value false")
    }
    #[doc = "QDEC TASKS_START: Start the quadrature decoder."]
    #[inline]
    pub(crate) fn write_qdec_tasks_start(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write QDEC TASKS_START reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "QDEC TASKS_STOP: Stop the quadrature decoder."]
    #[inline]
    pub(crate) fn write_qdec_tasks_stop(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write QDEC TASKS_STOP reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "QDEC TASKS_READCLRACC: Transfers the content from ACC registers to ACCREAD registers, and clears the ACC registers."]
    #[inline]
    pub(crate) fn write_qdec_tasks_readclracc(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write QDEC TASKS_READCLRACC reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "QDEC EVENTS_SAMPLERDY: A new sample is written to the sample register."]
    #[inline]
    pub(crate) fn read_qdec_events_samplerdy(&self) -> MemResult<u32> {
        todo ! ("read QDEC EVENTS_SAMPLERDY reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "QDEC EVENTS_SAMPLERDY: A new sample is written to the sample register."]
    #[inline]
    pub(crate) fn write_qdec_events_samplerdy(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write QDEC EVENTS_SAMPLERDY reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "QDEC EVENTS_REPORTRDY: REPORTPER number of samples accumulated in ACC register, and ACC register different than zero."]
    #[inline]
    pub(crate) fn read_qdec_events_reportrdy(&self) -> MemResult<u32> {
        todo ! ("read QDEC EVENTS_REPORTRDY reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "QDEC EVENTS_REPORTRDY: REPORTPER number of samples accumulated in ACC register, and ACC register different than zero."]
    #[inline]
    pub(crate) fn write_qdec_events_reportrdy(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write QDEC EVENTS_REPORTRDY reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "QDEC EVENTS_ACCOF: ACC or ACCDBL register overflow."]
    #[inline]
    pub(crate) fn read_qdec_events_accof(&self) -> MemResult<u32> {
        todo!("read QDEC EVENTS_ACCOF reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "QDEC EVENTS_ACCOF: ACC or ACCDBL register overflow."]
    #[inline]
    pub(crate) fn write_qdec_events_accof(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!(
            "write QDEC EVENTS_ACCOF reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "QDEC SHORTS REPORTRDY_READCLRACC: Shortcut between REPORTRDY event and READCLRACC task."]
    #[inline]
    pub(crate) fn read_qdec_shorts_reportrdy_readclracc(
        &self,
    ) -> MemResult<bool> {
        todo!("read QDEC SHORTS REPORTRDY_READCLRACC reset value false")
    }
    #[doc = "QDEC SHORTS REPORTRDY_READCLRACC: Shortcut between REPORTRDY event and READCLRACC task."]
    #[inline]
    pub(crate) fn write_qdec_shorts_reportrdy_readclracc(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write QDEC SHORTS REPORTRDY_READCLRACC reset value false")
    }
    #[doc = "QDEC SHORTS SAMPLERDY_STOP: Shortcut between SAMPLERDY event and STOP task."]
    #[inline]
    pub(crate) fn read_qdec_shorts_samplerdy_stop(&self) -> MemResult<bool> {
        todo!("read QDEC SHORTS SAMPLERDY_STOP reset value false")
    }
    #[doc = "QDEC SHORTS SAMPLERDY_STOP: Shortcut between SAMPLERDY event and STOP task."]
    #[inline]
    pub(crate) fn write_qdec_shorts_samplerdy_stop(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write QDEC SHORTS SAMPLERDY_STOP reset value false")
    }
    #[doc = "QDEC INTENSET SAMPLERDY: Enable interrupt on SAMPLERDY event."]
    #[inline]
    pub(crate) fn read_qdec_intenset_samplerdy(&self) -> MemResult<bool> {
        todo!("read QDEC INTENSET SAMPLERDY reset value false")
    }
    #[doc = "QDEC INTENSET SAMPLERDY: Enable interrupt on SAMPLERDY event."]
    #[inline]
    pub(crate) fn write_qdec_intenset_samplerdy(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write QDEC INTENSET SAMPLERDY reset value false")
    }
    #[doc = "QDEC INTENSET REPORTRDY: Enable interrupt on REPORTRDY event."]
    #[inline]
    pub(crate) fn read_qdec_intenset_reportrdy(&self) -> MemResult<bool> {
        todo!("read QDEC INTENSET REPORTRDY reset value false")
    }
    #[doc = "QDEC INTENSET REPORTRDY: Enable interrupt on REPORTRDY event."]
    #[inline]
    pub(crate) fn write_qdec_intenset_reportrdy(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write QDEC INTENSET REPORTRDY reset value false")
    }
    #[doc = "QDEC INTENSET ACCOF: Enable interrupt on ACCOF event."]
    #[inline]
    pub(crate) fn read_qdec_intenset_accof(&self) -> MemResult<bool> {
        todo!("read QDEC INTENSET ACCOF reset value false")
    }
    #[doc = "QDEC INTENSET ACCOF: Enable interrupt on ACCOF event."]
    #[inline]
    pub(crate) fn write_qdec_intenset_accof(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write QDEC INTENSET ACCOF reset value false")
    }
    #[doc = "QDEC INTENCLR SAMPLERDY: Disable interrupt on SAMPLERDY event."]
    #[inline]
    pub(crate) fn read_qdec_intenclr_samplerdy(&self) -> MemResult<bool> {
        todo!("read QDEC INTENCLR SAMPLERDY reset value false")
    }
    #[doc = "QDEC INTENCLR SAMPLERDY: Disable interrupt on SAMPLERDY event."]
    #[inline]
    pub(crate) fn write_qdec_intenclr_samplerdy(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write QDEC INTENCLR SAMPLERDY reset value false")
    }
    #[doc = "QDEC INTENCLR REPORTRDY: Disable interrupt on REPORTRDY event."]
    #[inline]
    pub(crate) fn read_qdec_intenclr_reportrdy(&self) -> MemResult<bool> {
        todo!("read QDEC INTENCLR REPORTRDY reset value false")
    }
    #[doc = "QDEC INTENCLR REPORTRDY: Disable interrupt on REPORTRDY event."]
    #[inline]
    pub(crate) fn write_qdec_intenclr_reportrdy(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write QDEC INTENCLR REPORTRDY reset value false")
    }
    #[doc = "QDEC INTENCLR ACCOF: Disable interrupt on ACCOF event."]
    #[inline]
    pub(crate) fn read_qdec_intenclr_accof(&self) -> MemResult<bool> {
        todo!("read QDEC INTENCLR ACCOF reset value false")
    }
    #[doc = "QDEC INTENCLR ACCOF: Disable interrupt on ACCOF event."]
    #[inline]
    pub(crate) fn write_qdec_intenclr_accof(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write QDEC INTENCLR ACCOF reset value false")
    }
    #[doc = "QDEC ENABLE ENABLE: Enable or disable QDEC."]
    #[inline]
    pub(crate) fn read_qdec_enable_enable(&self) -> MemResult<bool> {
        todo!("read QDEC ENABLE ENABLE reset value false")
    }
    #[doc = "QDEC ENABLE ENABLE: Enable or disable QDEC."]
    #[inline]
    pub(crate) fn write_qdec_enable_enable(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write QDEC ENABLE ENABLE reset value false")
    }
    #[doc = "QDEC LEDPOL LEDPOL: LED output pin polarity."]
    #[inline]
    pub(crate) fn read_qdec_ledpol_ledpol(&self) -> MemResult<bool> {
        todo!("read QDEC LEDPOL LEDPOL reset value false")
    }
    #[doc = "QDEC LEDPOL LEDPOL: LED output pin polarity."]
    #[inline]
    pub(crate) fn write_qdec_ledpol_ledpol(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write QDEC LEDPOL LEDPOL reset value false")
    }
    #[doc = "QDEC SAMPLEPER SAMPLEPER: Sample period."]
    #[inline]
    pub(crate) fn read_qdec_sampleper_sampleper(&self) -> MemResult<u8> {
        todo!("read QDEC SAMPLEPER SAMPLEPER reset value 0x00 mask 0x07")
    }
    #[doc = "QDEC SAMPLEPER SAMPLEPER: Sample period."]
    #[inline]
    pub(crate) fn write_qdec_sampleper_sampleper(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write QDEC SAMPLEPER SAMPLEPER reset value 0x00 mask 0x07")
    }
    #[doc = "QDEC SAMPLE: Motion sample value."]
    #[inline]
    pub(crate) fn read_qdec_sample(&self) -> MemResult<u32> {
        todo!("read QDEC SAMPLE reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "QDEC REPORTPER REPORTPER: Number of samples to generate an EVENT_REPORTRDY."]
    #[inline]
    pub(crate) fn read_qdec_reportper_reportper(&self) -> MemResult<u8> {
        todo!("read QDEC REPORTPER REPORTPER reset value 0x00 mask 0x07")
    }
    #[doc = "QDEC REPORTPER REPORTPER: Number of samples to generate an EVENT_REPORTRDY."]
    #[inline]
    pub(crate) fn write_qdec_reportper_reportper(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write QDEC REPORTPER REPORTPER reset value 0x00 mask 0x07")
    }
    #[doc = "QDEC ACC: Accumulated valid transitions register."]
    #[inline]
    pub(crate) fn read_qdec_acc(&self) -> MemResult<u32> {
        todo!("read QDEC ACC reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "QDEC ACCREAD: Snapshot of ACC register. Value generated by the TASKS_READCLEACC task."]
    #[inline]
    pub(crate) fn read_qdec_accread(&self) -> MemResult<u32> {
        todo!("read QDEC ACCREAD reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "QDEC PSELLED: Pin select for LED output."]
    #[inline]
    pub(crate) fn read_qdec_pselled(&self) -> MemResult<u32> {
        todo!(
            "read QDEC PSELLED reset value 0xffffffff mask 0xffffffffffffffff"
        )
    }
    #[doc = "QDEC PSELLED: Pin select for LED output."]
    #[inline]
    pub(crate) fn write_qdec_pselled(&mut self, _value: u32) -> MemResult<()> {
        todo!(
            "write QDEC PSELLED reset value 0xffffffff mask 0xffffffffffffffff"
        )
    }
    #[doc = "QDEC PSELA: Pin select for phase A input."]
    #[inline]
    pub(crate) fn read_qdec_psela(&self) -> MemResult<u32> {
        todo!("read QDEC PSELA reset value 0xffffffff mask 0xffffffffffffffff")
    }
    #[doc = "QDEC PSELA: Pin select for phase A input."]
    #[inline]
    pub(crate) fn write_qdec_psela(&mut self, _value: u32) -> MemResult<()> {
        todo!("write QDEC PSELA reset value 0xffffffff mask 0xffffffffffffffff")
    }
    #[doc = "QDEC PSELB: Pin select for phase B input."]
    #[inline]
    pub(crate) fn read_qdec_pselb(&self) -> MemResult<u32> {
        todo!("read QDEC PSELB reset value 0xffffffff mask 0xffffffffffffffff")
    }
    #[doc = "QDEC PSELB: Pin select for phase B input."]
    #[inline]
    pub(crate) fn write_qdec_pselb(&mut self, _value: u32) -> MemResult<()> {
        todo!("write QDEC PSELB reset value 0xffffffff mask 0xffffffffffffffff")
    }
    #[doc = "QDEC DBFEN DBFEN: Enable debounce input filters."]
    #[inline]
    pub(crate) fn read_qdec_dbfen_dbfen(&self) -> MemResult<bool> {
        todo!("read QDEC DBFEN DBFEN reset value false")
    }
    #[doc = "QDEC DBFEN DBFEN: Enable debounce input filters."]
    #[inline]
    pub(crate) fn write_qdec_dbfen_dbfen(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write QDEC DBFEN DBFEN reset value false")
    }
    #[doc = "QDEC LEDPRE LEDPRE: Period in us the LED in switched on prior to sampling."]
    #[inline]
    pub(crate) fn read_qdec_ledpre_ledpre(&self) -> MemResult<u16> {
        todo!("read QDEC LEDPRE LEDPRE reset value 0x10 mask 0x1ff")
    }
    #[doc = "QDEC LEDPRE LEDPRE: Period in us the LED in switched on prior to sampling."]
    #[inline]
    pub(crate) fn write_qdec_ledpre_ledpre(
        &mut self,
        _value: u16,
    ) -> MemResult<()> {
        todo!("write QDEC LEDPRE LEDPRE reset value 0x10 mask 0x1ff")
    }
    #[doc = "QDEC ACCDBL ACCDBL: Accumulated double (error) transitions."]
    #[inline]
    pub(crate) fn read_qdec_accdbl_accdbl(&self) -> MemResult<u8> {
        todo!("read QDEC ACCDBL ACCDBL reset value 0x00 mask 0x0f")
    }
    #[doc = "QDEC ACCDBLREAD ACCDBLREAD: Snapshot of accumulated double (error) transitions."]
    #[inline]
    pub(crate) fn read_qdec_accdblread_accdblread(&self) -> MemResult<u8> {
        todo!("read QDEC ACCDBLREAD ACCDBLREAD reset value 0x00 mask 0x0f")
    }
    #[doc = "QDEC POWER POWER: Peripheral power control."]
    #[inline]
    pub(crate) fn read_qdec_power_power(&self) -> MemResult<bool> {
        todo!("read QDEC POWER POWER reset value false")
    }
    #[doc = "QDEC POWER POWER: Peripheral power control."]
    #[inline]
    pub(crate) fn write_qdec_power_power(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write QDEC POWER POWER reset value false")
    }
    #[doc = "LPCOMP TASKS_START: Start the comparator."]
    #[inline]
    pub(crate) fn write_lpcomp_tasks_start(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!(
            "write LPCOMP TASKS_START reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "LPCOMP TASKS_STOP: Stop the comparator."]
    #[inline]
    pub(crate) fn write_lpcomp_tasks_stop(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!(
            "write LPCOMP TASKS_STOP reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "LPCOMP TASKS_SAMPLE: Sample comparator value."]
    #[inline]
    pub(crate) fn write_lpcomp_tasks_sample(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write LPCOMP TASKS_SAMPLE reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "LPCOMP EVENTS_READY: LPCOMP is ready and output is valid."]
    #[inline]
    pub(crate) fn read_lpcomp_events_ready(&self) -> MemResult<u32> {
        todo!(
            "read LPCOMP EVENTS_READY reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "LPCOMP EVENTS_READY: LPCOMP is ready and output is valid."]
    #[inline]
    pub(crate) fn write_lpcomp_events_ready(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write LPCOMP EVENTS_READY reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "LPCOMP EVENTS_DOWN: Input voltage crossed the threshold going down."]
    #[inline]
    pub(crate) fn read_lpcomp_events_down(&self) -> MemResult<u32> {
        todo!(
            "read LPCOMP EVENTS_DOWN reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "LPCOMP EVENTS_DOWN: Input voltage crossed the threshold going down."]
    #[inline]
    pub(crate) fn write_lpcomp_events_down(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!(
            "write LPCOMP EVENTS_DOWN reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "LPCOMP EVENTS_UP: Input voltage crossed the threshold going up."]
    #[inline]
    pub(crate) fn read_lpcomp_events_up(&self) -> MemResult<u32> {
        todo!("read LPCOMP EVENTS_UP reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "LPCOMP EVENTS_UP: Input voltage crossed the threshold going up."]
    #[inline]
    pub(crate) fn write_lpcomp_events_up(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write LPCOMP EVENTS_UP reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "LPCOMP EVENTS_CROSS: Input voltage crossed the threshold in any direction."]
    #[inline]
    pub(crate) fn read_lpcomp_events_cross(&self) -> MemResult<u32> {
        todo!(
            "read LPCOMP EVENTS_CROSS reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "LPCOMP EVENTS_CROSS: Input voltage crossed the threshold in any direction."]
    #[inline]
    pub(crate) fn write_lpcomp_events_cross(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write LPCOMP EVENTS_CROSS reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "LPCOMP SHORTS READY_SAMPLE: Shortcut between READY event and SAMPLE task."]
    #[inline]
    pub(crate) fn read_lpcomp_shorts_ready_sample(&self) -> MemResult<bool> {
        todo!("read LPCOMP SHORTS READY_SAMPLE reset value false")
    }
    #[doc = "LPCOMP SHORTS READY_SAMPLE: Shortcut between READY event and SAMPLE task."]
    #[inline]
    pub(crate) fn write_lpcomp_shorts_ready_sample(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write LPCOMP SHORTS READY_SAMPLE reset value false")
    }
    #[doc = "LPCOMP SHORTS READY_STOP: Shortcut between RADY event and STOP task."]
    #[inline]
    pub(crate) fn read_lpcomp_shorts_ready_stop(&self) -> MemResult<bool> {
        todo!("read LPCOMP SHORTS READY_STOP reset value false")
    }
    #[doc = "LPCOMP SHORTS READY_STOP: Shortcut between RADY event and STOP task."]
    #[inline]
    pub(crate) fn write_lpcomp_shorts_ready_stop(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write LPCOMP SHORTS READY_STOP reset value false")
    }
    #[doc = "LPCOMP SHORTS DOWN_STOP: Shortcut between DOWN event and STOP task."]
    #[inline]
    pub(crate) fn read_lpcomp_shorts_down_stop(&self) -> MemResult<bool> {
        todo!("read LPCOMP SHORTS DOWN_STOP reset value false")
    }
    #[doc = "LPCOMP SHORTS DOWN_STOP: Shortcut between DOWN event and STOP task."]
    #[inline]
    pub(crate) fn write_lpcomp_shorts_down_stop(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write LPCOMP SHORTS DOWN_STOP reset value false")
    }
    #[doc = "LPCOMP SHORTS UP_STOP: Shortcut between UP event and STOP task."]
    #[inline]
    pub(crate) fn read_lpcomp_shorts_up_stop(&self) -> MemResult<bool> {
        todo!("read LPCOMP SHORTS UP_STOP reset value false")
    }
    #[doc = "LPCOMP SHORTS UP_STOP: Shortcut between UP event and STOP task."]
    #[inline]
    pub(crate) fn write_lpcomp_shorts_up_stop(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write LPCOMP SHORTS UP_STOP reset value false")
    }
    #[doc = "LPCOMP SHORTS CROSS_STOP: Shortcut between CROSS event and STOP task."]
    #[inline]
    pub(crate) fn read_lpcomp_shorts_cross_stop(&self) -> MemResult<bool> {
        todo!("read LPCOMP SHORTS CROSS_STOP reset value false")
    }
    #[doc = "LPCOMP SHORTS CROSS_STOP: Shortcut between CROSS event and STOP task."]
    #[inline]
    pub(crate) fn write_lpcomp_shorts_cross_stop(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write LPCOMP SHORTS CROSS_STOP reset value false")
    }
    #[doc = "LPCOMP INTENSET READY: Enable interrupt on READY event."]
    #[inline]
    pub(crate) fn read_lpcomp_intenset_ready(&self) -> MemResult<bool> {
        todo!("read LPCOMP INTENSET READY reset value false")
    }
    #[doc = "LPCOMP INTENSET READY: Enable interrupt on READY event."]
    #[inline]
    pub(crate) fn write_lpcomp_intenset_ready(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write LPCOMP INTENSET READY reset value false")
    }
    #[doc = "LPCOMP INTENSET DOWN: Enable interrupt on DOWN event."]
    #[inline]
    pub(crate) fn read_lpcomp_intenset_down(&self) -> MemResult<bool> {
        todo!("read LPCOMP INTENSET DOWN reset value false")
    }
    #[doc = "LPCOMP INTENSET DOWN: Enable interrupt on DOWN event."]
    #[inline]
    pub(crate) fn write_lpcomp_intenset_down(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write LPCOMP INTENSET DOWN reset value false")
    }
    #[doc = "LPCOMP INTENSET UP: Enable interrupt on UP event."]
    #[inline]
    pub(crate) fn read_lpcomp_intenset_up(&self) -> MemResult<bool> {
        todo!("read LPCOMP INTENSET UP reset value false")
    }
    #[doc = "LPCOMP INTENSET UP: Enable interrupt on UP event."]
    #[inline]
    pub(crate) fn write_lpcomp_intenset_up(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write LPCOMP INTENSET UP reset value false")
    }
    #[doc = "LPCOMP INTENSET CROSS: Enable interrupt on CROSS event."]
    #[inline]
    pub(crate) fn read_lpcomp_intenset_cross(&self) -> MemResult<bool> {
        todo!("read LPCOMP INTENSET CROSS reset value false")
    }
    #[doc = "LPCOMP INTENSET CROSS: Enable interrupt on CROSS event."]
    #[inline]
    pub(crate) fn write_lpcomp_intenset_cross(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write LPCOMP INTENSET CROSS reset value false")
    }
    #[doc = "LPCOMP INTENCLR READY: Disable interrupt on READY event."]
    #[inline]
    pub(crate) fn read_lpcomp_intenclr_ready(&self) -> MemResult<bool> {
        todo!("read LPCOMP INTENCLR READY reset value false")
    }
    #[doc = "LPCOMP INTENCLR READY: Disable interrupt on READY event."]
    #[inline]
    pub(crate) fn write_lpcomp_intenclr_ready(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write LPCOMP INTENCLR READY reset value false")
    }
    #[doc = "LPCOMP INTENCLR DOWN: Disable interrupt on DOWN event."]
    #[inline]
    pub(crate) fn read_lpcomp_intenclr_down(&self) -> MemResult<bool> {
        todo!("read LPCOMP INTENCLR DOWN reset value false")
    }
    #[doc = "LPCOMP INTENCLR DOWN: Disable interrupt on DOWN event."]
    #[inline]
    pub(crate) fn write_lpcomp_intenclr_down(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write LPCOMP INTENCLR DOWN reset value false")
    }
    #[doc = "LPCOMP INTENCLR UP: Disable interrupt on UP event."]
    #[inline]
    pub(crate) fn read_lpcomp_intenclr_up(&self) -> MemResult<bool> {
        todo!("read LPCOMP INTENCLR UP reset value false")
    }
    #[doc = "LPCOMP INTENCLR UP: Disable interrupt on UP event."]
    #[inline]
    pub(crate) fn write_lpcomp_intenclr_up(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write LPCOMP INTENCLR UP reset value false")
    }
    #[doc = "LPCOMP INTENCLR CROSS: Disable interrupt on CROSS event."]
    #[inline]
    pub(crate) fn read_lpcomp_intenclr_cross(&self) -> MemResult<bool> {
        todo!("read LPCOMP INTENCLR CROSS reset value false")
    }
    #[doc = "LPCOMP INTENCLR CROSS: Disable interrupt on CROSS event."]
    #[inline]
    pub(crate) fn write_lpcomp_intenclr_cross(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write LPCOMP INTENCLR CROSS reset value false")
    }
    #[doc = "LPCOMP RESULT RESULT: Result of last compare. Decision point SAMPLE task."]
    #[inline]
    pub(crate) fn read_lpcomp_result_result(&self) -> MemResult<bool> {
        todo!("read LPCOMP RESULT RESULT reset value false")
    }
    #[doc = "LPCOMP ENABLE ENABLE: Enable or disable LPCOMP."]
    #[inline]
    pub(crate) fn read_lpcomp_enable_enable(&self) -> MemResult<u8> {
        todo!("read LPCOMP ENABLE ENABLE reset value 0x00 mask 0x03")
    }
    #[doc = "LPCOMP ENABLE ENABLE: Enable or disable LPCOMP."]
    #[inline]
    pub(crate) fn write_lpcomp_enable_enable(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write LPCOMP ENABLE ENABLE reset value 0x00 mask 0x03")
    }
    #[doc = "LPCOMP PSEL PSEL: Analog input pin select."]
    #[inline]
    pub(crate) fn read_lpcomp_psel_psel(&self) -> MemResult<u8> {
        todo!("read LPCOMP PSEL PSEL reset value 0x00 mask 0x07")
    }
    #[doc = "LPCOMP PSEL PSEL: Analog input pin select."]
    #[inline]
    pub(crate) fn write_lpcomp_psel_psel(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write LPCOMP PSEL PSEL reset value 0x00 mask 0x07")
    }
    #[doc = "LPCOMP REFSEL REFSEL: Reference select."]
    #[inline]
    pub(crate) fn read_lpcomp_refsel_refsel(&self) -> MemResult<u8> {
        todo!("read LPCOMP REFSEL REFSEL reset value 0x00 mask 0x07")
    }
    #[doc = "LPCOMP REFSEL REFSEL: Reference select."]
    #[inline]
    pub(crate) fn write_lpcomp_refsel_refsel(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write LPCOMP REFSEL REFSEL reset value 0x00 mask 0x07")
    }
    #[doc = "LPCOMP EXTREFSEL EXTREFSEL: External analog reference pin selection."]
    #[inline]
    pub(crate) fn read_lpcomp_extrefsel_extrefsel(&self) -> MemResult<bool> {
        todo!("read LPCOMP EXTREFSEL EXTREFSEL reset value false")
    }
    #[doc = "LPCOMP EXTREFSEL EXTREFSEL: External analog reference pin selection."]
    #[inline]
    pub(crate) fn write_lpcomp_extrefsel_extrefsel(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write LPCOMP EXTREFSEL EXTREFSEL reset value false")
    }
    #[doc = "LPCOMP ANADETECT ANADETECT: Analog detect configuration."]
    #[inline]
    pub(crate) fn read_lpcomp_anadetect_anadetect(&self) -> MemResult<u8> {
        todo!("read LPCOMP ANADETECT ANADETECT reset value 0x00 mask 0x03")
    }
    #[doc = "LPCOMP ANADETECT ANADETECT: Analog detect configuration."]
    #[inline]
    pub(crate) fn write_lpcomp_anadetect_anadetect(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write LPCOMP ANADETECT ANADETECT reset value 0x00 mask 0x03")
    }
    #[doc = "LPCOMP POWER POWER: Peripheral power control."]
    #[inline]
    pub(crate) fn read_lpcomp_power_power(&self) -> MemResult<bool> {
        todo!("read LPCOMP POWER POWER reset value false")
    }
    #[doc = "LPCOMP POWER POWER: Peripheral power control."]
    #[inline]
    pub(crate) fn write_lpcomp_power_power(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write LPCOMP POWER POWER reset value false")
    }
    #[doc = "SWI UNUSED: Unused."]
    #[inline]
    pub(crate) fn read_swi_unused(&self) -> MemResult<u32> {
        todo!("read SWI UNUSED reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "NVMC READY READY: NVMC ready."]
    #[inline]
    pub(crate) fn read_nvmc_ready_ready(&self) -> MemResult<bool> {
        todo!("read NVMC READY READY reset value false")
    }
    #[doc = "NVMC CONFIG WEN: Program write enable."]
    #[inline]
    pub(crate) fn read_nvmc_config_wen(&self) -> MemResult<u8> {
        todo!("read NVMC CONFIG WEN reset value 0x00 mask 0x03")
    }
    #[doc = "NVMC CONFIG WEN: Program write enable."]
    #[inline]
    pub(crate) fn write_nvmc_config_wen(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write NVMC CONFIG WEN reset value 0x00 mask 0x03")
    }
    #[doc = "NVMC ERASEPAGE: Register for erasing a non-protected non-volatile memory page.\n\nNVMC ERASEPCR1: Register for erasing a non-protected non-volatile memory page."]
    #[inline]
    pub(crate) fn read_nvmc_erasepage(&self) -> MemResult<u32> {
        todo ! ("read NVMC ERASEPAGE, NVMC ERASEPCR1 reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "NVMC ERASEPAGE: Register for erasing a non-protected non-volatile memory page.\n\nNVMC ERASEPCR1: Register for erasing a non-protected non-volatile memory page."]
    #[inline]
    pub(crate) fn write_nvmc_erasepage(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write NVMC ERASEPAGE, NVMC ERASEPCR1 reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "NVMC ERASEALL ERASEALL: Starts the erasing of all user NVM (code region 0/1 and UICR registers)."]
    #[inline]
    pub(crate) fn read_nvmc_eraseall_eraseall(&self) -> MemResult<bool> {
        todo!("read NVMC ERASEALL ERASEALL reset value false")
    }
    #[doc = "NVMC ERASEALL ERASEALL: Starts the erasing of all user NVM (code region 0/1 and UICR registers)."]
    #[inline]
    pub(crate) fn write_nvmc_eraseall_eraseall(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write NVMC ERASEALL ERASEALL reset value false")
    }
    #[doc = "NVMC ERASEPCR0: Register for erasing a protected non-volatile memory page."]
    #[inline]
    pub(crate) fn read_nvmc_erasepcr0(&self) -> MemResult<u32> {
        todo!("read NVMC ERASEPCR0 reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "NVMC ERASEPCR0: Register for erasing a protected non-volatile memory page."]
    #[inline]
    pub(crate) fn write_nvmc_erasepcr0(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write NVMC ERASEPCR0 reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "NVMC ERASEUICR ERASEUICR: It can only be used when all contents of code region 1 are erased."]
    #[inline]
    pub(crate) fn read_nvmc_eraseuicr_eraseuicr(&self) -> MemResult<bool> {
        todo!("read NVMC ERASEUICR ERASEUICR reset value false")
    }
    #[doc = "NVMC ERASEUICR ERASEUICR: It can only be used when all contents of code region 1 are erased."]
    #[inline]
    pub(crate) fn write_nvmc_eraseuicr_eraseuicr(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write NVMC ERASEUICR ERASEUICR reset value false")
    }
    #[doc = "PPI CHEN CH0: Enable PPI channel 0."]
    #[inline]
    pub(crate) fn read_ppi_chen_ch0(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(0))
    }
    #[doc = "PPI CHEN CH0: Enable PPI channel 0."]
    #[inline]
    pub(crate) fn write_ppi_chen_ch0(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.ppi.set_on(0, _value))
    }
    #[doc = "PPI CHEN CH1: Enable PPI channel 1."]
    #[inline]
    pub(crate) fn read_ppi_chen_ch1(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(1))
    }
    #[doc = "PPI CHEN CH1: Enable PPI channel 1."]
    #[inline]
    pub(crate) fn write_ppi_chen_ch1(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.ppi.set_on(1, _value))
    }
    #[doc = "PPI CHEN CH2: Enable PPI channel 2."]
    #[inline]
    pub(crate) fn read_ppi_chen_ch2(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(2))
    }
    #[doc = "PPI CHEN CH2: Enable PPI channel 2."]
    #[inline]
    pub(crate) fn write_ppi_chen_ch2(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.ppi.set_on(2, _value))
    }
    #[doc = "PPI CHEN CH3: Enable PPI channel 3."]
    #[inline]
    pub(crate) fn read_ppi_chen_ch3(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(3))
    }
    #[doc = "PPI CHEN CH3: Enable PPI channel 3."]
    #[inline]
    pub(crate) fn write_ppi_chen_ch3(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.ppi.set_on(3, _value))
    }
    #[doc = "PPI CHEN CH4: Enable PPI channel 4."]
    #[inline]
    pub(crate) fn read_ppi_chen_ch4(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(4))
    }
    #[doc = "PPI CHEN CH4: Enable PPI channel 4."]
    #[inline]
    pub(crate) fn write_ppi_chen_ch4(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.ppi.set_on(4, _value))
    }
    #[doc = "PPI CHEN CH5: Enable PPI channel 5."]
    #[inline]
    pub(crate) fn read_ppi_chen_ch5(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(5))
    }
    #[doc = "PPI CHEN CH5: Enable PPI channel 5."]
    #[inline]
    pub(crate) fn write_ppi_chen_ch5(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.ppi.set_on(5, _value))
    }
    #[doc = "PPI CHEN CH6: Enable PPI channel 6."]
    #[inline]
    pub(crate) fn read_ppi_chen_ch6(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(6))
    }
    #[doc = "PPI CHEN CH6: Enable PPI channel 6."]
    #[inline]
    pub(crate) fn write_ppi_chen_ch6(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.ppi.set_on(6, _value))
    }
    #[doc = "PPI CHEN CH7: Enable PPI channel 7."]
    #[inline]
    pub(crate) fn read_ppi_chen_ch7(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(7))
    }
    #[doc = "PPI CHEN CH7: Enable PPI channel 7."]
    #[inline]
    pub(crate) fn write_ppi_chen_ch7(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.ppi.set_on(7, _value))
    }
    #[doc = "PPI CHEN CH8: Enable PPI channel 8."]
    #[inline]
    pub(crate) fn read_ppi_chen_ch8(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(8))
    }
    #[doc = "PPI CHEN CH8: Enable PPI channel 8."]
    #[inline]
    pub(crate) fn write_ppi_chen_ch8(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.ppi.set_on(8, _value))
    }
    #[doc = "PPI CHEN CH9: Enable PPI channel 9."]
    #[inline]
    pub(crate) fn read_ppi_chen_ch9(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(9))
    }
    #[doc = "PPI CHEN CH9: Enable PPI channel 9."]
    #[inline]
    pub(crate) fn write_ppi_chen_ch9(&mut self, _value: bool) -> MemResult<()> {
        Ok(self.ppi.set_on(9, _value))
    }
    #[doc = "PPI CHEN CH10: Enable PPI channel 10."]
    #[inline]
    pub(crate) fn read_ppi_chen_ch10(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(10))
    }
    #[doc = "PPI CHEN CH10: Enable PPI channel 10."]
    #[inline]
    pub(crate) fn write_ppi_chen_ch10(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_on(10, _value))
    }
    #[doc = "PPI CHEN CH11: Enable PPI channel 11."]
    #[inline]
    pub(crate) fn read_ppi_chen_ch11(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(11))
    }
    #[doc = "PPI CHEN CH11: Enable PPI channel 11."]
    #[inline]
    pub(crate) fn write_ppi_chen_ch11(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_on(11, _value))
    }
    #[doc = "PPI CHEN CH12: Enable PPI channel 12."]
    #[inline]
    pub(crate) fn read_ppi_chen_ch12(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(12))
    }
    #[doc = "PPI CHEN CH12: Enable PPI channel 12."]
    #[inline]
    pub(crate) fn write_ppi_chen_ch12(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_on(12, _value))
    }
    #[doc = "PPI CHEN CH13: Enable PPI channel 13."]
    #[inline]
    pub(crate) fn read_ppi_chen_ch13(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(13))
    }
    #[doc = "PPI CHEN CH13: Enable PPI channel 13."]
    #[inline]
    pub(crate) fn write_ppi_chen_ch13(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_on(13, _value))
    }
    #[doc = "PPI CHEN CH14: Enable PPI channel 14."]
    #[inline]
    pub(crate) fn read_ppi_chen_ch14(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(14))
    }
    #[doc = "PPI CHEN CH14: Enable PPI channel 14."]
    #[inline]
    pub(crate) fn write_ppi_chen_ch14(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_on(14, _value))
    }
    #[doc = "PPI CHEN CH15: Enable PPI channel 15."]
    #[inline]
    pub(crate) fn read_ppi_chen_ch15(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(15))
    }
    #[doc = "PPI CHEN CH15: Enable PPI channel 15."]
    #[inline]
    pub(crate) fn write_ppi_chen_ch15(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_on(15, _value))
    }
    #[doc = "PPI CHEN CH20: Enable PPI channel 20."]
    #[inline]
    pub(crate) fn read_ppi_chen_ch20(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(20))
    }
    #[doc = "PPI CHEN CH20: Enable PPI channel 20."]
    #[inline]
    pub(crate) fn write_ppi_chen_ch20(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_on(20, _value))
    }
    #[doc = "PPI CHEN CH21: Enable PPI channel 21."]
    #[inline]
    pub(crate) fn read_ppi_chen_ch21(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(21))
    }
    #[doc = "PPI CHEN CH21: Enable PPI channel 21."]
    #[inline]
    pub(crate) fn write_ppi_chen_ch21(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_on(21, _value))
    }
    #[doc = "PPI CHEN CH22: Enable PPI channel 22."]
    #[inline]
    pub(crate) fn read_ppi_chen_ch22(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(22))
    }
    #[doc = "PPI CHEN CH22: Enable PPI channel 22."]
    #[inline]
    pub(crate) fn write_ppi_chen_ch22(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_on(22, _value))
    }
    #[doc = "PPI CHEN CH23: Enable PPI channel 23."]
    #[inline]
    pub(crate) fn read_ppi_chen_ch23(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(23))
    }
    #[doc = "PPI CHEN CH23: Enable PPI channel 23."]
    #[inline]
    pub(crate) fn write_ppi_chen_ch23(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_on(23, _value))
    }
    #[doc = "PPI CHEN CH24: Enable PPI channel 24."]
    #[inline]
    pub(crate) fn read_ppi_chen_ch24(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(24))
    }
    #[doc = "PPI CHEN CH24: Enable PPI channel 24."]
    #[inline]
    pub(crate) fn write_ppi_chen_ch24(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_on(24, _value))
    }
    #[doc = "PPI CHEN CH25: Enable PPI channel 25."]
    #[inline]
    pub(crate) fn read_ppi_chen_ch25(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(25))
    }
    #[doc = "PPI CHEN CH25: Enable PPI channel 25."]
    #[inline]
    pub(crate) fn write_ppi_chen_ch25(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_on(25, _value))
    }
    #[doc = "PPI CHEN CH26: Enable PPI channel 26."]
    #[inline]
    pub(crate) fn read_ppi_chen_ch26(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(26))
    }
    #[doc = "PPI CHEN CH26: Enable PPI channel 26."]
    #[inline]
    pub(crate) fn write_ppi_chen_ch26(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_on(26, _value))
    }
    #[doc = "PPI CHEN CH27: Enable PPI channel 27."]
    #[inline]
    pub(crate) fn read_ppi_chen_ch27(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(27))
    }
    #[doc = "PPI CHEN CH27: Enable PPI channel 27."]
    #[inline]
    pub(crate) fn write_ppi_chen_ch27(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_on(27, _value))
    }
    #[doc = "PPI CHEN CH28: Enable PPI channel 28."]
    #[inline]
    pub(crate) fn read_ppi_chen_ch28(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(28))
    }
    #[doc = "PPI CHEN CH28: Enable PPI channel 28."]
    #[inline]
    pub(crate) fn write_ppi_chen_ch28(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_on(28, _value))
    }
    #[doc = "PPI CHEN CH29: Enable PPI channel 29."]
    #[inline]
    pub(crate) fn read_ppi_chen_ch29(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(29))
    }
    #[doc = "PPI CHEN CH29: Enable PPI channel 29."]
    #[inline]
    pub(crate) fn write_ppi_chen_ch29(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_on(29, _value))
    }
    #[doc = "PPI CHEN CH30: Enable PPI channel 30."]
    #[inline]
    pub(crate) fn read_ppi_chen_ch30(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(30))
    }
    #[doc = "PPI CHEN CH30: Enable PPI channel 30."]
    #[inline]
    pub(crate) fn write_ppi_chen_ch30(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_on(30, _value))
    }
    #[doc = "PPI CHEN CH31: Enable PPI channel 31."]
    #[inline]
    pub(crate) fn read_ppi_chen_ch31(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(31))
    }
    #[doc = "PPI CHEN CH31: Enable PPI channel 31."]
    #[inline]
    pub(crate) fn write_ppi_chen_ch31(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_on(31, _value))
    }
    #[doc = "PPI CHENSET CH0: Enable PPI channel 0."]
    #[inline]
    pub(crate) fn read_ppi_chenset_ch0(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(0))
    }
    #[doc = "PPI CHENSET CH0: Enable PPI channel 0."]
    #[inline]
    pub(crate) fn write_ppi_chenset_ch0(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(0, true);
        }
        Ok(())
    }
    #[doc = "PPI CHENSET CH1: Enable PPI channel 1."]
    #[inline]
    pub(crate) fn read_ppi_chenset_ch1(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(1))
    }
    #[doc = "PPI CHENSET CH1: Enable PPI channel 1."]
    #[inline]
    pub(crate) fn write_ppi_chenset_ch1(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(1, true);
        }
        Ok(())
    }
    #[doc = "PPI CHENSET CH2: Enable PPI channel 2."]
    #[inline]
    pub(crate) fn read_ppi_chenset_ch2(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(2))
    }
    #[doc = "PPI CHENSET CH2: Enable PPI channel 2."]
    #[inline]
    pub(crate) fn write_ppi_chenset_ch2(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(2, true);
        }
        Ok(())
    }
    #[doc = "PPI CHENSET CH3: Enable PPI channel 3."]
    #[inline]
    pub(crate) fn read_ppi_chenset_ch3(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(3))
    }
    #[doc = "PPI CHENSET CH3: Enable PPI channel 3."]
    #[inline]
    pub(crate) fn write_ppi_chenset_ch3(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(3, true);
        }
        Ok(())
    }
    #[doc = "PPI CHENSET CH4: Enable PPI channel 4."]
    #[inline]
    pub(crate) fn read_ppi_chenset_ch4(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(4))
    }
    #[doc = "PPI CHENSET CH4: Enable PPI channel 4."]
    #[inline]
    pub(crate) fn write_ppi_chenset_ch4(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(4, true);
        }
        Ok(())
    }
    #[doc = "PPI CHENSET CH5: Enable PPI channel 5."]
    #[inline]
    pub(crate) fn read_ppi_chenset_ch5(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(5))
    }
    #[doc = "PPI CHENSET CH5: Enable PPI channel 5."]
    #[inline]
    pub(crate) fn write_ppi_chenset_ch5(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(5, true);
        }
        Ok(())
    }
    #[doc = "PPI CHENSET CH6: Enable PPI channel 6."]
    #[inline]
    pub(crate) fn read_ppi_chenset_ch6(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(6))
    }
    #[doc = "PPI CHENSET CH6: Enable PPI channel 6."]
    #[inline]
    pub(crate) fn write_ppi_chenset_ch6(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(6, true);
        }
        Ok(())
    }
    #[doc = "PPI CHENSET CH7: Enable PPI channel 7."]
    #[inline]
    pub(crate) fn read_ppi_chenset_ch7(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(7))
    }
    #[doc = "PPI CHENSET CH7: Enable PPI channel 7."]
    #[inline]
    pub(crate) fn write_ppi_chenset_ch7(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(7, true);
        }
        Ok(())
    }
    #[doc = "PPI CHENSET CH8: Enable PPI channel 8."]
    #[inline]
    pub(crate) fn read_ppi_chenset_ch8(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(8))
    }
    #[doc = "PPI CHENSET CH8: Enable PPI channel 8."]
    #[inline]
    pub(crate) fn write_ppi_chenset_ch8(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(8, true);
        }
        Ok(())
    }
    #[doc = "PPI CHENSET CH9: Enable PPI channel 9."]
    #[inline]
    pub(crate) fn read_ppi_chenset_ch9(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(9))
    }
    #[doc = "PPI CHENSET CH9: Enable PPI channel 9."]
    #[inline]
    pub(crate) fn write_ppi_chenset_ch9(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(9, true);
        }
        Ok(())
    }
    #[doc = "PPI CHENSET CH10: Enable PPI channel 10."]
    #[inline]
    pub(crate) fn read_ppi_chenset_ch10(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(10))
    }
    #[doc = "PPI CHENSET CH10: Enable PPI channel 10."]
    #[inline]
    pub(crate) fn write_ppi_chenset_ch10(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(10, true);
        }
        Ok(())
    }
    #[doc = "PPI CHENSET CH11: Enable PPI channel 11."]
    #[inline]
    pub(crate) fn read_ppi_chenset_ch11(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(11))
    }
    #[doc = "PPI CHENSET CH11: Enable PPI channel 11."]
    #[inline]
    pub(crate) fn write_ppi_chenset_ch11(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(11, true);
        }
        Ok(())
    }
    #[doc = "PPI CHENSET CH12: Enable PPI channel 12."]
    #[inline]
    pub(crate) fn read_ppi_chenset_ch12(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(12))
    }
    #[doc = "PPI CHENSET CH12: Enable PPI channel 12."]
    #[inline]
    pub(crate) fn write_ppi_chenset_ch12(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(12, true);
        }
        Ok(())
    }
    #[doc = "PPI CHENSET CH13: Enable PPI channel 13."]
    #[inline]
    pub(crate) fn read_ppi_chenset_ch13(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(13))
    }
    #[doc = "PPI CHENSET CH13: Enable PPI channel 13."]
    #[inline]
    pub(crate) fn write_ppi_chenset_ch13(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(13, true);
        }
        Ok(())
    }
    #[doc = "PPI CHENSET CH14: Enable PPI channel 14."]
    #[inline]
    pub(crate) fn read_ppi_chenset_ch14(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(14))
    }
    #[doc = "PPI CHENSET CH14: Enable PPI channel 14."]
    #[inline]
    pub(crate) fn write_ppi_chenset_ch14(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(14, true);
        }
        Ok(())
    }
    #[doc = "PPI CHENSET CH15: Enable PPI channel 15."]
    #[inline]
    pub(crate) fn read_ppi_chenset_ch15(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(15))
    }
    #[doc = "PPI CHENSET CH15: Enable PPI channel 15."]
    #[inline]
    pub(crate) fn write_ppi_chenset_ch15(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(15, true);
        }
        Ok(())
    }
    #[doc = "PPI CHENSET CH20: Enable PPI channel 20."]
    #[inline]
    pub(crate) fn read_ppi_chenset_ch20(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(20))
    }
    #[doc = "PPI CHENSET CH20: Enable PPI channel 20."]
    #[inline]
    pub(crate) fn write_ppi_chenset_ch20(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(20, true);
        }
        Ok(())
    }
    #[doc = "PPI CHENSET CH21: Enable PPI channel 21."]
    #[inline]
    pub(crate) fn read_ppi_chenset_ch21(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(21))
    }
    #[doc = "PPI CHENSET CH21: Enable PPI channel 21."]
    #[inline]
    pub(crate) fn write_ppi_chenset_ch21(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(21, true);
        }
        Ok(())
    }
    #[doc = "PPI CHENSET CH22: Enable PPI channel 22."]
    #[inline]
    pub(crate) fn read_ppi_chenset_ch22(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(22))
    }
    #[doc = "PPI CHENSET CH22: Enable PPI channel 22."]
    #[inline]
    pub(crate) fn write_ppi_chenset_ch22(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(22, true);
        }
        Ok(())
    }
    #[doc = "PPI CHENSET CH23: Enable PPI channel 23."]
    #[inline]
    pub(crate) fn read_ppi_chenset_ch23(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(23))
    }
    #[doc = "PPI CHENSET CH23: Enable PPI channel 23."]
    #[inline]
    pub(crate) fn write_ppi_chenset_ch23(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(23, true);
        }
        Ok(())
    }
    #[doc = "PPI CHENSET CH24: Enable PPI channel 24."]
    #[inline]
    pub(crate) fn read_ppi_chenset_ch24(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(24))
    }
    #[doc = "PPI CHENSET CH24: Enable PPI channel 24."]
    #[inline]
    pub(crate) fn write_ppi_chenset_ch24(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(24, true);
        }
        Ok(())
    }
    #[doc = "PPI CHENSET CH25: Enable PPI channel 25."]
    #[inline]
    pub(crate) fn read_ppi_chenset_ch25(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(25))
    }
    #[doc = "PPI CHENSET CH25: Enable PPI channel 25."]
    #[inline]
    pub(crate) fn write_ppi_chenset_ch25(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(25, true);
        }
        Ok(())
    }
    #[doc = "PPI CHENSET CH26: Enable PPI channel 26."]
    #[inline]
    pub(crate) fn read_ppi_chenset_ch26(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(26))
    }
    #[doc = "PPI CHENSET CH26: Enable PPI channel 26."]
    #[inline]
    pub(crate) fn write_ppi_chenset_ch26(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(26, true);
        }
        Ok(())
    }
    #[doc = "PPI CHENSET CH27: Enable PPI channel 27."]
    #[inline]
    pub(crate) fn read_ppi_chenset_ch27(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(27))
    }
    #[doc = "PPI CHENSET CH27: Enable PPI channel 27."]
    #[inline]
    pub(crate) fn write_ppi_chenset_ch27(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(27, true);
        }
        Ok(())
    }
    #[doc = "PPI CHENSET CH28: Enable PPI channel 28."]
    #[inline]
    pub(crate) fn read_ppi_chenset_ch28(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(28))
    }
    #[doc = "PPI CHENSET CH28: Enable PPI channel 28."]
    #[inline]
    pub(crate) fn write_ppi_chenset_ch28(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(28, true);
        }
        Ok(())
    }
    #[doc = "PPI CHENSET CH29: Enable PPI channel 29."]
    #[inline]
    pub(crate) fn read_ppi_chenset_ch29(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(29))
    }
    #[doc = "PPI CHENSET CH29: Enable PPI channel 29."]
    #[inline]
    pub(crate) fn write_ppi_chenset_ch29(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(29, true);
        }
        Ok(())
    }
    #[doc = "PPI CHENSET CH30: Enable PPI channel 30."]
    #[inline]
    pub(crate) fn read_ppi_chenset_ch30(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(30))
    }
    #[doc = "PPI CHENSET CH30: Enable PPI channel 30."]
    #[inline]
    pub(crate) fn write_ppi_chenset_ch30(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(30, true);
        }
        Ok(())
    }
    #[doc = "PPI CHENSET CH31: Enable PPI channel 31."]
    #[inline]
    pub(crate) fn read_ppi_chenset_ch31(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(31))
    }
    #[doc = "PPI CHENSET CH31: Enable PPI channel 31."]
    #[inline]
    pub(crate) fn write_ppi_chenset_ch31(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(31, true);
        }
        Ok(())
    }
    #[doc = "PPI CHENCLR CH0: Disable PPI channel 0."]
    #[inline]
    pub(crate) fn read_ppi_chenclr_ch0(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(0))
    }
    #[doc = "PPI CHENCLR CH0: Disable PPI channel 0."]
    #[inline]
    pub(crate) fn write_ppi_chenclr_ch0(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(0, false);
        }
        Ok(())
    }
    #[doc = "PPI CHENCLR CH1: Disable PPI channel 1."]
    #[inline]
    pub(crate) fn read_ppi_chenclr_ch1(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(1))
    }
    #[doc = "PPI CHENCLR CH1: Disable PPI channel 1."]
    #[inline]
    pub(crate) fn write_ppi_chenclr_ch1(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(1, false);
        }
        Ok(())
    }
    #[doc = "PPI CHENCLR CH2: Disable PPI channel 2."]
    #[inline]
    pub(crate) fn read_ppi_chenclr_ch2(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(2))
    }
    #[doc = "PPI CHENCLR CH2: Disable PPI channel 2."]
    #[inline]
    pub(crate) fn write_ppi_chenclr_ch2(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(2, false);
        }
        Ok(())
    }
    #[doc = "PPI CHENCLR CH3: Disable PPI channel 3."]
    #[inline]
    pub(crate) fn read_ppi_chenclr_ch3(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(3))
    }
    #[doc = "PPI CHENCLR CH3: Disable PPI channel 3."]
    #[inline]
    pub(crate) fn write_ppi_chenclr_ch3(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(3, false);
        }
        Ok(())
    }
    #[doc = "PPI CHENCLR CH4: Disable PPI channel 4."]
    #[inline]
    pub(crate) fn read_ppi_chenclr_ch4(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(4))
    }
    #[doc = "PPI CHENCLR CH4: Disable PPI channel 4."]
    #[inline]
    pub(crate) fn write_ppi_chenclr_ch4(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(4, false);
        }
        Ok(())
    }
    #[doc = "PPI CHENCLR CH5: Disable PPI channel 5."]
    #[inline]
    pub(crate) fn read_ppi_chenclr_ch5(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(5))
    }
    #[doc = "PPI CHENCLR CH5: Disable PPI channel 5."]
    #[inline]
    pub(crate) fn write_ppi_chenclr_ch5(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(5, false);
        }
        Ok(())
    }
    #[doc = "PPI CHENCLR CH6: Disable PPI channel 6."]
    #[inline]
    pub(crate) fn read_ppi_chenclr_ch6(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(6))
    }
    #[doc = "PPI CHENCLR CH6: Disable PPI channel 6."]
    #[inline]
    pub(crate) fn write_ppi_chenclr_ch6(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(6, false);
        }
        Ok(())
    }
    #[doc = "PPI CHENCLR CH7: Disable PPI channel 7."]
    #[inline]
    pub(crate) fn read_ppi_chenclr_ch7(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(7))
    }
    #[doc = "PPI CHENCLR CH7: Disable PPI channel 7."]
    #[inline]
    pub(crate) fn write_ppi_chenclr_ch7(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(7, false);
        }
        Ok(())
    }
    #[doc = "PPI CHENCLR CH8: Disable PPI channel 8."]
    #[inline]
    pub(crate) fn read_ppi_chenclr_ch8(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(8))
    }
    #[doc = "PPI CHENCLR CH8: Disable PPI channel 8."]
    #[inline]
    pub(crate) fn write_ppi_chenclr_ch8(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(8, false);
        }
        Ok(())
    }
    #[doc = "PPI CHENCLR CH9: Disable PPI channel 9."]
    #[inline]
    pub(crate) fn read_ppi_chenclr_ch9(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(9))
    }
    #[doc = "PPI CHENCLR CH9: Disable PPI channel 9."]
    #[inline]
    pub(crate) fn write_ppi_chenclr_ch9(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(9, false);
        }
        Ok(())
    }
    #[doc = "PPI CHENCLR CH10: Disable PPI channel 10."]
    #[inline]
    pub(crate) fn read_ppi_chenclr_ch10(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(10))
    }
    #[doc = "PPI CHENCLR CH10: Disable PPI channel 10."]
    #[inline]
    pub(crate) fn write_ppi_chenclr_ch10(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(10, false);
        }
        Ok(())
    }
    #[doc = "PPI CHENCLR CH11: Disable PPI channel 11."]
    #[inline]
    pub(crate) fn read_ppi_chenclr_ch11(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(11))
    }
    #[doc = "PPI CHENCLR CH11: Disable PPI channel 11."]
    #[inline]
    pub(crate) fn write_ppi_chenclr_ch11(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(11, false);
        }
        Ok(())
    }
    #[doc = "PPI CHENCLR CH12: Disable PPI channel 12."]
    #[inline]
    pub(crate) fn read_ppi_chenclr_ch12(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(12))
    }
    #[doc = "PPI CHENCLR CH12: Disable PPI channel 12."]
    #[inline]
    pub(crate) fn write_ppi_chenclr_ch12(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(12, false);
        }
        Ok(())
    }
    #[doc = "PPI CHENCLR CH13: Disable PPI channel 13."]
    #[inline]
    pub(crate) fn read_ppi_chenclr_ch13(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(13))
    }
    #[doc = "PPI CHENCLR CH13: Disable PPI channel 13."]
    #[inline]
    pub(crate) fn write_ppi_chenclr_ch13(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(13, false);
        }
        Ok(())
    }
    #[doc = "PPI CHENCLR CH14: Disable PPI channel 14."]
    #[inline]
    pub(crate) fn read_ppi_chenclr_ch14(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(14))
    }
    #[doc = "PPI CHENCLR CH14: Disable PPI channel 14."]
    #[inline]
    pub(crate) fn write_ppi_chenclr_ch14(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(14, false);
        }
        Ok(())
    }
    #[doc = "PPI CHENCLR CH15: Disable PPI channel 15."]
    #[inline]
    pub(crate) fn read_ppi_chenclr_ch15(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(15))
    }
    #[doc = "PPI CHENCLR CH15: Disable PPI channel 15."]
    #[inline]
    pub(crate) fn write_ppi_chenclr_ch15(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(15, false);
        }
        Ok(())
    }
    #[doc = "PPI CHENCLR CH20: Disable PPI channel 20."]
    #[inline]
    pub(crate) fn read_ppi_chenclr_ch20(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(20))
    }
    #[doc = "PPI CHENCLR CH20: Disable PPI channel 20."]
    #[inline]
    pub(crate) fn write_ppi_chenclr_ch20(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(20, false);
        }
        Ok(())
    }
    #[doc = "PPI CHENCLR CH21: Disable PPI channel 21."]
    #[inline]
    pub(crate) fn read_ppi_chenclr_ch21(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(21))
    }
    #[doc = "PPI CHENCLR CH21: Disable PPI channel 21."]
    #[inline]
    pub(crate) fn write_ppi_chenclr_ch21(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(21, false);
        }
        Ok(())
    }
    #[doc = "PPI CHENCLR CH22: Disable PPI channel 22."]
    #[inline]
    pub(crate) fn read_ppi_chenclr_ch22(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(22))
    }
    #[doc = "PPI CHENCLR CH22: Disable PPI channel 22."]
    #[inline]
    pub(crate) fn write_ppi_chenclr_ch22(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(22, false);
        }
        Ok(())
    }
    #[doc = "PPI CHENCLR CH23: Disable PPI channel 23."]
    #[inline]
    pub(crate) fn read_ppi_chenclr_ch23(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(23))
    }
    #[doc = "PPI CHENCLR CH23: Disable PPI channel 23."]
    #[inline]
    pub(crate) fn write_ppi_chenclr_ch23(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(23, false);
        }
        Ok(())
    }
    #[doc = "PPI CHENCLR CH24: Disable PPI channel 24."]
    #[inline]
    pub(crate) fn read_ppi_chenclr_ch24(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(24))
    }
    #[doc = "PPI CHENCLR CH24: Disable PPI channel 24."]
    #[inline]
    pub(crate) fn write_ppi_chenclr_ch24(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(24, false);
        }
        Ok(())
    }
    #[doc = "PPI CHENCLR CH25: Disable PPI channel 25."]
    #[inline]
    pub(crate) fn read_ppi_chenclr_ch25(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(25))
    }
    #[doc = "PPI CHENCLR CH25: Disable PPI channel 25."]
    #[inline]
    pub(crate) fn write_ppi_chenclr_ch25(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(25, false);
        }
        Ok(())
    }
    #[doc = "PPI CHENCLR CH26: Disable PPI channel 26."]
    #[inline]
    pub(crate) fn read_ppi_chenclr_ch26(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(26))
    }
    #[doc = "PPI CHENCLR CH26: Disable PPI channel 26."]
    #[inline]
    pub(crate) fn write_ppi_chenclr_ch26(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(26, false);
        }
        Ok(())
    }
    #[doc = "PPI CHENCLR CH27: Disable PPI channel 27."]
    #[inline]
    pub(crate) fn read_ppi_chenclr_ch27(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(27))
    }
    #[doc = "PPI CHENCLR CH27: Disable PPI channel 27."]
    #[inline]
    pub(crate) fn write_ppi_chenclr_ch27(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(27, false);
        }
        Ok(())
    }
    #[doc = "PPI CHENCLR CH28: Disable PPI channel 28."]
    #[inline]
    pub(crate) fn read_ppi_chenclr_ch28(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(28))
    }
    #[doc = "PPI CHENCLR CH28: Disable PPI channel 28."]
    #[inline]
    pub(crate) fn write_ppi_chenclr_ch28(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(28, false);
        }
        Ok(())
    }
    #[doc = "PPI CHENCLR CH29: Disable PPI channel 29."]
    #[inline]
    pub(crate) fn read_ppi_chenclr_ch29(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(29))
    }
    #[doc = "PPI CHENCLR CH29: Disable PPI channel 29."]
    #[inline]
    pub(crate) fn write_ppi_chenclr_ch29(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(29, false);
        }
        Ok(())
    }
    #[doc = "PPI CHENCLR CH30: Disable PPI channel 30."]
    #[inline]
    pub(crate) fn read_ppi_chenclr_ch30(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(30))
    }
    #[doc = "PPI CHENCLR CH30: Disable PPI channel 30."]
    #[inline]
    pub(crate) fn write_ppi_chenclr_ch30(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(30, false);
        }
        Ok(())
    }
    #[doc = "PPI CHENCLR CH31: Disable PPI channel 31."]
    #[inline]
    pub(crate) fn read_ppi_chenclr_ch31(&self) -> MemResult<bool> {
        Ok(self.ppi.is_on(31))
    }
    #[doc = "PPI CHENCLR CH31: Disable PPI channel 31."]
    #[inline]
    pub(crate) fn write_ppi_chenclr_ch31(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.ppi.set_on(31, false);
        }
        Ok(())
    }
    #[doc = "PPI CHG[%s] CH0: Include CH0 in channel group."]
    #[inline]
    pub(crate) fn read_ppi_chgn_ch0(&self, _dim: usize) -> MemResult<bool> {
        Ok(self.ppi.is_included(_dim, 0))
    }
    #[doc = "PPI CHG[%s] CH0: Include CH0 in channel group."]
    #[inline]
    pub(crate) fn write_ppi_chgn_ch0(
        &mut self,
        _dim: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_included(_dim, 0, _value))
    }
    #[doc = "PPI CHG[%s] CH1: Include CH1 in channel group."]
    #[inline]
    pub(crate) fn read_ppi_chgn_ch1(&self, _dim: usize) -> MemResult<bool> {
        Ok(self.ppi.is_included(_dim, 1))
    }
    #[doc = "PPI CHG[%s] CH1: Include CH1 in channel group."]
    #[inline]
    pub(crate) fn write_ppi_chgn_ch1(
        &mut self,
        _dim: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_included(_dim, 1, _value))
    }
    #[doc = "PPI CHG[%s] CH2: Include CH2 in channel group."]
    #[inline]
    pub(crate) fn read_ppi_chgn_ch2(&self, _dim: usize) -> MemResult<bool> {
        Ok(self.ppi.is_included(_dim, 2))
    }
    #[doc = "PPI CHG[%s] CH2: Include CH2 in channel group."]
    #[inline]
    pub(crate) fn write_ppi_chgn_ch2(
        &mut self,
        _dim: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_included(_dim, 2, _value))
    }
    #[doc = "PPI CHG[%s] CH3: Include CH3 in channel group."]
    #[inline]
    pub(crate) fn read_ppi_chgn_ch3(&self, _dim: usize) -> MemResult<bool> {
        Ok(self.ppi.is_included(_dim, 3))
    }
    #[doc = "PPI CHG[%s] CH3: Include CH3 in channel group."]
    #[inline]
    pub(crate) fn write_ppi_chgn_ch3(
        &mut self,
        _dim: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_included(_dim, 3, _value))
    }
    #[doc = "PPI CHG[%s] CH4: Include CH4 in channel group."]
    #[inline]
    pub(crate) fn read_ppi_chgn_ch4(&self, _dim: usize) -> MemResult<bool> {
        Ok(self.ppi.is_included(_dim, 4))
    }
    #[doc = "PPI CHG[%s] CH4: Include CH4 in channel group."]
    #[inline]
    pub(crate) fn write_ppi_chgn_ch4(
        &mut self,
        _dim: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_included(_dim, 4, _value))
    }
    #[doc = "PPI CHG[%s] CH5: Include CH5 in channel group."]
    #[inline]
    pub(crate) fn read_ppi_chgn_ch5(&self, _dim: usize) -> MemResult<bool> {
        Ok(self.ppi.is_included(_dim, 5))
    }
    #[doc = "PPI CHG[%s] CH5: Include CH5 in channel group."]
    #[inline]
    pub(crate) fn write_ppi_chgn_ch5(
        &mut self,
        _dim: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_included(_dim, 5, _value))
    }
    #[doc = "PPI CHG[%s] CH6: Include CH6 in channel group."]
    #[inline]
    pub(crate) fn read_ppi_chgn_ch6(&self, _dim: usize) -> MemResult<bool> {
        Ok(self.ppi.is_included(_dim, 6))
    }
    #[doc = "PPI CHG[%s] CH6: Include CH6 in channel group."]
    #[inline]
    pub(crate) fn write_ppi_chgn_ch6(
        &mut self,
        _dim: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_included(_dim, 6, _value))
    }
    #[doc = "PPI CHG[%s] CH7: Include CH7 in channel group."]
    #[inline]
    pub(crate) fn read_ppi_chgn_ch7(&self, _dim: usize) -> MemResult<bool> {
        Ok(self.ppi.is_included(_dim, 7))
    }
    #[doc = "PPI CHG[%s] CH7: Include CH7 in channel group."]
    #[inline]
    pub(crate) fn write_ppi_chgn_ch7(
        &mut self,
        _dim: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_included(_dim, 7, _value))
    }
    #[doc = "PPI CHG[%s] CH8: Include CH8 in channel group."]
    #[inline]
    pub(crate) fn read_ppi_chgn_ch8(&self, _dim: usize) -> MemResult<bool> {
        Ok(self.ppi.is_included(_dim, 8))
    }
    #[doc = "PPI CHG[%s] CH8: Include CH8 in channel group."]
    #[inline]
    pub(crate) fn write_ppi_chgn_ch8(
        &mut self,
        _dim: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_included(_dim, 8, _value))
    }
    #[doc = "PPI CHG[%s] CH9: Include CH9 in channel group."]
    #[inline]
    pub(crate) fn read_ppi_chgn_ch9(&self, _dim: usize) -> MemResult<bool> {
        Ok(self.ppi.is_included(_dim, 9))
    }
    #[doc = "PPI CHG[%s] CH9: Include CH9 in channel group."]
    #[inline]
    pub(crate) fn write_ppi_chgn_ch9(
        &mut self,
        _dim: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_included(_dim, 9, _value))
    }
    #[doc = "PPI CHG[%s] CH10: Include CH10 in channel group."]
    #[inline]
    pub(crate) fn read_ppi_chgn_ch10(&self, _dim: usize) -> MemResult<bool> {
        Ok(self.ppi.is_included(_dim, 10))
    }
    #[doc = "PPI CHG[%s] CH10: Include CH10 in channel group."]
    #[inline]
    pub(crate) fn write_ppi_chgn_ch10(
        &mut self,
        _dim: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_included(_dim, 10, _value))
    }
    #[doc = "PPI CHG[%s] CH11: Include CH11 in channel group."]
    #[inline]
    pub(crate) fn read_ppi_chgn_ch11(&self, _dim: usize) -> MemResult<bool> {
        Ok(self.ppi.is_included(_dim, 11))
    }
    #[doc = "PPI CHG[%s] CH11: Include CH11 in channel group."]
    #[inline]
    pub(crate) fn write_ppi_chgn_ch11(
        &mut self,
        _dim: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_included(_dim, 11, _value))
    }
    #[doc = "PPI CHG[%s] CH12: Include CH12 in channel group."]
    #[inline]
    pub(crate) fn read_ppi_chgn_ch12(&self, _dim: usize) -> MemResult<bool> {
        Ok(self.ppi.is_included(_dim, 12))
    }
    #[doc = "PPI CHG[%s] CH12: Include CH12 in channel group."]
    #[inline]
    pub(crate) fn write_ppi_chgn_ch12(
        &mut self,
        _dim: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_included(_dim, 12, _value))
    }
    #[doc = "PPI CHG[%s] CH13: Include CH13 in channel group."]
    #[inline]
    pub(crate) fn read_ppi_chgn_ch13(&self, _dim: usize) -> MemResult<bool> {
        Ok(self.ppi.is_included(_dim, 13))
    }
    #[doc = "PPI CHG[%s] CH13: Include CH13 in channel group."]
    #[inline]
    pub(crate) fn write_ppi_chgn_ch13(
        &mut self,
        _dim: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_included(_dim, 13, _value))
    }
    #[doc = "PPI CHG[%s] CH14: Include CH14 in channel group."]
    #[inline]
    pub(crate) fn read_ppi_chgn_ch14(&self, _dim: usize) -> MemResult<bool> {
        Ok(self.ppi.is_included(_dim, 14))
    }
    #[doc = "PPI CHG[%s] CH14: Include CH14 in channel group."]
    #[inline]
    pub(crate) fn write_ppi_chgn_ch14(
        &mut self,
        _dim: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_included(_dim, 14, _value))
    }
    #[doc = "PPI CHG[%s] CH15: Include CH15 in channel group."]
    #[inline]
    pub(crate) fn read_ppi_chgn_ch15(&self, _dim: usize) -> MemResult<bool> {
        Ok(self.ppi.is_included(_dim, 15))
    }
    #[doc = "PPI CHG[%s] CH15: Include CH15 in channel group."]
    #[inline]
    pub(crate) fn write_ppi_chgn_ch15(
        &mut self,
        _dim: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_included(_dim, 15, _value))
    }
    #[doc = "PPI CHG[%s] CH20: Include CH20 in channel group."]
    #[inline]
    pub(crate) fn read_ppi_chgn_ch20(&self, _dim: usize) -> MemResult<bool> {
        Ok(self.ppi.is_included(_dim, 20))
    }
    #[doc = "PPI CHG[%s] CH20: Include CH20 in channel group."]
    #[inline]
    pub(crate) fn write_ppi_chgn_ch20(
        &mut self,
        _dim: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_included(_dim, 20, _value))
    }
    #[doc = "PPI CHG[%s] CH21: Include CH21 in channel group."]
    #[inline]
    pub(crate) fn read_ppi_chgn_ch21(&self, _dim: usize) -> MemResult<bool> {
        Ok(self.ppi.is_included(_dim, 21))
    }
    #[doc = "PPI CHG[%s] CH21: Include CH21 in channel group."]
    #[inline]
    pub(crate) fn write_ppi_chgn_ch21(
        &mut self,
        _dim: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_included(_dim, 21, _value))
    }
    #[doc = "PPI CHG[%s] CH22: Include CH22 in channel group."]
    #[inline]
    pub(crate) fn read_ppi_chgn_ch22(&self, _dim: usize) -> MemResult<bool> {
        Ok(self.ppi.is_included(_dim, 22))
    }
    #[doc = "PPI CHG[%s] CH22: Include CH22 in channel group."]
    #[inline]
    pub(crate) fn write_ppi_chgn_ch22(
        &mut self,
        _dim: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_included(_dim, 22, _value))
    }
    #[doc = "PPI CHG[%s] CH23: Include CH23 in channel group."]
    #[inline]
    pub(crate) fn read_ppi_chgn_ch23(&self, _dim: usize) -> MemResult<bool> {
        Ok(self.ppi.is_included(_dim, 23))
    }
    #[doc = "PPI CHG[%s] CH23: Include CH23 in channel group."]
    #[inline]
    pub(crate) fn write_ppi_chgn_ch23(
        &mut self,
        _dim: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_included(_dim, 23, _value))
    }
    #[doc = "PPI CHG[%s] CH24: Include CH24 in channel group."]
    #[inline]
    pub(crate) fn read_ppi_chgn_ch24(&self, _dim: usize) -> MemResult<bool> {
        Ok(self.ppi.is_included(_dim, 24))
    }
    #[doc = "PPI CHG[%s] CH24: Include CH24 in channel group."]
    #[inline]
    pub(crate) fn write_ppi_chgn_ch24(
        &mut self,
        _dim: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_included(_dim, 24, _value))
    }
    #[doc = "PPI CHG[%s] CH25: Include CH25 in channel group."]
    #[inline]
    pub(crate) fn read_ppi_chgn_ch25(&self, _dim: usize) -> MemResult<bool> {
        Ok(self.ppi.is_included(_dim, 25))
    }
    #[doc = "PPI CHG[%s] CH25: Include CH25 in channel group."]
    #[inline]
    pub(crate) fn write_ppi_chgn_ch25(
        &mut self,
        _dim: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_included(_dim, 25, _value))
    }
    #[doc = "PPI CHG[%s] CH26: Include CH26 in channel group."]
    #[inline]
    pub(crate) fn read_ppi_chgn_ch26(&self, _dim: usize) -> MemResult<bool> {
        Ok(self.ppi.is_included(_dim, 26))
    }
    #[doc = "PPI CHG[%s] CH26: Include CH26 in channel group."]
    #[inline]
    pub(crate) fn write_ppi_chgn_ch26(
        &mut self,
        _dim: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_included(_dim, 26, _value))
    }
    #[doc = "PPI CHG[%s] CH27: Include CH27 in channel group."]
    #[inline]
    pub(crate) fn read_ppi_chgn_ch27(&self, _dim: usize) -> MemResult<bool> {
        Ok(self.ppi.is_included(_dim, 27))
    }
    #[doc = "PPI CHG[%s] CH27: Include CH27 in channel group."]
    #[inline]
    pub(crate) fn write_ppi_chgn_ch27(
        &mut self,
        _dim: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_included(_dim, 27, _value))
    }
    #[doc = "PPI CHG[%s] CH28: Include CH28 in channel group."]
    #[inline]
    pub(crate) fn read_ppi_chgn_ch28(&self, _dim: usize) -> MemResult<bool> {
        Ok(self.ppi.is_included(_dim, 28))
    }
    #[doc = "PPI CHG[%s] CH28: Include CH28 in channel group."]
    #[inline]
    pub(crate) fn write_ppi_chgn_ch28(
        &mut self,
        _dim: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_included(_dim, 28, _value))
    }
    #[doc = "PPI CHG[%s] CH29: Include CH29 in channel group."]
    #[inline]
    pub(crate) fn read_ppi_chgn_ch29(&self, _dim: usize) -> MemResult<bool> {
        Ok(self.ppi.is_included(_dim, 29))
    }
    #[doc = "PPI CHG[%s] CH29: Include CH29 in channel group."]
    #[inline]
    pub(crate) fn write_ppi_chgn_ch29(
        &mut self,
        _dim: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_included(_dim, 29, _value))
    }
    #[doc = "PPI CHG[%s] CH30: Include CH30 in channel group."]
    #[inline]
    pub(crate) fn read_ppi_chgn_ch30(&self, _dim: usize) -> MemResult<bool> {
        Ok(self.ppi.is_included(_dim, 30))
    }
    #[doc = "PPI CHG[%s] CH30: Include CH30 in channel group."]
    #[inline]
    pub(crate) fn write_ppi_chgn_ch30(
        &mut self,
        _dim: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_included(_dim, 30, _value))
    }
    #[doc = "PPI CHG[%s] CH31: Include CH31 in channel group."]
    #[inline]
    pub(crate) fn read_ppi_chgn_ch31(&self, _dim: usize) -> MemResult<bool> {
        Ok(self.ppi.is_included(_dim, 31))
    }
    #[doc = "PPI CHG[%s] CH31: Include CH31 in channel group."]
    #[inline]
    pub(crate) fn write_ppi_chgn_ch31(
        &mut self,
        _dim: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ppi.set_included(_dim, 31, _value))
    }
    #[doc = "FICR CODEPAGESIZE: Code memory page size in bytes."]
    #[inline]
    pub(crate) fn read_ficr_codepagesize(&self) -> MemResult<u32> {
        todo!("read FICR CODEPAGESIZE reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "FICR CODESIZE: Code memory size in pages."]
    #[inline]
    pub(crate) fn read_ficr_codesize(&self) -> MemResult<u32> {
        todo!("read FICR CODESIZE reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "FICR CLENR0: Length of code region 0 in bytes."]
    #[inline]
    pub(crate) fn read_ficr_clenr0(&self) -> MemResult<u32> {
        todo!("read FICR CLENR0 reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "FICR PPFC PPFC: Pre-programmed factory code present."]
    #[inline]
    pub(crate) fn read_ficr_ppfc_ppfc(&self) -> MemResult<u8> {
        todo!("read FICR PPFC PPFC reset value 0x00 mask 0xff")
    }
    #[doc = "FICR NUMRAMBLOCK: Number of individualy controllable RAM blocks."]
    #[inline]
    pub(crate) fn read_ficr_numramblock(&self) -> MemResult<u32> {
        todo!("read FICR NUMRAMBLOCK reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "FICR SIZERAMBLOCKS: Size of RAM blocks in bytes.\n\nFICR SIZERAMBLOCK[%s]: Deprecated array of size of RAM block in bytes. This name is kept for backward compatinility purposes. Use SIZERAMBLOCKS instead."]
    #[inline]
    pub(crate) fn read_ficr_sizeramblocks(
        &self,
        _dim: usize,
    ) -> MemResult<u32> {
        todo ! ("read FICR SIZERAMBLOCKS, FICR SIZERAMBLOCK[%s] reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "FICR CONFIGID HWID: Hardware Identification Number."]
    #[inline]
    pub(crate) fn read_ficr_configid_hwid(&self) -> MemResult<u16> {
        todo!("read FICR CONFIGID HWID reset value 0x00 mask 0xffff")
    }
    #[doc = "FICR CONFIGID FWID: Firmware Identification Number pre-loaded into the flash."]
    #[inline]
    pub(crate) fn read_ficr_configid_fwid(&self) -> MemResult<u16> {
        todo!("read FICR CONFIGID FWID reset value 0x00 mask 0xffff")
    }
    #[doc = "FICR DEVICEID[%s]: Device identifier."]
    #[inline]
    pub(crate) fn read_ficr_deviceidn(&self, _dim: usize) -> MemResult<u32> {
        todo!("read FICR DEVICEID[%s] reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "FICR ER[%s]: Encryption root."]
    #[inline]
    pub(crate) fn read_ficr_ern(&self, _dim: usize) -> MemResult<u32> {
        todo!("read FICR ER[%s] reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "FICR IR[%s]: Identity root."]
    #[inline]
    pub(crate) fn read_ficr_irn(&self, _dim: usize) -> MemResult<u32> {
        todo!("read FICR IR[%s] reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "FICR DEVICEADDRTYPE DEVICEADDRTYPE: Device address type."]
    #[inline]
    pub(crate) fn read_ficr_deviceaddrtype_deviceaddrtype(
        &self,
    ) -> MemResult<bool> {
        todo!("read FICR DEVICEADDRTYPE DEVICEADDRTYPE reset value false")
    }
    #[doc = "FICR DEVICEADDR[%s]: Device address."]
    #[inline]
    pub(crate) fn read_ficr_deviceaddrn(&self, _dim: usize) -> MemResult<u32> {
        todo!(
            "read FICR DEVICEADDR[%s] reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "FICR OVERRIDEEN NRF_1MBIT: Override default values for NRF_1Mbit mode."]
    #[inline]
    pub(crate) fn read_ficr_overrideen_nrf_1mbit(&self) -> MemResult<bool> {
        todo!("read FICR OVERRIDEEN NRF_1MBIT reset value false")
    }
    #[doc = "FICR OVERRIDEEN BLE_1MBIT: Override default values for BLE_1Mbit mode."]
    #[inline]
    pub(crate) fn read_ficr_overrideen_ble_1mbit(&self) -> MemResult<bool> {
        todo!("read FICR OVERRIDEEN BLE_1MBIT reset value false")
    }
    #[doc = "FICR NRF_1MBIT[%s]: Override values for the OVERRIDEn registers in RADIO for NRF_1Mbit mode."]
    #[inline]
    pub(crate) fn read_ficr_nrf_1mbitn(&self, _dim: usize) -> MemResult<u32> {
        todo!(
            "read FICR NRF_1MBIT[%s] reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "FICR BLE_1MBIT[%s]: Override values for the OVERRIDEn registers in RADIO for BLE_1Mbit mode."]
    #[inline]
    pub(crate) fn read_ficr_ble_1mbitn(&self, _dim: usize) -> MemResult<u32> {
        todo!(
            "read FICR BLE_1MBIT[%s] reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "UICR CLENR0: Length of code region 0."]
    #[inline]
    pub(crate) fn read_uicr_clenr0(&self) -> MemResult<u32> {
        todo!("read UICR CLENR0 reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "UICR CLENR0: Length of code region 0."]
    #[inline]
    pub(crate) fn write_uicr_clenr0(&mut self, _value: u32) -> MemResult<()> {
        todo!("write UICR CLENR0 reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "UICR RBPCONF PR0: Readback protect region 0. Will be ignored if pre-programmed factory code is present on the chip."]
    #[inline]
    pub(crate) fn read_uicr_rbpconf_pr0(&self) -> MemResult<u8> {
        todo!("read UICR RBPCONF PR0 reset value 0x00 mask 0xff")
    }
    #[doc = "UICR RBPCONF PR0: Readback protect region 0. Will be ignored if pre-programmed factory code is present on the chip."]
    #[inline]
    pub(crate) fn write_uicr_rbpconf_pr0(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write UICR RBPCONF PR0 reset value 0x00 mask 0xff")
    }
    #[doc = "UICR RBPCONF PALL: Readback protect all code in the device."]
    #[inline]
    pub(crate) fn read_uicr_rbpconf_pall(&self) -> MemResult<u8> {
        todo!("read UICR RBPCONF PALL reset value 0x00 mask 0xff")
    }
    #[doc = "UICR RBPCONF PALL: Readback protect all code in the device."]
    #[inline]
    pub(crate) fn write_uicr_rbpconf_pall(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write UICR RBPCONF PALL reset value 0x00 mask 0xff")
    }
    #[doc = "UICR XTALFREQ XTALFREQ: Reset value for CLOCK XTALFREQ register."]
    #[inline]
    pub(crate) fn read_uicr_xtalfreq_xtalfreq(&self) -> MemResult<u8> {
        todo!("read UICR XTALFREQ XTALFREQ reset value 0x00 mask 0xff")
    }
    #[doc = "UICR XTALFREQ XTALFREQ: Reset value for CLOCK XTALFREQ register."]
    #[inline]
    pub(crate) fn write_uicr_xtalfreq_xtalfreq(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo!("write UICR XTALFREQ XTALFREQ reset value 0x00 mask 0xff")
    }
    #[doc = "UICR FWID FWID: Identification number for the firmware loaded into the chip."]
    #[inline]
    pub(crate) fn read_uicr_fwid_fwid(&self) -> MemResult<u16> {
        todo!("read UICR FWID FWID reset value 0x00 mask 0xffff")
    }
    #[doc = "UICR BOOTLOADERADDR: Bootloader start address.\n\nUICR NRFFW[%s]: Reserved for Nordic firmware design."]
    #[inline]
    pub(crate) fn read_uicr_bootloaderaddr(
        &self,
        _dim: usize,
    ) -> MemResult<u32> {
        todo ! ("read UICR BOOTLOADERADDR, UICR NRFFW[%s] reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "UICR BOOTLOADERADDR: Bootloader start address.\n\nUICR NRFFW[%s]: Reserved for Nordic firmware design."]
    #[inline]
    pub(crate) fn write_uicr_bootloaderaddr(
        &mut self,
        _dim: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write UICR BOOTLOADERADDR, UICR NRFFW[%s] reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "UICR NRFHW[%s]: Reserved for Nordic hardware design."]
    #[inline]
    pub(crate) fn read_uicr_nrfhwn(&self, _dim: usize) -> MemResult<u32> {
        todo!("read UICR NRFHW[%s] reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "UICR NRFHW[%s]: Reserved for Nordic hardware design."]
    #[inline]
    pub(crate) fn write_uicr_nrfhwn(
        &mut self,
        _dim: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo!("write UICR NRFHW[%s] reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "UICR CUSTOMER[%s]: Reserved for customer."]
    #[inline]
    pub(crate) fn read_uicr_customern(&self, _dim: usize) -> MemResult<u32> {
        todo!("read UICR CUSTOMER[%s] reset value 0x00 mask 0xffffffffffffffff")
    }
    #[doc = "UICR CUSTOMER[%s]: Reserved for customer."]
    #[inline]
    pub(crate) fn write_uicr_customern(
        &mut self,
        _dim: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo!(
            "write UICR CUSTOMER[%s] reset value 0x00 mask 0xffffffffffffffff"
        )
    }
    #[doc = "GPIO OUT PIN0: Pin 0."]
    #[inline]
    pub(crate) fn read_gpio_out_pin0(&self) -> MemResult<bool> {
        Ok(self.gpio[0].is_out_high())
    }
    #[doc = "GPIO OUT PIN0: Pin 0."]
    #[inline]
    pub(crate) fn write_gpio_out_pin0(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[0].set_out_high(_value))
    }
    #[doc = "GPIO OUT PIN1: Pin 1."]
    #[inline]
    pub(crate) fn read_gpio_out_pin1(&self) -> MemResult<bool> {
        Ok(self.gpio[1].is_out_high())
    }
    #[doc = "GPIO OUT PIN1: Pin 1."]
    #[inline]
    pub(crate) fn write_gpio_out_pin1(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[1].set_out_high(_value))
    }
    #[doc = "GPIO OUT PIN2: Pin 2."]
    #[inline]
    pub(crate) fn read_gpio_out_pin2(&self) -> MemResult<bool> {
        Ok(self.gpio[2].is_out_high())
    }
    #[doc = "GPIO OUT PIN2: Pin 2."]
    #[inline]
    pub(crate) fn write_gpio_out_pin2(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[2].set_out_high(_value))
    }
    #[doc = "GPIO OUT PIN3: Pin 3."]
    #[inline]
    pub(crate) fn read_gpio_out_pin3(&self) -> MemResult<bool> {
        Ok(self.gpio[3].is_out_high())
    }
    #[doc = "GPIO OUT PIN3: Pin 3."]
    #[inline]
    pub(crate) fn write_gpio_out_pin3(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[3].set_out_high(_value))
    }
    #[doc = "GPIO OUT PIN4: Pin 4."]
    #[inline]
    pub(crate) fn read_gpio_out_pin4(&self) -> MemResult<bool> {
        Ok(self.gpio[4].is_out_high())
    }
    #[doc = "GPIO OUT PIN4: Pin 4."]
    #[inline]
    pub(crate) fn write_gpio_out_pin4(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[4].set_out_high(_value))
    }
    #[doc = "GPIO OUT PIN5: Pin 5."]
    #[inline]
    pub(crate) fn read_gpio_out_pin5(&self) -> MemResult<bool> {
        Ok(self.gpio[5].is_out_high())
    }
    #[doc = "GPIO OUT PIN5: Pin 5."]
    #[inline]
    pub(crate) fn write_gpio_out_pin5(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[5].set_out_high(_value))
    }
    #[doc = "GPIO OUT PIN6: Pin 6."]
    #[inline]
    pub(crate) fn read_gpio_out_pin6(&self) -> MemResult<bool> {
        Ok(self.gpio[6].is_out_high())
    }
    #[doc = "GPIO OUT PIN6: Pin 6."]
    #[inline]
    pub(crate) fn write_gpio_out_pin6(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[6].set_out_high(_value))
    }
    #[doc = "GPIO OUT PIN7: Pin 7."]
    #[inline]
    pub(crate) fn read_gpio_out_pin7(&self) -> MemResult<bool> {
        Ok(self.gpio[7].is_out_high())
    }
    #[doc = "GPIO OUT PIN7: Pin 7."]
    #[inline]
    pub(crate) fn write_gpio_out_pin7(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[7].set_out_high(_value))
    }
    #[doc = "GPIO OUT PIN8: Pin 8."]
    #[inline]
    pub(crate) fn read_gpio_out_pin8(&self) -> MemResult<bool> {
        Ok(self.gpio[8].is_out_high())
    }
    #[doc = "GPIO OUT PIN8: Pin 8."]
    #[inline]
    pub(crate) fn write_gpio_out_pin8(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[8].set_out_high(_value))
    }
    #[doc = "GPIO OUT PIN9: Pin 9."]
    #[inline]
    pub(crate) fn read_gpio_out_pin9(&self) -> MemResult<bool> {
        Ok(self.gpio[9].is_out_high())
    }
    #[doc = "GPIO OUT PIN9: Pin 9."]
    #[inline]
    pub(crate) fn write_gpio_out_pin9(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[9].set_out_high(_value))
    }
    #[doc = "GPIO OUT PIN10: Pin 10."]
    #[inline]
    pub(crate) fn read_gpio_out_pin10(&self) -> MemResult<bool> {
        Ok(self.gpio[10].is_out_high())
    }
    #[doc = "GPIO OUT PIN10: Pin 10."]
    #[inline]
    pub(crate) fn write_gpio_out_pin10(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[10].set_out_high(_value))
    }
    #[doc = "GPIO OUT PIN11: Pin 11."]
    #[inline]
    pub(crate) fn read_gpio_out_pin11(&self) -> MemResult<bool> {
        Ok(self.gpio[11].is_out_high())
    }
    #[doc = "GPIO OUT PIN11: Pin 11."]
    #[inline]
    pub(crate) fn write_gpio_out_pin11(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[11].set_out_high(_value))
    }
    #[doc = "GPIO OUT PIN12: Pin 12."]
    #[inline]
    pub(crate) fn read_gpio_out_pin12(&self) -> MemResult<bool> {
        Ok(self.gpio[12].is_out_high())
    }
    #[doc = "GPIO OUT PIN12: Pin 12."]
    #[inline]
    pub(crate) fn write_gpio_out_pin12(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[12].set_out_high(_value))
    }
    #[doc = "GPIO OUT PIN13: Pin 13."]
    #[inline]
    pub(crate) fn read_gpio_out_pin13(&self) -> MemResult<bool> {
        Ok(self.gpio[13].is_out_high())
    }
    #[doc = "GPIO OUT PIN13: Pin 13."]
    #[inline]
    pub(crate) fn write_gpio_out_pin13(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[13].set_out_high(_value))
    }
    #[doc = "GPIO OUT PIN14: Pin 14."]
    #[inline]
    pub(crate) fn read_gpio_out_pin14(&self) -> MemResult<bool> {
        Ok(self.gpio[14].is_out_high())
    }
    #[doc = "GPIO OUT PIN14: Pin 14."]
    #[inline]
    pub(crate) fn write_gpio_out_pin14(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[14].set_out_high(_value))
    }
    #[doc = "GPIO OUT PIN15: Pin 15."]
    #[inline]
    pub(crate) fn read_gpio_out_pin15(&self) -> MemResult<bool> {
        Ok(self.gpio[15].is_out_high())
    }
    #[doc = "GPIO OUT PIN15: Pin 15."]
    #[inline]
    pub(crate) fn write_gpio_out_pin15(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[15].set_out_high(_value))
    }
    #[doc = "GPIO OUT PIN16: Pin 16."]
    #[inline]
    pub(crate) fn read_gpio_out_pin16(&self) -> MemResult<bool> {
        Ok(self.gpio[16].is_out_high())
    }
    #[doc = "GPIO OUT PIN16: Pin 16."]
    #[inline]
    pub(crate) fn write_gpio_out_pin16(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[16].set_out_high(_value))
    }
    #[doc = "GPIO OUT PIN17: Pin 17."]
    #[inline]
    pub(crate) fn read_gpio_out_pin17(&self) -> MemResult<bool> {
        Ok(self.gpio[17].is_out_high())
    }
    #[doc = "GPIO OUT PIN17: Pin 17."]
    #[inline]
    pub(crate) fn write_gpio_out_pin17(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[17].set_out_high(_value))
    }
    #[doc = "GPIO OUT PIN18: Pin 18."]
    #[inline]
    pub(crate) fn read_gpio_out_pin18(&self) -> MemResult<bool> {
        Ok(self.gpio[18].is_out_high())
    }
    #[doc = "GPIO OUT PIN18: Pin 18."]
    #[inline]
    pub(crate) fn write_gpio_out_pin18(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[18].set_out_high(_value))
    }
    #[doc = "GPIO OUT PIN19: Pin 19."]
    #[inline]
    pub(crate) fn read_gpio_out_pin19(&self) -> MemResult<bool> {
        Ok(self.gpio[19].is_out_high())
    }
    #[doc = "GPIO OUT PIN19: Pin 19."]
    #[inline]
    pub(crate) fn write_gpio_out_pin19(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[19].set_out_high(_value))
    }
    #[doc = "GPIO OUT PIN20: Pin 20."]
    #[inline]
    pub(crate) fn read_gpio_out_pin20(&self) -> MemResult<bool> {
        Ok(self.gpio[20].is_out_high())
    }
    #[doc = "GPIO OUT PIN20: Pin 20."]
    #[inline]
    pub(crate) fn write_gpio_out_pin20(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[20].set_out_high(_value))
    }
    #[doc = "GPIO OUT PIN21: Pin 21."]
    #[inline]
    pub(crate) fn read_gpio_out_pin21(&self) -> MemResult<bool> {
        Ok(self.gpio[21].is_out_high())
    }
    #[doc = "GPIO OUT PIN21: Pin 21."]
    #[inline]
    pub(crate) fn write_gpio_out_pin21(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[21].set_out_high(_value))
    }
    #[doc = "GPIO OUT PIN22: Pin 22."]
    #[inline]
    pub(crate) fn read_gpio_out_pin22(&self) -> MemResult<bool> {
        Ok(self.gpio[22].is_out_high())
    }
    #[doc = "GPIO OUT PIN22: Pin 22."]
    #[inline]
    pub(crate) fn write_gpio_out_pin22(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[22].set_out_high(_value))
    }
    #[doc = "GPIO OUT PIN23: Pin 23."]
    #[inline]
    pub(crate) fn read_gpio_out_pin23(&self) -> MemResult<bool> {
        Ok(self.gpio[23].is_out_high())
    }
    #[doc = "GPIO OUT PIN23: Pin 23."]
    #[inline]
    pub(crate) fn write_gpio_out_pin23(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[23].set_out_high(_value))
    }
    #[doc = "GPIO OUT PIN24: Pin 24."]
    #[inline]
    pub(crate) fn read_gpio_out_pin24(&self) -> MemResult<bool> {
        Ok(self.gpio[24].is_out_high())
    }
    #[doc = "GPIO OUT PIN24: Pin 24."]
    #[inline]
    pub(crate) fn write_gpio_out_pin24(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[24].set_out_high(_value))
    }
    #[doc = "GPIO OUT PIN25: Pin 25."]
    #[inline]
    pub(crate) fn read_gpio_out_pin25(&self) -> MemResult<bool> {
        Ok(self.gpio[25].is_out_high())
    }
    #[doc = "GPIO OUT PIN25: Pin 25."]
    #[inline]
    pub(crate) fn write_gpio_out_pin25(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[25].set_out_high(_value))
    }
    #[doc = "GPIO OUT PIN26: Pin 26."]
    #[inline]
    pub(crate) fn read_gpio_out_pin26(&self) -> MemResult<bool> {
        Ok(self.gpio[26].is_out_high())
    }
    #[doc = "GPIO OUT PIN26: Pin 26."]
    #[inline]
    pub(crate) fn write_gpio_out_pin26(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[26].set_out_high(_value))
    }
    #[doc = "GPIO OUT PIN27: Pin 27."]
    #[inline]
    pub(crate) fn read_gpio_out_pin27(&self) -> MemResult<bool> {
        Ok(self.gpio[27].is_out_high())
    }
    #[doc = "GPIO OUT PIN27: Pin 27."]
    #[inline]
    pub(crate) fn write_gpio_out_pin27(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[27].set_out_high(_value))
    }
    #[doc = "GPIO OUT PIN28: Pin 28."]
    #[inline]
    pub(crate) fn read_gpio_out_pin28(&self) -> MemResult<bool> {
        Ok(self.gpio[28].is_out_high())
    }
    #[doc = "GPIO OUT PIN28: Pin 28."]
    #[inline]
    pub(crate) fn write_gpio_out_pin28(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[28].set_out_high(_value))
    }
    #[doc = "GPIO OUT PIN29: Pin 29."]
    #[inline]
    pub(crate) fn read_gpio_out_pin29(&self) -> MemResult<bool> {
        Ok(self.gpio[29].is_out_high())
    }
    #[doc = "GPIO OUT PIN29: Pin 29."]
    #[inline]
    pub(crate) fn write_gpio_out_pin29(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[29].set_out_high(_value))
    }
    #[doc = "GPIO OUT PIN30: Pin 30."]
    #[inline]
    pub(crate) fn read_gpio_out_pin30(&self) -> MemResult<bool> {
        Ok(self.gpio[30].is_out_high())
    }
    #[doc = "GPIO OUT PIN30: Pin 30."]
    #[inline]
    pub(crate) fn write_gpio_out_pin30(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[30].set_out_high(_value))
    }
    #[doc = "GPIO OUT PIN31: Pin 31."]
    #[inline]
    pub(crate) fn read_gpio_out_pin31(&self) -> MemResult<bool> {
        Ok(self.gpio[31].is_out_high())
    }
    #[doc = "GPIO OUT PIN31: Pin 31."]
    #[inline]
    pub(crate) fn write_gpio_out_pin31(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[31].set_out_high(_value))
    }
    #[doc = "GPIO OUTSET PIN0: Pin 0."]
    #[inline]
    pub(crate) fn read_gpio_outset_pin0(&self) -> MemResult<bool> {
        Ok(self.gpio[0].is_out_high())
    }
    #[doc = "GPIO OUTSET PIN0: Pin 0."]
    #[inline]
    pub(crate) fn write_gpio_outset_pin0(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[0].set_out_high(true);
        }
        Ok(())
    }
    #[doc = "GPIO OUTSET PIN1: Pin 1."]
    #[inline]
    pub(crate) fn read_gpio_outset_pin1(&self) -> MemResult<bool> {
        Ok(self.gpio[1].is_out_high())
    }
    #[doc = "GPIO OUTSET PIN1: Pin 1."]
    #[inline]
    pub(crate) fn write_gpio_outset_pin1(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[1].set_out_high(true);
        }
        Ok(())
    }
    #[doc = "GPIO OUTSET PIN2: Pin 2."]
    #[inline]
    pub(crate) fn read_gpio_outset_pin2(&self) -> MemResult<bool> {
        Ok(self.gpio[2].is_out_high())
    }
    #[doc = "GPIO OUTSET PIN2: Pin 2."]
    #[inline]
    pub(crate) fn write_gpio_outset_pin2(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[2].set_out_high(true);
        }
        Ok(())
    }
    #[doc = "GPIO OUTSET PIN3: Pin 3."]
    #[inline]
    pub(crate) fn read_gpio_outset_pin3(&self) -> MemResult<bool> {
        Ok(self.gpio[3].is_out_high())
    }
    #[doc = "GPIO OUTSET PIN3: Pin 3."]
    #[inline]
    pub(crate) fn write_gpio_outset_pin3(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[3].set_out_high(true);
        }
        Ok(())
    }
    #[doc = "GPIO OUTSET PIN4: Pin 4."]
    #[inline]
    pub(crate) fn read_gpio_outset_pin4(&self) -> MemResult<bool> {
        Ok(self.gpio[4].is_out_high())
    }
    #[doc = "GPIO OUTSET PIN4: Pin 4."]
    #[inline]
    pub(crate) fn write_gpio_outset_pin4(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[4].set_out_high(true);
        }
        Ok(())
    }
    #[doc = "GPIO OUTSET PIN5: Pin 5."]
    #[inline]
    pub(crate) fn read_gpio_outset_pin5(&self) -> MemResult<bool> {
        Ok(self.gpio[5].is_out_high())
    }
    #[doc = "GPIO OUTSET PIN5: Pin 5."]
    #[inline]
    pub(crate) fn write_gpio_outset_pin5(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[5].set_out_high(true);
        }
        Ok(())
    }
    #[doc = "GPIO OUTSET PIN6: Pin 6."]
    #[inline]
    pub(crate) fn read_gpio_outset_pin6(&self) -> MemResult<bool> {
        Ok(self.gpio[6].is_out_high())
    }
    #[doc = "GPIO OUTSET PIN6: Pin 6."]
    #[inline]
    pub(crate) fn write_gpio_outset_pin6(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[6].set_out_high(true);
        }
        Ok(())
    }
    #[doc = "GPIO OUTSET PIN7: Pin 7."]
    #[inline]
    pub(crate) fn read_gpio_outset_pin7(&self) -> MemResult<bool> {
        Ok(self.gpio[7].is_out_high())
    }
    #[doc = "GPIO OUTSET PIN7: Pin 7."]
    #[inline]
    pub(crate) fn write_gpio_outset_pin7(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[7].set_out_high(true);
        }
        Ok(())
    }
    #[doc = "GPIO OUTSET PIN8: Pin 8."]
    #[inline]
    pub(crate) fn read_gpio_outset_pin8(&self) -> MemResult<bool> {
        Ok(self.gpio[8].is_out_high())
    }
    #[doc = "GPIO OUTSET PIN8: Pin 8."]
    #[inline]
    pub(crate) fn write_gpio_outset_pin8(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[8].set_out_high(true);
        }
        Ok(())
    }
    #[doc = "GPIO OUTSET PIN9: Pin 9."]
    #[inline]
    pub(crate) fn read_gpio_outset_pin9(&self) -> MemResult<bool> {
        Ok(self.gpio[9].is_out_high())
    }
    #[doc = "GPIO OUTSET PIN9: Pin 9."]
    #[inline]
    pub(crate) fn write_gpio_outset_pin9(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[9].set_out_high(true);
        }
        Ok(())
    }
    #[doc = "GPIO OUTSET PIN10: Pin 10."]
    #[inline]
    pub(crate) fn read_gpio_outset_pin10(&self) -> MemResult<bool> {
        Ok(self.gpio[10].is_out_high())
    }
    #[doc = "GPIO OUTSET PIN10: Pin 10."]
    #[inline]
    pub(crate) fn write_gpio_outset_pin10(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[10].set_out_high(true);
        }
        Ok(())
    }
    #[doc = "GPIO OUTSET PIN11: Pin 11."]
    #[inline]
    pub(crate) fn read_gpio_outset_pin11(&self) -> MemResult<bool> {
        Ok(self.gpio[11].is_out_high())
    }
    #[doc = "GPIO OUTSET PIN11: Pin 11."]
    #[inline]
    pub(crate) fn write_gpio_outset_pin11(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[11].set_out_high(true);
        }
        Ok(())
    }
    #[doc = "GPIO OUTSET PIN12: Pin 12."]
    #[inline]
    pub(crate) fn read_gpio_outset_pin12(&self) -> MemResult<bool> {
        Ok(self.gpio[12].is_out_high())
    }
    #[doc = "GPIO OUTSET PIN12: Pin 12."]
    #[inline]
    pub(crate) fn write_gpio_outset_pin12(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[12].set_out_high(true);
        }
        Ok(())
    }
    #[doc = "GPIO OUTSET PIN13: Pin 13."]
    #[inline]
    pub(crate) fn read_gpio_outset_pin13(&self) -> MemResult<bool> {
        Ok(self.gpio[13].is_out_high())
    }
    #[doc = "GPIO OUTSET PIN13: Pin 13."]
    #[inline]
    pub(crate) fn write_gpio_outset_pin13(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[13].set_out_high(true);
        }
        Ok(())
    }
    #[doc = "GPIO OUTSET PIN14: Pin 14."]
    #[inline]
    pub(crate) fn read_gpio_outset_pin14(&self) -> MemResult<bool> {
        Ok(self.gpio[14].is_out_high())
    }
    #[doc = "GPIO OUTSET PIN14: Pin 14."]
    #[inline]
    pub(crate) fn write_gpio_outset_pin14(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[14].set_out_high(true);
        }
        Ok(())
    }
    #[doc = "GPIO OUTSET PIN15: Pin 15."]
    #[inline]
    pub(crate) fn read_gpio_outset_pin15(&self) -> MemResult<bool> {
        Ok(self.gpio[15].is_out_high())
    }
    #[doc = "GPIO OUTSET PIN15: Pin 15."]
    #[inline]
    pub(crate) fn write_gpio_outset_pin15(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[15].set_out_high(true);
        }
        Ok(())
    }
    #[doc = "GPIO OUTSET PIN16: Pin 16."]
    #[inline]
    pub(crate) fn read_gpio_outset_pin16(&self) -> MemResult<bool> {
        Ok(self.gpio[16].is_out_high())
    }
    #[doc = "GPIO OUTSET PIN16: Pin 16."]
    #[inline]
    pub(crate) fn write_gpio_outset_pin16(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[16].set_out_high(true);
        }
        Ok(())
    }
    #[doc = "GPIO OUTSET PIN17: Pin 17."]
    #[inline]
    pub(crate) fn read_gpio_outset_pin17(&self) -> MemResult<bool> {
        Ok(self.gpio[17].is_out_high())
    }
    #[doc = "GPIO OUTSET PIN17: Pin 17."]
    #[inline]
    pub(crate) fn write_gpio_outset_pin17(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[17].set_out_high(true);
        }
        Ok(())
    }
    #[doc = "GPIO OUTSET PIN18: Pin 18."]
    #[inline]
    pub(crate) fn read_gpio_outset_pin18(&self) -> MemResult<bool> {
        Ok(self.gpio[18].is_out_high())
    }
    #[doc = "GPIO OUTSET PIN18: Pin 18."]
    #[inline]
    pub(crate) fn write_gpio_outset_pin18(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[18].set_out_high(true);
        }
        Ok(())
    }
    #[doc = "GPIO OUTSET PIN19: Pin 19."]
    #[inline]
    pub(crate) fn read_gpio_outset_pin19(&self) -> MemResult<bool> {
        Ok(self.gpio[19].is_out_high())
    }
    #[doc = "GPIO OUTSET PIN19: Pin 19."]
    #[inline]
    pub(crate) fn write_gpio_outset_pin19(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[19].set_out_high(true);
        }
        Ok(())
    }
    #[doc = "GPIO OUTSET PIN20: Pin 20."]
    #[inline]
    pub(crate) fn read_gpio_outset_pin20(&self) -> MemResult<bool> {
        Ok(self.gpio[20].is_out_high())
    }
    #[doc = "GPIO OUTSET PIN20: Pin 20."]
    #[inline]
    pub(crate) fn write_gpio_outset_pin20(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[20].set_out_high(true);
        }
        Ok(())
    }
    #[doc = "GPIO OUTSET PIN21: Pin 21."]
    #[inline]
    pub(crate) fn read_gpio_outset_pin21(&self) -> MemResult<bool> {
        Ok(self.gpio[21].is_out_high())
    }
    #[doc = "GPIO OUTSET PIN21: Pin 21."]
    #[inline]
    pub(crate) fn write_gpio_outset_pin21(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[21].set_out_high(true);
        }
        Ok(())
    }
    #[doc = "GPIO OUTSET PIN22: Pin 22."]
    #[inline]
    pub(crate) fn read_gpio_outset_pin22(&self) -> MemResult<bool> {
        Ok(self.gpio[22].is_out_high())
    }
    #[doc = "GPIO OUTSET PIN22: Pin 22."]
    #[inline]
    pub(crate) fn write_gpio_outset_pin22(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[22].set_out_high(true);
        }
        Ok(())
    }
    #[doc = "GPIO OUTSET PIN23: Pin 23."]
    #[inline]
    pub(crate) fn read_gpio_outset_pin23(&self) -> MemResult<bool> {
        Ok(self.gpio[23].is_out_high())
    }
    #[doc = "GPIO OUTSET PIN23: Pin 23."]
    #[inline]
    pub(crate) fn write_gpio_outset_pin23(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[23].set_out_high(true);
        }
        Ok(())
    }
    #[doc = "GPIO OUTSET PIN24: Pin 24."]
    #[inline]
    pub(crate) fn read_gpio_outset_pin24(&self) -> MemResult<bool> {
        Ok(self.gpio[24].is_out_high())
    }
    #[doc = "GPIO OUTSET PIN24: Pin 24."]
    #[inline]
    pub(crate) fn write_gpio_outset_pin24(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[24].set_out_high(true);
        }
        Ok(())
    }
    #[doc = "GPIO OUTSET PIN25: Pin 25."]
    #[inline]
    pub(crate) fn read_gpio_outset_pin25(&self) -> MemResult<bool> {
        Ok(self.gpio[25].is_out_high())
    }
    #[doc = "GPIO OUTSET PIN25: Pin 25."]
    #[inline]
    pub(crate) fn write_gpio_outset_pin25(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[25].set_out_high(true);
        }
        Ok(())
    }
    #[doc = "GPIO OUTSET PIN26: Pin 26."]
    #[inline]
    pub(crate) fn read_gpio_outset_pin26(&self) -> MemResult<bool> {
        Ok(self.gpio[26].is_out_high())
    }
    #[doc = "GPIO OUTSET PIN26: Pin 26."]
    #[inline]
    pub(crate) fn write_gpio_outset_pin26(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[26].set_out_high(true);
        }
        Ok(())
    }
    #[doc = "GPIO OUTSET PIN27: Pin 27."]
    #[inline]
    pub(crate) fn read_gpio_outset_pin27(&self) -> MemResult<bool> {
        Ok(self.gpio[27].is_out_high())
    }
    #[doc = "GPIO OUTSET PIN27: Pin 27."]
    #[inline]
    pub(crate) fn write_gpio_outset_pin27(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[27].set_out_high(true);
        }
        Ok(())
    }
    #[doc = "GPIO OUTSET PIN28: Pin 28."]
    #[inline]
    pub(crate) fn read_gpio_outset_pin28(&self) -> MemResult<bool> {
        Ok(self.gpio[28].is_out_high())
    }
    #[doc = "GPIO OUTSET PIN28: Pin 28."]
    #[inline]
    pub(crate) fn write_gpio_outset_pin28(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[28].set_out_high(true);
        }
        Ok(())
    }
    #[doc = "GPIO OUTSET PIN29: Pin 29."]
    #[inline]
    pub(crate) fn read_gpio_outset_pin29(&self) -> MemResult<bool> {
        Ok(self.gpio[29].is_out_high())
    }
    #[doc = "GPIO OUTSET PIN29: Pin 29."]
    #[inline]
    pub(crate) fn write_gpio_outset_pin29(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[29].set_out_high(true);
        }
        Ok(())
    }
    #[doc = "GPIO OUTSET PIN30: Pin 30."]
    #[inline]
    pub(crate) fn read_gpio_outset_pin30(&self) -> MemResult<bool> {
        Ok(self.gpio[30].is_out_high())
    }
    #[doc = "GPIO OUTSET PIN30: Pin 30."]
    #[inline]
    pub(crate) fn write_gpio_outset_pin30(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[30].set_out_high(true);
        }
        Ok(())
    }
    #[doc = "GPIO OUTSET PIN31: Pin 31."]
    #[inline]
    pub(crate) fn read_gpio_outset_pin31(&self) -> MemResult<bool> {
        Ok(self.gpio[31].is_out_high())
    }
    #[doc = "GPIO OUTSET PIN31: Pin 31."]
    #[inline]
    pub(crate) fn write_gpio_outset_pin31(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[31].set_out_high(true);
        }
        Ok(())
    }
    #[doc = "GPIO OUTCLR PIN0: Pin 0."]
    #[inline]
    pub(crate) fn read_gpio_outclr_pin0(&self) -> MemResult<bool> {
        Ok(self.gpio[0].is_out_high())
    }
    #[doc = "GPIO OUTCLR PIN0: Pin 0."]
    #[inline]
    pub(crate) fn write_gpio_outclr_pin0(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[0].set_out_high(false);
        }
        Ok(())
    }
    #[doc = "GPIO OUTCLR PIN1: Pin 1."]
    #[inline]
    pub(crate) fn read_gpio_outclr_pin1(&self) -> MemResult<bool> {
        Ok(self.gpio[1].is_out_high())
    }
    #[doc = "GPIO OUTCLR PIN1: Pin 1."]
    #[inline]
    pub(crate) fn write_gpio_outclr_pin1(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[1].set_out_high(false);
        }
        Ok(())
    }
    #[doc = "GPIO OUTCLR PIN2: Pin 2."]
    #[inline]
    pub(crate) fn read_gpio_outclr_pin2(&self) -> MemResult<bool> {
        Ok(self.gpio[2].is_out_high())
    }
    #[doc = "GPIO OUTCLR PIN2: Pin 2."]
    #[inline]
    pub(crate) fn write_gpio_outclr_pin2(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[2].set_out_high(false);
        }
        Ok(())
    }
    #[doc = "GPIO OUTCLR PIN3: Pin 3."]
    #[inline]
    pub(crate) fn read_gpio_outclr_pin3(&self) -> MemResult<bool> {
        Ok(self.gpio[3].is_out_high())
    }
    #[doc = "GPIO OUTCLR PIN3: Pin 3."]
    #[inline]
    pub(crate) fn write_gpio_outclr_pin3(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[3].set_out_high(false);
        }
        Ok(())
    }
    #[doc = "GPIO OUTCLR PIN4: Pin 4."]
    #[inline]
    pub(crate) fn read_gpio_outclr_pin4(&self) -> MemResult<bool> {
        Ok(self.gpio[4].is_out_high())
    }
    #[doc = "GPIO OUTCLR PIN4: Pin 4."]
    #[inline]
    pub(crate) fn write_gpio_outclr_pin4(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[4].set_out_high(false);
        }
        Ok(())
    }
    #[doc = "GPIO OUTCLR PIN5: Pin 5."]
    #[inline]
    pub(crate) fn read_gpio_outclr_pin5(&self) -> MemResult<bool> {
        Ok(self.gpio[5].is_out_high())
    }
    #[doc = "GPIO OUTCLR PIN5: Pin 5."]
    #[inline]
    pub(crate) fn write_gpio_outclr_pin5(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[5].set_out_high(false);
        }
        Ok(())
    }
    #[doc = "GPIO OUTCLR PIN6: Pin 6."]
    #[inline]
    pub(crate) fn read_gpio_outclr_pin6(&self) -> MemResult<bool> {
        Ok(self.gpio[6].is_out_high())
    }
    #[doc = "GPIO OUTCLR PIN6: Pin 6."]
    #[inline]
    pub(crate) fn write_gpio_outclr_pin6(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[6].set_out_high(false);
        }
        Ok(())
    }
    #[doc = "GPIO OUTCLR PIN7: Pin 7."]
    #[inline]
    pub(crate) fn read_gpio_outclr_pin7(&self) -> MemResult<bool> {
        Ok(self.gpio[7].is_out_high())
    }
    #[doc = "GPIO OUTCLR PIN7: Pin 7."]
    #[inline]
    pub(crate) fn write_gpio_outclr_pin7(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[7].set_out_high(false);
        }
        Ok(())
    }
    #[doc = "GPIO OUTCLR PIN8: Pin 8."]
    #[inline]
    pub(crate) fn read_gpio_outclr_pin8(&self) -> MemResult<bool> {
        Ok(self.gpio[8].is_out_high())
    }
    #[doc = "GPIO OUTCLR PIN8: Pin 8."]
    #[inline]
    pub(crate) fn write_gpio_outclr_pin8(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[8].set_out_high(false);
        }
        Ok(())
    }
    #[doc = "GPIO OUTCLR PIN9: Pin 9."]
    #[inline]
    pub(crate) fn read_gpio_outclr_pin9(&self) -> MemResult<bool> {
        Ok(self.gpio[9].is_out_high())
    }
    #[doc = "GPIO OUTCLR PIN9: Pin 9."]
    #[inline]
    pub(crate) fn write_gpio_outclr_pin9(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[9].set_out_high(false);
        }
        Ok(())
    }
    #[doc = "GPIO OUTCLR PIN10: Pin 10."]
    #[inline]
    pub(crate) fn read_gpio_outclr_pin10(&self) -> MemResult<bool> {
        Ok(self.gpio[10].is_out_high())
    }
    #[doc = "GPIO OUTCLR PIN10: Pin 10."]
    #[inline]
    pub(crate) fn write_gpio_outclr_pin10(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[10].set_out_high(false);
        }
        Ok(())
    }
    #[doc = "GPIO OUTCLR PIN11: Pin 11."]
    #[inline]
    pub(crate) fn read_gpio_outclr_pin11(&self) -> MemResult<bool> {
        Ok(self.gpio[11].is_out_high())
    }
    #[doc = "GPIO OUTCLR PIN11: Pin 11."]
    #[inline]
    pub(crate) fn write_gpio_outclr_pin11(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[11].set_out_high(false);
        }
        Ok(())
    }
    #[doc = "GPIO OUTCLR PIN12: Pin 12."]
    #[inline]
    pub(crate) fn read_gpio_outclr_pin12(&self) -> MemResult<bool> {
        Ok(self.gpio[12].is_out_high())
    }
    #[doc = "GPIO OUTCLR PIN12: Pin 12."]
    #[inline]
    pub(crate) fn write_gpio_outclr_pin12(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[12].set_out_high(false);
        }
        Ok(())
    }
    #[doc = "GPIO OUTCLR PIN13: Pin 13."]
    #[inline]
    pub(crate) fn read_gpio_outclr_pin13(&self) -> MemResult<bool> {
        Ok(self.gpio[13].is_out_high())
    }
    #[doc = "GPIO OUTCLR PIN13: Pin 13."]
    #[inline]
    pub(crate) fn write_gpio_outclr_pin13(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[13].set_out_high(false);
        }
        Ok(())
    }
    #[doc = "GPIO OUTCLR PIN14: Pin 14."]
    #[inline]
    pub(crate) fn read_gpio_outclr_pin14(&self) -> MemResult<bool> {
        Ok(self.gpio[14].is_out_high())
    }
    #[doc = "GPIO OUTCLR PIN14: Pin 14."]
    #[inline]
    pub(crate) fn write_gpio_outclr_pin14(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[14].set_out_high(false);
        }
        Ok(())
    }
    #[doc = "GPIO OUTCLR PIN15: Pin 15."]
    #[inline]
    pub(crate) fn read_gpio_outclr_pin15(&self) -> MemResult<bool> {
        Ok(self.gpio[15].is_out_high())
    }
    #[doc = "GPIO OUTCLR PIN15: Pin 15."]
    #[inline]
    pub(crate) fn write_gpio_outclr_pin15(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[15].set_out_high(false);
        }
        Ok(())
    }
    #[doc = "GPIO OUTCLR PIN16: Pin 16."]
    #[inline]
    pub(crate) fn read_gpio_outclr_pin16(&self) -> MemResult<bool> {
        Ok(self.gpio[16].is_out_high())
    }
    #[doc = "GPIO OUTCLR PIN16: Pin 16."]
    #[inline]
    pub(crate) fn write_gpio_outclr_pin16(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[16].set_out_high(false);
        }
        Ok(())
    }
    #[doc = "GPIO OUTCLR PIN17: Pin 17."]
    #[inline]
    pub(crate) fn read_gpio_outclr_pin17(&self) -> MemResult<bool> {
        Ok(self.gpio[17].is_out_high())
    }
    #[doc = "GPIO OUTCLR PIN17: Pin 17."]
    #[inline]
    pub(crate) fn write_gpio_outclr_pin17(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[17].set_out_high(false);
        }
        Ok(())
    }
    #[doc = "GPIO OUTCLR PIN18: Pin 18."]
    #[inline]
    pub(crate) fn read_gpio_outclr_pin18(&self) -> MemResult<bool> {
        Ok(self.gpio[18].is_out_high())
    }
    #[doc = "GPIO OUTCLR PIN18: Pin 18."]
    #[inline]
    pub(crate) fn write_gpio_outclr_pin18(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[18].set_out_high(false);
        }
        Ok(())
    }
    #[doc = "GPIO OUTCLR PIN19: Pin 19."]
    #[inline]
    pub(crate) fn read_gpio_outclr_pin19(&self) -> MemResult<bool> {
        Ok(self.gpio[19].is_out_high())
    }
    #[doc = "GPIO OUTCLR PIN19: Pin 19."]
    #[inline]
    pub(crate) fn write_gpio_outclr_pin19(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[19].set_out_high(false);
        }
        Ok(())
    }
    #[doc = "GPIO OUTCLR PIN20: Pin 20."]
    #[inline]
    pub(crate) fn read_gpio_outclr_pin20(&self) -> MemResult<bool> {
        Ok(self.gpio[20].is_out_high())
    }
    #[doc = "GPIO OUTCLR PIN20: Pin 20."]
    #[inline]
    pub(crate) fn write_gpio_outclr_pin20(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[20].set_out_high(false);
        }
        Ok(())
    }
    #[doc = "GPIO OUTCLR PIN21: Pin 21."]
    #[inline]
    pub(crate) fn read_gpio_outclr_pin21(&self) -> MemResult<bool> {
        Ok(self.gpio[21].is_out_high())
    }
    #[doc = "GPIO OUTCLR PIN21: Pin 21."]
    #[inline]
    pub(crate) fn write_gpio_outclr_pin21(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[21].set_out_high(false);
        }
        Ok(())
    }
    #[doc = "GPIO OUTCLR PIN22: Pin 22."]
    #[inline]
    pub(crate) fn read_gpio_outclr_pin22(&self) -> MemResult<bool> {
        Ok(self.gpio[22].is_out_high())
    }
    #[doc = "GPIO OUTCLR PIN22: Pin 22."]
    #[inline]
    pub(crate) fn write_gpio_outclr_pin22(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[22].set_out_high(false);
        }
        Ok(())
    }
    #[doc = "GPIO OUTCLR PIN23: Pin 23."]
    #[inline]
    pub(crate) fn read_gpio_outclr_pin23(&self) -> MemResult<bool> {
        Ok(self.gpio[23].is_out_high())
    }
    #[doc = "GPIO OUTCLR PIN23: Pin 23."]
    #[inline]
    pub(crate) fn write_gpio_outclr_pin23(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[23].set_out_high(false);
        }
        Ok(())
    }
    #[doc = "GPIO OUTCLR PIN24: Pin 24."]
    #[inline]
    pub(crate) fn read_gpio_outclr_pin24(&self) -> MemResult<bool> {
        Ok(self.gpio[24].is_out_high())
    }
    #[doc = "GPIO OUTCLR PIN24: Pin 24."]
    #[inline]
    pub(crate) fn write_gpio_outclr_pin24(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[24].set_out_high(false);
        }
        Ok(())
    }
    #[doc = "GPIO OUTCLR PIN25: Pin 25."]
    #[inline]
    pub(crate) fn read_gpio_outclr_pin25(&self) -> MemResult<bool> {
        Ok(self.gpio[25].is_out_high())
    }
    #[doc = "GPIO OUTCLR PIN25: Pin 25."]
    #[inline]
    pub(crate) fn write_gpio_outclr_pin25(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[25].set_out_high(false);
        }
        Ok(())
    }
    #[doc = "GPIO OUTCLR PIN26: Pin 26."]
    #[inline]
    pub(crate) fn read_gpio_outclr_pin26(&self) -> MemResult<bool> {
        Ok(self.gpio[26].is_out_high())
    }
    #[doc = "GPIO OUTCLR PIN26: Pin 26."]
    #[inline]
    pub(crate) fn write_gpio_outclr_pin26(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[26].set_out_high(false);
        }
        Ok(())
    }
    #[doc = "GPIO OUTCLR PIN27: Pin 27."]
    #[inline]
    pub(crate) fn read_gpio_outclr_pin27(&self) -> MemResult<bool> {
        Ok(self.gpio[27].is_out_high())
    }
    #[doc = "GPIO OUTCLR PIN27: Pin 27."]
    #[inline]
    pub(crate) fn write_gpio_outclr_pin27(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[27].set_out_high(false);
        }
        Ok(())
    }
    #[doc = "GPIO OUTCLR PIN28: Pin 28."]
    #[inline]
    pub(crate) fn read_gpio_outclr_pin28(&self) -> MemResult<bool> {
        Ok(self.gpio[28].is_out_high())
    }
    #[doc = "GPIO OUTCLR PIN28: Pin 28."]
    #[inline]
    pub(crate) fn write_gpio_outclr_pin28(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[28].set_out_high(false);
        }
        Ok(())
    }
    #[doc = "GPIO OUTCLR PIN29: Pin 29."]
    #[inline]
    pub(crate) fn read_gpio_outclr_pin29(&self) -> MemResult<bool> {
        Ok(self.gpio[29].is_out_high())
    }
    #[doc = "GPIO OUTCLR PIN29: Pin 29."]
    #[inline]
    pub(crate) fn write_gpio_outclr_pin29(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[29].set_out_high(false);
        }
        Ok(())
    }
    #[doc = "GPIO OUTCLR PIN30: Pin 30."]
    #[inline]
    pub(crate) fn read_gpio_outclr_pin30(&self) -> MemResult<bool> {
        Ok(self.gpio[30].is_out_high())
    }
    #[doc = "GPIO OUTCLR PIN30: Pin 30."]
    #[inline]
    pub(crate) fn write_gpio_outclr_pin30(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[30].set_out_high(false);
        }
        Ok(())
    }
    #[doc = "GPIO OUTCLR PIN31: Pin 31."]
    #[inline]
    pub(crate) fn read_gpio_outclr_pin31(&self) -> MemResult<bool> {
        Ok(self.gpio[31].is_out_high())
    }
    #[doc = "GPIO OUTCLR PIN31: Pin 31."]
    #[inline]
    pub(crate) fn write_gpio_outclr_pin31(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[31].set_out_high(false);
        }
        Ok(())
    }
    #[doc = "GPIO IN PIN0: Pin 0."]
    #[inline]
    pub(crate) fn read_gpio_in_pin0(&self) -> MemResult<bool> {
        Ok(self.gpio[0].read_input())
    }
    #[doc = "GPIO IN PIN1: Pin 1."]
    #[inline]
    pub(crate) fn read_gpio_in_pin1(&self) -> MemResult<bool> {
        Ok(self.gpio[1].read_input())
    }
    #[doc = "GPIO IN PIN2: Pin 2."]
    #[inline]
    pub(crate) fn read_gpio_in_pin2(&self) -> MemResult<bool> {
        Ok(self.gpio[2].read_input())
    }
    #[doc = "GPIO IN PIN3: Pin 3."]
    #[inline]
    pub(crate) fn read_gpio_in_pin3(&self) -> MemResult<bool> {
        Ok(self.gpio[3].read_input())
    }
    #[doc = "GPIO IN PIN4: Pin 4."]
    #[inline]
    pub(crate) fn read_gpio_in_pin4(&self) -> MemResult<bool> {
        Ok(self.gpio[4].read_input())
    }
    #[doc = "GPIO IN PIN5: Pin 5."]
    #[inline]
    pub(crate) fn read_gpio_in_pin5(&self) -> MemResult<bool> {
        Ok(self.gpio[5].read_input())
    }
    #[doc = "GPIO IN PIN6: Pin 6."]
    #[inline]
    pub(crate) fn read_gpio_in_pin6(&self) -> MemResult<bool> {
        Ok(self.gpio[6].read_input())
    }
    #[doc = "GPIO IN PIN7: Pin 7."]
    #[inline]
    pub(crate) fn read_gpio_in_pin7(&self) -> MemResult<bool> {
        Ok(self.gpio[7].read_input())
    }
    #[doc = "GPIO IN PIN8: Pin 8."]
    #[inline]
    pub(crate) fn read_gpio_in_pin8(&self) -> MemResult<bool> {
        Ok(self.gpio[8].read_input())
    }
    #[doc = "GPIO IN PIN9: Pin 9."]
    #[inline]
    pub(crate) fn read_gpio_in_pin9(&self) -> MemResult<bool> {
        Ok(self.gpio[9].read_input())
    }
    #[doc = "GPIO IN PIN10: Pin 10."]
    #[inline]
    pub(crate) fn read_gpio_in_pin10(&self) -> MemResult<bool> {
        Ok(self.gpio[10].read_input())
    }
    #[doc = "GPIO IN PIN11: Pin 11."]
    #[inline]
    pub(crate) fn read_gpio_in_pin11(&self) -> MemResult<bool> {
        Ok(self.gpio[11].read_input())
    }
    #[doc = "GPIO IN PIN12: Pin 12."]
    #[inline]
    pub(crate) fn read_gpio_in_pin12(&self) -> MemResult<bool> {
        Ok(self.gpio[12].read_input())
    }
    #[doc = "GPIO IN PIN13: Pin 13."]
    #[inline]
    pub(crate) fn read_gpio_in_pin13(&self) -> MemResult<bool> {
        Ok(self.gpio[13].read_input())
    }
    #[doc = "GPIO IN PIN14: Pin 14."]
    #[inline]
    pub(crate) fn read_gpio_in_pin14(&self) -> MemResult<bool> {
        Ok(self.gpio[14].read_input())
    }
    #[doc = "GPIO IN PIN15: Pin 15."]
    #[inline]
    pub(crate) fn read_gpio_in_pin15(&self) -> MemResult<bool> {
        Ok(self.gpio[15].read_input())
    }
    #[doc = "GPIO IN PIN16: Pin 16."]
    #[inline]
    pub(crate) fn read_gpio_in_pin16(&self) -> MemResult<bool> {
        Ok(self.gpio[16].read_input())
    }
    #[doc = "GPIO IN PIN17: Pin 17."]
    #[inline]
    pub(crate) fn read_gpio_in_pin17(&self) -> MemResult<bool> {
        Ok(self.gpio[17].read_input())
    }
    #[doc = "GPIO IN PIN18: Pin 18."]
    #[inline]
    pub(crate) fn read_gpio_in_pin18(&self) -> MemResult<bool> {
        Ok(self.gpio[18].read_input())
    }
    #[doc = "GPIO IN PIN19: Pin 19."]
    #[inline]
    pub(crate) fn read_gpio_in_pin19(&self) -> MemResult<bool> {
        Ok(self.gpio[19].read_input())
    }
    #[doc = "GPIO IN PIN20: Pin 20."]
    #[inline]
    pub(crate) fn read_gpio_in_pin20(&self) -> MemResult<bool> {
        Ok(self.gpio[20].read_input())
    }
    #[doc = "GPIO IN PIN21: Pin 21."]
    #[inline]
    pub(crate) fn read_gpio_in_pin21(&self) -> MemResult<bool> {
        Ok(self.gpio[21].read_input())
    }
    #[doc = "GPIO IN PIN22: Pin 22."]
    #[inline]
    pub(crate) fn read_gpio_in_pin22(&self) -> MemResult<bool> {
        Ok(self.gpio[22].read_input())
    }
    #[doc = "GPIO IN PIN23: Pin 23."]
    #[inline]
    pub(crate) fn read_gpio_in_pin23(&self) -> MemResult<bool> {
        Ok(self.gpio[23].read_input())
    }
    #[doc = "GPIO IN PIN24: Pin 24."]
    #[inline]
    pub(crate) fn read_gpio_in_pin24(&self) -> MemResult<bool> {
        Ok(self.gpio[24].read_input())
    }
    #[doc = "GPIO IN PIN25: Pin 25."]
    #[inline]
    pub(crate) fn read_gpio_in_pin25(&self) -> MemResult<bool> {
        Ok(self.gpio[25].read_input())
    }
    #[doc = "GPIO IN PIN26: Pin 26."]
    #[inline]
    pub(crate) fn read_gpio_in_pin26(&self) -> MemResult<bool> {
        Ok(self.gpio[26].read_input())
    }
    #[doc = "GPIO IN PIN27: Pin 27."]
    #[inline]
    pub(crate) fn read_gpio_in_pin27(&self) -> MemResult<bool> {
        Ok(self.gpio[27].read_input())
    }
    #[doc = "GPIO IN PIN28: Pin 28."]
    #[inline]
    pub(crate) fn read_gpio_in_pin28(&self) -> MemResult<bool> {
        Ok(self.gpio[28].read_input())
    }
    #[doc = "GPIO IN PIN29: Pin 29."]
    #[inline]
    pub(crate) fn read_gpio_in_pin29(&self) -> MemResult<bool> {
        Ok(self.gpio[29].read_input())
    }
    #[doc = "GPIO IN PIN30: Pin 30."]
    #[inline]
    pub(crate) fn read_gpio_in_pin30(&self) -> MemResult<bool> {
        Ok(self.gpio[30].read_input())
    }
    #[doc = "GPIO IN PIN31: Pin 31."]
    #[inline]
    pub(crate) fn read_gpio_in_pin31(&self) -> MemResult<bool> {
        Ok(self.gpio[31].read_input())
    }
    #[doc = "GPIO DIR PIN0: Pin 0."]
    #[inline]
    pub(crate) fn read_gpio_dir_pin0(&self) -> MemResult<bool> {
        Ok(self.gpio[0].get_direction())
    }
    #[doc = "GPIO DIR PIN0: Pin 0."]
    #[inline]
    pub(crate) fn write_gpio_dir_pin0(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[0].set_direction(_value))
    }
    #[doc = "GPIO DIR PIN1: Pin 1."]
    #[inline]
    pub(crate) fn read_gpio_dir_pin1(&self) -> MemResult<bool> {
        Ok(self.gpio[1].get_direction())
    }
    #[doc = "GPIO DIR PIN1: Pin 1."]
    #[inline]
    pub(crate) fn write_gpio_dir_pin1(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[1].set_direction(_value))
    }
    #[doc = "GPIO DIR PIN2: Pin 2."]
    #[inline]
    pub(crate) fn read_gpio_dir_pin2(&self) -> MemResult<bool> {
        Ok(self.gpio[2].get_direction())
    }
    #[doc = "GPIO DIR PIN2: Pin 2."]
    #[inline]
    pub(crate) fn write_gpio_dir_pin2(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[2].set_direction(_value))
    }
    #[doc = "GPIO DIR PIN3: Pin 3."]
    #[inline]
    pub(crate) fn read_gpio_dir_pin3(&self) -> MemResult<bool> {
        Ok(self.gpio[3].get_direction())
    }
    #[doc = "GPIO DIR PIN3: Pin 3."]
    #[inline]
    pub(crate) fn write_gpio_dir_pin3(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[3].set_direction(_value))
    }
    #[doc = "GPIO DIR PIN4: Pin 4."]
    #[inline]
    pub(crate) fn read_gpio_dir_pin4(&self) -> MemResult<bool> {
        Ok(self.gpio[4].get_direction())
    }
    #[doc = "GPIO DIR PIN4: Pin 4."]
    #[inline]
    pub(crate) fn write_gpio_dir_pin4(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[4].set_direction(_value))
    }
    #[doc = "GPIO DIR PIN5: Pin 5."]
    #[inline]
    pub(crate) fn read_gpio_dir_pin5(&self) -> MemResult<bool> {
        Ok(self.gpio[5].get_direction())
    }
    #[doc = "GPIO DIR PIN5: Pin 5."]
    #[inline]
    pub(crate) fn write_gpio_dir_pin5(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[5].set_direction(_value))
    }
    #[doc = "GPIO DIR PIN6: Pin 6."]
    #[inline]
    pub(crate) fn read_gpio_dir_pin6(&self) -> MemResult<bool> {
        Ok(self.gpio[6].get_direction())
    }
    #[doc = "GPIO DIR PIN6: Pin 6."]
    #[inline]
    pub(crate) fn write_gpio_dir_pin6(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[6].set_direction(_value))
    }
    #[doc = "GPIO DIR PIN7: Pin 7."]
    #[inline]
    pub(crate) fn read_gpio_dir_pin7(&self) -> MemResult<bool> {
        Ok(self.gpio[7].get_direction())
    }
    #[doc = "GPIO DIR PIN7: Pin 7."]
    #[inline]
    pub(crate) fn write_gpio_dir_pin7(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[7].set_direction(_value))
    }
    #[doc = "GPIO DIR PIN8: Pin 8."]
    #[inline]
    pub(crate) fn read_gpio_dir_pin8(&self) -> MemResult<bool> {
        Ok(self.gpio[8].get_direction())
    }
    #[doc = "GPIO DIR PIN8: Pin 8."]
    #[inline]
    pub(crate) fn write_gpio_dir_pin8(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[8].set_direction(_value))
    }
    #[doc = "GPIO DIR PIN9: Pin 9."]
    #[inline]
    pub(crate) fn read_gpio_dir_pin9(&self) -> MemResult<bool> {
        Ok(self.gpio[9].get_direction())
    }
    #[doc = "GPIO DIR PIN9: Pin 9."]
    #[inline]
    pub(crate) fn write_gpio_dir_pin9(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[9].set_direction(_value))
    }
    #[doc = "GPIO DIR PIN10: Pin 10."]
    #[inline]
    pub(crate) fn read_gpio_dir_pin10(&self) -> MemResult<bool> {
        Ok(self.gpio[10].get_direction())
    }
    #[doc = "GPIO DIR PIN10: Pin 10."]
    #[inline]
    pub(crate) fn write_gpio_dir_pin10(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[10].set_direction(_value))
    }
    #[doc = "GPIO DIR PIN11: Pin 11."]
    #[inline]
    pub(crate) fn read_gpio_dir_pin11(&self) -> MemResult<bool> {
        Ok(self.gpio[11].get_direction())
    }
    #[doc = "GPIO DIR PIN11: Pin 11."]
    #[inline]
    pub(crate) fn write_gpio_dir_pin11(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[11].set_direction(_value))
    }
    #[doc = "GPIO DIR PIN12: Pin 12."]
    #[inline]
    pub(crate) fn read_gpio_dir_pin12(&self) -> MemResult<bool> {
        Ok(self.gpio[12].get_direction())
    }
    #[doc = "GPIO DIR PIN12: Pin 12."]
    #[inline]
    pub(crate) fn write_gpio_dir_pin12(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[12].set_direction(_value))
    }
    #[doc = "GPIO DIR PIN13: Pin 13."]
    #[inline]
    pub(crate) fn read_gpio_dir_pin13(&self) -> MemResult<bool> {
        Ok(self.gpio[13].get_direction())
    }
    #[doc = "GPIO DIR PIN13: Pin 13."]
    #[inline]
    pub(crate) fn write_gpio_dir_pin13(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[13].set_direction(_value))
    }
    #[doc = "GPIO DIR PIN14: Pin 14."]
    #[inline]
    pub(crate) fn read_gpio_dir_pin14(&self) -> MemResult<bool> {
        Ok(self.gpio[14].get_direction())
    }
    #[doc = "GPIO DIR PIN14: Pin 14."]
    #[inline]
    pub(crate) fn write_gpio_dir_pin14(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[14].set_direction(_value))
    }
    #[doc = "GPIO DIR PIN15: Pin 15."]
    #[inline]
    pub(crate) fn read_gpio_dir_pin15(&self) -> MemResult<bool> {
        Ok(self.gpio[15].get_direction())
    }
    #[doc = "GPIO DIR PIN15: Pin 15."]
    #[inline]
    pub(crate) fn write_gpio_dir_pin15(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[15].set_direction(_value))
    }
    #[doc = "GPIO DIR PIN16: Pin 16."]
    #[inline]
    pub(crate) fn read_gpio_dir_pin16(&self) -> MemResult<bool> {
        Ok(self.gpio[16].get_direction())
    }
    #[doc = "GPIO DIR PIN16: Pin 16."]
    #[inline]
    pub(crate) fn write_gpio_dir_pin16(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[16].set_direction(_value))
    }
    #[doc = "GPIO DIR PIN17: Pin 17."]
    #[inline]
    pub(crate) fn read_gpio_dir_pin17(&self) -> MemResult<bool> {
        Ok(self.gpio[17].get_direction())
    }
    #[doc = "GPIO DIR PIN17: Pin 17."]
    #[inline]
    pub(crate) fn write_gpio_dir_pin17(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[17].set_direction(_value))
    }
    #[doc = "GPIO DIR PIN18: Pin 18."]
    #[inline]
    pub(crate) fn read_gpio_dir_pin18(&self) -> MemResult<bool> {
        Ok(self.gpio[18].get_direction())
    }
    #[doc = "GPIO DIR PIN18: Pin 18."]
    #[inline]
    pub(crate) fn write_gpio_dir_pin18(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[18].set_direction(_value))
    }
    #[doc = "GPIO DIR PIN19: Pin 19."]
    #[inline]
    pub(crate) fn read_gpio_dir_pin19(&self) -> MemResult<bool> {
        Ok(self.gpio[19].get_direction())
    }
    #[doc = "GPIO DIR PIN19: Pin 19."]
    #[inline]
    pub(crate) fn write_gpio_dir_pin19(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[19].set_direction(_value))
    }
    #[doc = "GPIO DIR PIN20: Pin 20."]
    #[inline]
    pub(crate) fn read_gpio_dir_pin20(&self) -> MemResult<bool> {
        Ok(self.gpio[20].get_direction())
    }
    #[doc = "GPIO DIR PIN20: Pin 20."]
    #[inline]
    pub(crate) fn write_gpio_dir_pin20(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[20].set_direction(_value))
    }
    #[doc = "GPIO DIR PIN21: Pin 21."]
    #[inline]
    pub(crate) fn read_gpio_dir_pin21(&self) -> MemResult<bool> {
        Ok(self.gpio[21].get_direction())
    }
    #[doc = "GPIO DIR PIN21: Pin 21."]
    #[inline]
    pub(crate) fn write_gpio_dir_pin21(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[21].set_direction(_value))
    }
    #[doc = "GPIO DIR PIN22: Pin 22."]
    #[inline]
    pub(crate) fn read_gpio_dir_pin22(&self) -> MemResult<bool> {
        Ok(self.gpio[22].get_direction())
    }
    #[doc = "GPIO DIR PIN22: Pin 22."]
    #[inline]
    pub(crate) fn write_gpio_dir_pin22(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[22].set_direction(_value))
    }
    #[doc = "GPIO DIR PIN23: Pin 23."]
    #[inline]
    pub(crate) fn read_gpio_dir_pin23(&self) -> MemResult<bool> {
        Ok(self.gpio[23].get_direction())
    }
    #[doc = "GPIO DIR PIN23: Pin 23."]
    #[inline]
    pub(crate) fn write_gpio_dir_pin23(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[23].set_direction(_value))
    }
    #[doc = "GPIO DIR PIN24: Pin 24."]
    #[inline]
    pub(crate) fn read_gpio_dir_pin24(&self) -> MemResult<bool> {
        Ok(self.gpio[24].get_direction())
    }
    #[doc = "GPIO DIR PIN24: Pin 24."]
    #[inline]
    pub(crate) fn write_gpio_dir_pin24(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[24].set_direction(_value))
    }
    #[doc = "GPIO DIR PIN25: Pin 25."]
    #[inline]
    pub(crate) fn read_gpio_dir_pin25(&self) -> MemResult<bool> {
        Ok(self.gpio[25].get_direction())
    }
    #[doc = "GPIO DIR PIN25: Pin 25."]
    #[inline]
    pub(crate) fn write_gpio_dir_pin25(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[25].set_direction(_value))
    }
    #[doc = "GPIO DIR PIN26: Pin 26."]
    #[inline]
    pub(crate) fn read_gpio_dir_pin26(&self) -> MemResult<bool> {
        Ok(self.gpio[26].get_direction())
    }
    #[doc = "GPIO DIR PIN26: Pin 26."]
    #[inline]
    pub(crate) fn write_gpio_dir_pin26(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[26].set_direction(_value))
    }
    #[doc = "GPIO DIR PIN27: Pin 27."]
    #[inline]
    pub(crate) fn read_gpio_dir_pin27(&self) -> MemResult<bool> {
        Ok(self.gpio[27].get_direction())
    }
    #[doc = "GPIO DIR PIN27: Pin 27."]
    #[inline]
    pub(crate) fn write_gpio_dir_pin27(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[27].set_direction(_value))
    }
    #[doc = "GPIO DIR PIN28: Pin 28."]
    #[inline]
    pub(crate) fn read_gpio_dir_pin28(&self) -> MemResult<bool> {
        Ok(self.gpio[28].get_direction())
    }
    #[doc = "GPIO DIR PIN28: Pin 28."]
    #[inline]
    pub(crate) fn write_gpio_dir_pin28(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[28].set_direction(_value))
    }
    #[doc = "GPIO DIR PIN29: Pin 29."]
    #[inline]
    pub(crate) fn read_gpio_dir_pin29(&self) -> MemResult<bool> {
        Ok(self.gpio[29].get_direction())
    }
    #[doc = "GPIO DIR PIN29: Pin 29."]
    #[inline]
    pub(crate) fn write_gpio_dir_pin29(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[29].set_direction(_value))
    }
    #[doc = "GPIO DIR PIN30: Pin 30."]
    #[inline]
    pub(crate) fn read_gpio_dir_pin30(&self) -> MemResult<bool> {
        Ok(self.gpio[30].get_direction())
    }
    #[doc = "GPIO DIR PIN30: Pin 30."]
    #[inline]
    pub(crate) fn write_gpio_dir_pin30(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[30].set_direction(_value))
    }
    #[doc = "GPIO DIR PIN31: Pin 31."]
    #[inline]
    pub(crate) fn read_gpio_dir_pin31(&self) -> MemResult<bool> {
        Ok(self.gpio[31].get_direction())
    }
    #[doc = "GPIO DIR PIN31: Pin 31."]
    #[inline]
    pub(crate) fn write_gpio_dir_pin31(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[31].set_direction(_value))
    }
    #[doc = "GPIO DIRSET PIN0: Set as output pin 0."]
    #[inline]
    pub(crate) fn read_gpio_dirset_pin0(&self) -> MemResult<bool> {
        Ok(self.gpio[0].get_direction())
    }
    #[doc = "GPIO DIRSET PIN0: Set as output pin 0."]
    #[inline]
    pub(crate) fn write_gpio_dirset_pin0(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[0].set_direction(true);
        }
        Ok(())
    }
    #[doc = "GPIO DIRSET PIN1: Set as output pin 1."]
    #[inline]
    pub(crate) fn read_gpio_dirset_pin1(&self) -> MemResult<bool> {
        Ok(self.gpio[1].get_direction())
    }
    #[doc = "GPIO DIRSET PIN1: Set as output pin 1."]
    #[inline]
    pub(crate) fn write_gpio_dirset_pin1(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[1].set_direction(true);
        }
        Ok(())
    }
    #[doc = "GPIO DIRSET PIN2: Set as output pin 2."]
    #[inline]
    pub(crate) fn read_gpio_dirset_pin2(&self) -> MemResult<bool> {
        Ok(self.gpio[2].get_direction())
    }
    #[doc = "GPIO DIRSET PIN2: Set as output pin 2."]
    #[inline]
    pub(crate) fn write_gpio_dirset_pin2(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[2].set_direction(true);
        }
        Ok(())
    }
    #[doc = "GPIO DIRSET PIN3: Set as output pin 3."]
    #[inline]
    pub(crate) fn read_gpio_dirset_pin3(&self) -> MemResult<bool> {
        Ok(self.gpio[3].get_direction())
    }
    #[doc = "GPIO DIRSET PIN3: Set as output pin 3."]
    #[inline]
    pub(crate) fn write_gpio_dirset_pin3(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[3].set_direction(true);
        }
        Ok(())
    }
    #[doc = "GPIO DIRSET PIN4: Set as output pin 4."]
    #[inline]
    pub(crate) fn read_gpio_dirset_pin4(&self) -> MemResult<bool> {
        Ok(self.gpio[4].get_direction())
    }
    #[doc = "GPIO DIRSET PIN4: Set as output pin 4."]
    #[inline]
    pub(crate) fn write_gpio_dirset_pin4(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[4].set_direction(true);
        }
        Ok(())
    }
    #[doc = "GPIO DIRSET PIN5: Set as output pin 5."]
    #[inline]
    pub(crate) fn read_gpio_dirset_pin5(&self) -> MemResult<bool> {
        Ok(self.gpio[5].get_direction())
    }
    #[doc = "GPIO DIRSET PIN5: Set as output pin 5."]
    #[inline]
    pub(crate) fn write_gpio_dirset_pin5(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[5].set_direction(true);
        }
        Ok(())
    }
    #[doc = "GPIO DIRSET PIN6: Set as output pin 6."]
    #[inline]
    pub(crate) fn read_gpio_dirset_pin6(&self) -> MemResult<bool> {
        Ok(self.gpio[6].get_direction())
    }
    #[doc = "GPIO DIRSET PIN6: Set as output pin 6."]
    #[inline]
    pub(crate) fn write_gpio_dirset_pin6(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[6].set_direction(true);
        }
        Ok(())
    }
    #[doc = "GPIO DIRSET PIN7: Set as output pin 7."]
    #[inline]
    pub(crate) fn read_gpio_dirset_pin7(&self) -> MemResult<bool> {
        Ok(self.gpio[7].get_direction())
    }
    #[doc = "GPIO DIRSET PIN7: Set as output pin 7."]
    #[inline]
    pub(crate) fn write_gpio_dirset_pin7(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[7].set_direction(true);
        }
        Ok(())
    }
    #[doc = "GPIO DIRSET PIN8: Set as output pin 8."]
    #[inline]
    pub(crate) fn read_gpio_dirset_pin8(&self) -> MemResult<bool> {
        Ok(self.gpio[8].get_direction())
    }
    #[doc = "GPIO DIRSET PIN8: Set as output pin 8."]
    #[inline]
    pub(crate) fn write_gpio_dirset_pin8(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[8].set_direction(true);
        }
        Ok(())
    }
    #[doc = "GPIO DIRSET PIN9: Set as output pin 9."]
    #[inline]
    pub(crate) fn read_gpio_dirset_pin9(&self) -> MemResult<bool> {
        Ok(self.gpio[9].get_direction())
    }
    #[doc = "GPIO DIRSET PIN9: Set as output pin 9."]
    #[inline]
    pub(crate) fn write_gpio_dirset_pin9(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[9].set_direction(true);
        }
        Ok(())
    }
    #[doc = "GPIO DIRSET PIN10: Set as output pin 10."]
    #[inline]
    pub(crate) fn read_gpio_dirset_pin10(&self) -> MemResult<bool> {
        Ok(self.gpio[10].get_direction())
    }
    #[doc = "GPIO DIRSET PIN10: Set as output pin 10."]
    #[inline]
    pub(crate) fn write_gpio_dirset_pin10(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[10].set_direction(true);
        }
        Ok(())
    }
    #[doc = "GPIO DIRSET PIN11: Set as output pin 11."]
    #[inline]
    pub(crate) fn read_gpio_dirset_pin11(&self) -> MemResult<bool> {
        Ok(self.gpio[11].get_direction())
    }
    #[doc = "GPIO DIRSET PIN11: Set as output pin 11."]
    #[inline]
    pub(crate) fn write_gpio_dirset_pin11(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[11].set_direction(true);
        }
        Ok(())
    }
    #[doc = "GPIO DIRSET PIN12: Set as output pin 12."]
    #[inline]
    pub(crate) fn read_gpio_dirset_pin12(&self) -> MemResult<bool> {
        Ok(self.gpio[12].get_direction())
    }
    #[doc = "GPIO DIRSET PIN12: Set as output pin 12."]
    #[inline]
    pub(crate) fn write_gpio_dirset_pin12(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[12].set_direction(true);
        }
        Ok(())
    }
    #[doc = "GPIO DIRSET PIN13: Set as output pin 13."]
    #[inline]
    pub(crate) fn read_gpio_dirset_pin13(&self) -> MemResult<bool> {
        Ok(self.gpio[13].get_direction())
    }
    #[doc = "GPIO DIRSET PIN13: Set as output pin 13."]
    #[inline]
    pub(crate) fn write_gpio_dirset_pin13(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[13].set_direction(true);
        }
        Ok(())
    }
    #[doc = "GPIO DIRSET PIN14: Set as output pin 14."]
    #[inline]
    pub(crate) fn read_gpio_dirset_pin14(&self) -> MemResult<bool> {
        Ok(self.gpio[14].get_direction())
    }
    #[doc = "GPIO DIRSET PIN14: Set as output pin 14."]
    #[inline]
    pub(crate) fn write_gpio_dirset_pin14(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[14].set_direction(true);
        }
        Ok(())
    }
    #[doc = "GPIO DIRSET PIN15: Set as output pin 15."]
    #[inline]
    pub(crate) fn read_gpio_dirset_pin15(&self) -> MemResult<bool> {
        Ok(self.gpio[15].get_direction())
    }
    #[doc = "GPIO DIRSET PIN15: Set as output pin 15."]
    #[inline]
    pub(crate) fn write_gpio_dirset_pin15(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[15].set_direction(true);
        }
        Ok(())
    }
    #[doc = "GPIO DIRSET PIN16: Set as output pin 16."]
    #[inline]
    pub(crate) fn read_gpio_dirset_pin16(&self) -> MemResult<bool> {
        Ok(self.gpio[16].get_direction())
    }
    #[doc = "GPIO DIRSET PIN16: Set as output pin 16."]
    #[inline]
    pub(crate) fn write_gpio_dirset_pin16(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[16].set_direction(true);
        }
        Ok(())
    }
    #[doc = "GPIO DIRSET PIN17: Set as output pin 17."]
    #[inline]
    pub(crate) fn read_gpio_dirset_pin17(&self) -> MemResult<bool> {
        Ok(self.gpio[17].get_direction())
    }
    #[doc = "GPIO DIRSET PIN17: Set as output pin 17."]
    #[inline]
    pub(crate) fn write_gpio_dirset_pin17(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[17].set_direction(true);
        }
        Ok(())
    }
    #[doc = "GPIO DIRSET PIN18: Set as output pin 18."]
    #[inline]
    pub(crate) fn read_gpio_dirset_pin18(&self) -> MemResult<bool> {
        Ok(self.gpio[18].get_direction())
    }
    #[doc = "GPIO DIRSET PIN18: Set as output pin 18."]
    #[inline]
    pub(crate) fn write_gpio_dirset_pin18(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[18].set_direction(true);
        }
        Ok(())
    }
    #[doc = "GPIO DIRSET PIN19: Set as output pin 19."]
    #[inline]
    pub(crate) fn read_gpio_dirset_pin19(&self) -> MemResult<bool> {
        Ok(self.gpio[19].get_direction())
    }
    #[doc = "GPIO DIRSET PIN19: Set as output pin 19."]
    #[inline]
    pub(crate) fn write_gpio_dirset_pin19(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[19].set_direction(true);
        }
        Ok(())
    }
    #[doc = "GPIO DIRSET PIN20: Set as output pin 20."]
    #[inline]
    pub(crate) fn read_gpio_dirset_pin20(&self) -> MemResult<bool> {
        Ok(self.gpio[20].get_direction())
    }
    #[doc = "GPIO DIRSET PIN20: Set as output pin 20."]
    #[inline]
    pub(crate) fn write_gpio_dirset_pin20(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[20].set_direction(true);
        }
        Ok(())
    }
    #[doc = "GPIO DIRSET PIN21: Set as output pin 21."]
    #[inline]
    pub(crate) fn read_gpio_dirset_pin21(&self) -> MemResult<bool> {
        Ok(self.gpio[21].get_direction())
    }
    #[doc = "GPIO DIRSET PIN21: Set as output pin 21."]
    #[inline]
    pub(crate) fn write_gpio_dirset_pin21(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[21].set_direction(true);
        }
        Ok(())
    }
    #[doc = "GPIO DIRSET PIN22: Set as output pin 22."]
    #[inline]
    pub(crate) fn read_gpio_dirset_pin22(&self) -> MemResult<bool> {
        Ok(self.gpio[22].get_direction())
    }
    #[doc = "GPIO DIRSET PIN22: Set as output pin 22."]
    #[inline]
    pub(crate) fn write_gpio_dirset_pin22(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[22].set_direction(true);
        }
        Ok(())
    }
    #[doc = "GPIO DIRSET PIN23: Set as output pin 23."]
    #[inline]
    pub(crate) fn read_gpio_dirset_pin23(&self) -> MemResult<bool> {
        Ok(self.gpio[23].get_direction())
    }
    #[doc = "GPIO DIRSET PIN23: Set as output pin 23."]
    #[inline]
    pub(crate) fn write_gpio_dirset_pin23(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[23].set_direction(true);
        }
        Ok(())
    }
    #[doc = "GPIO DIRSET PIN24: Set as output pin 24."]
    #[inline]
    pub(crate) fn read_gpio_dirset_pin24(&self) -> MemResult<bool> {
        Ok(self.gpio[24].get_direction())
    }
    #[doc = "GPIO DIRSET PIN24: Set as output pin 24."]
    #[inline]
    pub(crate) fn write_gpio_dirset_pin24(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[24].set_direction(true);
        }
        Ok(())
    }
    #[doc = "GPIO DIRSET PIN25: Set as output pin 25."]
    #[inline]
    pub(crate) fn read_gpio_dirset_pin25(&self) -> MemResult<bool> {
        Ok(self.gpio[25].get_direction())
    }
    #[doc = "GPIO DIRSET PIN25: Set as output pin 25."]
    #[inline]
    pub(crate) fn write_gpio_dirset_pin25(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[25].set_direction(true);
        }
        Ok(())
    }
    #[doc = "GPIO DIRSET PIN26: Set as output pin 26."]
    #[inline]
    pub(crate) fn read_gpio_dirset_pin26(&self) -> MemResult<bool> {
        Ok(self.gpio[26].get_direction())
    }
    #[doc = "GPIO DIRSET PIN26: Set as output pin 26."]
    #[inline]
    pub(crate) fn write_gpio_dirset_pin26(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[26].set_direction(true);
        }
        Ok(())
    }
    #[doc = "GPIO DIRSET PIN27: Set as output pin 27."]
    #[inline]
    pub(crate) fn read_gpio_dirset_pin27(&self) -> MemResult<bool> {
        Ok(self.gpio[27].get_direction())
    }
    #[doc = "GPIO DIRSET PIN27: Set as output pin 27."]
    #[inline]
    pub(crate) fn write_gpio_dirset_pin27(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[27].set_direction(true);
        }
        Ok(())
    }
    #[doc = "GPIO DIRSET PIN28: Set as output pin 28."]
    #[inline]
    pub(crate) fn read_gpio_dirset_pin28(&self) -> MemResult<bool> {
        Ok(self.gpio[28].get_direction())
    }
    #[doc = "GPIO DIRSET PIN28: Set as output pin 28."]
    #[inline]
    pub(crate) fn write_gpio_dirset_pin28(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[28].set_direction(true);
        }
        Ok(())
    }
    #[doc = "GPIO DIRSET PIN29: Set as output pin 29."]
    #[inline]
    pub(crate) fn read_gpio_dirset_pin29(&self) -> MemResult<bool> {
        Ok(self.gpio[29].get_direction())
    }
    #[doc = "GPIO DIRSET PIN29: Set as output pin 29."]
    #[inline]
    pub(crate) fn write_gpio_dirset_pin29(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[29].set_direction(true);
        }
        Ok(())
    }
    #[doc = "GPIO DIRSET PIN30: Set as output pin 30."]
    #[inline]
    pub(crate) fn read_gpio_dirset_pin30(&self) -> MemResult<bool> {
        Ok(self.gpio[30].get_direction())
    }
    #[doc = "GPIO DIRSET PIN30: Set as output pin 30."]
    #[inline]
    pub(crate) fn write_gpio_dirset_pin30(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[30].set_direction(true);
        }
        Ok(())
    }
    #[doc = "GPIO DIRSET PIN31: Set as output pin 31."]
    #[inline]
    pub(crate) fn read_gpio_dirset_pin31(&self) -> MemResult<bool> {
        Ok(self.gpio[31].get_direction())
    }
    #[doc = "GPIO DIRSET PIN31: Set as output pin 31."]
    #[inline]
    pub(crate) fn write_gpio_dirset_pin31(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[31].set_direction(true);
        }
        Ok(())
    }
    #[doc = "GPIO DIRCLR PIN0: Set as input pin 0."]
    #[inline]
    pub(crate) fn read_gpio_dirclr_pin0(&self) -> MemResult<bool> {
        Ok(self.gpio[0].get_direction())
    }
    #[doc = "GPIO DIRCLR PIN0: Set as input pin 0."]
    #[inline]
    pub(crate) fn write_gpio_dirclr_pin0(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[0].set_direction(false);
        }
        Ok(())
    }
    #[doc = "GPIO DIRCLR PIN1: Set as input pin 1."]
    #[inline]
    pub(crate) fn read_gpio_dirclr_pin1(&self) -> MemResult<bool> {
        Ok(self.gpio[1].get_direction())
    }
    #[doc = "GPIO DIRCLR PIN1: Set as input pin 1."]
    #[inline]
    pub(crate) fn write_gpio_dirclr_pin1(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[1].set_direction(false);
        }
        Ok(())
    }
    #[doc = "GPIO DIRCLR PIN2: Set as input pin 2."]
    #[inline]
    pub(crate) fn read_gpio_dirclr_pin2(&self) -> MemResult<bool> {
        Ok(self.gpio[2].get_direction())
    }
    #[doc = "GPIO DIRCLR PIN2: Set as input pin 2."]
    #[inline]
    pub(crate) fn write_gpio_dirclr_pin2(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[2].set_direction(false);
        }
        Ok(())
    }
    #[doc = "GPIO DIRCLR PIN3: Set as input pin 3."]
    #[inline]
    pub(crate) fn read_gpio_dirclr_pin3(&self) -> MemResult<bool> {
        Ok(self.gpio[3].get_direction())
    }
    #[doc = "GPIO DIRCLR PIN3: Set as input pin 3."]
    #[inline]
    pub(crate) fn write_gpio_dirclr_pin3(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[3].set_direction(false);
        }
        Ok(())
    }
    #[doc = "GPIO DIRCLR PIN4: Set as input pin 4."]
    #[inline]
    pub(crate) fn read_gpio_dirclr_pin4(&self) -> MemResult<bool> {
        Ok(self.gpio[4].get_direction())
    }
    #[doc = "GPIO DIRCLR PIN4: Set as input pin 4."]
    #[inline]
    pub(crate) fn write_gpio_dirclr_pin4(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[4].set_direction(false);
        }
        Ok(())
    }
    #[doc = "GPIO DIRCLR PIN5: Set as input pin 5."]
    #[inline]
    pub(crate) fn read_gpio_dirclr_pin5(&self) -> MemResult<bool> {
        Ok(self.gpio[5].get_direction())
    }
    #[doc = "GPIO DIRCLR PIN5: Set as input pin 5."]
    #[inline]
    pub(crate) fn write_gpio_dirclr_pin5(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[5].set_direction(false);
        }
        Ok(())
    }
    #[doc = "GPIO DIRCLR PIN6: Set as input pin 6."]
    #[inline]
    pub(crate) fn read_gpio_dirclr_pin6(&self) -> MemResult<bool> {
        Ok(self.gpio[6].get_direction())
    }
    #[doc = "GPIO DIRCLR PIN6: Set as input pin 6."]
    #[inline]
    pub(crate) fn write_gpio_dirclr_pin6(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[6].set_direction(false);
        }
        Ok(())
    }
    #[doc = "GPIO DIRCLR PIN7: Set as input pin 7."]
    #[inline]
    pub(crate) fn read_gpio_dirclr_pin7(&self) -> MemResult<bool> {
        Ok(self.gpio[7].get_direction())
    }
    #[doc = "GPIO DIRCLR PIN7: Set as input pin 7."]
    #[inline]
    pub(crate) fn write_gpio_dirclr_pin7(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[7].set_direction(false);
        }
        Ok(())
    }
    #[doc = "GPIO DIRCLR PIN8: Set as input pin 8."]
    #[inline]
    pub(crate) fn read_gpio_dirclr_pin8(&self) -> MemResult<bool> {
        Ok(self.gpio[8].get_direction())
    }
    #[doc = "GPIO DIRCLR PIN8: Set as input pin 8."]
    #[inline]
    pub(crate) fn write_gpio_dirclr_pin8(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[8].set_direction(false);
        }
        Ok(())
    }
    #[doc = "GPIO DIRCLR PIN9: Set as input pin 9."]
    #[inline]
    pub(crate) fn read_gpio_dirclr_pin9(&self) -> MemResult<bool> {
        Ok(self.gpio[9].get_direction())
    }
    #[doc = "GPIO DIRCLR PIN9: Set as input pin 9."]
    #[inline]
    pub(crate) fn write_gpio_dirclr_pin9(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[9].set_direction(false);
        }
        Ok(())
    }
    #[doc = "GPIO DIRCLR PIN10: Set as input pin 10."]
    #[inline]
    pub(crate) fn read_gpio_dirclr_pin10(&self) -> MemResult<bool> {
        Ok(self.gpio[10].get_direction())
    }
    #[doc = "GPIO DIRCLR PIN10: Set as input pin 10."]
    #[inline]
    pub(crate) fn write_gpio_dirclr_pin10(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[10].set_direction(false);
        }
        Ok(())
    }
    #[doc = "GPIO DIRCLR PIN11: Set as input pin 11."]
    #[inline]
    pub(crate) fn read_gpio_dirclr_pin11(&self) -> MemResult<bool> {
        Ok(self.gpio[11].get_direction())
    }
    #[doc = "GPIO DIRCLR PIN11: Set as input pin 11."]
    #[inline]
    pub(crate) fn write_gpio_dirclr_pin11(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[11].set_direction(false);
        }
        Ok(())
    }
    #[doc = "GPIO DIRCLR PIN12: Set as input pin 12."]
    #[inline]
    pub(crate) fn read_gpio_dirclr_pin12(&self) -> MemResult<bool> {
        Ok(self.gpio[12].get_direction())
    }
    #[doc = "GPIO DIRCLR PIN12: Set as input pin 12."]
    #[inline]
    pub(crate) fn write_gpio_dirclr_pin12(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[12].set_direction(false);
        }
        Ok(())
    }
    #[doc = "GPIO DIRCLR PIN13: Set as input pin 13."]
    #[inline]
    pub(crate) fn read_gpio_dirclr_pin13(&self) -> MemResult<bool> {
        Ok(self.gpio[13].get_direction())
    }
    #[doc = "GPIO DIRCLR PIN13: Set as input pin 13."]
    #[inline]
    pub(crate) fn write_gpio_dirclr_pin13(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[13].set_direction(false);
        }
        Ok(())
    }
    #[doc = "GPIO DIRCLR PIN14: Set as input pin 14."]
    #[inline]
    pub(crate) fn read_gpio_dirclr_pin14(&self) -> MemResult<bool> {
        Ok(self.gpio[14].get_direction())
    }
    #[doc = "GPIO DIRCLR PIN14: Set as input pin 14."]
    #[inline]
    pub(crate) fn write_gpio_dirclr_pin14(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[14].set_direction(false);
        }
        Ok(())
    }
    #[doc = "GPIO DIRCLR PIN15: Set as input pin 15."]
    #[inline]
    pub(crate) fn read_gpio_dirclr_pin15(&self) -> MemResult<bool> {
        Ok(self.gpio[15].get_direction())
    }
    #[doc = "GPIO DIRCLR PIN15: Set as input pin 15."]
    #[inline]
    pub(crate) fn write_gpio_dirclr_pin15(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[15].set_direction(false);
        }
        Ok(())
    }
    #[doc = "GPIO DIRCLR PIN16: Set as input pin 16."]
    #[inline]
    pub(crate) fn read_gpio_dirclr_pin16(&self) -> MemResult<bool> {
        Ok(self.gpio[16].get_direction())
    }
    #[doc = "GPIO DIRCLR PIN16: Set as input pin 16."]
    #[inline]
    pub(crate) fn write_gpio_dirclr_pin16(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[16].set_direction(false);
        }
        Ok(())
    }
    #[doc = "GPIO DIRCLR PIN17: Set as input pin 17."]
    #[inline]
    pub(crate) fn read_gpio_dirclr_pin17(&self) -> MemResult<bool> {
        Ok(self.gpio[17].get_direction())
    }
    #[doc = "GPIO DIRCLR PIN17: Set as input pin 17."]
    #[inline]
    pub(crate) fn write_gpio_dirclr_pin17(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[17].set_direction(false);
        }
        Ok(())
    }
    #[doc = "GPIO DIRCLR PIN18: Set as input pin 18."]
    #[inline]
    pub(crate) fn read_gpio_dirclr_pin18(&self) -> MemResult<bool> {
        Ok(self.gpio[18].get_direction())
    }
    #[doc = "GPIO DIRCLR PIN18: Set as input pin 18."]
    #[inline]
    pub(crate) fn write_gpio_dirclr_pin18(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[18].set_direction(false);
        }
        Ok(())
    }
    #[doc = "GPIO DIRCLR PIN19: Set as input pin 19."]
    #[inline]
    pub(crate) fn read_gpio_dirclr_pin19(&self) -> MemResult<bool> {
        Ok(self.gpio[19].get_direction())
    }
    #[doc = "GPIO DIRCLR PIN19: Set as input pin 19."]
    #[inline]
    pub(crate) fn write_gpio_dirclr_pin19(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[19].set_direction(false);
        }
        Ok(())
    }
    #[doc = "GPIO DIRCLR PIN20: Set as input pin 20."]
    #[inline]
    pub(crate) fn read_gpio_dirclr_pin20(&self) -> MemResult<bool> {
        Ok(self.gpio[20].get_direction())
    }
    #[doc = "GPIO DIRCLR PIN20: Set as input pin 20."]
    #[inline]
    pub(crate) fn write_gpio_dirclr_pin20(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[20].set_direction(false);
        }
        Ok(())
    }
    #[doc = "GPIO DIRCLR PIN21: Set as input pin 21."]
    #[inline]
    pub(crate) fn read_gpio_dirclr_pin21(&self) -> MemResult<bool> {
        Ok(self.gpio[21].get_direction())
    }
    #[doc = "GPIO DIRCLR PIN21: Set as input pin 21."]
    #[inline]
    pub(crate) fn write_gpio_dirclr_pin21(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[21].set_direction(false);
        }
        Ok(())
    }
    #[doc = "GPIO DIRCLR PIN22: Set as input pin 22."]
    #[inline]
    pub(crate) fn read_gpio_dirclr_pin22(&self) -> MemResult<bool> {
        Ok(self.gpio[22].get_direction())
    }
    #[doc = "GPIO DIRCLR PIN22: Set as input pin 22."]
    #[inline]
    pub(crate) fn write_gpio_dirclr_pin22(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[22].set_direction(false);
        }
        Ok(())
    }
    #[doc = "GPIO DIRCLR PIN23: Set as input pin 23."]
    #[inline]
    pub(crate) fn read_gpio_dirclr_pin23(&self) -> MemResult<bool> {
        Ok(self.gpio[23].get_direction())
    }
    #[doc = "GPIO DIRCLR PIN23: Set as input pin 23."]
    #[inline]
    pub(crate) fn write_gpio_dirclr_pin23(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[23].set_direction(false);
        }
        Ok(())
    }
    #[doc = "GPIO DIRCLR PIN24: Set as input pin 24."]
    #[inline]
    pub(crate) fn read_gpio_dirclr_pin24(&self) -> MemResult<bool> {
        Ok(self.gpio[24].get_direction())
    }
    #[doc = "GPIO DIRCLR PIN24: Set as input pin 24."]
    #[inline]
    pub(crate) fn write_gpio_dirclr_pin24(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[24].set_direction(false);
        }
        Ok(())
    }
    #[doc = "GPIO DIRCLR PIN25: Set as input pin 25."]
    #[inline]
    pub(crate) fn read_gpio_dirclr_pin25(&self) -> MemResult<bool> {
        Ok(self.gpio[25].get_direction())
    }
    #[doc = "GPIO DIRCLR PIN25: Set as input pin 25."]
    #[inline]
    pub(crate) fn write_gpio_dirclr_pin25(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[25].set_direction(false);
        }
        Ok(())
    }
    #[doc = "GPIO DIRCLR PIN26: Set as input pin 26."]
    #[inline]
    pub(crate) fn read_gpio_dirclr_pin26(&self) -> MemResult<bool> {
        Ok(self.gpio[26].get_direction())
    }
    #[doc = "GPIO DIRCLR PIN26: Set as input pin 26."]
    #[inline]
    pub(crate) fn write_gpio_dirclr_pin26(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[26].set_direction(false);
        }
        Ok(())
    }
    #[doc = "GPIO DIRCLR PIN27: Set as input pin 27."]
    #[inline]
    pub(crate) fn read_gpio_dirclr_pin27(&self) -> MemResult<bool> {
        Ok(self.gpio[27].get_direction())
    }
    #[doc = "GPIO DIRCLR PIN27: Set as input pin 27."]
    #[inline]
    pub(crate) fn write_gpio_dirclr_pin27(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[27].set_direction(false);
        }
        Ok(())
    }
    #[doc = "GPIO DIRCLR PIN28: Set as input pin 28."]
    #[inline]
    pub(crate) fn read_gpio_dirclr_pin28(&self) -> MemResult<bool> {
        Ok(self.gpio[28].get_direction())
    }
    #[doc = "GPIO DIRCLR PIN28: Set as input pin 28."]
    #[inline]
    pub(crate) fn write_gpio_dirclr_pin28(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[28].set_direction(false);
        }
        Ok(())
    }
    #[doc = "GPIO DIRCLR PIN29: Set as input pin 29."]
    #[inline]
    pub(crate) fn read_gpio_dirclr_pin29(&self) -> MemResult<bool> {
        Ok(self.gpio[29].get_direction())
    }
    #[doc = "GPIO DIRCLR PIN29: Set as input pin 29."]
    #[inline]
    pub(crate) fn write_gpio_dirclr_pin29(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[29].set_direction(false);
        }
        Ok(())
    }
    #[doc = "GPIO DIRCLR PIN30: Set as input pin 30."]
    #[inline]
    pub(crate) fn read_gpio_dirclr_pin30(&self) -> MemResult<bool> {
        Ok(self.gpio[30].get_direction())
    }
    #[doc = "GPIO DIRCLR PIN30: Set as input pin 30."]
    #[inline]
    pub(crate) fn write_gpio_dirclr_pin30(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[30].set_direction(false);
        }
        Ok(())
    }
    #[doc = "GPIO DIRCLR PIN31: Set as input pin 31."]
    #[inline]
    pub(crate) fn read_gpio_dirclr_pin31(&self) -> MemResult<bool> {
        Ok(self.gpio[31].get_direction())
    }
    #[doc = "GPIO DIRCLR PIN31: Set as input pin 31."]
    #[inline]
    pub(crate) fn write_gpio_dirclr_pin31(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.gpio[31].set_direction(false);
        }
        Ok(())
    }
    #[doc = "GPIO PIN_CNF[%s] DIR: Pin direction."]
    #[inline]
    pub(crate) fn read_gpio_pin_cnfn_dir(
        &self,
        _dim: usize,
    ) -> MemResult<bool> {
        Ok(self.gpio[_dim].get_direction())
    }
    #[doc = "GPIO PIN_CNF[%s] DIR: Pin direction."]
    #[inline]
    pub(crate) fn write_gpio_pin_cnfn_dir(
        &mut self,
        _dim: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[_dim].set_direction(_value))
    }
    #[doc = "GPIO PIN_CNF[%s] INPUT: Connect or disconnect input path."]
    #[inline]
    pub(crate) fn read_gpio_pin_cnfn_input(
        &self,
        _dim: usize,
    ) -> MemResult<bool> {
        Ok(self.gpio[_dim].get_direction())
    }
    #[doc = "GPIO PIN_CNF[%s] INPUT: Connect or disconnect input path."]
    #[inline]
    pub(crate) fn write_gpio_pin_cnfn_input(
        &mut self,
        _dim: usize,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.gpio[_dim].set_direction(_value))
    }
    #[doc = "GPIO PIN_CNF[%s] PULL: Pull-up or -down configuration."]
    #[inline]
    pub(crate) fn read_gpio_pin_cnfn_pull(&self, _dim: usize) -> MemResult<u8> {
        Ok(self.gpio[_dim].get_pull() as u8)
    }
    #[doc = "GPIO PIN_CNF[%s] PULL: Pull-up or -down configuration."]
    #[inline]
    pub(crate) fn write_gpio_pin_cnfn_pull(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.gpio[_dim]
            .set_pull(_value.try_into().map_err(|_| MemError::WriteViolation)?))
    }
    #[doc = "GPIO PIN_CNF[%s] DRIVE: Drive configuration."]
    #[inline]
    pub(crate) fn read_gpio_pin_cnfn_drive(
        &self,
        _dim: usize,
    ) -> MemResult<u8> {
        Ok(self.gpio[_dim].get_drive() as u8)
    }
    #[doc = "GPIO PIN_CNF[%s] DRIVE: Drive configuration."]
    #[inline]
    pub(crate) fn write_gpio_pin_cnfn_drive(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.gpio[_dim].set_drive(
            _value.try_into().map_err(|_| MemError::WriteViolation)?,
        ))
    }
    #[doc = "GPIO PIN_CNF[%s] SENSE: Pin sensing mechanism."]
    #[inline]
    pub(crate) fn read_gpio_pin_cnfn_sense(
        &self,
        _dim: usize,
    ) -> MemResult<u8> {
        Ok(self.gpio[_dim].get_sense() as u8)
    }
    #[doc = "GPIO PIN_CNF[%s] SENSE: Pin sensing mechanism."]
    #[inline]
    pub(crate) fn write_gpio_pin_cnfn_sense(
        &mut self,
        _dim: usize,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.gpio[_dim].set_sense(
            _value.try_into().map_err(|_| MemError::WriteViolation)?,
        ))
    }
}
