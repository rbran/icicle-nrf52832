use icicle_vm::cpu::mem::MemResult;
#[derive(Default)]
#[doc = "UARTE0: UART with EasyDMA<br>UART0: Universal Asynchronous Receiver/Transmitter<br><br>Instances:<br>0x40002000: UARTE0, UART0<br>"]
pub struct Apb2 {
    #[doc = "TODO: implement things here"]
    _todo: (),
}
impl Apb2 {
    pub(crate) fn page_to_index(page: u64) -> usize {
        match page {
            262146u64 => 0usize,
            _ => unreachable!(),
        }
    }
    #[doc = "TASKS_STARTRX: Start UART receiver<br>TASKS_STARTRX: Start UART receiver<br>"]
    pub(crate) fn apb2_tasks_startrx0_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb2_tasks_startrx0 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_STOPRX: Stop UART receiver<br>TASKS_STOPRX: Stop UART receiver<br>"]
    pub(crate) fn apb2_tasks_stoprx4_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb2_tasks_stoprx4 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_STARTTX: Start UART transmitter<br>TASKS_STARTTX: Start UART transmitter<br>"]
    pub(crate) fn apb2_tasks_starttx8_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb2_tasks_starttx8 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_STOPTX: Stop UART transmitter<br>TASKS_STOPTX: Stop UART transmitter<br>"]
    pub(crate) fn apb2_tasks_stoptxc_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb2_tasks_stoptxc mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_SUSPEND: Suspend UART<br>"]
    pub(crate) fn apb2_tasks_suspend1c_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb2_tasks_suspend1c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_FLUSHRX: Flush RX FIFO into RX buffer<br>"]
    pub(crate) fn apb2_tasks_flushrx2c_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb2_tasks_flushrx2c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_CTS: CTS is activated (set low). Clear To Send.<br>EVENTS_CTS: CTS is activated (set low). Clear To Send.<br>"]
    pub(crate) fn apb2_events_cts100_read(&self) -> MemResult<u32> {
        todo ! ("read apb2_events_cts100 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_CTS: CTS is activated (set low). Clear To Send.<br>EVENTS_CTS: CTS is activated (set low). Clear To Send.<br>"]
    pub(crate) fn apb2_events_cts100_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb2_events_cts100 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_NCTS: CTS is deactivated (set high). Not Clear To Send.<br>EVENTS_NCTS: CTS is deactivated (set high). Not Clear To Send.<br>"]
    pub(crate) fn apb2_events_ncts104_read(&self) -> MemResult<u32> {
        todo ! ("read apb2_events_ncts104 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_NCTS: CTS is deactivated (set high). Not Clear To Send.<br>EVENTS_NCTS: CTS is deactivated (set high). Not Clear To Send.<br>"]
    pub(crate) fn apb2_events_ncts104_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb2_events_ncts104 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_RXDRDY: Data received in RXD<br>EVENTS_RXDRDY: Data received in RXD (but potentially not yet transferred to Data RAM)<br>"]
    pub(crate) fn apb2_events_rxdrdy108_read(&self) -> MemResult<u32> {
        todo ! ("read apb2_events_rxdrdy108 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_RXDRDY: Data received in RXD<br>EVENTS_RXDRDY: Data received in RXD (but potentially not yet transferred to Data RAM)<br>"]
    pub(crate) fn apb2_events_rxdrdy108_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb2_events_rxdrdy108 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ENDRX: Receive buffer is filled up<br>"]
    pub(crate) fn apb2_events_endrx110_read(&self) -> MemResult<u32> {
        todo ! ("read apb2_events_endrx110 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ENDRX: Receive buffer is filled up<br>"]
    pub(crate) fn apb2_events_endrx110_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb2_events_endrx110 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_TXDRDY: Data sent from TXD<br>EVENTS_TXDRDY: Data sent from TXD<br>"]
    pub(crate) fn apb2_events_txdrdy11c_read(&self) -> MemResult<u32> {
        todo ! ("read apb2_events_txdrdy11c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_TXDRDY: Data sent from TXD<br>EVENTS_TXDRDY: Data sent from TXD<br>"]
    pub(crate) fn apb2_events_txdrdy11c_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb2_events_txdrdy11c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ENDTX: Last TX byte transmitted<br>"]
    pub(crate) fn apb2_events_endtx120_read(&self) -> MemResult<u32> {
        todo ! ("read apb2_events_endtx120 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ENDTX: Last TX byte transmitted<br>"]
    pub(crate) fn apb2_events_endtx120_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb2_events_endtx120 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ERROR: Error detected<br>EVENTS_ERROR: Error detected<br>"]
    pub(crate) fn apb2_events_error124_read(&self) -> MemResult<u32> {
        todo ! ("read apb2_events_error124 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ERROR: Error detected<br>EVENTS_ERROR: Error detected<br>"]
    pub(crate) fn apb2_events_error124_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb2_events_error124 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_RXTO: Receiver timeout<br>EVENTS_RXTO: Receiver timeout<br>"]
    pub(crate) fn apb2_events_rxto144_read(&self) -> MemResult<u32> {
        todo ! ("read apb2_events_rxto144 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_RXTO: Receiver timeout<br>EVENTS_RXTO: Receiver timeout<br>"]
    pub(crate) fn apb2_events_rxto144_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb2_events_rxto144 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_RXSTARTED: UART receiver has started<br>"]
    pub(crate) fn apb2_events_rxstarted14c_read(&self) -> MemResult<u32> {
        todo ! ("read apb2_events_rxstarted14c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_RXSTARTED: UART receiver has started<br>"]
    pub(crate) fn apb2_events_rxstarted14c_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb2_events_rxstarted14c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_TXSTARTED: UART transmitter has started<br>"]
    pub(crate) fn apb2_events_txstarted150_read(&self) -> MemResult<u32> {
        todo ! ("read apb2_events_txstarted150 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_TXSTARTED: UART transmitter has started<br>"]
    pub(crate) fn apb2_events_txstarted150_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb2_events_txstarted150 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_TXSTOPPED: Transmitter stopped<br>"]
    pub(crate) fn apb2_events_txstopped158_read(&self) -> MemResult<u32> {
        todo ! ("read apb2_events_txstopped158 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_TXSTOPPED: Transmitter stopped<br>"]
    pub(crate) fn apb2_events_txstopped158_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb2_events_txstopped158 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "CTS_STARTRX: Shortcut between CTS event and STARTRX task<br>"]
    pub(crate) fn apb2_shorts200_cts_startrx_read(&self) -> MemResult<bool> {
        todo ! ("read CTS_STARTRX mwrite None write None rac None reset value false")
    }
    #[doc = "CTS_STARTRX: Shortcut between CTS event and STARTRX task<br>"]
    pub(crate) fn apb2_shorts200_cts_startrx_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write CTS_STARTRX mwrite None write None rac None reset value false")
    }
    #[doc = "NCTS_STOPRX: Shortcut between NCTS event and STOPRX task<br>"]
    pub(crate) fn apb2_shorts200_ncts_stoprx_read(&self) -> MemResult<bool> {
        todo ! ("read NCTS_STOPRX mwrite None write None rac None reset value false")
    }
    #[doc = "NCTS_STOPRX: Shortcut between NCTS event and STOPRX task<br>"]
    pub(crate) fn apb2_shorts200_ncts_stoprx_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write NCTS_STOPRX mwrite None write None rac None reset value false")
    }
    #[doc = "ENDRX_STARTRX: Shortcut between ENDRX event and STARTRX task<br>"]
    pub(crate) fn apb2_shorts200_endrx_startrx_read(&self) -> MemResult<bool> {
        todo ! ("read ENDRX_STARTRX mwrite None write None rac None reset value false")
    }
    #[doc = "ENDRX_STARTRX: Shortcut between ENDRX event and STARTRX task<br>"]
    pub(crate) fn apb2_shorts200_endrx_startrx_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write ENDRX_STARTRX mwrite None write None rac None reset value false")
    }
    #[doc = "ENDRX_STOPRX: Shortcut between ENDRX event and STOPRX task<br>"]
    pub(crate) fn apb2_shorts200_endrx_stoprx_read(&self) -> MemResult<bool> {
        todo ! ("read ENDRX_STOPRX mwrite None write None rac None reset value false")
    }
    #[doc = "ENDRX_STOPRX: Shortcut between ENDRX event and STOPRX task<br>"]
    pub(crate) fn apb2_shorts200_endrx_stoprx_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write ENDRX_STOPRX mwrite None write None rac None reset value false")
    }
    #[doc = "CTS: Enable or disable interrupt for CTS event<br>"]
    pub(crate) fn apb2_inten300_cts_read(&self) -> MemResult<bool> {
        todo!("read CTS mwrite None write None rac None reset value false")
    }
    #[doc = "CTS: Enable or disable interrupt for CTS event<br>"]
    pub(crate) fn apb2_inten300_cts_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CTS mwrite None write None rac None reset value false")
    }
    #[doc = "NCTS: Enable or disable interrupt for NCTS event<br>"]
    pub(crate) fn apb2_inten300_ncts_read(&self) -> MemResult<bool> {
        todo!("read NCTS mwrite None write None rac None reset value false")
    }
    #[doc = "NCTS: Enable or disable interrupt for NCTS event<br>"]
    pub(crate) fn apb2_inten300_ncts_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write NCTS mwrite None write None rac None reset value false")
    }
    #[doc = "RXDRDY: Enable or disable interrupt for RXDRDY event<br>"]
    pub(crate) fn apb2_inten300_rxdrdy_read(&self) -> MemResult<bool> {
        todo!("read RXDRDY mwrite None write None rac None reset value false")
    }
    #[doc = "RXDRDY: Enable or disable interrupt for RXDRDY event<br>"]
    pub(crate) fn apb2_inten300_rxdrdy_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RXDRDY mwrite None write None rac None reset value false")
    }
    #[doc = "ENDRX: Enable or disable interrupt for ENDRX event<br>"]
    pub(crate) fn apb2_inten300_endrx_read(&self) -> MemResult<bool> {
        todo!("read ENDRX mwrite None write None rac None reset value false")
    }
    #[doc = "ENDRX: Enable or disable interrupt for ENDRX event<br>"]
    pub(crate) fn apb2_inten300_endrx_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ENDRX mwrite None write None rac None reset value false")
    }
    #[doc = "TXDRDY: Enable or disable interrupt for TXDRDY event<br>"]
    pub(crate) fn apb2_inten300_txdrdy_read(&self) -> MemResult<bool> {
        todo!("read TXDRDY mwrite None write None rac None reset value false")
    }
    #[doc = "TXDRDY: Enable or disable interrupt for TXDRDY event<br>"]
    pub(crate) fn apb2_inten300_txdrdy_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write TXDRDY mwrite None write None rac None reset value false")
    }
    #[doc = "ENDTX: Enable or disable interrupt for ENDTX event<br>"]
    pub(crate) fn apb2_inten300_endtx_read(&self) -> MemResult<bool> {
        todo!("read ENDTX mwrite None write None rac None reset value false")
    }
    #[doc = "ENDTX: Enable or disable interrupt for ENDTX event<br>"]
    pub(crate) fn apb2_inten300_endtx_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ENDTX mwrite None write None rac None reset value false")
    }
    #[doc = "ERROR: Enable or disable interrupt for ERROR event<br>"]
    pub(crate) fn apb2_inten300_error_read(&self) -> MemResult<bool> {
        todo!("read ERROR mwrite None write None rac None reset value false")
    }
    #[doc = "ERROR: Enable or disable interrupt for ERROR event<br>"]
    pub(crate) fn apb2_inten300_error_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ERROR mwrite None write None rac None reset value false")
    }
    #[doc = "RXTO: Enable or disable interrupt for RXTO event<br>"]
    pub(crate) fn apb2_inten300_rxto_read(&self) -> MemResult<bool> {
        todo!("read RXTO mwrite None write None rac None reset value false")
    }
    #[doc = "RXTO: Enable or disable interrupt for RXTO event<br>"]
    pub(crate) fn apb2_inten300_rxto_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RXTO mwrite None write None rac None reset value false")
    }
    #[doc = "RXSTARTED: Enable or disable interrupt for RXSTARTED event<br>"]
    pub(crate) fn apb2_inten300_rxstarted_read(&self) -> MemResult<bool> {
        todo!(
            "read RXSTARTED mwrite None write None rac None reset value false"
        )
    }
    #[doc = "RXSTARTED: Enable or disable interrupt for RXSTARTED event<br>"]
    pub(crate) fn apb2_inten300_rxstarted_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write RXSTARTED mwrite None write None rac None reset value false"
        )
    }
    #[doc = "TXSTARTED: Enable or disable interrupt for TXSTARTED event<br>"]
    pub(crate) fn apb2_inten300_txstarted_read(&self) -> MemResult<bool> {
        todo!(
            "read TXSTARTED mwrite None write None rac None reset value false"
        )
    }
    #[doc = "TXSTARTED: Enable or disable interrupt for TXSTARTED event<br>"]
    pub(crate) fn apb2_inten300_txstarted_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write TXSTARTED mwrite None write None rac None reset value false"
        )
    }
    #[doc = "TXSTOPPED: Enable or disable interrupt for TXSTOPPED event<br>"]
    pub(crate) fn apb2_inten300_txstopped_read(&self) -> MemResult<bool> {
        todo!(
            "read TXSTOPPED mwrite None write None rac None reset value false"
        )
    }
    #[doc = "TXSTOPPED: Enable or disable interrupt for TXSTOPPED event<br>"]
    pub(crate) fn apb2_inten300_txstopped_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write TXSTOPPED mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CTS: Write '1' to Enable interrupt for CTS event<br>CTS: Write '1' to Enable interrupt for CTS event<br>"]
    pub(crate) fn apb2_intenset304_cts_read(&self) -> MemResult<bool> {
        todo!("read CTS, CTS mwrite None write None rac None reset value false")
    }
    #[doc = "CTS: Write '1' to Enable interrupt for CTS event<br>CTS: Write '1' to Enable interrupt for CTS event<br>"]
    pub(crate) fn apb2_intenset304_cts_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CTS, CTS mwrite None write None rac None reset value false"
        )
    }
    #[doc = "NCTS: Write '1' to Enable interrupt for NCTS event<br>NCTS: Write '1' to Enable interrupt for NCTS event<br>"]
    pub(crate) fn apb2_intenset304_ncts_read(&self) -> MemResult<bool> {
        todo!(
            "read NCTS, NCTS mwrite None write None rac None reset value false"
        )
    }
    #[doc = "NCTS: Write '1' to Enable interrupt for NCTS event<br>NCTS: Write '1' to Enable interrupt for NCTS event<br>"]
    pub(crate) fn apb2_intenset304_ncts_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write NCTS, NCTS mwrite None write None rac None reset value false")
    }
    #[doc = "RXDRDY: Write '1' to Enable interrupt for RXDRDY event<br>RXDRDY: Write '1' to Enable interrupt for RXDRDY event<br>"]
    pub(crate) fn apb2_intenset304_rxdrdy_read(&self) -> MemResult<bool> {
        todo ! ("read RXDRDY, RXDRDY mwrite None write None rac None reset value false")
    }
    #[doc = "RXDRDY: Write '1' to Enable interrupt for RXDRDY event<br>RXDRDY: Write '1' to Enable interrupt for RXDRDY event<br>"]
    pub(crate) fn apb2_intenset304_rxdrdy_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RXDRDY, RXDRDY mwrite None write None rac None reset value false")
    }
    #[doc = "ENDRX: Write '1' to Enable interrupt for ENDRX event<br>"]
    pub(crate) fn apb2_intenset304_endrx_read(&self) -> MemResult<bool> {
        todo!("read ENDRX mwrite None write None rac None reset value false")
    }
    #[doc = "ENDRX: Write '1' to Enable interrupt for ENDRX event<br>"]
    pub(crate) fn apb2_intenset304_endrx_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ENDRX mwrite None write None rac None reset value false")
    }
    #[doc = "TXDRDY: Write '1' to Enable interrupt for TXDRDY event<br>TXDRDY: Write '1' to Enable interrupt for TXDRDY event<br>"]
    pub(crate) fn apb2_intenset304_txdrdy_read(&self) -> MemResult<bool> {
        todo ! ("read TXDRDY, TXDRDY mwrite None write None rac None reset value false")
    }
    #[doc = "TXDRDY: Write '1' to Enable interrupt for TXDRDY event<br>TXDRDY: Write '1' to Enable interrupt for TXDRDY event<br>"]
    pub(crate) fn apb2_intenset304_txdrdy_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TXDRDY, TXDRDY mwrite None write None rac None reset value false")
    }
    #[doc = "ENDTX: Write '1' to Enable interrupt for ENDTX event<br>"]
    pub(crate) fn apb2_intenset304_endtx_read(&self) -> MemResult<bool> {
        todo!("read ENDTX mwrite None write None rac None reset value false")
    }
    #[doc = "ENDTX: Write '1' to Enable interrupt for ENDTX event<br>"]
    pub(crate) fn apb2_intenset304_endtx_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ENDTX mwrite None write None rac None reset value false")
    }
    #[doc = "ERROR: Write '1' to Enable interrupt for ERROR event<br>ERROR: Write '1' to Enable interrupt for ERROR event<br>"]
    pub(crate) fn apb2_intenset304_error_read(&self) -> MemResult<bool> {
        todo ! ("read ERROR, ERROR mwrite None write None rac None reset value false")
    }
    #[doc = "ERROR: Write '1' to Enable interrupt for ERROR event<br>ERROR: Write '1' to Enable interrupt for ERROR event<br>"]
    pub(crate) fn apb2_intenset304_error_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write ERROR, ERROR mwrite None write None rac None reset value false")
    }
    #[doc = "RXTO: Write '1' to Enable interrupt for RXTO event<br>RXTO: Write '1' to Enable interrupt for RXTO event<br>"]
    pub(crate) fn apb2_intenset304_rxto_read(&self) -> MemResult<bool> {
        todo!(
            "read RXTO, RXTO mwrite None write None rac None reset value false"
        )
    }
    #[doc = "RXTO: Write '1' to Enable interrupt for RXTO event<br>RXTO: Write '1' to Enable interrupt for RXTO event<br>"]
    pub(crate) fn apb2_intenset304_rxto_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RXTO, RXTO mwrite None write None rac None reset value false")
    }
    #[doc = "RXSTARTED: Write '1' to Enable interrupt for RXSTARTED event<br>"]
    pub(crate) fn apb2_intenset304_rxstarted_read(&self) -> MemResult<bool> {
        todo!(
            "read RXSTARTED mwrite None write None rac None reset value false"
        )
    }
    #[doc = "RXSTARTED: Write '1' to Enable interrupt for RXSTARTED event<br>"]
    pub(crate) fn apb2_intenset304_rxstarted_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write RXSTARTED mwrite None write None rac None reset value false"
        )
    }
    #[doc = "TXSTARTED: Write '1' to Enable interrupt for TXSTARTED event<br>"]
    pub(crate) fn apb2_intenset304_txstarted_read(&self) -> MemResult<bool> {
        todo!(
            "read TXSTARTED mwrite None write None rac None reset value false"
        )
    }
    #[doc = "TXSTARTED: Write '1' to Enable interrupt for TXSTARTED event<br>"]
    pub(crate) fn apb2_intenset304_txstarted_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write TXSTARTED mwrite None write None rac None reset value false"
        )
    }
    #[doc = "TXSTOPPED: Write '1' to Enable interrupt for TXSTOPPED event<br>"]
    pub(crate) fn apb2_intenset304_txstopped_read(&self) -> MemResult<bool> {
        todo!(
            "read TXSTOPPED mwrite None write None rac None reset value false"
        )
    }
    #[doc = "TXSTOPPED: Write '1' to Enable interrupt for TXSTOPPED event<br>"]
    pub(crate) fn apb2_intenset304_txstopped_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write TXSTOPPED mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CTS: Write '1' to Disable interrupt for CTS event<br>CTS: Write '1' to Disable interrupt for CTS event<br>"]
    pub(crate) fn apb2_intenclr308_cts_read(&self) -> MemResult<bool> {
        todo!("read CTS, CTS mwrite None write None rac None reset value false")
    }
    #[doc = "CTS: Write '1' to Disable interrupt for CTS event<br>CTS: Write '1' to Disable interrupt for CTS event<br>"]
    pub(crate) fn apb2_intenclr308_cts_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CTS, CTS mwrite None write None rac None reset value false"
        )
    }
    #[doc = "NCTS: Write '1' to Disable interrupt for NCTS event<br>NCTS: Write '1' to Disable interrupt for NCTS event<br>"]
    pub(crate) fn apb2_intenclr308_ncts_read(&self) -> MemResult<bool> {
        todo!(
            "read NCTS, NCTS mwrite None write None rac None reset value false"
        )
    }
    #[doc = "NCTS: Write '1' to Disable interrupt for NCTS event<br>NCTS: Write '1' to Disable interrupt for NCTS event<br>"]
    pub(crate) fn apb2_intenclr308_ncts_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write NCTS, NCTS mwrite None write None rac None reset value false")
    }
    #[doc = "RXDRDY: Write '1' to Disable interrupt for RXDRDY event<br>RXDRDY: Write '1' to Disable interrupt for RXDRDY event<br>"]
    pub(crate) fn apb2_intenclr308_rxdrdy_read(&self) -> MemResult<bool> {
        todo ! ("read RXDRDY, RXDRDY mwrite None write None rac None reset value false")
    }
    #[doc = "RXDRDY: Write '1' to Disable interrupt for RXDRDY event<br>RXDRDY: Write '1' to Disable interrupt for RXDRDY event<br>"]
    pub(crate) fn apb2_intenclr308_rxdrdy_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RXDRDY, RXDRDY mwrite None write None rac None reset value false")
    }
    #[doc = "ENDRX: Write '1' to Disable interrupt for ENDRX event<br>"]
    pub(crate) fn apb2_intenclr308_endrx_read(&self) -> MemResult<bool> {
        todo!("read ENDRX mwrite None write None rac None reset value false")
    }
    #[doc = "ENDRX: Write '1' to Disable interrupt for ENDRX event<br>"]
    pub(crate) fn apb2_intenclr308_endrx_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ENDRX mwrite None write None rac None reset value false")
    }
    #[doc = "TXDRDY: Write '1' to Disable interrupt for TXDRDY event<br>TXDRDY: Write '1' to Disable interrupt for TXDRDY event<br>"]
    pub(crate) fn apb2_intenclr308_txdrdy_read(&self) -> MemResult<bool> {
        todo ! ("read TXDRDY, TXDRDY mwrite None write None rac None reset value false")
    }
    #[doc = "TXDRDY: Write '1' to Disable interrupt for TXDRDY event<br>TXDRDY: Write '1' to Disable interrupt for TXDRDY event<br>"]
    pub(crate) fn apb2_intenclr308_txdrdy_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TXDRDY, TXDRDY mwrite None write None rac None reset value false")
    }
    #[doc = "ENDTX: Write '1' to Disable interrupt for ENDTX event<br>"]
    pub(crate) fn apb2_intenclr308_endtx_read(&self) -> MemResult<bool> {
        todo!("read ENDTX mwrite None write None rac None reset value false")
    }
    #[doc = "ENDTX: Write '1' to Disable interrupt for ENDTX event<br>"]
    pub(crate) fn apb2_intenclr308_endtx_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ENDTX mwrite None write None rac None reset value false")
    }
    #[doc = "ERROR: Write '1' to Disable interrupt for ERROR event<br>ERROR: Write '1' to Disable interrupt for ERROR event<br>"]
    pub(crate) fn apb2_intenclr308_error_read(&self) -> MemResult<bool> {
        todo ! ("read ERROR, ERROR mwrite None write None rac None reset value false")
    }
    #[doc = "ERROR: Write '1' to Disable interrupt for ERROR event<br>ERROR: Write '1' to Disable interrupt for ERROR event<br>"]
    pub(crate) fn apb2_intenclr308_error_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write ERROR, ERROR mwrite None write None rac None reset value false")
    }
    #[doc = "RXTO: Write '1' to Disable interrupt for RXTO event<br>RXTO: Write '1' to Disable interrupt for RXTO event<br>"]
    pub(crate) fn apb2_intenclr308_rxto_read(&self) -> MemResult<bool> {
        todo!(
            "read RXTO, RXTO mwrite None write None rac None reset value false"
        )
    }
    #[doc = "RXTO: Write '1' to Disable interrupt for RXTO event<br>RXTO: Write '1' to Disable interrupt for RXTO event<br>"]
    pub(crate) fn apb2_intenclr308_rxto_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RXTO, RXTO mwrite None write None rac None reset value false")
    }
    #[doc = "RXSTARTED: Write '1' to Disable interrupt for RXSTARTED event<br>"]
    pub(crate) fn apb2_intenclr308_rxstarted_read(&self) -> MemResult<bool> {
        todo!(
            "read RXSTARTED mwrite None write None rac None reset value false"
        )
    }
    #[doc = "RXSTARTED: Write '1' to Disable interrupt for RXSTARTED event<br>"]
    pub(crate) fn apb2_intenclr308_rxstarted_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write RXSTARTED mwrite None write None rac None reset value false"
        )
    }
    #[doc = "TXSTARTED: Write '1' to Disable interrupt for TXSTARTED event<br>"]
    pub(crate) fn apb2_intenclr308_txstarted_read(&self) -> MemResult<bool> {
        todo!(
            "read TXSTARTED mwrite None write None rac None reset value false"
        )
    }
    #[doc = "TXSTARTED: Write '1' to Disable interrupt for TXSTARTED event<br>"]
    pub(crate) fn apb2_intenclr308_txstarted_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write TXSTARTED mwrite None write None rac None reset value false"
        )
    }
    #[doc = "TXSTOPPED: Write '1' to Disable interrupt for TXSTOPPED event<br>"]
    pub(crate) fn apb2_intenclr308_txstopped_read(&self) -> MemResult<bool> {
        todo!(
            "read TXSTOPPED mwrite None write None rac None reset value false"
        )
    }
    #[doc = "TXSTOPPED: Write '1' to Disable interrupt for TXSTOPPED event<br>"]
    pub(crate) fn apb2_intenclr308_txstopped_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write TXSTOPPED mwrite None write None rac None reset value false"
        )
    }
    #[doc = "OVERRUN: Overrun error<br>OVERRUN: Overrun error<br>"]
    pub(crate) fn apb2_errorsrc480_overrun_read(&self) -> MemResult<bool> {
        todo ! ("read OVERRUN, OVERRUN mwrite None write None rac None reset value false")
    }
    #[doc = "OVERRUN: Overrun error<br>OVERRUN: Overrun error<br>"]
    pub(crate) fn apb2_errorsrc480_overrun_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write OVERRUN, OVERRUN mwrite None write None rac None reset value false")
    }
    #[doc = "PARITY: Parity error<br>PARITY: Parity error<br>"]
    pub(crate) fn apb2_errorsrc480_parity_read(&self) -> MemResult<bool> {
        todo ! ("read PARITY, PARITY mwrite None write None rac None reset value false")
    }
    #[doc = "PARITY: Parity error<br>PARITY: Parity error<br>"]
    pub(crate) fn apb2_errorsrc480_parity_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write PARITY, PARITY mwrite None write None rac None reset value false")
    }
    #[doc = "FRAMING: Framing error occurred<br>FRAMING: Framing error occurred<br>"]
    pub(crate) fn apb2_errorsrc480_framing_read(&self) -> MemResult<bool> {
        todo ! ("read FRAMING, FRAMING mwrite None write None rac None reset value false")
    }
    #[doc = "FRAMING: Framing error occurred<br>FRAMING: Framing error occurred<br>"]
    pub(crate) fn apb2_errorsrc480_framing_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write FRAMING, FRAMING mwrite None write None rac None reset value false")
    }
    #[doc = "BREAK: Break condition<br>BREAK: Break condition<br>"]
    pub(crate) fn apb2_errorsrc480_break_read(&self) -> MemResult<bool> {
        todo ! ("read BREAK, BREAK mwrite None write None rac None reset value false")
    }
    #[doc = "BREAK: Break condition<br>BREAK: Break condition<br>"]
    pub(crate) fn apb2_errorsrc480_break_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write BREAK, BREAK mwrite None write None rac None reset value false")
    }
    #[doc = "ENABLE: Enable or disable UARTE<br>ENABLE: Enable or disable UART<br>"]
    pub(crate) fn apb2_enable500_enable_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E17Apb2Enable500Enable> {
        todo ! ("read ENABLE, ENABLE mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "ENABLE: Enable or disable UARTE<br>ENABLE: Enable or disable UART<br>"]
    pub(crate) fn apb2_enable500_enable_write(
        &mut self,
        _value: crate::peripheral::enums::E17Apb2Enable500Enable,
    ) -> MemResult<()> {
        todo ! ("write ENABLE, ENABLE mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "PIN: Pin number<br>"]
    pub(crate) fn apb2_psel_rts0_pin_read(&self) -> MemResult<u8> {
        todo ! ("read PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "PIN: Pin number<br>"]
    pub(crate) fn apb2_psel_rts0_pin_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn apb2_psel_rts0_connect_read(&self) -> MemResult<bool> {
        todo!("read CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn apb2_psel_rts0_connect_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "PIN: Pin number<br>"]
    pub(crate) fn apb2_psel_txd4_pin_read(&self) -> MemResult<u8> {
        todo ! ("read PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "PIN: Pin number<br>"]
    pub(crate) fn apb2_psel_txd4_pin_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn apb2_psel_txd4_connect_read(&self) -> MemResult<bool> {
        todo!("read CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn apb2_psel_txd4_connect_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "PIN: Pin number<br>"]
    pub(crate) fn apb2_psel_cts8_pin_read(&self) -> MemResult<u8> {
        todo ! ("read PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "PIN: Pin number<br>"]
    pub(crate) fn apb2_psel_cts8_pin_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn apb2_psel_cts8_connect_read(&self) -> MemResult<bool> {
        todo!("read CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn apb2_psel_cts8_connect_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "PIN: Pin number<br>"]
    pub(crate) fn apb2_psel_rxdc_pin_read(&self) -> MemResult<u8> {
        todo ! ("read PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "PIN: Pin number<br>"]
    pub(crate) fn apb2_psel_rxdc_pin_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn apb2_psel_rxdc_connect_read(&self) -> MemResult<bool> {
        todo!("read CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn apb2_psel_rxdc_connect_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "RXD: RX data received in previous transfers, double buffered<br>"]
    pub(crate) fn apb2_rxd518_rxd_read(&self) -> MemResult<u8> {
        todo ! ("read RXD mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "TXD: TX data to be transferred<br>"]
    pub(crate) fn apb2_txd51c_txd_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write TXD mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "BAUDRATE: Baud rate<br>BAUDRATE: Baud rate<br>"]
    pub(crate) fn apb2_baudrate524_baudrate_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E18Apb2Baudrate524Baudrate> {
        todo ! ("read BAUDRATE, BAUDRATE mwrite None write None rac None reset value 0x4000000 mask 0xffffffff")
    }
    #[doc = "BAUDRATE: Baud rate<br>BAUDRATE: Baud rate<br>"]
    pub(crate) fn apb2_baudrate524_baudrate_write(
        &mut self,
        _value: crate::peripheral::enums::E18Apb2Baudrate524Baudrate,
    ) -> MemResult<()> {
        todo ! ("write BAUDRATE, BAUDRATE mwrite None write None rac None reset value 0x4000000 mask 0xffffffff")
    }
    #[doc = "PTR: Data pointer<br>"]
    pub(crate) fn apb2_rxd_ptr0_ptr_read(&self) -> MemResult<u32> {
        todo ! ("read PTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "PTR: Data pointer<br>"]
    pub(crate) fn apb2_rxd_ptr0_ptr_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write PTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "MAXCNT: Maximum number of bytes in receive buffer<br>"]
    pub(crate) fn apb2_rxd_maxcnt4_maxcnt_read(&self) -> MemResult<u8> {
        todo ! ("read MAXCNT mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "MAXCNT: Maximum number of bytes in receive buffer<br>"]
    pub(crate) fn apb2_rxd_maxcnt4_maxcnt_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write MAXCNT mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "AMOUNT: Number of bytes transferred in the last transaction<br>"]
    pub(crate) fn apb2_rxd_amount8_amount_read(&self) -> MemResult<u8> {
        todo ! ("read AMOUNT mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "PTR: Data pointer<br>"]
    pub(crate) fn apb2_txd_ptr0_ptr_read(&self) -> MemResult<u32> {
        todo ! ("read PTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "PTR: Data pointer<br>"]
    pub(crate) fn apb2_txd_ptr0_ptr_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write PTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "MAXCNT: Maximum number of bytes in transmit buffer<br>"]
    pub(crate) fn apb2_txd_maxcnt4_maxcnt_read(&self) -> MemResult<u8> {
        todo ! ("read MAXCNT mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "MAXCNT: Maximum number of bytes in transmit buffer<br>"]
    pub(crate) fn apb2_txd_maxcnt4_maxcnt_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write MAXCNT mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "AMOUNT: Number of bytes transferred in the last transaction<br>"]
    pub(crate) fn apb2_txd_amount8_amount_read(&self) -> MemResult<u8> {
        todo ! ("read AMOUNT mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "HWFC: Hardware flow control<br>HWFC: Hardware flow control<br>"]
    pub(crate) fn apb2_config56c_hwfc_read(&self) -> MemResult<bool> {
        todo!(
            "read HWFC, HWFC mwrite None write None rac None reset value false"
        )
    }
    #[doc = "HWFC: Hardware flow control<br>HWFC: Hardware flow control<br>"]
    pub(crate) fn apb2_config56c_hwfc_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write HWFC, HWFC mwrite None write None rac None reset value false")
    }
    #[doc = "PARITY: Parity<br>PARITY: Parity<br>"]
    pub(crate) fn apb2_config56c_parity_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E19Apb2Config56cParity> {
        todo ! ("read PARITY, PARITY mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "PARITY: Parity<br>PARITY: Parity<br>"]
    pub(crate) fn apb2_config56c_parity_write(
        &mut self,
        _value: crate::peripheral::enums::E19Apb2Config56cParity,
    ) -> MemResult<()> {
        todo ! ("write PARITY, PARITY mwrite None write None rac None reset value 0x00 mask 0x07")
    }
}
