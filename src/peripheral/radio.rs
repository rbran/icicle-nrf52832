use icicle_vm::cpu::mem::MemResult;
#[derive(Default)]
#[doc = "RADIO: 2.4 GHz Radio<br><br>Instances:<br>0x40001000: RADIO<br>"]
pub struct Radio {
    pub address_prefix: [u8; 8],
    pub receive_on_ap: [bool; 8],
    pub tx_on_ap: [bool; 8],
    pub address_prefix_on: [bool; 8],
}
impl Radio {
    pub(crate) fn page_to_index(page: u64) -> usize {
        match page {
            262145u64 => 0usize,
            _ => unreachable!(),
        }
    }
    #[doc = "TASKS_TXEN: Enable RADIO in TX mode<br>"]
    pub(crate) fn radio_tasks_txen0_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write radio_tasks_txen0 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_RXEN: Enable RADIO in RX mode<br>"]
    pub(crate) fn radio_tasks_rxen4_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write radio_tasks_rxen4 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_START: Start RADIO<br>"]
    pub(crate) fn radio_tasks_start8_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write radio_tasks_start8 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_STOP: Stop RADIO<br>"]
    pub(crate) fn radio_tasks_stopc_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write radio_tasks_stopc mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_DISABLE: Disable RADIO<br>"]
    pub(crate) fn radio_tasks_disable10_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write radio_tasks_disable10 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_RSSISTART: Start the RSSI and take one single sample of the receive signal strength.<br>"]
    pub(crate) fn radio_tasks_rssistart14_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write radio_tasks_rssistart14 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_RSSISTOP: Stop the RSSI measurement<br>"]
    pub(crate) fn radio_tasks_rssistop18_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write radio_tasks_rssistop18 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_BCSTART: Start the bit counter<br>"]
    pub(crate) fn radio_tasks_bcstart1c_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write radio_tasks_bcstart1c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_BCSTOP: Stop the bit counter<br>"]
    pub(crate) fn radio_tasks_bcstop20_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write radio_tasks_bcstop20 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_READY: RADIO has ramped up and is ready to be started<br>"]
    pub(crate) fn radio_events_ready100_read(&self) -> MemResult<u32> {
        todo ! ("read radio_events_ready100 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_READY: RADIO has ramped up and is ready to be started<br>"]
    pub(crate) fn radio_events_ready100_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write radio_events_ready100 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ADDRESS: Address sent or received<br>"]
    pub(crate) fn radio_events_address104_read(&self) -> MemResult<u32> {
        todo ! ("read radio_events_address104 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ADDRESS: Address sent or received<br>"]
    pub(crate) fn radio_events_address104_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write radio_events_address104 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_PAYLOAD: Packet payload sent or received<br>"]
    pub(crate) fn radio_events_payload108_read(&self) -> MemResult<u32> {
        todo ! ("read radio_events_payload108 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_PAYLOAD: Packet payload sent or received<br>"]
    pub(crate) fn radio_events_payload108_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write radio_events_payload108 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_END: Packet sent or received<br>"]
    pub(crate) fn radio_events_end10c_read(&self) -> MemResult<u32> {
        todo ! ("read radio_events_end10c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_END: Packet sent or received<br>"]
    pub(crate) fn radio_events_end10c_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write radio_events_end10c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_DISABLED: RADIO has been disabled<br>"]
    pub(crate) fn radio_events_disabled110_read(&self) -> MemResult<u32> {
        todo ! ("read radio_events_disabled110 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_DISABLED: RADIO has been disabled<br>"]
    pub(crate) fn radio_events_disabled110_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write radio_events_disabled110 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_DEVMATCH: A device address match occurred on the last received packet<br>"]
    pub(crate) fn radio_events_devmatch114_read(&self) -> MemResult<u32> {
        todo ! ("read radio_events_devmatch114 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_DEVMATCH: A device address match occurred on the last received packet<br>"]
    pub(crate) fn radio_events_devmatch114_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write radio_events_devmatch114 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_DEVMISS: No device address match occurred on the last received packet<br>"]
    pub(crate) fn radio_events_devmiss118_read(&self) -> MemResult<u32> {
        todo ! ("read radio_events_devmiss118 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_DEVMISS: No device address match occurred on the last received packet<br>"]
    pub(crate) fn radio_events_devmiss118_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write radio_events_devmiss118 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_RSSIEND: Sampling of receive signal strength complete.<br>"]
    pub(crate) fn radio_events_rssiend11c_read(&self) -> MemResult<u32> {
        todo ! ("read radio_events_rssiend11c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_RSSIEND: Sampling of receive signal strength complete.<br>"]
    pub(crate) fn radio_events_rssiend11c_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write radio_events_rssiend11c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_BCMATCH: Bit counter reached bit count value.<br>"]
    pub(crate) fn radio_events_bcmatch128_read(&self) -> MemResult<u32> {
        todo ! ("read radio_events_bcmatch128 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_BCMATCH: Bit counter reached bit count value.<br>"]
    pub(crate) fn radio_events_bcmatch128_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write radio_events_bcmatch128 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_CRCOK: Packet received with CRC ok<br>"]
    pub(crate) fn radio_events_crcok130_read(&self) -> MemResult<u32> {
        todo ! ("read radio_events_crcok130 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_CRCOK: Packet received with CRC ok<br>"]
    pub(crate) fn radio_events_crcok130_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write radio_events_crcok130 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_CRCERROR: Packet received with CRC error<br>"]
    pub(crate) fn radio_events_crcerror134_read(&self) -> MemResult<u32> {
        todo ! ("read radio_events_crcerror134 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_CRCERROR: Packet received with CRC error<br>"]
    pub(crate) fn radio_events_crcerror134_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write radio_events_crcerror134 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "READY_START: Shortcut between READY event and START task<br>"]
    pub(crate) fn radio_shorts200_ready_start_read(&self) -> MemResult<bool> {
        todo ! ("read READY_START mwrite None write None rac None reset value false")
    }
    #[doc = "READY_START: Shortcut between READY event and START task<br>"]
    pub(crate) fn radio_shorts200_ready_start_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write READY_START mwrite None write None rac None reset value false")
    }
    #[doc = "END_DISABLE: Shortcut between END event and DISABLE task<br>"]
    pub(crate) fn radio_shorts200_end_disable_read(&self) -> MemResult<bool> {
        todo ! ("read END_DISABLE mwrite None write None rac None reset value false")
    }
    #[doc = "END_DISABLE: Shortcut between END event and DISABLE task<br>"]
    pub(crate) fn radio_shorts200_end_disable_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write END_DISABLE mwrite None write None rac None reset value false")
    }
    #[doc = "DISABLED_TXEN: Shortcut between DISABLED event and TXEN task<br>"]
    pub(crate) fn radio_shorts200_disabled_txen_read(&self) -> MemResult<bool> {
        todo ! ("read DISABLED_TXEN mwrite None write None rac None reset value false")
    }
    #[doc = "DISABLED_TXEN: Shortcut between DISABLED event and TXEN task<br>"]
    pub(crate) fn radio_shorts200_disabled_txen_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write DISABLED_TXEN mwrite None write None rac None reset value false")
    }
    #[doc = "DISABLED_RXEN: Shortcut between DISABLED event and RXEN task<br>"]
    pub(crate) fn radio_shorts200_disabled_rxen_read(&self) -> MemResult<bool> {
        todo ! ("read DISABLED_RXEN mwrite None write None rac None reset value false")
    }
    #[doc = "DISABLED_RXEN: Shortcut between DISABLED event and RXEN task<br>"]
    pub(crate) fn radio_shorts200_disabled_rxen_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write DISABLED_RXEN mwrite None write None rac None reset value false")
    }
    #[doc = "ADDRESS_RSSISTART: Shortcut between ADDRESS event and RSSISTART task<br>"]
    pub(crate) fn radio_shorts200_address_rssistart_read(
        &self,
    ) -> MemResult<bool> {
        todo ! ("read ADDRESS_RSSISTART mwrite None write None rac None reset value false")
    }
    #[doc = "ADDRESS_RSSISTART: Shortcut between ADDRESS event and RSSISTART task<br>"]
    pub(crate) fn radio_shorts200_address_rssistart_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write ADDRESS_RSSISTART mwrite None write None rac None reset value false")
    }
    #[doc = "END_START: Shortcut between END event and START task<br>"]
    pub(crate) fn radio_shorts200_end_start_read(&self) -> MemResult<bool> {
        todo!(
            "read END_START mwrite None write None rac None reset value false"
        )
    }
    #[doc = "END_START: Shortcut between END event and START task<br>"]
    pub(crate) fn radio_shorts200_end_start_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write END_START mwrite None write None rac None reset value false"
        )
    }
    #[doc = "ADDRESS_BCSTART: Shortcut between ADDRESS event and BCSTART task<br>"]
    pub(crate) fn radio_shorts200_address_bcstart_read(
        &self,
    ) -> MemResult<bool> {
        todo ! ("read ADDRESS_BCSTART mwrite None write None rac None reset value false")
    }
    #[doc = "ADDRESS_BCSTART: Shortcut between ADDRESS event and BCSTART task<br>"]
    pub(crate) fn radio_shorts200_address_bcstart_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write ADDRESS_BCSTART mwrite None write None rac None reset value false")
    }
    #[doc = "DISABLED_RSSISTOP: Shortcut between DISABLED event and RSSISTOP task<br>"]
    pub(crate) fn radio_shorts200_disabled_rssistop_read(
        &self,
    ) -> MemResult<bool> {
        todo ! ("read DISABLED_RSSISTOP mwrite None write None rac None reset value false")
    }
    #[doc = "DISABLED_RSSISTOP: Shortcut between DISABLED event and RSSISTOP task<br>"]
    pub(crate) fn radio_shorts200_disabled_rssistop_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write DISABLED_RSSISTOP mwrite None write None rac None reset value false")
    }
    #[doc = "READY: Write '1' to Enable interrupt for READY event<br>"]
    pub(crate) fn radio_intenset304_ready_read(&self) -> MemResult<bool> {
        todo!("read READY mwrite None write None rac None reset value false")
    }
    #[doc = "READY: Write '1' to Enable interrupt for READY event<br>"]
    pub(crate) fn radio_intenset304_ready_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write READY mwrite None write None rac None reset value false")
    }
    #[doc = "ADDRESS: Write '1' to Enable interrupt for ADDRESS event<br>"]
    pub(crate) fn radio_intenset304_address_read(&self) -> MemResult<bool> {
        todo!("read ADDRESS mwrite None write None rac None reset value false")
    }
    #[doc = "ADDRESS: Write '1' to Enable interrupt for ADDRESS event<br>"]
    pub(crate) fn radio_intenset304_address_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ADDRESS mwrite None write None rac None reset value false")
    }
    #[doc = "PAYLOAD: Write '1' to Enable interrupt for PAYLOAD event<br>"]
    pub(crate) fn radio_intenset304_payload_read(&self) -> MemResult<bool> {
        todo!("read PAYLOAD mwrite None write None rac None reset value false")
    }
    #[doc = "PAYLOAD: Write '1' to Enable interrupt for PAYLOAD event<br>"]
    pub(crate) fn radio_intenset304_payload_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PAYLOAD mwrite None write None rac None reset value false")
    }
    #[doc = "END: Write '1' to Enable interrupt for END event<br>"]
    pub(crate) fn radio_intenset304_end_read(&self) -> MemResult<bool> {
        todo!("read END mwrite None write None rac None reset value false")
    }
    #[doc = "END: Write '1' to Enable interrupt for END event<br>"]
    pub(crate) fn radio_intenset304_end_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write END mwrite None write None rac None reset value false")
    }
    #[doc = "DISABLED: Write '1' to Enable interrupt for DISABLED event<br>"]
    pub(crate) fn radio_intenset304_disabled_read(&self) -> MemResult<bool> {
        todo!("read DISABLED mwrite None write None rac None reset value false")
    }
    #[doc = "DISABLED: Write '1' to Enable interrupt for DISABLED event<br>"]
    pub(crate) fn radio_intenset304_disabled_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write DISABLED mwrite None write None rac None reset value false"
        )
    }
    #[doc = "DEVMATCH: Write '1' to Enable interrupt for DEVMATCH event<br>"]
    pub(crate) fn radio_intenset304_devmatch_read(&self) -> MemResult<bool> {
        todo!("read DEVMATCH mwrite None write None rac None reset value false")
    }
    #[doc = "DEVMATCH: Write '1' to Enable interrupt for DEVMATCH event<br>"]
    pub(crate) fn radio_intenset304_devmatch_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write DEVMATCH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "DEVMISS: Write '1' to Enable interrupt for DEVMISS event<br>"]
    pub(crate) fn radio_intenset304_devmiss_read(&self) -> MemResult<bool> {
        todo!("read DEVMISS mwrite None write None rac None reset value false")
    }
    #[doc = "DEVMISS: Write '1' to Enable interrupt for DEVMISS event<br>"]
    pub(crate) fn radio_intenset304_devmiss_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write DEVMISS mwrite None write None rac None reset value false")
    }
    #[doc = "RSSIEND: Write '1' to Enable interrupt for RSSIEND event<br>"]
    pub(crate) fn radio_intenset304_rssiend_read(&self) -> MemResult<bool> {
        todo!("read RSSIEND mwrite None write None rac None reset value false")
    }
    #[doc = "RSSIEND: Write '1' to Enable interrupt for RSSIEND event<br>"]
    pub(crate) fn radio_intenset304_rssiend_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RSSIEND mwrite None write None rac None reset value false")
    }
    #[doc = "BCMATCH: Write '1' to Enable interrupt for BCMATCH event<br>"]
    pub(crate) fn radio_intenset304_bcmatch_read(&self) -> MemResult<bool> {
        todo!("read BCMATCH mwrite None write None rac None reset value false")
    }
    #[doc = "BCMATCH: Write '1' to Enable interrupt for BCMATCH event<br>"]
    pub(crate) fn radio_intenset304_bcmatch_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write BCMATCH mwrite None write None rac None reset value false")
    }
    #[doc = "CRCOK: Write '1' to Enable interrupt for CRCOK event<br>"]
    pub(crate) fn radio_intenset304_crcok_read(&self) -> MemResult<bool> {
        todo!("read CRCOK mwrite None write None rac None reset value false")
    }
    #[doc = "CRCOK: Write '1' to Enable interrupt for CRCOK event<br>"]
    pub(crate) fn radio_intenset304_crcok_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CRCOK mwrite None write None rac None reset value false")
    }
    #[doc = "CRCERROR: Write '1' to Enable interrupt for CRCERROR event<br>"]
    pub(crate) fn radio_intenset304_crcerror_read(&self) -> MemResult<bool> {
        todo!("read CRCERROR mwrite None write None rac None reset value false")
    }
    #[doc = "CRCERROR: Write '1' to Enable interrupt for CRCERROR event<br>"]
    pub(crate) fn radio_intenset304_crcerror_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CRCERROR mwrite None write None rac None reset value false"
        )
    }
    #[doc = "READY: Write '1' to Disable interrupt for READY event<br>"]
    pub(crate) fn radio_intenclr308_ready_read(&self) -> MemResult<bool> {
        todo!("read READY mwrite None write None rac None reset value false")
    }
    #[doc = "READY: Write '1' to Disable interrupt for READY event<br>"]
    pub(crate) fn radio_intenclr308_ready_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write READY mwrite None write None rac None reset value false")
    }
    #[doc = "ADDRESS: Write '1' to Disable interrupt for ADDRESS event<br>"]
    pub(crate) fn radio_intenclr308_address_read(&self) -> MemResult<bool> {
        todo!("read ADDRESS mwrite None write None rac None reset value false")
    }
    #[doc = "ADDRESS: Write '1' to Disable interrupt for ADDRESS event<br>"]
    pub(crate) fn radio_intenclr308_address_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ADDRESS mwrite None write None rac None reset value false")
    }
    #[doc = "PAYLOAD: Write '1' to Disable interrupt for PAYLOAD event<br>"]
    pub(crate) fn radio_intenclr308_payload_read(&self) -> MemResult<bool> {
        todo!("read PAYLOAD mwrite None write None rac None reset value false")
    }
    #[doc = "PAYLOAD: Write '1' to Disable interrupt for PAYLOAD event<br>"]
    pub(crate) fn radio_intenclr308_payload_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PAYLOAD mwrite None write None rac None reset value false")
    }
    #[doc = "END: Write '1' to Disable interrupt for END event<br>"]
    pub(crate) fn radio_intenclr308_end_read(&self) -> MemResult<bool> {
        todo!("read END mwrite None write None rac None reset value false")
    }
    #[doc = "END: Write '1' to Disable interrupt for END event<br>"]
    pub(crate) fn radio_intenclr308_end_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write END mwrite None write None rac None reset value false")
    }
    #[doc = "DISABLED: Write '1' to Disable interrupt for DISABLED event<br>"]
    pub(crate) fn radio_intenclr308_disabled_read(&self) -> MemResult<bool> {
        todo!("read DISABLED mwrite None write None rac None reset value false")
    }
    #[doc = "DISABLED: Write '1' to Disable interrupt for DISABLED event<br>"]
    pub(crate) fn radio_intenclr308_disabled_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write DISABLED mwrite None write None rac None reset value false"
        )
    }
    #[doc = "DEVMATCH: Write '1' to Disable interrupt for DEVMATCH event<br>"]
    pub(crate) fn radio_intenclr308_devmatch_read(&self) -> MemResult<bool> {
        todo!("read DEVMATCH mwrite None write None rac None reset value false")
    }
    #[doc = "DEVMATCH: Write '1' to Disable interrupt for DEVMATCH event<br>"]
    pub(crate) fn radio_intenclr308_devmatch_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write DEVMATCH mwrite None write None rac None reset value false"
        )
    }
    #[doc = "DEVMISS: Write '1' to Disable interrupt for DEVMISS event<br>"]
    pub(crate) fn radio_intenclr308_devmiss_read(&self) -> MemResult<bool> {
        todo!("read DEVMISS mwrite None write None rac None reset value false")
    }
    #[doc = "DEVMISS: Write '1' to Disable interrupt for DEVMISS event<br>"]
    pub(crate) fn radio_intenclr308_devmiss_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write DEVMISS mwrite None write None rac None reset value false")
    }
    #[doc = "RSSIEND: Write '1' to Disable interrupt for RSSIEND event<br>"]
    pub(crate) fn radio_intenclr308_rssiend_read(&self) -> MemResult<bool> {
        todo!("read RSSIEND mwrite None write None rac None reset value false")
    }
    #[doc = "RSSIEND: Write '1' to Disable interrupt for RSSIEND event<br>"]
    pub(crate) fn radio_intenclr308_rssiend_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RSSIEND mwrite None write None rac None reset value false")
    }
    #[doc = "BCMATCH: Write '1' to Disable interrupt for BCMATCH event<br>"]
    pub(crate) fn radio_intenclr308_bcmatch_read(&self) -> MemResult<bool> {
        todo!("read BCMATCH mwrite None write None rac None reset value false")
    }
    #[doc = "BCMATCH: Write '1' to Disable interrupt for BCMATCH event<br>"]
    pub(crate) fn radio_intenclr308_bcmatch_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write BCMATCH mwrite None write None rac None reset value false")
    }
    #[doc = "CRCOK: Write '1' to Disable interrupt for CRCOK event<br>"]
    pub(crate) fn radio_intenclr308_crcok_read(&self) -> MemResult<bool> {
        todo!("read CRCOK mwrite None write None rac None reset value false")
    }
    #[doc = "CRCOK: Write '1' to Disable interrupt for CRCOK event<br>"]
    pub(crate) fn radio_intenclr308_crcok_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CRCOK mwrite None write None rac None reset value false")
    }
    #[doc = "CRCERROR: Write '1' to Disable interrupt for CRCERROR event<br>"]
    pub(crate) fn radio_intenclr308_crcerror_read(&self) -> MemResult<bool> {
        todo!("read CRCERROR mwrite None write None rac None reset value false")
    }
    #[doc = "CRCERROR: Write '1' to Disable interrupt for CRCERROR event<br>"]
    pub(crate) fn radio_intenclr308_crcerror_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write CRCERROR mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CRCSTATUS: CRC status of packet received<br>"]
    pub(crate) fn radio_crcstatus400_crcstatus_read(&self) -> MemResult<bool> {
        todo!(
            "read CRCSTATUS mwrite None write None rac None reset value false"
        )
    }
    #[doc = "RXMATCH: Received address<br>"]
    pub(crate) fn radio_rxmatch408_rxmatch_read(&self) -> MemResult<u8> {
        todo ! ("read RXMATCH mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "RXCRC: CRC field of previously received packet<br>"]
    pub(crate) fn radio_rxcrc40c_rxcrc_read(&self) -> MemResult<u32> {
        todo ! ("read RXCRC mwrite None write None rac None reset value 0x00 mask 0xffffff")
    }
    #[doc = "DAI: Device address match index<br>"]
    pub(crate) fn radio_dai410_dai_read(&self) -> MemResult<u8> {
        todo ! ("read DAI mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "PACKETPTR: Packet pointer<br>"]
    pub(crate) fn radio_packetptr504_packetptr_read(&self) -> MemResult<u32> {
        todo ! ("read PACKETPTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "PACKETPTR: Packet pointer<br>"]
    pub(crate) fn radio_packetptr504_packetptr_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write PACKETPTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "FREQUENCY: Radio channel frequency<br>"]
    pub(crate) fn radio_frequency508_frequency_read(&self) -> MemResult<u8> {
        todo ! ("read FREQUENCY mwrite None write None rac None reset value 0x02 mask 0x7f")
    }
    #[doc = "FREQUENCY: Radio channel frequency<br>"]
    pub(crate) fn radio_frequency508_frequency_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write FREQUENCY mwrite None write None rac None reset value 0x02 mask 0x7f")
    }
    #[doc = "MAP: Channel map selection.<br>"]
    pub(crate) fn radio_frequency508_map_read(&self) -> MemResult<bool> {
        todo!("read MAP mwrite None write None rac None reset value false")
    }
    #[doc = "MAP: Channel map selection.<br>"]
    pub(crate) fn radio_frequency508_map_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MAP mwrite None write None rac None reset value false")
    }
    #[doc = "TXPOWER: RADIO output power.<br>"]
    pub(crate) fn radio_txpower50c_txpower_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E12RadioTxpower50cTxpower> {
        todo ! ("read TXPOWER mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "TXPOWER: RADIO output power.<br>"]
    pub(crate) fn radio_txpower50c_txpower_write(
        &mut self,
        _value: crate::peripheral::enums::E12RadioTxpower50cTxpower,
    ) -> MemResult<()> {
        todo ! ("write TXPOWER mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "MODE: Radio data rate and modulation setting. The radio supports Frequency-shift Keying (FSK) modulation.<br>"]
    pub(crate) fn radio_mode510_mode_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E13RadioMode510Mode> {
        todo ! ("read MODE mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "MODE: Radio data rate and modulation setting. The radio supports Frequency-shift Keying (FSK) modulation.<br>"]
    pub(crate) fn radio_mode510_mode_write(
        &mut self,
        _value: crate::peripheral::enums::E13RadioMode510Mode,
    ) -> MemResult<()> {
        todo ! ("write MODE mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "LFLEN: Length on air of LENGTH field in number of bits.<br>"]
    pub(crate) fn radio_pcnf0514_lflen_read(&self) -> MemResult<u8> {
        todo ! ("read LFLEN mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "LFLEN: Length on air of LENGTH field in number of bits.<br>"]
    pub(crate) fn radio_pcnf0514_lflen_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write LFLEN mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "S0LEN: Length on air of S0 field in number of bytes.<br>"]
    pub(crate) fn radio_pcnf0514_s0len_read(&self) -> MemResult<bool> {
        todo!("read S0LEN mwrite None write None rac None reset value false")
    }
    #[doc = "S0LEN: Length on air of S0 field in number of bytes.<br>"]
    pub(crate) fn radio_pcnf0514_s0len_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write S0LEN mwrite None write None rac None reset value false")
    }
    #[doc = "S1LEN: Length on air of S1 field in number of bits.<br>"]
    pub(crate) fn radio_pcnf0514_s1len_read(&self) -> MemResult<u8> {
        todo ! ("read S1LEN mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "S1LEN: Length on air of S1 field in number of bits.<br>"]
    pub(crate) fn radio_pcnf0514_s1len_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write S1LEN mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "S1INCL: Include or exclude S1 field in RAM<br>"]
    pub(crate) fn radio_pcnf0514_s1incl_read(&self) -> MemResult<bool> {
        todo!("read S1INCL mwrite None write None rac None reset value false")
    }
    #[doc = "S1INCL: Include or exclude S1 field in RAM<br>"]
    pub(crate) fn radio_pcnf0514_s1incl_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write S1INCL mwrite None write None rac None reset value false")
    }
    #[doc = "PLEN: Length of preamble on air. Decision point: TASKS_START task<br>"]
    pub(crate) fn radio_pcnf0514_plen_read(&self) -> MemResult<bool> {
        todo!("read PLEN mwrite None write None rac None reset value false")
    }
    #[doc = "PLEN: Length of preamble on air. Decision point: TASKS_START task<br>"]
    pub(crate) fn radio_pcnf0514_plen_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PLEN mwrite None write None rac None reset value false")
    }
    #[doc = "MAXLEN: Maximum length of packet payload. If the packet payload is larger than MAXLEN, the radio will truncate the payload to MAXLEN.<br>"]
    pub(crate) fn radio_pcnf1518_maxlen_read(&self) -> MemResult<u8> {
        todo ! ("read MAXLEN mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "MAXLEN: Maximum length of packet payload. If the packet payload is larger than MAXLEN, the radio will truncate the payload to MAXLEN.<br>"]
    pub(crate) fn radio_pcnf1518_maxlen_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write MAXLEN mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "STATLEN: Static length in number of bytes<br>"]
    pub(crate) fn radio_pcnf1518_statlen_read(&self) -> MemResult<u8> {
        todo ! ("read STATLEN mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "STATLEN: Static length in number of bytes<br>"]
    pub(crate) fn radio_pcnf1518_statlen_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write STATLEN mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "BALEN: Base address length in number of bytes<br>"]
    pub(crate) fn radio_pcnf1518_balen_read(&self) -> MemResult<u8> {
        todo ! ("read BALEN mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "BALEN: Base address length in number of bytes<br>"]
    pub(crate) fn radio_pcnf1518_balen_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write BALEN mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "ENDIAN: On air endianness of packet, this applies to the S0, LENGTH, S1 and the PAYLOAD fields.<br>"]
    pub(crate) fn radio_pcnf1518_endian_read(&self) -> MemResult<bool> {
        todo!("read ENDIAN mwrite None write None rac None reset value false")
    }
    #[doc = "ENDIAN: On air endianness of packet, this applies to the S0, LENGTH, S1 and the PAYLOAD fields.<br>"]
    pub(crate) fn radio_pcnf1518_endian_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ENDIAN mwrite None write None rac None reset value false")
    }
    #[doc = "WHITEEN: Enable or disable packet whitening<br>"]
    pub(crate) fn radio_pcnf1518_whiteen_read(&self) -> MemResult<bool> {
        todo!("read WHITEEN mwrite None write None rac None reset value false")
    }
    #[doc = "WHITEEN: Enable or disable packet whitening<br>"]
    pub(crate) fn radio_pcnf1518_whiteen_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write WHITEEN mwrite None write None rac None reset value false")
    }
    #[doc = "BASE0: Base address 0<br>"]
    pub(crate) fn radio_base051c_base0_read(&self) -> MemResult<u32> {
        todo ! ("read BASE0 mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "BASE0: Base address 0<br>"]
    pub(crate) fn radio_base051c_base0_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write BASE0 mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "BASE1: Base address 1<br>"]
    pub(crate) fn radio_base1520_base1_read(&self) -> MemResult<u32> {
        todo ! ("read BASE1 mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "BASE1: Base address 1<br>"]
    pub(crate) fn radio_base1520_base1_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write BASE1 mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "AP0: Address prefix 0.<br>"]
    pub(crate) fn radio_prefix0524_ap0_read(&self) -> MemResult<u8> {
        Ok(self.address_prefix[0])
    }
    #[doc = "AP0: Address prefix 0.<br>"]
    pub(crate) fn radio_prefix0524_ap0_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.address_prefix[0] = _value)
    }
    #[doc = "AP1: Address prefix 1.<br>"]
    pub(crate) fn radio_prefix0524_ap1_read(&self) -> MemResult<u8> {
        Ok(self.address_prefix[1])
    }
    #[doc = "AP1: Address prefix 1.<br>"]
    pub(crate) fn radio_prefix0524_ap1_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write AP1 mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "AP2: Address prefix 2.<br>"]
    pub(crate) fn radio_prefix0524_ap2_read(&self) -> MemResult<u8> {
        Ok(self.address_prefix[2])
    }
    #[doc = "AP2: Address prefix 2.<br>"]
    pub(crate) fn radio_prefix0524_ap2_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write AP2 mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "AP3: Address prefix 3.<br>"]
    pub(crate) fn radio_prefix0524_ap3_read(&self) -> MemResult<u8> {
        Ok(self.address_prefix[3])
    }
    #[doc = "AP3: Address prefix 3.<br>"]
    pub(crate) fn radio_prefix0524_ap3_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write AP3 mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "AP4: Address prefix 4.<br>"]
    pub(crate) fn radio_prefix1528_ap4_read(&self) -> MemResult<u8> {
        todo ! ("read AP4 mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "AP4: Address prefix 4.<br>"]
    pub(crate) fn radio_prefix1528_ap4_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.address_prefix[4] = _value)
    }
    #[doc = "AP5: Address prefix 5.<br>"]
    pub(crate) fn radio_prefix1528_ap5_read(&self) -> MemResult<u8> {
        todo ! ("read AP5 mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "AP5: Address prefix 5.<br>"]
    pub(crate) fn radio_prefix1528_ap5_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.address_prefix[5] = _value)
    }
    #[doc = "AP6: Address prefix 6.<br>"]
    pub(crate) fn radio_prefix1528_ap6_read(&self) -> MemResult<u8> {
        todo ! ("read AP6 mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "AP6: Address prefix 6.<br>"]
    pub(crate) fn radio_prefix1528_ap6_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.address_prefix[6] = _value)
    }
    #[doc = "AP7: Address prefix 7.<br>"]
    pub(crate) fn radio_prefix1528_ap7_read(&self) -> MemResult<u8> {
        todo ! ("read AP7 mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "AP7: Address prefix 7.<br>"]
    pub(crate) fn radio_prefix1528_ap7_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        Ok(self.address_prefix[7] = _value)
    }
    #[doc = "TXADDRESS: Transmit address select<br>"]
    pub(crate) fn radio_txaddress52c_txaddress_read(&self) -> MemResult<u8> {
        todo ! ("read TXADDRESS mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "TXADDRESS: Transmit address select<br>"]
    pub(crate) fn radio_txaddress52c_txaddress_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write TXADDRESS mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "ADDR0: Enable or disable reception on logical address 0.<br>"]
    pub(crate) fn radio_rxaddresses530_addr0_read(&self) -> MemResult<bool> {
        Ok(self.receive_on_ap[0])
    }
    #[doc = "ADDR0: Enable or disable reception on logical address 0.<br>"]
    pub(crate) fn radio_rxaddresses530_addr0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.receive_on_ap[0] = _value)
    }
    #[doc = "ADDR1: Enable or disable reception on logical address 1.<br>"]
    pub(crate) fn radio_rxaddresses530_addr1_read(&self) -> MemResult<bool> {
        Ok(self.receive_on_ap[1])
    }
    #[doc = "ADDR1: Enable or disable reception on logical address 1.<br>"]
    pub(crate) fn radio_rxaddresses530_addr1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.receive_on_ap[1] = _value)
    }
    #[doc = "ADDR2: Enable or disable reception on logical address 2.<br>"]
    pub(crate) fn radio_rxaddresses530_addr2_read(&self) -> MemResult<bool> {
        Ok(self.receive_on_ap[2])
    }
    #[doc = "ADDR2: Enable or disable reception on logical address 2.<br>"]
    pub(crate) fn radio_rxaddresses530_addr2_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.receive_on_ap[2] = _value)
    }
    #[doc = "ADDR3: Enable or disable reception on logical address 3.<br>"]
    pub(crate) fn radio_rxaddresses530_addr3_read(&self) -> MemResult<bool> {
        Ok(self.receive_on_ap[3])
    }
    #[doc = "ADDR3: Enable or disable reception on logical address 3.<br>"]
    pub(crate) fn radio_rxaddresses530_addr3_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.receive_on_ap[3] = _value)
    }
    #[doc = "ADDR4: Enable or disable reception on logical address 4.<br>"]
    pub(crate) fn radio_rxaddresses530_addr4_read(&self) -> MemResult<bool> {
        Ok(self.receive_on_ap[4])
    }
    #[doc = "ADDR4: Enable or disable reception on logical address 4.<br>"]
    pub(crate) fn radio_rxaddresses530_addr4_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.receive_on_ap[4] = _value)
    }
    #[doc = "ADDR5: Enable or disable reception on logical address 5.<br>"]
    pub(crate) fn radio_rxaddresses530_addr5_read(&self) -> MemResult<bool> {
        Ok(self.receive_on_ap[5])
    }
    #[doc = "ADDR5: Enable or disable reception on logical address 5.<br>"]
    pub(crate) fn radio_rxaddresses530_addr5_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.receive_on_ap[5] = _value)
    }
    #[doc = "ADDR6: Enable or disable reception on logical address 6.<br>"]
    pub(crate) fn radio_rxaddresses530_addr6_read(&self) -> MemResult<bool> {
        Ok(self.receive_on_ap[6])
    }
    #[doc = "ADDR6: Enable or disable reception on logical address 6.<br>"]
    pub(crate) fn radio_rxaddresses530_addr6_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.receive_on_ap[6] = _value)
    }
    #[doc = "ADDR7: Enable or disable reception on logical address 7.<br>"]
    pub(crate) fn radio_rxaddresses530_addr7_read(&self) -> MemResult<bool> {
        Ok(self.receive_on_ap[7])
    }
    #[doc = "ADDR7: Enable or disable reception on logical address 7.<br>"]
    pub(crate) fn radio_rxaddresses530_addr7_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.receive_on_ap[7] = _value)
    }
    #[doc = "LEN: CRC length in number of bytes.<br>"]
    pub(crate) fn radio_crccnf534_len_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E14RadioCrccnf534Len> {
        todo ! ("read LEN mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "LEN: CRC length in number of bytes.<br>"]
    pub(crate) fn radio_crccnf534_len_write(
        &mut self,
        _value: crate::peripheral::enums::E14RadioCrccnf534Len,
    ) -> MemResult<()> {
        todo ! ("write LEN mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "SKIPADDR: Include or exclude packet address field out of CRC calculation.<br>"]
    pub(crate) fn radio_crccnf534_skipaddr_read(&self) -> MemResult<bool> {
        todo!("read SKIPADDR mwrite None write None rac None reset value false")
    }
    #[doc = "SKIPADDR: Include or exclude packet address field out of CRC calculation.<br>"]
    pub(crate) fn radio_crccnf534_skipaddr_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write SKIPADDR mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CRCPOLY: CRC polynomial<br>"]
    pub(crate) fn radio_crcpoly538_crcpoly_read(&self) -> MemResult<u32> {
        todo ! ("read CRCPOLY mwrite None write None rac None reset value 0x00 mask 0xffffff")
    }
    #[doc = "CRCPOLY: CRC polynomial<br>"]
    pub(crate) fn radio_crcpoly538_crcpoly_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write CRCPOLY mwrite None write None rac None reset value 0x00 mask 0xffffff")
    }
    #[doc = "CRCINIT: CRC initial value<br>"]
    pub(crate) fn radio_crcinit53c_crcinit_read(&self) -> MemResult<u32> {
        todo ! ("read CRCINIT mwrite None write None rac None reset value 0x00 mask 0xffffff")
    }
    #[doc = "CRCINIT: CRC initial value<br>"]
    pub(crate) fn radio_crcinit53c_crcinit_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write CRCINIT mwrite None write None rac None reset value 0x00 mask 0xffffff")
    }
    #[doc = "UNUSED0: Unspecified<br>"]
    pub(crate) fn radio_unused0540_read(&self) -> MemResult<u32> {
        todo ! ("read radio_unused0540 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "UNUSED0: Unspecified<br>"]
    pub(crate) fn radio_unused0540_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write radio_unused0540 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TIFS: Inter Frame Spacing in us<br>"]
    pub(crate) fn radio_tifs544_tifs_read(&self) -> MemResult<u8> {
        todo ! ("read TIFS mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "TIFS: Inter Frame Spacing in us<br>"]
    pub(crate) fn radio_tifs544_tifs_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write TIFS mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "RSSISAMPLE: RSSI sample<br>"]
    pub(crate) fn radio_rssisample548_rssisample_read(&self) -> MemResult<u8> {
        todo ! ("read RSSISAMPLE mwrite None write None rac None reset value 0x00 mask 0x7f")
    }
    #[doc = "STATE: Current radio state<br>"]
    pub(crate) fn radio_state550_state_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E15RadioState550State> {
        todo ! ("read STATE mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "DATAWHITEIV: Data whitening initial value. Bit 6 is hard-wired to '1', writing '0' to it has no effect, and it will always be read back and used by the device as '1'.<br>"]
    pub(crate) fn radio_datawhiteiv554_datawhiteiv_read(
        &self,
    ) -> MemResult<u8> {
        todo ! ("read DATAWHITEIV mwrite None write None rac None reset value 0x40 mask 0x7f")
    }
    #[doc = "DATAWHITEIV: Data whitening initial value. Bit 6 is hard-wired to '1', writing '0' to it has no effect, and it will always be read back and used by the device as '1'.<br>"]
    pub(crate) fn radio_datawhiteiv554_datawhiteiv_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write DATAWHITEIV mwrite None write None rac None reset value 0x40 mask 0x7f")
    }
    #[doc = "BCC: Bit counter compare<br>"]
    pub(crate) fn radio_bcc560_bcc_read(&self) -> MemResult<u32> {
        todo ! ("read BCC mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "BCC: Bit counter compare<br>"]
    pub(crate) fn radio_bcc560_bcc_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write BCC mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "DAB: Device address base segment 0<br>"]
    pub(crate) fn radio_dabn600_dab_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<u32> {
        todo ! ("read DAB mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "DAB: Device address base segment 0<br>"]
    pub(crate) fn radio_dabn600_dab_write(
        &mut self,
        _reg_array: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write DAB mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "DAP: Device address prefix 0<br>"]
    pub(crate) fn radio_dapn620_dap_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<u16> {
        todo ! ("read DAP mwrite None write None rac None reset value 0x00 mask 0xffff")
    }
    #[doc = "DAP: Device address prefix 0<br>"]
    pub(crate) fn radio_dapn620_dap_write(
        &mut self,
        _reg_array: usize,
        _value: u16,
    ) -> MemResult<()> {
        todo ! ("write DAP mwrite None write None rac None reset value 0x00 mask 0xffff")
    }
    #[doc = "ENA0: Enable or disable device address matching using device address 0<br>"]
    pub(crate) fn radio_dacnf640_ena0_read(&self) -> MemResult<bool> {
        Ok(self.address_prefix_on[0])
    }
    #[doc = "ENA0: Enable or disable device address matching using device address 0<br>"]
    pub(crate) fn radio_dacnf640_ena0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.address_prefix_on[0] = _value)
    }
    #[doc = "ENA1: Enable or disable device address matching using device address 1<br>"]
    pub(crate) fn radio_dacnf640_ena1_read(&self) -> MemResult<bool> {
        Ok(self.address_prefix_on[1])
    }
    #[doc = "ENA1: Enable or disable device address matching using device address 1<br>"]
    pub(crate) fn radio_dacnf640_ena1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.address_prefix_on[1] = _value)
    }
    #[doc = "ENA2: Enable or disable device address matching using device address 2<br>"]
    pub(crate) fn radio_dacnf640_ena2_read(&self) -> MemResult<bool> {
        Ok(self.address_prefix_on[2])
    }
    #[doc = "ENA2: Enable or disable device address matching using device address 2<br>"]
    pub(crate) fn radio_dacnf640_ena2_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.address_prefix_on[2] = _value)
    }
    #[doc = "ENA3: Enable or disable device address matching using device address 3<br>"]
    pub(crate) fn radio_dacnf640_ena3_read(&self) -> MemResult<bool> {
        Ok(self.address_prefix_on[3])
    }
    #[doc = "ENA3: Enable or disable device address matching using device address 3<br>"]
    pub(crate) fn radio_dacnf640_ena3_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.address_prefix_on[3] = _value)
    }
    #[doc = "ENA4: Enable or disable device address matching using device address 4<br>"]
    pub(crate) fn radio_dacnf640_ena4_read(&self) -> MemResult<bool> {
        Ok(self.address_prefix_on[4])
    }
    #[doc = "ENA4: Enable or disable device address matching using device address 4<br>"]
    pub(crate) fn radio_dacnf640_ena4_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.address_prefix_on[4] = _value)
    }
    #[doc = "ENA5: Enable or disable device address matching using device address 5<br>"]
    pub(crate) fn radio_dacnf640_ena5_read(&self) -> MemResult<bool> {
        Ok(self.address_prefix_on[5])
    }
    #[doc = "ENA5: Enable or disable device address matching using device address 5<br>"]
    pub(crate) fn radio_dacnf640_ena5_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.address_prefix_on[5] = _value)
    }
    #[doc = "ENA6: Enable or disable device address matching using device address 6<br>"]
    pub(crate) fn radio_dacnf640_ena6_read(&self) -> MemResult<bool> {
        Ok(self.address_prefix_on[6])
    }
    #[doc = "ENA6: Enable or disable device address matching using device address 6<br>"]
    pub(crate) fn radio_dacnf640_ena6_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.address_prefix_on[6] = _value)
    }
    #[doc = "ENA7: Enable or disable device address matching using device address 7<br>"]
    pub(crate) fn radio_dacnf640_ena7_read(&self) -> MemResult<bool> {
        Ok(self.address_prefix_on[7])
    }
    #[doc = "ENA7: Enable or disable device address matching using device address 7<br>"]
    pub(crate) fn radio_dacnf640_ena7_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.address_prefix_on[7] = _value)
    }
    #[doc = "TXADD0: TxAdd for device address 0<br>"]
    pub(crate) fn radio_dacnf640_txadd0_read(&self) -> MemResult<bool> {
        Ok(self.tx_on_ap[0])
    }
    #[doc = "TXADD0: TxAdd for device address 0<br>"]
    pub(crate) fn radio_dacnf640_txadd0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.tx_on_ap[0] = _value)
    }
    #[doc = "TXADD1: TxAdd for device address 1<br>"]
    pub(crate) fn radio_dacnf640_txadd1_read(&self) -> MemResult<bool> {
        Ok(self.tx_on_ap[1])
    }
    #[doc = "TXADD1: TxAdd for device address 1<br>"]
    pub(crate) fn radio_dacnf640_txadd1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.tx_on_ap[1] = _value)
    }
    #[doc = "TXADD2: TxAdd for device address 2<br>"]
    pub(crate) fn radio_dacnf640_txadd2_read(&self) -> MemResult<bool> {
        Ok(self.tx_on_ap[2])
    }
    #[doc = "TXADD2: TxAdd for device address 2<br>"]
    pub(crate) fn radio_dacnf640_txadd2_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.tx_on_ap[2] = _value)
    }
    #[doc = "TXADD3: TxAdd for device address 3<br>"]
    pub(crate) fn radio_dacnf640_txadd3_read(&self) -> MemResult<bool> {
        Ok(self.tx_on_ap[3])
    }
    #[doc = "TXADD3: TxAdd for device address 3<br>"]
    pub(crate) fn radio_dacnf640_txadd3_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.tx_on_ap[3] = _value)
    }
    #[doc = "TXADD4: TxAdd for device address 4<br>"]
    pub(crate) fn radio_dacnf640_txadd4_read(&self) -> MemResult<bool> {
        Ok(self.tx_on_ap[4])
    }
    #[doc = "TXADD4: TxAdd for device address 4<br>"]
    pub(crate) fn radio_dacnf640_txadd4_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.tx_on_ap[4] = _value)
    }
    #[doc = "TXADD5: TxAdd for device address 5<br>"]
    pub(crate) fn radio_dacnf640_txadd5_read(&self) -> MemResult<bool> {
        Ok(self.tx_on_ap[5])
    }
    #[doc = "TXADD5: TxAdd for device address 5<br>"]
    pub(crate) fn radio_dacnf640_txadd5_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.tx_on_ap[5] = _value)
    }
    #[doc = "TXADD6: TxAdd for device address 6<br>"]
    pub(crate) fn radio_dacnf640_txadd6_read(&self) -> MemResult<bool> {
        Ok(self.tx_on_ap[6])
    }
    #[doc = "TXADD6: TxAdd for device address 6<br>"]
    pub(crate) fn radio_dacnf640_txadd6_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.tx_on_ap[6] = _value)
    }
    #[doc = "TXADD7: TxAdd for device address 7<br>"]
    pub(crate) fn radio_dacnf640_txadd7_read(&self) -> MemResult<bool> {
        Ok(self.tx_on_ap[7])
    }
    #[doc = "TXADD7: TxAdd for device address 7<br>"]
    pub(crate) fn radio_dacnf640_txadd7_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        Ok(self.tx_on_ap[7] = _value)
    }
    #[doc = "RU: Radio ramp-up time<br>"]
    pub(crate) fn radio_modecnf0650_ru_read(&self) -> MemResult<bool> {
        todo!("read RU mwrite None write None rac None reset value false")
    }
    #[doc = "RU: Radio ramp-up time<br>"]
    pub(crate) fn radio_modecnf0650_ru_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RU mwrite None write None rac None reset value false")
    }
    #[doc = "DTX: Default TX value<br>"]
    pub(crate) fn radio_modecnf0650_dtx_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E16RadioModecnf0650Dtx> {
        todo ! ("read DTX mwrite None write None rac None reset value 0x02 mask 0x03")
    }
    #[doc = "DTX: Default TX value<br>"]
    pub(crate) fn radio_modecnf0650_dtx_write(
        &mut self,
        _value: crate::peripheral::enums::E16RadioModecnf0650Dtx,
    ) -> MemResult<()> {
        todo ! ("write DTX mwrite None write None rac None reset value 0x02 mask 0x03")
    }
    #[doc = "POWER: Peripheral power control. The peripheral and its registers will be reset to its initial state by switching the peripheral off and then back on again.<br>"]
    pub(crate) fn radio_powerffc_power_read(&self) -> MemResult<bool> {
        todo!("read POWER mwrite None write None rac None reset value true")
    }
    #[doc = "POWER: Peripheral power control. The peripheral and its registers will be reset to its initial state by switching the peripheral off and then back on again.<br>"]
    pub(crate) fn radio_powerffc_power_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write POWER mwrite None write None rac None reset value true")
    }
}
