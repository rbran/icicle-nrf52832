#[doc = "Peripherals BPROT, POWER, CLOCK"]
pub mod apb0;
#[doc = "Peripherals CCM, AAR"]
pub mod apb15;
#[doc = "Peripherals COMP, LPCOMP"]
pub mod apb19;
#[doc = "Peripherals UARTE0, UART0"]
pub mod apb2;
#[doc = "Peripherals SWI0, EGU0"]
pub mod apb20;
#[doc = "Peripherals SPIM0, SPIS0, TWIM0, TWIS0, SPI0, TWI0"]
pub mod apb3;
#[doc = "Peripherals SPIM0, SPIS0, SPI0"]
pub mod apb35;
#[doc = "Peripheral DWT"]
pub mod dwt;
#[doc = "Peripheral ECB"]
pub mod ecb;
#[doc = "Values used for read/write by the peripheral fields"]
pub mod enums;
#[doc = "Peripheral FICR"]
pub mod ficr;
#[doc = "Peripheral FPU"]
pub mod fpu;
#[doc = "Peripheral GPIOTE"]
pub mod gpiote;
#[doc = "Peripheral I2S"]
pub mod i2s;
#[doc = "Peripheral MWU"]
pub mod mwu;
#[doc = "Peripheral NFCT"]
pub mod nfct;
#[doc = "Peripheral NVMC"]
pub mod nvmc;
#[doc = "Peripheral P0"]
pub mod p0;
#[doc = "Peripheral PDM"]
pub mod pdm;
#[doc = "Peripheral PPI"]
pub mod ppi;
#[doc = "Peripheral PWM0"]
pub mod pwm0;
#[doc = "Peripheral QDEC"]
pub mod qdec;
#[doc = "Peripheral RADIO"]
pub mod radio;
#[doc = "Register read/write from pages, used for behavior affecting multiple peripherals"]
pub(crate) mod registers;
#[doc = "Peripheral RNG"]
pub mod rng;
#[doc = "Peripheral RTC0"]
pub mod rtc0;
#[doc = "Peripheral SAADC"]
pub mod saadc;
#[doc = "Peripherals Control, ID, FPE, SysTick, NVIC, MPU"]
pub mod scs;
#[doc = "Peripheral TEMP"]
pub mod temp;
#[doc = "Peripheral TIMER0"]
pub mod timer0;
#[doc = "Peripheral TIMER3"]
pub mod timer3;
#[doc = "Peripheral UICR"]
pub mod uicr;
#[doc = "Peripheral WDT"]
pub mod wdt;
#[doc = "A Global Peripheral device"]
#[derive(Default)]
pub struct Peripherals {
    #[doc = "Peripheral at 0x10000000"]
    pub ficr: ficr::Ficr,
    #[doc = "Peripheral at 0x10001000"]
    pub uicr: uicr::Uicr,
    #[doc = "Peripheral at 0x40000000"]
    pub apb0: apb0::Apb0,
    #[doc = "Peripheral at 0x40001000"]
    pub radio: radio::Radio,
    #[doc = "Peripheral at 0x40002000"]
    pub apb2: apb2::Apb2,
    #[doc = "Peripheral at 0x40003000, 0x40004000"]
    pub apb3: [apb3::Apb3; 2],
    #[doc = "Peripheral at 0x40005000"]
    pub nfct: nfct::Nfct,
    #[doc = "Peripheral at 0x40006000"]
    pub gpiote: gpiote::Gpiote,
    #[doc = "Peripheral at 0x40007000"]
    pub saadc: saadc::Saadc,
    #[doc = "Peripheral at 0x40008000, 0x40009000, 0x4000a000"]
    pub timer0: [timer0::Timer0; 3],
    #[doc = "Peripheral at 0x4000b000, 0x40011000, 0x40024000"]
    pub rtc0: [rtc0::Rtc0; 3],
    #[doc = "Peripheral at 0x4000c000"]
    pub temp: temp::Temp,
    #[doc = "Peripheral at 0x4000d000"]
    pub rng: rng::Rng,
    #[doc = "Peripheral at 0x4000e000"]
    pub ecb: ecb::Ecb,
    #[doc = "Peripheral at 0x4000f000"]
    pub apb15: apb15::Apb15,
    #[doc = "Peripheral at 0x40010000"]
    pub wdt: wdt::Wdt,
    #[doc = "Peripheral at 0x40012000"]
    pub qdec: qdec::Qdec,
    #[doc = "Peripheral at 0x40013000"]
    pub apb19: apb19::Apb19,
    #[doc = "Peripheral at 0x40014000, 0x40015000, 0x40016000, 0x40017000, 0x40018000, 0x40019000"]
    pub apb20: [apb20::Apb20; 6],
    #[doc = "Peripheral at 0x4001a000, 0x4001b000"]
    pub timer3: [timer3::Timer3; 2],
    #[doc = "Peripheral at 0x4001c000, 0x40021000, 0x40022000"]
    pub pwm0: [pwm0::Pwm0; 3],
    #[doc = "Peripheral at 0x4001d000"]
    pub pdm: pdm::Pdm,
    #[doc = "Peripheral at 0x4001e000"]
    pub nvmc: nvmc::Nvmc,
    #[doc = "Peripheral at 0x4001f000"]
    pub ppi: ppi::Ppi,
    #[doc = "Peripheral at 0x40020000"]
    pub mwu: mwu::Mwu,
    #[doc = "Peripheral at 0x40025000"]
    pub i2s: i2s::I2s,
    #[doc = "Peripheral at 0x40026000"]
    pub fpu: fpu::Fpu,
    #[doc = "Peripheral at 0x50000000"]
    pub p0: p0::P0,
    #[doc = "Peripheral at 0xe0001000"]
    pub dwt: dwt::Dwt,
    #[doc = "Peripheral at 0xe000e000"]
    pub scs: scs::Scs,
    #[doc = "Peripheral at 0x40023000"]
    pub apb35: apb35::Apb35,
}
