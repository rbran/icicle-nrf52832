#[derive(Default)]
pub struct Wdt {
}

impl Wdt {
    /// Indicates whether or not the watchdog is running
    /// Table 167: RUNSTATUS
    pub fn run_status(&self) -> bool {
        // TODO allow wdt to be enabled/disabled
        true
    }
}
