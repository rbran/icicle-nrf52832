use icicle_vm::cpu::mem::MemResult;
#[derive(Default)]
#[doc = "QDEC: Quadrature Decoder<br><br>Instances:<br>0x40012000: QDEC<br>"]
pub struct Qdec {
    #[doc = "TODO: implement things here"]
    _todo: (),
}
impl Qdec {
    pub(crate) fn page_to_index(page: u64) -> usize {
        match page {
            262162u64 => 0usize,
            _ => unreachable!(),
        }
    }
    #[doc = "TASKS_START: Task starting the quadrature decoder<br>"]
    pub(crate) fn qdec_tasks_start0_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write qdec_tasks_start0 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_STOP: Task stopping the quadrature decoder<br>"]
    pub(crate) fn qdec_tasks_stop4_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write qdec_tasks_stop4 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_READCLRACC: Read and clear ACC and ACCDBL<br>"]
    pub(crate) fn qdec_tasks_readclracc8_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write qdec_tasks_readclracc8 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_RDCLRACC: Read and clear ACC<br>"]
    pub(crate) fn qdec_tasks_rdclraccc_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write qdec_tasks_rdclraccc mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "TASKS_RDCLRDBL: Read and clear ACCDBL<br>"]
    pub(crate) fn qdec_tasks_rdclrdbl10_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write qdec_tasks_rdclrdbl10 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_SAMPLERDY: Event being generated for every new sample value written to the SAMPLE register<br>"]
    pub(crate) fn qdec_events_samplerdy100_read(&self) -> MemResult<u32> {
        todo ! ("read qdec_events_samplerdy100 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_SAMPLERDY: Event being generated for every new sample value written to the SAMPLE register<br>"]
    pub(crate) fn qdec_events_samplerdy100_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write qdec_events_samplerdy100 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_REPORTRDY: Non-null report ready<br>"]
    pub(crate) fn qdec_events_reportrdy104_read(&self) -> MemResult<u32> {
        todo ! ("read qdec_events_reportrdy104 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_REPORTRDY: Non-null report ready<br>"]
    pub(crate) fn qdec_events_reportrdy104_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write qdec_events_reportrdy104 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ACCOF: ACC or ACCDBL register overflow<br>"]
    pub(crate) fn qdec_events_accof108_read(&self) -> MemResult<u32> {
        todo ! ("read qdec_events_accof108 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_ACCOF: ACC or ACCDBL register overflow<br>"]
    pub(crate) fn qdec_events_accof108_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write qdec_events_accof108 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_DBLRDY: Double displacement(s) detected<br>"]
    pub(crate) fn qdec_events_dblrdy10c_read(&self) -> MemResult<u32> {
        todo ! ("read qdec_events_dblrdy10c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_DBLRDY: Double displacement(s) detected<br>"]
    pub(crate) fn qdec_events_dblrdy10c_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write qdec_events_dblrdy10c mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_STOPPED: QDEC has been stopped<br>"]
    pub(crate) fn qdec_events_stopped110_read(&self) -> MemResult<u32> {
        todo ! ("read qdec_events_stopped110 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "EVENTS_STOPPED: QDEC has been stopped<br>"]
    pub(crate) fn qdec_events_stopped110_write(
        &mut self,
        _value: u32,
    ) -> MemResult<()> {
        todo ! ("write qdec_events_stopped110 mwrite None write None rac None reset value 0x00 mask 0x00")
    }
    #[doc = "REPORTRDY_READCLRACC: Shortcut between REPORTRDY event and READCLRACC task<br>"]
    pub(crate) fn qdec_shorts200_reportrdy_readclracc_read(
        &self,
    ) -> MemResult<bool> {
        todo ! ("read REPORTRDY_READCLRACC mwrite None write None rac None reset value false")
    }
    #[doc = "REPORTRDY_READCLRACC: Shortcut between REPORTRDY event and READCLRACC task<br>"]
    pub(crate) fn qdec_shorts200_reportrdy_readclracc_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write REPORTRDY_READCLRACC mwrite None write None rac None reset value false")
    }
    #[doc = "SAMPLERDY_STOP: Shortcut between SAMPLERDY event and STOP task<br>"]
    pub(crate) fn qdec_shorts200_samplerdy_stop_read(&self) -> MemResult<bool> {
        todo ! ("read SAMPLERDY_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "SAMPLERDY_STOP: Shortcut between SAMPLERDY event and STOP task<br>"]
    pub(crate) fn qdec_shorts200_samplerdy_stop_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write SAMPLERDY_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "REPORTRDY_RDCLRACC: Shortcut between REPORTRDY event and RDCLRACC task<br>"]
    pub(crate) fn qdec_shorts200_reportrdy_rdclracc_read(
        &self,
    ) -> MemResult<bool> {
        todo ! ("read REPORTRDY_RDCLRACC mwrite None write None rac None reset value false")
    }
    #[doc = "REPORTRDY_RDCLRACC: Shortcut between REPORTRDY event and RDCLRACC task<br>"]
    pub(crate) fn qdec_shorts200_reportrdy_rdclracc_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write REPORTRDY_RDCLRACC mwrite None write None rac None reset value false")
    }
    #[doc = "REPORTRDY_STOP: Shortcut between REPORTRDY event and STOP task<br>"]
    pub(crate) fn qdec_shorts200_reportrdy_stop_read(&self) -> MemResult<bool> {
        todo ! ("read REPORTRDY_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "REPORTRDY_STOP: Shortcut between REPORTRDY event and STOP task<br>"]
    pub(crate) fn qdec_shorts200_reportrdy_stop_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write REPORTRDY_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "DBLRDY_RDCLRDBL: Shortcut between DBLRDY event and RDCLRDBL task<br>"]
    pub(crate) fn qdec_shorts200_dblrdy_rdclrdbl_read(
        &self,
    ) -> MemResult<bool> {
        todo ! ("read DBLRDY_RDCLRDBL mwrite None write None rac None reset value false")
    }
    #[doc = "DBLRDY_RDCLRDBL: Shortcut between DBLRDY event and RDCLRDBL task<br>"]
    pub(crate) fn qdec_shorts200_dblrdy_rdclrdbl_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write DBLRDY_RDCLRDBL mwrite None write None rac None reset value false")
    }
    #[doc = "DBLRDY_STOP: Shortcut between DBLRDY event and STOP task<br>"]
    pub(crate) fn qdec_shorts200_dblrdy_stop_read(&self) -> MemResult<bool> {
        todo ! ("read DBLRDY_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "DBLRDY_STOP: Shortcut between DBLRDY event and STOP task<br>"]
    pub(crate) fn qdec_shorts200_dblrdy_stop_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write DBLRDY_STOP mwrite None write None rac None reset value false")
    }
    #[doc = "SAMPLERDY_READCLRACC: Shortcut between SAMPLERDY event and READCLRACC task<br>"]
    pub(crate) fn qdec_shorts200_samplerdy_readclracc_read(
        &self,
    ) -> MemResult<bool> {
        todo ! ("read SAMPLERDY_READCLRACC mwrite None write None rac None reset value false")
    }
    #[doc = "SAMPLERDY_READCLRACC: Shortcut between SAMPLERDY event and READCLRACC task<br>"]
    pub(crate) fn qdec_shorts200_samplerdy_readclracc_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo ! ("write SAMPLERDY_READCLRACC mwrite None write None rac None reset value false")
    }
    #[doc = "SAMPLERDY: Write '1' to Enable interrupt for SAMPLERDY event<br>"]
    pub(crate) fn qdec_intenset304_samplerdy_read(&self) -> MemResult<bool> {
        todo!(
            "read SAMPLERDY mwrite None write None rac None reset value false"
        )
    }
    #[doc = "SAMPLERDY: Write '1' to Enable interrupt for SAMPLERDY event<br>"]
    pub(crate) fn qdec_intenset304_samplerdy_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write SAMPLERDY mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REPORTRDY: Write '1' to Enable interrupt for REPORTRDY event<br>"]
    pub(crate) fn qdec_intenset304_reportrdy_read(&self) -> MemResult<bool> {
        todo!(
            "read REPORTRDY mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REPORTRDY: Write '1' to Enable interrupt for REPORTRDY event<br>"]
    pub(crate) fn qdec_intenset304_reportrdy_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REPORTRDY mwrite None write None rac None reset value false"
        )
    }
    #[doc = "ACCOF: Write '1' to Enable interrupt for ACCOF event<br>"]
    pub(crate) fn qdec_intenset304_accof_read(&self) -> MemResult<bool> {
        todo!("read ACCOF mwrite None write None rac None reset value false")
    }
    #[doc = "ACCOF: Write '1' to Enable interrupt for ACCOF event<br>"]
    pub(crate) fn qdec_intenset304_accof_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ACCOF mwrite None write None rac None reset value false")
    }
    #[doc = "DBLRDY: Write '1' to Enable interrupt for DBLRDY event<br>"]
    pub(crate) fn qdec_intenset304_dblrdy_read(&self) -> MemResult<bool> {
        todo!("read DBLRDY mwrite None write None rac None reset value false")
    }
    #[doc = "DBLRDY: Write '1' to Enable interrupt for DBLRDY event<br>"]
    pub(crate) fn qdec_intenset304_dblrdy_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write DBLRDY mwrite None write None rac None reset value false")
    }
    #[doc = "STOPPED: Write '1' to Enable interrupt for STOPPED event<br>"]
    pub(crate) fn qdec_intenset304_stopped_read(&self) -> MemResult<bool> {
        todo!("read STOPPED mwrite None write None rac None reset value false")
    }
    #[doc = "STOPPED: Write '1' to Enable interrupt for STOPPED event<br>"]
    pub(crate) fn qdec_intenset304_stopped_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write STOPPED mwrite None write None rac None reset value false")
    }
    #[doc = "SAMPLERDY: Write '1' to Disable interrupt for SAMPLERDY event<br>"]
    pub(crate) fn qdec_intenclr308_samplerdy_read(&self) -> MemResult<bool> {
        todo!(
            "read SAMPLERDY mwrite None write None rac None reset value false"
        )
    }
    #[doc = "SAMPLERDY: Write '1' to Disable interrupt for SAMPLERDY event<br>"]
    pub(crate) fn qdec_intenclr308_samplerdy_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write SAMPLERDY mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REPORTRDY: Write '1' to Disable interrupt for REPORTRDY event<br>"]
    pub(crate) fn qdec_intenclr308_reportrdy_read(&self) -> MemResult<bool> {
        todo!(
            "read REPORTRDY mwrite None write None rac None reset value false"
        )
    }
    #[doc = "REPORTRDY: Write '1' to Disable interrupt for REPORTRDY event<br>"]
    pub(crate) fn qdec_intenclr308_reportrdy_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!(
            "write REPORTRDY mwrite None write None rac None reset value false"
        )
    }
    #[doc = "ACCOF: Write '1' to Disable interrupt for ACCOF event<br>"]
    pub(crate) fn qdec_intenclr308_accof_read(&self) -> MemResult<bool> {
        todo!("read ACCOF mwrite None write None rac None reset value false")
    }
    #[doc = "ACCOF: Write '1' to Disable interrupt for ACCOF event<br>"]
    pub(crate) fn qdec_intenclr308_accof_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ACCOF mwrite None write None rac None reset value false")
    }
    #[doc = "DBLRDY: Write '1' to Disable interrupt for DBLRDY event<br>"]
    pub(crate) fn qdec_intenclr308_dblrdy_read(&self) -> MemResult<bool> {
        todo!("read DBLRDY mwrite None write None rac None reset value false")
    }
    #[doc = "DBLRDY: Write '1' to Disable interrupt for DBLRDY event<br>"]
    pub(crate) fn qdec_intenclr308_dblrdy_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write DBLRDY mwrite None write None rac None reset value false")
    }
    #[doc = "STOPPED: Write '1' to Disable interrupt for STOPPED event<br>"]
    pub(crate) fn qdec_intenclr308_stopped_read(&self) -> MemResult<bool> {
        todo!("read STOPPED mwrite None write None rac None reset value false")
    }
    #[doc = "STOPPED: Write '1' to Disable interrupt for STOPPED event<br>"]
    pub(crate) fn qdec_intenclr308_stopped_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write STOPPED mwrite None write None rac None reset value false")
    }
    #[doc = "ENABLE: Enable or disable the quadrature decoder<br>"]
    pub(crate) fn qdec_enable500_enable_read(&self) -> MemResult<bool> {
        todo!("read ENABLE mwrite None write None rac None reset value false")
    }
    #[doc = "ENABLE: Enable or disable the quadrature decoder<br>"]
    pub(crate) fn qdec_enable500_enable_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write ENABLE mwrite None write None rac None reset value false")
    }
    #[doc = "LEDPOL: LED output pin polarity<br>"]
    pub(crate) fn qdec_ledpol504_ledpol_read(&self) -> MemResult<bool> {
        todo!("read LEDPOL mwrite None write None rac None reset value false")
    }
    #[doc = "LEDPOL: LED output pin polarity<br>"]
    pub(crate) fn qdec_ledpol504_ledpol_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write LEDPOL mwrite None write None rac None reset value false")
    }
    #[doc = "SAMPLEPER: Sample period. The SAMPLE register will be updated for every new sample<br>"]
    pub(crate) fn qdec_sampleper508_sampleper_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E42QdecSampleper508Sampleper> {
        todo ! ("read SAMPLEPER mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "SAMPLEPER: Sample period. The SAMPLE register will be updated for every new sample<br>"]
    pub(crate) fn qdec_sampleper508_sampleper_write(
        &mut self,
        _value: crate::peripheral::enums::E42QdecSampleper508Sampleper,
    ) -> MemResult<()> {
        todo ! ("write SAMPLEPER mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "SAMPLE: Last motion sample<br>"]
    pub(crate) fn qdec_sample50c_sample_read(&self) -> MemResult<u32> {
        todo ! ("read SAMPLE mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "REPORTPER: Specifies the number of samples to be accumulated in the ACC register before the REPORTRDY and DBLRDY events can be generated<br>"]
    pub(crate) fn qdec_reportper510_reportper_read(
        &self,
    ) -> MemResult<crate::peripheral::enums::E43QdecReportper510Reportper> {
        todo ! ("read REPORTPER mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "REPORTPER: Specifies the number of samples to be accumulated in the ACC register before the REPORTRDY and DBLRDY events can be generated<br>"]
    pub(crate) fn qdec_reportper510_reportper_write(
        &mut self,
        _value: crate::peripheral::enums::E43QdecReportper510Reportper,
    ) -> MemResult<()> {
        todo ! ("write REPORTPER mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "ACC: Register accumulating all valid samples (not double transition) read from the SAMPLE register<br>"]
    pub(crate) fn qdec_acc514_acc_read(&self) -> MemResult<u32> {
        todo ! ("read ACC mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "ACCREAD: Snapshot of the ACC register.<br>"]
    pub(crate) fn qdec_accread518_accread_read(&self) -> MemResult<u32> {
        todo ! ("read ACCREAD mwrite None write None rac None reset value 0x00 mask 0xffffffff")
    }
    #[doc = "PIN: Pin number<br>"]
    pub(crate) fn qdec_psel_led0_pin_read(&self) -> MemResult<u8> {
        todo ! ("read PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "PIN: Pin number<br>"]
    pub(crate) fn qdec_psel_led0_pin_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn qdec_psel_led0_connect_read(&self) -> MemResult<bool> {
        todo!("read CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn qdec_psel_led0_connect_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "PIN: Pin number<br>"]
    pub(crate) fn qdec_psel_a4_pin_read(&self) -> MemResult<u8> {
        todo ! ("read PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "PIN: Pin number<br>"]
    pub(crate) fn qdec_psel_a4_pin_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn qdec_psel_a4_connect_read(&self) -> MemResult<bool> {
        todo!("read CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn qdec_psel_a4_connect_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "PIN: Pin number<br>"]
    pub(crate) fn qdec_psel_b8_pin_read(&self) -> MemResult<u8> {
        todo ! ("read PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "PIN: Pin number<br>"]
    pub(crate) fn qdec_psel_b8_pin_write(
        &mut self,
        _value: u8,
    ) -> MemResult<()> {
        todo ! ("write PIN mwrite None write None rac None reset value 0x1f mask 0x1f")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn qdec_psel_b8_connect_read(&self) -> MemResult<bool> {
        todo!("read CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "CONNECT: Connection<br>"]
    pub(crate) fn qdec_psel_b8_connect_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write CONNECT mwrite None write None rac None reset value true")
    }
    #[doc = "DBFEN: Enable input debounce filters<br>"]
    pub(crate) fn qdec_dbfen528_dbfen_read(&self) -> MemResult<bool> {
        todo!("read DBFEN mwrite None write None rac None reset value false")
    }
    #[doc = "DBFEN: Enable input debounce filters<br>"]
    pub(crate) fn qdec_dbfen528_dbfen_write(
        &mut self,
        _value: bool,
    ) -> MemResult<()> {
        todo!("write DBFEN mwrite None write None rac None reset value false")
    }
    #[doc = "LEDPRE: Period in us the LED is switched on prior to sampling<br>"]
    pub(crate) fn qdec_ledpre540_ledpre_read(&self) -> MemResult<u16> {
        todo ! ("read LEDPRE mwrite None write None rac None reset value 0x10 mask 0x1ff")
    }
    #[doc = "LEDPRE: Period in us the LED is switched on prior to sampling<br>"]
    pub(crate) fn qdec_ledpre540_ledpre_write(
        &mut self,
        _value: u16,
    ) -> MemResult<()> {
        todo ! ("write LEDPRE mwrite None write None rac None reset value 0x10 mask 0x1ff")
    }
    #[doc = "ACCDBL: Register accumulating the number of detected double or illegal transitions. ( SAMPLE = 2 ).<br>"]
    pub(crate) fn qdec_accdbl544_accdbl_read(&self) -> MemResult<u8> {
        todo ! ("read ACCDBL mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
    #[doc = "ACCDBLREAD: Snapshot of the ACCDBL register. This field is updated when the READCLRACC or RDCLRDBL task is triggered.<br>"]
    pub(crate) fn qdec_accdblread548_accdblread_read(&self) -> MemResult<u8> {
        todo ! ("read ACCDBLREAD mwrite None write None rac None reset value 0x00 mask 0x0f")
    }
}
