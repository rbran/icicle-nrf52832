use icicle_vm::cpu::mem::MemResult;
#[derive(Default)]
#[doc = "CCM: AES CCM Mode Encryption<br>AAR: Accelerated Address Resolver<br><br>Instances:<br>0x4000f000: CCM, AAR<br>"]
pub struct Apb15 {
    #[doc = "TODO: implement things here"]
    _todo: (),
}
impl Apb15 {
    pub(crate) fn page_to_index(page: u64) -> usize {
        match page {
            262159u64 => 0usize,
            _ => unreachable!(),
        }
    }
    #[doc = "TASKS_START: Start resolving addresses based on IRKs specified in the IRK data structure<br>TASKS_KSGEN: Start generation of key-stream. This operation will stop by itself when completed.<br>"]
    pub(crate) fn apb15_tasks_start0_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb15_tasks_start0 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_CRYPT: Start encryption/decryption. This operation will stop by itself when completed.<br>"]
    pub(crate) fn apb15_tasks_crypt4_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb15_tasks_crypt4 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_STOP: Stop encryption/decryption<br>TASKS_STOP: Stop resolving addresses<br>"]
    pub(crate) fn apb15_tasks_stop8_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb15_tasks_stop8 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ENDKSGEN: Key-stream generation complete<br>EVENTS_END: Address resolution procedure complete<br>"]
    pub(crate) fn apb15_events_endksgen100_read(&self) -> MemResult<u32> {
        todo ! ("read apb15_events_endksgen100 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ENDKSGEN: Key-stream generation complete<br>EVENTS_END: Address resolution procedure complete<br>"]
    pub(crate) fn apb15_events_endksgen100_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb15_events_endksgen100 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ENDCRYPT: Encrypt/decrypt complete<br>EVENTS_RESOLVED: Address resolved<br>"]
    pub(crate) fn apb15_events_endcrypt104_read(&self) -> MemResult<u32> {
        todo ! ("read apb15_events_endcrypt104 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ENDCRYPT: Encrypt/decrypt complete<br>EVENTS_RESOLVED: Address resolved<br>"]
    pub(crate) fn apb15_events_endcrypt104_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb15_events_endcrypt104 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ERROR: CCM error event<br>EVENTS_NOTRESOLVED: Address not resolved<br>"]
    pub(crate) fn apb15_events_error108_read(&self) -> MemResult<u32> {
        todo ! ("read apb15_events_error108 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ERROR: CCM error event<br>EVENTS_NOTRESOLVED: Address not resolved<br>"]
    pub(crate) fn apb15_events_error108_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write apb15_events_error108 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "ENDKSGEN_CRYPT: Shortcut between ENDKSGEN event and CRYPT task<br>"]
    pub(crate) fn apb15_shorts200_endksgen_crypt_read(
        &self,
    ) -> MemResult<bool> {
        todo ! ("read ENDKSGEN_CRYPT mwrite None write None rac None reset value false")
    }
    #[doc = "ENDKSGEN_CRYPT: Shortcut between ENDKSGEN event and CRYPT task<br>"]
    pub(crate) fn apb15_shorts200_endksgen_crypt_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write ENDKSGEN_CRYPT mwrite None write None rac None reset value false")
    }
    #[doc = "END: Write '1' to Enable interrupt for END event<br>ENDKSGEN: Write '1' to Enable interrupt for ENDKSGEN event<br>"]
    pub(crate) fn apb15_intenset304_end_read(&self) -> MemResult<bool> {
        todo ! ("read END, ENDKSGEN mwrite None write None rac None reset value false")
    }
    #[doc = "END: Write '1' to Enable interrupt for END event<br>ENDKSGEN: Write '1' to Enable interrupt for ENDKSGEN event<br>"]
    pub(crate) fn apb15_intenset304_end_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write END, ENDKSGEN mwrite None write None rac None reset value false")
    }
    #[doc = "RESOLVED: Write '1' to Enable interrupt for RESOLVED event<br>ENDCRYPT: Write '1' to Enable interrupt for ENDCRYPT event<br>"]
    pub(crate) fn apb15_intenset304_resolved_read(&self) -> MemResult<bool> {
        todo ! ("read RESOLVED, ENDCRYPT mwrite None write None rac None reset value false")
    }
    #[doc = "RESOLVED: Write '1' to Enable interrupt for RESOLVED event<br>ENDCRYPT: Write '1' to Enable interrupt for ENDCRYPT event<br>"]
    pub(crate) fn apb15_intenset304_resolved_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write RESOLVED, ENDCRYPT mwrite None write None rac None reset value false")
    }
    #[doc = "NOTRESOLVED: Write '1' to Enable interrupt for NOTRESOLVED event<br>ERROR: Write '1' to Enable interrupt for ERROR event<br>"]
    pub(crate) fn apb15_intenset304_notresolved_read(&self) -> MemResult<bool> {
        todo ! ("read NOTRESOLVED, ERROR mwrite None write None rac None reset value false")
    }
    #[doc = "NOTRESOLVED: Write '1' to Enable interrupt for NOTRESOLVED event<br>ERROR: Write '1' to Enable interrupt for ERROR event<br>"]
    pub(crate) fn apb15_intenset304_notresolved_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write NOTRESOLVED, ERROR mwrite None write None rac None reset value false")
    }
    #[doc = "ENDKSGEN: Write '1' to Disable interrupt for ENDKSGEN event<br>END: Write '1' to Disable interrupt for END event<br>"]
    pub(crate) fn apb15_intenclr308_endksgen_read(&self) -> MemResult<bool> {
        todo ! ("read ENDKSGEN, END mwrite None write None rac None reset value false")
    }
    #[doc = "ENDKSGEN: Write '1' to Disable interrupt for ENDKSGEN event<br>END: Write '1' to Disable interrupt for END event<br>"]
    pub(crate) fn apb15_intenclr308_endksgen_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write ENDKSGEN, END mwrite None write None rac None reset value false")
    }
    #[doc = "ENDCRYPT: Write '1' to Disable interrupt for ENDCRYPT event<br>RESOLVED: Write '1' to Disable interrupt for RESOLVED event<br>"]
    pub(crate) fn apb15_intenclr308_endcrypt_read(&self) -> MemResult<bool> {
        todo ! ("read ENDCRYPT, RESOLVED mwrite None write None rac None reset value false")
    }
    #[doc = "ENDCRYPT: Write '1' to Disable interrupt for ENDCRYPT event<br>RESOLVED: Write '1' to Disable interrupt for RESOLVED event<br>"]
    pub(crate) fn apb15_intenclr308_endcrypt_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write ENDCRYPT, RESOLVED mwrite None write None rac None reset value false")
    }
    #[doc = "ERROR: Write '1' to Disable interrupt for ERROR event<br>NOTRESOLVED: Write '1' to Disable interrupt for NOTRESOLVED event<br>"]
    pub(crate) fn apb15_intenclr308_error_read(&self) -> MemResult<bool> {
        todo ! ("read ERROR, NOTRESOLVED mwrite None write None rac None reset value false")
    }
    #[doc = "ERROR: Write '1' to Disable interrupt for ERROR event<br>NOTRESOLVED: Write '1' to Disable interrupt for NOTRESOLVED event<br>"]
    pub(crate) fn apb15_intenclr308_error_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write ERROR, NOTRESOLVED mwrite None write None rac None reset value false")
    }
    #[doc = "MICSTATUS: The result of the MIC check performed during the previous decryption operation<br>"]
    pub(crate) fn apb15_micstatus400_micstatus_read(&self) -> MemResult<bool> {
        todo!(
            "read MICSTATUS mwrite None write None rac None reset value false"
        )
    }
    #[doc = "ENABLE: Enable or disable CCM<br>ENABLE: Enable or disable AAR<br>"]
    pub(crate) fn apb15_enable500_enable_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E40Apb15Enable500Enable> {
        todo ! ("read ENABLE, ENABLE mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "ENABLE: Enable or disable CCM<br>ENABLE: Enable or disable AAR<br>"]
    pub(crate) fn apb15_enable500_enable_write(
        &mut self,
        _value: crate::peripheral::enums::E40Apb15Enable500Enable,
    ) -> MemResult<()> {
        todo ! ("write ENABLE, ENABLE mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "MODE: The mode of operation to be used<br>"]
    pub(crate) fn apb15_mode504_mode_read(&self) -> MemResult<bool> {
        todo!("read MODE mwrite None write None rac None reset value true")
    }
    #[doc = "MODE: The mode of operation to be used<br>"]
    pub(crate) fn apb15_mode504_mode_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write MODE mwrite None write None rac None reset value true")
    }
    #[doc = "DATARATE: Data rate that the CCM shall run in synch with<br>"]
    pub(crate) fn apb15_mode504_datarate_read(&self) -> MemResult<bool> {
        todo!("read DATARATE mwrite None write None rac None reset value false")
    }
    #[doc = "DATARATE: Data rate that the CCM shall run in synch with<br>"]
    pub(crate) fn apb15_mode504_datarate_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write DATARATE mwrite None write None rac None reset value false"
        )
    }
    #[doc = "LENGTH: Packet length configuration<br>"]
    pub(crate) fn apb15_mode504_length_read(&self) -> MemResult<bool> {
        todo!("read LENGTH mwrite None write None rac None reset value false")
    }
    #[doc = "LENGTH: Packet length configuration<br>"]
    pub(crate) fn apb15_mode504_length_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write LENGTH mwrite None write None rac None reset value false")
    }
    #[doc = "CNFPTR: Pointer to the data structure holding the AES key and the CCM NONCE vector (see Table 1 CCM data structure overview)<br>IRKPTR: Pointer to the IRK data structure<br>"]
    pub(crate) fn apb15_cnfptr508_cnfptr_read(&self) -> MemResult<u32> {
        todo ! ("read CNFPTR, IRKPTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "CNFPTR: Pointer to the data structure holding the AES key and the CCM NONCE vector (see Table 1 CCM data structure overview)<br>IRKPTR: Pointer to the IRK data structure<br>"]
    pub(crate) fn apb15_cnfptr508_cnfptr_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write CNFPTR, IRKPTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "INPTR: Input pointer<br>"]
    pub(crate) fn apb15_inptr50c_inptr_read(&self) -> MemResult<u32> {
        todo ! ("read INPTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "INPTR: Input pointer<br>"]
    pub(crate) fn apb15_inptr50c_inptr_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write INPTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "OUTPTR: Output pointer<br>ADDRPTR: Pointer to the resolvable address (6-bytes)<br>"]
    pub(crate) fn apb15_outptr510_outptr_read(&self) -> MemResult<u32> {
        todo ! ("read OUTPTR, ADDRPTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "OUTPTR: Output pointer<br>ADDRPTR: Pointer to the resolvable address (6-bytes)<br>"]
    pub(crate) fn apb15_outptr510_outptr_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write OUTPTR, ADDRPTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "SCRATCHPTR: Pointer to a scratch data area used for temporary storage during key-stream generation, MIC generation and encryption/decryption.<br>SCRATCHPTR: Pointer to a scratch data area used for temporary storage during resolution.A space of minimum 3 bytes must be reserved.<br>"]
    pub(crate) fn apb15_scratchptr514_scratchptr_read(&self) -> MemResult<u32> {
        todo ! ("read SCRATCHPTR, SCRATCHPTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "SCRATCHPTR: Pointer to a scratch data area used for temporary storage during key-stream generation, MIC generation and encryption/decryption.<br>SCRATCHPTR: Pointer to a scratch data area used for temporary storage during resolution.A space of minimum 3 bytes must be reserved.<br>"]
    pub(crate) fn apb15_scratchptr514_scratchptr_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write SCRATCHPTR, SCRATCHPTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
}
