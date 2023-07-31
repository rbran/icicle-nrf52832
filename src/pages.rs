fn buffer_mut(
    _start: u64,
    _end: u64,
    _byte: u64,
    _buf: &[u8],
) -> Option<&mut u8> {
    if _start > _byte || _end <= _byte {
        return None;
    }
    let addr = _buf.as_ptr() as usize + (_byte - _start) as usize;
    Some(unsafe { std::mem::transmute(addr) })
}
fn buffer_const(
    _start: u64,
    _end: u64,
    _byte: u64,
    _buf: &[u8],
) -> Option<&u8> {
    if _start > _byte || _end <= _byte {
        return None;
    }
    Some(&_buf[(_byte - _start) as usize])
}
pub struct PeripheralPage0x10000000(
    pub std::sync::Arc<std::sync::Mutex<super::peripheral::Peripherals>>,
);
impl icicle_vm::cpu::mem::IoMemory for PeripheralPage0x10000000 {
    fn read(
        &mut self,
        _addr: u64,
        _buf: &mut [u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 268435456;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (16..=23, 17..=24) => {
                if (_start >= 16 && _start < 20) || (_end > 16 && _end <= 20) {
                    self.0.lock().unwrap().read_ficr_codepagesize(
                        &mut buffer_mut(_start, _end, 16, _buf),
                        &mut buffer_mut(_start, _end, 17, _buf),
                        &mut buffer_mut(_start, _end, 18, _buf),
                        &mut buffer_mut(_start, _end, 19, _buf),
                    )?;
                }
                if (_start >= 20 && _start < 24) || (_end > 20 && _end <= 24) {
                    self.0.lock().unwrap().read_ficr_codesize(
                        &mut buffer_mut(_start, _end, 20, _buf),
                        &mut buffer_mut(_start, _end, 21, _buf),
                        &mut buffer_mut(_start, _end, 22, _buf),
                        &mut buffer_mut(_start, _end, 23, _buf),
                    )?;
                }
            }
            (40..=47, 41..=48) => {
                if (_start >= 40 && _start < 44) || (_end > 40 && _end <= 44) {
                    self.0.lock().unwrap().read_ficr_clenr0(
                        &mut buffer_mut(_start, _end, 40, _buf),
                        &mut buffer_mut(_start, _end, 41, _buf),
                        &mut buffer_mut(_start, _end, 42, _buf),
                        &mut buffer_mut(_start, _end, 43, _buf),
                    )?;
                }
                if (_start >= 44 && _start < 48) || (_end > 44 && _end <= 48) {
                    self.read_ficr_ppfc(
                        &mut buffer_mut(_start, _end, 44, _buf),
                        &mut buffer_mut(_start, _end, 45, _buf),
                        &mut buffer_mut(_start, _end, 46, _buf),
                        &mut buffer_mut(_start, _end, 47, _buf),
                    )?;
                }
            }
            (52..=71, 53..=72) => {
                if (_start >= 52 && _start < 56) || (_end > 52 && _end <= 56) {
                    self.0.lock().unwrap().read_ficr_numramblock(
                        &mut buffer_mut(_start, _end, 52, _buf),
                        &mut buffer_mut(_start, _end, 53, _buf),
                        &mut buffer_mut(_start, _end, 54, _buf),
                        &mut buffer_mut(_start, _end, 55, _buf),
                    )?;
                }
                if (_start >= 56 && _start < 60) || (_end > 56 && _end <= 60) {
                    self.0.lock().unwrap().read_ficr_sizeramblocks(
                        0,
                        &mut buffer_mut(_start, _end, 56, _buf),
                        &mut buffer_mut(_start, _end, 57, _buf),
                        &mut buffer_mut(_start, _end, 58, _buf),
                        &mut buffer_mut(_start, _end, 59, _buf),
                    )?;
                }
                if (_start >= 60 && _start < 64) || (_end > 60 && _end <= 64) {
                    self.0.lock().unwrap().read_ficr_sizeramblocks(
                        1,
                        &mut buffer_mut(_start, _end, 60, _buf),
                        &mut buffer_mut(_start, _end, 61, _buf),
                        &mut buffer_mut(_start, _end, 62, _buf),
                        &mut buffer_mut(_start, _end, 63, _buf),
                    )?;
                }
                if (_start >= 64 && _start < 68) || (_end > 64 && _end <= 68) {
                    self.0.lock().unwrap().read_ficr_sizeramblocks(
                        2,
                        &mut buffer_mut(_start, _end, 64, _buf),
                        &mut buffer_mut(_start, _end, 65, _buf),
                        &mut buffer_mut(_start, _end, 66, _buf),
                        &mut buffer_mut(_start, _end, 67, _buf),
                    )?;
                }
                if (_start >= 68 && _start < 72) || (_end > 68 && _end <= 72) {
                    self.0.lock().unwrap().read_ficr_sizeramblocks(
                        3,
                        &mut buffer_mut(_start, _end, 68, _buf),
                        &mut buffer_mut(_start, _end, 69, _buf),
                        &mut buffer_mut(_start, _end, 70, _buf),
                        &mut buffer_mut(_start, _end, 71, _buf),
                    )?;
                }
            }
            (92..=103, 93..=104) => {
                if (_start >= 92 && _start < 96) || (_end > 92 && _end <= 96) {
                    self.read_ficr_configid(
                        &mut buffer_mut(_start, _end, 92, _buf),
                        &mut buffer_mut(_start, _end, 93, _buf),
                        &mut buffer_mut(_start, _end, 94, _buf),
                        &mut buffer_mut(_start, _end, 95, _buf),
                    )?;
                }
                if (_start >= 96 && _start < 100) || (_end > 96 && _end <= 100)
                {
                    self.0.lock().unwrap().read_ficr_deviceidn(
                        0,
                        &mut buffer_mut(_start, _end, 96, _buf),
                        &mut buffer_mut(_start, _end, 97, _buf),
                        &mut buffer_mut(_start, _end, 98, _buf),
                        &mut buffer_mut(_start, _end, 99, _buf),
                    )?;
                }
                if (_start >= 100 && _start < 104)
                    || (_end > 100 && _end <= 104)
                {
                    self.0.lock().unwrap().read_ficr_deviceidn(
                        1,
                        &mut buffer_mut(_start, _end, 100, _buf),
                        &mut buffer_mut(_start, _end, 101, _buf),
                        &mut buffer_mut(_start, _end, 102, _buf),
                        &mut buffer_mut(_start, _end, 103, _buf),
                    )?;
                }
            }
            (128..=195, 129..=196) => {
                if (_start >= 128 && _start < 132)
                    || (_end > 128 && _end <= 132)
                {
                    self.0.lock().unwrap().read_ficr_ern(
                        0,
                        &mut buffer_mut(_start, _end, 128, _buf),
                        &mut buffer_mut(_start, _end, 129, _buf),
                        &mut buffer_mut(_start, _end, 130, _buf),
                        &mut buffer_mut(_start, _end, 131, _buf),
                    )?;
                }
                if (_start >= 132 && _start < 136)
                    || (_end > 132 && _end <= 136)
                {
                    self.0.lock().unwrap().read_ficr_ern(
                        1,
                        &mut buffer_mut(_start, _end, 132, _buf),
                        &mut buffer_mut(_start, _end, 133, _buf),
                        &mut buffer_mut(_start, _end, 134, _buf),
                        &mut buffer_mut(_start, _end, 135, _buf),
                    )?;
                }
                if (_start >= 136 && _start < 140)
                    || (_end > 136 && _end <= 140)
                {
                    self.0.lock().unwrap().read_ficr_ern(
                        2,
                        &mut buffer_mut(_start, _end, 136, _buf),
                        &mut buffer_mut(_start, _end, 137, _buf),
                        &mut buffer_mut(_start, _end, 138, _buf),
                        &mut buffer_mut(_start, _end, 139, _buf),
                    )?;
                }
                if (_start >= 140 && _start < 144)
                    || (_end > 140 && _end <= 144)
                {
                    self.0.lock().unwrap().read_ficr_ern(
                        3,
                        &mut buffer_mut(_start, _end, 140, _buf),
                        &mut buffer_mut(_start, _end, 141, _buf),
                        &mut buffer_mut(_start, _end, 142, _buf),
                        &mut buffer_mut(_start, _end, 143, _buf),
                    )?;
                }
                if (_start >= 144 && _start < 148)
                    || (_end > 144 && _end <= 148)
                {
                    self.0.lock().unwrap().read_ficr_irn(
                        0,
                        &mut buffer_mut(_start, _end, 144, _buf),
                        &mut buffer_mut(_start, _end, 145, _buf),
                        &mut buffer_mut(_start, _end, 146, _buf),
                        &mut buffer_mut(_start, _end, 147, _buf),
                    )?;
                }
                if (_start >= 148 && _start < 152)
                    || (_end > 148 && _end <= 152)
                {
                    self.0.lock().unwrap().read_ficr_irn(
                        1,
                        &mut buffer_mut(_start, _end, 148, _buf),
                        &mut buffer_mut(_start, _end, 149, _buf),
                        &mut buffer_mut(_start, _end, 150, _buf),
                        &mut buffer_mut(_start, _end, 151, _buf),
                    )?;
                }
                if (_start >= 152 && _start < 156)
                    || (_end > 152 && _end <= 156)
                {
                    self.0.lock().unwrap().read_ficr_irn(
                        2,
                        &mut buffer_mut(_start, _end, 152, _buf),
                        &mut buffer_mut(_start, _end, 153, _buf),
                        &mut buffer_mut(_start, _end, 154, _buf),
                        &mut buffer_mut(_start, _end, 155, _buf),
                    )?;
                }
                if (_start >= 156 && _start < 160)
                    || (_end > 156 && _end <= 160)
                {
                    self.0.lock().unwrap().read_ficr_irn(
                        3,
                        &mut buffer_mut(_start, _end, 156, _buf),
                        &mut buffer_mut(_start, _end, 157, _buf),
                        &mut buffer_mut(_start, _end, 158, _buf),
                        &mut buffer_mut(_start, _end, 159, _buf),
                    )?;
                }
                if (_start >= 160 && _start < 164)
                    || (_end > 160 && _end <= 164)
                {
                    self.read_ficr_deviceaddrtype(
                        &mut buffer_mut(_start, _end, 160, _buf),
                        &mut buffer_mut(_start, _end, 161, _buf),
                        &mut buffer_mut(_start, _end, 162, _buf),
                        &mut buffer_mut(_start, _end, 163, _buf),
                    )?;
                }
                if (_start >= 164 && _start < 168)
                    || (_end > 164 && _end <= 168)
                {
                    self.0.lock().unwrap().read_ficr_deviceaddrn(
                        0,
                        &mut buffer_mut(_start, _end, 164, _buf),
                        &mut buffer_mut(_start, _end, 165, _buf),
                        &mut buffer_mut(_start, _end, 166, _buf),
                        &mut buffer_mut(_start, _end, 167, _buf),
                    )?;
                }
                if (_start >= 168 && _start < 172)
                    || (_end > 168 && _end <= 172)
                {
                    self.0.lock().unwrap().read_ficr_deviceaddrn(
                        1,
                        &mut buffer_mut(_start, _end, 168, _buf),
                        &mut buffer_mut(_start, _end, 169, _buf),
                        &mut buffer_mut(_start, _end, 170, _buf),
                        &mut buffer_mut(_start, _end, 171, _buf),
                    )?;
                }
                if (_start >= 172 && _start < 176)
                    || (_end > 172 && _end <= 176)
                {
                    self.read_ficr_overrideen(
                        &mut buffer_mut(_start, _end, 172, _buf),
                        &mut buffer_mut(_start, _end, 173, _buf),
                        &mut buffer_mut(_start, _end, 174, _buf),
                        &mut buffer_mut(_start, _end, 175, _buf),
                    )?;
                }
                if (_start >= 176 && _start < 180)
                    || (_end > 176 && _end <= 180)
                {
                    self.0.lock().unwrap().read_ficr_nrf_1mbitn(
                        0,
                        &mut buffer_mut(_start, _end, 176, _buf),
                        &mut buffer_mut(_start, _end, 177, _buf),
                        &mut buffer_mut(_start, _end, 178, _buf),
                        &mut buffer_mut(_start, _end, 179, _buf),
                    )?;
                }
                if (_start >= 180 && _start < 184)
                    || (_end > 180 && _end <= 184)
                {
                    self.0.lock().unwrap().read_ficr_nrf_1mbitn(
                        1,
                        &mut buffer_mut(_start, _end, 180, _buf),
                        &mut buffer_mut(_start, _end, 181, _buf),
                        &mut buffer_mut(_start, _end, 182, _buf),
                        &mut buffer_mut(_start, _end, 183, _buf),
                    )?;
                }
                if (_start >= 184 && _start < 188)
                    || (_end > 184 && _end <= 188)
                {
                    self.0.lock().unwrap().read_ficr_nrf_1mbitn(
                        2,
                        &mut buffer_mut(_start, _end, 184, _buf),
                        &mut buffer_mut(_start, _end, 185, _buf),
                        &mut buffer_mut(_start, _end, 186, _buf),
                        &mut buffer_mut(_start, _end, 187, _buf),
                    )?;
                }
                if (_start >= 188 && _start < 192)
                    || (_end > 188 && _end <= 192)
                {
                    self.0.lock().unwrap().read_ficr_nrf_1mbitn(
                        3,
                        &mut buffer_mut(_start, _end, 188, _buf),
                        &mut buffer_mut(_start, _end, 189, _buf),
                        &mut buffer_mut(_start, _end, 190, _buf),
                        &mut buffer_mut(_start, _end, 191, _buf),
                    )?;
                }
                if (_start >= 192 && _start < 196)
                    || (_end > 192 && _end <= 196)
                {
                    self.0.lock().unwrap().read_ficr_nrf_1mbitn(
                        4,
                        &mut buffer_mut(_start, _end, 192, _buf),
                        &mut buffer_mut(_start, _end, 193, _buf),
                        &mut buffer_mut(_start, _end, 194, _buf),
                        &mut buffer_mut(_start, _end, 195, _buf),
                    )?;
                }
            }
            (236..=255, 237..=256) => {
                if (_start >= 236 && _start < 240)
                    || (_end > 236 && _end <= 240)
                {
                    self.0.lock().unwrap().read_ficr_ble_1mbitn(
                        0,
                        &mut buffer_mut(_start, _end, 236, _buf),
                        &mut buffer_mut(_start, _end, 237, _buf),
                        &mut buffer_mut(_start, _end, 238, _buf),
                        &mut buffer_mut(_start, _end, 239, _buf),
                    )?;
                }
                if (_start >= 240 && _start < 244)
                    || (_end > 240 && _end <= 244)
                {
                    self.0.lock().unwrap().read_ficr_ble_1mbitn(
                        1,
                        &mut buffer_mut(_start, _end, 240, _buf),
                        &mut buffer_mut(_start, _end, 241, _buf),
                        &mut buffer_mut(_start, _end, 242, _buf),
                        &mut buffer_mut(_start, _end, 243, _buf),
                    )?;
                }
                if (_start >= 244 && _start < 248)
                    || (_end > 244 && _end <= 248)
                {
                    self.0.lock().unwrap().read_ficr_ble_1mbitn(
                        2,
                        &mut buffer_mut(_start, _end, 244, _buf),
                        &mut buffer_mut(_start, _end, 245, _buf),
                        &mut buffer_mut(_start, _end, 246, _buf),
                        &mut buffer_mut(_start, _end, 247, _buf),
                    )?;
                }
                if (_start >= 248 && _start < 252)
                    || (_end > 248 && _end <= 252)
                {
                    self.0.lock().unwrap().read_ficr_ble_1mbitn(
                        3,
                        &mut buffer_mut(_start, _end, 248, _buf),
                        &mut buffer_mut(_start, _end, 249, _buf),
                        &mut buffer_mut(_start, _end, 250, _buf),
                        &mut buffer_mut(_start, _end, 251, _buf),
                    )?;
                }
                if (_start >= 252 && _start < 256)
                    || (_end > 252 && _end <= 256)
                {
                    self.0.lock().unwrap().read_ficr_ble_1mbitn(
                        4,
                        &mut buffer_mut(_start, _end, 252, _buf),
                        &mut buffer_mut(_start, _end, 253, _buf),
                        &mut buffer_mut(_start, _end, 254, _buf),
                        &mut buffer_mut(_start, _end, 255, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
    fn write(
        &mut self,
        _addr: u64,
        _buf: &[u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 268435456;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (16..=23, 17..=24) => {
                if (_start >= 16 && _start < 20) || (_end > 16 && _end <= 20) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 20 && _start < 24) || (_end > 20 && _end <= 24) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (40..=47, 41..=48) => {
                if (_start >= 40 && _start < 44) || (_end > 40 && _end <= 44) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 44 && _start < 48) || (_end > 44 && _end <= 48) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (52..=71, 53..=72) => {
                if (_start >= 52 && _start < 56) || (_end > 52 && _end <= 56) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 56 && _start < 60) || (_end > 56 && _end <= 60) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 60 && _start < 64) || (_end > 60 && _end <= 64) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 64 && _start < 68) || (_end > 64 && _end <= 68) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 68 && _start < 72) || (_end > 68 && _end <= 72) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (92..=103, 93..=104) => {
                if (_start >= 92 && _start < 96) || (_end > 92 && _end <= 96) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 96 && _start < 100) || (_end > 96 && _end <= 100)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 100 && _start < 104)
                    || (_end > 100 && _end <= 104)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (128..=195, 129..=196) => {
                if (_start >= 128 && _start < 132)
                    || (_end > 128 && _end <= 132)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 132 && _start < 136)
                    || (_end > 132 && _end <= 136)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 136 && _start < 140)
                    || (_end > 136 && _end <= 140)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 140 && _start < 144)
                    || (_end > 140 && _end <= 144)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 144 && _start < 148)
                    || (_end > 144 && _end <= 148)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 148 && _start < 152)
                    || (_end > 148 && _end <= 152)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 152 && _start < 156)
                    || (_end > 152 && _end <= 156)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 156 && _start < 160)
                    || (_end > 156 && _end <= 160)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 160 && _start < 164)
                    || (_end > 160 && _end <= 164)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 164 && _start < 168)
                    || (_end > 164 && _end <= 168)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 168 && _start < 172)
                    || (_end > 168 && _end <= 172)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 172 && _start < 176)
                    || (_end > 172 && _end <= 176)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 176 && _start < 180)
                    || (_end > 176 && _end <= 180)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 180 && _start < 184)
                    || (_end > 180 && _end <= 184)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 184 && _start < 188)
                    || (_end > 184 && _end <= 188)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 188 && _start < 192)
                    || (_end > 188 && _end <= 192)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 192 && _start < 196)
                    || (_end > 192 && _end <= 196)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (236..=255, 237..=256) => {
                if (_start >= 236 && _start < 240)
                    || (_end > 236 && _end <= 240)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 240 && _start < 244)
                    || (_end > 240 && _end <= 244)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 244 && _start < 248)
                    || (_end > 244 && _end <= 248)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 248 && _start < 252)
                    || (_end > 248 && _end <= 252)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 252 && _start < 256)
                    || (_end > 252 && _end <= 256)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
}
impl PeripheralPage0x10000000 {
    fn read_ficr_ppfc(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ficr_ppfc_ppfc()? << 0;
        }
        Ok(())
    }
    fn read_ficr_configid(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if _byte_0.is_some() || _byte_1.is_some() {
            self.0
                .lock()
                .unwrap()
                .read_ficr_configid_hwid(_byte_0, _byte_1)?;
        }
        if _byte_2.is_some() || _byte_3.is_some() {
            self.0
                .lock()
                .unwrap()
                .read_ficr_configid_fwid(_byte_2, _byte_3)?;
        }
        Ok(())
    }
    fn read_ficr_deviceaddrtype(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self
                .0
                .lock()
                .unwrap()
                .read_ficr_deviceaddrtype_deviceaddrtype()?
                << 0;
        }
        Ok(())
    }
    fn read_ficr_overrideen(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_ficr_overrideen_nrf_1mbit()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_ficr_overrideen_ble_1mbit()? << 3;
        }
        Ok(())
    }
}
pub struct PeripheralPage0x10001000(
    pub std::sync::Arc<std::sync::Mutex<super::peripheral::Peripherals>>,
);
impl icicle_vm::cpu::mem::IoMemory for PeripheralPage0x10001000 {
    fn read(
        &mut self,
        _addr: u64,
        _buf: &mut [u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 268439552;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=11, 1..=12) => {
                if (_start < 4) || (_end <= 4) {
                    self.0.lock().unwrap().read_uicr_clenr0(
                        &mut buffer_mut(_start, _end, 0, _buf),
                        &mut buffer_mut(_start, _end, 1, _buf),
                        &mut buffer_mut(_start, _end, 2, _buf),
                        &mut buffer_mut(_start, _end, 3, _buf),
                    )?;
                }
                if (_start >= 4 && _start < 8) || (_end > 4 && _end <= 8) {
                    self.read_uicr_rbpconf(
                        &mut buffer_mut(_start, _end, 4, _buf),
                        &mut buffer_mut(_start, _end, 5, _buf),
                        &mut buffer_mut(_start, _end, 6, _buf),
                        &mut buffer_mut(_start, _end, 7, _buf),
                    )?;
                }
                if (_start >= 8 && _start < 12) || (_end > 8 && _end <= 12) {
                    self.read_uicr_xtalfreq(
                        &mut buffer_mut(_start, _end, 8, _buf),
                        &mut buffer_mut(_start, _end, 9, _buf),
                        &mut buffer_mut(_start, _end, 10, _buf),
                        &mut buffer_mut(_start, _end, 11, _buf),
                    )?;
                }
            }
            (16..=255, 17..=256) => {
                if (_start >= 16 && _start < 20) || (_end > 16 && _end <= 20) {
                    self.read_uicr_fwid(
                        &mut buffer_mut(_start, _end, 16, _buf),
                        &mut buffer_mut(_start, _end, 17, _buf),
                        &mut buffer_mut(_start, _end, 18, _buf),
                        &mut buffer_mut(_start, _end, 19, _buf),
                    )?;
                }
                if (_start >= 20 && _start < 24) || (_end > 20 && _end <= 24) {
                    self.0.lock().unwrap().read_uicr_bootloaderaddr(
                        0,
                        &mut buffer_mut(_start, _end, 20, _buf),
                        &mut buffer_mut(_start, _end, 21, _buf),
                        &mut buffer_mut(_start, _end, 22, _buf),
                        &mut buffer_mut(_start, _end, 23, _buf),
                    )?;
                }
                if (_start >= 24 && _start < 28) || (_end > 24 && _end <= 28) {
                    self.0.lock().unwrap().read_uicr_bootloaderaddr(
                        1,
                        &mut buffer_mut(_start, _end, 24, _buf),
                        &mut buffer_mut(_start, _end, 25, _buf),
                        &mut buffer_mut(_start, _end, 26, _buf),
                        &mut buffer_mut(_start, _end, 27, _buf),
                    )?;
                }
                if (_start >= 28 && _start < 32) || (_end > 28 && _end <= 32) {
                    self.0.lock().unwrap().read_uicr_bootloaderaddr(
                        2,
                        &mut buffer_mut(_start, _end, 28, _buf),
                        &mut buffer_mut(_start, _end, 29, _buf),
                        &mut buffer_mut(_start, _end, 30, _buf),
                        &mut buffer_mut(_start, _end, 31, _buf),
                    )?;
                }
                if (_start >= 32 && _start < 36) || (_end > 32 && _end <= 36) {
                    self.0.lock().unwrap().read_uicr_bootloaderaddr(
                        3,
                        &mut buffer_mut(_start, _end, 32, _buf),
                        &mut buffer_mut(_start, _end, 33, _buf),
                        &mut buffer_mut(_start, _end, 34, _buf),
                        &mut buffer_mut(_start, _end, 35, _buf),
                    )?;
                }
                if (_start >= 36 && _start < 40) || (_end > 36 && _end <= 40) {
                    self.0.lock().unwrap().read_uicr_bootloaderaddr(
                        4,
                        &mut buffer_mut(_start, _end, 36, _buf),
                        &mut buffer_mut(_start, _end, 37, _buf),
                        &mut buffer_mut(_start, _end, 38, _buf),
                        &mut buffer_mut(_start, _end, 39, _buf),
                    )?;
                }
                if (_start >= 40 && _start < 44) || (_end > 40 && _end <= 44) {
                    self.0.lock().unwrap().read_uicr_bootloaderaddr(
                        5,
                        &mut buffer_mut(_start, _end, 40, _buf),
                        &mut buffer_mut(_start, _end, 41, _buf),
                        &mut buffer_mut(_start, _end, 42, _buf),
                        &mut buffer_mut(_start, _end, 43, _buf),
                    )?;
                }
                if (_start >= 44 && _start < 48) || (_end > 44 && _end <= 48) {
                    self.0.lock().unwrap().read_uicr_bootloaderaddr(
                        6,
                        &mut buffer_mut(_start, _end, 44, _buf),
                        &mut buffer_mut(_start, _end, 45, _buf),
                        &mut buffer_mut(_start, _end, 46, _buf),
                        &mut buffer_mut(_start, _end, 47, _buf),
                    )?;
                }
                if (_start >= 48 && _start < 52) || (_end > 48 && _end <= 52) {
                    self.0.lock().unwrap().read_uicr_bootloaderaddr(
                        7,
                        &mut buffer_mut(_start, _end, 48, _buf),
                        &mut buffer_mut(_start, _end, 49, _buf),
                        &mut buffer_mut(_start, _end, 50, _buf),
                        &mut buffer_mut(_start, _end, 51, _buf),
                    )?;
                }
                if (_start >= 52 && _start < 56) || (_end > 52 && _end <= 56) {
                    self.0.lock().unwrap().read_uicr_bootloaderaddr(
                        8,
                        &mut buffer_mut(_start, _end, 52, _buf),
                        &mut buffer_mut(_start, _end, 53, _buf),
                        &mut buffer_mut(_start, _end, 54, _buf),
                        &mut buffer_mut(_start, _end, 55, _buf),
                    )?;
                }
                if (_start >= 56 && _start < 60) || (_end > 56 && _end <= 60) {
                    self.0.lock().unwrap().read_uicr_bootloaderaddr(
                        9,
                        &mut buffer_mut(_start, _end, 56, _buf),
                        &mut buffer_mut(_start, _end, 57, _buf),
                        &mut buffer_mut(_start, _end, 58, _buf),
                        &mut buffer_mut(_start, _end, 59, _buf),
                    )?;
                }
                if (_start >= 60 && _start < 64) || (_end > 60 && _end <= 64) {
                    self.0.lock().unwrap().read_uicr_bootloaderaddr(
                        10,
                        &mut buffer_mut(_start, _end, 60, _buf),
                        &mut buffer_mut(_start, _end, 61, _buf),
                        &mut buffer_mut(_start, _end, 62, _buf),
                        &mut buffer_mut(_start, _end, 63, _buf),
                    )?;
                }
                if (_start >= 64 && _start < 68) || (_end > 64 && _end <= 68) {
                    self.0.lock().unwrap().read_uicr_bootloaderaddr(
                        11,
                        &mut buffer_mut(_start, _end, 64, _buf),
                        &mut buffer_mut(_start, _end, 65, _buf),
                        &mut buffer_mut(_start, _end, 66, _buf),
                        &mut buffer_mut(_start, _end, 67, _buf),
                    )?;
                }
                if (_start >= 68 && _start < 72) || (_end > 68 && _end <= 72) {
                    self.0.lock().unwrap().read_uicr_bootloaderaddr(
                        12,
                        &mut buffer_mut(_start, _end, 68, _buf),
                        &mut buffer_mut(_start, _end, 69, _buf),
                        &mut buffer_mut(_start, _end, 70, _buf),
                        &mut buffer_mut(_start, _end, 71, _buf),
                    )?;
                }
                if (_start >= 72 && _start < 76) || (_end > 72 && _end <= 76) {
                    self.0.lock().unwrap().read_uicr_bootloaderaddr(
                        13,
                        &mut buffer_mut(_start, _end, 72, _buf),
                        &mut buffer_mut(_start, _end, 73, _buf),
                        &mut buffer_mut(_start, _end, 74, _buf),
                        &mut buffer_mut(_start, _end, 75, _buf),
                    )?;
                }
                if (_start >= 76 && _start < 80) || (_end > 76 && _end <= 80) {
                    self.0.lock().unwrap().read_uicr_bootloaderaddr(
                        14,
                        &mut buffer_mut(_start, _end, 76, _buf),
                        &mut buffer_mut(_start, _end, 77, _buf),
                        &mut buffer_mut(_start, _end, 78, _buf),
                        &mut buffer_mut(_start, _end, 79, _buf),
                    )?;
                }
                if (_start >= 80 && _start < 84) || (_end > 80 && _end <= 84) {
                    self.0.lock().unwrap().read_uicr_nrfhwn(
                        0,
                        &mut buffer_mut(_start, _end, 80, _buf),
                        &mut buffer_mut(_start, _end, 81, _buf),
                        &mut buffer_mut(_start, _end, 82, _buf),
                        &mut buffer_mut(_start, _end, 83, _buf),
                    )?;
                }
                if (_start >= 84 && _start < 88) || (_end > 84 && _end <= 88) {
                    self.0.lock().unwrap().read_uicr_nrfhwn(
                        1,
                        &mut buffer_mut(_start, _end, 84, _buf),
                        &mut buffer_mut(_start, _end, 85, _buf),
                        &mut buffer_mut(_start, _end, 86, _buf),
                        &mut buffer_mut(_start, _end, 87, _buf),
                    )?;
                }
                if (_start >= 88 && _start < 92) || (_end > 88 && _end <= 92) {
                    self.0.lock().unwrap().read_uicr_nrfhwn(
                        2,
                        &mut buffer_mut(_start, _end, 88, _buf),
                        &mut buffer_mut(_start, _end, 89, _buf),
                        &mut buffer_mut(_start, _end, 90, _buf),
                        &mut buffer_mut(_start, _end, 91, _buf),
                    )?;
                }
                if (_start >= 92 && _start < 96) || (_end > 92 && _end <= 96) {
                    self.0.lock().unwrap().read_uicr_nrfhwn(
                        3,
                        &mut buffer_mut(_start, _end, 92, _buf),
                        &mut buffer_mut(_start, _end, 93, _buf),
                        &mut buffer_mut(_start, _end, 94, _buf),
                        &mut buffer_mut(_start, _end, 95, _buf),
                    )?;
                }
                if (_start >= 96 && _start < 100) || (_end > 96 && _end <= 100)
                {
                    self.0.lock().unwrap().read_uicr_nrfhwn(
                        4,
                        &mut buffer_mut(_start, _end, 96, _buf),
                        &mut buffer_mut(_start, _end, 97, _buf),
                        &mut buffer_mut(_start, _end, 98, _buf),
                        &mut buffer_mut(_start, _end, 99, _buf),
                    )?;
                }
                if (_start >= 100 && _start < 104)
                    || (_end > 100 && _end <= 104)
                {
                    self.0.lock().unwrap().read_uicr_nrfhwn(
                        5,
                        &mut buffer_mut(_start, _end, 100, _buf),
                        &mut buffer_mut(_start, _end, 101, _buf),
                        &mut buffer_mut(_start, _end, 102, _buf),
                        &mut buffer_mut(_start, _end, 103, _buf),
                    )?;
                }
                if (_start >= 104 && _start < 108)
                    || (_end > 104 && _end <= 108)
                {
                    self.0.lock().unwrap().read_uicr_nrfhwn(
                        6,
                        &mut buffer_mut(_start, _end, 104, _buf),
                        &mut buffer_mut(_start, _end, 105, _buf),
                        &mut buffer_mut(_start, _end, 106, _buf),
                        &mut buffer_mut(_start, _end, 107, _buf),
                    )?;
                }
                if (_start >= 108 && _start < 112)
                    || (_end > 108 && _end <= 112)
                {
                    self.0.lock().unwrap().read_uicr_nrfhwn(
                        7,
                        &mut buffer_mut(_start, _end, 108, _buf),
                        &mut buffer_mut(_start, _end, 109, _buf),
                        &mut buffer_mut(_start, _end, 110, _buf),
                        &mut buffer_mut(_start, _end, 111, _buf),
                    )?;
                }
                if (_start >= 112 && _start < 116)
                    || (_end > 112 && _end <= 116)
                {
                    self.0.lock().unwrap().read_uicr_nrfhwn(
                        8,
                        &mut buffer_mut(_start, _end, 112, _buf),
                        &mut buffer_mut(_start, _end, 113, _buf),
                        &mut buffer_mut(_start, _end, 114, _buf),
                        &mut buffer_mut(_start, _end, 115, _buf),
                    )?;
                }
                if (_start >= 116 && _start < 120)
                    || (_end > 116 && _end <= 120)
                {
                    self.0.lock().unwrap().read_uicr_nrfhwn(
                        9,
                        &mut buffer_mut(_start, _end, 116, _buf),
                        &mut buffer_mut(_start, _end, 117, _buf),
                        &mut buffer_mut(_start, _end, 118, _buf),
                        &mut buffer_mut(_start, _end, 119, _buf),
                    )?;
                }
                if (_start >= 120 && _start < 124)
                    || (_end > 120 && _end <= 124)
                {
                    self.0.lock().unwrap().read_uicr_nrfhwn(
                        10,
                        &mut buffer_mut(_start, _end, 120, _buf),
                        &mut buffer_mut(_start, _end, 121, _buf),
                        &mut buffer_mut(_start, _end, 122, _buf),
                        &mut buffer_mut(_start, _end, 123, _buf),
                    )?;
                }
                if (_start >= 124 && _start < 128)
                    || (_end > 124 && _end <= 128)
                {
                    self.0.lock().unwrap().read_uicr_nrfhwn(
                        11,
                        &mut buffer_mut(_start, _end, 124, _buf),
                        &mut buffer_mut(_start, _end, 125, _buf),
                        &mut buffer_mut(_start, _end, 126, _buf),
                        &mut buffer_mut(_start, _end, 127, _buf),
                    )?;
                }
                if (_start >= 128 && _start < 132)
                    || (_end > 128 && _end <= 132)
                {
                    self.0.lock().unwrap().read_uicr_customern(
                        0,
                        &mut buffer_mut(_start, _end, 128, _buf),
                        &mut buffer_mut(_start, _end, 129, _buf),
                        &mut buffer_mut(_start, _end, 130, _buf),
                        &mut buffer_mut(_start, _end, 131, _buf),
                    )?;
                }
                if (_start >= 132 && _start < 136)
                    || (_end > 132 && _end <= 136)
                {
                    self.0.lock().unwrap().read_uicr_customern(
                        1,
                        &mut buffer_mut(_start, _end, 132, _buf),
                        &mut buffer_mut(_start, _end, 133, _buf),
                        &mut buffer_mut(_start, _end, 134, _buf),
                        &mut buffer_mut(_start, _end, 135, _buf),
                    )?;
                }
                if (_start >= 136 && _start < 140)
                    || (_end > 136 && _end <= 140)
                {
                    self.0.lock().unwrap().read_uicr_customern(
                        2,
                        &mut buffer_mut(_start, _end, 136, _buf),
                        &mut buffer_mut(_start, _end, 137, _buf),
                        &mut buffer_mut(_start, _end, 138, _buf),
                        &mut buffer_mut(_start, _end, 139, _buf),
                    )?;
                }
                if (_start >= 140 && _start < 144)
                    || (_end > 140 && _end <= 144)
                {
                    self.0.lock().unwrap().read_uicr_customern(
                        3,
                        &mut buffer_mut(_start, _end, 140, _buf),
                        &mut buffer_mut(_start, _end, 141, _buf),
                        &mut buffer_mut(_start, _end, 142, _buf),
                        &mut buffer_mut(_start, _end, 143, _buf),
                    )?;
                }
                if (_start >= 144 && _start < 148)
                    || (_end > 144 && _end <= 148)
                {
                    self.0.lock().unwrap().read_uicr_customern(
                        4,
                        &mut buffer_mut(_start, _end, 144, _buf),
                        &mut buffer_mut(_start, _end, 145, _buf),
                        &mut buffer_mut(_start, _end, 146, _buf),
                        &mut buffer_mut(_start, _end, 147, _buf),
                    )?;
                }
                if (_start >= 148 && _start < 152)
                    || (_end > 148 && _end <= 152)
                {
                    self.0.lock().unwrap().read_uicr_customern(
                        5,
                        &mut buffer_mut(_start, _end, 148, _buf),
                        &mut buffer_mut(_start, _end, 149, _buf),
                        &mut buffer_mut(_start, _end, 150, _buf),
                        &mut buffer_mut(_start, _end, 151, _buf),
                    )?;
                }
                if (_start >= 152 && _start < 156)
                    || (_end > 152 && _end <= 156)
                {
                    self.0.lock().unwrap().read_uicr_customern(
                        6,
                        &mut buffer_mut(_start, _end, 152, _buf),
                        &mut buffer_mut(_start, _end, 153, _buf),
                        &mut buffer_mut(_start, _end, 154, _buf),
                        &mut buffer_mut(_start, _end, 155, _buf),
                    )?;
                }
                if (_start >= 156 && _start < 160)
                    || (_end > 156 && _end <= 160)
                {
                    self.0.lock().unwrap().read_uicr_customern(
                        7,
                        &mut buffer_mut(_start, _end, 156, _buf),
                        &mut buffer_mut(_start, _end, 157, _buf),
                        &mut buffer_mut(_start, _end, 158, _buf),
                        &mut buffer_mut(_start, _end, 159, _buf),
                    )?;
                }
                if (_start >= 160 && _start < 164)
                    || (_end > 160 && _end <= 164)
                {
                    self.0.lock().unwrap().read_uicr_customern(
                        8,
                        &mut buffer_mut(_start, _end, 160, _buf),
                        &mut buffer_mut(_start, _end, 161, _buf),
                        &mut buffer_mut(_start, _end, 162, _buf),
                        &mut buffer_mut(_start, _end, 163, _buf),
                    )?;
                }
                if (_start >= 164 && _start < 168)
                    || (_end > 164 && _end <= 168)
                {
                    self.0.lock().unwrap().read_uicr_customern(
                        9,
                        &mut buffer_mut(_start, _end, 164, _buf),
                        &mut buffer_mut(_start, _end, 165, _buf),
                        &mut buffer_mut(_start, _end, 166, _buf),
                        &mut buffer_mut(_start, _end, 167, _buf),
                    )?;
                }
                if (_start >= 168 && _start < 172)
                    || (_end > 168 && _end <= 172)
                {
                    self.0.lock().unwrap().read_uicr_customern(
                        10,
                        &mut buffer_mut(_start, _end, 168, _buf),
                        &mut buffer_mut(_start, _end, 169, _buf),
                        &mut buffer_mut(_start, _end, 170, _buf),
                        &mut buffer_mut(_start, _end, 171, _buf),
                    )?;
                }
                if (_start >= 172 && _start < 176)
                    || (_end > 172 && _end <= 176)
                {
                    self.0.lock().unwrap().read_uicr_customern(
                        11,
                        &mut buffer_mut(_start, _end, 172, _buf),
                        &mut buffer_mut(_start, _end, 173, _buf),
                        &mut buffer_mut(_start, _end, 174, _buf),
                        &mut buffer_mut(_start, _end, 175, _buf),
                    )?;
                }
                if (_start >= 176 && _start < 180)
                    || (_end > 176 && _end <= 180)
                {
                    self.0.lock().unwrap().read_uicr_customern(
                        12,
                        &mut buffer_mut(_start, _end, 176, _buf),
                        &mut buffer_mut(_start, _end, 177, _buf),
                        &mut buffer_mut(_start, _end, 178, _buf),
                        &mut buffer_mut(_start, _end, 179, _buf),
                    )?;
                }
                if (_start >= 180 && _start < 184)
                    || (_end > 180 && _end <= 184)
                {
                    self.0.lock().unwrap().read_uicr_customern(
                        13,
                        &mut buffer_mut(_start, _end, 180, _buf),
                        &mut buffer_mut(_start, _end, 181, _buf),
                        &mut buffer_mut(_start, _end, 182, _buf),
                        &mut buffer_mut(_start, _end, 183, _buf),
                    )?;
                }
                if (_start >= 184 && _start < 188)
                    || (_end > 184 && _end <= 188)
                {
                    self.0.lock().unwrap().read_uicr_customern(
                        14,
                        &mut buffer_mut(_start, _end, 184, _buf),
                        &mut buffer_mut(_start, _end, 185, _buf),
                        &mut buffer_mut(_start, _end, 186, _buf),
                        &mut buffer_mut(_start, _end, 187, _buf),
                    )?;
                }
                if (_start >= 188 && _start < 192)
                    || (_end > 188 && _end <= 192)
                {
                    self.0.lock().unwrap().read_uicr_customern(
                        15,
                        &mut buffer_mut(_start, _end, 188, _buf),
                        &mut buffer_mut(_start, _end, 189, _buf),
                        &mut buffer_mut(_start, _end, 190, _buf),
                        &mut buffer_mut(_start, _end, 191, _buf),
                    )?;
                }
                if (_start >= 192 && _start < 196)
                    || (_end > 192 && _end <= 196)
                {
                    self.0.lock().unwrap().read_uicr_customern(
                        16,
                        &mut buffer_mut(_start, _end, 192, _buf),
                        &mut buffer_mut(_start, _end, 193, _buf),
                        &mut buffer_mut(_start, _end, 194, _buf),
                        &mut buffer_mut(_start, _end, 195, _buf),
                    )?;
                }
                if (_start >= 196 && _start < 200)
                    || (_end > 196 && _end <= 200)
                {
                    self.0.lock().unwrap().read_uicr_customern(
                        17,
                        &mut buffer_mut(_start, _end, 196, _buf),
                        &mut buffer_mut(_start, _end, 197, _buf),
                        &mut buffer_mut(_start, _end, 198, _buf),
                        &mut buffer_mut(_start, _end, 199, _buf),
                    )?;
                }
                if (_start >= 200 && _start < 204)
                    || (_end > 200 && _end <= 204)
                {
                    self.0.lock().unwrap().read_uicr_customern(
                        18,
                        &mut buffer_mut(_start, _end, 200, _buf),
                        &mut buffer_mut(_start, _end, 201, _buf),
                        &mut buffer_mut(_start, _end, 202, _buf),
                        &mut buffer_mut(_start, _end, 203, _buf),
                    )?;
                }
                if (_start >= 204 && _start < 208)
                    || (_end > 204 && _end <= 208)
                {
                    self.0.lock().unwrap().read_uicr_customern(
                        19,
                        &mut buffer_mut(_start, _end, 204, _buf),
                        &mut buffer_mut(_start, _end, 205, _buf),
                        &mut buffer_mut(_start, _end, 206, _buf),
                        &mut buffer_mut(_start, _end, 207, _buf),
                    )?;
                }
                if (_start >= 208 && _start < 212)
                    || (_end > 208 && _end <= 212)
                {
                    self.0.lock().unwrap().read_uicr_customern(
                        20,
                        &mut buffer_mut(_start, _end, 208, _buf),
                        &mut buffer_mut(_start, _end, 209, _buf),
                        &mut buffer_mut(_start, _end, 210, _buf),
                        &mut buffer_mut(_start, _end, 211, _buf),
                    )?;
                }
                if (_start >= 212 && _start < 216)
                    || (_end > 212 && _end <= 216)
                {
                    self.0.lock().unwrap().read_uicr_customern(
                        21,
                        &mut buffer_mut(_start, _end, 212, _buf),
                        &mut buffer_mut(_start, _end, 213, _buf),
                        &mut buffer_mut(_start, _end, 214, _buf),
                        &mut buffer_mut(_start, _end, 215, _buf),
                    )?;
                }
                if (_start >= 216 && _start < 220)
                    || (_end > 216 && _end <= 220)
                {
                    self.0.lock().unwrap().read_uicr_customern(
                        22,
                        &mut buffer_mut(_start, _end, 216, _buf),
                        &mut buffer_mut(_start, _end, 217, _buf),
                        &mut buffer_mut(_start, _end, 218, _buf),
                        &mut buffer_mut(_start, _end, 219, _buf),
                    )?;
                }
                if (_start >= 220 && _start < 224)
                    || (_end > 220 && _end <= 224)
                {
                    self.0.lock().unwrap().read_uicr_customern(
                        23,
                        &mut buffer_mut(_start, _end, 220, _buf),
                        &mut buffer_mut(_start, _end, 221, _buf),
                        &mut buffer_mut(_start, _end, 222, _buf),
                        &mut buffer_mut(_start, _end, 223, _buf),
                    )?;
                }
                if (_start >= 224 && _start < 228)
                    || (_end > 224 && _end <= 228)
                {
                    self.0.lock().unwrap().read_uicr_customern(
                        24,
                        &mut buffer_mut(_start, _end, 224, _buf),
                        &mut buffer_mut(_start, _end, 225, _buf),
                        &mut buffer_mut(_start, _end, 226, _buf),
                        &mut buffer_mut(_start, _end, 227, _buf),
                    )?;
                }
                if (_start >= 228 && _start < 232)
                    || (_end > 228 && _end <= 232)
                {
                    self.0.lock().unwrap().read_uicr_customern(
                        25,
                        &mut buffer_mut(_start, _end, 228, _buf),
                        &mut buffer_mut(_start, _end, 229, _buf),
                        &mut buffer_mut(_start, _end, 230, _buf),
                        &mut buffer_mut(_start, _end, 231, _buf),
                    )?;
                }
                if (_start >= 232 && _start < 236)
                    || (_end > 232 && _end <= 236)
                {
                    self.0.lock().unwrap().read_uicr_customern(
                        26,
                        &mut buffer_mut(_start, _end, 232, _buf),
                        &mut buffer_mut(_start, _end, 233, _buf),
                        &mut buffer_mut(_start, _end, 234, _buf),
                        &mut buffer_mut(_start, _end, 235, _buf),
                    )?;
                }
                if (_start >= 236 && _start < 240)
                    || (_end > 236 && _end <= 240)
                {
                    self.0.lock().unwrap().read_uicr_customern(
                        27,
                        &mut buffer_mut(_start, _end, 236, _buf),
                        &mut buffer_mut(_start, _end, 237, _buf),
                        &mut buffer_mut(_start, _end, 238, _buf),
                        &mut buffer_mut(_start, _end, 239, _buf),
                    )?;
                }
                if (_start >= 240 && _start < 244)
                    || (_end > 240 && _end <= 244)
                {
                    self.0.lock().unwrap().read_uicr_customern(
                        28,
                        &mut buffer_mut(_start, _end, 240, _buf),
                        &mut buffer_mut(_start, _end, 241, _buf),
                        &mut buffer_mut(_start, _end, 242, _buf),
                        &mut buffer_mut(_start, _end, 243, _buf),
                    )?;
                }
                if (_start >= 244 && _start < 248)
                    || (_end > 244 && _end <= 248)
                {
                    self.0.lock().unwrap().read_uicr_customern(
                        29,
                        &mut buffer_mut(_start, _end, 244, _buf),
                        &mut buffer_mut(_start, _end, 245, _buf),
                        &mut buffer_mut(_start, _end, 246, _buf),
                        &mut buffer_mut(_start, _end, 247, _buf),
                    )?;
                }
                if (_start >= 248 && _start < 252)
                    || (_end > 248 && _end <= 252)
                {
                    self.0.lock().unwrap().read_uicr_customern(
                        30,
                        &mut buffer_mut(_start, _end, 248, _buf),
                        &mut buffer_mut(_start, _end, 249, _buf),
                        &mut buffer_mut(_start, _end, 250, _buf),
                        &mut buffer_mut(_start, _end, 251, _buf),
                    )?;
                }
                if (_start >= 252 && _start < 256)
                    || (_end > 252 && _end <= 256)
                {
                    self.0.lock().unwrap().read_uicr_customern(
                        31,
                        &mut buffer_mut(_start, _end, 252, _buf),
                        &mut buffer_mut(_start, _end, 253, _buf),
                        &mut buffer_mut(_start, _end, 254, _buf),
                        &mut buffer_mut(_start, _end, 255, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
    fn write(
        &mut self,
        _addr: u64,
        _buf: &[u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 268439552;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=11, 1..=12) => {
                if (_start < 4) || (_end <= 4) {
                    self.0.lock().unwrap().write_uicr_clenr0(
                        buffer_const(_start, _end, 0, _buf),
                        buffer_const(_start, _end, 1, _buf),
                        buffer_const(_start, _end, 2, _buf),
                        buffer_const(_start, _end, 3, _buf),
                    )?;
                }
                if (_start >= 4 && _start < 8) || (_end > 4 && _end <= 8) {
                    self.write_uicr_rbpconf(
                        buffer_const(_start, _end, 4, _buf),
                        buffer_const(_start, _end, 5, _buf),
                        buffer_const(_start, _end, 6, _buf),
                        buffer_const(_start, _end, 7, _buf),
                    )?;
                }
                if (_start >= 8 && _start < 12) || (_end > 8 && _end <= 12) {
                    self.write_uicr_xtalfreq(
                        buffer_const(_start, _end, 8, _buf),
                        buffer_const(_start, _end, 9, _buf),
                        buffer_const(_start, _end, 10, _buf),
                        buffer_const(_start, _end, 11, _buf),
                    )?;
                }
            }
            (16..=255, 17..=256) => {
                if (_start >= 16 && _start < 20) || (_end > 16 && _end <= 20) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 20 && _start < 24) || (_end > 20 && _end <= 24) {
                    self.0.lock().unwrap().write_uicr_bootloaderaddr(
                        0,
                        buffer_const(_start, _end, 20, _buf),
                        buffer_const(_start, _end, 21, _buf),
                        buffer_const(_start, _end, 22, _buf),
                        buffer_const(_start, _end, 23, _buf),
                    )?;
                }
                if (_start >= 24 && _start < 28) || (_end > 24 && _end <= 28) {
                    self.0.lock().unwrap().write_uicr_bootloaderaddr(
                        1,
                        buffer_const(_start, _end, 24, _buf),
                        buffer_const(_start, _end, 25, _buf),
                        buffer_const(_start, _end, 26, _buf),
                        buffer_const(_start, _end, 27, _buf),
                    )?;
                }
                if (_start >= 28 && _start < 32) || (_end > 28 && _end <= 32) {
                    self.0.lock().unwrap().write_uicr_bootloaderaddr(
                        2,
                        buffer_const(_start, _end, 28, _buf),
                        buffer_const(_start, _end, 29, _buf),
                        buffer_const(_start, _end, 30, _buf),
                        buffer_const(_start, _end, 31, _buf),
                    )?;
                }
                if (_start >= 32 && _start < 36) || (_end > 32 && _end <= 36) {
                    self.0.lock().unwrap().write_uicr_bootloaderaddr(
                        3,
                        buffer_const(_start, _end, 32, _buf),
                        buffer_const(_start, _end, 33, _buf),
                        buffer_const(_start, _end, 34, _buf),
                        buffer_const(_start, _end, 35, _buf),
                    )?;
                }
                if (_start >= 36 && _start < 40) || (_end > 36 && _end <= 40) {
                    self.0.lock().unwrap().write_uicr_bootloaderaddr(
                        4,
                        buffer_const(_start, _end, 36, _buf),
                        buffer_const(_start, _end, 37, _buf),
                        buffer_const(_start, _end, 38, _buf),
                        buffer_const(_start, _end, 39, _buf),
                    )?;
                }
                if (_start >= 40 && _start < 44) || (_end > 40 && _end <= 44) {
                    self.0.lock().unwrap().write_uicr_bootloaderaddr(
                        5,
                        buffer_const(_start, _end, 40, _buf),
                        buffer_const(_start, _end, 41, _buf),
                        buffer_const(_start, _end, 42, _buf),
                        buffer_const(_start, _end, 43, _buf),
                    )?;
                }
                if (_start >= 44 && _start < 48) || (_end > 44 && _end <= 48) {
                    self.0.lock().unwrap().write_uicr_bootloaderaddr(
                        6,
                        buffer_const(_start, _end, 44, _buf),
                        buffer_const(_start, _end, 45, _buf),
                        buffer_const(_start, _end, 46, _buf),
                        buffer_const(_start, _end, 47, _buf),
                    )?;
                }
                if (_start >= 48 && _start < 52) || (_end > 48 && _end <= 52) {
                    self.0.lock().unwrap().write_uicr_bootloaderaddr(
                        7,
                        buffer_const(_start, _end, 48, _buf),
                        buffer_const(_start, _end, 49, _buf),
                        buffer_const(_start, _end, 50, _buf),
                        buffer_const(_start, _end, 51, _buf),
                    )?;
                }
                if (_start >= 52 && _start < 56) || (_end > 52 && _end <= 56) {
                    self.0.lock().unwrap().write_uicr_bootloaderaddr(
                        8,
                        buffer_const(_start, _end, 52, _buf),
                        buffer_const(_start, _end, 53, _buf),
                        buffer_const(_start, _end, 54, _buf),
                        buffer_const(_start, _end, 55, _buf),
                    )?;
                }
                if (_start >= 56 && _start < 60) || (_end > 56 && _end <= 60) {
                    self.0.lock().unwrap().write_uicr_bootloaderaddr(
                        9,
                        buffer_const(_start, _end, 56, _buf),
                        buffer_const(_start, _end, 57, _buf),
                        buffer_const(_start, _end, 58, _buf),
                        buffer_const(_start, _end, 59, _buf),
                    )?;
                }
                if (_start >= 60 && _start < 64) || (_end > 60 && _end <= 64) {
                    self.0.lock().unwrap().write_uicr_bootloaderaddr(
                        10,
                        buffer_const(_start, _end, 60, _buf),
                        buffer_const(_start, _end, 61, _buf),
                        buffer_const(_start, _end, 62, _buf),
                        buffer_const(_start, _end, 63, _buf),
                    )?;
                }
                if (_start >= 64 && _start < 68) || (_end > 64 && _end <= 68) {
                    self.0.lock().unwrap().write_uicr_bootloaderaddr(
                        11,
                        buffer_const(_start, _end, 64, _buf),
                        buffer_const(_start, _end, 65, _buf),
                        buffer_const(_start, _end, 66, _buf),
                        buffer_const(_start, _end, 67, _buf),
                    )?;
                }
                if (_start >= 68 && _start < 72) || (_end > 68 && _end <= 72) {
                    self.0.lock().unwrap().write_uicr_bootloaderaddr(
                        12,
                        buffer_const(_start, _end, 68, _buf),
                        buffer_const(_start, _end, 69, _buf),
                        buffer_const(_start, _end, 70, _buf),
                        buffer_const(_start, _end, 71, _buf),
                    )?;
                }
                if (_start >= 72 && _start < 76) || (_end > 72 && _end <= 76) {
                    self.0.lock().unwrap().write_uicr_bootloaderaddr(
                        13,
                        buffer_const(_start, _end, 72, _buf),
                        buffer_const(_start, _end, 73, _buf),
                        buffer_const(_start, _end, 74, _buf),
                        buffer_const(_start, _end, 75, _buf),
                    )?;
                }
                if (_start >= 76 && _start < 80) || (_end > 76 && _end <= 80) {
                    self.0.lock().unwrap().write_uicr_bootloaderaddr(
                        14,
                        buffer_const(_start, _end, 76, _buf),
                        buffer_const(_start, _end, 77, _buf),
                        buffer_const(_start, _end, 78, _buf),
                        buffer_const(_start, _end, 79, _buf),
                    )?;
                }
                if (_start >= 80 && _start < 84) || (_end > 80 && _end <= 84) {
                    self.0.lock().unwrap().write_uicr_nrfhwn(
                        0,
                        buffer_const(_start, _end, 80, _buf),
                        buffer_const(_start, _end, 81, _buf),
                        buffer_const(_start, _end, 82, _buf),
                        buffer_const(_start, _end, 83, _buf),
                    )?;
                }
                if (_start >= 84 && _start < 88) || (_end > 84 && _end <= 88) {
                    self.0.lock().unwrap().write_uicr_nrfhwn(
                        1,
                        buffer_const(_start, _end, 84, _buf),
                        buffer_const(_start, _end, 85, _buf),
                        buffer_const(_start, _end, 86, _buf),
                        buffer_const(_start, _end, 87, _buf),
                    )?;
                }
                if (_start >= 88 && _start < 92) || (_end > 88 && _end <= 92) {
                    self.0.lock().unwrap().write_uicr_nrfhwn(
                        2,
                        buffer_const(_start, _end, 88, _buf),
                        buffer_const(_start, _end, 89, _buf),
                        buffer_const(_start, _end, 90, _buf),
                        buffer_const(_start, _end, 91, _buf),
                    )?;
                }
                if (_start >= 92 && _start < 96) || (_end > 92 && _end <= 96) {
                    self.0.lock().unwrap().write_uicr_nrfhwn(
                        3,
                        buffer_const(_start, _end, 92, _buf),
                        buffer_const(_start, _end, 93, _buf),
                        buffer_const(_start, _end, 94, _buf),
                        buffer_const(_start, _end, 95, _buf),
                    )?;
                }
                if (_start >= 96 && _start < 100) || (_end > 96 && _end <= 100)
                {
                    self.0.lock().unwrap().write_uicr_nrfhwn(
                        4,
                        buffer_const(_start, _end, 96, _buf),
                        buffer_const(_start, _end, 97, _buf),
                        buffer_const(_start, _end, 98, _buf),
                        buffer_const(_start, _end, 99, _buf),
                    )?;
                }
                if (_start >= 100 && _start < 104)
                    || (_end > 100 && _end <= 104)
                {
                    self.0.lock().unwrap().write_uicr_nrfhwn(
                        5,
                        buffer_const(_start, _end, 100, _buf),
                        buffer_const(_start, _end, 101, _buf),
                        buffer_const(_start, _end, 102, _buf),
                        buffer_const(_start, _end, 103, _buf),
                    )?;
                }
                if (_start >= 104 && _start < 108)
                    || (_end > 104 && _end <= 108)
                {
                    self.0.lock().unwrap().write_uicr_nrfhwn(
                        6,
                        buffer_const(_start, _end, 104, _buf),
                        buffer_const(_start, _end, 105, _buf),
                        buffer_const(_start, _end, 106, _buf),
                        buffer_const(_start, _end, 107, _buf),
                    )?;
                }
                if (_start >= 108 && _start < 112)
                    || (_end > 108 && _end <= 112)
                {
                    self.0.lock().unwrap().write_uicr_nrfhwn(
                        7,
                        buffer_const(_start, _end, 108, _buf),
                        buffer_const(_start, _end, 109, _buf),
                        buffer_const(_start, _end, 110, _buf),
                        buffer_const(_start, _end, 111, _buf),
                    )?;
                }
                if (_start >= 112 && _start < 116)
                    || (_end > 112 && _end <= 116)
                {
                    self.0.lock().unwrap().write_uicr_nrfhwn(
                        8,
                        buffer_const(_start, _end, 112, _buf),
                        buffer_const(_start, _end, 113, _buf),
                        buffer_const(_start, _end, 114, _buf),
                        buffer_const(_start, _end, 115, _buf),
                    )?;
                }
                if (_start >= 116 && _start < 120)
                    || (_end > 116 && _end <= 120)
                {
                    self.0.lock().unwrap().write_uicr_nrfhwn(
                        9,
                        buffer_const(_start, _end, 116, _buf),
                        buffer_const(_start, _end, 117, _buf),
                        buffer_const(_start, _end, 118, _buf),
                        buffer_const(_start, _end, 119, _buf),
                    )?;
                }
                if (_start >= 120 && _start < 124)
                    || (_end > 120 && _end <= 124)
                {
                    self.0.lock().unwrap().write_uicr_nrfhwn(
                        10,
                        buffer_const(_start, _end, 120, _buf),
                        buffer_const(_start, _end, 121, _buf),
                        buffer_const(_start, _end, 122, _buf),
                        buffer_const(_start, _end, 123, _buf),
                    )?;
                }
                if (_start >= 124 && _start < 128)
                    || (_end > 124 && _end <= 128)
                {
                    self.0.lock().unwrap().write_uicr_nrfhwn(
                        11,
                        buffer_const(_start, _end, 124, _buf),
                        buffer_const(_start, _end, 125, _buf),
                        buffer_const(_start, _end, 126, _buf),
                        buffer_const(_start, _end, 127, _buf),
                    )?;
                }
                if (_start >= 128 && _start < 132)
                    || (_end > 128 && _end <= 132)
                {
                    self.0.lock().unwrap().write_uicr_customern(
                        0,
                        buffer_const(_start, _end, 128, _buf),
                        buffer_const(_start, _end, 129, _buf),
                        buffer_const(_start, _end, 130, _buf),
                        buffer_const(_start, _end, 131, _buf),
                    )?;
                }
                if (_start >= 132 && _start < 136)
                    || (_end > 132 && _end <= 136)
                {
                    self.0.lock().unwrap().write_uicr_customern(
                        1,
                        buffer_const(_start, _end, 132, _buf),
                        buffer_const(_start, _end, 133, _buf),
                        buffer_const(_start, _end, 134, _buf),
                        buffer_const(_start, _end, 135, _buf),
                    )?;
                }
                if (_start >= 136 && _start < 140)
                    || (_end > 136 && _end <= 140)
                {
                    self.0.lock().unwrap().write_uicr_customern(
                        2,
                        buffer_const(_start, _end, 136, _buf),
                        buffer_const(_start, _end, 137, _buf),
                        buffer_const(_start, _end, 138, _buf),
                        buffer_const(_start, _end, 139, _buf),
                    )?;
                }
                if (_start >= 140 && _start < 144)
                    || (_end > 140 && _end <= 144)
                {
                    self.0.lock().unwrap().write_uicr_customern(
                        3,
                        buffer_const(_start, _end, 140, _buf),
                        buffer_const(_start, _end, 141, _buf),
                        buffer_const(_start, _end, 142, _buf),
                        buffer_const(_start, _end, 143, _buf),
                    )?;
                }
                if (_start >= 144 && _start < 148)
                    || (_end > 144 && _end <= 148)
                {
                    self.0.lock().unwrap().write_uicr_customern(
                        4,
                        buffer_const(_start, _end, 144, _buf),
                        buffer_const(_start, _end, 145, _buf),
                        buffer_const(_start, _end, 146, _buf),
                        buffer_const(_start, _end, 147, _buf),
                    )?;
                }
                if (_start >= 148 && _start < 152)
                    || (_end > 148 && _end <= 152)
                {
                    self.0.lock().unwrap().write_uicr_customern(
                        5,
                        buffer_const(_start, _end, 148, _buf),
                        buffer_const(_start, _end, 149, _buf),
                        buffer_const(_start, _end, 150, _buf),
                        buffer_const(_start, _end, 151, _buf),
                    )?;
                }
                if (_start >= 152 && _start < 156)
                    || (_end > 152 && _end <= 156)
                {
                    self.0.lock().unwrap().write_uicr_customern(
                        6,
                        buffer_const(_start, _end, 152, _buf),
                        buffer_const(_start, _end, 153, _buf),
                        buffer_const(_start, _end, 154, _buf),
                        buffer_const(_start, _end, 155, _buf),
                    )?;
                }
                if (_start >= 156 && _start < 160)
                    || (_end > 156 && _end <= 160)
                {
                    self.0.lock().unwrap().write_uicr_customern(
                        7,
                        buffer_const(_start, _end, 156, _buf),
                        buffer_const(_start, _end, 157, _buf),
                        buffer_const(_start, _end, 158, _buf),
                        buffer_const(_start, _end, 159, _buf),
                    )?;
                }
                if (_start >= 160 && _start < 164)
                    || (_end > 160 && _end <= 164)
                {
                    self.0.lock().unwrap().write_uicr_customern(
                        8,
                        buffer_const(_start, _end, 160, _buf),
                        buffer_const(_start, _end, 161, _buf),
                        buffer_const(_start, _end, 162, _buf),
                        buffer_const(_start, _end, 163, _buf),
                    )?;
                }
                if (_start >= 164 && _start < 168)
                    || (_end > 164 && _end <= 168)
                {
                    self.0.lock().unwrap().write_uicr_customern(
                        9,
                        buffer_const(_start, _end, 164, _buf),
                        buffer_const(_start, _end, 165, _buf),
                        buffer_const(_start, _end, 166, _buf),
                        buffer_const(_start, _end, 167, _buf),
                    )?;
                }
                if (_start >= 168 && _start < 172)
                    || (_end > 168 && _end <= 172)
                {
                    self.0.lock().unwrap().write_uicr_customern(
                        10,
                        buffer_const(_start, _end, 168, _buf),
                        buffer_const(_start, _end, 169, _buf),
                        buffer_const(_start, _end, 170, _buf),
                        buffer_const(_start, _end, 171, _buf),
                    )?;
                }
                if (_start >= 172 && _start < 176)
                    || (_end > 172 && _end <= 176)
                {
                    self.0.lock().unwrap().write_uicr_customern(
                        11,
                        buffer_const(_start, _end, 172, _buf),
                        buffer_const(_start, _end, 173, _buf),
                        buffer_const(_start, _end, 174, _buf),
                        buffer_const(_start, _end, 175, _buf),
                    )?;
                }
                if (_start >= 176 && _start < 180)
                    || (_end > 176 && _end <= 180)
                {
                    self.0.lock().unwrap().write_uicr_customern(
                        12,
                        buffer_const(_start, _end, 176, _buf),
                        buffer_const(_start, _end, 177, _buf),
                        buffer_const(_start, _end, 178, _buf),
                        buffer_const(_start, _end, 179, _buf),
                    )?;
                }
                if (_start >= 180 && _start < 184)
                    || (_end > 180 && _end <= 184)
                {
                    self.0.lock().unwrap().write_uicr_customern(
                        13,
                        buffer_const(_start, _end, 180, _buf),
                        buffer_const(_start, _end, 181, _buf),
                        buffer_const(_start, _end, 182, _buf),
                        buffer_const(_start, _end, 183, _buf),
                    )?;
                }
                if (_start >= 184 && _start < 188)
                    || (_end > 184 && _end <= 188)
                {
                    self.0.lock().unwrap().write_uicr_customern(
                        14,
                        buffer_const(_start, _end, 184, _buf),
                        buffer_const(_start, _end, 185, _buf),
                        buffer_const(_start, _end, 186, _buf),
                        buffer_const(_start, _end, 187, _buf),
                    )?;
                }
                if (_start >= 188 && _start < 192)
                    || (_end > 188 && _end <= 192)
                {
                    self.0.lock().unwrap().write_uicr_customern(
                        15,
                        buffer_const(_start, _end, 188, _buf),
                        buffer_const(_start, _end, 189, _buf),
                        buffer_const(_start, _end, 190, _buf),
                        buffer_const(_start, _end, 191, _buf),
                    )?;
                }
                if (_start >= 192 && _start < 196)
                    || (_end > 192 && _end <= 196)
                {
                    self.0.lock().unwrap().write_uicr_customern(
                        16,
                        buffer_const(_start, _end, 192, _buf),
                        buffer_const(_start, _end, 193, _buf),
                        buffer_const(_start, _end, 194, _buf),
                        buffer_const(_start, _end, 195, _buf),
                    )?;
                }
                if (_start >= 196 && _start < 200)
                    || (_end > 196 && _end <= 200)
                {
                    self.0.lock().unwrap().write_uicr_customern(
                        17,
                        buffer_const(_start, _end, 196, _buf),
                        buffer_const(_start, _end, 197, _buf),
                        buffer_const(_start, _end, 198, _buf),
                        buffer_const(_start, _end, 199, _buf),
                    )?;
                }
                if (_start >= 200 && _start < 204)
                    || (_end > 200 && _end <= 204)
                {
                    self.0.lock().unwrap().write_uicr_customern(
                        18,
                        buffer_const(_start, _end, 200, _buf),
                        buffer_const(_start, _end, 201, _buf),
                        buffer_const(_start, _end, 202, _buf),
                        buffer_const(_start, _end, 203, _buf),
                    )?;
                }
                if (_start >= 204 && _start < 208)
                    || (_end > 204 && _end <= 208)
                {
                    self.0.lock().unwrap().write_uicr_customern(
                        19,
                        buffer_const(_start, _end, 204, _buf),
                        buffer_const(_start, _end, 205, _buf),
                        buffer_const(_start, _end, 206, _buf),
                        buffer_const(_start, _end, 207, _buf),
                    )?;
                }
                if (_start >= 208 && _start < 212)
                    || (_end > 208 && _end <= 212)
                {
                    self.0.lock().unwrap().write_uicr_customern(
                        20,
                        buffer_const(_start, _end, 208, _buf),
                        buffer_const(_start, _end, 209, _buf),
                        buffer_const(_start, _end, 210, _buf),
                        buffer_const(_start, _end, 211, _buf),
                    )?;
                }
                if (_start >= 212 && _start < 216)
                    || (_end > 212 && _end <= 216)
                {
                    self.0.lock().unwrap().write_uicr_customern(
                        21,
                        buffer_const(_start, _end, 212, _buf),
                        buffer_const(_start, _end, 213, _buf),
                        buffer_const(_start, _end, 214, _buf),
                        buffer_const(_start, _end, 215, _buf),
                    )?;
                }
                if (_start >= 216 && _start < 220)
                    || (_end > 216 && _end <= 220)
                {
                    self.0.lock().unwrap().write_uicr_customern(
                        22,
                        buffer_const(_start, _end, 216, _buf),
                        buffer_const(_start, _end, 217, _buf),
                        buffer_const(_start, _end, 218, _buf),
                        buffer_const(_start, _end, 219, _buf),
                    )?;
                }
                if (_start >= 220 && _start < 224)
                    || (_end > 220 && _end <= 224)
                {
                    self.0.lock().unwrap().write_uicr_customern(
                        23,
                        buffer_const(_start, _end, 220, _buf),
                        buffer_const(_start, _end, 221, _buf),
                        buffer_const(_start, _end, 222, _buf),
                        buffer_const(_start, _end, 223, _buf),
                    )?;
                }
                if (_start >= 224 && _start < 228)
                    || (_end > 224 && _end <= 228)
                {
                    self.0.lock().unwrap().write_uicr_customern(
                        24,
                        buffer_const(_start, _end, 224, _buf),
                        buffer_const(_start, _end, 225, _buf),
                        buffer_const(_start, _end, 226, _buf),
                        buffer_const(_start, _end, 227, _buf),
                    )?;
                }
                if (_start >= 228 && _start < 232)
                    || (_end > 228 && _end <= 232)
                {
                    self.0.lock().unwrap().write_uicr_customern(
                        25,
                        buffer_const(_start, _end, 228, _buf),
                        buffer_const(_start, _end, 229, _buf),
                        buffer_const(_start, _end, 230, _buf),
                        buffer_const(_start, _end, 231, _buf),
                    )?;
                }
                if (_start >= 232 && _start < 236)
                    || (_end > 232 && _end <= 236)
                {
                    self.0.lock().unwrap().write_uicr_customern(
                        26,
                        buffer_const(_start, _end, 232, _buf),
                        buffer_const(_start, _end, 233, _buf),
                        buffer_const(_start, _end, 234, _buf),
                        buffer_const(_start, _end, 235, _buf),
                    )?;
                }
                if (_start >= 236 && _start < 240)
                    || (_end > 236 && _end <= 240)
                {
                    self.0.lock().unwrap().write_uicr_customern(
                        27,
                        buffer_const(_start, _end, 236, _buf),
                        buffer_const(_start, _end, 237, _buf),
                        buffer_const(_start, _end, 238, _buf),
                        buffer_const(_start, _end, 239, _buf),
                    )?;
                }
                if (_start >= 240 && _start < 244)
                    || (_end > 240 && _end <= 244)
                {
                    self.0.lock().unwrap().write_uicr_customern(
                        28,
                        buffer_const(_start, _end, 240, _buf),
                        buffer_const(_start, _end, 241, _buf),
                        buffer_const(_start, _end, 242, _buf),
                        buffer_const(_start, _end, 243, _buf),
                    )?;
                }
                if (_start >= 244 && _start < 248)
                    || (_end > 244 && _end <= 248)
                {
                    self.0.lock().unwrap().write_uicr_customern(
                        29,
                        buffer_const(_start, _end, 244, _buf),
                        buffer_const(_start, _end, 245, _buf),
                        buffer_const(_start, _end, 246, _buf),
                        buffer_const(_start, _end, 247, _buf),
                    )?;
                }
                if (_start >= 248 && _start < 252)
                    || (_end > 248 && _end <= 252)
                {
                    self.0.lock().unwrap().write_uicr_customern(
                        30,
                        buffer_const(_start, _end, 248, _buf),
                        buffer_const(_start, _end, 249, _buf),
                        buffer_const(_start, _end, 250, _buf),
                        buffer_const(_start, _end, 251, _buf),
                    )?;
                }
                if (_start >= 252 && _start < 256)
                    || (_end > 252 && _end <= 256)
                {
                    self.0.lock().unwrap().write_uicr_customern(
                        31,
                        buffer_const(_start, _end, 252, _buf),
                        buffer_const(_start, _end, 253, _buf),
                        buffer_const(_start, _end, 254, _buf),
                        buffer_const(_start, _end, 255, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
}
impl PeripheralPage0x10001000 {
    fn read_uicr_rbpconf(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_uicr_rbpconf_pr0()? << 0;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_uicr_rbpconf_pall()? << 0;
        }
        Ok(())
    }
    fn write_uicr_rbpconf(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_uicr_rbpconf_pr0((*byte >> 0) & 255)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_uicr_rbpconf_pall((*byte >> 0) & 255)?;
        }
        Ok(())
    }
    fn read_uicr_xtalfreq(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_uicr_xtalfreq_xtalfreq()? << 0;
        }
        Ok(())
    }
    fn write_uicr_xtalfreq(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_uicr_xtalfreq_xtalfreq((*byte >> 0) & 255)?;
        }
        Ok(())
    }
    fn read_uicr_fwid(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if _byte_0.is_some() || _byte_1.is_some() {
            self.0
                .lock()
                .unwrap()
                .read_uicr_fwid_fwid(_byte_0, _byte_1)?;
        }
        Ok(())
    }
}
pub struct PeripheralPage0x40000000(
    pub std::sync::Arc<std::sync::Mutex<super::peripheral::Peripherals>>,
);
impl icicle_vm::cpu::mem::IoMemory for PeripheralPage0x40000000 {
    fn read(
        &mut self,
        _addr: u64,
        _buf: &mut [u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073741824;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=27, 1..=28) => {
                if (_start < 4) || (_end <= 4) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 4 && _start < 8) || (_end > 4 && _end <= 8) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 8 && _start < 12) || (_end > 8 && _end <= 12) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 12 && _start < 16) || (_end > 12 && _end <= 16) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 16 && _start < 20) || (_end > 16 && _end <= 20) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 20 && _start < 24) || (_end > 20 && _end <= 24) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 24 && _start < 28) || (_end > 24 && _end <= 28) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (120..=127, 121..=128) => {
                if (_start >= 120 && _start < 124)
                    || (_end > 120 && _end <= 124)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 124 && _start < 128)
                    || (_end > 124 && _end <= 128)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (256..=275, 257..=276) => {
                if (_start >= 264 && _start < 268)
                    || (_end > 264 && _end <= 268)
                {
                    self.0.lock().unwrap().read_power_events_pofwarn(
                        &mut buffer_mut(_start, _end, 264, _buf),
                        &mut buffer_mut(_start, _end, 265, _buf),
                        &mut buffer_mut(_start, _end, 266, _buf),
                        &mut buffer_mut(_start, _end, 267, _buf),
                    )?;
                }
                if (_start >= 256 && _start < 260)
                    || (_end > 256 && _end <= 260)
                {
                    self.0.lock().unwrap().read_clock_events_hfclkstarted(
                        &mut buffer_mut(_start, _end, 256, _buf),
                        &mut buffer_mut(_start, _end, 257, _buf),
                        &mut buffer_mut(_start, _end, 258, _buf),
                        &mut buffer_mut(_start, _end, 259, _buf),
                    )?;
                }
                if (_start >= 260 && _start < 264)
                    || (_end > 260 && _end <= 264)
                {
                    self.0.lock().unwrap().read_clock_events_lfclkstarted(
                        &mut buffer_mut(_start, _end, 260, _buf),
                        &mut buffer_mut(_start, _end, 261, _buf),
                        &mut buffer_mut(_start, _end, 262, _buf),
                        &mut buffer_mut(_start, _end, 263, _buf),
                    )?;
                }
                if (_start >= 268 && _start < 272)
                    || (_end > 268 && _end <= 272)
                {
                    self.0.lock().unwrap().read_clock_events_done(
                        &mut buffer_mut(_start, _end, 268, _buf),
                        &mut buffer_mut(_start, _end, 269, _buf),
                        &mut buffer_mut(_start, _end, 270, _buf),
                        &mut buffer_mut(_start, _end, 271, _buf),
                    )?;
                }
                if (_start >= 272 && _start < 276)
                    || (_end > 272 && _end <= 276)
                {
                    self.0.lock().unwrap().read_clock_events_ctto(
                        &mut buffer_mut(_start, _end, 272, _buf),
                        &mut buffer_mut(_start, _end, 273, _buf),
                        &mut buffer_mut(_start, _end, 274, _buf),
                        &mut buffer_mut(_start, _end, 275, _buf),
                    )?;
                }
            }
            (772..=779, 773..=780) => {
                if (_start >= 772 && _start < 776)
                    || (_end > 772 && _end <= 776)
                {
                    self.read_powerclock_intenset(
                        &mut buffer_mut(_start, _end, 772, _buf),
                        &mut buffer_mut(_start, _end, 773, _buf),
                        &mut buffer_mut(_start, _end, 774, _buf),
                        &mut buffer_mut(_start, _end, 775, _buf),
                    )?;
                }
                if (_start >= 776 && _start < 780)
                    || (_end > 776 && _end <= 780)
                {
                    self.read_powerclock_intenclr(
                        &mut buffer_mut(_start, _end, 776, _buf),
                        &mut buffer_mut(_start, _end, 777, _buf),
                        &mut buffer_mut(_start, _end, 778, _buf),
                        &mut buffer_mut(_start, _end, 779, _buf),
                    )?;
                }
            }
            (1024..=1027, 1025..=1028) => {
                if (_start >= 1024 && _start < 1028)
                    || (_end > 1024 && _end <= 1028)
                {
                    self.read_power_resetreas(
                        &mut buffer_mut(_start, _end, 1024, _buf),
                        &mut buffer_mut(_start, _end, 1025, _buf),
                        &mut buffer_mut(_start, _end, 1026, _buf),
                        &mut buffer_mut(_start, _end, 1027, _buf),
                    )?;
                }
            }
            (1032..=1039, 1033..=1040) => {
                if (_start >= 1032 && _start < 1036)
                    || (_end > 1032 && _end <= 1036)
                {
                    self.read_clock_hfclkrun(
                        &mut buffer_mut(_start, _end, 1032, _buf),
                        &mut buffer_mut(_start, _end, 1033, _buf),
                        &mut buffer_mut(_start, _end, 1034, _buf),
                        &mut buffer_mut(_start, _end, 1035, _buf),
                    )?;
                }
                if (_start >= 1036 && _start < 1040)
                    || (_end > 1036 && _end <= 1040)
                {
                    self.read_clock_hfclkstat(
                        &mut buffer_mut(_start, _end, 1036, _buf),
                        &mut buffer_mut(_start, _end, 1037, _buf),
                        &mut buffer_mut(_start, _end, 1038, _buf),
                        &mut buffer_mut(_start, _end, 1039, _buf),
                    )?;
                }
            }
            (1044..=1055, 1045..=1056) => {
                if (_start >= 1044 && _start < 1048)
                    || (_end > 1044 && _end <= 1048)
                {
                    self.read_clock_lfclkrun(
                        &mut buffer_mut(_start, _end, 1044, _buf),
                        &mut buffer_mut(_start, _end, 1045, _buf),
                        &mut buffer_mut(_start, _end, 1046, _buf),
                        &mut buffer_mut(_start, _end, 1047, _buf),
                    )?;
                }
                if (_start >= 1048 && _start < 1052)
                    || (_end > 1048 && _end <= 1052)
                {
                    self.read_clock_lfclkstat(
                        &mut buffer_mut(_start, _end, 1048, _buf),
                        &mut buffer_mut(_start, _end, 1049, _buf),
                        &mut buffer_mut(_start, _end, 1050, _buf),
                        &mut buffer_mut(_start, _end, 1051, _buf),
                    )?;
                }
                if (_start >= 1052 && _start < 1056)
                    || (_end > 1052 && _end <= 1056)
                {
                    self.read_clock_lfclksrccopy(
                        &mut buffer_mut(_start, _end, 1052, _buf),
                        &mut buffer_mut(_start, _end, 1053, _buf),
                        &mut buffer_mut(_start, _end, 1054, _buf),
                        &mut buffer_mut(_start, _end, 1055, _buf),
                    )?;
                }
            }
            (1064..=1067, 1065..=1068) => {
                if (_start >= 1064 && _start < 1068)
                    || (_end > 1064 && _end <= 1068)
                {
                    self.read_power_ramstatus(
                        &mut buffer_mut(_start, _end, 1064, _buf),
                        &mut buffer_mut(_start, _end, 1065, _buf),
                        &mut buffer_mut(_start, _end, 1066, _buf),
                        &mut buffer_mut(_start, _end, 1067, _buf),
                    )?;
                }
            }
            (1280..=1283, 1281..=1284) => {
                if (_start >= 1280 && _start < 1284)
                    || (_end > 1280 && _end <= 1284)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (1296..=1299, 1297..=1300) => {
                if (_start >= 1296 && _start < 1300)
                    || (_end > 1296 && _end <= 1300)
                {
                    self.read_power_pofcon(
                        &mut buffer_mut(_start, _end, 1296, _buf),
                        &mut buffer_mut(_start, _end, 1297, _buf),
                        &mut buffer_mut(_start, _end, 1298, _buf),
                        &mut buffer_mut(_start, _end, 1299, _buf),
                    )?;
                }
            }
            (1304..=1311, 1305..=1312) => {
                if (_start >= 1308 && _start < 1312)
                    || (_end > 1308 && _end <= 1312)
                {
                    self.read_power_gpregret(
                        &mut buffer_mut(_start, _end, 1308, _buf),
                        &mut buffer_mut(_start, _end, 1309, _buf),
                        &mut buffer_mut(_start, _end, 1310, _buf),
                        &mut buffer_mut(_start, _end, 1311, _buf),
                    )?;
                }
                if (_start >= 1304 && _start < 1308)
                    || (_end > 1304 && _end <= 1308)
                {
                    self.read_clock_lfclksrc(
                        &mut buffer_mut(_start, _end, 1304, _buf),
                        &mut buffer_mut(_start, _end, 1305, _buf),
                        &mut buffer_mut(_start, _end, 1306, _buf),
                        &mut buffer_mut(_start, _end, 1307, _buf),
                    )?;
                }
            }
            (1316..=1327, 1317..=1328) => {
                if (_start >= 1316 && _start < 1320)
                    || (_end > 1316 && _end <= 1320)
                {
                    self.read_power_ramon(
                        &mut buffer_mut(_start, _end, 1316, _buf),
                        &mut buffer_mut(_start, _end, 1317, _buf),
                        &mut buffer_mut(_start, _end, 1318, _buf),
                        &mut buffer_mut(_start, _end, 1319, _buf),
                    )?;
                }
                if (_start >= 1320 && _start < 1324)
                    || (_end > 1320 && _end <= 1324)
                {
                    self.read_mpu_perr0(
                        &mut buffer_mut(_start, _end, 1320, _buf),
                        &mut buffer_mut(_start, _end, 1321, _buf),
                        &mut buffer_mut(_start, _end, 1322, _buf),
                        &mut buffer_mut(_start, _end, 1323, _buf),
                    )?;
                }
                if (_start >= 1324 && _start < 1328)
                    || (_end > 1324 && _end <= 1328)
                {
                    self.0.lock().unwrap().read_mpu_rlenr0(
                        &mut buffer_mut(_start, _end, 1324, _buf),
                        &mut buffer_mut(_start, _end, 1325, _buf),
                        &mut buffer_mut(_start, _end, 1326, _buf),
                        &mut buffer_mut(_start, _end, 1327, _buf),
                    )?;
                }
            }
            (1336..=1339, 1337..=1340) => {
                if (_start >= 1336 && _start < 1340)
                    || (_end > 1336 && _end <= 1340)
                {
                    self.read_clock_ctiv(
                        &mut buffer_mut(_start, _end, 1336, _buf),
                        &mut buffer_mut(_start, _end, 1337, _buf),
                        &mut buffer_mut(_start, _end, 1338, _buf),
                        &mut buffer_mut(_start, _end, 1339, _buf),
                    )?;
                }
            }
            (1348..=1351, 1349..=1352) => {
                if (_start >= 1348 && _start < 1352)
                    || (_end > 1348 && _end <= 1352)
                {
                    self.read_power_reset(
                        &mut buffer_mut(_start, _end, 1348, _buf),
                        &mut buffer_mut(_start, _end, 1349, _buf),
                        &mut buffer_mut(_start, _end, 1350, _buf),
                        &mut buffer_mut(_start, _end, 1351, _buf),
                    )?;
                }
            }
            (1360..=1367, 1361..=1368) => {
                if (_start >= 1364 && _start < 1368)
                    || (_end > 1364 && _end <= 1368)
                {
                    self.read_power_ramonb(
                        &mut buffer_mut(_start, _end, 1364, _buf),
                        &mut buffer_mut(_start, _end, 1365, _buf),
                        &mut buffer_mut(_start, _end, 1366, _buf),
                        &mut buffer_mut(_start, _end, 1367, _buf),
                    )?;
                }
                if (_start >= 1360 && _start < 1364)
                    || (_end > 1360 && _end <= 1364)
                {
                    self.read_clock_xtalfreq(
                        &mut buffer_mut(_start, _end, 1360, _buf),
                        &mut buffer_mut(_start, _end, 1361, _buf),
                        &mut buffer_mut(_start, _end, 1362, _buf),
                        &mut buffer_mut(_start, _end, 1363, _buf),
                    )?;
                }
            }
            (1400..=1403, 1401..=1404) => {
                if (_start >= 1400 && _start < 1404)
                    || (_end > 1400 && _end <= 1404)
                {
                    self.read_power_dcdcen(
                        &mut buffer_mut(_start, _end, 1400, _buf),
                        &mut buffer_mut(_start, _end, 1401, _buf),
                        &mut buffer_mut(_start, _end, 1402, _buf),
                        &mut buffer_mut(_start, _end, 1403, _buf),
                    )?;
                }
            }
            (1536..=1551, 1537..=1552) => {
                if (_start >= 1536 && _start < 1540)
                    || (_end > 1536 && _end <= 1540)
                {
                    self.read_mpu_protenset0(
                        &mut buffer_mut(_start, _end, 1536, _buf),
                        &mut buffer_mut(_start, _end, 1537, _buf),
                        &mut buffer_mut(_start, _end, 1538, _buf),
                        &mut buffer_mut(_start, _end, 1539, _buf),
                    )?;
                }
                if (_start >= 1540 && _start < 1544)
                    || (_end > 1540 && _end <= 1544)
                {
                    self.read_mpu_protenset1(
                        &mut buffer_mut(_start, _end, 1540, _buf),
                        &mut buffer_mut(_start, _end, 1541, _buf),
                        &mut buffer_mut(_start, _end, 1542, _buf),
                        &mut buffer_mut(_start, _end, 1543, _buf),
                    )?;
                }
                if (_start >= 1544 && _start < 1548)
                    || (_end > 1544 && _end <= 1548)
                {
                    self.read_mpu_disableindebug(
                        &mut buffer_mut(_start, _end, 1544, _buf),
                        &mut buffer_mut(_start, _end, 1545, _buf),
                        &mut buffer_mut(_start, _end, 1546, _buf),
                        &mut buffer_mut(_start, _end, 1547, _buf),
                    )?;
                }
                if (_start >= 1548 && _start < 1552)
                    || (_end > 1548 && _end <= 1552)
                {
                    self.read_mpu_protblocksize(
                        &mut buffer_mut(_start, _end, 1548, _buf),
                        &mut buffer_mut(_start, _end, 1549, _buf),
                        &mut buffer_mut(_start, _end, 1550, _buf),
                        &mut buffer_mut(_start, _end, 1551, _buf),
                    )?;
                }
            }
            (2568..=2571, 2569..=2572) => {
                if (_start >= 2568 && _start < 2572)
                    || (_end > 2568 && _end <= 2572)
                {
                    self.read_power_dcdcforce(
                        &mut buffer_mut(_start, _end, 2568, _buf),
                        &mut buffer_mut(_start, _end, 2569, _buf),
                        &mut buffer_mut(_start, _end, 2570, _buf),
                        &mut buffer_mut(_start, _end, 2571, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
    fn write(
        &mut self,
        _addr: u64,
        _buf: &[u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073741824;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=27, 1..=28) => {
                if (_start < 4) || (_end <= 4) {
                    self.0.lock().unwrap().write_clock_tasks_hfclkstart(
                        buffer_const(_start, _end, 0, _buf),
                        buffer_const(_start, _end, 1, _buf),
                        buffer_const(_start, _end, 2, _buf),
                        buffer_const(_start, _end, 3, _buf),
                    )?;
                }
                if (_start >= 4 && _start < 8) || (_end > 4 && _end <= 8) {
                    self.0.lock().unwrap().write_clock_tasks_hfclkstop(
                        buffer_const(_start, _end, 4, _buf),
                        buffer_const(_start, _end, 5, _buf),
                        buffer_const(_start, _end, 6, _buf),
                        buffer_const(_start, _end, 7, _buf),
                    )?;
                }
                if (_start >= 8 && _start < 12) || (_end > 8 && _end <= 12) {
                    self.0.lock().unwrap().write_clock_tasks_lfclkstart(
                        buffer_const(_start, _end, 8, _buf),
                        buffer_const(_start, _end, 9, _buf),
                        buffer_const(_start, _end, 10, _buf),
                        buffer_const(_start, _end, 11, _buf),
                    )?;
                }
                if (_start >= 12 && _start < 16) || (_end > 12 && _end <= 16) {
                    self.0.lock().unwrap().write_clock_tasks_lfclkstop(
                        buffer_const(_start, _end, 12, _buf),
                        buffer_const(_start, _end, 13, _buf),
                        buffer_const(_start, _end, 14, _buf),
                        buffer_const(_start, _end, 15, _buf),
                    )?;
                }
                if (_start >= 16 && _start < 20) || (_end > 16 && _end <= 20) {
                    self.0.lock().unwrap().write_clock_tasks_cal(
                        buffer_const(_start, _end, 16, _buf),
                        buffer_const(_start, _end, 17, _buf),
                        buffer_const(_start, _end, 18, _buf),
                        buffer_const(_start, _end, 19, _buf),
                    )?;
                }
                if (_start >= 20 && _start < 24) || (_end > 20 && _end <= 24) {
                    self.0.lock().unwrap().write_clock_tasks_ctstart(
                        buffer_const(_start, _end, 20, _buf),
                        buffer_const(_start, _end, 21, _buf),
                        buffer_const(_start, _end, 22, _buf),
                        buffer_const(_start, _end, 23, _buf),
                    )?;
                }
                if (_start >= 24 && _start < 28) || (_end > 24 && _end <= 28) {
                    self.0.lock().unwrap().write_clock_tasks_ctstop(
                        buffer_const(_start, _end, 24, _buf),
                        buffer_const(_start, _end, 25, _buf),
                        buffer_const(_start, _end, 26, _buf),
                        buffer_const(_start, _end, 27, _buf),
                    )?;
                }
            }
            (120..=127, 121..=128) => {
                if (_start >= 120 && _start < 124)
                    || (_end > 120 && _end <= 124)
                {
                    self.0.lock().unwrap().write_power_tasks_constlat(
                        buffer_const(_start, _end, 120, _buf),
                        buffer_const(_start, _end, 121, _buf),
                        buffer_const(_start, _end, 122, _buf),
                        buffer_const(_start, _end, 123, _buf),
                    )?;
                }
                if (_start >= 124 && _start < 128)
                    || (_end > 124 && _end <= 128)
                {
                    self.0.lock().unwrap().write_power_tasks_lowpwr(
                        buffer_const(_start, _end, 124, _buf),
                        buffer_const(_start, _end, 125, _buf),
                        buffer_const(_start, _end, 126, _buf),
                        buffer_const(_start, _end, 127, _buf),
                    )?;
                }
            }
            (256..=275, 257..=276) => {
                if (_start >= 264 && _start < 268)
                    || (_end > 264 && _end <= 268)
                {
                    self.0.lock().unwrap().write_power_events_pofwarn(
                        buffer_const(_start, _end, 264, _buf),
                        buffer_const(_start, _end, 265, _buf),
                        buffer_const(_start, _end, 266, _buf),
                        buffer_const(_start, _end, 267, _buf),
                    )?;
                }
                if (_start >= 256 && _start < 260)
                    || (_end > 256 && _end <= 260)
                {
                    self.0.lock().unwrap().write_clock_events_hfclkstarted(
                        buffer_const(_start, _end, 256, _buf),
                        buffer_const(_start, _end, 257, _buf),
                        buffer_const(_start, _end, 258, _buf),
                        buffer_const(_start, _end, 259, _buf),
                    )?;
                }
                if (_start >= 260 && _start < 264)
                    || (_end > 260 && _end <= 264)
                {
                    self.0.lock().unwrap().write_clock_events_lfclkstarted(
                        buffer_const(_start, _end, 260, _buf),
                        buffer_const(_start, _end, 261, _buf),
                        buffer_const(_start, _end, 262, _buf),
                        buffer_const(_start, _end, 263, _buf),
                    )?;
                }
                if (_start >= 268 && _start < 272)
                    || (_end > 268 && _end <= 272)
                {
                    self.0.lock().unwrap().write_clock_events_done(
                        buffer_const(_start, _end, 268, _buf),
                        buffer_const(_start, _end, 269, _buf),
                        buffer_const(_start, _end, 270, _buf),
                        buffer_const(_start, _end, 271, _buf),
                    )?;
                }
                if (_start >= 272 && _start < 276)
                    || (_end > 272 && _end <= 276)
                {
                    self.0.lock().unwrap().write_clock_events_ctto(
                        buffer_const(_start, _end, 272, _buf),
                        buffer_const(_start, _end, 273, _buf),
                        buffer_const(_start, _end, 274, _buf),
                        buffer_const(_start, _end, 275, _buf),
                    )?;
                }
            }
            (772..=779, 773..=780) => {
                if (_start >= 772 && _start < 776)
                    || (_end > 772 && _end <= 776)
                {
                    self.write_powerclock_intenset(
                        buffer_const(_start, _end, 772, _buf),
                        buffer_const(_start, _end, 773, _buf),
                        buffer_const(_start, _end, 774, _buf),
                        buffer_const(_start, _end, 775, _buf),
                    )?;
                }
                if (_start >= 776 && _start < 780)
                    || (_end > 776 && _end <= 780)
                {
                    self.write_powerclock_intenclr(
                        buffer_const(_start, _end, 776, _buf),
                        buffer_const(_start, _end, 777, _buf),
                        buffer_const(_start, _end, 778, _buf),
                        buffer_const(_start, _end, 779, _buf),
                    )?;
                }
            }
            (1024..=1027, 1025..=1028) => {
                if (_start >= 1024 && _start < 1028)
                    || (_end > 1024 && _end <= 1028)
                {
                    self.write_power_resetreas(
                        buffer_const(_start, _end, 1024, _buf),
                        buffer_const(_start, _end, 1025, _buf),
                        buffer_const(_start, _end, 1026, _buf),
                        buffer_const(_start, _end, 1027, _buf),
                    )?;
                }
            }
            (1032..=1039, 1033..=1040) => {
                if (_start >= 1032 && _start < 1036)
                    || (_end > 1032 && _end <= 1036)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 1036 && _start < 1040)
                    || (_end > 1036 && _end <= 1040)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (1044..=1055, 1045..=1056) => {
                if (_start >= 1044 && _start < 1048)
                    || (_end > 1044 && _end <= 1048)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 1048 && _start < 1052)
                    || (_end > 1048 && _end <= 1052)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 1052 && _start < 1056)
                    || (_end > 1052 && _end <= 1056)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (1064..=1067, 1065..=1068) => {
                if (_start >= 1064 && _start < 1068)
                    || (_end > 1064 && _end <= 1068)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (1280..=1283, 1281..=1284) => {
                if (_start >= 1280 && _start < 1284)
                    || (_end > 1280 && _end <= 1284)
                {
                    self.write_power_systemoff(
                        buffer_const(_start, _end, 1280, _buf),
                        buffer_const(_start, _end, 1281, _buf),
                        buffer_const(_start, _end, 1282, _buf),
                        buffer_const(_start, _end, 1283, _buf),
                    )?;
                }
            }
            (1296..=1299, 1297..=1300) => {
                if (_start >= 1296 && _start < 1300)
                    || (_end > 1296 && _end <= 1300)
                {
                    self.write_power_pofcon(
                        buffer_const(_start, _end, 1296, _buf),
                        buffer_const(_start, _end, 1297, _buf),
                        buffer_const(_start, _end, 1298, _buf),
                        buffer_const(_start, _end, 1299, _buf),
                    )?;
                }
            }
            (1304..=1311, 1305..=1312) => {
                if (_start >= 1308 && _start < 1312)
                    || (_end > 1308 && _end <= 1312)
                {
                    self.write_power_gpregret(
                        buffer_const(_start, _end, 1308, _buf),
                        buffer_const(_start, _end, 1309, _buf),
                        buffer_const(_start, _end, 1310, _buf),
                        buffer_const(_start, _end, 1311, _buf),
                    )?;
                }
                if (_start >= 1304 && _start < 1308)
                    || (_end > 1304 && _end <= 1308)
                {
                    self.write_clock_lfclksrc(
                        buffer_const(_start, _end, 1304, _buf),
                        buffer_const(_start, _end, 1305, _buf),
                        buffer_const(_start, _end, 1306, _buf),
                        buffer_const(_start, _end, 1307, _buf),
                    )?;
                }
            }
            (1316..=1327, 1317..=1328) => {
                if (_start >= 1316 && _start < 1320)
                    || (_end > 1316 && _end <= 1320)
                {
                    self.write_power_ramon(
                        buffer_const(_start, _end, 1316, _buf),
                        buffer_const(_start, _end, 1317, _buf),
                        buffer_const(_start, _end, 1318, _buf),
                        buffer_const(_start, _end, 1319, _buf),
                    )?;
                }
                if (_start >= 1320 && _start < 1324)
                    || (_end > 1320 && _end <= 1324)
                {
                    self.write_mpu_perr0(
                        buffer_const(_start, _end, 1320, _buf),
                        buffer_const(_start, _end, 1321, _buf),
                        buffer_const(_start, _end, 1322, _buf),
                        buffer_const(_start, _end, 1323, _buf),
                    )?;
                }
                if (_start >= 1324 && _start < 1328)
                    || (_end > 1324 && _end <= 1328)
                {
                    self.0.lock().unwrap().write_mpu_rlenr0(
                        buffer_const(_start, _end, 1324, _buf),
                        buffer_const(_start, _end, 1325, _buf),
                        buffer_const(_start, _end, 1326, _buf),
                        buffer_const(_start, _end, 1327, _buf),
                    )?;
                }
            }
            (1336..=1339, 1337..=1340) => {
                if (_start >= 1336 && _start < 1340)
                    || (_end > 1336 && _end <= 1340)
                {
                    self.write_clock_ctiv(
                        buffer_const(_start, _end, 1336, _buf),
                        buffer_const(_start, _end, 1337, _buf),
                        buffer_const(_start, _end, 1338, _buf),
                        buffer_const(_start, _end, 1339, _buf),
                    )?;
                }
            }
            (1348..=1351, 1349..=1352) => {
                if (_start >= 1348 && _start < 1352)
                    || (_end > 1348 && _end <= 1352)
                {
                    self.write_power_reset(
                        buffer_const(_start, _end, 1348, _buf),
                        buffer_const(_start, _end, 1349, _buf),
                        buffer_const(_start, _end, 1350, _buf),
                        buffer_const(_start, _end, 1351, _buf),
                    )?;
                }
            }
            (1360..=1367, 1361..=1368) => {
                if (_start >= 1364 && _start < 1368)
                    || (_end > 1364 && _end <= 1368)
                {
                    self.write_power_ramonb(
                        buffer_const(_start, _end, 1364, _buf),
                        buffer_const(_start, _end, 1365, _buf),
                        buffer_const(_start, _end, 1366, _buf),
                        buffer_const(_start, _end, 1367, _buf),
                    )?;
                }
                if (_start >= 1360 && _start < 1364)
                    || (_end > 1360 && _end <= 1364)
                {
                    self.write_clock_xtalfreq(
                        buffer_const(_start, _end, 1360, _buf),
                        buffer_const(_start, _end, 1361, _buf),
                        buffer_const(_start, _end, 1362, _buf),
                        buffer_const(_start, _end, 1363, _buf),
                    )?;
                }
            }
            (1400..=1403, 1401..=1404) => {
                if (_start >= 1400 && _start < 1404)
                    || (_end > 1400 && _end <= 1404)
                {
                    self.write_power_dcdcen(
                        buffer_const(_start, _end, 1400, _buf),
                        buffer_const(_start, _end, 1401, _buf),
                        buffer_const(_start, _end, 1402, _buf),
                        buffer_const(_start, _end, 1403, _buf),
                    )?;
                }
            }
            (1536..=1551, 1537..=1552) => {
                if (_start >= 1536 && _start < 1540)
                    || (_end > 1536 && _end <= 1540)
                {
                    self.write_mpu_protenset0(
                        buffer_const(_start, _end, 1536, _buf),
                        buffer_const(_start, _end, 1537, _buf),
                        buffer_const(_start, _end, 1538, _buf),
                        buffer_const(_start, _end, 1539, _buf),
                    )?;
                }
                if (_start >= 1540 && _start < 1544)
                    || (_end > 1540 && _end <= 1544)
                {
                    self.write_mpu_protenset1(
                        buffer_const(_start, _end, 1540, _buf),
                        buffer_const(_start, _end, 1541, _buf),
                        buffer_const(_start, _end, 1542, _buf),
                        buffer_const(_start, _end, 1543, _buf),
                    )?;
                }
                if (_start >= 1544 && _start < 1548)
                    || (_end > 1544 && _end <= 1548)
                {
                    self.write_mpu_disableindebug(
                        buffer_const(_start, _end, 1544, _buf),
                        buffer_const(_start, _end, 1545, _buf),
                        buffer_const(_start, _end, 1546, _buf),
                        buffer_const(_start, _end, 1547, _buf),
                    )?;
                }
                if (_start >= 1548 && _start < 1552)
                    || (_end > 1548 && _end <= 1552)
                {
                    self.write_mpu_protblocksize(
                        buffer_const(_start, _end, 1548, _buf),
                        buffer_const(_start, _end, 1549, _buf),
                        buffer_const(_start, _end, 1550, _buf),
                        buffer_const(_start, _end, 1551, _buf),
                    )?;
                }
            }
            (2568..=2571, 2569..=2572) => {
                if (_start >= 2568 && _start < 2572)
                    || (_end > 2568 && _end <= 2572)
                {
                    self.write_power_dcdcforce(
                        buffer_const(_start, _end, 2568, _buf),
                        buffer_const(_start, _end, 2569, _buf),
                        buffer_const(_start, _end, 2570, _buf),
                        buffer_const(_start, _end, 2571, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
}
impl PeripheralPage0x40000000 {
    fn read_powerclock_intenset(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_powerclock_intenset_pofwarn()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self
                .0
                .lock()
                .unwrap()
                .read_powerclock_intenset_hfclkstarted()?
                << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self
                .0
                .lock()
                .unwrap()
                .read_powerclock_intenset_lfclkstarted()?
                << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_powerclock_intenset_done()? << 3;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_powerclock_intenset_ctto()? << 4;
        }
        Ok(())
    }
    fn write_powerclock_intenset(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_powerclock_intenset_pofwarn((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_powerclock_intenset_hfclkstarted((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_powerclock_intenset_lfclkstarted((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_powerclock_intenset_done((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_powerclock_intenset_ctto((*byte >> 4) & 1)?;
        }
        Ok(())
    }
    fn read_powerclock_intenclr(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_powerclock_intenclr_pofwarn()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self
                .0
                .lock()
                .unwrap()
                .read_powerclock_intenclr_hfclkstarted()?
                << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self
                .0
                .lock()
                .unwrap()
                .read_powerclock_intenclr_lfclkstarted()?
                << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_powerclock_intenclr_done()? << 3;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_powerclock_intenclr_ctto()? << 4;
        }
        Ok(())
    }
    fn write_powerclock_intenclr(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_powerclock_intenclr_pofwarn((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_powerclock_intenclr_hfclkstarted((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_powerclock_intenclr_lfclkstarted((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_powerclock_intenclr_done((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_powerclock_intenclr_ctto((*byte >> 4) & 1)?;
        }
        Ok(())
    }
    fn read_power_resetreas(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_power_resetreas_resetpin()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_power_resetreas_dog()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_power_resetreas_sreq()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_power_resetreas_lockup()? << 3;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_power_resetreas_off()? << 0;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_power_resetreas_lpcomp()? << 1;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_power_resetreas_dif()? << 2;
        }
        Ok(())
    }
    fn write_power_resetreas(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_power_resetreas_resetpin((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_power_resetreas_dog((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_power_resetreas_sreq((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_power_resetreas_lockup((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_power_resetreas_off((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_power_resetreas_lpcomp((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_power_resetreas_dif((*byte >> 2) & 1)?;
        }
        Ok(())
    }
    fn read_power_ramstatus(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_power_ramstatus_ramblock0()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_power_ramstatus_ramblock1()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_power_ramstatus_ramblock2()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_power_ramstatus_ramblock3()? << 3;
        }
        Ok(())
    }
    fn write_power_systemoff(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_power_systemoff_systemoff((*byte >> 0) & 1)?;
        }
        Ok(())
    }
    fn read_power_pofcon(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_power_pofcon_pof()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_power_pofcon_threshold()? << 1;
        }
        Ok(())
    }
    fn write_power_pofcon(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_power_pofcon_pof((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_power_pofcon_threshold((*byte >> 1) & 3)?;
        }
        Ok(())
    }
    fn read_power_gpregret(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_power_gpregret_gpregret()? << 0;
        }
        Ok(())
    }
    fn write_power_gpregret(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_power_gpregret_gpregret((*byte >> 0) & 255)?;
        }
        Ok(())
    }
    fn read_power_ramon(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_power_ramon_onram0()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_power_ramon_onram1()? << 1;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_power_ramon_offram0()? << 0;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_power_ramon_offram1()? << 1;
        }
        Ok(())
    }
    fn write_power_ramon(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_power_ramon_onram0((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_power_ramon_onram1((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_power_ramon_offram0((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_power_ramon_offram1((*byte >> 1) & 1)?;
        }
        Ok(())
    }
    fn read_power_reset(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_power_reset_reset()? << 0;
        }
        Ok(())
    }
    fn write_power_reset(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_power_reset_reset((*byte >> 0) & 1)?;
        }
        Ok(())
    }
    fn read_power_ramonb(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_power_ramonb_onram2()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_power_ramonb_onram3()? << 1;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_power_ramonb_offram2()? << 0;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_power_ramonb_offram3()? << 1;
        }
        Ok(())
    }
    fn write_power_ramonb(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_power_ramonb_onram2((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_power_ramonb_onram3((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_power_ramonb_offram2((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_power_ramonb_offram3((*byte >> 1) & 1)?;
        }
        Ok(())
    }
    fn read_power_dcdcen(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_power_dcdcen_dcdcen()? << 0;
        }
        Ok(())
    }
    fn write_power_dcdcen(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_power_dcdcen_dcdcen((*byte >> 0) & 1)?;
        }
        Ok(())
    }
    fn read_power_dcdcforce(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_power_dcdcforce_forceoff()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_power_dcdcforce_forceon()? << 1;
        }
        Ok(())
    }
    fn write_power_dcdcforce(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_power_dcdcforce_forceoff((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_power_dcdcforce_forceon((*byte >> 1) & 1)?;
        }
        Ok(())
    }
    fn read_clock_hfclkrun(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_clock_hfclkrun_status()? << 0;
        }
        Ok(())
    }
    fn read_clock_hfclkstat(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_clock_hfclkstat_src()? << 0;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_clock_hfclkstat_state()? << 0;
        }
        Ok(())
    }
    fn read_clock_lfclkrun(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_clock_lfclkrun_status()? << 0;
        }
        Ok(())
    }
    fn read_clock_lfclkstat(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_clock_lfclkstat_src()? << 0;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_clock_lfclkstat_state()? << 0;
        }
        Ok(())
    }
    fn read_clock_lfclksrccopy(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_clock_lfclksrccopy_src()? << 0;
        }
        Ok(())
    }
    fn read_clock_lfclksrc(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_clock_lfclksrc_src()? << 0;
        }
        Ok(())
    }
    fn write_clock_lfclksrc(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_clock_lfclksrc_src((*byte >> 0) & 3)?;
        }
        Ok(())
    }
    fn read_clock_ctiv(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_clock_ctiv_ctiv()? << 0;
        }
        Ok(())
    }
    fn write_clock_ctiv(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_clock_ctiv_ctiv((*byte >> 0) & 127)?;
        }
        Ok(())
    }
    fn read_clock_xtalfreq(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 255;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 255;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 255;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_clock_xtalfreq_xtalfreq()? << 0;
        }
        Ok(())
    }
    fn write_clock_xtalfreq(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_clock_xtalfreq_xtalfreq((*byte >> 0) & 255)?;
        }
        Ok(())
    }
    fn read_mpu_perr0(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_mpu_perr0_power_clock()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_mpu_perr0_radio()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_mpu_perr0_uart0()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_mpu_perr0_spi0_twi0()? << 3;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_mpu_perr0_spi1_twi1()? << 4;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_mpu_perr0_gpiote()? << 6;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_mpu_perr0_adc()? << 7;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_mpu_perr0_timer0()? << 0;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_mpu_perr0_timer1()? << 1;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_mpu_perr0_timer2()? << 2;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_mpu_perr0_rtc0()? << 3;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_mpu_perr0_temp()? << 4;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_mpu_perr0_rng()? << 5;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_mpu_perr0_ecb()? << 6;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_mpu_perr0_ccm_aar()? << 7;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_mpu_perr0_wdt()? << 0;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_mpu_perr0_rtc1()? << 1;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_mpu_perr0_qdec()? << 2;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_mpu_perr0_lpcomp()? << 3;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_mpu_perr0_nvmc()? << 6;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_mpu_perr0_ppi()? << 7;
        }
        Ok(())
    }
    fn write_mpu_perr0(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_perr0_power_clock((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_perr0_radio((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_perr0_uart0((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_perr0_spi0_twi0((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_perr0_spi1_twi1((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_perr0_gpiote((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_perr0_adc((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_perr0_timer0((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_perr0_timer1((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_perr0_timer2((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_perr0_rtc0((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_perr0_temp((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_perr0_rng((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_perr0_ecb((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_perr0_ccm_aar((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_perr0_wdt((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_perr0_rtc1((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_perr0_qdec((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_perr0_lpcomp((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_perr0_nvmc((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_perr0_ppi((*byte >> 7) & 1)?;
        }
        Ok(())
    }
    fn read_mpu_protenset0(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset0_protreg0()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset0_protreg1()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset0_protreg2()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset0_protreg3()? << 3;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset0_protreg4()? << 4;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset0_protreg5()? << 5;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset0_protreg6()? << 6;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset0_protreg7()? << 7;
        }
        if let Some(byte) = _byte_1 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset0_protreg8()? << 0;
        }
        if let Some(byte) = _byte_1 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset0_protreg9()? << 1;
        }
        if let Some(byte) = _byte_1 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset0_protreg10()? << 2;
        }
        if let Some(byte) = _byte_1 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset0_protreg11()? << 3;
        }
        if let Some(byte) = _byte_1 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset0_protreg12()? << 4;
        }
        if let Some(byte) = _byte_1 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset0_protreg13()? << 5;
        }
        if let Some(byte) = _byte_1 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset0_protreg14()? << 6;
        }
        if let Some(byte) = _byte_1 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset0_protreg15()? << 7;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset0_protreg16()? << 0;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset0_protreg17()? << 1;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset0_protreg18()? << 2;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset0_protreg19()? << 3;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset0_protreg20()? << 4;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset0_protreg21()? << 5;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset0_protreg22()? << 6;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset0_protreg23()? << 7;
        }
        if let Some(byte) = _byte_3 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset0_protreg24()? << 0;
        }
        if let Some(byte) = _byte_3 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset0_protreg25()? << 1;
        }
        if let Some(byte) = _byte_3 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset0_protreg26()? << 2;
        }
        if let Some(byte) = _byte_3 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset0_protreg27()? << 3;
        }
        if let Some(byte) = _byte_3 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset0_protreg28()? << 4;
        }
        if let Some(byte) = _byte_3 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset0_protreg29()? << 5;
        }
        if let Some(byte) = _byte_3 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset0_protreg30()? << 6;
        }
        if let Some(byte) = _byte_3 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset0_protreg31()? << 7;
        }
        Ok(())
    }
    fn write_mpu_protenset0(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset0_protreg0((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset0_protreg1((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset0_protreg2((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset0_protreg3((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset0_protreg4((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset0_protreg5((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset0_protreg6((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset0_protreg7((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset0_protreg8((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset0_protreg9((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset0_protreg10((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset0_protreg11((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset0_protreg12((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset0_protreg13((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset0_protreg14((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset0_protreg15((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset0_protreg16((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset0_protreg17((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset0_protreg18((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset0_protreg19((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset0_protreg20((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset0_protreg21((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset0_protreg22((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset0_protreg23((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset0_protreg24((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset0_protreg25((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset0_protreg26((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset0_protreg27((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset0_protreg28((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset0_protreg29((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset0_protreg30((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset0_protreg31((*byte >> 7) & 1)?;
        }
        Ok(())
    }
    fn read_mpu_protenset1(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset1_protreg32()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset1_protreg33()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset1_protreg34()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset1_protreg35()? << 3;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset1_protreg36()? << 4;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset1_protreg37()? << 5;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset1_protreg38()? << 6;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset1_protreg39()? << 7;
        }
        if let Some(byte) = _byte_1 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset1_protreg40()? << 0;
        }
        if let Some(byte) = _byte_1 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset1_protreg41()? << 1;
        }
        if let Some(byte) = _byte_1 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset1_protreg42()? << 2;
        }
        if let Some(byte) = _byte_1 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset1_protreg43()? << 3;
        }
        if let Some(byte) = _byte_1 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset1_protreg44()? << 4;
        }
        if let Some(byte) = _byte_1 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset1_protreg45()? << 5;
        }
        if let Some(byte) = _byte_1 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset1_protreg46()? << 6;
        }
        if let Some(byte) = _byte_1 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset1_protreg47()? << 7;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset1_protreg48()? << 0;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset1_protreg49()? << 1;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset1_protreg50()? << 2;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset1_protreg51()? << 3;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset1_protreg52()? << 4;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset1_protreg53()? << 5;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset1_protreg54()? << 6;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset1_protreg55()? << 7;
        }
        if let Some(byte) = _byte_3 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset1_protreg56()? << 0;
        }
        if let Some(byte) = _byte_3 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset1_protreg57()? << 1;
        }
        if let Some(byte) = _byte_3 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset1_protreg58()? << 2;
        }
        if let Some(byte) = _byte_3 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset1_protreg59()? << 3;
        }
        if let Some(byte) = _byte_3 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset1_protreg60()? << 4;
        }
        if let Some(byte) = _byte_3 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset1_protreg61()? << 5;
        }
        if let Some(byte) = _byte_3 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset1_protreg62()? << 6;
        }
        if let Some(byte) = _byte_3 {
            **byte |=
                self.0.lock().unwrap().read_mpu_protenset1_protreg63()? << 7;
        }
        Ok(())
    }
    fn write_mpu_protenset1(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset1_protreg32((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset1_protreg33((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset1_protreg34((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset1_protreg35((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset1_protreg36((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset1_protreg37((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset1_protreg38((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset1_protreg39((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset1_protreg40((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset1_protreg41((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset1_protreg42((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset1_protreg43((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset1_protreg44((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset1_protreg45((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset1_protreg46((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset1_protreg47((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset1_protreg48((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset1_protreg49((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset1_protreg50((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset1_protreg51((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset1_protreg52((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset1_protreg53((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset1_protreg54((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset1_protreg55((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset1_protreg56((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset1_protreg57((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset1_protreg58((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset1_protreg59((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset1_protreg60((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset1_protreg61((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset1_protreg62((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protenset1_protreg63((*byte >> 7) & 1)?;
        }
        Ok(())
    }
    fn read_mpu_disableindebug(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self
                .0
                .lock()
                .unwrap()
                .read_mpu_disableindebug_disableindebug()?
                << 0;
        }
        Ok(())
    }
    fn write_mpu_disableindebug(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_disableindebug_disableindebug((*byte >> 0) & 1)?;
        }
        Ok(())
    }
    fn read_mpu_protblocksize(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self
                .0
                .lock()
                .unwrap()
                .read_mpu_protblocksize_protblocksize()?
                << 0;
        }
        Ok(())
    }
    fn write_mpu_protblocksize(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_mpu_protblocksize_protblocksize((*byte >> 0) & 3)?;
        }
        Ok(())
    }
}
pub struct PeripheralPage0x40001000(
    pub std::sync::Arc<std::sync::Mutex<super::peripheral::Peripherals>>,
);
impl icicle_vm::cpu::mem::IoMemory for PeripheralPage0x40001000 {
    fn read(
        &mut self,
        _addr: u64,
        _buf: &mut [u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073745920;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=35, 1..=36) => {
                if (_start < 4) || (_end <= 4) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 4 && _start < 8) || (_end > 4 && _end <= 8) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 8 && _start < 12) || (_end > 8 && _end <= 12) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 12 && _start < 16) || (_end > 12 && _end <= 16) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 16 && _start < 20) || (_end > 16 && _end <= 20) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 20 && _start < 24) || (_end > 20 && _end <= 24) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 24 && _start < 28) || (_end > 24 && _end <= 28) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 28 && _start < 32) || (_end > 28 && _end <= 32) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 32 && _start < 36) || (_end > 32 && _end <= 36) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (256..=287, 257..=288) => {
                if (_start >= 256 && _start < 260)
                    || (_end > 256 && _end <= 260)
                {
                    self.0.lock().unwrap().read_radio_events_ready(
                        &mut buffer_mut(_start, _end, 256, _buf),
                        &mut buffer_mut(_start, _end, 257, _buf),
                        &mut buffer_mut(_start, _end, 258, _buf),
                        &mut buffer_mut(_start, _end, 259, _buf),
                    )?;
                }
                if (_start >= 260 && _start < 264)
                    || (_end > 260 && _end <= 264)
                {
                    self.0.lock().unwrap().read_radio_events_address(
                        &mut buffer_mut(_start, _end, 260, _buf),
                        &mut buffer_mut(_start, _end, 261, _buf),
                        &mut buffer_mut(_start, _end, 262, _buf),
                        &mut buffer_mut(_start, _end, 263, _buf),
                    )?;
                }
                if (_start >= 264 && _start < 268)
                    || (_end > 264 && _end <= 268)
                {
                    self.0.lock().unwrap().read_radio_events_payload(
                        &mut buffer_mut(_start, _end, 264, _buf),
                        &mut buffer_mut(_start, _end, 265, _buf),
                        &mut buffer_mut(_start, _end, 266, _buf),
                        &mut buffer_mut(_start, _end, 267, _buf),
                    )?;
                }
                if (_start >= 268 && _start < 272)
                    || (_end > 268 && _end <= 272)
                {
                    self.0.lock().unwrap().read_radio_events_end(
                        &mut buffer_mut(_start, _end, 268, _buf),
                        &mut buffer_mut(_start, _end, 269, _buf),
                        &mut buffer_mut(_start, _end, 270, _buf),
                        &mut buffer_mut(_start, _end, 271, _buf),
                    )?;
                }
                if (_start >= 272 && _start < 276)
                    || (_end > 272 && _end <= 276)
                {
                    self.0.lock().unwrap().read_radio_events_disabled(
                        &mut buffer_mut(_start, _end, 272, _buf),
                        &mut buffer_mut(_start, _end, 273, _buf),
                        &mut buffer_mut(_start, _end, 274, _buf),
                        &mut buffer_mut(_start, _end, 275, _buf),
                    )?;
                }
                if (_start >= 276 && _start < 280)
                    || (_end > 276 && _end <= 280)
                {
                    self.0.lock().unwrap().read_radio_events_devmatch(
                        &mut buffer_mut(_start, _end, 276, _buf),
                        &mut buffer_mut(_start, _end, 277, _buf),
                        &mut buffer_mut(_start, _end, 278, _buf),
                        &mut buffer_mut(_start, _end, 279, _buf),
                    )?;
                }
                if (_start >= 280 && _start < 284)
                    || (_end > 280 && _end <= 284)
                {
                    self.0.lock().unwrap().read_radio_events_devmiss(
                        &mut buffer_mut(_start, _end, 280, _buf),
                        &mut buffer_mut(_start, _end, 281, _buf),
                        &mut buffer_mut(_start, _end, 282, _buf),
                        &mut buffer_mut(_start, _end, 283, _buf),
                    )?;
                }
                if (_start >= 284 && _start < 288)
                    || (_end > 284 && _end <= 288)
                {
                    self.0.lock().unwrap().read_radio_events_rssiend(
                        &mut buffer_mut(_start, _end, 284, _buf),
                        &mut buffer_mut(_start, _end, 285, _buf),
                        &mut buffer_mut(_start, _end, 286, _buf),
                        &mut buffer_mut(_start, _end, 287, _buf),
                    )?;
                }
            }
            (296..=299, 297..=300) => {
                if (_start >= 296 && _start < 300)
                    || (_end > 296 && _end <= 300)
                {
                    self.0.lock().unwrap().read_radio_events_bcmatch(
                        &mut buffer_mut(_start, _end, 296, _buf),
                        &mut buffer_mut(_start, _end, 297, _buf),
                        &mut buffer_mut(_start, _end, 298, _buf),
                        &mut buffer_mut(_start, _end, 299, _buf),
                    )?;
                }
            }
            (512..=515, 513..=516) => {
                if (_start >= 512 && _start < 516)
                    || (_end > 512 && _end <= 516)
                {
                    self.read_radio_shorts(
                        &mut buffer_mut(_start, _end, 512, _buf),
                        &mut buffer_mut(_start, _end, 513, _buf),
                        &mut buffer_mut(_start, _end, 514, _buf),
                        &mut buffer_mut(_start, _end, 515, _buf),
                    )?;
                }
            }
            (772..=779, 773..=780) => {
                if (_start >= 772 && _start < 776)
                    || (_end > 772 && _end <= 776)
                {
                    self.read_radio_intenset(
                        &mut buffer_mut(_start, _end, 772, _buf),
                        &mut buffer_mut(_start, _end, 773, _buf),
                        &mut buffer_mut(_start, _end, 774, _buf),
                        &mut buffer_mut(_start, _end, 775, _buf),
                    )?;
                }
                if (_start >= 776 && _start < 780)
                    || (_end > 776 && _end <= 780)
                {
                    self.read_radio_intenclr(
                        &mut buffer_mut(_start, _end, 776, _buf),
                        &mut buffer_mut(_start, _end, 777, _buf),
                        &mut buffer_mut(_start, _end, 778, _buf),
                        &mut buffer_mut(_start, _end, 779, _buf),
                    )?;
                }
            }
            (1024..=1027, 1025..=1028) => {
                if (_start >= 1024 && _start < 1028)
                    || (_end > 1024 && _end <= 1028)
                {
                    self.read_radio_crcstatus(
                        &mut buffer_mut(_start, _end, 1024, _buf),
                        &mut buffer_mut(_start, _end, 1025, _buf),
                        &mut buffer_mut(_start, _end, 1026, _buf),
                        &mut buffer_mut(_start, _end, 1027, _buf),
                    )?;
                }
            }
            (1032..=1043, 1033..=1044) => {
                if (_start >= 1032 && _start < 1036)
                    || (_end > 1032 && _end <= 1036)
                {
                    self.read_radio_rxmatch(
                        &mut buffer_mut(_start, _end, 1032, _buf),
                        &mut buffer_mut(_start, _end, 1033, _buf),
                        &mut buffer_mut(_start, _end, 1034, _buf),
                        &mut buffer_mut(_start, _end, 1035, _buf),
                    )?;
                }
                if (_start >= 1036 && _start < 1040)
                    || (_end > 1036 && _end <= 1040)
                {
                    self.read_radio_rxcrc(
                        &mut buffer_mut(_start, _end, 1036, _buf),
                        &mut buffer_mut(_start, _end, 1037, _buf),
                        &mut buffer_mut(_start, _end, 1038, _buf),
                        &mut buffer_mut(_start, _end, 1039, _buf),
                    )?;
                }
                if (_start >= 1040 && _start < 1044)
                    || (_end > 1040 && _end <= 1044)
                {
                    self.read_radio_dai(
                        &mut buffer_mut(_start, _end, 1040, _buf),
                        &mut buffer_mut(_start, _end, 1041, _buf),
                        &mut buffer_mut(_start, _end, 1042, _buf),
                        &mut buffer_mut(_start, _end, 1043, _buf),
                    )?;
                }
            }
            (1284..=1355, 1285..=1356) => {
                if (_start >= 1284 && _start < 1288)
                    || (_end > 1284 && _end <= 1288)
                {
                    self.0.lock().unwrap().read_radio_packetptr(
                        &mut buffer_mut(_start, _end, 1284, _buf),
                        &mut buffer_mut(_start, _end, 1285, _buf),
                        &mut buffer_mut(_start, _end, 1286, _buf),
                        &mut buffer_mut(_start, _end, 1287, _buf),
                    )?;
                }
                if (_start >= 1288 && _start < 1292)
                    || (_end > 1288 && _end <= 1292)
                {
                    self.read_radio_frequency(
                        &mut buffer_mut(_start, _end, 1288, _buf),
                        &mut buffer_mut(_start, _end, 1289, _buf),
                        &mut buffer_mut(_start, _end, 1290, _buf),
                        &mut buffer_mut(_start, _end, 1291, _buf),
                    )?;
                }
                if (_start >= 1292 && _start < 1296)
                    || (_end > 1292 && _end <= 1296)
                {
                    self.read_radio_txpower(
                        &mut buffer_mut(_start, _end, 1292, _buf),
                        &mut buffer_mut(_start, _end, 1293, _buf),
                        &mut buffer_mut(_start, _end, 1294, _buf),
                        &mut buffer_mut(_start, _end, 1295, _buf),
                    )?;
                }
                if (_start >= 1296 && _start < 1300)
                    || (_end > 1296 && _end <= 1300)
                {
                    self.read_radio_mode(
                        &mut buffer_mut(_start, _end, 1296, _buf),
                        &mut buffer_mut(_start, _end, 1297, _buf),
                        &mut buffer_mut(_start, _end, 1298, _buf),
                        &mut buffer_mut(_start, _end, 1299, _buf),
                    )?;
                }
                if (_start >= 1300 && _start < 1304)
                    || (_end > 1300 && _end <= 1304)
                {
                    self.read_radio_pcnf0(
                        &mut buffer_mut(_start, _end, 1300, _buf),
                        &mut buffer_mut(_start, _end, 1301, _buf),
                        &mut buffer_mut(_start, _end, 1302, _buf),
                        &mut buffer_mut(_start, _end, 1303, _buf),
                    )?;
                }
                if (_start >= 1304 && _start < 1308)
                    || (_end > 1304 && _end <= 1308)
                {
                    self.read_radio_pcnf1(
                        &mut buffer_mut(_start, _end, 1304, _buf),
                        &mut buffer_mut(_start, _end, 1305, _buf),
                        &mut buffer_mut(_start, _end, 1306, _buf),
                        &mut buffer_mut(_start, _end, 1307, _buf),
                    )?;
                }
                if (_start >= 1308 && _start < 1312)
                    || (_end > 1308 && _end <= 1312)
                {
                    self.0.lock().unwrap().read_radio_base0(
                        &mut buffer_mut(_start, _end, 1308, _buf),
                        &mut buffer_mut(_start, _end, 1309, _buf),
                        &mut buffer_mut(_start, _end, 1310, _buf),
                        &mut buffer_mut(_start, _end, 1311, _buf),
                    )?;
                }
                if (_start >= 1312 && _start < 1316)
                    || (_end > 1312 && _end <= 1316)
                {
                    self.0.lock().unwrap().read_radio_base1(
                        &mut buffer_mut(_start, _end, 1312, _buf),
                        &mut buffer_mut(_start, _end, 1313, _buf),
                        &mut buffer_mut(_start, _end, 1314, _buf),
                        &mut buffer_mut(_start, _end, 1315, _buf),
                    )?;
                }
                if (_start >= 1316 && _start < 1320)
                    || (_end > 1316 && _end <= 1320)
                {
                    self.read_radio_prefix0(
                        &mut buffer_mut(_start, _end, 1316, _buf),
                        &mut buffer_mut(_start, _end, 1317, _buf),
                        &mut buffer_mut(_start, _end, 1318, _buf),
                        &mut buffer_mut(_start, _end, 1319, _buf),
                    )?;
                }
                if (_start >= 1320 && _start < 1324)
                    || (_end > 1320 && _end <= 1324)
                {
                    self.read_radio_prefix1(
                        &mut buffer_mut(_start, _end, 1320, _buf),
                        &mut buffer_mut(_start, _end, 1321, _buf),
                        &mut buffer_mut(_start, _end, 1322, _buf),
                        &mut buffer_mut(_start, _end, 1323, _buf),
                    )?;
                }
                if (_start >= 1324 && _start < 1328)
                    || (_end > 1324 && _end <= 1328)
                {
                    self.read_radio_txaddress(
                        &mut buffer_mut(_start, _end, 1324, _buf),
                        &mut buffer_mut(_start, _end, 1325, _buf),
                        &mut buffer_mut(_start, _end, 1326, _buf),
                        &mut buffer_mut(_start, _end, 1327, _buf),
                    )?;
                }
                if (_start >= 1328 && _start < 1332)
                    || (_end > 1328 && _end <= 1332)
                {
                    self.read_radio_rxaddresses(
                        &mut buffer_mut(_start, _end, 1328, _buf),
                        &mut buffer_mut(_start, _end, 1329, _buf),
                        &mut buffer_mut(_start, _end, 1330, _buf),
                        &mut buffer_mut(_start, _end, 1331, _buf),
                    )?;
                }
                if (_start >= 1332 && _start < 1336)
                    || (_end > 1332 && _end <= 1336)
                {
                    self.read_radio_crccnf(
                        &mut buffer_mut(_start, _end, 1332, _buf),
                        &mut buffer_mut(_start, _end, 1333, _buf),
                        &mut buffer_mut(_start, _end, 1334, _buf),
                        &mut buffer_mut(_start, _end, 1335, _buf),
                    )?;
                }
                if (_start >= 1336 && _start < 1340)
                    || (_end > 1336 && _end <= 1340)
                {
                    self.read_radio_crcpoly(
                        &mut buffer_mut(_start, _end, 1336, _buf),
                        &mut buffer_mut(_start, _end, 1337, _buf),
                        &mut buffer_mut(_start, _end, 1338, _buf),
                        &mut buffer_mut(_start, _end, 1339, _buf),
                    )?;
                }
                if (_start >= 1340 && _start < 1344)
                    || (_end > 1340 && _end <= 1344)
                {
                    self.read_radio_crcinit(
                        &mut buffer_mut(_start, _end, 1340, _buf),
                        &mut buffer_mut(_start, _end, 1341, _buf),
                        &mut buffer_mut(_start, _end, 1342, _buf),
                        &mut buffer_mut(_start, _end, 1343, _buf),
                    )?;
                }
                if (_start >= 1344 && _start < 1348)
                    || (_end > 1344 && _end <= 1348)
                {
                    self.read_radio_test(
                        &mut buffer_mut(_start, _end, 1344, _buf),
                        &mut buffer_mut(_start, _end, 1345, _buf),
                        &mut buffer_mut(_start, _end, 1346, _buf),
                        &mut buffer_mut(_start, _end, 1347, _buf),
                    )?;
                }
                if (_start >= 1348 && _start < 1352)
                    || (_end > 1348 && _end <= 1352)
                {
                    self.read_radio_tifs(
                        &mut buffer_mut(_start, _end, 1348, _buf),
                        &mut buffer_mut(_start, _end, 1349, _buf),
                        &mut buffer_mut(_start, _end, 1350, _buf),
                        &mut buffer_mut(_start, _end, 1351, _buf),
                    )?;
                }
                if (_start >= 1352 && _start < 1356)
                    || (_end > 1352 && _end <= 1356)
                {
                    self.read_radio_rssisample(
                        &mut buffer_mut(_start, _end, 1352, _buf),
                        &mut buffer_mut(_start, _end, 1353, _buf),
                        &mut buffer_mut(_start, _end, 1354, _buf),
                        &mut buffer_mut(_start, _end, 1355, _buf),
                    )?;
                }
            }
            (1360..=1367, 1361..=1368) => {
                if (_start >= 1360 && _start < 1364)
                    || (_end > 1360 && _end <= 1364)
                {
                    self.read_radio_state(
                        &mut buffer_mut(_start, _end, 1360, _buf),
                        &mut buffer_mut(_start, _end, 1361, _buf),
                        &mut buffer_mut(_start, _end, 1362, _buf),
                        &mut buffer_mut(_start, _end, 1363, _buf),
                    )?;
                }
                if (_start >= 1364 && _start < 1368)
                    || (_end > 1364 && _end <= 1368)
                {
                    self.read_radio_datawhiteiv(
                        &mut buffer_mut(_start, _end, 1364, _buf),
                        &mut buffer_mut(_start, _end, 1365, _buf),
                        &mut buffer_mut(_start, _end, 1366, _buf),
                        &mut buffer_mut(_start, _end, 1367, _buf),
                    )?;
                }
            }
            (1376..=1379, 1377..=1380) => {
                if (_start >= 1376 && _start < 1380)
                    || (_end > 1376 && _end <= 1380)
                {
                    self.0.lock().unwrap().read_radio_bcc(
                        &mut buffer_mut(_start, _end, 1376, _buf),
                        &mut buffer_mut(_start, _end, 1377, _buf),
                        &mut buffer_mut(_start, _end, 1378, _buf),
                        &mut buffer_mut(_start, _end, 1379, _buf),
                    )?;
                }
            }
            (1536..=1603, 1537..=1604) => {
                if (_start >= 1536 && _start < 1540)
                    || (_end > 1536 && _end <= 1540)
                {
                    self.0.lock().unwrap().read_radio_dabn(
                        0,
                        &mut buffer_mut(_start, _end, 1536, _buf),
                        &mut buffer_mut(_start, _end, 1537, _buf),
                        &mut buffer_mut(_start, _end, 1538, _buf),
                        &mut buffer_mut(_start, _end, 1539, _buf),
                    )?;
                }
                if (_start >= 1540 && _start < 1544)
                    || (_end > 1540 && _end <= 1544)
                {
                    self.0.lock().unwrap().read_radio_dabn(
                        1,
                        &mut buffer_mut(_start, _end, 1540, _buf),
                        &mut buffer_mut(_start, _end, 1541, _buf),
                        &mut buffer_mut(_start, _end, 1542, _buf),
                        &mut buffer_mut(_start, _end, 1543, _buf),
                    )?;
                }
                if (_start >= 1544 && _start < 1548)
                    || (_end > 1544 && _end <= 1548)
                {
                    self.0.lock().unwrap().read_radio_dabn(
                        2,
                        &mut buffer_mut(_start, _end, 1544, _buf),
                        &mut buffer_mut(_start, _end, 1545, _buf),
                        &mut buffer_mut(_start, _end, 1546, _buf),
                        &mut buffer_mut(_start, _end, 1547, _buf),
                    )?;
                }
                if (_start >= 1548 && _start < 1552)
                    || (_end > 1548 && _end <= 1552)
                {
                    self.0.lock().unwrap().read_radio_dabn(
                        3,
                        &mut buffer_mut(_start, _end, 1548, _buf),
                        &mut buffer_mut(_start, _end, 1549, _buf),
                        &mut buffer_mut(_start, _end, 1550, _buf),
                        &mut buffer_mut(_start, _end, 1551, _buf),
                    )?;
                }
                if (_start >= 1552 && _start < 1556)
                    || (_end > 1552 && _end <= 1556)
                {
                    self.0.lock().unwrap().read_radio_dabn(
                        4,
                        &mut buffer_mut(_start, _end, 1552, _buf),
                        &mut buffer_mut(_start, _end, 1553, _buf),
                        &mut buffer_mut(_start, _end, 1554, _buf),
                        &mut buffer_mut(_start, _end, 1555, _buf),
                    )?;
                }
                if (_start >= 1556 && _start < 1560)
                    || (_end > 1556 && _end <= 1560)
                {
                    self.0.lock().unwrap().read_radio_dabn(
                        5,
                        &mut buffer_mut(_start, _end, 1556, _buf),
                        &mut buffer_mut(_start, _end, 1557, _buf),
                        &mut buffer_mut(_start, _end, 1558, _buf),
                        &mut buffer_mut(_start, _end, 1559, _buf),
                    )?;
                }
                if (_start >= 1560 && _start < 1564)
                    || (_end > 1560 && _end <= 1564)
                {
                    self.0.lock().unwrap().read_radio_dabn(
                        6,
                        &mut buffer_mut(_start, _end, 1560, _buf),
                        &mut buffer_mut(_start, _end, 1561, _buf),
                        &mut buffer_mut(_start, _end, 1562, _buf),
                        &mut buffer_mut(_start, _end, 1563, _buf),
                    )?;
                }
                if (_start >= 1564 && _start < 1568)
                    || (_end > 1564 && _end <= 1568)
                {
                    self.0.lock().unwrap().read_radio_dabn(
                        7,
                        &mut buffer_mut(_start, _end, 1564, _buf),
                        &mut buffer_mut(_start, _end, 1565, _buf),
                        &mut buffer_mut(_start, _end, 1566, _buf),
                        &mut buffer_mut(_start, _end, 1567, _buf),
                    )?;
                }
                if (_start >= 1568 && _start < 1572)
                    || (_end > 1568 && _end <= 1572)
                {
                    self.read_radio_dapn(
                        0,
                        &mut buffer_mut(_start, _end, 1568, _buf),
                        &mut buffer_mut(_start, _end, 1569, _buf),
                        &mut buffer_mut(_start, _end, 1570, _buf),
                        &mut buffer_mut(_start, _end, 1571, _buf),
                    )?;
                }
                if (_start >= 1572 && _start < 1576)
                    || (_end > 1572 && _end <= 1576)
                {
                    self.read_radio_dapn(
                        1,
                        &mut buffer_mut(_start, _end, 1572, _buf),
                        &mut buffer_mut(_start, _end, 1573, _buf),
                        &mut buffer_mut(_start, _end, 1574, _buf),
                        &mut buffer_mut(_start, _end, 1575, _buf),
                    )?;
                }
                if (_start >= 1576 && _start < 1580)
                    || (_end > 1576 && _end <= 1580)
                {
                    self.read_radio_dapn(
                        2,
                        &mut buffer_mut(_start, _end, 1576, _buf),
                        &mut buffer_mut(_start, _end, 1577, _buf),
                        &mut buffer_mut(_start, _end, 1578, _buf),
                        &mut buffer_mut(_start, _end, 1579, _buf),
                    )?;
                }
                if (_start >= 1580 && _start < 1584)
                    || (_end > 1580 && _end <= 1584)
                {
                    self.read_radio_dapn(
                        3,
                        &mut buffer_mut(_start, _end, 1580, _buf),
                        &mut buffer_mut(_start, _end, 1581, _buf),
                        &mut buffer_mut(_start, _end, 1582, _buf),
                        &mut buffer_mut(_start, _end, 1583, _buf),
                    )?;
                }
                if (_start >= 1584 && _start < 1588)
                    || (_end > 1584 && _end <= 1588)
                {
                    self.read_radio_dapn(
                        4,
                        &mut buffer_mut(_start, _end, 1584, _buf),
                        &mut buffer_mut(_start, _end, 1585, _buf),
                        &mut buffer_mut(_start, _end, 1586, _buf),
                        &mut buffer_mut(_start, _end, 1587, _buf),
                    )?;
                }
                if (_start >= 1588 && _start < 1592)
                    || (_end > 1588 && _end <= 1592)
                {
                    self.read_radio_dapn(
                        5,
                        &mut buffer_mut(_start, _end, 1588, _buf),
                        &mut buffer_mut(_start, _end, 1589, _buf),
                        &mut buffer_mut(_start, _end, 1590, _buf),
                        &mut buffer_mut(_start, _end, 1591, _buf),
                    )?;
                }
                if (_start >= 1592 && _start < 1596)
                    || (_end > 1592 && _end <= 1596)
                {
                    self.read_radio_dapn(
                        6,
                        &mut buffer_mut(_start, _end, 1592, _buf),
                        &mut buffer_mut(_start, _end, 1593, _buf),
                        &mut buffer_mut(_start, _end, 1594, _buf),
                        &mut buffer_mut(_start, _end, 1595, _buf),
                    )?;
                }
                if (_start >= 1596 && _start < 1600)
                    || (_end > 1596 && _end <= 1600)
                {
                    self.read_radio_dapn(
                        7,
                        &mut buffer_mut(_start, _end, 1596, _buf),
                        &mut buffer_mut(_start, _end, 1597, _buf),
                        &mut buffer_mut(_start, _end, 1598, _buf),
                        &mut buffer_mut(_start, _end, 1599, _buf),
                    )?;
                }
                if (_start >= 1600 && _start < 1604)
                    || (_end > 1600 && _end <= 1604)
                {
                    self.read_radio_dacnf(
                        &mut buffer_mut(_start, _end, 1600, _buf),
                        &mut buffer_mut(_start, _end, 1601, _buf),
                        &mut buffer_mut(_start, _end, 1602, _buf),
                        &mut buffer_mut(_start, _end, 1603, _buf),
                    )?;
                }
            }
            (1828..=1847, 1829..=1848) => {
                if (_start >= 1828 && _start < 1832)
                    || (_end > 1828 && _end <= 1832)
                {
                    self.read_radio_override0(
                        &mut buffer_mut(_start, _end, 1828, _buf),
                        &mut buffer_mut(_start, _end, 1829, _buf),
                        &mut buffer_mut(_start, _end, 1830, _buf),
                        &mut buffer_mut(_start, _end, 1831, _buf),
                    )?;
                }
                if (_start >= 1832 && _start < 1836)
                    || (_end > 1832 && _end <= 1836)
                {
                    self.read_radio_override1(
                        &mut buffer_mut(_start, _end, 1832, _buf),
                        &mut buffer_mut(_start, _end, 1833, _buf),
                        &mut buffer_mut(_start, _end, 1834, _buf),
                        &mut buffer_mut(_start, _end, 1835, _buf),
                    )?;
                }
                if (_start >= 1836 && _start < 1840)
                    || (_end > 1836 && _end <= 1840)
                {
                    self.read_radio_override2(
                        &mut buffer_mut(_start, _end, 1836, _buf),
                        &mut buffer_mut(_start, _end, 1837, _buf),
                        &mut buffer_mut(_start, _end, 1838, _buf),
                        &mut buffer_mut(_start, _end, 1839, _buf),
                    )?;
                }
                if (_start >= 1840 && _start < 1844)
                    || (_end > 1840 && _end <= 1844)
                {
                    self.read_radio_override3(
                        &mut buffer_mut(_start, _end, 1840, _buf),
                        &mut buffer_mut(_start, _end, 1841, _buf),
                        &mut buffer_mut(_start, _end, 1842, _buf),
                        &mut buffer_mut(_start, _end, 1843, _buf),
                    )?;
                }
                if (_start >= 1844 && _start < 1848)
                    || (_end > 1844 && _end <= 1848)
                {
                    self.read_radio_override4(
                        &mut buffer_mut(_start, _end, 1844, _buf),
                        &mut buffer_mut(_start, _end, 1845, _buf),
                        &mut buffer_mut(_start, _end, 1846, _buf),
                        &mut buffer_mut(_start, _end, 1847, _buf),
                    )?;
                }
            }
            (4092..=4095, 4093..=4096) => {
                if (_start >= 4092 && _start < 4096)
                    || (_end > 4092 && _end <= 4096)
                {
                    self.read_radio_power(
                        &mut buffer_mut(_start, _end, 4092, _buf),
                        &mut buffer_mut(_start, _end, 4093, _buf),
                        &mut buffer_mut(_start, _end, 4094, _buf),
                        &mut buffer_mut(_start, _end, 4095, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
    fn write(
        &mut self,
        _addr: u64,
        _buf: &[u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073745920;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=35, 1..=36) => {
                if (_start < 4) || (_end <= 4) {
                    self.0.lock().unwrap().write_radio_tasks_txen(
                        buffer_const(_start, _end, 0, _buf),
                        buffer_const(_start, _end, 1, _buf),
                        buffer_const(_start, _end, 2, _buf),
                        buffer_const(_start, _end, 3, _buf),
                    )?;
                }
                if (_start >= 4 && _start < 8) || (_end > 4 && _end <= 8) {
                    self.0.lock().unwrap().write_radio_tasks_rxen(
                        buffer_const(_start, _end, 4, _buf),
                        buffer_const(_start, _end, 5, _buf),
                        buffer_const(_start, _end, 6, _buf),
                        buffer_const(_start, _end, 7, _buf),
                    )?;
                }
                if (_start >= 8 && _start < 12) || (_end > 8 && _end <= 12) {
                    self.0.lock().unwrap().write_radio_tasks_start(
                        buffer_const(_start, _end, 8, _buf),
                        buffer_const(_start, _end, 9, _buf),
                        buffer_const(_start, _end, 10, _buf),
                        buffer_const(_start, _end, 11, _buf),
                    )?;
                }
                if (_start >= 12 && _start < 16) || (_end > 12 && _end <= 16) {
                    self.0.lock().unwrap().write_radio_tasks_stop(
                        buffer_const(_start, _end, 12, _buf),
                        buffer_const(_start, _end, 13, _buf),
                        buffer_const(_start, _end, 14, _buf),
                        buffer_const(_start, _end, 15, _buf),
                    )?;
                }
                if (_start >= 16 && _start < 20) || (_end > 16 && _end <= 20) {
                    self.0.lock().unwrap().write_radio_tasks_disable(
                        buffer_const(_start, _end, 16, _buf),
                        buffer_const(_start, _end, 17, _buf),
                        buffer_const(_start, _end, 18, _buf),
                        buffer_const(_start, _end, 19, _buf),
                    )?;
                }
                if (_start >= 20 && _start < 24) || (_end > 20 && _end <= 24) {
                    self.0.lock().unwrap().write_radio_tasks_rssistart(
                        buffer_const(_start, _end, 20, _buf),
                        buffer_const(_start, _end, 21, _buf),
                        buffer_const(_start, _end, 22, _buf),
                        buffer_const(_start, _end, 23, _buf),
                    )?;
                }
                if (_start >= 24 && _start < 28) || (_end > 24 && _end <= 28) {
                    self.0.lock().unwrap().write_radio_tasks_rssistop(
                        buffer_const(_start, _end, 24, _buf),
                        buffer_const(_start, _end, 25, _buf),
                        buffer_const(_start, _end, 26, _buf),
                        buffer_const(_start, _end, 27, _buf),
                    )?;
                }
                if (_start >= 28 && _start < 32) || (_end > 28 && _end <= 32) {
                    self.0.lock().unwrap().write_radio_tasks_bcstart(
                        buffer_const(_start, _end, 28, _buf),
                        buffer_const(_start, _end, 29, _buf),
                        buffer_const(_start, _end, 30, _buf),
                        buffer_const(_start, _end, 31, _buf),
                    )?;
                }
                if (_start >= 32 && _start < 36) || (_end > 32 && _end <= 36) {
                    self.0.lock().unwrap().write_radio_tasks_bcstop(
                        buffer_const(_start, _end, 32, _buf),
                        buffer_const(_start, _end, 33, _buf),
                        buffer_const(_start, _end, 34, _buf),
                        buffer_const(_start, _end, 35, _buf),
                    )?;
                }
            }
            (256..=287, 257..=288) => {
                if (_start >= 256 && _start < 260)
                    || (_end > 256 && _end <= 260)
                {
                    self.0.lock().unwrap().write_radio_events_ready(
                        buffer_const(_start, _end, 256, _buf),
                        buffer_const(_start, _end, 257, _buf),
                        buffer_const(_start, _end, 258, _buf),
                        buffer_const(_start, _end, 259, _buf),
                    )?;
                }
                if (_start >= 260 && _start < 264)
                    || (_end > 260 && _end <= 264)
                {
                    self.0.lock().unwrap().write_radio_events_address(
                        buffer_const(_start, _end, 260, _buf),
                        buffer_const(_start, _end, 261, _buf),
                        buffer_const(_start, _end, 262, _buf),
                        buffer_const(_start, _end, 263, _buf),
                    )?;
                }
                if (_start >= 264 && _start < 268)
                    || (_end > 264 && _end <= 268)
                {
                    self.0.lock().unwrap().write_radio_events_payload(
                        buffer_const(_start, _end, 264, _buf),
                        buffer_const(_start, _end, 265, _buf),
                        buffer_const(_start, _end, 266, _buf),
                        buffer_const(_start, _end, 267, _buf),
                    )?;
                }
                if (_start >= 268 && _start < 272)
                    || (_end > 268 && _end <= 272)
                {
                    self.0.lock().unwrap().write_radio_events_end(
                        buffer_const(_start, _end, 268, _buf),
                        buffer_const(_start, _end, 269, _buf),
                        buffer_const(_start, _end, 270, _buf),
                        buffer_const(_start, _end, 271, _buf),
                    )?;
                }
                if (_start >= 272 && _start < 276)
                    || (_end > 272 && _end <= 276)
                {
                    self.0.lock().unwrap().write_radio_events_disabled(
                        buffer_const(_start, _end, 272, _buf),
                        buffer_const(_start, _end, 273, _buf),
                        buffer_const(_start, _end, 274, _buf),
                        buffer_const(_start, _end, 275, _buf),
                    )?;
                }
                if (_start >= 276 && _start < 280)
                    || (_end > 276 && _end <= 280)
                {
                    self.0.lock().unwrap().write_radio_events_devmatch(
                        buffer_const(_start, _end, 276, _buf),
                        buffer_const(_start, _end, 277, _buf),
                        buffer_const(_start, _end, 278, _buf),
                        buffer_const(_start, _end, 279, _buf),
                    )?;
                }
                if (_start >= 280 && _start < 284)
                    || (_end > 280 && _end <= 284)
                {
                    self.0.lock().unwrap().write_radio_events_devmiss(
                        buffer_const(_start, _end, 280, _buf),
                        buffer_const(_start, _end, 281, _buf),
                        buffer_const(_start, _end, 282, _buf),
                        buffer_const(_start, _end, 283, _buf),
                    )?;
                }
                if (_start >= 284 && _start < 288)
                    || (_end > 284 && _end <= 288)
                {
                    self.0.lock().unwrap().write_radio_events_rssiend(
                        buffer_const(_start, _end, 284, _buf),
                        buffer_const(_start, _end, 285, _buf),
                        buffer_const(_start, _end, 286, _buf),
                        buffer_const(_start, _end, 287, _buf),
                    )?;
                }
            }
            (296..=299, 297..=300) => {
                if (_start >= 296 && _start < 300)
                    || (_end > 296 && _end <= 300)
                {
                    self.0.lock().unwrap().write_radio_events_bcmatch(
                        buffer_const(_start, _end, 296, _buf),
                        buffer_const(_start, _end, 297, _buf),
                        buffer_const(_start, _end, 298, _buf),
                        buffer_const(_start, _end, 299, _buf),
                    )?;
                }
            }
            (512..=515, 513..=516) => {
                if (_start >= 512 && _start < 516)
                    || (_end > 512 && _end <= 516)
                {
                    self.write_radio_shorts(
                        buffer_const(_start, _end, 512, _buf),
                        buffer_const(_start, _end, 513, _buf),
                        buffer_const(_start, _end, 514, _buf),
                        buffer_const(_start, _end, 515, _buf),
                    )?;
                }
            }
            (772..=779, 773..=780) => {
                if (_start >= 772 && _start < 776)
                    || (_end > 772 && _end <= 776)
                {
                    self.write_radio_intenset(
                        buffer_const(_start, _end, 772, _buf),
                        buffer_const(_start, _end, 773, _buf),
                        buffer_const(_start, _end, 774, _buf),
                        buffer_const(_start, _end, 775, _buf),
                    )?;
                }
                if (_start >= 776 && _start < 780)
                    || (_end > 776 && _end <= 780)
                {
                    self.write_radio_intenclr(
                        buffer_const(_start, _end, 776, _buf),
                        buffer_const(_start, _end, 777, _buf),
                        buffer_const(_start, _end, 778, _buf),
                        buffer_const(_start, _end, 779, _buf),
                    )?;
                }
            }
            (1024..=1027, 1025..=1028) => {
                if (_start >= 1024 && _start < 1028)
                    || (_end > 1024 && _end <= 1028)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (1032..=1043, 1033..=1044) => {
                if (_start >= 1032 && _start < 1036)
                    || (_end > 1032 && _end <= 1036)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 1036 && _start < 1040)
                    || (_end > 1036 && _end <= 1040)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 1040 && _start < 1044)
                    || (_end > 1040 && _end <= 1044)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (1284..=1355, 1285..=1356) => {
                if (_start >= 1284 && _start < 1288)
                    || (_end > 1284 && _end <= 1288)
                {
                    self.0.lock().unwrap().write_radio_packetptr(
                        buffer_const(_start, _end, 1284, _buf),
                        buffer_const(_start, _end, 1285, _buf),
                        buffer_const(_start, _end, 1286, _buf),
                        buffer_const(_start, _end, 1287, _buf),
                    )?;
                }
                if (_start >= 1288 && _start < 1292)
                    || (_end > 1288 && _end <= 1292)
                {
                    self.write_radio_frequency(
                        buffer_const(_start, _end, 1288, _buf),
                        buffer_const(_start, _end, 1289, _buf),
                        buffer_const(_start, _end, 1290, _buf),
                        buffer_const(_start, _end, 1291, _buf),
                    )?;
                }
                if (_start >= 1292 && _start < 1296)
                    || (_end > 1292 && _end <= 1296)
                {
                    self.write_radio_txpower(
                        buffer_const(_start, _end, 1292, _buf),
                        buffer_const(_start, _end, 1293, _buf),
                        buffer_const(_start, _end, 1294, _buf),
                        buffer_const(_start, _end, 1295, _buf),
                    )?;
                }
                if (_start >= 1296 && _start < 1300)
                    || (_end > 1296 && _end <= 1300)
                {
                    self.write_radio_mode(
                        buffer_const(_start, _end, 1296, _buf),
                        buffer_const(_start, _end, 1297, _buf),
                        buffer_const(_start, _end, 1298, _buf),
                        buffer_const(_start, _end, 1299, _buf),
                    )?;
                }
                if (_start >= 1300 && _start < 1304)
                    || (_end > 1300 && _end <= 1304)
                {
                    self.write_radio_pcnf0(
                        buffer_const(_start, _end, 1300, _buf),
                        buffer_const(_start, _end, 1301, _buf),
                        buffer_const(_start, _end, 1302, _buf),
                        buffer_const(_start, _end, 1303, _buf),
                    )?;
                }
                if (_start >= 1304 && _start < 1308)
                    || (_end > 1304 && _end <= 1308)
                {
                    self.write_radio_pcnf1(
                        buffer_const(_start, _end, 1304, _buf),
                        buffer_const(_start, _end, 1305, _buf),
                        buffer_const(_start, _end, 1306, _buf),
                        buffer_const(_start, _end, 1307, _buf),
                    )?;
                }
                if (_start >= 1308 && _start < 1312)
                    || (_end > 1308 && _end <= 1312)
                {
                    self.0.lock().unwrap().write_radio_base0(
                        buffer_const(_start, _end, 1308, _buf),
                        buffer_const(_start, _end, 1309, _buf),
                        buffer_const(_start, _end, 1310, _buf),
                        buffer_const(_start, _end, 1311, _buf),
                    )?;
                }
                if (_start >= 1312 && _start < 1316)
                    || (_end > 1312 && _end <= 1316)
                {
                    self.0.lock().unwrap().write_radio_base1(
                        buffer_const(_start, _end, 1312, _buf),
                        buffer_const(_start, _end, 1313, _buf),
                        buffer_const(_start, _end, 1314, _buf),
                        buffer_const(_start, _end, 1315, _buf),
                    )?;
                }
                if (_start >= 1316 && _start < 1320)
                    || (_end > 1316 && _end <= 1320)
                {
                    self.write_radio_prefix0(
                        buffer_const(_start, _end, 1316, _buf),
                        buffer_const(_start, _end, 1317, _buf),
                        buffer_const(_start, _end, 1318, _buf),
                        buffer_const(_start, _end, 1319, _buf),
                    )?;
                }
                if (_start >= 1320 && _start < 1324)
                    || (_end > 1320 && _end <= 1324)
                {
                    self.write_radio_prefix1(
                        buffer_const(_start, _end, 1320, _buf),
                        buffer_const(_start, _end, 1321, _buf),
                        buffer_const(_start, _end, 1322, _buf),
                        buffer_const(_start, _end, 1323, _buf),
                    )?;
                }
                if (_start >= 1324 && _start < 1328)
                    || (_end > 1324 && _end <= 1328)
                {
                    self.write_radio_txaddress(
                        buffer_const(_start, _end, 1324, _buf),
                        buffer_const(_start, _end, 1325, _buf),
                        buffer_const(_start, _end, 1326, _buf),
                        buffer_const(_start, _end, 1327, _buf),
                    )?;
                }
                if (_start >= 1328 && _start < 1332)
                    || (_end > 1328 && _end <= 1332)
                {
                    self.write_radio_rxaddresses(
                        buffer_const(_start, _end, 1328, _buf),
                        buffer_const(_start, _end, 1329, _buf),
                        buffer_const(_start, _end, 1330, _buf),
                        buffer_const(_start, _end, 1331, _buf),
                    )?;
                }
                if (_start >= 1332 && _start < 1336)
                    || (_end > 1332 && _end <= 1336)
                {
                    self.write_radio_crccnf(
                        buffer_const(_start, _end, 1332, _buf),
                        buffer_const(_start, _end, 1333, _buf),
                        buffer_const(_start, _end, 1334, _buf),
                        buffer_const(_start, _end, 1335, _buf),
                    )?;
                }
                if (_start >= 1336 && _start < 1340)
                    || (_end > 1336 && _end <= 1340)
                {
                    self.write_radio_crcpoly(
                        buffer_const(_start, _end, 1336, _buf),
                        buffer_const(_start, _end, 1337, _buf),
                        buffer_const(_start, _end, 1338, _buf),
                        buffer_const(_start, _end, 1339, _buf),
                    )?;
                }
                if (_start >= 1340 && _start < 1344)
                    || (_end > 1340 && _end <= 1344)
                {
                    self.write_radio_crcinit(
                        buffer_const(_start, _end, 1340, _buf),
                        buffer_const(_start, _end, 1341, _buf),
                        buffer_const(_start, _end, 1342, _buf),
                        buffer_const(_start, _end, 1343, _buf),
                    )?;
                }
                if (_start >= 1344 && _start < 1348)
                    || (_end > 1344 && _end <= 1348)
                {
                    self.write_radio_test(
                        buffer_const(_start, _end, 1344, _buf),
                        buffer_const(_start, _end, 1345, _buf),
                        buffer_const(_start, _end, 1346, _buf),
                        buffer_const(_start, _end, 1347, _buf),
                    )?;
                }
                if (_start >= 1348 && _start < 1352)
                    || (_end > 1348 && _end <= 1352)
                {
                    self.write_radio_tifs(
                        buffer_const(_start, _end, 1348, _buf),
                        buffer_const(_start, _end, 1349, _buf),
                        buffer_const(_start, _end, 1350, _buf),
                        buffer_const(_start, _end, 1351, _buf),
                    )?;
                }
                if (_start >= 1352 && _start < 1356)
                    || (_end > 1352 && _end <= 1356)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (1360..=1367, 1361..=1368) => {
                if (_start >= 1360 && _start < 1364)
                    || (_end > 1360 && _end <= 1364)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 1364 && _start < 1368)
                    || (_end > 1364 && _end <= 1368)
                {
                    self.write_radio_datawhiteiv(
                        buffer_const(_start, _end, 1364, _buf),
                        buffer_const(_start, _end, 1365, _buf),
                        buffer_const(_start, _end, 1366, _buf),
                        buffer_const(_start, _end, 1367, _buf),
                    )?;
                }
            }
            (1376..=1379, 1377..=1380) => {
                if (_start >= 1376 && _start < 1380)
                    || (_end > 1376 && _end <= 1380)
                {
                    self.0.lock().unwrap().write_radio_bcc(
                        buffer_const(_start, _end, 1376, _buf),
                        buffer_const(_start, _end, 1377, _buf),
                        buffer_const(_start, _end, 1378, _buf),
                        buffer_const(_start, _end, 1379, _buf),
                    )?;
                }
            }
            (1536..=1603, 1537..=1604) => {
                if (_start >= 1536 && _start < 1540)
                    || (_end > 1536 && _end <= 1540)
                {
                    self.0.lock().unwrap().write_radio_dabn(
                        0,
                        buffer_const(_start, _end, 1536, _buf),
                        buffer_const(_start, _end, 1537, _buf),
                        buffer_const(_start, _end, 1538, _buf),
                        buffer_const(_start, _end, 1539, _buf),
                    )?;
                }
                if (_start >= 1540 && _start < 1544)
                    || (_end > 1540 && _end <= 1544)
                {
                    self.0.lock().unwrap().write_radio_dabn(
                        1,
                        buffer_const(_start, _end, 1540, _buf),
                        buffer_const(_start, _end, 1541, _buf),
                        buffer_const(_start, _end, 1542, _buf),
                        buffer_const(_start, _end, 1543, _buf),
                    )?;
                }
                if (_start >= 1544 && _start < 1548)
                    || (_end > 1544 && _end <= 1548)
                {
                    self.0.lock().unwrap().write_radio_dabn(
                        2,
                        buffer_const(_start, _end, 1544, _buf),
                        buffer_const(_start, _end, 1545, _buf),
                        buffer_const(_start, _end, 1546, _buf),
                        buffer_const(_start, _end, 1547, _buf),
                    )?;
                }
                if (_start >= 1548 && _start < 1552)
                    || (_end > 1548 && _end <= 1552)
                {
                    self.0.lock().unwrap().write_radio_dabn(
                        3,
                        buffer_const(_start, _end, 1548, _buf),
                        buffer_const(_start, _end, 1549, _buf),
                        buffer_const(_start, _end, 1550, _buf),
                        buffer_const(_start, _end, 1551, _buf),
                    )?;
                }
                if (_start >= 1552 && _start < 1556)
                    || (_end > 1552 && _end <= 1556)
                {
                    self.0.lock().unwrap().write_radio_dabn(
                        4,
                        buffer_const(_start, _end, 1552, _buf),
                        buffer_const(_start, _end, 1553, _buf),
                        buffer_const(_start, _end, 1554, _buf),
                        buffer_const(_start, _end, 1555, _buf),
                    )?;
                }
                if (_start >= 1556 && _start < 1560)
                    || (_end > 1556 && _end <= 1560)
                {
                    self.0.lock().unwrap().write_radio_dabn(
                        5,
                        buffer_const(_start, _end, 1556, _buf),
                        buffer_const(_start, _end, 1557, _buf),
                        buffer_const(_start, _end, 1558, _buf),
                        buffer_const(_start, _end, 1559, _buf),
                    )?;
                }
                if (_start >= 1560 && _start < 1564)
                    || (_end > 1560 && _end <= 1564)
                {
                    self.0.lock().unwrap().write_radio_dabn(
                        6,
                        buffer_const(_start, _end, 1560, _buf),
                        buffer_const(_start, _end, 1561, _buf),
                        buffer_const(_start, _end, 1562, _buf),
                        buffer_const(_start, _end, 1563, _buf),
                    )?;
                }
                if (_start >= 1564 && _start < 1568)
                    || (_end > 1564 && _end <= 1568)
                {
                    self.0.lock().unwrap().write_radio_dabn(
                        7,
                        buffer_const(_start, _end, 1564, _buf),
                        buffer_const(_start, _end, 1565, _buf),
                        buffer_const(_start, _end, 1566, _buf),
                        buffer_const(_start, _end, 1567, _buf),
                    )?;
                }
                if (_start >= 1568 && _start < 1572)
                    || (_end > 1568 && _end <= 1572)
                {
                    self.write_radio_dapn(
                        0,
                        buffer_const(_start, _end, 1568, _buf),
                        buffer_const(_start, _end, 1569, _buf),
                        buffer_const(_start, _end, 1570, _buf),
                        buffer_const(_start, _end, 1571, _buf),
                    )?;
                }
                if (_start >= 1572 && _start < 1576)
                    || (_end > 1572 && _end <= 1576)
                {
                    self.write_radio_dapn(
                        1,
                        buffer_const(_start, _end, 1572, _buf),
                        buffer_const(_start, _end, 1573, _buf),
                        buffer_const(_start, _end, 1574, _buf),
                        buffer_const(_start, _end, 1575, _buf),
                    )?;
                }
                if (_start >= 1576 && _start < 1580)
                    || (_end > 1576 && _end <= 1580)
                {
                    self.write_radio_dapn(
                        2,
                        buffer_const(_start, _end, 1576, _buf),
                        buffer_const(_start, _end, 1577, _buf),
                        buffer_const(_start, _end, 1578, _buf),
                        buffer_const(_start, _end, 1579, _buf),
                    )?;
                }
                if (_start >= 1580 && _start < 1584)
                    || (_end > 1580 && _end <= 1584)
                {
                    self.write_radio_dapn(
                        3,
                        buffer_const(_start, _end, 1580, _buf),
                        buffer_const(_start, _end, 1581, _buf),
                        buffer_const(_start, _end, 1582, _buf),
                        buffer_const(_start, _end, 1583, _buf),
                    )?;
                }
                if (_start >= 1584 && _start < 1588)
                    || (_end > 1584 && _end <= 1588)
                {
                    self.write_radio_dapn(
                        4,
                        buffer_const(_start, _end, 1584, _buf),
                        buffer_const(_start, _end, 1585, _buf),
                        buffer_const(_start, _end, 1586, _buf),
                        buffer_const(_start, _end, 1587, _buf),
                    )?;
                }
                if (_start >= 1588 && _start < 1592)
                    || (_end > 1588 && _end <= 1592)
                {
                    self.write_radio_dapn(
                        5,
                        buffer_const(_start, _end, 1588, _buf),
                        buffer_const(_start, _end, 1589, _buf),
                        buffer_const(_start, _end, 1590, _buf),
                        buffer_const(_start, _end, 1591, _buf),
                    )?;
                }
                if (_start >= 1592 && _start < 1596)
                    || (_end > 1592 && _end <= 1596)
                {
                    self.write_radio_dapn(
                        6,
                        buffer_const(_start, _end, 1592, _buf),
                        buffer_const(_start, _end, 1593, _buf),
                        buffer_const(_start, _end, 1594, _buf),
                        buffer_const(_start, _end, 1595, _buf),
                    )?;
                }
                if (_start >= 1596 && _start < 1600)
                    || (_end > 1596 && _end <= 1600)
                {
                    self.write_radio_dapn(
                        7,
                        buffer_const(_start, _end, 1596, _buf),
                        buffer_const(_start, _end, 1597, _buf),
                        buffer_const(_start, _end, 1598, _buf),
                        buffer_const(_start, _end, 1599, _buf),
                    )?;
                }
                if (_start >= 1600 && _start < 1604)
                    || (_end > 1600 && _end <= 1604)
                {
                    self.write_radio_dacnf(
                        buffer_const(_start, _end, 1600, _buf),
                        buffer_const(_start, _end, 1601, _buf),
                        buffer_const(_start, _end, 1602, _buf),
                        buffer_const(_start, _end, 1603, _buf),
                    )?;
                }
            }
            (1828..=1847, 1829..=1848) => {
                if (_start >= 1828 && _start < 1832)
                    || (_end > 1828 && _end <= 1832)
                {
                    self.write_radio_override0(
                        buffer_const(_start, _end, 1828, _buf),
                        buffer_const(_start, _end, 1829, _buf),
                        buffer_const(_start, _end, 1830, _buf),
                        buffer_const(_start, _end, 1831, _buf),
                    )?;
                }
                if (_start >= 1832 && _start < 1836)
                    || (_end > 1832 && _end <= 1836)
                {
                    self.write_radio_override1(
                        buffer_const(_start, _end, 1832, _buf),
                        buffer_const(_start, _end, 1833, _buf),
                        buffer_const(_start, _end, 1834, _buf),
                        buffer_const(_start, _end, 1835, _buf),
                    )?;
                }
                if (_start >= 1836 && _start < 1840)
                    || (_end > 1836 && _end <= 1840)
                {
                    self.write_radio_override2(
                        buffer_const(_start, _end, 1836, _buf),
                        buffer_const(_start, _end, 1837, _buf),
                        buffer_const(_start, _end, 1838, _buf),
                        buffer_const(_start, _end, 1839, _buf),
                    )?;
                }
                if (_start >= 1840 && _start < 1844)
                    || (_end > 1840 && _end <= 1844)
                {
                    self.write_radio_override3(
                        buffer_const(_start, _end, 1840, _buf),
                        buffer_const(_start, _end, 1841, _buf),
                        buffer_const(_start, _end, 1842, _buf),
                        buffer_const(_start, _end, 1843, _buf),
                    )?;
                }
                if (_start >= 1844 && _start < 1848)
                    || (_end > 1844 && _end <= 1848)
                {
                    self.write_radio_override4(
                        buffer_const(_start, _end, 1844, _buf),
                        buffer_const(_start, _end, 1845, _buf),
                        buffer_const(_start, _end, 1846, _buf),
                        buffer_const(_start, _end, 1847, _buf),
                    )?;
                }
            }
            (4092..=4095, 4093..=4096) => {
                if (_start >= 4092 && _start < 4096)
                    || (_end > 4092 && _end <= 4096)
                {
                    self.write_radio_power(
                        buffer_const(_start, _end, 4092, _buf),
                        buffer_const(_start, _end, 4093, _buf),
                        buffer_const(_start, _end, 4094, _buf),
                        buffer_const(_start, _end, 4095, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
}
impl PeripheralPage0x40001000 {
    fn read_radio_shorts(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_radio_shorts_ready_start()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_radio_shorts_end_disable()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_radio_shorts_disabled_txen()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_radio_shorts_disabled_rxen()? << 3;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self
                .0
                .lock()
                .unwrap()
                .read_radio_shorts_address_rssistart()?
                << 4;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_radio_shorts_end_start()? << 5;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_radio_shorts_address_bcstart()?
                    << 6;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self
                .0
                .lock()
                .unwrap()
                .read_radio_shorts_disabled_rssistop()?
                << 0;
        }
        Ok(())
    }
    fn write_radio_shorts(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_shorts_ready_start((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_shorts_end_disable((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_shorts_disabled_txen((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_shorts_disabled_rxen((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_shorts_address_rssistart((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_shorts_end_start((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_shorts_address_bcstart((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_radio_shorts_disabled_rssistop((*byte >> 0) & 1)?;
        }
        Ok(())
    }
    fn read_radio_intenset(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_radio_intenset_ready()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_radio_intenset_address()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_radio_intenset_payload()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_radio_intenset_end()? << 3;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_radio_intenset_disabled()? << 4;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_radio_intenset_devmatch()? << 5;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_radio_intenset_devmiss()? << 6;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_radio_intenset_rssiend()? << 7;
        }
        if let Some(byte) = _byte_1 {
            **byte |=
                self.0.lock().unwrap().read_radio_intenset_bcmatch()? << 2;
        }
        Ok(())
    }
    fn write_radio_intenset(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_intenset_ready((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_intenset_address((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_intenset_payload((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_intenset_end((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_intenset_disabled((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_intenset_devmatch((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_intenset_devmiss((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_intenset_rssiend((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_radio_intenset_bcmatch((*byte >> 2) & 1)?;
        }
        Ok(())
    }
    fn read_radio_intenclr(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_radio_intenclr_ready()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_radio_intenclr_address()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_radio_intenclr_payload()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_radio_intenclr_end()? << 3;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_radio_intenclr_disabled()? << 4;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_radio_intenclr_devmatch()? << 5;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_radio_intenclr_devmiss()? << 6;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_radio_intenclr_rssiend()? << 7;
        }
        if let Some(byte) = _byte_1 {
            **byte |=
                self.0.lock().unwrap().read_radio_intenclr_bcmatch()? << 2;
        }
        Ok(())
    }
    fn write_radio_intenclr(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_intenclr_ready((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_intenclr_address((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_intenclr_payload((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_intenclr_end((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_intenclr_disabled((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_intenclr_devmatch((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_intenclr_devmiss((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_intenclr_rssiend((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_radio_intenclr_bcmatch((*byte >> 2) & 1)?;
        }
        Ok(())
    }
    fn read_radio_crcstatus(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_radio_crcstatus_crcstatus()? << 0;
        }
        Ok(())
    }
    fn read_radio_rxmatch(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_radio_rxmatch_rxmatch()? << 0;
        }
        Ok(())
    }
    fn read_radio_rxcrc(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if _byte_0.is_some() || _byte_1.is_some() || _byte_2.is_some() {
            self.0
                .lock()
                .unwrap()
                .read_radio_rxcrc_rxcrc(_byte_0, _byte_1, _byte_2)?;
        }
        Ok(())
    }
    fn read_radio_dai(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_radio_dai_dai()? << 0;
        }
        Ok(())
    }
    fn read_radio_frequency(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_radio_frequency_frequency()? << 0;
        }
        Ok(())
    }
    fn write_radio_frequency(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_frequency_frequency((*byte >> 0) & 127)?;
        }
        Ok(())
    }
    fn read_radio_txpower(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_radio_txpower_txpower()? << 0;
        }
        Ok(())
    }
    fn write_radio_txpower(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_txpower_txpower((*byte >> 0) & 255)?;
        }
        Ok(())
    }
    fn read_radio_mode(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_radio_mode_mode()? << 0;
        }
        Ok(())
    }
    fn write_radio_mode(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_mode_mode((*byte >> 0) & 3)?;
        }
        Ok(())
    }
    fn read_radio_pcnf0(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_radio_pcnf0_lflen()? << 0;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_radio_pcnf0_s0len()? << 0;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_radio_pcnf0_s1len()? << 0;
        }
        Ok(())
    }
    fn write_radio_pcnf0(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_pcnf0_lflen((*byte >> 0) & 15)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_radio_pcnf0_s0len((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_radio_pcnf0_s1len((*byte >> 0) & 15)?;
        }
        Ok(())
    }
    fn read_radio_pcnf1(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_radio_pcnf1_maxlen()? << 0;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_radio_pcnf1_statlen()? << 0;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_radio_pcnf1_balen()? << 0;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_radio_pcnf1_endian()? << 0;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_radio_pcnf1_whiteen()? << 1;
        }
        Ok(())
    }
    fn write_radio_pcnf1(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_pcnf1_maxlen((*byte >> 0) & 255)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_radio_pcnf1_statlen((*byte >> 0) & 255)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_radio_pcnf1_balen((*byte >> 0) & 7)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_radio_pcnf1_endian((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_radio_pcnf1_whiteen((*byte >> 1) & 1)?;
        }
        Ok(())
    }
    fn read_radio_prefix0(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_radio_prefix0_ap0()? << 0;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_radio_prefix0_ap1()? << 0;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_radio_prefix0_ap2()? << 0;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_radio_prefix0_ap3()? << 0;
        }
        Ok(())
    }
    fn write_radio_prefix0(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_prefix0_ap0((*byte >> 0) & 255)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_radio_prefix0_ap1((*byte >> 0) & 255)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_radio_prefix0_ap2((*byte >> 0) & 255)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_radio_prefix0_ap3((*byte >> 0) & 255)?;
        }
        Ok(())
    }
    fn read_radio_prefix1(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_radio_prefix1_ap4()? << 0;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_radio_prefix1_ap5()? << 0;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_radio_prefix1_ap6()? << 0;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_radio_prefix1_ap7()? << 0;
        }
        Ok(())
    }
    fn write_radio_prefix1(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_prefix1_ap4((*byte >> 0) & 255)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_radio_prefix1_ap5((*byte >> 0) & 255)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_radio_prefix1_ap6((*byte >> 0) & 255)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_radio_prefix1_ap7((*byte >> 0) & 255)?;
        }
        Ok(())
    }
    fn read_radio_txaddress(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_radio_txaddress_txaddress()? << 0;
        }
        Ok(())
    }
    fn write_radio_txaddress(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_txaddress_txaddress((*byte >> 0) & 7)?;
        }
        Ok(())
    }
    fn read_radio_rxaddresses(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_radio_rxaddresses_addr0()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_radio_rxaddresses_addr1()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_radio_rxaddresses_addr2()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_radio_rxaddresses_addr3()? << 3;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_radio_rxaddresses_addr4()? << 4;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_radio_rxaddresses_addr5()? << 5;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_radio_rxaddresses_addr6()? << 6;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_radio_rxaddresses_addr7()? << 7;
        }
        Ok(())
    }
    fn write_radio_rxaddresses(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_rxaddresses_addr0((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_rxaddresses_addr1((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_rxaddresses_addr2((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_rxaddresses_addr3((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_rxaddresses_addr4((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_rxaddresses_addr5((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_rxaddresses_addr6((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_rxaddresses_addr7((*byte >> 7) & 1)?;
        }
        Ok(())
    }
    fn read_radio_crccnf(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_radio_crccnf_len()? << 0;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_radio_crccnf_skipaddr()? << 0;
        }
        Ok(())
    }
    fn write_radio_crccnf(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_crccnf_len((*byte >> 0) & 3)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_radio_crccnf_skipaddr((*byte >> 0) & 1)?;
        }
        Ok(())
    }
    fn read_radio_crcpoly(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if _byte_0.is_some() || _byte_1.is_some() || _byte_2.is_some() {
            self.0
                .lock()
                .unwrap()
                .read_radio_crcpoly_crcpoly(_byte_0, _byte_1, _byte_2)?;
        }
        Ok(())
    }
    fn write_radio_crcpoly(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if _byte_0.is_some() || _byte_1.is_some() || _byte_2.is_some() {
            self.0
                .lock()
                .unwrap()
                .write_radio_crcpoly_crcpoly(_byte_0, _byte_1, _byte_2)?;
        }
        Ok(())
    }
    fn read_radio_crcinit(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if _byte_0.is_some() || _byte_1.is_some() || _byte_2.is_some() {
            self.0
                .lock()
                .unwrap()
                .read_radio_crcinit_crcinit(_byte_0, _byte_1, _byte_2)?;
        }
        Ok(())
    }
    fn write_radio_crcinit(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if _byte_0.is_some() || _byte_1.is_some() || _byte_2.is_some() {
            self.0
                .lock()
                .unwrap()
                .write_radio_crcinit_crcinit(_byte_0, _byte_1, _byte_2)?;
        }
        Ok(())
    }
    fn read_radio_test(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_radio_test_constcarrier()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_radio_test_plllock()? << 1;
        }
        Ok(())
    }
    fn write_radio_test(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_test_constcarrier((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_test_plllock((*byte >> 1) & 1)?;
        }
        Ok(())
    }
    fn read_radio_tifs(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_radio_tifs_tifs()? << 0;
        }
        Ok(())
    }
    fn write_radio_tifs(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_tifs_tifs((*byte >> 0) & 255)?;
        }
        Ok(())
    }
    fn read_radio_rssisample(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_radio_rssisample_rssisample()? << 0;
        }
        Ok(())
    }
    fn read_radio_state(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_radio_state_state()? << 0;
        }
        Ok(())
    }
    fn read_radio_datawhiteiv(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self
                .0
                .lock()
                .unwrap()
                .read_radio_datawhiteiv_datawhiteiv()?
                << 0;
        }
        Ok(())
    }
    fn write_radio_datawhiteiv(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_datawhiteiv_datawhiteiv((*byte >> 0) & 127)?;
        }
        Ok(())
    }
    fn read_radio_dapn(
        &self,
        _dim: usize,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if _byte_0.is_some() || _byte_1.is_some() {
            self.0
                .lock()
                .unwrap()
                .read_radio_dapn_dap(_dim, _byte_0, _byte_1)?;
        }
        Ok(())
    }
    fn write_radio_dapn(
        &self,
        _dim: usize,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if _byte_0.is_some() || _byte_1.is_some() {
            self.0
                .lock()
                .unwrap()
                .write_radio_dapn_dap(_dim, _byte_0, _byte_1)?;
        }
        Ok(())
    }
    fn read_radio_dacnf(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_radio_dacnf_ena0()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_radio_dacnf_ena1()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_radio_dacnf_ena2()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_radio_dacnf_ena3()? << 3;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_radio_dacnf_ena4()? << 4;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_radio_dacnf_ena5()? << 5;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_radio_dacnf_ena6()? << 6;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_radio_dacnf_ena7()? << 7;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_radio_dacnf_txadd0()? << 0;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_radio_dacnf_txadd1()? << 1;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_radio_dacnf_txadd2()? << 2;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_radio_dacnf_txadd3()? << 3;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_radio_dacnf_txadd4()? << 4;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_radio_dacnf_txadd5()? << 5;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_radio_dacnf_txadd6()? << 6;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_radio_dacnf_txadd7()? << 7;
        }
        Ok(())
    }
    fn write_radio_dacnf(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_dacnf_ena0((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_dacnf_ena1((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_dacnf_ena2((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_dacnf_ena3((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_dacnf_ena4((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_dacnf_ena5((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_dacnf_ena6((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_dacnf_ena7((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_radio_dacnf_txadd0((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_radio_dacnf_txadd1((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_radio_dacnf_txadd2((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_radio_dacnf_txadd3((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_radio_dacnf_txadd4((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_radio_dacnf_txadd5((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_radio_dacnf_txadd6((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_radio_dacnf_txadd7((*byte >> 7) & 1)?;
        }
        Ok(())
    }
    fn read_radio_override0(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if _byte_0.is_some()
            || _byte_1.is_some()
            || _byte_2.is_some()
            || _byte_3.is_some()
        {
            self.0.lock().unwrap().read_radio_override0_override0(
                _byte_0, _byte_1, _byte_2, _byte_3,
            )?;
        }
        Ok(())
    }
    fn write_radio_override0(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if _byte_0.is_some()
            || _byte_1.is_some()
            || _byte_2.is_some()
            || _byte_3.is_some()
        {
            self.0.lock().unwrap().write_radio_override0_override0(
                _byte_0, _byte_1, _byte_2, _byte_3,
            )?;
        }
        Ok(())
    }
    fn read_radio_override1(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if _byte_0.is_some()
            || _byte_1.is_some()
            || _byte_2.is_some()
            || _byte_3.is_some()
        {
            self.0.lock().unwrap().read_radio_override1_override1(
                _byte_0, _byte_1, _byte_2, _byte_3,
            )?;
        }
        Ok(())
    }
    fn write_radio_override1(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if _byte_0.is_some()
            || _byte_1.is_some()
            || _byte_2.is_some()
            || _byte_3.is_some()
        {
            self.0.lock().unwrap().write_radio_override1_override1(
                _byte_0, _byte_1, _byte_2, _byte_3,
            )?;
        }
        Ok(())
    }
    fn read_radio_override2(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if _byte_0.is_some()
            || _byte_1.is_some()
            || _byte_2.is_some()
            || _byte_3.is_some()
        {
            self.0.lock().unwrap().read_radio_override2_override2(
                _byte_0, _byte_1, _byte_2, _byte_3,
            )?;
        }
        Ok(())
    }
    fn write_radio_override2(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if _byte_0.is_some()
            || _byte_1.is_some()
            || _byte_2.is_some()
            || _byte_3.is_some()
        {
            self.0.lock().unwrap().write_radio_override2_override2(
                _byte_0, _byte_1, _byte_2, _byte_3,
            )?;
        }
        Ok(())
    }
    fn read_radio_override3(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if _byte_0.is_some()
            || _byte_1.is_some()
            || _byte_2.is_some()
            || _byte_3.is_some()
        {
            self.0.lock().unwrap().read_radio_override3_override3(
                _byte_0, _byte_1, _byte_2, _byte_3,
            )?;
        }
        Ok(())
    }
    fn write_radio_override3(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if _byte_0.is_some()
            || _byte_1.is_some()
            || _byte_2.is_some()
            || _byte_3.is_some()
        {
            self.0.lock().unwrap().write_radio_override3_override3(
                _byte_0, _byte_1, _byte_2, _byte_3,
            )?;
        }
        Ok(())
    }
    fn read_radio_override4(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if _byte_0.is_some()
            || _byte_1.is_some()
            || _byte_2.is_some()
            || _byte_3.is_some()
        {
            self.0.lock().unwrap().read_radio_override4_override4(
                _byte_0, _byte_1, _byte_2, _byte_3,
            )?;
        }
        if let Some(byte) = _byte_3 {
            **byte |=
                self.0.lock().unwrap().read_radio_override4_enable()? << 7;
        }
        Ok(())
    }
    fn write_radio_override4(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if _byte_0.is_some()
            || _byte_1.is_some()
            || _byte_2.is_some()
            || _byte_3.is_some()
        {
            self.0.lock().unwrap().write_radio_override4_override4(
                _byte_0, _byte_1, _byte_2, _byte_3,
            )?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_radio_override4_enable((*byte >> 7) & 1)?;
        }
        Ok(())
    }
    fn read_radio_power(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_radio_power_power()? << 0;
        }
        Ok(())
    }
    fn write_radio_power(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_radio_power_power((*byte >> 0) & 1)?;
        }
        Ok(())
    }
}
pub struct PeripheralPage0x40002000(
    pub std::sync::Arc<std::sync::Mutex<super::peripheral::Peripherals>>,
);
impl icicle_vm::cpu::mem::IoMemory for PeripheralPage0x40002000 {
    fn read(
        &mut self,
        _addr: u64,
        _buf: &mut [u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073750016;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=15, 1..=16) => {
                if (_start < 4) || (_end <= 4) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 4 && _start < 8) || (_end > 4 && _end <= 8) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 8 && _start < 12) || (_end > 8 && _end <= 12) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 12 && _start < 16) || (_end > 12 && _end <= 16) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (28..=31, 29..=32) => {
                if (_start >= 28 && _start < 32) || (_end > 28 && _end <= 32) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (256..=267, 257..=268) => {
                if (_start >= 256 && _start < 260)
                    || (_end > 256 && _end <= 260)
                {
                    self.0.lock().unwrap().read_uart0_events_cts(
                        &mut buffer_mut(_start, _end, 256, _buf),
                        &mut buffer_mut(_start, _end, 257, _buf),
                        &mut buffer_mut(_start, _end, 258, _buf),
                        &mut buffer_mut(_start, _end, 259, _buf),
                    )?;
                }
                if (_start >= 260 && _start < 264)
                    || (_end > 260 && _end <= 264)
                {
                    self.0.lock().unwrap().read_uart0_events_ncts(
                        &mut buffer_mut(_start, _end, 260, _buf),
                        &mut buffer_mut(_start, _end, 261, _buf),
                        &mut buffer_mut(_start, _end, 262, _buf),
                        &mut buffer_mut(_start, _end, 263, _buf),
                    )?;
                }
                if (_start >= 264 && _start < 268)
                    || (_end > 264 && _end <= 268)
                {
                    self.0.lock().unwrap().read_uart0_events_rxdrdy(
                        &mut buffer_mut(_start, _end, 264, _buf),
                        &mut buffer_mut(_start, _end, 265, _buf),
                        &mut buffer_mut(_start, _end, 266, _buf),
                        &mut buffer_mut(_start, _end, 267, _buf),
                    )?;
                }
            }
            (284..=287, 285..=288) => {
                if (_start >= 284 && _start < 288)
                    || (_end > 284 && _end <= 288)
                {
                    self.0.lock().unwrap().read_uart0_events_txdrdy(
                        &mut buffer_mut(_start, _end, 284, _buf),
                        &mut buffer_mut(_start, _end, 285, _buf),
                        &mut buffer_mut(_start, _end, 286, _buf),
                        &mut buffer_mut(_start, _end, 287, _buf),
                    )?;
                }
            }
            (292..=295, 293..=296) => {
                if (_start >= 292 && _start < 296)
                    || (_end > 292 && _end <= 296)
                {
                    self.0.lock().unwrap().read_uart0_events_error(
                        &mut buffer_mut(_start, _end, 292, _buf),
                        &mut buffer_mut(_start, _end, 293, _buf),
                        &mut buffer_mut(_start, _end, 294, _buf),
                        &mut buffer_mut(_start, _end, 295, _buf),
                    )?;
                }
            }
            (324..=327, 325..=328) => {
                if (_start >= 324 && _start < 328)
                    || (_end > 324 && _end <= 328)
                {
                    self.0.lock().unwrap().read_uart0_events_rxto(
                        &mut buffer_mut(_start, _end, 324, _buf),
                        &mut buffer_mut(_start, _end, 325, _buf),
                        &mut buffer_mut(_start, _end, 326, _buf),
                        &mut buffer_mut(_start, _end, 327, _buf),
                    )?;
                }
            }
            (512..=515, 513..=516) => {
                if (_start >= 512 && _start < 516)
                    || (_end > 512 && _end <= 516)
                {
                    self.read_uart0_shorts(
                        &mut buffer_mut(_start, _end, 512, _buf),
                        &mut buffer_mut(_start, _end, 513, _buf),
                        &mut buffer_mut(_start, _end, 514, _buf),
                        &mut buffer_mut(_start, _end, 515, _buf),
                    )?;
                }
            }
            (772..=779, 773..=780) => {
                if (_start >= 772 && _start < 776)
                    || (_end > 772 && _end <= 776)
                {
                    self.read_uart0_intenset(
                        &mut buffer_mut(_start, _end, 772, _buf),
                        &mut buffer_mut(_start, _end, 773, _buf),
                        &mut buffer_mut(_start, _end, 774, _buf),
                        &mut buffer_mut(_start, _end, 775, _buf),
                    )?;
                }
                if (_start >= 776 && _start < 780)
                    || (_end > 776 && _end <= 780)
                {
                    self.read_uart0_intenclr(
                        &mut buffer_mut(_start, _end, 776, _buf),
                        &mut buffer_mut(_start, _end, 777, _buf),
                        &mut buffer_mut(_start, _end, 778, _buf),
                        &mut buffer_mut(_start, _end, 779, _buf),
                    )?;
                }
            }
            (1152..=1155, 1153..=1156) => {
                if (_start >= 1152 && _start < 1156)
                    || (_end > 1152 && _end <= 1156)
                {
                    self.read_uart0_errorsrc(
                        &mut buffer_mut(_start, _end, 1152, _buf),
                        &mut buffer_mut(_start, _end, 1153, _buf),
                        &mut buffer_mut(_start, _end, 1154, _buf),
                        &mut buffer_mut(_start, _end, 1155, _buf),
                    )?;
                }
            }
            (1280..=1283, 1281..=1284) => {
                if (_start >= 1280 && _start < 1284)
                    || (_end > 1280 && _end <= 1284)
                {
                    self.read_uart0_enable(
                        &mut buffer_mut(_start, _end, 1280, _buf),
                        &mut buffer_mut(_start, _end, 1281, _buf),
                        &mut buffer_mut(_start, _end, 1282, _buf),
                        &mut buffer_mut(_start, _end, 1283, _buf),
                    )?;
                }
            }
            (1288..=1311, 1289..=1312) => {
                if (_start >= 1288 && _start < 1292)
                    || (_end > 1288 && _end <= 1292)
                {
                    self.0.lock().unwrap().read_uart0_pselrts(
                        &mut buffer_mut(_start, _end, 1288, _buf),
                        &mut buffer_mut(_start, _end, 1289, _buf),
                        &mut buffer_mut(_start, _end, 1290, _buf),
                        &mut buffer_mut(_start, _end, 1291, _buf),
                    )?;
                }
                if (_start >= 1292 && _start < 1296)
                    || (_end > 1292 && _end <= 1296)
                {
                    self.0.lock().unwrap().read_uart0_pseltxd(
                        &mut buffer_mut(_start, _end, 1292, _buf),
                        &mut buffer_mut(_start, _end, 1293, _buf),
                        &mut buffer_mut(_start, _end, 1294, _buf),
                        &mut buffer_mut(_start, _end, 1295, _buf),
                    )?;
                }
                if (_start >= 1296 && _start < 1300)
                    || (_end > 1296 && _end <= 1300)
                {
                    self.0.lock().unwrap().read_uart0_pselcts(
                        &mut buffer_mut(_start, _end, 1296, _buf),
                        &mut buffer_mut(_start, _end, 1297, _buf),
                        &mut buffer_mut(_start, _end, 1298, _buf),
                        &mut buffer_mut(_start, _end, 1299, _buf),
                    )?;
                }
                if (_start >= 1300 && _start < 1304)
                    || (_end > 1300 && _end <= 1304)
                {
                    self.0.lock().unwrap().read_uart0_pselrxd(
                        &mut buffer_mut(_start, _end, 1300, _buf),
                        &mut buffer_mut(_start, _end, 1301, _buf),
                        &mut buffer_mut(_start, _end, 1302, _buf),
                        &mut buffer_mut(_start, _end, 1303, _buf),
                    )?;
                }
                if (_start >= 1304 && _start < 1308)
                    || (_end > 1304 && _end <= 1308)
                {
                    self.read_uart0_rxd(
                        &mut buffer_mut(_start, _end, 1304, _buf),
                        &mut buffer_mut(_start, _end, 1305, _buf),
                        &mut buffer_mut(_start, _end, 1306, _buf),
                        &mut buffer_mut(_start, _end, 1307, _buf),
                    )?;
                }
                if (_start >= 1308 && _start < 1312)
                    || (_end > 1308 && _end <= 1312)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (1316..=1319, 1317..=1320) => {
                if (_start >= 1316 && _start < 1320)
                    || (_end > 1316 && _end <= 1320)
                {
                    self.read_uart0_baudrate(
                        &mut buffer_mut(_start, _end, 1316, _buf),
                        &mut buffer_mut(_start, _end, 1317, _buf),
                        &mut buffer_mut(_start, _end, 1318, _buf),
                        &mut buffer_mut(_start, _end, 1319, _buf),
                    )?;
                }
            }
            (1388..=1391, 1389..=1392) => {
                if (_start >= 1388 && _start < 1392)
                    || (_end > 1388 && _end <= 1392)
                {
                    self.read_uart0_config(
                        &mut buffer_mut(_start, _end, 1388, _buf),
                        &mut buffer_mut(_start, _end, 1389, _buf),
                        &mut buffer_mut(_start, _end, 1390, _buf),
                        &mut buffer_mut(_start, _end, 1391, _buf),
                    )?;
                }
            }
            (4092..=4095, 4093..=4096) => {
                if (_start >= 4092 && _start < 4096)
                    || (_end > 4092 && _end <= 4096)
                {
                    self.read_uart0_power(
                        &mut buffer_mut(_start, _end, 4092, _buf),
                        &mut buffer_mut(_start, _end, 4093, _buf),
                        &mut buffer_mut(_start, _end, 4094, _buf),
                        &mut buffer_mut(_start, _end, 4095, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
    fn write(
        &mut self,
        _addr: u64,
        _buf: &[u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073750016;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=15, 1..=16) => {
                if (_start < 4) || (_end <= 4) {
                    self.0.lock().unwrap().write_uart0_tasks_startrx(
                        buffer_const(_start, _end, 0, _buf),
                        buffer_const(_start, _end, 1, _buf),
                        buffer_const(_start, _end, 2, _buf),
                        buffer_const(_start, _end, 3, _buf),
                    )?;
                }
                if (_start >= 4 && _start < 8) || (_end > 4 && _end <= 8) {
                    self.0.lock().unwrap().write_uart0_tasks_stoprx(
                        buffer_const(_start, _end, 4, _buf),
                        buffer_const(_start, _end, 5, _buf),
                        buffer_const(_start, _end, 6, _buf),
                        buffer_const(_start, _end, 7, _buf),
                    )?;
                }
                if (_start >= 8 && _start < 12) || (_end > 8 && _end <= 12) {
                    self.0.lock().unwrap().write_uart0_tasks_starttx(
                        buffer_const(_start, _end, 8, _buf),
                        buffer_const(_start, _end, 9, _buf),
                        buffer_const(_start, _end, 10, _buf),
                        buffer_const(_start, _end, 11, _buf),
                    )?;
                }
                if (_start >= 12 && _start < 16) || (_end > 12 && _end <= 16) {
                    self.0.lock().unwrap().write_uart0_tasks_stoptx(
                        buffer_const(_start, _end, 12, _buf),
                        buffer_const(_start, _end, 13, _buf),
                        buffer_const(_start, _end, 14, _buf),
                        buffer_const(_start, _end, 15, _buf),
                    )?;
                }
            }
            (28..=31, 29..=32) => {
                if (_start >= 28 && _start < 32) || (_end > 28 && _end <= 32) {
                    self.0.lock().unwrap().write_uart0_tasks_suspend(
                        buffer_const(_start, _end, 28, _buf),
                        buffer_const(_start, _end, 29, _buf),
                        buffer_const(_start, _end, 30, _buf),
                        buffer_const(_start, _end, 31, _buf),
                    )?;
                }
            }
            (256..=267, 257..=268) => {
                if (_start >= 256 && _start < 260)
                    || (_end > 256 && _end <= 260)
                {
                    self.0.lock().unwrap().write_uart0_events_cts(
                        buffer_const(_start, _end, 256, _buf),
                        buffer_const(_start, _end, 257, _buf),
                        buffer_const(_start, _end, 258, _buf),
                        buffer_const(_start, _end, 259, _buf),
                    )?;
                }
                if (_start >= 260 && _start < 264)
                    || (_end > 260 && _end <= 264)
                {
                    self.0.lock().unwrap().write_uart0_events_ncts(
                        buffer_const(_start, _end, 260, _buf),
                        buffer_const(_start, _end, 261, _buf),
                        buffer_const(_start, _end, 262, _buf),
                        buffer_const(_start, _end, 263, _buf),
                    )?;
                }
                if (_start >= 264 && _start < 268)
                    || (_end > 264 && _end <= 268)
                {
                    self.0.lock().unwrap().write_uart0_events_rxdrdy(
                        buffer_const(_start, _end, 264, _buf),
                        buffer_const(_start, _end, 265, _buf),
                        buffer_const(_start, _end, 266, _buf),
                        buffer_const(_start, _end, 267, _buf),
                    )?;
                }
            }
            (284..=287, 285..=288) => {
                if (_start >= 284 && _start < 288)
                    || (_end > 284 && _end <= 288)
                {
                    self.0.lock().unwrap().write_uart0_events_txdrdy(
                        buffer_const(_start, _end, 284, _buf),
                        buffer_const(_start, _end, 285, _buf),
                        buffer_const(_start, _end, 286, _buf),
                        buffer_const(_start, _end, 287, _buf),
                    )?;
                }
            }
            (292..=295, 293..=296) => {
                if (_start >= 292 && _start < 296)
                    || (_end > 292 && _end <= 296)
                {
                    self.0.lock().unwrap().write_uart0_events_error(
                        buffer_const(_start, _end, 292, _buf),
                        buffer_const(_start, _end, 293, _buf),
                        buffer_const(_start, _end, 294, _buf),
                        buffer_const(_start, _end, 295, _buf),
                    )?;
                }
            }
            (324..=327, 325..=328) => {
                if (_start >= 324 && _start < 328)
                    || (_end > 324 && _end <= 328)
                {
                    self.0.lock().unwrap().write_uart0_events_rxto(
                        buffer_const(_start, _end, 324, _buf),
                        buffer_const(_start, _end, 325, _buf),
                        buffer_const(_start, _end, 326, _buf),
                        buffer_const(_start, _end, 327, _buf),
                    )?;
                }
            }
            (512..=515, 513..=516) => {
                if (_start >= 512 && _start < 516)
                    || (_end > 512 && _end <= 516)
                {
                    self.write_uart0_shorts(
                        buffer_const(_start, _end, 512, _buf),
                        buffer_const(_start, _end, 513, _buf),
                        buffer_const(_start, _end, 514, _buf),
                        buffer_const(_start, _end, 515, _buf),
                    )?;
                }
            }
            (772..=779, 773..=780) => {
                if (_start >= 772 && _start < 776)
                    || (_end > 772 && _end <= 776)
                {
                    self.write_uart0_intenset(
                        buffer_const(_start, _end, 772, _buf),
                        buffer_const(_start, _end, 773, _buf),
                        buffer_const(_start, _end, 774, _buf),
                        buffer_const(_start, _end, 775, _buf),
                    )?;
                }
                if (_start >= 776 && _start < 780)
                    || (_end > 776 && _end <= 780)
                {
                    self.write_uart0_intenclr(
                        buffer_const(_start, _end, 776, _buf),
                        buffer_const(_start, _end, 777, _buf),
                        buffer_const(_start, _end, 778, _buf),
                        buffer_const(_start, _end, 779, _buf),
                    )?;
                }
            }
            (1152..=1155, 1153..=1156) => {
                if (_start >= 1152 && _start < 1156)
                    || (_end > 1152 && _end <= 1156)
                {
                    self.write_uart0_errorsrc(
                        buffer_const(_start, _end, 1152, _buf),
                        buffer_const(_start, _end, 1153, _buf),
                        buffer_const(_start, _end, 1154, _buf),
                        buffer_const(_start, _end, 1155, _buf),
                    )?;
                }
            }
            (1280..=1283, 1281..=1284) => {
                if (_start >= 1280 && _start < 1284)
                    || (_end > 1280 && _end <= 1284)
                {
                    self.write_uart0_enable(
                        buffer_const(_start, _end, 1280, _buf),
                        buffer_const(_start, _end, 1281, _buf),
                        buffer_const(_start, _end, 1282, _buf),
                        buffer_const(_start, _end, 1283, _buf),
                    )?;
                }
            }
            (1288..=1311, 1289..=1312) => {
                if (_start >= 1288 && _start < 1292)
                    || (_end > 1288 && _end <= 1292)
                {
                    self.0.lock().unwrap().write_uart0_pselrts(
                        buffer_const(_start, _end, 1288, _buf),
                        buffer_const(_start, _end, 1289, _buf),
                        buffer_const(_start, _end, 1290, _buf),
                        buffer_const(_start, _end, 1291, _buf),
                    )?;
                }
                if (_start >= 1292 && _start < 1296)
                    || (_end > 1292 && _end <= 1296)
                {
                    self.0.lock().unwrap().write_uart0_pseltxd(
                        buffer_const(_start, _end, 1292, _buf),
                        buffer_const(_start, _end, 1293, _buf),
                        buffer_const(_start, _end, 1294, _buf),
                        buffer_const(_start, _end, 1295, _buf),
                    )?;
                }
                if (_start >= 1296 && _start < 1300)
                    || (_end > 1296 && _end <= 1300)
                {
                    self.0.lock().unwrap().write_uart0_pselcts(
                        buffer_const(_start, _end, 1296, _buf),
                        buffer_const(_start, _end, 1297, _buf),
                        buffer_const(_start, _end, 1298, _buf),
                        buffer_const(_start, _end, 1299, _buf),
                    )?;
                }
                if (_start >= 1300 && _start < 1304)
                    || (_end > 1300 && _end <= 1304)
                {
                    self.0.lock().unwrap().write_uart0_pselrxd(
                        buffer_const(_start, _end, 1300, _buf),
                        buffer_const(_start, _end, 1301, _buf),
                        buffer_const(_start, _end, 1302, _buf),
                        buffer_const(_start, _end, 1303, _buf),
                    )?;
                }
                if (_start >= 1304 && _start < 1308)
                    || (_end > 1304 && _end <= 1308)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 1308 && _start < 1312)
                    || (_end > 1308 && _end <= 1312)
                {
                    self.write_uart0_txd(
                        buffer_const(_start, _end, 1308, _buf),
                        buffer_const(_start, _end, 1309, _buf),
                        buffer_const(_start, _end, 1310, _buf),
                        buffer_const(_start, _end, 1311, _buf),
                    )?;
                }
            }
            (1316..=1319, 1317..=1320) => {
                if (_start >= 1316 && _start < 1320)
                    || (_end > 1316 && _end <= 1320)
                {
                    self.write_uart0_baudrate(
                        buffer_const(_start, _end, 1316, _buf),
                        buffer_const(_start, _end, 1317, _buf),
                        buffer_const(_start, _end, 1318, _buf),
                        buffer_const(_start, _end, 1319, _buf),
                    )?;
                }
            }
            (1388..=1391, 1389..=1392) => {
                if (_start >= 1388 && _start < 1392)
                    || (_end > 1388 && _end <= 1392)
                {
                    self.write_uart0_config(
                        buffer_const(_start, _end, 1388, _buf),
                        buffer_const(_start, _end, 1389, _buf),
                        buffer_const(_start, _end, 1390, _buf),
                        buffer_const(_start, _end, 1391, _buf),
                    )?;
                }
            }
            (4092..=4095, 4093..=4096) => {
                if (_start >= 4092 && _start < 4096)
                    || (_end > 4092 && _end <= 4096)
                {
                    self.write_uart0_power(
                        buffer_const(_start, _end, 4092, _buf),
                        buffer_const(_start, _end, 4093, _buf),
                        buffer_const(_start, _end, 4094, _buf),
                        buffer_const(_start, _end, 4095, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
}
impl PeripheralPage0x40002000 {
    fn read_uart0_shorts(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_uart0_shorts_cts_startrx()? << 3;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_uart0_shorts_ncts_stoprx()? << 4;
        }
        Ok(())
    }
    fn write_uart0_shorts(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_uart0_shorts_cts_startrx((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_uart0_shorts_ncts_stoprx((*byte >> 4) & 1)?;
        }
        Ok(())
    }
    fn read_uart0_intenset(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_uart0_intenset_cts()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_uart0_intenset_ncts()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_uart0_intenset_rxdrdy()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_uart0_intenset_txdrdy()? << 7;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_uart0_intenset_error()? << 1;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_uart0_intenset_rxto()? << 1;
        }
        Ok(())
    }
    fn write_uart0_intenset(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_uart0_intenset_cts((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_uart0_intenset_ncts((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_uart0_intenset_rxdrdy((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_uart0_intenset_txdrdy((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_uart0_intenset_error((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_uart0_intenset_rxto((*byte >> 1) & 1)?;
        }
        Ok(())
    }
    fn read_uart0_intenclr(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_uart0_intenclr_cts()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_uart0_intenclr_ncts()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_uart0_intenclr_rxdrdy()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_uart0_intenclr_txdrdy()? << 7;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_uart0_intenclr_error()? << 1;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_uart0_intenclr_rxto()? << 1;
        }
        Ok(())
    }
    fn write_uart0_intenclr(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_uart0_intenclr_cts((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_uart0_intenclr_ncts((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_uart0_intenclr_rxdrdy((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_uart0_intenclr_txdrdy((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_uart0_intenclr_error((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_uart0_intenclr_rxto((*byte >> 1) & 1)?;
        }
        Ok(())
    }
    fn read_uart0_errorsrc(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_uart0_errorsrc_overrun()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_uart0_errorsrc_parity()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_uart0_errorsrc_framing()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_uart0_errorsrc_break()? << 3;
        }
        Ok(())
    }
    fn write_uart0_errorsrc(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_uart0_errorsrc_overrun((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_uart0_errorsrc_parity((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_uart0_errorsrc_framing((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_uart0_errorsrc_break((*byte >> 3) & 1)?;
        }
        Ok(())
    }
    fn read_uart0_enable(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_uart0_enable_enable()? << 0;
        }
        Ok(())
    }
    fn write_uart0_enable(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_uart0_enable_enable((*byte >> 0) & 7)?;
        }
        Ok(())
    }
    fn read_uart0_rxd(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_uart0_rxd_rxd()? << 0;
        }
        Ok(())
    }
    fn write_uart0_txd(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_uart0_txd_txd((*byte >> 0) & 255)?;
        }
        Ok(())
    }
    fn read_uart0_baudrate(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if _byte_0.is_some()
            || _byte_1.is_some()
            || _byte_2.is_some()
            || _byte_3.is_some()
        {
            self.0.lock().unwrap().read_uart0_baudrate_baudrate(
                _byte_0, _byte_1, _byte_2, _byte_3,
            )?;
        }
        Ok(())
    }
    fn write_uart0_baudrate(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if _byte_0.is_some()
            || _byte_1.is_some()
            || _byte_2.is_some()
            || _byte_3.is_some()
        {
            self.0.lock().unwrap().write_uart0_baudrate_baudrate(
                _byte_0, _byte_1, _byte_2, _byte_3,
            )?;
        }
        Ok(())
    }
    fn read_uart0_config(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_uart0_config_hwfc()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_uart0_config_parity()? << 1;
        }
        Ok(())
    }
    fn write_uart0_config(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_uart0_config_hwfc((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_uart0_config_parity((*byte >> 1) & 7)?;
        }
        Ok(())
    }
    fn read_uart0_power(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_uart0_power_power()? << 0;
        }
        Ok(())
    }
    fn write_uart0_power(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_uart0_power_power((*byte >> 0) & 1)?;
        }
        Ok(())
    }
}
pub struct PeripheralPage0x40003000(
    pub std::sync::Arc<std::sync::Mutex<super::peripheral::Peripherals>>,
);
impl icicle_vm::cpu::mem::IoMemory for PeripheralPage0x40003000 {
    fn read(
        &mut self,
        _addr: u64,
        _buf: &mut [u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073754112;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=3, 1..=4) => {
                if (_start < 4) || (_end <= 4) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (8..=11, 9..=12) => {
                if (_start >= 8 && _start < 12) || (_end > 8 && _end <= 12) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (20..=23, 21..=24) => {
                if (_start >= 20 && _start < 24) || (_end > 20 && _end <= 24) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (28..=35, 29..=36) => {
                if (_start >= 28 && _start < 32) || (_end > 28 && _end <= 32) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 32 && _start < 36) || (_end > 32 && _end <= 36) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (260..=267, 261..=268) => {
                if (_start >= 264 && _start < 268)
                    || (_end > 264 && _end <= 268)
                {
                    self.0.lock().unwrap().read_spi0twi0_events_ready(
                        &mut buffer_mut(_start, _end, 264, _buf),
                        &mut buffer_mut(_start, _end, 265, _buf),
                        &mut buffer_mut(_start, _end, 266, _buf),
                        &mut buffer_mut(_start, _end, 267, _buf),
                    )?;
                }
                if (_start >= 260 && _start < 264)
                    || (_end > 260 && _end <= 264)
                {
                    self.0.lock().unwrap().read_twi0_events_stopped(
                        &mut buffer_mut(_start, _end, 260, _buf),
                        &mut buffer_mut(_start, _end, 261, _buf),
                        &mut buffer_mut(_start, _end, 262, _buf),
                        &mut buffer_mut(_start, _end, 263, _buf),
                    )?;
                }
            }
            (284..=287, 285..=288) => {
                if (_start >= 284 && _start < 288)
                    || (_end > 284 && _end <= 288)
                {
                    self.0.lock().unwrap().read_twi0_events_txdsent(
                        &mut buffer_mut(_start, _end, 284, _buf),
                        &mut buffer_mut(_start, _end, 285, _buf),
                        &mut buffer_mut(_start, _end, 286, _buf),
                        &mut buffer_mut(_start, _end, 287, _buf),
                    )?;
                }
            }
            (292..=295, 293..=296) => {
                if (_start >= 292 && _start < 296)
                    || (_end > 292 && _end <= 296)
                {
                    self.0.lock().unwrap().read_twi0_events_error(
                        &mut buffer_mut(_start, _end, 292, _buf),
                        &mut buffer_mut(_start, _end, 293, _buf),
                        &mut buffer_mut(_start, _end, 294, _buf),
                        &mut buffer_mut(_start, _end, 295, _buf),
                    )?;
                }
            }
            (312..=315, 313..=316) => {
                if (_start >= 312 && _start < 316)
                    || (_end > 312 && _end <= 316)
                {
                    self.0.lock().unwrap().read_twi0_events_bb(
                        &mut buffer_mut(_start, _end, 312, _buf),
                        &mut buffer_mut(_start, _end, 313, _buf),
                        &mut buffer_mut(_start, _end, 314, _buf),
                        &mut buffer_mut(_start, _end, 315, _buf),
                    )?;
                }
            }
            (328..=331, 329..=332) => {
                if (_start >= 328 && _start < 332)
                    || (_end > 328 && _end <= 332)
                {
                    self.0.lock().unwrap().read_twi0_events_suspended(
                        &mut buffer_mut(_start, _end, 328, _buf),
                        &mut buffer_mut(_start, _end, 329, _buf),
                        &mut buffer_mut(_start, _end, 330, _buf),
                        &mut buffer_mut(_start, _end, 331, _buf),
                    )?;
                }
            }
            (512..=515, 513..=516) => {
                if (_start >= 512 && _start < 516)
                    || (_end > 512 && _end <= 516)
                {
                    self.read_twi0_shorts(
                        &mut buffer_mut(_start, _end, 512, _buf),
                        &mut buffer_mut(_start, _end, 513, _buf),
                        &mut buffer_mut(_start, _end, 514, _buf),
                        &mut buffer_mut(_start, _end, 515, _buf),
                    )?;
                }
            }
            (772..=779, 773..=780) => {
                if (_start >= 772 && _start < 776)
                    || (_end > 772 && _end <= 776)
                {
                    self.read_spi0twi0_intenset(
                        &mut buffer_mut(_start, _end, 772, _buf),
                        &mut buffer_mut(_start, _end, 773, _buf),
                        &mut buffer_mut(_start, _end, 774, _buf),
                        &mut buffer_mut(_start, _end, 775, _buf),
                    )?;
                }
                if (_start >= 776 && _start < 780)
                    || (_end > 776 && _end <= 780)
                {
                    self.read_spi0twi0_intenclr(
                        &mut buffer_mut(_start, _end, 776, _buf),
                        &mut buffer_mut(_start, _end, 777, _buf),
                        &mut buffer_mut(_start, _end, 778, _buf),
                        &mut buffer_mut(_start, _end, 779, _buf),
                    )?;
                }
            }
            (1220..=1223, 1221..=1224) => {
                if (_start >= 1220 && _start < 1224)
                    || (_end > 1220 && _end <= 1224)
                {
                    self.read_twi0_errorsrc(
                        &mut buffer_mut(_start, _end, 1220, _buf),
                        &mut buffer_mut(_start, _end, 1221, _buf),
                        &mut buffer_mut(_start, _end, 1222, _buf),
                        &mut buffer_mut(_start, _end, 1223, _buf),
                    )?;
                }
            }
            (1280..=1283, 1281..=1284) => {
                if (_start >= 1280 && _start < 1284)
                    || (_end > 1280 && _end <= 1284)
                {
                    self.read_spi0twi0_enable(
                        &mut buffer_mut(_start, _end, 1280, _buf),
                        &mut buffer_mut(_start, _end, 1281, _buf),
                        &mut buffer_mut(_start, _end, 1282, _buf),
                        &mut buffer_mut(_start, _end, 1283, _buf),
                    )?;
                }
            }
            (1288..=1299, 1289..=1300) => {
                if (_start >= 1288 && _start < 1292)
                    || (_end > 1288 && _end <= 1292)
                {
                    self.0.lock().unwrap().read_spi0twi0_pselsck(
                        &mut buffer_mut(_start, _end, 1288, _buf),
                        &mut buffer_mut(_start, _end, 1289, _buf),
                        &mut buffer_mut(_start, _end, 1290, _buf),
                        &mut buffer_mut(_start, _end, 1291, _buf),
                    )?;
                }
                if (_start >= 1292 && _start < 1296)
                    || (_end > 1292 && _end <= 1296)
                {
                    self.0.lock().unwrap().read_spi0twi0_pselmosi(
                        &mut buffer_mut(_start, _end, 1292, _buf),
                        &mut buffer_mut(_start, _end, 1293, _buf),
                        &mut buffer_mut(_start, _end, 1294, _buf),
                        &mut buffer_mut(_start, _end, 1295, _buf),
                    )?;
                }
                if (_start >= 1296 && _start < 1300)
                    || (_end > 1296 && _end <= 1300)
                {
                    self.0.lock().unwrap().read_spi0_pselmiso(
                        &mut buffer_mut(_start, _end, 1296, _buf),
                        &mut buffer_mut(_start, _end, 1297, _buf),
                        &mut buffer_mut(_start, _end, 1298, _buf),
                        &mut buffer_mut(_start, _end, 1299, _buf),
                    )?;
                }
            }
            (1304..=1311, 1305..=1312) => {
                if (_start >= 1304 && _start < 1308)
                    || (_end > 1304 && _end <= 1308)
                {
                    self.read_spi0twi0_rxd(
                        &mut buffer_mut(_start, _end, 1304, _buf),
                        &mut buffer_mut(_start, _end, 1305, _buf),
                        &mut buffer_mut(_start, _end, 1306, _buf),
                        &mut buffer_mut(_start, _end, 1307, _buf),
                    )?;
                }
                if (_start >= 1308 && _start < 1312)
                    || (_end > 1308 && _end <= 1312)
                {
                    self.read_spi0twi0_txd(
                        &mut buffer_mut(_start, _end, 1308, _buf),
                        &mut buffer_mut(_start, _end, 1309, _buf),
                        &mut buffer_mut(_start, _end, 1310, _buf),
                        &mut buffer_mut(_start, _end, 1311, _buf),
                    )?;
                }
            }
            (1316..=1319, 1317..=1320) => {
                if (_start >= 1316 && _start < 1320)
                    || (_end > 1316 && _end <= 1320)
                {
                    self.read_spi0twi0_frequency(
                        &mut buffer_mut(_start, _end, 1316, _buf),
                        &mut buffer_mut(_start, _end, 1317, _buf),
                        &mut buffer_mut(_start, _end, 1318, _buf),
                        &mut buffer_mut(_start, _end, 1319, _buf),
                    )?;
                }
            }
            (1364..=1367, 1365..=1368) => {
                if (_start >= 1364 && _start < 1368)
                    || (_end > 1364 && _end <= 1368)
                {
                    self.read_spi0_config(
                        &mut buffer_mut(_start, _end, 1364, _buf),
                        &mut buffer_mut(_start, _end, 1365, _buf),
                        &mut buffer_mut(_start, _end, 1366, _buf),
                        &mut buffer_mut(_start, _end, 1367, _buf),
                    )?;
                }
            }
            (1416..=1419, 1417..=1420) => {
                if (_start >= 1416 && _start < 1420)
                    || (_end > 1416 && _end <= 1420)
                {
                    self.read_twi0_address(
                        &mut buffer_mut(_start, _end, 1416, _buf),
                        &mut buffer_mut(_start, _end, 1417, _buf),
                        &mut buffer_mut(_start, _end, 1418, _buf),
                        &mut buffer_mut(_start, _end, 1419, _buf),
                    )?;
                }
            }
            (4092..=4095, 4093..=4096) => {
                if (_start >= 4092 && _start < 4096)
                    || (_end > 4092 && _end <= 4096)
                {
                    self.read_spi0twi0_power(
                        &mut buffer_mut(_start, _end, 4092, _buf),
                        &mut buffer_mut(_start, _end, 4093, _buf),
                        &mut buffer_mut(_start, _end, 4094, _buf),
                        &mut buffer_mut(_start, _end, 4095, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
    fn write(
        &mut self,
        _addr: u64,
        _buf: &[u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073754112;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=3, 1..=4) => {
                if (_start < 4) || (_end <= 4) {
                    self.0.lock().unwrap().write_twi0_tasks_startrx(
                        buffer_const(_start, _end, 0, _buf),
                        buffer_const(_start, _end, 1, _buf),
                        buffer_const(_start, _end, 2, _buf),
                        buffer_const(_start, _end, 3, _buf),
                    )?;
                }
            }
            (8..=11, 9..=12) => {
                if (_start >= 8 && _start < 12) || (_end > 8 && _end <= 12) {
                    self.0.lock().unwrap().write_twi0_tasks_starttx(
                        buffer_const(_start, _end, 8, _buf),
                        buffer_const(_start, _end, 9, _buf),
                        buffer_const(_start, _end, 10, _buf),
                        buffer_const(_start, _end, 11, _buf),
                    )?;
                }
            }
            (20..=23, 21..=24) => {
                if (_start >= 20 && _start < 24) || (_end > 20 && _end <= 24) {
                    self.0.lock().unwrap().write_twi0_tasks_stop(
                        buffer_const(_start, _end, 20, _buf),
                        buffer_const(_start, _end, 21, _buf),
                        buffer_const(_start, _end, 22, _buf),
                        buffer_const(_start, _end, 23, _buf),
                    )?;
                }
            }
            (28..=35, 29..=36) => {
                if (_start >= 28 && _start < 32) || (_end > 28 && _end <= 32) {
                    self.0.lock().unwrap().write_twi0_tasks_suspend(
                        buffer_const(_start, _end, 28, _buf),
                        buffer_const(_start, _end, 29, _buf),
                        buffer_const(_start, _end, 30, _buf),
                        buffer_const(_start, _end, 31, _buf),
                    )?;
                }
                if (_start >= 32 && _start < 36) || (_end > 32 && _end <= 36) {
                    self.0.lock().unwrap().write_twi0_tasks_resume(
                        buffer_const(_start, _end, 32, _buf),
                        buffer_const(_start, _end, 33, _buf),
                        buffer_const(_start, _end, 34, _buf),
                        buffer_const(_start, _end, 35, _buf),
                    )?;
                }
            }
            (260..=267, 261..=268) => {
                if (_start >= 264 && _start < 268)
                    || (_end > 264 && _end <= 268)
                {
                    self.0.lock().unwrap().write_spi0twi0_events_ready(
                        buffer_const(_start, _end, 264, _buf),
                        buffer_const(_start, _end, 265, _buf),
                        buffer_const(_start, _end, 266, _buf),
                        buffer_const(_start, _end, 267, _buf),
                    )?;
                }
                if (_start >= 260 && _start < 264)
                    || (_end > 260 && _end <= 264)
                {
                    self.0.lock().unwrap().write_twi0_events_stopped(
                        buffer_const(_start, _end, 260, _buf),
                        buffer_const(_start, _end, 261, _buf),
                        buffer_const(_start, _end, 262, _buf),
                        buffer_const(_start, _end, 263, _buf),
                    )?;
                }
            }
            (284..=287, 285..=288) => {
                if (_start >= 284 && _start < 288)
                    || (_end > 284 && _end <= 288)
                {
                    self.0.lock().unwrap().write_twi0_events_txdsent(
                        buffer_const(_start, _end, 284, _buf),
                        buffer_const(_start, _end, 285, _buf),
                        buffer_const(_start, _end, 286, _buf),
                        buffer_const(_start, _end, 287, _buf),
                    )?;
                }
            }
            (292..=295, 293..=296) => {
                if (_start >= 292 && _start < 296)
                    || (_end > 292 && _end <= 296)
                {
                    self.0.lock().unwrap().write_twi0_events_error(
                        buffer_const(_start, _end, 292, _buf),
                        buffer_const(_start, _end, 293, _buf),
                        buffer_const(_start, _end, 294, _buf),
                        buffer_const(_start, _end, 295, _buf),
                    )?;
                }
            }
            (312..=315, 313..=316) => {
                if (_start >= 312 && _start < 316)
                    || (_end > 312 && _end <= 316)
                {
                    self.0.lock().unwrap().write_twi0_events_bb(
                        buffer_const(_start, _end, 312, _buf),
                        buffer_const(_start, _end, 313, _buf),
                        buffer_const(_start, _end, 314, _buf),
                        buffer_const(_start, _end, 315, _buf),
                    )?;
                }
            }
            (328..=331, 329..=332) => {
                if (_start >= 328 && _start < 332)
                    || (_end > 328 && _end <= 332)
                {
                    self.0.lock().unwrap().write_twi0_events_suspended(
                        buffer_const(_start, _end, 328, _buf),
                        buffer_const(_start, _end, 329, _buf),
                        buffer_const(_start, _end, 330, _buf),
                        buffer_const(_start, _end, 331, _buf),
                    )?;
                }
            }
            (512..=515, 513..=516) => {
                if (_start >= 512 && _start < 516)
                    || (_end > 512 && _end <= 516)
                {
                    self.write_twi0_shorts(
                        buffer_const(_start, _end, 512, _buf),
                        buffer_const(_start, _end, 513, _buf),
                        buffer_const(_start, _end, 514, _buf),
                        buffer_const(_start, _end, 515, _buf),
                    )?;
                }
            }
            (772..=779, 773..=780) => {
                if (_start >= 772 && _start < 776)
                    || (_end > 772 && _end <= 776)
                {
                    self.write_spi0twi0_intenset(
                        buffer_const(_start, _end, 772, _buf),
                        buffer_const(_start, _end, 773, _buf),
                        buffer_const(_start, _end, 774, _buf),
                        buffer_const(_start, _end, 775, _buf),
                    )?;
                }
                if (_start >= 776 && _start < 780)
                    || (_end > 776 && _end <= 780)
                {
                    self.write_spi0twi0_intenclr(
                        buffer_const(_start, _end, 776, _buf),
                        buffer_const(_start, _end, 777, _buf),
                        buffer_const(_start, _end, 778, _buf),
                        buffer_const(_start, _end, 779, _buf),
                    )?;
                }
            }
            (1220..=1223, 1221..=1224) => {
                if (_start >= 1220 && _start < 1224)
                    || (_end > 1220 && _end <= 1224)
                {
                    self.write_twi0_errorsrc(
                        buffer_const(_start, _end, 1220, _buf),
                        buffer_const(_start, _end, 1221, _buf),
                        buffer_const(_start, _end, 1222, _buf),
                        buffer_const(_start, _end, 1223, _buf),
                    )?;
                }
            }
            (1280..=1283, 1281..=1284) => {
                if (_start >= 1280 && _start < 1284)
                    || (_end > 1280 && _end <= 1284)
                {
                    self.write_spi0twi0_enable(
                        buffer_const(_start, _end, 1280, _buf),
                        buffer_const(_start, _end, 1281, _buf),
                        buffer_const(_start, _end, 1282, _buf),
                        buffer_const(_start, _end, 1283, _buf),
                    )?;
                }
            }
            (1288..=1299, 1289..=1300) => {
                if (_start >= 1288 && _start < 1292)
                    || (_end > 1288 && _end <= 1292)
                {
                    self.0.lock().unwrap().write_spi0twi0_pselsck(
                        buffer_const(_start, _end, 1288, _buf),
                        buffer_const(_start, _end, 1289, _buf),
                        buffer_const(_start, _end, 1290, _buf),
                        buffer_const(_start, _end, 1291, _buf),
                    )?;
                }
                if (_start >= 1292 && _start < 1296)
                    || (_end > 1292 && _end <= 1296)
                {
                    self.0.lock().unwrap().write_spi0twi0_pselmosi(
                        buffer_const(_start, _end, 1292, _buf),
                        buffer_const(_start, _end, 1293, _buf),
                        buffer_const(_start, _end, 1294, _buf),
                        buffer_const(_start, _end, 1295, _buf),
                    )?;
                }
                if (_start >= 1296 && _start < 1300)
                    || (_end > 1296 && _end <= 1300)
                {
                    self.0.lock().unwrap().write_spi0_pselmiso(
                        buffer_const(_start, _end, 1296, _buf),
                        buffer_const(_start, _end, 1297, _buf),
                        buffer_const(_start, _end, 1298, _buf),
                        buffer_const(_start, _end, 1299, _buf),
                    )?;
                }
            }
            (1304..=1311, 1305..=1312) => {
                if (_start >= 1304 && _start < 1308)
                    || (_end > 1304 && _end <= 1308)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 1308 && _start < 1312)
                    || (_end > 1308 && _end <= 1312)
                {
                    self.write_spi0twi0_txd(
                        buffer_const(_start, _end, 1308, _buf),
                        buffer_const(_start, _end, 1309, _buf),
                        buffer_const(_start, _end, 1310, _buf),
                        buffer_const(_start, _end, 1311, _buf),
                    )?;
                }
            }
            (1316..=1319, 1317..=1320) => {
                if (_start >= 1316 && _start < 1320)
                    || (_end > 1316 && _end <= 1320)
                {
                    self.write_spi0twi0_frequency(
                        buffer_const(_start, _end, 1316, _buf),
                        buffer_const(_start, _end, 1317, _buf),
                        buffer_const(_start, _end, 1318, _buf),
                        buffer_const(_start, _end, 1319, _buf),
                    )?;
                }
            }
            (1364..=1367, 1365..=1368) => {
                if (_start >= 1364 && _start < 1368)
                    || (_end > 1364 && _end <= 1368)
                {
                    self.write_spi0_config(
                        buffer_const(_start, _end, 1364, _buf),
                        buffer_const(_start, _end, 1365, _buf),
                        buffer_const(_start, _end, 1366, _buf),
                        buffer_const(_start, _end, 1367, _buf),
                    )?;
                }
            }
            (1416..=1419, 1417..=1420) => {
                if (_start >= 1416 && _start < 1420)
                    || (_end > 1416 && _end <= 1420)
                {
                    self.write_twi0_address(
                        buffer_const(_start, _end, 1416, _buf),
                        buffer_const(_start, _end, 1417, _buf),
                        buffer_const(_start, _end, 1418, _buf),
                        buffer_const(_start, _end, 1419, _buf),
                    )?;
                }
            }
            (4092..=4095, 4093..=4096) => {
                if (_start >= 4092 && _start < 4096)
                    || (_end > 4092 && _end <= 4096)
                {
                    self.write_spi0twi0_power(
                        buffer_const(_start, _end, 4092, _buf),
                        buffer_const(_start, _end, 4093, _buf),
                        buffer_const(_start, _end, 4094, _buf),
                        buffer_const(_start, _end, 4095, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
}
impl PeripheralPage0x40003000 {
    fn read_spi0twi0_intenset(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_spi0twi0_intenset_ready()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_spi0twi0_intenset_stopped()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_spi0twi0_intenset_txdsent()? << 7;
        }
        if let Some(byte) = _byte_1 {
            **byte |=
                self.0.lock().unwrap().read_spi0twi0_intenset_error()? << 1;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_spi0twi0_intenset_bb()? << 6;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_spi0twi0_intenset_suspended()? << 2;
        }
        Ok(())
    }
    fn write_spi0twi0_intenset(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_spi0twi0_intenset_ready((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_spi0twi0_intenset_stopped((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_spi0twi0_intenset_txdsent((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_spi0twi0_intenset_error((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_spi0twi0_intenset_bb((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_spi0twi0_intenset_suspended((*byte >> 2) & 1)?;
        }
        Ok(())
    }
    fn read_spi0twi0_intenclr(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_spi0twi0_intenclr_ready()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_spi0twi0_intenclr_stopped()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_spi0twi0_intenclr_txdsent()? << 7;
        }
        if let Some(byte) = _byte_1 {
            **byte |=
                self.0.lock().unwrap().read_spi0twi0_intenclr_error()? << 1;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_spi0twi0_intenclr_bb()? << 6;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_spi0twi0_intenclr_suspended()? << 2;
        }
        Ok(())
    }
    fn write_spi0twi0_intenclr(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_spi0twi0_intenclr_ready((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_spi0twi0_intenclr_stopped((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_spi0twi0_intenclr_txdsent((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_spi0twi0_intenclr_error((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_spi0twi0_intenclr_bb((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_spi0twi0_intenclr_suspended((*byte >> 2) & 1)?;
        }
        Ok(())
    }
    fn read_spi0twi0_enable(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_spi0twi0_enable_enable()? << 0;
        }
        Ok(())
    }
    fn write_spi0twi0_enable(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_spi0twi0_enable_enable((*byte >> 0) & 7)?;
        }
        Ok(())
    }
    fn read_spi0twi0_rxd(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_spi0twi0_rxd_rxd()? << 0;
        }
        Ok(())
    }
    fn read_spi0twi0_txd(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_spi0twi0_txd_txd()? << 0;
        }
        Ok(())
    }
    fn write_spi0twi0_txd(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_spi0twi0_txd_txd((*byte >> 0) & 255)?;
        }
        Ok(())
    }
    fn read_spi0twi0_frequency(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if _byte_0.is_some()
            || _byte_1.is_some()
            || _byte_2.is_some()
            || _byte_3.is_some()
        {
            self.0.lock().unwrap().read_spi0twi0_frequency_frequency(
                _byte_0, _byte_1, _byte_2, _byte_3,
            )?;
        }
        Ok(())
    }
    fn write_spi0twi0_frequency(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if _byte_0.is_some()
            || _byte_1.is_some()
            || _byte_2.is_some()
            || _byte_3.is_some()
        {
            self.0.lock().unwrap().write_spi0twi0_frequency_frequency(
                _byte_0, _byte_1, _byte_2, _byte_3,
            )?;
        }
        Ok(())
    }
    fn read_spi0_config(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_spi0_config_order()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_spi0_config_cpha()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_spi0_config_cpol()? << 2;
        }
        Ok(())
    }
    fn write_spi0_config(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_spi0_config_order((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_spi0_config_cpha((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_spi0_config_cpol((*byte >> 2) & 1)?;
        }
        Ok(())
    }
    fn read_spi0twi0_power(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_spi0twi0_power_power()? << 0;
        }
        Ok(())
    }
    fn write_spi0twi0_power(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_spi0twi0_power_power((*byte >> 0) & 1)?;
        }
        Ok(())
    }
    fn read_twi0_shorts(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_twi0_shorts_bb_suspend()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_twi0_shorts_bb_stop()? << 1;
        }
        Ok(())
    }
    fn write_twi0_shorts(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_twi0_shorts_bb_suspend((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_twi0_shorts_bb_stop((*byte >> 1) & 1)?;
        }
        Ok(())
    }
    fn read_twi0_errorsrc(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_twi0_errorsrc_overrun()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_twi0_errorsrc_anack()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_twi0_errorsrc_dnack()? << 2;
        }
        Ok(())
    }
    fn write_twi0_errorsrc(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_twi0_errorsrc_overrun((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_twi0_errorsrc_anack((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_twi0_errorsrc_dnack((*byte >> 2) & 1)?;
        }
        Ok(())
    }
    fn read_twi0_address(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_twi0_address_address()? << 0;
        }
        Ok(())
    }
    fn write_twi0_address(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_twi0_address_address((*byte >> 0) & 127)?;
        }
        Ok(())
    }
}
pub struct PeripheralPage0x40004000(
    pub std::sync::Arc<std::sync::Mutex<super::peripheral::Peripherals>>,
);
impl icicle_vm::cpu::mem::IoMemory for PeripheralPage0x40004000 {
    fn read(
        &mut self,
        _addr: u64,
        _buf: &mut [u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073758208;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (36..=43, 37..=44) => {
                if (_start >= 36 && _start < 40) || (_end > 36 && _end <= 40) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 40 && _start < 44) || (_end > 40 && _end <= 44) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (260..=263, 261..=264) => {
                if (_start >= 260 && _start < 264)
                    || (_end > 260 && _end <= 264)
                {
                    self.0.lock().unwrap().read_spis1_events_end(
                        &mut buffer_mut(_start, _end, 260, _buf),
                        &mut buffer_mut(_start, _end, 261, _buf),
                        &mut buffer_mut(_start, _end, 262, _buf),
                        &mut buffer_mut(_start, _end, 263, _buf),
                    )?;
                }
            }
            (272..=275, 273..=276) => {
                if (_start >= 272 && _start < 276)
                    || (_end > 272 && _end <= 276)
                {
                    self.0.lock().unwrap().read_spis1_events_endrx(
                        &mut buffer_mut(_start, _end, 272, _buf),
                        &mut buffer_mut(_start, _end, 273, _buf),
                        &mut buffer_mut(_start, _end, 274, _buf),
                        &mut buffer_mut(_start, _end, 275, _buf),
                    )?;
                }
            }
            (296..=299, 297..=300) => {
                if (_start >= 296 && _start < 300)
                    || (_end > 296 && _end <= 300)
                {
                    self.0.lock().unwrap().read_spis1_events_acquired(
                        &mut buffer_mut(_start, _end, 296, _buf),
                        &mut buffer_mut(_start, _end, 297, _buf),
                        &mut buffer_mut(_start, _end, 298, _buf),
                        &mut buffer_mut(_start, _end, 299, _buf),
                    )?;
                }
            }
            (512..=515, 513..=516) => {
                if (_start >= 512 && _start < 516)
                    || (_end > 512 && _end <= 516)
                {
                    self.read_spis1_shorts(
                        &mut buffer_mut(_start, _end, 512, _buf),
                        &mut buffer_mut(_start, _end, 513, _buf),
                        &mut buffer_mut(_start, _end, 514, _buf),
                        &mut buffer_mut(_start, _end, 515, _buf),
                    )?;
                }
            }
            (772..=779, 773..=780) => {
                if (_start >= 772 && _start < 776)
                    || (_end > 772 && _end <= 776)
                {
                    self.read_spis1_intenset(
                        &mut buffer_mut(_start, _end, 772, _buf),
                        &mut buffer_mut(_start, _end, 773, _buf),
                        &mut buffer_mut(_start, _end, 774, _buf),
                        &mut buffer_mut(_start, _end, 775, _buf),
                    )?;
                }
                if (_start >= 776 && _start < 780)
                    || (_end > 776 && _end <= 780)
                {
                    self.read_spis1_intenclr(
                        &mut buffer_mut(_start, _end, 776, _buf),
                        &mut buffer_mut(_start, _end, 777, _buf),
                        &mut buffer_mut(_start, _end, 778, _buf),
                        &mut buffer_mut(_start, _end, 779, _buf),
                    )?;
                }
            }
            (1024..=1027, 1025..=1028) => {
                if (_start >= 1024 && _start < 1028)
                    || (_end > 1024 && _end <= 1028)
                {
                    self.read_spis1_semstat(
                        &mut buffer_mut(_start, _end, 1024, _buf),
                        &mut buffer_mut(_start, _end, 1025, _buf),
                        &mut buffer_mut(_start, _end, 1026, _buf),
                        &mut buffer_mut(_start, _end, 1027, _buf),
                    )?;
                }
            }
            (1088..=1091, 1089..=1092) => {
                if (_start >= 1088 && _start < 1092)
                    || (_end > 1088 && _end <= 1092)
                {
                    self.read_spis1_status(
                        &mut buffer_mut(_start, _end, 1088, _buf),
                        &mut buffer_mut(_start, _end, 1089, _buf),
                        &mut buffer_mut(_start, _end, 1090, _buf),
                        &mut buffer_mut(_start, _end, 1091, _buf),
                    )?;
                }
            }
            (1280..=1283, 1281..=1284) => {
                if (_start >= 1280 && _start < 1284)
                    || (_end > 1280 && _end <= 1284)
                {
                    self.read_spis1_enable(
                        &mut buffer_mut(_start, _end, 1280, _buf),
                        &mut buffer_mut(_start, _end, 1281, _buf),
                        &mut buffer_mut(_start, _end, 1282, _buf),
                        &mut buffer_mut(_start, _end, 1283, _buf),
                    )?;
                }
            }
            (1288..=1303, 1289..=1304) => {
                if (_start >= 1288 && _start < 1292)
                    || (_end > 1288 && _end <= 1292)
                {
                    self.0.lock().unwrap().read_spis1_pselsck(
                        &mut buffer_mut(_start, _end, 1288, _buf),
                        &mut buffer_mut(_start, _end, 1289, _buf),
                        &mut buffer_mut(_start, _end, 1290, _buf),
                        &mut buffer_mut(_start, _end, 1291, _buf),
                    )?;
                }
                if (_start >= 1292 && _start < 1296)
                    || (_end > 1292 && _end <= 1296)
                {
                    self.0.lock().unwrap().read_spis1_pselmiso(
                        &mut buffer_mut(_start, _end, 1292, _buf),
                        &mut buffer_mut(_start, _end, 1293, _buf),
                        &mut buffer_mut(_start, _end, 1294, _buf),
                        &mut buffer_mut(_start, _end, 1295, _buf),
                    )?;
                }
                if (_start >= 1296 && _start < 1300)
                    || (_end > 1296 && _end <= 1300)
                {
                    self.0.lock().unwrap().read_spis1_pselmosi(
                        &mut buffer_mut(_start, _end, 1296, _buf),
                        &mut buffer_mut(_start, _end, 1297, _buf),
                        &mut buffer_mut(_start, _end, 1298, _buf),
                        &mut buffer_mut(_start, _end, 1299, _buf),
                    )?;
                }
                if (_start >= 1300 && _start < 1304)
                    || (_end > 1300 && _end <= 1304)
                {
                    self.0.lock().unwrap().read_spis1_pselcsn(
                        &mut buffer_mut(_start, _end, 1300, _buf),
                        &mut buffer_mut(_start, _end, 1301, _buf),
                        &mut buffer_mut(_start, _end, 1302, _buf),
                        &mut buffer_mut(_start, _end, 1303, _buf),
                    )?;
                }
            }
            (1332..=1343, 1333..=1344) => {
                if (_start >= 1332 && _start < 1336)
                    || (_end > 1332 && _end <= 1336)
                {
                    self.0.lock().unwrap().read_spis1_rxdptr(
                        &mut buffer_mut(_start, _end, 1332, _buf),
                        &mut buffer_mut(_start, _end, 1333, _buf),
                        &mut buffer_mut(_start, _end, 1334, _buf),
                        &mut buffer_mut(_start, _end, 1335, _buf),
                    )?;
                }
                if (_start >= 1336 && _start < 1340)
                    || (_end > 1336 && _end <= 1340)
                {
                    self.read_spis1_maxrx(
                        &mut buffer_mut(_start, _end, 1336, _buf),
                        &mut buffer_mut(_start, _end, 1337, _buf),
                        &mut buffer_mut(_start, _end, 1338, _buf),
                        &mut buffer_mut(_start, _end, 1339, _buf),
                    )?;
                }
                if (_start >= 1340 && _start < 1344)
                    || (_end > 1340 && _end <= 1344)
                {
                    self.read_spis1_amountrx(
                        &mut buffer_mut(_start, _end, 1340, _buf),
                        &mut buffer_mut(_start, _end, 1341, _buf),
                        &mut buffer_mut(_start, _end, 1342, _buf),
                        &mut buffer_mut(_start, _end, 1343, _buf),
                    )?;
                }
            }
            (1348..=1359, 1349..=1360) => {
                if (_start >= 1348 && _start < 1352)
                    || (_end > 1348 && _end <= 1352)
                {
                    self.0.lock().unwrap().read_spis1_txdptr(
                        &mut buffer_mut(_start, _end, 1348, _buf),
                        &mut buffer_mut(_start, _end, 1349, _buf),
                        &mut buffer_mut(_start, _end, 1350, _buf),
                        &mut buffer_mut(_start, _end, 1351, _buf),
                    )?;
                }
                if (_start >= 1352 && _start < 1356)
                    || (_end > 1352 && _end <= 1356)
                {
                    self.read_spis1_maxtx(
                        &mut buffer_mut(_start, _end, 1352, _buf),
                        &mut buffer_mut(_start, _end, 1353, _buf),
                        &mut buffer_mut(_start, _end, 1354, _buf),
                        &mut buffer_mut(_start, _end, 1355, _buf),
                    )?;
                }
                if (_start >= 1356 && _start < 1360)
                    || (_end > 1356 && _end <= 1360)
                {
                    self.read_spis1_amounttx(
                        &mut buffer_mut(_start, _end, 1356, _buf),
                        &mut buffer_mut(_start, _end, 1357, _buf),
                        &mut buffer_mut(_start, _end, 1358, _buf),
                        &mut buffer_mut(_start, _end, 1359, _buf),
                    )?;
                }
            }
            (1364..=1367, 1365..=1368) => {
                if (_start >= 1364 && _start < 1368)
                    || (_end > 1364 && _end <= 1368)
                {
                    self.read_spis1_config(
                        &mut buffer_mut(_start, _end, 1364, _buf),
                        &mut buffer_mut(_start, _end, 1365, _buf),
                        &mut buffer_mut(_start, _end, 1366, _buf),
                        &mut buffer_mut(_start, _end, 1367, _buf),
                    )?;
                }
            }
            (1372..=1375, 1373..=1376) => {
                if (_start >= 1372 && _start < 1376)
                    || (_end > 1372 && _end <= 1376)
                {
                    self.read_spis1_def(
                        &mut buffer_mut(_start, _end, 1372, _buf),
                        &mut buffer_mut(_start, _end, 1373, _buf),
                        &mut buffer_mut(_start, _end, 1374, _buf),
                        &mut buffer_mut(_start, _end, 1375, _buf),
                    )?;
                }
            }
            (1472..=1475, 1473..=1476) => {
                if (_start >= 1472 && _start < 1476)
                    || (_end > 1472 && _end <= 1476)
                {
                    self.read_spis1_orc(
                        &mut buffer_mut(_start, _end, 1472, _buf),
                        &mut buffer_mut(_start, _end, 1473, _buf),
                        &mut buffer_mut(_start, _end, 1474, _buf),
                        &mut buffer_mut(_start, _end, 1475, _buf),
                    )?;
                }
            }
            (4092..=4095, 4093..=4096) => {
                if (_start >= 4092 && _start < 4096)
                    || (_end > 4092 && _end <= 4096)
                {
                    self.read_spis1_power(
                        &mut buffer_mut(_start, _end, 4092, _buf),
                        &mut buffer_mut(_start, _end, 4093, _buf),
                        &mut buffer_mut(_start, _end, 4094, _buf),
                        &mut buffer_mut(_start, _end, 4095, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
    fn write(
        &mut self,
        _addr: u64,
        _buf: &[u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073758208;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (36..=43, 37..=44) => {
                if (_start >= 36 && _start < 40) || (_end > 36 && _end <= 40) {
                    self.0.lock().unwrap().write_spis1_tasks_acquire(
                        buffer_const(_start, _end, 36, _buf),
                        buffer_const(_start, _end, 37, _buf),
                        buffer_const(_start, _end, 38, _buf),
                        buffer_const(_start, _end, 39, _buf),
                    )?;
                }
                if (_start >= 40 && _start < 44) || (_end > 40 && _end <= 44) {
                    self.0.lock().unwrap().write_spis1_tasks_release(
                        buffer_const(_start, _end, 40, _buf),
                        buffer_const(_start, _end, 41, _buf),
                        buffer_const(_start, _end, 42, _buf),
                        buffer_const(_start, _end, 43, _buf),
                    )?;
                }
            }
            (260..=263, 261..=264) => {
                if (_start >= 260 && _start < 264)
                    || (_end > 260 && _end <= 264)
                {
                    self.0.lock().unwrap().write_spis1_events_end(
                        buffer_const(_start, _end, 260, _buf),
                        buffer_const(_start, _end, 261, _buf),
                        buffer_const(_start, _end, 262, _buf),
                        buffer_const(_start, _end, 263, _buf),
                    )?;
                }
            }
            (272..=275, 273..=276) => {
                if (_start >= 272 && _start < 276)
                    || (_end > 272 && _end <= 276)
                {
                    self.0.lock().unwrap().write_spis1_events_endrx(
                        buffer_const(_start, _end, 272, _buf),
                        buffer_const(_start, _end, 273, _buf),
                        buffer_const(_start, _end, 274, _buf),
                        buffer_const(_start, _end, 275, _buf),
                    )?;
                }
            }
            (296..=299, 297..=300) => {
                if (_start >= 296 && _start < 300)
                    || (_end > 296 && _end <= 300)
                {
                    self.0.lock().unwrap().write_spis1_events_acquired(
                        buffer_const(_start, _end, 296, _buf),
                        buffer_const(_start, _end, 297, _buf),
                        buffer_const(_start, _end, 298, _buf),
                        buffer_const(_start, _end, 299, _buf),
                    )?;
                }
            }
            (512..=515, 513..=516) => {
                if (_start >= 512 && _start < 516)
                    || (_end > 512 && _end <= 516)
                {
                    self.write_spis1_shorts(
                        buffer_const(_start, _end, 512, _buf),
                        buffer_const(_start, _end, 513, _buf),
                        buffer_const(_start, _end, 514, _buf),
                        buffer_const(_start, _end, 515, _buf),
                    )?;
                }
            }
            (772..=779, 773..=780) => {
                if (_start >= 772 && _start < 776)
                    || (_end > 772 && _end <= 776)
                {
                    self.write_spis1_intenset(
                        buffer_const(_start, _end, 772, _buf),
                        buffer_const(_start, _end, 773, _buf),
                        buffer_const(_start, _end, 774, _buf),
                        buffer_const(_start, _end, 775, _buf),
                    )?;
                }
                if (_start >= 776 && _start < 780)
                    || (_end > 776 && _end <= 780)
                {
                    self.write_spis1_intenclr(
                        buffer_const(_start, _end, 776, _buf),
                        buffer_const(_start, _end, 777, _buf),
                        buffer_const(_start, _end, 778, _buf),
                        buffer_const(_start, _end, 779, _buf),
                    )?;
                }
            }
            (1024..=1027, 1025..=1028) => {
                if (_start >= 1024 && _start < 1028)
                    || (_end > 1024 && _end <= 1028)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (1088..=1091, 1089..=1092) => {
                if (_start >= 1088 && _start < 1092)
                    || (_end > 1088 && _end <= 1092)
                {
                    self.write_spis1_status(
                        buffer_const(_start, _end, 1088, _buf),
                        buffer_const(_start, _end, 1089, _buf),
                        buffer_const(_start, _end, 1090, _buf),
                        buffer_const(_start, _end, 1091, _buf),
                    )?;
                }
            }
            (1280..=1283, 1281..=1284) => {
                if (_start >= 1280 && _start < 1284)
                    || (_end > 1280 && _end <= 1284)
                {
                    self.write_spis1_enable(
                        buffer_const(_start, _end, 1280, _buf),
                        buffer_const(_start, _end, 1281, _buf),
                        buffer_const(_start, _end, 1282, _buf),
                        buffer_const(_start, _end, 1283, _buf),
                    )?;
                }
            }
            (1288..=1303, 1289..=1304) => {
                if (_start >= 1288 && _start < 1292)
                    || (_end > 1288 && _end <= 1292)
                {
                    self.0.lock().unwrap().write_spis1_pselsck(
                        buffer_const(_start, _end, 1288, _buf),
                        buffer_const(_start, _end, 1289, _buf),
                        buffer_const(_start, _end, 1290, _buf),
                        buffer_const(_start, _end, 1291, _buf),
                    )?;
                }
                if (_start >= 1292 && _start < 1296)
                    || (_end > 1292 && _end <= 1296)
                {
                    self.0.lock().unwrap().write_spis1_pselmiso(
                        buffer_const(_start, _end, 1292, _buf),
                        buffer_const(_start, _end, 1293, _buf),
                        buffer_const(_start, _end, 1294, _buf),
                        buffer_const(_start, _end, 1295, _buf),
                    )?;
                }
                if (_start >= 1296 && _start < 1300)
                    || (_end > 1296 && _end <= 1300)
                {
                    self.0.lock().unwrap().write_spis1_pselmosi(
                        buffer_const(_start, _end, 1296, _buf),
                        buffer_const(_start, _end, 1297, _buf),
                        buffer_const(_start, _end, 1298, _buf),
                        buffer_const(_start, _end, 1299, _buf),
                    )?;
                }
                if (_start >= 1300 && _start < 1304)
                    || (_end > 1300 && _end <= 1304)
                {
                    self.0.lock().unwrap().write_spis1_pselcsn(
                        buffer_const(_start, _end, 1300, _buf),
                        buffer_const(_start, _end, 1301, _buf),
                        buffer_const(_start, _end, 1302, _buf),
                        buffer_const(_start, _end, 1303, _buf),
                    )?;
                }
            }
            (1332..=1343, 1333..=1344) => {
                if (_start >= 1332 && _start < 1336)
                    || (_end > 1332 && _end <= 1336)
                {
                    self.0.lock().unwrap().write_spis1_rxdptr(
                        buffer_const(_start, _end, 1332, _buf),
                        buffer_const(_start, _end, 1333, _buf),
                        buffer_const(_start, _end, 1334, _buf),
                        buffer_const(_start, _end, 1335, _buf),
                    )?;
                }
                if (_start >= 1336 && _start < 1340)
                    || (_end > 1336 && _end <= 1340)
                {
                    self.write_spis1_maxrx(
                        buffer_const(_start, _end, 1336, _buf),
                        buffer_const(_start, _end, 1337, _buf),
                        buffer_const(_start, _end, 1338, _buf),
                        buffer_const(_start, _end, 1339, _buf),
                    )?;
                }
                if (_start >= 1340 && _start < 1344)
                    || (_end > 1340 && _end <= 1344)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (1348..=1359, 1349..=1360) => {
                if (_start >= 1348 && _start < 1352)
                    || (_end > 1348 && _end <= 1352)
                {
                    self.0.lock().unwrap().write_spis1_txdptr(
                        buffer_const(_start, _end, 1348, _buf),
                        buffer_const(_start, _end, 1349, _buf),
                        buffer_const(_start, _end, 1350, _buf),
                        buffer_const(_start, _end, 1351, _buf),
                    )?;
                }
                if (_start >= 1352 && _start < 1356)
                    || (_end > 1352 && _end <= 1356)
                {
                    self.write_spis1_maxtx(
                        buffer_const(_start, _end, 1352, _buf),
                        buffer_const(_start, _end, 1353, _buf),
                        buffer_const(_start, _end, 1354, _buf),
                        buffer_const(_start, _end, 1355, _buf),
                    )?;
                }
                if (_start >= 1356 && _start < 1360)
                    || (_end > 1356 && _end <= 1360)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (1364..=1367, 1365..=1368) => {
                if (_start >= 1364 && _start < 1368)
                    || (_end > 1364 && _end <= 1368)
                {
                    self.write_spis1_config(
                        buffer_const(_start, _end, 1364, _buf),
                        buffer_const(_start, _end, 1365, _buf),
                        buffer_const(_start, _end, 1366, _buf),
                        buffer_const(_start, _end, 1367, _buf),
                    )?;
                }
            }
            (1372..=1375, 1373..=1376) => {
                if (_start >= 1372 && _start < 1376)
                    || (_end > 1372 && _end <= 1376)
                {
                    self.write_spis1_def(
                        buffer_const(_start, _end, 1372, _buf),
                        buffer_const(_start, _end, 1373, _buf),
                        buffer_const(_start, _end, 1374, _buf),
                        buffer_const(_start, _end, 1375, _buf),
                    )?;
                }
            }
            (1472..=1475, 1473..=1476) => {
                if (_start >= 1472 && _start < 1476)
                    || (_end > 1472 && _end <= 1476)
                {
                    self.write_spis1_orc(
                        buffer_const(_start, _end, 1472, _buf),
                        buffer_const(_start, _end, 1473, _buf),
                        buffer_const(_start, _end, 1474, _buf),
                        buffer_const(_start, _end, 1475, _buf),
                    )?;
                }
            }
            (4092..=4095, 4093..=4096) => {
                if (_start >= 4092 && _start < 4096)
                    || (_end > 4092 && _end <= 4096)
                {
                    self.write_spis1_power(
                        buffer_const(_start, _end, 4092, _buf),
                        buffer_const(_start, _end, 4093, _buf),
                        buffer_const(_start, _end, 4094, _buf),
                        buffer_const(_start, _end, 4095, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
}
impl PeripheralPage0x40004000 {
    fn read_spis1_shorts(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_spis1_shorts_end_acquire()? << 2;
        }
        Ok(())
    }
    fn write_spis1_shorts(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_spis1_shorts_end_acquire((*byte >> 2) & 1)?;
        }
        Ok(())
    }
    fn read_spis1_intenset(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_spis1_intenset_end()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_spis1_intenset_endrx()? << 4;
        }
        if let Some(byte) = _byte_1 {
            **byte |=
                self.0.lock().unwrap().read_spis1_intenset_acquired()? << 2;
        }
        Ok(())
    }
    fn write_spis1_intenset(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_spis1_intenset_end((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_spis1_intenset_endrx((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_spis1_intenset_acquired((*byte >> 2) & 1)?;
        }
        Ok(())
    }
    fn read_spis1_intenclr(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_spis1_intenclr_end()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_spis1_intenclr_endrx()? << 4;
        }
        if let Some(byte) = _byte_1 {
            **byte |=
                self.0.lock().unwrap().read_spis1_intenclr_acquired()? << 2;
        }
        Ok(())
    }
    fn write_spis1_intenclr(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_spis1_intenclr_end((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_spis1_intenclr_endrx((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_spis1_intenclr_acquired((*byte >> 2) & 1)?;
        }
        Ok(())
    }
    fn read_spis1_semstat(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_spis1_semstat_semstat()? << 0;
        }
        Ok(())
    }
    fn read_spis1_status(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_spis1_status_overread()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_spis1_status_overflow()? << 1;
        }
        Ok(())
    }
    fn write_spis1_status(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_spis1_status_overread((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_spis1_status_overflow((*byte >> 1) & 1)?;
        }
        Ok(())
    }
    fn read_spis1_enable(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_spis1_enable_enable()? << 0;
        }
        Ok(())
    }
    fn write_spis1_enable(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_spis1_enable_enable((*byte >> 0) & 7)?;
        }
        Ok(())
    }
    fn read_spis1_maxrx(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_spis1_maxrx_maxrx()? << 0;
        }
        Ok(())
    }
    fn write_spis1_maxrx(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_spis1_maxrx_maxrx((*byte >> 0) & 255)?;
        }
        Ok(())
    }
    fn read_spis1_amountrx(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_spis1_amountrx_amountrx()? << 0;
        }
        Ok(())
    }
    fn read_spis1_maxtx(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_spis1_maxtx_maxtx()? << 0;
        }
        Ok(())
    }
    fn write_spis1_maxtx(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_spis1_maxtx_maxtx((*byte >> 0) & 255)?;
        }
        Ok(())
    }
    fn read_spis1_amounttx(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_spis1_amounttx_amounttx()? << 0;
        }
        Ok(())
    }
    fn read_spis1_config(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_spis1_config_order()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_spis1_config_cpha()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_spis1_config_cpol()? << 2;
        }
        Ok(())
    }
    fn write_spis1_config(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_spis1_config_order((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_spis1_config_cpha((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_spis1_config_cpol((*byte >> 2) & 1)?;
        }
        Ok(())
    }
    fn read_spis1_def(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_spis1_def_def()? << 0;
        }
        Ok(())
    }
    fn write_spis1_def(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_spis1_def_def((*byte >> 0) & 255)?;
        }
        Ok(())
    }
    fn read_spis1_orc(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_spis1_orc_orc()? << 0;
        }
        Ok(())
    }
    fn write_spis1_orc(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_spis1_orc_orc((*byte >> 0) & 255)?;
        }
        Ok(())
    }
    fn read_spis1_power(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_spis1_power_power()? << 0;
        }
        Ok(())
    }
    fn write_spis1_power(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_spis1_power_power((*byte >> 0) & 1)?;
        }
        Ok(())
    }
}
pub struct PeripheralPage0x40006000(
    pub std::sync::Arc<std::sync::Mutex<super::peripheral::Peripherals>>,
);
impl icicle_vm::cpu::mem::IoMemory for PeripheralPage0x40006000 {
    fn read(
        &mut self,
        _addr: u64,
        _buf: &mut [u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073766400;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=15, 1..=16) => {
                if (_start < 4) || (_end <= 4) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 4 && _start < 8) || (_end > 4 && _end <= 8) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 8 && _start < 12) || (_end > 8 && _end <= 12) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 12 && _start < 16) || (_end > 12 && _end <= 16) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (256..=271, 257..=272) => {
                if (_start >= 256 && _start < 260)
                    || (_end > 256 && _end <= 260)
                {
                    self.0.lock().unwrap().read_gpiote_events_inn(
                        0,
                        &mut buffer_mut(_start, _end, 256, _buf),
                        &mut buffer_mut(_start, _end, 257, _buf),
                        &mut buffer_mut(_start, _end, 258, _buf),
                        &mut buffer_mut(_start, _end, 259, _buf),
                    )?;
                }
                if (_start >= 260 && _start < 264)
                    || (_end > 260 && _end <= 264)
                {
                    self.0.lock().unwrap().read_gpiote_events_inn(
                        1,
                        &mut buffer_mut(_start, _end, 260, _buf),
                        &mut buffer_mut(_start, _end, 261, _buf),
                        &mut buffer_mut(_start, _end, 262, _buf),
                        &mut buffer_mut(_start, _end, 263, _buf),
                    )?;
                }
                if (_start >= 264 && _start < 268)
                    || (_end > 264 && _end <= 268)
                {
                    self.0.lock().unwrap().read_gpiote_events_inn(
                        2,
                        &mut buffer_mut(_start, _end, 264, _buf),
                        &mut buffer_mut(_start, _end, 265, _buf),
                        &mut buffer_mut(_start, _end, 266, _buf),
                        &mut buffer_mut(_start, _end, 267, _buf),
                    )?;
                }
                if (_start >= 268 && _start < 272)
                    || (_end > 268 && _end <= 272)
                {
                    self.0.lock().unwrap().read_gpiote_events_inn(
                        3,
                        &mut buffer_mut(_start, _end, 268, _buf),
                        &mut buffer_mut(_start, _end, 269, _buf),
                        &mut buffer_mut(_start, _end, 270, _buf),
                        &mut buffer_mut(_start, _end, 271, _buf),
                    )?;
                }
            }
            (380..=383, 381..=384) => {
                if (_start >= 380 && _start < 384)
                    || (_end > 380 && _end <= 384)
                {
                    self.0.lock().unwrap().read_gpiote_events_port(
                        &mut buffer_mut(_start, _end, 380, _buf),
                        &mut buffer_mut(_start, _end, 381, _buf),
                        &mut buffer_mut(_start, _end, 382, _buf),
                        &mut buffer_mut(_start, _end, 383, _buf),
                    )?;
                }
            }
            (772..=779, 773..=780) => {
                if (_start >= 772 && _start < 776)
                    || (_end > 772 && _end <= 776)
                {
                    self.read_gpiote_intenset(
                        &mut buffer_mut(_start, _end, 772, _buf),
                        &mut buffer_mut(_start, _end, 773, _buf),
                        &mut buffer_mut(_start, _end, 774, _buf),
                        &mut buffer_mut(_start, _end, 775, _buf),
                    )?;
                }
                if (_start >= 776 && _start < 780)
                    || (_end > 776 && _end <= 780)
                {
                    self.read_gpiote_intenclr(
                        &mut buffer_mut(_start, _end, 776, _buf),
                        &mut buffer_mut(_start, _end, 777, _buf),
                        &mut buffer_mut(_start, _end, 778, _buf),
                        &mut buffer_mut(_start, _end, 779, _buf),
                    )?;
                }
            }
            (1296..=1311, 1297..=1312) => {
                if (_start >= 1296 && _start < 1300)
                    || (_end > 1296 && _end <= 1300)
                {
                    self.read_gpiote_confign(
                        0,
                        &mut buffer_mut(_start, _end, 1296, _buf),
                        &mut buffer_mut(_start, _end, 1297, _buf),
                        &mut buffer_mut(_start, _end, 1298, _buf),
                        &mut buffer_mut(_start, _end, 1299, _buf),
                    )?;
                }
                if (_start >= 1300 && _start < 1304)
                    || (_end > 1300 && _end <= 1304)
                {
                    self.read_gpiote_confign(
                        1,
                        &mut buffer_mut(_start, _end, 1300, _buf),
                        &mut buffer_mut(_start, _end, 1301, _buf),
                        &mut buffer_mut(_start, _end, 1302, _buf),
                        &mut buffer_mut(_start, _end, 1303, _buf),
                    )?;
                }
                if (_start >= 1304 && _start < 1308)
                    || (_end > 1304 && _end <= 1308)
                {
                    self.read_gpiote_confign(
                        2,
                        &mut buffer_mut(_start, _end, 1304, _buf),
                        &mut buffer_mut(_start, _end, 1305, _buf),
                        &mut buffer_mut(_start, _end, 1306, _buf),
                        &mut buffer_mut(_start, _end, 1307, _buf),
                    )?;
                }
                if (_start >= 1308 && _start < 1312)
                    || (_end > 1308 && _end <= 1312)
                {
                    self.read_gpiote_confign(
                        3,
                        &mut buffer_mut(_start, _end, 1308, _buf),
                        &mut buffer_mut(_start, _end, 1309, _buf),
                        &mut buffer_mut(_start, _end, 1310, _buf),
                        &mut buffer_mut(_start, _end, 1311, _buf),
                    )?;
                }
            }
            (4092..=4095, 4093..=4096) => {
                if (_start >= 4092 && _start < 4096)
                    || (_end > 4092 && _end <= 4096)
                {
                    self.read_gpiote_power(
                        &mut buffer_mut(_start, _end, 4092, _buf),
                        &mut buffer_mut(_start, _end, 4093, _buf),
                        &mut buffer_mut(_start, _end, 4094, _buf),
                        &mut buffer_mut(_start, _end, 4095, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
    fn write(
        &mut self,
        _addr: u64,
        _buf: &[u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073766400;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=15, 1..=16) => {
                if (_start < 4) || (_end <= 4) {
                    self.0.lock().unwrap().write_gpiote_tasks_outn(
                        0,
                        buffer_const(_start, _end, 0, _buf),
                        buffer_const(_start, _end, 1, _buf),
                        buffer_const(_start, _end, 2, _buf),
                        buffer_const(_start, _end, 3, _buf),
                    )?;
                }
                if (_start >= 4 && _start < 8) || (_end > 4 && _end <= 8) {
                    self.0.lock().unwrap().write_gpiote_tasks_outn(
                        1,
                        buffer_const(_start, _end, 4, _buf),
                        buffer_const(_start, _end, 5, _buf),
                        buffer_const(_start, _end, 6, _buf),
                        buffer_const(_start, _end, 7, _buf),
                    )?;
                }
                if (_start >= 8 && _start < 12) || (_end > 8 && _end <= 12) {
                    self.0.lock().unwrap().write_gpiote_tasks_outn(
                        2,
                        buffer_const(_start, _end, 8, _buf),
                        buffer_const(_start, _end, 9, _buf),
                        buffer_const(_start, _end, 10, _buf),
                        buffer_const(_start, _end, 11, _buf),
                    )?;
                }
                if (_start >= 12 && _start < 16) || (_end > 12 && _end <= 16) {
                    self.0.lock().unwrap().write_gpiote_tasks_outn(
                        3,
                        buffer_const(_start, _end, 12, _buf),
                        buffer_const(_start, _end, 13, _buf),
                        buffer_const(_start, _end, 14, _buf),
                        buffer_const(_start, _end, 15, _buf),
                    )?;
                }
            }
            (256..=271, 257..=272) => {
                if (_start >= 256 && _start < 260)
                    || (_end > 256 && _end <= 260)
                {
                    self.0.lock().unwrap().write_gpiote_events_inn(
                        0,
                        buffer_const(_start, _end, 256, _buf),
                        buffer_const(_start, _end, 257, _buf),
                        buffer_const(_start, _end, 258, _buf),
                        buffer_const(_start, _end, 259, _buf),
                    )?;
                }
                if (_start >= 260 && _start < 264)
                    || (_end > 260 && _end <= 264)
                {
                    self.0.lock().unwrap().write_gpiote_events_inn(
                        1,
                        buffer_const(_start, _end, 260, _buf),
                        buffer_const(_start, _end, 261, _buf),
                        buffer_const(_start, _end, 262, _buf),
                        buffer_const(_start, _end, 263, _buf),
                    )?;
                }
                if (_start >= 264 && _start < 268)
                    || (_end > 264 && _end <= 268)
                {
                    self.0.lock().unwrap().write_gpiote_events_inn(
                        2,
                        buffer_const(_start, _end, 264, _buf),
                        buffer_const(_start, _end, 265, _buf),
                        buffer_const(_start, _end, 266, _buf),
                        buffer_const(_start, _end, 267, _buf),
                    )?;
                }
                if (_start >= 268 && _start < 272)
                    || (_end > 268 && _end <= 272)
                {
                    self.0.lock().unwrap().write_gpiote_events_inn(
                        3,
                        buffer_const(_start, _end, 268, _buf),
                        buffer_const(_start, _end, 269, _buf),
                        buffer_const(_start, _end, 270, _buf),
                        buffer_const(_start, _end, 271, _buf),
                    )?;
                }
            }
            (380..=383, 381..=384) => {
                if (_start >= 380 && _start < 384)
                    || (_end > 380 && _end <= 384)
                {
                    self.0.lock().unwrap().write_gpiote_events_port(
                        buffer_const(_start, _end, 380, _buf),
                        buffer_const(_start, _end, 381, _buf),
                        buffer_const(_start, _end, 382, _buf),
                        buffer_const(_start, _end, 383, _buf),
                    )?;
                }
            }
            (772..=779, 773..=780) => {
                if (_start >= 772 && _start < 776)
                    || (_end > 772 && _end <= 776)
                {
                    self.write_gpiote_intenset(
                        buffer_const(_start, _end, 772, _buf),
                        buffer_const(_start, _end, 773, _buf),
                        buffer_const(_start, _end, 774, _buf),
                        buffer_const(_start, _end, 775, _buf),
                    )?;
                }
                if (_start >= 776 && _start < 780)
                    || (_end > 776 && _end <= 780)
                {
                    self.write_gpiote_intenclr(
                        buffer_const(_start, _end, 776, _buf),
                        buffer_const(_start, _end, 777, _buf),
                        buffer_const(_start, _end, 778, _buf),
                        buffer_const(_start, _end, 779, _buf),
                    )?;
                }
            }
            (1296..=1311, 1297..=1312) => {
                if (_start >= 1296 && _start < 1300)
                    || (_end > 1296 && _end <= 1300)
                {
                    self.write_gpiote_confign(
                        0,
                        buffer_const(_start, _end, 1296, _buf),
                        buffer_const(_start, _end, 1297, _buf),
                        buffer_const(_start, _end, 1298, _buf),
                        buffer_const(_start, _end, 1299, _buf),
                    )?;
                }
                if (_start >= 1300 && _start < 1304)
                    || (_end > 1300 && _end <= 1304)
                {
                    self.write_gpiote_confign(
                        1,
                        buffer_const(_start, _end, 1300, _buf),
                        buffer_const(_start, _end, 1301, _buf),
                        buffer_const(_start, _end, 1302, _buf),
                        buffer_const(_start, _end, 1303, _buf),
                    )?;
                }
                if (_start >= 1304 && _start < 1308)
                    || (_end > 1304 && _end <= 1308)
                {
                    self.write_gpiote_confign(
                        2,
                        buffer_const(_start, _end, 1304, _buf),
                        buffer_const(_start, _end, 1305, _buf),
                        buffer_const(_start, _end, 1306, _buf),
                        buffer_const(_start, _end, 1307, _buf),
                    )?;
                }
                if (_start >= 1308 && _start < 1312)
                    || (_end > 1308 && _end <= 1312)
                {
                    self.write_gpiote_confign(
                        3,
                        buffer_const(_start, _end, 1308, _buf),
                        buffer_const(_start, _end, 1309, _buf),
                        buffer_const(_start, _end, 1310, _buf),
                        buffer_const(_start, _end, 1311, _buf),
                    )?;
                }
            }
            (4092..=4095, 4093..=4096) => {
                if (_start >= 4092 && _start < 4096)
                    || (_end > 4092 && _end <= 4096)
                {
                    self.write_gpiote_power(
                        buffer_const(_start, _end, 4092, _buf),
                        buffer_const(_start, _end, 4093, _buf),
                        buffer_const(_start, _end, 4094, _buf),
                        buffer_const(_start, _end, 4095, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
}
impl PeripheralPage0x40006000 {
    fn read_gpiote_intenset(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpiote_intenset_in0()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpiote_intenset_in1()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpiote_intenset_in2()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpiote_intenset_in3()? << 3;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpiote_intenset_port()? << 7;
        }
        Ok(())
    }
    fn write_gpiote_intenset(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpiote_intenset_in0((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpiote_intenset_in1((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpiote_intenset_in2((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpiote_intenset_in3((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpiote_intenset_port((*byte >> 7) & 1)?;
        }
        Ok(())
    }
    fn read_gpiote_intenclr(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpiote_intenclr_in0()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpiote_intenclr_in1()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpiote_intenclr_in2()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpiote_intenclr_in3()? << 3;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpiote_intenclr_port()? << 7;
        }
        Ok(())
    }
    fn write_gpiote_intenclr(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpiote_intenclr_in0((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpiote_intenclr_in1((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpiote_intenclr_in2((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpiote_intenclr_in3((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpiote_intenclr_port((*byte >> 7) & 1)?;
        }
        Ok(())
    }
    fn read_gpiote_confign(
        &self,
        _dim: usize,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_gpiote_confign_mode(_dim)? << 0;
        }
        if let Some(byte) = _byte_1 {
            **byte |=
                self.0.lock().unwrap().read_gpiote_confign_psel(_dim)? << 0;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_gpiote_confign_polarity(_dim)? << 0;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_gpiote_confign_outinit(_dim)? << 4;
        }
        Ok(())
    }
    fn write_gpiote_confign(
        &self,
        _dim: usize,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpiote_confign_mode(_dim, (*byte >> 0) & 3)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpiote_confign_psel(_dim, (*byte >> 0) & 31)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpiote_confign_polarity(_dim, (*byte >> 0) & 3)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpiote_confign_outinit(_dim, (*byte >> 4) & 1)?;
        }
        Ok(())
    }
    fn read_gpiote_power(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpiote_power_power()? << 0;
        }
        Ok(())
    }
    fn write_gpiote_power(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpiote_power_power((*byte >> 0) & 1)?;
        }
        Ok(())
    }
}
pub struct PeripheralPage0x40007000(
    pub std::sync::Arc<std::sync::Mutex<super::peripheral::Peripherals>>,
);
impl icicle_vm::cpu::mem::IoMemory for PeripheralPage0x40007000 {
    fn read(
        &mut self,
        _addr: u64,
        _buf: &mut [u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073770496;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=7, 1..=8) => {
                if (_start < 4) || (_end <= 4) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 4 && _start < 8) || (_end > 4 && _end <= 8) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (256..=259, 257..=260) => {
                if (_start >= 256 && _start < 260)
                    || (_end > 256 && _end <= 260)
                {
                    self.0.lock().unwrap().read_adc_events_end(
                        &mut buffer_mut(_start, _end, 256, _buf),
                        &mut buffer_mut(_start, _end, 257, _buf),
                        &mut buffer_mut(_start, _end, 258, _buf),
                        &mut buffer_mut(_start, _end, 259, _buf),
                    )?;
                }
            }
            (772..=779, 773..=780) => {
                if (_start >= 772 && _start < 776)
                    || (_end > 772 && _end <= 776)
                {
                    self.read_adc_intenset(
                        &mut buffer_mut(_start, _end, 772, _buf),
                        &mut buffer_mut(_start, _end, 773, _buf),
                        &mut buffer_mut(_start, _end, 774, _buf),
                        &mut buffer_mut(_start, _end, 775, _buf),
                    )?;
                }
                if (_start >= 776 && _start < 780)
                    || (_end > 776 && _end <= 780)
                {
                    self.read_adc_intenclr(
                        &mut buffer_mut(_start, _end, 776, _buf),
                        &mut buffer_mut(_start, _end, 777, _buf),
                        &mut buffer_mut(_start, _end, 778, _buf),
                        &mut buffer_mut(_start, _end, 779, _buf),
                    )?;
                }
            }
            (1024..=1027, 1025..=1028) => {
                if (_start >= 1024 && _start < 1028)
                    || (_end > 1024 && _end <= 1028)
                {
                    self.read_adc_busy(
                        &mut buffer_mut(_start, _end, 1024, _buf),
                        &mut buffer_mut(_start, _end, 1025, _buf),
                        &mut buffer_mut(_start, _end, 1026, _buf),
                        &mut buffer_mut(_start, _end, 1027, _buf),
                    )?;
                }
            }
            (1280..=1291, 1281..=1292) => {
                if (_start >= 1280 && _start < 1284)
                    || (_end > 1280 && _end <= 1284)
                {
                    self.read_adc_enable(
                        &mut buffer_mut(_start, _end, 1280, _buf),
                        &mut buffer_mut(_start, _end, 1281, _buf),
                        &mut buffer_mut(_start, _end, 1282, _buf),
                        &mut buffer_mut(_start, _end, 1283, _buf),
                    )?;
                }
                if (_start >= 1284 && _start < 1288)
                    || (_end > 1284 && _end <= 1288)
                {
                    self.read_adc_config(
                        &mut buffer_mut(_start, _end, 1284, _buf),
                        &mut buffer_mut(_start, _end, 1285, _buf),
                        &mut buffer_mut(_start, _end, 1286, _buf),
                        &mut buffer_mut(_start, _end, 1287, _buf),
                    )?;
                }
                if (_start >= 1288 && _start < 1292)
                    || (_end > 1288 && _end <= 1292)
                {
                    self.read_adc_result(
                        &mut buffer_mut(_start, _end, 1288, _buf),
                        &mut buffer_mut(_start, _end, 1289, _buf),
                        &mut buffer_mut(_start, _end, 1290, _buf),
                        &mut buffer_mut(_start, _end, 1291, _buf),
                    )?;
                }
            }
            (4092..=4095, 4093..=4096) => {
                if (_start >= 4092 && _start < 4096)
                    || (_end > 4092 && _end <= 4096)
                {
                    self.read_adc_power(
                        &mut buffer_mut(_start, _end, 4092, _buf),
                        &mut buffer_mut(_start, _end, 4093, _buf),
                        &mut buffer_mut(_start, _end, 4094, _buf),
                        &mut buffer_mut(_start, _end, 4095, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
    fn write(
        &mut self,
        _addr: u64,
        _buf: &[u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073770496;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=7, 1..=8) => {
                if (_start < 4) || (_end <= 4) {
                    self.0.lock().unwrap().write_adc_tasks_start(
                        buffer_const(_start, _end, 0, _buf),
                        buffer_const(_start, _end, 1, _buf),
                        buffer_const(_start, _end, 2, _buf),
                        buffer_const(_start, _end, 3, _buf),
                    )?;
                }
                if (_start >= 4 && _start < 8) || (_end > 4 && _end <= 8) {
                    self.0.lock().unwrap().write_adc_tasks_stop(
                        buffer_const(_start, _end, 4, _buf),
                        buffer_const(_start, _end, 5, _buf),
                        buffer_const(_start, _end, 6, _buf),
                        buffer_const(_start, _end, 7, _buf),
                    )?;
                }
            }
            (256..=259, 257..=260) => {
                if (_start >= 256 && _start < 260)
                    || (_end > 256 && _end <= 260)
                {
                    self.0.lock().unwrap().write_adc_events_end(
                        buffer_const(_start, _end, 256, _buf),
                        buffer_const(_start, _end, 257, _buf),
                        buffer_const(_start, _end, 258, _buf),
                        buffer_const(_start, _end, 259, _buf),
                    )?;
                }
            }
            (772..=779, 773..=780) => {
                if (_start >= 772 && _start < 776)
                    || (_end > 772 && _end <= 776)
                {
                    self.write_adc_intenset(
                        buffer_const(_start, _end, 772, _buf),
                        buffer_const(_start, _end, 773, _buf),
                        buffer_const(_start, _end, 774, _buf),
                        buffer_const(_start, _end, 775, _buf),
                    )?;
                }
                if (_start >= 776 && _start < 780)
                    || (_end > 776 && _end <= 780)
                {
                    self.write_adc_intenclr(
                        buffer_const(_start, _end, 776, _buf),
                        buffer_const(_start, _end, 777, _buf),
                        buffer_const(_start, _end, 778, _buf),
                        buffer_const(_start, _end, 779, _buf),
                    )?;
                }
            }
            (1024..=1027, 1025..=1028) => {
                if (_start >= 1024 && _start < 1028)
                    || (_end > 1024 && _end <= 1028)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (1280..=1291, 1281..=1292) => {
                if (_start >= 1280 && _start < 1284)
                    || (_end > 1280 && _end <= 1284)
                {
                    self.write_adc_enable(
                        buffer_const(_start, _end, 1280, _buf),
                        buffer_const(_start, _end, 1281, _buf),
                        buffer_const(_start, _end, 1282, _buf),
                        buffer_const(_start, _end, 1283, _buf),
                    )?;
                }
                if (_start >= 1284 && _start < 1288)
                    || (_end > 1284 && _end <= 1288)
                {
                    self.write_adc_config(
                        buffer_const(_start, _end, 1284, _buf),
                        buffer_const(_start, _end, 1285, _buf),
                        buffer_const(_start, _end, 1286, _buf),
                        buffer_const(_start, _end, 1287, _buf),
                    )?;
                }
                if (_start >= 1288 && _start < 1292)
                    || (_end > 1288 && _end <= 1292)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (4092..=4095, 4093..=4096) => {
                if (_start >= 4092 && _start < 4096)
                    || (_end > 4092 && _end <= 4096)
                {
                    self.write_adc_power(
                        buffer_const(_start, _end, 4092, _buf),
                        buffer_const(_start, _end, 4093, _buf),
                        buffer_const(_start, _end, 4094, _buf),
                        buffer_const(_start, _end, 4095, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
}
impl PeripheralPage0x40007000 {
    fn read_adc_intenset(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_adc_intenset_end()? << 0;
        }
        Ok(())
    }
    fn write_adc_intenset(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_adc_intenset_end((*byte >> 0) & 1)?;
        }
        Ok(())
    }
    fn read_adc_intenclr(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_adc_intenclr_end()? << 0;
        }
        Ok(())
    }
    fn write_adc_intenclr(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_adc_intenclr_end((*byte >> 0) & 1)?;
        }
        Ok(())
    }
    fn read_adc_busy(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_adc_busy_busy()? << 0;
        }
        Ok(())
    }
    fn read_adc_enable(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_adc_enable_enable()? << 0;
        }
        Ok(())
    }
    fn write_adc_enable(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_adc_enable_enable((*byte >> 0) & 3)?;
        }
        Ok(())
    }
    fn read_adc_config(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_adc_config_res()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_adc_config_inpsel()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_adc_config_refsel()? << 5;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_adc_config_psel()? << 0;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_adc_config_extrefsel()? << 0;
        }
        Ok(())
    }
    fn write_adc_config(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_adc_config_res((*byte >> 0) & 3)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_adc_config_inpsel((*byte >> 2) & 7)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_adc_config_refsel((*byte >> 5) & 3)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_adc_config_psel((*byte >> 0) & 255)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_adc_config_extrefsel((*byte >> 0) & 3)?;
        }
        Ok(())
    }
    fn read_adc_result(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if _byte_0.is_some() || _byte_1.is_some() {
            self.0
                .lock()
                .unwrap()
                .read_adc_result_result(_byte_0, _byte_1)?;
        }
        Ok(())
    }
    fn read_adc_power(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_adc_power_power()? << 0;
        }
        Ok(())
    }
    fn write_adc_power(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_adc_power_power((*byte >> 0) & 1)?;
        }
        Ok(())
    }
}
pub struct PeripheralPage0x40008000(
    pub std::sync::Arc<std::sync::Mutex<super::peripheral::Peripherals>>,
);
impl icicle_vm::cpu::mem::IoMemory for PeripheralPage0x40008000 {
    fn read(
        &mut self,
        _addr: u64,
        _buf: &mut [u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073774592;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=19, 1..=20) => {
                if (_start < 4) || (_end <= 4) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 4 && _start < 8) || (_end > 4 && _end <= 8) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 8 && _start < 12) || (_end > 8 && _end <= 12) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 12 && _start < 16) || (_end > 12 && _end <= 16) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 16 && _start < 20) || (_end > 16 && _end <= 20) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (64..=79, 65..=80) => {
                if (_start >= 64 && _start < 68) || (_end > 64 && _end <= 68) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 68 && _start < 72) || (_end > 68 && _end <= 72) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 72 && _start < 76) || (_end > 72 && _end <= 76) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 76 && _start < 80) || (_end > 76 && _end <= 80) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (320..=335, 321..=336) => {
                if (_start >= 320 && _start < 324)
                    || (_end > 320 && _end <= 324)
                {
                    self.0.lock().unwrap().read_timer0_events_comparen(
                        0,
                        &mut buffer_mut(_start, _end, 320, _buf),
                        &mut buffer_mut(_start, _end, 321, _buf),
                        &mut buffer_mut(_start, _end, 322, _buf),
                        &mut buffer_mut(_start, _end, 323, _buf),
                    )?;
                }
                if (_start >= 324 && _start < 328)
                    || (_end > 324 && _end <= 328)
                {
                    self.0.lock().unwrap().read_timer0_events_comparen(
                        1,
                        &mut buffer_mut(_start, _end, 324, _buf),
                        &mut buffer_mut(_start, _end, 325, _buf),
                        &mut buffer_mut(_start, _end, 326, _buf),
                        &mut buffer_mut(_start, _end, 327, _buf),
                    )?;
                }
                if (_start >= 328 && _start < 332)
                    || (_end > 328 && _end <= 332)
                {
                    self.0.lock().unwrap().read_timer0_events_comparen(
                        2,
                        &mut buffer_mut(_start, _end, 328, _buf),
                        &mut buffer_mut(_start, _end, 329, _buf),
                        &mut buffer_mut(_start, _end, 330, _buf),
                        &mut buffer_mut(_start, _end, 331, _buf),
                    )?;
                }
                if (_start >= 332 && _start < 336)
                    || (_end > 332 && _end <= 336)
                {
                    self.0.lock().unwrap().read_timer0_events_comparen(
                        3,
                        &mut buffer_mut(_start, _end, 332, _buf),
                        &mut buffer_mut(_start, _end, 333, _buf),
                        &mut buffer_mut(_start, _end, 334, _buf),
                        &mut buffer_mut(_start, _end, 335, _buf),
                    )?;
                }
            }
            (512..=515, 513..=516) => {
                if (_start >= 512 && _start < 516)
                    || (_end > 512 && _end <= 516)
                {
                    self.read_timer0_shorts(
                        &mut buffer_mut(_start, _end, 512, _buf),
                        &mut buffer_mut(_start, _end, 513, _buf),
                        &mut buffer_mut(_start, _end, 514, _buf),
                        &mut buffer_mut(_start, _end, 515, _buf),
                    )?;
                }
            }
            (772..=779, 773..=780) => {
                if (_start >= 772 && _start < 776)
                    || (_end > 772 && _end <= 776)
                {
                    self.read_timer0_intenset(
                        &mut buffer_mut(_start, _end, 772, _buf),
                        &mut buffer_mut(_start, _end, 773, _buf),
                        &mut buffer_mut(_start, _end, 774, _buf),
                        &mut buffer_mut(_start, _end, 775, _buf),
                    )?;
                }
                if (_start >= 776 && _start < 780)
                    || (_end > 776 && _end <= 780)
                {
                    self.read_timer0_intenclr(
                        &mut buffer_mut(_start, _end, 776, _buf),
                        &mut buffer_mut(_start, _end, 777, _buf),
                        &mut buffer_mut(_start, _end, 778, _buf),
                        &mut buffer_mut(_start, _end, 779, _buf),
                    )?;
                }
            }
            (1284..=1291, 1285..=1292) => {
                if (_start >= 1284 && _start < 1288)
                    || (_end > 1284 && _end <= 1288)
                {
                    self.read_timer0_mode(
                        &mut buffer_mut(_start, _end, 1284, _buf),
                        &mut buffer_mut(_start, _end, 1285, _buf),
                        &mut buffer_mut(_start, _end, 1286, _buf),
                        &mut buffer_mut(_start, _end, 1287, _buf),
                    )?;
                }
                if (_start >= 1288 && _start < 1292)
                    || (_end > 1288 && _end <= 1292)
                {
                    self.read_timer0_bitmode(
                        &mut buffer_mut(_start, _end, 1288, _buf),
                        &mut buffer_mut(_start, _end, 1289, _buf),
                        &mut buffer_mut(_start, _end, 1290, _buf),
                        &mut buffer_mut(_start, _end, 1291, _buf),
                    )?;
                }
            }
            (1296..=1299, 1297..=1300) => {
                if (_start >= 1296 && _start < 1300)
                    || (_end > 1296 && _end <= 1300)
                {
                    self.read_timer0_prescaler(
                        &mut buffer_mut(_start, _end, 1296, _buf),
                        &mut buffer_mut(_start, _end, 1297, _buf),
                        &mut buffer_mut(_start, _end, 1298, _buf),
                        &mut buffer_mut(_start, _end, 1299, _buf),
                    )?;
                }
            }
            (1344..=1359, 1345..=1360) => {
                if (_start >= 1344 && _start < 1348)
                    || (_end > 1344 && _end <= 1348)
                {
                    self.0.lock().unwrap().read_timer0_ccn(
                        0,
                        &mut buffer_mut(_start, _end, 1344, _buf),
                        &mut buffer_mut(_start, _end, 1345, _buf),
                        &mut buffer_mut(_start, _end, 1346, _buf),
                        &mut buffer_mut(_start, _end, 1347, _buf),
                    )?;
                }
                if (_start >= 1348 && _start < 1352)
                    || (_end > 1348 && _end <= 1352)
                {
                    self.0.lock().unwrap().read_timer0_ccn(
                        1,
                        &mut buffer_mut(_start, _end, 1348, _buf),
                        &mut buffer_mut(_start, _end, 1349, _buf),
                        &mut buffer_mut(_start, _end, 1350, _buf),
                        &mut buffer_mut(_start, _end, 1351, _buf),
                    )?;
                }
                if (_start >= 1352 && _start < 1356)
                    || (_end > 1352 && _end <= 1356)
                {
                    self.0.lock().unwrap().read_timer0_ccn(
                        2,
                        &mut buffer_mut(_start, _end, 1352, _buf),
                        &mut buffer_mut(_start, _end, 1353, _buf),
                        &mut buffer_mut(_start, _end, 1354, _buf),
                        &mut buffer_mut(_start, _end, 1355, _buf),
                    )?;
                }
                if (_start >= 1356 && _start < 1360)
                    || (_end > 1356 && _end <= 1360)
                {
                    self.0.lock().unwrap().read_timer0_ccn(
                        3,
                        &mut buffer_mut(_start, _end, 1356, _buf),
                        &mut buffer_mut(_start, _end, 1357, _buf),
                        &mut buffer_mut(_start, _end, 1358, _buf),
                        &mut buffer_mut(_start, _end, 1359, _buf),
                    )?;
                }
            }
            (4092..=4095, 4093..=4096) => {
                if (_start >= 4092 && _start < 4096)
                    || (_end > 4092 && _end <= 4096)
                {
                    self.read_timer0_power(
                        &mut buffer_mut(_start, _end, 4092, _buf),
                        &mut buffer_mut(_start, _end, 4093, _buf),
                        &mut buffer_mut(_start, _end, 4094, _buf),
                        &mut buffer_mut(_start, _end, 4095, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
    fn write(
        &mut self,
        _addr: u64,
        _buf: &[u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073774592;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=19, 1..=20) => {
                if (_start < 4) || (_end <= 4) {
                    self.0.lock().unwrap().write_timer0_tasks_start(
                        buffer_const(_start, _end, 0, _buf),
                        buffer_const(_start, _end, 1, _buf),
                        buffer_const(_start, _end, 2, _buf),
                        buffer_const(_start, _end, 3, _buf),
                    )?;
                }
                if (_start >= 4 && _start < 8) || (_end > 4 && _end <= 8) {
                    self.0.lock().unwrap().write_timer0_tasks_stop(
                        buffer_const(_start, _end, 4, _buf),
                        buffer_const(_start, _end, 5, _buf),
                        buffer_const(_start, _end, 6, _buf),
                        buffer_const(_start, _end, 7, _buf),
                    )?;
                }
                if (_start >= 8 && _start < 12) || (_end > 8 && _end <= 12) {
                    self.0.lock().unwrap().write_timer0_tasks_count(
                        buffer_const(_start, _end, 8, _buf),
                        buffer_const(_start, _end, 9, _buf),
                        buffer_const(_start, _end, 10, _buf),
                        buffer_const(_start, _end, 11, _buf),
                    )?;
                }
                if (_start >= 12 && _start < 16) || (_end > 12 && _end <= 16) {
                    self.0.lock().unwrap().write_timer0_tasks_clear(
                        buffer_const(_start, _end, 12, _buf),
                        buffer_const(_start, _end, 13, _buf),
                        buffer_const(_start, _end, 14, _buf),
                        buffer_const(_start, _end, 15, _buf),
                    )?;
                }
                if (_start >= 16 && _start < 20) || (_end > 16 && _end <= 20) {
                    self.0.lock().unwrap().write_timer0_tasks_shutdown(
                        buffer_const(_start, _end, 16, _buf),
                        buffer_const(_start, _end, 17, _buf),
                        buffer_const(_start, _end, 18, _buf),
                        buffer_const(_start, _end, 19, _buf),
                    )?;
                }
            }
            (64..=79, 65..=80) => {
                if (_start >= 64 && _start < 68) || (_end > 64 && _end <= 68) {
                    self.0.lock().unwrap().write_timer0_tasks_capturen(
                        0,
                        buffer_const(_start, _end, 64, _buf),
                        buffer_const(_start, _end, 65, _buf),
                        buffer_const(_start, _end, 66, _buf),
                        buffer_const(_start, _end, 67, _buf),
                    )?;
                }
                if (_start >= 68 && _start < 72) || (_end > 68 && _end <= 72) {
                    self.0.lock().unwrap().write_timer0_tasks_capturen(
                        1,
                        buffer_const(_start, _end, 68, _buf),
                        buffer_const(_start, _end, 69, _buf),
                        buffer_const(_start, _end, 70, _buf),
                        buffer_const(_start, _end, 71, _buf),
                    )?;
                }
                if (_start >= 72 && _start < 76) || (_end > 72 && _end <= 76) {
                    self.0.lock().unwrap().write_timer0_tasks_capturen(
                        2,
                        buffer_const(_start, _end, 72, _buf),
                        buffer_const(_start, _end, 73, _buf),
                        buffer_const(_start, _end, 74, _buf),
                        buffer_const(_start, _end, 75, _buf),
                    )?;
                }
                if (_start >= 76 && _start < 80) || (_end > 76 && _end <= 80) {
                    self.0.lock().unwrap().write_timer0_tasks_capturen(
                        3,
                        buffer_const(_start, _end, 76, _buf),
                        buffer_const(_start, _end, 77, _buf),
                        buffer_const(_start, _end, 78, _buf),
                        buffer_const(_start, _end, 79, _buf),
                    )?;
                }
            }
            (320..=335, 321..=336) => {
                if (_start >= 320 && _start < 324)
                    || (_end > 320 && _end <= 324)
                {
                    self.0.lock().unwrap().write_timer0_events_comparen(
                        0,
                        buffer_const(_start, _end, 320, _buf),
                        buffer_const(_start, _end, 321, _buf),
                        buffer_const(_start, _end, 322, _buf),
                        buffer_const(_start, _end, 323, _buf),
                    )?;
                }
                if (_start >= 324 && _start < 328)
                    || (_end > 324 && _end <= 328)
                {
                    self.0.lock().unwrap().write_timer0_events_comparen(
                        1,
                        buffer_const(_start, _end, 324, _buf),
                        buffer_const(_start, _end, 325, _buf),
                        buffer_const(_start, _end, 326, _buf),
                        buffer_const(_start, _end, 327, _buf),
                    )?;
                }
                if (_start >= 328 && _start < 332)
                    || (_end > 328 && _end <= 332)
                {
                    self.0.lock().unwrap().write_timer0_events_comparen(
                        2,
                        buffer_const(_start, _end, 328, _buf),
                        buffer_const(_start, _end, 329, _buf),
                        buffer_const(_start, _end, 330, _buf),
                        buffer_const(_start, _end, 331, _buf),
                    )?;
                }
                if (_start >= 332 && _start < 336)
                    || (_end > 332 && _end <= 336)
                {
                    self.0.lock().unwrap().write_timer0_events_comparen(
                        3,
                        buffer_const(_start, _end, 332, _buf),
                        buffer_const(_start, _end, 333, _buf),
                        buffer_const(_start, _end, 334, _buf),
                        buffer_const(_start, _end, 335, _buf),
                    )?;
                }
            }
            (512..=515, 513..=516) => {
                if (_start >= 512 && _start < 516)
                    || (_end > 512 && _end <= 516)
                {
                    self.write_timer0_shorts(
                        buffer_const(_start, _end, 512, _buf),
                        buffer_const(_start, _end, 513, _buf),
                        buffer_const(_start, _end, 514, _buf),
                        buffer_const(_start, _end, 515, _buf),
                    )?;
                }
            }
            (772..=779, 773..=780) => {
                if (_start >= 772 && _start < 776)
                    || (_end > 772 && _end <= 776)
                {
                    self.write_timer0_intenset(
                        buffer_const(_start, _end, 772, _buf),
                        buffer_const(_start, _end, 773, _buf),
                        buffer_const(_start, _end, 774, _buf),
                        buffer_const(_start, _end, 775, _buf),
                    )?;
                }
                if (_start >= 776 && _start < 780)
                    || (_end > 776 && _end <= 780)
                {
                    self.write_timer0_intenclr(
                        buffer_const(_start, _end, 776, _buf),
                        buffer_const(_start, _end, 777, _buf),
                        buffer_const(_start, _end, 778, _buf),
                        buffer_const(_start, _end, 779, _buf),
                    )?;
                }
            }
            (1284..=1291, 1285..=1292) => {
                if (_start >= 1284 && _start < 1288)
                    || (_end > 1284 && _end <= 1288)
                {
                    self.write_timer0_mode(
                        buffer_const(_start, _end, 1284, _buf),
                        buffer_const(_start, _end, 1285, _buf),
                        buffer_const(_start, _end, 1286, _buf),
                        buffer_const(_start, _end, 1287, _buf),
                    )?;
                }
                if (_start >= 1288 && _start < 1292)
                    || (_end > 1288 && _end <= 1292)
                {
                    self.write_timer0_bitmode(
                        buffer_const(_start, _end, 1288, _buf),
                        buffer_const(_start, _end, 1289, _buf),
                        buffer_const(_start, _end, 1290, _buf),
                        buffer_const(_start, _end, 1291, _buf),
                    )?;
                }
            }
            (1296..=1299, 1297..=1300) => {
                if (_start >= 1296 && _start < 1300)
                    || (_end > 1296 && _end <= 1300)
                {
                    self.write_timer0_prescaler(
                        buffer_const(_start, _end, 1296, _buf),
                        buffer_const(_start, _end, 1297, _buf),
                        buffer_const(_start, _end, 1298, _buf),
                        buffer_const(_start, _end, 1299, _buf),
                    )?;
                }
            }
            (1344..=1359, 1345..=1360) => {
                if (_start >= 1344 && _start < 1348)
                    || (_end > 1344 && _end <= 1348)
                {
                    self.0.lock().unwrap().write_timer0_ccn(
                        0,
                        buffer_const(_start, _end, 1344, _buf),
                        buffer_const(_start, _end, 1345, _buf),
                        buffer_const(_start, _end, 1346, _buf),
                        buffer_const(_start, _end, 1347, _buf),
                    )?;
                }
                if (_start >= 1348 && _start < 1352)
                    || (_end > 1348 && _end <= 1352)
                {
                    self.0.lock().unwrap().write_timer0_ccn(
                        1,
                        buffer_const(_start, _end, 1348, _buf),
                        buffer_const(_start, _end, 1349, _buf),
                        buffer_const(_start, _end, 1350, _buf),
                        buffer_const(_start, _end, 1351, _buf),
                    )?;
                }
                if (_start >= 1352 && _start < 1356)
                    || (_end > 1352 && _end <= 1356)
                {
                    self.0.lock().unwrap().write_timer0_ccn(
                        2,
                        buffer_const(_start, _end, 1352, _buf),
                        buffer_const(_start, _end, 1353, _buf),
                        buffer_const(_start, _end, 1354, _buf),
                        buffer_const(_start, _end, 1355, _buf),
                    )?;
                }
                if (_start >= 1356 && _start < 1360)
                    || (_end > 1356 && _end <= 1360)
                {
                    self.0.lock().unwrap().write_timer0_ccn(
                        3,
                        buffer_const(_start, _end, 1356, _buf),
                        buffer_const(_start, _end, 1357, _buf),
                        buffer_const(_start, _end, 1358, _buf),
                        buffer_const(_start, _end, 1359, _buf),
                    )?;
                }
            }
            (4092..=4095, 4093..=4096) => {
                if (_start >= 4092 && _start < 4096)
                    || (_end > 4092 && _end <= 4096)
                {
                    self.write_timer0_power(
                        buffer_const(_start, _end, 4092, _buf),
                        buffer_const(_start, _end, 4093, _buf),
                        buffer_const(_start, _end, 4094, _buf),
                        buffer_const(_start, _end, 4095, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
}
impl PeripheralPage0x40008000 {
    fn read_timer0_shorts(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_timer0_shorts_compare0_clear()?
                    << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_timer0_shorts_compare1_clear()?
                    << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_timer0_shorts_compare2_clear()?
                    << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_timer0_shorts_compare3_clear()?
                    << 3;
        }
        if let Some(byte) = _byte_1 {
            **byte |=
                self.0.lock().unwrap().read_timer0_shorts_compare0_stop()? << 0;
        }
        if let Some(byte) = _byte_1 {
            **byte |=
                self.0.lock().unwrap().read_timer0_shorts_compare1_stop()? << 1;
        }
        if let Some(byte) = _byte_1 {
            **byte |=
                self.0.lock().unwrap().read_timer0_shorts_compare2_stop()? << 2;
        }
        if let Some(byte) = _byte_1 {
            **byte |=
                self.0.lock().unwrap().read_timer0_shorts_compare3_stop()? << 3;
        }
        Ok(())
    }
    fn write_timer0_shorts(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_timer0_shorts_compare0_clear((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_timer0_shorts_compare1_clear((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_timer0_shorts_compare2_clear((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_timer0_shorts_compare3_clear((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_timer0_shorts_compare0_stop((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_timer0_shorts_compare1_stop((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_timer0_shorts_compare2_stop((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_timer0_shorts_compare3_stop((*byte >> 3) & 1)?;
        }
        Ok(())
    }
    fn read_timer0_intenset(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_timer0_intenset_compare0()? << 0;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_timer0_intenset_compare1()? << 1;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_timer0_intenset_compare2()? << 2;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_timer0_intenset_compare3()? << 3;
        }
        Ok(())
    }
    fn write_timer0_intenset(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_timer0_intenset_compare0((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_timer0_intenset_compare1((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_timer0_intenset_compare2((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_timer0_intenset_compare3((*byte >> 3) & 1)?;
        }
        Ok(())
    }
    fn read_timer0_intenclr(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_timer0_intenclr_compare0()? << 0;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_timer0_intenclr_compare1()? << 1;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_timer0_intenclr_compare2()? << 2;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_timer0_intenclr_compare3()? << 3;
        }
        Ok(())
    }
    fn write_timer0_intenclr(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_timer0_intenclr_compare0((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_timer0_intenclr_compare1((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_timer0_intenclr_compare2((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_timer0_intenclr_compare3((*byte >> 3) & 1)?;
        }
        Ok(())
    }
    fn read_timer0_mode(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_timer0_mode_mode()? << 0;
        }
        Ok(())
    }
    fn write_timer0_mode(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_timer0_mode_mode((*byte >> 0) & 1)?;
        }
        Ok(())
    }
    fn read_timer0_bitmode(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_timer0_bitmode_bitmode()? << 0;
        }
        Ok(())
    }
    fn write_timer0_bitmode(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_timer0_bitmode_bitmode((*byte >> 0) & 3)?;
        }
        Ok(())
    }
    fn read_timer0_prescaler(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_timer0_prescaler_prescaler()? << 0;
        }
        Ok(())
    }
    fn write_timer0_prescaler(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_timer0_prescaler_prescaler((*byte >> 0) & 15)?;
        }
        Ok(())
    }
    fn read_timer0_power(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_timer0_power_power()? << 0;
        }
        Ok(())
    }
    fn write_timer0_power(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_timer0_power_power((*byte >> 0) & 1)?;
        }
        Ok(())
    }
}
pub struct PeripheralPage0x4000B000(
    pub std::sync::Arc<std::sync::Mutex<super::peripheral::Peripherals>>,
);
impl icicle_vm::cpu::mem::IoMemory for PeripheralPage0x4000B000 {
    fn read(
        &mut self,
        _addr: u64,
        _buf: &mut [u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073786880;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=15, 1..=16) => {
                if (_start < 4) || (_end <= 4) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 4 && _start < 8) || (_end > 4 && _end <= 8) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 8 && _start < 12) || (_end > 8 && _end <= 12) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 12 && _start < 16) || (_end > 12 && _end <= 16) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (256..=263, 257..=264) => {
                if (_start >= 256 && _start < 260)
                    || (_end > 256 && _end <= 260)
                {
                    self.0.lock().unwrap().read_rtc0_events_tick(
                        &mut buffer_mut(_start, _end, 256, _buf),
                        &mut buffer_mut(_start, _end, 257, _buf),
                        &mut buffer_mut(_start, _end, 258, _buf),
                        &mut buffer_mut(_start, _end, 259, _buf),
                    )?;
                }
                if (_start >= 260 && _start < 264)
                    || (_end > 260 && _end <= 264)
                {
                    self.0.lock().unwrap().read_rtc0_events_ovrflw(
                        &mut buffer_mut(_start, _end, 260, _buf),
                        &mut buffer_mut(_start, _end, 261, _buf),
                        &mut buffer_mut(_start, _end, 262, _buf),
                        &mut buffer_mut(_start, _end, 263, _buf),
                    )?;
                }
            }
            (320..=335, 321..=336) => {
                if (_start >= 320 && _start < 324)
                    || (_end > 320 && _end <= 324)
                {
                    self.0.lock().unwrap().read_rtc0_events_comparen(
                        0,
                        &mut buffer_mut(_start, _end, 320, _buf),
                        &mut buffer_mut(_start, _end, 321, _buf),
                        &mut buffer_mut(_start, _end, 322, _buf),
                        &mut buffer_mut(_start, _end, 323, _buf),
                    )?;
                }
                if (_start >= 324 && _start < 328)
                    || (_end > 324 && _end <= 328)
                {
                    self.0.lock().unwrap().read_rtc0_events_comparen(
                        1,
                        &mut buffer_mut(_start, _end, 324, _buf),
                        &mut buffer_mut(_start, _end, 325, _buf),
                        &mut buffer_mut(_start, _end, 326, _buf),
                        &mut buffer_mut(_start, _end, 327, _buf),
                    )?;
                }
                if (_start >= 328 && _start < 332)
                    || (_end > 328 && _end <= 332)
                {
                    self.0.lock().unwrap().read_rtc0_events_comparen(
                        2,
                        &mut buffer_mut(_start, _end, 328, _buf),
                        &mut buffer_mut(_start, _end, 329, _buf),
                        &mut buffer_mut(_start, _end, 330, _buf),
                        &mut buffer_mut(_start, _end, 331, _buf),
                    )?;
                }
                if (_start >= 332 && _start < 336)
                    || (_end > 332 && _end <= 336)
                {
                    self.0.lock().unwrap().read_rtc0_events_comparen(
                        3,
                        &mut buffer_mut(_start, _end, 332, _buf),
                        &mut buffer_mut(_start, _end, 333, _buf),
                        &mut buffer_mut(_start, _end, 334, _buf),
                        &mut buffer_mut(_start, _end, 335, _buf),
                    )?;
                }
            }
            (772..=779, 773..=780) => {
                if (_start >= 772 && _start < 776)
                    || (_end > 772 && _end <= 776)
                {
                    self.read_rtc0_intenset(
                        &mut buffer_mut(_start, _end, 772, _buf),
                        &mut buffer_mut(_start, _end, 773, _buf),
                        &mut buffer_mut(_start, _end, 774, _buf),
                        &mut buffer_mut(_start, _end, 775, _buf),
                    )?;
                }
                if (_start >= 776 && _start < 780)
                    || (_end > 776 && _end <= 780)
                {
                    self.read_rtc0_intenclr(
                        &mut buffer_mut(_start, _end, 776, _buf),
                        &mut buffer_mut(_start, _end, 777, _buf),
                        &mut buffer_mut(_start, _end, 778, _buf),
                        &mut buffer_mut(_start, _end, 779, _buf),
                    )?;
                }
            }
            (832..=843, 833..=844) => {
                if (_start >= 832 && _start < 836)
                    || (_end > 832 && _end <= 836)
                {
                    self.read_rtc0_evten(
                        &mut buffer_mut(_start, _end, 832, _buf),
                        &mut buffer_mut(_start, _end, 833, _buf),
                        &mut buffer_mut(_start, _end, 834, _buf),
                        &mut buffer_mut(_start, _end, 835, _buf),
                    )?;
                }
                if (_start >= 836 && _start < 840)
                    || (_end > 836 && _end <= 840)
                {
                    self.read_rtc0_evtenset(
                        &mut buffer_mut(_start, _end, 836, _buf),
                        &mut buffer_mut(_start, _end, 837, _buf),
                        &mut buffer_mut(_start, _end, 838, _buf),
                        &mut buffer_mut(_start, _end, 839, _buf),
                    )?;
                }
                if (_start >= 840 && _start < 844)
                    || (_end > 840 && _end <= 844)
                {
                    self.read_rtc0_evtenclr(
                        &mut buffer_mut(_start, _end, 840, _buf),
                        &mut buffer_mut(_start, _end, 841, _buf),
                        &mut buffer_mut(_start, _end, 842, _buf),
                        &mut buffer_mut(_start, _end, 843, _buf),
                    )?;
                }
            }
            (1284..=1291, 1285..=1292) => {
                if (_start >= 1284 && _start < 1288)
                    || (_end > 1284 && _end <= 1288)
                {
                    self.read_rtc0_counter(
                        &mut buffer_mut(_start, _end, 1284, _buf),
                        &mut buffer_mut(_start, _end, 1285, _buf),
                        &mut buffer_mut(_start, _end, 1286, _buf),
                        &mut buffer_mut(_start, _end, 1287, _buf),
                    )?;
                }
                if (_start >= 1288 && _start < 1292)
                    || (_end > 1288 && _end <= 1292)
                {
                    self.read_rtc0_prescaler(
                        &mut buffer_mut(_start, _end, 1288, _buf),
                        &mut buffer_mut(_start, _end, 1289, _buf),
                        &mut buffer_mut(_start, _end, 1290, _buf),
                        &mut buffer_mut(_start, _end, 1291, _buf),
                    )?;
                }
            }
            (1344..=1359, 1345..=1360) => {
                if (_start >= 1344 && _start < 1348)
                    || (_end > 1344 && _end <= 1348)
                {
                    self.read_rtc0_ccn(
                        0,
                        &mut buffer_mut(_start, _end, 1344, _buf),
                        &mut buffer_mut(_start, _end, 1345, _buf),
                        &mut buffer_mut(_start, _end, 1346, _buf),
                        &mut buffer_mut(_start, _end, 1347, _buf),
                    )?;
                }
                if (_start >= 1348 && _start < 1352)
                    || (_end > 1348 && _end <= 1352)
                {
                    self.read_rtc0_ccn(
                        1,
                        &mut buffer_mut(_start, _end, 1348, _buf),
                        &mut buffer_mut(_start, _end, 1349, _buf),
                        &mut buffer_mut(_start, _end, 1350, _buf),
                        &mut buffer_mut(_start, _end, 1351, _buf),
                    )?;
                }
                if (_start >= 1352 && _start < 1356)
                    || (_end > 1352 && _end <= 1356)
                {
                    self.read_rtc0_ccn(
                        2,
                        &mut buffer_mut(_start, _end, 1352, _buf),
                        &mut buffer_mut(_start, _end, 1353, _buf),
                        &mut buffer_mut(_start, _end, 1354, _buf),
                        &mut buffer_mut(_start, _end, 1355, _buf),
                    )?;
                }
                if (_start >= 1356 && _start < 1360)
                    || (_end > 1356 && _end <= 1360)
                {
                    self.read_rtc0_ccn(
                        3,
                        &mut buffer_mut(_start, _end, 1356, _buf),
                        &mut buffer_mut(_start, _end, 1357, _buf),
                        &mut buffer_mut(_start, _end, 1358, _buf),
                        &mut buffer_mut(_start, _end, 1359, _buf),
                    )?;
                }
            }
            (4092..=4095, 4093..=4096) => {
                if (_start >= 4092 && _start < 4096)
                    || (_end > 4092 && _end <= 4096)
                {
                    self.read_rtc0_power(
                        &mut buffer_mut(_start, _end, 4092, _buf),
                        &mut buffer_mut(_start, _end, 4093, _buf),
                        &mut buffer_mut(_start, _end, 4094, _buf),
                        &mut buffer_mut(_start, _end, 4095, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
    fn write(
        &mut self,
        _addr: u64,
        _buf: &[u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073786880;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=15, 1..=16) => {
                if (_start < 4) || (_end <= 4) {
                    self.0.lock().unwrap().write_rtc0_tasks_start(
                        buffer_const(_start, _end, 0, _buf),
                        buffer_const(_start, _end, 1, _buf),
                        buffer_const(_start, _end, 2, _buf),
                        buffer_const(_start, _end, 3, _buf),
                    )?;
                }
                if (_start >= 4 && _start < 8) || (_end > 4 && _end <= 8) {
                    self.0.lock().unwrap().write_rtc0_tasks_stop(
                        buffer_const(_start, _end, 4, _buf),
                        buffer_const(_start, _end, 5, _buf),
                        buffer_const(_start, _end, 6, _buf),
                        buffer_const(_start, _end, 7, _buf),
                    )?;
                }
                if (_start >= 8 && _start < 12) || (_end > 8 && _end <= 12) {
                    self.0.lock().unwrap().write_rtc0_tasks_clear(
                        buffer_const(_start, _end, 8, _buf),
                        buffer_const(_start, _end, 9, _buf),
                        buffer_const(_start, _end, 10, _buf),
                        buffer_const(_start, _end, 11, _buf),
                    )?;
                }
                if (_start >= 12 && _start < 16) || (_end > 12 && _end <= 16) {
                    self.0.lock().unwrap().write_rtc0_tasks_trigovrflw(
                        buffer_const(_start, _end, 12, _buf),
                        buffer_const(_start, _end, 13, _buf),
                        buffer_const(_start, _end, 14, _buf),
                        buffer_const(_start, _end, 15, _buf),
                    )?;
                }
            }
            (256..=263, 257..=264) => {
                if (_start >= 256 && _start < 260)
                    || (_end > 256 && _end <= 260)
                {
                    self.0.lock().unwrap().write_rtc0_events_tick(
                        buffer_const(_start, _end, 256, _buf),
                        buffer_const(_start, _end, 257, _buf),
                        buffer_const(_start, _end, 258, _buf),
                        buffer_const(_start, _end, 259, _buf),
                    )?;
                }
                if (_start >= 260 && _start < 264)
                    || (_end > 260 && _end <= 264)
                {
                    self.0.lock().unwrap().write_rtc0_events_ovrflw(
                        buffer_const(_start, _end, 260, _buf),
                        buffer_const(_start, _end, 261, _buf),
                        buffer_const(_start, _end, 262, _buf),
                        buffer_const(_start, _end, 263, _buf),
                    )?;
                }
            }
            (320..=335, 321..=336) => {
                if (_start >= 320 && _start < 324)
                    || (_end > 320 && _end <= 324)
                {
                    self.0.lock().unwrap().write_rtc0_events_comparen(
                        0,
                        buffer_const(_start, _end, 320, _buf),
                        buffer_const(_start, _end, 321, _buf),
                        buffer_const(_start, _end, 322, _buf),
                        buffer_const(_start, _end, 323, _buf),
                    )?;
                }
                if (_start >= 324 && _start < 328)
                    || (_end > 324 && _end <= 328)
                {
                    self.0.lock().unwrap().write_rtc0_events_comparen(
                        1,
                        buffer_const(_start, _end, 324, _buf),
                        buffer_const(_start, _end, 325, _buf),
                        buffer_const(_start, _end, 326, _buf),
                        buffer_const(_start, _end, 327, _buf),
                    )?;
                }
                if (_start >= 328 && _start < 332)
                    || (_end > 328 && _end <= 332)
                {
                    self.0.lock().unwrap().write_rtc0_events_comparen(
                        2,
                        buffer_const(_start, _end, 328, _buf),
                        buffer_const(_start, _end, 329, _buf),
                        buffer_const(_start, _end, 330, _buf),
                        buffer_const(_start, _end, 331, _buf),
                    )?;
                }
                if (_start >= 332 && _start < 336)
                    || (_end > 332 && _end <= 336)
                {
                    self.0.lock().unwrap().write_rtc0_events_comparen(
                        3,
                        buffer_const(_start, _end, 332, _buf),
                        buffer_const(_start, _end, 333, _buf),
                        buffer_const(_start, _end, 334, _buf),
                        buffer_const(_start, _end, 335, _buf),
                    )?;
                }
            }
            (772..=779, 773..=780) => {
                if (_start >= 772 && _start < 776)
                    || (_end > 772 && _end <= 776)
                {
                    self.write_rtc0_intenset(
                        buffer_const(_start, _end, 772, _buf),
                        buffer_const(_start, _end, 773, _buf),
                        buffer_const(_start, _end, 774, _buf),
                        buffer_const(_start, _end, 775, _buf),
                    )?;
                }
                if (_start >= 776 && _start < 780)
                    || (_end > 776 && _end <= 780)
                {
                    self.write_rtc0_intenclr(
                        buffer_const(_start, _end, 776, _buf),
                        buffer_const(_start, _end, 777, _buf),
                        buffer_const(_start, _end, 778, _buf),
                        buffer_const(_start, _end, 779, _buf),
                    )?;
                }
            }
            (832..=843, 833..=844) => {
                if (_start >= 832 && _start < 836)
                    || (_end > 832 && _end <= 836)
                {
                    self.write_rtc0_evten(
                        buffer_const(_start, _end, 832, _buf),
                        buffer_const(_start, _end, 833, _buf),
                        buffer_const(_start, _end, 834, _buf),
                        buffer_const(_start, _end, 835, _buf),
                    )?;
                }
                if (_start >= 836 && _start < 840)
                    || (_end > 836 && _end <= 840)
                {
                    self.write_rtc0_evtenset(
                        buffer_const(_start, _end, 836, _buf),
                        buffer_const(_start, _end, 837, _buf),
                        buffer_const(_start, _end, 838, _buf),
                        buffer_const(_start, _end, 839, _buf),
                    )?;
                }
                if (_start >= 840 && _start < 844)
                    || (_end > 840 && _end <= 844)
                {
                    self.write_rtc0_evtenclr(
                        buffer_const(_start, _end, 840, _buf),
                        buffer_const(_start, _end, 841, _buf),
                        buffer_const(_start, _end, 842, _buf),
                        buffer_const(_start, _end, 843, _buf),
                    )?;
                }
            }
            (1284..=1291, 1285..=1292) => {
                if (_start >= 1284 && _start < 1288)
                    || (_end > 1284 && _end <= 1288)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 1288 && _start < 1292)
                    || (_end > 1288 && _end <= 1292)
                {
                    self.write_rtc0_prescaler(
                        buffer_const(_start, _end, 1288, _buf),
                        buffer_const(_start, _end, 1289, _buf),
                        buffer_const(_start, _end, 1290, _buf),
                        buffer_const(_start, _end, 1291, _buf),
                    )?;
                }
            }
            (1344..=1359, 1345..=1360) => {
                if (_start >= 1344 && _start < 1348)
                    || (_end > 1344 && _end <= 1348)
                {
                    self.write_rtc0_ccn(
                        0,
                        buffer_const(_start, _end, 1344, _buf),
                        buffer_const(_start, _end, 1345, _buf),
                        buffer_const(_start, _end, 1346, _buf),
                        buffer_const(_start, _end, 1347, _buf),
                    )?;
                }
                if (_start >= 1348 && _start < 1352)
                    || (_end > 1348 && _end <= 1352)
                {
                    self.write_rtc0_ccn(
                        1,
                        buffer_const(_start, _end, 1348, _buf),
                        buffer_const(_start, _end, 1349, _buf),
                        buffer_const(_start, _end, 1350, _buf),
                        buffer_const(_start, _end, 1351, _buf),
                    )?;
                }
                if (_start >= 1352 && _start < 1356)
                    || (_end > 1352 && _end <= 1356)
                {
                    self.write_rtc0_ccn(
                        2,
                        buffer_const(_start, _end, 1352, _buf),
                        buffer_const(_start, _end, 1353, _buf),
                        buffer_const(_start, _end, 1354, _buf),
                        buffer_const(_start, _end, 1355, _buf),
                    )?;
                }
                if (_start >= 1356 && _start < 1360)
                    || (_end > 1356 && _end <= 1360)
                {
                    self.write_rtc0_ccn(
                        3,
                        buffer_const(_start, _end, 1356, _buf),
                        buffer_const(_start, _end, 1357, _buf),
                        buffer_const(_start, _end, 1358, _buf),
                        buffer_const(_start, _end, 1359, _buf),
                    )?;
                }
            }
            (4092..=4095, 4093..=4096) => {
                if (_start >= 4092 && _start < 4096)
                    || (_end > 4092 && _end <= 4096)
                {
                    self.write_rtc0_power(
                        buffer_const(_start, _end, 4092, _buf),
                        buffer_const(_start, _end, 4093, _buf),
                        buffer_const(_start, _end, 4094, _buf),
                        buffer_const(_start, _end, 4095, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
}
impl PeripheralPage0x4000B000 {
    fn read_rtc0_intenset(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_rtc0_intenset_tick()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_rtc0_intenset_ovrflw()? << 1;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_rtc0_intenset_compare0()? << 0;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_rtc0_intenset_compare1()? << 1;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_rtc0_intenset_compare2()? << 2;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_rtc0_intenset_compare3()? << 3;
        }
        Ok(())
    }
    fn write_rtc0_intenset(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_rtc0_intenset_tick((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_rtc0_intenset_ovrflw((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_rtc0_intenset_compare0((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_rtc0_intenset_compare1((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_rtc0_intenset_compare2((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_rtc0_intenset_compare3((*byte >> 3) & 1)?;
        }
        Ok(())
    }
    fn read_rtc0_intenclr(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_rtc0_intenclr_tick()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_rtc0_intenclr_ovrflw()? << 1;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_rtc0_intenclr_compare0()? << 0;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_rtc0_intenclr_compare1()? << 1;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_rtc0_intenclr_compare2()? << 2;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_rtc0_intenclr_compare3()? << 3;
        }
        Ok(())
    }
    fn write_rtc0_intenclr(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_rtc0_intenclr_tick((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_rtc0_intenclr_ovrflw((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_rtc0_intenclr_compare0((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_rtc0_intenclr_compare1((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_rtc0_intenclr_compare2((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_rtc0_intenclr_compare3((*byte >> 3) & 1)?;
        }
        Ok(())
    }
    fn read_rtc0_evten(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_rtc0_evten_tick()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_rtc0_evten_ovrflw()? << 1;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_rtc0_evten_compare0()? << 0;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_rtc0_evten_compare1()? << 1;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_rtc0_evten_compare2()? << 2;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_rtc0_evten_compare3()? << 3;
        }
        Ok(())
    }
    fn write_rtc0_evten(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_rtc0_evten_tick((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_rtc0_evten_ovrflw((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_rtc0_evten_compare0((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_rtc0_evten_compare1((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_rtc0_evten_compare2((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_rtc0_evten_compare3((*byte >> 3) & 1)?;
        }
        Ok(())
    }
    fn read_rtc0_evtenset(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_rtc0_evtenset_tick()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_rtc0_evtenset_ovrflw()? << 1;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_rtc0_evtenset_compare0()? << 0;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_rtc0_evtenset_compare1()? << 1;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_rtc0_evtenset_compare2()? << 2;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_rtc0_evtenset_compare3()? << 3;
        }
        Ok(())
    }
    fn write_rtc0_evtenset(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_rtc0_evtenset_tick((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_rtc0_evtenset_ovrflw((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_rtc0_evtenset_compare0((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_rtc0_evtenset_compare1((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_rtc0_evtenset_compare2((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_rtc0_evtenset_compare3((*byte >> 3) & 1)?;
        }
        Ok(())
    }
    fn read_rtc0_evtenclr(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_rtc0_evtenclr_tick()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_rtc0_evtenclr_ovrflw()? << 1;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_rtc0_evtenclr_compare0()? << 0;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_rtc0_evtenclr_compare1()? << 1;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_rtc0_evtenclr_compare2()? << 2;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_rtc0_evtenclr_compare3()? << 3;
        }
        Ok(())
    }
    fn write_rtc0_evtenclr(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_rtc0_evtenclr_tick((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_rtc0_evtenclr_ovrflw((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_rtc0_evtenclr_compare0((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_rtc0_evtenclr_compare1((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_rtc0_evtenclr_compare2((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_rtc0_evtenclr_compare3((*byte >> 3) & 1)?;
        }
        Ok(())
    }
    fn read_rtc0_counter(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if _byte_0.is_some() || _byte_1.is_some() || _byte_2.is_some() {
            self.0
                .lock()
                .unwrap()
                .read_rtc0_counter_counter(_byte_0, _byte_1, _byte_2)?;
        }
        Ok(())
    }
    fn read_rtc0_prescaler(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if _byte_0.is_some() || _byte_1.is_some() {
            self.0
                .lock()
                .unwrap()
                .read_rtc0_prescaler_prescaler(_byte_0, _byte_1)?;
        }
        Ok(())
    }
    fn write_rtc0_prescaler(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if _byte_0.is_some() || _byte_1.is_some() {
            self.0
                .lock()
                .unwrap()
                .write_rtc0_prescaler_prescaler(_byte_0, _byte_1)?;
        }
        Ok(())
    }
    fn read_rtc0_ccn(
        &self,
        _dim: usize,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if _byte_0.is_some() || _byte_1.is_some() || _byte_2.is_some() {
            self.0
                .lock()
                .unwrap()
                .read_rtc0_ccn_compare(_dim, _byte_0, _byte_1, _byte_2)?;
        }
        Ok(())
    }
    fn write_rtc0_ccn(
        &self,
        _dim: usize,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if _byte_0.is_some() || _byte_1.is_some() || _byte_2.is_some() {
            self.0
                .lock()
                .unwrap()
                .write_rtc0_ccn_compare(_dim, _byte_0, _byte_1, _byte_2)?;
        }
        Ok(())
    }
    fn read_rtc0_power(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_rtc0_power_power()? << 0;
        }
        Ok(())
    }
    fn write_rtc0_power(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_rtc0_power_power((*byte >> 0) & 1)?;
        }
        Ok(())
    }
}
pub struct PeripheralPage0x4000C000(
    pub std::sync::Arc<std::sync::Mutex<super::peripheral::Peripherals>>,
);
impl icicle_vm::cpu::mem::IoMemory for PeripheralPage0x4000C000 {
    fn read(
        &mut self,
        _addr: u64,
        _buf: &mut [u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073790976;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=7, 1..=8) => {
                if (_start < 4) || (_end <= 4) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 4 && _start < 8) || (_end > 4 && _end <= 8) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (256..=259, 257..=260) => {
                if (_start >= 256 && _start < 260)
                    || (_end > 256 && _end <= 260)
                {
                    self.0.lock().unwrap().read_temp_events_datardy(
                        &mut buffer_mut(_start, _end, 256, _buf),
                        &mut buffer_mut(_start, _end, 257, _buf),
                        &mut buffer_mut(_start, _end, 258, _buf),
                        &mut buffer_mut(_start, _end, 259, _buf),
                    )?;
                }
            }
            (772..=779, 773..=780) => {
                if (_start >= 772 && _start < 776)
                    || (_end > 772 && _end <= 776)
                {
                    self.read_temp_intenset(
                        &mut buffer_mut(_start, _end, 772, _buf),
                        &mut buffer_mut(_start, _end, 773, _buf),
                        &mut buffer_mut(_start, _end, 774, _buf),
                        &mut buffer_mut(_start, _end, 775, _buf),
                    )?;
                }
                if (_start >= 776 && _start < 780)
                    || (_end > 776 && _end <= 780)
                {
                    self.read_temp_intenclr(
                        &mut buffer_mut(_start, _end, 776, _buf),
                        &mut buffer_mut(_start, _end, 777, _buf),
                        &mut buffer_mut(_start, _end, 778, _buf),
                        &mut buffer_mut(_start, _end, 779, _buf),
                    )?;
                }
            }
            (1288..=1291, 1289..=1292) => {
                if (_start >= 1288 && _start < 1292)
                    || (_end > 1288 && _end <= 1292)
                {
                    self.0.lock().unwrap().read_temp_temp(
                        &mut buffer_mut(_start, _end, 1288, _buf),
                        &mut buffer_mut(_start, _end, 1289, _buf),
                        &mut buffer_mut(_start, _end, 1290, _buf),
                        &mut buffer_mut(_start, _end, 1291, _buf),
                    )?;
                }
            }
            (4092..=4095, 4093..=4096) => {
                if (_start >= 4092 && _start < 4096)
                    || (_end > 4092 && _end <= 4096)
                {
                    self.read_temp_power(
                        &mut buffer_mut(_start, _end, 4092, _buf),
                        &mut buffer_mut(_start, _end, 4093, _buf),
                        &mut buffer_mut(_start, _end, 4094, _buf),
                        &mut buffer_mut(_start, _end, 4095, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
    fn write(
        &mut self,
        _addr: u64,
        _buf: &[u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073790976;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=7, 1..=8) => {
                if (_start < 4) || (_end <= 4) {
                    self.0.lock().unwrap().write_temp_tasks_start(
                        buffer_const(_start, _end, 0, _buf),
                        buffer_const(_start, _end, 1, _buf),
                        buffer_const(_start, _end, 2, _buf),
                        buffer_const(_start, _end, 3, _buf),
                    )?;
                }
                if (_start >= 4 && _start < 8) || (_end > 4 && _end <= 8) {
                    self.0.lock().unwrap().write_temp_tasks_stop(
                        buffer_const(_start, _end, 4, _buf),
                        buffer_const(_start, _end, 5, _buf),
                        buffer_const(_start, _end, 6, _buf),
                        buffer_const(_start, _end, 7, _buf),
                    )?;
                }
            }
            (256..=259, 257..=260) => {
                if (_start >= 256 && _start < 260)
                    || (_end > 256 && _end <= 260)
                {
                    self.0.lock().unwrap().write_temp_events_datardy(
                        buffer_const(_start, _end, 256, _buf),
                        buffer_const(_start, _end, 257, _buf),
                        buffer_const(_start, _end, 258, _buf),
                        buffer_const(_start, _end, 259, _buf),
                    )?;
                }
            }
            (772..=779, 773..=780) => {
                if (_start >= 772 && _start < 776)
                    || (_end > 772 && _end <= 776)
                {
                    self.write_temp_intenset(
                        buffer_const(_start, _end, 772, _buf),
                        buffer_const(_start, _end, 773, _buf),
                        buffer_const(_start, _end, 774, _buf),
                        buffer_const(_start, _end, 775, _buf),
                    )?;
                }
                if (_start >= 776 && _start < 780)
                    || (_end > 776 && _end <= 780)
                {
                    self.write_temp_intenclr(
                        buffer_const(_start, _end, 776, _buf),
                        buffer_const(_start, _end, 777, _buf),
                        buffer_const(_start, _end, 778, _buf),
                        buffer_const(_start, _end, 779, _buf),
                    )?;
                }
            }
            (1288..=1291, 1289..=1292) => {
                if (_start >= 1288 && _start < 1292)
                    || (_end > 1288 && _end <= 1292)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (4092..=4095, 4093..=4096) => {
                if (_start >= 4092 && _start < 4096)
                    || (_end > 4092 && _end <= 4096)
                {
                    self.write_temp_power(
                        buffer_const(_start, _end, 4092, _buf),
                        buffer_const(_start, _end, 4093, _buf),
                        buffer_const(_start, _end, 4094, _buf),
                        buffer_const(_start, _end, 4095, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
}
impl PeripheralPage0x4000C000 {
    fn read_temp_intenset(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_temp_intenset_datardy()? << 0;
        }
        Ok(())
    }
    fn write_temp_intenset(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_temp_intenset_datardy((*byte >> 0) & 1)?;
        }
        Ok(())
    }
    fn read_temp_intenclr(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_temp_intenclr_datardy()? << 0;
        }
        Ok(())
    }
    fn write_temp_intenclr(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_temp_intenclr_datardy((*byte >> 0) & 1)?;
        }
        Ok(())
    }
    fn read_temp_power(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_temp_power_power()? << 0;
        }
        Ok(())
    }
    fn write_temp_power(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_temp_power_power((*byte >> 0) & 1)?;
        }
        Ok(())
    }
}
pub struct PeripheralPage0x4000D000(
    pub std::sync::Arc<std::sync::Mutex<super::peripheral::Peripherals>>,
);
impl icicle_vm::cpu::mem::IoMemory for PeripheralPage0x4000D000 {
    fn read(
        &mut self,
        _addr: u64,
        _buf: &mut [u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073795072;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=7, 1..=8) => {
                if (_start < 4) || (_end <= 4) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 4 && _start < 8) || (_end > 4 && _end <= 8) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (256..=259, 257..=260) => {
                if (_start >= 256 && _start < 260)
                    || (_end > 256 && _end <= 260)
                {
                    self.0.lock().unwrap().read_rng_events_valrdy(
                        &mut buffer_mut(_start, _end, 256, _buf),
                        &mut buffer_mut(_start, _end, 257, _buf),
                        &mut buffer_mut(_start, _end, 258, _buf),
                        &mut buffer_mut(_start, _end, 259, _buf),
                    )?;
                }
            }
            (512..=515, 513..=516) => {
                if (_start >= 512 && _start < 516)
                    || (_end > 512 && _end <= 516)
                {
                    self.read_rng_shorts(
                        &mut buffer_mut(_start, _end, 512, _buf),
                        &mut buffer_mut(_start, _end, 513, _buf),
                        &mut buffer_mut(_start, _end, 514, _buf),
                        &mut buffer_mut(_start, _end, 515, _buf),
                    )?;
                }
            }
            (772..=779, 773..=780) => {
                if (_start >= 772 && _start < 776)
                    || (_end > 772 && _end <= 776)
                {
                    self.read_rng_intenset(
                        &mut buffer_mut(_start, _end, 772, _buf),
                        &mut buffer_mut(_start, _end, 773, _buf),
                        &mut buffer_mut(_start, _end, 774, _buf),
                        &mut buffer_mut(_start, _end, 775, _buf),
                    )?;
                }
                if (_start >= 776 && _start < 780)
                    || (_end > 776 && _end <= 780)
                {
                    self.read_rng_intenclr(
                        &mut buffer_mut(_start, _end, 776, _buf),
                        &mut buffer_mut(_start, _end, 777, _buf),
                        &mut buffer_mut(_start, _end, 778, _buf),
                        &mut buffer_mut(_start, _end, 779, _buf),
                    )?;
                }
            }
            (1284..=1291, 1285..=1292) => {
                if (_start >= 1284 && _start < 1288)
                    || (_end > 1284 && _end <= 1288)
                {
                    self.read_rng_config(
                        &mut buffer_mut(_start, _end, 1284, _buf),
                        &mut buffer_mut(_start, _end, 1285, _buf),
                        &mut buffer_mut(_start, _end, 1286, _buf),
                        &mut buffer_mut(_start, _end, 1287, _buf),
                    )?;
                }
                if (_start >= 1288 && _start < 1292)
                    || (_end > 1288 && _end <= 1292)
                {
                    self.read_rng_value(
                        &mut buffer_mut(_start, _end, 1288, _buf),
                        &mut buffer_mut(_start, _end, 1289, _buf),
                        &mut buffer_mut(_start, _end, 1290, _buf),
                        &mut buffer_mut(_start, _end, 1291, _buf),
                    )?;
                }
            }
            (4092..=4095, 4093..=4096) => {
                if (_start >= 4092 && _start < 4096)
                    || (_end > 4092 && _end <= 4096)
                {
                    self.read_rng_power(
                        &mut buffer_mut(_start, _end, 4092, _buf),
                        &mut buffer_mut(_start, _end, 4093, _buf),
                        &mut buffer_mut(_start, _end, 4094, _buf),
                        &mut buffer_mut(_start, _end, 4095, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
    fn write(
        &mut self,
        _addr: u64,
        _buf: &[u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073795072;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=7, 1..=8) => {
                if (_start < 4) || (_end <= 4) {
                    self.0.lock().unwrap().write_rng_tasks_start(
                        buffer_const(_start, _end, 0, _buf),
                        buffer_const(_start, _end, 1, _buf),
                        buffer_const(_start, _end, 2, _buf),
                        buffer_const(_start, _end, 3, _buf),
                    )?;
                }
                if (_start >= 4 && _start < 8) || (_end > 4 && _end <= 8) {
                    self.0.lock().unwrap().write_rng_tasks_stop(
                        buffer_const(_start, _end, 4, _buf),
                        buffer_const(_start, _end, 5, _buf),
                        buffer_const(_start, _end, 6, _buf),
                        buffer_const(_start, _end, 7, _buf),
                    )?;
                }
            }
            (256..=259, 257..=260) => {
                if (_start >= 256 && _start < 260)
                    || (_end > 256 && _end <= 260)
                {
                    self.0.lock().unwrap().write_rng_events_valrdy(
                        buffer_const(_start, _end, 256, _buf),
                        buffer_const(_start, _end, 257, _buf),
                        buffer_const(_start, _end, 258, _buf),
                        buffer_const(_start, _end, 259, _buf),
                    )?;
                }
            }
            (512..=515, 513..=516) => {
                if (_start >= 512 && _start < 516)
                    || (_end > 512 && _end <= 516)
                {
                    self.write_rng_shorts(
                        buffer_const(_start, _end, 512, _buf),
                        buffer_const(_start, _end, 513, _buf),
                        buffer_const(_start, _end, 514, _buf),
                        buffer_const(_start, _end, 515, _buf),
                    )?;
                }
            }
            (772..=779, 773..=780) => {
                if (_start >= 772 && _start < 776)
                    || (_end > 772 && _end <= 776)
                {
                    self.write_rng_intenset(
                        buffer_const(_start, _end, 772, _buf),
                        buffer_const(_start, _end, 773, _buf),
                        buffer_const(_start, _end, 774, _buf),
                        buffer_const(_start, _end, 775, _buf),
                    )?;
                }
                if (_start >= 776 && _start < 780)
                    || (_end > 776 && _end <= 780)
                {
                    self.write_rng_intenclr(
                        buffer_const(_start, _end, 776, _buf),
                        buffer_const(_start, _end, 777, _buf),
                        buffer_const(_start, _end, 778, _buf),
                        buffer_const(_start, _end, 779, _buf),
                    )?;
                }
            }
            (1284..=1291, 1285..=1292) => {
                if (_start >= 1284 && _start < 1288)
                    || (_end > 1284 && _end <= 1288)
                {
                    self.write_rng_config(
                        buffer_const(_start, _end, 1284, _buf),
                        buffer_const(_start, _end, 1285, _buf),
                        buffer_const(_start, _end, 1286, _buf),
                        buffer_const(_start, _end, 1287, _buf),
                    )?;
                }
                if (_start >= 1288 && _start < 1292)
                    || (_end > 1288 && _end <= 1292)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (4092..=4095, 4093..=4096) => {
                if (_start >= 4092 && _start < 4096)
                    || (_end > 4092 && _end <= 4096)
                {
                    self.write_rng_power(
                        buffer_const(_start, _end, 4092, _buf),
                        buffer_const(_start, _end, 4093, _buf),
                        buffer_const(_start, _end, 4094, _buf),
                        buffer_const(_start, _end, 4095, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
}
impl PeripheralPage0x4000D000 {
    fn read_rng_shorts(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_rng_shorts_valrdy_stop()? << 0;
        }
        Ok(())
    }
    fn write_rng_shorts(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_rng_shorts_valrdy_stop((*byte >> 0) & 1)?;
        }
        Ok(())
    }
    fn read_rng_intenset(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_rng_intenset_valrdy()? << 0;
        }
        Ok(())
    }
    fn write_rng_intenset(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_rng_intenset_valrdy((*byte >> 0) & 1)?;
        }
        Ok(())
    }
    fn read_rng_intenclr(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_rng_intenclr_valrdy()? << 0;
        }
        Ok(())
    }
    fn write_rng_intenclr(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_rng_intenclr_valrdy((*byte >> 0) & 1)?;
        }
        Ok(())
    }
    fn read_rng_config(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_rng_config_dercen()? << 0;
        }
        Ok(())
    }
    fn write_rng_config(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_rng_config_dercen((*byte >> 0) & 1)?;
        }
        Ok(())
    }
    fn read_rng_value(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_rng_value_value()? << 0;
        }
        Ok(())
    }
    fn read_rng_power(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_rng_power_power()? << 0;
        }
        Ok(())
    }
    fn write_rng_power(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_rng_power_power((*byte >> 0) & 1)?;
        }
        Ok(())
    }
}
pub struct PeripheralPage0x4000E000(
    pub std::sync::Arc<std::sync::Mutex<super::peripheral::Peripherals>>,
);
impl icicle_vm::cpu::mem::IoMemory for PeripheralPage0x4000E000 {
    fn read(
        &mut self,
        _addr: u64,
        _buf: &mut [u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073799168;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=7, 1..=8) => {
                if (_start < 4) || (_end <= 4) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 4 && _start < 8) || (_end > 4 && _end <= 8) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (256..=263, 257..=264) => {
                if (_start >= 256 && _start < 260)
                    || (_end > 256 && _end <= 260)
                {
                    self.0.lock().unwrap().read_ecb_events_endecb(
                        &mut buffer_mut(_start, _end, 256, _buf),
                        &mut buffer_mut(_start, _end, 257, _buf),
                        &mut buffer_mut(_start, _end, 258, _buf),
                        &mut buffer_mut(_start, _end, 259, _buf),
                    )?;
                }
                if (_start >= 260 && _start < 264)
                    || (_end > 260 && _end <= 264)
                {
                    self.0.lock().unwrap().read_ecb_events_errorecb(
                        &mut buffer_mut(_start, _end, 260, _buf),
                        &mut buffer_mut(_start, _end, 261, _buf),
                        &mut buffer_mut(_start, _end, 262, _buf),
                        &mut buffer_mut(_start, _end, 263, _buf),
                    )?;
                }
            }
            (772..=779, 773..=780) => {
                if (_start >= 772 && _start < 776)
                    || (_end > 772 && _end <= 776)
                {
                    self.read_ecb_intenset(
                        &mut buffer_mut(_start, _end, 772, _buf),
                        &mut buffer_mut(_start, _end, 773, _buf),
                        &mut buffer_mut(_start, _end, 774, _buf),
                        &mut buffer_mut(_start, _end, 775, _buf),
                    )?;
                }
                if (_start >= 776 && _start < 780)
                    || (_end > 776 && _end <= 780)
                {
                    self.read_ecb_intenclr(
                        &mut buffer_mut(_start, _end, 776, _buf),
                        &mut buffer_mut(_start, _end, 777, _buf),
                        &mut buffer_mut(_start, _end, 778, _buf),
                        &mut buffer_mut(_start, _end, 779, _buf),
                    )?;
                }
            }
            (1284..=1287, 1285..=1288) => {
                if (_start >= 1284 && _start < 1288)
                    || (_end > 1284 && _end <= 1288)
                {
                    self.0.lock().unwrap().read_ecb_ecbdataptr(
                        &mut buffer_mut(_start, _end, 1284, _buf),
                        &mut buffer_mut(_start, _end, 1285, _buf),
                        &mut buffer_mut(_start, _end, 1286, _buf),
                        &mut buffer_mut(_start, _end, 1287, _buf),
                    )?;
                }
            }
            (4092..=4095, 4093..=4096) => {
                if (_start >= 4092 && _start < 4096)
                    || (_end > 4092 && _end <= 4096)
                {
                    self.read_ecb_power(
                        &mut buffer_mut(_start, _end, 4092, _buf),
                        &mut buffer_mut(_start, _end, 4093, _buf),
                        &mut buffer_mut(_start, _end, 4094, _buf),
                        &mut buffer_mut(_start, _end, 4095, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
    fn write(
        &mut self,
        _addr: u64,
        _buf: &[u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073799168;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=7, 1..=8) => {
                if (_start < 4) || (_end <= 4) {
                    self.0.lock().unwrap().write_ecb_tasks_startecb(
                        buffer_const(_start, _end, 0, _buf),
                        buffer_const(_start, _end, 1, _buf),
                        buffer_const(_start, _end, 2, _buf),
                        buffer_const(_start, _end, 3, _buf),
                    )?;
                }
                if (_start >= 4 && _start < 8) || (_end > 4 && _end <= 8) {
                    self.0.lock().unwrap().write_ecb_tasks_stopecb(
                        buffer_const(_start, _end, 4, _buf),
                        buffer_const(_start, _end, 5, _buf),
                        buffer_const(_start, _end, 6, _buf),
                        buffer_const(_start, _end, 7, _buf),
                    )?;
                }
            }
            (256..=263, 257..=264) => {
                if (_start >= 256 && _start < 260)
                    || (_end > 256 && _end <= 260)
                {
                    self.0.lock().unwrap().write_ecb_events_endecb(
                        buffer_const(_start, _end, 256, _buf),
                        buffer_const(_start, _end, 257, _buf),
                        buffer_const(_start, _end, 258, _buf),
                        buffer_const(_start, _end, 259, _buf),
                    )?;
                }
                if (_start >= 260 && _start < 264)
                    || (_end > 260 && _end <= 264)
                {
                    self.0.lock().unwrap().write_ecb_events_errorecb(
                        buffer_const(_start, _end, 260, _buf),
                        buffer_const(_start, _end, 261, _buf),
                        buffer_const(_start, _end, 262, _buf),
                        buffer_const(_start, _end, 263, _buf),
                    )?;
                }
            }
            (772..=779, 773..=780) => {
                if (_start >= 772 && _start < 776)
                    || (_end > 772 && _end <= 776)
                {
                    self.write_ecb_intenset(
                        buffer_const(_start, _end, 772, _buf),
                        buffer_const(_start, _end, 773, _buf),
                        buffer_const(_start, _end, 774, _buf),
                        buffer_const(_start, _end, 775, _buf),
                    )?;
                }
                if (_start >= 776 && _start < 780)
                    || (_end > 776 && _end <= 780)
                {
                    self.write_ecb_intenclr(
                        buffer_const(_start, _end, 776, _buf),
                        buffer_const(_start, _end, 777, _buf),
                        buffer_const(_start, _end, 778, _buf),
                        buffer_const(_start, _end, 779, _buf),
                    )?;
                }
            }
            (1284..=1287, 1285..=1288) => {
                if (_start >= 1284 && _start < 1288)
                    || (_end > 1284 && _end <= 1288)
                {
                    self.0.lock().unwrap().write_ecb_ecbdataptr(
                        buffer_const(_start, _end, 1284, _buf),
                        buffer_const(_start, _end, 1285, _buf),
                        buffer_const(_start, _end, 1286, _buf),
                        buffer_const(_start, _end, 1287, _buf),
                    )?;
                }
            }
            (4092..=4095, 4093..=4096) => {
                if (_start >= 4092 && _start < 4096)
                    || (_end > 4092 && _end <= 4096)
                {
                    self.write_ecb_power(
                        buffer_const(_start, _end, 4092, _buf),
                        buffer_const(_start, _end, 4093, _buf),
                        buffer_const(_start, _end, 4094, _buf),
                        buffer_const(_start, _end, 4095, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
}
impl PeripheralPage0x4000E000 {
    fn read_ecb_intenset(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ecb_intenset_endecb()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ecb_intenset_errorecb()? << 1;
        }
        Ok(())
    }
    fn write_ecb_intenset(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ecb_intenset_endecb((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ecb_intenset_errorecb((*byte >> 1) & 1)?;
        }
        Ok(())
    }
    fn read_ecb_intenclr(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ecb_intenclr_endecb()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ecb_intenclr_errorecb()? << 1;
        }
        Ok(())
    }
    fn write_ecb_intenclr(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ecb_intenclr_endecb((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ecb_intenclr_errorecb((*byte >> 1) & 1)?;
        }
        Ok(())
    }
    fn read_ecb_power(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ecb_power_power()? << 0;
        }
        Ok(())
    }
    fn write_ecb_power(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ecb_power_power((*byte >> 0) & 1)?;
        }
        Ok(())
    }
}
pub struct PeripheralPage0x4000F000(
    pub std::sync::Arc<std::sync::Mutex<super::peripheral::Peripherals>>,
);
impl icicle_vm::cpu::mem::IoMemory for PeripheralPage0x4000F000 {
    fn read(
        &mut self,
        _addr: u64,
        _buf: &mut [u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073803264;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=11, 1..=12) => {
                if (_start < 4) || (_end <= 4) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 8 && _start < 12) || (_end > 8 && _end <= 12) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 4 && _start < 8) || (_end > 4 && _end <= 8) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (256..=267, 257..=268) => {
                if (_start >= 256 && _start < 260)
                    || (_end > 256 && _end <= 260)
                {
                    self.0.lock().unwrap().read_aarccm_events_end(
                        &mut buffer_mut(_start, _end, 256, _buf),
                        &mut buffer_mut(_start, _end, 257, _buf),
                        &mut buffer_mut(_start, _end, 258, _buf),
                        &mut buffer_mut(_start, _end, 259, _buf),
                    )?;
                }
                if (_start >= 260 && _start < 264)
                    || (_end > 260 && _end <= 264)
                {
                    self.0.lock().unwrap().read_aarccm_events_resolved(
                        &mut buffer_mut(_start, _end, 260, _buf),
                        &mut buffer_mut(_start, _end, 261, _buf),
                        &mut buffer_mut(_start, _end, 262, _buf),
                        &mut buffer_mut(_start, _end, 263, _buf),
                    )?;
                }
                if (_start >= 264 && _start < 268)
                    || (_end > 264 && _end <= 268)
                {
                    self.0.lock().unwrap().read_aarccm_events_notresolved(
                        &mut buffer_mut(_start, _end, 264, _buf),
                        &mut buffer_mut(_start, _end, 265, _buf),
                        &mut buffer_mut(_start, _end, 266, _buf),
                        &mut buffer_mut(_start, _end, 267, _buf),
                    )?;
                }
            }
            (512..=515, 513..=516) => {
                if (_start >= 512 && _start < 516)
                    || (_end > 512 && _end <= 516)
                {
                    self.read_ccm_shorts(
                        &mut buffer_mut(_start, _end, 512, _buf),
                        &mut buffer_mut(_start, _end, 513, _buf),
                        &mut buffer_mut(_start, _end, 514, _buf),
                        &mut buffer_mut(_start, _end, 515, _buf),
                    )?;
                }
            }
            (772..=779, 773..=780) => {
                if (_start >= 772 && _start < 776)
                    || (_end > 772 && _end <= 776)
                {
                    self.read_aarccm_intenset(
                        &mut buffer_mut(_start, _end, 772, _buf),
                        &mut buffer_mut(_start, _end, 773, _buf),
                        &mut buffer_mut(_start, _end, 774, _buf),
                        &mut buffer_mut(_start, _end, 775, _buf),
                    )?;
                }
                if (_start >= 776 && _start < 780)
                    || (_end > 776 && _end <= 780)
                {
                    self.read_aarccm_intenclr(
                        &mut buffer_mut(_start, _end, 776, _buf),
                        &mut buffer_mut(_start, _end, 777, _buf),
                        &mut buffer_mut(_start, _end, 778, _buf),
                        &mut buffer_mut(_start, _end, 779, _buf),
                    )?;
                }
            }
            (1024..=1027, 1025..=1028) => {
                if (_start >= 1024 && _start < 1028)
                    || (_end > 1024 && _end <= 1028)
                {
                    self.read_aarccm_status(
                        &mut buffer_mut(_start, _end, 1024, _buf),
                        &mut buffer_mut(_start, _end, 1025, _buf),
                        &mut buffer_mut(_start, _end, 1026, _buf),
                        &mut buffer_mut(_start, _end, 1027, _buf),
                    )?;
                }
            }
            (1280..=1303, 1281..=1304) => {
                if (_start >= 1280 && _start < 1284)
                    || (_end > 1280 && _end <= 1284)
                {
                    self.read_aarccm_enable(
                        &mut buffer_mut(_start, _end, 1280, _buf),
                        &mut buffer_mut(_start, _end, 1281, _buf),
                        &mut buffer_mut(_start, _end, 1282, _buf),
                        &mut buffer_mut(_start, _end, 1283, _buf),
                    )?;
                }
                if (_start >= 1284 && _start < 1288)
                    || (_end > 1284 && _end <= 1288)
                {
                    self.read_aarccm_nirk(
                        &mut buffer_mut(_start, _end, 1284, _buf),
                        &mut buffer_mut(_start, _end, 1285, _buf),
                        &mut buffer_mut(_start, _end, 1286, _buf),
                        &mut buffer_mut(_start, _end, 1287, _buf),
                    )?;
                }
                if (_start >= 1288 && _start < 1292)
                    || (_end > 1288 && _end <= 1292)
                {
                    self.0.lock().unwrap().read_aarccm_irkptr(
                        &mut buffer_mut(_start, _end, 1288, _buf),
                        &mut buffer_mut(_start, _end, 1289, _buf),
                        &mut buffer_mut(_start, _end, 1290, _buf),
                        &mut buffer_mut(_start, _end, 1291, _buf),
                    )?;
                }
                if (_start >= 1296 && _start < 1300)
                    || (_end > 1296 && _end <= 1300)
                {
                    self.0.lock().unwrap().read_aarccm_addrptr(
                        &mut buffer_mut(_start, _end, 1296, _buf),
                        &mut buffer_mut(_start, _end, 1297, _buf),
                        &mut buffer_mut(_start, _end, 1298, _buf),
                        &mut buffer_mut(_start, _end, 1299, _buf),
                    )?;
                }
                if (_start >= 1300 && _start < 1304)
                    || (_end > 1300 && _end <= 1304)
                {
                    self.0.lock().unwrap().read_aarccm_scratchptr(
                        &mut buffer_mut(_start, _end, 1300, _buf),
                        &mut buffer_mut(_start, _end, 1301, _buf),
                        &mut buffer_mut(_start, _end, 1302, _buf),
                        &mut buffer_mut(_start, _end, 1303, _buf),
                    )?;
                }
                if (_start >= 1292 && _start < 1296)
                    || (_end > 1292 && _end <= 1296)
                {
                    self.0.lock().unwrap().read_ccm_inptr(
                        &mut buffer_mut(_start, _end, 1292, _buf),
                        &mut buffer_mut(_start, _end, 1293, _buf),
                        &mut buffer_mut(_start, _end, 1294, _buf),
                        &mut buffer_mut(_start, _end, 1295, _buf),
                    )?;
                }
            }
            (4092..=4095, 4093..=4096) => {
                if (_start >= 4092 && _start < 4096)
                    || (_end > 4092 && _end <= 4096)
                {
                    self.read_aarccm_power(
                        &mut buffer_mut(_start, _end, 4092, _buf),
                        &mut buffer_mut(_start, _end, 4093, _buf),
                        &mut buffer_mut(_start, _end, 4094, _buf),
                        &mut buffer_mut(_start, _end, 4095, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
    fn write(
        &mut self,
        _addr: u64,
        _buf: &[u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073803264;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=11, 1..=12) => {
                if (_start < 4) || (_end <= 4) {
                    self.0.lock().unwrap().write_aarccm_tasks_start(
                        buffer_const(_start, _end, 0, _buf),
                        buffer_const(_start, _end, 1, _buf),
                        buffer_const(_start, _end, 2, _buf),
                        buffer_const(_start, _end, 3, _buf),
                    )?;
                }
                if (_start >= 8 && _start < 12) || (_end > 8 && _end <= 12) {
                    self.0.lock().unwrap().write_aarccm_tasks_stop(
                        buffer_const(_start, _end, 8, _buf),
                        buffer_const(_start, _end, 9, _buf),
                        buffer_const(_start, _end, 10, _buf),
                        buffer_const(_start, _end, 11, _buf),
                    )?;
                }
                if (_start >= 4 && _start < 8) || (_end > 4 && _end <= 8) {
                    self.0.lock().unwrap().write_ccm_tasks_crypt(
                        buffer_const(_start, _end, 4, _buf),
                        buffer_const(_start, _end, 5, _buf),
                        buffer_const(_start, _end, 6, _buf),
                        buffer_const(_start, _end, 7, _buf),
                    )?;
                }
            }
            (256..=267, 257..=268) => {
                if (_start >= 256 && _start < 260)
                    || (_end > 256 && _end <= 260)
                {
                    self.0.lock().unwrap().write_aarccm_events_end(
                        buffer_const(_start, _end, 256, _buf),
                        buffer_const(_start, _end, 257, _buf),
                        buffer_const(_start, _end, 258, _buf),
                        buffer_const(_start, _end, 259, _buf),
                    )?;
                }
                if (_start >= 260 && _start < 264)
                    || (_end > 260 && _end <= 264)
                {
                    self.0.lock().unwrap().write_aarccm_events_resolved(
                        buffer_const(_start, _end, 260, _buf),
                        buffer_const(_start, _end, 261, _buf),
                        buffer_const(_start, _end, 262, _buf),
                        buffer_const(_start, _end, 263, _buf),
                    )?;
                }
                if (_start >= 264 && _start < 268)
                    || (_end > 264 && _end <= 268)
                {
                    self.0.lock().unwrap().write_aarccm_events_notresolved(
                        buffer_const(_start, _end, 264, _buf),
                        buffer_const(_start, _end, 265, _buf),
                        buffer_const(_start, _end, 266, _buf),
                        buffer_const(_start, _end, 267, _buf),
                    )?;
                }
            }
            (512..=515, 513..=516) => {
                if (_start >= 512 && _start < 516)
                    || (_end > 512 && _end <= 516)
                {
                    self.write_ccm_shorts(
                        buffer_const(_start, _end, 512, _buf),
                        buffer_const(_start, _end, 513, _buf),
                        buffer_const(_start, _end, 514, _buf),
                        buffer_const(_start, _end, 515, _buf),
                    )?;
                }
            }
            (772..=779, 773..=780) => {
                if (_start >= 772 && _start < 776)
                    || (_end > 772 && _end <= 776)
                {
                    self.write_aarccm_intenset(
                        buffer_const(_start, _end, 772, _buf),
                        buffer_const(_start, _end, 773, _buf),
                        buffer_const(_start, _end, 774, _buf),
                        buffer_const(_start, _end, 775, _buf),
                    )?;
                }
                if (_start >= 776 && _start < 780)
                    || (_end > 776 && _end <= 780)
                {
                    self.write_aarccm_intenclr(
                        buffer_const(_start, _end, 776, _buf),
                        buffer_const(_start, _end, 777, _buf),
                        buffer_const(_start, _end, 778, _buf),
                        buffer_const(_start, _end, 779, _buf),
                    )?;
                }
            }
            (1024..=1027, 1025..=1028) => {
                if (_start >= 1024 && _start < 1028)
                    || (_end > 1024 && _end <= 1028)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (1280..=1303, 1281..=1304) => {
                if (_start >= 1280 && _start < 1284)
                    || (_end > 1280 && _end <= 1284)
                {
                    self.write_aarccm_enable(
                        buffer_const(_start, _end, 1280, _buf),
                        buffer_const(_start, _end, 1281, _buf),
                        buffer_const(_start, _end, 1282, _buf),
                        buffer_const(_start, _end, 1283, _buf),
                    )?;
                }
                if (_start >= 1284 && _start < 1288)
                    || (_end > 1284 && _end <= 1288)
                {
                    self.write_aarccm_nirk(
                        buffer_const(_start, _end, 1284, _buf),
                        buffer_const(_start, _end, 1285, _buf),
                        buffer_const(_start, _end, 1286, _buf),
                        buffer_const(_start, _end, 1287, _buf),
                    )?;
                }
                if (_start >= 1288 && _start < 1292)
                    || (_end > 1288 && _end <= 1292)
                {
                    self.0.lock().unwrap().write_aarccm_irkptr(
                        buffer_const(_start, _end, 1288, _buf),
                        buffer_const(_start, _end, 1289, _buf),
                        buffer_const(_start, _end, 1290, _buf),
                        buffer_const(_start, _end, 1291, _buf),
                    )?;
                }
                if (_start >= 1296 && _start < 1300)
                    || (_end > 1296 && _end <= 1300)
                {
                    self.0.lock().unwrap().write_aarccm_addrptr(
                        buffer_const(_start, _end, 1296, _buf),
                        buffer_const(_start, _end, 1297, _buf),
                        buffer_const(_start, _end, 1298, _buf),
                        buffer_const(_start, _end, 1299, _buf),
                    )?;
                }
                if (_start >= 1300 && _start < 1304)
                    || (_end > 1300 && _end <= 1304)
                {
                    self.0.lock().unwrap().write_aarccm_scratchptr(
                        buffer_const(_start, _end, 1300, _buf),
                        buffer_const(_start, _end, 1301, _buf),
                        buffer_const(_start, _end, 1302, _buf),
                        buffer_const(_start, _end, 1303, _buf),
                    )?;
                }
                if (_start >= 1292 && _start < 1296)
                    || (_end > 1292 && _end <= 1296)
                {
                    self.0.lock().unwrap().write_ccm_inptr(
                        buffer_const(_start, _end, 1292, _buf),
                        buffer_const(_start, _end, 1293, _buf),
                        buffer_const(_start, _end, 1294, _buf),
                        buffer_const(_start, _end, 1295, _buf),
                    )?;
                }
            }
            (4092..=4095, 4093..=4096) => {
                if (_start >= 4092 && _start < 4096)
                    || (_end > 4092 && _end <= 4096)
                {
                    self.write_aarccm_power(
                        buffer_const(_start, _end, 4092, _buf),
                        buffer_const(_start, _end, 4093, _buf),
                        buffer_const(_start, _end, 4094, _buf),
                        buffer_const(_start, _end, 4095, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
}
impl PeripheralPage0x4000F000 {
    fn read_aarccm_intenset(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_aarccm_intenset_end()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_aarccm_intenset_resolved()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_aarccm_intenset_notresolved()? << 2;
        }
        Ok(())
    }
    fn write_aarccm_intenset(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_aarccm_intenset_end((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_aarccm_intenset_resolved((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_aarccm_intenset_notresolved((*byte >> 2) & 1)?;
        }
        Ok(())
    }
    fn read_aarccm_intenclr(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_aarccm_intenclr_end()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_aarccm_intenclr_resolved()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_aarccm_intenclr_notresolved()? << 2;
        }
        Ok(())
    }
    fn write_aarccm_intenclr(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_aarccm_intenclr_end((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_aarccm_intenclr_resolved((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_aarccm_intenclr_notresolved((*byte >> 2) & 1)?;
        }
        Ok(())
    }
    fn read_aarccm_status(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_aarccm_status_status()? << 0;
        }
        Ok(())
    }
    fn read_aarccm_enable(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_aarccm_enable_enable()? << 0;
        }
        Ok(())
    }
    fn write_aarccm_enable(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_aarccm_enable_enable((*byte >> 0) & 3)?;
        }
        Ok(())
    }
    fn read_aarccm_nirk(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_aarccm_nirk_nirk()? << 0;
        }
        Ok(())
    }
    fn write_aarccm_nirk(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_aarccm_nirk_nirk((*byte >> 0) & 31)?;
        }
        Ok(())
    }
    fn read_aarccm_power(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_aarccm_power_power()? << 0;
        }
        Ok(())
    }
    fn write_aarccm_power(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_aarccm_power_power((*byte >> 0) & 1)?;
        }
        Ok(())
    }
    fn read_ccm_shorts(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_ccm_shorts_endksgen_crypt()? << 0;
        }
        Ok(())
    }
    fn write_ccm_shorts(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ccm_shorts_endksgen_crypt((*byte >> 0) & 1)?;
        }
        Ok(())
    }
}
pub struct PeripheralPage0x40010000(
    pub std::sync::Arc<std::sync::Mutex<super::peripheral::Peripherals>>,
);
impl icicle_vm::cpu::mem::IoMemory for PeripheralPage0x40010000 {
    fn read(
        &mut self,
        _addr: u64,
        _buf: &mut [u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073807360;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=3, 1..=4) => {
                if (_start < 4) || (_end <= 4) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (256..=259, 257..=260) => {
                if (_start >= 256 && _start < 260)
                    || (_end > 256 && _end <= 260)
                {
                    self.0.lock().unwrap().read_wdt_events_timeout(
                        &mut buffer_mut(_start, _end, 256, _buf),
                        &mut buffer_mut(_start, _end, 257, _buf),
                        &mut buffer_mut(_start, _end, 258, _buf),
                        &mut buffer_mut(_start, _end, 259, _buf),
                    )?;
                }
            }
            (772..=779, 773..=780) => {
                if (_start >= 772 && _start < 776)
                    || (_end > 772 && _end <= 776)
                {
                    self.read_wdt_intenset(
                        &mut buffer_mut(_start, _end, 772, _buf),
                        &mut buffer_mut(_start, _end, 773, _buf),
                        &mut buffer_mut(_start, _end, 774, _buf),
                        &mut buffer_mut(_start, _end, 775, _buf),
                    )?;
                }
                if (_start >= 776 && _start < 780)
                    || (_end > 776 && _end <= 780)
                {
                    self.read_wdt_intenclr(
                        &mut buffer_mut(_start, _end, 776, _buf),
                        &mut buffer_mut(_start, _end, 777, _buf),
                        &mut buffer_mut(_start, _end, 778, _buf),
                        &mut buffer_mut(_start, _end, 779, _buf),
                    )?;
                }
            }
            (1024..=1031, 1025..=1032) => {
                if (_start >= 1024 && _start < 1028)
                    || (_end > 1024 && _end <= 1028)
                {
                    self.read_wdt_runstatus(
                        &mut buffer_mut(_start, _end, 1024, _buf),
                        &mut buffer_mut(_start, _end, 1025, _buf),
                        &mut buffer_mut(_start, _end, 1026, _buf),
                        &mut buffer_mut(_start, _end, 1027, _buf),
                    )?;
                }
                if (_start >= 1028 && _start < 1032)
                    || (_end > 1028 && _end <= 1032)
                {
                    self.read_wdt_reqstatus(
                        &mut buffer_mut(_start, _end, 1028, _buf),
                        &mut buffer_mut(_start, _end, 1029, _buf),
                        &mut buffer_mut(_start, _end, 1030, _buf),
                        &mut buffer_mut(_start, _end, 1031, _buf),
                    )?;
                }
            }
            (1284..=1295, 1285..=1296) => {
                if (_start >= 1284 && _start < 1288)
                    || (_end > 1284 && _end <= 1288)
                {
                    self.0.lock().unwrap().read_wdt_crv(
                        &mut buffer_mut(_start, _end, 1284, _buf),
                        &mut buffer_mut(_start, _end, 1285, _buf),
                        &mut buffer_mut(_start, _end, 1286, _buf),
                        &mut buffer_mut(_start, _end, 1287, _buf),
                    )?;
                }
                if (_start >= 1288 && _start < 1292)
                    || (_end > 1288 && _end <= 1292)
                {
                    self.read_wdt_rren(
                        &mut buffer_mut(_start, _end, 1288, _buf),
                        &mut buffer_mut(_start, _end, 1289, _buf),
                        &mut buffer_mut(_start, _end, 1290, _buf),
                        &mut buffer_mut(_start, _end, 1291, _buf),
                    )?;
                }
                if (_start >= 1292 && _start < 1296)
                    || (_end > 1292 && _end <= 1296)
                {
                    self.read_wdt_config(
                        &mut buffer_mut(_start, _end, 1292, _buf),
                        &mut buffer_mut(_start, _end, 1293, _buf),
                        &mut buffer_mut(_start, _end, 1294, _buf),
                        &mut buffer_mut(_start, _end, 1295, _buf),
                    )?;
                }
            }
            (1536..=1567, 1537..=1568) => {
                if (_start >= 1536 && _start < 1540)
                    || (_end > 1536 && _end <= 1540)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 1540 && _start < 1544)
                    || (_end > 1540 && _end <= 1544)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 1544 && _start < 1548)
                    || (_end > 1544 && _end <= 1548)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 1548 && _start < 1552)
                    || (_end > 1548 && _end <= 1552)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 1552 && _start < 1556)
                    || (_end > 1552 && _end <= 1556)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 1556 && _start < 1560)
                    || (_end > 1556 && _end <= 1560)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 1560 && _start < 1564)
                    || (_end > 1560 && _end <= 1564)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 1564 && _start < 1568)
                    || (_end > 1564 && _end <= 1568)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (4092..=4095, 4093..=4096) => {
                if (_start >= 4092 && _start < 4096)
                    || (_end > 4092 && _end <= 4096)
                {
                    self.read_wdt_power(
                        &mut buffer_mut(_start, _end, 4092, _buf),
                        &mut buffer_mut(_start, _end, 4093, _buf),
                        &mut buffer_mut(_start, _end, 4094, _buf),
                        &mut buffer_mut(_start, _end, 4095, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
    fn write(
        &mut self,
        _addr: u64,
        _buf: &[u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073807360;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=3, 1..=4) => {
                if (_start < 4) || (_end <= 4) {
                    self.0.lock().unwrap().write_wdt_tasks_start(
                        buffer_const(_start, _end, 0, _buf),
                        buffer_const(_start, _end, 1, _buf),
                        buffer_const(_start, _end, 2, _buf),
                        buffer_const(_start, _end, 3, _buf),
                    )?;
                }
            }
            (256..=259, 257..=260) => {
                if (_start >= 256 && _start < 260)
                    || (_end > 256 && _end <= 260)
                {
                    self.0.lock().unwrap().write_wdt_events_timeout(
                        buffer_const(_start, _end, 256, _buf),
                        buffer_const(_start, _end, 257, _buf),
                        buffer_const(_start, _end, 258, _buf),
                        buffer_const(_start, _end, 259, _buf),
                    )?;
                }
            }
            (772..=779, 773..=780) => {
                if (_start >= 772 && _start < 776)
                    || (_end > 772 && _end <= 776)
                {
                    self.write_wdt_intenset(
                        buffer_const(_start, _end, 772, _buf),
                        buffer_const(_start, _end, 773, _buf),
                        buffer_const(_start, _end, 774, _buf),
                        buffer_const(_start, _end, 775, _buf),
                    )?;
                }
                if (_start >= 776 && _start < 780)
                    || (_end > 776 && _end <= 780)
                {
                    self.write_wdt_intenclr(
                        buffer_const(_start, _end, 776, _buf),
                        buffer_const(_start, _end, 777, _buf),
                        buffer_const(_start, _end, 778, _buf),
                        buffer_const(_start, _end, 779, _buf),
                    )?;
                }
            }
            (1024..=1031, 1025..=1032) => {
                if (_start >= 1024 && _start < 1028)
                    || (_end > 1024 && _end <= 1028)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 1028 && _start < 1032)
                    || (_end > 1028 && _end <= 1032)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (1284..=1295, 1285..=1296) => {
                if (_start >= 1284 && _start < 1288)
                    || (_end > 1284 && _end <= 1288)
                {
                    self.0.lock().unwrap().write_wdt_crv(
                        buffer_const(_start, _end, 1284, _buf),
                        buffer_const(_start, _end, 1285, _buf),
                        buffer_const(_start, _end, 1286, _buf),
                        buffer_const(_start, _end, 1287, _buf),
                    )?;
                }
                if (_start >= 1288 && _start < 1292)
                    || (_end > 1288 && _end <= 1292)
                {
                    self.write_wdt_rren(
                        buffer_const(_start, _end, 1288, _buf),
                        buffer_const(_start, _end, 1289, _buf),
                        buffer_const(_start, _end, 1290, _buf),
                        buffer_const(_start, _end, 1291, _buf),
                    )?;
                }
                if (_start >= 1292 && _start < 1296)
                    || (_end > 1292 && _end <= 1296)
                {
                    self.write_wdt_config(
                        buffer_const(_start, _end, 1292, _buf),
                        buffer_const(_start, _end, 1293, _buf),
                        buffer_const(_start, _end, 1294, _buf),
                        buffer_const(_start, _end, 1295, _buf),
                    )?;
                }
            }
            (1536..=1567, 1537..=1568) => {
                if (_start >= 1536 && _start < 1540)
                    || (_end > 1536 && _end <= 1540)
                {
                    self.write_wdt_rrn(
                        0,
                        buffer_const(_start, _end, 1536, _buf),
                        buffer_const(_start, _end, 1537, _buf),
                        buffer_const(_start, _end, 1538, _buf),
                        buffer_const(_start, _end, 1539, _buf),
                    )?;
                }
                if (_start >= 1540 && _start < 1544)
                    || (_end > 1540 && _end <= 1544)
                {
                    self.write_wdt_rrn(
                        1,
                        buffer_const(_start, _end, 1540, _buf),
                        buffer_const(_start, _end, 1541, _buf),
                        buffer_const(_start, _end, 1542, _buf),
                        buffer_const(_start, _end, 1543, _buf),
                    )?;
                }
                if (_start >= 1544 && _start < 1548)
                    || (_end > 1544 && _end <= 1548)
                {
                    self.write_wdt_rrn(
                        2,
                        buffer_const(_start, _end, 1544, _buf),
                        buffer_const(_start, _end, 1545, _buf),
                        buffer_const(_start, _end, 1546, _buf),
                        buffer_const(_start, _end, 1547, _buf),
                    )?;
                }
                if (_start >= 1548 && _start < 1552)
                    || (_end > 1548 && _end <= 1552)
                {
                    self.write_wdt_rrn(
                        3,
                        buffer_const(_start, _end, 1548, _buf),
                        buffer_const(_start, _end, 1549, _buf),
                        buffer_const(_start, _end, 1550, _buf),
                        buffer_const(_start, _end, 1551, _buf),
                    )?;
                }
                if (_start >= 1552 && _start < 1556)
                    || (_end > 1552 && _end <= 1556)
                {
                    self.write_wdt_rrn(
                        4,
                        buffer_const(_start, _end, 1552, _buf),
                        buffer_const(_start, _end, 1553, _buf),
                        buffer_const(_start, _end, 1554, _buf),
                        buffer_const(_start, _end, 1555, _buf),
                    )?;
                }
                if (_start >= 1556 && _start < 1560)
                    || (_end > 1556 && _end <= 1560)
                {
                    self.write_wdt_rrn(
                        5,
                        buffer_const(_start, _end, 1556, _buf),
                        buffer_const(_start, _end, 1557, _buf),
                        buffer_const(_start, _end, 1558, _buf),
                        buffer_const(_start, _end, 1559, _buf),
                    )?;
                }
                if (_start >= 1560 && _start < 1564)
                    || (_end > 1560 && _end <= 1564)
                {
                    self.write_wdt_rrn(
                        6,
                        buffer_const(_start, _end, 1560, _buf),
                        buffer_const(_start, _end, 1561, _buf),
                        buffer_const(_start, _end, 1562, _buf),
                        buffer_const(_start, _end, 1563, _buf),
                    )?;
                }
                if (_start >= 1564 && _start < 1568)
                    || (_end > 1564 && _end <= 1568)
                {
                    self.write_wdt_rrn(
                        7,
                        buffer_const(_start, _end, 1564, _buf),
                        buffer_const(_start, _end, 1565, _buf),
                        buffer_const(_start, _end, 1566, _buf),
                        buffer_const(_start, _end, 1567, _buf),
                    )?;
                }
            }
            (4092..=4095, 4093..=4096) => {
                if (_start >= 4092 && _start < 4096)
                    || (_end > 4092 && _end <= 4096)
                {
                    self.write_wdt_power(
                        buffer_const(_start, _end, 4092, _buf),
                        buffer_const(_start, _end, 4093, _buf),
                        buffer_const(_start, _end, 4094, _buf),
                        buffer_const(_start, _end, 4095, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
}
impl PeripheralPage0x40010000 {
    fn read_wdt_intenset(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_wdt_intenset_timeout()? << 0;
        }
        Ok(())
    }
    fn write_wdt_intenset(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_wdt_intenset_timeout((*byte >> 0) & 1)?;
        }
        Ok(())
    }
    fn read_wdt_intenclr(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_wdt_intenclr_timeout()? << 0;
        }
        Ok(())
    }
    fn write_wdt_intenclr(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_wdt_intenclr_timeout((*byte >> 0) & 1)?;
        }
        Ok(())
    }
    fn read_wdt_runstatus(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_wdt_runstatus_runstatus()? << 0;
        }
        Ok(())
    }
    fn read_wdt_reqstatus(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_wdt_reqstatus_rr0()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_wdt_reqstatus_rr1()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_wdt_reqstatus_rr2()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_wdt_reqstatus_rr3()? << 3;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_wdt_reqstatus_rr4()? << 4;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_wdt_reqstatus_rr5()? << 5;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_wdt_reqstatus_rr6()? << 6;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_wdt_reqstatus_rr7()? << 7;
        }
        Ok(())
    }
    fn read_wdt_rren(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_wdt_rren_rr0()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_wdt_rren_rr1()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_wdt_rren_rr2()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_wdt_rren_rr3()? << 3;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_wdt_rren_rr4()? << 4;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_wdt_rren_rr5()? << 5;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_wdt_rren_rr6()? << 6;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_wdt_rren_rr7()? << 7;
        }
        Ok(())
    }
    fn write_wdt_rren(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_wdt_rren_rr0((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_wdt_rren_rr1((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_wdt_rren_rr2((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_wdt_rren_rr3((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_wdt_rren_rr4((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_wdt_rren_rr5((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_wdt_rren_rr6((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_wdt_rren_rr7((*byte >> 7) & 1)?;
        }
        Ok(())
    }
    fn read_wdt_config(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_wdt_config_sleep()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_wdt_config_halt()? << 3;
        }
        Ok(())
    }
    fn write_wdt_config(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_wdt_config_sleep((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_wdt_config_halt((*byte >> 3) & 1)?;
        }
        Ok(())
    }
    fn write_wdt_rrn(
        &self,
        _dim: usize,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if _byte_0.is_some()
            || _byte_1.is_some()
            || _byte_2.is_some()
            || _byte_3.is_some()
        {
            self.0
                .lock()
                .unwrap()
                .write_wdt_rrn_rr(_dim, _byte_0, _byte_1, _byte_2, _byte_3)?;
        }
        Ok(())
    }
    fn read_wdt_power(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_wdt_power_power()? << 0;
        }
        Ok(())
    }
    fn write_wdt_power(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_wdt_power_power((*byte >> 0) & 1)?;
        }
        Ok(())
    }
}
pub struct PeripheralPage0x40012000(
    pub std::sync::Arc<std::sync::Mutex<super::peripheral::Peripherals>>,
);
impl icicle_vm::cpu::mem::IoMemory for PeripheralPage0x40012000 {
    fn read(
        &mut self,
        _addr: u64,
        _buf: &mut [u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073815552;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=11, 1..=12) => {
                if (_start < 4) || (_end <= 4) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 4 && _start < 8) || (_end > 4 && _end <= 8) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 8 && _start < 12) || (_end > 8 && _end <= 12) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (256..=267, 257..=268) => {
                if (_start >= 256 && _start < 260)
                    || (_end > 256 && _end <= 260)
                {
                    self.0.lock().unwrap().read_qdec_events_samplerdy(
                        &mut buffer_mut(_start, _end, 256, _buf),
                        &mut buffer_mut(_start, _end, 257, _buf),
                        &mut buffer_mut(_start, _end, 258, _buf),
                        &mut buffer_mut(_start, _end, 259, _buf),
                    )?;
                }
                if (_start >= 260 && _start < 264)
                    || (_end > 260 && _end <= 264)
                {
                    self.0.lock().unwrap().read_qdec_events_reportrdy(
                        &mut buffer_mut(_start, _end, 260, _buf),
                        &mut buffer_mut(_start, _end, 261, _buf),
                        &mut buffer_mut(_start, _end, 262, _buf),
                        &mut buffer_mut(_start, _end, 263, _buf),
                    )?;
                }
                if (_start >= 264 && _start < 268)
                    || (_end > 264 && _end <= 268)
                {
                    self.0.lock().unwrap().read_qdec_events_accof(
                        &mut buffer_mut(_start, _end, 264, _buf),
                        &mut buffer_mut(_start, _end, 265, _buf),
                        &mut buffer_mut(_start, _end, 266, _buf),
                        &mut buffer_mut(_start, _end, 267, _buf),
                    )?;
                }
            }
            (512..=515, 513..=516) => {
                if (_start >= 512 && _start < 516)
                    || (_end > 512 && _end <= 516)
                {
                    self.read_qdec_shorts(
                        &mut buffer_mut(_start, _end, 512, _buf),
                        &mut buffer_mut(_start, _end, 513, _buf),
                        &mut buffer_mut(_start, _end, 514, _buf),
                        &mut buffer_mut(_start, _end, 515, _buf),
                    )?;
                }
            }
            (772..=779, 773..=780) => {
                if (_start >= 772 && _start < 776)
                    || (_end > 772 && _end <= 776)
                {
                    self.read_qdec_intenset(
                        &mut buffer_mut(_start, _end, 772, _buf),
                        &mut buffer_mut(_start, _end, 773, _buf),
                        &mut buffer_mut(_start, _end, 774, _buf),
                        &mut buffer_mut(_start, _end, 775, _buf),
                    )?;
                }
                if (_start >= 776 && _start < 780)
                    || (_end > 776 && _end <= 780)
                {
                    self.read_qdec_intenclr(
                        &mut buffer_mut(_start, _end, 776, _buf),
                        &mut buffer_mut(_start, _end, 777, _buf),
                        &mut buffer_mut(_start, _end, 778, _buf),
                        &mut buffer_mut(_start, _end, 779, _buf),
                    )?;
                }
            }
            (1280..=1323, 1281..=1324) => {
                if (_start >= 1280 && _start < 1284)
                    || (_end > 1280 && _end <= 1284)
                {
                    self.read_qdec_enable(
                        &mut buffer_mut(_start, _end, 1280, _buf),
                        &mut buffer_mut(_start, _end, 1281, _buf),
                        &mut buffer_mut(_start, _end, 1282, _buf),
                        &mut buffer_mut(_start, _end, 1283, _buf),
                    )?;
                }
                if (_start >= 1284 && _start < 1288)
                    || (_end > 1284 && _end <= 1288)
                {
                    self.read_qdec_ledpol(
                        &mut buffer_mut(_start, _end, 1284, _buf),
                        &mut buffer_mut(_start, _end, 1285, _buf),
                        &mut buffer_mut(_start, _end, 1286, _buf),
                        &mut buffer_mut(_start, _end, 1287, _buf),
                    )?;
                }
                if (_start >= 1288 && _start < 1292)
                    || (_end > 1288 && _end <= 1292)
                {
                    self.read_qdec_sampleper(
                        &mut buffer_mut(_start, _end, 1288, _buf),
                        &mut buffer_mut(_start, _end, 1289, _buf),
                        &mut buffer_mut(_start, _end, 1290, _buf),
                        &mut buffer_mut(_start, _end, 1291, _buf),
                    )?;
                }
                if (_start >= 1292 && _start < 1296)
                    || (_end > 1292 && _end <= 1296)
                {
                    self.read_qdec_sample(
                        &mut buffer_mut(_start, _end, 1292, _buf),
                        &mut buffer_mut(_start, _end, 1293, _buf),
                        &mut buffer_mut(_start, _end, 1294, _buf),
                        &mut buffer_mut(_start, _end, 1295, _buf),
                    )?;
                }
                if (_start >= 1296 && _start < 1300)
                    || (_end > 1296 && _end <= 1300)
                {
                    self.read_qdec_reportper(
                        &mut buffer_mut(_start, _end, 1296, _buf),
                        &mut buffer_mut(_start, _end, 1297, _buf),
                        &mut buffer_mut(_start, _end, 1298, _buf),
                        &mut buffer_mut(_start, _end, 1299, _buf),
                    )?;
                }
                if (_start >= 1300 && _start < 1304)
                    || (_end > 1300 && _end <= 1304)
                {
                    self.0.lock().unwrap().read_qdec_acc(
                        &mut buffer_mut(_start, _end, 1300, _buf),
                        &mut buffer_mut(_start, _end, 1301, _buf),
                        &mut buffer_mut(_start, _end, 1302, _buf),
                        &mut buffer_mut(_start, _end, 1303, _buf),
                    )?;
                }
                if (_start >= 1304 && _start < 1308)
                    || (_end > 1304 && _end <= 1308)
                {
                    self.0.lock().unwrap().read_qdec_accread(
                        &mut buffer_mut(_start, _end, 1304, _buf),
                        &mut buffer_mut(_start, _end, 1305, _buf),
                        &mut buffer_mut(_start, _end, 1306, _buf),
                        &mut buffer_mut(_start, _end, 1307, _buf),
                    )?;
                }
                if (_start >= 1308 && _start < 1312)
                    || (_end > 1308 && _end <= 1312)
                {
                    self.0.lock().unwrap().read_qdec_pselled(
                        &mut buffer_mut(_start, _end, 1308, _buf),
                        &mut buffer_mut(_start, _end, 1309, _buf),
                        &mut buffer_mut(_start, _end, 1310, _buf),
                        &mut buffer_mut(_start, _end, 1311, _buf),
                    )?;
                }
                if (_start >= 1312 && _start < 1316)
                    || (_end > 1312 && _end <= 1316)
                {
                    self.0.lock().unwrap().read_qdec_psela(
                        &mut buffer_mut(_start, _end, 1312, _buf),
                        &mut buffer_mut(_start, _end, 1313, _buf),
                        &mut buffer_mut(_start, _end, 1314, _buf),
                        &mut buffer_mut(_start, _end, 1315, _buf),
                    )?;
                }
                if (_start >= 1316 && _start < 1320)
                    || (_end > 1316 && _end <= 1320)
                {
                    self.0.lock().unwrap().read_qdec_pselb(
                        &mut buffer_mut(_start, _end, 1316, _buf),
                        &mut buffer_mut(_start, _end, 1317, _buf),
                        &mut buffer_mut(_start, _end, 1318, _buf),
                        &mut buffer_mut(_start, _end, 1319, _buf),
                    )?;
                }
                if (_start >= 1320 && _start < 1324)
                    || (_end > 1320 && _end <= 1324)
                {
                    self.read_qdec_dbfen(
                        &mut buffer_mut(_start, _end, 1320, _buf),
                        &mut buffer_mut(_start, _end, 1321, _buf),
                        &mut buffer_mut(_start, _end, 1322, _buf),
                        &mut buffer_mut(_start, _end, 1323, _buf),
                    )?;
                }
            }
            (1344..=1355, 1345..=1356) => {
                if (_start >= 1344 && _start < 1348)
                    || (_end > 1344 && _end <= 1348)
                {
                    self.read_qdec_ledpre(
                        &mut buffer_mut(_start, _end, 1344, _buf),
                        &mut buffer_mut(_start, _end, 1345, _buf),
                        &mut buffer_mut(_start, _end, 1346, _buf),
                        &mut buffer_mut(_start, _end, 1347, _buf),
                    )?;
                }
                if (_start >= 1348 && _start < 1352)
                    || (_end > 1348 && _end <= 1352)
                {
                    self.read_qdec_accdbl(
                        &mut buffer_mut(_start, _end, 1348, _buf),
                        &mut buffer_mut(_start, _end, 1349, _buf),
                        &mut buffer_mut(_start, _end, 1350, _buf),
                        &mut buffer_mut(_start, _end, 1351, _buf),
                    )?;
                }
                if (_start >= 1352 && _start < 1356)
                    || (_end > 1352 && _end <= 1356)
                {
                    self.read_qdec_accdblread(
                        &mut buffer_mut(_start, _end, 1352, _buf),
                        &mut buffer_mut(_start, _end, 1353, _buf),
                        &mut buffer_mut(_start, _end, 1354, _buf),
                        &mut buffer_mut(_start, _end, 1355, _buf),
                    )?;
                }
            }
            (4092..=4095, 4093..=4096) => {
                if (_start >= 4092 && _start < 4096)
                    || (_end > 4092 && _end <= 4096)
                {
                    self.read_qdec_power(
                        &mut buffer_mut(_start, _end, 4092, _buf),
                        &mut buffer_mut(_start, _end, 4093, _buf),
                        &mut buffer_mut(_start, _end, 4094, _buf),
                        &mut buffer_mut(_start, _end, 4095, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
    fn write(
        &mut self,
        _addr: u64,
        _buf: &[u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073815552;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=11, 1..=12) => {
                if (_start < 4) || (_end <= 4) {
                    self.0.lock().unwrap().write_qdec_tasks_start(
                        buffer_const(_start, _end, 0, _buf),
                        buffer_const(_start, _end, 1, _buf),
                        buffer_const(_start, _end, 2, _buf),
                        buffer_const(_start, _end, 3, _buf),
                    )?;
                }
                if (_start >= 4 && _start < 8) || (_end > 4 && _end <= 8) {
                    self.0.lock().unwrap().write_qdec_tasks_stop(
                        buffer_const(_start, _end, 4, _buf),
                        buffer_const(_start, _end, 5, _buf),
                        buffer_const(_start, _end, 6, _buf),
                        buffer_const(_start, _end, 7, _buf),
                    )?;
                }
                if (_start >= 8 && _start < 12) || (_end > 8 && _end <= 12) {
                    self.0.lock().unwrap().write_qdec_tasks_readclracc(
                        buffer_const(_start, _end, 8, _buf),
                        buffer_const(_start, _end, 9, _buf),
                        buffer_const(_start, _end, 10, _buf),
                        buffer_const(_start, _end, 11, _buf),
                    )?;
                }
            }
            (256..=267, 257..=268) => {
                if (_start >= 256 && _start < 260)
                    || (_end > 256 && _end <= 260)
                {
                    self.0.lock().unwrap().write_qdec_events_samplerdy(
                        buffer_const(_start, _end, 256, _buf),
                        buffer_const(_start, _end, 257, _buf),
                        buffer_const(_start, _end, 258, _buf),
                        buffer_const(_start, _end, 259, _buf),
                    )?;
                }
                if (_start >= 260 && _start < 264)
                    || (_end > 260 && _end <= 264)
                {
                    self.0.lock().unwrap().write_qdec_events_reportrdy(
                        buffer_const(_start, _end, 260, _buf),
                        buffer_const(_start, _end, 261, _buf),
                        buffer_const(_start, _end, 262, _buf),
                        buffer_const(_start, _end, 263, _buf),
                    )?;
                }
                if (_start >= 264 && _start < 268)
                    || (_end > 264 && _end <= 268)
                {
                    self.0.lock().unwrap().write_qdec_events_accof(
                        buffer_const(_start, _end, 264, _buf),
                        buffer_const(_start, _end, 265, _buf),
                        buffer_const(_start, _end, 266, _buf),
                        buffer_const(_start, _end, 267, _buf),
                    )?;
                }
            }
            (512..=515, 513..=516) => {
                if (_start >= 512 && _start < 516)
                    || (_end > 512 && _end <= 516)
                {
                    self.write_qdec_shorts(
                        buffer_const(_start, _end, 512, _buf),
                        buffer_const(_start, _end, 513, _buf),
                        buffer_const(_start, _end, 514, _buf),
                        buffer_const(_start, _end, 515, _buf),
                    )?;
                }
            }
            (772..=779, 773..=780) => {
                if (_start >= 772 && _start < 776)
                    || (_end > 772 && _end <= 776)
                {
                    self.write_qdec_intenset(
                        buffer_const(_start, _end, 772, _buf),
                        buffer_const(_start, _end, 773, _buf),
                        buffer_const(_start, _end, 774, _buf),
                        buffer_const(_start, _end, 775, _buf),
                    )?;
                }
                if (_start >= 776 && _start < 780)
                    || (_end > 776 && _end <= 780)
                {
                    self.write_qdec_intenclr(
                        buffer_const(_start, _end, 776, _buf),
                        buffer_const(_start, _end, 777, _buf),
                        buffer_const(_start, _end, 778, _buf),
                        buffer_const(_start, _end, 779, _buf),
                    )?;
                }
            }
            (1280..=1323, 1281..=1324) => {
                if (_start >= 1280 && _start < 1284)
                    || (_end > 1280 && _end <= 1284)
                {
                    self.write_qdec_enable(
                        buffer_const(_start, _end, 1280, _buf),
                        buffer_const(_start, _end, 1281, _buf),
                        buffer_const(_start, _end, 1282, _buf),
                        buffer_const(_start, _end, 1283, _buf),
                    )?;
                }
                if (_start >= 1284 && _start < 1288)
                    || (_end > 1284 && _end <= 1288)
                {
                    self.write_qdec_ledpol(
                        buffer_const(_start, _end, 1284, _buf),
                        buffer_const(_start, _end, 1285, _buf),
                        buffer_const(_start, _end, 1286, _buf),
                        buffer_const(_start, _end, 1287, _buf),
                    )?;
                }
                if (_start >= 1288 && _start < 1292)
                    || (_end > 1288 && _end <= 1292)
                {
                    self.write_qdec_sampleper(
                        buffer_const(_start, _end, 1288, _buf),
                        buffer_const(_start, _end, 1289, _buf),
                        buffer_const(_start, _end, 1290, _buf),
                        buffer_const(_start, _end, 1291, _buf),
                    )?;
                }
                if (_start >= 1292 && _start < 1296)
                    || (_end > 1292 && _end <= 1296)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 1296 && _start < 1300)
                    || (_end > 1296 && _end <= 1300)
                {
                    self.write_qdec_reportper(
                        buffer_const(_start, _end, 1296, _buf),
                        buffer_const(_start, _end, 1297, _buf),
                        buffer_const(_start, _end, 1298, _buf),
                        buffer_const(_start, _end, 1299, _buf),
                    )?;
                }
                if (_start >= 1300 && _start < 1304)
                    || (_end > 1300 && _end <= 1304)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 1304 && _start < 1308)
                    || (_end > 1304 && _end <= 1308)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 1308 && _start < 1312)
                    || (_end > 1308 && _end <= 1312)
                {
                    self.0.lock().unwrap().write_qdec_pselled(
                        buffer_const(_start, _end, 1308, _buf),
                        buffer_const(_start, _end, 1309, _buf),
                        buffer_const(_start, _end, 1310, _buf),
                        buffer_const(_start, _end, 1311, _buf),
                    )?;
                }
                if (_start >= 1312 && _start < 1316)
                    || (_end > 1312 && _end <= 1316)
                {
                    self.0.lock().unwrap().write_qdec_psela(
                        buffer_const(_start, _end, 1312, _buf),
                        buffer_const(_start, _end, 1313, _buf),
                        buffer_const(_start, _end, 1314, _buf),
                        buffer_const(_start, _end, 1315, _buf),
                    )?;
                }
                if (_start >= 1316 && _start < 1320)
                    || (_end > 1316 && _end <= 1320)
                {
                    self.0.lock().unwrap().write_qdec_pselb(
                        buffer_const(_start, _end, 1316, _buf),
                        buffer_const(_start, _end, 1317, _buf),
                        buffer_const(_start, _end, 1318, _buf),
                        buffer_const(_start, _end, 1319, _buf),
                    )?;
                }
                if (_start >= 1320 && _start < 1324)
                    || (_end > 1320 && _end <= 1324)
                {
                    self.write_qdec_dbfen(
                        buffer_const(_start, _end, 1320, _buf),
                        buffer_const(_start, _end, 1321, _buf),
                        buffer_const(_start, _end, 1322, _buf),
                        buffer_const(_start, _end, 1323, _buf),
                    )?;
                }
            }
            (1344..=1355, 1345..=1356) => {
                if (_start >= 1344 && _start < 1348)
                    || (_end > 1344 && _end <= 1348)
                {
                    self.write_qdec_ledpre(
                        buffer_const(_start, _end, 1344, _buf),
                        buffer_const(_start, _end, 1345, _buf),
                        buffer_const(_start, _end, 1346, _buf),
                        buffer_const(_start, _end, 1347, _buf),
                    )?;
                }
                if (_start >= 1348 && _start < 1352)
                    || (_end > 1348 && _end <= 1352)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 1352 && _start < 1356)
                    || (_end > 1352 && _end <= 1356)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (4092..=4095, 4093..=4096) => {
                if (_start >= 4092 && _start < 4096)
                    || (_end > 4092 && _end <= 4096)
                {
                    self.write_qdec_power(
                        buffer_const(_start, _end, 4092, _buf),
                        buffer_const(_start, _end, 4093, _buf),
                        buffer_const(_start, _end, 4094, _buf),
                        buffer_const(_start, _end, 4095, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
}
impl PeripheralPage0x40012000 {
    fn read_qdec_shorts(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self
                .0
                .lock()
                .unwrap()
                .read_qdec_shorts_reportrdy_readclracc()?
                << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_qdec_shorts_samplerdy_stop()? << 1;
        }
        Ok(())
    }
    fn write_qdec_shorts(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_qdec_shorts_reportrdy_readclracc((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_qdec_shorts_samplerdy_stop((*byte >> 1) & 1)?;
        }
        Ok(())
    }
    fn read_qdec_intenset(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_qdec_intenset_samplerdy()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_qdec_intenset_reportrdy()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_qdec_intenset_accof()? << 2;
        }
        Ok(())
    }
    fn write_qdec_intenset(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_qdec_intenset_samplerdy((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_qdec_intenset_reportrdy((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_qdec_intenset_accof((*byte >> 2) & 1)?;
        }
        Ok(())
    }
    fn read_qdec_intenclr(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_qdec_intenclr_samplerdy()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_qdec_intenclr_reportrdy()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_qdec_intenclr_accof()? << 2;
        }
        Ok(())
    }
    fn write_qdec_intenclr(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_qdec_intenclr_samplerdy((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_qdec_intenclr_reportrdy((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_qdec_intenclr_accof((*byte >> 2) & 1)?;
        }
        Ok(())
    }
    fn read_qdec_enable(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_qdec_enable_enable()? << 0;
        }
        Ok(())
    }
    fn write_qdec_enable(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_qdec_enable_enable((*byte >> 0) & 1)?;
        }
        Ok(())
    }
    fn read_qdec_ledpol(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_qdec_ledpol_ledpol()? << 0;
        }
        Ok(())
    }
    fn write_qdec_ledpol(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_qdec_ledpol_ledpol((*byte >> 0) & 1)?;
        }
        Ok(())
    }
    fn read_qdec_sampleper(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_qdec_sampleper_sampleper()? << 0;
        }
        Ok(())
    }
    fn write_qdec_sampleper(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_qdec_sampleper_sampleper((*byte >> 0) & 7)?;
        }
        Ok(())
    }
    fn read_qdec_sample(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if _byte_0.is_some()
            || _byte_1.is_some()
            || _byte_2.is_some()
            || _byte_3.is_some()
        {
            self.0
                .lock()
                .unwrap()
                .read_qdec_sample_sample(_byte_0, _byte_1, _byte_2, _byte_3)?;
        }
        Ok(())
    }
    fn read_qdec_reportper(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_qdec_reportper_reportper()? << 0;
        }
        Ok(())
    }
    fn write_qdec_reportper(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_qdec_reportper_reportper((*byte >> 0) & 7)?;
        }
        Ok(())
    }
    fn read_qdec_dbfen(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_qdec_dbfen_dbfen()? << 0;
        }
        Ok(())
    }
    fn write_qdec_dbfen(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_qdec_dbfen_dbfen((*byte >> 0) & 1)?;
        }
        Ok(())
    }
    fn read_qdec_ledpre(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if _byte_0.is_some() || _byte_1.is_some() {
            self.0
                .lock()
                .unwrap()
                .read_qdec_ledpre_ledpre(_byte_0, _byte_1)?;
        }
        Ok(())
    }
    fn write_qdec_ledpre(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if _byte_0.is_some() || _byte_1.is_some() {
            self.0
                .lock()
                .unwrap()
                .write_qdec_ledpre_ledpre(_byte_0, _byte_1)?;
        }
        Ok(())
    }
    fn read_qdec_accdbl(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_qdec_accdbl_accdbl()? << 0;
        }
        Ok(())
    }
    fn read_qdec_accdblread(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_qdec_accdblread_accdblread()? << 0;
        }
        Ok(())
    }
    fn read_qdec_power(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_qdec_power_power()? << 0;
        }
        Ok(())
    }
    fn write_qdec_power(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_qdec_power_power((*byte >> 0) & 1)?;
        }
        Ok(())
    }
}
pub struct PeripheralPage0x40013000(
    pub std::sync::Arc<std::sync::Mutex<super::peripheral::Peripherals>>,
);
impl icicle_vm::cpu::mem::IoMemory for PeripheralPage0x40013000 {
    fn read(
        &mut self,
        _addr: u64,
        _buf: &mut [u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073819648;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=11, 1..=12) => {
                if (_start < 4) || (_end <= 4) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 4 && _start < 8) || (_end > 4 && _end <= 8) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 8 && _start < 12) || (_end > 8 && _end <= 12) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (256..=271, 257..=272) => {
                if (_start >= 256 && _start < 260)
                    || (_end > 256 && _end <= 260)
                {
                    self.0.lock().unwrap().read_lpcomp_events_ready(
                        &mut buffer_mut(_start, _end, 256, _buf),
                        &mut buffer_mut(_start, _end, 257, _buf),
                        &mut buffer_mut(_start, _end, 258, _buf),
                        &mut buffer_mut(_start, _end, 259, _buf),
                    )?;
                }
                if (_start >= 260 && _start < 264)
                    || (_end > 260 && _end <= 264)
                {
                    self.0.lock().unwrap().read_lpcomp_events_down(
                        &mut buffer_mut(_start, _end, 260, _buf),
                        &mut buffer_mut(_start, _end, 261, _buf),
                        &mut buffer_mut(_start, _end, 262, _buf),
                        &mut buffer_mut(_start, _end, 263, _buf),
                    )?;
                }
                if (_start >= 264 && _start < 268)
                    || (_end > 264 && _end <= 268)
                {
                    self.0.lock().unwrap().read_lpcomp_events_up(
                        &mut buffer_mut(_start, _end, 264, _buf),
                        &mut buffer_mut(_start, _end, 265, _buf),
                        &mut buffer_mut(_start, _end, 266, _buf),
                        &mut buffer_mut(_start, _end, 267, _buf),
                    )?;
                }
                if (_start >= 268 && _start < 272)
                    || (_end > 268 && _end <= 272)
                {
                    self.0.lock().unwrap().read_lpcomp_events_cross(
                        &mut buffer_mut(_start, _end, 268, _buf),
                        &mut buffer_mut(_start, _end, 269, _buf),
                        &mut buffer_mut(_start, _end, 270, _buf),
                        &mut buffer_mut(_start, _end, 271, _buf),
                    )?;
                }
            }
            (512..=515, 513..=516) => {
                if (_start >= 512 && _start < 516)
                    || (_end > 512 && _end <= 516)
                {
                    self.read_lpcomp_shorts(
                        &mut buffer_mut(_start, _end, 512, _buf),
                        &mut buffer_mut(_start, _end, 513, _buf),
                        &mut buffer_mut(_start, _end, 514, _buf),
                        &mut buffer_mut(_start, _end, 515, _buf),
                    )?;
                }
            }
            (772..=779, 773..=780) => {
                if (_start >= 772 && _start < 776)
                    || (_end > 772 && _end <= 776)
                {
                    self.read_lpcomp_intenset(
                        &mut buffer_mut(_start, _end, 772, _buf),
                        &mut buffer_mut(_start, _end, 773, _buf),
                        &mut buffer_mut(_start, _end, 774, _buf),
                        &mut buffer_mut(_start, _end, 775, _buf),
                    )?;
                }
                if (_start >= 776 && _start < 780)
                    || (_end > 776 && _end <= 780)
                {
                    self.read_lpcomp_intenclr(
                        &mut buffer_mut(_start, _end, 776, _buf),
                        &mut buffer_mut(_start, _end, 777, _buf),
                        &mut buffer_mut(_start, _end, 778, _buf),
                        &mut buffer_mut(_start, _end, 779, _buf),
                    )?;
                }
            }
            (1024..=1027, 1025..=1028) => {
                if (_start >= 1024 && _start < 1028)
                    || (_end > 1024 && _end <= 1028)
                {
                    self.read_lpcomp_result(
                        &mut buffer_mut(_start, _end, 1024, _buf),
                        &mut buffer_mut(_start, _end, 1025, _buf),
                        &mut buffer_mut(_start, _end, 1026, _buf),
                        &mut buffer_mut(_start, _end, 1027, _buf),
                    )?;
                }
            }
            (1280..=1295, 1281..=1296) => {
                if (_start >= 1280 && _start < 1284)
                    || (_end > 1280 && _end <= 1284)
                {
                    self.read_lpcomp_enable(
                        &mut buffer_mut(_start, _end, 1280, _buf),
                        &mut buffer_mut(_start, _end, 1281, _buf),
                        &mut buffer_mut(_start, _end, 1282, _buf),
                        &mut buffer_mut(_start, _end, 1283, _buf),
                    )?;
                }
                if (_start >= 1284 && _start < 1288)
                    || (_end > 1284 && _end <= 1288)
                {
                    self.read_lpcomp_psel(
                        &mut buffer_mut(_start, _end, 1284, _buf),
                        &mut buffer_mut(_start, _end, 1285, _buf),
                        &mut buffer_mut(_start, _end, 1286, _buf),
                        &mut buffer_mut(_start, _end, 1287, _buf),
                    )?;
                }
                if (_start >= 1288 && _start < 1292)
                    || (_end > 1288 && _end <= 1292)
                {
                    self.read_lpcomp_refsel(
                        &mut buffer_mut(_start, _end, 1288, _buf),
                        &mut buffer_mut(_start, _end, 1289, _buf),
                        &mut buffer_mut(_start, _end, 1290, _buf),
                        &mut buffer_mut(_start, _end, 1291, _buf),
                    )?;
                }
                if (_start >= 1292 && _start < 1296)
                    || (_end > 1292 && _end <= 1296)
                {
                    self.read_lpcomp_extrefsel(
                        &mut buffer_mut(_start, _end, 1292, _buf),
                        &mut buffer_mut(_start, _end, 1293, _buf),
                        &mut buffer_mut(_start, _end, 1294, _buf),
                        &mut buffer_mut(_start, _end, 1295, _buf),
                    )?;
                }
            }
            (1312..=1315, 1313..=1316) => {
                if (_start >= 1312 && _start < 1316)
                    || (_end > 1312 && _end <= 1316)
                {
                    self.read_lpcomp_anadetect(
                        &mut buffer_mut(_start, _end, 1312, _buf),
                        &mut buffer_mut(_start, _end, 1313, _buf),
                        &mut buffer_mut(_start, _end, 1314, _buf),
                        &mut buffer_mut(_start, _end, 1315, _buf),
                    )?;
                }
            }
            (4092..=4095, 4093..=4096) => {
                if (_start >= 4092 && _start < 4096)
                    || (_end > 4092 && _end <= 4096)
                {
                    self.read_lpcomp_power(
                        &mut buffer_mut(_start, _end, 4092, _buf),
                        &mut buffer_mut(_start, _end, 4093, _buf),
                        &mut buffer_mut(_start, _end, 4094, _buf),
                        &mut buffer_mut(_start, _end, 4095, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
    fn write(
        &mut self,
        _addr: u64,
        _buf: &[u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073819648;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=11, 1..=12) => {
                if (_start < 4) || (_end <= 4) {
                    self.0.lock().unwrap().write_lpcomp_tasks_start(
                        buffer_const(_start, _end, 0, _buf),
                        buffer_const(_start, _end, 1, _buf),
                        buffer_const(_start, _end, 2, _buf),
                        buffer_const(_start, _end, 3, _buf),
                    )?;
                }
                if (_start >= 4 && _start < 8) || (_end > 4 && _end <= 8) {
                    self.0.lock().unwrap().write_lpcomp_tasks_stop(
                        buffer_const(_start, _end, 4, _buf),
                        buffer_const(_start, _end, 5, _buf),
                        buffer_const(_start, _end, 6, _buf),
                        buffer_const(_start, _end, 7, _buf),
                    )?;
                }
                if (_start >= 8 && _start < 12) || (_end > 8 && _end <= 12) {
                    self.0.lock().unwrap().write_lpcomp_tasks_sample(
                        buffer_const(_start, _end, 8, _buf),
                        buffer_const(_start, _end, 9, _buf),
                        buffer_const(_start, _end, 10, _buf),
                        buffer_const(_start, _end, 11, _buf),
                    )?;
                }
            }
            (256..=271, 257..=272) => {
                if (_start >= 256 && _start < 260)
                    || (_end > 256 && _end <= 260)
                {
                    self.0.lock().unwrap().write_lpcomp_events_ready(
                        buffer_const(_start, _end, 256, _buf),
                        buffer_const(_start, _end, 257, _buf),
                        buffer_const(_start, _end, 258, _buf),
                        buffer_const(_start, _end, 259, _buf),
                    )?;
                }
                if (_start >= 260 && _start < 264)
                    || (_end > 260 && _end <= 264)
                {
                    self.0.lock().unwrap().write_lpcomp_events_down(
                        buffer_const(_start, _end, 260, _buf),
                        buffer_const(_start, _end, 261, _buf),
                        buffer_const(_start, _end, 262, _buf),
                        buffer_const(_start, _end, 263, _buf),
                    )?;
                }
                if (_start >= 264 && _start < 268)
                    || (_end > 264 && _end <= 268)
                {
                    self.0.lock().unwrap().write_lpcomp_events_up(
                        buffer_const(_start, _end, 264, _buf),
                        buffer_const(_start, _end, 265, _buf),
                        buffer_const(_start, _end, 266, _buf),
                        buffer_const(_start, _end, 267, _buf),
                    )?;
                }
                if (_start >= 268 && _start < 272)
                    || (_end > 268 && _end <= 272)
                {
                    self.0.lock().unwrap().write_lpcomp_events_cross(
                        buffer_const(_start, _end, 268, _buf),
                        buffer_const(_start, _end, 269, _buf),
                        buffer_const(_start, _end, 270, _buf),
                        buffer_const(_start, _end, 271, _buf),
                    )?;
                }
            }
            (512..=515, 513..=516) => {
                if (_start >= 512 && _start < 516)
                    || (_end > 512 && _end <= 516)
                {
                    self.write_lpcomp_shorts(
                        buffer_const(_start, _end, 512, _buf),
                        buffer_const(_start, _end, 513, _buf),
                        buffer_const(_start, _end, 514, _buf),
                        buffer_const(_start, _end, 515, _buf),
                    )?;
                }
            }
            (772..=779, 773..=780) => {
                if (_start >= 772 && _start < 776)
                    || (_end > 772 && _end <= 776)
                {
                    self.write_lpcomp_intenset(
                        buffer_const(_start, _end, 772, _buf),
                        buffer_const(_start, _end, 773, _buf),
                        buffer_const(_start, _end, 774, _buf),
                        buffer_const(_start, _end, 775, _buf),
                    )?;
                }
                if (_start >= 776 && _start < 780)
                    || (_end > 776 && _end <= 780)
                {
                    self.write_lpcomp_intenclr(
                        buffer_const(_start, _end, 776, _buf),
                        buffer_const(_start, _end, 777, _buf),
                        buffer_const(_start, _end, 778, _buf),
                        buffer_const(_start, _end, 779, _buf),
                    )?;
                }
            }
            (1024..=1027, 1025..=1028) => {
                if (_start >= 1024 && _start < 1028)
                    || (_end > 1024 && _end <= 1028)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (1280..=1295, 1281..=1296) => {
                if (_start >= 1280 && _start < 1284)
                    || (_end > 1280 && _end <= 1284)
                {
                    self.write_lpcomp_enable(
                        buffer_const(_start, _end, 1280, _buf),
                        buffer_const(_start, _end, 1281, _buf),
                        buffer_const(_start, _end, 1282, _buf),
                        buffer_const(_start, _end, 1283, _buf),
                    )?;
                }
                if (_start >= 1284 && _start < 1288)
                    || (_end > 1284 && _end <= 1288)
                {
                    self.write_lpcomp_psel(
                        buffer_const(_start, _end, 1284, _buf),
                        buffer_const(_start, _end, 1285, _buf),
                        buffer_const(_start, _end, 1286, _buf),
                        buffer_const(_start, _end, 1287, _buf),
                    )?;
                }
                if (_start >= 1288 && _start < 1292)
                    || (_end > 1288 && _end <= 1292)
                {
                    self.write_lpcomp_refsel(
                        buffer_const(_start, _end, 1288, _buf),
                        buffer_const(_start, _end, 1289, _buf),
                        buffer_const(_start, _end, 1290, _buf),
                        buffer_const(_start, _end, 1291, _buf),
                    )?;
                }
                if (_start >= 1292 && _start < 1296)
                    || (_end > 1292 && _end <= 1296)
                {
                    self.write_lpcomp_extrefsel(
                        buffer_const(_start, _end, 1292, _buf),
                        buffer_const(_start, _end, 1293, _buf),
                        buffer_const(_start, _end, 1294, _buf),
                        buffer_const(_start, _end, 1295, _buf),
                    )?;
                }
            }
            (1312..=1315, 1313..=1316) => {
                if (_start >= 1312 && _start < 1316)
                    || (_end > 1312 && _end <= 1316)
                {
                    self.write_lpcomp_anadetect(
                        buffer_const(_start, _end, 1312, _buf),
                        buffer_const(_start, _end, 1313, _buf),
                        buffer_const(_start, _end, 1314, _buf),
                        buffer_const(_start, _end, 1315, _buf),
                    )?;
                }
            }
            (4092..=4095, 4093..=4096) => {
                if (_start >= 4092 && _start < 4096)
                    || (_end > 4092 && _end <= 4096)
                {
                    self.write_lpcomp_power(
                        buffer_const(_start, _end, 4092, _buf),
                        buffer_const(_start, _end, 4093, _buf),
                        buffer_const(_start, _end, 4094, _buf),
                        buffer_const(_start, _end, 4095, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
}
impl PeripheralPage0x40013000 {
    fn read_lpcomp_shorts(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_lpcomp_shorts_ready_sample()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_lpcomp_shorts_ready_stop()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_lpcomp_shorts_down_stop()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_lpcomp_shorts_up_stop()? << 3;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_lpcomp_shorts_cross_stop()? << 4;
        }
        Ok(())
    }
    fn write_lpcomp_shorts(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_lpcomp_shorts_ready_sample((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_lpcomp_shorts_ready_stop((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_lpcomp_shorts_down_stop((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_lpcomp_shorts_up_stop((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_lpcomp_shorts_cross_stop((*byte >> 4) & 1)?;
        }
        Ok(())
    }
    fn read_lpcomp_intenset(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_lpcomp_intenset_ready()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_lpcomp_intenset_down()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_lpcomp_intenset_up()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_lpcomp_intenset_cross()? << 3;
        }
        Ok(())
    }
    fn write_lpcomp_intenset(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_lpcomp_intenset_ready((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_lpcomp_intenset_down((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_lpcomp_intenset_up((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_lpcomp_intenset_cross((*byte >> 3) & 1)?;
        }
        Ok(())
    }
    fn read_lpcomp_intenclr(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_lpcomp_intenclr_ready()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_lpcomp_intenclr_down()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_lpcomp_intenclr_up()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_lpcomp_intenclr_cross()? << 3;
        }
        Ok(())
    }
    fn write_lpcomp_intenclr(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_lpcomp_intenclr_ready((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_lpcomp_intenclr_down((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_lpcomp_intenclr_up((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_lpcomp_intenclr_cross((*byte >> 3) & 1)?;
        }
        Ok(())
    }
    fn read_lpcomp_result(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_lpcomp_result_result()? << 0;
        }
        Ok(())
    }
    fn read_lpcomp_enable(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_lpcomp_enable_enable()? << 0;
        }
        Ok(())
    }
    fn write_lpcomp_enable(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_lpcomp_enable_enable((*byte >> 0) & 3)?;
        }
        Ok(())
    }
    fn read_lpcomp_psel(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_lpcomp_psel_psel()? << 0;
        }
        Ok(())
    }
    fn write_lpcomp_psel(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_lpcomp_psel_psel((*byte >> 0) & 7)?;
        }
        Ok(())
    }
    fn read_lpcomp_refsel(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_lpcomp_refsel_refsel()? << 0;
        }
        Ok(())
    }
    fn write_lpcomp_refsel(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_lpcomp_refsel_refsel((*byte >> 0) & 7)?;
        }
        Ok(())
    }
    fn read_lpcomp_extrefsel(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_lpcomp_extrefsel_extrefsel()? << 0;
        }
        Ok(())
    }
    fn write_lpcomp_extrefsel(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_lpcomp_extrefsel_extrefsel((*byte >> 0) & 1)?;
        }
        Ok(())
    }
    fn read_lpcomp_anadetect(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_lpcomp_anadetect_anadetect()? << 0;
        }
        Ok(())
    }
    fn write_lpcomp_anadetect(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_lpcomp_anadetect_anadetect((*byte >> 0) & 3)?;
        }
        Ok(())
    }
    fn read_lpcomp_power(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_lpcomp_power_power()? << 0;
        }
        Ok(())
    }
    fn write_lpcomp_power(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_lpcomp_power_power((*byte >> 0) & 1)?;
        }
        Ok(())
    }
}
pub struct PeripheralPage0x40014000(
    pub std::sync::Arc<std::sync::Mutex<super::peripheral::Peripherals>>,
);
impl icicle_vm::cpu::mem::IoMemory for PeripheralPage0x40014000 {
    fn read(
        &mut self,
        _addr: u64,
        _buf: &mut [u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073823744;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=3, 1..=4) => {
                if (_start < 4) || (_end <= 4) {
                    self.0.lock().unwrap().read_swi_unused(
                        &mut buffer_mut(_start, _end, 0, _buf),
                        &mut buffer_mut(_start, _end, 1, _buf),
                        &mut buffer_mut(_start, _end, 2, _buf),
                        &mut buffer_mut(_start, _end, 3, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
    fn write(
        &mut self,
        _addr: u64,
        _buf: &[u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073823744;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=3, 1..=4) => {
                if (_start < 4) || (_end <= 4) {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
}
impl PeripheralPage0x40014000 {}
pub struct PeripheralPage0x4001E000(
    pub std::sync::Arc<std::sync::Mutex<super::peripheral::Peripherals>>,
);
impl icicle_vm::cpu::mem::IoMemory for PeripheralPage0x4001E000 {
    fn read(
        &mut self,
        _addr: u64,
        _buf: &mut [u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073864704;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (1024..=1027, 1025..=1028) => {
                if (_start >= 1024 && _start < 1028)
                    || (_end > 1024 && _end <= 1028)
                {
                    self.read_nvmc_ready(
                        &mut buffer_mut(_start, _end, 1024, _buf),
                        &mut buffer_mut(_start, _end, 1025, _buf),
                        &mut buffer_mut(_start, _end, 1026, _buf),
                        &mut buffer_mut(_start, _end, 1027, _buf),
                    )?;
                }
            }
            (1284..=1303, 1285..=1304) => {
                if (_start >= 1284 && _start < 1288)
                    || (_end > 1284 && _end <= 1288)
                {
                    self.read_nvmc_config(
                        &mut buffer_mut(_start, _end, 1284, _buf),
                        &mut buffer_mut(_start, _end, 1285, _buf),
                        &mut buffer_mut(_start, _end, 1286, _buf),
                        &mut buffer_mut(_start, _end, 1287, _buf),
                    )?;
                }
                if (_start >= 1288 && _start < 1292)
                    || (_end > 1288 && _end <= 1292)
                {
                    self.0.lock().unwrap().read_nvmc_erasepage(
                        &mut buffer_mut(_start, _end, 1288, _buf),
                        &mut buffer_mut(_start, _end, 1289, _buf),
                        &mut buffer_mut(_start, _end, 1290, _buf),
                        &mut buffer_mut(_start, _end, 1291, _buf),
                    )?;
                }
                if (_start >= 1292 && _start < 1296)
                    || (_end > 1292 && _end <= 1296)
                {
                    self.read_nvmc_eraseall(
                        &mut buffer_mut(_start, _end, 1292, _buf),
                        &mut buffer_mut(_start, _end, 1293, _buf),
                        &mut buffer_mut(_start, _end, 1294, _buf),
                        &mut buffer_mut(_start, _end, 1295, _buf),
                    )?;
                }
                if (_start >= 1296 && _start < 1300)
                    || (_end > 1296 && _end <= 1300)
                {
                    self.0.lock().unwrap().read_nvmc_erasepcr0(
                        &mut buffer_mut(_start, _end, 1296, _buf),
                        &mut buffer_mut(_start, _end, 1297, _buf),
                        &mut buffer_mut(_start, _end, 1298, _buf),
                        &mut buffer_mut(_start, _end, 1299, _buf),
                    )?;
                }
                if (_start >= 1300 && _start < 1304)
                    || (_end > 1300 && _end <= 1304)
                {
                    self.read_nvmc_eraseuicr(
                        &mut buffer_mut(_start, _end, 1300, _buf),
                        &mut buffer_mut(_start, _end, 1301, _buf),
                        &mut buffer_mut(_start, _end, 1302, _buf),
                        &mut buffer_mut(_start, _end, 1303, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
    fn write(
        &mut self,
        _addr: u64,
        _buf: &[u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073864704;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (1024..=1027, 1025..=1028) => {
                if (_start >= 1024 && _start < 1028)
                    || (_end > 1024 && _end <= 1028)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
            }
            (1284..=1303, 1285..=1304) => {
                if (_start >= 1284 && _start < 1288)
                    || (_end > 1284 && _end <= 1288)
                {
                    self.write_nvmc_config(
                        buffer_const(_start, _end, 1284, _buf),
                        buffer_const(_start, _end, 1285, _buf),
                        buffer_const(_start, _end, 1286, _buf),
                        buffer_const(_start, _end, 1287, _buf),
                    )?;
                }
                if (_start >= 1288 && _start < 1292)
                    || (_end > 1288 && _end <= 1292)
                {
                    self.0.lock().unwrap().write_nvmc_erasepage(
                        buffer_const(_start, _end, 1288, _buf),
                        buffer_const(_start, _end, 1289, _buf),
                        buffer_const(_start, _end, 1290, _buf),
                        buffer_const(_start, _end, 1291, _buf),
                    )?;
                }
                if (_start >= 1292 && _start < 1296)
                    || (_end > 1292 && _end <= 1296)
                {
                    self.write_nvmc_eraseall(
                        buffer_const(_start, _end, 1292, _buf),
                        buffer_const(_start, _end, 1293, _buf),
                        buffer_const(_start, _end, 1294, _buf),
                        buffer_const(_start, _end, 1295, _buf),
                    )?;
                }
                if (_start >= 1296 && _start < 1300)
                    || (_end > 1296 && _end <= 1300)
                {
                    self.0.lock().unwrap().write_nvmc_erasepcr0(
                        buffer_const(_start, _end, 1296, _buf),
                        buffer_const(_start, _end, 1297, _buf),
                        buffer_const(_start, _end, 1298, _buf),
                        buffer_const(_start, _end, 1299, _buf),
                    )?;
                }
                if (_start >= 1300 && _start < 1304)
                    || (_end > 1300 && _end <= 1304)
                {
                    self.write_nvmc_eraseuicr(
                        buffer_const(_start, _end, 1300, _buf),
                        buffer_const(_start, _end, 1301, _buf),
                        buffer_const(_start, _end, 1302, _buf),
                        buffer_const(_start, _end, 1303, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
}
impl PeripheralPage0x4001E000 {
    fn read_nvmc_ready(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_nvmc_ready_ready()? << 0;
        }
        Ok(())
    }
    fn read_nvmc_config(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_nvmc_config_wen()? << 0;
        }
        Ok(())
    }
    fn write_nvmc_config(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_nvmc_config_wen((*byte >> 0) & 3)?;
        }
        Ok(())
    }
    fn read_nvmc_eraseall(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_nvmc_eraseall_eraseall()? << 0;
        }
        Ok(())
    }
    fn write_nvmc_eraseall(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_nvmc_eraseall_eraseall((*byte >> 0) & 1)?;
        }
        Ok(())
    }
    fn read_nvmc_eraseuicr(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_nvmc_eraseuicr_eraseuicr()? << 0;
        }
        Ok(())
    }
    fn write_nvmc_eraseuicr(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_nvmc_eraseuicr_eraseuicr((*byte >> 0) & 1)?;
        }
        Ok(())
    }
}
pub struct PeripheralPage0x4001F000(
    pub std::sync::Arc<std::sync::Mutex<super::peripheral::Peripherals>>,
);
impl icicle_vm::cpu::mem::IoMemory for PeripheralPage0x4001F000 {
    fn read(
        &mut self,
        _addr: u64,
        _buf: &mut [u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073868800;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=7, 1..=8) => {}
            (1280..=1291, 1281..=1292) => {
                if (_start >= 1280 && _start < 1284)
                    || (_end > 1280 && _end <= 1284)
                {
                    self.read_ppi_chen(
                        &mut buffer_mut(_start, _end, 1280, _buf),
                        &mut buffer_mut(_start, _end, 1281, _buf),
                        &mut buffer_mut(_start, _end, 1282, _buf),
                        &mut buffer_mut(_start, _end, 1283, _buf),
                    )?;
                }
                if (_start >= 1284 && _start < 1288)
                    || (_end > 1284 && _end <= 1288)
                {
                    self.read_ppi_chenset(
                        &mut buffer_mut(_start, _end, 1284, _buf),
                        &mut buffer_mut(_start, _end, 1285, _buf),
                        &mut buffer_mut(_start, _end, 1286, _buf),
                        &mut buffer_mut(_start, _end, 1287, _buf),
                    )?;
                }
                if (_start >= 1288 && _start < 1292)
                    || (_end > 1288 && _end <= 1292)
                {
                    self.read_ppi_chenclr(
                        &mut buffer_mut(_start, _end, 1288, _buf),
                        &mut buffer_mut(_start, _end, 1289, _buf),
                        &mut buffer_mut(_start, _end, 1290, _buf),
                        &mut buffer_mut(_start, _end, 1291, _buf),
                    )?;
                }
            }
            (2048..=2063, 2049..=2064) => {
                if (_start >= 2048 && _start < 2052)
                    || (_end > 2048 && _end <= 2052)
                {
                    self.read_ppi_chgn(
                        0,
                        &mut buffer_mut(_start, _end, 2048, _buf),
                        &mut buffer_mut(_start, _end, 2049, _buf),
                        &mut buffer_mut(_start, _end, 2050, _buf),
                        &mut buffer_mut(_start, _end, 2051, _buf),
                    )?;
                }
                if (_start >= 2052 && _start < 2056)
                    || (_end > 2052 && _end <= 2056)
                {
                    self.read_ppi_chgn(
                        1,
                        &mut buffer_mut(_start, _end, 2052, _buf),
                        &mut buffer_mut(_start, _end, 2053, _buf),
                        &mut buffer_mut(_start, _end, 2054, _buf),
                        &mut buffer_mut(_start, _end, 2055, _buf),
                    )?;
                }
                if (_start >= 2056 && _start < 2060)
                    || (_end > 2056 && _end <= 2060)
                {
                    self.read_ppi_chgn(
                        2,
                        &mut buffer_mut(_start, _end, 2056, _buf),
                        &mut buffer_mut(_start, _end, 2057, _buf),
                        &mut buffer_mut(_start, _end, 2058, _buf),
                        &mut buffer_mut(_start, _end, 2059, _buf),
                    )?;
                }
                if (_start >= 2060 && _start < 2064)
                    || (_end > 2060 && _end <= 2064)
                {
                    self.read_ppi_chgn(
                        3,
                        &mut buffer_mut(_start, _end, 2060, _buf),
                        &mut buffer_mut(_start, _end, 2061, _buf),
                        &mut buffer_mut(_start, _end, 2062, _buf),
                        &mut buffer_mut(_start, _end, 2063, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
    fn write(
        &mut self,
        _addr: u64,
        _buf: &[u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1073868800;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (..=7, 1..=8) => {}
            (1280..=1291, 1281..=1292) => {
                if (_start >= 1280 && _start < 1284)
                    || (_end > 1280 && _end <= 1284)
                {
                    self.write_ppi_chen(
                        buffer_const(_start, _end, 1280, _buf),
                        buffer_const(_start, _end, 1281, _buf),
                        buffer_const(_start, _end, 1282, _buf),
                        buffer_const(_start, _end, 1283, _buf),
                    )?;
                }
                if (_start >= 1284 && _start < 1288)
                    || (_end > 1284 && _end <= 1288)
                {
                    self.write_ppi_chenset(
                        buffer_const(_start, _end, 1284, _buf),
                        buffer_const(_start, _end, 1285, _buf),
                        buffer_const(_start, _end, 1286, _buf),
                        buffer_const(_start, _end, 1287, _buf),
                    )?;
                }
                if (_start >= 1288 && _start < 1292)
                    || (_end > 1288 && _end <= 1292)
                {
                    self.write_ppi_chenclr(
                        buffer_const(_start, _end, 1288, _buf),
                        buffer_const(_start, _end, 1289, _buf),
                        buffer_const(_start, _end, 1290, _buf),
                        buffer_const(_start, _end, 1291, _buf),
                    )?;
                }
            }
            (2048..=2063, 2049..=2064) => {
                if (_start >= 2048 && _start < 2052)
                    || (_end > 2048 && _end <= 2052)
                {
                    self.write_ppi_chgn(
                        0,
                        buffer_const(_start, _end, 2048, _buf),
                        buffer_const(_start, _end, 2049, _buf),
                        buffer_const(_start, _end, 2050, _buf),
                        buffer_const(_start, _end, 2051, _buf),
                    )?;
                }
                if (_start >= 2052 && _start < 2056)
                    || (_end > 2052 && _end <= 2056)
                {
                    self.write_ppi_chgn(
                        1,
                        buffer_const(_start, _end, 2052, _buf),
                        buffer_const(_start, _end, 2053, _buf),
                        buffer_const(_start, _end, 2054, _buf),
                        buffer_const(_start, _end, 2055, _buf),
                    )?;
                }
                if (_start >= 2056 && _start < 2060)
                    || (_end > 2056 && _end <= 2060)
                {
                    self.write_ppi_chgn(
                        2,
                        buffer_const(_start, _end, 2056, _buf),
                        buffer_const(_start, _end, 2057, _buf),
                        buffer_const(_start, _end, 2058, _buf),
                        buffer_const(_start, _end, 2059, _buf),
                    )?;
                }
                if (_start >= 2060 && _start < 2064)
                    || (_end > 2060 && _end <= 2064)
                {
                    self.write_ppi_chgn(
                        3,
                        buffer_const(_start, _end, 2060, _buf),
                        buffer_const(_start, _end, 2061, _buf),
                        buffer_const(_start, _end, 2062, _buf),
                        buffer_const(_start, _end, 2063, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
}
impl PeripheralPage0x4001F000 {
    fn read_ppi_chen(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ppi_chen_ch0()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ppi_chen_ch1()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ppi_chen_ch2()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ppi_chen_ch3()? << 3;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ppi_chen_ch4()? << 4;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ppi_chen_ch5()? << 5;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ppi_chen_ch6()? << 6;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ppi_chen_ch7()? << 7;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_ppi_chen_ch8()? << 0;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_ppi_chen_ch9()? << 1;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_ppi_chen_ch10()? << 2;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_ppi_chen_ch11()? << 3;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_ppi_chen_ch12()? << 4;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_ppi_chen_ch13()? << 5;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_ppi_chen_ch14()? << 6;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_ppi_chen_ch15()? << 7;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_ppi_chen_ch20()? << 4;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_ppi_chen_ch21()? << 5;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_ppi_chen_ch22()? << 6;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_ppi_chen_ch23()? << 7;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_ppi_chen_ch24()? << 0;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_ppi_chen_ch25()? << 1;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_ppi_chen_ch26()? << 2;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_ppi_chen_ch27()? << 3;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_ppi_chen_ch28()? << 4;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_ppi_chen_ch29()? << 5;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_ppi_chen_ch30()? << 6;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_ppi_chen_ch31()? << 7;
        }
        Ok(())
    }
    fn write_ppi_chen(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chen_ch0((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chen_ch1((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chen_ch2((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chen_ch3((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chen_ch4((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chen_ch5((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chen_ch6((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chen_ch7((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chen_ch8((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chen_ch9((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chen_ch10((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chen_ch11((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chen_ch12((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chen_ch13((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chen_ch14((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chen_ch15((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chen_ch20((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chen_ch21((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chen_ch22((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chen_ch23((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chen_ch24((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chen_ch25((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chen_ch26((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chen_ch27((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chen_ch28((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chen_ch29((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chen_ch30((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chen_ch31((*byte >> 7) & 1)?;
        }
        Ok(())
    }
    fn read_ppi_chenset(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ppi_chenset_ch0()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ppi_chenset_ch1()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ppi_chenset_ch2()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ppi_chenset_ch3()? << 3;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ppi_chenset_ch4()? << 4;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ppi_chenset_ch5()? << 5;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ppi_chenset_ch6()? << 6;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ppi_chenset_ch7()? << 7;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_ppi_chenset_ch8()? << 0;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_ppi_chenset_ch9()? << 1;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_ppi_chenset_ch10()? << 2;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_ppi_chenset_ch11()? << 3;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_ppi_chenset_ch12()? << 4;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_ppi_chenset_ch13()? << 5;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_ppi_chenset_ch14()? << 6;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_ppi_chenset_ch15()? << 7;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_ppi_chenset_ch20()? << 4;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_ppi_chenset_ch21()? << 5;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_ppi_chenset_ch22()? << 6;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_ppi_chenset_ch23()? << 7;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_ppi_chenset_ch24()? << 0;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_ppi_chenset_ch25()? << 1;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_ppi_chenset_ch26()? << 2;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_ppi_chenset_ch27()? << 3;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_ppi_chenset_ch28()? << 4;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_ppi_chenset_ch29()? << 5;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_ppi_chenset_ch30()? << 6;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_ppi_chenset_ch31()? << 7;
        }
        Ok(())
    }
    fn write_ppi_chenset(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenset_ch0((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenset_ch1((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenset_ch2((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenset_ch3((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenset_ch4((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenset_ch5((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenset_ch6((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenset_ch7((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenset_ch8((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenset_ch9((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenset_ch10((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenset_ch11((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenset_ch12((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenset_ch13((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenset_ch14((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenset_ch15((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenset_ch20((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenset_ch21((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenset_ch22((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenset_ch23((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenset_ch24((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenset_ch25((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenset_ch26((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenset_ch27((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenset_ch28((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenset_ch29((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenset_ch30((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenset_ch31((*byte >> 7) & 1)?;
        }
        Ok(())
    }
    fn read_ppi_chenclr(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ppi_chenclr_ch0()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ppi_chenclr_ch1()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ppi_chenclr_ch2()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ppi_chenclr_ch3()? << 3;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ppi_chenclr_ch4()? << 4;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ppi_chenclr_ch5()? << 5;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ppi_chenclr_ch6()? << 6;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ppi_chenclr_ch7()? << 7;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_ppi_chenclr_ch8()? << 0;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_ppi_chenclr_ch9()? << 1;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_ppi_chenclr_ch10()? << 2;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_ppi_chenclr_ch11()? << 3;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_ppi_chenclr_ch12()? << 4;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_ppi_chenclr_ch13()? << 5;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_ppi_chenclr_ch14()? << 6;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_ppi_chenclr_ch15()? << 7;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_ppi_chenclr_ch20()? << 4;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_ppi_chenclr_ch21()? << 5;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_ppi_chenclr_ch22()? << 6;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_ppi_chenclr_ch23()? << 7;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_ppi_chenclr_ch24()? << 0;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_ppi_chenclr_ch25()? << 1;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_ppi_chenclr_ch26()? << 2;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_ppi_chenclr_ch27()? << 3;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_ppi_chenclr_ch28()? << 4;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_ppi_chenclr_ch29()? << 5;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_ppi_chenclr_ch30()? << 6;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_ppi_chenclr_ch31()? << 7;
        }
        Ok(())
    }
    fn write_ppi_chenclr(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenclr_ch0((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenclr_ch1((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenclr_ch2((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenclr_ch3((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenclr_ch4((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenclr_ch5((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenclr_ch6((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenclr_ch7((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenclr_ch8((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenclr_ch9((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenclr_ch10((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenclr_ch11((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenclr_ch12((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenclr_ch13((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenclr_ch14((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenclr_ch15((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenclr_ch20((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenclr_ch21((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenclr_ch22((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenclr_ch23((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenclr_ch24((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenclr_ch25((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenclr_ch26((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenclr_ch27((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenclr_ch28((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenclr_ch29((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenclr_ch30((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chenclr_ch31((*byte >> 7) & 1)?;
        }
        Ok(())
    }
    fn read_ppi_chgn(
        &self,
        _dim: usize,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ppi_chgn_ch0(_dim)? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ppi_chgn_ch1(_dim)? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ppi_chgn_ch2(_dim)? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ppi_chgn_ch3(_dim)? << 3;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ppi_chgn_ch4(_dim)? << 4;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ppi_chgn_ch5(_dim)? << 5;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ppi_chgn_ch6(_dim)? << 6;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_ppi_chgn_ch7(_dim)? << 7;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_ppi_chgn_ch8(_dim)? << 0;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_ppi_chgn_ch9(_dim)? << 1;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_ppi_chgn_ch10(_dim)? << 2;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_ppi_chgn_ch11(_dim)? << 3;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_ppi_chgn_ch12(_dim)? << 4;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_ppi_chgn_ch13(_dim)? << 5;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_ppi_chgn_ch14(_dim)? << 6;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_ppi_chgn_ch15(_dim)? << 7;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_ppi_chgn_ch20(_dim)? << 4;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_ppi_chgn_ch21(_dim)? << 5;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_ppi_chgn_ch22(_dim)? << 6;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_ppi_chgn_ch23(_dim)? << 7;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_ppi_chgn_ch24(_dim)? << 0;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_ppi_chgn_ch25(_dim)? << 1;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_ppi_chgn_ch26(_dim)? << 2;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_ppi_chgn_ch27(_dim)? << 3;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_ppi_chgn_ch28(_dim)? << 4;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_ppi_chgn_ch29(_dim)? << 5;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_ppi_chgn_ch30(_dim)? << 6;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_ppi_chgn_ch31(_dim)? << 7;
        }
        Ok(())
    }
    fn write_ppi_chgn(
        &self,
        _dim: usize,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chgn_ch0(_dim, (*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chgn_ch1(_dim, (*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chgn_ch2(_dim, (*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chgn_ch3(_dim, (*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chgn_ch4(_dim, (*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chgn_ch5(_dim, (*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chgn_ch6(_dim, (*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chgn_ch7(_dim, (*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chgn_ch8(_dim, (*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chgn_ch9(_dim, (*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chgn_ch10(_dim, (*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chgn_ch11(_dim, (*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chgn_ch12(_dim, (*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chgn_ch13(_dim, (*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chgn_ch14(_dim, (*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chgn_ch15(_dim, (*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chgn_ch20(_dim, (*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chgn_ch21(_dim, (*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chgn_ch22(_dim, (*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chgn_ch23(_dim, (*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chgn_ch24(_dim, (*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chgn_ch25(_dim, (*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chgn_ch26(_dim, (*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chgn_ch27(_dim, (*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chgn_ch28(_dim, (*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chgn_ch29(_dim, (*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chgn_ch30(_dim, (*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_ppi_chgn_ch31(_dim, (*byte >> 7) & 1)?;
        }
        Ok(())
    }
}
pub struct PeripheralPage0x50000000(
    pub std::sync::Arc<std::sync::Mutex<super::peripheral::Peripherals>>,
);
impl icicle_vm::cpu::mem::IoMemory for PeripheralPage0x50000000 {
    fn read(
        &mut self,
        _addr: u64,
        _buf: &mut [u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1342177280;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (1284..=1311, 1285..=1312) => {
                if (_start >= 1284 && _start < 1288)
                    || (_end > 1284 && _end <= 1288)
                {
                    self.read_gpio_out(
                        &mut buffer_mut(_start, _end, 1284, _buf),
                        &mut buffer_mut(_start, _end, 1285, _buf),
                        &mut buffer_mut(_start, _end, 1286, _buf),
                        &mut buffer_mut(_start, _end, 1287, _buf),
                    )?;
                }
                if (_start >= 1288 && _start < 1292)
                    || (_end > 1288 && _end <= 1292)
                {
                    self.read_gpio_outset(
                        &mut buffer_mut(_start, _end, 1288, _buf),
                        &mut buffer_mut(_start, _end, 1289, _buf),
                        &mut buffer_mut(_start, _end, 1290, _buf),
                        &mut buffer_mut(_start, _end, 1291, _buf),
                    )?;
                }
                if (_start >= 1292 && _start < 1296)
                    || (_end > 1292 && _end <= 1296)
                {
                    self.read_gpio_outclr(
                        &mut buffer_mut(_start, _end, 1292, _buf),
                        &mut buffer_mut(_start, _end, 1293, _buf),
                        &mut buffer_mut(_start, _end, 1294, _buf),
                        &mut buffer_mut(_start, _end, 1295, _buf),
                    )?;
                }
                if (_start >= 1296 && _start < 1300)
                    || (_end > 1296 && _end <= 1300)
                {
                    self.read_gpio_in(
                        &mut buffer_mut(_start, _end, 1296, _buf),
                        &mut buffer_mut(_start, _end, 1297, _buf),
                        &mut buffer_mut(_start, _end, 1298, _buf),
                        &mut buffer_mut(_start, _end, 1299, _buf),
                    )?;
                }
                if (_start >= 1300 && _start < 1304)
                    || (_end > 1300 && _end <= 1304)
                {
                    self.read_gpio_dir(
                        &mut buffer_mut(_start, _end, 1300, _buf),
                        &mut buffer_mut(_start, _end, 1301, _buf),
                        &mut buffer_mut(_start, _end, 1302, _buf),
                        &mut buffer_mut(_start, _end, 1303, _buf),
                    )?;
                }
                if (_start >= 1304 && _start < 1308)
                    || (_end > 1304 && _end <= 1308)
                {
                    self.read_gpio_dirset(
                        &mut buffer_mut(_start, _end, 1304, _buf),
                        &mut buffer_mut(_start, _end, 1305, _buf),
                        &mut buffer_mut(_start, _end, 1306, _buf),
                        &mut buffer_mut(_start, _end, 1307, _buf),
                    )?;
                }
                if (_start >= 1308 && _start < 1312)
                    || (_end > 1308 && _end <= 1312)
                {
                    self.read_gpio_dirclr(
                        &mut buffer_mut(_start, _end, 1308, _buf),
                        &mut buffer_mut(_start, _end, 1309, _buf),
                        &mut buffer_mut(_start, _end, 1310, _buf),
                        &mut buffer_mut(_start, _end, 1311, _buf),
                    )?;
                }
            }
            (1792..=1919, 1793..=1920) => {
                if (_start >= 1792 && _start < 1796)
                    || (_end > 1792 && _end <= 1796)
                {
                    self.read_gpio_pin_cnfn(
                        0,
                        &mut buffer_mut(_start, _end, 1792, _buf),
                        &mut buffer_mut(_start, _end, 1793, _buf),
                        &mut buffer_mut(_start, _end, 1794, _buf),
                        &mut buffer_mut(_start, _end, 1795, _buf),
                    )?;
                }
                if (_start >= 1796 && _start < 1800)
                    || (_end > 1796 && _end <= 1800)
                {
                    self.read_gpio_pin_cnfn(
                        1,
                        &mut buffer_mut(_start, _end, 1796, _buf),
                        &mut buffer_mut(_start, _end, 1797, _buf),
                        &mut buffer_mut(_start, _end, 1798, _buf),
                        &mut buffer_mut(_start, _end, 1799, _buf),
                    )?;
                }
                if (_start >= 1800 && _start < 1804)
                    || (_end > 1800 && _end <= 1804)
                {
                    self.read_gpio_pin_cnfn(
                        2,
                        &mut buffer_mut(_start, _end, 1800, _buf),
                        &mut buffer_mut(_start, _end, 1801, _buf),
                        &mut buffer_mut(_start, _end, 1802, _buf),
                        &mut buffer_mut(_start, _end, 1803, _buf),
                    )?;
                }
                if (_start >= 1804 && _start < 1808)
                    || (_end > 1804 && _end <= 1808)
                {
                    self.read_gpio_pin_cnfn(
                        3,
                        &mut buffer_mut(_start, _end, 1804, _buf),
                        &mut buffer_mut(_start, _end, 1805, _buf),
                        &mut buffer_mut(_start, _end, 1806, _buf),
                        &mut buffer_mut(_start, _end, 1807, _buf),
                    )?;
                }
                if (_start >= 1808 && _start < 1812)
                    || (_end > 1808 && _end <= 1812)
                {
                    self.read_gpio_pin_cnfn(
                        4,
                        &mut buffer_mut(_start, _end, 1808, _buf),
                        &mut buffer_mut(_start, _end, 1809, _buf),
                        &mut buffer_mut(_start, _end, 1810, _buf),
                        &mut buffer_mut(_start, _end, 1811, _buf),
                    )?;
                }
                if (_start >= 1812 && _start < 1816)
                    || (_end > 1812 && _end <= 1816)
                {
                    self.read_gpio_pin_cnfn(
                        5,
                        &mut buffer_mut(_start, _end, 1812, _buf),
                        &mut buffer_mut(_start, _end, 1813, _buf),
                        &mut buffer_mut(_start, _end, 1814, _buf),
                        &mut buffer_mut(_start, _end, 1815, _buf),
                    )?;
                }
                if (_start >= 1816 && _start < 1820)
                    || (_end > 1816 && _end <= 1820)
                {
                    self.read_gpio_pin_cnfn(
                        6,
                        &mut buffer_mut(_start, _end, 1816, _buf),
                        &mut buffer_mut(_start, _end, 1817, _buf),
                        &mut buffer_mut(_start, _end, 1818, _buf),
                        &mut buffer_mut(_start, _end, 1819, _buf),
                    )?;
                }
                if (_start >= 1820 && _start < 1824)
                    || (_end > 1820 && _end <= 1824)
                {
                    self.read_gpio_pin_cnfn(
                        7,
                        &mut buffer_mut(_start, _end, 1820, _buf),
                        &mut buffer_mut(_start, _end, 1821, _buf),
                        &mut buffer_mut(_start, _end, 1822, _buf),
                        &mut buffer_mut(_start, _end, 1823, _buf),
                    )?;
                }
                if (_start >= 1824 && _start < 1828)
                    || (_end > 1824 && _end <= 1828)
                {
                    self.read_gpio_pin_cnfn(
                        8,
                        &mut buffer_mut(_start, _end, 1824, _buf),
                        &mut buffer_mut(_start, _end, 1825, _buf),
                        &mut buffer_mut(_start, _end, 1826, _buf),
                        &mut buffer_mut(_start, _end, 1827, _buf),
                    )?;
                }
                if (_start >= 1828 && _start < 1832)
                    || (_end > 1828 && _end <= 1832)
                {
                    self.read_gpio_pin_cnfn(
                        9,
                        &mut buffer_mut(_start, _end, 1828, _buf),
                        &mut buffer_mut(_start, _end, 1829, _buf),
                        &mut buffer_mut(_start, _end, 1830, _buf),
                        &mut buffer_mut(_start, _end, 1831, _buf),
                    )?;
                }
                if (_start >= 1832 && _start < 1836)
                    || (_end > 1832 && _end <= 1836)
                {
                    self.read_gpio_pin_cnfn(
                        10,
                        &mut buffer_mut(_start, _end, 1832, _buf),
                        &mut buffer_mut(_start, _end, 1833, _buf),
                        &mut buffer_mut(_start, _end, 1834, _buf),
                        &mut buffer_mut(_start, _end, 1835, _buf),
                    )?;
                }
                if (_start >= 1836 && _start < 1840)
                    || (_end > 1836 && _end <= 1840)
                {
                    self.read_gpio_pin_cnfn(
                        11,
                        &mut buffer_mut(_start, _end, 1836, _buf),
                        &mut buffer_mut(_start, _end, 1837, _buf),
                        &mut buffer_mut(_start, _end, 1838, _buf),
                        &mut buffer_mut(_start, _end, 1839, _buf),
                    )?;
                }
                if (_start >= 1840 && _start < 1844)
                    || (_end > 1840 && _end <= 1844)
                {
                    self.read_gpio_pin_cnfn(
                        12,
                        &mut buffer_mut(_start, _end, 1840, _buf),
                        &mut buffer_mut(_start, _end, 1841, _buf),
                        &mut buffer_mut(_start, _end, 1842, _buf),
                        &mut buffer_mut(_start, _end, 1843, _buf),
                    )?;
                }
                if (_start >= 1844 && _start < 1848)
                    || (_end > 1844 && _end <= 1848)
                {
                    self.read_gpio_pin_cnfn(
                        13,
                        &mut buffer_mut(_start, _end, 1844, _buf),
                        &mut buffer_mut(_start, _end, 1845, _buf),
                        &mut buffer_mut(_start, _end, 1846, _buf),
                        &mut buffer_mut(_start, _end, 1847, _buf),
                    )?;
                }
                if (_start >= 1848 && _start < 1852)
                    || (_end > 1848 && _end <= 1852)
                {
                    self.read_gpio_pin_cnfn(
                        14,
                        &mut buffer_mut(_start, _end, 1848, _buf),
                        &mut buffer_mut(_start, _end, 1849, _buf),
                        &mut buffer_mut(_start, _end, 1850, _buf),
                        &mut buffer_mut(_start, _end, 1851, _buf),
                    )?;
                }
                if (_start >= 1852 && _start < 1856)
                    || (_end > 1852 && _end <= 1856)
                {
                    self.read_gpio_pin_cnfn(
                        15,
                        &mut buffer_mut(_start, _end, 1852, _buf),
                        &mut buffer_mut(_start, _end, 1853, _buf),
                        &mut buffer_mut(_start, _end, 1854, _buf),
                        &mut buffer_mut(_start, _end, 1855, _buf),
                    )?;
                }
                if (_start >= 1856 && _start < 1860)
                    || (_end > 1856 && _end <= 1860)
                {
                    self.read_gpio_pin_cnfn(
                        16,
                        &mut buffer_mut(_start, _end, 1856, _buf),
                        &mut buffer_mut(_start, _end, 1857, _buf),
                        &mut buffer_mut(_start, _end, 1858, _buf),
                        &mut buffer_mut(_start, _end, 1859, _buf),
                    )?;
                }
                if (_start >= 1860 && _start < 1864)
                    || (_end > 1860 && _end <= 1864)
                {
                    self.read_gpio_pin_cnfn(
                        17,
                        &mut buffer_mut(_start, _end, 1860, _buf),
                        &mut buffer_mut(_start, _end, 1861, _buf),
                        &mut buffer_mut(_start, _end, 1862, _buf),
                        &mut buffer_mut(_start, _end, 1863, _buf),
                    )?;
                }
                if (_start >= 1864 && _start < 1868)
                    || (_end > 1864 && _end <= 1868)
                {
                    self.read_gpio_pin_cnfn(
                        18,
                        &mut buffer_mut(_start, _end, 1864, _buf),
                        &mut buffer_mut(_start, _end, 1865, _buf),
                        &mut buffer_mut(_start, _end, 1866, _buf),
                        &mut buffer_mut(_start, _end, 1867, _buf),
                    )?;
                }
                if (_start >= 1868 && _start < 1872)
                    || (_end > 1868 && _end <= 1872)
                {
                    self.read_gpio_pin_cnfn(
                        19,
                        &mut buffer_mut(_start, _end, 1868, _buf),
                        &mut buffer_mut(_start, _end, 1869, _buf),
                        &mut buffer_mut(_start, _end, 1870, _buf),
                        &mut buffer_mut(_start, _end, 1871, _buf),
                    )?;
                }
                if (_start >= 1872 && _start < 1876)
                    || (_end > 1872 && _end <= 1876)
                {
                    self.read_gpio_pin_cnfn(
                        20,
                        &mut buffer_mut(_start, _end, 1872, _buf),
                        &mut buffer_mut(_start, _end, 1873, _buf),
                        &mut buffer_mut(_start, _end, 1874, _buf),
                        &mut buffer_mut(_start, _end, 1875, _buf),
                    )?;
                }
                if (_start >= 1876 && _start < 1880)
                    || (_end > 1876 && _end <= 1880)
                {
                    self.read_gpio_pin_cnfn(
                        21,
                        &mut buffer_mut(_start, _end, 1876, _buf),
                        &mut buffer_mut(_start, _end, 1877, _buf),
                        &mut buffer_mut(_start, _end, 1878, _buf),
                        &mut buffer_mut(_start, _end, 1879, _buf),
                    )?;
                }
                if (_start >= 1880 && _start < 1884)
                    || (_end > 1880 && _end <= 1884)
                {
                    self.read_gpio_pin_cnfn(
                        22,
                        &mut buffer_mut(_start, _end, 1880, _buf),
                        &mut buffer_mut(_start, _end, 1881, _buf),
                        &mut buffer_mut(_start, _end, 1882, _buf),
                        &mut buffer_mut(_start, _end, 1883, _buf),
                    )?;
                }
                if (_start >= 1884 && _start < 1888)
                    || (_end > 1884 && _end <= 1888)
                {
                    self.read_gpio_pin_cnfn(
                        23,
                        &mut buffer_mut(_start, _end, 1884, _buf),
                        &mut buffer_mut(_start, _end, 1885, _buf),
                        &mut buffer_mut(_start, _end, 1886, _buf),
                        &mut buffer_mut(_start, _end, 1887, _buf),
                    )?;
                }
                if (_start >= 1888 && _start < 1892)
                    || (_end > 1888 && _end <= 1892)
                {
                    self.read_gpio_pin_cnfn(
                        24,
                        &mut buffer_mut(_start, _end, 1888, _buf),
                        &mut buffer_mut(_start, _end, 1889, _buf),
                        &mut buffer_mut(_start, _end, 1890, _buf),
                        &mut buffer_mut(_start, _end, 1891, _buf),
                    )?;
                }
                if (_start >= 1892 && _start < 1896)
                    || (_end > 1892 && _end <= 1896)
                {
                    self.read_gpio_pin_cnfn(
                        25,
                        &mut buffer_mut(_start, _end, 1892, _buf),
                        &mut buffer_mut(_start, _end, 1893, _buf),
                        &mut buffer_mut(_start, _end, 1894, _buf),
                        &mut buffer_mut(_start, _end, 1895, _buf),
                    )?;
                }
                if (_start >= 1896 && _start < 1900)
                    || (_end > 1896 && _end <= 1900)
                {
                    self.read_gpio_pin_cnfn(
                        26,
                        &mut buffer_mut(_start, _end, 1896, _buf),
                        &mut buffer_mut(_start, _end, 1897, _buf),
                        &mut buffer_mut(_start, _end, 1898, _buf),
                        &mut buffer_mut(_start, _end, 1899, _buf),
                    )?;
                }
                if (_start >= 1900 && _start < 1904)
                    || (_end > 1900 && _end <= 1904)
                {
                    self.read_gpio_pin_cnfn(
                        27,
                        &mut buffer_mut(_start, _end, 1900, _buf),
                        &mut buffer_mut(_start, _end, 1901, _buf),
                        &mut buffer_mut(_start, _end, 1902, _buf),
                        &mut buffer_mut(_start, _end, 1903, _buf),
                    )?;
                }
                if (_start >= 1904 && _start < 1908)
                    || (_end > 1904 && _end <= 1908)
                {
                    self.read_gpio_pin_cnfn(
                        28,
                        &mut buffer_mut(_start, _end, 1904, _buf),
                        &mut buffer_mut(_start, _end, 1905, _buf),
                        &mut buffer_mut(_start, _end, 1906, _buf),
                        &mut buffer_mut(_start, _end, 1907, _buf),
                    )?;
                }
                if (_start >= 1908 && _start < 1912)
                    || (_end > 1908 && _end <= 1912)
                {
                    self.read_gpio_pin_cnfn(
                        29,
                        &mut buffer_mut(_start, _end, 1908, _buf),
                        &mut buffer_mut(_start, _end, 1909, _buf),
                        &mut buffer_mut(_start, _end, 1910, _buf),
                        &mut buffer_mut(_start, _end, 1911, _buf),
                    )?;
                }
                if (_start >= 1912 && _start < 1916)
                    || (_end > 1912 && _end <= 1916)
                {
                    self.read_gpio_pin_cnfn(
                        30,
                        &mut buffer_mut(_start, _end, 1912, _buf),
                        &mut buffer_mut(_start, _end, 1913, _buf),
                        &mut buffer_mut(_start, _end, 1914, _buf),
                        &mut buffer_mut(_start, _end, 1915, _buf),
                    )?;
                }
                if (_start >= 1916 && _start < 1920)
                    || (_end > 1916 && _end <= 1920)
                {
                    self.read_gpio_pin_cnfn(
                        31,
                        &mut buffer_mut(_start, _end, 1916, _buf),
                        &mut buffer_mut(_start, _end, 1917, _buf),
                        &mut buffer_mut(_start, _end, 1918, _buf),
                        &mut buffer_mut(_start, _end, 1919, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
    fn write(
        &mut self,
        _addr: u64,
        _buf: &[u8],
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        let _start = _addr - 1342177280;
        let _end = _start + u64::try_from(_buf.len()).unwrap();
        match (_start, _end) {
            (1284..=1311, 1285..=1312) => {
                if (_start >= 1284 && _start < 1288)
                    || (_end > 1284 && _end <= 1288)
                {
                    self.write_gpio_out(
                        buffer_const(_start, _end, 1284, _buf),
                        buffer_const(_start, _end, 1285, _buf),
                        buffer_const(_start, _end, 1286, _buf),
                        buffer_const(_start, _end, 1287, _buf),
                    )?;
                }
                if (_start >= 1288 && _start < 1292)
                    || (_end > 1288 && _end <= 1292)
                {
                    self.write_gpio_outset(
                        buffer_const(_start, _end, 1288, _buf),
                        buffer_const(_start, _end, 1289, _buf),
                        buffer_const(_start, _end, 1290, _buf),
                        buffer_const(_start, _end, 1291, _buf),
                    )?;
                }
                if (_start >= 1292 && _start < 1296)
                    || (_end > 1292 && _end <= 1296)
                {
                    self.write_gpio_outclr(
                        buffer_const(_start, _end, 1292, _buf),
                        buffer_const(_start, _end, 1293, _buf),
                        buffer_const(_start, _end, 1294, _buf),
                        buffer_const(_start, _end, 1295, _buf),
                    )?;
                }
                if (_start >= 1296 && _start < 1300)
                    || (_end > 1296 && _end <= 1300)
                {
                    return Err(icicle_vm::cpu::mem::MemError::WriteViolation);
                }
                if (_start >= 1300 && _start < 1304)
                    || (_end > 1300 && _end <= 1304)
                {
                    self.write_gpio_dir(
                        buffer_const(_start, _end, 1300, _buf),
                        buffer_const(_start, _end, 1301, _buf),
                        buffer_const(_start, _end, 1302, _buf),
                        buffer_const(_start, _end, 1303, _buf),
                    )?;
                }
                if (_start >= 1304 && _start < 1308)
                    || (_end > 1304 && _end <= 1308)
                {
                    self.write_gpio_dirset(
                        buffer_const(_start, _end, 1304, _buf),
                        buffer_const(_start, _end, 1305, _buf),
                        buffer_const(_start, _end, 1306, _buf),
                        buffer_const(_start, _end, 1307, _buf),
                    )?;
                }
                if (_start >= 1308 && _start < 1312)
                    || (_end > 1308 && _end <= 1312)
                {
                    self.write_gpio_dirclr(
                        buffer_const(_start, _end, 1308, _buf),
                        buffer_const(_start, _end, 1309, _buf),
                        buffer_const(_start, _end, 1310, _buf),
                        buffer_const(_start, _end, 1311, _buf),
                    )?;
                }
            }
            (1792..=1919, 1793..=1920) => {
                if (_start >= 1792 && _start < 1796)
                    || (_end > 1792 && _end <= 1796)
                {
                    self.write_gpio_pin_cnfn(
                        0,
                        buffer_const(_start, _end, 1792, _buf),
                        buffer_const(_start, _end, 1793, _buf),
                        buffer_const(_start, _end, 1794, _buf),
                        buffer_const(_start, _end, 1795, _buf),
                    )?;
                }
                if (_start >= 1796 && _start < 1800)
                    || (_end > 1796 && _end <= 1800)
                {
                    self.write_gpio_pin_cnfn(
                        1,
                        buffer_const(_start, _end, 1796, _buf),
                        buffer_const(_start, _end, 1797, _buf),
                        buffer_const(_start, _end, 1798, _buf),
                        buffer_const(_start, _end, 1799, _buf),
                    )?;
                }
                if (_start >= 1800 && _start < 1804)
                    || (_end > 1800 && _end <= 1804)
                {
                    self.write_gpio_pin_cnfn(
                        2,
                        buffer_const(_start, _end, 1800, _buf),
                        buffer_const(_start, _end, 1801, _buf),
                        buffer_const(_start, _end, 1802, _buf),
                        buffer_const(_start, _end, 1803, _buf),
                    )?;
                }
                if (_start >= 1804 && _start < 1808)
                    || (_end > 1804 && _end <= 1808)
                {
                    self.write_gpio_pin_cnfn(
                        3,
                        buffer_const(_start, _end, 1804, _buf),
                        buffer_const(_start, _end, 1805, _buf),
                        buffer_const(_start, _end, 1806, _buf),
                        buffer_const(_start, _end, 1807, _buf),
                    )?;
                }
                if (_start >= 1808 && _start < 1812)
                    || (_end > 1808 && _end <= 1812)
                {
                    self.write_gpio_pin_cnfn(
                        4,
                        buffer_const(_start, _end, 1808, _buf),
                        buffer_const(_start, _end, 1809, _buf),
                        buffer_const(_start, _end, 1810, _buf),
                        buffer_const(_start, _end, 1811, _buf),
                    )?;
                }
                if (_start >= 1812 && _start < 1816)
                    || (_end > 1812 && _end <= 1816)
                {
                    self.write_gpio_pin_cnfn(
                        5,
                        buffer_const(_start, _end, 1812, _buf),
                        buffer_const(_start, _end, 1813, _buf),
                        buffer_const(_start, _end, 1814, _buf),
                        buffer_const(_start, _end, 1815, _buf),
                    )?;
                }
                if (_start >= 1816 && _start < 1820)
                    || (_end > 1816 && _end <= 1820)
                {
                    self.write_gpio_pin_cnfn(
                        6,
                        buffer_const(_start, _end, 1816, _buf),
                        buffer_const(_start, _end, 1817, _buf),
                        buffer_const(_start, _end, 1818, _buf),
                        buffer_const(_start, _end, 1819, _buf),
                    )?;
                }
                if (_start >= 1820 && _start < 1824)
                    || (_end > 1820 && _end <= 1824)
                {
                    self.write_gpio_pin_cnfn(
                        7,
                        buffer_const(_start, _end, 1820, _buf),
                        buffer_const(_start, _end, 1821, _buf),
                        buffer_const(_start, _end, 1822, _buf),
                        buffer_const(_start, _end, 1823, _buf),
                    )?;
                }
                if (_start >= 1824 && _start < 1828)
                    || (_end > 1824 && _end <= 1828)
                {
                    self.write_gpio_pin_cnfn(
                        8,
                        buffer_const(_start, _end, 1824, _buf),
                        buffer_const(_start, _end, 1825, _buf),
                        buffer_const(_start, _end, 1826, _buf),
                        buffer_const(_start, _end, 1827, _buf),
                    )?;
                }
                if (_start >= 1828 && _start < 1832)
                    || (_end > 1828 && _end <= 1832)
                {
                    self.write_gpio_pin_cnfn(
                        9,
                        buffer_const(_start, _end, 1828, _buf),
                        buffer_const(_start, _end, 1829, _buf),
                        buffer_const(_start, _end, 1830, _buf),
                        buffer_const(_start, _end, 1831, _buf),
                    )?;
                }
                if (_start >= 1832 && _start < 1836)
                    || (_end > 1832 && _end <= 1836)
                {
                    self.write_gpio_pin_cnfn(
                        10,
                        buffer_const(_start, _end, 1832, _buf),
                        buffer_const(_start, _end, 1833, _buf),
                        buffer_const(_start, _end, 1834, _buf),
                        buffer_const(_start, _end, 1835, _buf),
                    )?;
                }
                if (_start >= 1836 && _start < 1840)
                    || (_end > 1836 && _end <= 1840)
                {
                    self.write_gpio_pin_cnfn(
                        11,
                        buffer_const(_start, _end, 1836, _buf),
                        buffer_const(_start, _end, 1837, _buf),
                        buffer_const(_start, _end, 1838, _buf),
                        buffer_const(_start, _end, 1839, _buf),
                    )?;
                }
                if (_start >= 1840 && _start < 1844)
                    || (_end > 1840 && _end <= 1844)
                {
                    self.write_gpio_pin_cnfn(
                        12,
                        buffer_const(_start, _end, 1840, _buf),
                        buffer_const(_start, _end, 1841, _buf),
                        buffer_const(_start, _end, 1842, _buf),
                        buffer_const(_start, _end, 1843, _buf),
                    )?;
                }
                if (_start >= 1844 && _start < 1848)
                    || (_end > 1844 && _end <= 1848)
                {
                    self.write_gpio_pin_cnfn(
                        13,
                        buffer_const(_start, _end, 1844, _buf),
                        buffer_const(_start, _end, 1845, _buf),
                        buffer_const(_start, _end, 1846, _buf),
                        buffer_const(_start, _end, 1847, _buf),
                    )?;
                }
                if (_start >= 1848 && _start < 1852)
                    || (_end > 1848 && _end <= 1852)
                {
                    self.write_gpio_pin_cnfn(
                        14,
                        buffer_const(_start, _end, 1848, _buf),
                        buffer_const(_start, _end, 1849, _buf),
                        buffer_const(_start, _end, 1850, _buf),
                        buffer_const(_start, _end, 1851, _buf),
                    )?;
                }
                if (_start >= 1852 && _start < 1856)
                    || (_end > 1852 && _end <= 1856)
                {
                    self.write_gpio_pin_cnfn(
                        15,
                        buffer_const(_start, _end, 1852, _buf),
                        buffer_const(_start, _end, 1853, _buf),
                        buffer_const(_start, _end, 1854, _buf),
                        buffer_const(_start, _end, 1855, _buf),
                    )?;
                }
                if (_start >= 1856 && _start < 1860)
                    || (_end > 1856 && _end <= 1860)
                {
                    self.write_gpio_pin_cnfn(
                        16,
                        buffer_const(_start, _end, 1856, _buf),
                        buffer_const(_start, _end, 1857, _buf),
                        buffer_const(_start, _end, 1858, _buf),
                        buffer_const(_start, _end, 1859, _buf),
                    )?;
                }
                if (_start >= 1860 && _start < 1864)
                    || (_end > 1860 && _end <= 1864)
                {
                    self.write_gpio_pin_cnfn(
                        17,
                        buffer_const(_start, _end, 1860, _buf),
                        buffer_const(_start, _end, 1861, _buf),
                        buffer_const(_start, _end, 1862, _buf),
                        buffer_const(_start, _end, 1863, _buf),
                    )?;
                }
                if (_start >= 1864 && _start < 1868)
                    || (_end > 1864 && _end <= 1868)
                {
                    self.write_gpio_pin_cnfn(
                        18,
                        buffer_const(_start, _end, 1864, _buf),
                        buffer_const(_start, _end, 1865, _buf),
                        buffer_const(_start, _end, 1866, _buf),
                        buffer_const(_start, _end, 1867, _buf),
                    )?;
                }
                if (_start >= 1868 && _start < 1872)
                    || (_end > 1868 && _end <= 1872)
                {
                    self.write_gpio_pin_cnfn(
                        19,
                        buffer_const(_start, _end, 1868, _buf),
                        buffer_const(_start, _end, 1869, _buf),
                        buffer_const(_start, _end, 1870, _buf),
                        buffer_const(_start, _end, 1871, _buf),
                    )?;
                }
                if (_start >= 1872 && _start < 1876)
                    || (_end > 1872 && _end <= 1876)
                {
                    self.write_gpio_pin_cnfn(
                        20,
                        buffer_const(_start, _end, 1872, _buf),
                        buffer_const(_start, _end, 1873, _buf),
                        buffer_const(_start, _end, 1874, _buf),
                        buffer_const(_start, _end, 1875, _buf),
                    )?;
                }
                if (_start >= 1876 && _start < 1880)
                    || (_end > 1876 && _end <= 1880)
                {
                    self.write_gpio_pin_cnfn(
                        21,
                        buffer_const(_start, _end, 1876, _buf),
                        buffer_const(_start, _end, 1877, _buf),
                        buffer_const(_start, _end, 1878, _buf),
                        buffer_const(_start, _end, 1879, _buf),
                    )?;
                }
                if (_start >= 1880 && _start < 1884)
                    || (_end > 1880 && _end <= 1884)
                {
                    self.write_gpio_pin_cnfn(
                        22,
                        buffer_const(_start, _end, 1880, _buf),
                        buffer_const(_start, _end, 1881, _buf),
                        buffer_const(_start, _end, 1882, _buf),
                        buffer_const(_start, _end, 1883, _buf),
                    )?;
                }
                if (_start >= 1884 && _start < 1888)
                    || (_end > 1884 && _end <= 1888)
                {
                    self.write_gpio_pin_cnfn(
                        23,
                        buffer_const(_start, _end, 1884, _buf),
                        buffer_const(_start, _end, 1885, _buf),
                        buffer_const(_start, _end, 1886, _buf),
                        buffer_const(_start, _end, 1887, _buf),
                    )?;
                }
                if (_start >= 1888 && _start < 1892)
                    || (_end > 1888 && _end <= 1892)
                {
                    self.write_gpio_pin_cnfn(
                        24,
                        buffer_const(_start, _end, 1888, _buf),
                        buffer_const(_start, _end, 1889, _buf),
                        buffer_const(_start, _end, 1890, _buf),
                        buffer_const(_start, _end, 1891, _buf),
                    )?;
                }
                if (_start >= 1892 && _start < 1896)
                    || (_end > 1892 && _end <= 1896)
                {
                    self.write_gpio_pin_cnfn(
                        25,
                        buffer_const(_start, _end, 1892, _buf),
                        buffer_const(_start, _end, 1893, _buf),
                        buffer_const(_start, _end, 1894, _buf),
                        buffer_const(_start, _end, 1895, _buf),
                    )?;
                }
                if (_start >= 1896 && _start < 1900)
                    || (_end > 1896 && _end <= 1900)
                {
                    self.write_gpio_pin_cnfn(
                        26,
                        buffer_const(_start, _end, 1896, _buf),
                        buffer_const(_start, _end, 1897, _buf),
                        buffer_const(_start, _end, 1898, _buf),
                        buffer_const(_start, _end, 1899, _buf),
                    )?;
                }
                if (_start >= 1900 && _start < 1904)
                    || (_end > 1900 && _end <= 1904)
                {
                    self.write_gpio_pin_cnfn(
                        27,
                        buffer_const(_start, _end, 1900, _buf),
                        buffer_const(_start, _end, 1901, _buf),
                        buffer_const(_start, _end, 1902, _buf),
                        buffer_const(_start, _end, 1903, _buf),
                    )?;
                }
                if (_start >= 1904 && _start < 1908)
                    || (_end > 1904 && _end <= 1908)
                {
                    self.write_gpio_pin_cnfn(
                        28,
                        buffer_const(_start, _end, 1904, _buf),
                        buffer_const(_start, _end, 1905, _buf),
                        buffer_const(_start, _end, 1906, _buf),
                        buffer_const(_start, _end, 1907, _buf),
                    )?;
                }
                if (_start >= 1908 && _start < 1912)
                    || (_end > 1908 && _end <= 1912)
                {
                    self.write_gpio_pin_cnfn(
                        29,
                        buffer_const(_start, _end, 1908, _buf),
                        buffer_const(_start, _end, 1909, _buf),
                        buffer_const(_start, _end, 1910, _buf),
                        buffer_const(_start, _end, 1911, _buf),
                    )?;
                }
                if (_start >= 1912 && _start < 1916)
                    || (_end > 1912 && _end <= 1916)
                {
                    self.write_gpio_pin_cnfn(
                        30,
                        buffer_const(_start, _end, 1912, _buf),
                        buffer_const(_start, _end, 1913, _buf),
                        buffer_const(_start, _end, 1914, _buf),
                        buffer_const(_start, _end, 1915, _buf),
                    )?;
                }
                if (_start >= 1916 && _start < 1920)
                    || (_end > 1916 && _end <= 1920)
                {
                    self.write_gpio_pin_cnfn(
                        31,
                        buffer_const(_start, _end, 1916, _buf),
                        buffer_const(_start, _end, 1917, _buf),
                        buffer_const(_start, _end, 1918, _buf),
                        buffer_const(_start, _end, 1919, _buf),
                    )?;
                }
            }
            _ => return Err(icicle_vm::cpu::mem::MemError::Unmapped),
        }
        Ok(())
    }
}
impl PeripheralPage0x50000000 {
    fn read_gpio_out(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_out_pin0()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_out_pin1()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_out_pin2()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_out_pin3()? << 3;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_out_pin4()? << 4;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_out_pin5()? << 5;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_out_pin6()? << 6;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_out_pin7()? << 7;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_out_pin8()? << 0;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_out_pin9()? << 1;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_out_pin10()? << 2;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_out_pin11()? << 3;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_out_pin12()? << 4;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_out_pin13()? << 5;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_out_pin14()? << 6;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_out_pin15()? << 7;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_out_pin16()? << 0;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_out_pin17()? << 1;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_out_pin18()? << 2;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_out_pin19()? << 3;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_out_pin20()? << 4;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_out_pin21()? << 5;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_out_pin22()? << 6;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_out_pin23()? << 7;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_out_pin24()? << 0;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_out_pin25()? << 1;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_out_pin26()? << 2;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_out_pin27()? << 3;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_out_pin28()? << 4;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_out_pin29()? << 5;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_out_pin30()? << 6;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_out_pin31()? << 7;
        }
        Ok(())
    }
    fn write_gpio_out(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_out_pin0((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_out_pin1((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_out_pin2((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_out_pin3((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_out_pin4((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_out_pin5((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_out_pin6((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_out_pin7((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_out_pin8((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_out_pin9((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_out_pin10((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_out_pin11((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_out_pin12((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_out_pin13((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_out_pin14((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_out_pin15((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_out_pin16((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_out_pin17((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_out_pin18((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_out_pin19((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_out_pin20((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_out_pin21((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_out_pin22((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_out_pin23((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_out_pin24((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_out_pin25((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_out_pin26((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_out_pin27((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_out_pin28((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_out_pin29((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_out_pin30((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_out_pin31((*byte >> 7) & 1)?;
        }
        Ok(())
    }
    fn read_gpio_outset(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_outset_pin0()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_outset_pin1()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_outset_pin2()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_outset_pin3()? << 3;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_outset_pin4()? << 4;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_outset_pin5()? << 5;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_outset_pin6()? << 6;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_outset_pin7()? << 7;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_outset_pin8()? << 0;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_outset_pin9()? << 1;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_outset_pin10()? << 2;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_outset_pin11()? << 3;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_outset_pin12()? << 4;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_outset_pin13()? << 5;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_outset_pin14()? << 6;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_outset_pin15()? << 7;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_outset_pin16()? << 0;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_outset_pin17()? << 1;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_outset_pin18()? << 2;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_outset_pin19()? << 3;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_outset_pin20()? << 4;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_outset_pin21()? << 5;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_outset_pin22()? << 6;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_outset_pin23()? << 7;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_outset_pin24()? << 0;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_outset_pin25()? << 1;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_outset_pin26()? << 2;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_outset_pin27()? << 3;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_outset_pin28()? << 4;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_outset_pin29()? << 5;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_outset_pin30()? << 6;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_outset_pin31()? << 7;
        }
        Ok(())
    }
    fn write_gpio_outset(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outset_pin0((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outset_pin1((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outset_pin2((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outset_pin3((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outset_pin4((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outset_pin5((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outset_pin6((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outset_pin7((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outset_pin8((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outset_pin9((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outset_pin10((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outset_pin11((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outset_pin12((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outset_pin13((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outset_pin14((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outset_pin15((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outset_pin16((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outset_pin17((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outset_pin18((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outset_pin19((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outset_pin20((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outset_pin21((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outset_pin22((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outset_pin23((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outset_pin24((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outset_pin25((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outset_pin26((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outset_pin27((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outset_pin28((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outset_pin29((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outset_pin30((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outset_pin31((*byte >> 7) & 1)?;
        }
        Ok(())
    }
    fn read_gpio_outclr(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_outclr_pin0()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_outclr_pin1()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_outclr_pin2()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_outclr_pin3()? << 3;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_outclr_pin4()? << 4;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_outclr_pin5()? << 5;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_outclr_pin6()? << 6;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_outclr_pin7()? << 7;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_outclr_pin8()? << 0;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_outclr_pin9()? << 1;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_outclr_pin10()? << 2;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_outclr_pin11()? << 3;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_outclr_pin12()? << 4;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_outclr_pin13()? << 5;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_outclr_pin14()? << 6;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_outclr_pin15()? << 7;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_outclr_pin16()? << 0;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_outclr_pin17()? << 1;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_outclr_pin18()? << 2;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_outclr_pin19()? << 3;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_outclr_pin20()? << 4;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_outclr_pin21()? << 5;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_outclr_pin22()? << 6;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_outclr_pin23()? << 7;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_outclr_pin24()? << 0;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_outclr_pin25()? << 1;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_outclr_pin26()? << 2;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_outclr_pin27()? << 3;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_outclr_pin28()? << 4;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_outclr_pin29()? << 5;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_outclr_pin30()? << 6;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_outclr_pin31()? << 7;
        }
        Ok(())
    }
    fn write_gpio_outclr(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outclr_pin0((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outclr_pin1((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outclr_pin2((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outclr_pin3((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outclr_pin4((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outclr_pin5((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outclr_pin6((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outclr_pin7((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outclr_pin8((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outclr_pin9((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outclr_pin10((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outclr_pin11((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outclr_pin12((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outclr_pin13((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outclr_pin14((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outclr_pin15((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outclr_pin16((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outclr_pin17((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outclr_pin18((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outclr_pin19((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outclr_pin20((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outclr_pin21((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outclr_pin22((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outclr_pin23((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outclr_pin24((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outclr_pin25((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outclr_pin26((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outclr_pin27((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outclr_pin28((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outclr_pin29((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outclr_pin30((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_outclr_pin31((*byte >> 7) & 1)?;
        }
        Ok(())
    }
    fn read_gpio_in(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_in_pin0()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_in_pin1()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_in_pin2()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_in_pin3()? << 3;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_in_pin4()? << 4;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_in_pin5()? << 5;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_in_pin6()? << 6;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_in_pin7()? << 7;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_in_pin8()? << 0;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_in_pin9()? << 1;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_in_pin10()? << 2;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_in_pin11()? << 3;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_in_pin12()? << 4;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_in_pin13()? << 5;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_in_pin14()? << 6;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_in_pin15()? << 7;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_in_pin16()? << 0;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_in_pin17()? << 1;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_in_pin18()? << 2;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_in_pin19()? << 3;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_in_pin20()? << 4;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_in_pin21()? << 5;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_in_pin22()? << 6;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_in_pin23()? << 7;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_in_pin24()? << 0;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_in_pin25()? << 1;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_in_pin26()? << 2;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_in_pin27()? << 3;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_in_pin28()? << 4;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_in_pin29()? << 5;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_in_pin30()? << 6;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_in_pin31()? << 7;
        }
        Ok(())
    }
    fn read_gpio_dir(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_dir_pin0()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_dir_pin1()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_dir_pin2()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_dir_pin3()? << 3;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_dir_pin4()? << 4;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_dir_pin5()? << 5;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_dir_pin6()? << 6;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_dir_pin7()? << 7;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_dir_pin8()? << 0;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_dir_pin9()? << 1;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_dir_pin10()? << 2;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_dir_pin11()? << 3;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_dir_pin12()? << 4;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_dir_pin13()? << 5;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_dir_pin14()? << 6;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_dir_pin15()? << 7;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_dir_pin16()? << 0;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_dir_pin17()? << 1;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_dir_pin18()? << 2;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_dir_pin19()? << 3;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_dir_pin20()? << 4;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_dir_pin21()? << 5;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_dir_pin22()? << 6;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_dir_pin23()? << 7;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_dir_pin24()? << 0;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_dir_pin25()? << 1;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_dir_pin26()? << 2;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_dir_pin27()? << 3;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_dir_pin28()? << 4;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_dir_pin29()? << 5;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_dir_pin30()? << 6;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_dir_pin31()? << 7;
        }
        Ok(())
    }
    fn write_gpio_dir(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dir_pin0((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dir_pin1((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dir_pin2((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dir_pin3((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dir_pin4((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dir_pin5((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dir_pin6((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dir_pin7((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dir_pin8((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dir_pin9((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dir_pin10((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dir_pin11((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dir_pin12((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dir_pin13((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dir_pin14((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dir_pin15((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dir_pin16((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dir_pin17((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dir_pin18((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dir_pin19((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dir_pin20((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dir_pin21((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dir_pin22((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dir_pin23((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dir_pin24((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dir_pin25((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dir_pin26((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dir_pin27((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dir_pin28((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dir_pin29((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dir_pin30((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dir_pin31((*byte >> 7) & 1)?;
        }
        Ok(())
    }
    fn read_gpio_dirset(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_dirset_pin0()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_dirset_pin1()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_dirset_pin2()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_dirset_pin3()? << 3;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_dirset_pin4()? << 4;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_dirset_pin5()? << 5;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_dirset_pin6()? << 6;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_dirset_pin7()? << 7;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_dirset_pin8()? << 0;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_dirset_pin9()? << 1;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_dirset_pin10()? << 2;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_dirset_pin11()? << 3;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_dirset_pin12()? << 4;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_dirset_pin13()? << 5;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_dirset_pin14()? << 6;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_dirset_pin15()? << 7;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_dirset_pin16()? << 0;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_dirset_pin17()? << 1;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_dirset_pin18()? << 2;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_dirset_pin19()? << 3;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_dirset_pin20()? << 4;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_dirset_pin21()? << 5;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_dirset_pin22()? << 6;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_dirset_pin23()? << 7;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_dirset_pin24()? << 0;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_dirset_pin25()? << 1;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_dirset_pin26()? << 2;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_dirset_pin27()? << 3;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_dirset_pin28()? << 4;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_dirset_pin29()? << 5;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_dirset_pin30()? << 6;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_dirset_pin31()? << 7;
        }
        Ok(())
    }
    fn write_gpio_dirset(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirset_pin0((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirset_pin1((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirset_pin2((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirset_pin3((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirset_pin4((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirset_pin5((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirset_pin6((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirset_pin7((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirset_pin8((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirset_pin9((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirset_pin10((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirset_pin11((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirset_pin12((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirset_pin13((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirset_pin14((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirset_pin15((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirset_pin16((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirset_pin17((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirset_pin18((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirset_pin19((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirset_pin20((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirset_pin21((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirset_pin22((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirset_pin23((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirset_pin24((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirset_pin25((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirset_pin26((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirset_pin27((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirset_pin28((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirset_pin29((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirset_pin30((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirset_pin31((*byte >> 7) & 1)?;
        }
        Ok(())
    }
    fn read_gpio_dirclr(
        &self,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_dirclr_pin0()? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_dirclr_pin1()? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_dirclr_pin2()? << 2;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_dirclr_pin3()? << 3;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_dirclr_pin4()? << 4;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_dirclr_pin5()? << 5;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_dirclr_pin6()? << 6;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_dirclr_pin7()? << 7;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_dirclr_pin8()? << 0;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_dirclr_pin9()? << 1;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_dirclr_pin10()? << 2;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_dirclr_pin11()? << 3;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_dirclr_pin12()? << 4;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_dirclr_pin13()? << 5;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_dirclr_pin14()? << 6;
        }
        if let Some(byte) = _byte_1 {
            **byte |= self.0.lock().unwrap().read_gpio_dirclr_pin15()? << 7;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_dirclr_pin16()? << 0;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_dirclr_pin17()? << 1;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_dirclr_pin18()? << 2;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_dirclr_pin19()? << 3;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_dirclr_pin20()? << 4;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_dirclr_pin21()? << 5;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_dirclr_pin22()? << 6;
        }
        if let Some(byte) = _byte_2 {
            **byte |= self.0.lock().unwrap().read_gpio_dirclr_pin23()? << 7;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_dirclr_pin24()? << 0;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_dirclr_pin25()? << 1;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_dirclr_pin26()? << 2;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_dirclr_pin27()? << 3;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_dirclr_pin28()? << 4;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_dirclr_pin29()? << 5;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_dirclr_pin30()? << 6;
        }
        if let Some(byte) = _byte_3 {
            **byte |= self.0.lock().unwrap().read_gpio_dirclr_pin31()? << 7;
        }
        Ok(())
    }
    fn write_gpio_dirclr(
        &self,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirclr_pin0((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirclr_pin1((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirclr_pin2((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirclr_pin3((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirclr_pin4((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirclr_pin5((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirclr_pin6((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirclr_pin7((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirclr_pin8((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirclr_pin9((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirclr_pin10((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirclr_pin11((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirclr_pin12((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirclr_pin13((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirclr_pin14((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirclr_pin15((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirclr_pin16((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirclr_pin17((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirclr_pin18((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirclr_pin19((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirclr_pin20((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirclr_pin21((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirclr_pin22((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirclr_pin23((*byte >> 7) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirclr_pin24((*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirclr_pin25((*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirclr_pin26((*byte >> 2) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirclr_pin27((*byte >> 3) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirclr_pin28((*byte >> 4) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirclr_pin29((*byte >> 5) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirclr_pin30((*byte >> 6) & 1)?;
        }
        if let Some(byte) = _byte_3 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_dirclr_pin31((*byte >> 7) & 1)?;
        }
        Ok(())
    }
    fn read_gpio_pin_cnfn(
        &self,
        _dim: usize,
        _byte_0: &mut Option<&mut u8>,
        _byte_1: &mut Option<&mut u8>,
        _byte_2: &mut Option<&mut u8>,
        _byte_3: &mut Option<&mut u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(_byte) = _byte_0 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_1 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_2 {
            **_byte = 0;
        }
        if let Some(_byte) = _byte_3 {
            **_byte = 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |= self.0.lock().unwrap().read_gpio_pin_cnfn_dir(_dim)? << 0;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_gpio_pin_cnfn_input(_dim)? << 1;
        }
        if let Some(byte) = _byte_0 {
            **byte |=
                self.0.lock().unwrap().read_gpio_pin_cnfn_pull(_dim)? << 2;
        }
        if let Some(byte) = _byte_1 {
            **byte |=
                self.0.lock().unwrap().read_gpio_pin_cnfn_drive(_dim)? << 0;
        }
        if let Some(byte) = _byte_2 {
            **byte |=
                self.0.lock().unwrap().read_gpio_pin_cnfn_sense(_dim)? << 0;
        }
        Ok(())
    }
    fn write_gpio_pin_cnfn(
        &self,
        _dim: usize,
        _byte_0: Option<&u8>,
        _byte_1: Option<&u8>,
        _byte_2: Option<&u8>,
        _byte_3: Option<&u8>,
    ) -> icicle_vm::cpu::mem::MemResult<()> {
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_pin_cnfn_dir(_dim, (*byte >> 0) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_pin_cnfn_input(_dim, (*byte >> 1) & 1)?;
        }
        if let Some(byte) = _byte_0 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_pin_cnfn_pull(_dim, (*byte >> 2) & 3)?;
        }
        if let Some(byte) = _byte_1 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_pin_cnfn_drive(_dim, (*byte >> 0) & 7)?;
        }
        if let Some(byte) = _byte_2 {
            self.0
                .lock()
                .unwrap()
                .write_gpio_pin_cnfn_sense(_dim, (*byte >> 0) & 3)?;
        }
        Ok(())
    }
}
