#[derive(Default)]
pub struct Radio {
    address_prefix: [u8; 8],
    receive_on_ap: [bool; 8],
    tx_on_ap: [bool; 8],
    address_prefix_on: [bool; 8],
}

impl Radio {
    pub fn address_prefix(&self, ap: usize) -> u8 {
        self.address_prefix[ap]
    }
    pub fn set_address_prefix(&mut self, ap: usize, value: u8) {
        self.address_prefix[ap] = value
    }
    pub fn receive_on_ap(&self, ap: usize) -> bool {
        self.receive_on_ap[ap]
    }
    pub fn set_receive_on_ap(&mut self, ap: usize, value: bool) {
        self.receive_on_ap[ap] = value
    }
    pub fn address_prefix_on(&self, ap: usize) -> bool {
        self.address_prefix_on[ap]
    }
    pub fn set_address_prefix_on(&mut self, ap: usize, value: bool) {
        self.address_prefix_on[ap] = value
    }
    pub fn tx_on_ap(&self, ap: usize) -> bool {
        self.tx_on_ap[ap]
    }
    pub fn set_tx_on_ap(&mut self, ap: usize, value: bool) {
        self.tx_on_ap[ap] = value
    }
}
