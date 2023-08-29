use icicle_vm::cpu::mem::MemResult;

use super::enums::LfclkSrc;
use super::event;

#[derive(Default)]
#[doc = "BPROT: Block Protect<br>POWER: Power control<br>CLOCK: Clock control<br><br>Instances:<br>0x40000000: BPROT, POWER, CLOCK<br>"]
pub struct Apb0 {
    pub source: LfclkSrc,
    // FUTURE: use the `core::mem::variant_count` to avoid using a number
    pub events: [event::Event; 7],
    pub bypass: bool,
    pub external: bool,

    pub ram: [RamBlock; 4],
}

#[derive(Default)]
pub struct RamBlock {
    pub keep_on: bool,
    pub retain: bool,
}

impl RamBlock {
    fn is_on(&self) -> bool {
        // if needs to implement a "powering up" delay, do it here
        self.keep_on
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
enum EventId {
    HFCLKSTARTED = 0,
    LFCLKSTARTED = 1,
    POFWARN = 2,
    DONE = 3,
    CTTO = 4,
    SLEEPENTER = 5,
    SLEEPEXIT = 6,
}

impl Apb0 {
    pub(crate) fn page_to_index(page: u64) -> usize {
        match page {
            262144u64 => 0usize,
            _ => unreachable!(),
        }
    }
    #[doc = "TASKS_HFCLKSTART: Start HFCLK crystal oscillator<br>"]
    pub(crate) fn apb0_tasks_hfclkstart0_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb0_tasks_hfclkstart0 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_HFCLKSTOP: Stop HFCLK crystal oscillator<br>"]
    pub(crate) fn apb0_tasks_hfclkstop4_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb0_tasks_hfclkstop4 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_LFCLKSTART: Start LFCLK source<br>"]
    pub(crate) fn apb0_tasks_lfclkstart8_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        // TODO for now we don't care about this task, just rementer to write
        // to the event of finishing.
        if _value != 0 {
            //todo!("Trigger task LFCLKSTART")
        }
        Ok(())
    }
    #[doc = "TASKS_LFCLKSTOP: Stop LFCLK source<br>"]
    pub(crate) fn apb0_tasks_lfclkstopc_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb0_tasks_lfclkstopc mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_CAL: Start calibration of LFRC oscillator<br>"]
    pub(crate) fn apb0_tasks_cal10_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb0_tasks_cal10 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_CTSTART: Start calibration timer<br>"]
    pub(crate) fn apb0_tasks_ctstart14_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb0_tasks_ctstart14 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_CTSTOP: Stop calibration timer<br>"]
    pub(crate) fn apb0_tasks_ctstop18_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb0_tasks_ctstop18 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_CONSTLAT: Enable constant latency mode<br>"]
    pub(crate) fn apb0_tasks_constlat78_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb0_tasks_constlat78 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_LOWPWR: Enable low power mode (variable latency)<br>"]
    pub(crate) fn apb0_tasks_lowpwr7c_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb0_tasks_lowpwr7c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_HFCLKSTARTED: HFCLK oscillator started<br>"]
    pub(crate) fn apb0_events_hfclkstarted100_read(&self) -> MemResult<u32> {
        Ok(self.events[EventId::HFCLKSTARTED as usize].triggered as u32)
    }
    #[doc = "EVENTS_HFCLKSTARTED: HFCLK oscillator started<br>"]
    pub(crate) fn apb0_events_hfclkstarted100_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(
            self.events[EventId::HFCLKSTARTED as usize]
                .trigger_on_write(_value),
        )
    }
    #[doc = "EVENTS_LFCLKSTARTED: LFCLK started<br>"]
    pub(crate) fn apb0_events_lfclkstarted104_read(&self) -> MemResult<u32> {
        Ok(self.events[EventId::LFCLKSTARTED as usize].triggered as u32)
    }
    #[doc = "EVENTS_LFCLKSTARTED: LFCLK started<br>"]
    pub(crate) fn apb0_events_lfclkstarted104_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(
            self.events[EventId::LFCLKSTARTED as usize]
                .trigger_on_write(_value),
        )
    }
    #[doc = "EVENTS_POFWARN: Power failure warning<br>"]
    pub(crate) fn apb0_events_pofwarn108_read(&self) -> MemResult<u32> {
        Ok(self.events[EventId::POFWARN as usize].triggered as u32)
    }
    #[doc = "EVENTS_POFWARN: Power failure warning<br>"]
    pub(crate) fn apb0_events_pofwarn108_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.events[EventId::POFWARN as usize].trigger_on_write(_value))
    }
    #[doc = "EVENTS_DONE: Calibration of LFCLK RC oscillator complete event<br>"]
    pub(crate) fn apb0_events_done10c_read(&self) -> MemResult<u32> {
        Ok(self.events[EventId::DONE as usize].triggered as u32)
    }
    #[doc = "EVENTS_DONE: Calibration of LFCLK RC oscillator complete event<br>"]
    pub(crate) fn apb0_events_done10c_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.events[EventId::DONE as usize].trigger_on_write(_value))
    }
    #[doc = "EVENTS_CTTO: Calibration timer timeout<br>"]
    pub(crate) fn apb0_events_ctto110_read(&self) -> MemResult<u32> {
        Ok(self.events[EventId::CTTO as usize].triggered as u32)
    }
    #[doc = "EVENTS_CTTO: Calibration timer timeout<br>"]
    pub(crate) fn apb0_events_ctto110_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.events[EventId::CTTO as usize].trigger_on_write(_value))
    }
    #[doc = "EVENTS_SLEEPENTER: CPU entered WFI/WFE sleep<br>"]
    pub(crate) fn apb0_events_sleepenter114_read(&self) -> MemResult<u32> {
        Ok(self.events[EventId::SLEEPENTER as usize].triggered as u32)
    }
    #[doc = "EVENTS_SLEEPENTER: CPU entered WFI/WFE sleep<br>"]
    pub(crate) fn apb0_events_sleepenter114_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.events[EventId::SLEEPENTER as usize].trigger_on_write(_value))
    }
    #[doc = "EVENTS_SLEEPEXIT: CPU exited WFI/WFE sleep<br>"]
    pub(crate) fn apb0_events_sleepexit118_read(&self) -> MemResult<u32> {
        Ok(self.events[EventId::SLEEPEXIT as usize].triggered as u32)
    }
    #[doc = "EVENTS_SLEEPEXIT: CPU exited WFI/WFE sleep<br>"]
    pub(crate) fn apb0_events_sleepexit118_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        Ok(self.events[EventId::SLEEPEXIT as usize].trigger_on_write(_value))
    }
    #[doc = "HFCLKSTARTED: Write '1' to Enable interrupt for HFCLKSTARTED event<br>"]
    pub(crate) fn apb0_intenset304_hfclkstarted_read(&self) -> MemResult<bool> {
        Ok(self.events[EventId::HFCLKSTARTED as usize].triggered)
    }
    #[doc = "HFCLKSTARTED: Write '1' to Enable interrupt for HFCLKSTARTED event<br>"]
    pub(crate) fn apb0_intenset304_hfclkstarted_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.events[EventId::HFCLKSTARTED as usize].triggered = true;
        }
        Ok(())
    }
    #[doc = "LFCLKSTARTED: Write '1' to Enable interrupt for LFCLKSTARTED event<br>"]
    pub(crate) fn apb0_intenset304_lfclkstarted_read(&self) -> MemResult<bool> {
        Ok(self.events[EventId::LFCLKSTARTED as usize].triggered)
    }
    #[doc = "LFCLKSTARTED: Write '1' to Enable interrupt for LFCLKSTARTED event<br>"]
    pub(crate) fn apb0_intenset304_lfclkstarted_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.events[EventId::LFCLKSTARTED as usize].triggered = true;
        }
        Ok(())
    }
    #[doc = "POFWARN: Write '1' to Enable interrupt for POFWARN event<br>"]
    pub(crate) fn apb0_intenset304_pofwarn_read(&self) -> MemResult<bool> {
        Ok(self.events[EventId::POFWARN as usize].triggered)
    }
    #[doc = "POFWARN: Write '1' to Enable interrupt for POFWARN event<br>"]
    pub(crate) fn apb0_intenset304_pofwarn_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.events[EventId::POFWARN as usize].triggered = true;
        }
        Ok(())
    }
    #[doc = "DONE: Write '1' to Enable interrupt for DONE event<br>"]
    pub(crate) fn apb0_intenset304_done_read(&self) -> MemResult<bool> {
        Ok(self.events[EventId::DONE as usize].triggered)
    }
    #[doc = "DONE: Write '1' to Enable interrupt for DONE event<br>"]
    pub(crate) fn apb0_intenset304_done_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.events[EventId::DONE as usize].triggered = true;
        }
        Ok(())
    }
    #[doc = "CTTO: Write '1' to Enable interrupt for CTTO event<br>"]
    pub(crate) fn apb0_intenset304_ctto_read(&self) -> MemResult<bool> {
        Ok(self.events[EventId::CTTO as usize].triggered)
    }
    #[doc = "CTTO: Write '1' to Enable interrupt for CTTO event<br>"]
    pub(crate) fn apb0_intenset304_ctto_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.events[EventId::CTTO as usize].triggered = true;
        }
        Ok(())
    }
    #[doc = "SLEEPENTER: Write '1' to Enable interrupt for SLEEPENTER event<br>"]
    pub(crate) fn apb0_intenset304_sleepenter_read(&self) -> MemResult<bool> {
        todo!(
            "read SLEEPENTER mwrite None write None rac None reset value false"
        )
    }
    #[doc = "SLEEPENTER: Write '1' to Enable interrupt for SLEEPENTER event<br>"]
    pub(crate) fn apb0_intenset304_sleepenter_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.events[EventId::SLEEPENTER as usize].triggered = true;
        }
        Ok(())
    }
    #[doc = "SLEEPEXIT: Write '1' to Enable interrupt for SLEEPEXIT event<br>"]
    pub(crate) fn apb0_intenset304_sleepexit_read(&self) -> MemResult<bool> {
        todo!(
            "read SLEEPEXIT mwrite None write None rac None reset value false"
        )
    }
    #[doc = "SLEEPEXIT: Write '1' to Enable interrupt for SLEEPEXIT event<br>"]
    pub(crate) fn apb0_intenset304_sleepexit_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.events[EventId::SLEEPEXIT as usize].triggered = true;
        }
        Ok(())
    }
    #[doc = "HFCLKSTARTED: Write '1' to Disable interrupt for HFCLKSTARTED event<br>"]
    pub(crate) fn apb0_intenclr308_hfclkstarted_read(&self) -> MemResult<bool> {
        Ok(self.events[EventId::HFCLKSTARTED as usize].triggered)
    }
    #[doc = "HFCLKSTARTED: Write '1' to Disable interrupt for HFCLKSTARTED event<br>"]
    pub(crate) fn apb0_intenclr308_hfclkstarted_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.events[EventId::HFCLKSTARTED as usize].triggered = false;
        }
        Ok(())
    }
    #[doc = "LFCLKSTARTED: Write '1' to Disable interrupt for LFCLKSTARTED event<br>"]
    pub(crate) fn apb0_intenclr308_lfclkstarted_read(&self) -> MemResult<bool> {
        Ok(self.events[EventId::LFCLKSTARTED as usize].triggered)
    }
    #[doc = "LFCLKSTARTED: Write '1' to Disable interrupt for LFCLKSTARTED event<br>"]
    pub(crate) fn apb0_intenclr308_lfclkstarted_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.events[EventId::LFCLKSTARTED as usize].triggered = false;
        }
        Ok(())
    }
    #[doc = "POFWARN: Write '1' to Disable interrupt for POFWARN event<br>"]
    pub(crate) fn apb0_intenclr308_pofwarn_read(&self) -> MemResult<bool> {
        Ok(self.events[EventId::POFWARN as usize].triggered)
    }
    #[doc = "POFWARN: Write '1' to Disable interrupt for POFWARN event<br>"]
    pub(crate) fn apb0_intenclr308_pofwarn_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.events[EventId::POFWARN as usize].triggered = false;
        }
        Ok(())
    }
    #[doc = "DONE: Write '1' to Disable interrupt for DONE event<br>"]
    pub(crate) fn apb0_intenclr308_done_read(&self) -> MemResult<bool> {
        Ok(self.events[EventId::DONE as usize].triggered)
    }
    #[doc = "DONE: Write '1' to Disable interrupt for DONE event<br>"]
    pub(crate) fn apb0_intenclr308_done_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.events[EventId::DONE as usize].triggered = false;
        }
        Ok(())
    }
    #[doc = "CTTO: Write '1' to Disable interrupt for CTTO event<br>"]
    pub(crate) fn apb0_intenclr308_ctto_read(&self) -> MemResult<bool> {
        Ok(self.events[EventId::CTTO as usize].triggered)
    }
    #[doc = "CTTO: Write '1' to Disable interrupt for CTTO event<br>"]
    pub(crate) fn apb0_intenclr308_ctto_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.events[EventId::CTTO as usize].triggered = false;
        }
        Ok(())
    }
    #[doc = "SLEEPENTER: Write '1' to Disable interrupt for SLEEPENTER event<br>"]
    pub(crate) fn apb0_intenclr308_sleepenter_read(&self) -> MemResult<bool> {
        Ok(self.events[EventId::SLEEPENTER as usize].triggered)
    }
    #[doc = "SLEEPENTER: Write '1' to Disable interrupt for SLEEPENTER event<br>"]
    pub(crate) fn apb0_intenclr308_sleepenter_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        if _value {
            self.events[EventId::SLEEPENTER as usize].triggered = false;
        }
        Ok(())
    }
    #[doc = "SLEEPEXIT: Write '1' to Disable interrupt for SLEEPEXIT event<br>"]
    pub(crate) fn apb0_intenclr308_sleepexit_read(&self) -> MemResult<bool> {
        Ok(self.events[EventId::SLEEPEXIT as usize].triggered)
    }
    #[doc = "SLEEPEXIT: Write '1' to Disable interrupt for SLEEPEXIT event<br>"]
    pub(crate) fn apb0_intenclr308_sleepexit_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write SLEEPEXIT mwrite None write None rac None reset value false"
        )
    }
    #[doc = "RESETPIN: Reset from pin-reset detected<br>"]
    pub(crate) fn apb0_resetreas400_resetpin_read(&self) -> MemResult<bool> {
        todo!("read RESETPIN mwrite None write None rac None reset value false")
    }
    #[doc = "RESETPIN: Reset from pin-reset detected<br>"]
    pub(crate) fn apb0_resetreas400_resetpin_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write RESETPIN mwrite None write None rac None reset value false"
        )
    }
    #[doc = "DOG: Reset from watchdog detected<br>"]
    pub(crate) fn apb0_resetreas400_dog_read(&self) -> MemResult<bool> {
        todo!("read DOG mwrite None write None rac None reset value false")
    }
    #[doc = "DOG: Reset from watchdog detected<br>"]
    pub(crate) fn apb0_resetreas400_dog_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write DOG mwrite None write None rac None reset value false")
    }
    #[doc = "SREQ: Reset from soft reset detected<br>"]
    pub(crate) fn apb0_resetreas400_sreq_read(&self) -> MemResult<bool> {
        todo!("read SREQ mwrite None write None rac None reset value false")
    }
    #[doc = "SREQ: Reset from soft reset detected<br>"]
    pub(crate) fn apb0_resetreas400_sreq_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SREQ mwrite None write None rac None reset value false")
    }
    #[doc = "LOCKUP: Reset from CPU lock-up detected<br>"]
    pub(crate) fn apb0_resetreas400_lockup_read(&self) -> MemResult<bool> {
        todo!("read LOCKUP mwrite None write None rac None reset value false")
    }
    #[doc = "LOCKUP: Reset from CPU lock-up detected<br>"]
    pub(crate) fn apb0_resetreas400_lockup_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write LOCKUP mwrite None write None rac None reset value false")
    }
    #[doc = "OFF: Reset due to wake up from System OFF mode when wakeup is triggered from DETECT signal from GPIO<br>"]
    pub(crate) fn apb0_resetreas400_off_read(&self) -> MemResult<bool> {
        todo!("read OFF mwrite None write None rac None reset value false")
    }
    #[doc = "OFF: Reset due to wake up from System OFF mode when wakeup is triggered from DETECT signal from GPIO<br>"]
    pub(crate) fn apb0_resetreas400_off_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write OFF mwrite None write None rac None reset value false")
    }
    #[doc = "LPCOMP: Reset due to wake up from System OFF mode when wakeup is triggered from ANADETECT signal from LPCOMP<br>"]
    pub(crate) fn apb0_resetreas400_lpcomp_read(&self) -> MemResult<bool> {
        todo!("read LPCOMP mwrite None write None rac None reset value false")
    }
    #[doc = "LPCOMP: Reset due to wake up from System OFF mode when wakeup is triggered from ANADETECT signal from LPCOMP<br>"]
    pub(crate) fn apb0_resetreas400_lpcomp_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write LPCOMP mwrite None write None rac None reset value false")
    }
    #[doc = "DIF: Reset due to wake up from System OFF mode when wakeup is triggered from entering into debug interface mode<br>"]
    pub(crate) fn apb0_resetreas400_dif_read(&self) -> MemResult<bool> {
        todo!("read DIF mwrite None write None rac None reset value false")
    }
    #[doc = "DIF: Reset due to wake up from System OFF mode when wakeup is triggered from entering into debug interface mode<br>"]
    pub(crate) fn apb0_resetreas400_dif_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write DIF mwrite None write None rac None reset value false")
    }
    #[doc = "NFC: Reset due to wake up from System OFF mode by NFC field detect<br>"]
    pub(crate) fn apb0_resetreas400_nfc_read(&self) -> MemResult<bool> {
        todo!("read NFC mwrite None write None rac None reset value false")
    }
    #[doc = "NFC: Reset due to wake up from System OFF mode by NFC field detect<br>"]
    pub(crate) fn apb0_resetreas400_nfc_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write NFC mwrite None write None rac None reset value false")
    }
    #[doc = "STATUS: HFCLKSTART task triggered or not<br>"]
    pub(crate) fn apb0_hfclkrun408_status_read(&self) -> MemResult<bool> {
        todo!("read STATUS mwrite None write None rac None reset value false")
    }
    #[doc = "SRC: Source of HFCLK<br>"]
    pub(crate) fn apb0_hfclkstat40c_src_read(&self) -> MemResult<bool> {
        todo!("read SRC mwrite None write None rac None reset value false")
    }
    #[doc = "STATE: HFCLK state<br>"]
    pub(crate) fn apb0_hfclkstat40c_state_read(&self) -> MemResult<bool> {
        todo!("read STATE mwrite None write None rac None reset value false")
    }
    #[doc = "STATUS: LFCLKSTART task triggered or not<br>"]
    pub(crate) fn apb0_lfclkrun414_status_read(&self) -> MemResult<bool> {
        todo!("read STATUS mwrite None write None rac None reset value false")
    }
    #[doc = "SRC: Source of LFCLK<br>"]
    pub(crate) fn apb0_lfclkstat418_src_read(&self) -> MemResult<LfclkSrc> {
        todo ! ("read SRC mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "STATE: LFCLK state<br>"]
    pub(crate) fn apb0_lfclkstat418_state_read(&self) -> MemResult<bool> {
        todo!("read STATE mwrite None write None rac None reset value false")
    }
    #[doc = "SRC: Clock source<br>"]
    pub(crate) fn apb0_lfclksrccopy41c_src_read(&self) -> MemResult<LfclkSrc> {
        Ok(self.source)
    }
    #[doc = "RAMBLOCK0: RAM block 0 is on or off/powering up<br>"]
    pub(crate) fn apb0_ramstatus428_ramblock0_read(&self) -> MemResult<bool> {
        Ok(self.ram[0].is_on())
    }
    #[doc = "RAMBLOCK1: RAM block 1 is on or off/powering up<br>"]
    pub(crate) fn apb0_ramstatus428_ramblock1_read(&self) -> MemResult<bool> {
        Ok(self.ram[1].is_on())
    }
    #[doc = "RAMBLOCK2: RAM block 2 is on or off/powering up<br>"]
    pub(crate) fn apb0_ramstatus428_ramblock2_read(&self) -> MemResult<bool> {
        Ok(self.ram[2].is_on())
    }
    #[doc = "RAMBLOCK3: RAM block 3 is on or off/powering up<br>"]
    pub(crate) fn apb0_ramstatus428_ramblock3_read(&self) -> MemResult<bool> {
        Ok(self.ram[3].is_on())
    }
    #[doc = "SYSTEMOFF: Enable System OFF mode<br>"]
    pub(crate) fn apb0_systemoff500_systemoff_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write SYSTEMOFF mwrite None write None rac None reset value false"
        )
    }
    #[doc = "POF: Enable or disable power failure comparator<br>"]
    pub(crate) fn apb0_pofcon510_pof_read(&self) -> MemResult<bool> {
        todo!("read POF mwrite None write None rac None reset value false")
    }
    #[doc = "POF: Enable or disable power failure comparator<br>"]
    pub(crate) fn apb0_pofcon510_pof_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write POF mwrite None write None rac None reset value false")
    }
    #[doc = "THRESHOLD: Power failure comparator threshold setting<br>"]
    pub(crate) fn apb0_pofcon510_threshold_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E8Apb0Pofcon510Threshold> {
        todo ! ("read THRESHOLD mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "THRESHOLD: Power failure comparator threshold setting<br>"]
    pub(crate) fn apb0_pofcon510_threshold_write(
        &mut self,
        _value: crate::peripheral::enums::E8Apb0Pofcon510Threshold,
    ) -> MemResult<()> {
        todo ! ("write THRESHOLD mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "SRC: Clock source<br>"]
    pub(crate) fn apb0_lfclksrc518_src_read(&self) -> MemResult<LfclkSrc> {
        Ok(self.source)
    }
    #[doc = "SRC: Clock source<br>"]
    pub(crate) fn apb0_lfclksrc518_src_write(
        &mut self,
        _value: LfclkSrc,
    ) -> MemResult<()> {
        Ok(self.source = _value)
    }
    #[doc = "BYPASS: Enable or disable bypass of LFCLK crystal oscillator with external clock source<br>"]
    pub(crate) fn apb0_lfclksrc518_bypass_read(&self) -> MemResult<bool> {
        Ok(self.bypass)
    }
    #[doc = "BYPASS: Enable or disable bypass of LFCLK crystal oscillator with external clock source<br>"]
    pub(crate) fn apb0_lfclksrc518_bypass_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.bypass = _value)
    }
    #[doc = "EXTERNAL: Enable or disable external source for LFCLK<br>"]
    pub(crate) fn apb0_lfclksrc518_external_read(&self) -> MemResult<bool> {
        Ok(self.external)
    }
    #[doc = "EXTERNAL: Enable or disable external source for LFCLK<br>"]
    pub(crate) fn apb0_lfclksrc518_external_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.external = _value)
    }
    #[doc = "GPREGRET: General purpose retention register<br>"]
    pub(crate) fn apb0_gpregret51c_gpregret_read(&self) -> MemResult<u8> {
        todo ! ("read GPREGRET mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "GPREGRET: General purpose retention register<br>"]
    pub(crate) fn apb0_gpregret51c_gpregret_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write GPREGRET mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "GPREGRET: General purpose retention register<br>"]
    pub(crate) fn apb0_gpregret2520_gpregret_read(&self) -> MemResult<u8> {
        todo ! ("read GPREGRET mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "GPREGRET: General purpose retention register<br>"]
    pub(crate) fn apb0_gpregret2520_gpregret_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write GPREGRET mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "ONRAM0: Keep RAM block 0 on or off in system ON Mode<br>"]
    pub(crate) fn apb0_ramon524_onram0_read(&self) -> MemResult<bool> {
        Ok(self.ram[0].keep_on)
    }
    #[doc = "ONRAM0: Keep RAM block 0 on or off in system ON Mode<br>"]
    pub(crate) fn apb0_ramon524_onram0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ram[0].keep_on = _value)
    }
    #[doc = "ONRAM1: Keep RAM block 1 on or off in system ON Mode<br>"]
    pub(crate) fn apb0_ramon524_onram1_read(&self) -> MemResult<bool> {
        Ok(self.ram[1].keep_on)
    }
    #[doc = "ONRAM1: Keep RAM block 1 on or off in system ON Mode<br>"]
    pub(crate) fn apb0_ramon524_onram1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ram[1].keep_on = _value)
    }
    #[doc = "OFFRAM0: Keep retention on RAM block 0 when RAM block is switched off<br>"]
    pub(crate) fn apb0_ramon524_offram0_read(&self) -> MemResult<bool> {
        Ok(self.ram[0].retain)
    }
    #[doc = "OFFRAM0: Keep retention on RAM block 0 when RAM block is switched off<br>"]
    pub(crate) fn apb0_ramon524_offram0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ram[0].retain = _value)
    }
    #[doc = "OFFRAM1: Keep retention on RAM block 1 when RAM block is switched off<br>"]
    pub(crate) fn apb0_ramon524_offram1_read(&self) -> MemResult<bool> {
        Ok(self.ram[1].retain)
    }
    #[doc = "OFFRAM1: Keep retention on RAM block 1 when RAM block is switched off<br>"]
    pub(crate) fn apb0_ramon524_offram1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ram[1].retain = _value)
    }
    #[doc = "CTIV: Calibration timer interval in multiple of 0.25 seconds. Range: 0.25 seconds to 31.75 seconds.<br>"]
    pub(crate) fn apb0_ctiv538_ctiv_read(&self) -> MemResult<u8> {
        todo ! ("read CTIV mwrite None write None rac None reset value 0x00 mask 0x7f")
    }
    #[doc = "CTIV: Calibration timer interval in multiple of 0.25 seconds. Range: 0.25 seconds to 31.75 seconds.<br>"]
    pub(crate) fn apb0_ctiv538_ctiv_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write CTIV mwrite None write None rac None reset value 0x00 mask 0x7f")
    }
    #[doc = "ONRAM2: Keep RAM block 2 on or off in system ON Mode<br>"]
    pub(crate) fn apb0_ramonb554_onram2_read(&self) -> MemResult<bool> {
        Ok(self.ram[2].keep_on)
    }
    #[doc = "ONRAM2: Keep RAM block 2 on or off in system ON Mode<br>"]
    pub(crate) fn apb0_ramonb554_onram2_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ram[2].keep_on = _value)
    }
    #[doc = "ONRAM3: Keep RAM block 3 on or off in system ON Mode<br>"]
    pub(crate) fn apb0_ramonb554_onram3_read(&self) -> MemResult<bool> {
        Ok(self.ram[3].keep_on)
    }
    #[doc = "ONRAM3: Keep RAM block 3 on or off in system ON Mode<br>"]
    pub(crate) fn apb0_ramonb554_onram3_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ram[3].keep_on = _value)
    }
    #[doc = "OFFRAM2: Keep retention on RAM block 2 when RAM block is switched off<br>"]
    pub(crate) fn apb0_ramonb554_offram2_read(&self) -> MemResult<bool> {
        Ok(self.ram[2].retain)
    }
    #[doc = "OFFRAM2: Keep retention on RAM block 2 when RAM block is switched off<br>"]
    pub(crate) fn apb0_ramonb554_offram2_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ram[2].retain = _value)
    }
    #[doc = "OFFRAM3: Keep retention on RAM block 3 when RAM block is switched off<br>"]
    pub(crate) fn apb0_ramonb554_offram3_read(&self) -> MemResult<bool> {
        Ok(self.ram[3].retain)
    }
    #[doc = "OFFRAM3: Keep retention on RAM block 3 when RAM block is switched off<br>"]
    pub(crate) fn apb0_ramonb554_offram3_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.ram[3].retain = _value)
    }
    #[doc = "TRACEPORTSPEED: Speed of Trace Port clock. Note that the TRACECLK pin will output this clock divided by two.<br>"]
    pub(crate) fn apb0_traceconfig55c_traceportspeed_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E10Apb0Traceconfig55cTraceportspeed>
    {
        todo ! ("read TRACEPORTSPEED mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "TRACEPORTSPEED: Speed of Trace Port clock. Note that the TRACECLK pin will output this clock divided by two.<br>"]
    pub(crate) fn apb0_traceconfig55c_traceportspeed_write(
        &mut self,
        _value: crate::peripheral::enums::E10Apb0Traceconfig55cTraceportspeed,
    ) -> MemResult<()> {
        todo ! ("write TRACEPORTSPEED mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "TRACEMUX: Pin multiplexing of trace signals.<br>"]
    pub(crate) fn apb0_traceconfig55c_tracemux_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E11Apb0Traceconfig55cTracemux>
    {
        todo ! ("read TRACEMUX mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "TRACEMUX: Pin multiplexing of trace signals.<br>"]
    pub(crate) fn apb0_traceconfig55c_tracemux_write(
        &mut self,
        _value: crate::peripheral::enums::E11Apb0Traceconfig55cTracemux,
    ) -> MemResult<()> {
        todo ! ("write TRACEMUX mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "DCDCEN: Enable or disable DC/DC converter<br>"]
    pub(crate) fn apb0_dcdcen578_dcdcen_read(&self) -> MemResult<bool> {
        todo!("read DCDCEN mwrite None write None rac None reset value false")
    }
    #[doc = "DCDCEN: Enable or disable DC/DC converter<br>"]
    pub(crate) fn apb0_dcdcen578_dcdcen_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write DCDCEN mwrite None write None rac None reset value false")
    }
    #[doc = "REGION0: Enable protection for region 0. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region0_read(&self) -> MemResult<bool> {
        todo!("read REGION0 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION0: Enable protection for region 0. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write REGION0 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION1: Enable protection for region 1. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region1_read(&self) -> MemResult<bool> {
        todo!("read REGION1 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION1: Enable protection for region 1. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write REGION1 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION2: Enable protection for region 2. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region2_read(&self) -> MemResult<bool> {
        todo!("read REGION2 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION2: Enable protection for region 2. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region2_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write REGION2 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION3: Enable protection for region 3. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region3_read(&self) -> MemResult<bool> {
        todo!("read REGION3 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION3: Enable protection for region 3. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region3_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write REGION3 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION4: Enable protection for region 4. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region4_read(&self) -> MemResult<bool> {
        todo!("read REGION4 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION4: Enable protection for region 4. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region4_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write REGION4 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION5: Enable protection for region 5. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region5_read(&self) -> MemResult<bool> {
        todo!("read REGION5 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION5: Enable protection for region 5. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region5_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write REGION5 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION6: Enable protection for region 6. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region6_read(&self) -> MemResult<bool> {
        todo!("read REGION6 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION6: Enable protection for region 6. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region6_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write REGION6 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION7: Enable protection for region 7. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region7_read(&self) -> MemResult<bool> {
        todo!("read REGION7 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION7: Enable protection for region 7. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region7_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write REGION7 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION8: Enable protection for region 8. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region8_read(&self) -> MemResult<bool> {
        todo!("read REGION8 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION8: Enable protection for region 8. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region8_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write REGION8 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION9: Enable protection for region 9. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region9_read(&self) -> MemResult<bool> {
        todo!("read REGION9 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION9: Enable protection for region 9. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region9_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write REGION9 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION10: Enable protection for region 10. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region10_read(&self) -> MemResult<bool> {
        todo!("read REGION10 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION10: Enable protection for region 10. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region10_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION10 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION11: Enable protection for region 11. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region11_read(&self) -> MemResult<bool> {
        todo!("read REGION11 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION11: Enable protection for region 11. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region11_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION11 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION12: Enable protection for region 12. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region12_read(&self) -> MemResult<bool> {
        todo!("read REGION12 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION12: Enable protection for region 12. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region12_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION12 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION13: Enable protection for region 13. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region13_read(&self) -> MemResult<bool> {
        todo!("read REGION13 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION13: Enable protection for region 13. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region13_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION13 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION14: Enable protection for region 14. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region14_read(&self) -> MemResult<bool> {
        todo!("read REGION14 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION14: Enable protection for region 14. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region14_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION14 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION15: Enable protection for region 15. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region15_read(&self) -> MemResult<bool> {
        todo!("read REGION15 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION15: Enable protection for region 15. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region15_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION15 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION16: Enable protection for region 16. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region16_read(&self) -> MemResult<bool> {
        todo!("read REGION16 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION16: Enable protection for region 16. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region16_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION16 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION17: Enable protection for region 17. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region17_read(&self) -> MemResult<bool> {
        todo!("read REGION17 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION17: Enable protection for region 17. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region17_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION17 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION18: Enable protection for region 18. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region18_read(&self) -> MemResult<bool> {
        todo!("read REGION18 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION18: Enable protection for region 18. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region18_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION18 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION19: Enable protection for region 19. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region19_read(&self) -> MemResult<bool> {
        todo!("read REGION19 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION19: Enable protection for region 19. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region19_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION19 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION20: Enable protection for region 20. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region20_read(&self) -> MemResult<bool> {
        todo!("read REGION20 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION20: Enable protection for region 20. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region20_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION20 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION21: Enable protection for region 21. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region21_read(&self) -> MemResult<bool> {
        todo!("read REGION21 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION21: Enable protection for region 21. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region21_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION21 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION22: Enable protection for region 22. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region22_read(&self) -> MemResult<bool> {
        todo!("read REGION22 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION22: Enable protection for region 22. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region22_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION22 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION23: Enable protection for region 23. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region23_read(&self) -> MemResult<bool> {
        todo!("read REGION23 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION23: Enable protection for region 23. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region23_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION23 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION24: Enable protection for region 24. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region24_read(&self) -> MemResult<bool> {
        todo!("read REGION24 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION24: Enable protection for region 24. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region24_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION24 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION25: Enable protection for region 25. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region25_read(&self) -> MemResult<bool> {
        todo!("read REGION25 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION25: Enable protection for region 25. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region25_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION25 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION26: Enable protection for region 26. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region26_read(&self) -> MemResult<bool> {
        todo!("read REGION26 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION26: Enable protection for region 26. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region26_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION26 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION27: Enable protection for region 27. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region27_read(&self) -> MemResult<bool> {
        todo!("read REGION27 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION27: Enable protection for region 27. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region27_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION27 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION28: Enable protection for region 28. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region28_read(&self) -> MemResult<bool> {
        todo!("read REGION28 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION28: Enable protection for region 28. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region28_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION28 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION29: Enable protection for region 29. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region29_read(&self) -> MemResult<bool> {
        todo!("read REGION29 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION29: Enable protection for region 29. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region29_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION29 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION30: Enable protection for region 30. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region30_read(&self) -> MemResult<bool> {
        todo!("read REGION30 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION30: Enable protection for region 30. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region30_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION30 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION31: Enable protection for region 31. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region31_read(&self) -> MemResult<bool> {
        todo!("read REGION31 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION31: Enable protection for region 31. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config0600_region31_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION31 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION32: Enable protection for region 32. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region32_read(&self) -> MemResult<bool> {
        todo!("read REGION32 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION32: Enable protection for region 32. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region32_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION32 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION33: Enable protection for region 33. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region33_read(&self) -> MemResult<bool> {
        todo!("read REGION33 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION33: Enable protection for region 33. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region33_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION33 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION34: Enable protection for region 34. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region34_read(&self) -> MemResult<bool> {
        todo!("read REGION34 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION34: Enable protection for region 34. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region34_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION34 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION35: Enable protection for region 35. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region35_read(&self) -> MemResult<bool> {
        todo!("read REGION35 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION35: Enable protection for region 35. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region35_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION35 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION36: Enable protection for region 36. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region36_read(&self) -> MemResult<bool> {
        todo!("read REGION36 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION36: Enable protection for region 36. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region36_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION36 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION37: Enable protection for region 37. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region37_read(&self) -> MemResult<bool> {
        todo!("read REGION37 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION37: Enable protection for region 37. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region37_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION37 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION38: Enable protection for region 38. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region38_read(&self) -> MemResult<bool> {
        todo!("read REGION38 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION38: Enable protection for region 38. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region38_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION38 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION39: Enable protection for region 39. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region39_read(&self) -> MemResult<bool> {
        todo!("read REGION39 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION39: Enable protection for region 39. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region39_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION39 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION40: Enable protection for region 40. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region40_read(&self) -> MemResult<bool> {
        todo!("read REGION40 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION40: Enable protection for region 40. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region40_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION40 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION41: Enable protection for region 41. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region41_read(&self) -> MemResult<bool> {
        todo!("read REGION41 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION41: Enable protection for region 41. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region41_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION41 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION42: Enable protection for region 42. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region42_read(&self) -> MemResult<bool> {
        todo!("read REGION42 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION42: Enable protection for region 42. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region42_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION42 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION43: Enable protection for region 43. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region43_read(&self) -> MemResult<bool> {
        todo!("read REGION43 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION43: Enable protection for region 43. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region43_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION43 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION44: Enable protection for region 44. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region44_read(&self) -> MemResult<bool> {
        todo!("read REGION44 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION44: Enable protection for region 44. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region44_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION44 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION45: Enable protection for region 45. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region45_read(&self) -> MemResult<bool> {
        todo!("read REGION45 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION45: Enable protection for region 45. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region45_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION45 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION46: Enable protection for region 46. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region46_read(&self) -> MemResult<bool> {
        todo!("read REGION46 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION46: Enable protection for region 46. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region46_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION46 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION47: Enable protection for region 47. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region47_read(&self) -> MemResult<bool> {
        todo!("read REGION47 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION47: Enable protection for region 47. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region47_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION47 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION48: Enable protection for region 48. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region48_read(&self) -> MemResult<bool> {
        todo!("read REGION48 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION48: Enable protection for region 48. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region48_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION48 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION49: Enable protection for region 49. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region49_read(&self) -> MemResult<bool> {
        todo!("read REGION49 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION49: Enable protection for region 49. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region49_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION49 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION50: Enable protection for region 50. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region50_read(&self) -> MemResult<bool> {
        todo!("read REGION50 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION50: Enable protection for region 50. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region50_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION50 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION51: Enable protection for region 51. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region51_read(&self) -> MemResult<bool> {
        todo!("read REGION51 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION51: Enable protection for region 51. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region51_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION51 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION52: Enable protection for region 52. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region52_read(&self) -> MemResult<bool> {
        todo!("read REGION52 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION52: Enable protection for region 52. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region52_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION52 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION53: Enable protection for region 53. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region53_read(&self) -> MemResult<bool> {
        todo!("read REGION53 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION53: Enable protection for region 53. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region53_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION53 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION54: Enable protection for region 54. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region54_read(&self) -> MemResult<bool> {
        todo!("read REGION54 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION54: Enable protection for region 54. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region54_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION54 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION55: Enable protection for region 55. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region55_read(&self) -> MemResult<bool> {
        todo!("read REGION55 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION55: Enable protection for region 55. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region55_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION55 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION56: Enable protection for region 56. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region56_read(&self) -> MemResult<bool> {
        todo!("read REGION56 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION56: Enable protection for region 56. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region56_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION56 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION57: Enable protection for region 57. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region57_read(&self) -> MemResult<bool> {
        todo!("read REGION57 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION57: Enable protection for region 57. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region57_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION57 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION58: Enable protection for region 58. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region58_read(&self) -> MemResult<bool> {
        todo!("read REGION58 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION58: Enable protection for region 58. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region58_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION58 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION59: Enable protection for region 59. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region59_read(&self) -> MemResult<bool> {
        todo!("read REGION59 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION59: Enable protection for region 59. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region59_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION59 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION60: Enable protection for region 60. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region60_read(&self) -> MemResult<bool> {
        todo!("read REGION60 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION60: Enable protection for region 60. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region60_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION60 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION61: Enable protection for region 61. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region61_read(&self) -> MemResult<bool> {
        todo!("read REGION61 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION61: Enable protection for region 61. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region61_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION61 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION62: Enable protection for region 62. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region62_read(&self) -> MemResult<bool> {
        todo!("read REGION62 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION62: Enable protection for region 62. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region62_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION62 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION63: Enable protection for region 63. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region63_read(&self) -> MemResult<bool> {
        todo!("read REGION63 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION63: Enable protection for region 63. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config1604_region63_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION63 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "DISABLEINDEBUG: Disable the protection mechanism for NVM regions while in debug interface mode. This register will only disable the protection mechanism if the device is in debug interface mode.<br>"]
    pub(crate) fn apb0_disableindebug608_disableindebug_read(
        &self,
    ) -> MemResult<bool> {
        todo ! ("read DISABLEINDEBUG mwrite None write None rac None reset value true")
    }
    #[doc = "DISABLEINDEBUG: Disable the protection mechanism for NVM regions while in debug interface mode. This register will only disable the protection mechanism if the device is in debug interface mode.<br>"]
    pub(crate) fn apb0_disableindebug608_disableindebug_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write DISABLEINDEBUG mwrite None write None rac None reset value true")
    }
    #[doc = "UNUSED0: Unspecified<br>"]
    pub(crate) fn apb0_unused060c_read(&self) -> MemResult<u32> {
        todo ! ("read apb0_unused060c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "UNUSED0: Unspecified<br>"]
    pub(crate) fn apb0_unused060c_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb0_unused060c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "REGION64: Enable protection for region 64. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region64_read(&self) -> MemResult<bool> {
        todo!("read REGION64 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION64: Enable protection for region 64. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region64_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION64 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION65: Enable protection for region 65. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region65_read(&self) -> MemResult<bool> {
        todo!("read REGION65 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION65: Enable protection for region 65. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region65_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION65 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION66: Enable protection for region 66. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region66_read(&self) -> MemResult<bool> {
        todo!("read REGION66 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION66: Enable protection for region 66. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region66_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION66 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION67: Enable protection for region 67. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region67_read(&self) -> MemResult<bool> {
        todo!("read REGION67 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION67: Enable protection for region 67. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region67_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION67 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION68: Enable protection for region 68. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region68_read(&self) -> MemResult<bool> {
        todo!("read REGION68 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION68: Enable protection for region 68. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region68_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION68 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION69: Enable protection for region 69. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region69_read(&self) -> MemResult<bool> {
        todo!("read REGION69 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION69: Enable protection for region 69. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region69_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION69 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION70: Enable protection for region 70. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region70_read(&self) -> MemResult<bool> {
        todo!("read REGION70 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION70: Enable protection for region 70. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region70_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION70 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION71: Enable protection for region 71. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region71_read(&self) -> MemResult<bool> {
        todo!("read REGION71 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION71: Enable protection for region 71. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region71_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION71 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION72: Enable protection for region 72. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region72_read(&self) -> MemResult<bool> {
        todo!("read REGION72 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION72: Enable protection for region 72. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region72_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION72 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION73: Enable protection for region 73. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region73_read(&self) -> MemResult<bool> {
        todo!("read REGION73 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION73: Enable protection for region 73. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region73_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION73 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION74: Enable protection for region 74. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region74_read(&self) -> MemResult<bool> {
        todo!("read REGION74 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION74: Enable protection for region 74. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region74_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION74 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION75: Enable protection for region 75. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region75_read(&self) -> MemResult<bool> {
        todo!("read REGION75 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION75: Enable protection for region 75. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region75_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION75 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION76: Enable protection for region 76. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region76_read(&self) -> MemResult<bool> {
        todo!("read REGION76 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION76: Enable protection for region 76. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region76_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION76 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION77: Enable protection for region 77. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region77_read(&self) -> MemResult<bool> {
        todo!("read REGION77 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION77: Enable protection for region 77. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region77_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION77 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION78: Enable protection for region 78. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region78_read(&self) -> MemResult<bool> {
        todo!("read REGION78 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION78: Enable protection for region 78. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region78_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION78 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION79: Enable protection for region 79. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region79_read(&self) -> MemResult<bool> {
        todo!("read REGION79 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION79: Enable protection for region 79. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region79_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION79 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION80: Enable protection for region 80. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region80_read(&self) -> MemResult<bool> {
        todo!("read REGION80 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION80: Enable protection for region 80. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region80_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION80 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION81: Enable protection for region 81. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region81_read(&self) -> MemResult<bool> {
        todo!("read REGION81 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION81: Enable protection for region 81. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region81_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION81 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION82: Enable protection for region 82. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region82_read(&self) -> MemResult<bool> {
        todo!("read REGION82 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION82: Enable protection for region 82. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region82_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION82 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION83: Enable protection for region 83. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region83_read(&self) -> MemResult<bool> {
        todo!("read REGION83 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION83: Enable protection for region 83. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region83_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION83 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION84: Enable protection for region 84. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region84_read(&self) -> MemResult<bool> {
        todo!("read REGION84 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION84: Enable protection for region 84. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region84_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION84 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION85: Enable protection for region 85. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region85_read(&self) -> MemResult<bool> {
        todo!("read REGION85 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION85: Enable protection for region 85. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region85_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION85 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION86: Enable protection for region 86. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region86_read(&self) -> MemResult<bool> {
        todo!("read REGION86 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION86: Enable protection for region 86. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region86_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION86 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION87: Enable protection for region 87. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region87_read(&self) -> MemResult<bool> {
        todo!("read REGION87 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION87: Enable protection for region 87. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region87_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION87 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION88: Enable protection for region 88. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region88_read(&self) -> MemResult<bool> {
        todo!("read REGION88 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION88: Enable protection for region 88. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region88_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION88 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION89: Enable protection for region 89. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region89_read(&self) -> MemResult<bool> {
        todo!("read REGION89 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION89: Enable protection for region 89. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region89_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION89 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION90: Enable protection for region 90. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region90_read(&self) -> MemResult<bool> {
        todo!("read REGION90 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION90: Enable protection for region 90. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region90_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION90 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION91: Enable protection for region 91. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region91_read(&self) -> MemResult<bool> {
        todo!("read REGION91 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION91: Enable protection for region 91. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region91_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION91 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION92: Enable protection for region 92. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region92_read(&self) -> MemResult<bool> {
        todo!("read REGION92 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION92: Enable protection for region 92. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region92_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION92 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION93: Enable protection for region 93. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region93_read(&self) -> MemResult<bool> {
        todo!("read REGION93 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION93: Enable protection for region 93. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region93_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION93 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION94: Enable protection for region 94. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region94_read(&self) -> MemResult<bool> {
        todo!("read REGION94 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION94: Enable protection for region 94. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region94_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION94 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION95: Enable protection for region 95. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region95_read(&self) -> MemResult<bool> {
        todo!("read REGION95 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION95: Enable protection for region 95. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config2610_region95_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION95 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION96: Enable protection for region 96. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region96_read(&self) -> MemResult<bool> {
        todo!("read REGION96 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION96: Enable protection for region 96. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region96_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION96 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION97: Enable protection for region 97. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region97_read(&self) -> MemResult<bool> {
        todo!("read REGION97 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION97: Enable protection for region 97. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region97_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION97 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION98: Enable protection for region 98. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region98_read(&self) -> MemResult<bool> {
        todo!("read REGION98 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION98: Enable protection for region 98. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region98_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION98 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION99: Enable protection for region 99. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region99_read(&self) -> MemResult<bool> {
        todo!("read REGION99 mwrite None write None rac None reset value false")
    }
    #[doc = "REGION99: Enable protection for region 99. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region99_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION99 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION100: Enable protection for region 100. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region100_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION100 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION100: Enable protection for region 100. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region100_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION100 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION101: Enable protection for region 101. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region101_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION101 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION101: Enable protection for region 101. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region101_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION101 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION102: Enable protection for region 102. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region102_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION102 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION102: Enable protection for region 102. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region102_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION102 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION103: Enable protection for region 103. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region103_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION103 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION103: Enable protection for region 103. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region103_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION103 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION104: Enable protection for region 104. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region104_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION104 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION104: Enable protection for region 104. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region104_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION104 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION105: Enable protection for region 105. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region105_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION105 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION105: Enable protection for region 105. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region105_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION105 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION106: Enable protection for region 106. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region106_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION106 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION106: Enable protection for region 106. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region106_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION106 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION107: Enable protection for region 107. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region107_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION107 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION107: Enable protection for region 107. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region107_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION107 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION108: Enable protection for region 108. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region108_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION108 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION108: Enable protection for region 108. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region108_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION108 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION109: Enable protection for region 109. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region109_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION109 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION109: Enable protection for region 109. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region109_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION109 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION110: Enable protection for region 110. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region110_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION110 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION110: Enable protection for region 110. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region110_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION110 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION111: Enable protection for region 111. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region111_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION111 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION111: Enable protection for region 111. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region111_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION111 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION112: Enable protection for region 112. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region112_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION112 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION112: Enable protection for region 112. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region112_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION112 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION113: Enable protection for region 113. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region113_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION113 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION113: Enable protection for region 113. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region113_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION113 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION114: Enable protection for region 114. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region114_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION114 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION114: Enable protection for region 114. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region114_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION114 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION115: Enable protection for region 115. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region115_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION115 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION115: Enable protection for region 115. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region115_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION115 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION116: Enable protection for region 116. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region116_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION116 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION116: Enable protection for region 116. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region116_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION116 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION117: Enable protection for region 117. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region117_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION117 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION117: Enable protection for region 117. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region117_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION117 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION118: Enable protection for region 118. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region118_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION118 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION118: Enable protection for region 118. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region118_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION118 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION119: Enable protection for region 119. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region119_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION119 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION119: Enable protection for region 119. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region119_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION119 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION120: Enable protection for region 120. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region120_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION120 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION120: Enable protection for region 120. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region120_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION120 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION121: Enable protection for region 121. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region121_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION121 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION121: Enable protection for region 121. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region121_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION121 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION122: Enable protection for region 122. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region122_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION122 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION122: Enable protection for region 122. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region122_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION122 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION123: Enable protection for region 123. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region123_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION123 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION123: Enable protection for region 123. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region123_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION123 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION124: Enable protection for region 124. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region124_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION124 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION124: Enable protection for region 124. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region124_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION124 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION125: Enable protection for region 125. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region125_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION125 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION125: Enable protection for region 125. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region125_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION125 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION126: Enable protection for region 126. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region126_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION126 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION126: Enable protection for region 126. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region126_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION126 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION127: Enable protection for region 127. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region127_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION127 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION127: Enable protection for region 127. Write '0' has no effect.<br>"]
    pub(crate) fn apb0_config3614_region127_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION127 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "S0POWER: Keep RAM section S0 ON or OFF in System ON mode.<br>"]
    pub(crate) fn apb0_ramn_power0_s0power_read(
        &self,
        _ramn: usize,
    ) -> MemResult<bool> {
        todo!("read S0POWER mwrite None write None rac None reset value true")
    }
    #[doc = "S0POWER: Keep RAM section S0 ON or OFF in System ON mode.<br>"]
    pub(crate) fn apb0_ramn_power0_s0power_write(
        &mut self,
        _ramn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write S0POWER mwrite None write None rac None reset value true")
    }
    #[doc = "S1POWER: Keep RAM section S1 ON or OFF in System ON mode.<br>"]
    pub(crate) fn apb0_ramn_power0_s1power_read(
        &self,
        _ramn: usize,
    ) -> MemResult<bool> {
        todo!("read S1POWER mwrite None write None rac None reset value true")
    }
    #[doc = "S1POWER: Keep RAM section S1 ON or OFF in System ON mode.<br>"]
    pub(crate) fn apb0_ramn_power0_s1power_write(
        &mut self,
        _ramn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write S1POWER mwrite None write None rac None reset value true")
    }
    #[doc = "S0RETENTION: Keep retention on RAM section S0 when RAM section is in OFF<br>"]
    pub(crate) fn apb0_ramn_power0_s0retention_read(
        &self,
        _ramn: usize,
    ) -> MemResult<bool> {
        todo ! ("read S0RETENTION mwrite None write None rac None reset value false")
    }
    #[doc = "S0RETENTION: Keep retention on RAM section S0 when RAM section is in OFF<br>"]
    pub(crate) fn apb0_ramn_power0_s0retention_write(
        &mut self,
        _ramn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write S0RETENTION mwrite None write None rac None reset value false")
    }
    #[doc = "S1RETENTION: Keep retention on RAM section S1 when RAM section is in OFF<br>"]
    pub(crate) fn apb0_ramn_power0_s1retention_read(
        &self,
        _ramn: usize,
    ) -> MemResult<bool> {
        todo ! ("read S1RETENTION mwrite None write None rac None reset value false")
    }
    #[doc = "S1RETENTION: Keep retention on RAM section S1 when RAM section is in OFF<br>"]
    pub(crate) fn apb0_ramn_power0_s1retention_write(
        &mut self,
        _ramn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write S1RETENTION mwrite None write None rac None reset value false")
    }
    #[doc = "S0POWER: Keep RAM section S0 of RAM0 on or off in System ON mode<br>"]
    pub(crate) fn apb0_ramn_powerset4_s0power_write(
        &mut self,
        _ramn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write S0POWER mwrite None write None rac None reset value true")
    }
    #[doc = "S1POWER: Keep RAM section S1 of RAM0 on or off in System ON mode<br>"]
    pub(crate) fn apb0_ramn_powerset4_s1power_write(
        &mut self,
        _ramn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write S1POWER mwrite None write None rac None reset value true")
    }
    #[doc = "S0RETENTION: Keep retention on RAM section S0 when RAM section is switched off<br>"]
    pub(crate) fn apb0_ramn_powerset4_s0retention_write(
        &mut self,
        _ramn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write S0RETENTION mwrite None write None rac None reset value false")
    }
    #[doc = "S1RETENTION: Keep retention on RAM section S1 when RAM section is switched off<br>"]
    pub(crate) fn apb0_ramn_powerset4_s1retention_write(
        &mut self,
        _ramn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write S1RETENTION mwrite None write None rac None reset value false")
    }
    #[doc = "S0POWER: Keep RAM section S0 of RAM0 on or off in System ON mode<br>"]
    pub(crate) fn apb0_ramn_powerclr8_s0power_write(
        &mut self,
        _ramn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write S0POWER mwrite None write None rac None reset value true")
    }
    #[doc = "S1POWER: Keep RAM section S1 of RAM0 on or off in System ON mode<br>"]
    pub(crate) fn apb0_ramn_powerclr8_s1power_write(
        &mut self,
        _ramn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write S1POWER mwrite None write None rac None reset value true")
    }
    #[doc = "S0RETENTION: Keep retention on RAM section S0 when RAM section is switched off<br>"]
    pub(crate) fn apb0_ramn_powerclr8_s0retention_write(
        &mut self,
        _ramn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write S0RETENTION mwrite None write None rac None reset value false")
    }
    #[doc = "S1RETENTION: Keep retention on RAM section S1 when RAM section is switched off<br>"]
    pub(crate) fn apb0_ramn_powerclr8_s1retention_write(
        &mut self,
        _ramn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write S1RETENTION mwrite None write None rac None reset value false")
    }
}
