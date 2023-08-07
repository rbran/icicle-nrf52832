#[derive(Default)]
pub struct Wdt {
    on: bool
}

impl Wdt {
    /// Indicates whether or not the watchdog is running
    /// Table 167: RUNSTATUS
    pub fn run_status(&self) -> bool {
        self.on
    }
    pub fn is_on(&self) -> bool {
        self.on
    }
    pub fn set_on(&mut self, on: bool) {
        self.on = on
    }
}
