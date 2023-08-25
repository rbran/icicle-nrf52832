#[derive(Debug, Clone, Copy)]
pub enum E0FicrInfoPart0Part {
    #[doc = "N52832: nRF52832<br>"]
    E0N52832,
    #[doc = "Unspecified: Unspecified<br>"]
    E1Unspecified,
}
impl From<E0FicrInfoPart0Part> for u32 {
    fn from(value: E0FicrInfoPart0Part) -> u32 {
        match value {
            E0FicrInfoPart0Part::E0N52832 => 337970,
            E0FicrInfoPart0Part::E1Unspecified => 4294967295,
        }
    }
}
impl TryFrom<u32> for E0FicrInfoPart0Part {
    type Error = ();
    fn try_from(value: u32) -> Result<E0FicrInfoPart0Part, Self::Error> {
        match value {
            337970 => Ok(Self::E0N52832),
            4294967295 => Ok(Self::E1Unspecified),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E1FicrInfoVariant4Variant {
    #[doc = "AAAA: AAAA<br>"]
    E0Aaaa,
    #[doc = "AAAB: AAAB<br>"]
    E1Aaab,
    #[doc = "AAB0: AAB0<br>"]
    E2Aab0,
    #[doc = "AABA: AABA<br>"]
    E3Aaba,
    #[doc = "AABB: AABB<br>"]
    E4Aabb,
    #[doc = "AAE0: AAE0<br>"]
    E5Aae0,
    #[doc = "Unspecified: Unspecified<br>"]
    E6Unspecified,
}
impl From<E1FicrInfoVariant4Variant> for u32 {
    fn from(value: E1FicrInfoVariant4Variant) -> u32 {
        match value {
            E1FicrInfoVariant4Variant::E0Aaaa => 1094795585,
            E1FicrInfoVariant4Variant::E1Aaab => 1094795586,
            E1FicrInfoVariant4Variant::E2Aab0 => 1094795824,
            E1FicrInfoVariant4Variant::E3Aaba => 1094795841,
            E1FicrInfoVariant4Variant::E4Aabb => 1094795842,
            E1FicrInfoVariant4Variant::E5Aae0 => 1094796592,
            E1FicrInfoVariant4Variant::E6Unspecified => 4294967295,
        }
    }
}
impl TryFrom<u32> for E1FicrInfoVariant4Variant {
    type Error = ();
    fn try_from(value: u32) -> Result<E1FicrInfoVariant4Variant, Self::Error> {
        match value {
            1094795585 => Ok(Self::E0Aaaa),
            1094795586 => Ok(Self::E1Aaab),
            1094795824 => Ok(Self::E2Aab0),
            1094795841 => Ok(Self::E3Aaba),
            1094795842 => Ok(Self::E4Aabb),
            1094796592 => Ok(Self::E5Aae0),
            4294967295 => Ok(Self::E6Unspecified),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E2FicrInfoPackage8Package {
    #[doc = "QF: QFxx - 48-pin QFN<br>"]
    E0Qf,
    #[doc = "CH: CHxx - 7x8 WLCSP 56 balls<br>"]
    E1Ch,
    #[doc = "CI: CIxx - 7x8 WLCSP 56 balls<br>"]
    E2Ci,
    #[doc = "CK: CKxx - 7x8 WLCSP 56 balls with backside coating for light protection<br>"]
    E3Ck,
    #[doc = "Unspecified: Unspecified<br>"]
    E4Unspecified,
}
impl From<E2FicrInfoPackage8Package> for u32 {
    fn from(value: E2FicrInfoPackage8Package) -> u32 {
        match value {
            E2FicrInfoPackage8Package::E0Qf => 8192,
            E2FicrInfoPackage8Package::E1Ch => 8193,
            E2FicrInfoPackage8Package::E2Ci => 8194,
            E2FicrInfoPackage8Package::E3Ck => 8197,
            E2FicrInfoPackage8Package::E4Unspecified => 4294967295,
        }
    }
}
impl TryFrom<u32> for E2FicrInfoPackage8Package {
    type Error = ();
    fn try_from(value: u32) -> Result<E2FicrInfoPackage8Package, Self::Error> {
        match value {
            8192 => Ok(Self::E0Qf),
            8193 => Ok(Self::E1Ch),
            8194 => Ok(Self::E2Ci),
            8197 => Ok(Self::E3Ck),
            4294967295 => Ok(Self::E4Unspecified),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E3FicrInfoRamcRam {
    #[doc = "K16: 16 kByte RAM<br>"]
    E0K16,
    #[doc = "K32: 32 kByte RAM<br>"]
    E1K32,
    #[doc = "K64: 64 kByte RAM<br>"]
    E2K64,
    #[doc = "Unspecified: Unspecified<br>"]
    E3Unspecified,
}
impl From<E3FicrInfoRamcRam> for u32 {
    fn from(value: E3FicrInfoRamcRam) -> u32 {
        match value {
            E3FicrInfoRamcRam::E0K16 => 16,
            E3FicrInfoRamcRam::E1K32 => 32,
            E3FicrInfoRamcRam::E2K64 => 64,
            E3FicrInfoRamcRam::E3Unspecified => 4294967295,
        }
    }
}
impl TryFrom<u32> for E3FicrInfoRamcRam {
    type Error = ();
    fn try_from(value: u32) -> Result<E3FicrInfoRamcRam, Self::Error> {
        match value {
            16 => Ok(Self::E0K16),
            32 => Ok(Self::E1K32),
            64 => Ok(Self::E2K64),
            4294967295 => Ok(Self::E3Unspecified),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E4FicrInfoFlash10Flash {
    #[doc = "K128: 128 kByte FLASH<br>"]
    E0K128,
    #[doc = "K256: 256 kByte FLASH<br>"]
    E1K256,
    #[doc = "K512: 512 kByte FLASH<br>"]
    E2K512,
    #[doc = "Unspecified: Unspecified<br>"]
    E3Unspecified,
}
impl From<E4FicrInfoFlash10Flash> for u32 {
    fn from(value: E4FicrInfoFlash10Flash) -> u32 {
        match value {
            E4FicrInfoFlash10Flash::E0K128 => 128,
            E4FicrInfoFlash10Flash::E1K256 => 256,
            E4FicrInfoFlash10Flash::E2K512 => 512,
            E4FicrInfoFlash10Flash::E3Unspecified => 4294967295,
        }
    }
}
impl TryFrom<u32> for E4FicrInfoFlash10Flash {
    type Error = ();
    fn try_from(value: u32) -> Result<E4FicrInfoFlash10Flash, Self::Error> {
        match value {
            128 => Ok(Self::E0K128),
            256 => Ok(Self::E1K256),
            512 => Ok(Self::E2K512),
            4294967295 => Ok(Self::E3Unspecified),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E5UicrApprotect208Pall {
    #[doc = "Enabled: Enable<br>"]
    E0Enabled,
    #[doc = "Disabled: Disable<br>"]
    E1Disabled,
}
impl From<E5UicrApprotect208Pall> for u8 {
    fn from(value: E5UicrApprotect208Pall) -> u8 {
        match value {
            E5UicrApprotect208Pall::E0Enabled => 0,
            E5UicrApprotect208Pall::E1Disabled => 255,
        }
    }
}
impl TryFrom<u8> for E5UicrApprotect208Pall {
    type Error = ();
    fn try_from(value: u8) -> Result<E5UicrApprotect208Pall, Self::Error> {
        match value {
            0 => Ok(Self::E0Enabled),
            255 => Ok(Self::E1Disabled),
            _ => Err(()),
        }
    }
}
#[derive(Default, Debug, Clone, Copy)]
pub enum LfclkSrc {
    #[default]
    #[doc = "RC: 32.768 kHz RC oscillator<br>"]
    E0Rc,
    #[doc = "Xtal: 32.768 kHz crystal oscillator<br>"]
    E1Xtal,
    #[doc = "Synth: 32.768 kHz synthesized from HFCLK<br>"]
    E2Synth,
}
impl From<LfclkSrc> for u8 {
    fn from(value: LfclkSrc) -> u8 {
        match value {
            LfclkSrc::E0Rc => 0,
            LfclkSrc::E1Xtal => 1,
            LfclkSrc::E2Synth => 2,
        }
    }
}
impl TryFrom<u8> for LfclkSrc {
    type Error = ();
    fn try_from(value: u8) -> Result<LfclkSrc, Self::Error> {
        match value {
            0 => Ok(Self::E0Rc),
            1 => Ok(Self::E1Xtal),
            2 => Ok(Self::E2Synth),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum E8Apb0Pofcon510Threshold {
    #[doc = "V17: Set threshold to 1.7 V<br>"]
    E0V17,
    #[doc = "V18: Set threshold to 1.8 V<br>"]
    E1V18,
    #[doc = "V19: Set threshold to 1.9 V<br>"]
    E2V19,
    #[doc = "V20: Set threshold to 2.0 V<br>"]
    E3V20,
    #[doc = "V21: Set threshold to 2.1 V<br>"]
    E4V21,
    #[doc = "V22: Set threshold to 2.2 V<br>"]
    E5V22,
    #[doc = "V23: Set threshold to 2.3 V<br>"]
    E6V23,
    #[doc = "V24: Set threshold to 2.4 V<br>"]
    E7V24,
    #[doc = "V25: Set threshold to 2.5 V<br>"]
    E8V25,
    #[doc = "V26: Set threshold to 2.6 V<br>"]
    E9V26,
    #[doc = "V27: Set threshold to 2.7 V<br>"]
    E10V27,
    #[doc = "V28: Set threshold to 2.8 V<br>"]
    E11V28,
}
impl From<E8Apb0Pofcon510Threshold> for u8 {
    fn from(value: E8Apb0Pofcon510Threshold) -> u8 {
        match value {
            E8Apb0Pofcon510Threshold::E0V17 => 4,
            E8Apb0Pofcon510Threshold::E1V18 => 5,
            E8Apb0Pofcon510Threshold::E2V19 => 6,
            E8Apb0Pofcon510Threshold::E3V20 => 7,
            E8Apb0Pofcon510Threshold::E4V21 => 8,
            E8Apb0Pofcon510Threshold::E5V22 => 9,
            E8Apb0Pofcon510Threshold::E6V23 => 10,
            E8Apb0Pofcon510Threshold::E7V24 => 11,
            E8Apb0Pofcon510Threshold::E8V25 => 12,
            E8Apb0Pofcon510Threshold::E9V26 => 13,
            E8Apb0Pofcon510Threshold::E10V27 => 14,
            E8Apb0Pofcon510Threshold::E11V28 => 15,
        }
    }
}
impl TryFrom<u8> for E8Apb0Pofcon510Threshold {
    type Error = ();
    fn try_from(value: u8) -> Result<E8Apb0Pofcon510Threshold, Self::Error> {
        match value {
            4 => Ok(Self::E0V17),
            5 => Ok(Self::E1V18),
            6 => Ok(Self::E2V19),
            7 => Ok(Self::E3V20),
            8 => Ok(Self::E4V21),
            9 => Ok(Self::E5V22),
            10 => Ok(Self::E6V23),
            11 => Ok(Self::E7V24),
            12 => Ok(Self::E8V25),
            13 => Ok(Self::E9V26),
            14 => Ok(Self::E10V27),
            15 => Ok(Self::E11V28),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum E10Apb0Traceconfig55cTraceportspeed {
    #[doc = "32MHz: 32 MHz Trace Port clock (TRACECLK = 16 MHz)<br>"]
    E032mhz,
    #[doc = "16MHz: 16 MHz Trace Port clock (TRACECLK = 8 MHz)<br>"]
    E116mhz,
    #[doc = "8MHz: 8 MHz Trace Port clock (TRACECLK = 4 MHz)<br>"]
    E28mhz,
    #[doc = "4MHz: 4 MHz Trace Port clock (TRACECLK = 2 MHz)<br>"]
    E34mhz,
}
impl From<E10Apb0Traceconfig55cTraceportspeed> for u8 {
    fn from(value: E10Apb0Traceconfig55cTraceportspeed) -> u8 {
        match value {
            E10Apb0Traceconfig55cTraceportspeed::E032mhz => 0,
            E10Apb0Traceconfig55cTraceportspeed::E116mhz => 1,
            E10Apb0Traceconfig55cTraceportspeed::E28mhz => 2,
            E10Apb0Traceconfig55cTraceportspeed::E34mhz => 3,
        }
    }
}
impl From<u8> for E10Apb0Traceconfig55cTraceportspeed {
    fn from(value: u8) -> E10Apb0Traceconfig55cTraceportspeed {
        match value {
            0 => Self::E032mhz,
            1 => Self::E116mhz,
            2 => Self::E28mhz,
            3 => Self::E34mhz,
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E11Apb0Traceconfig55cTracemux {
    #[doc = "GPIO: GPIOs multiplexed onto all trace-pins<br>"]
    E0Gpio,
    #[doc = "Serial: SWO multiplexed onto P0.18, GPIO multiplexed onto other trace pins<br>"]
    E1Serial,
    #[doc = "Parallel: TRACECLK and TRACEDATA multiplexed onto P0.20, P0.18, P0.16, P0.15 and P0.14.<br>"]
    E2Parallel,
}
impl From<E11Apb0Traceconfig55cTracemux> for u8 {
    fn from(value: E11Apb0Traceconfig55cTracemux) -> u8 {
        match value {
            E11Apb0Traceconfig55cTracemux::E0Gpio => 0,
            E11Apb0Traceconfig55cTracemux::E1Serial => 1,
            E11Apb0Traceconfig55cTracemux::E2Parallel => 2,
        }
    }
}
impl TryFrom<u8> for E11Apb0Traceconfig55cTracemux {
    type Error = ();
    fn try_from(
        value: u8,
    ) -> Result<E11Apb0Traceconfig55cTracemux, Self::Error> {
        match value {
            0 => Ok(Self::E0Gpio),
            1 => Ok(Self::E1Serial),
            2 => Ok(Self::E2Parallel),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E12RadioTxpower50cTxpower {
    #[doc = "0dBm: 0 dBm<br>"]
    E00dbm,
    #[doc = "Pos3dBm: +3 dBm<br>"]
    E1Pos3dbm,
    #[doc = "Pos4dBm: +4 dBm<br>"]
    E2Pos4dbm,
    #[doc = "Neg40dBm: -40 dBm<br>"]
    E3Neg40dbm,
    #[doc = "Neg20dBm: -20 dBm<br>"]
    E4Neg20dbm,
    #[doc = "Neg16dBm: -16 dBm<br>"]
    E5Neg16dbm,
    #[doc = "Neg12dBm: -12 dBm<br>"]
    E6Neg12dbm,
    #[doc = "Neg8dBm: -8 dBm<br>"]
    E7Neg8dbm,
    #[doc = "Neg4dBm: -4 dBm<br>"]
    E8Neg4dbm,
}
impl From<E12RadioTxpower50cTxpower> for u8 {
    fn from(value: E12RadioTxpower50cTxpower) -> u8 {
        match value {
            E12RadioTxpower50cTxpower::E00dbm => 0,
            E12RadioTxpower50cTxpower::E1Pos3dbm => 3,
            E12RadioTxpower50cTxpower::E2Pos4dbm => 4,
            E12RadioTxpower50cTxpower::E3Neg40dbm => 216,
            E12RadioTxpower50cTxpower::E4Neg20dbm => 236,
            E12RadioTxpower50cTxpower::E5Neg16dbm => 240,
            E12RadioTxpower50cTxpower::E6Neg12dbm => 244,
            E12RadioTxpower50cTxpower::E7Neg8dbm => 248,
            E12RadioTxpower50cTxpower::E8Neg4dbm => 252,
        }
    }
}
impl TryFrom<u8> for E12RadioTxpower50cTxpower {
    type Error = ();
    fn try_from(value: u8) -> Result<E12RadioTxpower50cTxpower, Self::Error> {
        match value {
            0 => Ok(Self::E00dbm),
            3 => Ok(Self::E1Pos3dbm),
            4 => Ok(Self::E2Pos4dbm),
            216 => Ok(Self::E3Neg40dbm),
            236 => Ok(Self::E4Neg20dbm),
            240 => Ok(Self::E5Neg16dbm),
            244 => Ok(Self::E6Neg12dbm),
            248 => Ok(Self::E7Neg8dbm),
            252 => Ok(Self::E8Neg4dbm),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E13RadioMode510Mode {
    #[doc = "Nrf_1Mbit: 1 Mbit/s Nordic proprietary radio mode<br>"]
    E0Nrf1mbit,
    #[doc = "Nrf_2Mbit: 2 Mbit/s Nordic proprietary radio mode<br>"]
    E1Nrf2mbit,
    #[doc = "Nrf_250Kbit: Deprecated enumerator -  250 kbit/s Nordic proprietary radio mode<br>"]
    E2Nrf250kbit,
    #[doc = "Ble_1Mbit: 1 Mbit/s Bluetooth Low Energy<br>"]
    E3Ble1mbit,
    #[doc = "Ble_2Mbit: 2 Mbit/s Bluetooth Low Energy<br>"]
    E4Ble2mbit,
}
impl From<E13RadioMode510Mode> for u8 {
    fn from(value: E13RadioMode510Mode) -> u8 {
        match value {
            E13RadioMode510Mode::E0Nrf1mbit => 0,
            E13RadioMode510Mode::E1Nrf2mbit => 1,
            E13RadioMode510Mode::E2Nrf250kbit => 2,
            E13RadioMode510Mode::E3Ble1mbit => 3,
            E13RadioMode510Mode::E4Ble2mbit => 4,
        }
    }
}
impl TryFrom<u8> for E13RadioMode510Mode {
    type Error = ();
    fn try_from(value: u8) -> Result<E13RadioMode510Mode, Self::Error> {
        match value {
            0 => Ok(Self::E0Nrf1mbit),
            1 => Ok(Self::E1Nrf2mbit),
            2 => Ok(Self::E2Nrf250kbit),
            3 => Ok(Self::E3Ble1mbit),
            4 => Ok(Self::E4Ble2mbit),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E14RadioCrccnf534Len {
    #[doc = "Disabled: CRC length is zero and CRC calculation is disabled<br>"]
    E0Disabled,
    #[doc = "One: CRC length is one byte and CRC calculation is enabled<br>"]
    E1One,
    #[doc = "Two: CRC length is two bytes and CRC calculation is enabled<br>"]
    E2Two,
    #[doc = "Three: CRC length is three bytes and CRC calculation is enabled<br>"]
    E3Three,
}
impl From<E14RadioCrccnf534Len> for u8 {
    fn from(value: E14RadioCrccnf534Len) -> u8 {
        match value {
            E14RadioCrccnf534Len::E0Disabled => 0,
            E14RadioCrccnf534Len::E1One => 1,
            E14RadioCrccnf534Len::E2Two => 2,
            E14RadioCrccnf534Len::E3Three => 3,
        }
    }
}
impl From<u8> for E14RadioCrccnf534Len {
    fn from(value: u8) -> E14RadioCrccnf534Len {
        match value {
            0 => Self::E0Disabled,
            1 => Self::E1One,
            2 => Self::E2Two,
            3 => Self::E3Three,
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E15RadioState550State {
    #[doc = "Disabled: RADIO is in the Disabled state<br>"]
    E0Disabled,
    #[doc = "RxRu: RADIO is in the RXRU state<br>"]
    E1Rxru,
    #[doc = "RxIdle: RADIO is in the RXIDLE state<br>"]
    E2Rxidle,
    #[doc = "Rx: RADIO is in the RX state<br>"]
    E3Rx,
    #[doc = "RxDisable: RADIO is in the RXDISABLED state<br>"]
    E4Rxdisable,
    #[doc = "TxRu: RADIO is in the TXRU state<br>"]
    E5Txru,
    #[doc = "TxIdle: RADIO is in the TXIDLE state<br>"]
    E6Txidle,
    #[doc = "Tx: RADIO is in the TX state<br>"]
    E7Tx,
    #[doc = "TxDisable: RADIO is in the TXDISABLED state<br>"]
    E8Txdisable,
}
impl From<E15RadioState550State> for u8 {
    fn from(value: E15RadioState550State) -> u8 {
        match value {
            E15RadioState550State::E0Disabled => 0,
            E15RadioState550State::E1Rxru => 1,
            E15RadioState550State::E2Rxidle => 2,
            E15RadioState550State::E3Rx => 3,
            E15RadioState550State::E4Rxdisable => 4,
            E15RadioState550State::E5Txru => 9,
            E15RadioState550State::E6Txidle => 10,
            E15RadioState550State::E7Tx => 11,
            E15RadioState550State::E8Txdisable => 12,
        }
    }
}
impl TryFrom<u8> for E15RadioState550State {
    type Error = ();
    fn try_from(value: u8) -> Result<E15RadioState550State, Self::Error> {
        match value {
            0 => Ok(Self::E0Disabled),
            1 => Ok(Self::E1Rxru),
            2 => Ok(Self::E2Rxidle),
            3 => Ok(Self::E3Rx),
            4 => Ok(Self::E4Rxdisable),
            9 => Ok(Self::E5Txru),
            10 => Ok(Self::E6Txidle),
            11 => Ok(Self::E7Tx),
            12 => Ok(Self::E8Txdisable),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E16RadioModecnf0650Dtx {
    #[doc = "B1: Transmit '1'<br>"]
    E0B1,
    #[doc = "B0: Transmit '0'<br>"]
    E1B0,
    #[doc = "Center: Transmit center frequency<br>"]
    E2Center,
}
impl From<E16RadioModecnf0650Dtx> for u8 {
    fn from(value: E16RadioModecnf0650Dtx) -> u8 {
        match value {
            E16RadioModecnf0650Dtx::E0B1 => 0,
            E16RadioModecnf0650Dtx::E1B0 => 1,
            E16RadioModecnf0650Dtx::E2Center => 2,
        }
    }
}
impl TryFrom<u8> for E16RadioModecnf0650Dtx {
    type Error = ();
    fn try_from(value: u8) -> Result<E16RadioModecnf0650Dtx, Self::Error> {
        match value {
            0 => Ok(Self::E0B1),
            1 => Ok(Self::E1B0),
            2 => Ok(Self::E2Center),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E17Apb2Enable500Enable {
    #[doc = "Disabled: Disable UARTE<br>Disabled: Disable UART<br>"]
    E0Disabled,
    #[doc = "Enabled: Enable UART<br>"]
    E1Enabled,
    #[doc = "Enabled: Enable UARTE<br>"]
    E2Enabled,
}
impl From<E17Apb2Enable500Enable> for u8 {
    fn from(value: E17Apb2Enable500Enable) -> u8 {
        match value {
            E17Apb2Enable500Enable::E0Disabled => 0,
            E17Apb2Enable500Enable::E1Enabled => 4,
            E17Apb2Enable500Enable::E2Enabled => 8,
        }
    }
}
impl TryFrom<u8> for E17Apb2Enable500Enable {
    type Error = ();
    fn try_from(value: u8) -> Result<E17Apb2Enable500Enable, Self::Error> {
        match value {
            0 => Ok(Self::E0Disabled),
            4 => Ok(Self::E1Enabled),
            8 => Ok(Self::E2Enabled),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E18Apb2Baudrate524Baudrate {
    #[doc = "Baud1200: 1200 baud (actual rate: 1205)<br>Baud1200: 1200 baud (actual rate: 1205)<br>"]
    E0Baud1200,
    #[doc = "Baud2400: 2400 baud (actual rate: 2396)<br>Baud2400: 2400 baud (actual rate: 2396)<br>"]
    E1Baud2400,
    #[doc = "Baud4800: 4800 baud (actual rate: 4808)<br>Baud4800: 4800 baud (actual rate: 4808)<br>"]
    E2Baud4800,
    #[doc = "Baud9600: 9600 baud (actual rate: 9598)<br>Baud9600: 9600 baud (actual rate: 9598)<br>"]
    E3Baud9600,
    #[doc = "Baud14400: 14400 baud (actual rate: 14401)<br>"]
    E4Baud14400,
    #[doc = "Baud14400: 14400 baud (actual rate: 14414)<br>"]
    E5Baud14400,
    #[doc = "Baud19200: 19200 baud (actual rate: 19208)<br>Baud19200: 19200 baud (actual rate: 19208)<br>"]
    E6Baud19200,
    #[doc = "Baud28800: 28800 baud (actual rate: 28777)<br>"]
    E7Baud28800,
    #[doc = "Baud28800: 28800 baud (actual rate: 28829)<br>"]
    E8Baud28800,
    #[doc = "Baud31250: 31250 baud<br>Baud31250: 31250 baud<br>"]
    E9Baud31250,
    #[doc = "Baud38400: 38400 baud (actual rate: 38369)<br>"]
    E10Baud38400,
    #[doc = "Baud38400: 38400 baud (actual rate: 38462)<br>"]
    E11Baud38400,
    #[doc = "Baud56000: 56000 baud (actual rate: 55944)<br>Baud56000: 56000 baud (actual rate: 55944)<br>"]
    E12Baud56000,
    #[doc = "Baud57600: 57600 baud (actual rate: 57554)<br>"]
    E13Baud57600,
    #[doc = "Baud57600: 57600 baud (actual rate: 57762)<br>"]
    E14Baud57600,
    #[doc = "Baud76800: 76800 baud (actual rate: 76923)<br>Baud76800: 76800 baud (actual rate: 76923)<br>"]
    E15Baud76800,
    #[doc = "Baud115200: 115200 baud (actual rate: 115108)<br>"]
    E16Baud115200,
    #[doc = "Baud115200: 115200 baud (actual rate: 115942)<br>"]
    E17Baud115200,
    #[doc = "Baud230400: 230400 baud (actual rate: 231884)<br>"]
    E18Baud230400,
    #[doc = "Baud230400: 230400 baud (actual rate: 231884)<br>"]
    E19Baud230400,
    #[doc = "Baud250000: 250000 baud<br>Baud250000: 250000 baud<br>"]
    E20Baud250000,
    #[doc = "Baud460800: 460800 baud (actual rate: 457143)<br>"]
    E21Baud460800,
    #[doc = "Baud460800: 460800 baud (actual rate: 470588)<br>"]
    E22Baud460800,
    #[doc = "Baud921600: 921600 baud (actual rate: 941176)<br>"]
    E23Baud921600,
    #[doc = "Baud921600: 921600 baud (actual rate: 941176)<br>"]
    E24Baud921600,
    #[doc = "Baud1M: 1Mega baud<br>Baud1M: 1Mega baud<br>"]
    E25Baud1m,
}
impl From<E18Apb2Baudrate524Baudrate> for u32 {
    fn from(value: E18Apb2Baudrate524Baudrate) -> u32 {
        match value {
            E18Apb2Baudrate524Baudrate::E0Baud1200 => 323584,
            E18Apb2Baudrate524Baudrate::E1Baud2400 => 643072,
            E18Apb2Baudrate524Baudrate::E2Baud4800 => 1290240,
            E18Apb2Baudrate524Baudrate::E3Baud9600 => 2576384,
            E18Apb2Baudrate524Baudrate::E4Baud14400 => 3862528,
            E18Apb2Baudrate524Baudrate::E5Baud14400 => 3866624,
            E18Apb2Baudrate524Baudrate::E6Baud19200 => 5152768,
            E18Apb2Baudrate524Baudrate::E7Baud28800 => 7716864,
            E18Apb2Baudrate524Baudrate::E8Baud28800 => 7729152,
            E18Apb2Baudrate524Baudrate::E9Baud31250 => 8388608,
            E18Apb2Baudrate524Baudrate::E10Baud38400 => 10289152,
            E18Apb2Baudrate524Baudrate::E11Baud38400 => 10309632,
            E18Apb2Baudrate524Baudrate::E12Baud56000 => 15007744,
            E18Apb2Baudrate524Baudrate::E13Baud57600 => 15400960,
            E18Apb2Baudrate524Baudrate::E14Baud57600 => 15462400,
            E18Apb2Baudrate524Baudrate::E15Baud76800 => 20615168,
            E18Apb2Baudrate524Baudrate::E16Baud115200 => 30801920,
            E18Apb2Baudrate524Baudrate::E17Baud115200 => 30924800,
            E18Apb2Baudrate524Baudrate::E18Baud230400 => 61845504,
            E18Apb2Baudrate524Baudrate::E19Baud230400 => 61865984,
            E18Apb2Baudrate524Baudrate::E20Baud250000 => 67108864,
            E18Apb2Baudrate524Baudrate::E21Baud460800 => 121634816,
            E18Apb2Baudrate524Baudrate::E22Baud460800 => 123695104,
            E18Apb2Baudrate524Baudrate::E23Baud921600 => 247386112,
            E18Apb2Baudrate524Baudrate::E24Baud921600 => 251658240,
            E18Apb2Baudrate524Baudrate::E25Baud1m => 268435456,
        }
    }
}
impl TryFrom<u32> for E18Apb2Baudrate524Baudrate {
    type Error = ();
    fn try_from(value: u32) -> Result<E18Apb2Baudrate524Baudrate, Self::Error> {
        match value {
            323584 => Ok(Self::E0Baud1200),
            643072 => Ok(Self::E1Baud2400),
            1290240 => Ok(Self::E2Baud4800),
            2576384 => Ok(Self::E3Baud9600),
            3862528 => Ok(Self::E4Baud14400),
            3866624 => Ok(Self::E5Baud14400),
            5152768 => Ok(Self::E6Baud19200),
            7716864 => Ok(Self::E7Baud28800),
            7729152 => Ok(Self::E8Baud28800),
            8388608 => Ok(Self::E9Baud31250),
            10289152 => Ok(Self::E10Baud38400),
            10309632 => Ok(Self::E11Baud38400),
            15007744 => Ok(Self::E12Baud56000),
            15400960 => Ok(Self::E13Baud57600),
            15462400 => Ok(Self::E14Baud57600),
            20615168 => Ok(Self::E15Baud76800),
            30801920 => Ok(Self::E16Baud115200),
            30924800 => Ok(Self::E17Baud115200),
            61845504 => Ok(Self::E18Baud230400),
            61865984 => Ok(Self::E19Baud230400),
            67108864 => Ok(Self::E20Baud250000),
            121634816 => Ok(Self::E21Baud460800),
            123695104 => Ok(Self::E22Baud460800),
            247386112 => Ok(Self::E23Baud921600),
            251658240 => Ok(Self::E24Baud921600),
            268435456 => Ok(Self::E25Baud1m),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E19Apb2Config56cParity {
    #[doc = "Excluded: Exclude parity bit<br>Excluded: Exclude parity bit<br>"]
    E0Excluded,
    #[doc = "Included: Include parity bit<br>Included: Include parity bit<br>"]
    E1Included,
}
impl From<E19Apb2Config56cParity> for u8 {
    fn from(value: E19Apb2Config56cParity) -> u8 {
        match value {
            E19Apb2Config56cParity::E0Excluded => 0,
            E19Apb2Config56cParity::E1Included => 7,
        }
    }
}
impl TryFrom<u8> for E19Apb2Config56cParity {
    type Error = ();
    fn try_from(value: u8) -> Result<E19Apb2Config56cParity, Self::Error> {
        match value {
            0 => Ok(Self::E0Excluded),
            7 => Ok(Self::E1Included),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E20Apb3Semstat400Semstat {
    #[doc = "Free: Semaphore is free<br>"]
    E0Free,
    #[doc = "CPU: Semaphore is assigned to CPU<br>"]
    E1Cpu,
    #[doc = "SPIS: Semaphore is assigned to SPI slave<br>"]
    E2Spis,
    #[doc = "CPUPending: Semaphore is assigned to SPI but a handover to the CPU is pending<br>"]
    E3Cpupending,
}
impl From<E20Apb3Semstat400Semstat> for u8 {
    fn from(value: E20Apb3Semstat400Semstat) -> u8 {
        match value {
            E20Apb3Semstat400Semstat::E0Free => 0,
            E20Apb3Semstat400Semstat::E1Cpu => 1,
            E20Apb3Semstat400Semstat::E2Spis => 2,
            E20Apb3Semstat400Semstat::E3Cpupending => 3,
        }
    }
}
impl From<u8> for E20Apb3Semstat400Semstat {
    fn from(value: u8) -> E20Apb3Semstat400Semstat {
        match value {
            0 => Self::E0Free,
            1 => Self::E1Cpu,
            2 => Self::E2Spis,
            3 => Self::E3Cpupending,
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E21Apb3Enable500Enable {
    #[doc = "Disabled: Disable SPI<br>Disabled: Disable SPI slave<br>Disabled: Disable TWIS<br>Disabled: Disable SPIM<br>Disabled: Disable TWI<br>Disabled: Disable TWIM<br>"]
    E0Disabled,
    #[doc = "Enabled: Enable SPI<br>"]
    E1Enabled,
    #[doc = "Enabled: Enable SPI slave<br>"]
    E2Enabled,
    #[doc = "Enabled: Enable TWI<br>"]
    E3Enabled,
    #[doc = "Enabled: Enable TWIM<br>"]
    E4Enabled,
    #[doc = "Enabled: Enable SPIM<br>"]
    E5Enabled,
    #[doc = "Enabled: Enable TWIS<br>"]
    E6Enabled,
}
impl From<E21Apb3Enable500Enable> for u8 {
    fn from(value: E21Apb3Enable500Enable) -> u8 {
        match value {
            E21Apb3Enable500Enable::E0Disabled => 0,
            E21Apb3Enable500Enable::E1Enabled => 1,
            E21Apb3Enable500Enable::E2Enabled => 2,
            E21Apb3Enable500Enable::E3Enabled => 5,
            E21Apb3Enable500Enable::E4Enabled => 6,
            E21Apb3Enable500Enable::E5Enabled => 7,
            E21Apb3Enable500Enable::E6Enabled => 9,
        }
    }
}
impl TryFrom<u8> for E21Apb3Enable500Enable {
    type Error = ();
    fn try_from(value: u8) -> Result<E21Apb3Enable500Enable, Self::Error> {
        match value {
            0 => Ok(Self::E0Disabled),
            1 => Ok(Self::E1Enabled),
            2 => Ok(Self::E2Enabled),
            5 => Ok(Self::E3Enabled),
            6 => Ok(Self::E4Enabled),
            7 => Ok(Self::E5Enabled),
            9 => Ok(Self::E6Enabled),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E22Apb3Frequency524Frequency {
    #[doc = "K100: 100 kbps<br>K100: 100 kbps<br>"]
    E0K100,
    #[doc = "K125: 125 kbps<br>K125: 125 kbps<br>"]
    E1K125,
    #[doc = "K250: 250 kbps<br>K250: 250 kbps<br>K250: 250 kbps<br>K250: 250 kbps<br>"]
    E2K250,
    #[doc = "K400: 400 kbps<br>"]
    E3K400,
    #[doc = "K400: 400 kbps (actual rate 410.256 kbps)<br>"]
    E4K400,
    #[doc = "K500: 500 kbps<br>K500: 500 kbps<br>"]
    E5K500,
    #[doc = "M1: 1 Mbps<br>M1: 1 Mbps<br>"]
    E6M1,
    #[doc = "M2: 2 Mbps<br>M2: 2 Mbps<br>"]
    E7M2,
    #[doc = "M4: 4 Mbps<br>M4: 4 Mbps<br>"]
    E8M4,
    #[doc = "M8: 8 Mbps<br>M8: 8 Mbps<br>"]
    E9M8,
}
impl From<E22Apb3Frequency524Frequency> for u32 {
    fn from(value: E22Apb3Frequency524Frequency) -> u32 {
        match value {
            E22Apb3Frequency524Frequency::E0K100 => 26738688,
            E22Apb3Frequency524Frequency::E1K125 => 33554432,
            E22Apb3Frequency524Frequency::E2K250 => 67108864,
            E22Apb3Frequency524Frequency::E3K400 => 104857600,
            E22Apb3Frequency524Frequency::E4K400 => 107479040,
            E22Apb3Frequency524Frequency::E5K500 => 134217728,
            E22Apb3Frequency524Frequency::E6M1 => 268435456,
            E22Apb3Frequency524Frequency::E7M2 => 536870912,
            E22Apb3Frequency524Frequency::E8M4 => 1073741824,
            E22Apb3Frequency524Frequency::E9M8 => 2147483648,
        }
    }
}
impl TryFrom<u32> for E22Apb3Frequency524Frequency {
    type Error = ();
    fn try_from(
        value: u32,
    ) -> Result<E22Apb3Frequency524Frequency, Self::Error> {
        match value {
            26738688 => Ok(Self::E0K100),
            33554432 => Ok(Self::E1K125),
            67108864 => Ok(Self::E2K250),
            104857600 => Ok(Self::E3K400),
            107479040 => Ok(Self::E4K400),
            134217728 => Ok(Self::E5K500),
            268435456 => Ok(Self::E6M1),
            536870912 => Ok(Self::E7M2),
            1073741824 => Ok(Self::E8M4),
            2147483648 => Ok(Self::E9M8),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E23Apb3RxdListcList {
    #[doc = "Disabled: Disable EasyDMA list<br>Disabled: Disable EasyDMA list<br>"]
    E0Disabled,
    #[doc = "ArrayList: Use array list<br>ArrayList: Use array list<br>"]
    E1Arraylist,
}
impl From<E23Apb3RxdListcList> for u8 {
    fn from(value: E23Apb3RxdListcList) -> u8 {
        match value {
            E23Apb3RxdListcList::E0Disabled => 0,
            E23Apb3RxdListcList::E1Arraylist => 1,
        }
    }
}
impl TryFrom<u8> for E23Apb3RxdListcList {
    type Error = ();
    fn try_from(value: u8) -> Result<E23Apb3RxdListcList, Self::Error> {
        match value {
            0 => Ok(Self::E0Disabled),
            1 => Ok(Self::E1Arraylist),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E24Apb3TxdListcList {
    #[doc = "Disabled: Disable EasyDMA list<br>Disabled: Disable EasyDMA list<br>"]
    E0Disabled,
    #[doc = "ArrayList: Use array list<br>ArrayList: Use array list<br>"]
    E1Arraylist,
}
impl From<E24Apb3TxdListcList> for u8 {
    fn from(value: E24Apb3TxdListcList) -> u8 {
        match value {
            E24Apb3TxdListcList::E0Disabled => 0,
            E24Apb3TxdListcList::E1Arraylist => 1,
        }
    }
}
impl TryFrom<u8> for E24Apb3TxdListcList {
    type Error = ();
    fn try_from(value: u8) -> Result<E24Apb3TxdListcList, Self::Error> {
        match value {
            0 => Ok(Self::E0Disabled),
            1 => Ok(Self::E1Arraylist),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E25NfctFramedelaymode50cFramedelaymode {
    #[doc = "FreeRun: Transmission is independent of frame timer and will start when the STARTTX task is triggered. No timeout.<br>"]
    E0Freerun,
    #[doc = "Window: Frame is transmitted between FRAMEDELAYMIN and FRAMEDELAYMAX<br>"]
    E1Window,
    #[doc = "ExactVal: Frame is transmitted exactly at FRAMEDELAYMAX<br>"]
    E2Exactval,
    #[doc = "WindowGrid: Frame is transmitted on a bit grid between FRAMEDELAYMIN and FRAMEDELAYMAX<br>"]
    E3Windowgrid,
}
impl From<E25NfctFramedelaymode50cFramedelaymode> for u8 {
    fn from(value: E25NfctFramedelaymode50cFramedelaymode) -> u8 {
        match value {
            E25NfctFramedelaymode50cFramedelaymode::E0Freerun => 0,
            E25NfctFramedelaymode50cFramedelaymode::E1Window => 1,
            E25NfctFramedelaymode50cFramedelaymode::E2Exactval => 2,
            E25NfctFramedelaymode50cFramedelaymode::E3Windowgrid => 3,
        }
    }
}
impl From<u8> for E25NfctFramedelaymode50cFramedelaymode {
    fn from(value: u8) -> E25NfctFramedelaymode50cFramedelaymode {
        match value {
            0 => Self::E0Freerun,
            1 => Self::E1Window,
            2 => Self::E2Exactval,
            3 => Self::E3Windowgrid,
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E26NfctSensres5a0Bitframesdd {
    #[doc = "SDD00000: SDD pattern 00000<br>"]
    E0Sdd00000,
    #[doc = "SDD00001: SDD pattern 00001<br>"]
    E1Sdd00001,
    #[doc = "SDD00010: SDD pattern 00010<br>"]
    E2Sdd00010,
    #[doc = "SDD00100: SDD pattern 00100<br>"]
    E3Sdd00100,
    #[doc = "SDD01000: SDD pattern 01000<br>"]
    E4Sdd01000,
    #[doc = "SDD10000: SDD pattern 10000<br>"]
    E5Sdd10000,
}
impl From<E26NfctSensres5a0Bitframesdd> for u8 {
    fn from(value: E26NfctSensres5a0Bitframesdd) -> u8 {
        match value {
            E26NfctSensres5a0Bitframesdd::E0Sdd00000 => 0,
            E26NfctSensres5a0Bitframesdd::E1Sdd00001 => 1,
            E26NfctSensres5a0Bitframesdd::E2Sdd00010 => 2,
            E26NfctSensres5a0Bitframesdd::E3Sdd00100 => 4,
            E26NfctSensres5a0Bitframesdd::E4Sdd01000 => 8,
            E26NfctSensres5a0Bitframesdd::E5Sdd10000 => 16,
        }
    }
}
impl TryFrom<u8> for E26NfctSensres5a0Bitframesdd {
    type Error = ();
    fn try_from(
        value: u8,
    ) -> Result<E26NfctSensres5a0Bitframesdd, Self::Error> {
        match value {
            0 => Ok(Self::E0Sdd00000),
            1 => Ok(Self::E1Sdd00001),
            2 => Ok(Self::E2Sdd00010),
            4 => Ok(Self::E3Sdd00100),
            8 => Ok(Self::E4Sdd01000),
            16 => Ok(Self::E5Sdd10000),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E27NfctSensres5a0Nfcidsize {
    #[doc = "NFCID1Single: NFCID1 size: single (4 bytes)<br>"]
    E0Nfcid1single,
    #[doc = "NFCID1Double: NFCID1 size: double (7 bytes)<br>"]
    E1Nfcid1double,
    #[doc = "NFCID1Triple: NFCID1 size: triple (10 bytes)<br>"]
    E2Nfcid1triple,
}
impl From<E27NfctSensres5a0Nfcidsize> for u8 {
    fn from(value: E27NfctSensres5a0Nfcidsize) -> u8 {
        match value {
            E27NfctSensres5a0Nfcidsize::E0Nfcid1single => 0,
            E27NfctSensres5a0Nfcidsize::E1Nfcid1double => 1,
            E27NfctSensres5a0Nfcidsize::E2Nfcid1triple => 2,
        }
    }
}
impl TryFrom<u8> for E27NfctSensres5a0Nfcidsize {
    type Error = ();
    fn try_from(value: u8) -> Result<E27NfctSensres5a0Nfcidsize, Self::Error> {
        match value {
            0 => Ok(Self::E0Nfcid1single),
            1 => Ok(Self::E1Nfcid1double),
            2 => Ok(Self::E2Nfcid1triple),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E28GpioteConfign510Mode {
    #[doc = "Disabled: Disabled. Pin specified by PSEL will not be acquired by the GPIOTE module.<br>"]
    E0Disabled,
    #[doc = "Event: Event mode<br>"]
    E1Event,
    #[doc = "Task: Task mode<br>"]
    E2Task,
}
impl From<E28GpioteConfign510Mode> for u8 {
    fn from(value: E28GpioteConfign510Mode) -> u8 {
        match value {
            E28GpioteConfign510Mode::E0Disabled => 0,
            E28GpioteConfign510Mode::E1Event => 1,
            E28GpioteConfign510Mode::E2Task => 3,
        }
    }
}
impl TryFrom<u8> for E28GpioteConfign510Mode {
    type Error = ();
    fn try_from(value: u8) -> Result<E28GpioteConfign510Mode, Self::Error> {
        match value {
            0 => Ok(Self::E0Disabled),
            1 => Ok(Self::E1Event),
            3 => Ok(Self::E2Task),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E29GpioteConfign510Polarity {
    #[doc = "None: Task mode: No effect on pin from OUT\\[n\\] task. Event mode: no IN\\[n\\] event generated on pin activity.<br>"]
    E0None,
    #[doc = "LoToHi: Task mode: Set pin from OUT\\[n\\] task. Event mode: Generate IN\\[n\\] event when rising edge on pin.<br>"]
    E1Lotohi,
    #[doc = "HiToLo: Task mode: Clear pin from OUT\\[n\\] task. Event mode: Generate IN\\[n\\] event when falling edge on pin.<br>"]
    E2Hitolo,
    #[doc = "Toggle: Task mode: Toggle pin from OUT\\[n\\]. Event mode: Generate IN\\[n\\] when any change on pin.<br>"]
    E3Toggle,
}
impl From<E29GpioteConfign510Polarity> for u8 {
    fn from(value: E29GpioteConfign510Polarity) -> u8 {
        match value {
            E29GpioteConfign510Polarity::E0None => 0,
            E29GpioteConfign510Polarity::E1Lotohi => 1,
            E29GpioteConfign510Polarity::E2Hitolo => 2,
            E29GpioteConfign510Polarity::E3Toggle => 3,
        }
    }
}
impl From<u8> for E29GpioteConfign510Polarity {
    fn from(value: u8) -> E29GpioteConfign510Polarity {
        match value {
            0 => Self::E0None,
            1 => Self::E1Lotohi,
            2 => Self::E2Hitolo,
            3 => Self::E3Toggle,
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E30SaadcChnPselp0Pselp {
    #[doc = "NC: Not connected<br>"]
    E0Nc,
    #[doc = "AnalogInput0: AIN0<br>"]
    E1Analoginput0,
    #[doc = "AnalogInput1: AIN1<br>"]
    E2Analoginput1,
    #[doc = "AnalogInput2: AIN2<br>"]
    E3Analoginput2,
    #[doc = "AnalogInput3: AIN3<br>"]
    E4Analoginput3,
    #[doc = "AnalogInput4: AIN4<br>"]
    E5Analoginput4,
    #[doc = "AnalogInput5: AIN5<br>"]
    E6Analoginput5,
    #[doc = "AnalogInput6: AIN6<br>"]
    E7Analoginput6,
    #[doc = "AnalogInput7: AIN7<br>"]
    E8Analoginput7,
    #[doc = "VDD: VDD<br>"]
    E9Vdd,
}
impl From<E30SaadcChnPselp0Pselp> for u8 {
    fn from(value: E30SaadcChnPselp0Pselp) -> u8 {
        match value {
            E30SaadcChnPselp0Pselp::E0Nc => 0,
            E30SaadcChnPselp0Pselp::E1Analoginput0 => 1,
            E30SaadcChnPselp0Pselp::E2Analoginput1 => 2,
            E30SaadcChnPselp0Pselp::E3Analoginput2 => 3,
            E30SaadcChnPselp0Pselp::E4Analoginput3 => 4,
            E30SaadcChnPselp0Pselp::E5Analoginput4 => 5,
            E30SaadcChnPselp0Pselp::E6Analoginput5 => 6,
            E30SaadcChnPselp0Pselp::E7Analoginput6 => 7,
            E30SaadcChnPselp0Pselp::E8Analoginput7 => 8,
            E30SaadcChnPselp0Pselp::E9Vdd => 9,
        }
    }
}
impl TryFrom<u8> for E30SaadcChnPselp0Pselp {
    type Error = ();
    fn try_from(value: u8) -> Result<E30SaadcChnPselp0Pselp, Self::Error> {
        match value {
            0 => Ok(Self::E0Nc),
            1 => Ok(Self::E1Analoginput0),
            2 => Ok(Self::E2Analoginput1),
            3 => Ok(Self::E3Analoginput2),
            4 => Ok(Self::E4Analoginput3),
            5 => Ok(Self::E5Analoginput4),
            6 => Ok(Self::E6Analoginput5),
            7 => Ok(Self::E7Analoginput6),
            8 => Ok(Self::E8Analoginput7),
            9 => Ok(Self::E9Vdd),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E31SaadcChnPseln4Pseln {
    #[doc = "NC: Not connected<br>"]
    E0Nc,
    #[doc = "AnalogInput0: AIN0<br>"]
    E1Analoginput0,
    #[doc = "AnalogInput1: AIN1<br>"]
    E2Analoginput1,
    #[doc = "AnalogInput2: AIN2<br>"]
    E3Analoginput2,
    #[doc = "AnalogInput3: AIN3<br>"]
    E4Analoginput3,
    #[doc = "AnalogInput4: AIN4<br>"]
    E5Analoginput4,
    #[doc = "AnalogInput5: AIN5<br>"]
    E6Analoginput5,
    #[doc = "AnalogInput6: AIN6<br>"]
    E7Analoginput6,
    #[doc = "AnalogInput7: AIN7<br>"]
    E8Analoginput7,
    #[doc = "VDD: VDD<br>"]
    E9Vdd,
}
impl From<E31SaadcChnPseln4Pseln> for u8 {
    fn from(value: E31SaadcChnPseln4Pseln) -> u8 {
        match value {
            E31SaadcChnPseln4Pseln::E0Nc => 0,
            E31SaadcChnPseln4Pseln::E1Analoginput0 => 1,
            E31SaadcChnPseln4Pseln::E2Analoginput1 => 2,
            E31SaadcChnPseln4Pseln::E3Analoginput2 => 3,
            E31SaadcChnPseln4Pseln::E4Analoginput3 => 4,
            E31SaadcChnPseln4Pseln::E5Analoginput4 => 5,
            E31SaadcChnPseln4Pseln::E6Analoginput5 => 6,
            E31SaadcChnPseln4Pseln::E7Analoginput6 => 7,
            E31SaadcChnPseln4Pseln::E8Analoginput7 => 8,
            E31SaadcChnPseln4Pseln::E9Vdd => 9,
        }
    }
}
impl TryFrom<u8> for E31SaadcChnPseln4Pseln {
    type Error = ();
    fn try_from(value: u8) -> Result<E31SaadcChnPseln4Pseln, Self::Error> {
        match value {
            0 => Ok(Self::E0Nc),
            1 => Ok(Self::E1Analoginput0),
            2 => Ok(Self::E2Analoginput1),
            3 => Ok(Self::E3Analoginput2),
            4 => Ok(Self::E4Analoginput3),
            5 => Ok(Self::E5Analoginput4),
            6 => Ok(Self::E6Analoginput5),
            7 => Ok(Self::E7Analoginput6),
            8 => Ok(Self::E8Analoginput7),
            9 => Ok(Self::E9Vdd),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E32SaadcChnConfig8Resp {
    #[doc = "Bypass: Bypass resistor ladder<br>"]
    E0Bypass,
    #[doc = "Pulldown: Pull-down to GND<br>"]
    E1Pulldown,
    #[doc = "Pullup: Pull-up to VDD<br>"]
    E2Pullup,
    #[doc = "VDD1_2: Set input at VDD/2<br>"]
    E3Vdd12,
}
impl From<E32SaadcChnConfig8Resp> for u8 {
    fn from(value: E32SaadcChnConfig8Resp) -> u8 {
        match value {
            E32SaadcChnConfig8Resp::E0Bypass => 0,
            E32SaadcChnConfig8Resp::E1Pulldown => 1,
            E32SaadcChnConfig8Resp::E2Pullup => 2,
            E32SaadcChnConfig8Resp::E3Vdd12 => 3,
        }
    }
}
impl From<u8> for E32SaadcChnConfig8Resp {
    fn from(value: u8) -> E32SaadcChnConfig8Resp {
        match value {
            0 => Self::E0Bypass,
            1 => Self::E1Pulldown,
            2 => Self::E2Pullup,
            3 => Self::E3Vdd12,
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E33SaadcChnConfig8Resn {
    #[doc = "Bypass: Bypass resistor ladder<br>"]
    E0Bypass,
    #[doc = "Pulldown: Pull-down to GND<br>"]
    E1Pulldown,
    #[doc = "Pullup: Pull-up to VDD<br>"]
    E2Pullup,
    #[doc = "VDD1_2: Set input at VDD/2<br>"]
    E3Vdd12,
}
impl From<E33SaadcChnConfig8Resn> for u8 {
    fn from(value: E33SaadcChnConfig8Resn) -> u8 {
        match value {
            E33SaadcChnConfig8Resn::E0Bypass => 0,
            E33SaadcChnConfig8Resn::E1Pulldown => 1,
            E33SaadcChnConfig8Resn::E2Pullup => 2,
            E33SaadcChnConfig8Resn::E3Vdd12 => 3,
        }
    }
}
impl From<u8> for E33SaadcChnConfig8Resn {
    fn from(value: u8) -> E33SaadcChnConfig8Resn {
        match value {
            0 => Self::E0Bypass,
            1 => Self::E1Pulldown,
            2 => Self::E2Pullup,
            3 => Self::E3Vdd12,
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E34SaadcChnConfig8Gain {
    #[doc = "Gain1_6: 1/6<br>"]
    E0Gain16,
    #[doc = "Gain1_5: 1/5<br>"]
    E1Gain15,
    #[doc = "Gain1_4: 1/4<br>"]
    E2Gain14,
    #[doc = "Gain1_3: 1/3<br>"]
    E3Gain13,
    #[doc = "Gain1_2: 1/2<br>"]
    E4Gain12,
    #[doc = "Gain1: 1<br>"]
    E5Gain1,
    #[doc = "Gain2: 2<br>"]
    E6Gain2,
    #[doc = "Gain4: 4<br>"]
    E7Gain4,
}
impl From<E34SaadcChnConfig8Gain> for u8 {
    fn from(value: E34SaadcChnConfig8Gain) -> u8 {
        match value {
            E34SaadcChnConfig8Gain::E0Gain16 => 0,
            E34SaadcChnConfig8Gain::E1Gain15 => 1,
            E34SaadcChnConfig8Gain::E2Gain14 => 2,
            E34SaadcChnConfig8Gain::E3Gain13 => 3,
            E34SaadcChnConfig8Gain::E4Gain12 => 4,
            E34SaadcChnConfig8Gain::E5Gain1 => 5,
            E34SaadcChnConfig8Gain::E6Gain2 => 6,
            E34SaadcChnConfig8Gain::E7Gain4 => 7,
        }
    }
}
impl From<u8> for E34SaadcChnConfig8Gain {
    fn from(value: u8) -> E34SaadcChnConfig8Gain {
        match value {
            0 => Self::E0Gain16,
            1 => Self::E1Gain15,
            2 => Self::E2Gain14,
            3 => Self::E3Gain13,
            4 => Self::E4Gain12,
            5 => Self::E5Gain1,
            6 => Self::E6Gain2,
            7 => Self::E7Gain4,
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E35SaadcChnConfig8Tacq {
    #[doc = "3us: 3 us<br>"]
    E03us,
    #[doc = "5us: 5 us<br>"]
    E15us,
    #[doc = "10us: 10 us<br>"]
    E210us,
    #[doc = "15us: 15 us<br>"]
    E315us,
    #[doc = "20us: 20 us<br>"]
    E420us,
    #[doc = "40us: 40 us<br>"]
    E540us,
}
impl From<E35SaadcChnConfig8Tacq> for u8 {
    fn from(value: E35SaadcChnConfig8Tacq) -> u8 {
        match value {
            E35SaadcChnConfig8Tacq::E03us => 0,
            E35SaadcChnConfig8Tacq::E15us => 1,
            E35SaadcChnConfig8Tacq::E210us => 2,
            E35SaadcChnConfig8Tacq::E315us => 3,
            E35SaadcChnConfig8Tacq::E420us => 4,
            E35SaadcChnConfig8Tacq::E540us => 5,
        }
    }
}
impl TryFrom<u8> for E35SaadcChnConfig8Tacq {
    type Error = ();
    fn try_from(value: u8) -> Result<E35SaadcChnConfig8Tacq, Self::Error> {
        match value {
            0 => Ok(Self::E03us),
            1 => Ok(Self::E15us),
            2 => Ok(Self::E210us),
            3 => Ok(Self::E315us),
            4 => Ok(Self::E420us),
            5 => Ok(Self::E540us),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E36SaadcResolution5f0Val {
    #[doc = "8bit: 8 bit<br>"]
    E08bit,
    #[doc = "10bit: 10 bit<br>"]
    E110bit,
    #[doc = "12bit: 12 bit<br>"]
    E212bit,
    #[doc = "14bit: 14 bit<br>"]
    E314bit,
}
impl From<E36SaadcResolution5f0Val> for u8 {
    fn from(value: E36SaadcResolution5f0Val) -> u8 {
        match value {
            E36SaadcResolution5f0Val::E08bit => 0,
            E36SaadcResolution5f0Val::E110bit => 1,
            E36SaadcResolution5f0Val::E212bit => 2,
            E36SaadcResolution5f0Val::E314bit => 3,
        }
    }
}
impl TryFrom<u8> for E36SaadcResolution5f0Val {
    type Error = ();
    fn try_from(value: u8) -> Result<E36SaadcResolution5f0Val, Self::Error> {
        match value {
            0 => Ok(Self::E08bit),
            1 => Ok(Self::E110bit),
            2 => Ok(Self::E212bit),
            3 => Ok(Self::E314bit),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E37SaadcOversample5f4Oversample {
    #[doc = "Bypass: Bypass oversampling<br>"]
    E0Bypass,
    #[doc = "Over2x: Oversample 2x<br>"]
    E1Over2x,
    #[doc = "Over4x: Oversample 4x<br>"]
    E2Over4x,
    #[doc = "Over8x: Oversample 8x<br>"]
    E3Over8x,
    #[doc = "Over16x: Oversample 16x<br>"]
    E4Over16x,
    #[doc = "Over32x: Oversample 32x<br>"]
    E5Over32x,
    #[doc = "Over64x: Oversample 64x<br>"]
    E6Over64x,
    #[doc = "Over128x: Oversample 128x<br>"]
    E7Over128x,
    #[doc = "Over256x: Oversample 256x<br>"]
    E8Over256x,
}
impl From<E37SaadcOversample5f4Oversample> for u8 {
    fn from(value: E37SaadcOversample5f4Oversample) -> u8 {
        match value {
            E37SaadcOversample5f4Oversample::E0Bypass => 0,
            E37SaadcOversample5f4Oversample::E1Over2x => 1,
            E37SaadcOversample5f4Oversample::E2Over4x => 2,
            E37SaadcOversample5f4Oversample::E3Over8x => 3,
            E37SaadcOversample5f4Oversample::E4Over16x => 4,
            E37SaadcOversample5f4Oversample::E5Over32x => 5,
            E37SaadcOversample5f4Oversample::E6Over64x => 6,
            E37SaadcOversample5f4Oversample::E7Over128x => 7,
            E37SaadcOversample5f4Oversample::E8Over256x => 8,
        }
    }
}
impl TryFrom<u8> for E37SaadcOversample5f4Oversample {
    type Error = ();
    fn try_from(
        value: u8,
    ) -> Result<E37SaadcOversample5f4Oversample, Self::Error> {
        match value {
            0 => Ok(Self::E0Bypass),
            1 => Ok(Self::E1Over2x),
            2 => Ok(Self::E2Over4x),
            3 => Ok(Self::E3Over8x),
            4 => Ok(Self::E4Over16x),
            5 => Ok(Self::E5Over32x),
            6 => Ok(Self::E6Over64x),
            7 => Ok(Self::E7Over128x),
            8 => Ok(Self::E8Over256x),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E38Timer0Mode504Mode {
    #[doc = "Timer: Select Timer mode<br>"]
    E0Timer,
    #[doc = "Counter: Deprecated enumerator -  Select Counter mode<br>"]
    E1Counter,
    #[doc = "LowPowerCounter: Select Low Power Counter mode<br>"]
    E2Lowpowercounter,
}
impl From<E38Timer0Mode504Mode> for u8 {
    fn from(value: E38Timer0Mode504Mode) -> u8 {
        match value {
            E38Timer0Mode504Mode::E0Timer => 0,
            E38Timer0Mode504Mode::E1Counter => 1,
            E38Timer0Mode504Mode::E2Lowpowercounter => 2,
        }
    }
}
impl TryFrom<u8> for E38Timer0Mode504Mode {
    type Error = ();
    fn try_from(value: u8) -> Result<E38Timer0Mode504Mode, Self::Error> {
        match value {
            0 => Ok(Self::E0Timer),
            1 => Ok(Self::E1Counter),
            2 => Ok(Self::E2Lowpowercounter),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E39Timer0Bitmode508Bitmode {
    #[doc = "16Bit: 16 bit timer bit width<br>"]
    E016bit,
    #[doc = "08Bit: 8 bit timer bit width<br>"]
    E108bit,
    #[doc = "24Bit: 24 bit timer bit width<br>"]
    E224bit,
    #[doc = "32Bit: 32 bit timer bit width<br>"]
    E332bit,
}
impl From<E39Timer0Bitmode508Bitmode> for u8 {
    fn from(value: E39Timer0Bitmode508Bitmode) -> u8 {
        match value {
            E39Timer0Bitmode508Bitmode::E016bit => 0,
            E39Timer0Bitmode508Bitmode::E108bit => 1,
            E39Timer0Bitmode508Bitmode::E224bit => 2,
            E39Timer0Bitmode508Bitmode::E332bit => 3,
        }
    }
}
impl From<u8> for E39Timer0Bitmode508Bitmode {
    fn from(value: u8) -> E39Timer0Bitmode508Bitmode {
        match value {
            0 => Self::E016bit,
            1 => Self::E108bit,
            2 => Self::E224bit,
            3 => Self::E332bit,
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E40Apb15Enable500Enable {
    #[doc = "Disabled: Disable<br>Disabled: Disable<br>"]
    E0Disabled,
    #[doc = "Enabled: Enable<br>"]
    E1Enabled,
    #[doc = "Enabled: Enable<br>"]
    E2Enabled,
}
impl From<E40Apb15Enable500Enable> for u8 {
    fn from(value: E40Apb15Enable500Enable) -> u8 {
        match value {
            E40Apb15Enable500Enable::E0Disabled => 0,
            E40Apb15Enable500Enable::E1Enabled => 2,
            E40Apb15Enable500Enable::E2Enabled => 3,
        }
    }
}
impl TryFrom<u8> for E40Apb15Enable500Enable {
    type Error = ();
    fn try_from(value: u8) -> Result<E40Apb15Enable500Enable, Self::Error> {
        match value {
            0 => Ok(Self::E0Disabled),
            2 => Ok(Self::E1Enabled),
            3 => Ok(Self::E2Enabled),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E41WdtRrn600Rr {
    #[doc = "Reload: Value to request a reload of the watchdog timer<br>"]
    E0Reload,
}
impl From<E41WdtRrn600Rr> for u32 {
    fn from(value: E41WdtRrn600Rr) -> u32 {
        match value {
            E41WdtRrn600Rr::E0Reload => 1850885685,
        }
    }
}
impl TryFrom<u32> for E41WdtRrn600Rr {
    type Error = ();
    fn try_from(value: u32) -> Result<E41WdtRrn600Rr, Self::Error> {
        match value {
            1850885685 => Ok(Self::E0Reload),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E42QdecSampleper508Sampleper {
    #[doc = "128us: 128 us<br>"]
    E0128us,
    #[doc = "256us: 256 us<br>"]
    E1256us,
    #[doc = "512us: 512 us<br>"]
    E2512us,
    #[doc = "1024us: 1024 us<br>"]
    E31024us,
    #[doc = "2048us: 2048 us<br>"]
    E42048us,
    #[doc = "4096us: 4096 us<br>"]
    E54096us,
    #[doc = "8192us: 8192 us<br>"]
    E68192us,
    #[doc = "16384us: 16384 us<br>"]
    E716384us,
    #[doc = "32ms: 32768 us<br>"]
    E832ms,
    #[doc = "65ms: 65536 us<br>"]
    E965ms,
    #[doc = "131ms: 131072 us<br>"]
    E10131ms,
}
impl From<E42QdecSampleper508Sampleper> for u8 {
    fn from(value: E42QdecSampleper508Sampleper) -> u8 {
        match value {
            E42QdecSampleper508Sampleper::E0128us => 0,
            E42QdecSampleper508Sampleper::E1256us => 1,
            E42QdecSampleper508Sampleper::E2512us => 2,
            E42QdecSampleper508Sampleper::E31024us => 3,
            E42QdecSampleper508Sampleper::E42048us => 4,
            E42QdecSampleper508Sampleper::E54096us => 5,
            E42QdecSampleper508Sampleper::E68192us => 6,
            E42QdecSampleper508Sampleper::E716384us => 7,
            E42QdecSampleper508Sampleper::E832ms => 8,
            E42QdecSampleper508Sampleper::E965ms => 9,
            E42QdecSampleper508Sampleper::E10131ms => 10,
        }
    }
}
impl TryFrom<u8> for E42QdecSampleper508Sampleper {
    type Error = ();
    fn try_from(
        value: u8,
    ) -> Result<E42QdecSampleper508Sampleper, Self::Error> {
        match value {
            0 => Ok(Self::E0128us),
            1 => Ok(Self::E1256us),
            2 => Ok(Self::E2512us),
            3 => Ok(Self::E31024us),
            4 => Ok(Self::E42048us),
            5 => Ok(Self::E54096us),
            6 => Ok(Self::E68192us),
            7 => Ok(Self::E716384us),
            8 => Ok(Self::E832ms),
            9 => Ok(Self::E965ms),
            10 => Ok(Self::E10131ms),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E43QdecReportper510Reportper {
    #[doc = "10Smpl: 10 samples / report<br>"]
    E010smpl,
    #[doc = "40Smpl: 40 samples / report<br>"]
    E140smpl,
    #[doc = "80Smpl: 80 samples / report<br>"]
    E280smpl,
    #[doc = "120Smpl: 120 samples / report<br>"]
    E3120smpl,
    #[doc = "160Smpl: 160 samples / report<br>"]
    E4160smpl,
    #[doc = "200Smpl: 200 samples / report<br>"]
    E5200smpl,
    #[doc = "240Smpl: 240 samples / report<br>"]
    E6240smpl,
    #[doc = "280Smpl: 280 samples / report<br>"]
    E7280smpl,
    #[doc = "1Smpl: 1 sample / report<br>"]
    E81smpl,
}
impl From<E43QdecReportper510Reportper> for u8 {
    fn from(value: E43QdecReportper510Reportper) -> u8 {
        match value {
            E43QdecReportper510Reportper::E010smpl => 0,
            E43QdecReportper510Reportper::E140smpl => 1,
            E43QdecReportper510Reportper::E280smpl => 2,
            E43QdecReportper510Reportper::E3120smpl => 3,
            E43QdecReportper510Reportper::E4160smpl => 4,
            E43QdecReportper510Reportper::E5200smpl => 5,
            E43QdecReportper510Reportper::E6240smpl => 6,
            E43QdecReportper510Reportper::E7280smpl => 7,
            E43QdecReportper510Reportper::E81smpl => 8,
        }
    }
}
impl TryFrom<u8> for E43QdecReportper510Reportper {
    type Error = ();
    fn try_from(
        value: u8,
    ) -> Result<E43QdecReportper510Reportper, Self::Error> {
        match value {
            0 => Ok(Self::E010smpl),
            1 => Ok(Self::E140smpl),
            2 => Ok(Self::E280smpl),
            3 => Ok(Self::E3120smpl),
            4 => Ok(Self::E4160smpl),
            5 => Ok(Self::E5200smpl),
            6 => Ok(Self::E6240smpl),
            7 => Ok(Self::E7280smpl),
            8 => Ok(Self::E81smpl),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E44Apb19Enable500Enable {
    #[doc = "Disabled: Disable<br>Disabled: Disable<br>"]
    E0Disabled,
    #[doc = "Enabled: Enable<br>"]
    E1Enabled,
    #[doc = "Enabled: Enable<br>"]
    E2Enabled,
}
impl From<E44Apb19Enable500Enable> for u8 {
    fn from(value: E44Apb19Enable500Enable) -> u8 {
        match value {
            E44Apb19Enable500Enable::E0Disabled => 0,
            E44Apb19Enable500Enable::E1Enabled => 1,
            E44Apb19Enable500Enable::E2Enabled => 2,
        }
    }
}
impl TryFrom<u8> for E44Apb19Enable500Enable {
    type Error = ();
    fn try_from(value: u8) -> Result<E44Apb19Enable500Enable, Self::Error> {
        match value {
            0 => Ok(Self::E0Disabled),
            1 => Ok(Self::E1Enabled),
            2 => Ok(Self::E2Enabled),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E45Apb19Psel504Psel {
    #[doc = "AnalogInput0: AIN0 selected as analog input<br>AnalogInput0: AIN0 selected as analog input<br>"]
    E0Analoginput0,
    #[doc = "AnalogInput1: AIN1 selected as analog input<br>AnalogInput1: AIN1 selected as analog input<br>"]
    E1Analoginput1,
    #[doc = "AnalogInput2: AIN2 selected as analog input<br>AnalogInput2: AIN2 selected as analog input<br>"]
    E2Analoginput2,
    #[doc = "AnalogInput3: AIN3 selected as analog input<br>AnalogInput3: AIN3 selected as analog input<br>"]
    E3Analoginput3,
    #[doc = "AnalogInput4: AIN4 selected as analog input<br>AnalogInput4: AIN4 selected as analog input<br>"]
    E4Analoginput4,
    #[doc = "AnalogInput5: AIN5 selected as analog input<br>AnalogInput5: AIN5 selected as analog input<br>"]
    E5Analoginput5,
    #[doc = "AnalogInput6: AIN6 selected as analog input<br>AnalogInput6: AIN6 selected as analog input<br>"]
    E6Analoginput6,
    #[doc = "AnalogInput7: AIN7 selected as analog input<br>AnalogInput7: AIN7 selected as analog input<br>"]
    E7Analoginput7,
}
impl From<E45Apb19Psel504Psel> for u8 {
    fn from(value: E45Apb19Psel504Psel) -> u8 {
        match value {
            E45Apb19Psel504Psel::E0Analoginput0 => 0,
            E45Apb19Psel504Psel::E1Analoginput1 => 1,
            E45Apb19Psel504Psel::E2Analoginput2 => 2,
            E45Apb19Psel504Psel::E3Analoginput3 => 3,
            E45Apb19Psel504Psel::E4Analoginput4 => 4,
            E45Apb19Psel504Psel::E5Analoginput5 => 5,
            E45Apb19Psel504Psel::E6Analoginput6 => 6,
            E45Apb19Psel504Psel::E7Analoginput7 => 7,
        }
    }
}
impl From<u8> for E45Apb19Psel504Psel {
    fn from(value: u8) -> E45Apb19Psel504Psel {
        match value {
            0 => Self::E0Analoginput0,
            1 => Self::E1Analoginput1,
            2 => Self::E2Analoginput2,
            3 => Self::E3Analoginput3,
            4 => Self::E4Analoginput4,
            5 => Self::E5Analoginput5,
            6 => Self::E6Analoginput6,
            7 => Self::E7Analoginput7,
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E46Apb19Refsel508Refsel {
    #[doc = "Int1V2: VREF = internal 1.2 V reference (VDD &gt;= 1.7 V)<br>"]
    E0Int1v2,
    #[doc = "Int1V8: VREF = internal 1.8 V reference (VDD &gt;= VREF + 0.2 V)<br>"]
    E1Int1v8,
    #[doc = "Int2V4: VREF = internal 2.4 V reference (VDD &gt;= VREF + 0.2 V)<br>"]
    E2Int2v4,
    #[doc = "VDD: VREF = VDD<br>"]
    E3Vdd,
    #[doc = "ARef: VREF = AREF (VDD &gt;= VREF &gt;= AREFMIN)<br>"]
    E4Aref,
}
impl From<E46Apb19Refsel508Refsel> for u8 {
    fn from(value: E46Apb19Refsel508Refsel) -> u8 {
        match value {
            E46Apb19Refsel508Refsel::E0Int1v2 => 0,
            E46Apb19Refsel508Refsel::E1Int1v8 => 1,
            E46Apb19Refsel508Refsel::E2Int2v4 => 2,
            E46Apb19Refsel508Refsel::E3Vdd => 4,
            E46Apb19Refsel508Refsel::E4Aref => 7,
        }
    }
}
impl TryFrom<u8> for E46Apb19Refsel508Refsel {
    type Error = ();
    fn try_from(value: u8) -> Result<E46Apb19Refsel508Refsel, Self::Error> {
        match value {
            0 => Ok(Self::E0Int1v2),
            1 => Ok(Self::E1Int1v8),
            2 => Ok(Self::E2Int2v4),
            4 => Ok(Self::E3Vdd),
            7 => Ok(Self::E4Aref),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E47Apb19Anadetect520Anadetect {
    #[doc = "Cross: Generate ANADETECT on crossing, both upward crossing and downward crossing<br>"]
    E0Cross,
    #[doc = "Up: Generate ANADETECT on upward crossing only<br>"]
    E1Up,
    #[doc = "Down: Generate ANADETECT on downward crossing only<br>"]
    E2Down,
}
impl From<E47Apb19Anadetect520Anadetect> for u8 {
    fn from(value: E47Apb19Anadetect520Anadetect) -> u8 {
        match value {
            E47Apb19Anadetect520Anadetect::E0Cross => 0,
            E47Apb19Anadetect520Anadetect::E1Up => 1,
            E47Apb19Anadetect520Anadetect::E2Down => 2,
        }
    }
}
impl TryFrom<u8> for E47Apb19Anadetect520Anadetect {
    type Error = ();
    fn try_from(
        value: u8,
    ) -> Result<E47Apb19Anadetect520Anadetect, Self::Error> {
        match value {
            0 => Ok(Self::E0Cross),
            1 => Ok(Self::E1Up),
            2 => Ok(Self::E2Down),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E48Apb19Mode534Sp {
    #[doc = "Low: Low-power mode<br>"]
    E0Low,
    #[doc = "Normal: Normal mode<br>"]
    E1Normal,
    #[doc = "High: High-speed mode<br>"]
    E2High,
}
impl From<E48Apb19Mode534Sp> for u8 {
    fn from(value: E48Apb19Mode534Sp) -> u8 {
        match value {
            E48Apb19Mode534Sp::E0Low => 0,
            E48Apb19Mode534Sp::E1Normal => 1,
            E48Apb19Mode534Sp::E2High => 2,
        }
    }
}
impl TryFrom<u8> for E48Apb19Mode534Sp {
    type Error = ();
    fn try_from(value: u8) -> Result<E48Apb19Mode534Sp, Self::Error> {
        match value {
            0 => Ok(Self::E0Low),
            1 => Ok(Self::E1Normal),
            2 => Ok(Self::E2High),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E49Apb19Isource53cIsource {
    #[doc = "Off: Current source disabled<br>"]
    E0Off,
    #[doc = "Ien2mA5: Current source enabled (+/- 2.5 uA)<br>"]
    E1Ien2ma5,
    #[doc = "Ien5mA: Current source enabled (+/- 5 uA)<br>"]
    E2Ien5ma,
    #[doc = "Ien10mA: Current source enabled (+/- 10 uA)<br>"]
    E3Ien10ma,
}
impl From<E49Apb19Isource53cIsource> for u8 {
    fn from(value: E49Apb19Isource53cIsource) -> u8 {
        match value {
            E49Apb19Isource53cIsource::E0Off => 0,
            E49Apb19Isource53cIsource::E1Ien2ma5 => 1,
            E49Apb19Isource53cIsource::E2Ien5ma => 2,
            E49Apb19Isource53cIsource::E3Ien10ma => 3,
        }
    }
}
impl From<u8> for E49Apb19Isource53cIsource {
    fn from(value: u8) -> E49Apb19Isource53cIsource {
        match value {
            0 => Self::E0Off,
            1 => Self::E1Ien2ma5,
            2 => Self::E2Ien5ma,
            3 => Self::E3Ien10ma,
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E50Timer3Mode504Mode {
    #[doc = "Timer: Select Timer mode<br>"]
    E0Timer,
    #[doc = "Counter: Deprecated enumerator -  Select Counter mode<br>"]
    E1Counter,
    #[doc = "LowPowerCounter: Select Low Power Counter mode<br>"]
    E2Lowpowercounter,
}
impl From<E50Timer3Mode504Mode> for u8 {
    fn from(value: E50Timer3Mode504Mode) -> u8 {
        match value {
            E50Timer3Mode504Mode::E0Timer => 0,
            E50Timer3Mode504Mode::E1Counter => 1,
            E50Timer3Mode504Mode::E2Lowpowercounter => 2,
        }
    }
}
impl TryFrom<u8> for E50Timer3Mode504Mode {
    type Error = ();
    fn try_from(value: u8) -> Result<E50Timer3Mode504Mode, Self::Error> {
        match value {
            0 => Ok(Self::E0Timer),
            1 => Ok(Self::E1Counter),
            2 => Ok(Self::E2Lowpowercounter),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E51Timer3Bitmode508Bitmode {
    #[doc = "16Bit: 16 bit timer bit width<br>"]
    E016bit,
    #[doc = "08Bit: 8 bit timer bit width<br>"]
    E108bit,
    #[doc = "24Bit: 24 bit timer bit width<br>"]
    E224bit,
    #[doc = "32Bit: 32 bit timer bit width<br>"]
    E332bit,
}
impl From<E51Timer3Bitmode508Bitmode> for u8 {
    fn from(value: E51Timer3Bitmode508Bitmode) -> u8 {
        match value {
            E51Timer3Bitmode508Bitmode::E016bit => 0,
            E51Timer3Bitmode508Bitmode::E108bit => 1,
            E51Timer3Bitmode508Bitmode::E224bit => 2,
            E51Timer3Bitmode508Bitmode::E332bit => 3,
        }
    }
}
impl From<u8> for E51Timer3Bitmode508Bitmode {
    fn from(value: u8) -> E51Timer3Bitmode508Bitmode {
        match value {
            0 => Self::E016bit,
            1 => Self::E108bit,
            2 => Self::E224bit,
            3 => Self::E332bit,
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E52Pwm0Prescaler50cPrescaler {
    #[doc = "DIV_1: Divide by   1 (16MHz)<br>"]
    E0Div1,
    #[doc = "DIV_2: Divide by   2 ( 8MHz)<br>"]
    E1Div2,
    #[doc = "DIV_4: Divide by   4 ( 4MHz)<br>"]
    E2Div4,
    #[doc = "DIV_8: Divide by   8 ( 2MHz)<br>"]
    E3Div8,
    #[doc = "DIV_16: Divide by  16 ( 1MHz)<br>"]
    E4Div16,
    #[doc = "DIV_32: Divide by  32 ( 500kHz)<br>"]
    E5Div32,
    #[doc = "DIV_64: Divide by  64 ( 250kHz)<br>"]
    E6Div64,
    #[doc = "DIV_128: Divide by 128 ( 125kHz)<br>"]
    E7Div128,
}
impl From<E52Pwm0Prescaler50cPrescaler> for u8 {
    fn from(value: E52Pwm0Prescaler50cPrescaler) -> u8 {
        match value {
            E52Pwm0Prescaler50cPrescaler::E0Div1 => 0,
            E52Pwm0Prescaler50cPrescaler::E1Div2 => 1,
            E52Pwm0Prescaler50cPrescaler::E2Div4 => 2,
            E52Pwm0Prescaler50cPrescaler::E3Div8 => 3,
            E52Pwm0Prescaler50cPrescaler::E4Div16 => 4,
            E52Pwm0Prescaler50cPrescaler::E5Div32 => 5,
            E52Pwm0Prescaler50cPrescaler::E6Div64 => 6,
            E52Pwm0Prescaler50cPrescaler::E7Div128 => 7,
        }
    }
}
impl From<u8> for E52Pwm0Prescaler50cPrescaler {
    fn from(value: u8) -> E52Pwm0Prescaler50cPrescaler {
        match value {
            0 => Self::E0Div1,
            1 => Self::E1Div2,
            2 => Self::E2Div4,
            3 => Self::E3Div8,
            4 => Self::E4Div16,
            5 => Self::E5Div32,
            6 => Self::E6Div64,
            7 => Self::E7Div128,
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E53Pwm0Decoder510Load {
    #[doc = "Common: 1st half word (16-bit) used in all PWM channels 0..3<br>"]
    E0Common,
    #[doc = "Grouped: 1st half word (16-bit) used in channel 0..1; 2nd word in channel 2..3<br>"]
    E1Grouped,
    #[doc = "Individual: 1st half word (16-bit) in ch.0; 2nd in ch.1; ...; 4th in ch.3<br>"]
    E2Individual,
    #[doc = "WaveForm: 1st half word (16-bit) in ch.0; 2nd in ch.1; ...; 4th in COUNTERTOP<br>"]
    E3Waveform,
}
impl From<E53Pwm0Decoder510Load> for u8 {
    fn from(value: E53Pwm0Decoder510Load) -> u8 {
        match value {
            E53Pwm0Decoder510Load::E0Common => 0,
            E53Pwm0Decoder510Load::E1Grouped => 1,
            E53Pwm0Decoder510Load::E2Individual => 2,
            E53Pwm0Decoder510Load::E3Waveform => 3,
        }
    }
}
impl From<u8> for E53Pwm0Decoder510Load {
    fn from(value: u8) -> E53Pwm0Decoder510Load {
        match value {
            0 => Self::E0Common,
            1 => Self::E1Grouped,
            2 => Self::E2Individual,
            3 => Self::E3Waveform,
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E54Pwm0Loop514Cnt {
    #[doc = "Disabled: Looping disabled (stop at the end of the sequence)<br>"]
    E0Disabled,
}
impl From<E54Pwm0Loop514Cnt> for u16 {
    fn from(value: E54Pwm0Loop514Cnt) -> u16 {
        match value {
            E54Pwm0Loop514Cnt::E0Disabled => 0,
        }
    }
}
impl TryFrom<u16> for E54Pwm0Loop514Cnt {
    type Error = ();
    fn try_from(value: u16) -> Result<E54Pwm0Loop514Cnt, Self::Error> {
        match value {
            0 => Ok(Self::E0Disabled),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E55Pwm0SeqnCnt4Cnt {
    #[doc = "Disabled: Sequence is disabled, and shall not be started as it is empty<br>"]
    E0Disabled,
}
impl From<E55Pwm0SeqnCnt4Cnt> for u16 {
    fn from(value: E55Pwm0SeqnCnt4Cnt) -> u16 {
        match value {
            E55Pwm0SeqnCnt4Cnt::E0Disabled => 0,
        }
    }
}
impl TryFrom<u16> for E55Pwm0SeqnCnt4Cnt {
    type Error = ();
    fn try_from(value: u16) -> Result<E55Pwm0SeqnCnt4Cnt, Self::Error> {
        match value {
            0 => Ok(Self::E0Disabled),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E56Pwm0SeqnRefresh8Cnt {
    #[doc = "Continuous: Update every PWM period<br>"]
    E0Continuous,
}
impl From<E56Pwm0SeqnRefresh8Cnt> for u32 {
    fn from(value: E56Pwm0SeqnRefresh8Cnt) -> u32 {
        match value {
            E56Pwm0SeqnRefresh8Cnt::E0Continuous => 0,
        }
    }
}
impl TryFrom<u32> for E56Pwm0SeqnRefresh8Cnt {
    type Error = ();
    fn try_from(value: u32) -> Result<E56Pwm0SeqnRefresh8Cnt, Self::Error> {
        match value {
            0 => Ok(Self::E0Continuous),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E57PdmPdmclkctrl504Freq {
    #[doc = "1000K: PDM_CLK = 32 MHz / 32 = 1.000 MHz<br>"]
    E01000k,
    #[doc = "Default: PDM_CLK = 32 MHz / 31 = 1.032 MHz<br>"]
    E1Default,
    #[doc = "1067K: PDM_CLK = 32 MHz / 30 = 1.067 MHz<br>"]
    E21067k,
}
impl From<E57PdmPdmclkctrl504Freq> for u32 {
    fn from(value: E57PdmPdmclkctrl504Freq) -> u32 {
        match value {
            E57PdmPdmclkctrl504Freq::E01000k => 134217728,
            E57PdmPdmclkctrl504Freq::E1Default => 138412032,
            E57PdmPdmclkctrl504Freq::E21067k => 142606336,
        }
    }
}
impl TryFrom<u32> for E57PdmPdmclkctrl504Freq {
    type Error = ();
    fn try_from(value: u32) -> Result<E57PdmPdmclkctrl504Freq, Self::Error> {
        match value {
            134217728 => Ok(Self::E01000k),
            138412032 => Ok(Self::E1Default),
            142606336 => Ok(Self::E21067k),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E58PdmGainl518Gainl {
    #[doc = "MinGain: -20dB gain adjustment (minimum)<br>"]
    E0Mingain,
    #[doc = "DefaultGain: 0dB gain adjustment ('2500 RMS' requirement)<br>"]
    E1Defaultgain,
    #[doc = "MaxGain: +20dB gain adjustment (maximum)<br>"]
    E2Maxgain,
}
impl From<E58PdmGainl518Gainl> for u8 {
    fn from(value: E58PdmGainl518Gainl) -> u8 {
        match value {
            E58PdmGainl518Gainl::E0Mingain => 0,
            E58PdmGainl518Gainl::E1Defaultgain => 40,
            E58PdmGainl518Gainl::E2Maxgain => 80,
        }
    }
}
impl TryFrom<u8> for E58PdmGainl518Gainl {
    type Error = ();
    fn try_from(value: u8) -> Result<E58PdmGainl518Gainl, Self::Error> {
        match value {
            0 => Ok(Self::E0Mingain),
            40 => Ok(Self::E1Defaultgain),
            80 => Ok(Self::E2Maxgain),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E59PdmGainr51cGainr {
    #[doc = "MinGain: -20dB gain adjustment (minimum)<br>"]
    E0Mingain,
    #[doc = "DefaultGain: 0dB gain adjustment ('2500 RMS' requirement)<br>"]
    E1Defaultgain,
    #[doc = "MaxGain: +20dB gain adjustment (maximum)<br>"]
    E2Maxgain,
}
impl From<E59PdmGainr51cGainr> for u8 {
    fn from(value: E59PdmGainr51cGainr) -> u8 {
        match value {
            E59PdmGainr51cGainr::E0Mingain => 0,
            E59PdmGainr51cGainr::E1Defaultgain => 40,
            E59PdmGainr51cGainr::E2Maxgain => 80,
        }
    }
}
impl TryFrom<u8> for E59PdmGainr51cGainr {
    type Error = ();
    fn try_from(value: u8) -> Result<E59PdmGainr51cGainr, Self::Error> {
        match value {
            0 => Ok(Self::E0Mingain),
            40 => Ok(Self::E1Defaultgain),
            80 => Ok(Self::E2Maxgain),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E60NvmcConfig504Wen {
    #[doc = "Ren: Read only access<br>"]
    E0Ren,
    #[doc = "Wen: Write Enabled<br>"]
    E1Wen,
    #[doc = "Een: Erase enabled<br>"]
    E2Een,
}
impl From<E60NvmcConfig504Wen> for u8 {
    fn from(value: E60NvmcConfig504Wen) -> u8 {
        match value {
            E60NvmcConfig504Wen::E0Ren => 0,
            E60NvmcConfig504Wen::E1Wen => 1,
            E60NvmcConfig504Wen::E2Een => 2,
        }
    }
}
impl TryFrom<u8> for E60NvmcConfig504Wen {
    type Error = ();
    fn try_from(value: u8) -> Result<E60NvmcConfig504Wen, Self::Error> {
        match value {
            0 => Ok(Self::E0Ren),
            1 => Ok(Self::E1Wen),
            2 => Ok(Self::E2Een),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E61I2sConfigMckfreq10Mckfreq {
    #[doc = "32MDIV125: 32 MHz / 125 = 0.256 MHz<br>"]
    E032mdiv125,
    #[doc = "32MDIV63: 32 MHz / 63 = 0.5079365 MHz<br>"]
    E132mdiv63,
    #[doc = "32MDIV42: 32 MHz / 42 = 0.7619048 MHz<br>"]
    E232mdiv42,
    #[doc = "32MDIV32: 32 MHz / 32 = 1.0 MHz<br>"]
    E332mdiv32,
    #[doc = "32MDIV31: 32 MHz / 31 = 1.0322581 MHz<br>"]
    E432mdiv31,
    #[doc = "32MDIV30: 32 MHz / 30 = 1.0666667 MHz<br>"]
    E532mdiv30,
    #[doc = "32MDIV23: 32 MHz / 23 = 1.3913043 MHz<br>"]
    E632mdiv23,
    #[doc = "32MDIV21: 32 MHz / 21 = 1.5238095<br>"]
    E732mdiv21,
    #[doc = "32MDIV16: 32 MHz / 16 = 2.0 MHz<br>"]
    E832mdiv16,
    #[doc = "32MDIV15: 32 MHz / 15 = 2.1333333 MHz<br>"]
    E932mdiv15,
    #[doc = "32MDIV11: 32 MHz / 11 = 2.9090909 MHz<br>"]
    E1032mdiv11,
    #[doc = "32MDIV10: 32 MHz / 10 = 3.2 MHz<br>"]
    E1132mdiv10,
    #[doc = "32MDIV8: 32 MHz / 8 = 4.0 MHz<br>"]
    E1232mdiv8,
    #[doc = "32MDIV6: 32 MHz / 6 = 5.3333333 MHz<br>"]
    E1332mdiv6,
    #[doc = "32MDIV5: 32 MHz / 5 = 6.4 MHz<br>"]
    E1432mdiv5,
    #[doc = "32MDIV4: 32 MHz / 4 = 8.0 MHz<br>"]
    E1532mdiv4,
    #[doc = "32MDIV3: 32 MHz / 3 = 10.6666667 MHz<br>"]
    E1632mdiv3,
    #[doc = "32MDIV2: 32 MHz / 2 = 16.0 MHz<br>"]
    E1732mdiv2,
}
impl From<E61I2sConfigMckfreq10Mckfreq> for u32 {
    fn from(value: E61I2sConfigMckfreq10Mckfreq) -> u32 {
        match value {
            E61I2sConfigMckfreq10Mckfreq::E032mdiv125 => 34340864,
            E61I2sConfigMckfreq10Mckfreq::E132mdiv63 => 68157440,
            E61I2sConfigMckfreq10Mckfreq::E232mdiv42 => 100663296,
            E61I2sConfigMckfreq10Mckfreq::E332mdiv32 => 134217728,
            E61I2sConfigMckfreq10Mckfreq::E432mdiv31 => 138412032,
            E61I2sConfigMckfreq10Mckfreq::E532mdiv30 => 142606336,
            E61I2sConfigMckfreq10Mckfreq::E632mdiv23 => 184549376,
            E61I2sConfigMckfreq10Mckfreq::E732mdiv21 => 201326592,
            E61I2sConfigMckfreq10Mckfreq::E832mdiv16 => 268435456,
            E61I2sConfigMckfreq10Mckfreq::E932mdiv15 => 285212672,
            E61I2sConfigMckfreq10Mckfreq::E1032mdiv11 => 369098752,
            E61I2sConfigMckfreq10Mckfreq::E1132mdiv10 => 402653184,
            E61I2sConfigMckfreq10Mckfreq::E1232mdiv8 => 536870912,
            E61I2sConfigMckfreq10Mckfreq::E1332mdiv6 => 671088640,
            E61I2sConfigMckfreq10Mckfreq::E1432mdiv5 => 805306368,
            E61I2sConfigMckfreq10Mckfreq::E1532mdiv4 => 1073741824,
            E61I2sConfigMckfreq10Mckfreq::E1632mdiv3 => 1342177280,
            E61I2sConfigMckfreq10Mckfreq::E1732mdiv2 => 2147483648,
        }
    }
}
impl TryFrom<u32> for E61I2sConfigMckfreq10Mckfreq {
    type Error = ();
    fn try_from(
        value: u32,
    ) -> Result<E61I2sConfigMckfreq10Mckfreq, Self::Error> {
        match value {
            34340864 => Ok(Self::E032mdiv125),
            68157440 => Ok(Self::E132mdiv63),
            100663296 => Ok(Self::E232mdiv42),
            134217728 => Ok(Self::E332mdiv32),
            138412032 => Ok(Self::E432mdiv31),
            142606336 => Ok(Self::E532mdiv30),
            184549376 => Ok(Self::E632mdiv23),
            201326592 => Ok(Self::E732mdiv21),
            268435456 => Ok(Self::E832mdiv16),
            285212672 => Ok(Self::E932mdiv15),
            369098752 => Ok(Self::E1032mdiv11),
            402653184 => Ok(Self::E1132mdiv10),
            536870912 => Ok(Self::E1232mdiv8),
            671088640 => Ok(Self::E1332mdiv6),
            805306368 => Ok(Self::E1432mdiv5),
            1073741824 => Ok(Self::E1532mdiv4),
            1342177280 => Ok(Self::E1632mdiv3),
            2147483648 => Ok(Self::E1732mdiv2),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E62I2sConfigRatio14Ratio {
    #[doc = "32X: LRCK = MCK / 32<br>"]
    E032x,
    #[doc = "48X: LRCK = MCK / 48<br>"]
    E148x,
    #[doc = "64X: LRCK = MCK / 64<br>"]
    E264x,
    #[doc = "96X: LRCK = MCK / 96<br>"]
    E396x,
    #[doc = "128X: LRCK = MCK / 128<br>"]
    E4128x,
    #[doc = "192X: LRCK = MCK / 192<br>"]
    E5192x,
    #[doc = "256X: LRCK = MCK / 256<br>"]
    E6256x,
    #[doc = "384X: LRCK = MCK / 384<br>"]
    E7384x,
    #[doc = "512X: LRCK = MCK / 512<br>"]
    E8512x,
}
impl From<E62I2sConfigRatio14Ratio> for u8 {
    fn from(value: E62I2sConfigRatio14Ratio) -> u8 {
        match value {
            E62I2sConfigRatio14Ratio::E032x => 0,
            E62I2sConfigRatio14Ratio::E148x => 1,
            E62I2sConfigRatio14Ratio::E264x => 2,
            E62I2sConfigRatio14Ratio::E396x => 3,
            E62I2sConfigRatio14Ratio::E4128x => 4,
            E62I2sConfigRatio14Ratio::E5192x => 5,
            E62I2sConfigRatio14Ratio::E6256x => 6,
            E62I2sConfigRatio14Ratio::E7384x => 7,
            E62I2sConfigRatio14Ratio::E8512x => 8,
        }
    }
}
impl TryFrom<u8> for E62I2sConfigRatio14Ratio {
    type Error = ();
    fn try_from(value: u8) -> Result<E62I2sConfigRatio14Ratio, Self::Error> {
        match value {
            0 => Ok(Self::E032x),
            1 => Ok(Self::E148x),
            2 => Ok(Self::E264x),
            3 => Ok(Self::E396x),
            4 => Ok(Self::E4128x),
            5 => Ok(Self::E5192x),
            6 => Ok(Self::E6256x),
            7 => Ok(Self::E7384x),
            8 => Ok(Self::E8512x),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E63I2sConfigSwidth18Swidth {
    #[doc = "8Bit: 8 bit.<br>"]
    E08bit,
    #[doc = "16Bit: 16 bit.<br>"]
    E116bit,
    #[doc = "24Bit: 24 bit.<br>"]
    E224bit,
}
impl From<E63I2sConfigSwidth18Swidth> for u8 {
    fn from(value: E63I2sConfigSwidth18Swidth) -> u8 {
        match value {
            E63I2sConfigSwidth18Swidth::E08bit => 0,
            E63I2sConfigSwidth18Swidth::E116bit => 1,
            E63I2sConfigSwidth18Swidth::E224bit => 2,
        }
    }
}
impl TryFrom<u8> for E63I2sConfigSwidth18Swidth {
    type Error = ();
    fn try_from(value: u8) -> Result<E63I2sConfigSwidth18Swidth, Self::Error> {
        match value {
            0 => Ok(Self::E08bit),
            1 => Ok(Self::E116bit),
            2 => Ok(Self::E224bit),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E64I2sConfigChannels24Channels {
    #[doc = "Stereo: Stereo.<br>"]
    E0Stereo,
    #[doc = "Left: Left only.<br>"]
    E1Left,
    #[doc = "Right: Right only.<br>"]
    E2Right,
}
impl From<E64I2sConfigChannels24Channels> for u8 {
    fn from(value: E64I2sConfigChannels24Channels) -> u8 {
        match value {
            E64I2sConfigChannels24Channels::E0Stereo => 0,
            E64I2sConfigChannels24Channels::E1Left => 1,
            E64I2sConfigChannels24Channels::E2Right => 2,
        }
    }
}
impl TryFrom<u8> for E64I2sConfigChannels24Channels {
    type Error = ();
    fn try_from(
        value: u8,
    ) -> Result<E64I2sConfigChannels24Channels, Self::Error> {
        match value {
            0 => Ok(Self::E0Stereo),
            1 => Ok(Self::E1Left),
            2 => Ok(Self::E2Right),
            _ => Err(()),
        }
    }
}
#[derive(Default, Debug, Clone, Copy)]
pub enum PinPull {
    #[default]
    #[doc = "Disabled: No pull<br>"]
    E0Disabled,
    #[doc = "Pulldown: Pull down on pin<br>"]
    E1Pulldown,
    #[doc = "Pullup: Pull up on pin<br>"]
    E2Pullup,
}
impl From<PinPull> for u8 {
    fn from(value: PinPull) -> u8 {
        match value {
            PinPull::E0Disabled => 0,
            PinPull::E1Pulldown => 1,
            PinPull::E2Pullup => 3,
        }
    }
}
impl TryFrom<u8> for PinPull {
    type Error = ();
    fn try_from(value: u8) -> Result<PinPull, Self::Error> {
        match value {
            0 => Ok(Self::E0Disabled),
            1 => Ok(Self::E1Pulldown),
            3 => Ok(Self::E2Pullup),
            _ => Err(()),
        }
    }
}
#[derive(Default, Debug, Clone, Copy)]
pub enum PinDrive {
    #[default]
    #[doc = "S0S1: Standard '0', standard '1'<br>"]
    E0S0s1,
    #[doc = "H0S1: High drive '0', standard '1'<br>"]
    E1H0s1,
    #[doc = "S0H1: Standard '0', high drive '1'<br>"]
    E2S0h1,
    #[doc = "H0H1: High drive '0', high 'drive '1''<br>"]
    E3H0h1,
    #[doc = "D0S1: Disconnect '0' standard '1' (normally used for wired-or connections)<br>"]
    E4D0s1,
    #[doc = "D0H1: Disconnect '0', high drive '1' (normally used for wired-or connections)<br>"]
    E5D0h1,
    #[doc = "S0D1: Standard '0'. disconnect '1' (normally used for wired-and connections)<br>"]
    E6S0d1,
    #[doc = "H0D1: High drive '0', disconnect '1' (normally used for wired-and connections)<br>"]
    E7H0d1,
}
impl From<PinDrive> for u8 {
    fn from(value: PinDrive) -> u8 {
        match value {
            PinDrive::E0S0s1 => 0,
            PinDrive::E1H0s1 => 1,
            PinDrive::E2S0h1 => 2,
            PinDrive::E3H0h1 => 3,
            PinDrive::E4D0s1 => 4,
            PinDrive::E5D0h1 => 5,
            PinDrive::E6S0d1 => 6,
            PinDrive::E7H0d1 => 7,
        }
    }
}
impl From<u8> for PinDrive {
    fn from(value: u8) -> PinDrive {
        match value {
            0 => Self::E0S0s1,
            1 => Self::E1H0s1,
            2 => Self::E2S0h1,
            3 => Self::E3H0h1,
            4 => Self::E4D0s1,
            5 => Self::E5D0h1,
            6 => Self::E6S0d1,
            7 => Self::E7H0d1,
            _ => unreachable!(),
        }
    }
}
#[derive(Default, Debug, Clone, Copy)]
pub enum PinSense {
    #[default]
    #[doc = "Disabled: Disabled<br>"]
    E0Disabled,
    #[doc = "High: Sense for high level<br>"]
    E1High,
    #[doc = "Low: Sense for low level<br>"]
    E2Low,
}
impl From<PinSense> for u8 {
    fn from(value: PinSense) -> u8 {
        match value {
            PinSense::E0Disabled => 0,
            PinSense::E1High => 2,
            PinSense::E2Low => 3,
        }
    }
}
impl TryFrom<u8> for PinSense {
    type Error = ();
    fn try_from(value: u8) -> Result<PinSense, Self::Error> {
        match value {
            0 => Ok(Self::E0Disabled),
            2 => Ok(Self::E1High),
            3 => Ok(Self::E2Low),
            _ => Err(()),
        }
    }
}
#[doc = "ID_PFR0_STATE1"]
#[derive(Debug, Clone, Copy)]
pub enum E68ScsIdPfr0d40State1 {
    #[doc = "Thumb_Thumb2: Thumb Thumb2<br>"]
    E0ThumbThumb2,
}
impl From<E68ScsIdPfr0d40State1> for u8 {
    fn from(value: E68ScsIdPfr0d40State1) -> u8 {
        match value {
            E68ScsIdPfr0d40State1::E0ThumbThumb2 => 3,
        }
    }
}
impl TryFrom<u8> for E68ScsIdPfr0d40State1 {
    type Error = ();
    fn try_from(value: u8) -> Result<E68ScsIdPfr0d40State1, Self::Error> {
        match value {
            3 => Ok(Self::E0ThumbThumb2),
            _ => Err(()),
        }
    }
}
#[doc = "ID_PFR1_M_PROFILE"]
#[derive(Debug, Clone, Copy)]
pub enum E69ScsIdPfr1d44MProfile {
    #[doc = "Two_stack: Two stack<br>"]
    E0TwoStack,
}
impl From<E69ScsIdPfr1d44MProfile> for u8 {
    fn from(value: E69ScsIdPfr1d44MProfile) -> u8 {
        match value {
            E69ScsIdPfr1d44MProfile::E0TwoStack => 2,
        }
    }
}
impl TryFrom<u8> for E69ScsIdPfr1d44MProfile {
    type Error = ();
    fn try_from(value: u8) -> Result<E69ScsIdPfr1d44MProfile, Self::Error> {
        match value {
            2 => Ok(Self::E0TwoStack),
            _ => Err(()),
        }
    }
}
#[doc = "E_FEATURE_SUPPORT"]
#[derive(Debug, Clone, Copy)]
pub enum E70ScsIdDfr0d48MProfile {
    #[doc = "Not_supported: Not supported<br>"]
    E0NotSupported,
    #[doc = "Supported: Supported<br>"]
    E1Supported,
}
impl From<E70ScsIdDfr0d48MProfile> for u8 {
    fn from(value: E70ScsIdDfr0d48MProfile) -> u8 {
        match value {
            E70ScsIdDfr0d48MProfile::E0NotSupported => 0,
            E70ScsIdDfr0d48MProfile::E1Supported => 1,
        }
    }
}
impl TryFrom<u8> for E70ScsIdDfr0d48MProfile {
    type Error = ();
    fn try_from(value: u8) -> Result<E70ScsIdDfr0d48MProfile, Self::Error> {
        match value {
            0 => Ok(Self::E0NotSupported),
            1 => Ok(Self::E1Supported),
            _ => Err(()),
        }
    }
}
#[doc = "ID_MMFR0_PMSA"]
#[derive(Debug, Clone, Copy)]
pub enum E71ScsIdMmfr0d50Pmsa {
    #[doc = "Not_supported: Not supported<br>"]
    E0NotSupported,
    #[doc = "PMSAv7: PMSAv7<br>"]
    E1Pmsav7,
}
impl From<E71ScsIdMmfr0d50Pmsa> for u8 {
    fn from(value: E71ScsIdMmfr0d50Pmsa) -> u8 {
        match value {
            E71ScsIdMmfr0d50Pmsa::E0NotSupported => 0,
            E71ScsIdMmfr0d50Pmsa::E1Pmsav7 => 3,
        }
    }
}
impl TryFrom<u8> for E71ScsIdMmfr0d50Pmsa {
    type Error = ();
    fn try_from(value: u8) -> Result<E71ScsIdMmfr0d50Pmsa, Self::Error> {
        match value {
            0 => Ok(Self::E0NotSupported),
            3 => Ok(Self::E1Pmsav7),
            _ => Err(()),
        }
    }
}
#[doc = "ID_MMFR0_OUTER_SHARABILITY"]
#[derive(Debug, Clone, Copy)]
pub enum E72ScsIdMmfr0d50OutermostShareability {
    #[doc = "Non_cacheable: Non cacheable<br>"]
    E0NonCacheable,
    #[doc = "Ignored: Ignored<br>"]
    E1Ignored,
}
impl From<E72ScsIdMmfr0d50OutermostShareability> for u8 {
    fn from(value: E72ScsIdMmfr0d50OutermostShareability) -> u8 {
        match value {
            E72ScsIdMmfr0d50OutermostShareability::E0NonCacheable => 0,
            E72ScsIdMmfr0d50OutermostShareability::E1Ignored => 15,
        }
    }
}
impl TryFrom<u8> for E72ScsIdMmfr0d50OutermostShareability {
    type Error = ();
    fn try_from(
        value: u8,
    ) -> Result<E72ScsIdMmfr0d50OutermostShareability, Self::Error> {
        match value {
            0 => Ok(Self::E0NonCacheable),
            15 => Ok(Self::E1Ignored),
            _ => Err(()),
        }
    }
}
#[doc = "ID_MMFR0_SHARABILITY"]
#[derive(Debug, Clone, Copy)]
pub enum E73ScsIdMmfr0d50ShareabilityLevels {
    #[doc = "One_level: One level<br>"]
    E0OneLevel,
}
impl From<E73ScsIdMmfr0d50ShareabilityLevels> for u8 {
    fn from(value: E73ScsIdMmfr0d50ShareabilityLevels) -> u8 {
        match value {
            E73ScsIdMmfr0d50ShareabilityLevels::E0OneLevel => 0,
        }
    }
}
impl TryFrom<u8> for E73ScsIdMmfr0d50ShareabilityLevels {
    type Error = ();
    fn try_from(
        value: u8,
    ) -> Result<E73ScsIdMmfr0d50ShareabilityLevels, Self::Error> {
        match value {
            0 => Ok(Self::E0OneLevel),
            _ => Err(()),
        }
    }
}
#[doc = "ID_ISAR0_COPROC"]
#[derive(Debug, Clone, Copy)]
pub enum E74ScsIdIsar0d60CoprocInstrs {
    #[doc = "None: None<br>"]
    E0None,
    #[doc = "Generic: Generic<br>"]
    E1Generic,
    #[doc = "Generic2: Generic2<br>"]
    E2Generic2,
    #[doc = "MCRR_MRRC: MCRR MRRC<br>"]
    E3McrrMrrc,
    #[doc = "MCRR2_MRRC2: MCRR2 MRRC2<br>"]
    E4Mcrr2Mrrc2,
}
impl From<E74ScsIdIsar0d60CoprocInstrs> for u8 {
    fn from(value: E74ScsIdIsar0d60CoprocInstrs) -> u8 {
        match value {
            E74ScsIdIsar0d60CoprocInstrs::E0None => 0,
            E74ScsIdIsar0d60CoprocInstrs::E1Generic => 1,
            E74ScsIdIsar0d60CoprocInstrs::E2Generic2 => 2,
            E74ScsIdIsar0d60CoprocInstrs::E3McrrMrrc => 3,
            E74ScsIdIsar0d60CoprocInstrs::E4Mcrr2Mrrc2 => 4,
        }
    }
}
impl TryFrom<u8> for E74ScsIdIsar0d60CoprocInstrs {
    type Error = ();
    fn try_from(
        value: u8,
    ) -> Result<E74ScsIdIsar0d60CoprocInstrs, Self::Error> {
        match value {
            0 => Ok(Self::E0None),
            1 => Ok(Self::E1Generic),
            2 => Ok(Self::E2Generic2),
            3 => Ok(Self::E3McrrMrrc),
            4 => Ok(Self::E4Mcrr2Mrrc2),
            _ => Err(()),
        }
    }
}
#[doc = "ID_ISAR1_INTERWORK"]
#[derive(Debug, Clone, Copy)]
pub enum E75ScsIdIsar1d64InterworkInstrs {
    #[doc = "None: None<br>"]
    E0None,
    #[doc = "BX: BX<br>"]
    E1Bx,
    #[doc = "BX_BLX2: BX BLX2<br>"]
    E2BxBlx2,
}
impl From<E75ScsIdIsar1d64InterworkInstrs> for u8 {
    fn from(value: E75ScsIdIsar1d64InterworkInstrs) -> u8 {
        match value {
            E75ScsIdIsar1d64InterworkInstrs::E0None => 0,
            E75ScsIdIsar1d64InterworkInstrs::E1Bx => 1,
            E75ScsIdIsar1d64InterworkInstrs::E2BxBlx2 => 2,
        }
    }
}
impl TryFrom<u8> for E75ScsIdIsar1d64InterworkInstrs {
    type Error = ();
    fn try_from(
        value: u8,
    ) -> Result<E75ScsIdIsar1d64InterworkInstrs, Self::Error> {
        match value {
            0 => Ok(Self::E0None),
            1 => Ok(Self::E1Bx),
            2 => Ok(Self::E2BxBlx2),
            _ => Err(()),
        }
    }
}
#[doc = "E_BASIC_FULL"]
#[derive(Debug, Clone, Copy)]
pub enum E76ScsIdIsar4d70WritebackInstrs {
    #[doc = "Basic: Basic<br>"]
    E0Basic,
    #[doc = "Full: Full<br>"]
    E1Full,
}
impl From<E76ScsIdIsar4d70WritebackInstrs> for u8 {
    fn from(value: E76ScsIdIsar4d70WritebackInstrs) -> u8 {
        match value {
            E76ScsIdIsar4d70WritebackInstrs::E0Basic => 0,
            E76ScsIdIsar4d70WritebackInstrs::E1Full => 1,
        }
    }
}
impl TryFrom<u8> for E76ScsIdIsar4d70WritebackInstrs {
    type Error = ();
    fn try_from(
        value: u8,
    ) -> Result<E76ScsIdIsar4d70WritebackInstrs, Self::Error> {
        match value {
            0 => Ok(Self::E0Basic),
            1 => Ok(Self::E1Full),
            _ => Err(()),
        }
    }
}
#[doc = "E_CP_ACC_PERMISSION"]
#[derive(Debug, Clone, Copy)]
pub enum E77ScsCpacrd88Cp0 {
    #[doc = "Access_denied: Access denied<br>"]
    E0AccessDenied,
    #[doc = "Privileged_mode_access_only: Privileged mode access only<br>"]
    E1PrivilegedModeAccessOnly,
    #[doc = "Reserved: Reserved<br>"]
    E2Reserved,
    #[doc = "Full_access: Full access<br>"]
    E3FullAccess,
}
impl From<E77ScsCpacrd88Cp0> for u8 {
    fn from(value: E77ScsCpacrd88Cp0) -> u8 {
        match value {
            E77ScsCpacrd88Cp0::E0AccessDenied => 0,
            E77ScsCpacrd88Cp0::E1PrivilegedModeAccessOnly => 1,
            E77ScsCpacrd88Cp0::E2Reserved => 2,
            E77ScsCpacrd88Cp0::E3FullAccess => 3,
        }
    }
}
impl From<u8> for E77ScsCpacrd88Cp0 {
    fn from(value: u8) -> E77ScsCpacrd88Cp0 {
        match value {
            0 => Self::E0AccessDenied,
            1 => Self::E1PrivilegedModeAccessOnly,
            2 => Self::E2Reserved,
            3 => Self::E3FullAccess,
            _ => unreachable!(),
        }
    }
}
#[doc = "ACCESS_PERMISSIONS"]
#[derive(Debug, Clone, Copy)]
pub enum E78ScsMpuRasrda0Ap {
    #[doc = "Any_access_generates_a_permission_fault: Any access generates a permission fault<br>"]
    E0AnyAccessGeneratesAPermissionFault,
    #[doc = "Privileged_access_only: Privileged access only<br>"]
    E1PrivilegedAccessOnly,
    #[doc = "Any_unprivileged_write_generates_a_permission_fault: Any unprivileged write generates a permission fault<br>"]
    E2AnyUnprivilegedWriteGeneratesAPermissionFault,
    #[doc = "Full_access: Full access<br>"]
    E3FullAccess,
    #[doc = "Reserved: Reserved<br>"]
    E4Reserved,
    #[doc = "Privileged_read_only: Privileged read only<br>"]
    E5PrivilegedReadOnly,
    #[doc = "Privileged_and_unprivileged_read_only: Privileged and unprivileged read only<br>"]
    E6PrivilegedAndUnprivilegedReadOnly,
    #[doc = "Privileged_and_unprivileged_read_only: Privileged and unprivileged read only<br>"]
    E7PrivilegedAndUnprivilegedReadOnly,
}
impl From<E78ScsMpuRasrda0Ap> for u8 {
    fn from(value: E78ScsMpuRasrda0Ap) -> u8 {
        match value { E78ScsMpuRasrda0Ap :: E0AnyAccessGeneratesAPermissionFault => 0 , E78ScsMpuRasrda0Ap :: E1PrivilegedAccessOnly => 1 , E78ScsMpuRasrda0Ap :: E2AnyUnprivilegedWriteGeneratesAPermissionFault => 2 , E78ScsMpuRasrda0Ap :: E3FullAccess => 3 , E78ScsMpuRasrda0Ap :: E4Reserved => 4 , E78ScsMpuRasrda0Ap :: E5PrivilegedReadOnly => 5 , E78ScsMpuRasrda0Ap :: E6PrivilegedAndUnprivilegedReadOnly => 6 , E78ScsMpuRasrda0Ap :: E7PrivilegedAndUnprivilegedReadOnly => 7 , }
    }
}
impl From<u8> for E78ScsMpuRasrda0Ap {
    fn from(value: u8) -> E78ScsMpuRasrda0Ap {
        match value {
            0 => Self::E0AnyAccessGeneratesAPermissionFault,
            1 => Self::E1PrivilegedAccessOnly,
            2 => Self::E2AnyUnprivilegedWriteGeneratesAPermissionFault,
            3 => Self::E3FullAccess,
            4 => Self::E4Reserved,
            5 => Self::E5PrivilegedReadOnly,
            6 => Self::E6PrivilegedAndUnprivilegedReadOnly,
            7 => Self::E7PrivilegedAndUnprivilegedReadOnly,
            _ => unreachable!(),
        }
    }
}
#[doc = "E_MVFR0_SIMD"]
#[derive(Debug, Clone, Copy)]
pub enum E79ScsMvfr0f40ASimd {
    #[doc = "_16x64:  16x64<br>"]
    E0_16x64,
}
impl From<E79ScsMvfr0f40ASimd> for u8 {
    fn from(value: E79ScsMvfr0f40ASimd) -> u8 {
        match value {
            E79ScsMvfr0f40ASimd::E0_16x64 => 1,
        }
    }
}
impl TryFrom<u8> for E79ScsMvfr0f40ASimd {
    type Error = ();
    fn try_from(value: u8) -> Result<E79ScsMvfr0f40ASimd, Self::Error> {
        match value {
            1 => Ok(Self::E0_16x64),
            _ => Err(()),
        }
    }
}
#[doc = "E_SUPPORTED2"]
#[derive(Debug, Clone, Copy)]
pub enum E80ScsMvfr0f40SinglePrecision {
    #[doc = "Supported: Supported<br>"]
    E0Supported,
}
impl From<E80ScsMvfr0f40SinglePrecision> for u8 {
    fn from(value: E80ScsMvfr0f40SinglePrecision) -> u8 {
        match value {
            E80ScsMvfr0f40SinglePrecision::E0Supported => 2,
        }
    }
}
impl TryFrom<u8> for E80ScsMvfr0f40SinglePrecision {
    type Error = ();
    fn try_from(
        value: u8,
    ) -> Result<E80ScsMvfr0f40SinglePrecision, Self::Error> {
        match value {
            2 => Ok(Self::E0Supported),
            _ => Err(()),
        }
    }
}
#[doc = "E_NOT_SUPPORTED"]
#[derive(Debug, Clone, Copy)]
pub enum E81ScsMvfr0f40DoublePrecision {
    #[doc = "Not_Supported: Not Supported<br>"]
    E0NotSupported,
}
impl From<E81ScsMvfr0f40DoublePrecision> for u8 {
    fn from(value: E81ScsMvfr0f40DoublePrecision) -> u8 {
        match value {
            E81ScsMvfr0f40DoublePrecision::E0NotSupported => 0,
        }
    }
}
impl TryFrom<u8> for E81ScsMvfr0f40DoublePrecision {
    type Error = ();
    fn try_from(
        value: u8,
    ) -> Result<E81ScsMvfr0f40DoublePrecision, Self::Error> {
        match value {
            0 => Ok(Self::E0NotSupported),
            _ => Err(()),
        }
    }
}
#[doc = "E_SUPPORTED"]
#[derive(Debug, Clone, Copy)]
pub enum E82ScsMvfr0f40Divide {
    #[doc = "Supported: Supported<br>"]
    E0Supported,
}
impl From<E82ScsMvfr0f40Divide> for u8 {
    fn from(value: E82ScsMvfr0f40Divide) -> u8 {
        match value {
            E82ScsMvfr0f40Divide::E0Supported => 1,
        }
    }
}
impl TryFrom<u8> for E82ScsMvfr0f40Divide {
    type Error = ();
    fn try_from(value: u8) -> Result<E82ScsMvfr0f40Divide, Self::Error> {
        match value {
            1 => Ok(Self::E0Supported),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E83Apb35Enable500Enable {
    #[doc = "Disabled: Disable SPI<br>Disabled: Disable SPIM<br>Disabled: Disable SPI slave<br>"]
    E0Disabled,
    #[doc = "Enabled: Enable SPI<br>"]
    E1Enabled,
    #[doc = "Enabled: Enable SPI slave<br>"]
    E2Enabled,
    #[doc = "Enabled: Enable SPIM<br>"]
    E3Enabled,
}
impl From<E83Apb35Enable500Enable> for u8 {
    fn from(value: E83Apb35Enable500Enable) -> u8 {
        match value {
            E83Apb35Enable500Enable::E0Disabled => 0,
            E83Apb35Enable500Enable::E1Enabled => 1,
            E83Apb35Enable500Enable::E2Enabled => 2,
            E83Apb35Enable500Enable::E3Enabled => 7,
        }
    }
}
impl TryFrom<u8> for E83Apb35Enable500Enable {
    type Error = ();
    fn try_from(value: u8) -> Result<E83Apb35Enable500Enable, Self::Error> {
        match value {
            0 => Ok(Self::E0Disabled),
            1 => Ok(Self::E1Enabled),
            2 => Ok(Self::E2Enabled),
            7 => Ok(Self::E3Enabled),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E84Apb35Frequency524Frequency {
    #[doc = "K125: 125 kbps<br>K125: 125 kbps<br>"]
    E0K125,
    #[doc = "K250: 250 kbps<br>K250: 250 kbps<br>"]
    E1K250,
    #[doc = "K500: 500 kbps<br>K500: 500 kbps<br>"]
    E2K500,
    #[doc = "M1: 1 Mbps<br>M1: 1 Mbps<br>"]
    E3M1,
    #[doc = "M2: 2 Mbps<br>M2: 2 Mbps<br>"]
    E4M2,
    #[doc = "M4: 4 Mbps<br>M4: 4 Mbps<br>"]
    E5M4,
    #[doc = "M8: 8 Mbps<br>M8: 8 Mbps<br>"]
    E6M8,
}
impl From<E84Apb35Frequency524Frequency> for u32 {
    fn from(value: E84Apb35Frequency524Frequency) -> u32 {
        match value {
            E84Apb35Frequency524Frequency::E0K125 => 33554432,
            E84Apb35Frequency524Frequency::E1K250 => 67108864,
            E84Apb35Frequency524Frequency::E2K500 => 134217728,
            E84Apb35Frequency524Frequency::E3M1 => 268435456,
            E84Apb35Frequency524Frequency::E4M2 => 536870912,
            E84Apb35Frequency524Frequency::E5M4 => 1073741824,
            E84Apb35Frequency524Frequency::E6M8 => 2147483648,
        }
    }
}
impl TryFrom<u32> for E84Apb35Frequency524Frequency {
    type Error = ();
    fn try_from(
        value: u32,
    ) -> Result<E84Apb35Frequency524Frequency, Self::Error> {
        match value {
            33554432 => Ok(Self::E0K125),
            67108864 => Ok(Self::E1K250),
            134217728 => Ok(Self::E2K500),
            268435456 => Ok(Self::E3M1),
            536870912 => Ok(Self::E4M2),
            1073741824 => Ok(Self::E5M4),
            2147483648 => Ok(Self::E6M8),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E85Apb35RxdListcList {
    #[doc = "Disabled: Disable EasyDMA list<br>"]
    E0Disabled,
    #[doc = "ArrayList: Use array list<br>"]
    E1Arraylist,
}
impl From<E85Apb35RxdListcList> for u8 {
    fn from(value: E85Apb35RxdListcList) -> u8 {
        match value {
            E85Apb35RxdListcList::E0Disabled => 0,
            E85Apb35RxdListcList::E1Arraylist => 1,
        }
    }
}
impl TryFrom<u8> for E85Apb35RxdListcList {
    type Error = ();
    fn try_from(value: u8) -> Result<E85Apb35RxdListcList, Self::Error> {
        match value {
            0 => Ok(Self::E0Disabled),
            1 => Ok(Self::E1Arraylist),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum E86Apb35TxdListcList {
    #[doc = "Disabled: Disable EasyDMA list<br>"]
    E0Disabled,
    #[doc = "ArrayList: Use array list<br>"]
    E1Arraylist,
}
impl From<E86Apb35TxdListcList> for u8 {
    fn from(value: E86Apb35TxdListcList) -> u8 {
        match value {
            E86Apb35TxdListcList::E0Disabled => 0,
            E86Apb35TxdListcList::E1Arraylist => 1,
        }
    }
}
impl TryFrom<u8> for E86Apb35TxdListcList {
    type Error = ();
    fn try_from(value: u8) -> Result<E86Apb35TxdListcList, Self::Error> {
        match value {
            0 => Ok(Self::E0Disabled),
            1 => Ok(Self::E1Arraylist),
            _ => Err(()),
        }
    }
}
