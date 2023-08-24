use icicle_vm::cpu::mem::MemResult;
#[derive(Default)]
#[doc = "FICR: Factory Information Configuration Registers<br><br>Instances:<br>0x10000000: FICR<br>"]
pub struct Ficr {
    #[doc = "TODO: implement things here"]
    _todo: (),
}
impl Ficr {
    pub(crate) fn page_to_index(page: u64) -> usize {
        match page {
            65536u64 => 0usize,
            _ => unreachable!(),
        }
    }
    #[doc = "CODEPAGESIZE: Code memory page size<br>"]
    pub(crate) fn ficr_codepagesize10_codepagesize_read(
        &self,
    ) -> MemResult<u32> {
        todo ! ("read CODEPAGESIZE mwrite None write None rac None reset value 0xffffffff mask 0xffffffff")
    }
    #[doc = "CODESIZE: Code memory size in number of pages<br>"]
    pub(crate) fn ficr_codesize14_codesize_read(&self) -> MemResult<u32> {
        todo ! ("read CODESIZE mwrite None write None rac None reset value 0xffffffff mask 0xffffffff")
    }
    #[doc = "DEVICEID: 64 bit unique device identifier<br>"]
    pub(crate) fn ficr_deviceidn60_deviceid_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<u32> {
        todo ! ("read DEVICEID mwrite None write None rac None reset value 0xffffffff mask 0xffffffff")
    }
    #[doc = "ER: Encryption Root, word n<br>"]
    pub(crate) fn ficr_ern80_er_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<u32> {
        todo ! ("read ER mwrite None write None rac None reset value 0xffffffff mask 0xffffffff")
    }
    #[doc = "IR: Identity Root, word n<br>"]
    pub(crate) fn ficr_irn90_ir_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<u32> {
        todo ! ("read IR mwrite None write None rac None reset value 0xffffffff mask 0xffffffff")
    }
    #[doc = "DEVICEADDRTYPE: Device address type<br>"]
    pub(crate) fn ficr_deviceaddrtypea0_deviceaddrtype_read(
        &self,
    ) -> MemResult<bool> {
        todo ! ("read DEVICEADDRTYPE mwrite None write None rac None reset value true")
    }
    #[doc = "DEVICEADDR: 48 bit device address<br>"]
    pub(crate) fn ficr_deviceaddrna4_deviceaddr_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<u32> {
        todo ! ("read DEVICEADDR mwrite None write None rac None reset value 0xffffffff mask 0xffffffff")
    }
    #[doc = "PART: Part code<br>"]
    pub(crate) fn ficr_info_part0_part_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E0FicrInfoPart0Part> {
        todo ! ("read PART mwrite None write None rac None reset value 0x52832 mask 0xffffffff")
    }
    #[doc = "VARIANT: Part Variant, Hardware version and Production configuration, encoded as ASCII<br>"]
    pub(crate) fn ficr_info_variant4_variant_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E1FicrInfoVariant4Variant> {
        todo ! ("read VARIANT mwrite None write None rac None reset value 0x41414142 mask 0xffffffff")
    }
    #[doc = "PACKAGE: Package option<br>"]
    pub(crate) fn ficr_info_package8_package_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E2FicrInfoPackage8Package> {
        todo ! ("read PACKAGE mwrite None write None rac None reset value 0x2000 mask 0xffffffff")
    }
    #[doc = "RAM: RAM variant<br>"]
    pub(crate) fn ficr_info_ramc_ram_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E3FicrInfoRamcRam> {
        todo ! ("read RAM mwrite None write None rac None reset value 0x40 mask 0xffffffff")
    }
    #[doc = "FLASH: Flash variant<br>"]
    pub(crate) fn ficr_info_flash10_flash_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E4FicrInfoFlash10Flash> {
        todo ! ("read FLASH mwrite None write None rac None reset value 0x200 mask 0xffffffff")
    }
    #[doc = "UNUSED0\\[%s\\]: Description collection\\[0\\]: Unspecified<br>"]
    pub(crate) fn ficr_info_unused0n14_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<u32> {
        todo ! ("read ficr_info_unused0n14 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "UNUSED0\\[%s\\]: Description collection\\[0\\]: Unspecified<br>"]
    pub(crate) fn ficr_info_unused0n14_write(
        &mut self,
        _reg_array: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write ficr_info_unused0n14 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "A: A (slope definition) register.<br>"]
    pub(crate) fn ficr_temp_a00_a_read(&self) -> MemResult<u16> {
        todo ! ("read A mwrite None write None rac None reset value 0x320 mask 0xfff")
    }
    #[doc = "A: A (slope definition) register.<br>"]
    pub(crate) fn ficr_temp_a14_a_read(&self) -> MemResult<u16> {
        todo ! ("read A mwrite None write None rac None reset value 0x343 mask 0xfff")
    }
    #[doc = "A: A (slope definition) register.<br>"]
    pub(crate) fn ficr_temp_a28_a_read(&self) -> MemResult<u16> {
        todo ! ("read A mwrite None write None rac None reset value 0x35d mask 0xfff")
    }
    #[doc = "A: A (slope definition) register.<br>"]
    pub(crate) fn ficr_temp_a3c_a_read(&self) -> MemResult<u16> {
        todo ! ("read A mwrite None write None rac None reset value 0x400 mask 0xfff")
    }
    #[doc = "A: A (slope definition) register.<br>"]
    pub(crate) fn ficr_temp_a410_a_read(&self) -> MemResult<u16> {
        todo ! ("read A mwrite None write None rac None reset value 0x452 mask 0xfff")
    }
    #[doc = "A: A (slope definition) register.<br>"]
    pub(crate) fn ficr_temp_a514_a_read(&self) -> MemResult<u16> {
        todo ! ("read A mwrite None write None rac None reset value 0x37b mask 0xfff")
    }
    #[doc = "B: B (y-intercept)<br>"]
    pub(crate) fn ficr_temp_b018_b_read(&self) -> MemResult<u16> {
        todo ! ("read B mwrite None write None rac None reset value 0x3fcc mask 0x3fff")
    }
    #[doc = "B: B (y-intercept)<br>"]
    pub(crate) fn ficr_temp_b11c_b_read(&self) -> MemResult<u16> {
        todo ! ("read B mwrite None write None rac None reset value 0x3f98 mask 0x3fff")
    }
    #[doc = "B: B (y-intercept)<br>"]
    pub(crate) fn ficr_temp_b220_b_read(&self) -> MemResult<u16> {
        todo ! ("read B mwrite None write None rac None reset value 0x3f98 mask 0x3fff")
    }
    #[doc = "B: B (y-intercept)<br>"]
    pub(crate) fn ficr_temp_b324_b_read(&self) -> MemResult<u16> {
        todo ! ("read B mwrite None write None rac None reset value 0x12 mask 0x3fff")
    }
    #[doc = "B: B (y-intercept)<br>"]
    pub(crate) fn ficr_temp_b428_b_read(&self) -> MemResult<u16> {
        todo ! ("read B mwrite None write None rac None reset value 0x4d mask 0x3fff")
    }
    #[doc = "B: B (y-intercept)<br>"]
    pub(crate) fn ficr_temp_b52c_b_read(&self) -> MemResult<u16> {
        todo ! ("read B mwrite None write None rac None reset value 0x3e10 mask 0x3fff")
    }
    #[doc = "T: T (segment end)register.<br>"]
    pub(crate) fn ficr_temp_t030_t_read(&self) -> MemResult<u8> {
        todo!(
            "read T mwrite None write None rac None reset value 0xe2 mask 0xff"
        )
    }
    #[doc = "T: T (segment end)register.<br>"]
    pub(crate) fn ficr_temp_t134_t_read(&self) -> MemResult<u8> {
        todo!(
            "read T mwrite None write None rac None reset value 0x00 mask 0xff"
        )
    }
    #[doc = "T: T (segment end)register.<br>"]
    pub(crate) fn ficr_temp_t238_t_read(&self) -> MemResult<u8> {
        todo!(
            "read T mwrite None write None rac None reset value 0x14 mask 0xff"
        )
    }
    #[doc = "T: T (segment end)register.<br>"]
    pub(crate) fn ficr_temp_t33c_t_read(&self) -> MemResult<u8> {
        todo!(
            "read T mwrite None write None rac None reset value 0x19 mask 0xff"
        )
    }
    #[doc = "T: T (segment end)register.<br>"]
    pub(crate) fn ficr_temp_t440_t_read(&self) -> MemResult<u8> {
        todo!(
            "read T mwrite None write None rac None reset value 0x50 mask 0xff"
        )
    }
    #[doc = "MFGID: Default Manufacturer ID: Nordic Semiconductor ASA has ICM 0x5F<br>"]
    pub(crate) fn ficr_nfc_tagheader00_mfgid_read(&self) -> MemResult<u8> {
        todo ! ("read MFGID mwrite None write None rac None reset value 0x5f mask 0xff")
    }
    #[doc = "UD1: Unique identifier byte 1<br>"]
    pub(crate) fn ficr_nfc_tagheader00_ud1_read(&self) -> MemResult<u8> {
        todo ! ("read UD1 mwrite None write None rac None reset value 0xff mask 0xff")
    }
    #[doc = "UD2: Unique identifier byte 2<br>"]
    pub(crate) fn ficr_nfc_tagheader00_ud2_read(&self) -> MemResult<u8> {
        todo ! ("read UD2 mwrite None write None rac None reset value 0xff mask 0xff")
    }
    #[doc = "UD3: Unique identifier byte 3<br>"]
    pub(crate) fn ficr_nfc_tagheader00_ud3_read(&self) -> MemResult<u8> {
        todo ! ("read UD3 mwrite None write None rac None reset value 0xff mask 0xff")
    }
    #[doc = "UD4: Unique identifier byte 4<br>"]
    pub(crate) fn ficr_nfc_tagheader14_ud4_read(&self) -> MemResult<u8> {
        todo ! ("read UD4 mwrite None write None rac None reset value 0xff mask 0xff")
    }
    #[doc = "UD5: Unique identifier byte 5<br>"]
    pub(crate) fn ficr_nfc_tagheader14_ud5_read(&self) -> MemResult<u8> {
        todo ! ("read UD5 mwrite None write None rac None reset value 0xff mask 0xff")
    }
    #[doc = "UD6: Unique identifier byte 6<br>"]
    pub(crate) fn ficr_nfc_tagheader14_ud6_read(&self) -> MemResult<u8> {
        todo ! ("read UD6 mwrite None write None rac None reset value 0xff mask 0xff")
    }
    #[doc = "UD7: Unique identifier byte 7<br>"]
    pub(crate) fn ficr_nfc_tagheader14_ud7_read(&self) -> MemResult<u8> {
        todo ! ("read UD7 mwrite None write None rac None reset value 0xff mask 0xff")
    }
    #[doc = "UD8: Unique identifier byte 8<br>"]
    pub(crate) fn ficr_nfc_tagheader28_ud8_read(&self) -> MemResult<u8> {
        todo ! ("read UD8 mwrite None write None rac None reset value 0xff mask 0xff")
    }
    #[doc = "UD9: Unique identifier byte 9<br>"]
    pub(crate) fn ficr_nfc_tagheader28_ud9_read(&self) -> MemResult<u8> {
        todo ! ("read UD9 mwrite None write None rac None reset value 0xff mask 0xff")
    }
    #[doc = "UD10: Unique identifier byte 10<br>"]
    pub(crate) fn ficr_nfc_tagheader28_ud10_read(&self) -> MemResult<u8> {
        todo ! ("read UD10 mwrite None write None rac None reset value 0xff mask 0xff")
    }
    #[doc = "UD11: Unique identifier byte 11<br>"]
    pub(crate) fn ficr_nfc_tagheader28_ud11_read(&self) -> MemResult<u8> {
        todo ! ("read UD11 mwrite None write None rac None reset value 0xff mask 0xff")
    }
    #[doc = "UD12: Unique identifier byte 12<br>"]
    pub(crate) fn ficr_nfc_tagheader3c_ud12_read(&self) -> MemResult<u8> {
        todo ! ("read UD12 mwrite None write None rac None reset value 0xff mask 0xff")
    }
    #[doc = "UD13: Unique identifier byte 13<br>"]
    pub(crate) fn ficr_nfc_tagheader3c_ud13_read(&self) -> MemResult<u8> {
        todo ! ("read UD13 mwrite None write None rac None reset value 0xff mask 0xff")
    }
    #[doc = "UD14: Unique identifier byte 14<br>"]
    pub(crate) fn ficr_nfc_tagheader3c_ud14_read(&self) -> MemResult<u8> {
        todo ! ("read UD14 mwrite None write None rac None reset value 0xff mask 0xff")
    }
    #[doc = "UD15: Unique identifier byte 15<br>"]
    pub(crate) fn ficr_nfc_tagheader3c_ud15_read(&self) -> MemResult<u8> {
        todo ! ("read UD15 mwrite None write None rac None reset value 0xff mask 0xff")
    }
}
