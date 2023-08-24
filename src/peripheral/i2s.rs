use icicle_vm::cpu::mem::MemResult;
#[derive(Default)]
#[doc = "I2S: Inter-IC Sound<br><br>Instances:<br>0x40025000: I2S<br>"]
pub struct I2s {
    #[doc = "TODO: implement things here"]
    _todo: (),
}
impl I2s {
    pub(crate) fn page_to_index(page: u64) -> usize {
        match page {
            262181u64 => 0usize,
            _ => unreachable!(),
        }
    }
    #[doc = "TASKS_START: Starts continuous I2S transfer. Also starts MCK generator when this is enabled.<br>"]
    pub(crate) fn i2s_tasks_start0_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write i2s_tasks_start0 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_STOP: Stops I2S transfer. Also stops MCK generator. Triggering this task will cause the {event:STOPPED} event to be generated.<br>"]
    pub(crate) fn i2s_tasks_stop4_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write i2s_tasks_stop4 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_RXPTRUPD: The RXD.PTR register has been copied to internal double-buffers. When the I2S module is started and RX is enabled, this event will be generated for every RXTXD.MAXCNT words that are received on the SDIN pin.<br>"]
    pub(crate) fn i2s_events_rxptrupd104_read(&self) -> MemResult<u32> {
        todo ! ("read i2s_events_rxptrupd104 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_RXPTRUPD: The RXD.PTR register has been copied to internal double-buffers. When the I2S module is started and RX is enabled, this event will be generated for every RXTXD.MAXCNT words that are received on the SDIN pin.<br>"]
    pub(crate) fn i2s_events_rxptrupd104_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write i2s_events_rxptrupd104 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_STOPPED: I2S transfer stopped.<br>"]
    pub(crate) fn i2s_events_stopped108_read(&self) -> MemResult<u32> {
        todo ! ("read i2s_events_stopped108 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_STOPPED: I2S transfer stopped.<br>"]
    pub(crate) fn i2s_events_stopped108_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write i2s_events_stopped108 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_TXPTRUPD: The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin.<br>"]
    pub(crate) fn i2s_events_txptrupd114_read(&self) -> MemResult<u32> {
        todo ! ("read i2s_events_txptrupd114 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_TXPTRUPD: The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin.<br>"]
    pub(crate) fn i2s_events_txptrupd114_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write i2s_events_txptrupd114 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "RXPTRUPD: Enable or disable interrupt for RXPTRUPD event<br>"]
    pub(crate) fn i2s_inten300_rxptrupd_read(&self) -> MemResult<bool> {
        todo!("read RXPTRUPD mwrite None write None rac None reset value false")
    }
    #[doc = "RXPTRUPD: Enable or disable interrupt for RXPTRUPD event<br>"]
    pub(crate) fn i2s_inten300_rxptrupd_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write RXPTRUPD mwrite None write None rac None reset value false"
        )
    }
    #[doc = "STOPPED: Enable or disable interrupt for STOPPED event<br>"]
    pub(crate) fn i2s_inten300_stopped_read(&self) -> MemResult<bool> {
        todo!("read STOPPED mwrite None write None rac None reset value false")
    }
    #[doc = "STOPPED: Enable or disable interrupt for STOPPED event<br>"]
    pub(crate) fn i2s_inten300_stopped_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write STOPPED mwrite None write None rac None reset value false")
    }
    #[doc = "TXPTRUPD: Enable or disable interrupt for TXPTRUPD event<br>"]
    pub(crate) fn i2s_inten300_txptrupd_read(&self) -> MemResult<bool> {
        todo!("read TXPTRUPD mwrite None write None rac None reset value false")
    }
    #[doc = "TXPTRUPD: Enable or disable interrupt for TXPTRUPD event<br>"]
    pub(crate) fn i2s_inten300_txptrupd_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write TXPTRUPD mwrite None write None rac None reset value false"
        )
    }
    #[doc = "RXPTRUPD: Write '1' to Enable interrupt for RXPTRUPD event<br>"]
    pub(crate) fn i2s_intenset304_rxptrupd_read(&self) -> MemResult<bool> {
        todo!("read RXPTRUPD mwrite None write None rac None reset value false")
    }
    #[doc = "RXPTRUPD: Write '1' to Enable interrupt for RXPTRUPD event<br>"]
    pub(crate) fn i2s_intenset304_rxptrupd_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write RXPTRUPD mwrite None write None rac None reset value false"
        )
    }
    #[doc = "STOPPED: Write '1' to Enable interrupt for STOPPED event<br>"]
    pub(crate) fn i2s_intenset304_stopped_read(&self) -> MemResult<bool> {
        todo!("read STOPPED mwrite None write None rac None reset value false")
    }
    #[doc = "STOPPED: Write '1' to Enable interrupt for STOPPED event<br>"]
    pub(crate) fn i2s_intenset304_stopped_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write STOPPED mwrite None write None rac None reset value false")
    }
    #[doc = "TXPTRUPD: Write '1' to Enable interrupt for TXPTRUPD event<br>"]
    pub(crate) fn i2s_intenset304_txptrupd_read(&self) -> MemResult<bool> {
        todo!("read TXPTRUPD mwrite None write None rac None reset value false")
    }
    #[doc = "TXPTRUPD: Write '1' to Enable interrupt for TXPTRUPD event<br>"]
    pub(crate) fn i2s_intenset304_txptrupd_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write TXPTRUPD mwrite None write None rac None reset value false"
        )
    }
    #[doc = "RXPTRUPD: Write '1' to Disable interrupt for RXPTRUPD event<br>"]
    pub(crate) fn i2s_intenclr308_rxptrupd_read(&self) -> MemResult<bool> {
        todo!("read RXPTRUPD mwrite None write None rac None reset value false")
    }
    #[doc = "RXPTRUPD: Write '1' to Disable interrupt for RXPTRUPD event<br>"]
    pub(crate) fn i2s_intenclr308_rxptrupd_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write RXPTRUPD mwrite None write None rac None reset value false"
        )
    }
    #[doc = "STOPPED: Write '1' to Disable interrupt for STOPPED event<br>"]
    pub(crate) fn i2s_intenclr308_stopped_read(&self) -> MemResult<bool> {
        todo!("read STOPPED mwrite None write None rac None reset value false")
    }
    #[doc = "STOPPED: Write '1' to Disable interrupt for STOPPED event<br>"]
    pub(crate) fn i2s_intenclr308_stopped_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write STOPPED mwrite None write None rac None reset value false")
    }
    #[doc = "TXPTRUPD: Write '1' to Disable interrupt for TXPTRUPD event<br>"]
    pub(crate) fn i2s_intenclr308_txptrupd_read(&self) -> MemResult<bool> {
        todo!("read TXPTRUPD mwrite None write None rac None reset value false")
    }
    #[doc = "TXPTRUPD: Write '1' to Disable interrupt for TXPTRUPD event<br>"]
    pub(crate) fn i2s_intenclr308_txptrupd_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write TXPTRUPD mwrite None write None rac None reset value false"
        )
    }
    #[doc = "ENABLE: Enable I2S module.<br>"]
    pub(crate) fn i2s_enable500_enable_read(&self) -> MemResult<bool> {
        todo!("read ENABLE mwrite None write None rac None reset value false")
    }
    #[doc = "ENABLE: Enable I2S module.<br>"]
    pub(crate) fn i2s_enable500_enable_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ENABLE mwrite None write None rac None reset value false")
    }
    #[doc = "MODE: I2S mode.<br>"]
    pub(crate) fn i2s_config_mode0_mode_read(&self) -> MemResult<bool> {
        todo!("read MODE mwrite None write None rac None reset value false")
    }
    #[doc = "MODE: I2S mode.<br>"]
    pub(crate) fn i2s_config_mode0_mode_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MODE mwrite None write None rac None reset value false")
    }
    #[doc = "RXEN: Reception (RX) enable.<br>"]
    pub(crate) fn i2s_config_rxen4_rxen_read(&self) -> MemResult<bool> {
        todo!("read RXEN mwrite None write None rac None reset value false")
    }
    #[doc = "RXEN: Reception (RX) enable.<br>"]
    pub(crate) fn i2s_config_rxen4_rxen_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RXEN mwrite None write None rac None reset value false")
    }
    #[doc = "TXEN: Transmission (TX) enable.<br>"]
    pub(crate) fn i2s_config_txen8_txen_read(&self) -> MemResult<bool> {
        todo!("read TXEN mwrite None write None rac None reset value true")
    }
    #[doc = "TXEN: Transmission (TX) enable.<br>"]
    pub(crate) fn i2s_config_txen8_txen_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write TXEN mwrite None write None rac None reset value true")
    }
    #[doc = "MCKEN: Master clock generator enable.<br>"]
    pub(crate) fn i2s_config_mckenc_mcken_read(&self) -> MemResult<bool> {
        todo!("read MCKEN mwrite None write None rac None reset value true")
    }
    #[doc = "MCKEN: Master clock generator enable.<br>"]
    pub(crate) fn i2s_config_mckenc_mcken_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MCKEN mwrite None write None rac None reset value true")
    }
    #[doc = "MCKFREQ: Master clock generator frequency.<br>"]
    pub(crate) fn i2s_config_mckfreq10_mckfreq_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E61I2sConfigMckfreq10Mckfreq> {
        todo ! ("read MCKFREQ mwrite None write None rac None reset value 0x20000000 mask 0xffffffff")
    }
    #[doc = "MCKFREQ: Master clock generator frequency.<br>"]
    pub(crate) fn i2s_config_mckfreq10_mckfreq_write(
        &mut self,
        _value: crate::peripheral::enums::E61I2sConfigMckfreq10Mckfreq,
    ) -> MemResult<()> {
        todo ! ("write MCKFREQ mwrite None write None rac None reset value 0x20000000 mask 0xffffffff")
    }
    #[doc = "RATIO: MCK / LRCK ratio.<br>"]
    pub(crate) fn i2s_config_ratio14_ratio_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E62I2sConfigRatio14Ratio> {
        todo ! ("read RATIO mwrite None write None rac None reset value 0x06 mask 0x0f")
    }
    #[doc = "RATIO: MCK / LRCK ratio.<br>"]
    pub(crate) fn i2s_config_ratio14_ratio_write(
        &mut self,
        _value: crate::peripheral::enums::E62I2sConfigRatio14Ratio,
    ) -> MemResult<()> {
        todo ! ("write RATIO mwrite None write None rac None reset value 0x06 mask 0x0f")
    }
    #[doc = "SWIDTH: Sample width.<br>"]
    pub(crate) fn i2s_config_swidth18_swidth_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E63I2sConfigSwidth18Swidth> {
        todo ! ("read SWIDTH mwrite None write None rac None reset value 0x01 mask 0x03")
    }
    #[doc = "SWIDTH: Sample width.<br>"]
    pub(crate) fn i2s_config_swidth18_swidth_write(
        &mut self,
        _value: crate::peripheral::enums::E63I2sConfigSwidth18Swidth,
    ) -> MemResult<()> {
        todo ! ("write SWIDTH mwrite None write None rac None reset value 0x01 mask 0x03")
    }
    #[doc = "ALIGN: Alignment of sample within a frame.<br>"]
    pub(crate) fn i2s_config_align1c_align_read(&self) -> MemResult<bool> {
        todo!("read ALIGN mwrite None write None rac None reset value false")
    }
    #[doc = "ALIGN: Alignment of sample within a frame.<br>"]
    pub(crate) fn i2s_config_align1c_align_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ALIGN mwrite None write None rac None reset value false")
    }
    #[doc = "FORMAT: Frame format.<br>"]
    pub(crate) fn i2s_config_format20_format_read(&self) -> MemResult<bool> {
        todo!("read FORMAT mwrite None write None rac None reset value false")
    }
    #[doc = "FORMAT: Frame format.<br>"]
    pub(crate) fn i2s_config_format20_format_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write FORMAT mwrite None write None rac None reset value false")
    }
    #[doc = "CHANNELS: Enable channels.<br>"]
    pub(crate) fn i2s_config_channels24_channels_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E64I2sConfigChannels24Channels>
    {
        todo ! ("read CHANNELS mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "CHANNELS: Enable channels.<br>"]
    pub(crate) fn i2s_config_channels24_channels_write(
        &mut self,
        _value: crate::peripheral::enums::E64I2sConfigChannels24Channels,
    ) -> MemResult<()> {
        todo ! ("write CHANNELS mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "PTR: Receive buffer Data RAM start address. When receiving, words containing samples will be written to this address. This address is a word aligned Data RAM address.<br>"]
    pub(crate) fn i2s_rxd_ptr0_ptr_read(&self) -> MemResult<u32> {
        todo ! ("read PTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "PTR: Receive buffer Data RAM start address. When receiving, words containing samples will be written to this address. This address is a word aligned Data RAM address.<br>"]
    pub(crate) fn i2s_rxd_ptr0_ptr_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write PTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "PTR: Transmit buffer Data RAM start address. When transmitting, words containing samples will be fetched from this address. This address is a word aligned Data RAM address.<br>"]
    pub(crate) fn i2s_txd_ptr0_ptr_read(&self) -> MemResult<u32> {
        todo ! ("read PTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "PTR: Transmit buffer Data RAM start address. When transmitting, words containing samples will be fetched from this address. This address is a word aligned Data RAM address.<br>"]
    pub(crate) fn i2s_txd_ptr0_ptr_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write PTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "MAXCNT: Size of RXD and TXD buffers in number of 32 bit words.<br>"]
    pub(crate) fn i2s_rxtxd_maxcnt0_maxcnt_read(&self) -> MemResult<u16> {
        todo ! ("read MAXCNT mwrite None write None rac None reset value 0x00 mask 0x3fff")
    }
    #[doc = "MAXCNT: Size of RXD and TXD buffers in number of 32 bit words.<br>"]
    pub(crate) fn i2s_rxtxd_maxcnt0_maxcnt_write(
        &mut self,
        _value: u16,
    ) -> MemResult<()> {
        todo ! ("write MAXCNT mwrite None write None rac None reset value 0x00 mask 0x3fff")
    }
    #[doc = "PIN: Pin number<br>"]
    pub(crate) fn i2s_psel_mck0_pin_read(&self) -> MemResult<u8> {
        todo ! ("read PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "PIN: Pin number<br>"]
    pub(crate) fn i2s_psel_mck0_pin_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn i2s_psel_mck0_connect_read(&self) -> MemResult<bool> {
        todo!("read CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn i2s_psel_mck0_connect_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "PIN: Pin number<br>"]
    pub(crate) fn i2s_psel_sck4_pin_read(&self) -> MemResult<u8> {
        todo ! ("read PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "PIN: Pin number<br>"]
    pub(crate) fn i2s_psel_sck4_pin_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn i2s_psel_sck4_connect_read(&self) -> MemResult<bool> {
        todo!("read CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn i2s_psel_sck4_connect_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "PIN: Pin number<br>"]
    pub(crate) fn i2s_psel_lrck8_pin_read(&self) -> MemResult<u8> {
        todo ! ("read PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "PIN: Pin number<br>"]
    pub(crate) fn i2s_psel_lrck8_pin_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn i2s_psel_lrck8_connect_read(&self) -> MemResult<bool> {
        todo!("read CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn i2s_psel_lrck8_connect_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "PIN: Pin number<br>"]
    pub(crate) fn i2s_psel_sdinc_pin_read(&self) -> MemResult<u8> {
        todo ! ("read PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "PIN: Pin number<br>"]
    pub(crate) fn i2s_psel_sdinc_pin_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn i2s_psel_sdinc_connect_read(&self) -> MemResult<bool> {
        todo!("read CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn i2s_psel_sdinc_connect_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "PIN: Pin number<br>"]
    pub(crate) fn i2s_psel_sdout10_pin_read(&self) -> MemResult<u8> {
        todo ! ("read PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "PIN: Pin number<br>"]
    pub(crate) fn i2s_psel_sdout10_pin_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn i2s_psel_sdout10_connect_read(&self) -> MemResult<bool> {
        todo!("read CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn i2s_psel_sdout10_connect_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CONNECT mwrite None write None rac None reset value true")
    }
}
