#[derive(Default)]
pub struct PPI {
    group: [PPIChannelGroup; 4],
    channels: [PpiSoftChannel; 16],
    hard_channels: [PpiHardChannel; 12],
}

impl PPI {
    pub fn is_on(&self, channel: usize) -> bool {
        match channel {
            0..=15 => self.channels[channel].on,
            20..=31 => self.hard_channels[channel - 20].on,
            _ => unreachable!(),
        }
    }
    pub fn set_on(&mut self, channel: usize, on: bool) {
        match channel {
            0..=15 => self.channels[channel].on = on,
            20..=31 => self.hard_channels[channel - 20].on = on,
            _ => unreachable!(),
        }
    }
    pub fn is_included(&self, channel: usize, included: usize) -> bool {
        self.group[channel].is_included(included)
    }
    pub fn set_included(&mut self, channel: usize, included: usize, on: bool) {
        self.group[channel].set_included(included, on)
    }
    pub fn event(&self, channel: usize) -> u32 {
        match channel {
            0..=15 => self.channels[channel].event,
            20..=31 => todo!(),
            _ => unreachable!(),
        }
    }
    pub fn task(&self, channel: usize) -> u32 {
        match channel {
            0..=15 => self.channels[channel].task,
            20..=31 => todo!(),
            _ => unreachable!(),
        }
    }
}

#[derive(Default)]
pub struct PPIChannelGroup {
    included: [bool; 32],
}

impl PPIChannelGroup {
    pub fn is_included(&self, included: usize) -> bool {
        self.included[included]
    }
    pub fn set_included(&mut self, included: usize, on: bool) {
        self.included[included] = on
    }
}

#[derive(Default)]
pub struct PpiSoftChannel {
    on: bool,
    event: u32,
    task: u32,
}

#[derive(Default)]
pub struct PpiHardChannel {
    on: bool,
}
