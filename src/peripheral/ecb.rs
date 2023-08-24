use icicle_vm::cpu::mem::MemResult;
#[derive(Default)]
#[doc = "ECB: AES ECB Mode Encryption<br><br>Instances:<br>0x4000e000: ECB<br>"]
pub struct Ecb {
    #[doc = "TODO: implement things here"]
    _todo: (),
}
impl Ecb {
    pub(crate) fn page_to_index(page: u64) -> usize {
        match page {
            262158u64 => 0usize,
            _ => unreachable!(),
        }
    }
    #[doc = "TASKS_STARTECB: Start ECB block encrypt<br>"]
    pub(crate) fn ecb_tasks_startecb0_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write ecb_tasks_startecb0 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_STOPECB: Abort a possible executing ECB operation<br>"]
    pub(crate) fn ecb_tasks_stopecb4_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write ecb_tasks_stopecb4 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ENDECB: ECB block encrypt complete<br>"]
    pub(crate) fn ecb_events_endecb100_read(&self) -> MemResult<u32> {
        todo ! ("read ecb_events_endecb100 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ENDECB: ECB block encrypt complete<br>"]
    pub(crate) fn ecb_events_endecb100_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write ecb_events_endecb100 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ERRORECB: ECB block encrypt aborted because of a STOPECB task or due to an error<br>"]
    pub(crate) fn ecb_events_errorecb104_read(&self) -> MemResult<u32> {
        todo ! ("read ecb_events_errorecb104 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ERRORECB: ECB block encrypt aborted because of a STOPECB task or due to an error<br>"]
    pub(crate) fn ecb_events_errorecb104_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write ecb_events_errorecb104 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "ENDECB: Write '1' to Enable interrupt for ENDECB event<br>"]
    pub(crate) fn ecb_intenset304_endecb_read(&self) -> MemResult<bool> {
        todo!("read ENDECB mwrite None write None rac None reset value false")
    }
    #[doc = "ENDECB: Write '1' to Enable interrupt for ENDECB event<br>"]
    pub(crate) fn ecb_intenset304_endecb_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ENDECB mwrite None write None rac None reset value false")
    }
    #[doc = "ERRORECB: Write '1' to Enable interrupt for ERRORECB event<br>"]
    pub(crate) fn ecb_intenset304_errorecb_read(&self) -> MemResult<bool> {
        todo!("read ERRORECB mwrite None write None rac None reset value false")
    }
    #[doc = "ERRORECB: Write '1' to Enable interrupt for ERRORECB event<br>"]
    pub(crate) fn ecb_intenset304_errorecb_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write ERRORECB mwrite None write None rac None reset value false"
        )
    }
    #[doc = "ENDECB: Write '1' to Disable interrupt for ENDECB event<br>"]
    pub(crate) fn ecb_intenclr308_endecb_read(&self) -> MemResult<bool> {
        todo!("read ENDECB mwrite None write None rac None reset value false")
    }
    #[doc = "ENDECB: Write '1' to Disable interrupt for ENDECB event<br>"]
    pub(crate) fn ecb_intenclr308_endecb_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ENDECB mwrite None write None rac None reset value false")
    }
    #[doc = "ERRORECB: Write '1' to Disable interrupt for ERRORECB event<br>"]
    pub(crate) fn ecb_intenclr308_errorecb_read(&self) -> MemResult<bool> {
        todo!("read ERRORECB mwrite None write None rac None reset value false")
    }
    #[doc = "ERRORECB: Write '1' to Disable interrupt for ERRORECB event<br>"]
    pub(crate) fn ecb_intenclr308_errorecb_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write ERRORECB mwrite None write None rac None reset value false"
        )
    }
    #[doc = "ECBDATAPTR: Pointer to the ECB data structure (see Table 1 ECB data structure overview)<br>"]
    pub(crate) fn ecb_ecbdataptr504_ecbdataptr_read(&self) -> MemResult<u32> {
        todo ! ("read ECBDATAPTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "ECBDATAPTR: Pointer to the ECB data structure (see Table 1 ECB data structure overview)<br>"]
    pub(crate) fn ecb_ecbdataptr504_ecbdataptr_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write ECBDATAPTR mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
}
