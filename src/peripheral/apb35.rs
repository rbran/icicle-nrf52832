use icicle_vm::cpu::mem::MemResult;
#[derive(Default)]
#[doc = "SPIM0: Serial Peripheral Interface Master with EasyDMA 0<br>SPIS0: SPI Slave 0<br>SPI0: Serial Peripheral Interface 0<br><br>Instances:<br>0x40023000: SPIM2, SPIS2, SPI2<br>"]
pub struct Apb35 {
    #[doc = "TODO: implement things here"]
    _todo: (),
}
impl Apb35 {
    pub(crate) fn page_to_index(page: u64) -> usize {
        match page {
            262179u64 => 0usize,
            _ => unreachable!(),
        }
    }
    #[doc = "TASKS_START: Start SPI transaction<br>"]
    pub(crate) fn apb35_tasks_start10_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb35_tasks_start10 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_STOP: Stop SPI transaction<br>"]
    pub(crate) fn apb35_tasks_stop14_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb35_tasks_stop14 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_SUSPEND: Suspend SPI transaction<br>"]
    pub(crate) fn apb35_tasks_suspend1c_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb35_tasks_suspend1c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_RESUME: Resume SPI transaction<br>"]
    pub(crate) fn apb35_tasks_resume20_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb35_tasks_resume20 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_ACQUIRE: Acquire SPI semaphore<br>"]
    pub(crate) fn apb35_tasks_acquire24_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb35_tasks_acquire24 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_RELEASE: Release SPI semaphore, enabling the SPI slave to acquire it<br>"]
    pub(crate) fn apb35_tasks_release28_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb35_tasks_release28 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_STOPPED: SPI transaction has stopped<br>EVENTS_END: Granted transaction completed<br>"]
    pub(crate) fn apb35_events_stopped104_read(&self) -> MemResult<u32> {
        todo ! ("read apb35_events_stopped104 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_STOPPED: SPI transaction has stopped<br>EVENTS_END: Granted transaction completed<br>"]
    pub(crate) fn apb35_events_stopped104_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb35_events_stopped104 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_READY: TXD byte sent and RXD byte received<br>"]
    pub(crate) fn apb35_events_ready108_read(&self) -> MemResult<u32> {
        todo ! ("read apb35_events_ready108 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_READY: TXD byte sent and RXD byte received<br>"]
    pub(crate) fn apb35_events_ready108_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb35_events_ready108 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ENDRX: End of RXD buffer reached<br>EVENTS_ENDRX: End of RXD buffer reached<br>"]
    pub(crate) fn apb35_events_endrx110_read(&self) -> MemResult<u32> {
        todo ! ("read apb35_events_endrx110 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ENDRX: End of RXD buffer reached<br>EVENTS_ENDRX: End of RXD buffer reached<br>"]
    pub(crate) fn apb35_events_endrx110_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb35_events_endrx110 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_END: End of RXD buffer and TXD buffer reached<br>"]
    pub(crate) fn apb35_events_end118_read(&self) -> MemResult<u32> {
        todo ! ("read apb35_events_end118 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_END: End of RXD buffer and TXD buffer reached<br>"]
    pub(crate) fn apb35_events_end118_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb35_events_end118 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ENDTX: End of TXD buffer reached<br>"]
    pub(crate) fn apb35_events_endtx120_read(&self) -> MemResult<u32> {
        todo ! ("read apb35_events_endtx120 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ENDTX: End of TXD buffer reached<br>"]
    pub(crate) fn apb35_events_endtx120_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb35_events_endtx120 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ACQUIRED: Semaphore acquired<br>"]
    pub(crate) fn apb35_events_acquired128_read(&self) -> MemResult<u32> {
        todo ! ("read apb35_events_acquired128 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ACQUIRED: Semaphore acquired<br>"]
    pub(crate) fn apb35_events_acquired128_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb35_events_acquired128 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_STARTED: Transaction started<br>"]
    pub(crate) fn apb35_events_started14c_read(&self) -> MemResult<u32> {
        todo ! ("read apb35_events_started14c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_STARTED: Transaction started<br>"]
    pub(crate) fn apb35_events_started14c_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb35_events_started14c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "END_ACQUIRE: Shortcut between END event and ACQUIRE task<br>"]
    pub(crate) fn apb35_shorts200_end_acquire_read(&self) -> MemResult<bool> {
        todo ! ("read END_ACQUIRE mwrite None write None rac None reset value false")
    }
    #[doc = "END_ACQUIRE: Shortcut between END event and ACQUIRE task<br>"]
    pub(crate) fn apb35_shorts200_end_acquire_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write END_ACQUIRE mwrite None write None rac None reset value false")
    }
    #[doc = "END_START: Shortcut between END event and START task<br>"]
    pub(crate) fn apb35_shorts200_end_start_read(&self) -> MemResult<bool> {
        todo!(
            "read END_START mwrite None write None rac None reset value false"
        )
    }
    #[doc = "END_START: Shortcut between END event and START task<br>"]
    pub(crate) fn apb35_shorts200_end_start_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write END_START mwrite None write None rac None reset value false"
        )
    }
    #[doc = "STOPPED: Write '1' to Enable interrupt for STOPPED event<br>END: Write '1' to Enable interrupt for END event<br>"]
    pub(crate) fn apb35_intenset304_stopped_read(&self) -> MemResult<bool> {
        todo ! ("read STOPPED, END mwrite None write None rac None reset value false")
    }
    #[doc = "STOPPED: Write '1' to Enable interrupt for STOPPED event<br>END: Write '1' to Enable interrupt for END event<br>"]
    pub(crate) fn apb35_intenset304_stopped_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write STOPPED, END mwrite None write None rac None reset value false")
    }
    #[doc = "READY: Write '1' to Enable interrupt for READY event<br>"]
    pub(crate) fn apb35_intenset304_ready_read(&self) -> MemResult<bool> {
        todo!("read READY mwrite None write None rac None reset value false")
    }
    #[doc = "READY: Write '1' to Enable interrupt for READY event<br>"]
    pub(crate) fn apb35_intenset304_ready_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write READY mwrite None write None rac None reset value false")
    }
    #[doc = "ENDRX: Write '1' to Enable interrupt for ENDRX event<br>ENDRX: Write '1' to Enable interrupt for ENDRX event<br>"]
    pub(crate) fn apb35_intenset304_endrx_read(&self) -> MemResult<bool> {
        todo ! ("read ENDRX, ENDRX mwrite None write None rac None reset value false")
    }
    #[doc = "ENDRX: Write '1' to Enable interrupt for ENDRX event<br>ENDRX: Write '1' to Enable interrupt for ENDRX event<br>"]
    pub(crate) fn apb35_intenset304_endrx_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write ENDRX, ENDRX mwrite None write None rac None reset value false")
    }
    #[doc = "END: Write '1' to Enable interrupt for END event<br>"]
    pub(crate) fn apb35_intenset304_end_read(&self) -> MemResult<bool> {
        todo!("read END mwrite None write None rac None reset value false")
    }
    #[doc = "END: Write '1' to Enable interrupt for END event<br>"]
    pub(crate) fn apb35_intenset304_end_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write END mwrite None write None rac None reset value false")
    }
    #[doc = "ENDTX: Write '1' to Enable interrupt for ENDTX event<br>"]
    pub(crate) fn apb35_intenset304_endtx_read(&self) -> MemResult<bool> {
        todo!("read ENDTX mwrite None write None rac None reset value false")
    }
    #[doc = "ENDTX: Write '1' to Enable interrupt for ENDTX event<br>"]
    pub(crate) fn apb35_intenset304_endtx_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ENDTX mwrite None write None rac None reset value false")
    }
    #[doc = "ACQUIRED: Write '1' to Enable interrupt for ACQUIRED event<br>"]
    pub(crate) fn apb35_intenset304_acquired_read(&self) -> MemResult<bool> {
        todo!("read ACQUIRED mwrite None write None rac None reset value false")
    }
    #[doc = "ACQUIRED: Write '1' to Enable interrupt for ACQUIRED event<br>"]
    pub(crate) fn apb35_intenset304_acquired_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write ACQUIRED mwrite None write None rac None reset value false"
        )
    }
    #[doc = "STARTED: Write '1' to Enable interrupt for STARTED event<br>"]
    pub(crate) fn apb35_intenset304_started_read(&self) -> MemResult<bool> {
        todo!("read STARTED mwrite None write None rac None reset value false")
    }
    #[doc = "STARTED: Write '1' to Enable interrupt for STARTED event<br>"]
    pub(crate) fn apb35_intenset304_started_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write STARTED mwrite None write None rac None reset value false")
    }
    #[doc = "STOPPED: Write '1' to Disable interrupt for STOPPED event<br>END: Write '1' to Disable interrupt for END event<br>"]
    pub(crate) fn apb35_intenclr308_stopped_read(&self) -> MemResult<bool> {
        todo ! ("read STOPPED, END mwrite None write None rac None reset value false")
    }
    #[doc = "STOPPED: Write '1' to Disable interrupt for STOPPED event<br>END: Write '1' to Disable interrupt for END event<br>"]
    pub(crate) fn apb35_intenclr308_stopped_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write STOPPED, END mwrite None write None rac None reset value false")
    }
    #[doc = "READY: Write '1' to Disable interrupt for READY event<br>"]
    pub(crate) fn apb35_intenclr308_ready_read(&self) -> MemResult<bool> {
        todo!("read READY mwrite None write None rac None reset value false")
    }
    #[doc = "READY: Write '1' to Disable interrupt for READY event<br>"]
    pub(crate) fn apb35_intenclr308_ready_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write READY mwrite None write None rac None reset value false")
    }
    #[doc = "ENDRX: Write '1' to Disable interrupt for ENDRX event<br>ENDRX: Write '1' to Disable interrupt for ENDRX event<br>"]
    pub(crate) fn apb35_intenclr308_endrx_read(&self) -> MemResult<bool> {
        todo ! ("read ENDRX, ENDRX mwrite None write None rac None reset value false")
    }
    #[doc = "ENDRX: Write '1' to Disable interrupt for ENDRX event<br>ENDRX: Write '1' to Disable interrupt for ENDRX event<br>"]
    pub(crate) fn apb35_intenclr308_endrx_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write ENDRX, ENDRX mwrite None write None rac None reset value false")
    }
    #[doc = "END: Write '1' to Disable interrupt for END event<br>"]
    pub(crate) fn apb35_intenclr308_end_read(&self) -> MemResult<bool> {
        todo!("read END mwrite None write None rac None reset value false")
    }
    #[doc = "END: Write '1' to Disable interrupt for END event<br>"]
    pub(crate) fn apb35_intenclr308_end_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write END mwrite None write None rac None reset value false")
    }
    #[doc = "ENDTX: Write '1' to Disable interrupt for ENDTX event<br>"]
    pub(crate) fn apb35_intenclr308_endtx_read(&self) -> MemResult<bool> {
        todo!("read ENDTX mwrite None write None rac None reset value false")
    }
    #[doc = "ENDTX: Write '1' to Disable interrupt for ENDTX event<br>"]
    pub(crate) fn apb35_intenclr308_endtx_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ENDTX mwrite None write None rac None reset value false")
    }
    #[doc = "ACQUIRED: Write '1' to Disable interrupt for ACQUIRED event<br>"]
    pub(crate) fn apb35_intenclr308_acquired_read(&self) -> MemResult<bool> {
        todo!("read ACQUIRED mwrite None write None rac None reset value false")
    }
    #[doc = "ACQUIRED: Write '1' to Disable interrupt for ACQUIRED event<br>"]
    pub(crate) fn apb35_intenclr308_acquired_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write ACQUIRED mwrite None write None rac None reset value false"
        )
    }
    #[doc = "STARTED: Write '1' to Disable interrupt for STARTED event<br>"]
    pub(crate) fn apb35_intenclr308_started_read(&self) -> MemResult<bool> {
        todo!("read STARTED mwrite None write None rac None reset value false")
    }
    #[doc = "STARTED: Write '1' to Disable interrupt for STARTED event<br>"]
    pub(crate) fn apb35_intenclr308_started_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write STARTED mwrite None write None rac None reset value false")
    }
    #[doc = "SEMSTAT: Semaphore status<br>"]
    pub(crate) fn apb35_semstat400_semstat_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E20Apb3Semstat400Semstat> {
        todo ! ("read SEMSTAT mwrite None write None rac None reset value 0x01 mask 0x03")
    }
    #[doc = "OVERREAD: TX buffer over-read detected, and prevented<br>"]
    pub(crate) fn apb35_status440_overread_read(&self) -> MemResult<bool> {
        todo!("read OVERREAD mwrite None write None rac None reset value false")
    }
    #[doc = "OVERREAD: TX buffer over-read detected, and prevented<br>"]
    pub(crate) fn apb35_status440_overread_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write OVERREAD mwrite None write None rac None reset value false"
        )
    }
    #[doc = "OVERFLOW: RX buffer overflow detected, and prevented<br>"]
    pub(crate) fn apb35_status440_overflow_read(&self) -> MemResult<bool> {
        todo!("read OVERFLOW mwrite None write None rac None reset value false")
    }
    #[doc = "OVERFLOW: RX buffer overflow detected, and prevented<br>"]
    pub(crate) fn apb35_status440_overflow_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write OVERFLOW mwrite None write None rac None reset value false"
        )
    }
    #[doc = "ENABLE: Enable or disable SPI<br>ENABLE: Enable or disable SPIM<br>ENABLE: Enable or disable SPI slave<br>"]
    pub(crate) fn apb35_enable500_enable_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E83Apb35Enable500Enable> {
        todo ! ("read ENABLE, ENABLE, ENABLE mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "ENABLE: Enable or disable SPI<br>ENABLE: Enable or disable SPIM<br>ENABLE: Enable or disable SPI slave<br>"]
    pub(crate) fn apb35_enable500_enable_write(
        &mut self,
        _value: crate::peripheral::enums::E83Apb35Enable500Enable,
    ) -> MemResult<()> {
        todo ! ("write ENABLE, ENABLE, ENABLE mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "PIN: Pin number<br>PIN: Pin number<br>"]
    pub(crate) fn apb35_psel_sck0_pin_read(&self) -> MemResult<u8> {
        todo ! ("read PIN, PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "PIN: Pin number<br>PIN: Pin number<br>"]
    pub(crate) fn apb35_psel_sck0_pin_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write PIN, PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "CONNECT: Connection<br>CONNECT: Connection<br>"]
    pub(crate) fn apb35_psel_sck0_connect_read(&self) -> MemResult<bool> {
        todo ! ("read CONNECT, CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "CONNECT: Connection<br>CONNECT: Connection<br>"]
    pub(crate) fn apb35_psel_sck0_connect_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write CONNECT, CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "PIN: Pin number<br>PIN: Pin number<br>"]
    pub(crate) fn apb35_psel_miso4_pin_read(&self) -> MemResult<u8> {
        todo ! ("read PIN, PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "PIN: Pin number<br>PIN: Pin number<br>"]
    pub(crate) fn apb35_psel_miso4_pin_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write PIN, PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "CONNECT: Connection<br>CONNECT: Connection<br>"]
    pub(crate) fn apb35_psel_miso4_connect_read(&self) -> MemResult<bool> {
        todo ! ("read CONNECT, CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "CONNECT: Connection<br>CONNECT: Connection<br>"]
    pub(crate) fn apb35_psel_miso4_connect_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write CONNECT, CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "PIN: Pin number<br>PIN: Pin number<br>"]
    pub(crate) fn apb35_psel_mosi8_pin_read(&self) -> MemResult<u8> {
        todo ! ("read PIN, PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "PIN: Pin number<br>PIN: Pin number<br>"]
    pub(crate) fn apb35_psel_mosi8_pin_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write PIN, PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "CONNECT: Connection<br>CONNECT: Connection<br>"]
    pub(crate) fn apb35_psel_mosi8_connect_read(&self) -> MemResult<bool> {
        todo ! ("read CONNECT, CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "CONNECT: Connection<br>CONNECT: Connection<br>"]
    pub(crate) fn apb35_psel_mosi8_connect_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write CONNECT, CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "PIN: Pin number<br>"]
    pub(crate) fn apb35_psel_csnc_pin_read(&self) -> MemResult<u8> {
        todo ! ("read PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "PIN: Pin number<br>"]
    pub(crate) fn apb35_psel_csnc_pin_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn apb35_psel_csnc_connect_read(&self) -> MemResult<bool> {
        todo!("read CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn apb35_psel_csnc_connect_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "RXD: RX data received. Double buffered<br>"]
    pub(crate) fn apb35_rxd518_rxd_read(&self) -> MemResult<u8> {
        todo ! ("read RXD mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "TXD: TX data to send. Double buffered<br>"]
    pub(crate) fn apb35_txd51c_txd_read(&self) -> MemResult<u8> {
        todo ! ("read TXD mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "TXD: TX data to send. Double buffered<br>"]
    pub(crate) fn apb35_txd51c_txd_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write TXD mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "FREQUENCY: SPI master data rate<br>FREQUENCY: SPI master data rate<br>"]
    pub(crate) fn apb35_frequency524_frequency_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E84Apb35Frequency524Frequency>
    {
        todo ! ("read FREQUENCY, FREQUENCY mwrite None write None rac None reset value 0x4000000 mask 0xffffffff")
    }
    #[doc = "FREQUENCY: SPI master data rate<br>FREQUENCY: SPI master data rate<br>"]
    pub(crate) fn apb35_frequency524_frequency_write(
        &mut self,
        _value: crate::peripheral::enums::E84Apb35Frequency524Frequency,
    ) -> MemResult<()> {
        todo ! ("write FREQUENCY, FREQUENCY mwrite None write None rac None reset value 0x4000000 mask 0xffffffff")
    }
    #[doc = "PTR: Data pointer<br>PTR: RXD data pointer<br>"]
    pub(crate) fn apb35_rxd_ptr0_ptr_read(&self) -> MemResult<u32> {
        todo ! ("read PTR, PTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "PTR: Data pointer<br>PTR: RXD data pointer<br>"]
    pub(crate) fn apb35_rxd_ptr0_ptr_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write PTR, PTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "MAXCNT: Maximum number of bytes in receive buffer<br>MAXCNT: Maximum number of bytes in receive buffer<br>"]
    pub(crate) fn apb35_rxd_maxcnt4_maxcnt_read(&self) -> MemResult<u8> {
        todo ! ("read MAXCNT, MAXCNT mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "MAXCNT: Maximum number of bytes in receive buffer<br>MAXCNT: Maximum number of bytes in receive buffer<br>"]
    pub(crate) fn apb35_rxd_maxcnt4_maxcnt_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write MAXCNT, MAXCNT mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "AMOUNT: Number of bytes transferred in the last transaction<br>AMOUNT: Number of bytes received in the last granted transaction<br>"]
    pub(crate) fn apb35_rxd_amount8_amount_read(&self) -> MemResult<u8> {
        todo ! ("read AMOUNT, AMOUNT mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "LIST: List type<br>"]
    pub(crate) fn apb35_rxd_listc_list_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E85Apb35RxdListcList> {
        todo ! ("read LIST mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "LIST: List type<br>"]
    pub(crate) fn apb35_rxd_listc_list_write(
        &mut self,
        _value: crate::peripheral::enums::E85Apb35RxdListcList,
    ) -> MemResult<()> {
        todo ! ("write LIST mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "PTR: Data pointer<br>PTR: TXD data pointer<br>"]
    pub(crate) fn apb35_txd_ptr0_ptr_read(&self) -> MemResult<u32> {
        todo ! ("read PTR, PTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "PTR: Data pointer<br>PTR: TXD data pointer<br>"]
    pub(crate) fn apb35_txd_ptr0_ptr_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write PTR, PTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "MAXCNT: Maximum number of bytes in transmit buffer<br>MAXCNT: Maximum number of bytes in transmit buffer<br>"]
    pub(crate) fn apb35_txd_maxcnt4_maxcnt_read(&self) -> MemResult<u8> {
        todo ! ("read MAXCNT, MAXCNT mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "MAXCNT: Maximum number of bytes in transmit buffer<br>MAXCNT: Maximum number of bytes in transmit buffer<br>"]
    pub(crate) fn apb35_txd_maxcnt4_maxcnt_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write MAXCNT, MAXCNT mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "AMOUNT: Number of bytes transferred in the last transaction<br>AMOUNT: Number of bytes transmitted in last granted transaction<br>"]
    pub(crate) fn apb35_txd_amount8_amount_read(&self) -> MemResult<u8> {
        todo ! ("read AMOUNT, AMOUNT mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "LIST: List type<br>"]
    pub(crate) fn apb35_txd_listc_list_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E86Apb35TxdListcList> {
        todo ! ("read LIST mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "LIST: List type<br>"]
    pub(crate) fn apb35_txd_listc_list_write(
        &mut self,
        _value: crate::peripheral::enums::E86Apb35TxdListcList,
    ) -> MemResult<()> {
        todo ! ("write LIST mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "ORDER: Bit order<br>ORDER: Bit order<br>ORDER: Bit order<br>"]
    pub(crate) fn apb35_config554_order_read(&self) -> MemResult<bool> {
        todo ! ("read ORDER, ORDER, ORDER mwrite None write None rac None reset value false")
    }
    #[doc = "ORDER: Bit order<br>ORDER: Bit order<br>ORDER: Bit order<br>"]
    pub(crate) fn apb35_config554_order_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write ORDER, ORDER, ORDER mwrite None write None rac None reset value false")
    }
    #[doc = "CPHA: Serial clock (SCK) phase<br>CPHA: Serial clock (SCK) phase<br>CPHA: Serial clock (SCK) phase<br>"]
    pub(crate) fn apb35_config554_cpha_read(&self) -> MemResult<bool> {
        todo ! ("read CPHA, CPHA, CPHA mwrite None write None rac None reset value false")
    }
    #[doc = "CPHA: Serial clock (SCK) phase<br>CPHA: Serial clock (SCK) phase<br>CPHA: Serial clock (SCK) phase<br>"]
    pub(crate) fn apb35_config554_cpha_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write CPHA, CPHA, CPHA mwrite None write None rac None reset value false")
    }
    #[doc = "CPOL: Serial clock (SCK) polarity<br>CPOL: Serial clock (SCK) polarity<br>CPOL: Serial clock (SCK) polarity<br>"]
    pub(crate) fn apb35_config554_cpol_read(&self) -> MemResult<bool> {
        todo ! ("read CPOL, CPOL, CPOL mwrite None write None rac None reset value false")
    }
    #[doc = "CPOL: Serial clock (SCK) polarity<br>CPOL: Serial clock (SCK) polarity<br>CPOL: Serial clock (SCK) polarity<br>"]
    pub(crate) fn apb35_config554_cpol_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write CPOL, CPOL, CPOL mwrite None write None rac None reset value false")
    }
    #[doc = "DEF: Default character. Character clocked out in case of an ignored transaction.<br>"]
    pub(crate) fn apb35_def55c_def_read(&self) -> MemResult<u8> {
        todo ! ("read DEF mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "DEF: Default character. Character clocked out in case of an ignored transaction.<br>"]
    pub(crate) fn apb35_def55c_def_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write DEF mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "ORC: Over-read character. Character clocked out after an over-read of the transmit buffer.<br>ORC: Over-read character. Character clocked out in case and over-read of the TXD buffer.<br>"]
    pub(crate) fn apb35_orc5c0_orc_read(&self) -> MemResult<u8> {
        todo ! ("read ORC, ORC mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "ORC: Over-read character. Character clocked out after an over-read of the transmit buffer.<br>ORC: Over-read character. Character clocked out in case and over-read of the TXD buffer.<br>"]
    pub(crate) fn apb35_orc5c0_orc_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write ORC, ORC mwrite None write None rac None reset value 0x00 mask 0xff")
    }
}
