use icicle_vm::cpu::mem::MemResult;

pub struct Interrupts {
    pub interrupts_enabled: [bool; 256],
}
impl Default for Interrupts {
    fn default() -> Self {
        Self {
            interrupts_enabled: [false; 256],
        }
    }
}
impl Interrupts {
    pub fn nvic_read<'a>(
        &self,
        _nvic: usize,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        for (byte_i, byte) in [_byte_0, _byte_1, _byte_2, _byte_3]
            .into_iter()
            .enumerate()
            .filter_map(|(i, b)| Some((i, b.as_mut()?)))
        {
            for bit in 0..8 {
                let _inter_num = (_nvic * 32) + bit + (byte_i * 8);
                **byte |= (self.is_on(_inter_num) as u8) << bit;
            }
        }
        Ok(())
    }
    pub fn nvic_write(
        &mut self,
        _nvic: usize,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        let bytes = [_byte_0, _byte_1, _byte_2, _byte_3]
            .into_iter()
            .enumerate()
            .filter_map(|(i, b)| Some((i, b?)));
        let _nvic_bit = _nvic * 32;
        for (byte_i, byte) in bytes {
            for bit in 0..8 {
                let _inter_num = _nvic_bit + (byte_i * 8) + bit;
                if ((*byte >> bit) & 1) != 0 {
                    self.set_on(_inter_num, true);
                }
            }
        }
        Ok(())
    }
    pub fn nvic_clr(
        &mut self,
        _nvic: usize,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> MemResult<()> {
        let bytes = [_byte_0, _byte_1, _byte_2, _byte_3]
            .into_iter()
            .enumerate()
            .filter_map(|(i, b)| Some((i, b?)));
        let _nvic_bit = _nvic * 32;
        for (byte_i, byte) in bytes {
            for bit in 0..8 {
                let _inter_num = _nvic_bit + (byte_i * 8) + bit;
                if ((*byte >> bit) & 1) == 0 {
                    self.set_on(_inter_num, false);
                }
            }
        }
        Ok(())
    }
    #[inline]
    pub fn is_on(&self, _interrupt: usize) -> bool {
        self.interrupts_enabled[_interrupt]
    }
    #[inline]
    pub fn set_on(&mut self, _interrupt: usize, _on: bool) {
        self.interrupts_enabled[_interrupt] = _on
    }
}
