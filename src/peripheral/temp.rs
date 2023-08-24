use icicle_vm::cpu::mem::MemResult;
#[derive(Default)]
#[doc = "TEMP: Temperature Sensor<br><br>Instances:<br>0x4000c000: TEMP<br>"]
pub struct Temp {
    #[doc = "TODO: implement things here"]
    _todo: (),
}
impl Temp {
    pub(crate) fn page_to_index(page: u64) -> usize {
        match page {
            262156u64 => 0usize,
            _ => unreachable!(),
        }
    }
    #[doc = "TASKS_START: Start temperature measurement<br>"]
    pub(crate) fn temp_tasks_start0_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write temp_tasks_start0 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_STOP: Stop temperature measurement<br>"]
    pub(crate) fn temp_tasks_stop4_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write temp_tasks_stop4 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_DATARDY: Temperature measurement complete, data ready<br>"]
    pub(crate) fn temp_events_datardy100_read(&self) -> MemResult<u32> {
        todo ! ("read temp_events_datardy100 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_DATARDY: Temperature measurement complete, data ready<br>"]
    pub(crate) fn temp_events_datardy100_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write temp_events_datardy100 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "DATARDY: Write '1' to Enable interrupt for DATARDY event<br>"]
    pub(crate) fn temp_intenset304_datardy_read(&self) -> MemResult<bool> {
        todo!("read DATARDY mwrite None write None rac None reset value false")
    }
    #[doc = "DATARDY: Write '1' to Enable interrupt for DATARDY event<br>"]
    pub(crate) fn temp_intenset304_datardy_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write DATARDY mwrite None write None rac None reset value false")
    }
    #[doc = "DATARDY: Write '1' to Disable interrupt for DATARDY event<br>"]
    pub(crate) fn temp_intenclr308_datardy_read(&self) -> MemResult<bool> {
        todo!("read DATARDY mwrite None write None rac None reset value false")
    }
    #[doc = "DATARDY: Write '1' to Disable interrupt for DATARDY event<br>"]
    pub(crate) fn temp_intenclr308_datardy_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write DATARDY mwrite None write None rac None reset value false")
    }
    #[doc = "TEMP: Temperature in degC (0.25deg steps)<br>"]
    pub(crate) fn temp_temp508_temp_read(&self) -> MemResult<u32> {
        todo ! ("read TEMP mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "A0: Slope of 1st piece wise linear function<br>"]
    pub(crate) fn temp_a0520_a0_read(&self) -> MemResult<u16> {
        todo ! ("read A0 mwrite None write None rac None reset value 0x320 mask 0xfff")
    }
    #[doc = "A0: Slope of 1st piece wise linear function<br>"]
    pub(crate) fn temp_a0520_a0_write(&mut self, _value: u16) -> MemResult<()> {
        todo ! ("write A0 mwrite None write None rac None reset value 0x320 mask 0xfff")
    }
    #[doc = "A1: Slope of 2nd piece wise linear function<br>"]
    pub(crate) fn temp_a1524_a1_read(&self) -> MemResult<u16> {
        todo ! ("read A1 mwrite None write None rac None reset value 0x343 mask 0xfff")
    }
    #[doc = "A1: Slope of 2nd piece wise linear function<br>"]
    pub(crate) fn temp_a1524_a1_write(&mut self, _value: u16) -> MemResult<()> {
        todo ! ("write A1 mwrite None write None rac None reset value 0x343 mask 0xfff")
    }
    #[doc = "A2: Slope of 3rd piece wise linear function<br>"]
    pub(crate) fn temp_a2528_a2_read(&self) -> MemResult<u16> {
        todo ! ("read A2 mwrite None write None rac None reset value 0x35d mask 0xfff")
    }
    #[doc = "A2: Slope of 3rd piece wise linear function<br>"]
    pub(crate) fn temp_a2528_a2_write(&mut self, _value: u16) -> MemResult<()> {
        todo ! ("write A2 mwrite None write None rac None reset value 0x35d mask 0xfff")
    }
    #[doc = "A3: Slope of 4th piece wise linear function<br>"]
    pub(crate) fn temp_a352c_a3_read(&self) -> MemResult<u16> {
        todo ! ("read A3 mwrite None write None rac None reset value 0x400 mask 0xfff")
    }
    #[doc = "A3: Slope of 4th piece wise linear function<br>"]
    pub(crate) fn temp_a352c_a3_write(&mut self, _value: u16) -> MemResult<()> {
        todo ! ("write A3 mwrite None write None rac None reset value 0x400 mask 0xfff")
    }
    #[doc = "A4: Slope of 5th piece wise linear function<br>"]
    pub(crate) fn temp_a4530_a4_read(&self) -> MemResult<u16> {
        todo ! ("read A4 mwrite None write None rac None reset value 0x47f mask 0xfff")
    }
    #[doc = "A4: Slope of 5th piece wise linear function<br>"]
    pub(crate) fn temp_a4530_a4_write(&mut self, _value: u16) -> MemResult<()> {
        todo ! ("write A4 mwrite None write None rac None reset value 0x47f mask 0xfff")
    }
    #[doc = "A5: Slope of 6th piece wise linear function<br>"]
    pub(crate) fn temp_a5534_a5_read(&self) -> MemResult<u16> {
        todo ! ("read A5 mwrite None write None rac None reset value 0x37b mask 0xfff")
    }
    #[doc = "A5: Slope of 6th piece wise linear function<br>"]
    pub(crate) fn temp_a5534_a5_write(&mut self, _value: u16) -> MemResult<()> {
        todo ! ("write A5 mwrite None write None rac None reset value 0x37b mask 0xfff")
    }
    #[doc = "B0: y-intercept of 1st piece wise linear function<br>"]
    pub(crate) fn temp_b0540_b0_read(&self) -> MemResult<u16> {
        todo ! ("read B0 mwrite None write None rac None reset value 0x3fcc mask 0x3fff")
    }
    #[doc = "B0: y-intercept of 1st piece wise linear function<br>"]
    pub(crate) fn temp_b0540_b0_write(&mut self, _value: u16) -> MemResult<()> {
        todo ! ("write B0 mwrite None write None rac None reset value 0x3fcc mask 0x3fff")
    }
    #[doc = "B1: y-intercept of 2nd piece wise linear function<br>"]
    pub(crate) fn temp_b1544_b1_read(&self) -> MemResult<u16> {
        todo ! ("read B1 mwrite None write None rac None reset value 0x3f98 mask 0x3fff")
    }
    #[doc = "B1: y-intercept of 2nd piece wise linear function<br>"]
    pub(crate) fn temp_b1544_b1_write(&mut self, _value: u16) -> MemResult<()> {
        todo ! ("write B1 mwrite None write None rac None reset value 0x3f98 mask 0x3fff")
    }
    #[doc = "B2: y-intercept of 3rd piece wise linear function<br>"]
    pub(crate) fn temp_b2548_b2_read(&self) -> MemResult<u16> {
        todo ! ("read B2 mwrite None write None rac None reset value 0x3f98 mask 0x3fff")
    }
    #[doc = "B2: y-intercept of 3rd piece wise linear function<br>"]
    pub(crate) fn temp_b2548_b2_write(&mut self, _value: u16) -> MemResult<()> {
        todo ! ("write B2 mwrite None write None rac None reset value 0x3f98 mask 0x3fff")
    }
    #[doc = "B3: y-intercept of 4th piece wise linear function<br>"]
    pub(crate) fn temp_b354c_b3_read(&self) -> MemResult<u16> {
        todo ! ("read B3 mwrite None write None rac None reset value 0x12 mask 0x3fff")
    }
    #[doc = "B3: y-intercept of 4th piece wise linear function<br>"]
    pub(crate) fn temp_b354c_b3_write(&mut self, _value: u16) -> MemResult<()> {
        todo ! ("write B3 mwrite None write None rac None reset value 0x12 mask 0x3fff")
    }
    #[doc = "B4: y-intercept of 5th piece wise linear function<br>"]
    pub(crate) fn temp_b4550_b4_read(&self) -> MemResult<u16> {
        todo ! ("read B4 mwrite None write None rac None reset value 0x6a mask 0x3fff")
    }
    #[doc = "B4: y-intercept of 5th piece wise linear function<br>"]
    pub(crate) fn temp_b4550_b4_write(&mut self, _value: u16) -> MemResult<()> {
        todo ! ("write B4 mwrite None write None rac None reset value 0x6a mask 0x3fff")
    }
    #[doc = "B5: y-intercept of 6th piece wise linear function<br>"]
    pub(crate) fn temp_b5554_b5_read(&self) -> MemResult<u16> {
        todo ! ("read B5 mwrite None write None rac None reset value 0x3dd0 mask 0x3fff")
    }
    #[doc = "B5: y-intercept of 6th piece wise linear function<br>"]
    pub(crate) fn temp_b5554_b5_write(&mut self, _value: u16) -> MemResult<()> {
        todo ! ("write B5 mwrite None write None rac None reset value 0x3dd0 mask 0x3fff")
    }
    #[doc = "T0: End point of 1st piece wise linear function<br>"]
    pub(crate) fn temp_t0560_t0_read(&self) -> MemResult<u8> {
        todo ! ("read T0 mwrite None write None rac None reset value 0xe2 mask 0xff")
    }
    #[doc = "T0: End point of 1st piece wise linear function<br>"]
    pub(crate) fn temp_t0560_t0_write(&mut self, _value: u8) -> MemResult<()> {
        todo ! ("write T0 mwrite None write None rac None reset value 0xe2 mask 0xff")
    }
    #[doc = "T1: End point of 2nd piece wise linear function<br>"]
    pub(crate) fn temp_t1564_t1_read(&self) -> MemResult<u8> {
        todo ! ("read T1 mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "T1: End point of 2nd piece wise linear function<br>"]
    pub(crate) fn temp_t1564_t1_write(&mut self, _value: u8) -> MemResult<()> {
        todo ! ("write T1 mwrite None write None rac None reset value 0x00 mask 0xff")
    }
    #[doc = "T2: End point of 3rd piece wise linear function<br>"]
    pub(crate) fn temp_t2568_t2_read(&self) -> MemResult<u8> {
        todo ! ("read T2 mwrite None write None rac None reset value 0x14 mask 0xff")
    }
    #[doc = "T2: End point of 3rd piece wise linear function<br>"]
    pub(crate) fn temp_t2568_t2_write(&mut self, _value: u8) -> MemResult<()> {
        todo ! ("write T2 mwrite None write None rac None reset value 0x14 mask 0xff")
    }
    #[doc = "T3: End point of 4th piece wise linear function<br>"]
    pub(crate) fn temp_t356c_t3_read(&self) -> MemResult<u8> {
        todo ! ("read T3 mwrite None write None rac None reset value 0x19 mask 0xff")
    }
    #[doc = "T3: End point of 4th piece wise linear function<br>"]
    pub(crate) fn temp_t356c_t3_write(&mut self, _value: u8) -> MemResult<()> {
        todo ! ("write T3 mwrite None write None rac None reset value 0x19 mask 0xff")
    }
    #[doc = "T4: End point of 5th piece wise linear function<br>"]
    pub(crate) fn temp_t4570_t4_read(&self) -> MemResult<u8> {
        todo ! ("read T4 mwrite None write None rac None reset value 0x50 mask 0xff")
    }
    #[doc = "T4: End point of 5th piece wise linear function<br>"]
    pub(crate) fn temp_t4570_t4_write(&mut self, _value: u8) -> MemResult<()> {
        todo ! ("write T4 mwrite None write None rac None reset value 0x50 mask 0xff")
    }
}
