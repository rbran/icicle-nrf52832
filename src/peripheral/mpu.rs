#[derive(Default, Clone, Copy)]
pub struct MpuRegion {
    protected: bool,
}

impl MpuRegion {
    pub fn is_protected(&self) -> bool {
        self.protected
    }
    pub fn set_protected(&mut self, protected: bool) {
        self.protected = protected
    }
}
