#[derive(Default, Debug, Clone, Copy)]
pub struct Event {
    pub on: bool,
    pub triggered: bool,
}

/// The documentation recomend writing '1' to enable a task, here we allow any
/// value `!= 0`
pub const fn trigger_task(_value: u32) -> bool {
    _value != 0
}

