use icicle_vm::cpu::mem::MemResult;
#[derive(Default)]
#[doc = "MWU: Memory Watch Unit<br><br>Instances:<br>0x40020000: MWU<br>"]
pub struct Mwu {
    #[doc = "TODO: implement things here"]
    _todo: (),
}
impl Mwu {
    pub(crate) fn page_to_index(page: u64) -> usize {
        match page {
            262176u64 => 0usize,
            _ => unreachable!(),
        }
    }
    #[doc = "WA: Description cluster\\[0\\]:  Write access to region 0 detected<br>"]
    pub(crate) fn mwu_events_regionn_wa0_read(
        &self,
        _events_regionn: usize,
    ) -> MemResult<u32> {
        todo ! ("read mwu_events_regionn_wa0 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "WA: Description cluster\\[0\\]:  Write access to region 0 detected<br>"]
    pub(crate) fn mwu_events_regionn_wa0_write(
        &mut self,
        _events_regionn: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write mwu_events_regionn_wa0 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "RA: Description cluster\\[0\\]:  Read access to region 0 detected<br>"]
    pub(crate) fn mwu_events_regionn_ra4_read(
        &self,
        _events_regionn: usize,
    ) -> MemResult<u32> {
        todo ! ("read mwu_events_regionn_ra4 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "RA: Description cluster\\[0\\]:  Read access to region 0 detected<br>"]
    pub(crate) fn mwu_events_regionn_ra4_write(
        &mut self,
        _events_regionn: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write mwu_events_regionn_ra4 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "WA: Description cluster\\[0\\]:  Write access to peripheral region 0 detected<br>"]
    pub(crate) fn mwu_events_pregionn_wa0_read(
        &self,
        _events_pregionn: usize,
    ) -> MemResult<u32> {
        todo ! ("read mwu_events_pregionn_wa0 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "WA: Description cluster\\[0\\]:  Write access to peripheral region 0 detected<br>"]
    pub(crate) fn mwu_events_pregionn_wa0_write(
        &mut self,
        _events_pregionn: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write mwu_events_pregionn_wa0 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "RA: Description cluster\\[0\\]:  Read access to peripheral region 0 detected<br>"]
    pub(crate) fn mwu_events_pregionn_ra4_read(
        &self,
        _events_pregionn: usize,
    ) -> MemResult<u32> {
        todo ! ("read mwu_events_pregionn_ra4 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "RA: Description cluster\\[0\\]:  Read access to peripheral region 0 detected<br>"]
    pub(crate) fn mwu_events_pregionn_ra4_write(
        &mut self,
        _events_pregionn: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write mwu_events_pregionn_ra4 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "REGION0WA: Enable or disable interrupt for REGION\\[0\\].WA event<br>"]
    pub(crate) fn mwu_inten300_region0wa_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION0WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION0WA: Enable or disable interrupt for REGION\\[0\\].WA event<br>"]
    pub(crate) fn mwu_inten300_region0wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION0WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION0RA: Enable or disable interrupt for REGION\\[0\\].RA event<br>"]
    pub(crate) fn mwu_inten300_region0ra_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION0RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION0RA: Enable or disable interrupt for REGION\\[0\\].RA event<br>"]
    pub(crate) fn mwu_inten300_region0ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION0RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION1WA: Enable or disable interrupt for REGION\\[1\\].WA event<br>"]
    pub(crate) fn mwu_inten300_region1wa_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION1WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION1WA: Enable or disable interrupt for REGION\\[1\\].WA event<br>"]
    pub(crate) fn mwu_inten300_region1wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION1WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION1RA: Enable or disable interrupt for REGION\\[1\\].RA event<br>"]
    pub(crate) fn mwu_inten300_region1ra_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION1RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION1RA: Enable or disable interrupt for REGION\\[1\\].RA event<br>"]
    pub(crate) fn mwu_inten300_region1ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION1RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION2WA: Enable or disable interrupt for REGION\\[2\\].WA event<br>"]
    pub(crate) fn mwu_inten300_region2wa_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION2WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION2WA: Enable or disable interrupt for REGION\\[2\\].WA event<br>"]
    pub(crate) fn mwu_inten300_region2wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION2WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION2RA: Enable or disable interrupt for REGION\\[2\\].RA event<br>"]
    pub(crate) fn mwu_inten300_region2ra_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION2RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION2RA: Enable or disable interrupt for REGION\\[2\\].RA event<br>"]
    pub(crate) fn mwu_inten300_region2ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION2RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION3WA: Enable or disable interrupt for REGION\\[3\\].WA event<br>"]
    pub(crate) fn mwu_inten300_region3wa_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION3WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION3WA: Enable or disable interrupt for REGION\\[3\\].WA event<br>"]
    pub(crate) fn mwu_inten300_region3wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION3WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION3RA: Enable or disable interrupt for REGION\\[3\\].RA event<br>"]
    pub(crate) fn mwu_inten300_region3ra_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION3RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION3RA: Enable or disable interrupt for REGION\\[3\\].RA event<br>"]
    pub(crate) fn mwu_inten300_region3ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION3RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PREGION0WA: Enable or disable interrupt for PREGION\\[0\\].WA event<br>"]
    pub(crate) fn mwu_inten300_pregion0wa_read(&self) -> MemResult<bool> {
        todo!(
            "read PREGION0WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PREGION0WA: Enable or disable interrupt for PREGION\\[0\\].WA event<br>"]
    pub(crate) fn mwu_inten300_pregion0wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write PREGION0WA mwrite None write None rac None reset value false")
    }
    #[doc = "PREGION0RA: Enable or disable interrupt for PREGION\\[0\\].RA event<br>"]
    pub(crate) fn mwu_inten300_pregion0ra_read(&self) -> MemResult<bool> {
        todo!(
            "read PREGION0RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PREGION0RA: Enable or disable interrupt for PREGION\\[0\\].RA event<br>"]
    pub(crate) fn mwu_inten300_pregion0ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write PREGION0RA mwrite None write None rac None reset value false")
    }
    #[doc = "PREGION1WA: Enable or disable interrupt for PREGION\\[1\\].WA event<br>"]
    pub(crate) fn mwu_inten300_pregion1wa_read(&self) -> MemResult<bool> {
        todo!(
            "read PREGION1WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PREGION1WA: Enable or disable interrupt for PREGION\\[1\\].WA event<br>"]
    pub(crate) fn mwu_inten300_pregion1wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write PREGION1WA mwrite None write None rac None reset value false")
    }
    #[doc = "PREGION1RA: Enable or disable interrupt for PREGION\\[1\\].RA event<br>"]
    pub(crate) fn mwu_inten300_pregion1ra_read(&self) -> MemResult<bool> {
        todo!(
            "read PREGION1RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PREGION1RA: Enable or disable interrupt for PREGION\\[1\\].RA event<br>"]
    pub(crate) fn mwu_inten300_pregion1ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write PREGION1RA mwrite None write None rac None reset value false")
    }
    #[doc = "REGION0WA: Write '1' to Enable interrupt for REGION\\[0\\].WA event<br>"]
    pub(crate) fn mwu_intenset304_region0wa_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION0WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION0WA: Write '1' to Enable interrupt for REGION\\[0\\].WA event<br>"]
    pub(crate) fn mwu_intenset304_region0wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION0WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION0RA: Write '1' to Enable interrupt for REGION\\[0\\].RA event<br>"]
    pub(crate) fn mwu_intenset304_region0ra_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION0RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION0RA: Write '1' to Enable interrupt for REGION\\[0\\].RA event<br>"]
    pub(crate) fn mwu_intenset304_region0ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION0RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION1WA: Write '1' to Enable interrupt for REGION\\[1\\].WA event<br>"]
    pub(crate) fn mwu_intenset304_region1wa_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION1WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION1WA: Write '1' to Enable interrupt for REGION\\[1\\].WA event<br>"]
    pub(crate) fn mwu_intenset304_region1wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION1WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION1RA: Write '1' to Enable interrupt for REGION\\[1\\].RA event<br>"]
    pub(crate) fn mwu_intenset304_region1ra_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION1RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION1RA: Write '1' to Enable interrupt for REGION\\[1\\].RA event<br>"]
    pub(crate) fn mwu_intenset304_region1ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION1RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION2WA: Write '1' to Enable interrupt for REGION\\[2\\].WA event<br>"]
    pub(crate) fn mwu_intenset304_region2wa_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION2WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION2WA: Write '1' to Enable interrupt for REGION\\[2\\].WA event<br>"]
    pub(crate) fn mwu_intenset304_region2wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION2WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION2RA: Write '1' to Enable interrupt for REGION\\[2\\].RA event<br>"]
    pub(crate) fn mwu_intenset304_region2ra_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION2RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION2RA: Write '1' to Enable interrupt for REGION\\[2\\].RA event<br>"]
    pub(crate) fn mwu_intenset304_region2ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION2RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION3WA: Write '1' to Enable interrupt for REGION\\[3\\].WA event<br>"]
    pub(crate) fn mwu_intenset304_region3wa_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION3WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION3WA: Write '1' to Enable interrupt for REGION\\[3\\].WA event<br>"]
    pub(crate) fn mwu_intenset304_region3wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION3WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION3RA: Write '1' to Enable interrupt for REGION\\[3\\].RA event<br>"]
    pub(crate) fn mwu_intenset304_region3ra_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION3RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION3RA: Write '1' to Enable interrupt for REGION\\[3\\].RA event<br>"]
    pub(crate) fn mwu_intenset304_region3ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION3RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PREGION0WA: Write '1' to Enable interrupt for PREGION\\[0\\].WA event<br>"]
    pub(crate) fn mwu_intenset304_pregion0wa_read(&self) -> MemResult<bool> {
        todo!(
            "read PREGION0WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PREGION0WA: Write '1' to Enable interrupt for PREGION\\[0\\].WA event<br>"]
    pub(crate) fn mwu_intenset304_pregion0wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write PREGION0WA mwrite None write None rac None reset value false")
    }
    #[doc = "PREGION0RA: Write '1' to Enable interrupt for PREGION\\[0\\].RA event<br>"]
    pub(crate) fn mwu_intenset304_pregion0ra_read(&self) -> MemResult<bool> {
        todo!(
            "read PREGION0RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PREGION0RA: Write '1' to Enable interrupt for PREGION\\[0\\].RA event<br>"]
    pub(crate) fn mwu_intenset304_pregion0ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write PREGION0RA mwrite None write None rac None reset value false")
    }
    #[doc = "PREGION1WA: Write '1' to Enable interrupt for PREGION\\[1\\].WA event<br>"]
    pub(crate) fn mwu_intenset304_pregion1wa_read(&self) -> MemResult<bool> {
        todo!(
            "read PREGION1WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PREGION1WA: Write '1' to Enable interrupt for PREGION\\[1\\].WA event<br>"]
    pub(crate) fn mwu_intenset304_pregion1wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write PREGION1WA mwrite None write None rac None reset value false")
    }
    #[doc = "PREGION1RA: Write '1' to Enable interrupt for PREGION\\[1\\].RA event<br>"]
    pub(crate) fn mwu_intenset304_pregion1ra_read(&self) -> MemResult<bool> {
        todo!(
            "read PREGION1RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PREGION1RA: Write '1' to Enable interrupt for PREGION\\[1\\].RA event<br>"]
    pub(crate) fn mwu_intenset304_pregion1ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write PREGION1RA mwrite None write None rac None reset value false")
    }
    #[doc = "REGION0WA: Write '1' to Disable interrupt for REGION\\[0\\].WA event<br>"]
    pub(crate) fn mwu_intenclr308_region0wa_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION0WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION0WA: Write '1' to Disable interrupt for REGION\\[0\\].WA event<br>"]
    pub(crate) fn mwu_intenclr308_region0wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION0WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION0RA: Write '1' to Disable interrupt for REGION\\[0\\].RA event<br>"]
    pub(crate) fn mwu_intenclr308_region0ra_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION0RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION0RA: Write '1' to Disable interrupt for REGION\\[0\\].RA event<br>"]
    pub(crate) fn mwu_intenclr308_region0ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION0RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION1WA: Write '1' to Disable interrupt for REGION\\[1\\].WA event<br>"]
    pub(crate) fn mwu_intenclr308_region1wa_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION1WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION1WA: Write '1' to Disable interrupt for REGION\\[1\\].WA event<br>"]
    pub(crate) fn mwu_intenclr308_region1wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION1WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION1RA: Write '1' to Disable interrupt for REGION\\[1\\].RA event<br>"]
    pub(crate) fn mwu_intenclr308_region1ra_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION1RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION1RA: Write '1' to Disable interrupt for REGION\\[1\\].RA event<br>"]
    pub(crate) fn mwu_intenclr308_region1ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION1RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION2WA: Write '1' to Disable interrupt for REGION\\[2\\].WA event<br>"]
    pub(crate) fn mwu_intenclr308_region2wa_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION2WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION2WA: Write '1' to Disable interrupt for REGION\\[2\\].WA event<br>"]
    pub(crate) fn mwu_intenclr308_region2wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION2WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION2RA: Write '1' to Disable interrupt for REGION\\[2\\].RA event<br>"]
    pub(crate) fn mwu_intenclr308_region2ra_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION2RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION2RA: Write '1' to Disable interrupt for REGION\\[2\\].RA event<br>"]
    pub(crate) fn mwu_intenclr308_region2ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION2RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION3WA: Write '1' to Disable interrupt for REGION\\[3\\].WA event<br>"]
    pub(crate) fn mwu_intenclr308_region3wa_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION3WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION3WA: Write '1' to Disable interrupt for REGION\\[3\\].WA event<br>"]
    pub(crate) fn mwu_intenclr308_region3wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION3WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION3RA: Write '1' to Disable interrupt for REGION\\[3\\].RA event<br>"]
    pub(crate) fn mwu_intenclr308_region3ra_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION3RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION3RA: Write '1' to Disable interrupt for REGION\\[3\\].RA event<br>"]
    pub(crate) fn mwu_intenclr308_region3ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION3RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PREGION0WA: Write '1' to Disable interrupt for PREGION\\[0\\].WA event<br>"]
    pub(crate) fn mwu_intenclr308_pregion0wa_read(&self) -> MemResult<bool> {
        todo!(
            "read PREGION0WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PREGION0WA: Write '1' to Disable interrupt for PREGION\\[0\\].WA event<br>"]
    pub(crate) fn mwu_intenclr308_pregion0wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write PREGION0WA mwrite None write None rac None reset value false")
    }
    #[doc = "PREGION0RA: Write '1' to Disable interrupt for PREGION\\[0\\].RA event<br>"]
    pub(crate) fn mwu_intenclr308_pregion0ra_read(&self) -> MemResult<bool> {
        todo!(
            "read PREGION0RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PREGION0RA: Write '1' to Disable interrupt for PREGION\\[0\\].RA event<br>"]
    pub(crate) fn mwu_intenclr308_pregion0ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write PREGION0RA mwrite None write None rac None reset value false")
    }
    #[doc = "PREGION1WA: Write '1' to Disable interrupt for PREGION\\[1\\].WA event<br>"]
    pub(crate) fn mwu_intenclr308_pregion1wa_read(&self) -> MemResult<bool> {
        todo!(
            "read PREGION1WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PREGION1WA: Write '1' to Disable interrupt for PREGION\\[1\\].WA event<br>"]
    pub(crate) fn mwu_intenclr308_pregion1wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write PREGION1WA mwrite None write None rac None reset value false")
    }
    #[doc = "PREGION1RA: Write '1' to Disable interrupt for PREGION\\[1\\].RA event<br>"]
    pub(crate) fn mwu_intenclr308_pregion1ra_read(&self) -> MemResult<bool> {
        todo!(
            "read PREGION1RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PREGION1RA: Write '1' to Disable interrupt for PREGION\\[1\\].RA event<br>"]
    pub(crate) fn mwu_intenclr308_pregion1ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write PREGION1RA mwrite None write None rac None reset value false")
    }
    #[doc = "REGION0WA: Enable or disable non-maskable interrupt for REGION\\[0\\].WA event<br>"]
    pub(crate) fn mwu_nmien320_region0wa_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION0WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION0WA: Enable or disable non-maskable interrupt for REGION\\[0\\].WA event<br>"]
    pub(crate) fn mwu_nmien320_region0wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION0WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION0RA: Enable or disable non-maskable interrupt for REGION\\[0\\].RA event<br>"]
    pub(crate) fn mwu_nmien320_region0ra_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION0RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION0RA: Enable or disable non-maskable interrupt for REGION\\[0\\].RA event<br>"]
    pub(crate) fn mwu_nmien320_region0ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION0RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION1WA: Enable or disable non-maskable interrupt for REGION\\[1\\].WA event<br>"]
    pub(crate) fn mwu_nmien320_region1wa_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION1WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION1WA: Enable or disable non-maskable interrupt for REGION\\[1\\].WA event<br>"]
    pub(crate) fn mwu_nmien320_region1wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION1WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION1RA: Enable or disable non-maskable interrupt for REGION\\[1\\].RA event<br>"]
    pub(crate) fn mwu_nmien320_region1ra_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION1RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION1RA: Enable or disable non-maskable interrupt for REGION\\[1\\].RA event<br>"]
    pub(crate) fn mwu_nmien320_region1ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION1RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION2WA: Enable or disable non-maskable interrupt for REGION\\[2\\].WA event<br>"]
    pub(crate) fn mwu_nmien320_region2wa_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION2WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION2WA: Enable or disable non-maskable interrupt for REGION\\[2\\].WA event<br>"]
    pub(crate) fn mwu_nmien320_region2wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION2WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION2RA: Enable or disable non-maskable interrupt for REGION\\[2\\].RA event<br>"]
    pub(crate) fn mwu_nmien320_region2ra_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION2RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION2RA: Enable or disable non-maskable interrupt for REGION\\[2\\].RA event<br>"]
    pub(crate) fn mwu_nmien320_region2ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION2RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION3WA: Enable or disable non-maskable interrupt for REGION\\[3\\].WA event<br>"]
    pub(crate) fn mwu_nmien320_region3wa_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION3WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION3WA: Enable or disable non-maskable interrupt for REGION\\[3\\].WA event<br>"]
    pub(crate) fn mwu_nmien320_region3wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION3WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION3RA: Enable or disable non-maskable interrupt for REGION\\[3\\].RA event<br>"]
    pub(crate) fn mwu_nmien320_region3ra_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION3RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION3RA: Enable or disable non-maskable interrupt for REGION\\[3\\].RA event<br>"]
    pub(crate) fn mwu_nmien320_region3ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION3RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PREGION0WA: Enable or disable non-maskable interrupt for PREGION\\[0\\].WA event<br>"]
    pub(crate) fn mwu_nmien320_pregion0wa_read(&self) -> MemResult<bool> {
        todo!(
            "read PREGION0WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PREGION0WA: Enable or disable non-maskable interrupt for PREGION\\[0\\].WA event<br>"]
    pub(crate) fn mwu_nmien320_pregion0wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write PREGION0WA mwrite None write None rac None reset value false")
    }
    #[doc = "PREGION0RA: Enable or disable non-maskable interrupt for PREGION\\[0\\].RA event<br>"]
    pub(crate) fn mwu_nmien320_pregion0ra_read(&self) -> MemResult<bool> {
        todo!(
            "read PREGION0RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PREGION0RA: Enable or disable non-maskable interrupt for PREGION\\[0\\].RA event<br>"]
    pub(crate) fn mwu_nmien320_pregion0ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write PREGION0RA mwrite None write None rac None reset value false")
    }
    #[doc = "PREGION1WA: Enable or disable non-maskable interrupt for PREGION\\[1\\].WA event<br>"]
    pub(crate) fn mwu_nmien320_pregion1wa_read(&self) -> MemResult<bool> {
        todo!(
            "read PREGION1WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PREGION1WA: Enable or disable non-maskable interrupt for PREGION\\[1\\].WA event<br>"]
    pub(crate) fn mwu_nmien320_pregion1wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write PREGION1WA mwrite None write None rac None reset value false")
    }
    #[doc = "PREGION1RA: Enable or disable non-maskable interrupt for PREGION\\[1\\].RA event<br>"]
    pub(crate) fn mwu_nmien320_pregion1ra_read(&self) -> MemResult<bool> {
        todo!(
            "read PREGION1RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PREGION1RA: Enable or disable non-maskable interrupt for PREGION\\[1\\].RA event<br>"]
    pub(crate) fn mwu_nmien320_pregion1ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write PREGION1RA mwrite None write None rac None reset value false")
    }
    #[doc = "REGION0WA: Write '1' to Enable non-maskable interrupt for REGION\\[0\\].WA event<br>"]
    pub(crate) fn mwu_nmienset324_region0wa_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION0WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION0WA: Write '1' to Enable non-maskable interrupt for REGION\\[0\\].WA event<br>"]
    pub(crate) fn mwu_nmienset324_region0wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION0WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION0RA: Write '1' to Enable non-maskable interrupt for REGION\\[0\\].RA event<br>"]
    pub(crate) fn mwu_nmienset324_region0ra_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION0RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION0RA: Write '1' to Enable non-maskable interrupt for REGION\\[0\\].RA event<br>"]
    pub(crate) fn mwu_nmienset324_region0ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION0RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION1WA: Write '1' to Enable non-maskable interrupt for REGION\\[1\\].WA event<br>"]
    pub(crate) fn mwu_nmienset324_region1wa_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION1WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION1WA: Write '1' to Enable non-maskable interrupt for REGION\\[1\\].WA event<br>"]
    pub(crate) fn mwu_nmienset324_region1wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION1WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION1RA: Write '1' to Enable non-maskable interrupt for REGION\\[1\\].RA event<br>"]
    pub(crate) fn mwu_nmienset324_region1ra_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION1RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION1RA: Write '1' to Enable non-maskable interrupt for REGION\\[1\\].RA event<br>"]
    pub(crate) fn mwu_nmienset324_region1ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION1RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION2WA: Write '1' to Enable non-maskable interrupt for REGION\\[2\\].WA event<br>"]
    pub(crate) fn mwu_nmienset324_region2wa_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION2WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION2WA: Write '1' to Enable non-maskable interrupt for REGION\\[2\\].WA event<br>"]
    pub(crate) fn mwu_nmienset324_region2wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION2WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION2RA: Write '1' to Enable non-maskable interrupt for REGION\\[2\\].RA event<br>"]
    pub(crate) fn mwu_nmienset324_region2ra_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION2RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION2RA: Write '1' to Enable non-maskable interrupt for REGION\\[2\\].RA event<br>"]
    pub(crate) fn mwu_nmienset324_region2ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION2RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION3WA: Write '1' to Enable non-maskable interrupt for REGION\\[3\\].WA event<br>"]
    pub(crate) fn mwu_nmienset324_region3wa_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION3WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION3WA: Write '1' to Enable non-maskable interrupt for REGION\\[3\\].WA event<br>"]
    pub(crate) fn mwu_nmienset324_region3wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION3WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION3RA: Write '1' to Enable non-maskable interrupt for REGION\\[3\\].RA event<br>"]
    pub(crate) fn mwu_nmienset324_region3ra_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION3RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION3RA: Write '1' to Enable non-maskable interrupt for REGION\\[3\\].RA event<br>"]
    pub(crate) fn mwu_nmienset324_region3ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION3RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PREGION0WA: Write '1' to Enable non-maskable interrupt for PREGION\\[0\\].WA event<br>"]
    pub(crate) fn mwu_nmienset324_pregion0wa_read(&self) -> MemResult<bool> {
        todo!(
            "read PREGION0WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PREGION0WA: Write '1' to Enable non-maskable interrupt for PREGION\\[0\\].WA event<br>"]
    pub(crate) fn mwu_nmienset324_pregion0wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write PREGION0WA mwrite None write None rac None reset value false")
    }
    #[doc = "PREGION0RA: Write '1' to Enable non-maskable interrupt for PREGION\\[0\\].RA event<br>"]
    pub(crate) fn mwu_nmienset324_pregion0ra_read(&self) -> MemResult<bool> {
        todo!(
            "read PREGION0RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PREGION0RA: Write '1' to Enable non-maskable interrupt for PREGION\\[0\\].RA event<br>"]
    pub(crate) fn mwu_nmienset324_pregion0ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write PREGION0RA mwrite None write None rac None reset value false")
    }
    #[doc = "PREGION1WA: Write '1' to Enable non-maskable interrupt for PREGION\\[1\\].WA event<br>"]
    pub(crate) fn mwu_nmienset324_pregion1wa_read(&self) -> MemResult<bool> {
        todo!(
            "read PREGION1WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PREGION1WA: Write '1' to Enable non-maskable interrupt for PREGION\\[1\\].WA event<br>"]
    pub(crate) fn mwu_nmienset324_pregion1wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write PREGION1WA mwrite None write None rac None reset value false")
    }
    #[doc = "PREGION1RA: Write '1' to Enable non-maskable interrupt for PREGION\\[1\\].RA event<br>"]
    pub(crate) fn mwu_nmienset324_pregion1ra_read(&self) -> MemResult<bool> {
        todo!(
            "read PREGION1RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PREGION1RA: Write '1' to Enable non-maskable interrupt for PREGION\\[1\\].RA event<br>"]
    pub(crate) fn mwu_nmienset324_pregion1ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write PREGION1RA mwrite None write None rac None reset value false")
    }
    #[doc = "REGION0WA: Write '1' to Disable non-maskable interrupt for REGION\\[0\\].WA event<br>"]
    pub(crate) fn mwu_nmienclr328_region0wa_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION0WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION0WA: Write '1' to Disable non-maskable interrupt for REGION\\[0\\].WA event<br>"]
    pub(crate) fn mwu_nmienclr328_region0wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION0WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION0RA: Write '1' to Disable non-maskable interrupt for REGION\\[0\\].RA event<br>"]
    pub(crate) fn mwu_nmienclr328_region0ra_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION0RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION0RA: Write '1' to Disable non-maskable interrupt for REGION\\[0\\].RA event<br>"]
    pub(crate) fn mwu_nmienclr328_region0ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION0RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION1WA: Write '1' to Disable non-maskable interrupt for REGION\\[1\\].WA event<br>"]
    pub(crate) fn mwu_nmienclr328_region1wa_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION1WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION1WA: Write '1' to Disable non-maskable interrupt for REGION\\[1\\].WA event<br>"]
    pub(crate) fn mwu_nmienclr328_region1wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION1WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION1RA: Write '1' to Disable non-maskable interrupt for REGION\\[1\\].RA event<br>"]
    pub(crate) fn mwu_nmienclr328_region1ra_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION1RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION1RA: Write '1' to Disable non-maskable interrupt for REGION\\[1\\].RA event<br>"]
    pub(crate) fn mwu_nmienclr328_region1ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION1RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION2WA: Write '1' to Disable non-maskable interrupt for REGION\\[2\\].WA event<br>"]
    pub(crate) fn mwu_nmienclr328_region2wa_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION2WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION2WA: Write '1' to Disable non-maskable interrupt for REGION\\[2\\].WA event<br>"]
    pub(crate) fn mwu_nmienclr328_region2wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION2WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION2RA: Write '1' to Disable non-maskable interrupt for REGION\\[2\\].RA event<br>"]
    pub(crate) fn mwu_nmienclr328_region2ra_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION2RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION2RA: Write '1' to Disable non-maskable interrupt for REGION\\[2\\].RA event<br>"]
    pub(crate) fn mwu_nmienclr328_region2ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION2RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION3WA: Write '1' to Disable non-maskable interrupt for REGION\\[3\\].WA event<br>"]
    pub(crate) fn mwu_nmienclr328_region3wa_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION3WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION3WA: Write '1' to Disable non-maskable interrupt for REGION\\[3\\].WA event<br>"]
    pub(crate) fn mwu_nmienclr328_region3wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION3WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION3RA: Write '1' to Disable non-maskable interrupt for REGION\\[3\\].RA event<br>"]
    pub(crate) fn mwu_nmienclr328_region3ra_read(&self) -> MemResult<bool> {
        todo!(
            "read REGION3RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REGION3RA: Write '1' to Disable non-maskable interrupt for REGION\\[3\\].RA event<br>"]
    pub(crate) fn mwu_nmienclr328_region3ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REGION3RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PREGION0WA: Write '1' to Disable non-maskable interrupt for PREGION\\[0\\].WA event<br>"]
    pub(crate) fn mwu_nmienclr328_pregion0wa_read(&self) -> MemResult<bool> {
        todo!(
            "read PREGION0WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PREGION0WA: Write '1' to Disable non-maskable interrupt for PREGION\\[0\\].WA event<br>"]
    pub(crate) fn mwu_nmienclr328_pregion0wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write PREGION0WA mwrite None write None rac None reset value false")
    }
    #[doc = "PREGION0RA: Write '1' to Disable non-maskable interrupt for PREGION\\[0\\].RA event<br>"]
    pub(crate) fn mwu_nmienclr328_pregion0ra_read(&self) -> MemResult<bool> {
        todo!(
            "read PREGION0RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PREGION0RA: Write '1' to Disable non-maskable interrupt for PREGION\\[0\\].RA event<br>"]
    pub(crate) fn mwu_nmienclr328_pregion0ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write PREGION0RA mwrite None write None rac None reset value false")
    }
    #[doc = "PREGION1WA: Write '1' to Disable non-maskable interrupt for PREGION\\[1\\].WA event<br>"]
    pub(crate) fn mwu_nmienclr328_pregion1wa_read(&self) -> MemResult<bool> {
        todo!(
            "read PREGION1WA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PREGION1WA: Write '1' to Disable non-maskable interrupt for PREGION\\[1\\].WA event<br>"]
    pub(crate) fn mwu_nmienclr328_pregion1wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write PREGION1WA mwrite None write None rac None reset value false")
    }
    #[doc = "PREGION1RA: Write '1' to Disable non-maskable interrupt for PREGION\\[1\\].RA event<br>"]
    pub(crate) fn mwu_nmienclr328_pregion1ra_read(&self) -> MemResult<bool> {
        todo!(
            "read PREGION1RA mwrite None write None rac None reset value false"
        )
    }
    #[doc = "PREGION1RA: Write '1' to Disable non-maskable interrupt for PREGION\\[1\\].RA event<br>"]
    pub(crate) fn mwu_nmienclr328_pregion1ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write PREGION1RA mwrite None write None rac None reset value false")
    }
    #[doc = "SR0: Subregion 0 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr0_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR0 mwrite None write None rac None reset value false")
    }
    #[doc = "SR0: Subregion 0 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr0_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR0 mwrite None write None rac None reset value false")
    }
    #[doc = "SR1: Subregion 1 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr1_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR1 mwrite None write None rac None reset value false")
    }
    #[doc = "SR1: Subregion 1 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr1_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR1 mwrite None write None rac None reset value false")
    }
    #[doc = "SR2: Subregion 2 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr2_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR2 mwrite None write None rac None reset value false")
    }
    #[doc = "SR2: Subregion 2 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr2_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR2 mwrite None write None rac None reset value false")
    }
    #[doc = "SR3: Subregion 3 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr3_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR3 mwrite None write None rac None reset value false")
    }
    #[doc = "SR3: Subregion 3 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr3_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR3 mwrite None write None rac None reset value false")
    }
    #[doc = "SR4: Subregion 4 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr4_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR4 mwrite None write None rac None reset value false")
    }
    #[doc = "SR4: Subregion 4 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr4_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR4 mwrite None write None rac None reset value false")
    }
    #[doc = "SR5: Subregion 5 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr5_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR5 mwrite None write None rac None reset value false")
    }
    #[doc = "SR5: Subregion 5 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr5_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR5 mwrite None write None rac None reset value false")
    }
    #[doc = "SR6: Subregion 6 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr6_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR6 mwrite None write None rac None reset value false")
    }
    #[doc = "SR6: Subregion 6 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr6_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR6 mwrite None write None rac None reset value false")
    }
    #[doc = "SR7: Subregion 7 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr7_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR7 mwrite None write None rac None reset value false")
    }
    #[doc = "SR7: Subregion 7 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr7_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR7 mwrite None write None rac None reset value false")
    }
    #[doc = "SR8: Subregion 8 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr8_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR8 mwrite None write None rac None reset value false")
    }
    #[doc = "SR8: Subregion 8 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr8_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR8 mwrite None write None rac None reset value false")
    }
    #[doc = "SR9: Subregion 9 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr9_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR9 mwrite None write None rac None reset value false")
    }
    #[doc = "SR9: Subregion 9 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr9_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR9 mwrite None write None rac None reset value false")
    }
    #[doc = "SR10: Subregion 10 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr10_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR10 mwrite None write None rac None reset value false")
    }
    #[doc = "SR10: Subregion 10 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr10_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR10 mwrite None write None rac None reset value false")
    }
    #[doc = "SR11: Subregion 11 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr11_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR11 mwrite None write None rac None reset value false")
    }
    #[doc = "SR11: Subregion 11 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr11_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR11 mwrite None write None rac None reset value false")
    }
    #[doc = "SR12: Subregion 12 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr12_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR12 mwrite None write None rac None reset value false")
    }
    #[doc = "SR12: Subregion 12 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr12_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR12 mwrite None write None rac None reset value false")
    }
    #[doc = "SR13: Subregion 13 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr13_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR13 mwrite None write None rac None reset value false")
    }
    #[doc = "SR13: Subregion 13 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr13_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR13 mwrite None write None rac None reset value false")
    }
    #[doc = "SR14: Subregion 14 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr14_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR14 mwrite None write None rac None reset value false")
    }
    #[doc = "SR14: Subregion 14 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr14_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR14 mwrite None write None rac None reset value false")
    }
    #[doc = "SR15: Subregion 15 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr15_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR15 mwrite None write None rac None reset value false")
    }
    #[doc = "SR15: Subregion 15 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr15_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR15 mwrite None write None rac None reset value false")
    }
    #[doc = "SR16: Subregion 16 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr16_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR16 mwrite None write None rac None reset value false")
    }
    #[doc = "SR16: Subregion 16 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr16_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR16 mwrite None write None rac None reset value false")
    }
    #[doc = "SR17: Subregion 17 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr17_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR17 mwrite None write None rac None reset value false")
    }
    #[doc = "SR17: Subregion 17 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr17_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR17 mwrite None write None rac None reset value false")
    }
    #[doc = "SR18: Subregion 18 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr18_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR18 mwrite None write None rac None reset value false")
    }
    #[doc = "SR18: Subregion 18 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr18_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR18 mwrite None write None rac None reset value false")
    }
    #[doc = "SR19: Subregion 19 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr19_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR19 mwrite None write None rac None reset value false")
    }
    #[doc = "SR19: Subregion 19 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr19_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR19 mwrite None write None rac None reset value false")
    }
    #[doc = "SR20: Subregion 20 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr20_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR20 mwrite None write None rac None reset value false")
    }
    #[doc = "SR20: Subregion 20 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr20_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR20 mwrite None write None rac None reset value false")
    }
    #[doc = "SR21: Subregion 21 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr21_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR21 mwrite None write None rac None reset value false")
    }
    #[doc = "SR21: Subregion 21 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr21_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR21 mwrite None write None rac None reset value false")
    }
    #[doc = "SR22: Subregion 22 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr22_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR22 mwrite None write None rac None reset value false")
    }
    #[doc = "SR22: Subregion 22 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr22_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR22 mwrite None write None rac None reset value false")
    }
    #[doc = "SR23: Subregion 23 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr23_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR23 mwrite None write None rac None reset value false")
    }
    #[doc = "SR23: Subregion 23 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr23_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR23 mwrite None write None rac None reset value false")
    }
    #[doc = "SR24: Subregion 24 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr24_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR24 mwrite None write None rac None reset value false")
    }
    #[doc = "SR24: Subregion 24 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr24_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR24 mwrite None write None rac None reset value false")
    }
    #[doc = "SR25: Subregion 25 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr25_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR25 mwrite None write None rac None reset value false")
    }
    #[doc = "SR25: Subregion 25 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr25_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR25 mwrite None write None rac None reset value false")
    }
    #[doc = "SR26: Subregion 26 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr26_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR26 mwrite None write None rac None reset value false")
    }
    #[doc = "SR26: Subregion 26 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr26_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR26 mwrite None write None rac None reset value false")
    }
    #[doc = "SR27: Subregion 27 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr27_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR27 mwrite None write None rac None reset value false")
    }
    #[doc = "SR27: Subregion 27 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr27_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR27 mwrite None write None rac None reset value false")
    }
    #[doc = "SR28: Subregion 28 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr28_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR28 mwrite None write None rac None reset value false")
    }
    #[doc = "SR28: Subregion 28 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr28_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR28 mwrite None write None rac None reset value false")
    }
    #[doc = "SR29: Subregion 29 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr29_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR29 mwrite None write None rac None reset value false")
    }
    #[doc = "SR29: Subregion 29 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr29_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR29 mwrite None write None rac None reset value false")
    }
    #[doc = "SR30: Subregion 30 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr30_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR30 mwrite None write None rac None reset value false")
    }
    #[doc = "SR30: Subregion 30 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr30_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR30 mwrite None write None rac None reset value false")
    }
    #[doc = "SR31: Subregion 31 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr31_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR31 mwrite None write None rac None reset value false")
    }
    #[doc = "SR31: Subregion 31 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatwa0_sr31_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR31 mwrite None write None rac None reset value false")
    }
    #[doc = "SR0: Subregion 0 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr0_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR0 mwrite None write None rac None reset value false")
    }
    #[doc = "SR0: Subregion 0 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr0_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR0 mwrite None write None rac None reset value false")
    }
    #[doc = "SR1: Subregion 1 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr1_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR1 mwrite None write None rac None reset value false")
    }
    #[doc = "SR1: Subregion 1 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr1_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR1 mwrite None write None rac None reset value false")
    }
    #[doc = "SR2: Subregion 2 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr2_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR2 mwrite None write None rac None reset value false")
    }
    #[doc = "SR2: Subregion 2 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr2_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR2 mwrite None write None rac None reset value false")
    }
    #[doc = "SR3: Subregion 3 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr3_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR3 mwrite None write None rac None reset value false")
    }
    #[doc = "SR3: Subregion 3 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr3_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR3 mwrite None write None rac None reset value false")
    }
    #[doc = "SR4: Subregion 4 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr4_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR4 mwrite None write None rac None reset value false")
    }
    #[doc = "SR4: Subregion 4 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr4_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR4 mwrite None write None rac None reset value false")
    }
    #[doc = "SR5: Subregion 5 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr5_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR5 mwrite None write None rac None reset value false")
    }
    #[doc = "SR5: Subregion 5 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr5_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR5 mwrite None write None rac None reset value false")
    }
    #[doc = "SR6: Subregion 6 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr6_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR6 mwrite None write None rac None reset value false")
    }
    #[doc = "SR6: Subregion 6 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr6_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR6 mwrite None write None rac None reset value false")
    }
    #[doc = "SR7: Subregion 7 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr7_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR7 mwrite None write None rac None reset value false")
    }
    #[doc = "SR7: Subregion 7 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr7_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR7 mwrite None write None rac None reset value false")
    }
    #[doc = "SR8: Subregion 8 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr8_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR8 mwrite None write None rac None reset value false")
    }
    #[doc = "SR8: Subregion 8 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr8_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR8 mwrite None write None rac None reset value false")
    }
    #[doc = "SR9: Subregion 9 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr9_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR9 mwrite None write None rac None reset value false")
    }
    #[doc = "SR9: Subregion 9 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr9_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR9 mwrite None write None rac None reset value false")
    }
    #[doc = "SR10: Subregion 10 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr10_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR10 mwrite None write None rac None reset value false")
    }
    #[doc = "SR10: Subregion 10 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr10_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR10 mwrite None write None rac None reset value false")
    }
    #[doc = "SR11: Subregion 11 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr11_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR11 mwrite None write None rac None reset value false")
    }
    #[doc = "SR11: Subregion 11 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr11_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR11 mwrite None write None rac None reset value false")
    }
    #[doc = "SR12: Subregion 12 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr12_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR12 mwrite None write None rac None reset value false")
    }
    #[doc = "SR12: Subregion 12 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr12_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR12 mwrite None write None rac None reset value false")
    }
    #[doc = "SR13: Subregion 13 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr13_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR13 mwrite None write None rac None reset value false")
    }
    #[doc = "SR13: Subregion 13 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr13_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR13 mwrite None write None rac None reset value false")
    }
    #[doc = "SR14: Subregion 14 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr14_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR14 mwrite None write None rac None reset value false")
    }
    #[doc = "SR14: Subregion 14 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr14_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR14 mwrite None write None rac None reset value false")
    }
    #[doc = "SR15: Subregion 15 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr15_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR15 mwrite None write None rac None reset value false")
    }
    #[doc = "SR15: Subregion 15 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr15_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR15 mwrite None write None rac None reset value false")
    }
    #[doc = "SR16: Subregion 16 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr16_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR16 mwrite None write None rac None reset value false")
    }
    #[doc = "SR16: Subregion 16 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr16_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR16 mwrite None write None rac None reset value false")
    }
    #[doc = "SR17: Subregion 17 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr17_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR17 mwrite None write None rac None reset value false")
    }
    #[doc = "SR17: Subregion 17 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr17_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR17 mwrite None write None rac None reset value false")
    }
    #[doc = "SR18: Subregion 18 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr18_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR18 mwrite None write None rac None reset value false")
    }
    #[doc = "SR18: Subregion 18 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr18_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR18 mwrite None write None rac None reset value false")
    }
    #[doc = "SR19: Subregion 19 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr19_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR19 mwrite None write None rac None reset value false")
    }
    #[doc = "SR19: Subregion 19 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr19_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR19 mwrite None write None rac None reset value false")
    }
    #[doc = "SR20: Subregion 20 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr20_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR20 mwrite None write None rac None reset value false")
    }
    #[doc = "SR20: Subregion 20 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr20_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR20 mwrite None write None rac None reset value false")
    }
    #[doc = "SR21: Subregion 21 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr21_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR21 mwrite None write None rac None reset value false")
    }
    #[doc = "SR21: Subregion 21 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr21_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR21 mwrite None write None rac None reset value false")
    }
    #[doc = "SR22: Subregion 22 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr22_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR22 mwrite None write None rac None reset value false")
    }
    #[doc = "SR22: Subregion 22 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr22_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR22 mwrite None write None rac None reset value false")
    }
    #[doc = "SR23: Subregion 23 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr23_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR23 mwrite None write None rac None reset value false")
    }
    #[doc = "SR23: Subregion 23 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr23_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR23 mwrite None write None rac None reset value false")
    }
    #[doc = "SR24: Subregion 24 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr24_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR24 mwrite None write None rac None reset value false")
    }
    #[doc = "SR24: Subregion 24 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr24_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR24 mwrite None write None rac None reset value false")
    }
    #[doc = "SR25: Subregion 25 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr25_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR25 mwrite None write None rac None reset value false")
    }
    #[doc = "SR25: Subregion 25 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr25_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR25 mwrite None write None rac None reset value false")
    }
    #[doc = "SR26: Subregion 26 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr26_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR26 mwrite None write None rac None reset value false")
    }
    #[doc = "SR26: Subregion 26 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr26_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR26 mwrite None write None rac None reset value false")
    }
    #[doc = "SR27: Subregion 27 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr27_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR27 mwrite None write None rac None reset value false")
    }
    #[doc = "SR27: Subregion 27 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr27_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR27 mwrite None write None rac None reset value false")
    }
    #[doc = "SR28: Subregion 28 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr28_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR28 mwrite None write None rac None reset value false")
    }
    #[doc = "SR28: Subregion 28 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr28_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR28 mwrite None write None rac None reset value false")
    }
    #[doc = "SR29: Subregion 29 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr29_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR29 mwrite None write None rac None reset value false")
    }
    #[doc = "SR29: Subregion 29 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr29_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR29 mwrite None write None rac None reset value false")
    }
    #[doc = "SR30: Subregion 30 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr30_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR30 mwrite None write None rac None reset value false")
    }
    #[doc = "SR30: Subregion 30 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr30_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR30 mwrite None write None rac None reset value false")
    }
    #[doc = "SR31: Subregion 31 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr31_read(
        &self,
        _perregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR31 mwrite None write None rac None reset value false")
    }
    #[doc = "SR31: Subregion 31 in region 0 (write '1' to clear)<br>"]
    pub(crate) fn mwu_perregionn_substatra4_sr31_write(
        &mut self,
        _perregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR31 mwrite None write None rac None reset value false")
    }
    #[doc = "RGN0WA: Enable/disable write access watch in region\\[0\\]<br>"]
    pub(crate) fn mwu_regionen510_rgn0wa_read(&self) -> MemResult<bool> {
        todo!("read RGN0WA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN0WA: Enable/disable write access watch in region\\[0\\]<br>"]
    pub(crate) fn mwu_regionen510_rgn0wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RGN0WA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN0RA: Enable/disable read access watch in region\\[0\\]<br>"]
    pub(crate) fn mwu_regionen510_rgn0ra_read(&self) -> MemResult<bool> {
        todo!("read RGN0RA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN0RA: Enable/disable read access watch in region\\[0\\]<br>"]
    pub(crate) fn mwu_regionen510_rgn0ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RGN0RA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN1WA: Enable/disable write access watch in region\\[1\\]<br>"]
    pub(crate) fn mwu_regionen510_rgn1wa_read(&self) -> MemResult<bool> {
        todo!("read RGN1WA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN1WA: Enable/disable write access watch in region\\[1\\]<br>"]
    pub(crate) fn mwu_regionen510_rgn1wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RGN1WA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN1RA: Enable/disable read access watch in region\\[1\\]<br>"]
    pub(crate) fn mwu_regionen510_rgn1ra_read(&self) -> MemResult<bool> {
        todo!("read RGN1RA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN1RA: Enable/disable read access watch in region\\[1\\]<br>"]
    pub(crate) fn mwu_regionen510_rgn1ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RGN1RA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN2WA: Enable/disable write access watch in region\\[2\\]<br>"]
    pub(crate) fn mwu_regionen510_rgn2wa_read(&self) -> MemResult<bool> {
        todo!("read RGN2WA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN2WA: Enable/disable write access watch in region\\[2\\]<br>"]
    pub(crate) fn mwu_regionen510_rgn2wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RGN2WA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN2RA: Enable/disable read access watch in region\\[2\\]<br>"]
    pub(crate) fn mwu_regionen510_rgn2ra_read(&self) -> MemResult<bool> {
        todo!("read RGN2RA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN2RA: Enable/disable read access watch in region\\[2\\]<br>"]
    pub(crate) fn mwu_regionen510_rgn2ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RGN2RA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN3WA: Enable/disable write access watch in region\\[3\\]<br>"]
    pub(crate) fn mwu_regionen510_rgn3wa_read(&self) -> MemResult<bool> {
        todo!("read RGN3WA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN3WA: Enable/disable write access watch in region\\[3\\]<br>"]
    pub(crate) fn mwu_regionen510_rgn3wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RGN3WA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN3RA: Enable/disable read access watch in region\\[3\\]<br>"]
    pub(crate) fn mwu_regionen510_rgn3ra_read(&self) -> MemResult<bool> {
        todo!("read RGN3RA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN3RA: Enable/disable read access watch in region\\[3\\]<br>"]
    pub(crate) fn mwu_regionen510_rgn3ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RGN3RA mwrite None write None rac None reset value false")
    }
    #[doc = "PRGN0WA: Enable/disable write access watch in PREGION\\[0\\]<br>"]
    pub(crate) fn mwu_regionen510_prgn0wa_read(&self) -> MemResult<bool> {
        todo!("read PRGN0WA mwrite None write None rac None reset value false")
    }
    #[doc = "PRGN0WA: Enable/disable write access watch in PREGION\\[0\\]<br>"]
    pub(crate) fn mwu_regionen510_prgn0wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PRGN0WA mwrite None write None rac None reset value false")
    }
    #[doc = "PRGN0RA: Enable/disable read access watch in PREGION\\[0\\]<br>"]
    pub(crate) fn mwu_regionen510_prgn0ra_read(&self) -> MemResult<bool> {
        todo!("read PRGN0RA mwrite None write None rac None reset value false")
    }
    #[doc = "PRGN0RA: Enable/disable read access watch in PREGION\\[0\\]<br>"]
    pub(crate) fn mwu_regionen510_prgn0ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PRGN0RA mwrite None write None rac None reset value false")
    }
    #[doc = "PRGN1WA: Enable/disable write access watch in PREGION\\[1\\]<br>"]
    pub(crate) fn mwu_regionen510_prgn1wa_read(&self) -> MemResult<bool> {
        todo!("read PRGN1WA mwrite None write None rac None reset value false")
    }
    #[doc = "PRGN1WA: Enable/disable write access watch in PREGION\\[1\\]<br>"]
    pub(crate) fn mwu_regionen510_prgn1wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PRGN1WA mwrite None write None rac None reset value false")
    }
    #[doc = "PRGN1RA: Enable/disable read access watch in PREGION\\[1\\]<br>"]
    pub(crate) fn mwu_regionen510_prgn1ra_read(&self) -> MemResult<bool> {
        todo!("read PRGN1RA mwrite None write None rac None reset value false")
    }
    #[doc = "PRGN1RA: Enable/disable read access watch in PREGION\\[1\\]<br>"]
    pub(crate) fn mwu_regionen510_prgn1ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PRGN1RA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN0WA: Enable write access watch in region\\[0\\]<br>"]
    pub(crate) fn mwu_regionenset514_rgn0wa_read(&self) -> MemResult<bool> {
        todo!("read RGN0WA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN0WA: Enable write access watch in region\\[0\\]<br>"]
    pub(crate) fn mwu_regionenset514_rgn0wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RGN0WA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN0RA: Enable read access watch in region\\[0\\]<br>"]
    pub(crate) fn mwu_regionenset514_rgn0ra_read(&self) -> MemResult<bool> {
        todo!("read RGN0RA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN0RA: Enable read access watch in region\\[0\\]<br>"]
    pub(crate) fn mwu_regionenset514_rgn0ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RGN0RA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN1WA: Enable write access watch in region\\[1\\]<br>"]
    pub(crate) fn mwu_regionenset514_rgn1wa_read(&self) -> MemResult<bool> {
        todo!("read RGN1WA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN1WA: Enable write access watch in region\\[1\\]<br>"]
    pub(crate) fn mwu_regionenset514_rgn1wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RGN1WA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN1RA: Enable read access watch in region\\[1\\]<br>"]
    pub(crate) fn mwu_regionenset514_rgn1ra_read(&self) -> MemResult<bool> {
        todo!("read RGN1RA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN1RA: Enable read access watch in region\\[1\\]<br>"]
    pub(crate) fn mwu_regionenset514_rgn1ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RGN1RA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN2WA: Enable write access watch in region\\[2\\]<br>"]
    pub(crate) fn mwu_regionenset514_rgn2wa_read(&self) -> MemResult<bool> {
        todo!("read RGN2WA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN2WA: Enable write access watch in region\\[2\\]<br>"]
    pub(crate) fn mwu_regionenset514_rgn2wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RGN2WA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN2RA: Enable read access watch in region\\[2\\]<br>"]
    pub(crate) fn mwu_regionenset514_rgn2ra_read(&self) -> MemResult<bool> {
        todo!("read RGN2RA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN2RA: Enable read access watch in region\\[2\\]<br>"]
    pub(crate) fn mwu_regionenset514_rgn2ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RGN2RA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN3WA: Enable write access watch in region\\[3\\]<br>"]
    pub(crate) fn mwu_regionenset514_rgn3wa_read(&self) -> MemResult<bool> {
        todo!("read RGN3WA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN3WA: Enable write access watch in region\\[3\\]<br>"]
    pub(crate) fn mwu_regionenset514_rgn3wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RGN3WA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN3RA: Enable read access watch in region\\[3\\]<br>"]
    pub(crate) fn mwu_regionenset514_rgn3ra_read(&self) -> MemResult<bool> {
        todo!("read RGN3RA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN3RA: Enable read access watch in region\\[3\\]<br>"]
    pub(crate) fn mwu_regionenset514_rgn3ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RGN3RA mwrite None write None rac None reset value false")
    }
    #[doc = "PRGN0WA: Enable write access watch in PREGION\\[0\\]<br>"]
    pub(crate) fn mwu_regionenset514_prgn0wa_read(&self) -> MemResult<bool> {
        todo!("read PRGN0WA mwrite None write None rac None reset value false")
    }
    #[doc = "PRGN0WA: Enable write access watch in PREGION\\[0\\]<br>"]
    pub(crate) fn mwu_regionenset514_prgn0wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PRGN0WA mwrite None write None rac None reset value false")
    }
    #[doc = "PRGN0RA: Enable read access watch in PREGION\\[0\\]<br>"]
    pub(crate) fn mwu_regionenset514_prgn0ra_read(&self) -> MemResult<bool> {
        todo!("read PRGN0RA mwrite None write None rac None reset value false")
    }
    #[doc = "PRGN0RA: Enable read access watch in PREGION\\[0\\]<br>"]
    pub(crate) fn mwu_regionenset514_prgn0ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PRGN0RA mwrite None write None rac None reset value false")
    }
    #[doc = "PRGN1WA: Enable write access watch in PREGION\\[1\\]<br>"]
    pub(crate) fn mwu_regionenset514_prgn1wa_read(&self) -> MemResult<bool> {
        todo!("read PRGN1WA mwrite None write None rac None reset value false")
    }
    #[doc = "PRGN1WA: Enable write access watch in PREGION\\[1\\]<br>"]
    pub(crate) fn mwu_regionenset514_prgn1wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PRGN1WA mwrite None write None rac None reset value false")
    }
    #[doc = "PRGN1RA: Enable read access watch in PREGION\\[1\\]<br>"]
    pub(crate) fn mwu_regionenset514_prgn1ra_read(&self) -> MemResult<bool> {
        todo!("read PRGN1RA mwrite None write None rac None reset value false")
    }
    #[doc = "PRGN1RA: Enable read access watch in PREGION\\[1\\]<br>"]
    pub(crate) fn mwu_regionenset514_prgn1ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PRGN1RA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN0WA: Disable write access watch in region\\[0\\]<br>"]
    pub(crate) fn mwu_regionenclr518_rgn0wa_read(&self) -> MemResult<bool> {
        todo!("read RGN0WA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN0WA: Disable write access watch in region\\[0\\]<br>"]
    pub(crate) fn mwu_regionenclr518_rgn0wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RGN0WA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN0RA: Disable read access watch in region\\[0\\]<br>"]
    pub(crate) fn mwu_regionenclr518_rgn0ra_read(&self) -> MemResult<bool> {
        todo!("read RGN0RA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN0RA: Disable read access watch in region\\[0\\]<br>"]
    pub(crate) fn mwu_regionenclr518_rgn0ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RGN0RA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN1WA: Disable write access watch in region\\[1\\]<br>"]
    pub(crate) fn mwu_regionenclr518_rgn1wa_read(&self) -> MemResult<bool> {
        todo!("read RGN1WA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN1WA: Disable write access watch in region\\[1\\]<br>"]
    pub(crate) fn mwu_regionenclr518_rgn1wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RGN1WA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN1RA: Disable read access watch in region\\[1\\]<br>"]
    pub(crate) fn mwu_regionenclr518_rgn1ra_read(&self) -> MemResult<bool> {
        todo!("read RGN1RA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN1RA: Disable read access watch in region\\[1\\]<br>"]
    pub(crate) fn mwu_regionenclr518_rgn1ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RGN1RA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN2WA: Disable write access watch in region\\[2\\]<br>"]
    pub(crate) fn mwu_regionenclr518_rgn2wa_read(&self) -> MemResult<bool> {
        todo!("read RGN2WA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN2WA: Disable write access watch in region\\[2\\]<br>"]
    pub(crate) fn mwu_regionenclr518_rgn2wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RGN2WA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN2RA: Disable read access watch in region\\[2\\]<br>"]
    pub(crate) fn mwu_regionenclr518_rgn2ra_read(&self) -> MemResult<bool> {
        todo!("read RGN2RA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN2RA: Disable read access watch in region\\[2\\]<br>"]
    pub(crate) fn mwu_regionenclr518_rgn2ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RGN2RA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN3WA: Disable write access watch in region\\[3\\]<br>"]
    pub(crate) fn mwu_regionenclr518_rgn3wa_read(&self) -> MemResult<bool> {
        todo!("read RGN3WA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN3WA: Disable write access watch in region\\[3\\]<br>"]
    pub(crate) fn mwu_regionenclr518_rgn3wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RGN3WA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN3RA: Disable read access watch in region\\[3\\]<br>"]
    pub(crate) fn mwu_regionenclr518_rgn3ra_read(&self) -> MemResult<bool> {
        todo!("read RGN3RA mwrite None write None rac None reset value false")
    }
    #[doc = "RGN3RA: Disable read access watch in region\\[3\\]<br>"]
    pub(crate) fn mwu_regionenclr518_rgn3ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write RGN3RA mwrite None write None rac None reset value false")
    }
    #[doc = "PRGN0WA: Disable write access watch in PREGION\\[0\\]<br>"]
    pub(crate) fn mwu_regionenclr518_prgn0wa_read(&self) -> MemResult<bool> {
        todo!("read PRGN0WA mwrite None write None rac None reset value false")
    }
    #[doc = "PRGN0WA: Disable write access watch in PREGION\\[0\\]<br>"]
    pub(crate) fn mwu_regionenclr518_prgn0wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PRGN0WA mwrite None write None rac None reset value false")
    }
    #[doc = "PRGN0RA: Disable read access watch in PREGION\\[0\\]<br>"]
    pub(crate) fn mwu_regionenclr518_prgn0ra_read(&self) -> MemResult<bool> {
        todo!("read PRGN0RA mwrite None write None rac None reset value false")
    }
    #[doc = "PRGN0RA: Disable read access watch in PREGION\\[0\\]<br>"]
    pub(crate) fn mwu_regionenclr518_prgn0ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PRGN0RA mwrite None write None rac None reset value false")
    }
    #[doc = "PRGN1WA: Disable write access watch in PREGION\\[1\\]<br>"]
    pub(crate) fn mwu_regionenclr518_prgn1wa_read(&self) -> MemResult<bool> {
        todo!("read PRGN1WA mwrite None write None rac None reset value false")
    }
    #[doc = "PRGN1WA: Disable write access watch in PREGION\\[1\\]<br>"]
    pub(crate) fn mwu_regionenclr518_prgn1wa_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PRGN1WA mwrite None write None rac None reset value false")
    }
    #[doc = "PRGN1RA: Disable read access watch in PREGION\\[1\\]<br>"]
    pub(crate) fn mwu_regionenclr518_prgn1ra_read(&self) -> MemResult<bool> {
        todo!("read PRGN1RA mwrite None write None rac None reset value false")
    }
    #[doc = "PRGN1RA: Disable read access watch in PREGION\\[1\\]<br>"]
    pub(crate) fn mwu_regionenclr518_prgn1ra_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PRGN1RA mwrite None write None rac None reset value false")
    }
    #[doc = "START: Start address for region<br>"]
    pub(crate) fn mwu_regionn_start0_start_read(
        &self,
        _regionn: usize,
    ) -> MemResult<u32> {
        todo ! ("read START mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "START: Start address for region<br>"]
    pub(crate) fn mwu_regionn_start0_start_write(
        &mut self,
        _regionn: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write START mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "END: End address of region.<br>"]
    pub(crate) fn mwu_regionn_end4_end_read(
        &self,
        _regionn: usize,
    ) -> MemResult<u32> {
        todo ! ("read END mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "END: End address of region.<br>"]
    pub(crate) fn mwu_regionn_end4_end_write(
        &mut self,
        _regionn: usize,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write END mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "START: Reserved for future use<br>"]
    pub(crate) fn mwu_pregionn_start0_start_read(
        &self,
        _pregionn: usize,
    ) -> MemResult<u32> {
        todo ! ("read START mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "END: Reserved for future use<br>"]
    pub(crate) fn mwu_pregionn_end4_end_read(
        &self,
        _pregionn: usize,
    ) -> MemResult<u32> {
        todo ! ("read END mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "SR0: Include or exclude subregion 0 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr0_read(
        &self,
        _pregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR0 mwrite None write None rac None reset value false")
    }
    #[doc = "SR0: Include or exclude subregion 0 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr0_write(
        &mut self,
        _pregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR0 mwrite None write None rac None reset value false")
    }
    #[doc = "SR1: Include or exclude subregion 1 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr1_read(
        &self,
        _pregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR1 mwrite None write None rac None reset value false")
    }
    #[doc = "SR1: Include or exclude subregion 1 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr1_write(
        &mut self,
        _pregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR1 mwrite None write None rac None reset value false")
    }
    #[doc = "SR2: Include or exclude subregion 2 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr2_read(
        &self,
        _pregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR2 mwrite None write None rac None reset value false")
    }
    #[doc = "SR2: Include or exclude subregion 2 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr2_write(
        &mut self,
        _pregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR2 mwrite None write None rac None reset value false")
    }
    #[doc = "SR3: Include or exclude subregion 3 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr3_read(
        &self,
        _pregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR3 mwrite None write None rac None reset value false")
    }
    #[doc = "SR3: Include or exclude subregion 3 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr3_write(
        &mut self,
        _pregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR3 mwrite None write None rac None reset value false")
    }
    #[doc = "SR4: Include or exclude subregion 4 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr4_read(
        &self,
        _pregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR4 mwrite None write None rac None reset value false")
    }
    #[doc = "SR4: Include or exclude subregion 4 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr4_write(
        &mut self,
        _pregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR4 mwrite None write None rac None reset value false")
    }
    #[doc = "SR5: Include or exclude subregion 5 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr5_read(
        &self,
        _pregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR5 mwrite None write None rac None reset value false")
    }
    #[doc = "SR5: Include or exclude subregion 5 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr5_write(
        &mut self,
        _pregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR5 mwrite None write None rac None reset value false")
    }
    #[doc = "SR6: Include or exclude subregion 6 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr6_read(
        &self,
        _pregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR6 mwrite None write None rac None reset value false")
    }
    #[doc = "SR6: Include or exclude subregion 6 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr6_write(
        &mut self,
        _pregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR6 mwrite None write None rac None reset value false")
    }
    #[doc = "SR7: Include or exclude subregion 7 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr7_read(
        &self,
        _pregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR7 mwrite None write None rac None reset value false")
    }
    #[doc = "SR7: Include or exclude subregion 7 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr7_write(
        &mut self,
        _pregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR7 mwrite None write None rac None reset value false")
    }
    #[doc = "SR8: Include or exclude subregion 8 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr8_read(
        &self,
        _pregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR8 mwrite None write None rac None reset value false")
    }
    #[doc = "SR8: Include or exclude subregion 8 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr8_write(
        &mut self,
        _pregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR8 mwrite None write None rac None reset value false")
    }
    #[doc = "SR9: Include or exclude subregion 9 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr9_read(
        &self,
        _pregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR9 mwrite None write None rac None reset value false")
    }
    #[doc = "SR9: Include or exclude subregion 9 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr9_write(
        &mut self,
        _pregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR9 mwrite None write None rac None reset value false")
    }
    #[doc = "SR10: Include or exclude subregion 10 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr10_read(
        &self,
        _pregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR10 mwrite None write None rac None reset value false")
    }
    #[doc = "SR10: Include or exclude subregion 10 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr10_write(
        &mut self,
        _pregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR10 mwrite None write None rac None reset value false")
    }
    #[doc = "SR11: Include or exclude subregion 11 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr11_read(
        &self,
        _pregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR11 mwrite None write None rac None reset value false")
    }
    #[doc = "SR11: Include or exclude subregion 11 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr11_write(
        &mut self,
        _pregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR11 mwrite None write None rac None reset value false")
    }
    #[doc = "SR12: Include or exclude subregion 12 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr12_read(
        &self,
        _pregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR12 mwrite None write None rac None reset value false")
    }
    #[doc = "SR12: Include or exclude subregion 12 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr12_write(
        &mut self,
        _pregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR12 mwrite None write None rac None reset value false")
    }
    #[doc = "SR13: Include or exclude subregion 13 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr13_read(
        &self,
        _pregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR13 mwrite None write None rac None reset value false")
    }
    #[doc = "SR13: Include or exclude subregion 13 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr13_write(
        &mut self,
        _pregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR13 mwrite None write None rac None reset value false")
    }
    #[doc = "SR14: Include or exclude subregion 14 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr14_read(
        &self,
        _pregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR14 mwrite None write None rac None reset value false")
    }
    #[doc = "SR14: Include or exclude subregion 14 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr14_write(
        &mut self,
        _pregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR14 mwrite None write None rac None reset value false")
    }
    #[doc = "SR15: Include or exclude subregion 15 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr15_read(
        &self,
        _pregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR15 mwrite None write None rac None reset value false")
    }
    #[doc = "SR15: Include or exclude subregion 15 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr15_write(
        &mut self,
        _pregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR15 mwrite None write None rac None reset value false")
    }
    #[doc = "SR16: Include or exclude subregion 16 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr16_read(
        &self,
        _pregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR16 mwrite None write None rac None reset value false")
    }
    #[doc = "SR16: Include or exclude subregion 16 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr16_write(
        &mut self,
        _pregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR16 mwrite None write None rac None reset value false")
    }
    #[doc = "SR17: Include or exclude subregion 17 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr17_read(
        &self,
        _pregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR17 mwrite None write None rac None reset value false")
    }
    #[doc = "SR17: Include or exclude subregion 17 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr17_write(
        &mut self,
        _pregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR17 mwrite None write None rac None reset value false")
    }
    #[doc = "SR18: Include or exclude subregion 18 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr18_read(
        &self,
        _pregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR18 mwrite None write None rac None reset value false")
    }
    #[doc = "SR18: Include or exclude subregion 18 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr18_write(
        &mut self,
        _pregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR18 mwrite None write None rac None reset value false")
    }
    #[doc = "SR19: Include or exclude subregion 19 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr19_read(
        &self,
        _pregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR19 mwrite None write None rac None reset value false")
    }
    #[doc = "SR19: Include or exclude subregion 19 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr19_write(
        &mut self,
        _pregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR19 mwrite None write None rac None reset value false")
    }
    #[doc = "SR20: Include or exclude subregion 20 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr20_read(
        &self,
        _pregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR20 mwrite None write None rac None reset value false")
    }
    #[doc = "SR20: Include or exclude subregion 20 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr20_write(
        &mut self,
        _pregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR20 mwrite None write None rac None reset value false")
    }
    #[doc = "SR21: Include or exclude subregion 21 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr21_read(
        &self,
        _pregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR21 mwrite None write None rac None reset value false")
    }
    #[doc = "SR21: Include or exclude subregion 21 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr21_write(
        &mut self,
        _pregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR21 mwrite None write None rac None reset value false")
    }
    #[doc = "SR22: Include or exclude subregion 22 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr22_read(
        &self,
        _pregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR22 mwrite None write None rac None reset value false")
    }
    #[doc = "SR22: Include or exclude subregion 22 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr22_write(
        &mut self,
        _pregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR22 mwrite None write None rac None reset value false")
    }
    #[doc = "SR23: Include or exclude subregion 23 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr23_read(
        &self,
        _pregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR23 mwrite None write None rac None reset value false")
    }
    #[doc = "SR23: Include or exclude subregion 23 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr23_write(
        &mut self,
        _pregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR23 mwrite None write None rac None reset value false")
    }
    #[doc = "SR24: Include or exclude subregion 24 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr24_read(
        &self,
        _pregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR24 mwrite None write None rac None reset value false")
    }
    #[doc = "SR24: Include or exclude subregion 24 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr24_write(
        &mut self,
        _pregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR24 mwrite None write None rac None reset value false")
    }
    #[doc = "SR25: Include or exclude subregion 25 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr25_read(
        &self,
        _pregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR25 mwrite None write None rac None reset value false")
    }
    #[doc = "SR25: Include or exclude subregion 25 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr25_write(
        &mut self,
        _pregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR25 mwrite None write None rac None reset value false")
    }
    #[doc = "SR26: Include or exclude subregion 26 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr26_read(
        &self,
        _pregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR26 mwrite None write None rac None reset value false")
    }
    #[doc = "SR26: Include or exclude subregion 26 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr26_write(
        &mut self,
        _pregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR26 mwrite None write None rac None reset value false")
    }
    #[doc = "SR27: Include or exclude subregion 27 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr27_read(
        &self,
        _pregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR27 mwrite None write None rac None reset value false")
    }
    #[doc = "SR27: Include or exclude subregion 27 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr27_write(
        &mut self,
        _pregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR27 mwrite None write None rac None reset value false")
    }
    #[doc = "SR28: Include or exclude subregion 28 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr28_read(
        &self,
        _pregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR28 mwrite None write None rac None reset value false")
    }
    #[doc = "SR28: Include or exclude subregion 28 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr28_write(
        &mut self,
        _pregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR28 mwrite None write None rac None reset value false")
    }
    #[doc = "SR29: Include or exclude subregion 29 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr29_read(
        &self,
        _pregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR29 mwrite None write None rac None reset value false")
    }
    #[doc = "SR29: Include or exclude subregion 29 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr29_write(
        &mut self,
        _pregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR29 mwrite None write None rac None reset value false")
    }
    #[doc = "SR30: Include or exclude subregion 30 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr30_read(
        &self,
        _pregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR30 mwrite None write None rac None reset value false")
    }
    #[doc = "SR30: Include or exclude subregion 30 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr30_write(
        &mut self,
        _pregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR30 mwrite None write None rac None reset value false")
    }
    #[doc = "SR31: Include or exclude subregion 31 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr31_read(
        &self,
        _pregionn: usize,
    ) -> MemResult<bool> {
        todo!("read SR31 mwrite None write None rac None reset value false")
    }
    #[doc = "SR31: Include or exclude subregion 31 in region<br>"]
    pub(crate) fn mwu_pregionn_subs8_sr31_write(
        &mut self,
        _pregionn: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write SR31 mwrite None write None rac None reset value false")
    }
}
