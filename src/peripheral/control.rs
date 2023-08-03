use icicle_vm::cpu::mem::{MemError, MemResult};

#[derive(Default)]
pub struct Control {
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
    priorities: [u8; 6],
}

impl Control {
    pub fn sleep_on_exit(&self) -> bool {
        self.sleep_on_exit
    }
    pub fn set_sleep_on_exit(&mut self, sleep: bool) {
        self.sleep_on_exit = sleep
    }
    pub fn sleep_deep(&self) -> bool {
        self.sleep_deep
    }
    pub fn set_sleep_deep(&mut self, deep: bool) {
        self.sleep_on_exit = deep
    }
    pub fn event_on_pending(&self) -> bool {
        self.event_on_pending
    }
    pub fn set_event_on_pending(&mut self, deep: bool) {
        self.event_on_pending = deep
    }
    pub fn priority(&self, handler: SysHandlerPriority) -> u8 {
        if (handler as usize) < 6 {
            self.priorities[handler as usize]
        } else {
            0
        }
    }
    pub fn set_priority(
        &mut self,
        handler: SysHandlerPriority,
        priority: u8,
    ) -> MemResult<()> {
        if (handler as usize) < 6 {
            self.priorities[handler as usize] = priority;
            Ok(())
        } else {
            Err(MemError::WriteViolation)
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
