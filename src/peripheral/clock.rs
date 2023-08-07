#[derive(Default)]
pub struct Clock {
    source: Source,
}

impl Clock {
    pub fn source(&self) -> Source {
        self.source
    }
    pub fn set_source(&mut self, source: Source) {
        self.source = source
    }
    pub fn lfclkstarted(&self) -> bool {
        // TODO handle the this event
        true
    }
    pub fn set_lfclkstarted(&mut self) {
        // TODO handle the this event
    }
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
