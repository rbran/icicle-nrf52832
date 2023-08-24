use icicle_vm::cpu::mem::MemResult;
#[derive(Default)]
#[doc = "SPIM0: Serial Peripheral Interface Master with EasyDMA 0<br>SPIS0: SPI Slave 0<br>TWIM0: I2C compatible Two-Wire Master Interface with EasyDMA 0<br>TWIS0: I2C compatible Two-Wire Slave Interface with EasyDMA 0<br>SPI0: Serial Peripheral Interface 0<br>TWI0: I2C compatible Two-Wire Interface 0<br><br>Instances:<br>0x40003000: SPIM0, SPIS0, TWIM0, TWIS0, SPI0, TWI0<br>0x40004000: SPIM1, SPIS1, TWIM1, TWIS1, SPI1, TWI1<br>"]
pub struct Apb3 {
    #[doc = "TODO: implement things here"]
    _todo: (),
}
impl Apb3 {
    pub(crate) fn page_to_index(page: u64) -> usize {
        match page {
            262147u64 => 0usize,
            262148u64 => 1usize,
            _ => unreachable!(),
        }
    }
    #[doc = "TASKS_STARTRX: Start TWI receive sequence<br>TASKS_STARTRX: Start TWI receive sequence<br>"]
    pub(crate) fn apb3_tasks_startrx0_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb3_tasks_startrx0 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_STARTTX: Start TWI transmit sequence<br>TASKS_STARTTX: Start TWI transmit sequence<br>"]
    pub(crate) fn apb3_tasks_starttx8_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb3_tasks_starttx8 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_START: Start SPI transaction<br>"]
    pub(crate) fn apb3_tasks_start10_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb3_tasks_start10 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_STOP: Stop TWI transaction<br>TASKS_STOP: Stop SPI transaction<br>TASKS_STOP: Stop TWI transaction. Must be issued while the TWI master is not suspended.<br>TASKS_STOP: Stop TWI transaction<br>"]
    pub(crate) fn apb3_tasks_stop14_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb3_tasks_stop14 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_SUSPEND: Suspend SPI transaction<br>TASKS_SUSPEND: Suspend TWI transaction<br>TASKS_SUSPEND: Suspend TWI transaction<br>TASKS_SUSPEND: Suspend TWI transaction<br>"]
    pub(crate) fn apb3_tasks_suspend1c_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb3_tasks_suspend1c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_RESUME: Resume TWI transaction<br>TASKS_RESUME: Resume TWI transaction<br>TASKS_RESUME: Resume SPI transaction<br>TASKS_RESUME: Resume TWI transaction<br>"]
    pub(crate) fn apb3_tasks_resume20_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb3_tasks_resume20 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_ACQUIRE: Acquire SPI semaphore<br>"]
    pub(crate) fn apb3_tasks_acquire24_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb3_tasks_acquire24 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_RELEASE: Release SPI semaphore, enabling the SPI slave to acquire it<br>"]
    pub(crate) fn apb3_tasks_release28_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb3_tasks_release28 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_PREPARERX: Prepare the TWI slave to respond to a write command<br>"]
    pub(crate) fn apb3_tasks_preparerx30_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb3_tasks_preparerx30 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_PREPARETX: Prepare the TWI slave to respond to a read command<br>"]
    pub(crate) fn apb3_tasks_preparetx34_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb3_tasks_preparetx34 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_STOPPED: TWI stopped<br>EVENTS_END: Granted transaction completed<br>EVENTS_STOPPED: TWI stopped<br>EVENTS_STOPPED: TWI stopped<br>EVENTS_STOPPED: SPI transaction has stopped<br>"]
    pub(crate) fn apb3_events_stopped104_read(&self) -> MemResult<u32> {
        todo ! ("read apb3_events_stopped104 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_STOPPED: TWI stopped<br>EVENTS_END: Granted transaction completed<br>EVENTS_STOPPED: TWI stopped<br>EVENTS_STOPPED: TWI stopped<br>EVENTS_STOPPED: SPI transaction has stopped<br>"]
    pub(crate) fn apb3_events_stopped104_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb3_events_stopped104 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_READY: TXD byte sent and RXD byte received<br>EVENTS_RXDREADY: TWI RXD byte received<br>"]
    pub(crate) fn apb3_events_ready108_read(&self) -> MemResult<u32> {
        todo ! ("read apb3_events_ready108 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_READY: TXD byte sent and RXD byte received<br>EVENTS_RXDREADY: TWI RXD byte received<br>"]
    pub(crate) fn apb3_events_ready108_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb3_events_ready108 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ENDRX: End of RXD buffer reached<br>EVENTS_ENDRX: End of RXD buffer reached<br>"]
    pub(crate) fn apb3_events_endrx110_read(&self) -> MemResult<u32> {
        todo ! ("read apb3_events_endrx110 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ENDRX: End of RXD buffer reached<br>EVENTS_ENDRX: End of RXD buffer reached<br>"]
    pub(crate) fn apb3_events_endrx110_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb3_events_endrx110 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_END: End of RXD buffer and TXD buffer reached<br>"]
    pub(crate) fn apb3_events_end118_read(&self) -> MemResult<u32> {
        todo ! ("read apb3_events_end118 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_END: End of RXD buffer and TXD buffer reached<br>"]
    pub(crate) fn apb3_events_end118_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb3_events_end118 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_TXDSENT: TWI TXD byte sent<br>"]
    pub(crate) fn apb3_events_txdsent11c_read(&self) -> MemResult<u32> {
        todo ! ("read apb3_events_txdsent11c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_TXDSENT: TWI TXD byte sent<br>"]
    pub(crate) fn apb3_events_txdsent11c_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb3_events_txdsent11c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ENDTX: End of TXD buffer reached<br>"]
    pub(crate) fn apb3_events_endtx120_read(&self) -> MemResult<u32> {
        todo ! ("read apb3_events_endtx120 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ENDTX: End of TXD buffer reached<br>"]
    pub(crate) fn apb3_events_endtx120_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb3_events_endtx120 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ERROR: TWI error<br>EVENTS_ERROR: TWI error<br>EVENTS_ERROR: TWI error<br>"]
    pub(crate) fn apb3_events_error124_read(&self) -> MemResult<u32> {
        todo ! ("read apb3_events_error124 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ERROR: TWI error<br>EVENTS_ERROR: TWI error<br>EVENTS_ERROR: TWI error<br>"]
    pub(crate) fn apb3_events_error124_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb3_events_error124 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ACQUIRED: Semaphore acquired<br>"]
    pub(crate) fn apb3_events_acquired128_read(&self) -> MemResult<u32> {
        todo ! ("read apb3_events_acquired128 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ACQUIRED: Semaphore acquired<br>"]
    pub(crate) fn apb3_events_acquired128_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb3_events_acquired128 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_BB: TWI byte boundary, generated before each byte that is sent or received<br>"]
    pub(crate) fn apb3_events_bb138_read(&self) -> MemResult<u32> {
        todo ! ("read apb3_events_bb138 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_BB: TWI byte boundary, generated before each byte that is sent or received<br>"]
    pub(crate) fn apb3_events_bb138_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb3_events_bb138 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_SUSPENDED: TWI entered the suspended state<br>EVENTS_SUSPENDED: Last byte has been sent out after the SUSPEND task has been issued, TWI traffic is now suspended.<br>"]
    pub(crate) fn apb3_events_suspended148_read(&self) -> MemResult<u32> {
        todo ! ("read apb3_events_suspended148 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_SUSPENDED: TWI entered the suspended state<br>EVENTS_SUSPENDED: Last byte has been sent out after the SUSPEND task has been issued, TWI traffic is now suspended.<br>"]
    pub(crate) fn apb3_events_suspended148_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb3_events_suspended148 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_STARTED: Transaction started<br>EVENTS_RXSTARTED: Receive sequence started<br>EVENTS_RXSTARTED: Receive sequence started<br>"]
    pub(crate) fn apb3_events_started14c_read(&self) -> MemResult<u32> {
        todo ! ("read apb3_events_started14c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_STARTED: Transaction started<br>EVENTS_RXSTARTED: Receive sequence started<br>EVENTS_RXSTARTED: Receive sequence started<br>"]
    pub(crate) fn apb3_events_started14c_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb3_events_started14c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_TXSTARTED: Transmit sequence started<br>EVENTS_TXSTARTED: Transmit sequence started<br>"]
    pub(crate) fn apb3_events_txstarted150_read(&self) -> MemResult<u32> {
        todo ! ("read apb3_events_txstarted150 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_TXSTARTED: Transmit sequence started<br>EVENTS_TXSTARTED: Transmit sequence started<br>"]
    pub(crate) fn apb3_events_txstarted150_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb3_events_txstarted150 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_LASTRX: Byte boundary, starting to receive the last byte<br>"]
    pub(crate) fn apb3_events_lastrx15c_read(&self) -> MemResult<u32> {
        todo ! ("read apb3_events_lastrx15c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_LASTRX: Byte boundary, starting to receive the last byte<br>"]
    pub(crate) fn apb3_events_lastrx15c_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb3_events_lastrx15c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_LASTTX: Byte boundary, starting to transmit the last byte<br>"]
    pub(crate) fn apb3_events_lasttx160_read(&self) -> MemResult<u32> {
        todo ! ("read apb3_events_lasttx160 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_LASTTX: Byte boundary, starting to transmit the last byte<br>"]
    pub(crate) fn apb3_events_lasttx160_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb3_events_lasttx160 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_WRITE: Write command received<br>"]
    pub(crate) fn apb3_events_write164_read(&self) -> MemResult<u32> {
        todo ! ("read apb3_events_write164 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_WRITE: Write command received<br>"]
    pub(crate) fn apb3_events_write164_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb3_events_write164 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_READ: Read command received<br>"]
    pub(crate) fn apb3_events_read168_read(&self) -> MemResult<u32> {
        todo ! ("read apb3_events_read168 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_READ: Read command received<br>"]
    pub(crate) fn apb3_events_read168_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb3_events_read168 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "BB_SUSPEND: Shortcut between BB event and SUSPEND task<br>"]
    pub(crate) fn apb3_shorts200_bb_suspend_read(&self) -> MemResult<bool> {
        todo!(
            "read BB_SUSPEND mwrite None write None rac None reset value false"
        )
    }
    #[doc = "BB_SUSPEND: Shortcut between BB event and SUSPEND task<br>"]
    pub(crate) fn apb3_shorts200_bb_suspend_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write BB_SUSPEND mwrite None write None rac None reset value false")
    }
    #[doc = "BB_STOP: Shortcut between BB event and STOP task<br>"]
    pub(crate) fn apb3_shorts200_bb_stop_read(&self) -> MemResult<bool> {
        todo!("read BB_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "BB_STOP: Shortcut between BB event and STOP task<br>"]
    pub(crate) fn apb3_shorts200_bb_stop_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write BB_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "END_ACQUIRE: Shortcut between END event and ACQUIRE task<br>"]
    pub(crate) fn apb3_shorts200_end_acquire_read(&self) -> MemResult<bool> {
        todo ! ("read END_ACQUIRE mwrite None write None rac None reset value false")
    }
    #[doc = "END_ACQUIRE: Shortcut between END event and ACQUIRE task<br>"]
    pub(crate) fn apb3_shorts200_end_acquire_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write END_ACQUIRE mwrite None write None rac None reset value false")
    }
    #[doc = "LASTTX_STARTRX: Shortcut between LASTTX event and STARTRX task<br>"]
    pub(crate) fn apb3_shorts200_lasttx_startrx_read(&self) -> MemResult<bool> {
        todo ! ("read LASTTX_STARTRX mwrite None write None rac None reset value false")
    }
    #[doc = "LASTTX_STARTRX: Shortcut between LASTTX event and STARTRX task<br>"]
    pub(crate) fn apb3_shorts200_lasttx_startrx_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write LASTTX_STARTRX mwrite None write None rac None reset value false")
    }
    #[doc = "LASTTX_SUSPEND: Shortcut between LASTTX event and SUSPEND task<br>"]
    pub(crate) fn apb3_shorts200_lasttx_suspend_read(&self) -> MemResult<bool> {
        todo ! ("read LASTTX_SUSPEND mwrite None write None rac None reset value false")
    }
    #[doc = "LASTTX_SUSPEND: Shortcut between LASTTX event and SUSPEND task<br>"]
    pub(crate) fn apb3_shorts200_lasttx_suspend_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write LASTTX_SUSPEND mwrite None write None rac None reset value false")
    }
    #[doc = "LASTTX_STOP: Shortcut between LASTTX event and STOP task<br>"]
    pub(crate) fn apb3_shorts200_lasttx_stop_read(&self) -> MemResult<bool> {
        todo ! ("read LASTTX_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "LASTTX_STOP: Shortcut between LASTTX event and STOP task<br>"]
    pub(crate) fn apb3_shorts200_lasttx_stop_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write LASTTX_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "LASTRX_STARTTX: Shortcut between LASTRX event and STARTTX task<br>"]
    pub(crate) fn apb3_shorts200_lastrx_starttx_read(&self) -> MemResult<bool> {
        todo ! ("read LASTRX_STARTTX mwrite None write None rac None reset value false")
    }
    #[doc = "LASTRX_STARTTX: Shortcut between LASTRX event and STARTTX task<br>"]
    pub(crate) fn apb3_shorts200_lastrx_starttx_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write LASTRX_STARTTX mwrite None write None rac None reset value false")
    }
    #[doc = "LASTRX_STOP: Shortcut between LASTRX event and STOP task<br>"]
    pub(crate) fn apb3_shorts200_lastrx_stop_read(&self) -> MemResult<bool> {
        todo ! ("read LASTRX_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "LASTRX_STOP: Shortcut between LASTRX event and STOP task<br>"]
    pub(crate) fn apb3_shorts200_lastrx_stop_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write LASTRX_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "WRITE_SUSPEND: Shortcut between WRITE event and SUSPEND task<br>"]
    pub(crate) fn apb3_shorts200_write_suspend_read(&self) -> MemResult<bool> {
        todo ! ("read WRITE_SUSPEND mwrite None write None rac None reset value false")
    }
    #[doc = "WRITE_SUSPEND: Shortcut between WRITE event and SUSPEND task<br>"]
    pub(crate) fn apb3_shorts200_write_suspend_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write WRITE_SUSPEND mwrite None write None rac None reset value false")
    }
    #[doc = "READ_SUSPEND: Shortcut between READ event and SUSPEND task<br>"]
    pub(crate) fn apb3_shorts200_read_suspend_read(&self) -> MemResult<bool> {
        todo ! ("read READ_SUSPEND mwrite None write None rac None reset value false")
    }
    #[doc = "READ_SUSPEND: Shortcut between READ event and SUSPEND task<br>"]
    pub(crate) fn apb3_shorts200_read_suspend_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write READ_SUSPEND mwrite None write None rac None reset value false")
    }
    #[doc = "END_START: Shortcut between END event and START task<br>"]
    pub(crate) fn apb3_shorts200_end_start_read(&self) -> MemResult<bool> {
        todo!(
            "read END_START mwrite None write None rac None reset value false"
        )
    }
    #[doc = "END_START: Shortcut between END event and START task<br>"]
    pub(crate) fn apb3_shorts200_end_start_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write END_START mwrite None write None rac None reset value false"
        )
    }
    #[doc = "STOPPED: Enable or disable interrupt for STOPPED event<br>STOPPED: Enable or disable interrupt for STOPPED event<br>"]
    pub(crate) fn apb3_inten300_stopped_read(&self) -> MemResult<bool> {
        todo ! ("read STOPPED, STOPPED mwrite None write None rac None reset value false")
    }
    #[doc = "STOPPED: Enable or disable interrupt for STOPPED event<br>STOPPED: Enable or disable interrupt for STOPPED event<br>"]
    pub(crate) fn apb3_inten300_stopped_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write STOPPED, STOPPED mwrite None write None rac None reset value false")
    }
    #[doc = "ERROR: Enable or disable interrupt for ERROR event<br>ERROR: Enable or disable interrupt for ERROR event<br>"]
    pub(crate) fn apb3_inten300_error_read(&self) -> MemResult<bool> {
        todo ! ("read ERROR, ERROR mwrite None write None rac None reset value false")
    }
    #[doc = "ERROR: Enable or disable interrupt for ERROR event<br>ERROR: Enable or disable interrupt for ERROR event<br>"]
    pub(crate) fn apb3_inten300_error_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write ERROR, ERROR mwrite None write None rac None reset value false")
    }
    #[doc = "SUSPENDED: Enable or disable interrupt for SUSPENDED event<br>"]
    pub(crate) fn apb3_inten300_suspended_read(&self) -> MemResult<bool> {
        todo!(
            "read SUSPENDED mwrite None write None rac None reset value false"
        )
    }
    #[doc = "SUSPENDED: Enable or disable interrupt for SUSPENDED event<br>"]
    pub(crate) fn apb3_inten300_suspended_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write SUSPENDED mwrite None write None rac None reset value false"
        )
    }
    #[doc = "RXSTARTED: Enable or disable interrupt for RXSTARTED event<br>RXSTARTED: Enable or disable interrupt for RXSTARTED event<br>"]
    pub(crate) fn apb3_inten300_rxstarted_read(&self) -> MemResult<bool> {
        todo ! ("read RXSTARTED, RXSTARTED mwrite None write None rac None reset value false")
    }
    #[doc = "RXSTARTED: Enable or disable interrupt for RXSTARTED event<br>RXSTARTED: Enable or disable interrupt for RXSTARTED event<br>"]
    pub(crate) fn apb3_inten300_rxstarted_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RXSTARTED, RXSTARTED mwrite None write None rac None reset value false")
    }
    #[doc = "TXSTARTED: Enable or disable interrupt for TXSTARTED event<br>TXSTARTED: Enable or disable interrupt for TXSTARTED event<br>"]
    pub(crate) fn apb3_inten300_txstarted_read(&self) -> MemResult<bool> {
        todo ! ("read TXSTARTED, TXSTARTED mwrite None write None rac None reset value false")
    }
    #[doc = "TXSTARTED: Enable or disable interrupt for TXSTARTED event<br>TXSTARTED: Enable or disable interrupt for TXSTARTED event<br>"]
    pub(crate) fn apb3_inten300_txstarted_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TXSTARTED, TXSTARTED mwrite None write None rac None reset value false")
    }
    #[doc = "LASTRX: Enable or disable interrupt for LASTRX event<br>"]
    pub(crate) fn apb3_inten300_lastrx_read(&self) -> MemResult<bool> {
        todo!("read LASTRX mwrite None write None rac None reset value false")
    }
    #[doc = "LASTRX: Enable or disable interrupt for LASTRX event<br>"]
    pub(crate) fn apb3_inten300_lastrx_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write LASTRX mwrite None write None rac None reset value false")
    }
    #[doc = "LASTTX: Enable or disable interrupt for LASTTX event<br>"]
    pub(crate) fn apb3_inten300_lasttx_read(&self) -> MemResult<bool> {
        todo!("read LASTTX mwrite None write None rac None reset value false")
    }
    #[doc = "LASTTX: Enable or disable interrupt for LASTTX event<br>"]
    pub(crate) fn apb3_inten300_lasttx_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write LASTTX mwrite None write None rac None reset value false")
    }
    #[doc = "WRITE: Enable or disable interrupt for WRITE event<br>"]
    pub(crate) fn apb3_inten300_write_read(&self) -> MemResult<bool> {
        todo!("read WRITE mwrite None write None rac None reset value false")
    }
    #[doc = "WRITE: Enable or disable interrupt for WRITE event<br>"]
    pub(crate) fn apb3_inten300_write_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write WRITE mwrite None write None rac None reset value false")
    }
    #[doc = "READ: Enable or disable interrupt for READ event<br>"]
    pub(crate) fn apb3_inten300_read_read(&self) -> MemResult<bool> {
        todo!("read READ mwrite None write None rac None reset value false")
    }
    #[doc = "READ: Enable or disable interrupt for READ event<br>"]
    pub(crate) fn apb3_inten300_read_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write READ mwrite None write None rac None reset value false")
    }
    #[doc = "STOPPED: Write '1' to Enable interrupt for STOPPED event<br>STOPPED: Write '1' to Enable interrupt for STOPPED event<br>STOPPED: Write '1' to Enable interrupt for STOPPED event<br>STOPPED: Write '1' to Enable interrupt for STOPPED event<br>END: Write '1' to Enable interrupt for END event<br>"]
    pub(crate) fn apb3_intenset304_stopped_read(&self) -> MemResult<bool> {
        todo ! ("read STOPPED, STOPPED, STOPPED, STOPPED, END mwrite None write None rac None reset value false")
    }
    #[doc = "STOPPED: Write '1' to Enable interrupt for STOPPED event<br>STOPPED: Write '1' to Enable interrupt for STOPPED event<br>STOPPED: Write '1' to Enable interrupt for STOPPED event<br>STOPPED: Write '1' to Enable interrupt for STOPPED event<br>END: Write '1' to Enable interrupt for END event<br>"]
    pub(crate) fn apb3_intenset304_stopped_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write STOPPED, STOPPED, STOPPED, STOPPED, END mwrite None write None rac None reset value false")
    }
    #[doc = "READY: Write '1' to Enable interrupt for READY event<br>RXDREADY: Write '1' to Enable interrupt for RXDREADY event<br>"]
    pub(crate) fn apb3_intenset304_ready_read(&self) -> MemResult<bool> {
        todo ! ("read READY, RXDREADY mwrite None write None rac None reset value false")
    }
    #[doc = "READY: Write '1' to Enable interrupt for READY event<br>RXDREADY: Write '1' to Enable interrupt for RXDREADY event<br>"]
    pub(crate) fn apb3_intenset304_ready_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write READY, RXDREADY mwrite None write None rac None reset value false")
    }
    #[doc = "ENDRX: Write '1' to Enable interrupt for ENDRX event<br>ENDRX: Write '1' to Enable interrupt for ENDRX event<br>"]
    pub(crate) fn apb3_intenset304_endrx_read(&self) -> MemResult<bool> {
        todo ! ("read ENDRX, ENDRX mwrite None write None rac None reset value false")
    }
    #[doc = "ENDRX: Write '1' to Enable interrupt for ENDRX event<br>ENDRX: Write '1' to Enable interrupt for ENDRX event<br>"]
    pub(crate) fn apb3_intenset304_endrx_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write ENDRX, ENDRX mwrite None write None rac None reset value false")
    }
    #[doc = "END: Write '1' to Enable interrupt for END event<br>"]
    pub(crate) fn apb3_intenset304_end_read(&self) -> MemResult<bool> {
        todo!("read END mwrite None write None rac None reset value false")
    }
    #[doc = "END: Write '1' to Enable interrupt for END event<br>"]
    pub(crate) fn apb3_intenset304_end_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write END mwrite None write None rac None reset value false")
    }
    #[doc = "TXDSENT: Write '1' to Enable interrupt for TXDSENT event<br>"]
    pub(crate) fn apb3_intenset304_txdsent_read(&self) -> MemResult<bool> {
        todo!("read TXDSENT mwrite None write None rac None reset value false")
    }
    #[doc = "TXDSENT: Write '1' to Enable interrupt for TXDSENT event<br>"]
    pub(crate) fn apb3_intenset304_txdsent_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write TXDSENT mwrite None write None rac None reset value false")
    }
    #[doc = "ENDTX: Write '1' to Enable interrupt for ENDTX event<br>"]
    pub(crate) fn apb3_intenset304_endtx_read(&self) -> MemResult<bool> {
        todo!("read ENDTX mwrite None write None rac None reset value false")
    }
    #[doc = "ENDTX: Write '1' to Enable interrupt for ENDTX event<br>"]
    pub(crate) fn apb3_intenset304_endtx_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ENDTX mwrite None write None rac None reset value false")
    }
    #[doc = "ERROR: Write '1' to Enable interrupt for ERROR event<br>ERROR: Write '1' to Enable interrupt for ERROR event<br>ERROR: Write '1' to Enable interrupt for ERROR event<br>"]
    pub(crate) fn apb3_intenset304_error_read(&self) -> MemResult<bool> {
        todo ! ("read ERROR, ERROR, ERROR mwrite None write None rac None reset value false")
    }
    #[doc = "ERROR: Write '1' to Enable interrupt for ERROR event<br>ERROR: Write '1' to Enable interrupt for ERROR event<br>ERROR: Write '1' to Enable interrupt for ERROR event<br>"]
    pub(crate) fn apb3_intenset304_error_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write ERROR, ERROR, ERROR mwrite None write None rac None reset value false")
    }
    #[doc = "ACQUIRED: Write '1' to Enable interrupt for ACQUIRED event<br>"]
    pub(crate) fn apb3_intenset304_acquired_read(&self) -> MemResult<bool> {
        todo!("read ACQUIRED mwrite None write None rac None reset value false")
    }
    #[doc = "ACQUIRED: Write '1' to Enable interrupt for ACQUIRED event<br>"]
    pub(crate) fn apb3_intenset304_acquired_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write ACQUIRED mwrite None write None rac None reset value false"
        )
    }
    #[doc = "BB: Write '1' to Enable interrupt for BB event<br>"]
    pub(crate) fn apb3_intenset304_bb_read(&self) -> MemResult<bool> {
        todo!("read BB mwrite None write None rac None reset value false")
    }
    #[doc = "BB: Write '1' to Enable interrupt for BB event<br>"]
    pub(crate) fn apb3_intenset304_bb_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write BB mwrite None write None rac None reset value false")
    }
    #[doc = "SUSPENDED: Write '1' to Enable interrupt for SUSPENDED event<br>SUSPENDED: Write '1' to Enable interrupt for SUSPENDED event<br>"]
    pub(crate) fn apb3_intenset304_suspended_read(&self) -> MemResult<bool> {
        todo ! ("read SUSPENDED, SUSPENDED mwrite None write None rac None reset value false")
    }
    #[doc = "SUSPENDED: Write '1' to Enable interrupt for SUSPENDED event<br>SUSPENDED: Write '1' to Enable interrupt for SUSPENDED event<br>"]
    pub(crate) fn apb3_intenset304_suspended_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write SUSPENDED, SUSPENDED mwrite None write None rac None reset value false")
    }
    #[doc = "RXSTARTED: Write '1' to Enable interrupt for RXSTARTED event<br>RXSTARTED: Write '1' to Enable interrupt for RXSTARTED event<br>STARTED: Write '1' to Enable interrupt for STARTED event<br>"]
    pub(crate) fn apb3_intenset304_rxstarted_read(&self) -> MemResult<bool> {
        todo ! ("read RXSTARTED, RXSTARTED, STARTED mwrite None write None rac None reset value false")
    }
    #[doc = "RXSTARTED: Write '1' to Enable interrupt for RXSTARTED event<br>RXSTARTED: Write '1' to Enable interrupt for RXSTARTED event<br>STARTED: Write '1' to Enable interrupt for STARTED event<br>"]
    pub(crate) fn apb3_intenset304_rxstarted_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RXSTARTED, RXSTARTED, STARTED mwrite None write None rac None reset value false")
    }
    #[doc = "TXSTARTED: Write '1' to Enable interrupt for TXSTARTED event<br>TXSTARTED: Write '1' to Enable interrupt for TXSTARTED event<br>"]
    pub(crate) fn apb3_intenset304_txstarted_read(&self) -> MemResult<bool> {
        todo ! ("read TXSTARTED, TXSTARTED mwrite None write None rac None reset value false")
    }
    #[doc = "TXSTARTED: Write '1' to Enable interrupt for TXSTARTED event<br>TXSTARTED: Write '1' to Enable interrupt for TXSTARTED event<br>"]
    pub(crate) fn apb3_intenset304_txstarted_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TXSTARTED, TXSTARTED mwrite None write None rac None reset value false")
    }
    #[doc = "LASTRX: Write '1' to Enable interrupt for LASTRX event<br>"]
    pub(crate) fn apb3_intenset304_lastrx_read(&self) -> MemResult<bool> {
        todo!("read LASTRX mwrite None write None rac None reset value false")
    }
    #[doc = "LASTRX: Write '1' to Enable interrupt for LASTRX event<br>"]
    pub(crate) fn apb3_intenset304_lastrx_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write LASTRX mwrite None write None rac None reset value false")
    }
    #[doc = "LASTTX: Write '1' to Enable interrupt for LASTTX event<br>"]
    pub(crate) fn apb3_intenset304_lasttx_read(&self) -> MemResult<bool> {
        todo!("read LASTTX mwrite None write None rac None reset value false")
    }
    #[doc = "LASTTX: Write '1' to Enable interrupt for LASTTX event<br>"]
    pub(crate) fn apb3_intenset304_lasttx_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write LASTTX mwrite None write None rac None reset value false")
    }
    #[doc = "WRITE: Write '1' to Enable interrupt for WRITE event<br>"]
    pub(crate) fn apb3_intenset304_write_read(&self) -> MemResult<bool> {
        todo!("read WRITE mwrite None write None rac None reset value false")
    }
    #[doc = "WRITE: Write '1' to Enable interrupt for WRITE event<br>"]
    pub(crate) fn apb3_intenset304_write_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write WRITE mwrite None write None rac None reset value false")
    }
    #[doc = "READ: Write '1' to Enable interrupt for READ event<br>"]
    pub(crate) fn apb3_intenset304_read_read(&self) -> MemResult<bool> {
        todo!("read READ mwrite None write None rac None reset value false")
    }
    #[doc = "READ: Write '1' to Enable interrupt for READ event<br>"]
    pub(crate) fn apb3_intenset304_read_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write READ mwrite None write None rac None reset value false")
    }
    #[doc = "STOPPED: Write '1' to Disable interrupt for STOPPED event<br>STOPPED: Write '1' to Disable interrupt for STOPPED event<br>END: Write '1' to Disable interrupt for END event<br>STOPPED: Write '1' to Disable interrupt for STOPPED event<br>STOPPED: Write '1' to Disable interrupt for STOPPED event<br>"]
    pub(crate) fn apb3_intenclr308_stopped_read(&self) -> MemResult<bool> {
        todo ! ("read STOPPED, STOPPED, END, STOPPED, STOPPED mwrite None write None rac None reset value false")
    }
    #[doc = "STOPPED: Write '1' to Disable interrupt for STOPPED event<br>STOPPED: Write '1' to Disable interrupt for STOPPED event<br>END: Write '1' to Disable interrupt for END event<br>STOPPED: Write '1' to Disable interrupt for STOPPED event<br>STOPPED: Write '1' to Disable interrupt for STOPPED event<br>"]
    pub(crate) fn apb3_intenclr308_stopped_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write STOPPED, STOPPED, END, STOPPED, STOPPED mwrite None write None rac None reset value false")
    }
    #[doc = "READY: Write '1' to Disable interrupt for READY event<br>RXDREADY: Write '1' to Disable interrupt for RXDREADY event<br>"]
    pub(crate) fn apb3_intenclr308_ready_read(&self) -> MemResult<bool> {
        todo ! ("read READY, RXDREADY mwrite None write None rac None reset value false")
    }
    #[doc = "READY: Write '1' to Disable interrupt for READY event<br>RXDREADY: Write '1' to Disable interrupt for RXDREADY event<br>"]
    pub(crate) fn apb3_intenclr308_ready_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write READY, RXDREADY mwrite None write None rac None reset value false")
    }
    #[doc = "ENDRX: Write '1' to Disable interrupt for ENDRX event<br>ENDRX: Write '1' to Disable interrupt for ENDRX event<br>"]
    pub(crate) fn apb3_intenclr308_endrx_read(&self) -> MemResult<bool> {
        todo ! ("read ENDRX, ENDRX mwrite None write None rac None reset value false")
    }
    #[doc = "ENDRX: Write '1' to Disable interrupt for ENDRX event<br>ENDRX: Write '1' to Disable interrupt for ENDRX event<br>"]
    pub(crate) fn apb3_intenclr308_endrx_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write ENDRX, ENDRX mwrite None write None rac None reset value false")
    }
    #[doc = "END: Write '1' to Disable interrupt for END event<br>"]
    pub(crate) fn apb3_intenclr308_end_read(&self) -> MemResult<bool> {
        todo!("read END mwrite None write None rac None reset value false")
    }
    #[doc = "END: Write '1' to Disable interrupt for END event<br>"]
    pub(crate) fn apb3_intenclr308_end_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write END mwrite None write None rac None reset value false")
    }
    #[doc = "TXDSENT: Write '1' to Disable interrupt for TXDSENT event<br>"]
    pub(crate) fn apb3_intenclr308_txdsent_read(&self) -> MemResult<bool> {
        todo!("read TXDSENT mwrite None write None rac None reset value false")
    }
    #[doc = "TXDSENT: Write '1' to Disable interrupt for TXDSENT event<br>"]
    pub(crate) fn apb3_intenclr308_txdsent_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write TXDSENT mwrite None write None rac None reset value false")
    }
    #[doc = "ENDTX: Write '1' to Disable interrupt for ENDTX event<br>"]
    pub(crate) fn apb3_intenclr308_endtx_read(&self) -> MemResult<bool> {
        todo!("read ENDTX mwrite None write None rac None reset value false")
    }
    #[doc = "ENDTX: Write '1' to Disable interrupt for ENDTX event<br>"]
    pub(crate) fn apb3_intenclr308_endtx_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ENDTX mwrite None write None rac None reset value false")
    }
    #[doc = "ERROR: Write '1' to Disable interrupt for ERROR event<br>ERROR: Write '1' to Disable interrupt for ERROR event<br>ERROR: Write '1' to Disable interrupt for ERROR event<br>"]
    pub(crate) fn apb3_intenclr308_error_read(&self) -> MemResult<bool> {
        todo ! ("read ERROR, ERROR, ERROR mwrite None write None rac None reset value false")
    }
    #[doc = "ERROR: Write '1' to Disable interrupt for ERROR event<br>ERROR: Write '1' to Disable interrupt for ERROR event<br>ERROR: Write '1' to Disable interrupt for ERROR event<br>"]
    pub(crate) fn apb3_intenclr308_error_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write ERROR, ERROR, ERROR mwrite None write None rac None reset value false")
    }
    #[doc = "ACQUIRED: Write '1' to Disable interrupt for ACQUIRED event<br>"]
    pub(crate) fn apb3_intenclr308_acquired_read(&self) -> MemResult<bool> {
        todo!("read ACQUIRED mwrite None write None rac None reset value false")
    }
    #[doc = "ACQUIRED: Write '1' to Disable interrupt for ACQUIRED event<br>"]
    pub(crate) fn apb3_intenclr308_acquired_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write ACQUIRED mwrite None write None rac None reset value false"
        )
    }
    #[doc = "BB: Write '1' to Disable interrupt for BB event<br>"]
    pub(crate) fn apb3_intenclr308_bb_read(&self) -> MemResult<bool> {
        todo!("read BB mwrite None write None rac None reset value false")
    }
    #[doc = "BB: Write '1' to Disable interrupt for BB event<br>"]
    pub(crate) fn apb3_intenclr308_bb_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write BB mwrite None write None rac None reset value false")
    }
    #[doc = "SUSPENDED: Write '1' to Disable interrupt for SUSPENDED event<br>SUSPENDED: Write '1' to Disable interrupt for SUSPENDED event<br>"]
    pub(crate) fn apb3_intenclr308_suspended_read(&self) -> MemResult<bool> {
        todo ! ("read SUSPENDED, SUSPENDED mwrite None write None rac None reset value false")
    }
    #[doc = "SUSPENDED: Write '1' to Disable interrupt for SUSPENDED event<br>SUSPENDED: Write '1' to Disable interrupt for SUSPENDED event<br>"]
    pub(crate) fn apb3_intenclr308_suspended_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write SUSPENDED, SUSPENDED mwrite None write None rac None reset value false")
    }
    #[doc = "RXSTARTED: Write '1' to Disable interrupt for RXSTARTED event<br>RXSTARTED: Write '1' to Disable interrupt for RXSTARTED event<br>STARTED: Write '1' to Disable interrupt for STARTED event<br>"]
    pub(crate) fn apb3_intenclr308_rxstarted_read(&self) -> MemResult<bool> {
        todo ! ("read RXSTARTED, RXSTARTED, STARTED mwrite None write None rac None reset value false")
    }
    #[doc = "RXSTARTED: Write '1' to Disable interrupt for RXSTARTED event<br>RXSTARTED: Write '1' to Disable interrupt for RXSTARTED event<br>STARTED: Write '1' to Disable interrupt for STARTED event<br>"]
    pub(crate) fn apb3_intenclr308_rxstarted_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RXSTARTED, RXSTARTED, STARTED mwrite None write None rac None reset value false")
    }
    #[doc = "TXSTARTED: Write '1' to Disable interrupt for TXSTARTED event<br>TXSTARTED: Write '1' to Disable interrupt for TXSTARTED event<br>"]
    pub(crate) fn apb3_intenclr308_txstarted_read(&self) -> MemResult<bool> {
        todo ! ("read TXSTARTED, TXSTARTED mwrite None write None rac None reset value false")
    }
    #[doc = "TXSTARTED: Write '1' to Disable interrupt for TXSTARTED event<br>TXSTARTED: Write '1' to Disable interrupt for TXSTARTED event<br>"]
    pub(crate) fn apb3_intenclr308_txstarted_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TXSTARTED, TXSTARTED mwrite None write None rac None reset value false")
    }
    #[doc = "LASTRX: Write '1' to Disable interrupt for LASTRX event<br>"]
    pub(crate) fn apb3_intenclr308_lastrx_read(&self) -> MemResult<bool> {
        todo!("read LASTRX mwrite None write None rac None reset value false")
    }
    #[doc = "LASTRX: Write '1' to Disable interrupt for LASTRX event<br>"]
    pub(crate) fn apb3_intenclr308_lastrx_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write LASTRX mwrite None write None rac None reset value false")
    }
    #[doc = "LASTTX: Write '1' to Disable interrupt for LASTTX event<br>"]
    pub(crate) fn apb3_intenclr308_lasttx_read(&self) -> MemResult<bool> {
        todo!("read LASTTX mwrite None write None rac None reset value false")
    }
    #[doc = "LASTTX: Write '1' to Disable interrupt for LASTTX event<br>"]
    pub(crate) fn apb3_intenclr308_lasttx_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write LASTTX mwrite None write None rac None reset value false")
    }
    #[doc = "WRITE: Write '1' to Disable interrupt for WRITE event<br>"]
    pub(crate) fn apb3_intenclr308_write_read(&self) -> MemResult<bool> {
        todo!("read WRITE mwrite None write None rac None reset value false")
    }
    #[doc = "WRITE: Write '1' to Disable interrupt for WRITE event<br>"]
    pub(crate) fn apb3_intenclr308_write_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write WRITE mwrite None write None rac None reset value false")
    }
    #[doc = "READ: Write '1' to Disable interrupt for READ event<br>"]
    pub(crate) fn apb3_intenclr308_read_read(&self) -> MemResult<bool> {
        todo!("read READ mwrite None write None rac None reset value false")
    }
    #[doc = "READ: Write '1' to Disable interrupt for READ event<br>"]
    pub(crate) fn apb3_intenclr308_read_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write READ mwrite None write None rac None reset value false")
    }
    #[doc = "SEMSTAT: Semaphore status<br>"]
    pub(crate) fn apb3_semstat400_semstat_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E20Apb3Semstat400Semstat> {
        todo ! ("read SEMSTAT mwrite None write None rac None reset value 0x01 mask 0x03")
    }
    #[doc = "OVERREAD: TX buffer over-read detected, and prevented<br>"]
    pub(crate) fn apb3_status440_overread_read(&self) -> MemResult<bool> {
        todo!("read OVERREAD mwrite None write None rac None reset value false")
    }
    #[doc = "OVERREAD: TX buffer over-read detected, and prevented<br>"]
    pub(crate) fn apb3_status440_overread_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write OVERREAD mwrite None write None rac None reset value false"
        )
    }
    #[doc = "OVERFLOW: RX buffer overflow detected, and prevented<br>"]
    pub(crate) fn apb3_status440_overflow_read(&self) -> MemResult<bool> {
        todo!("read OVERFLOW mwrite None write None rac None reset value false")
    }
    #[doc = "OVERFLOW: RX buffer overflow detected, and prevented<br>"]
    pub(crate) fn apb3_status440_overflow_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write OVERFLOW mwrite None write None rac None reset value false"
        )
    }
    #[doc = "OVERRUN: Overrun error<br>OVERRUN: Overrun error<br>"]
    pub(crate) fn apb3_errorsrc4c4_overrun_read(&self) -> MemResult<bool> {
        todo ! ("read OVERRUN, OVERRUN mwrite None write None rac None reset value false")
    }
    #[doc = "OVERRUN: Overrun error<br>OVERRUN: Overrun error<br>"]
    pub(crate) fn apb3_errorsrc4c4_overrun_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write OVERRUN, OVERRUN mwrite None write None rac None reset value false")
    }
    #[doc = "ANACK: NACK received after sending the address (write '1' to clear)<br>ANACK: NACK received after sending the address (write '1' to clear)<br>"]
    pub(crate) fn apb3_errorsrc4c4_anack_read(&self) -> MemResult<bool> {
        todo ! ("read ANACK, ANACK mwrite None write None rac None reset value false")
    }
    #[doc = "ANACK: NACK received after sending the address (write '1' to clear)<br>ANACK: NACK received after sending the address (write '1' to clear)<br>"]
    pub(crate) fn apb3_errorsrc4c4_anack_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write ANACK, ANACK mwrite None write None rac None reset value false")
    }
    #[doc = "DNACK: NACK received after sending a data byte (write '1' to clear)<br>DNACK: NACK received after sending a data byte (write '1' to clear)<br>"]
    pub(crate) fn apb3_errorsrc4c4_dnack_read(&self) -> MemResult<bool> {
        todo ! ("read DNACK, DNACK mwrite None write None rac None reset value false")
    }
    #[doc = "DNACK: NACK received after sending a data byte (write '1' to clear)<br>DNACK: NACK received after sending a data byte (write '1' to clear)<br>"]
    pub(crate) fn apb3_errorsrc4c4_dnack_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write DNACK, DNACK mwrite None write None rac None reset value false")
    }
    #[doc = "OVERFLOW: RX buffer overflow detected, and prevented<br>"]
    pub(crate) fn apb3_errorsrc4d0_overflow_read(&self) -> MemResult<bool> {
        todo!("read OVERFLOW mwrite None write None rac None reset value false")
    }
    #[doc = "OVERFLOW: RX buffer overflow detected, and prevented<br>"]
    pub(crate) fn apb3_errorsrc4d0_overflow_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write OVERFLOW mwrite None write None rac None reset value false"
        )
    }
    #[doc = "DNACK: NACK sent after receiving a data byte<br>"]
    pub(crate) fn apb3_errorsrc4d0_dnack_read(&self) -> MemResult<bool> {
        todo!("read DNACK mwrite None write None rac None reset value false")
    }
    #[doc = "DNACK: NACK sent after receiving a data byte<br>"]
    pub(crate) fn apb3_errorsrc4d0_dnack_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write DNACK mwrite None write None rac None reset value false")
    }
    #[doc = "OVERREAD: TX buffer over-read detected, and prevented<br>"]
    pub(crate) fn apb3_errorsrc4d0_overread_read(&self) -> MemResult<bool> {
        todo!("read OVERREAD mwrite None write None rac None reset value false")
    }
    #[doc = "OVERREAD: TX buffer over-read detected, and prevented<br>"]
    pub(crate) fn apb3_errorsrc4d0_overread_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write OVERREAD mwrite None write None rac None reset value false"
        )
    }
    #[doc = "MATCH: Which of the addresses in {ADDRESS} matched the incoming address<br>"]
    pub(crate) fn apb3_match4d4_match_read(&self) -> MemResult<bool> {
        todo!("read MATCH mwrite None write None rac None reset value false")
    }
    #[doc = "ENABLE: Enable or disable SPI<br>ENABLE: Enable or disable SPI slave<br>ENABLE: Enable or disable TWIS<br>ENABLE: Enable or disable SPIM<br>ENABLE: Enable or disable TWI<br>ENABLE: Enable or disable TWIM<br>"]
    pub(crate) fn apb3_enable500_enable_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E21Apb3Enable500Enable> {
        todo ! ("read ENABLE, ENABLE, ENABLE, ENABLE, ENABLE, ENABLE mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "ENABLE: Enable or disable SPI<br>ENABLE: Enable or disable SPI slave<br>ENABLE: Enable or disable TWIS<br>ENABLE: Enable or disable SPIM<br>ENABLE: Enable or disable TWI<br>ENABLE: Enable or disable TWIM<br>"]
    pub(crate) fn apb3_enable500_enable_write(
        &mut self,
        _value: crate::peripheral::enums::E21Apb3Enable500Enable,
    ) -> MemResult<()> {
        todo ! ("write ENABLE, ENABLE, ENABLE, ENABLE, ENABLE, ENABLE mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "PIN: Pin number<br>PIN: Pin number<br>PIN: Pin number<br>PIN: Pin number<br>"]
    pub(crate) fn apb3_psel_sck0_pin_read(&self) -> MemResult<u8> {
        todo ! ("read PIN, PIN, PIN, PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "PIN: Pin number<br>PIN: Pin number<br>PIN: Pin number<br>PIN: Pin number<br>"]
    pub(crate) fn apb3_psel_sck0_pin_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write PIN, PIN, PIN, PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "CONNECT: Connection<br>CONNECT: Connection<br>CONNECT: Connection<br>CONNECT: Connection<br>"]
    pub(crate) fn apb3_psel_sck0_connect_read(&self) -> MemResult<bool> {
        todo ! ("read CONNECT, CONNECT, CONNECT, CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "CONNECT: Connection<br>CONNECT: Connection<br>CONNECT: Connection<br>CONNECT: Connection<br>"]
    pub(crate) fn apb3_psel_sck0_connect_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write CONNECT, CONNECT, CONNECT, CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "PIN: Pin number<br>PIN: Pin number<br>PIN: Pin number<br>PIN: Pin number<br>"]
    pub(crate) fn apb3_psel_miso4_pin_read(&self) -> MemResult<u8> {
        todo ! ("read PIN, PIN, PIN, PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "PIN: Pin number<br>PIN: Pin number<br>PIN: Pin number<br>PIN: Pin number<br>"]
    pub(crate) fn apb3_psel_miso4_pin_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write PIN, PIN, PIN, PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "CONNECT: Connection<br>CONNECT: Connection<br>CONNECT: Connection<br>CONNECT: Connection<br>"]
    pub(crate) fn apb3_psel_miso4_connect_read(&self) -> MemResult<bool> {
        todo ! ("read CONNECT, CONNECT, CONNECT, CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "CONNECT: Connection<br>CONNECT: Connection<br>CONNECT: Connection<br>CONNECT: Connection<br>"]
    pub(crate) fn apb3_psel_miso4_connect_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write CONNECT, CONNECT, CONNECT, CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "PIN: Pin number<br>PIN: Pin number<br>"]
    pub(crate) fn apb3_psel_mosi8_pin_read(&self) -> MemResult<u8> {
        todo ! ("read PIN, PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "PIN: Pin number<br>PIN: Pin number<br>"]
    pub(crate) fn apb3_psel_mosi8_pin_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write PIN, PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "CONNECT: Connection<br>CONNECT: Connection<br>"]
    pub(crate) fn apb3_psel_mosi8_connect_read(&self) -> MemResult<bool> {
        todo ! ("read CONNECT, CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "CONNECT: Connection<br>CONNECT: Connection<br>"]
    pub(crate) fn apb3_psel_mosi8_connect_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write CONNECT, CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "PIN: Pin number<br>"]
    pub(crate) fn apb3_psel_csnc_pin_read(&self) -> MemResult<u8> {
        todo ! ("read PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "PIN: Pin number<br>"]
    pub(crate) fn apb3_psel_csnc_pin_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn apb3_psel_csnc_connect_read(&self) -> MemResult<bool> {
        todo!("read CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn apb3_psel_csnc_connect_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "RXD: RX data received. Double buffered<br>RXD: RXD register<br>"]
    pub(crate) fn apb3_rxd518_rxd_read(&self) -> MemResult<u8> {
        todo ! ("read RXD, RXD mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "TXD: TX data to send. Double buffered<br>TXD: TXD register<br>"]
    pub(crate) fn apb3_txd51c_txd_read(&self) -> MemResult<u8> {
        todo ! ("read TXD, TXD mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "TXD: TX data to send. Double buffered<br>TXD: TXD register<br>"]
    pub(crate) fn apb3_txd51c_txd_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write TXD, TXD mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "FREQUENCY: TWI master clock frequency<br>FREQUENCY: SPI master data rate<br>FREQUENCY: TWI master clock frequency<br>FREQUENCY: SPI master data rate<br>"]
    pub(crate) fn apb3_frequency524_frequency_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E22Apb3Frequency524Frequency> {
        todo ! ("read FREQUENCY, FREQUENCY, FREQUENCY, FREQUENCY mwrite None write None rac None reset value 0x4000000 mask 0xffffffff")
    }
    #[doc = "FREQUENCY: TWI master clock frequency<br>FREQUENCY: SPI master data rate<br>FREQUENCY: TWI master clock frequency<br>FREQUENCY: SPI master data rate<br>"]
    pub(crate) fn apb3_frequency524_frequency_write(
        &mut self,
        _value: crate::peripheral::enums::E22Apb3Frequency524Frequency,
    ) -> MemResult<()> {
        todo ! ("write FREQUENCY, FREQUENCY, FREQUENCY, FREQUENCY mwrite None write None rac None reset value 0x4000000 mask 0xffffffff")
    }
    #[doc = "PTR: Data pointer<br>PTR: Data pointer<br>PTR: RXD Data pointer<br>PTR: RXD data pointer<br>"]
    pub(crate) fn apb3_rxd_ptr0_ptr_read(&self) -> MemResult<u32> {
        todo ! ("read PTR, PTR, PTR, PTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "PTR: Data pointer<br>PTR: Data pointer<br>PTR: RXD Data pointer<br>PTR: RXD data pointer<br>"]
    pub(crate) fn apb3_rxd_ptr0_ptr_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write PTR, PTR, PTR, PTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "MAXCNT: Maximum number of bytes in receive buffer<br>MAXCNT: Maximum number of bytes in receive buffer<br>MAXCNT: Maximum number of bytes in RXD buffer<br>MAXCNT: Maximum number of bytes in receive buffer<br>"]
    pub(crate) fn apb3_rxd_maxcnt4_maxcnt_read(&self) -> MemResult<u8> {
        todo ! ("read MAXCNT, MAXCNT, MAXCNT, MAXCNT mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "MAXCNT: Maximum number of bytes in receive buffer<br>MAXCNT: Maximum number of bytes in receive buffer<br>MAXCNT: Maximum number of bytes in RXD buffer<br>MAXCNT: Maximum number of bytes in receive buffer<br>"]
    pub(crate) fn apb3_rxd_maxcnt4_maxcnt_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write MAXCNT, MAXCNT, MAXCNT, MAXCNT mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "AMOUNT: Number of bytes transferred in the last transaction<br>AMOUNT: Number of bytes transferred in the last transaction. In case of NACK error, includes the NACK'ed byte.<br>AMOUNT: Number of bytes transferred in the last RXD transaction<br>AMOUNT: Number of bytes received in the last granted transaction<br>"]
    pub(crate) fn apb3_rxd_amount8_amount_read(&self) -> MemResult<u8> {
        todo ! ("read AMOUNT, AMOUNT, AMOUNT, AMOUNT mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "LIST: List type<br>LIST: List type<br>"]
    pub(crate) fn apb3_rxd_listc_list_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E23Apb3RxdListcList> {
        todo ! ("read LIST, LIST mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "LIST: List type<br>LIST: List type<br>"]
    pub(crate) fn apb3_rxd_listc_list_write(
        &mut self,
        _value: crate::peripheral::enums::E23Apb3RxdListcList,
    ) -> MemResult<()> {
        todo ! ("write LIST, LIST mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "PTR: Data pointer<br>PTR: Data pointer<br>PTR: TXD data pointer<br>PTR: TXD Data pointer<br>"]
    pub(crate) fn apb3_txd_ptr0_ptr_read(&self) -> MemResult<u32> {
        todo ! ("read PTR, PTR, PTR, PTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "PTR: Data pointer<br>PTR: Data pointer<br>PTR: TXD data pointer<br>PTR: TXD Data pointer<br>"]
    pub(crate) fn apb3_txd_ptr0_ptr_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write PTR, PTR, PTR, PTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "MAXCNT: Maximum number of bytes in transmit buffer<br>MAXCNT: Maximum number of bytes in transmit buffer<br>MAXCNT: Maximum number of bytes in transmit buffer<br>MAXCNT: Maximum number of bytes in TXD buffer<br>"]
    pub(crate) fn apb3_txd_maxcnt4_maxcnt_read(&self) -> MemResult<u8> {
        todo ! ("read MAXCNT, MAXCNT, MAXCNT, MAXCNT mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "MAXCNT: Maximum number of bytes in transmit buffer<br>MAXCNT: Maximum number of bytes in transmit buffer<br>MAXCNT: Maximum number of bytes in transmit buffer<br>MAXCNT: Maximum number of bytes in TXD buffer<br>"]
    pub(crate) fn apb3_txd_maxcnt4_maxcnt_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write MAXCNT, MAXCNT, MAXCNT, MAXCNT mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "AMOUNT: Number of bytes transferred in the last transaction<br>AMOUNT: Number of bytes transferred in the last transaction. In case of NACK error, includes the NACK'ed byte.<br>AMOUNT: Number of bytes transmitted in last granted transaction<br>AMOUNT: Number of bytes transferred in the last TXD transaction<br>"]
    pub(crate) fn apb3_txd_amount8_amount_read(&self) -> MemResult<u8> {
        todo ! ("read AMOUNT, AMOUNT, AMOUNT, AMOUNT mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "LIST: List type<br>LIST: List type<br>"]
    pub(crate) fn apb3_txd_listc_list_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E24Apb3TxdListcList> {
        todo ! ("read LIST, LIST mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "LIST: List type<br>LIST: List type<br>"]
    pub(crate) fn apb3_txd_listc_list_write(
        &mut self,
        _value: crate::peripheral::enums::E24Apb3TxdListcList,
    ) -> MemResult<()> {
        todo ! ("write LIST, LIST mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "ORDER: Bit order<br>ORDER: Bit order<br>ORDER: Bit order<br>"]
    pub(crate) fn apb3_config554_order_read(&self) -> MemResult<bool> {
        todo ! ("read ORDER, ORDER, ORDER mwrite None write None rac None reset value false")
    }
    #[doc = "ORDER: Bit order<br>ORDER: Bit order<br>ORDER: Bit order<br>"]
    pub(crate) fn apb3_config554_order_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write ORDER, ORDER, ORDER mwrite None write None rac None reset value false")
    }
    #[doc = "CPHA: Serial clock (SCK) phase<br>CPHA: Serial clock (SCK) phase<br>CPHA: Serial clock (SCK) phase<br>"]
    pub(crate) fn apb3_config554_cpha_read(&self) -> MemResult<bool> {
        todo ! ("read CPHA, CPHA, CPHA mwrite None write None rac None reset value false")
    }
    #[doc = "CPHA: Serial clock (SCK) phase<br>CPHA: Serial clock (SCK) phase<br>CPHA: Serial clock (SCK) phase<br>"]
    pub(crate) fn apb3_config554_cpha_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write CPHA, CPHA, CPHA mwrite None write None rac None reset value false")
    }
    #[doc = "CPOL: Serial clock (SCK) polarity<br>CPOL: Serial clock (SCK) polarity<br>CPOL: Serial clock (SCK) polarity<br>"]
    pub(crate) fn apb3_config554_cpol_read(&self) -> MemResult<bool> {
        todo ! ("read CPOL, CPOL, CPOL mwrite None write None rac None reset value false")
    }
    #[doc = "CPOL: Serial clock (SCK) polarity<br>CPOL: Serial clock (SCK) polarity<br>CPOL: Serial clock (SCK) polarity<br>"]
    pub(crate) fn apb3_config554_cpol_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write CPOL, CPOL, CPOL mwrite None write None rac None reset value false")
    }
    #[doc = "DEF: Default character. Character clocked out in case of an ignored transaction.<br>"]
    pub(crate) fn apb3_def55c_def_read(&self) -> MemResult<u8> {
        todo ! ("read DEF mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "DEF: Default character. Character clocked out in case of an ignored transaction.<br>"]
    pub(crate) fn apb3_def55c_def_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write DEF mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "ADDRESS: TWI slave address<br>ADDRESS: Address used in the TWI transfer<br>ADDRESS: Address used in the TWI transfer<br>"]
    pub(crate) fn apb3_addressn588_address_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<u8> {
        todo ! ("read ADDRESS, ADDRESS, ADDRESS mwrite None write None rac None reset value 0x00 mask 0x7f")
    }
    #[doc = "ADDRESS: TWI slave address<br>ADDRESS: Address used in the TWI transfer<br>ADDRESS: Address used in the TWI transfer<br>"]
    pub(crate) fn apb3_addressn588_address_write(
        &mut self,
        _reg_array: usize,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write ADDRESS, ADDRESS, ADDRESS mwrite None write None rac None reset value 0x00 mask 0x7f")
    }
    #[doc = "ADDRESS0: Enable or disable address matching on ADDRESS\\[0\\]<br>"]
    pub(crate) fn apb3_config594_address0_read(&self) -> MemResult<bool> {
        todo!("read ADDRESS0 mwrite None write None rac None reset value true")
    }
    #[doc = "ADDRESS0: Enable or disable address matching on ADDRESS\\[0\\]<br>"]
    pub(crate) fn apb3_config594_address0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ADDRESS0 mwrite None write None rac None reset value true")
    }
    #[doc = "ADDRESS1: Enable or disable address matching on ADDRESS\\[1\\]<br>"]
    pub(crate) fn apb3_config594_address1_read(&self) -> MemResult<bool> {
        todo!("read ADDRESS1 mwrite None write None rac None reset value false")
    }
    #[doc = "ADDRESS1: Enable or disable address matching on ADDRESS\\[1\\]<br>"]
    pub(crate) fn apb3_config594_address1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write ADDRESS1 mwrite None write None rac None reset value false"
        )
    }
    #[doc = "ORC: Over-read character. Character clocked out after an over-read of the transmit buffer.<br>ORC: Over-read character. Character clocked out in case and over-read of the TXD buffer.<br>ORC: Over-read character. Character sent out in case of an over-read of the transmit buffer.<br>"]
    pub(crate) fn apb3_orc5c0_orc_read(&self) -> MemResult<u8> {
        todo ! ("read ORC, ORC, ORC mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "ORC: Over-read character. Character clocked out after an over-read of the transmit buffer.<br>ORC: Over-read character. Character clocked out in case and over-read of the TXD buffer.<br>ORC: Over-read character. Character sent out in case of an over-read of the transmit buffer.<br>"]
    pub(crate) fn apb3_orc5c0_orc_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write ORC, ORC, ORC mwrite None write None rac None reset value 0x00 mask 0xff")
    }
}
