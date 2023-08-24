use icicle_vm::cpu::mem::MemResult;
#[derive(Default)]
#[doc = "UICR: User Information Configuration Registers<br><br>Instances:<br>0x10001000: UICR<br>"]
pub struct Uicr {
    #[doc = "TODO: implement things here"]
    _todo: (),
}
impl Uicr {
    pub(crate) fn page_to_index(page: u64) -> usize {
        match page {
            65537u64 => 0usize,
            _ => unreachable!(),
        }
    }
    #[doc = "UNUSED0: Unspecified<br>"]
    pub(crate) fn uicr_unused00_read(&self) -> MemResult<u32> {
        todo ! ("read uicr_unused00 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "UNUSED0: Unspecified<br>"]
    pub(crate) fn uicr_unused00_write(&mut self, _value: u32) -> MemResult<()> {
        todo ! ("write uicr_unused00 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "UNUSED1: Unspecified<br>"]
    pub(crate) fn uicr_unused14_read(&self) -> MemResult<u32> {
        todo ! ("read uicr_unused14 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "UNUSED1: Unspecified<br>"]
    pub(crate) fn uicr_unused14_write(&mut self, _value: u32) -> MemResult<()> {
        todo ! ("write uicr_unused14 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "UNUSED2: Unspecified<br>"]
    pub(crate) fn uicr_unused28_read(&self) -> MemResult<u32> {
        todo ! ("read uicr_unused28 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "UNUSED2: Unspecified<br>"]
    pub(crate) fn uicr_unused28_write(&mut self, _value: u32) -> MemResult<()> {
        todo ! ("write uicr_unused28 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "UNUSED3: Unspecified<br>"]
    pub(crate) fn uicr_unused310_read(&self) -> MemResult<u32> {
        todo ! ("read uicr_unused310 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "UNUSED3: Unspecified<br>"]
    pub(crate) fn uicr_unused310_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write uicr_unused310 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "NRFFW: Reserved for Nordic firmware design<br>"]
    pub(crate) fn uicr_nrffwn14_nrffw_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<u32> {
        todo ! ("read NRFFW mwrite None write None rac None reset value 0xffffffff mask 0xffffffff")
    }
    #[doc = "NRFFW: Reserved for Nordic firmware design<br>"]
    pub(crate) fn uicr_nrffwn14_nrffw_write(
        &mut self,
        _reg_array: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write NRFFW mwrite None write None rac None reset value 0xffffffff mask 0xffffffff")
    }
    #[doc = "NRFHW: Reserved for Nordic hardware design<br>"]
    pub(crate) fn uicr_nrfhwn50_nrfhw_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<u32> {
        todo ! ("read NRFHW mwrite None write None rac None reset value 0xffffffff mask 0xffffffff")
    }
    #[doc = "NRFHW: Reserved for Nordic hardware design<br>"]
    pub(crate) fn uicr_nrfhwn50_nrfhw_write(
        &mut self,
        _reg_array: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write NRFHW mwrite None write None rac None reset value 0xffffffff mask 0xffffffff")
    }
    #[doc = "CUSTOMER: Reserved for customer<br>"]
    pub(crate) fn uicr_customern80_customer_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<u32> {
        todo ! ("read CUSTOMER mwrite None write None rac None reset value 0xffffffff mask 0xffffffff")
    }
    #[doc = "CUSTOMER: Reserved for customer<br>"]
    pub(crate) fn uicr_customern80_customer_write(
        &mut self,
        _reg_array: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write CUSTOMER mwrite None write None rac None reset value 0xffffffff mask 0xffffffff")
    }
    #[doc = "PIN: GPIO number P0.n onto which Reset is exposed<br>"]
    pub(crate) fn uicr_pselresetn200_pin_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<u8> {
        todo ! ("read PIN mwrite None write None rac None reset value 0x3f mask 0x3f")
    }
    #[doc = "PIN: GPIO number P0.n onto which Reset is exposed<br>"]
    pub(crate) fn uicr_pselresetn200_pin_write(
        &mut self,
        _reg_array: usize,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write PIN mwrite None write None rac None reset value 0x3f mask 0x3f")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn uicr_pselresetn200_connect_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn uicr_pselresetn200_connect_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "PALL: Enable or disable Access Port protection. Any other value than 0xFF being written to this field will enable protection.<br>"]
    pub(crate) fn uicr_approtect208_pall_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E5UicrApprotect208Pall> {
        todo ! ("read PALL mwrite None write None rac None reset value 0xff mask 0xff")
    }
    #[doc = "PALL: Enable or disable Access Port protection. Any other value than 0xFF being written to this field will enable protection.<br>"]
    pub(crate) fn uicr_approtect208_pall_write(
        &mut self,
        _value: crate::peripheral::enums::E5UicrApprotect208Pall,
    ) -> MemResult<()> {
        todo ! ("write PALL mwrite None write None rac None reset value 0xff mask 0xff")
    }
    #[doc = "PROTECT: Setting of pins dedicated to NFC functionality<br>"]
    pub(crate) fn uicr_nfcpins20c_protect_read(&self) -> MemResult<bool> {
        todo!("read PROTECT mwrite None write None rac None reset value true")
    }
    #[doc = "PROTECT: Setting of pins dedicated to NFC functionality<br>"]
    pub(crate) fn uicr_nfcpins20c_protect_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PROTECT mwrite None write None rac None reset value true")
    }
}
