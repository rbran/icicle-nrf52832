use icicle_vm::cpu::mem::MemResult;

pub struct Interrupts {
    pub interrupts_enabled: [bool; 256],
    pub interrupts_pending: [bool; 256],
    pub interrupts_priority: [u8; 240],
}
impl Default for Interrupts {
    fn default() -> Self {
        Self {
            interrupts_enabled: [false; 256],
            interrupts_pending: [false; 256],
            interrupts_priority: [0; 240],
        }
    }
}
impl Interrupts {
    pub fn nvic_enable<'a>(
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
            **byte = 0;
            for bit in 0..8 {
                let _inter_num = (_nvic * 32) + bit + (byte_i * 8);
                **byte |= (self.is_on(_inter_num) as u8) << bit;
            }
        }
        Ok(())
    }
    pub fn nvic_set_enable(
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
    pub fn nvic_clr_enable(
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

    pub fn nvic_pending<'a>(
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
            **byte = 0;
            for bit in 0..8 {
                let _inter_num = (_nvic * 32) + bit + (byte_i * 8);
                **byte |= (self.is_pending(_inter_num) as u8) << bit;
            }
        }
        Ok(())
    }
    pub fn nvic_set_pending(
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
                    self.set_pending(_inter_num, true);
                }
            }
        }
        Ok(())
    }
    pub fn nvic_clr_pending(
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
                    self.set_pending(_inter_num, false);
                }
            }
        }
        Ok(())
    }
    #[inline]
    pub fn is_pending(&self, _interrupt: usize) -> bool {
        self.interrupts_pending[_interrupt]
    }
    #[inline]
    pub fn set_pending(&mut self, _interrupt: usize, _on: bool) {
        self.interrupts_pending[_interrupt] = _on
    }

    pub fn nvic_active<'a>(
        &self,
        _nvic: usize,
        _byte_0: &mut Option<&'a mut u8>,
        _byte_1: &mut Option<&'a mut u8>,
        _byte_2: &mut Option<&'a mut u8>,
        _byte_3: &mut Option<&'a mut u8>,
    ) -> MemResult<()> {
        // From "Cortex-M4 Device Generic User Guide" "4.2.6"
        // A bit reads as one if the status of the corresponding interrupt is
        // active or active and pending.
        //
        // What the Fuck is this suppose to mean?
        // "active || (active && pending)"?
        // so basically only if active?
        // I'll assume it means "active || pending"
        for (byte_i, byte) in [_byte_0, _byte_1, _byte_2, _byte_3]
            .into_iter()
            .enumerate()
            .filter_map(|(i, b)| Some((i, b.as_mut()?)))
        {
            **byte = 0;
            for bit in 0..8 {
                let _inter_num = (_nvic * 32) + bit + (byte_i * 8);
                let active =
                    self.is_pending(_inter_num) || self.is_pending(_inter_num);
                **byte |= (active as u8) << bit;
            }
        }
        Ok(())
    }

    #[inline]
    pub fn priority(&self, _interrupt: usize) -> u8 {
        if _interrupt >= 240 {
            unreachable!()
        }
        self.interrupts_priority[_interrupt]
    }
    #[inline]
    pub fn set_priority(&mut self, _interrupt: usize, _pri: u8) {
        if _interrupt >= 240 {
            unreachable!()
        }
        self.interrupts_priority[_interrupt] = _pri
    }
}
