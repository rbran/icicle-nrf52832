pub const ID: usize = 0;

use super::event::Event;

#[derive(Default)]
pub struct PowerClock {
    source: Source,
    // FUTURE: use the `core::mem::variant_count` to avoid using a number
    events: [Event; 7],
    bypass: bool,
    external: bool,
}

impl PowerClock {
    pub fn source(&self) -> Source {
        self.source
    }
    pub fn set_source(&mut self, source: Source) {
        self.source = source
    }
    pub fn event(&self, event: EventId) -> &Event {
        &self.events[event as usize]
    }
    pub fn event_mut(&mut self, event: EventId) -> &mut Event {
        &mut self.events[event as usize]
    }
    pub fn bypass(&self) -> bool {
        self.bypass
    }
    pub fn set_bypass(&mut self, on: bool) {
        self.bypass = on
    }
    pub fn external(&self) -> bool {
        self.external
    }
    pub fn set_external(&mut self, on: bool) {
        self.external = on
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum EventId {
    HFCLKSTARTED = 0,
    LFCLKSTARTED = 1,
    POFWARN = 2,
    DONE = 3,
    CTTO = 4,
    SLEEPENTER = 5,
    SLEEPEXIT = 6,
}

#[derive(Clone, Copy, Default)]
#[repr(u8)]
pub enum Source {
    #[default]
    Rc = 0,
    Xtal = 1,
    Synth = 2,
}

impl TryFrom<u8> for Source {
    type Error = ();
    fn try_from(value: u8) -> Result<Source, Self::Error> {
        match value {
            0 => Ok(Source::Rc),
            1 => Ok(Source::Xtal),
            2 => Ok(Source::Synth),
            _ => Err(()),
        }
    }
}
