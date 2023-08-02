#[derive(Default)]
pub struct Gpio {
    pull: GpioPull,
    drive: GpioDrive,
    sense: GpioSense,
    buffered: bool,
    input: bool,
    output_high: bool,
}

impl Gpio {
    pub fn is_out_high(&self) -> bool {
        self.output_high
    }
    pub fn set_out_high(&mut self, _high: bool) {
        self.output_high = _high;
    }
    pub fn read_input(&self) -> bool {
        todo!()
    }
    pub fn get_direction(&self) -> bool {
        !self.input
    }
    pub fn set_direction(&mut self, _output: bool) {
        self.input = !_output
    }
    pub fn is_connected_buffer(&self) -> bool {
        self.buffered
    }
    pub fn set_connected_buffer(&mut self, _on: bool) {
        self.buffered = _on
    }
    pub fn get_pull(&self) -> GpioPull {
        self.pull
    }
    pub fn set_pull(&mut self, _pull: GpioPull) {
        self.pull = _pull
    }
    pub fn get_drive(&self) -> GpioDrive {
        self.drive
    }
    pub fn set_drive(&mut self, _drive: GpioDrive) {
        self.drive = _drive
    }
    pub fn get_sense(&self) -> GpioSense {
        self.sense
    }
    pub fn set_sense(&mut self, _sense: GpioSense) {
        self.sense = _sense
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Default)]
pub enum GpioPull {
    #[default]
    None = 0,
    Down = 1,
    Up = 3,
}
impl TryFrom<u8> for GpioPull {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GpioPull::None),
            1 => Ok(GpioPull::Down),
            3 => Ok(GpioPull::Up),
            _ => Err(()),
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Default)]
pub enum GpioDrive {
    /// Standard '0', standard '1'
    #[default]
    S0S1 = 0,
    /// High drive '0', standard '1'
    H0S1 = 1,
    /// Standard '0', high drive '1'
    S0H1 = 2,
    /// High drive '0', high 'drive '1''
    H0H1 = 3,
    /// Disconnect '0' standard '1'
    D0S1 = 4,
    /// Disconnect '0', high drive '1'
    D0H1 = 5,
    /// Standard '0'. disconnect '1'
    S0D1 = 6,
    /// High drive '0', disconnect '1'
    H0D1 = 7,
}
impl TryFrom<u8> for GpioDrive {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::S0S1),
            1 => Ok(Self::H0S1),
            2 => Ok(Self::S0H1),
            3 => Ok(Self::H0H1),
            4 => Ok(Self::D0S1),
            5 => Ok(Self::D0H1),
            6 => Ok(Self::S0D1),
            7 => Ok(Self::H0D1),
            _ => Err(()),
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Default)]
pub enum GpioSense {
    #[default]
    Disabled = 0,
    High = 2,
    Low = 3,
}
impl TryFrom<u8> for GpioSense {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GpioSense::Disabled),
            2 => Ok(GpioSense::High),
            3 => Ok(GpioSense::Low),
            _ => Err(()),
        }
    }
}
