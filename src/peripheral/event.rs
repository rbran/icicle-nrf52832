#[derive(Default, Debug, Clone, Copy)]
pub struct Event {
    pub on: bool,
    pub triggered: bool,
}

impl Event {
    pub(crate) fn trigger_on_write(&mut self, _value: u32) {
        if trigger_task(_value) {
            self.triggered = true;
        }
    }

    pub(crate) fn clean_on_write(&mut self, _value: u32) {
        if clean_event(_value) {
            self.triggered = false;
        }
    }
}

/// The documentation recomend writing '1' to enable a task, here we allow any
/// value `!= 0`
pub(crate) const fn trigger_task(value: u32) -> bool {
    value != 0
}

/// The documentation says that only writting '0' to a event will clean it's
/// value
pub(crate) const fn clean_event(value: u32) -> bool {
    value == 0
}
