use icicle_vm::cpu::mem::MemResult;

#[doc = "Control: System Control registers<br>ID: ID registers<br>FPE: System Control registers for the FP extension<br>SysTick: System Timer registers<br>NVIC: Nested Vectored Interrupt Controller registers<br>MPU: Memory Protection Unit<br><br>Instances:<br>0xe000e000: Control, ID, FPE, SysTick, NVIC, MPU<br>"]
pub struct Scs {
    pub interrupts_enabled: [u32; 8],
    pub interrupts_pending: [u32; 8],
    pub interrupts_priority: [u8; 240],

    /// Indicates sleep-on-exit when returning from Handler mode to Thread mode:
    /// 0 = do not sleep when returning to Thread mode.
    /// 1 = enter sleep, or deep sleep, on return from an ISR.
    /// Setting this bit to 1 enables an interrupt driven application to avoid
    /// returning to an empty main application.
    sleep_on_exit: bool,
    /// Controls whether the processor uses sleep or deep sleep as its low power
    /// mode: 0 = sleep, 1 = deep sleep.
    sleep_deep: bool,
    /// Send Event on Pending bit:
    /// 0 = only enabled interrupts or events can wakeup the processor, disabled
    /// interrupts are excluded
    ///
    /// 1 = enabled events and all interrupts, including disabled interrupts,
    /// can wakeup the processor.
    ///
    /// When an event or interrupt enters pending state, the event signal wakes up the processor
    /// from WFE. If the processor is not waiting for an event, the event is registered and affects the
    /// next WFE.
    /// The processor also wakes up on execution of an SEV instruction or an external event
    event_on_pending: bool,
    /// 4.3.8 System Handler Priority Registers
    /// The SHPR1-SHPR3 registers set the priority level, 0 to 255, of the
    /// exception handlers that have configurable priority.
    priorities: [u8; 12],
}
impl Default for Scs {
    fn default() -> Self {
        Self {
            interrupts_enabled: [0; 8],
            interrupts_pending: [0; 8],
            interrupts_priority: [0; 240],

            sleep_on_exit: false,
            sleep_deep: false,
            event_on_pending: false,
            priorities: [0; 12],
        }
    }
}

#[derive(Clone, Copy)]
#[repr(usize)]
pub enum SysHandlerPriority {
    /// handler 4, MemManage
    Pri4 = 0,
    /// handler 5, BusFault
    Pri5 = 1,
    /// handler 6, UsageFault
    Pri6 = 2,
    /// handler 11, SVCall
    Pri11 = 3,
    /// handler 14, PendSV
    Pri14 = 4,
    /// handler 15, SysTick exception
    Pri15 = 5,
    /// Reserved
    Pri7,
    /// Reserved
    Pri8,
    /// Reserved
    Pri9,
    /// Reserved
    Pri10,
    /// Reserved
    Pri12,
    /// Reserved
    Pri13,
}

