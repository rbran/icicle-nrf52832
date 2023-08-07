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
    pub fn nvic_enable<'a>(&self, _nvic: usize) -> u32 {
        (0..32)
            .map(|bit| (self.is_on(_nvic * 32 + bit) as u32) << bit)
            .fold(0, |acc, x| acc | x)
    }
    pub fn nvic_set_enable(&mut self, _nvic: usize, _value: u32) {
        for bit in 0..32 {
            if ((_value >> bit) & 1) != 0 {
                self.set_on(_nvic * 32 + bit, true);
            }
        }
    }
    pub fn nvic_clr_enable(&mut self, _nvic: usize, _value: u32) {
        for bit in 0..32 {
            if ((_value >> bit) & 1) == 0 {
                self.set_on(_nvic * 32 + bit, false);
            }
        }
    }
    #[inline]
    pub fn is_on(&self, _interrupt: usize) -> bool {
        self.interrupts_enabled[_interrupt]
    }
    #[inline]
    pub fn set_on(&mut self, _interrupt: usize, _on: bool) {
        self.interrupts_enabled[_interrupt] = _on
    }

    pub fn nvic_pending<'a>(&self, _nvic: usize) -> u32 {
        (0..32)
            .map(|bit| (self.is_pending(_nvic * 32 + bit) as u32) << bit)
            .fold(0, |acc, x| acc | x)
    }
    pub fn nvic_set_pending(&mut self, _nvic: usize, _value: u32) {
        for bit in 0..32 {
            if ((_value >> bit) & 1) != 0 {
                self.set_pending(_nvic * 32 + bit, true);
            }
        }
    }
    pub fn nvic_clr_pending(&mut self, _nvic: usize, _value: u32) {
        for bit in 0..32 {
            if ((_value >> bit) & 1) == 0 {
                self.set_pending(_nvic * 32 + bit, false);
            }
        }
    }
    #[inline]
    pub fn is_pending(&self, _interrupt: usize) -> bool {
        self.interrupts_pending[_interrupt]
    }
    #[inline]
    pub fn set_pending(&mut self, _interrupt: usize, _on: bool) {
        self.interrupts_pending[_interrupt] = _on
    }

    pub fn nvic_active<'a>(&self, _nvic: usize) -> u32 {
        // From "Cortex-M4 Device Generic User Guide" "4.2.6"
        // A bit reads as one if the status of the corresponding interrupt is
        // active or active and pending.
        //
        // What the Fuck is this suppose to mean?
        // "active || (active && pending)"?
        // so basically only if active?
        // I'll assume it means "active || pending"
        (0..32)
            .map(|bit| {
                let nvic_bit = _nvic * 32 + bit;
                ((self.is_on(nvic_bit) | self.is_pending(nvic_bit)) as u32)
                    << bit
            })
            .fold(0, |acc, x| acc | x)
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
