use icicle_vm::cpu::mem::MemResult;
#[derive(Default)]
#[doc = "P0: GPIO Port 1<br><br>Instances:<br>0x50000000: P0<br>"]
pub struct P0 {
    #[doc = "TODO: implement things here"]
    _todo: (),
}
impl P0 {
    pub(crate) fn page_to_index(page: u64) -> usize {
        match page {
            327680u64 => 0usize,
            _ => unreachable!(),
        }
    }
    #[doc = "PIN0: Pin 0<br>"]
    pub(crate) fn p0_out504_pin0_read(&self) -> MemResult<bool> {
        todo!("read PIN0 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN0: Pin 0<br>"]
    pub(crate) fn p0_out504_pin0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN0 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN1: Pin 1<br>"]
    pub(crate) fn p0_out504_pin1_read(&self) -> MemResult<bool> {
        todo!("read PIN1 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN1: Pin 1<br>"]
    pub(crate) fn p0_out504_pin1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN1 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN2: Pin 2<br>"]
    pub(crate) fn p0_out504_pin2_read(&self) -> MemResult<bool> {
        todo!("read PIN2 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN2: Pin 2<br>"]
    pub(crate) fn p0_out504_pin2_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN2 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN3: Pin 3<br>"]
    pub(crate) fn p0_out504_pin3_read(&self) -> MemResult<bool> {
        todo!("read PIN3 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN3: Pin 3<br>"]
    pub(crate) fn p0_out504_pin3_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN3 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN4: Pin 4<br>"]
    pub(crate) fn p0_out504_pin4_read(&self) -> MemResult<bool> {
        todo!("read PIN4 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN4: Pin 4<br>"]
    pub(crate) fn p0_out504_pin4_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN4 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN5: Pin 5<br>"]
    pub(crate) fn p0_out504_pin5_read(&self) -> MemResult<bool> {
        todo!("read PIN5 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN5: Pin 5<br>"]
    pub(crate) fn p0_out504_pin5_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN5 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN6: Pin 6<br>"]
    pub(crate) fn p0_out504_pin6_read(&self) -> MemResult<bool> {
        todo!("read PIN6 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN6: Pin 6<br>"]
    pub(crate) fn p0_out504_pin6_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN6 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN7: Pin 7<br>"]
    pub(crate) fn p0_out504_pin7_read(&self) -> MemResult<bool> {
        todo!("read PIN7 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN7: Pin 7<br>"]
    pub(crate) fn p0_out504_pin7_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN7 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN8: Pin 8<br>"]
    pub(crate) fn p0_out504_pin8_read(&self) -> MemResult<bool> {
        todo!("read PIN8 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN8: Pin 8<br>"]
    pub(crate) fn p0_out504_pin8_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN8 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN9: Pin 9<br>"]
    pub(crate) fn p0_out504_pin9_read(&self) -> MemResult<bool> {
        todo!("read PIN9 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN9: Pin 9<br>"]
    pub(crate) fn p0_out504_pin9_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN9 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN10: Pin 10<br>"]
    pub(crate) fn p0_out504_pin10_read(&self) -> MemResult<bool> {
        todo!("read PIN10 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN10: Pin 10<br>"]
    pub(crate) fn p0_out504_pin10_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN10 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN11: Pin 11<br>"]
    pub(crate) fn p0_out504_pin11_read(&self) -> MemResult<bool> {
        todo!("read PIN11 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN11: Pin 11<br>"]
    pub(crate) fn p0_out504_pin11_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN11 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN12: Pin 12<br>"]
    pub(crate) fn p0_out504_pin12_read(&self) -> MemResult<bool> {
        todo!("read PIN12 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN12: Pin 12<br>"]
    pub(crate) fn p0_out504_pin12_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN12 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN13: Pin 13<br>"]
    pub(crate) fn p0_out504_pin13_read(&self) -> MemResult<bool> {
        todo!("read PIN13 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN13: Pin 13<br>"]
    pub(crate) fn p0_out504_pin13_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN13 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN14: Pin 14<br>"]
    pub(crate) fn p0_out504_pin14_read(&self) -> MemResult<bool> {
        todo!("read PIN14 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN14: Pin 14<br>"]
    pub(crate) fn p0_out504_pin14_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN14 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN15: Pin 15<br>"]
    pub(crate) fn p0_out504_pin15_read(&self) -> MemResult<bool> {
        todo!("read PIN15 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN15: Pin 15<br>"]
    pub(crate) fn p0_out504_pin15_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN15 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN16: Pin 16<br>"]
    pub(crate) fn p0_out504_pin16_read(&self) -> MemResult<bool> {
        todo!("read PIN16 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN16: Pin 16<br>"]
    pub(crate) fn p0_out504_pin16_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN16 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN17: Pin 17<br>"]
    pub(crate) fn p0_out504_pin17_read(&self) -> MemResult<bool> {
        todo!("read PIN17 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN17: Pin 17<br>"]
    pub(crate) fn p0_out504_pin17_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN17 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN18: Pin 18<br>"]
    pub(crate) fn p0_out504_pin18_read(&self) -> MemResult<bool> {
        todo!("read PIN18 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN18: Pin 18<br>"]
    pub(crate) fn p0_out504_pin18_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN18 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN19: Pin 19<br>"]
    pub(crate) fn p0_out504_pin19_read(&self) -> MemResult<bool> {
        todo!("read PIN19 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN19: Pin 19<br>"]
    pub(crate) fn p0_out504_pin19_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN19 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN20: Pin 20<br>"]
    pub(crate) fn p0_out504_pin20_read(&self) -> MemResult<bool> {
        todo!("read PIN20 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN20: Pin 20<br>"]
    pub(crate) fn p0_out504_pin20_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN20 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN21: Pin 21<br>"]
    pub(crate) fn p0_out504_pin21_read(&self) -> MemResult<bool> {
        todo!("read PIN21 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN21: Pin 21<br>"]
    pub(crate) fn p0_out504_pin21_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN21 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN22: Pin 22<br>"]
    pub(crate) fn p0_out504_pin22_read(&self) -> MemResult<bool> {
        todo!("read PIN22 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN22: Pin 22<br>"]
    pub(crate) fn p0_out504_pin22_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN22 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN23: Pin 23<br>"]
    pub(crate) fn p0_out504_pin23_read(&self) -> MemResult<bool> {
        todo!("read PIN23 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN23: Pin 23<br>"]
    pub(crate) fn p0_out504_pin23_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN23 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN24: Pin 24<br>"]
    pub(crate) fn p0_out504_pin24_read(&self) -> MemResult<bool> {
        todo!("read PIN24 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN24: Pin 24<br>"]
    pub(crate) fn p0_out504_pin24_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN24 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN25: Pin 25<br>"]
    pub(crate) fn p0_out504_pin25_read(&self) -> MemResult<bool> {
        todo!("read PIN25 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN25: Pin 25<br>"]
    pub(crate) fn p0_out504_pin25_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN25 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN26: Pin 26<br>"]
    pub(crate) fn p0_out504_pin26_read(&self) -> MemResult<bool> {
        todo!("read PIN26 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN26: Pin 26<br>"]
    pub(crate) fn p0_out504_pin26_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN26 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN27: Pin 27<br>"]
    pub(crate) fn p0_out504_pin27_read(&self) -> MemResult<bool> {
        todo!("read PIN27 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN27: Pin 27<br>"]
    pub(crate) fn p0_out504_pin27_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN27 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN28: Pin 28<br>"]
    pub(crate) fn p0_out504_pin28_read(&self) -> MemResult<bool> {
        todo!("read PIN28 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN28: Pin 28<br>"]
    pub(crate) fn p0_out504_pin28_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN28 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN29: Pin 29<br>"]
    pub(crate) fn p0_out504_pin29_read(&self) -> MemResult<bool> {
        todo!("read PIN29 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN29: Pin 29<br>"]
    pub(crate) fn p0_out504_pin29_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN29 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN30: Pin 30<br>"]
    pub(crate) fn p0_out504_pin30_read(&self) -> MemResult<bool> {
        todo!("read PIN30 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN30: Pin 30<br>"]
    pub(crate) fn p0_out504_pin30_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN30 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN31: Pin 31<br>"]
    pub(crate) fn p0_out504_pin31_read(&self) -> MemResult<bool> {
        todo!("read PIN31 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN31: Pin 31<br>"]
    pub(crate) fn p0_out504_pin31_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN31 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN0: Pin 0<br>"]
    pub(crate) fn p0_outset508_pin0_read(&self) -> MemResult<bool> {
        todo!("read PIN0 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN0: Pin 0<br>"]
    pub(crate) fn p0_outset508_pin0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN0 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN1: Pin 1<br>"]
    pub(crate) fn p0_outset508_pin1_read(&self) -> MemResult<bool> {
        todo!("read PIN1 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN1: Pin 1<br>"]
    pub(crate) fn p0_outset508_pin1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN1 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN2: Pin 2<br>"]
    pub(crate) fn p0_outset508_pin2_read(&self) -> MemResult<bool> {
        todo!("read PIN2 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN2: Pin 2<br>"]
    pub(crate) fn p0_outset508_pin2_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN2 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN3: Pin 3<br>"]
    pub(crate) fn p0_outset508_pin3_read(&self) -> MemResult<bool> {
        todo!("read PIN3 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN3: Pin 3<br>"]
    pub(crate) fn p0_outset508_pin3_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN3 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN4: Pin 4<br>"]
    pub(crate) fn p0_outset508_pin4_read(&self) -> MemResult<bool> {
        todo!("read PIN4 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN4: Pin 4<br>"]
    pub(crate) fn p0_outset508_pin4_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN4 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN5: Pin 5<br>"]
    pub(crate) fn p0_outset508_pin5_read(&self) -> MemResult<bool> {
        todo!("read PIN5 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN5: Pin 5<br>"]
    pub(crate) fn p0_outset508_pin5_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN5 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN6: Pin 6<br>"]
    pub(crate) fn p0_outset508_pin6_read(&self) -> MemResult<bool> {
        todo!("read PIN6 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN6: Pin 6<br>"]
    pub(crate) fn p0_outset508_pin6_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN6 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN7: Pin 7<br>"]
    pub(crate) fn p0_outset508_pin7_read(&self) -> MemResult<bool> {
        todo!("read PIN7 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN7: Pin 7<br>"]
    pub(crate) fn p0_outset508_pin7_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN7 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN8: Pin 8<br>"]
    pub(crate) fn p0_outset508_pin8_read(&self) -> MemResult<bool> {
        todo!("read PIN8 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN8: Pin 8<br>"]
    pub(crate) fn p0_outset508_pin8_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN8 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN9: Pin 9<br>"]
    pub(crate) fn p0_outset508_pin9_read(&self) -> MemResult<bool> {
        todo!("read PIN9 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN9: Pin 9<br>"]
    pub(crate) fn p0_outset508_pin9_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN9 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN10: Pin 10<br>"]
    pub(crate) fn p0_outset508_pin10_read(&self) -> MemResult<bool> {
        todo!("read PIN10 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN10: Pin 10<br>"]
    pub(crate) fn p0_outset508_pin10_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN10 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN11: Pin 11<br>"]
    pub(crate) fn p0_outset508_pin11_read(&self) -> MemResult<bool> {
        todo!("read PIN11 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN11: Pin 11<br>"]
    pub(crate) fn p0_outset508_pin11_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN11 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN12: Pin 12<br>"]
    pub(crate) fn p0_outset508_pin12_read(&self) -> MemResult<bool> {
        todo!("read PIN12 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN12: Pin 12<br>"]
    pub(crate) fn p0_outset508_pin12_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN12 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN13: Pin 13<br>"]
    pub(crate) fn p0_outset508_pin13_read(&self) -> MemResult<bool> {
        todo!("read PIN13 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN13: Pin 13<br>"]
    pub(crate) fn p0_outset508_pin13_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN13 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN14: Pin 14<br>"]
    pub(crate) fn p0_outset508_pin14_read(&self) -> MemResult<bool> {
        todo!("read PIN14 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN14: Pin 14<br>"]
    pub(crate) fn p0_outset508_pin14_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN14 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN15: Pin 15<br>"]
    pub(crate) fn p0_outset508_pin15_read(&self) -> MemResult<bool> {
        todo!("read PIN15 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN15: Pin 15<br>"]
    pub(crate) fn p0_outset508_pin15_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN15 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN16: Pin 16<br>"]
    pub(crate) fn p0_outset508_pin16_read(&self) -> MemResult<bool> {
        todo!("read PIN16 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN16: Pin 16<br>"]
    pub(crate) fn p0_outset508_pin16_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN16 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN17: Pin 17<br>"]
    pub(crate) fn p0_outset508_pin17_read(&self) -> MemResult<bool> {
        todo!("read PIN17 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN17: Pin 17<br>"]
    pub(crate) fn p0_outset508_pin17_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN17 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN18: Pin 18<br>"]
    pub(crate) fn p0_outset508_pin18_read(&self) -> MemResult<bool> {
        todo!("read PIN18 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN18: Pin 18<br>"]
    pub(crate) fn p0_outset508_pin18_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN18 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN19: Pin 19<br>"]
    pub(crate) fn p0_outset508_pin19_read(&self) -> MemResult<bool> {
        todo!("read PIN19 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN19: Pin 19<br>"]
    pub(crate) fn p0_outset508_pin19_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN19 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN20: Pin 20<br>"]
    pub(crate) fn p0_outset508_pin20_read(&self) -> MemResult<bool> {
        todo!("read PIN20 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN20: Pin 20<br>"]
    pub(crate) fn p0_outset508_pin20_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN20 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN21: Pin 21<br>"]
    pub(crate) fn p0_outset508_pin21_read(&self) -> MemResult<bool> {
        todo!("read PIN21 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN21: Pin 21<br>"]
    pub(crate) fn p0_outset508_pin21_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN21 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN22: Pin 22<br>"]
    pub(crate) fn p0_outset508_pin22_read(&self) -> MemResult<bool> {
        todo!("read PIN22 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN22: Pin 22<br>"]
    pub(crate) fn p0_outset508_pin22_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN22 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN23: Pin 23<br>"]
    pub(crate) fn p0_outset508_pin23_read(&self) -> MemResult<bool> {
        todo!("read PIN23 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN23: Pin 23<br>"]
    pub(crate) fn p0_outset508_pin23_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN23 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN24: Pin 24<br>"]
    pub(crate) fn p0_outset508_pin24_read(&self) -> MemResult<bool> {
        todo!("read PIN24 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN24: Pin 24<br>"]
    pub(crate) fn p0_outset508_pin24_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN24 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN25: Pin 25<br>"]
    pub(crate) fn p0_outset508_pin25_read(&self) -> MemResult<bool> {
        todo!("read PIN25 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN25: Pin 25<br>"]
    pub(crate) fn p0_outset508_pin25_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN25 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN26: Pin 26<br>"]
    pub(crate) fn p0_outset508_pin26_read(&self) -> MemResult<bool> {
        todo!("read PIN26 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN26: Pin 26<br>"]
    pub(crate) fn p0_outset508_pin26_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN26 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN27: Pin 27<br>"]
    pub(crate) fn p0_outset508_pin27_read(&self) -> MemResult<bool> {
        todo!("read PIN27 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN27: Pin 27<br>"]
    pub(crate) fn p0_outset508_pin27_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN27 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN28: Pin 28<br>"]
    pub(crate) fn p0_outset508_pin28_read(&self) -> MemResult<bool> {
        todo!("read PIN28 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN28: Pin 28<br>"]
    pub(crate) fn p0_outset508_pin28_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN28 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN29: Pin 29<br>"]
    pub(crate) fn p0_outset508_pin29_read(&self) -> MemResult<bool> {
        todo!("read PIN29 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN29: Pin 29<br>"]
    pub(crate) fn p0_outset508_pin29_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN29 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN30: Pin 30<br>"]
    pub(crate) fn p0_outset508_pin30_read(&self) -> MemResult<bool> {
        todo!("read PIN30 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN30: Pin 30<br>"]
    pub(crate) fn p0_outset508_pin30_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN30 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN31: Pin 31<br>"]
    pub(crate) fn p0_outset508_pin31_read(&self) -> MemResult<bool> {
        todo!("read PIN31 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN31: Pin 31<br>"]
    pub(crate) fn p0_outset508_pin31_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN31 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN0: Pin 0<br>"]
    pub(crate) fn p0_outclr50c_pin0_read(&self) -> MemResult<bool> {
        todo!("read PIN0 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN0: Pin 0<br>"]
    pub(crate) fn p0_outclr50c_pin0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN0 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN1: Pin 1<br>"]
    pub(crate) fn p0_outclr50c_pin1_read(&self) -> MemResult<bool> {
        todo!("read PIN1 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN1: Pin 1<br>"]
    pub(crate) fn p0_outclr50c_pin1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN1 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN2: Pin 2<br>"]
    pub(crate) fn p0_outclr50c_pin2_read(&self) -> MemResult<bool> {
        todo!("read PIN2 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN2: Pin 2<br>"]
    pub(crate) fn p0_outclr50c_pin2_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN2 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN3: Pin 3<br>"]
    pub(crate) fn p0_outclr50c_pin3_read(&self) -> MemResult<bool> {
        todo!("read PIN3 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN3: Pin 3<br>"]
    pub(crate) fn p0_outclr50c_pin3_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN3 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN4: Pin 4<br>"]
    pub(crate) fn p0_outclr50c_pin4_read(&self) -> MemResult<bool> {
        todo!("read PIN4 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN4: Pin 4<br>"]
    pub(crate) fn p0_outclr50c_pin4_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN4 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN5: Pin 5<br>"]
    pub(crate) fn p0_outclr50c_pin5_read(&self) -> MemResult<bool> {
        todo!("read PIN5 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN5: Pin 5<br>"]
    pub(crate) fn p0_outclr50c_pin5_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN5 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN6: Pin 6<br>"]
    pub(crate) fn p0_outclr50c_pin6_read(&self) -> MemResult<bool> {
        todo!("read PIN6 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN6: Pin 6<br>"]
    pub(crate) fn p0_outclr50c_pin6_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN6 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN7: Pin 7<br>"]
    pub(crate) fn p0_outclr50c_pin7_read(&self) -> MemResult<bool> {
        todo!("read PIN7 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN7: Pin 7<br>"]
    pub(crate) fn p0_outclr50c_pin7_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN7 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN8: Pin 8<br>"]
    pub(crate) fn p0_outclr50c_pin8_read(&self) -> MemResult<bool> {
        todo!("read PIN8 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN8: Pin 8<br>"]
    pub(crate) fn p0_outclr50c_pin8_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN8 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN9: Pin 9<br>"]
    pub(crate) fn p0_outclr50c_pin9_read(&self) -> MemResult<bool> {
        todo!("read PIN9 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN9: Pin 9<br>"]
    pub(crate) fn p0_outclr50c_pin9_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN9 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN10: Pin 10<br>"]
    pub(crate) fn p0_outclr50c_pin10_read(&self) -> MemResult<bool> {
        todo!("read PIN10 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN10: Pin 10<br>"]
    pub(crate) fn p0_outclr50c_pin10_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN10 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN11: Pin 11<br>"]
    pub(crate) fn p0_outclr50c_pin11_read(&self) -> MemResult<bool> {
        todo!("read PIN11 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN11: Pin 11<br>"]
    pub(crate) fn p0_outclr50c_pin11_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN11 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN12: Pin 12<br>"]
    pub(crate) fn p0_outclr50c_pin12_read(&self) -> MemResult<bool> {
        todo!("read PIN12 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN12: Pin 12<br>"]
    pub(crate) fn p0_outclr50c_pin12_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN12 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN13: Pin 13<br>"]
    pub(crate) fn p0_outclr50c_pin13_read(&self) -> MemResult<bool> {
        todo!("read PIN13 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN13: Pin 13<br>"]
    pub(crate) fn p0_outclr50c_pin13_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN13 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN14: Pin 14<br>"]
    pub(crate) fn p0_outclr50c_pin14_read(&self) -> MemResult<bool> {
        todo!("read PIN14 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN14: Pin 14<br>"]
    pub(crate) fn p0_outclr50c_pin14_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN14 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN15: Pin 15<br>"]
    pub(crate) fn p0_outclr50c_pin15_read(&self) -> MemResult<bool> {
        todo!("read PIN15 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN15: Pin 15<br>"]
    pub(crate) fn p0_outclr50c_pin15_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN15 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN16: Pin 16<br>"]
    pub(crate) fn p0_outclr50c_pin16_read(&self) -> MemResult<bool> {
        todo!("read PIN16 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN16: Pin 16<br>"]
    pub(crate) fn p0_outclr50c_pin16_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN16 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN17: Pin 17<br>"]
    pub(crate) fn p0_outclr50c_pin17_read(&self) -> MemResult<bool> {
        todo!("read PIN17 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN17: Pin 17<br>"]
    pub(crate) fn p0_outclr50c_pin17_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN17 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN18: Pin 18<br>"]
    pub(crate) fn p0_outclr50c_pin18_read(&self) -> MemResult<bool> {
        todo!("read PIN18 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN18: Pin 18<br>"]
    pub(crate) fn p0_outclr50c_pin18_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN18 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN19: Pin 19<br>"]
    pub(crate) fn p0_outclr50c_pin19_read(&self) -> MemResult<bool> {
        todo!("read PIN19 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN19: Pin 19<br>"]
    pub(crate) fn p0_outclr50c_pin19_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN19 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN20: Pin 20<br>"]
    pub(crate) fn p0_outclr50c_pin20_read(&self) -> MemResult<bool> {
        todo!("read PIN20 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN20: Pin 20<br>"]
    pub(crate) fn p0_outclr50c_pin20_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN20 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN21: Pin 21<br>"]
    pub(crate) fn p0_outclr50c_pin21_read(&self) -> MemResult<bool> {
        todo!("read PIN21 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN21: Pin 21<br>"]
    pub(crate) fn p0_outclr50c_pin21_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN21 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN22: Pin 22<br>"]
    pub(crate) fn p0_outclr50c_pin22_read(&self) -> MemResult<bool> {
        todo!("read PIN22 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN22: Pin 22<br>"]
    pub(crate) fn p0_outclr50c_pin22_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN22 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN23: Pin 23<br>"]
    pub(crate) fn p0_outclr50c_pin23_read(&self) -> MemResult<bool> {
        todo!("read PIN23 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN23: Pin 23<br>"]
    pub(crate) fn p0_outclr50c_pin23_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN23 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN24: Pin 24<br>"]
    pub(crate) fn p0_outclr50c_pin24_read(&self) -> MemResult<bool> {
        todo!("read PIN24 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN24: Pin 24<br>"]
    pub(crate) fn p0_outclr50c_pin24_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN24 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN25: Pin 25<br>"]
    pub(crate) fn p0_outclr50c_pin25_read(&self) -> MemResult<bool> {
        todo!("read PIN25 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN25: Pin 25<br>"]
    pub(crate) fn p0_outclr50c_pin25_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN25 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN26: Pin 26<br>"]
    pub(crate) fn p0_outclr50c_pin26_read(&self) -> MemResult<bool> {
        todo!("read PIN26 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN26: Pin 26<br>"]
    pub(crate) fn p0_outclr50c_pin26_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN26 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN27: Pin 27<br>"]
    pub(crate) fn p0_outclr50c_pin27_read(&self) -> MemResult<bool> {
        todo!("read PIN27 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN27: Pin 27<br>"]
    pub(crate) fn p0_outclr50c_pin27_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN27 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN28: Pin 28<br>"]
    pub(crate) fn p0_outclr50c_pin28_read(&self) -> MemResult<bool> {
        todo!("read PIN28 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN28: Pin 28<br>"]
    pub(crate) fn p0_outclr50c_pin28_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN28 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN29: Pin 29<br>"]
    pub(crate) fn p0_outclr50c_pin29_read(&self) -> MemResult<bool> {
        todo!("read PIN29 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN29: Pin 29<br>"]
    pub(crate) fn p0_outclr50c_pin29_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN29 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN30: Pin 30<br>"]
    pub(crate) fn p0_outclr50c_pin30_read(&self) -> MemResult<bool> {
        todo!("read PIN30 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN30: Pin 30<br>"]
    pub(crate) fn p0_outclr50c_pin30_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN30 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN31: Pin 31<br>"]
    pub(crate) fn p0_outclr50c_pin31_read(&self) -> MemResult<bool> {
        todo!("read PIN31 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN31: Pin 31<br>"]
    pub(crate) fn p0_outclr50c_pin31_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN31 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN0: Pin 0<br>"]
    pub(crate) fn p0_in510_pin0_read(&self) -> MemResult<bool> {
        todo!("read PIN0 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN1: Pin 1<br>"]
    pub(crate) fn p0_in510_pin1_read(&self) -> MemResult<bool> {
        todo!("read PIN1 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN2: Pin 2<br>"]
    pub(crate) fn p0_in510_pin2_read(&self) -> MemResult<bool> {
        todo!("read PIN2 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN3: Pin 3<br>"]
    pub(crate) fn p0_in510_pin3_read(&self) -> MemResult<bool> {
        todo!("read PIN3 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN4: Pin 4<br>"]
    pub(crate) fn p0_in510_pin4_read(&self) -> MemResult<bool> {
        todo!("read PIN4 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN5: Pin 5<br>"]
    pub(crate) fn p0_in510_pin5_read(&self) -> MemResult<bool> {
        todo!("read PIN5 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN6: Pin 6<br>"]
    pub(crate) fn p0_in510_pin6_read(&self) -> MemResult<bool> {
        todo!("read PIN6 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN7: Pin 7<br>"]
    pub(crate) fn p0_in510_pin7_read(&self) -> MemResult<bool> {
        todo!("read PIN7 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN8: Pin 8<br>"]
    pub(crate) fn p0_in510_pin8_read(&self) -> MemResult<bool> {
        todo!("read PIN8 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN9: Pin 9<br>"]
    pub(crate) fn p0_in510_pin9_read(&self) -> MemResult<bool> {
        todo!("read PIN9 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN10: Pin 10<br>"]
    pub(crate) fn p0_in510_pin10_read(&self) -> MemResult<bool> {
        todo!("read PIN10 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN11: Pin 11<br>"]
    pub(crate) fn p0_in510_pin11_read(&self) -> MemResult<bool> {
        todo!("read PIN11 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN12: Pin 12<br>"]
    pub(crate) fn p0_in510_pin12_read(&self) -> MemResult<bool> {
        todo!("read PIN12 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN13: Pin 13<br>"]
    pub(crate) fn p0_in510_pin13_read(&self) -> MemResult<bool> {
        todo!("read PIN13 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN14: Pin 14<br>"]
    pub(crate) fn p0_in510_pin14_read(&self) -> MemResult<bool> {
        todo!("read PIN14 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN15: Pin 15<br>"]
    pub(crate) fn p0_in510_pin15_read(&self) -> MemResult<bool> {
        todo!("read PIN15 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN16: Pin 16<br>"]
    pub(crate) fn p0_in510_pin16_read(&self) -> MemResult<bool> {
        todo!("read PIN16 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN17: Pin 17<br>"]
    pub(crate) fn p0_in510_pin17_read(&self) -> MemResult<bool> {
        todo!("read PIN17 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN18: Pin 18<br>"]
    pub(crate) fn p0_in510_pin18_read(&self) -> MemResult<bool> {
        todo!("read PIN18 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN19: Pin 19<br>"]
    pub(crate) fn p0_in510_pin19_read(&self) -> MemResult<bool> {
        todo!("read PIN19 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN20: Pin 20<br>"]
    pub(crate) fn p0_in510_pin20_read(&self) -> MemResult<bool> {
        todo!("read PIN20 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN21: Pin 21<br>"]
    pub(crate) fn p0_in510_pin21_read(&self) -> MemResult<bool> {
        todo!("read PIN21 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN22: Pin 22<br>"]
    pub(crate) fn p0_in510_pin22_read(&self) -> MemResult<bool> {
        todo!("read PIN22 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN23: Pin 23<br>"]
    pub(crate) fn p0_in510_pin23_read(&self) -> MemResult<bool> {
        todo!("read PIN23 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN24: Pin 24<br>"]
    pub(crate) fn p0_in510_pin24_read(&self) -> MemResult<bool> {
        todo!("read PIN24 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN25: Pin 25<br>"]
    pub(crate) fn p0_in510_pin25_read(&self) -> MemResult<bool> {
        todo!("read PIN25 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN26: Pin 26<br>"]
    pub(crate) fn p0_in510_pin26_read(&self) -> MemResult<bool> {
        todo!("read PIN26 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN27: Pin 27<br>"]
    pub(crate) fn p0_in510_pin27_read(&self) -> MemResult<bool> {
        todo!("read PIN27 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN28: Pin 28<br>"]
    pub(crate) fn p0_in510_pin28_read(&self) -> MemResult<bool> {
        todo!("read PIN28 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN29: Pin 29<br>"]
    pub(crate) fn p0_in510_pin29_read(&self) -> MemResult<bool> {
        todo!("read PIN29 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN30: Pin 30<br>"]
    pub(crate) fn p0_in510_pin30_read(&self) -> MemResult<bool> {
        todo!("read PIN30 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN31: Pin 31<br>"]
    pub(crate) fn p0_in510_pin31_read(&self) -> MemResult<bool> {
        todo!("read PIN31 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN0: Pin 0<br>"]
    pub(crate) fn p0_dir514_pin0_read(&self) -> MemResult<bool> {
        todo!("read PIN0 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN0: Pin 0<br>"]
    pub(crate) fn p0_dir514_pin0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN0 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN1: Pin 1<br>"]
    pub(crate) fn p0_dir514_pin1_read(&self) -> MemResult<bool> {
        todo!("read PIN1 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN1: Pin 1<br>"]
    pub(crate) fn p0_dir514_pin1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN1 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN2: Pin 2<br>"]
    pub(crate) fn p0_dir514_pin2_read(&self) -> MemResult<bool> {
        todo!("read PIN2 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN2: Pin 2<br>"]
    pub(crate) fn p0_dir514_pin2_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN2 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN3: Pin 3<br>"]
    pub(crate) fn p0_dir514_pin3_read(&self) -> MemResult<bool> {
        todo!("read PIN3 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN3: Pin 3<br>"]
    pub(crate) fn p0_dir514_pin3_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN3 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN4: Pin 4<br>"]
    pub(crate) fn p0_dir514_pin4_read(&self) -> MemResult<bool> {
        todo!("read PIN4 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN4: Pin 4<br>"]
    pub(crate) fn p0_dir514_pin4_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN4 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN5: Pin 5<br>"]
    pub(crate) fn p0_dir514_pin5_read(&self) -> MemResult<bool> {
        todo!("read PIN5 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN5: Pin 5<br>"]
    pub(crate) fn p0_dir514_pin5_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN5 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN6: Pin 6<br>"]
    pub(crate) fn p0_dir514_pin6_read(&self) -> MemResult<bool> {
        todo!("read PIN6 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN6: Pin 6<br>"]
    pub(crate) fn p0_dir514_pin6_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN6 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN7: Pin 7<br>"]
    pub(crate) fn p0_dir514_pin7_read(&self) -> MemResult<bool> {
        todo!("read PIN7 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN7: Pin 7<br>"]
    pub(crate) fn p0_dir514_pin7_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN7 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN8: Pin 8<br>"]
    pub(crate) fn p0_dir514_pin8_read(&self) -> MemResult<bool> {
        todo!("read PIN8 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN8: Pin 8<br>"]
    pub(crate) fn p0_dir514_pin8_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN8 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN9: Pin 9<br>"]
    pub(crate) fn p0_dir514_pin9_read(&self) -> MemResult<bool> {
        todo!("read PIN9 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN9: Pin 9<br>"]
    pub(crate) fn p0_dir514_pin9_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN9 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN10: Pin 10<br>"]
    pub(crate) fn p0_dir514_pin10_read(&self) -> MemResult<bool> {
        todo!("read PIN10 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN10: Pin 10<br>"]
    pub(crate) fn p0_dir514_pin10_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN10 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN11: Pin 11<br>"]
    pub(crate) fn p0_dir514_pin11_read(&self) -> MemResult<bool> {
        todo!("read PIN11 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN11: Pin 11<br>"]
    pub(crate) fn p0_dir514_pin11_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN11 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN12: Pin 12<br>"]
    pub(crate) fn p0_dir514_pin12_read(&self) -> MemResult<bool> {
        todo!("read PIN12 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN12: Pin 12<br>"]
    pub(crate) fn p0_dir514_pin12_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN12 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN13: Pin 13<br>"]
    pub(crate) fn p0_dir514_pin13_read(&self) -> MemResult<bool> {
        todo!("read PIN13 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN13: Pin 13<br>"]
    pub(crate) fn p0_dir514_pin13_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN13 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN14: Pin 14<br>"]
    pub(crate) fn p0_dir514_pin14_read(&self) -> MemResult<bool> {
        todo!("read PIN14 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN14: Pin 14<br>"]
    pub(crate) fn p0_dir514_pin14_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN14 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN15: Pin 15<br>"]
    pub(crate) fn p0_dir514_pin15_read(&self) -> MemResult<bool> {
        todo!("read PIN15 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN15: Pin 15<br>"]
    pub(crate) fn p0_dir514_pin15_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN15 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN16: Pin 16<br>"]
    pub(crate) fn p0_dir514_pin16_read(&self) -> MemResult<bool> {
        todo!("read PIN16 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN16: Pin 16<br>"]
    pub(crate) fn p0_dir514_pin16_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN16 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN17: Pin 17<br>"]
    pub(crate) fn p0_dir514_pin17_read(&self) -> MemResult<bool> {
        todo!("read PIN17 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN17: Pin 17<br>"]
    pub(crate) fn p0_dir514_pin17_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN17 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN18: Pin 18<br>"]
    pub(crate) fn p0_dir514_pin18_read(&self) -> MemResult<bool> {
        todo!("read PIN18 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN18: Pin 18<br>"]
    pub(crate) fn p0_dir514_pin18_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN18 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN19: Pin 19<br>"]
    pub(crate) fn p0_dir514_pin19_read(&self) -> MemResult<bool> {
        todo!("read PIN19 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN19: Pin 19<br>"]
    pub(crate) fn p0_dir514_pin19_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN19 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN20: Pin 20<br>"]
    pub(crate) fn p0_dir514_pin20_read(&self) -> MemResult<bool> {
        todo!("read PIN20 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN20: Pin 20<br>"]
    pub(crate) fn p0_dir514_pin20_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN20 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN21: Pin 21<br>"]
    pub(crate) fn p0_dir514_pin21_read(&self) -> MemResult<bool> {
        todo!("read PIN21 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN21: Pin 21<br>"]
    pub(crate) fn p0_dir514_pin21_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN21 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN22: Pin 22<br>"]
    pub(crate) fn p0_dir514_pin22_read(&self) -> MemResult<bool> {
        todo!("read PIN22 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN22: Pin 22<br>"]
    pub(crate) fn p0_dir514_pin22_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN22 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN23: Pin 23<br>"]
    pub(crate) fn p0_dir514_pin23_read(&self) -> MemResult<bool> {
        todo!("read PIN23 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN23: Pin 23<br>"]
    pub(crate) fn p0_dir514_pin23_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN23 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN24: Pin 24<br>"]
    pub(crate) fn p0_dir514_pin24_read(&self) -> MemResult<bool> {
        todo!("read PIN24 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN24: Pin 24<br>"]
    pub(crate) fn p0_dir514_pin24_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN24 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN25: Pin 25<br>"]
    pub(crate) fn p0_dir514_pin25_read(&self) -> MemResult<bool> {
        todo!("read PIN25 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN25: Pin 25<br>"]
    pub(crate) fn p0_dir514_pin25_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN25 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN26: Pin 26<br>"]
    pub(crate) fn p0_dir514_pin26_read(&self) -> MemResult<bool> {
        todo!("read PIN26 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN26: Pin 26<br>"]
    pub(crate) fn p0_dir514_pin26_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN26 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN27: Pin 27<br>"]
    pub(crate) fn p0_dir514_pin27_read(&self) -> MemResult<bool> {
        todo!("read PIN27 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN27: Pin 27<br>"]
    pub(crate) fn p0_dir514_pin27_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN27 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN28: Pin 28<br>"]
    pub(crate) fn p0_dir514_pin28_read(&self) -> MemResult<bool> {
        todo!("read PIN28 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN28: Pin 28<br>"]
    pub(crate) fn p0_dir514_pin28_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN28 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN29: Pin 29<br>"]
    pub(crate) fn p0_dir514_pin29_read(&self) -> MemResult<bool> {
        todo!("read PIN29 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN29: Pin 29<br>"]
    pub(crate) fn p0_dir514_pin29_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN29 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN30: Pin 30<br>"]
    pub(crate) fn p0_dir514_pin30_read(&self) -> MemResult<bool> {
        todo!("read PIN30 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN30: Pin 30<br>"]
    pub(crate) fn p0_dir514_pin30_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN30 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN31: Pin 31<br>"]
    pub(crate) fn p0_dir514_pin31_read(&self) -> MemResult<bool> {
        todo!("read PIN31 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN31: Pin 31<br>"]
    pub(crate) fn p0_dir514_pin31_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN31 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN0: Set as output pin 0<br>"]
    pub(crate) fn p0_dirset518_pin0_read(&self) -> MemResult<bool> {
        todo!("read PIN0 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN0: Set as output pin 0<br>"]
    pub(crate) fn p0_dirset518_pin0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN0 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN1: Set as output pin 1<br>"]
    pub(crate) fn p0_dirset518_pin1_read(&self) -> MemResult<bool> {
        todo!("read PIN1 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN1: Set as output pin 1<br>"]
    pub(crate) fn p0_dirset518_pin1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN1 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN2: Set as output pin 2<br>"]
    pub(crate) fn p0_dirset518_pin2_read(&self) -> MemResult<bool> {
        todo!("read PIN2 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN2: Set as output pin 2<br>"]
    pub(crate) fn p0_dirset518_pin2_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN2 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN3: Set as output pin 3<br>"]
    pub(crate) fn p0_dirset518_pin3_read(&self) -> MemResult<bool> {
        todo!("read PIN3 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN3: Set as output pin 3<br>"]
    pub(crate) fn p0_dirset518_pin3_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN3 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN4: Set as output pin 4<br>"]
    pub(crate) fn p0_dirset518_pin4_read(&self) -> MemResult<bool> {
        todo!("read PIN4 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN4: Set as output pin 4<br>"]
    pub(crate) fn p0_dirset518_pin4_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN4 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN5: Set as output pin 5<br>"]
    pub(crate) fn p0_dirset518_pin5_read(&self) -> MemResult<bool> {
        todo!("read PIN5 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN5: Set as output pin 5<br>"]
    pub(crate) fn p0_dirset518_pin5_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN5 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN6: Set as output pin 6<br>"]
    pub(crate) fn p0_dirset518_pin6_read(&self) -> MemResult<bool> {
        todo!("read PIN6 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN6: Set as output pin 6<br>"]
    pub(crate) fn p0_dirset518_pin6_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN6 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN7: Set as output pin 7<br>"]
    pub(crate) fn p0_dirset518_pin7_read(&self) -> MemResult<bool> {
        todo!("read PIN7 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN7: Set as output pin 7<br>"]
    pub(crate) fn p0_dirset518_pin7_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN7 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN8: Set as output pin 8<br>"]
    pub(crate) fn p0_dirset518_pin8_read(&self) -> MemResult<bool> {
        todo!("read PIN8 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN8: Set as output pin 8<br>"]
    pub(crate) fn p0_dirset518_pin8_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN8 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN9: Set as output pin 9<br>"]
    pub(crate) fn p0_dirset518_pin9_read(&self) -> MemResult<bool> {
        todo!("read PIN9 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN9: Set as output pin 9<br>"]
    pub(crate) fn p0_dirset518_pin9_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN9 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN10: Set as output pin 10<br>"]
    pub(crate) fn p0_dirset518_pin10_read(&self) -> MemResult<bool> {
        todo!("read PIN10 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN10: Set as output pin 10<br>"]
    pub(crate) fn p0_dirset518_pin10_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN10 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN11: Set as output pin 11<br>"]
    pub(crate) fn p0_dirset518_pin11_read(&self) -> MemResult<bool> {
        todo!("read PIN11 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN11: Set as output pin 11<br>"]
    pub(crate) fn p0_dirset518_pin11_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN11 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN12: Set as output pin 12<br>"]
    pub(crate) fn p0_dirset518_pin12_read(&self) -> MemResult<bool> {
        todo!("read PIN12 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN12: Set as output pin 12<br>"]
    pub(crate) fn p0_dirset518_pin12_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN12 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN13: Set as output pin 13<br>"]
    pub(crate) fn p0_dirset518_pin13_read(&self) -> MemResult<bool> {
        todo!("read PIN13 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN13: Set as output pin 13<br>"]
    pub(crate) fn p0_dirset518_pin13_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN13 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN14: Set as output pin 14<br>"]
    pub(crate) fn p0_dirset518_pin14_read(&self) -> MemResult<bool> {
        todo!("read PIN14 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN14: Set as output pin 14<br>"]
    pub(crate) fn p0_dirset518_pin14_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN14 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN15: Set as output pin 15<br>"]
    pub(crate) fn p0_dirset518_pin15_read(&self) -> MemResult<bool> {
        todo!("read PIN15 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN15: Set as output pin 15<br>"]
    pub(crate) fn p0_dirset518_pin15_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN15 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN16: Set as output pin 16<br>"]
    pub(crate) fn p0_dirset518_pin16_read(&self) -> MemResult<bool> {
        todo!("read PIN16 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN16: Set as output pin 16<br>"]
    pub(crate) fn p0_dirset518_pin16_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN16 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN17: Set as output pin 17<br>"]
    pub(crate) fn p0_dirset518_pin17_read(&self) -> MemResult<bool> {
        todo!("read PIN17 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN17: Set as output pin 17<br>"]
    pub(crate) fn p0_dirset518_pin17_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN17 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN18: Set as output pin 18<br>"]
    pub(crate) fn p0_dirset518_pin18_read(&self) -> MemResult<bool> {
        todo!("read PIN18 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN18: Set as output pin 18<br>"]
    pub(crate) fn p0_dirset518_pin18_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN18 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN19: Set as output pin 19<br>"]
    pub(crate) fn p0_dirset518_pin19_read(&self) -> MemResult<bool> {
        todo!("read PIN19 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN19: Set as output pin 19<br>"]
    pub(crate) fn p0_dirset518_pin19_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN19 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN20: Set as output pin 20<br>"]
    pub(crate) fn p0_dirset518_pin20_read(&self) -> MemResult<bool> {
        todo!("read PIN20 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN20: Set as output pin 20<br>"]
    pub(crate) fn p0_dirset518_pin20_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN20 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN21: Set as output pin 21<br>"]
    pub(crate) fn p0_dirset518_pin21_read(&self) -> MemResult<bool> {
        todo!("read PIN21 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN21: Set as output pin 21<br>"]
    pub(crate) fn p0_dirset518_pin21_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN21 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN22: Set as output pin 22<br>"]
    pub(crate) fn p0_dirset518_pin22_read(&self) -> MemResult<bool> {
        todo!("read PIN22 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN22: Set as output pin 22<br>"]
    pub(crate) fn p0_dirset518_pin22_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN22 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN23: Set as output pin 23<br>"]
    pub(crate) fn p0_dirset518_pin23_read(&self) -> MemResult<bool> {
        todo!("read PIN23 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN23: Set as output pin 23<br>"]
    pub(crate) fn p0_dirset518_pin23_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN23 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN24: Set as output pin 24<br>"]
    pub(crate) fn p0_dirset518_pin24_read(&self) -> MemResult<bool> {
        todo!("read PIN24 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN24: Set as output pin 24<br>"]
    pub(crate) fn p0_dirset518_pin24_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN24 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN25: Set as output pin 25<br>"]
    pub(crate) fn p0_dirset518_pin25_read(&self) -> MemResult<bool> {
        todo!("read PIN25 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN25: Set as output pin 25<br>"]
    pub(crate) fn p0_dirset518_pin25_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN25 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN26: Set as output pin 26<br>"]
    pub(crate) fn p0_dirset518_pin26_read(&self) -> MemResult<bool> {
        todo!("read PIN26 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN26: Set as output pin 26<br>"]
    pub(crate) fn p0_dirset518_pin26_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN26 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN27: Set as output pin 27<br>"]
    pub(crate) fn p0_dirset518_pin27_read(&self) -> MemResult<bool> {
        todo!("read PIN27 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN27: Set as output pin 27<br>"]
    pub(crate) fn p0_dirset518_pin27_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN27 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN28: Set as output pin 28<br>"]
    pub(crate) fn p0_dirset518_pin28_read(&self) -> MemResult<bool> {
        todo!("read PIN28 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN28: Set as output pin 28<br>"]
    pub(crate) fn p0_dirset518_pin28_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN28 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN29: Set as output pin 29<br>"]
    pub(crate) fn p0_dirset518_pin29_read(&self) -> MemResult<bool> {
        todo!("read PIN29 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN29: Set as output pin 29<br>"]
    pub(crate) fn p0_dirset518_pin29_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN29 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN30: Set as output pin 30<br>"]
    pub(crate) fn p0_dirset518_pin30_read(&self) -> MemResult<bool> {
        todo!("read PIN30 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN30: Set as output pin 30<br>"]
    pub(crate) fn p0_dirset518_pin30_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN30 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN31: Set as output pin 31<br>"]
    pub(crate) fn p0_dirset518_pin31_read(&self) -> MemResult<bool> {
        todo!("read PIN31 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN31: Set as output pin 31<br>"]
    pub(crate) fn p0_dirset518_pin31_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN31 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN0: Set as input pin 0<br>"]
    pub(crate) fn p0_dirclr51c_pin0_read(&self) -> MemResult<bool> {
        todo!("read PIN0 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN0: Set as input pin 0<br>"]
    pub(crate) fn p0_dirclr51c_pin0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN0 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN1: Set as input pin 1<br>"]
    pub(crate) fn p0_dirclr51c_pin1_read(&self) -> MemResult<bool> {
        todo!("read PIN1 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN1: Set as input pin 1<br>"]
    pub(crate) fn p0_dirclr51c_pin1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN1 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN2: Set as input pin 2<br>"]
    pub(crate) fn p0_dirclr51c_pin2_read(&self) -> MemResult<bool> {
        todo!("read PIN2 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN2: Set as input pin 2<br>"]
    pub(crate) fn p0_dirclr51c_pin2_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN2 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN3: Set as input pin 3<br>"]
    pub(crate) fn p0_dirclr51c_pin3_read(&self) -> MemResult<bool> {
        todo!("read PIN3 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN3: Set as input pin 3<br>"]
    pub(crate) fn p0_dirclr51c_pin3_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN3 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN4: Set as input pin 4<br>"]
    pub(crate) fn p0_dirclr51c_pin4_read(&self) -> MemResult<bool> {
        todo!("read PIN4 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN4: Set as input pin 4<br>"]
    pub(crate) fn p0_dirclr51c_pin4_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN4 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN5: Set as input pin 5<br>"]
    pub(crate) fn p0_dirclr51c_pin5_read(&self) -> MemResult<bool> {
        todo!("read PIN5 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN5: Set as input pin 5<br>"]
    pub(crate) fn p0_dirclr51c_pin5_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN5 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN6: Set as input pin 6<br>"]
    pub(crate) fn p0_dirclr51c_pin6_read(&self) -> MemResult<bool> {
        todo!("read PIN6 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN6: Set as input pin 6<br>"]
    pub(crate) fn p0_dirclr51c_pin6_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN6 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN7: Set as input pin 7<br>"]
    pub(crate) fn p0_dirclr51c_pin7_read(&self) -> MemResult<bool> {
        todo!("read PIN7 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN7: Set as input pin 7<br>"]
    pub(crate) fn p0_dirclr51c_pin7_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN7 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN8: Set as input pin 8<br>"]
    pub(crate) fn p0_dirclr51c_pin8_read(&self) -> MemResult<bool> {
        todo!("read PIN8 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN8: Set as input pin 8<br>"]
    pub(crate) fn p0_dirclr51c_pin8_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN8 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN9: Set as input pin 9<br>"]
    pub(crate) fn p0_dirclr51c_pin9_read(&self) -> MemResult<bool> {
        todo!("read PIN9 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN9: Set as input pin 9<br>"]
    pub(crate) fn p0_dirclr51c_pin9_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN9 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN10: Set as input pin 10<br>"]
    pub(crate) fn p0_dirclr51c_pin10_read(&self) -> MemResult<bool> {
        todo!("read PIN10 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN10: Set as input pin 10<br>"]
    pub(crate) fn p0_dirclr51c_pin10_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN10 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN11: Set as input pin 11<br>"]
    pub(crate) fn p0_dirclr51c_pin11_read(&self) -> MemResult<bool> {
        todo!("read PIN11 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN11: Set as input pin 11<br>"]
    pub(crate) fn p0_dirclr51c_pin11_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN11 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN12: Set as input pin 12<br>"]
    pub(crate) fn p0_dirclr51c_pin12_read(&self) -> MemResult<bool> {
        todo!("read PIN12 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN12: Set as input pin 12<br>"]
    pub(crate) fn p0_dirclr51c_pin12_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN12 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN13: Set as input pin 13<br>"]
    pub(crate) fn p0_dirclr51c_pin13_read(&self) -> MemResult<bool> {
        todo!("read PIN13 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN13: Set as input pin 13<br>"]
    pub(crate) fn p0_dirclr51c_pin13_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN13 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN14: Set as input pin 14<br>"]
    pub(crate) fn p0_dirclr51c_pin14_read(&self) -> MemResult<bool> {
        todo!("read PIN14 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN14: Set as input pin 14<br>"]
    pub(crate) fn p0_dirclr51c_pin14_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN14 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN15: Set as input pin 15<br>"]
    pub(crate) fn p0_dirclr51c_pin15_read(&self) -> MemResult<bool> {
        todo!("read PIN15 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN15: Set as input pin 15<br>"]
    pub(crate) fn p0_dirclr51c_pin15_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN15 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN16: Set as input pin 16<br>"]
    pub(crate) fn p0_dirclr51c_pin16_read(&self) -> MemResult<bool> {
        todo!("read PIN16 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN16: Set as input pin 16<br>"]
    pub(crate) fn p0_dirclr51c_pin16_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN16 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN17: Set as input pin 17<br>"]
    pub(crate) fn p0_dirclr51c_pin17_read(&self) -> MemResult<bool> {
        todo!("read PIN17 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN17: Set as input pin 17<br>"]
    pub(crate) fn p0_dirclr51c_pin17_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN17 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN18: Set as input pin 18<br>"]
    pub(crate) fn p0_dirclr51c_pin18_read(&self) -> MemResult<bool> {
        todo!("read PIN18 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN18: Set as input pin 18<br>"]
    pub(crate) fn p0_dirclr51c_pin18_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN18 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN19: Set as input pin 19<br>"]
    pub(crate) fn p0_dirclr51c_pin19_read(&self) -> MemResult<bool> {
        todo!("read PIN19 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN19: Set as input pin 19<br>"]
    pub(crate) fn p0_dirclr51c_pin19_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN19 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN20: Set as input pin 20<br>"]
    pub(crate) fn p0_dirclr51c_pin20_read(&self) -> MemResult<bool> {
        todo!("read PIN20 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN20: Set as input pin 20<br>"]
    pub(crate) fn p0_dirclr51c_pin20_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN20 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN21: Set as input pin 21<br>"]
    pub(crate) fn p0_dirclr51c_pin21_read(&self) -> MemResult<bool> {
        todo!("read PIN21 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN21: Set as input pin 21<br>"]
    pub(crate) fn p0_dirclr51c_pin21_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN21 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN22: Set as input pin 22<br>"]
    pub(crate) fn p0_dirclr51c_pin22_read(&self) -> MemResult<bool> {
        todo!("read PIN22 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN22: Set as input pin 22<br>"]
    pub(crate) fn p0_dirclr51c_pin22_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN22 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN23: Set as input pin 23<br>"]
    pub(crate) fn p0_dirclr51c_pin23_read(&self) -> MemResult<bool> {
        todo!("read PIN23 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN23: Set as input pin 23<br>"]
    pub(crate) fn p0_dirclr51c_pin23_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN23 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN24: Set as input pin 24<br>"]
    pub(crate) fn p0_dirclr51c_pin24_read(&self) -> MemResult<bool> {
        todo!("read PIN24 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN24: Set as input pin 24<br>"]
    pub(crate) fn p0_dirclr51c_pin24_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN24 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN25: Set as input pin 25<br>"]
    pub(crate) fn p0_dirclr51c_pin25_read(&self) -> MemResult<bool> {
        todo!("read PIN25 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN25: Set as input pin 25<br>"]
    pub(crate) fn p0_dirclr51c_pin25_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN25 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN26: Set as input pin 26<br>"]
    pub(crate) fn p0_dirclr51c_pin26_read(&self) -> MemResult<bool> {
        todo!("read PIN26 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN26: Set as input pin 26<br>"]
    pub(crate) fn p0_dirclr51c_pin26_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN26 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN27: Set as input pin 27<br>"]
    pub(crate) fn p0_dirclr51c_pin27_read(&self) -> MemResult<bool> {
        todo!("read PIN27 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN27: Set as input pin 27<br>"]
    pub(crate) fn p0_dirclr51c_pin27_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN27 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN28: Set as input pin 28<br>"]
    pub(crate) fn p0_dirclr51c_pin28_read(&self) -> MemResult<bool> {
        todo!("read PIN28 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN28: Set as input pin 28<br>"]
    pub(crate) fn p0_dirclr51c_pin28_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN28 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN29: Set as input pin 29<br>"]
    pub(crate) fn p0_dirclr51c_pin29_read(&self) -> MemResult<bool> {
        todo!("read PIN29 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN29: Set as input pin 29<br>"]
    pub(crate) fn p0_dirclr51c_pin29_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN29 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN30: Set as input pin 30<br>"]
    pub(crate) fn p0_dirclr51c_pin30_read(&self) -> MemResult<bool> {
        todo!("read PIN30 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN30: Set as input pin 30<br>"]
    pub(crate) fn p0_dirclr51c_pin30_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN30 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN31: Set as input pin 31<br>"]
    pub(crate) fn p0_dirclr51c_pin31_read(&self) -> MemResult<bool> {
        todo!("read PIN31 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN31: Set as input pin 31<br>"]
    pub(crate) fn p0_dirclr51c_pin31_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN31 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN0: Status on whether PIN0 has met criteria set in PIN_CNF0.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin0_read(&self) -> MemResult<bool> {
        todo!("read PIN0 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN0: Status on whether PIN0 has met criteria set in PIN_CNF0.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin0_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN0 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN1: Status on whether PIN1 has met criteria set in PIN_CNF1.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin1_read(&self) -> MemResult<bool> {
        todo!("read PIN1 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN1: Status on whether PIN1 has met criteria set in PIN_CNF1.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin1_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN1 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN2: Status on whether PIN2 has met criteria set in PIN_CNF2.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin2_read(&self) -> MemResult<bool> {
        todo!("read PIN2 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN2: Status on whether PIN2 has met criteria set in PIN_CNF2.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin2_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN2 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN3: Status on whether PIN3 has met criteria set in PIN_CNF3.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin3_read(&self) -> MemResult<bool> {
        todo!("read PIN3 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN3: Status on whether PIN3 has met criteria set in PIN_CNF3.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin3_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN3 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN4: Status on whether PIN4 has met criteria set in PIN_CNF4.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin4_read(&self) -> MemResult<bool> {
        todo!("read PIN4 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN4: Status on whether PIN4 has met criteria set in PIN_CNF4.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin4_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN4 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN5: Status on whether PIN5 has met criteria set in PIN_CNF5.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin5_read(&self) -> MemResult<bool> {
        todo!("read PIN5 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN5: Status on whether PIN5 has met criteria set in PIN_CNF5.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin5_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN5 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN6: Status on whether PIN6 has met criteria set in PIN_CNF6.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin6_read(&self) -> MemResult<bool> {
        todo!("read PIN6 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN6: Status on whether PIN6 has met criteria set in PIN_CNF6.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin6_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN6 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN7: Status on whether PIN7 has met criteria set in PIN_CNF7.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin7_read(&self) -> MemResult<bool> {
        todo!("read PIN7 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN7: Status on whether PIN7 has met criteria set in PIN_CNF7.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin7_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN7 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN8: Status on whether PIN8 has met criteria set in PIN_CNF8.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin8_read(&self) -> MemResult<bool> {
        todo!("read PIN8 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN8: Status on whether PIN8 has met criteria set in PIN_CNF8.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin8_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN8 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN9: Status on whether PIN9 has met criteria set in PIN_CNF9.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin9_read(&self) -> MemResult<bool> {
        todo!("read PIN9 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN9: Status on whether PIN9 has met criteria set in PIN_CNF9.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin9_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN9 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN10: Status on whether PIN10 has met criteria set in PIN_CNF10.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin10_read(&self) -> MemResult<bool> {
        todo!("read PIN10 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN10: Status on whether PIN10 has met criteria set in PIN_CNF10.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin10_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN10 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN11: Status on whether PIN11 has met criteria set in PIN_CNF11.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin11_read(&self) -> MemResult<bool> {
        todo!("read PIN11 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN11: Status on whether PIN11 has met criteria set in PIN_CNF11.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin11_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN11 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN12: Status on whether PIN12 has met criteria set in PIN_CNF12.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin12_read(&self) -> MemResult<bool> {
        todo!("read PIN12 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN12: Status on whether PIN12 has met criteria set in PIN_CNF12.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin12_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN12 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN13: Status on whether PIN13 has met criteria set in PIN_CNF13.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin13_read(&self) -> MemResult<bool> {
        todo!("read PIN13 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN13: Status on whether PIN13 has met criteria set in PIN_CNF13.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin13_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN13 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN14: Status on whether PIN14 has met criteria set in PIN_CNF14.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin14_read(&self) -> MemResult<bool> {
        todo!("read PIN14 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN14: Status on whether PIN14 has met criteria set in PIN_CNF14.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin14_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN14 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN15: Status on whether PIN15 has met criteria set in PIN_CNF15.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin15_read(&self) -> MemResult<bool> {
        todo!("read PIN15 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN15: Status on whether PIN15 has met criteria set in PIN_CNF15.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin15_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN15 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN16: Status on whether PIN16 has met criteria set in PIN_CNF16.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin16_read(&self) -> MemResult<bool> {
        todo!("read PIN16 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN16: Status on whether PIN16 has met criteria set in PIN_CNF16.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin16_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN16 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN17: Status on whether PIN17 has met criteria set in PIN_CNF17.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin17_read(&self) -> MemResult<bool> {
        todo!("read PIN17 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN17: Status on whether PIN17 has met criteria set in PIN_CNF17.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin17_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN17 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN18: Status on whether PIN18 has met criteria set in PIN_CNF18.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin18_read(&self) -> MemResult<bool> {
        todo!("read PIN18 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN18: Status on whether PIN18 has met criteria set in PIN_CNF18.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin18_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN18 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN19: Status on whether PIN19 has met criteria set in PIN_CNF19.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin19_read(&self) -> MemResult<bool> {
        todo!("read PIN19 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN19: Status on whether PIN19 has met criteria set in PIN_CNF19.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin19_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN19 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN20: Status on whether PIN20 has met criteria set in PIN_CNF20.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin20_read(&self) -> MemResult<bool> {
        todo!("read PIN20 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN20: Status on whether PIN20 has met criteria set in PIN_CNF20.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin20_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN20 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN21: Status on whether PIN21 has met criteria set in PIN_CNF21.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin21_read(&self) -> MemResult<bool> {
        todo!("read PIN21 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN21: Status on whether PIN21 has met criteria set in PIN_CNF21.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin21_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN21 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN22: Status on whether PIN22 has met criteria set in PIN_CNF22.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin22_read(&self) -> MemResult<bool> {
        todo!("read PIN22 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN22: Status on whether PIN22 has met criteria set in PIN_CNF22.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin22_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN22 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN23: Status on whether PIN23 has met criteria set in PIN_CNF23.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin23_read(&self) -> MemResult<bool> {
        todo!("read PIN23 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN23: Status on whether PIN23 has met criteria set in PIN_CNF23.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin23_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN23 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN24: Status on whether PIN24 has met criteria set in PIN_CNF24.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin24_read(&self) -> MemResult<bool> {
        todo!("read PIN24 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN24: Status on whether PIN24 has met criteria set in PIN_CNF24.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin24_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN24 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN25: Status on whether PIN25 has met criteria set in PIN_CNF25.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin25_read(&self) -> MemResult<bool> {
        todo!("read PIN25 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN25: Status on whether PIN25 has met criteria set in PIN_CNF25.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin25_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN25 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN26: Status on whether PIN26 has met criteria set in PIN_CNF26.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin26_read(&self) -> MemResult<bool> {
        todo!("read PIN26 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN26: Status on whether PIN26 has met criteria set in PIN_CNF26.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin26_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN26 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN27: Status on whether PIN27 has met criteria set in PIN_CNF27.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin27_read(&self) -> MemResult<bool> {
        todo!("read PIN27 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN27: Status on whether PIN27 has met criteria set in PIN_CNF27.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin27_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN27 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN28: Status on whether PIN28 has met criteria set in PIN_CNF28.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin28_read(&self) -> MemResult<bool> {
        todo!("read PIN28 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN28: Status on whether PIN28 has met criteria set in PIN_CNF28.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin28_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN28 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN29: Status on whether PIN29 has met criteria set in PIN_CNF29.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin29_read(&self) -> MemResult<bool> {
        todo!("read PIN29 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN29: Status on whether PIN29 has met criteria set in PIN_CNF29.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin29_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN29 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN30: Status on whether PIN30 has met criteria set in PIN_CNF30.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin30_read(&self) -> MemResult<bool> {
        todo!("read PIN30 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN30: Status on whether PIN30 has met criteria set in PIN_CNF30.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin30_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN30 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN31: Status on whether PIN31 has met criteria set in PIN_CNF31.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin31_read(&self) -> MemResult<bool> {
        todo!("read PIN31 mwrite None write None rac None reset value false")
    }
    #[doc = "PIN31: Status on whether PIN31 has met criteria set in PIN_CNF31.SENSE register. Write '1' to clear.<br>"]
    pub(crate) fn p0_latch520_pin31_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write PIN31 mwrite None write None rac None reset value false")
    }
    #[doc = "DETECTMODE: Select between default DETECT signal behaviour and LDETECT mode<br>"]
    pub(crate) fn p0_detectmode524_detectmode_read(&self) -> MemResult<bool> {
        todo!(
            "read DETECTMODE mwrite None write None rac None reset value false"
        )
    }
    #[doc = "DETECTMODE: Select between default DETECT signal behaviour and LDETECT mode<br>"]
    pub(crate) fn p0_detectmode524_detectmode_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write DETECTMODE mwrite None write None rac None reset value false")
    }
    #[doc = "DIR: Pin direction. Same physical register as DIR register<br>"]
    pub(crate) fn p0_pin_cnfn700_dir_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read DIR mwrite None write None rac None reset value false")
    }
    #[doc = "DIR: Pin direction. Same physical register as DIR register<br>"]
    pub(crate) fn p0_pin_cnfn700_dir_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write DIR mwrite None write None rac None reset value false")
    }
    #[doc = "INPUT: Connect or disconnect input buffer<br>"]
    pub(crate) fn p0_pin_cnfn700_input_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<bool> {
        todo!("read INPUT mwrite None write None rac None reset value true")
    }
    #[doc = "INPUT: Connect or disconnect input buffer<br>"]
    pub(crate) fn p0_pin_cnfn700_input_write(
        &mut self,
        _reg_array: usize,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write INPUT mwrite None write None rac None reset value true")
    }
    #[doc = "PULL: Pull configuration<br>"]
    pub(crate) fn p0_pin_cnfn700_pull_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<crate::peripheral::enums::E65P0PinCnfn700Pull> {
        todo ! ("read PULL mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "PULL: Pull configuration<br>"]
    pub(crate) fn p0_pin_cnfn700_pull_write(
        &mut self,
        _reg_array: usize,
        _value: crate::peripheral::enums::E65P0PinCnfn700Pull,
    ) -> MemResult<()> {
        todo ! ("write PULL mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "DRIVE: Drive configuration<br>"]
    pub(crate) fn p0_pin_cnfn700_drive_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<crate::peripheral::enums::E66P0PinCnfn700Drive> {
        todo ! ("read DRIVE mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "DRIVE: Drive configuration<br>"]
    pub(crate) fn p0_pin_cnfn700_drive_write(
        &mut self,
        _reg_array: usize,
        _value: crate::peripheral::enums::E66P0PinCnfn700Drive,
    ) -> MemResult<()> {
        todo ! ("write DRIVE mwrite None write None rac None reset value 0x00 mask 0x07")
    }
    #[doc = "SENSE: Pin sensing mechanism<br>"]
    pub(crate) fn p0_pin_cnfn700_sense_read(
        &self,
        _reg_array: usize,
    ) -> MemResult<crate::peripheral::enums::E67P0PinCnfn700Sense> {
        todo ! ("read SENSE mwrite None write None rac None reset value 0x00 mask 0x03")
    }
    #[doc = "SENSE: Pin sensing mechanism<br>"]
    pub(crate) fn p0_pin_cnfn700_sense_write(
        &mut self,
        _reg_array: usize,
        _value: crate::peripheral::enums::E67P0PinCnfn700Sense,
    ) -> MemResult<()> {
        todo ! ("write SENSE mwrite None write None rac None reset value 0x00 mask 0x03")
    }
}