impl Scs {
    pub(crate) fn page_to_index(page: u64) -> usize {
        match page {
            917518u64 => 0usize,
            _ => unreachable!(),
        }
    }
    #[doc = "LSPACT: Indicates whether Lazy preservation of the FP state is active<br>ENABLE: Indicates the enabled status of the SysTick counter<br>DISMCYCINT: Disables interruption of multi-cycle instructions<br>SEPARATE: Indicates support for separate instruction and data address maps<br>"]
    pub(crate) fn scs_fpccr0_lspact_read(&self) -> MemResult<bool> {
        todo ! ("read LSPACT, ENABLE, DISMCYCINT, SEPARATE mwrite None write None rac None reset value false")
    }
    #[doc = "LSPACT: Indicates whether Lazy preservation of the FP state is active<br>ENABLE: Indicates the enabled status of the SysTick counter<br>DISMCYCINT: Disables interruption of multi-cycle instructions<br>SEPARATE: Indicates support for separate instruction and data address maps<br>"]
    pub(crate) fn scs_fpccr0_lspact_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write LSPACT, ENABLE, DISMCYCINT, SEPARATE mwrite None write None rac None reset value false")
    }
    #[doc = "USER: Indicates the privilege level of the software executing when the processor allocated the FP stack frame<br>TICKINT: Indicates whether counting to 0 causes the status of the SysTick exception to change to pending<br>DISDEFWBUF: Disables write buffer use during default memory map accesses<br>"]
    pub(crate) fn scs_fpccr0_user_read(&self) -> MemResult<bool> {
        todo ! ("read USER, TICKINT, DISDEFWBUF mwrite None write None rac None reset value false")
    }
    #[doc = "USER: Indicates the privilege level of the software executing when the processor allocated the FP stack frame<br>TICKINT: Indicates whether counting to 0 causes the status of the SysTick exception to change to pending<br>DISDEFWBUF: Disables write buffer use during default memory map accesses<br>"]
    pub(crate) fn scs_fpccr0_user_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write USER, TICKINT, DISDEFWBUF mwrite None write None rac None reset value false")
    }
    #[doc = "CLKSOURCE: Indicates the SysTick clock source<br>DISFOLD: Disables folding of IT instructions<br>"]
    pub(crate) fn scs_fpccr0_clksource_read(&self) -> MemResult<bool> {
        todo ! ("read CLKSOURCE, DISFOLD mwrite None write None rac None reset value false")
    }
    #[doc = "CLKSOURCE: Indicates the SysTick clock source<br>DISFOLD: Disables folding of IT instructions<br>"]
    pub(crate) fn scs_fpccr0_clksource_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write CLKSOURCE, DISFOLD mwrite None write None rac None reset value false")
    }
    #[doc = "THREAD: Indicates the processor mode when it allocated the FP stack frame<br>"]
    pub(crate) fn scs_fpccr0_thread_read(&self) -> MemResult<bool> {
        todo!("read THREAD mwrite None write None rac None reset value false")
    }
    #[doc = "THREAD: Indicates the processor mode when it allocated the FP stack frame<br>"]
    pub(crate) fn scs_fpccr0_thread_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write THREAD mwrite None write None rac None reset value false")
    }
    #[doc = "HFRDY: Indicates whether the software executing when the processor allocated the FP stack frame was able to set the HardFault exception to pending<br>"]
    pub(crate) fn scs_fpccr0_hfrdy_read(&self) -> MemResult<bool> {
        todo!("read HFRDY mwrite None write None rac None reset value false")
    }
    #[doc = "HFRDY: Indicates whether the software executing when the processor allocated the FP stack frame was able to set the HardFault exception to pending<br>"]
    pub(crate) fn scs_fpccr0_hfrdy_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write HFRDY mwrite None write None rac None reset value false")
    }
    #[doc = "MMRDY: Indicates whether the software executing when the processor allocated the FP stack frame was able to set the MemManage exception to pending<br>"]
    pub(crate) fn scs_fpccr0_mmrdy_read(&self) -> MemResult<bool> {
        todo!("read MMRDY mwrite None write None rac None reset value false")
    }
    #[doc = "MMRDY: Indicates whether the software executing when the processor allocated the FP stack frame was able to set the MemManage exception to pending<br>"]
    pub(crate) fn scs_fpccr0_mmrdy_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MMRDY mwrite None write None rac None reset value false")
    }
    #[doc = "BFRDY: Indicates whether the software executing when the processor allocated the FP stack frame was able to set the BusFault exception to pending<br>"]
    pub(crate) fn scs_fpccr0_bfrdy_read(&self) -> MemResult<bool> {
        todo!("read BFRDY mwrite None write None rac None reset value false")
    }
    #[doc = "BFRDY: Indicates whether the software executing when the processor allocated the FP stack frame was able to set the BusFault exception to pending<br>"]
    pub(crate) fn scs_fpccr0_bfrdy_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write BFRDY mwrite None write None rac None reset value false")
    }
    #[doc = "MONRDY: Indicates whether the software executing when the processor allocated the FP stack frame was able to set the DebugMonitor exception to pending<br>DISFPCA: Disable automatic update of CONTROL.FPCA<br>"]
    pub(crate) fn scs_fpccr0_monrdy_read(&self) -> MemResult<bool> {
        todo ! ("read MONRDY, DISFPCA mwrite None write None rac None reset value false")
    }
    #[doc = "MONRDY: Indicates whether the software executing when the processor allocated the FP stack frame was able to set the DebugMonitor exception to pending<br>DISFPCA: Disable automatic update of CONTROL.FPCA<br>"]
    pub(crate) fn scs_fpccr0_monrdy_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write MONRDY, DISFPCA mwrite None write None rac None reset value false")
    }
    #[doc = "DISOOFP: Disables floating point instructions completing out of order with respect to integer instructions<br>"]
    pub(crate) fn scs_fpccr0_disoofp_read(&self) -> MemResult<bool> {
        todo!("read DISOOFP mwrite None write None rac None reset value false")
    }
    #[doc = "DISOOFP: Disables floating point instructions completing out of order with respect to integer instructions<br>"]
    pub(crate) fn scs_fpccr0_disoofp_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write DISOOFP mwrite None write None rac None reset value false")
    }
    #[doc = "COUNTFLAG: Indicates whether the counter has counted to 0 since the last read of this register<br>"]
    pub(crate) fn scs_fpccr0_countflag_read(&self) -> MemResult<bool> {
        todo!(
            "read COUNTFLAG mwrite None write None rac None reset value false"
        )
    }
    #[doc = "COUNTFLAG: Indicates whether the counter has counted to 0 since the last read of this register<br>"]
    pub(crate) fn scs_fpccr0_countflag_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write COUNTFLAG mwrite None write None rac None reset value false"
        )
    }
    #[doc = "Variant: Implementation defined<br>"]
    pub(crate) fn scs_fpccr0_variant_read(&self) -> MemResult<u8> {
        todo ! ("read Variant mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "Variant: Implementation defined<br>"]
    pub(crate) fn scs_fpccr0_variant_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write Variant mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "Implementer: Implementer code<br>"]
    pub(crate) fn scs_fpccr0_implementer_read(&self) -> MemResult<u8> {
        todo ! ("read Implementer mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "Implementer: Implementer code<br>"]
    pub(crate) fn scs_fpccr0_implementer_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write Implementer mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "LSPEN: Enables lazy context save of FP state<br>"]
    pub(crate) fn scs_fpccr0_lspen_read(&self) -> MemResult<bool> {
        todo!("read LSPEN mwrite None write None rac None reset value false")
    }
    #[doc = "LSPEN: Enables lazy context save of FP state<br>"]
    pub(crate) fn scs_fpccr0_lspen_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write LSPEN mwrite None write None rac None reset value false")
    }
    #[doc = "ASPEN: When this bit is set to 1, execution of a floating-point instruction sets the CONTROL.FPCA bit to 1<br>"]
    pub(crate) fn scs_fpccr0_aspen_read(&self) -> MemResult<bool> {
        todo!("read ASPEN mwrite None write None rac None reset value false")
    }
    #[doc = "ASPEN: When this bit is set to 1, execution of a floating-point instruction sets the CONTROL.FPCA bit to 1<br>"]
    pub(crate) fn scs_fpccr0_aspen_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ASPEN mwrite None write None rac None reset value false")
    }
    #[doc = "ENABLE: Enables the MPU<br>"]
    pub(crate) fn scs_mpu_ctrl4_enable_read(&self) -> MemResult<bool> {
        todo!("read ENABLE mwrite None write None rac None reset value false")
    }
    #[doc = "ENABLE: Enables the MPU<br>"]
    pub(crate) fn scs_mpu_ctrl4_enable_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ENABLE mwrite None write None rac None reset value false")
    }
    #[doc = "HFNMIENA: When the ENABLE bit is set to 1, controls whether handlers executing with priority less than 0 access memory with the MPU enabled or with the MPU disabled<br>"]
    pub(crate) fn scs_mpu_ctrl4_hfnmiena_read(&self) -> MemResult<bool> {
        todo!("read HFNMIENA mwrite None write None rac None reset value false")
    }
    #[doc = "HFNMIENA: When the ENABLE bit is set to 1, controls whether handlers executing with priority less than 0 access memory with the MPU enabled or with the MPU disabled<br>"]
    pub(crate) fn scs_mpu_ctrl4_hfnmiena_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write HFNMIENA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PRIVDEFENA: When the ENABLE bit is set to 1, Enables the default memory map as a background region for privileged access<br>"]
    pub(crate) fn scs_mpu_ctrl4_privdefena_read(&self) -> MemResult<bool> {
        todo!(
            "read PRIVDEFENA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PRIVDEFENA: When the ENABLE bit is set to 1, Enables the default memory map as a background region for privileged access<br>"]
    pub(crate) fn scs_mpu_ctrl4_privdefena_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write PRIVDEFENA mwrite None write None rac None reset value false")
    }
    #[doc = "REGION: Indicates the memory region accessed by MPU_RBAR and MPU_RSAR<br>"]
    pub(crate) fn scs_mpu_rnr8_region_read(&self) -> MemResult<u8> {
        todo ! ("read REGION mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "REGION: Indicates the memory region accessed by MPU_RBAR and MPU_RSAR<br>"]
    pub(crate) fn scs_mpu_rnr8_region_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write REGION mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "RMode: Default value for FPSCR.RMode<br>"]
    pub(crate) fn scs_mpu_rnr8_rmode_read(&self) -> MemResult<u8> {
        todo ! ("read RMode mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "RMode: Default value for FPSCR.RMode<br>"]
    pub(crate) fn scs_mpu_rnr8_rmode_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write RMode mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "FZ: Default value for FPSCR.FZ<br>"]
    pub(crate) fn scs_mpu_rnr8_fz_read(&self) -> MemResult<bool> {
        todo!("read FZ mwrite None write None rac None reset value false")
    }
    #[doc = "FZ: Default value for FPSCR.FZ<br>"]
    pub(crate) fn scs_mpu_rnr8_fz_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write FZ mwrite None write None rac None reset value false")
    }
    #[doc = "DN: Default value for FPSCR.DN<br>"]
    pub(crate) fn scs_mpu_rnr8_dn_read(&self) -> MemResult<bool> {
        todo!("read DN mwrite None write None rac None reset value false")
    }
    #[doc = "DN: Default value for FPSCR.DN<br>"]
    pub(crate) fn scs_mpu_rnr8_dn_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write DN mwrite None write None rac None reset value false")
    }
    #[doc = "AHP: Default value for FPSCR.AHP<br>"]
    pub(crate) fn scs_mpu_rnr8_ahp_read(&self) -> MemResult<bool> {
        todo!("read AHP mwrite None write None rac None reset value false")
    }
    #[doc = "AHP: Default value for FPSCR.AHP<br>"]
    pub(crate) fn scs_mpu_rnr8_ahp_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write AHP mwrite None write None rac None reset value false")
    }
    #[doc = "REGION: On writes, can specify the number of the region to update. On reads, returns bits \\[3:0\\] of MPU_RNR<br>A_SIMD: Indicates the size of the FP register bank<br>"]
    pub(crate) fn scs_mpu_rbarc_region_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E68ScsMpuRbarcRegion> {
        todo ! ("read REGION, A_SIMD mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "REGION: On writes, can specify the number of the region to update. On reads, returns bits \\[3:0\\] of MPU_RNR<br>A_SIMD: Indicates the size of the FP register bank<br>"]
    pub(crate) fn scs_mpu_rbarc_region_write(
        &mut self,
        _value: crate::peripheral::enums::E68ScsMpuRbarcRegion,
    ) -> MemResult<()> {
        todo ! ("write REGION, A_SIMD mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "VALID: On writes, indicates whether the region to update is specified by MPU_RNR.REGION, or by the REGION value specified in this write. When using the REGION value specified by this write, MPU_RNR.REGION is updated to this value<br>"]
    pub(crate) fn scs_mpu_rbarc_valid_read(&self) -> MemResult<bool> {
        todo!("read VALID mwrite None write None rac None reset value false")
    }
    #[doc = "VALID: On writes, indicates whether the region to update is specified by MPU_RNR.REGION, or by the REGION value specified in this write. When using the REGION value specified by this write, MPU_RNR.REGION is updated to this value<br>"]
    pub(crate) fn scs_mpu_rbarc_valid_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write VALID mwrite None write None rac None reset value false")
    }
    #[doc = "ADDR: Base address of the region<br>"]
    pub(crate) fn scs_mpu_rbarc_addr_read(&self) -> MemResult<u32> {
        todo ! ("read ADDR mwrite None write None rac None reset value 0x00 mask 0x7ffffff")
    }
    #[doc = "ADDR: Base address of the region<br>"]
    pub(crate) fn scs_mpu_rbarc_addr_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write ADDR mwrite None write None rac None reset value 0x00 mask 0x7ffffff")
    }
    #[doc = "Double_precision: Indicates the hardware support for FP double-precision operations<br>"]
    pub(crate) fn scs_mpu_rbarc_double_precision_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E69ScsMpuRbarcDoublePrecision>
    {
        todo ! ("read Double_precision mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "Double_precision: Indicates the hardware support for FP double-precision operations<br>"]
    pub(crate) fn scs_mpu_rbarc_double_precision_write(
        &mut self,
        _value: crate::peripheral::enums::E69ScsMpuRbarcDoublePrecision,
    ) -> MemResult<()> {
        todo ! ("write Double_precision mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "FP_exception_trapping: Indicates whether the FP hardware implementation supports exception trapping<br>"]
    pub(crate) fn scs_mpu_rbarc_fp_exception_trapping_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E69ScsMpuRbarcDoublePrecision>
    {
        todo ! ("read FP_exception_trapping mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "FP_exception_trapping: Indicates whether the FP hardware implementation supports exception trapping<br>"]
    pub(crate) fn scs_mpu_rbarc_fp_exception_trapping_write(
        &mut self,
        _value: crate::peripheral::enums::E69ScsMpuRbarcDoublePrecision,
    ) -> MemResult<()> {
        todo ! ("write FP_exception_trapping mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "Divide: Indicates the hardware support for FP divide operations<br>"]
    pub(crate) fn scs_mpu_rbarc_divide_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E70ScsMpuRbarcDivide> {
        todo ! ("read Divide mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "Divide: Indicates the hardware support for FP divide operations<br>"]
    pub(crate) fn scs_mpu_rbarc_divide_write(
        &mut self,
        _value: crate::peripheral::enums::E70ScsMpuRbarcDivide,
    ) -> MemResult<()> {
        todo ! ("write Divide mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "Square_root: Indicates the hardware support for FP square root operations<br>"]
    pub(crate) fn scs_mpu_rbarc_square_root_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E70ScsMpuRbarcDivide> {
        todo ! ("read Square_root mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "Square_root: Indicates the hardware support for FP square root operations<br>"]
    pub(crate) fn scs_mpu_rbarc_square_root_write(
        &mut self,
        _value: crate::peripheral::enums::E70ScsMpuRbarcDivide,
    ) -> MemResult<()> {
        todo ! ("write Square_root mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "Short_vectors: Indicates the hardware support for FP short vectors<br>"]
    pub(crate) fn scs_mpu_rbarc_short_vectors_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E69ScsMpuRbarcDoublePrecision>
    {
        todo ! ("read Short_vectors mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "Short_vectors: Indicates the hardware support for FP short vectors<br>"]
    pub(crate) fn scs_mpu_rbarc_short_vectors_write(
        &mut self,
        _value: crate::peripheral::enums::E69ScsMpuRbarcDoublePrecision,
    ) -> MemResult<()> {
        todo ! ("write Short_vectors mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "FP_Rounding_modes: Indicates the rounding modes supported by the FP floating-point hardware<br>"]
    pub(crate) fn scs_mpu_rbarc_fp_rounding_modes_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E70ScsMpuRbarcDivide> {
        todo ! ("read FP_Rounding_modes mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "FP_Rounding_modes: Indicates the rounding modes supported by the FP floating-point hardware<br>"]
    pub(crate) fn scs_mpu_rbarc_fp_rounding_modes_write(
        &mut self,
        _value: crate::peripheral::enums::E70ScsMpuRbarcDivide,
    ) -> MemResult<()> {
        todo ! ("write FP_Rounding_modes mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "SKEW: Indicates whether the 10ms calibration value is exact<br>"]
    pub(crate) fn scs_mpu_rbarc_skew_read(&self) -> MemResult<bool> {
        todo!("read SKEW mwrite None write None rac None reset value false")
    }
    #[doc = "SKEW: Indicates whether the 10ms calibration value is exact<br>"]
    pub(crate) fn scs_mpu_rbarc_skew_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SKEW mwrite None write None rac None reset value false")
    }
    #[doc = "NOREF: Indicates whether the IMPLEMENTATION DEFINED reference clock is provided<br>"]
    pub(crate) fn scs_mpu_rbarc_noref_read(&self) -> MemResult<bool> {
        todo!("read NOREF mwrite None write None rac None reset value false")
    }
    #[doc = "NOREF: Indicates whether the IMPLEMENTATION DEFINED reference clock is provided<br>"]
    pub(crate) fn scs_mpu_rbarc_noref_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write NOREF mwrite None write None rac None reset value false")
    }
    #[doc = "ENABLE: Enables this region<br>"]
    pub(crate) fn scs_mpu_rasr10_enable_read(&self) -> MemResult<bool> {
        todo!("read ENABLE mwrite None write None rac None reset value false")
    }
    #[doc = "ENABLE: Enables this region<br>"]
    pub(crate) fn scs_mpu_rasr10_enable_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ENABLE mwrite None write None rac None reset value false")
    }
    #[doc = "SIZE: Indicates the region size<br>"]
    pub(crate) fn scs_mpu_rasr10_size_read(&self) -> MemResult<u8> {
        todo ! ("read SIZE mwrite None write None rac None reset value 0x00 mask 0x1f")
    }
    #[doc = "SIZE: Indicates the region size<br>"]
    pub(crate) fn scs_mpu_rasr10_size_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write SIZE mwrite None write None rac None reset value 0x00 mask 0x1f")
    }
    #[doc = "D_NaN: Indicates whether the FP hardware implementation supports only the Default NaN mode<br>"]
    pub(crate) fn scs_mpu_rasr10_d_nan_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E70ScsMpuRbarcDivide> {
        todo ! ("read D_NaN mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "D_NaN: Indicates whether the FP hardware implementation supports only the Default NaN mode<br>"]
    pub(crate) fn scs_mpu_rasr10_d_nan_write(
        &mut self,
        _value: crate::peripheral::enums::E70ScsMpuRbarcDivide,
    ) -> MemResult<()> {
        todo ! ("write D_NaN mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "SRD: Subregion Disable<br>"]
    pub(crate) fn scs_mpu_rasr10_srd_read(&self) -> MemResult<u8> {
        todo ! ("read SRD mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "SRD: Subregion Disable<br>"]
    pub(crate) fn scs_mpu_rasr10_srd_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write SRD mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "B: MPU Region Attribute field<br>"]
    pub(crate) fn scs_mpu_rasr10_b_read(&self) -> MemResult<bool> {
        todo!("read B mwrite None write None rac None reset value false")
    }
    #[doc = "B: MPU Region Attribute field<br>"]
    pub(crate) fn scs_mpu_rasr10_b_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write B mwrite None write None rac None reset value false")
    }
    #[doc = "C: MPU Region Attribute field<br>"]
    pub(crate) fn scs_mpu_rasr10_c_read(&self) -> MemResult<bool> {
        todo!("read C mwrite None write None rac None reset value false")
    }
    #[doc = "C: MPU Region Attribute field<br>"]
    pub(crate) fn scs_mpu_rasr10_c_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write C mwrite None write None rac None reset value false")
    }
    #[doc = "S: MPU Region Attribute field: Sharable<br>"]
    pub(crate) fn scs_mpu_rasr10_s_read(&self) -> MemResult<bool> {
        todo!("read S mwrite None write None rac None reset value false")
    }
    #[doc = "S: MPU Region Attribute field: Sharable<br>"]
    pub(crate) fn scs_mpu_rasr10_s_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write S mwrite None write None rac None reset value false")
    }
    #[doc = "TEX: MPU Region Attribute field<br>"]
    pub(crate) fn scs_mpu_rasr10_tex_read(&self) -> MemResult<u8> {
        todo ! ("read TEX mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "TEX: MPU Region Attribute field<br>"]
    pub(crate) fn scs_mpu_rasr10_tex_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write TEX mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "AP: Access permissions<br>"]
    pub(crate) fn scs_mpu_rasr10_ap_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E71ScsMpuRasr10Ap> {
        todo ! ("read AP mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "AP: Access permissions<br>"]
    pub(crate) fn scs_mpu_rasr10_ap_write(
        &mut self,
        _value: crate::peripheral::enums::E71ScsMpuRasr10Ap,
    ) -> MemResult<()> {
        todo ! ("write AP mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "XN: Execute Never<br>"]
    pub(crate) fn scs_mpu_rasr10_xn_read(&self) -> MemResult<bool> {
        todo!("read XN mwrite None write None rac None reset value false")
    }
    #[doc = "XN: Execute Never<br>"]
    pub(crate) fn scs_mpu_rasr10_xn_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write XN mwrite None write None rac None reset value false")
    }
    #[doc = "REGION: On writes, can specify the number of the region to update. On reads, returns bits \\[3:0\\] of MPU_RNR<br>"]
    pub(crate) fn scs_mpu_rbar_a114_region_read(&self) -> MemResult<u8> {
        todo ! ("read REGION mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "REGION: On writes, can specify the number of the region to update. On reads, returns bits \\[3:0\\] of MPU_RNR<br>"]
    pub(crate) fn scs_mpu_rbar_a114_region_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write REGION mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "VALID: On writes, indicates whether the region to update is specified by MPU_RNR.REGION, or by the REGION value specified in this write. When using the REGION value specified by this write, MPU_RNR.REGION is updated to this value<br>"]
    pub(crate) fn scs_mpu_rbar_a114_valid_read(&self) -> MemResult<bool> {
        todo!("read VALID mwrite None write None rac None reset value false")
    }
    #[doc = "VALID: On writes, indicates whether the region to update is specified by MPU_RNR.REGION, or by the REGION value specified in this write. When using the REGION value specified by this write, MPU_RNR.REGION is updated to this value<br>"]
    pub(crate) fn scs_mpu_rbar_a114_valid_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write VALID mwrite None write None rac None reset value false")
    }
    #[doc = "ADDR: Base address of the region<br>"]
    pub(crate) fn scs_mpu_rbar_a114_addr_read(&self) -> MemResult<u32> {
        todo ! ("read ADDR mwrite None write None rac None reset value 0x00 mask 0x7ffffff")
    }
    #[doc = "ADDR: Base address of the region<br>"]
    pub(crate) fn scs_mpu_rbar_a114_addr_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write ADDR mwrite None write None rac None reset value 0x00 mask 0x7ffffff")
    }
    #[doc = "ENABLE: Enables this region<br>"]
    pub(crate) fn scs_mpu_rasr_a118_enable_read(&self) -> MemResult<bool> {
        todo!("read ENABLE mwrite None write None rac None reset value false")
    }
    #[doc = "ENABLE: Enables this region<br>"]
    pub(crate) fn scs_mpu_rasr_a118_enable_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ENABLE mwrite None write None rac None reset value false")
    }
    #[doc = "SIZE: Indicates the region size<br>"]
    pub(crate) fn scs_mpu_rasr_a118_size_read(&self) -> MemResult<u8> {
        todo ! ("read SIZE mwrite None write None rac None reset value 0x00 mask 0x1f")
    }
    #[doc = "SIZE: Indicates the region size<br>"]
    pub(crate) fn scs_mpu_rasr_a118_size_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write SIZE mwrite None write None rac None reset value 0x00 mask 0x1f")
    }
    #[doc = "SRD: Subregion Disable<br>"]
    pub(crate) fn scs_mpu_rasr_a118_srd_read(&self) -> MemResult<u8> {
        todo ! ("read SRD mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "SRD: Subregion Disable<br>"]
    pub(crate) fn scs_mpu_rasr_a118_srd_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write SRD mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "B: MPU Region Attribute field<br>"]
    pub(crate) fn scs_mpu_rasr_a118_b_read(&self) -> MemResult<bool> {
        todo!("read B mwrite None write None rac None reset value false")
    }
    #[doc = "B: MPU Region Attribute field<br>"]
    pub(crate) fn scs_mpu_rasr_a118_b_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write B mwrite None write None rac None reset value false")
    }
    #[doc = "C: MPU Region Attribute field<br>"]
    pub(crate) fn scs_mpu_rasr_a118_c_read(&self) -> MemResult<bool> {
        todo!("read C mwrite None write None rac None reset value false")
    }
    #[doc = "C: MPU Region Attribute field<br>"]
    pub(crate) fn scs_mpu_rasr_a118_c_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write C mwrite None write None rac None reset value false")
    }
    #[doc = "S: MPU Region Attribute field: Sharable<br>"]
    pub(crate) fn scs_mpu_rasr_a118_s_read(&self) -> MemResult<bool> {
        todo!("read S mwrite None write None rac None reset value false")
    }
    #[doc = "S: MPU Region Attribute field: Sharable<br>"]
    pub(crate) fn scs_mpu_rasr_a118_s_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write S mwrite None write None rac None reset value false")
    }
    #[doc = "TEX: MPU Region Attribute field<br>"]
    pub(crate) fn scs_mpu_rasr_a118_tex_read(&self) -> MemResult<u8> {
        todo ! ("read TEX mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "TEX: MPU Region Attribute field<br>"]
    pub(crate) fn scs_mpu_rasr_a118_tex_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write TEX mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "AP: Access permissions<br>"]
    pub(crate) fn scs_mpu_rasr_a118_ap_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E71ScsMpuRasr10Ap> {
        todo ! ("read AP mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "AP: Access permissions<br>"]
    pub(crate) fn scs_mpu_rasr_a118_ap_write(
        &mut self,
        _value: crate::peripheral::enums::E71ScsMpuRasr10Ap,
    ) -> MemResult<()> {
        todo ! ("write AP mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "XN: Execute Never<br>"]
    pub(crate) fn scs_mpu_rasr_a118_xn_read(&self) -> MemResult<bool> {
        todo!("read XN mwrite None write None rac None reset value false")
    }
    #[doc = "XN: Execute Never<br>"]
    pub(crate) fn scs_mpu_rasr_a118_xn_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write XN mwrite None write None rac None reset value false")
    }
    #[doc = "REGION: On writes, can specify the number of the region to update. On reads, returns bits \\[3:0\\] of MPU_RNR<br>"]
    pub(crate) fn scs_mpu_rbar_a21c_region_read(&self) -> MemResult<u8> {
        todo ! ("read REGION mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "REGION: On writes, can specify the number of the region to update. On reads, returns bits \\[3:0\\] of MPU_RNR<br>"]
    pub(crate) fn scs_mpu_rbar_a21c_region_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write REGION mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "VALID: On writes, indicates whether the region to update is specified by MPU_RNR.REGION, or by the REGION value specified in this write. When using the REGION value specified by this write, MPU_RNR.REGION is updated to this value<br>"]
    pub(crate) fn scs_mpu_rbar_a21c_valid_read(&self) -> MemResult<bool> {
        todo!("read VALID mwrite None write None rac None reset value false")
    }
    #[doc = "VALID: On writes, indicates whether the region to update is specified by MPU_RNR.REGION, or by the REGION value specified in this write. When using the REGION value specified by this write, MPU_RNR.REGION is updated to this value<br>"]
    pub(crate) fn scs_mpu_rbar_a21c_valid_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write VALID mwrite None write None rac None reset value false")
    }
    #[doc = "ADDR: Base address of the region<br>"]
    pub(crate) fn scs_mpu_rbar_a21c_addr_read(&self) -> MemResult<u32> {
        todo ! ("read ADDR mwrite None write None rac None reset value 0x00 mask 0x7ffffff")
    }
    #[doc = "ADDR: Base address of the region<br>"]
    pub(crate) fn scs_mpu_rbar_a21c_addr_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write ADDR mwrite None write None rac None reset value 0x00 mask 0x7ffffff")
    }
    #[doc = "ENABLE: Enables this region<br>"]
    pub(crate) fn scs_mpu_rasr_a220_enable_read(&self) -> MemResult<bool> {
        todo!("read ENABLE mwrite None write None rac None reset value false")
    }
    #[doc = "ENABLE: Enables this region<br>"]
    pub(crate) fn scs_mpu_rasr_a220_enable_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ENABLE mwrite None write None rac None reset value false")
    }
    #[doc = "SIZE: Indicates the region size<br>"]
    pub(crate) fn scs_mpu_rasr_a220_size_read(&self) -> MemResult<u8> {
        todo ! ("read SIZE mwrite None write None rac None reset value 0x00 mask 0x1f")
    }
    #[doc = "SIZE: Indicates the region size<br>"]
    pub(crate) fn scs_mpu_rasr_a220_size_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write SIZE mwrite None write None rac None reset value 0x00 mask 0x1f")
    }
    #[doc = "SRD: Subregion Disable<br>"]
    pub(crate) fn scs_mpu_rasr_a220_srd_read(&self) -> MemResult<u8> {
        todo ! ("read SRD mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "SRD: Subregion Disable<br>"]
    pub(crate) fn scs_mpu_rasr_a220_srd_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write SRD mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "B: MPU Region Attribute field<br>"]
    pub(crate) fn scs_mpu_rasr_a220_b_read(&self) -> MemResult<bool> {
        todo!("read B mwrite None write None rac None reset value false")
    }
    #[doc = "B: MPU Region Attribute field<br>"]
    pub(crate) fn scs_mpu_rasr_a220_b_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write B mwrite None write None rac None reset value false")
    }
    #[doc = "C: MPU Region Attribute field<br>"]
    pub(crate) fn scs_mpu_rasr_a220_c_read(&self) -> MemResult<bool> {
        todo!("read C mwrite None write None rac None reset value false")
    }
    #[doc = "C: MPU Region Attribute field<br>"]
    pub(crate) fn scs_mpu_rasr_a220_c_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write C mwrite None write None rac None reset value false")
    }
    #[doc = "S: MPU Region Attribute field: Sharable<br>"]
    pub(crate) fn scs_mpu_rasr_a220_s_read(&self) -> MemResult<bool> {
        todo!("read S mwrite None write None rac None reset value false")
    }
    #[doc = "S: MPU Region Attribute field: Sharable<br>"]
    pub(crate) fn scs_mpu_rasr_a220_s_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write S mwrite None write None rac None reset value false")
    }
    #[doc = "TEX: MPU Region Attribute field<br>"]
    pub(crate) fn scs_mpu_rasr_a220_tex_read(&self) -> MemResult<u8> {
        todo ! ("read TEX mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "TEX: MPU Region Attribute field<br>"]
    pub(crate) fn scs_mpu_rasr_a220_tex_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write TEX mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "AP: Access permissions<br>"]
    pub(crate) fn scs_mpu_rasr_a220_ap_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E71ScsMpuRasr10Ap> {
        todo ! ("read AP mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "AP: Access permissions<br>"]
    pub(crate) fn scs_mpu_rasr_a220_ap_write(
        &mut self,
        _value: crate::peripheral::enums::E71ScsMpuRasr10Ap,
    ) -> MemResult<()> {
        todo ! ("write AP mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "XN: Execute Never<br>"]
    pub(crate) fn scs_mpu_rasr_a220_xn_read(&self) -> MemResult<bool> {
        todo!("read XN mwrite None write None rac None reset value false")
    }
    #[doc = "XN: Execute Never<br>"]
    pub(crate) fn scs_mpu_rasr_a220_xn_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write XN mwrite None write None rac None reset value false")
    }
    #[doc = "REGION: On writes, can specify the number of the region to update. On reads, returns bits \\[3:0\\] of MPU_RNR<br>"]
    pub(crate) fn scs_mpu_rbar_a324_region_read(&self) -> MemResult<u8> {
        todo ! ("read REGION mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "REGION: On writes, can specify the number of the region to update. On reads, returns bits \\[3:0\\] of MPU_RNR<br>"]
    pub(crate) fn scs_mpu_rbar_a324_region_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write REGION mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "VALID: On writes, indicates whether the region to update is specified by MPU_RNR.REGION, or by the REGION value specified in this write. When using the REGION value specified by this write, MPU_RNR.REGION is updated to this value<br>"]
    pub(crate) fn scs_mpu_rbar_a324_valid_read(&self) -> MemResult<bool> {
        todo!("read VALID mwrite None write None rac None reset value false")
    }
    #[doc = "VALID: On writes, indicates whether the region to update is specified by MPU_RNR.REGION, or by the REGION value specified in this write. When using the REGION value specified by this write, MPU_RNR.REGION is updated to this value<br>"]
    pub(crate) fn scs_mpu_rbar_a324_valid_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write VALID mwrite None write None rac None reset value false")
    }
    #[doc = "ADDR: Base address of the region<br>"]
    pub(crate) fn scs_mpu_rbar_a324_addr_read(&self) -> MemResult<u32> {
        todo ! ("read ADDR mwrite None write None rac None reset value 0x00 mask 0x7ffffff")
    }
    #[doc = "ADDR: Base address of the region<br>"]
    pub(crate) fn scs_mpu_rbar_a324_addr_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write ADDR mwrite None write None rac None reset value 0x00 mask 0x7ffffff")
    }
    #[doc = "ENABLE: Enables this region<br>"]
    pub(crate) fn scs_mpu_rasr_a328_enable_read(&self) -> MemResult<bool> {
        todo!("read ENABLE mwrite None write None rac None reset value false")
    }
    #[doc = "ENABLE: Enables this region<br>"]
    pub(crate) fn scs_mpu_rasr_a328_enable_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ENABLE mwrite None write None rac None reset value false")
    }
    #[doc = "SIZE: Indicates the region size<br>"]
    pub(crate) fn scs_mpu_rasr_a328_size_read(&self) -> MemResult<u8> {
        todo ! ("read SIZE mwrite None write None rac None reset value 0x00 mask 0x1f")
    }
    #[doc = "SIZE: Indicates the region size<br>"]
    pub(crate) fn scs_mpu_rasr_a328_size_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write SIZE mwrite None write None rac None reset value 0x00 mask 0x1f")
    }
    #[doc = "SRD: Subregion Disable<br>"]
    pub(crate) fn scs_mpu_rasr_a328_srd_read(&self) -> MemResult<u8> {
        todo ! ("read SRD mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "SRD: Subregion Disable<br>"]
    pub(crate) fn scs_mpu_rasr_a328_srd_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write SRD mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "B: MPU Region Attribute field<br>"]
    pub(crate) fn scs_mpu_rasr_a328_b_read(&self) -> MemResult<bool> {
        todo!("read B mwrite None write None rac None reset value false")
    }
    #[doc = "B: MPU Region Attribute field<br>"]
    pub(crate) fn scs_mpu_rasr_a328_b_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write B mwrite None write None rac None reset value false")
    }
    #[doc = "C: MPU Region Attribute field<br>"]
    pub(crate) fn scs_mpu_rasr_a328_c_read(&self) -> MemResult<bool> {
        todo!("read C mwrite None write None rac None reset value false")
    }
    #[doc = "C: MPU Region Attribute field<br>"]
    pub(crate) fn scs_mpu_rasr_a328_c_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write C mwrite None write None rac None reset value false")
    }
    #[doc = "S: MPU Region Attribute field: Sharable<br>"]
    pub(crate) fn scs_mpu_rasr_a328_s_read(&self) -> MemResult<bool> {
        todo!("read S mwrite None write None rac None reset value false")
    }
    #[doc = "S: MPU Region Attribute field: Sharable<br>"]
    pub(crate) fn scs_mpu_rasr_a328_s_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write S mwrite None write None rac None reset value false")
    }
    #[doc = "TEX: MPU Region Attribute field<br>"]
    pub(crate) fn scs_mpu_rasr_a328_tex_read(&self) -> MemResult<u8> {
        todo ! ("read TEX mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "TEX: MPU Region Attribute field<br>"]
    pub(crate) fn scs_mpu_rasr_a328_tex_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write TEX mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "AP: Access permissions<br>"]
    pub(crate) fn scs_mpu_rasr_a328_ap_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E71ScsMpuRasr10Ap> {
        todo ! ("read AP mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "AP: Access permissions<br>"]
    pub(crate) fn scs_mpu_rasr_a328_ap_write(
        &mut self,
        _value: crate::peripheral::enums::E71ScsMpuRasr10Ap,
    ) -> MemResult<()> {
        todo ! ("write AP mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "XN: Execute Never<br>"]
    pub(crate) fn scs_mpu_rasr_a328_xn_read(&self) -> MemResult<bool> {
        todo!("read XN mwrite None write None rac None reset value false")
    }
    #[doc = "XN: Execute Never<br>"]
    pub(crate) fn scs_mpu_rasr_a328_xn_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write XN mwrite None write None rac None reset value false")
    }
    #[doc = "State1: Thumb instruction set support<br>"]
    pub(crate) fn scs_id_pfr040_state1_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E72ScsIdPfr040State1> {
        todo ! ("read State1 mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "M_Profile: M profile programmers' model<br>"]
    pub(crate) fn scs_id_pfr144_m_profile_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E73ScsIdPfr144MProfile> {
        todo ! ("read M_Profile mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "M_Profile: Debug model, M profile<br>"]
    pub(crate) fn scs_id_dfr048_m_profile_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E74ScsIdDfr048MProfile> {
        todo ! ("read M_Profile mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "ID_AFR0: Implementation defined features<br>"]
    pub(crate) fn scs_id_afr04c_read(&self) -> MemResult<u32> {
        todo ! ("read scs_id_afr04c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "PMSA: Indicates support for a PMSA<br>"]
    pub(crate) fn scs_id_mmfr050_pmsa_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E75ScsIdMmfr050Pmsa> {
        todo ! ("read PMSA mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "Outermost_shareability: Indicates the outermost shareability domain implemented<br>"]
    pub(crate) fn scs_id_mmfr050_outermost_shareability_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E76ScsIdMmfr050OutermostShareability>
    {
        todo ! ("read Outermost_shareability mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "Shareability_levels: Indicates the number of shareability levels implemented<br>"]
    pub(crate) fn scs_id_mmfr050_shareability_levels_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E77ScsIdMmfr050ShareabilityLevels>
    {
        todo ! ("read Shareability_levels mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "Auxiliary_registers: Indicates support for Auxiliary registers<br>"]
    pub(crate) fn scs_id_mmfr050_auxiliary_registers_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E74ScsIdDfr048MProfile> {
        todo ! ("read Auxiliary_registers mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "ID_MMFR1: Memory Model Features<br>"]
    pub(crate) fn scs_id_mmfr154_read(&self) -> MemResult<u32> {
        todo ! ("read scs_id_mmfr154 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "WFI: Indicates support for wait-for-interrupt stalling. <br>"]
    pub(crate) fn scs_id_mmfr258_wfi_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E74ScsIdDfr048MProfile> {
        todo ! ("read WFI mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "ID_MMFR3: Memory Model Features<br>"]
    pub(crate) fn scs_id_mmfr35c_read(&self) -> MemResult<u32> {
        todo ! ("read scs_id_mmfr35c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "BitCount_instrs: Indicates support for bit counting instructions. <br>"]
    pub(crate) fn scs_id_isar060_bitcount_instrs_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E74ScsIdDfr048MProfile> {
        todo ! ("read BitCount_instrs mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "Bitfield_instrs: Indicates support for bitfield instructions. <br>"]
    pub(crate) fn scs_id_isar060_bitfield_instrs_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E74ScsIdDfr048MProfile> {
        todo ! ("read Bitfield_instrs mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "CmpBranch_instrs: Indicates support for combined compare and branch instructions.<br>"]
    pub(crate) fn scs_id_isar060_cmpbranch_instrs_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E74ScsIdDfr048MProfile> {
        todo ! ("read CmpBranch_instrs mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "Coproc_instrs: Indicates the supported coprocessor instructions<br>"]
    pub(crate) fn scs_id_isar060_coproc_instrs_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E78ScsIdIsar060CoprocInstrs> {
        todo ! ("read Coproc_instrs mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "Debug_instrs: Indicates the supported debug instructions<br>"]
    pub(crate) fn scs_id_isar060_debug_instrs_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E74ScsIdDfr048MProfile> {
        todo ! ("read Debug_instrs mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "Divide_instrs: Indicates the supported divide instructions<br>"]
    pub(crate) fn scs_id_isar060_divide_instrs_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E74ScsIdDfr048MProfile> {
        todo ! ("read Divide_instrs mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "Extend_instrs: Indicates support for sign or zero extend instructions. <br>"]
    pub(crate) fn scs_id_isar164_extend_instrs_read(&self) -> MemResult<u8> {
        todo ! ("read Extend_instrs mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "IfThen_instrs: Indicates support for IfThen instructions.<br>"]
    pub(crate) fn scs_id_isar164_ifthen_instrs_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E74ScsIdDfr048MProfile> {
        todo ! ("read IfThen_instrs mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "Immediate_instrs: Indicates support for immediate instructions. <br>"]
    pub(crate) fn scs_id_isar164_immediate_instrs_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E74ScsIdDfr048MProfile> {
        todo ! ("read Immediate_instrs mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "Interwork_instrs: Indicates support for instructions that branch between ARM and Thumb code.<br>"]
    pub(crate) fn scs_id_isar164_interwork_instrs_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E79ScsIdIsar164InterworkInstrs>
    {
        todo ! ("read Interwork_instrs mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "LoadStore_instrs: Indicates the supported additional load and store instructions. <br>"]
    pub(crate) fn scs_id_isar268_loadstore_instrs_read(&self) -> MemResult<u8> {
        todo ! ("read LoadStore_instrs mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "MemHint_instrs: Indicates the implemented Memory Hint instructions.<br>"]
    pub(crate) fn scs_id_isar268_memhint_instrs_read(&self) -> MemResult<u8> {
        todo ! ("read MemHint_instrs mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "MultiAccessInt_instrs: Indicates the supported multi-access interruptible instructions.<br>"]
    pub(crate) fn scs_id_isar268_multiaccessint_instrs_read(
        &self,
    ) -> MemResult<u8> {
        todo ! ("read MultiAccessInt_instrs mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "Mult_instrs: Indicates the supported additional multiply instructions. <br>"]
    pub(crate) fn scs_id_isar268_mult_instrs_read(&self) -> MemResult<u8> {
        todo ! ("read Mult_instrs mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "MultS_instrs: Indicates the supported advanced signed multiply instructions.<br>"]
    pub(crate) fn scs_id_isar268_mults_instrs_read(&self) -> MemResult<u8> {
        todo ! ("read MultS_instrs mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "MultU_instrs: Indicates the supported advanced unsigned multiply instructions.<br>"]
    pub(crate) fn scs_id_isar268_multu_instrs_read(&self) -> MemResult<u8> {
        todo ! ("read MultU_instrs mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "Reversal_instrs: Indicates the supported reversal instructions. <br>"]
    pub(crate) fn scs_id_isar268_reversal_instrs_read(&self) -> MemResult<u8> {
        todo ! ("read Reversal_instrs mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "Saturate_instrs: Indicates support for saturate instructions. <br>"]
    pub(crate) fn scs_id_isar36c_saturate_instrs_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E74ScsIdDfr048MProfile> {
        todo ! ("read Saturate_instrs mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "SIMD_instrs: Indicates support for Single Instruction Multiple Data (SIMD) instructions.<br>"]
    pub(crate) fn scs_id_isar36c_simd_instrs_read(&self) -> MemResult<u8> {
        todo ! ("read SIMD_instrs mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "SVC_instrs: Indicates support for SVC instructions.<br>"]
    pub(crate) fn scs_id_isar36c_svc_instrs_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E74ScsIdDfr048MProfile> {
        todo ! ("read SVC_instrs mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "SynchPrim_instrs: Indicates support for synchronization primitive instructions.<br>"]
    pub(crate) fn scs_id_isar36c_synchprim_instrs_read(&self) -> MemResult<u8> {
        todo ! ("read SynchPrim_instrs mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "TabBranch_instrs: Indicates support for table branch instructions. <br>"]
    pub(crate) fn scs_id_isar36c_tabbranch_instrs_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E74ScsIdDfr048MProfile> {
        todo ! ("read TabBranch_instrs mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "ThumbCopy_instrs: Indicates support for Thumb copy instructions. <br>"]
    pub(crate) fn scs_id_isar36c_thumbcopy_instrs_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E74ScsIdDfr048MProfile> {
        todo ! ("read ThumbCopy_instrs mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "TrueNOP_instrs: Indicates support for true NOP instructions.<br>"]
    pub(crate) fn scs_id_isar36c_truenop_instrs_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E74ScsIdDfr048MProfile> {
        todo ! ("read TrueNOP_instrs mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "Unpriv_instrs: Indicates the supported unprivileged instructions<br>"]
    pub(crate) fn scs_id_isar470_unpriv_instrs_read(&self) -> MemResult<u8> {
        todo ! ("read Unpriv_instrs mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "WithShifts_instrs: Indicates the support for instructions with shifts:<br>"]
    pub(crate) fn scs_id_isar470_withshifts_instrs_read(
        &self,
    ) -> MemResult<u8> {
        todo ! ("read WithShifts_instrs mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "Writeback_instrs: Indicates support for Writeback addressing modes<br>"]
    pub(crate) fn scs_id_isar470_writeback_instrs_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E80ScsIdIsar470WritebackInstrs>
    {
        todo ! ("read Writeback_instrs mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "Barrier_instrs: Indicates the supported barrier instructions. <br>"]
    pub(crate) fn scs_id_isar470_barrier_instrs_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E74ScsIdDfr048MProfile> {
        todo ! ("read Barrier_instrs mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "SynchPrim_instrs_frac: Indicates support for Synchronization Primitives<br>"]
    pub(crate) fn scs_id_isar470_synchprim_instrs_frac_read(
        &self,
    ) -> MemResult<u8> {
        todo ! ("read SynchPrim_instrs_frac mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "PSR_M_instrs: Indicates support for saturate instructions. <br>"]
    pub(crate) fn scs_id_isar470_psr_m_instrs_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E74ScsIdDfr048MProfile> {
        todo ! ("read PSR_M_instrs mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "NVIC_ISER0: Enables, or reads the enable state of a group of interrupts<br>"]
    pub(crate) fn scs_nvic_iser0fc_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_enabled[0])
    }
    #[doc = "NVIC_ISER0: Enables, or reads the enable state of a group of interrupts<br>"]
    pub fn scs_nvic_iser0fc_write(&mut self, _value: u32) -> MemResult<()> {
        Ok(self.interrupts_enabled[0] |= _value)
    }
    #[doc = "NVIC_ISER1: Enables, or reads the enable state of a group of interrupts<br>"]
    pub(crate) fn scs_nvic_iser1100_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_enabled[1])
    }
    #[doc = "NVIC_ISER1: Enables, or reads the enable state of a group of interrupts<br>"]
    pub fn scs_nvic_iser1100_write(&mut self, _value: u32) -> MemResult<()> {
        Ok(self.interrupts_enabled[1] |= _value)
    }
    #[doc = "NVIC_ISER2: Enables, or reads the enable state of a group of interrupts<br>"]
    pub(crate) fn scs_nvic_iser2104_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_enabled[2])
    }
    #[doc = "NVIC_ISER2: Enables, or reads the enable state of a group of interrupts<br>"]
    pub fn scs_nvic_iser2104_write(&mut self, _value: u32) -> MemResult<()> {
        Ok(self.interrupts_enabled[2] |= _value)
    }
    #[doc = "NVIC_ISER3: Enables, or reads the enable state of a group of interrupts<br>"]
    pub(crate) fn scs_nvic_iser3108_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_enabled[3])
    }
    #[doc = "NVIC_ISER3: Enables, or reads the enable state of a group of interrupts<br>"]
    pub fn scs_nvic_iser3108_write(&mut self, _value: u32) -> MemResult<()> {
        Ok(self.interrupts_enabled[3] |= _value)
    }
    #[doc = "NVIC_ISER4: Enables, or reads the enable state of a group of interrupts<br>"]
    pub(crate) fn scs_nvic_iser410c_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_enabled[4])
    }
    #[doc = "NVIC_ISER4: Enables, or reads the enable state of a group of interrupts<br>"]
    pub fn scs_nvic_iser410c_write(&mut self, _value: u32) -> MemResult<()> {
        Ok(self.interrupts_enabled[4] |= _value)
    }
    #[doc = "NVIC_ISER5: Enables, or reads the enable state of a group of interrupts<br>"]
    pub(crate) fn scs_nvic_iser5110_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_enabled[5])
    }
    #[doc = "NVIC_ISER5: Enables, or reads the enable state of a group of interrupts<br>"]
    pub fn scs_nvic_iser5110_write(&mut self, _value: u32) -> MemResult<()> {
        Ok(self.interrupts_enabled[5] |= _value)
    }
    #[doc = "NVIC_ISER6: Enables, or reads the enable state of a group of interrupts<br>"]
    pub(crate) fn scs_nvic_iser6114_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_enabled[6])
    }
    #[doc = "NVIC_ISER6: Enables, or reads the enable state of a group of interrupts<br>"]
    pub fn scs_nvic_iser6114_write(&mut self, _value: u32) -> MemResult<()> {
        Ok(self.interrupts_enabled[6] |= _value)
    }
    #[doc = "NVIC_ISER7: Enables, or reads the enable state of a group of interrupts<br>"]
    pub(crate) fn scs_nvic_iser7118_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_enabled[7])
    }
    #[doc = "NVIC_ISER7: Enables, or reads the enable state of a group of interrupts<br>"]
    pub fn scs_nvic_iser7118_write(&mut self, _value: u32) -> MemResult<()> {
        Ok(self.interrupts_enabled[7] |= _value)
    }
    #[doc = "NVIC_ICER0: Disables, or reads the enable state of a group of interrupts<br>"]
    pub(crate) fn scs_nvic_icer017c_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_enabled[0])
    }
    #[doc = "NVIC_ICER0: Disables, or reads the enable state of a group of interrupts<br>"]
    pub fn scs_nvic_icer017c_write(&mut self, _value: u32) -> MemResult<()> {
        Ok(self.interrupts_enabled[0] &= !_value)
    }
    #[doc = "NVIC_ICER1: Disables, or reads the enable state of a group of interrupts<br>"]
    pub(crate) fn scs_nvic_icer1180_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_enabled[1])
    }
    #[doc = "NVIC_ICER1: Disables, or reads the enable state of a group of interrupts<br>"]
    pub fn scs_nvic_icer1180_write(&mut self, _value: u32) -> MemResult<()> {
        Ok(self.interrupts_enabled[1] &= !_value)
    }
    #[doc = "NVIC_ICER2: Disables, or reads the enable state of a group of interrupts<br>"]
    pub(crate) fn scs_nvic_icer2184_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_enabled[2])
    }
    #[doc = "NVIC_ICER2: Disables, or reads the enable state of a group of interrupts<br>"]
    pub fn scs_nvic_icer2184_write(&mut self, _value: u32) -> MemResult<()> {
        Ok(self.interrupts_enabled[2] &= !_value)
    }
    #[doc = "NVIC_ICER3: Disables, or reads the enable state of a group of interrupts<br>"]
    pub(crate) fn scs_nvic_icer3188_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_enabled[3])
    }
    #[doc = "NVIC_ICER3: Disables, or reads the enable state of a group of interrupts<br>"]
    pub fn scs_nvic_icer3188_write(&mut self, _value: u32) -> MemResult<()> {
        Ok(self.interrupts_enabled[3] &= !_value)
    }
    #[doc = "NVIC_ICER4: Disables, or reads the enable state of a group of interrupts<br>"]
    pub(crate) fn scs_nvic_icer418c_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_enabled[4])
    }
    #[doc = "NVIC_ICER4: Disables, or reads the enable state of a group of interrupts<br>"]
    pub fn scs_nvic_icer418c_write(&mut self, _value: u32) -> MemResult<()> {
        Ok(self.interrupts_enabled[4] &= !_value)
    }
    #[doc = "NVIC_ICER5: Disables, or reads the enable state of a group of interrupts<br>"]
    pub(crate) fn scs_nvic_icer5190_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_enabled[5])
    }
    #[doc = "NVIC_ICER5: Disables, or reads the enable state of a group of interrupts<br>"]
    pub fn scs_nvic_icer5190_write(&mut self, _value: u32) -> MemResult<()> {
        Ok(self.interrupts_enabled[5] &= !_value)
    }
    #[doc = "NVIC_ICER6: Disables, or reads the enable state of a group of interrupts<br>"]
    pub(crate) fn scs_nvic_icer6194_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_enabled[6])
    }
    #[doc = "NVIC_ICER6: Disables, or reads the enable state of a group of interrupts<br>"]
    pub fn scs_nvic_icer6194_write(&mut self, _value: u32) -> MemResult<()> {
        Ok(self.interrupts_enabled[6] &= !_value)
    }
    #[doc = "NVIC_ICER7: Disables, or reads the enable state of a group of interrupts<br>"]
    pub(crate) fn scs_nvic_icer7198_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_enabled[7])
    }
    #[doc = "NVIC_ICER7: Disables, or reads the enable state of a group of interrupts<br>"]
    pub fn scs_nvic_icer7198_write(&mut self, _value: u32) -> MemResult<()> {
        Ok(self.interrupts_enabled[7] &= !_value)
    }
    #[doc = "NVIC_ISPR0: For a group of interrupts, changes interrupt status to pending, or shows the current pending\nstatus<br>"]
    pub(crate) fn scs_nvic_ispr01fc_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_pending[0])
    }
    #[doc = "NVIC_ISPR0: For a group of interrupts, changes interrupt status to pending, or shows the current pending\nstatus<br>"]
    pub fn scs_nvic_ispr01fc_write(&mut self, _value: u32) -> MemResult<()> {
        Ok(self.interrupts_pending[0] |= _value)
    }
    #[doc = "NVIC_ISPR1: For a group of interrupts, changes interrupt status to pending, or shows the current pending\nstatus<br>"]
    pub(crate) fn scs_nvic_ispr1200_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_pending[1])
    }
    #[doc = "NVIC_ISPR1: For a group of interrupts, changes interrupt status to pending, or shows the current pending\nstatus<br>"]
    pub fn scs_nvic_ispr1200_write(&mut self, _value: u32) -> MemResult<()> {
        Ok(self.interrupts_pending[1] |= _value)
    }
    #[doc = "NVIC_ISPR2: For a group of interrupts, changes interrupt status to pending, or shows the current pending\nstatus<br>"]
    pub(crate) fn scs_nvic_ispr2204_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_pending[2])
    }
    #[doc = "NVIC_ISPR2: For a group of interrupts, changes interrupt status to pending, or shows the current pending\nstatus<br>"]
    pub fn scs_nvic_ispr2204_write(&mut self, _value: u32) -> MemResult<()> {
        Ok(self.interrupts_pending[2] |= _value)
    }
    #[doc = "NVIC_ISPR3: For a group of interrupts, changes interrupt status to pending, or shows the current pending\nstatus<br>"]
    pub(crate) fn scs_nvic_ispr3208_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_pending[3])
    }
    #[doc = "NVIC_ISPR3: For a group of interrupts, changes interrupt status to pending, or shows the current pending\nstatus<br>"]
    pub fn scs_nvic_ispr3208_write(&mut self, _value: u32) -> MemResult<()> {
        Ok(self.interrupts_pending[3] |= _value)
    }
    #[doc = "NVIC_ISPR4: For a group of interrupts, changes interrupt status to pending, or shows the current pending\nstatus<br>"]
    pub(crate) fn scs_nvic_ispr420c_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_pending[4])
    }
    #[doc = "NVIC_ISPR4: For a group of interrupts, changes interrupt status to pending, or shows the current pending\nstatus<br>"]
    pub fn scs_nvic_ispr420c_write(&mut self, _value: u32) -> MemResult<()> {
        Ok(self.interrupts_pending[4] |= _value)
    }
    #[doc = "NVIC_ISPR5: For a group of interrupts, changes interrupt status to pending, or shows the current pending\nstatus<br>"]
    pub(crate) fn scs_nvic_ispr5210_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_pending[5])
    }
    #[doc = "NVIC_ISPR5: For a group of interrupts, changes interrupt status to pending, or shows the current pending\nstatus<br>"]
    pub fn scs_nvic_ispr5210_write(&mut self, _value: u32) -> MemResult<()> {
        Ok(self.interrupts_pending[5] |= _value)
    }
    #[doc = "NVIC_ISPR6: For a group of interrupts, changes interrupt status to pending, or shows the current pending\nstatus<br>"]
    pub(crate) fn scs_nvic_ispr6214_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_pending[6])
    }
    #[doc = "NVIC_ISPR6: For a group of interrupts, changes interrupt status to pending, or shows the current pending\nstatus<br>"]
    pub fn scs_nvic_ispr6214_write(&mut self, _value: u32) -> MemResult<()> {
        Ok(self.interrupts_pending[6] |= _value)
    }
    #[doc = "NVIC_ISPR7: For a group of interrupts, changes interrupt status to pending, or shows the current pending\nstatus<br>"]
    pub(crate) fn scs_nvic_ispr7218_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_pending[7])
    }
    #[doc = "NVIC_ISPR7: For a group of interrupts, changes interrupt status to pending, or shows the current pending\nstatus<br>"]
    pub fn scs_nvic_ispr7218_write(&mut self, _value: u32) -> MemResult<()> {
        Ok(self.interrupts_pending[7] |= _value)
    }
    #[doc = "NVIC_ICPR0: For a group of interrupts, clears the interrupt pending status, or shows the current pending\nstatus<br>"]
    pub(crate) fn scs_nvic_icpr027c_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_pending[0])
    }
    #[doc = "NVIC_ICPR0: For a group of interrupts, clears the interrupt pending status, or shows the current pending\nstatus<br>"]
    pub fn scs_nvic_icpr027c_write(&mut self, _value: u32) -> MemResult<()> {
        Ok(self.interrupts_pending[0] &= !_value)
    }
    #[doc = "NVIC_ICPR1: For a group of interrupts, clears the interrupt pending status, or shows the current pending\nstatus<br>"]
    pub(crate) fn scs_nvic_icpr1280_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_pending[1])
    }
    #[doc = "NVIC_ICPR1: For a group of interrupts, clears the interrupt pending status, or shows the current pending\nstatus<br>"]
    pub fn scs_nvic_icpr1280_write(&mut self, _value: u32) -> MemResult<()> {
        Ok(self.interrupts_pending[1] &= !_value)
    }
    #[doc = "NVIC_ICPR2: For a group of interrupts, clears the interrupt pending status, or shows the current pending\nstatus<br>"]
    pub(crate) fn scs_nvic_icpr2284_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_pending[2])
    }
    #[doc = "NVIC_ICPR2: For a group of interrupts, clears the interrupt pending status, or shows the current pending\nstatus<br>"]
    pub fn scs_nvic_icpr2284_write(&mut self, _value: u32) -> MemResult<()> {
        Ok(self.interrupts_pending[2] &= !_value)
    }
    #[doc = "NVIC_ICPR3: For a group of interrupts, clears the interrupt pending status, or shows the current pending\nstatus<br>"]
    pub(crate) fn scs_nvic_icpr3288_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_pending[3])
    }
    #[doc = "NVIC_ICPR3: For a group of interrupts, clears the interrupt pending status, or shows the current pending\nstatus<br>"]
    pub fn scs_nvic_icpr3288_write(&mut self, _value: u32) -> MemResult<()> {
        Ok(self.interrupts_pending[3] &= !_value)
    }
    #[doc = "NVIC_ICPR4: For a group of interrupts, clears the interrupt pending status, or shows the current pending\nstatus<br>"]
    pub(crate) fn scs_nvic_icpr428c_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_pending[4])
    }
    #[doc = "NVIC_ICPR4: For a group of interrupts, clears the interrupt pending status, or shows the current pending\nstatus<br>"]
    pub fn scs_nvic_icpr428c_write(&mut self, _value: u32) -> MemResult<()> {
        Ok(self.interrupts_pending[4] &= !_value)
    }
    #[doc = "NVIC_ICPR5: For a group of interrupts, clears the interrupt pending status, or shows the current pending\nstatus<br>"]
    pub(crate) fn scs_nvic_icpr5290_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_pending[5])
    }
    #[doc = "NVIC_ICPR5: For a group of interrupts, clears the interrupt pending status, or shows the current pending\nstatus<br>"]
    pub fn scs_nvic_icpr5290_write(&mut self, _value: u32) -> MemResult<()> {
        Ok(self.interrupts_pending[5] &= !_value)
    }
    #[doc = "NVIC_ICPR6: For a group of interrupts, clears the interrupt pending status, or shows the current pending\nstatus<br>"]
    pub(crate) fn scs_nvic_icpr6294_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_pending[6])
    }
    #[doc = "NVIC_ICPR6: For a group of interrupts, clears the interrupt pending status, or shows the current pending\nstatus<br>"]
    pub fn scs_nvic_icpr6294_write(&mut self, _value: u32) -> MemResult<()> {
        Ok(self.interrupts_pending[6] &= !_value)
    }
    #[doc = "NVIC_ICPR7: For a group of interrupts, clears the interrupt pending status, or shows the current pending\nstatus<br>"]
    pub(crate) fn scs_nvic_icpr7298_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_pending[7])
    }
    #[doc = "NVIC_ICPR7: For a group of interrupts, clears the interrupt pending status, or shows the current pending\nstatus<br>"]
    pub fn scs_nvic_icpr7298_write(&mut self, _value: u32) -> MemResult<()> {
        Ok(self.interrupts_pending[7] &= !_value)
    }
    #[doc = "NVIC_IABR0: For a group of 32 interrupts, shows whether each interrupt is active<br>"]
    pub(crate) fn scs_nvic_iabr02fc_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_enabled[0])
    }
    #[doc = "NVIC_IABR0: For a group of 32 interrupts, shows whether each interrupt is active<br>"]
    pub(crate) fn scs_nvic_iabr02fc_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write scs_nvic_iabr02fc mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "NVIC_IABR1: For a group of 32 interrupts, shows whether each interrupt is active<br>"]
    pub(crate) fn scs_nvic_iabr1300_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_enabled[1])
    }
    #[doc = "NVIC_IABR1: For a group of 32 interrupts, shows whether each interrupt is active<br>"]
    pub(crate) fn scs_nvic_iabr1300_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write scs_nvic_iabr1300 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "NVIC_IABR2: For a group of 32 interrupts, shows whether each interrupt is active<br>"]
    pub(crate) fn scs_nvic_iabr2304_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_enabled[2])
    }
    #[doc = "NVIC_IABR2: For a group of 32 interrupts, shows whether each interrupt is active<br>"]
    pub(crate) fn scs_nvic_iabr2304_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write scs_nvic_iabr2304 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "NVIC_IABR3: For a group of 32 interrupts, shows whether each interrupt is active<br>"]
    pub(crate) fn scs_nvic_iabr3308_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_enabled[3])
    }
    #[doc = "NVIC_IABR3: For a group of 32 interrupts, shows whether each interrupt is active<br>"]
    pub(crate) fn scs_nvic_iabr3308_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write scs_nvic_iabr3308 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "NVIC_IABR4: For a group of 32 interrupts, shows whether each interrupt is active<br>"]
    pub(crate) fn scs_nvic_iabr430c_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_enabled[4])
    }
    #[doc = "NVIC_IABR4: For a group of 32 interrupts, shows whether each interrupt is active<br>"]
    pub(crate) fn scs_nvic_iabr430c_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write scs_nvic_iabr430c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "NVIC_IABR5: For a group of 32 interrupts, shows whether each interrupt is active<br>"]
    pub(crate) fn scs_nvic_iabr5310_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_enabled[5])
    }
    #[doc = "NVIC_IABR5: For a group of 32 interrupts, shows whether each interrupt is active<br>"]
    pub(crate) fn scs_nvic_iabr5310_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write scs_nvic_iabr5310 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "NVIC_IABR6: For a group of 32 interrupts, shows whether each interrupt is active<br>"]
    pub(crate) fn scs_nvic_iabr6314_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_enabled[6])
    }
    #[doc = "NVIC_IABR6: For a group of 32 interrupts, shows whether each interrupt is active<br>"]
    pub(crate) fn scs_nvic_iabr6314_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write scs_nvic_iabr6314 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "NVIC_IABR7: For a group of 32 interrupts, shows whether each interrupt is active<br>"]
    pub(crate) fn scs_nvic_iabr7318_read(&self) -> MemResult<u32> {
        Ok(self.interrupts_enabled[7])
    }
    #[doc = "NVIC_IABR7: For a group of 32 interrupts, shows whether each interrupt is active<br>"]
    pub(crate) fn scs_nvic_iabr7318_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write scs_nvic_iabr7318 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "PRI_N0: Priority of interrupt 0<br>"]
    pub(crate) fn scs_nvic_ipr03fc_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 0) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 0<br>"]
    pub fn scs_nvic_ipr03fc_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 0) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 1<br>"]
    pub(crate) fn scs_nvic_ipr03fc_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 0) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 1<br>"]
    pub fn scs_nvic_ipr03fc_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 0) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 2<br>"]
    pub(crate) fn scs_nvic_ipr03fc_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 0) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 2<br>"]
    pub fn scs_nvic_ipr03fc_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 0) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 3<br>"]
    pub(crate) fn scs_nvic_ipr03fc_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 0) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 3<br>"]
    pub fn scs_nvic_ipr03fc_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 0) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 4<br>"]
    pub(crate) fn scs_nvic_ipr1400_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 1) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 4<br>"]
    pub fn scs_nvic_ipr1400_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 1) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 5<br>"]
    pub(crate) fn scs_nvic_ipr1400_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 1) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 5<br>"]
    pub fn scs_nvic_ipr1400_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 1) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 6<br>"]
    pub(crate) fn scs_nvic_ipr1400_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 1) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 6<br>"]
    pub fn scs_nvic_ipr1400_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 1) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 7<br>"]
    pub(crate) fn scs_nvic_ipr1400_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 1) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 7<br>"]
    pub fn scs_nvic_ipr1400_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 1) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 8<br>"]
    pub(crate) fn scs_nvic_ipr2404_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 2) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 8<br>"]
    pub fn scs_nvic_ipr2404_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 2) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 9<br>"]
    pub(crate) fn scs_nvic_ipr2404_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 2) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 9<br>"]
    pub fn scs_nvic_ipr2404_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 2) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 10<br>"]
    pub(crate) fn scs_nvic_ipr2404_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 2) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 10<br>"]
    pub fn scs_nvic_ipr2404_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 2) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 11<br>"]
    pub(crate) fn scs_nvic_ipr2404_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 2) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 11<br>"]
    pub fn scs_nvic_ipr2404_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 2) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 12<br>"]
    pub(crate) fn scs_nvic_ipr3408_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 3) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 12<br>"]
    pub fn scs_nvic_ipr3408_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 3) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 13<br>"]
    pub(crate) fn scs_nvic_ipr3408_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 3) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 13<br>"]
    pub fn scs_nvic_ipr3408_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 3) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 14<br>"]
    pub(crate) fn scs_nvic_ipr3408_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 3) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 14<br>"]
    pub fn scs_nvic_ipr3408_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 3) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 15<br>"]
    pub(crate) fn scs_nvic_ipr3408_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 3) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 15<br>"]
    pub fn scs_nvic_ipr3408_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 3) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 16<br>"]
    pub(crate) fn scs_nvic_ipr440c_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 4) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 16<br>"]
    pub fn scs_nvic_ipr440c_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 4) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 17<br>"]
    pub(crate) fn scs_nvic_ipr440c_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 4) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 17<br>"]
    pub fn scs_nvic_ipr440c_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 4) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 18<br>"]
    pub(crate) fn scs_nvic_ipr440c_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 4) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 18<br>"]
    pub fn scs_nvic_ipr440c_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 4) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 19<br>"]
    pub(crate) fn scs_nvic_ipr440c_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 4) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 19<br>"]
    pub fn scs_nvic_ipr440c_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 4) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 20<br>"]
    pub(crate) fn scs_nvic_ipr5410_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 5) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 20<br>"]
    pub fn scs_nvic_ipr5410_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 5) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 21<br>"]
    pub(crate) fn scs_nvic_ipr5410_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 5) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 21<br>"]
    pub fn scs_nvic_ipr5410_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 5) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 22<br>"]
    pub(crate) fn scs_nvic_ipr5410_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 5) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 22<br>"]
    pub fn scs_nvic_ipr5410_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 5) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 23<br>"]
    pub(crate) fn scs_nvic_ipr5410_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 5) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 23<br>"]
    pub fn scs_nvic_ipr5410_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 5) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 24<br>"]
    pub(crate) fn scs_nvic_ipr6414_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 6) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 24<br>"]
    pub fn scs_nvic_ipr6414_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 6) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 25<br>"]
    pub(crate) fn scs_nvic_ipr6414_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 6) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 25<br>"]
    pub fn scs_nvic_ipr6414_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 6) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 26<br>"]
    pub(crate) fn scs_nvic_ipr6414_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 6) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 26<br>"]
    pub fn scs_nvic_ipr6414_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 6) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 27<br>"]
    pub(crate) fn scs_nvic_ipr6414_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 6) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 27<br>"]
    pub fn scs_nvic_ipr6414_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 6) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 28<br>"]
    pub(crate) fn scs_nvic_ipr7418_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 7) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 28<br>"]
    pub fn scs_nvic_ipr7418_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 7) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 29<br>"]
    pub(crate) fn scs_nvic_ipr7418_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 7) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 29<br>"]
    pub fn scs_nvic_ipr7418_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 7) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 30<br>"]
    pub(crate) fn scs_nvic_ipr7418_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 7) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 30<br>"]
    pub fn scs_nvic_ipr7418_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 7) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 31<br>"]
    pub(crate) fn scs_nvic_ipr7418_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 7) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 31<br>"]
    pub fn scs_nvic_ipr7418_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 7) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 32<br>"]
    pub(crate) fn scs_nvic_ipr841c_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 8) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 32<br>"]
    pub fn scs_nvic_ipr841c_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 8) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 33<br>"]
    pub(crate) fn scs_nvic_ipr841c_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 8) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 33<br>"]
    pub fn scs_nvic_ipr841c_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 8) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 34<br>"]
    pub(crate) fn scs_nvic_ipr841c_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 8) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 34<br>"]
    pub fn scs_nvic_ipr841c_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 8) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 35<br>"]
    pub(crate) fn scs_nvic_ipr841c_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 8) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 35<br>"]
    pub fn scs_nvic_ipr841c_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 8) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 36<br>"]
    pub(crate) fn scs_nvic_ipr9420_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 9) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 36<br>"]
    pub fn scs_nvic_ipr9420_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 9) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 37<br>"]
    pub(crate) fn scs_nvic_ipr9420_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 9) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 37<br>"]
    pub fn scs_nvic_ipr9420_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 9) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 38<br>"]
    pub(crate) fn scs_nvic_ipr9420_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 9) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 38<br>"]
    pub fn scs_nvic_ipr9420_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 9) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 39<br>"]
    pub(crate) fn scs_nvic_ipr9420_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 9) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 39<br>"]
    pub fn scs_nvic_ipr9420_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 9) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 40<br>"]
    pub(crate) fn scs_nvic_ipr10424_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 10) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 40<br>"]
    pub fn scs_nvic_ipr10424_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 10) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 41<br>"]
    pub(crate) fn scs_nvic_ipr10424_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 10) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 41<br>"]
    pub fn scs_nvic_ipr10424_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 10) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 42<br>"]
    pub(crate) fn scs_nvic_ipr10424_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 10) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 42<br>"]
    pub fn scs_nvic_ipr10424_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 10) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 43<br>"]
    pub(crate) fn scs_nvic_ipr10424_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 10) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 43<br>"]
    pub fn scs_nvic_ipr10424_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 10) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 44<br>"]
    pub(crate) fn scs_nvic_ipr11428_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 11) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 44<br>"]
    pub fn scs_nvic_ipr11428_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 11) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 45<br>"]
    pub(crate) fn scs_nvic_ipr11428_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 11) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 45<br>"]
    pub fn scs_nvic_ipr11428_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 11) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 46<br>"]
    pub(crate) fn scs_nvic_ipr11428_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 11) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 46<br>"]
    pub fn scs_nvic_ipr11428_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 11) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 47<br>"]
    pub(crate) fn scs_nvic_ipr11428_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 11) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 47<br>"]
    pub fn scs_nvic_ipr11428_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 11) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 48<br>"]
    pub(crate) fn scs_nvic_ipr1242c_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 12) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 48<br>"]
    pub fn scs_nvic_ipr1242c_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 12) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 49<br>"]
    pub(crate) fn scs_nvic_ipr1242c_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 12) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 49<br>"]
    pub fn scs_nvic_ipr1242c_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 12) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 50<br>"]
    pub(crate) fn scs_nvic_ipr1242c_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 12) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 50<br>"]
    pub fn scs_nvic_ipr1242c_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 12) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 51<br>"]
    pub(crate) fn scs_nvic_ipr1242c_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 12) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 51<br>"]
    pub fn scs_nvic_ipr1242c_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 12) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 52<br>"]
    pub(crate) fn scs_nvic_ipr13430_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 13) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 52<br>"]
    pub fn scs_nvic_ipr13430_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 13) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 53<br>"]
    pub(crate) fn scs_nvic_ipr13430_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 13) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 53<br>"]
    pub fn scs_nvic_ipr13430_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 13) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 54<br>"]
    pub(crate) fn scs_nvic_ipr13430_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 13) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 54<br>"]
    pub fn scs_nvic_ipr13430_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 13) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 55<br>"]
    pub(crate) fn scs_nvic_ipr13430_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 13) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 55<br>"]
    pub fn scs_nvic_ipr13430_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 13) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 56<br>"]
    pub(crate) fn scs_nvic_ipr14434_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 14) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 56<br>"]
    pub fn scs_nvic_ipr14434_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 14) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 57<br>"]
    pub(crate) fn scs_nvic_ipr14434_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 14) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 57<br>"]
    pub fn scs_nvic_ipr14434_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 14) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 58<br>"]
    pub(crate) fn scs_nvic_ipr14434_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 14) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 58<br>"]
    pub fn scs_nvic_ipr14434_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 14) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 59<br>"]
    pub(crate) fn scs_nvic_ipr14434_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 14) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 59<br>"]
    pub fn scs_nvic_ipr14434_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 14) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 60<br>"]
    pub(crate) fn scs_nvic_ipr15438_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 15) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 60<br>"]
    pub fn scs_nvic_ipr15438_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 15) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 61<br>"]
    pub(crate) fn scs_nvic_ipr15438_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 15) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 61<br>"]
    pub fn scs_nvic_ipr15438_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 15) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 62<br>"]
    pub(crate) fn scs_nvic_ipr15438_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 15) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 62<br>"]
    pub fn scs_nvic_ipr15438_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 15) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 63<br>"]
    pub(crate) fn scs_nvic_ipr15438_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 15) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 63<br>"]
    pub fn scs_nvic_ipr15438_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 15) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 64<br>"]
    pub(crate) fn scs_nvic_ipr1643c_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 16) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 64<br>"]
    pub fn scs_nvic_ipr1643c_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 16) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 65<br>"]
    pub(crate) fn scs_nvic_ipr1643c_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 16) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 65<br>"]
    pub fn scs_nvic_ipr1643c_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 16) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 66<br>"]
    pub(crate) fn scs_nvic_ipr1643c_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 16) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 66<br>"]
    pub fn scs_nvic_ipr1643c_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 16) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 67<br>"]
    pub(crate) fn scs_nvic_ipr1643c_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 16) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 67<br>"]
    pub fn scs_nvic_ipr1643c_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 16) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 68<br>"]
    pub(crate) fn scs_nvic_ipr17440_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 17) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 68<br>"]
    pub fn scs_nvic_ipr17440_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 17) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 69<br>"]
    pub(crate) fn scs_nvic_ipr17440_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 17) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 69<br>"]
    pub fn scs_nvic_ipr17440_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 17) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 70<br>"]
    pub(crate) fn scs_nvic_ipr17440_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 17) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 70<br>"]
    pub fn scs_nvic_ipr17440_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 17) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 71<br>"]
    pub(crate) fn scs_nvic_ipr17440_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 17) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 71<br>"]
    pub fn scs_nvic_ipr17440_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 17) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 72<br>"]
    pub(crate) fn scs_nvic_ipr18444_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 18) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 72<br>"]
    pub fn scs_nvic_ipr18444_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 18) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 73<br>"]
    pub(crate) fn scs_nvic_ipr18444_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 18) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 73<br>"]
    pub fn scs_nvic_ipr18444_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 18) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 74<br>"]
    pub(crate) fn scs_nvic_ipr18444_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 18) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 74<br>"]
    pub fn scs_nvic_ipr18444_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 18) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 75<br>"]
    pub(crate) fn scs_nvic_ipr18444_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 18) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 75<br>"]
    pub fn scs_nvic_ipr18444_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 18) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 76<br>"]
    pub(crate) fn scs_nvic_ipr19448_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 19) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 76<br>"]
    pub fn scs_nvic_ipr19448_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 19) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 77<br>"]
    pub(crate) fn scs_nvic_ipr19448_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 19) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 77<br>"]
    pub fn scs_nvic_ipr19448_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 19) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 78<br>"]
    pub(crate) fn scs_nvic_ipr19448_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 19) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 78<br>"]
    pub fn scs_nvic_ipr19448_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 19) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 79<br>"]
    pub(crate) fn scs_nvic_ipr19448_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 19) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 79<br>"]
    pub fn scs_nvic_ipr19448_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 19) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 80<br>"]
    pub(crate) fn scs_nvic_ipr2044c_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 20) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 80<br>"]
    pub fn scs_nvic_ipr2044c_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 20) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 81<br>"]
    pub(crate) fn scs_nvic_ipr2044c_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 20) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 81<br>"]
    pub fn scs_nvic_ipr2044c_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 20) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 82<br>"]
    pub(crate) fn scs_nvic_ipr2044c_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 20) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 82<br>"]
    pub fn scs_nvic_ipr2044c_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 20) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 83<br>"]
    pub(crate) fn scs_nvic_ipr2044c_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 20) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 83<br>"]
    pub fn scs_nvic_ipr2044c_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 20) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 84<br>"]
    pub(crate) fn scs_nvic_ipr21450_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 21) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 84<br>"]
    pub fn scs_nvic_ipr21450_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 21) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 85<br>"]
    pub(crate) fn scs_nvic_ipr21450_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 21) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 85<br>"]
    pub fn scs_nvic_ipr21450_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 21) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 86<br>"]
    pub(crate) fn scs_nvic_ipr21450_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 21) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 86<br>"]
    pub fn scs_nvic_ipr21450_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 21) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 87<br>"]
    pub(crate) fn scs_nvic_ipr21450_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 21) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 87<br>"]
    pub fn scs_nvic_ipr21450_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 21) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 88<br>"]
    pub(crate) fn scs_nvic_ipr22454_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 22) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 88<br>"]
    pub fn scs_nvic_ipr22454_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 22) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 89<br>"]
    pub(crate) fn scs_nvic_ipr22454_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 22) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 89<br>"]
    pub fn scs_nvic_ipr22454_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 22) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 90<br>"]
    pub(crate) fn scs_nvic_ipr22454_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 22) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 90<br>"]
    pub fn scs_nvic_ipr22454_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 22) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 91<br>"]
    pub(crate) fn scs_nvic_ipr22454_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 22) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 91<br>"]
    pub fn scs_nvic_ipr22454_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 22) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 92<br>"]
    pub(crate) fn scs_nvic_ipr23458_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 23) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 92<br>"]
    pub fn scs_nvic_ipr23458_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 23) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 93<br>"]
    pub(crate) fn scs_nvic_ipr23458_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 23) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 93<br>"]
    pub fn scs_nvic_ipr23458_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 23) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 94<br>"]
    pub(crate) fn scs_nvic_ipr23458_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 23) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 94<br>"]
    pub fn scs_nvic_ipr23458_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 23) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 95<br>"]
    pub(crate) fn scs_nvic_ipr23458_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 23) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 95<br>"]
    pub fn scs_nvic_ipr23458_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 23) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 96<br>"]
    pub(crate) fn scs_nvic_ipr2445c_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 24) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 96<br>"]
    pub fn scs_nvic_ipr2445c_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 24) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 97<br>"]
    pub(crate) fn scs_nvic_ipr2445c_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 24) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 97<br>"]
    pub fn scs_nvic_ipr2445c_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 24) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 98<br>"]
    pub(crate) fn scs_nvic_ipr2445c_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 24) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 98<br>"]
    pub fn scs_nvic_ipr2445c_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 24) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 99<br>"]
    pub(crate) fn scs_nvic_ipr2445c_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 24) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 99<br>"]
    pub fn scs_nvic_ipr2445c_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 24) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 100<br>"]
    pub(crate) fn scs_nvic_ipr25460_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 25) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 100<br>"]
    pub fn scs_nvic_ipr25460_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 25) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 101<br>"]
    pub(crate) fn scs_nvic_ipr25460_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 25) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 101<br>"]
    pub fn scs_nvic_ipr25460_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 25) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 102<br>"]
    pub(crate) fn scs_nvic_ipr25460_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 25) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 102<br>"]
    pub fn scs_nvic_ipr25460_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 25) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 103<br>"]
    pub(crate) fn scs_nvic_ipr25460_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 25) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 103<br>"]
    pub fn scs_nvic_ipr25460_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 25) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 104<br>"]
    pub(crate) fn scs_nvic_ipr26464_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 26) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 104<br>"]
    pub fn scs_nvic_ipr26464_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 26) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 105<br>"]
    pub(crate) fn scs_nvic_ipr26464_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 26) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 105<br>"]
    pub fn scs_nvic_ipr26464_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 26) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 106<br>"]
    pub(crate) fn scs_nvic_ipr26464_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 26) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 106<br>"]
    pub fn scs_nvic_ipr26464_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 26) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 107<br>"]
    pub(crate) fn scs_nvic_ipr26464_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 26) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 107<br>"]
    pub fn scs_nvic_ipr26464_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 26) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 108<br>"]
    pub(crate) fn scs_nvic_ipr27468_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 27) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 108<br>"]
    pub fn scs_nvic_ipr27468_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 27) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 109<br>"]
    pub(crate) fn scs_nvic_ipr27468_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 27) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 109<br>"]
    pub fn scs_nvic_ipr27468_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 27) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 110<br>"]
    pub(crate) fn scs_nvic_ipr27468_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 27) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 110<br>"]
    pub fn scs_nvic_ipr27468_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 27) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 111<br>"]
    pub(crate) fn scs_nvic_ipr27468_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 27) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 111<br>"]
    pub fn scs_nvic_ipr27468_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 27) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 112<br>"]
    pub(crate) fn scs_nvic_ipr2846c_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 28) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 112<br>"]
    pub fn scs_nvic_ipr2846c_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 28) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 113<br>"]
    pub(crate) fn scs_nvic_ipr2846c_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 28) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 113<br>"]
    pub fn scs_nvic_ipr2846c_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 28) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 114<br>"]
    pub(crate) fn scs_nvic_ipr2846c_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 28) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 114<br>"]
    pub fn scs_nvic_ipr2846c_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 28) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 115<br>"]
    pub(crate) fn scs_nvic_ipr2846c_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 28) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 115<br>"]
    pub fn scs_nvic_ipr2846c_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 28) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 116<br>"]
    pub(crate) fn scs_nvic_ipr29470_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 29) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 116<br>"]
    pub fn scs_nvic_ipr29470_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 29) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 117<br>"]
    pub(crate) fn scs_nvic_ipr29470_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 29) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 117<br>"]
    pub fn scs_nvic_ipr29470_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 29) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 118<br>"]
    pub(crate) fn scs_nvic_ipr29470_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 29) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 118<br>"]
    pub fn scs_nvic_ipr29470_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 29) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 119<br>"]
    pub(crate) fn scs_nvic_ipr29470_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 29) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 119<br>"]
    pub fn scs_nvic_ipr29470_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 29) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 120<br>"]
    pub(crate) fn scs_nvic_ipr30474_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 30) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 120<br>"]
    pub fn scs_nvic_ipr30474_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 30) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 121<br>"]
    pub(crate) fn scs_nvic_ipr30474_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 30) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 121<br>"]
    pub fn scs_nvic_ipr30474_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 30) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 122<br>"]
    pub(crate) fn scs_nvic_ipr30474_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 30) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 122<br>"]
    pub fn scs_nvic_ipr30474_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 30) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 123<br>"]
    pub(crate) fn scs_nvic_ipr30474_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 30) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 123<br>"]
    pub fn scs_nvic_ipr30474_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 30) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 124<br>"]
    pub(crate) fn scs_nvic_ipr31478_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 31) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 124<br>"]
    pub fn scs_nvic_ipr31478_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 31) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 125<br>"]
    pub(crate) fn scs_nvic_ipr31478_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 31) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 125<br>"]
    pub fn scs_nvic_ipr31478_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 31) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 126<br>"]
    pub(crate) fn scs_nvic_ipr31478_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 31) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 126<br>"]
    pub fn scs_nvic_ipr31478_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 31) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 127<br>"]
    pub(crate) fn scs_nvic_ipr31478_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 31) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 127<br>"]
    pub fn scs_nvic_ipr31478_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 31) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 128<br>"]
    pub(crate) fn scs_nvic_ipr3247c_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 32) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 128<br>"]
    pub fn scs_nvic_ipr3247c_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 32) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 129<br>"]
    pub(crate) fn scs_nvic_ipr3247c_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 32) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 129<br>"]
    pub fn scs_nvic_ipr3247c_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 32) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 130<br>"]
    pub(crate) fn scs_nvic_ipr3247c_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 32) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 130<br>"]
    pub fn scs_nvic_ipr3247c_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 32) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 131<br>"]
    pub(crate) fn scs_nvic_ipr3247c_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 32) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 131<br>"]
    pub fn scs_nvic_ipr3247c_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 32) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 132<br>"]
    pub(crate) fn scs_nvic_ipr33480_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 33) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 132<br>"]
    pub fn scs_nvic_ipr33480_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 33) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 133<br>"]
    pub(crate) fn scs_nvic_ipr33480_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 33) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 133<br>"]
    pub fn scs_nvic_ipr33480_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 33) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 134<br>"]
    pub(crate) fn scs_nvic_ipr33480_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 33) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 134<br>"]
    pub fn scs_nvic_ipr33480_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 33) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 135<br>"]
    pub(crate) fn scs_nvic_ipr33480_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 33) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 135<br>"]
    pub fn scs_nvic_ipr33480_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 33) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 136<br>"]
    pub(crate) fn scs_nvic_ipr34484_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 34) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 136<br>"]
    pub fn scs_nvic_ipr34484_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 34) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 137<br>"]
    pub(crate) fn scs_nvic_ipr34484_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 34) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 137<br>"]
    pub fn scs_nvic_ipr34484_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 34) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 138<br>"]
    pub(crate) fn scs_nvic_ipr34484_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 34) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 138<br>"]
    pub fn scs_nvic_ipr34484_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 34) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 139<br>"]
    pub(crate) fn scs_nvic_ipr34484_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 34) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 139<br>"]
    pub fn scs_nvic_ipr34484_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 34) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 140<br>"]
    pub(crate) fn scs_nvic_ipr35488_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 35) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 140<br>"]
    pub fn scs_nvic_ipr35488_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 35) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 141<br>"]
    pub(crate) fn scs_nvic_ipr35488_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 35) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 141<br>"]
    pub fn scs_nvic_ipr35488_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 35) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 142<br>"]
    pub(crate) fn scs_nvic_ipr35488_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 35) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 142<br>"]
    pub fn scs_nvic_ipr35488_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 35) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 143<br>"]
    pub(crate) fn scs_nvic_ipr35488_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 35) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 143<br>"]
    pub fn scs_nvic_ipr35488_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 35) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 144<br>"]
    pub(crate) fn scs_nvic_ipr3648c_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 36) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 144<br>"]
    pub fn scs_nvic_ipr3648c_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 36) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 145<br>"]
    pub(crate) fn scs_nvic_ipr3648c_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 36) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 145<br>"]
    pub fn scs_nvic_ipr3648c_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 36) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 146<br>"]
    pub(crate) fn scs_nvic_ipr3648c_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 36) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 146<br>"]
    pub fn scs_nvic_ipr3648c_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 36) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 147<br>"]
    pub(crate) fn scs_nvic_ipr3648c_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 36) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 147<br>"]
    pub fn scs_nvic_ipr3648c_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 36) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 148<br>"]
    pub(crate) fn scs_nvic_ipr37490_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 37) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 148<br>"]
    pub fn scs_nvic_ipr37490_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 37) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 149<br>"]
    pub(crate) fn scs_nvic_ipr37490_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 37) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 149<br>"]
    pub fn scs_nvic_ipr37490_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 37) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 150<br>"]
    pub(crate) fn scs_nvic_ipr37490_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 37) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 150<br>"]
    pub fn scs_nvic_ipr37490_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 37) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 151<br>"]
    pub(crate) fn scs_nvic_ipr37490_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 37) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 151<br>"]
    pub fn scs_nvic_ipr37490_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 37) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 152<br>"]
    pub(crate) fn scs_nvic_ipr38494_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 38) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 152<br>"]
    pub fn scs_nvic_ipr38494_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 38) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 153<br>"]
    pub(crate) fn scs_nvic_ipr38494_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 38) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 153<br>"]
    pub fn scs_nvic_ipr38494_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 38) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 154<br>"]
    pub(crate) fn scs_nvic_ipr38494_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 38) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 154<br>"]
    pub fn scs_nvic_ipr38494_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 38) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 155<br>"]
    pub(crate) fn scs_nvic_ipr38494_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 38) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 155<br>"]
    pub fn scs_nvic_ipr38494_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 38) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 156<br>"]
    pub(crate) fn scs_nvic_ipr39498_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 39) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 156<br>"]
    pub fn scs_nvic_ipr39498_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 39) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 157<br>"]
    pub(crate) fn scs_nvic_ipr39498_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 39) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 157<br>"]
    pub fn scs_nvic_ipr39498_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 39) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 158<br>"]
    pub(crate) fn scs_nvic_ipr39498_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 39) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 158<br>"]
    pub fn scs_nvic_ipr39498_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 39) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 159<br>"]
    pub(crate) fn scs_nvic_ipr39498_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 39) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 159<br>"]
    pub fn scs_nvic_ipr39498_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 39) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 160<br>"]
    pub(crate) fn scs_nvic_ipr4049c_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 40) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 160<br>"]
    pub fn scs_nvic_ipr4049c_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 40) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 161<br>"]
    pub(crate) fn scs_nvic_ipr4049c_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 40) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 161<br>"]
    pub fn scs_nvic_ipr4049c_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 40) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 162<br>"]
    pub(crate) fn scs_nvic_ipr4049c_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 40) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 162<br>"]
    pub fn scs_nvic_ipr4049c_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 40) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 163<br>"]
    pub(crate) fn scs_nvic_ipr4049c_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 40) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 163<br>"]
    pub fn scs_nvic_ipr4049c_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 40) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 164<br>"]
    pub(crate) fn scs_nvic_ipr414a0_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 41) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 164<br>"]
    pub fn scs_nvic_ipr414a0_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 41) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 165<br>"]
    pub(crate) fn scs_nvic_ipr414a0_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 41) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 165<br>"]
    pub fn scs_nvic_ipr414a0_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 41) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 166<br>"]
    pub(crate) fn scs_nvic_ipr414a0_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 41) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 166<br>"]
    pub fn scs_nvic_ipr414a0_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 41) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 167<br>"]
    pub(crate) fn scs_nvic_ipr414a0_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 41) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 167<br>"]
    pub fn scs_nvic_ipr414a0_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 41) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 168<br>"]
    pub(crate) fn scs_nvic_ipr424a4_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 42) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 168<br>"]
    pub fn scs_nvic_ipr424a4_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 42) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 169<br>"]
    pub(crate) fn scs_nvic_ipr424a4_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 42) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 169<br>"]
    pub fn scs_nvic_ipr424a4_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 42) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 170<br>"]
    pub(crate) fn scs_nvic_ipr424a4_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 42) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 170<br>"]
    pub fn scs_nvic_ipr424a4_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 42) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 171<br>"]
    pub(crate) fn scs_nvic_ipr424a4_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 42) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 171<br>"]
    pub fn scs_nvic_ipr424a4_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 42) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 172<br>"]
    pub(crate) fn scs_nvic_ipr434a8_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 43) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 172<br>"]
    pub fn scs_nvic_ipr434a8_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 43) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 173<br>"]
    pub(crate) fn scs_nvic_ipr434a8_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 43) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 173<br>"]
    pub fn scs_nvic_ipr434a8_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 43) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 174<br>"]
    pub(crate) fn scs_nvic_ipr434a8_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 43) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 174<br>"]
    pub fn scs_nvic_ipr434a8_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 43) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 175<br>"]
    pub(crate) fn scs_nvic_ipr434a8_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 43) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 175<br>"]
    pub fn scs_nvic_ipr434a8_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 43) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 176<br>"]
    pub(crate) fn scs_nvic_ipr444ac_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 44) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 176<br>"]
    pub fn scs_nvic_ipr444ac_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 44) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 177<br>"]
    pub(crate) fn scs_nvic_ipr444ac_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 44) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 177<br>"]
    pub fn scs_nvic_ipr444ac_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 44) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 178<br>"]
    pub(crate) fn scs_nvic_ipr444ac_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 44) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 178<br>"]
    pub fn scs_nvic_ipr444ac_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 44) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 179<br>"]
    pub(crate) fn scs_nvic_ipr444ac_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 44) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 179<br>"]
    pub fn scs_nvic_ipr444ac_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 44) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 180<br>"]
    pub(crate) fn scs_nvic_ipr454b0_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 45) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 180<br>"]
    pub fn scs_nvic_ipr454b0_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 45) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 181<br>"]
    pub(crate) fn scs_nvic_ipr454b0_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 45) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 181<br>"]
    pub fn scs_nvic_ipr454b0_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 45) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 182<br>"]
    pub(crate) fn scs_nvic_ipr454b0_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 45) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 182<br>"]
    pub fn scs_nvic_ipr454b0_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 45) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 183<br>"]
    pub(crate) fn scs_nvic_ipr454b0_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 45) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 183<br>"]
    pub fn scs_nvic_ipr454b0_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 45) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 184<br>"]
    pub(crate) fn scs_nvic_ipr464b4_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 46) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 184<br>"]
    pub fn scs_nvic_ipr464b4_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 46) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 185<br>"]
    pub(crate) fn scs_nvic_ipr464b4_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 46) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 185<br>"]
    pub fn scs_nvic_ipr464b4_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 46) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 186<br>"]
    pub(crate) fn scs_nvic_ipr464b4_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 46) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 186<br>"]
    pub fn scs_nvic_ipr464b4_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 46) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 187<br>"]
    pub(crate) fn scs_nvic_ipr464b4_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 46) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 187<br>"]
    pub fn scs_nvic_ipr464b4_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 46) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 188<br>"]
    pub(crate) fn scs_nvic_ipr474b8_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 47) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 188<br>"]
    pub fn scs_nvic_ipr474b8_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 47) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 189<br>"]
    pub(crate) fn scs_nvic_ipr474b8_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 47) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 189<br>"]
    pub fn scs_nvic_ipr474b8_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 47) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 190<br>"]
    pub(crate) fn scs_nvic_ipr474b8_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 47) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 190<br>"]
    pub fn scs_nvic_ipr474b8_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 47) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 191<br>"]
    pub(crate) fn scs_nvic_ipr474b8_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 47) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 191<br>"]
    pub fn scs_nvic_ipr474b8_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 47) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 192<br>"]
    pub(crate) fn scs_nvic_ipr484bc_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 48) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 192<br>"]
    pub fn scs_nvic_ipr484bc_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 48) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 193<br>"]
    pub(crate) fn scs_nvic_ipr484bc_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 48) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 193<br>"]
    pub fn scs_nvic_ipr484bc_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 48) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 194<br>"]
    pub(crate) fn scs_nvic_ipr484bc_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 48) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 194<br>"]
    pub fn scs_nvic_ipr484bc_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 48) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 195<br>"]
    pub(crate) fn scs_nvic_ipr484bc_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 48) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 195<br>"]
    pub fn scs_nvic_ipr484bc_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 48) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 196<br>"]
    pub(crate) fn scs_nvic_ipr494c0_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 49) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 196<br>"]
    pub fn scs_nvic_ipr494c0_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 49) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 197<br>"]
    pub(crate) fn scs_nvic_ipr494c0_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 49) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 197<br>"]
    pub fn scs_nvic_ipr494c0_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 49) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 198<br>"]
    pub(crate) fn scs_nvic_ipr494c0_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 49) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 198<br>"]
    pub fn scs_nvic_ipr494c0_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 49) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 199<br>"]
    pub(crate) fn scs_nvic_ipr494c0_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 49) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 199<br>"]
    pub fn scs_nvic_ipr494c0_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 49) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 200<br>"]
    pub(crate) fn scs_nvic_ipr504c4_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 50) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 200<br>"]
    pub fn scs_nvic_ipr504c4_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 50) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 201<br>"]
    pub(crate) fn scs_nvic_ipr504c4_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 50) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 201<br>"]
    pub fn scs_nvic_ipr504c4_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 50) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 202<br>"]
    pub(crate) fn scs_nvic_ipr504c4_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 50) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 202<br>"]
    pub fn scs_nvic_ipr504c4_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 50) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 203<br>"]
    pub(crate) fn scs_nvic_ipr504c4_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 50) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 203<br>"]
    pub fn scs_nvic_ipr504c4_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 50) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 204<br>"]
    pub(crate) fn scs_nvic_ipr514c8_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 51) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 204<br>"]
    pub fn scs_nvic_ipr514c8_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 51) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 205<br>"]
    pub(crate) fn scs_nvic_ipr514c8_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 51) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 205<br>"]
    pub fn scs_nvic_ipr514c8_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 51) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 206<br>"]
    pub(crate) fn scs_nvic_ipr514c8_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 51) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 206<br>"]
    pub fn scs_nvic_ipr514c8_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 51) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 207<br>"]
    pub(crate) fn scs_nvic_ipr514c8_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 51) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 207<br>"]
    pub fn scs_nvic_ipr514c8_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 51) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 208<br>"]
    pub(crate) fn scs_nvic_ipr524cc_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 52) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 208<br>"]
    pub fn scs_nvic_ipr524cc_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 52) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 209<br>"]
    pub(crate) fn scs_nvic_ipr524cc_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 52) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 209<br>"]
    pub fn scs_nvic_ipr524cc_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 52) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 210<br>"]
    pub(crate) fn scs_nvic_ipr524cc_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 52) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 210<br>"]
    pub fn scs_nvic_ipr524cc_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 52) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 211<br>"]
    pub(crate) fn scs_nvic_ipr524cc_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 52) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 211<br>"]
    pub fn scs_nvic_ipr524cc_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 52) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 212<br>"]
    pub(crate) fn scs_nvic_ipr534d0_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 53) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 212<br>"]
    pub fn scs_nvic_ipr534d0_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 53) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 213<br>"]
    pub(crate) fn scs_nvic_ipr534d0_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 53) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 213<br>"]
    pub fn scs_nvic_ipr534d0_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 53) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 214<br>"]
    pub(crate) fn scs_nvic_ipr534d0_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 53) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 214<br>"]
    pub fn scs_nvic_ipr534d0_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 53) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 215<br>"]
    pub(crate) fn scs_nvic_ipr534d0_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 53) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 215<br>"]
    pub fn scs_nvic_ipr534d0_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 53) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 216<br>"]
    pub(crate) fn scs_nvic_ipr544d4_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 54) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 216<br>"]
    pub fn scs_nvic_ipr544d4_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 54) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 217<br>"]
    pub(crate) fn scs_nvic_ipr544d4_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 54) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 217<br>"]
    pub fn scs_nvic_ipr544d4_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 54) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 218<br>"]
    pub(crate) fn scs_nvic_ipr544d4_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 54) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 218<br>"]
    pub fn scs_nvic_ipr544d4_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 54) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 219<br>"]
    pub(crate) fn scs_nvic_ipr544d4_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 54) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 219<br>"]
    pub fn scs_nvic_ipr544d4_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 54) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 220<br>"]
    pub(crate) fn scs_nvic_ipr554d8_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 55) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 220<br>"]
    pub fn scs_nvic_ipr554d8_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 55) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 221<br>"]
    pub(crate) fn scs_nvic_ipr554d8_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 55) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 221<br>"]
    pub fn scs_nvic_ipr554d8_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 55) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 222<br>"]
    pub(crate) fn scs_nvic_ipr554d8_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 55) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 222<br>"]
    pub fn scs_nvic_ipr554d8_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 55) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 223<br>"]
    pub(crate) fn scs_nvic_ipr554d8_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 55) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 223<br>"]
    pub fn scs_nvic_ipr554d8_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 55) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 224<br>"]
    pub(crate) fn scs_nvic_ipr564dc_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 56) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 224<br>"]
    pub fn scs_nvic_ipr564dc_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 56) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 225<br>"]
    pub(crate) fn scs_nvic_ipr564dc_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 56) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 225<br>"]
    pub fn scs_nvic_ipr564dc_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 56) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 226<br>"]
    pub(crate) fn scs_nvic_ipr564dc_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 56) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 226<br>"]
    pub fn scs_nvic_ipr564dc_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 56) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 227<br>"]
    pub(crate) fn scs_nvic_ipr564dc_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 56) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 227<br>"]
    pub fn scs_nvic_ipr564dc_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 56) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 228<br>"]
    pub(crate) fn scs_nvic_ipr574e0_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 57) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 228<br>"]
    pub fn scs_nvic_ipr574e0_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 57) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 229<br>"]
    pub(crate) fn scs_nvic_ipr574e0_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 57) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 229<br>"]
    pub fn scs_nvic_ipr574e0_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 57) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 230<br>"]
    pub(crate) fn scs_nvic_ipr574e0_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 57) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 230<br>"]
    pub fn scs_nvic_ipr574e0_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 57) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 231<br>"]
    pub(crate) fn scs_nvic_ipr574e0_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 57) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 231<br>"]
    pub fn scs_nvic_ipr574e0_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 57) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 232<br>"]
    pub(crate) fn scs_nvic_ipr584e4_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 58) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 232<br>"]
    pub fn scs_nvic_ipr584e4_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 58) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 233<br>"]
    pub(crate) fn scs_nvic_ipr584e4_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 58) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 233<br>"]
    pub fn scs_nvic_ipr584e4_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 58) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 234<br>"]
    pub(crate) fn scs_nvic_ipr584e4_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 58) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 234<br>"]
    pub fn scs_nvic_ipr584e4_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 58) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 235<br>"]
    pub(crate) fn scs_nvic_ipr584e4_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 58) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 235<br>"]
    pub fn scs_nvic_ipr584e4_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 58) + 3] = _value)
    }
    #[doc = "PRI_N0: Priority of interrupt 236<br>"]
    pub(crate) fn scs_nvic_ipr594e8_pri_n0_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 59) + 0])
    }
    #[doc = "PRI_N0: Priority of interrupt 236<br>"]
    pub fn scs_nvic_ipr594e8_pri_n0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 59) + 0] = _value)
    }
    #[doc = "PRI_N1: Priority of interrupt 237<br>"]
    pub(crate) fn scs_nvic_ipr594e8_pri_n1_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 59) + 1])
    }
    #[doc = "PRI_N1: Priority of interrupt 237<br>"]
    pub fn scs_nvic_ipr594e8_pri_n1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 59) + 1] = _value)
    }
    #[doc = "PRI_N2: Priority of interrupt 238<br>"]
    pub(crate) fn scs_nvic_ipr594e8_pri_n2_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 59) + 2])
    }
    #[doc = "PRI_N2: Priority of interrupt 238<br>"]
    pub fn scs_nvic_ipr594e8_pri_n2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 59) + 2] = _value)
    }
    #[doc = "PRI_N3: Priority of interrupt 239<br>"]
    pub(crate) fn scs_nvic_ipr594e8_pri_n3_read(&self) -> MemResult<u8> {
        Ok(self.interrupts_priority[(4 * 59) + 3])
    }
    #[doc = "PRI_N3: Priority of interrupt 239<br>"]
    pub fn scs_nvic_ipr594e8_pri_n3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.interrupts_priority[(4 * 59) + 3] = _value)
    }
    #[doc = "VECTACTIVE: The exception number for the current executing exception<br>"]
    pub(crate) fn scs_icsrcfc_vectactive_read(&self) -> MemResult<u16> {
        todo ! ("read VECTACTIVE mwrite None write None rac None reset value 0x00 mask 0x1ff")
    }
    #[doc = "VECTACTIVE: The exception number for the current executing exception<br>"]
    pub(crate) fn scs_icsrcfc_vectactive_write(
        &mut self,
        _value: u16,
    ) -> MemResult<()> {
        todo ! ("write VECTACTIVE mwrite None write None rac None reset value 0x00 mask 0x1ff")
    }
    #[doc = "RETTOBASE: In Handler mode, indicates whether there is an active exception other than the exception indicated by the current value of the IPSR<br>"]
    pub(crate) fn scs_icsrcfc_rettobase_read(&self) -> MemResult<bool> {
        todo!(
            "read RETTOBASE mwrite None write None rac None reset value false"
        )
    }
    #[doc = "RETTOBASE: In Handler mode, indicates whether there is an active exception other than the exception indicated by the current value of the IPSR<br>"]
    pub(crate) fn scs_icsrcfc_rettobase_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write RETTOBASE mwrite None write None rac None reset value false"
        )
    }
    #[doc = "VECTPENDING: The exception number for the highest priority pending exception. 0 indicates no pending exceptions<br>"]
    pub(crate) fn scs_icsrcfc_vectpending_read(&self) -> MemResult<u16> {
        todo ! ("read VECTPENDING mwrite None write None rac None reset value 0x00 mask 0x1ff")
    }
    #[doc = "VECTPENDING: The exception number for the highest priority pending exception. 0 indicates no pending exceptions<br>"]
    pub(crate) fn scs_icsrcfc_vectpending_write(
        &mut self,
        _value: u16,
    ) -> MemResult<()> {
        todo ! ("write VECTPENDING mwrite None write None rac None reset value 0x00 mask 0x1ff")
    }
    #[doc = "ISRPENDING: Indicates if an external configurable, NVIC generated, interrupt is pending<br>"]
    pub(crate) fn scs_icsrcfc_isrpending_read(&self) -> MemResult<bool> {
        todo!(
            "read ISRPENDING mwrite None write None rac None reset value false"
        )
    }
    #[doc = "ISRPENDING: Indicates if an external configurable, NVIC generated, interrupt is pending<br>"]
    pub(crate) fn scs_icsrcfc_isrpending_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write ISRPENDING mwrite None write None rac None reset value false")
    }
    #[doc = "ISRPREEMPT: Indicates whether a pending exception will be serviced on exit from debug halt state<br>"]
    pub(crate) fn scs_icsrcfc_isrpreempt_read(&self) -> MemResult<bool> {
        todo!(
            "read ISRPREEMPT mwrite None write None rac None reset value false"
        )
    }
    #[doc = "ISRPREEMPT: Indicates whether a pending exception will be serviced on exit from debug halt state<br>"]
    pub(crate) fn scs_icsrcfc_isrpreempt_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write ISRPREEMPT mwrite None write None rac None reset value false")
    }
    #[doc = "PENDSTCLR: Clears a pending SysTick, whether set here or by the timer hardware<br>"]
    pub(crate) fn scs_icsrcfc_pendstclr_read(&self) -> MemResult<bool> {
        todo!(
            "read PENDSTCLR mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PENDSTCLR: Clears a pending SysTick, whether set here or by the timer hardware<br>"]
    pub(crate) fn scs_icsrcfc_pendstclr_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write PENDSTCLR mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PENDSTSET: Sets a pending SysTick or reads back the current state<br>"]
    pub(crate) fn scs_icsrcfc_pendstset_read(&self) -> MemResult<bool> {
        todo!(
            "read PENDSTSET mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PENDSTSET: Sets a pending SysTick or reads back the current state<br>"]
    pub(crate) fn scs_icsrcfc_pendstset_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write PENDSTSET mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PENDSVCLR: Clears a pending PendSV interrupt<br>"]
    pub(crate) fn scs_icsrcfc_pendsvclr_read(&self) -> MemResult<bool> {
        todo!(
            "read PENDSVCLR mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PENDSVCLR: Clears a pending PendSV interrupt<br>"]
    pub(crate) fn scs_icsrcfc_pendsvclr_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write PENDSVCLR mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PENDSVSET: Sets a pending PendSV interrupt or reads back the current state<br>"]
    pub(crate) fn scs_icsrcfc_pendsvset_read(&self) -> MemResult<bool> {
        todo!(
            "read PENDSVSET mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PENDSVSET: Sets a pending PendSV interrupt or reads back the current state<br>"]
    pub(crate) fn scs_icsrcfc_pendsvset_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write PENDSVSET mwrite None write None rac None reset value false"
        )
    }
    #[doc = "NMIPENDSET: Activates an NMI exception or reads back the current state<br>"]
    pub(crate) fn scs_icsrcfc_nmipendset_read(&self) -> MemResult<bool> {
        todo!(
            "read NMIPENDSET mwrite None write None rac None reset value false"
        )
    }
    #[doc = "NMIPENDSET: Activates an NMI exception or reads back the current state<br>"]
    pub(crate) fn scs_icsrcfc_nmipendset_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write NMIPENDSET mwrite None write None rac None reset value false")
    }
    #[doc = "TBLOFF: Bits \\[31:7\\] of the vector table address<br>"]
    pub(crate) fn scs_vtord00_tbloff_read(&self) -> MemResult<u32> {
        todo ! ("read TBLOFF mwrite None write None rac None reset value 0x00 mask 0x1ffffff")
    }
    #[doc = "TBLOFF: Bits \\[31:7\\] of the vector table address<br>"]
    pub(crate) fn scs_vtord00_tbloff_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write TBLOFF mwrite None write None rac None reset value 0x00 mask 0x1ffffff")
    }
    #[doc = "TBLBASE: Determines whether the vector table is in the code or SRAM memory region<br>"]
    pub(crate) fn scs_vtord00_tblbase_read(&self) -> MemResult<bool> {
        todo!("read TBLBASE mwrite None write None rac None reset value false")
    }
    #[doc = "TBLBASE: Determines whether the vector table is in the code or SRAM memory region<br>"]
    pub(crate) fn scs_vtord00_tblbase_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write TBLBASE mwrite None write None rac None reset value false")
    }
    #[doc = "VECTCLRACTIVE: Clears all active state information for fixed and configurable exceptions<br>"]
    pub(crate) fn scs_aircrd04_vectclractive_read(&self) -> MemResult<bool> {
        todo ! ("read VECTCLRACTIVE mwrite None write None rac None reset value false")
    }
    #[doc = "VECTCLRACTIVE: Clears all active state information for fixed and configurable exceptions<br>"]
    pub(crate) fn scs_aircrd04_vectclractive_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write VECTCLRACTIVE mwrite None write None rac None reset value false")
    }
    #[doc = "SYSRESETREQ: System Reset Request<br>"]
    pub(crate) fn scs_aircrd04_sysresetreq_read(&self) -> MemResult<bool> {
        todo ! ("read SYSRESETREQ mwrite None write None rac None reset value false")
    }
    #[doc = "SYSRESETREQ: System Reset Request<br>"]
    pub(crate) fn scs_aircrd04_sysresetreq_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write SYSRESETREQ mwrite None write None rac None reset value false")
    }
    #[doc = "PRIGROUP: Priority grouping, indicates the    binary point position.<br>"]
    pub(crate) fn scs_aircrd04_prigroup_read(&self) -> MemResult<u8> {
        todo ! ("read PRIGROUP mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "PRIGROUP: Priority grouping, indicates the    binary point position.<br>"]
    pub(crate) fn scs_aircrd04_prigroup_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write PRIGROUP mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "ENDIANNESS: Indicates the memory system data endianness<br>"]
    pub(crate) fn scs_aircrd04_endianness_read(&self) -> MemResult<bool> {
        todo!(
            "read ENDIANNESS mwrite None write None rac None reset value false"
        )
    }
    #[doc = "ENDIANNESS: Indicates the memory system data endianness<br>"]
    pub(crate) fn scs_aircrd04_endianness_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write ENDIANNESS mwrite None write None rac None reset value false")
    }
    #[doc = "VECTKEY: Vector Key<br>VECTKEYSTAT: UNKNOWN<br>"]
    pub(crate) fn scs_aircrd04_vectkey_read(&self) -> MemResult<u16> {
        todo ! ("read VECTKEY, VECTKEYSTAT mwrite None write None rac None reset value 0x00 mask 0xffff")
    }
    #[doc = "VECTKEY: Vector Key<br>VECTKEYSTAT: UNKNOWN<br>"]
    pub(crate) fn scs_aircrd04_vectkey_write(
        &mut self,
        _value: u16,
    ) -> MemResult<()> {
        todo ! ("write VECTKEY, VECTKEYSTAT mwrite None write None rac None reset value 0x00 mask 0xffff")
    }
    #[doc = "SLEEPONEXIT: whether, on an exit from an ISR that returns to the base level of execution priority, the processor enters a sleep state<br>"]
    pub(crate) fn scs_scrd08_sleeponexit_read(&self) -> MemResult<bool> {
        Ok(self.sleep_on_exit)
    }
    #[doc = "SLEEPONEXIT: whether, on an exit from an ISR that returns to the base level of execution priority, the processor enters a sleep state<br>"]
    pub(crate) fn scs_scrd08_sleeponexit_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.sleep_on_exit = _value)
    }
    #[doc = "SLEEPDEEP: Hint indicating that waking from sleep might take longer<br>"]
    pub(crate) fn scs_scrd08_sleepdeep_read(&self) -> MemResult<bool> {
        Ok(self.sleep_deep)
    }
    #[doc = "SLEEPDEEP: Hint indicating that waking from sleep might take longer<br>"]
    pub(crate) fn scs_scrd08_sleepdeep_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.sleep_deep = _value)
    }
    #[doc = "SEVONPEND: Determines whether an interrupt transition from inactive state to pending state is a wakeup event<br>"]
    pub(crate) fn scs_scrd08_sevonpend_read(&self) -> MemResult<bool> {
        Ok(self.event_on_pending)
    }
    #[doc = "SEVONPEND: Determines whether an interrupt transition from inactive state to pending state is a wakeup event<br>"]
    pub(crate) fn scs_scrd08_sevonpend_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.event_on_pending = _value)
    }
    #[doc = "NONBASETHRDENA: Controls whether the processor can enter Thread mode at an execution priority level other than base level<br>"]
    pub(crate) fn scs_ccrd0c_nonbasethrdena_read(&self) -> MemResult<bool> {
        todo ! ("read NONBASETHRDENA mwrite None write None rac None reset value false")
    }
    #[doc = "NONBASETHRDENA: Controls whether the processor can enter Thread mode at an execution priority level other than base level<br>"]
    pub(crate) fn scs_ccrd0c_nonbasethrdena_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write NONBASETHRDENA mwrite None write None rac None reset value false")
    }
    #[doc = "USERSETMPEND: Controls whether unprivileged software can access the STIR<br>"]
    pub(crate) fn scs_ccrd0c_usersetmpend_read(&self) -> MemResult<bool> {
        todo ! ("read USERSETMPEND mwrite None write None rac None reset value false")
    }
    #[doc = "USERSETMPEND: Controls whether unprivileged software can access the STIR<br>"]
    pub(crate) fn scs_ccrd0c_usersetmpend_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write USERSETMPEND mwrite None write None rac None reset value false")
    }
    #[doc = "UNALIGN_TRP: Controls the trapping of unaligned word or halfword accesses<br>"]
    pub(crate) fn scs_ccrd0c_unalign_trp_read(&self) -> MemResult<bool> {
        todo ! ("read UNALIGN_TRP mwrite None write None rac None reset value false")
    }
    #[doc = "UNALIGN_TRP: Controls the trapping of unaligned word or halfword accesses<br>"]
    pub(crate) fn scs_ccrd0c_unalign_trp_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write UNALIGN_TRP mwrite None write None rac None reset value false")
    }
    #[doc = "DIV_0_TRP: Controls the trap on divide by 0<br>"]
    pub(crate) fn scs_ccrd0c_div_0_trp_read(&self) -> MemResult<bool> {
        todo!(
            "read DIV_0_TRP mwrite None write None rac None reset value false"
        )
    }
    #[doc = "DIV_0_TRP: Controls the trap on divide by 0<br>"]
    pub(crate) fn scs_ccrd0c_div_0_trp_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write DIV_0_TRP mwrite None write None rac None reset value false"
        )
    }
    #[doc = "BFHFNMIGN: Determines the effect of precise data access faults on handlers running at priority -1 or priority -2<br>"]
    pub(crate) fn scs_ccrd0c_bfhfnmign_read(&self) -> MemResult<bool> {
        todo!(
            "read BFHFNMIGN mwrite None write None rac None reset value false"
        )
    }
    #[doc = "BFHFNMIGN: Determines the effect of precise data access faults on handlers running at priority -1 or priority -2<br>"]
    pub(crate) fn scs_ccrd0c_bfhfnmign_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write BFHFNMIGN mwrite None write None rac None reset value false"
        )
    }
    #[doc = "STKALIGN: Determines whether the exception entry sequence guarantees 8-byte stack frame alignment, adjusting the SP if necessary before saving state<br>"]
    pub(crate) fn scs_ccrd0c_stkalign_read(&self) -> MemResult<bool> {
        todo!("read STKALIGN mwrite None write None rac None reset value false")
    }
    #[doc = "STKALIGN: Determines whether the exception entry sequence guarantees 8-byte stack frame alignment, adjusting the SP if necessary before saving state<br>"]
    pub(crate) fn scs_ccrd0c_stkalign_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write STKALIGN mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PRI_4: Priority of system handler 4, MemManage<br>"]
    pub(crate) fn scs_shpr1d10_pri_4_read(&self) -> MemResult<u8> {
        Ok(self.priorities[SysHandlerPriority::Pri4 as usize])
    }
    #[doc = "PRI_4: Priority of system handler 4, MemManage<br>"]
    pub(crate) fn scs_shpr1d10_pri_4_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.priorities[SysHandlerPriority::Pri4 as usize] = _value)
    }
    #[doc = "PRI_5: Priority of system handler 5, BusFault<br>"]
    pub(crate) fn scs_shpr1d10_pri_5_read(&self) -> MemResult<u8> {
        Ok(self.priorities[SysHandlerPriority::Pri5 as usize])
    }
    #[doc = "PRI_5: Priority of system handler 5, BusFault<br>"]
    pub(crate) fn scs_shpr1d10_pri_5_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.priorities[SysHandlerPriority::Pri5 as usize] = _value)
    }
    #[doc = "PRI_6: Priority of system handler 6, UsageFault<br>"]
    pub(crate) fn scs_shpr1d10_pri_6_read(&self) -> MemResult<u8> {
        Ok(self.priorities[SysHandlerPriority::Pri6 as usize])
    }
    #[doc = "PRI_6: Priority of system handler 6, UsageFault<br>"]
    pub(crate) fn scs_shpr1d10_pri_6_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.priorities[SysHandlerPriority::Pri6 as usize] = _value)
    }
    #[doc = "PRI_7: Priority of system handler 7<br>"]
    pub(crate) fn scs_shpr1d10_pri_7_read(&self) -> MemResult<u8> {
        Ok(self.priorities[SysHandlerPriority::Pri7 as usize])
    }
    #[doc = "PRI_7: Priority of system handler 7<br>"]
    pub fn scs_shpr1d10_pri_7_write(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.priorities[SysHandlerPriority::Pri7 as usize] = _value)
    }
    #[doc = "PRI_8: Priority of system handler 8<br>"]
    pub(crate) fn scs_shpr2d14_pri_8_read(&self) -> MemResult<u8> {
        Ok(self.priorities[SysHandlerPriority::Pri8 as usize])
    }
    #[doc = "PRI_8: Priority of system handler 8<br>"]
    pub fn scs_shpr2d14_pri_8_write(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.priorities[SysHandlerPriority::Pri8 as usize] = _value)
    }
    #[doc = "PRI_9: Priority of system handler 9<br>"]
    pub(crate) fn scs_shpr2d14_pri_9_read(&self) -> MemResult<u8> {
        Ok(self.priorities[SysHandlerPriority::Pri9 as usize])
    }
    #[doc = "PRI_9: Priority of system handler 9<br>"]
    pub fn scs_shpr2d14_pri_9_write(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.priorities[SysHandlerPriority::Pri9 as usize] = _value)
    }
    #[doc = "PRI_10: Priority of system handler 10<br>"]
    pub(crate) fn scs_shpr2d14_pri_10_read(&self) -> MemResult<u8> {
        Ok(self.priorities[SysHandlerPriority::Pri10 as usize])
    }
    #[doc = "PRI_10: Priority of system handler 10<br>"]
    pub fn scs_shpr2d14_pri_10_write(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.priorities[SysHandlerPriority::Pri10 as usize] = _value)
    }
    #[doc = "PRI_11: Priority of system handler 11, SVCall<br>"]
    pub(crate) fn scs_shpr2d14_pri_11_read(&self) -> MemResult<u8> {
        Ok(self.priorities[SysHandlerPriority::Pri11 as usize])
    }
    #[doc = "PRI_11: Priority of system handler 11, SVCall<br>"]
    pub fn scs_shpr2d14_pri_11_write(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.priorities[SysHandlerPriority::Pri11 as usize] = _value)
    }
    #[doc = "PRI_12: Priority of system handler 4, DebugMonitor<br>"]
    pub(crate) fn scs_shpr3d18_pri_12_read(&self) -> MemResult<u8> {
        Ok(self.priorities[SysHandlerPriority::Pri12 as usize])
    }
    #[doc = "PRI_12: Priority of system handler 4, DebugMonitor<br>"]
    pub fn scs_shpr3d18_pri_12_write(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.priorities[SysHandlerPriority::Pri12 as usize] = _value)
    }
    #[doc = "PRI_13: Priority of system handler 13<br>"]
    pub(crate) fn scs_shpr3d18_pri_13_read(&self) -> MemResult<u8> {
        Ok(self.priorities[SysHandlerPriority::Pri13 as usize])
    }
    #[doc = "PRI_13: Priority of system handler 13<br>"]
    pub fn scs_shpr3d18_pri_13_write(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.priorities[SysHandlerPriority::Pri13 as usize] = _value)
    }
    #[doc = "PRI_14: Priority of system handler 14, PendSV<br>"]
    pub(crate) fn scs_shpr3d18_pri_14_read(&self) -> MemResult<u8> {
        Ok(self.priorities[SysHandlerPriority::Pri14 as usize])
    }
    #[doc = "PRI_14: Priority of system handler 14, PendSV<br>"]
    pub fn scs_shpr3d18_pri_14_write(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.priorities[SysHandlerPriority::Pri14 as usize] = _value)
    }
    #[doc = "PRI_15: Priority of system handler 15, SysTick<br>"]
    pub(crate) fn scs_shpr3d18_pri_15_read(&self) -> MemResult<u8> {
        Ok(self.priorities[SysHandlerPriority::Pri15 as usize])
    }
    #[doc = "PRI_15: Priority of system handler 15, SysTick<br>"]
    pub fn scs_shpr3d18_pri_15_write(&mut self, _value: u8) -> MemResult<()> {
        Ok(self.priorities[SysHandlerPriority::Pri15 as usize] = _value)
    }
    #[doc = "MEMFAULTACT: MemManage active<br>"]
    pub(crate) fn scs_shcsrd1c_memfaultact_read(&self) -> MemResult<bool> {
        todo ! ("read MEMFAULTACT mwrite None write None rac None reset value false")
    }
    #[doc = "MEMFAULTACT: MemManage active<br>"]
    pub(crate) fn scs_shcsrd1c_memfaultact_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write MEMFAULTACT mwrite None write None rac None reset value false")
    }
    #[doc = "BUSFAULTACT: BusFault active<br>"]
    pub(crate) fn scs_shcsrd1c_busfaultact_read(&self) -> MemResult<bool> {
        todo ! ("read BUSFAULTACT mwrite None write None rac None reset value false")
    }
    #[doc = "BUSFAULTACT: BusFault active<br>"]
    pub(crate) fn scs_shcsrd1c_busfaultact_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write BUSFAULTACT mwrite None write None rac None reset value false")
    }
    #[doc = "USGFAULTACT: UsageFault active<br>"]
    pub(crate) fn scs_shcsrd1c_usgfaultact_read(&self) -> MemResult<bool> {
        todo ! ("read USGFAULTACT mwrite None write None rac None reset value false")
    }
    #[doc = "USGFAULTACT: UsageFault active<br>"]
    pub(crate) fn scs_shcsrd1c_usgfaultact_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write USGFAULTACT mwrite None write None rac None reset value false")
    }
    #[doc = "SVCALLACT: SVCall active<br>"]
    pub(crate) fn scs_shcsrd1c_svcallact_read(&self) -> MemResult<bool> {
        todo!(
            "read SVCALLACT mwrite None write None rac None reset value false"
        )
    }
    #[doc = "SVCALLACT: SVCall active<br>"]
    pub(crate) fn scs_shcsrd1c_svcallact_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write SVCALLACT mwrite None write None rac None reset value false"
        )
    }
    #[doc = "MONITORACT: Monitor active<br>"]
    pub(crate) fn scs_shcsrd1c_monitoract_read(&self) -> MemResult<bool> {
        todo!(
            "read MONITORACT mwrite None write None rac None reset value false"
        )
    }
    #[doc = "MONITORACT: Monitor active<br>"]
    pub(crate) fn scs_shcsrd1c_monitoract_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write MONITORACT mwrite None write None rac None reset value false")
    }
    #[doc = "PENDSVACT: PendSV active<br>"]
    pub(crate) fn scs_shcsrd1c_pendsvact_read(&self) -> MemResult<bool> {
        todo!(
            "read PENDSVACT mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PENDSVACT: PendSV active<br>"]
    pub(crate) fn scs_shcsrd1c_pendsvact_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write PENDSVACT mwrite None write None rac None reset value false"
        )
    }
    #[doc = "SYSTICKACT: SysTick active<br>"]
    pub(crate) fn scs_shcsrd1c_systickact_read(&self) -> MemResult<bool> {
        todo!(
            "read SYSTICKACT mwrite None write None rac None reset value false"
        )
    }
    #[doc = "SYSTICKACT: SysTick active<br>"]
    pub(crate) fn scs_shcsrd1c_systickact_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write SYSTICKACT mwrite None write None rac None reset value false")
    }
    #[doc = "USGFAULTPENDED: UsageFault pending<br>"]
    pub(crate) fn scs_shcsrd1c_usgfaultpended_read(&self) -> MemResult<bool> {
        todo ! ("read USGFAULTPENDED mwrite None write None rac None reset value false")
    }
    #[doc = "USGFAULTPENDED: UsageFault pending<br>"]
    pub(crate) fn scs_shcsrd1c_usgfaultpended_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write USGFAULTPENDED mwrite None write None rac None reset value false")
    }
    #[doc = "MEMFAULTPENDED: MemManage pending<br>"]
    pub(crate) fn scs_shcsrd1c_memfaultpended_read(&self) -> MemResult<bool> {
        todo ! ("read MEMFAULTPENDED mwrite None write None rac None reset value false")
    }
    #[doc = "MEMFAULTPENDED: MemManage pending<br>"]
    pub(crate) fn scs_shcsrd1c_memfaultpended_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write MEMFAULTPENDED mwrite None write None rac None reset value false")
    }
    #[doc = "BUSFAULTPENDED: BusFault pending<br>"]
    pub(crate) fn scs_shcsrd1c_busfaultpended_read(&self) -> MemResult<bool> {
        todo ! ("read BUSFAULTPENDED mwrite None write None rac None reset value false")
    }
    #[doc = "BUSFAULTPENDED: BusFault pending<br>"]
    pub(crate) fn scs_shcsrd1c_busfaultpended_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write BUSFAULTPENDED mwrite None write None rac None reset value false")
    }
    #[doc = "SVCALLPENDED: SVCall pending<br>"]
    pub(crate) fn scs_shcsrd1c_svcallpended_read(&self) -> MemResult<bool> {
        todo ! ("read SVCALLPENDED mwrite None write None rac None reset value false")
    }
    #[doc = "SVCALLPENDED: SVCall pending<br>"]
    pub(crate) fn scs_shcsrd1c_svcallpended_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write SVCALLPENDED mwrite None write None rac None reset value false")
    }
    #[doc = "MEMFAULTENA: Enable MemManage fault<br>"]
    pub(crate) fn scs_shcsrd1c_memfaultena_read(&self) -> MemResult<bool> {
        todo ! ("read MEMFAULTENA mwrite None write None rac None reset value false")
    }
    #[doc = "MEMFAULTENA: Enable MemManage fault<br>"]
    pub(crate) fn scs_shcsrd1c_memfaultena_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write MEMFAULTENA mwrite None write None rac None reset value false")
    }
    #[doc = "BUSFAULTENA: Enable BusFault<br>"]
    pub(crate) fn scs_shcsrd1c_busfaultena_read(&self) -> MemResult<bool> {
        todo ! ("read BUSFAULTENA mwrite None write None rac None reset value false")
    }
    #[doc = "BUSFAULTENA: Enable BusFault<br>"]
    pub(crate) fn scs_shcsrd1c_busfaultena_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write BUSFAULTENA mwrite None write None rac None reset value false")
    }
    #[doc = "USGFAULTENA: Enable UsageFault<br>"]
    pub(crate) fn scs_shcsrd1c_usgfaultena_read(&self) -> MemResult<bool> {
        todo ! ("read USGFAULTENA mwrite None write None rac None reset value false")
    }
    #[doc = "USGFAULTENA: Enable UsageFault<br>"]
    pub(crate) fn scs_shcsrd1c_usgfaultena_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write USGFAULTENA mwrite None write None rac None reset value false")
    }
    #[doc = "IACCVIOL: MPU or Execute Never (XN) default memory map access violation on an instruction fetch<br>"]
    pub(crate) fn scs_cfsrd20_iaccviol_read(&self) -> MemResult<bool> {
        todo!("read IACCVIOL mwrite None write None rac None reset value false")
    }
    #[doc = "IACCVIOL: MPU or Execute Never (XN) default memory map access violation on an instruction fetch<br>"]
    pub(crate) fn scs_cfsrd20_iaccviol_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write IACCVIOL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "DACCVIOL: Data access violation. The MMAR shows the data address that the load or store tried to access<br>"]
    pub(crate) fn scs_cfsrd20_daccviol_read(&self) -> MemResult<bool> {
        todo!("read DACCVIOL mwrite None write None rac None reset value false")
    }
    #[doc = "DACCVIOL: Data access violation. The MMAR shows the data address that the load or store tried to access<br>"]
    pub(crate) fn scs_cfsrd20_daccviol_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write DACCVIOL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "MUNSTKERR: Derived MemManage fault on exception return<br>"]
    pub(crate) fn scs_cfsrd20_munstkerr_read(&self) -> MemResult<bool> {
        todo!(
            "read MUNSTKERR mwrite None write None rac None reset value false"
        )
    }
    #[doc = "MUNSTKERR: Derived MemManage fault on exception return<br>"]
    pub(crate) fn scs_cfsrd20_munstkerr_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write MUNSTKERR mwrite None write None rac None reset value false"
        )
    }
    #[doc = "MSTKERR: Derived MemManage fault on exception entry<br>"]
    pub(crate) fn scs_cfsrd20_mstkerr_read(&self) -> MemResult<bool> {
        todo!("read MSTKERR mwrite None write None rac None reset value false")
    }
    #[doc = "MSTKERR: Derived MemManage fault on exception entry<br>"]
    pub(crate) fn scs_cfsrd20_mstkerr_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MSTKERR mwrite None write None rac None reset value false")
    }
    #[doc = "MLSPERR: MemManage fault during FP lazy state preservation<br>"]
    pub(crate) fn scs_cfsrd20_mlsperr_read(&self) -> MemResult<bool> {
        todo!("read MLSPERR mwrite None write None rac None reset value false")
    }
    #[doc = "MLSPERR: MemManage fault during FP lazy state preservation<br>"]
    pub(crate) fn scs_cfsrd20_mlsperr_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MLSPERR mwrite None write None rac None reset value false")
    }
    #[doc = "MMARVALID: MMAR has valid contents<br>"]
    pub(crate) fn scs_cfsrd20_mmarvalid_read(&self) -> MemResult<bool> {
        todo!(
            "read MMARVALID mwrite None write None rac None reset value false"
        )
    }
    #[doc = "MMARVALID: MMAR has valid contents<br>"]
    pub(crate) fn scs_cfsrd20_mmarvalid_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write MMARVALID mwrite None write None rac None reset value false"
        )
    }
    #[doc = "IBUSERR: Bus fault on an instruction prefetch<br>"]
    pub(crate) fn scs_cfsrd20_ibuserr_read(&self) -> MemResult<bool> {
        todo!("read IBUSERR mwrite None write None rac None reset value false")
    }
    #[doc = "IBUSERR: Bus fault on an instruction prefetch<br>"]
    pub(crate) fn scs_cfsrd20_ibuserr_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write IBUSERR mwrite None write None rac None reset value false")
    }
    #[doc = "PRECISERR: Precise data access error<br>"]
    pub(crate) fn scs_cfsrd20_preciserr_read(&self) -> MemResult<bool> {
        todo!(
            "read PRECISERR mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PRECISERR: Precise data access error<br>"]
    pub(crate) fn scs_cfsrd20_preciserr_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write PRECISERR mwrite None write None rac None reset value false"
        )
    }
    #[doc = "IMPRECISERR: Imprecise data access error<br>"]
    pub(crate) fn scs_cfsrd20_impreciserr_read(&self) -> MemResult<bool> {
        todo ! ("read IMPRECISERR mwrite None write None rac None reset value false")
    }
    #[doc = "IMPRECISERR: Imprecise data access error<br>"]
    pub(crate) fn scs_cfsrd20_impreciserr_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write IMPRECISERR mwrite None write None rac None reset value false")
    }
    #[doc = "UNSTKERR: Derived bus fault on exception return<br>"]
    pub(crate) fn scs_cfsrd20_unstkerr_read(&self) -> MemResult<bool> {
        todo!("read UNSTKERR mwrite None write None rac None reset value false")
    }
    #[doc = "UNSTKERR: Derived bus fault on exception return<br>"]
    pub(crate) fn scs_cfsrd20_unstkerr_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write UNSTKERR mwrite None write None rac None reset value false"
        )
    }
    #[doc = "STKERR: Derived bus fault on exception entry<br>"]
    pub(crate) fn scs_cfsrd20_stkerr_read(&self) -> MemResult<bool> {
        todo!("read STKERR mwrite None write None rac None reset value false")
    }
    #[doc = "STKERR: Derived bus fault on exception entry<br>"]
    pub(crate) fn scs_cfsrd20_stkerr_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write STKERR mwrite None write None rac None reset value false")
    }
    #[doc = "LSPERR: Bus fault during FP lazy state preservation<br>"]
    pub(crate) fn scs_cfsrd20_lsperr_read(&self) -> MemResult<bool> {
        todo!("read LSPERR mwrite None write None rac None reset value false")
    }
    #[doc = "LSPERR: Bus fault during FP lazy state preservation<br>"]
    pub(crate) fn scs_cfsrd20_lsperr_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write LSPERR mwrite None write None rac None reset value false")
    }
    #[doc = "BFARVALID: BFAR has valid contents<br>"]
    pub(crate) fn scs_cfsrd20_bfarvalid_read(&self) -> MemResult<bool> {
        todo!(
            "read BFARVALID mwrite None write None rac None reset value false"
        )
    }
    #[doc = "BFARVALID: BFAR has valid contents<br>"]
    pub(crate) fn scs_cfsrd20_bfarvalid_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write BFARVALID mwrite None write None rac None reset value false"
        )
    }
    #[doc = "UNDEFINSTR: Processor has attempted to execute an undefined instruction.<br>"]
    pub(crate) fn scs_cfsrd20_undefinstr_read(&self) -> MemResult<bool> {
        todo!(
            "read UNDEFINSTR mwrite None write None rac None reset value false"
        )
    }
    #[doc = "UNDEFINSTR: Processor has attempted to execute an undefined instruction.<br>"]
    pub(crate) fn scs_cfsrd20_undefinstr_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write UNDEFINSTR mwrite None write None rac None reset value false")
    }
    #[doc = "INVSTATE: Instruction executed with invalid EPSR.T or EPSR.IT field<br>"]
    pub(crate) fn scs_cfsrd20_invstate_read(&self) -> MemResult<bool> {
        todo!("read INVSTATE mwrite None write None rac None reset value false")
    }
    #[doc = "INVSTATE: Instruction executed with invalid EPSR.T or EPSR.IT field<br>"]
    pub(crate) fn scs_cfsrd20_invstate_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write INVSTATE mwrite None write None rac None reset value false"
        )
    }
    #[doc = "INVPC: Integrity check error on EXC_RETURN<br>"]
    pub(crate) fn scs_cfsrd20_invpc_read(&self) -> MemResult<bool> {
        todo!("read INVPC mwrite None write None rac None reset value false")
    }
    #[doc = "INVPC: Integrity check error on EXC_RETURN<br>"]
    pub(crate) fn scs_cfsrd20_invpc_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write INVPC mwrite None write None rac None reset value false")
    }
    #[doc = "NOCP: Coprocessor access error<br>"]
    pub(crate) fn scs_cfsrd20_nocp_read(&self) -> MemResult<bool> {
        todo!("read NOCP mwrite None write None rac None reset value false")
    }
    #[doc = "NOCP: Coprocessor access error<br>"]
    pub(crate) fn scs_cfsrd20_nocp_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write NOCP mwrite None write None rac None reset value false")
    }
    #[doc = "UNALIGNED: Unaligned access error<br>"]
    pub(crate) fn scs_cfsrd20_unaligned_read(&self) -> MemResult<bool> {
        todo!(
            "read UNALIGNED mwrite None write None rac None reset value false"
        )
    }
    #[doc = "UNALIGNED: Unaligned access error<br>"]
    pub(crate) fn scs_cfsrd20_unaligned_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write UNALIGNED mwrite None write None rac None reset value false"
        )
    }
    #[doc = "DIVBYZERO: Divide by zero error<br>"]
    pub(crate) fn scs_cfsrd20_divbyzero_read(&self) -> MemResult<bool> {
        todo!(
            "read DIVBYZERO mwrite None write None rac None reset value false"
        )
    }
    #[doc = "DIVBYZERO: Divide by zero error<br>"]
    pub(crate) fn scs_cfsrd20_divbyzero_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write DIVBYZERO mwrite None write None rac None reset value false"
        )
    }
    #[doc = "VECTTBL: Indicates a fault has occurred because of a vector table read error on exception processing<br>"]
    pub(crate) fn scs_hfsrd24_vecttbl_read(&self) -> MemResult<bool> {
        todo!("read VECTTBL mwrite None write None rac None reset value false")
    }
    #[doc = "VECTTBL: Indicates a fault has occurred because of a vector table read error on exception processing<br>"]
    pub(crate) fn scs_hfsrd24_vecttbl_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write VECTTBL mwrite None write None rac None reset value false")
    }
    #[doc = "FORCED: Indicates a fault with configurable priority has been escalated to a HardFault<br>"]
    pub(crate) fn scs_hfsrd24_forced_read(&self) -> MemResult<bool> {
        todo!("read FORCED mwrite None write None rac None reset value false")
    }
    #[doc = "FORCED: Indicates a fault with configurable priority has been escalated to a HardFault<br>"]
    pub(crate) fn scs_hfsrd24_forced_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write FORCED mwrite None write None rac None reset value false")
    }
    #[doc = "DEBUGEVT: Indicates a Debug event has occurred<br>"]
    pub(crate) fn scs_hfsrd24_debugevt_read(&self) -> MemResult<bool> {
        todo!("read DEBUGEVT mwrite None write None rac None reset value false")
    }
    #[doc = "DEBUGEVT: Indicates a Debug event has occurred<br>"]
    pub(crate) fn scs_hfsrd24_debugevt_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write DEBUGEVT mwrite None write None rac None reset value false"
        )
    }
    #[doc = "HALTED: Indicates a debug event generated by C_HALT or C_STEP request or setting DEMCR.MON_STEP<br>"]
    pub(crate) fn scs_dfsrd28_halted_read(&self) -> MemResult<bool> {
        todo!("read HALTED mwrite None write None rac None reset value false")
    }
    #[doc = "HALTED: Indicates a debug event generated by C_HALT or C_STEP request or setting DEMCR.MON_STEP<br>"]
    pub(crate) fn scs_dfsrd28_halted_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write HALTED mwrite None write None rac None reset value false")
    }
    #[doc = "BKPT: Indicates a debug event generated by BKPT instruction execution or a breakpoint match in FPB<br>"]
    pub(crate) fn scs_dfsrd28_bkpt_read(&self) -> MemResult<bool> {
        todo!("read BKPT mwrite None write None rac None reset value false")
    }
    #[doc = "BKPT: Indicates a debug event generated by BKPT instruction execution or a breakpoint match in FPB<br>"]
    pub(crate) fn scs_dfsrd28_bkpt_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write BKPT mwrite None write None rac None reset value false")
    }
    #[doc = "DWTTRAP: Indicates a debug event generated by the DWT<br>"]
    pub(crate) fn scs_dfsrd28_dwttrap_read(&self) -> MemResult<bool> {
        todo!("read DWTTRAP mwrite None write None rac None reset value false")
    }
    #[doc = "DWTTRAP: Indicates a debug event generated by the DWT<br>"]
    pub(crate) fn scs_dfsrd28_dwttrap_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write DWTTRAP mwrite None write None rac None reset value false")
    }
    #[doc = "VCATCH: Indicates triggering of a Vector catch<br>"]
    pub(crate) fn scs_dfsrd28_vcatch_read(&self) -> MemResult<bool> {
        todo!("read VCATCH mwrite None write None rac None reset value false")
    }
    #[doc = "VCATCH: Indicates triggering of a Vector catch<br>"]
    pub(crate) fn scs_dfsrd28_vcatch_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write VCATCH mwrite None write None rac None reset value false")
    }
    #[doc = "EXTERNAL: Indicates a debug event generated because of the assertion of EDBGRQ<br>"]
    pub(crate) fn scs_dfsrd28_external_read(&self) -> MemResult<bool> {
        todo!("read EXTERNAL mwrite None write None rac None reset value false")
    }
    #[doc = "EXTERNAL: Indicates a debug event generated because of the assertion of EDBGRQ<br>"]
    pub(crate) fn scs_dfsrd28_external_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write EXTERNAL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "MMFAR: Shows the address of the memory location that caused an MMU fault<br>"]
    pub(crate) fn scs_mmfard2c_read(&self) -> MemResult<u32> {
        todo ! ("read scs_mmfard2c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "MMFAR: Shows the address of the memory location that caused an MMU fault<br>"]
    pub(crate) fn scs_mmfard2c_write(&mut self, _value: u32) -> MemResult<()> {
        todo ! ("write scs_mmfard2c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "BFAR: Shows the address associated with a precise data access fault<br>"]
    pub(crate) fn scs_bfard30_read(&self) -> MemResult<u32> {
        todo ! ("read scs_bfard30 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "BFAR: Shows the address associated with a precise data access fault<br>"]
    pub(crate) fn scs_bfard30_write(&mut self, _value: u32) -> MemResult<()> {
        todo ! ("write scs_bfard30 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "AFSR: Latched version of the AUXFAULT inputs<br>"]
    pub(crate) fn scs_afsrd34_read(&self) -> MemResult<u32> {
        todo ! ("read scs_afsrd34 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "AFSR: Latched version of the AUXFAULT inputs<br>"]
    pub(crate) fn scs_afsrd34_write(&mut self, _value: u32) -> MemResult<()> {
        todo ! ("write scs_afsrd34 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "CP0: Defines access permissions for the CP0 coprocessor.<br>"]
    pub(crate) fn scs_cpacrd80_cp0_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E81ScsCpacrd80Cp0> {
        todo ! ("read CP0 mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "CP0: Defines access permissions for the CP0 coprocessor.<br>"]
    pub(crate) fn scs_cpacrd80_cp0_write(
        &mut self,
        _value: crate::peripheral::enums::E81ScsCpacrd80Cp0,
    ) -> MemResult<()> {
        todo ! ("write CP0 mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "CP1: Defines access permissions for the CP1 coprocessor.<br>"]
    pub(crate) fn scs_cpacrd80_cp1_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E81ScsCpacrd80Cp0> {
        todo ! ("read CP1 mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "CP1: Defines access permissions for the CP1 coprocessor.<br>"]
    pub(crate) fn scs_cpacrd80_cp1_write(
        &mut self,
        _value: crate::peripheral::enums::E81ScsCpacrd80Cp0,
    ) -> MemResult<()> {
        todo ! ("write CP1 mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "CP2: Defines access permissions for the CP2 coprocessor.<br>"]
    pub(crate) fn scs_cpacrd80_cp2_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E81ScsCpacrd80Cp0> {
        todo ! ("read CP2 mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "CP2: Defines access permissions for the CP2 coprocessor.<br>"]
    pub(crate) fn scs_cpacrd80_cp2_write(
        &mut self,
        _value: crate::peripheral::enums::E81ScsCpacrd80Cp0,
    ) -> MemResult<()> {
        todo ! ("write CP2 mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "CP3: Defines access permissions for the CP3 coprocessor.<br>"]
    pub(crate) fn scs_cpacrd80_cp3_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E81ScsCpacrd80Cp0> {
        todo ! ("read CP3 mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "CP3: Defines access permissions for the CP3 coprocessor.<br>"]
    pub(crate) fn scs_cpacrd80_cp3_write(
        &mut self,
        _value: crate::peripheral::enums::E81ScsCpacrd80Cp0,
    ) -> MemResult<()> {
        todo ! ("write CP3 mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "CP4: Defines access permissions for the CP4 coprocessor.<br>"]
    pub(crate) fn scs_cpacrd80_cp4_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E81ScsCpacrd80Cp0> {
        todo ! ("read CP4 mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "CP4: Defines access permissions for the CP4 coprocessor.<br>"]
    pub(crate) fn scs_cpacrd80_cp4_write(
        &mut self,
        _value: crate::peripheral::enums::E81ScsCpacrd80Cp0,
    ) -> MemResult<()> {
        todo ! ("write CP4 mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "CP5: Defines access permissions for the CP5 coprocessor.<br>"]
    pub(crate) fn scs_cpacrd80_cp5_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E81ScsCpacrd80Cp0> {
        todo ! ("read CP5 mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "CP5: Defines access permissions for the CP5 coprocessor.<br>"]
    pub(crate) fn scs_cpacrd80_cp5_write(
        &mut self,
        _value: crate::peripheral::enums::E81ScsCpacrd80Cp0,
    ) -> MemResult<()> {
        todo ! ("write CP5 mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "CP6: Defines access permissions for the CP6 coprocessor.<br>"]
    pub(crate) fn scs_cpacrd80_cp6_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E81ScsCpacrd80Cp0> {
        todo ! ("read CP6 mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "CP6: Defines access permissions for the CP6 coprocessor.<br>"]
    pub(crate) fn scs_cpacrd80_cp6_write(
        &mut self,
        _value: crate::peripheral::enums::E81ScsCpacrd80Cp0,
    ) -> MemResult<()> {
        todo ! ("write CP6 mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "CP7: Defines access permissions for the CP7 coprocessor.<br>"]
    pub(crate) fn scs_cpacrd80_cp7_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E81ScsCpacrd80Cp0> {
        todo ! ("read CP7 mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "CP7: Defines access permissions for the CP7 coprocessor.<br>"]
    pub(crate) fn scs_cpacrd80_cp7_write(
        &mut self,
        _value: crate::peripheral::enums::E81ScsCpacrd80Cp0,
    ) -> MemResult<()> {
        todo ! ("write CP7 mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "CP10: Defines access permissions for the CP10 coprocessor.<br>"]
    pub(crate) fn scs_cpacrd80_cp10_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E81ScsCpacrd80Cp0> {
        todo ! ("read CP10 mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "CP10: Defines access permissions for the CP10 coprocessor.<br>"]
    pub(crate) fn scs_cpacrd80_cp10_write(
        &mut self,
        _value: crate::peripheral::enums::E81ScsCpacrd80Cp0,
    ) -> MemResult<()> {
        todo ! ("write CP10 mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "CP11: Defines access permissions for the CP11 coprocessor.<br>"]
    pub(crate) fn scs_cpacrd80_cp11_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E81ScsCpacrd80Cp0> {
        todo ! ("read CP11 mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "CP11: Defines access permissions for the CP11 coprocessor.<br>"]
    pub(crate) fn scs_cpacrd80_cp11_write(
        &mut self,
        _value: crate::peripheral::enums::E81ScsCpacrd80Cp0,
    ) -> MemResult<()> {
        todo ! ("write CP11 mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "MON_EN: Enable the DebugMonitor exception<br>"]
    pub(crate) fn scs_demcrdf4_mon_en_read(&self) -> MemResult<bool> {
        //TODO debug off by default
        // todo!("read MON_EN mwrite None write None rac None reset value false")
        Ok(false)
    }
    #[doc = "MON_EN: Enable the DebugMonitor exception<br>"]
    pub(crate) fn scs_demcrdf4_mon_en_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        //TODO debug off by default
        // todo!("write MON_EN mwrite None write None rac None reset value false")
        if _value {
            todo!("write Control DEMCR MON_EN reset value false")
        }
        Ok(())
    }
    #[doc = "MON_PEND: Sets or clears the pending state of the DebugMonitor exception<br>"]
    pub(crate) fn scs_demcrdf4_mon_pend_read(&self) -> MemResult<bool> {
        //TODO debug off by default
        // todo!("read MON_PEND mwrite None write None rac None reset value false")
        Ok(false)
    }
    #[doc = "MON_PEND: Sets or clears the pending state of the DebugMonitor exception<br>"]
    pub(crate) fn scs_demcrdf4_mon_pend_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        //TODO debug off by default
        //todo!(
        //    "write MON_PEND mwrite None write None rac None reset value false"
        //)
        if _value {
            todo!("write Control DEMCR MON_PEND reset value false")
        }
        Ok(())
    }
    #[doc = "INTID: Indicates the interrupt to be triggered. The value written is (ExceptionNumber - 16)<br>"]
    pub(crate) fn scs_stiref8_intid_write(
        &mut self,
        _value: u16,
    ) -> MemResult<()> {
        todo ! ("write INTID mwrite None write None rac None reset value 0x00 mask 0x1ff")
    }
}
