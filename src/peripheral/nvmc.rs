use icicle_vm::cpu::mem::MemResult;
#[derive(Default)]
#[doc = "NVMC: Non Volatile Memory Controller<br><br>Instances:<br>0x4001e000: NVMC<br>"]
pub struct Nvmc {
    #[doc = "TODO: implement things here"]
    _todo: (),
}
impl Nvmc {
    pub(crate) fn page_to_index(page: u64) -> usize {
        match page {
            262174u64 => 0usize,
            _ => unreachable!(),
        }
    }
    #[doc = "READY: NVMC is ready or busy<br>"]
    pub(crate) fn nvmc_ready400_ready_read(&self) -> MemResult<bool> {
        todo!("read READY mwrite None write None rac None reset value false")
    }
    #[doc = "WEN: Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used. Enabling write or erase will invalidate the cache and keep it invalidated.<br>"]
    pub(crate) fn nvmc_config504_wen_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E60NvmcConfig504Wen> {
        todo ! ("read WEN mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "WEN: Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used. Enabling write or erase will invalidate the cache and keep it invalidated.<br>"]
    pub(crate) fn nvmc_config504_wen_write(
        &mut self,
        _value: crate::peripheral::enums::E60NvmcConfig504Wen,
    ) -> MemResult<()> {
        todo ! ("write WEN mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "ERASEPAGE: Register for starting erase of a page in Code area<br>ERASEPCR1: Register for erasing a page in Code area. Equivalent to ERASEPAGE.<br>"]
    pub(crate) fn nvmc_erasepage508_erasepage_read(&self) -> MemResult<u32> {
        todo ! ("read ERASEPAGE, ERASEPCR1 mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "ERASEPAGE: Register for starting erase of a page in Code area<br>ERASEPCR1: Register for erasing a page in Code area. Equivalent to ERASEPAGE.<br>"]
    pub(crate) fn nvmc_erasepage508_erasepage_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write ERASEPAGE, ERASEPCR1 mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "ERASEALL: Erase all non-volatile memory including UICR registers. Note that code erase has to be enabled by CONFIG.EEN before the UICR can be erased.<br>"]
    pub(crate) fn nvmc_eraseall50c_eraseall_read(&self) -> MemResult<bool> {
        todo!("read ERASEALL mwrite None write None rac None reset value false")
    }
    #[doc = "ERASEALL: Erase all non-volatile memory including UICR registers. Note that code erase has to be enabled by CONFIG.EEN before the UICR can be erased.<br>"]
    pub(crate) fn nvmc_eraseall50c_eraseall_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write ERASEALL mwrite None write None rac None reset value false"
        )
    }
    #[doc = "ERASEPCR0: Register for starting erase of a page in Code area. Equivalent to ERASEPAGE.<br>"]
    pub(crate) fn nvmc_erasepcr0510_erasepcr0_read(&self) -> MemResult<u32> {
        todo ! ("read ERASEPCR0 mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "ERASEPCR0: Register for starting erase of a page in Code area. Equivalent to ERASEPAGE.<br>"]
    pub(crate) fn nvmc_erasepcr0510_erasepcr0_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write ERASEPCR0 mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "ERASEUICR: Register starting erase of all User Information Configuration Registers. Note that code erase has to be enabled by CONFIG.EEN before the UICR can be erased.<br>"]
    pub(crate) fn nvmc_eraseuicr514_eraseuicr_read(&self) -> MemResult<bool> {
        todo!(
            "read ERASEUICR mwrite None write None rac None reset value false"
        )
    }
    #[doc = "ERASEUICR: Register starting erase of all User Information Configuration Registers. Note that code erase has to be enabled by CONFIG.EEN before the UICR can be erased.<br>"]
    pub(crate) fn nvmc_eraseuicr514_eraseuicr_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write ERASEUICR mwrite None write None rac None reset value false"
        )
    }
    #[doc = "CACHEEN: Cache enable<br>"]
    pub(crate) fn nvmc_icachecnf540_cacheen_read(&self) -> MemResult<bool> {
        todo!("read CACHEEN mwrite None write None rac None reset value false")
    }
    #[doc = "CACHEEN: Cache enable<br>"]
    pub(crate) fn nvmc_icachecnf540_cacheen_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CACHEEN mwrite None write None rac None reset value false")
    }
    #[doc = "CACHEPROFEN: Cache profiling enable<br>"]
    pub(crate) fn nvmc_icachecnf540_cacheprofen_read(&self) -> MemResult<bool> {
        todo ! ("read CACHEPROFEN mwrite None write None rac None reset value false")
    }
    #[doc = "CACHEPROFEN: Cache profiling enable<br>"]
    pub(crate) fn nvmc_icachecnf540_cacheprofen_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write CACHEPROFEN mwrite None write None rac None reset value false")
    }
    #[doc = "HITS: Number of cache hits<br>"]
    pub(crate) fn nvmc_ihit548_hits_read(&self) -> MemResult<u32> {
        todo ! ("read HITS mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "HITS: Number of cache hits<br>"]
    pub(crate) fn nvmc_ihit548_hits_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write HITS mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "MISSES: Number of cache misses<br>"]
    pub(crate) fn nvmc_imiss54c_misses_read(&self) -> MemResult<u32> {
        todo ! ("read MISSES mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "MISSES: Number of cache misses<br>"]
    pub(crate) fn nvmc_imiss54c_misses_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write MISSES mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
}
