#[derive(Default)]
pub struct RamBlock {
    keep_on: bool,
    retain_on: bool,
}

impl RamBlock {
    pub fn is_on(&self) -> bool {
        self.keep_on
    }
    pub fn is_keep_on(&self) -> bool {
        self.keep_on
    }
    pub fn set_keep_on(&mut self, _on: bool) {
        self.keep_on = _on
    }
    pub fn is_retain_on(&self) -> bool {
        self.retain_on
    }
    pub fn set_retain_on(&mut self, _on: bool) {
        self.retain_on = _on
    }
}
