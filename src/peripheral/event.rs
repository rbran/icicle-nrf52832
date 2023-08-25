#[derive(Default, Debug, Clone, Copy)]
pub struct Event {
    pub on: bool,
    pub triggered: bool,
}

impl Event {
    pub fn trigger_on_write(&mut self, _value: u32) {
        if trigger_task(_value) {
            self.triggered = true;
        }
    }
}

/// The documentation recomend writing '1' to enable a task, here we allow any
/// value `!= 0`
pub const fn trigger_task(value: u32) -> bool {
    value != 0
}
