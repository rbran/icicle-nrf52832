use icicle_vm::cpu::mem::MemResult;
#[derive(Default)]
#[doc = "FPU: FPU<br><br>Instances:<br>0x40026000: FPU<br>"]
pub struct Fpu {
    #[doc = "TODO: implement things here"]
    _todo: (),
}
impl Fpu {
    pub(crate) fn page_to_index(page: u64) -> usize {
        match page {
            262182u64 => 0usize,
            _ => unreachable!(),
        }
    }
    #[doc = "UNUSED: Unused.<br>"]
    pub(crate) fn fpu_unused0_read(&self) -> MemResult<u32> {
        todo ! ("read fpu_unused0 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
}
