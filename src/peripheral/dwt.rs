use icicle_vm::cpu::mem::MemResult;
#[derive(Default)]
#[doc = "DWT: System Control registers<br><br>Instances:<br>0xe0001000: DWT<br>"]
pub struct Dwt {
    #[doc = "TODO: implement things here"]
    _todo: (),
}
impl Dwt {
    pub(crate) fn page_to_index(page: u64) -> usize {
        match page {
            917505u64 => 0usize,
            _ => unreachable!(),
        }
    }
    #[doc = "DWT_CTRL: Control Register<br>"]
    pub(crate) fn dwt_dwt_ctrl0_read(&self) -> MemResult<u32> {
        todo ! ("read dwt_dwt_ctrl0 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_CTRL: Control Register<br>"]
    pub(crate) fn dwt_dwt_ctrl0_write(&mut self, _value: u32) -> MemResult<()> {
        todo ! ("write dwt_dwt_ctrl0 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_CYCCNT: Cycle Count Register<br>"]
    pub(crate) fn dwt_dwt_cyccnt4_read(&self) -> MemResult<u32> {
        todo ! ("read dwt_dwt_cyccnt4 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_CYCCNT: Cycle Count Register<br>"]
    pub(crate) fn dwt_dwt_cyccnt4_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write dwt_dwt_cyccnt4 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_CPICNT: CPI Count Register<br>"]
    pub(crate) fn dwt_dwt_cpicnt8_read(&self) -> MemResult<u32> {
        todo ! ("read dwt_dwt_cpicnt8 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_CPICNT: CPI Count Register<br>"]
    pub(crate) fn dwt_dwt_cpicnt8_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write dwt_dwt_cpicnt8 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_EXCCNT: Exception Overhead Count Register<br>"]
    pub(crate) fn dwt_dwt_exccntc_read(&self) -> MemResult<u32> {
        todo ! ("read dwt_dwt_exccntc mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_EXCCNT: Exception Overhead Count Register<br>"]
    pub(crate) fn dwt_dwt_exccntc_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write dwt_dwt_exccntc mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_SLEEPCNT: Sleep Count Register<br>"]
    pub(crate) fn dwt_dwt_sleepcnt10_read(&self) -> MemResult<u32> {
        todo ! ("read dwt_dwt_sleepcnt10 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_SLEEPCNT: Sleep Count Register<br>"]
    pub(crate) fn dwt_dwt_sleepcnt10_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write dwt_dwt_sleepcnt10 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_LSUCNT: LSU Count Register<br>"]
    pub(crate) fn dwt_dwt_lsucnt14_read(&self) -> MemResult<u32> {
        todo ! ("read dwt_dwt_lsucnt14 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_LSUCNT: LSU Count Register<br>"]
    pub(crate) fn dwt_dwt_lsucnt14_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write dwt_dwt_lsucnt14 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_FOLDCNT: Folded-instruction Count Register<br>"]
    pub(crate) fn dwt_dwt_foldcnt18_read(&self) -> MemResult<u32> {
        todo ! ("read dwt_dwt_foldcnt18 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_FOLDCNT: Folded-instruction Count Register<br>"]
    pub(crate) fn dwt_dwt_foldcnt18_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write dwt_dwt_foldcnt18 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_PCSR: Program Counter Sample Register<br>"]
    pub(crate) fn dwt_dwt_pcsr1c_read(&self) -> MemResult<u32> {
        todo ! ("read dwt_dwt_pcsr1c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_COMP0: Comparator Register0<br>"]
    pub(crate) fn dwt_dwt_comp020_read(&self) -> MemResult<u32> {
        todo ! ("read dwt_dwt_comp020 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_COMP0: Comparator Register0<br>"]
    pub(crate) fn dwt_dwt_comp020_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write dwt_dwt_comp020 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_MASK0: Mask Register0<br>"]
    pub(crate) fn dwt_dwt_mask024_read(&self) -> MemResult<u32> {
        todo ! ("read dwt_dwt_mask024 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_MASK0: Mask Register0<br>"]
    pub(crate) fn dwt_dwt_mask024_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write dwt_dwt_mask024 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_FUNCTION0: Function Register0<br>"]
    pub(crate) fn dwt_dwt_function028_read(&self) -> MemResult<u32> {
        todo ! ("read dwt_dwt_function028 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_FUNCTION0: Function Register0<br>"]
    pub(crate) fn dwt_dwt_function028_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write dwt_dwt_function028 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_COMP1: Comparator Register1<br>"]
    pub(crate) fn dwt_dwt_comp130_read(&self) -> MemResult<u32> {
        todo ! ("read dwt_dwt_comp130 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_COMP1: Comparator Register1<br>"]
    pub(crate) fn dwt_dwt_comp130_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write dwt_dwt_comp130 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_MASK1: Mask Register1<br>"]
    pub(crate) fn dwt_dwt_mask134_read(&self) -> MemResult<u32> {
        todo ! ("read dwt_dwt_mask134 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_MASK1: Mask Register1<br>"]
    pub(crate) fn dwt_dwt_mask134_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write dwt_dwt_mask134 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_FUNCTION1: Function Register1<br>"]
    pub(crate) fn dwt_dwt_function138_read(&self) -> MemResult<u32> {
        todo ! ("read dwt_dwt_function138 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_FUNCTION1: Function Register1<br>"]
    pub(crate) fn dwt_dwt_function138_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write dwt_dwt_function138 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_COMP2: Comparator Register2<br>"]
    pub(crate) fn dwt_dwt_comp240_read(&self) -> MemResult<u32> {
        todo ! ("read dwt_dwt_comp240 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_COMP2: Comparator Register2<br>"]
    pub(crate) fn dwt_dwt_comp240_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write dwt_dwt_comp240 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_MASK2: Mask Register2<br>"]
    pub(crate) fn dwt_dwt_mask244_read(&self) -> MemResult<u32> {
        todo ! ("read dwt_dwt_mask244 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_MASK2: Mask Register2<br>"]
    pub(crate) fn dwt_dwt_mask244_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write dwt_dwt_mask244 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_FUNCTION2: Function Register2<br>"]
    pub(crate) fn dwt_dwt_function248_read(&self) -> MemResult<u32> {
        todo ! ("read dwt_dwt_function248 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_FUNCTION2: Function Register2<br>"]
    pub(crate) fn dwt_dwt_function248_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write dwt_dwt_function248 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_COMP3: Comparator Register3<br>"]
    pub(crate) fn dwt_dwt_comp350_read(&self) -> MemResult<u32> {
        todo ! ("read dwt_dwt_comp350 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_COMP3: Comparator Register3<br>"]
    pub(crate) fn dwt_dwt_comp350_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write dwt_dwt_comp350 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_MASK3: Mask Register3<br>"]
    pub(crate) fn dwt_dwt_mask354_read(&self) -> MemResult<u32> {
        todo ! ("read dwt_dwt_mask354 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_MASK3: Mask Register3<br>"]
    pub(crate) fn dwt_dwt_mask354_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write dwt_dwt_mask354 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_FUNCTION3: Function Register3<br>"]
    pub(crate) fn dwt_dwt_function358_read(&self) -> MemResult<u32> {
        todo ! ("read dwt_dwt_function358 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DWT_FUNCTION3: Function Register3<br>"]
    pub(crate) fn dwt_dwt_function358_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write dwt_dwt_function358 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
}
