use icicle_vm::cpu::mem::MemResult;
#[derive(Default)]
#[doc = "NFCT: NFC-A compatible radio<br><br>Instances:<br>0x40005000: NFCT<br>"]
pub struct Nfct {
    #[doc = "TODO: implement things here"]
    _todo: (),
}
impl Nfct {
    pub(crate) fn page_to_index(page: u64) -> usize {
        match page {
            262149u64 => 0usize,
            _ => unreachable!(),
        }
    }
    #[doc = "TASKS_ACTIVATE: Activate NFC peripheral for incoming and outgoing frames, change state to activated<br>"]
    pub(crate) fn nfct_tasks_activate0_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write nfct_tasks_activate0 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_DISABLE: Disable NFC peripheral<br>"]
    pub(crate) fn nfct_tasks_disable4_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write nfct_tasks_disable4 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_SENSE: Enable NFC sense field mode, change state to sense mode<br>"]
    pub(crate) fn nfct_tasks_sense8_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write nfct_tasks_sense8 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_STARTTX: Start transmission of a outgoing frame, change state to transmit<br>"]
    pub(crate) fn nfct_tasks_starttxc_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write nfct_tasks_starttxc mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_ENABLERXDATA: Initializes the EasyDMA for receive.<br>"]
    pub(crate) fn nfct_tasks_enablerxdata1c_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write nfct_tasks_enablerxdata1c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_GOIDLE: Force state machine to IDLE state<br>"]
    pub(crate) fn nfct_tasks_goidle24_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write nfct_tasks_goidle24 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_GOSLEEP: Force state machine to SLEEP_A state<br>"]
    pub(crate) fn nfct_tasks_gosleep28_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write nfct_tasks_gosleep28 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_READY: The NFC peripheral is ready to receive and send frames<br>"]
    pub(crate) fn nfct_events_ready100_read(&self) -> MemResult<u32> {
        todo ! ("read nfct_events_ready100 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_READY: The NFC peripheral is ready to receive and send frames<br>"]
    pub(crate) fn nfct_events_ready100_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write nfct_events_ready100 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_FIELDDETECTED: Remote NFC field detected<br>"]
    pub(crate) fn nfct_events_fielddetected104_read(&self) -> MemResult<u32> {
        todo ! ("read nfct_events_fielddetected104 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_FIELDDETECTED: Remote NFC field detected<br>"]
    pub(crate) fn nfct_events_fielddetected104_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write nfct_events_fielddetected104 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_FIELDLOST: Remote NFC field lost<br>"]
    pub(crate) fn nfct_events_fieldlost108_read(&self) -> MemResult<u32> {
        todo ! ("read nfct_events_fieldlost108 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_FIELDLOST: Remote NFC field lost<br>"]
    pub(crate) fn nfct_events_fieldlost108_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write nfct_events_fieldlost108 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_TXFRAMESTART: Marks the start of the first symbol of a transmitted frame<br>"]
    pub(crate) fn nfct_events_txframestart10c_read(&self) -> MemResult<u32> {
        todo ! ("read nfct_events_txframestart10c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_TXFRAMESTART: Marks the start of the first symbol of a transmitted frame<br>"]
    pub(crate) fn nfct_events_txframestart10c_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write nfct_events_txframestart10c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_TXFRAMEEND: Marks the end of the last transmitted on-air symbol of a frame<br>"]
    pub(crate) fn nfct_events_txframeend110_read(&self) -> MemResult<u32> {
        todo ! ("read nfct_events_txframeend110 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_TXFRAMEEND: Marks the end of the last transmitted on-air symbol of a frame<br>"]
    pub(crate) fn nfct_events_txframeend110_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write nfct_events_txframeend110 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_RXFRAMESTART: Marks the end of the first symbol of a received frame<br>"]
    pub(crate) fn nfct_events_rxframestart114_read(&self) -> MemResult<u32> {
        todo ! ("read nfct_events_rxframestart114 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_RXFRAMESTART: Marks the end of the first symbol of a received frame<br>"]
    pub(crate) fn nfct_events_rxframestart114_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write nfct_events_rxframestart114 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_RXFRAMEEND: Received data have been checked (CRC, parity) and transferred to RAM, and EasyDMA has ended accessing the RX buffer<br>"]
    pub(crate) fn nfct_events_rxframeend118_read(&self) -> MemResult<u32> {
        todo ! ("read nfct_events_rxframeend118 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_RXFRAMEEND: Received data have been checked (CRC, parity) and transferred to RAM, and EasyDMA has ended accessing the RX buffer<br>"]
    pub(crate) fn nfct_events_rxframeend118_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write nfct_events_rxframeend118 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ERROR: NFC error reported. The ERRORSTATUS register contains details on the source of the error.<br>"]
    pub(crate) fn nfct_events_error11c_read(&self) -> MemResult<u32> {
        todo ! ("read nfct_events_error11c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ERROR: NFC error reported. The ERRORSTATUS register contains details on the source of the error.<br>"]
    pub(crate) fn nfct_events_error11c_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write nfct_events_error11c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_RXERROR: NFC RX frame error reported. The FRAMESTATUS.RX register contains details on the source of the error.<br>"]
    pub(crate) fn nfct_events_rxerror128_read(&self) -> MemResult<u32> {
        todo ! ("read nfct_events_rxerror128 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_RXERROR: NFC RX frame error reported. The FRAMESTATUS.RX register contains details on the source of the error.<br>"]
    pub(crate) fn nfct_events_rxerror128_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write nfct_events_rxerror128 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ENDRX: RX buffer (as defined by PACKETPTR and MAXLEN) in Data RAM full.<br>"]
    pub(crate) fn nfct_events_endrx12c_read(&self) -> MemResult<u32> {
        todo ! ("read nfct_events_endrx12c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ENDRX: RX buffer (as defined by PACKETPTR and MAXLEN) in Data RAM full.<br>"]
    pub(crate) fn nfct_events_endrx12c_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write nfct_events_endrx12c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ENDTX: Transmission of data in RAM has ended, and EasyDMA has ended accessing the TX buffer<br>"]
    pub(crate) fn nfct_events_endtx130_read(&self) -> MemResult<u32> {
        todo ! ("read nfct_events_endtx130 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ENDTX: Transmission of data in RAM has ended, and EasyDMA has ended accessing the TX buffer<br>"]
    pub(crate) fn nfct_events_endtx130_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write nfct_events_endtx130 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_AUTOCOLRESSTARTED: Auto collision resolution process has started<br>"]
    pub(crate) fn nfct_events_autocolresstarted138_read(
        &self,
    ) -> MemResult<u32> {
        todo ! ("read nfct_events_autocolresstarted138 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_AUTOCOLRESSTARTED: Auto collision resolution process has started<br>"]
    pub(crate) fn nfct_events_autocolresstarted138_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write nfct_events_autocolresstarted138 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_COLLISION: NFC Auto collision resolution error reported.<br>"]
    pub(crate) fn nfct_events_collision148_read(&self) -> MemResult<u32> {
        todo ! ("read nfct_events_collision148 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_COLLISION: NFC Auto collision resolution error reported.<br>"]
    pub(crate) fn nfct_events_collision148_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write nfct_events_collision148 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_SELECTED: NFC Auto collision resolution successfully completed<br>"]
    pub(crate) fn nfct_events_selected14c_read(&self) -> MemResult<u32> {
        todo ! ("read nfct_events_selected14c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_SELECTED: NFC Auto collision resolution successfully completed<br>"]
    pub(crate) fn nfct_events_selected14c_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write nfct_events_selected14c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_STARTED: EasyDMA is ready to receive or send frames.<br>"]
    pub(crate) fn nfct_events_started150_read(&self) -> MemResult<u32> {
        todo ! ("read nfct_events_started150 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_STARTED: EasyDMA is ready to receive or send frames.<br>"]
    pub(crate) fn nfct_events_started150_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write nfct_events_started150 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "FIELDDETECTED_ACTIVATE: Shortcut between FIELDDETECTED event and ACTIVATE task<br>"]
    pub(crate) fn nfct_shorts200_fielddetected_activate_read(
        &self,
    ) -> MemResult<bool> {
        todo ! ("read FIELDDETECTED_ACTIVATE mwrite None write None rac None reset value false")
    }
    #[doc = "FIELDDETECTED_ACTIVATE: Shortcut between FIELDDETECTED event and ACTIVATE task<br>"]
    pub(crate) fn nfct_shorts200_fielddetected_activate_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write FIELDDETECTED_ACTIVATE mwrite None write None rac None reset value false")
    }
    #[doc = "FIELDLOST_SENSE: Shortcut between FIELDLOST event and SENSE task<br>"]
    pub(crate) fn nfct_shorts200_fieldlost_sense_read(
        &self,
    ) -> MemResult<bool> {
        todo ! ("read FIELDLOST_SENSE mwrite None write None rac None reset value false")
    }
    #[doc = "FIELDLOST_SENSE: Shortcut between FIELDLOST event and SENSE task<br>"]
    pub(crate) fn nfct_shorts200_fieldlost_sense_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write FIELDLOST_SENSE mwrite None write None rac None reset value false")
    }
    #[doc = "READY: Enable or disable interrupt for READY event<br>"]
    pub(crate) fn nfct_inten300_ready_read(&self) -> MemResult<bool> {
        todo!("read READY mwrite None write None rac None reset value false")
    }
    #[doc = "READY: Enable or disable interrupt for READY event<br>"]
    pub(crate) fn nfct_inten300_ready_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write READY mwrite None write None rac None reset value false")
    }
    #[doc = "FIELDDETECTED: Enable or disable interrupt for FIELDDETECTED event<br>"]
    pub(crate) fn nfct_inten300_fielddetected_read(&self) -> MemResult<bool> {
        todo ! ("read FIELDDETECTED mwrite None write None rac None reset value false")
    }
    #[doc = "FIELDDETECTED: Enable or disable interrupt for FIELDDETECTED event<br>"]
    pub(crate) fn nfct_inten300_fielddetected_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write FIELDDETECTED mwrite None write None rac None reset value false")
    }
    #[doc = "FIELDLOST: Enable or disable interrupt for FIELDLOST event<br>"]
    pub(crate) fn nfct_inten300_fieldlost_read(&self) -> MemResult<bool> {
        todo!(
            "read FIELDLOST mwrite None write None rac None reset value false"
        )
    }
    #[doc = "FIELDLOST: Enable or disable interrupt for FIELDLOST event<br>"]
    pub(crate) fn nfct_inten300_fieldlost_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write FIELDLOST mwrite None write None rac None reset value false"
        )
    }
    #[doc = "TXFRAMESTART: Enable or disable interrupt for TXFRAMESTART event<br>"]
    pub(crate) fn nfct_inten300_txframestart_read(&self) -> MemResult<bool> {
        todo ! ("read TXFRAMESTART mwrite None write None rac None reset value false")
    }
    #[doc = "TXFRAMESTART: Enable or disable interrupt for TXFRAMESTART event<br>"]
    pub(crate) fn nfct_inten300_txframestart_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TXFRAMESTART mwrite None write None rac None reset value false")
    }
    #[doc = "TXFRAMEEND: Enable or disable interrupt for TXFRAMEEND event<br>"]
    pub(crate) fn nfct_inten300_txframeend_read(&self) -> MemResult<bool> {
        todo!(
            "read TXFRAMEEND mwrite None write None rac None reset value false"
        )
    }
    #[doc = "TXFRAMEEND: Enable or disable interrupt for TXFRAMEEND event<br>"]
    pub(crate) fn nfct_inten300_txframeend_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TXFRAMEEND mwrite None write None rac None reset value false")
    }
    #[doc = "RXFRAMESTART: Enable or disable interrupt for RXFRAMESTART event<br>"]
    pub(crate) fn nfct_inten300_rxframestart_read(&self) -> MemResult<bool> {
        todo ! ("read RXFRAMESTART mwrite None write None rac None reset value false")
    }
    #[doc = "RXFRAMESTART: Enable or disable interrupt for RXFRAMESTART event<br>"]
    pub(crate) fn nfct_inten300_rxframestart_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RXFRAMESTART mwrite None write None rac None reset value false")
    }
    #[doc = "RXFRAMEEND: Enable or disable interrupt for RXFRAMEEND event<br>"]
    pub(crate) fn nfct_inten300_rxframeend_read(&self) -> MemResult<bool> {
        todo!(
            "read RXFRAMEEND mwrite None write None rac None reset value false"
        )
    }
    #[doc = "RXFRAMEEND: Enable or disable interrupt for RXFRAMEEND event<br>"]
    pub(crate) fn nfct_inten300_rxframeend_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RXFRAMEEND mwrite None write None rac None reset value false")
    }
    #[doc = "ERROR: Enable or disable interrupt for ERROR event<br>"]
    pub(crate) fn nfct_inten300_error_read(&self) -> MemResult<bool> {
        todo!("read ERROR mwrite None write None rac None reset value false")
    }
    #[doc = "ERROR: Enable or disable interrupt for ERROR event<br>"]
    pub(crate) fn nfct_inten300_error_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ERROR mwrite None write None rac None reset value false")
    }
    #[doc = "RXERROR: Enable or disable interrupt for RXERROR event<br>"]
    pub(crate) fn nfct_inten300_rxerror_read(&self) -> MemResult<bool> {
        todo!("read RXERROR mwrite None write None rac None reset value false")
    }
    #[doc = "RXERROR: Enable or disable interrupt for RXERROR event<br>"]
    pub(crate) fn nfct_inten300_rxerror_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RXERROR mwrite None write None rac None reset value false")
    }
    #[doc = "ENDRX: Enable or disable interrupt for ENDRX event<br>"]
    pub(crate) fn nfct_inten300_endrx_read(&self) -> MemResult<bool> {
        todo!("read ENDRX mwrite None write None rac None reset value false")
    }
    #[doc = "ENDRX: Enable or disable interrupt for ENDRX event<br>"]
    pub(crate) fn nfct_inten300_endrx_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ENDRX mwrite None write None rac None reset value false")
    }
    #[doc = "ENDTX: Enable or disable interrupt for ENDTX event<br>"]
    pub(crate) fn nfct_inten300_endtx_read(&self) -> MemResult<bool> {
        todo!("read ENDTX mwrite None write None rac None reset value false")
    }
    #[doc = "ENDTX: Enable or disable interrupt for ENDTX event<br>"]
    pub(crate) fn nfct_inten300_endtx_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ENDTX mwrite None write None rac None reset value false")
    }
    #[doc = "AUTOCOLRESSTARTED: Enable or disable interrupt for AUTOCOLRESSTARTED event<br>"]
    pub(crate) fn nfct_inten300_autocolresstarted_read(
        &self,
    ) -> MemResult<bool> {
        todo ! ("read AUTOCOLRESSTARTED mwrite None write None rac None reset value false")
    }
    #[doc = "AUTOCOLRESSTARTED: Enable or disable interrupt for AUTOCOLRESSTARTED event<br>"]
    pub(crate) fn nfct_inten300_autocolresstarted_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write AUTOCOLRESSTARTED mwrite None write None rac None reset value false")
    }
    #[doc = "COLLISION: Enable or disable interrupt for COLLISION event<br>"]
    pub(crate) fn nfct_inten300_collision_read(&self) -> MemResult<bool> {
        todo!(
            "read COLLISION mwrite None write None rac None reset value false"
        )
    }
    #[doc = "COLLISION: Enable or disable interrupt for COLLISION event<br>"]
    pub(crate) fn nfct_inten300_collision_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write COLLISION mwrite None write None rac None reset value false"
        )
    }
    #[doc = "SELECTED: Enable or disable interrupt for SELECTED event<br>"]
    pub(crate) fn nfct_inten300_selected_read(&self) -> MemResult<bool> {
        todo!("read SELECTED mwrite None write None rac None reset value false")
    }
    #[doc = "SELECTED: Enable or disable interrupt for SELECTED event<br>"]
    pub(crate) fn nfct_inten300_selected_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write SELECTED mwrite None write None rac None reset value false"
        )
    }
    #[doc = "STARTED: Enable or disable interrupt for STARTED event<br>"]
    pub(crate) fn nfct_inten300_started_read(&self) -> MemResult<bool> {
        todo!("read STARTED mwrite None write None rac None reset value false")
    }
    #[doc = "STARTED: Enable or disable interrupt for STARTED event<br>"]
    pub(crate) fn nfct_inten300_started_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write STARTED mwrite None write None rac None reset value false")
    }
    #[doc = "READY: Write '1' to Enable interrupt for READY event<br>"]
    pub(crate) fn nfct_intenset304_ready_read(&self) -> MemResult<bool> {
        todo!("read READY mwrite None write None rac None reset value false")
    }
    #[doc = "READY: Write '1' to Enable interrupt for READY event<br>"]
    pub(crate) fn nfct_intenset304_ready_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write READY mwrite None write None rac None reset value false")
    }
    #[doc = "FIELDDETECTED: Write '1' to Enable interrupt for FIELDDETECTED event<br>"]
    pub(crate) fn nfct_intenset304_fielddetected_read(
        &self,
    ) -> MemResult<bool> {
        todo ! ("read FIELDDETECTED mwrite None write None rac None reset value false")
    }
    #[doc = "FIELDDETECTED: Write '1' to Enable interrupt for FIELDDETECTED event<br>"]
    pub(crate) fn nfct_intenset304_fielddetected_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write FIELDDETECTED mwrite None write None rac None reset value false")
    }
    #[doc = "FIELDLOST: Write '1' to Enable interrupt for FIELDLOST event<br>"]
    pub(crate) fn nfct_intenset304_fieldlost_read(&self) -> MemResult<bool> {
        todo!(
            "read FIELDLOST mwrite None write None rac None reset value false"
        )
    }
    #[doc = "FIELDLOST: Write '1' to Enable interrupt for FIELDLOST event<br>"]
    pub(crate) fn nfct_intenset304_fieldlost_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write FIELDLOST mwrite None write None rac None reset value false"
        )
    }
    #[doc = "TXFRAMESTART: Write '1' to Enable interrupt for TXFRAMESTART event<br>"]
    pub(crate) fn nfct_intenset304_txframestart_read(&self) -> MemResult<bool> {
        todo ! ("read TXFRAMESTART mwrite None write None rac None reset value false")
    }
    #[doc = "TXFRAMESTART: Write '1' to Enable interrupt for TXFRAMESTART event<br>"]
    pub(crate) fn nfct_intenset304_txframestart_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TXFRAMESTART mwrite None write None rac None reset value false")
    }
    #[doc = "TXFRAMEEND: Write '1' to Enable interrupt for TXFRAMEEND event<br>"]
    pub(crate) fn nfct_intenset304_txframeend_read(&self) -> MemResult<bool> {
        todo!(
            "read TXFRAMEEND mwrite None write None rac None reset value false"
        )
    }
    #[doc = "TXFRAMEEND: Write '1' to Enable interrupt for TXFRAMEEND event<br>"]
    pub(crate) fn nfct_intenset304_txframeend_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TXFRAMEEND mwrite None write None rac None reset value false")
    }
    #[doc = "RXFRAMESTART: Write '1' to Enable interrupt for RXFRAMESTART event<br>"]
    pub(crate) fn nfct_intenset304_rxframestart_read(&self) -> MemResult<bool> {
        todo ! ("read RXFRAMESTART mwrite None write None rac None reset value false")
    }
    #[doc = "RXFRAMESTART: Write '1' to Enable interrupt for RXFRAMESTART event<br>"]
    pub(crate) fn nfct_intenset304_rxframestart_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RXFRAMESTART mwrite None write None rac None reset value false")
    }
    #[doc = "RXFRAMEEND: Write '1' to Enable interrupt for RXFRAMEEND event<br>"]
    pub(crate) fn nfct_intenset304_rxframeend_read(&self) -> MemResult<bool> {
        todo!(
            "read RXFRAMEEND mwrite None write None rac None reset value false"
        )
    }
    #[doc = "RXFRAMEEND: Write '1' to Enable interrupt for RXFRAMEEND event<br>"]
    pub(crate) fn nfct_intenset304_rxframeend_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RXFRAMEEND mwrite None write None rac None reset value false")
    }
    #[doc = "ERROR: Write '1' to Enable interrupt for ERROR event<br>"]
    pub(crate) fn nfct_intenset304_error_read(&self) -> MemResult<bool> {
        todo!("read ERROR mwrite None write None rac None reset value false")
    }
    #[doc = "ERROR: Write '1' to Enable interrupt for ERROR event<br>"]
    pub(crate) fn nfct_intenset304_error_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ERROR mwrite None write None rac None reset value false")
    }
    #[doc = "RXERROR: Write '1' to Enable interrupt for RXERROR event<br>"]
    pub(crate) fn nfct_intenset304_rxerror_read(&self) -> MemResult<bool> {
        todo!("read RXERROR mwrite None write None rac None reset value false")
    }
    #[doc = "RXERROR: Write '1' to Enable interrupt for RXERROR event<br>"]
    pub(crate) fn nfct_intenset304_rxerror_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RXERROR mwrite None write None rac None reset value false")
    }
    #[doc = "ENDRX: Write '1' to Enable interrupt for ENDRX event<br>"]
    pub(crate) fn nfct_intenset304_endrx_read(&self) -> MemResult<bool> {
        todo!("read ENDRX mwrite None write None rac None reset value false")
    }
    #[doc = "ENDRX: Write '1' to Enable interrupt for ENDRX event<br>"]
    pub(crate) fn nfct_intenset304_endrx_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ENDRX mwrite None write None rac None reset value false")
    }
    #[doc = "ENDTX: Write '1' to Enable interrupt for ENDTX event<br>"]
    pub(crate) fn nfct_intenset304_endtx_read(&self) -> MemResult<bool> {
        todo!("read ENDTX mwrite None write None rac None reset value false")
    }
    #[doc = "ENDTX: Write '1' to Enable interrupt for ENDTX event<br>"]
    pub(crate) fn nfct_intenset304_endtx_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ENDTX mwrite None write None rac None reset value false")
    }
    #[doc = "AUTOCOLRESSTARTED: Write '1' to Enable interrupt for AUTOCOLRESSTARTED event<br>"]
    pub(crate) fn nfct_intenset304_autocolresstarted_read(
        &self,
    ) -> MemResult<bool> {
        todo ! ("read AUTOCOLRESSTARTED mwrite None write None rac None reset value false")
    }
    #[doc = "AUTOCOLRESSTARTED: Write '1' to Enable interrupt for AUTOCOLRESSTARTED event<br>"]
    pub(crate) fn nfct_intenset304_autocolresstarted_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write AUTOCOLRESSTARTED mwrite None write None rac None reset value false")
    }
    #[doc = "COLLISION: Write '1' to Enable interrupt for COLLISION event<br>"]
    pub(crate) fn nfct_intenset304_collision_read(&self) -> MemResult<bool> {
        todo!(
            "read COLLISION mwrite None write None rac None reset value false"
        )
    }
    #[doc = "COLLISION: Write '1' to Enable interrupt for COLLISION event<br>"]
    pub(crate) fn nfct_intenset304_collision_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write COLLISION mwrite None write None rac None reset value false"
        )
    }
    #[doc = "SELECTED: Write '1' to Enable interrupt for SELECTED event<br>"]
    pub(crate) fn nfct_intenset304_selected_read(&self) -> MemResult<bool> {
        todo!("read SELECTED mwrite None write None rac None reset value false")
    }
    #[doc = "SELECTED: Write '1' to Enable interrupt for SELECTED event<br>"]
    pub(crate) fn nfct_intenset304_selected_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write SELECTED mwrite None write None rac None reset value false"
        )
    }
    #[doc = "STARTED: Write '1' to Enable interrupt for STARTED event<br>"]
    pub(crate) fn nfct_intenset304_started_read(&self) -> MemResult<bool> {
        todo!("read STARTED mwrite None write None rac None reset value false")
    }
    #[doc = "STARTED: Write '1' to Enable interrupt for STARTED event<br>"]
    pub(crate) fn nfct_intenset304_started_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write STARTED mwrite None write None rac None reset value false")
    }
    #[doc = "READY: Write '1' to Disable interrupt for READY event<br>"]
    pub(crate) fn nfct_intenclr308_ready_read(&self) -> MemResult<bool> {
        todo!("read READY mwrite None write None rac None reset value false")
    }
    #[doc = "READY: Write '1' to Disable interrupt for READY event<br>"]
    pub(crate) fn nfct_intenclr308_ready_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write READY mwrite None write None rac None reset value false")
    }
    #[doc = "FIELDDETECTED: Write '1' to Disable interrupt for FIELDDETECTED event<br>"]
    pub(crate) fn nfct_intenclr308_fielddetected_read(
        &self,
    ) -> MemResult<bool> {
        todo ! ("read FIELDDETECTED mwrite None write None rac None reset value false")
    }
    #[doc = "FIELDDETECTED: Write '1' to Disable interrupt for FIELDDETECTED event<br>"]
    pub(crate) fn nfct_intenclr308_fielddetected_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write FIELDDETECTED mwrite None write None rac None reset value false")
    }
    #[doc = "FIELDLOST: Write '1' to Disable interrupt for FIELDLOST event<br>"]
    pub(crate) fn nfct_intenclr308_fieldlost_read(&self) -> MemResult<bool> {
        todo!(
            "read FIELDLOST mwrite None write None rac None reset value false"
        )
    }
    #[doc = "FIELDLOST: Write '1' to Disable interrupt for FIELDLOST event<br>"]
    pub(crate) fn nfct_intenclr308_fieldlost_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write FIELDLOST mwrite None write None rac None reset value false"
        )
    }
    #[doc = "TXFRAMESTART: Write '1' to Disable interrupt for TXFRAMESTART event<br>"]
    pub(crate) fn nfct_intenclr308_txframestart_read(&self) -> MemResult<bool> {
        todo ! ("read TXFRAMESTART mwrite None write None rac None reset value false")
    }
    #[doc = "TXFRAMESTART: Write '1' to Disable interrupt for TXFRAMESTART event<br>"]
    pub(crate) fn nfct_intenclr308_txframestart_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TXFRAMESTART mwrite None write None rac None reset value false")
    }
    #[doc = "TXFRAMEEND: Write '1' to Disable interrupt for TXFRAMEEND event<br>"]
    pub(crate) fn nfct_intenclr308_txframeend_read(&self) -> MemResult<bool> {
        todo!(
            "read TXFRAMEEND mwrite None write None rac None reset value false"
        )
    }
    #[doc = "TXFRAMEEND: Write '1' to Disable interrupt for TXFRAMEEND event<br>"]
    pub(crate) fn nfct_intenclr308_txframeend_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write TXFRAMEEND mwrite None write None rac None reset value false")
    }
    #[doc = "RXFRAMESTART: Write '1' to Disable interrupt for RXFRAMESTART event<br>"]
    pub(crate) fn nfct_intenclr308_rxframestart_read(&self) -> MemResult<bool> {
        todo ! ("read RXFRAMESTART mwrite None write None rac None reset value false")
    }
    #[doc = "RXFRAMESTART: Write '1' to Disable interrupt for RXFRAMESTART event<br>"]
    pub(crate) fn nfct_intenclr308_rxframestart_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RXFRAMESTART mwrite None write None rac None reset value false")
    }
    #[doc = "RXFRAMEEND: Write '1' to Disable interrupt for RXFRAMEEND event<br>"]
    pub(crate) fn nfct_intenclr308_rxframeend_read(&self) -> MemResult<bool> {
        todo!(
            "read RXFRAMEEND mwrite None write None rac None reset value false"
        )
    }
    #[doc = "RXFRAMEEND: Write '1' to Disable interrupt for RXFRAMEEND event<br>"]
    pub(crate) fn nfct_intenclr308_rxframeend_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RXFRAMEEND mwrite None write None rac None reset value false")
    }
    #[doc = "ERROR: Write '1' to Disable interrupt for ERROR event<br>"]
    pub(crate) fn nfct_intenclr308_error_read(&self) -> MemResult<bool> {
        todo!("read ERROR mwrite None write None rac None reset value false")
    }
    #[doc = "ERROR: Write '1' to Disable interrupt for ERROR event<br>"]
    pub(crate) fn nfct_intenclr308_error_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ERROR mwrite None write None rac None reset value false")
    }
    #[doc = "RXERROR: Write '1' to Disable interrupt for RXERROR event<br>"]
    pub(crate) fn nfct_intenclr308_rxerror_read(&self) -> MemResult<bool> {
        todo!("read RXERROR mwrite None write None rac None reset value false")
    }
    #[doc = "RXERROR: Write '1' to Disable interrupt for RXERROR event<br>"]
    pub(crate) fn nfct_intenclr308_rxerror_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RXERROR mwrite None write None rac None reset value false")
    }
    #[doc = "ENDRX: Write '1' to Disable interrupt for ENDRX event<br>"]
    pub(crate) fn nfct_intenclr308_endrx_read(&self) -> MemResult<bool> {
        todo!("read ENDRX mwrite None write None rac None reset value false")
    }
    #[doc = "ENDRX: Write '1' to Disable interrupt for ENDRX event<br>"]
    pub(crate) fn nfct_intenclr308_endrx_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ENDRX mwrite None write None rac None reset value false")
    }
    #[doc = "ENDTX: Write '1' to Disable interrupt for ENDTX event<br>"]
    pub(crate) fn nfct_intenclr308_endtx_read(&self) -> MemResult<bool> {
        todo!("read ENDTX mwrite None write None rac None reset value false")
    }
    #[doc = "ENDTX: Write '1' to Disable interrupt for ENDTX event<br>"]
    pub(crate) fn nfct_intenclr308_endtx_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ENDTX mwrite None write None rac None reset value false")
    }
    #[doc = "AUTOCOLRESSTARTED: Write '1' to Disable interrupt for AUTOCOLRESSTARTED event<br>"]
    pub(crate) fn nfct_intenclr308_autocolresstarted_read(
        &self,
    ) -> MemResult<bool> {
        todo ! ("read AUTOCOLRESSTARTED mwrite None write None rac None reset value false")
    }
    #[doc = "AUTOCOLRESSTARTED: Write '1' to Disable interrupt for AUTOCOLRESSTARTED event<br>"]
    pub(crate) fn nfct_intenclr308_autocolresstarted_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write AUTOCOLRESSTARTED mwrite None write None rac None reset value false")
    }
    #[doc = "COLLISION: Write '1' to Disable interrupt for COLLISION event<br>"]
    pub(crate) fn nfct_intenclr308_collision_read(&self) -> MemResult<bool> {
        todo!(
            "read COLLISION mwrite None write None rac None reset value false"
        )
    }
    #[doc = "COLLISION: Write '1' to Disable interrupt for COLLISION event<br>"]
    pub(crate) fn nfct_intenclr308_collision_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write COLLISION mwrite None write None rac None reset value false"
        )
    }
    #[doc = "SELECTED: Write '1' to Disable interrupt for SELECTED event<br>"]
    pub(crate) fn nfct_intenclr308_selected_read(&self) -> MemResult<bool> {
        todo!("read SELECTED mwrite None write None rac None reset value false")
    }
    #[doc = "SELECTED: Write '1' to Disable interrupt for SELECTED event<br>"]
    pub(crate) fn nfct_intenclr308_selected_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write SELECTED mwrite None write None rac None reset value false"
        )
    }
    #[doc = "STARTED: Write '1' to Disable interrupt for STARTED event<br>"]
    pub(crate) fn nfct_intenclr308_started_read(&self) -> MemResult<bool> {
        todo!("read STARTED mwrite None write None rac None reset value false")
    }
    #[doc = "STARTED: Write '1' to Disable interrupt for STARTED event<br>"]
    pub(crate) fn nfct_intenclr308_started_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write STARTED mwrite None write None rac None reset value false")
    }
    #[doc = "FRAMEDELAYTIMEOUT: No STARTTX task triggered before expiration of the time set in FRAMEDELAYMAX<br>"]
    pub(crate) fn nfct_errorstatus404_framedelaytimeout_read(
        &self,
    ) -> MemResult<bool> {
        todo ! ("read FRAMEDELAYTIMEOUT mwrite None write None rac None reset value false")
    }
    #[doc = "FRAMEDELAYTIMEOUT: No STARTTX task triggered before expiration of the time set in FRAMEDELAYMAX<br>"]
    pub(crate) fn nfct_errorstatus404_framedelaytimeout_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write FRAMEDELAYTIMEOUT mwrite None write None rac None reset value false")
    }
    #[doc = "NFCFIELDTOOSTRONG: Field level is too high at max load resistance<br>"]
    pub(crate) fn nfct_errorstatus404_nfcfieldtoostrong_read(
        &self,
    ) -> MemResult<bool> {
        todo ! ("read NFCFIELDTOOSTRONG mwrite None write None rac None reset value false")
    }
    #[doc = "NFCFIELDTOOSTRONG: Field level is too high at max load resistance<br>"]
    pub(crate) fn nfct_errorstatus404_nfcfieldtoostrong_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write NFCFIELDTOOSTRONG mwrite None write None rac None reset value false")
    }
    #[doc = "NFCFIELDTOOWEAK: Field level is too low at min load resistance<br>"]
    pub(crate) fn nfct_errorstatus404_nfcfieldtooweak_read(
        &self,
    ) -> MemResult<bool> {
        todo ! ("read NFCFIELDTOOWEAK mwrite None write None rac None reset value false")
    }
    #[doc = "NFCFIELDTOOWEAK: Field level is too low at min load resistance<br>"]
    pub(crate) fn nfct_errorstatus404_nfcfieldtooweak_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write NFCFIELDTOOWEAK mwrite None write None rac None reset value false")
    }
    #[doc = "CRCERROR: No valid End of Frame detected<br>"]
    pub(crate) fn nfct_framestatus_rx0_crcerror_read(&self) -> MemResult<bool> {
        todo!("read CRCERROR mwrite None write None rac None reset value false")
    }
    #[doc = "CRCERROR: No valid End of Frame detected<br>"]
    pub(crate) fn nfct_framestatus_rx0_crcerror_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CRCERROR mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PARITYSTATUS: Parity status of received frame<br>"]
    pub(crate) fn nfct_framestatus_rx0_paritystatus_read(
        &self,
    ) -> MemResult<bool> {
        todo ! ("read PARITYSTATUS mwrite None write None rac None reset value false")
    }
    #[doc = "PARITYSTATUS: Parity status of received frame<br>"]
    pub(crate) fn nfct_framestatus_rx0_paritystatus_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write PARITYSTATUS mwrite None write None rac None reset value false")
    }
    #[doc = "OVERRUN: Overrun detected<br>"]
    pub(crate) fn nfct_framestatus_rx0_overrun_read(&self) -> MemResult<bool> {
        todo!("read OVERRUN mwrite None write None rac None reset value false")
    }
    #[doc = "OVERRUN: Overrun detected<br>"]
    pub(crate) fn nfct_framestatus_rx0_overrun_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write OVERRUN mwrite None write None rac None reset value false")
    }
    #[doc = "CURRENTLOADCTRL: Current value driven to the NFC Load Control<br>"]
    pub(crate) fn nfct_currentloadctrl430_currentloadctrl_read(
        &self,
    ) -> MemResult<u8> {
        todo ! ("read CURRENTLOADCTRL mwrite None write None rac None reset value 0x00 mask 0x3f")
    }
    #[doc = "FIELDPRESENT: Indicates the presence or not of a valid field. Available only in the activated state.<br>"]
    pub(crate) fn nfct_fieldpresent43c_fieldpresent_read(
        &self,
    ) -> MemResult<bool> {
        todo ! ("read FIELDPRESENT mwrite None write None rac None reset value false")
    }
    #[doc = "LOCKDETECT: Indicates if the low level has locked to the field<br>"]
    pub(crate) fn nfct_fieldpresent43c_lockdetect_read(
        &self,
    ) -> MemResult<bool> {
        todo!(
            "read LOCKDETECT mwrite None write None rac None reset value false"
        )
    }
    #[doc = "FRAMEDELAYMIN: Minimum frame delay in number of 13.56 MHz clocks<br>"]
    pub(crate) fn nfct_framedelaymin504_framedelaymin_read(
        &self,
    ) -> MemResult<u16> {
        todo ! ("read FRAMEDELAYMIN mwrite None write None rac None reset value 0x480 mask 0xffff")
    }
    #[doc = "FRAMEDELAYMIN: Minimum frame delay in number of 13.56 MHz clocks<br>"]
    pub(crate) fn nfct_framedelaymin504_framedelaymin_write(
        &mut self,
        _value: u16,
    ) -> MemResult<()> {
        todo ! ("write FRAMEDELAYMIN mwrite None write None rac None reset value 0x480 mask 0xffff")
    }
    #[doc = "FRAMEDELAYMAX: Maximum frame delay in number of 13.56 MHz clocks<br>"]
    pub(crate) fn nfct_framedelaymax508_framedelaymax_read(
        &self,
    ) -> MemResult<u16> {
        todo ! ("read FRAMEDELAYMAX mwrite None write None rac None reset value 0x1000 mask 0xffff")
    }
    #[doc = "FRAMEDELAYMAX: Maximum frame delay in number of 13.56 MHz clocks<br>"]
    pub(crate) fn nfct_framedelaymax508_framedelaymax_write(
        &mut self,
        _value: u16,
    ) -> MemResult<()> {
        todo ! ("write FRAMEDELAYMAX mwrite None write None rac None reset value 0x1000 mask 0xffff")
    }
    #[doc = "FRAMEDELAYMODE: Configuration register for the Frame Delay Timer<br>"]
    pub(crate) fn nfct_framedelaymode50c_framedelaymode_read(
        &self,
    ) -> MemResult<
        crate::peripheral::enums::E25NfctFramedelaymode50cFramedelaymode,
    > {
        todo ! ("read FRAMEDELAYMODE mwrite None write None rac None reset value 0x01 mask 0x03")
    }
    #[doc = "FRAMEDELAYMODE: Configuration register for the Frame Delay Timer<br>"]
    pub(crate) fn nfct_framedelaymode50c_framedelaymode_write(
        &mut self,
        _value : crate :: peripheral :: enums :: E25NfctFramedelaymode50cFramedelaymode,
    ) -> MemResult<()> {
        todo ! ("write FRAMEDELAYMODE mwrite None write None rac None reset value 0x01 mask 0x03")
    }
    #[doc = "PTR: Packet pointer for TXD and RXD data storage in Data RAM. This address is a byte aligned RAM address.<br>"]
    pub(crate) fn nfct_packetptr510_ptr_read(&self) -> MemResult<u32> {
        todo ! ("read PTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "PTR: Packet pointer for TXD and RXD data storage in Data RAM. This address is a byte aligned RAM address.<br>"]
    pub(crate) fn nfct_packetptr510_ptr_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write PTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "MAXLEN: Size of allocated for TXD and RXD data storage buffer in Data RAM<br>"]
    pub(crate) fn nfct_maxlen514_maxlen_read(&self) -> MemResult<u16> {
        todo ! ("read MAXLEN mwrite None write None rac None reset value 0x00 mask 0x1ff")
    }
    #[doc = "MAXLEN: Size of allocated for TXD and RXD data storage buffer in Data RAM<br>"]
    pub(crate) fn nfct_maxlen514_maxlen_write(
        &mut self,
        _value: u16,
    ) -> MemResult<()> {
        todo ! ("write MAXLEN mwrite None write None rac None reset value 0x00 mask 0x1ff")
    }
    #[doc = "PARITY: Adding parity or not in the frame<br>"]
    pub(crate) fn nfct_txd_frameconfig0_parity_read(&self) -> MemResult<bool> {
        todo!("read PARITY mwrite None write None rac None reset value true")
    }
    #[doc = "PARITY: Adding parity or not in the frame<br>"]
    pub(crate) fn nfct_txd_frameconfig0_parity_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PARITY mwrite None write None rac None reset value true")
    }
    #[doc = "DISCARDMODE: Discarding unused bits in start or at end of a Frame<br>"]
    pub(crate) fn nfct_txd_frameconfig0_discardmode_read(
        &self,
    ) -> MemResult<bool> {
        todo!(
            "read DISCARDMODE mwrite None write None rac None reset value true"
        )
    }
    #[doc = "DISCARDMODE: Discarding unused bits in start or at end of a Frame<br>"]
    pub(crate) fn nfct_txd_frameconfig0_discardmode_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write DISCARDMODE mwrite None write None rac None reset value true")
    }
    #[doc = "SOF: Adding SoF or not in TX frames<br>"]
    pub(crate) fn nfct_txd_frameconfig0_sof_read(&self) -> MemResult<bool> {
        todo!("read SOF mwrite None write None rac None reset value true")
    }
    #[doc = "SOF: Adding SoF or not in TX frames<br>"]
    pub(crate) fn nfct_txd_frameconfig0_sof_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SOF mwrite None write None rac None reset value true")
    }
    #[doc = "CRCMODETX: CRC mode for outgoing frames<br>"]
    pub(crate) fn nfct_txd_frameconfig0_crcmodetx_read(
        &self,
    ) -> MemResult<bool> {
        todo!("read CRCMODETX mwrite None write None rac None reset value true")
    }
    #[doc = "CRCMODETX: CRC mode for outgoing frames<br>"]
    pub(crate) fn nfct_txd_frameconfig0_crcmodetx_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CRCMODETX mwrite None write None rac None reset value true"
        )
    }
    #[doc = "TXDATABITS: Number of bits in the last or first byte read from RAM that shall be included in the frame (excluding parity bit).<br>"]
    pub(crate) fn nfct_txd_amount4_txdatabits_read(&self) -> MemResult<u8> {
        todo ! ("read TXDATABITS mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "TXDATABITS: Number of bits in the last or first byte read from RAM that shall be included in the frame (excluding parity bit).<br>"]
    pub(crate) fn nfct_txd_amount4_txdatabits_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write TXDATABITS mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "TXDATABYTES: Number of complete bytes that shall be included in the frame, excluding CRC, parity and framing<br>"]
    pub(crate) fn nfct_txd_amount4_txdatabytes_read(&self) -> MemResult<u16> {
        todo ! ("read TXDATABYTES mwrite None write None rac None reset value 0x00 mask 0x1ff")
    }
    #[doc = "TXDATABYTES: Number of complete bytes that shall be included in the frame, excluding CRC, parity and framing<br>"]
    pub(crate) fn nfct_txd_amount4_txdatabytes_write(
        &mut self,
        _value: u16,
    ) -> MemResult<()> {
        todo ! ("write TXDATABYTES mwrite None write None rac None reset value 0x00 mask 0x1ff")
    }
    #[doc = "PARITY: Parity expected or not in RX frame<br>"]
    pub(crate) fn nfct_rxd_frameconfig0_parity_read(&self) -> MemResult<bool> {
        todo!("read PARITY mwrite None write None rac None reset value true")
    }
    #[doc = "PARITY: Parity expected or not in RX frame<br>"]
    pub(crate) fn nfct_rxd_frameconfig0_parity_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PARITY mwrite None write None rac None reset value true")
    }
    #[doc = "SOF: SoF expected or not in RX frames<br>"]
    pub(crate) fn nfct_rxd_frameconfig0_sof_read(&self) -> MemResult<bool> {
        todo!("read SOF mwrite None write None rac None reset value true")
    }
    #[doc = "SOF: SoF expected or not in RX frames<br>"]
    pub(crate) fn nfct_rxd_frameconfig0_sof_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SOF mwrite None write None rac None reset value true")
    }
    #[doc = "CRCMODERX: CRC mode for incoming frames<br>"]
    pub(crate) fn nfct_rxd_frameconfig0_crcmoderx_read(
        &self,
    ) -> MemResult<bool> {
        todo!("read CRCMODERX mwrite None write None rac None reset value true")
    }
    #[doc = "CRCMODERX: CRC mode for incoming frames<br>"]
    pub(crate) fn nfct_rxd_frameconfig0_crcmoderx_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CRCMODERX mwrite None write None rac None reset value true"
        )
    }
    #[doc = "RXDATABITS: Number of bits in the last byte in the frame, if less than 8 (including CRC, but excluding parity and SoF/EoF framing).<br>"]
    pub(crate) fn nfct_rxd_amount4_rxdatabits_read(&self) -> MemResult<u8> {
        todo ! ("read RXDATABITS mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "RXDATABYTES: Number of complete bytes received in the frame (including CRC, but excluding parity and SoF/EoF framing)<br>"]
    pub(crate) fn nfct_rxd_amount4_rxdatabytes_read(&self) -> MemResult<u16> {
        todo ! ("read RXDATABYTES mwrite None write None rac None reset value 0x00 mask 0x1ff")
    }
    #[doc = "NFCID1_Z: NFCID1 byte Z (very last byte sent)<br>"]
    pub(crate) fn nfct_nfcid1_last590_nfcid1_z_read(&self) -> MemResult<u8> {
        todo ! ("read NFCID1_Z mwrite None write None rac None reset value 0x63 mask 0xff")
    }
    #[doc = "NFCID1_Z: NFCID1 byte Z (very last byte sent)<br>"]
    pub(crate) fn nfct_nfcid1_last590_nfcid1_z_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write NFCID1_Z mwrite None write None rac None reset value 0x63 mask 0xff")
    }
    #[doc = "NFCID1_Y: NFCID1 byte Y<br>"]
    pub(crate) fn nfct_nfcid1_last590_nfcid1_y_read(&self) -> MemResult<u8> {
        todo ! ("read NFCID1_Y mwrite None write None rac None reset value 0x63 mask 0xff")
    }
    #[doc = "NFCID1_Y: NFCID1 byte Y<br>"]
    pub(crate) fn nfct_nfcid1_last590_nfcid1_y_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write NFCID1_Y mwrite None write None rac None reset value 0x63 mask 0xff")
    }
    #[doc = "NFCID1_X: NFCID1 byte X<br>"]
    pub(crate) fn nfct_nfcid1_last590_nfcid1_x_read(&self) -> MemResult<u8> {
        todo ! ("read NFCID1_X mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "NFCID1_X: NFCID1 byte X<br>"]
    pub(crate) fn nfct_nfcid1_last590_nfcid1_x_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write NFCID1_X mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "NFCID1_W: NFCID1 byte W<br>"]
    pub(crate) fn nfct_nfcid1_last590_nfcid1_w_read(&self) -> MemResult<u8> {
        todo ! ("read NFCID1_W mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "NFCID1_W: NFCID1 byte W<br>"]
    pub(crate) fn nfct_nfcid1_last590_nfcid1_w_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write NFCID1_W mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "NFCID1_V: NFCID1 byte V<br>"]
    pub(crate) fn nfct_nfcid1_2nd_last594_nfcid1_v_read(
        &self,
    ) -> MemResult<u8> {
        todo ! ("read NFCID1_V mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "NFCID1_V: NFCID1 byte V<br>"]
    pub(crate) fn nfct_nfcid1_2nd_last594_nfcid1_v_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write NFCID1_V mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "NFCID1_U: NFCID1 byte U<br>"]
    pub(crate) fn nfct_nfcid1_2nd_last594_nfcid1_u_read(
        &self,
    ) -> MemResult<u8> {
        todo ! ("read NFCID1_U mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "NFCID1_U: NFCID1 byte U<br>"]
    pub(crate) fn nfct_nfcid1_2nd_last594_nfcid1_u_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write NFCID1_U mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "NFCID1_T: NFCID1 byte T<br>"]
    pub(crate) fn nfct_nfcid1_2nd_last594_nfcid1_t_read(
        &self,
    ) -> MemResult<u8> {
        todo ! ("read NFCID1_T mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "NFCID1_T: NFCID1 byte T<br>"]
    pub(crate) fn nfct_nfcid1_2nd_last594_nfcid1_t_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write NFCID1_T mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "NFCID1_S: NFCID1 byte S<br>"]
    pub(crate) fn nfct_nfcid1_3rd_last598_nfcid1_s_read(
        &self,
    ) -> MemResult<u8> {
        todo ! ("read NFCID1_S mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "NFCID1_S: NFCID1 byte S<br>"]
    pub(crate) fn nfct_nfcid1_3rd_last598_nfcid1_s_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write NFCID1_S mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "NFCID1_R: NFCID1 byte R<br>"]
    pub(crate) fn nfct_nfcid1_3rd_last598_nfcid1_r_read(
        &self,
    ) -> MemResult<u8> {
        todo ! ("read NFCID1_R mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "NFCID1_R: NFCID1 byte R<br>"]
    pub(crate) fn nfct_nfcid1_3rd_last598_nfcid1_r_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write NFCID1_R mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "NFCID1_Q: NFCID1 byte Q<br>"]
    pub(crate) fn nfct_nfcid1_3rd_last598_nfcid1_q_read(
        &self,
    ) -> MemResult<u8> {
        todo ! ("read NFCID1_Q mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "NFCID1_Q: NFCID1 byte Q<br>"]
    pub(crate) fn nfct_nfcid1_3rd_last598_nfcid1_q_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write NFCID1_Q mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "BITFRAMESDD: Bit frame SDD as defined by the b5:b1 of byte 1 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification<br>"]
    pub(crate) fn nfct_sensres5a0_bitframesdd_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E26NfctSensres5a0Bitframesdd> {
        todo ! ("read BITFRAMESDD mwrite None write None rac None reset value 0x01 mask 0x1f")
    }
    #[doc = "BITFRAMESDD: Bit frame SDD as defined by the b5:b1 of byte 1 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification<br>"]
    pub(crate) fn nfct_sensres5a0_bitframesdd_write(
        &mut self,
        _value: crate::peripheral::enums::E26NfctSensres5a0Bitframesdd,
    ) -> MemResult<()> {
        todo ! ("write BITFRAMESDD mwrite None write None rac None reset value 0x01 mask 0x1f")
    }
    #[doc = "RFU5: Reserved for future use. Shall be 0.<br>"]
    pub(crate) fn nfct_sensres5a0_rfu5_read(&self) -> MemResult<bool> {
        todo!("read RFU5 mwrite None write None rac None reset value false")
    }
    #[doc = "RFU5: Reserved for future use. Shall be 0.<br>"]
    pub(crate) fn nfct_sensres5a0_rfu5_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RFU5 mwrite None write None rac None reset value false")
    }
    #[doc = "NFCIDSIZE: NFCID1 size. This value is used by the Auto collision resolution engine.<br>"]
    pub(crate) fn nfct_sensres5a0_nfcidsize_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E27NfctSensres5a0Nfcidsize> {
        todo ! ("read NFCIDSIZE mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "NFCIDSIZE: NFCID1 size. This value is used by the Auto collision resolution engine.<br>"]
    pub(crate) fn nfct_sensres5a0_nfcidsize_write(
        &mut self,
        _value: crate::peripheral::enums::E27NfctSensres5a0Nfcidsize,
    ) -> MemResult<()> {
        todo ! ("write NFCIDSIZE mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "PLATFCONFIG: Tag platform configuration as defined by the b4:b1 of byte 2 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification<br>"]
    pub(crate) fn nfct_sensres5a0_platfconfig_read(&self) -> MemResult<u8> {
        todo ! ("read PLATFCONFIG mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "PLATFCONFIG: Tag platform configuration as defined by the b4:b1 of byte 2 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification<br>"]
    pub(crate) fn nfct_sensres5a0_platfconfig_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write PLATFCONFIG mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "RFU74: Reserved for future use. Shall be 0.<br>"]
    pub(crate) fn nfct_sensres5a0_rfu74_read(&self) -> MemResult<u8> {
        todo ! ("read RFU74 mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "RFU74: Reserved for future use. Shall be 0.<br>"]
    pub(crate) fn nfct_sensres5a0_rfu74_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write RFU74 mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "RFU10: Reserved for future use. Shall be 0.<br>"]
    pub(crate) fn nfct_selres5a4_rfu10_read(&self) -> MemResult<u8> {
        todo ! ("read RFU10 mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "RFU10: Reserved for future use. Shall be 0.<br>"]
    pub(crate) fn nfct_selres5a4_rfu10_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write RFU10 mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "CASCADE: Cascade bit (controlled by hardware, write has no effect)<br>"]
    pub(crate) fn nfct_selres5a4_cascade_read(&self) -> MemResult<bool> {
        todo!("read CASCADE mwrite None write None rac None reset value false")
    }
    #[doc = "CASCADE: Cascade bit (controlled by hardware, write has no effect)<br>"]
    pub(crate) fn nfct_selres5a4_cascade_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CASCADE mwrite None write None rac None reset value false")
    }
    #[doc = "RFU43: Reserved for future use. Shall be 0.<br>"]
    pub(crate) fn nfct_selres5a4_rfu43_read(&self) -> MemResult<u8> {
        todo ! ("read RFU43 mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "RFU43: Reserved for future use. Shall be 0.<br>"]
    pub(crate) fn nfct_selres5a4_rfu43_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write RFU43 mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "PROTOCOL: Protocol as defined by the b7:b6 of SEL_RES response in the NFC Forum, NFC Digital Protocol Technical Specification<br>"]
    pub(crate) fn nfct_selres5a4_protocol_read(&self) -> MemResult<u8> {
        todo ! ("read PROTOCOL mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "PROTOCOL: Protocol as defined by the b7:b6 of SEL_RES response in the NFC Forum, NFC Digital Protocol Technical Specification<br>"]
    pub(crate) fn nfct_selres5a4_protocol_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write PROTOCOL mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "RFU7: Reserved for future use. Shall be 0.<br>"]
    pub(crate) fn nfct_selres5a4_rfu7_read(&self) -> MemResult<bool> {
        todo!("read RFU7 mwrite None write None rac None reset value false")
    }
    #[doc = "RFU7: Reserved for future use. Shall be 0.<br>"]
    pub(crate) fn nfct_selres5a4_rfu7_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RFU7 mwrite None write None rac None reset value false")
    }
}
