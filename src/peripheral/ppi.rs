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
        self.group[channel].included[included]
    }
    pub fn set_included(&mut self, channel: usize, included: usize, on: bool) {
        self.group[channel].included[included] = on
    }
    pub fn event(&self, channel: usize) -> u32 {
        self.channels[channel].event
    }
    pub fn set_event(&mut self, channel: usize, event: u32) {
        self.channels[channel].event = event
    }
    pub fn task(&self, channel: usize) -> u32 {
        self.channels[channel].task
    }
    pub fn set_task(&mut self, channel: usize, task: u32) {
        self.channels[channel].task = task
    }
    pub fn set_group_on(&mut self, group: usize, on: bool) {
        self.group[group].on = on
    }
}

#[derive(Default)]
pub struct PPIChannelGroup {
    on: bool,
    included: [bool; 32],
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
