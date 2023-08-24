mod pages;
#[doc = "All peripheral related data is contained here"]
pub mod peripheral;
#[doc = "Map Peripherals to Icicle VM"]
pub fn map_cpu(
    _pe: &std::sync::Arc<std::sync::Mutex<peripheral::Peripherals>>,
    _cpu: &mut icicle_vm::cpu::Cpu,
) {
    let io = _cpu
        .mem
        .register_io_handler(pages::Ficr(std::sync::Arc::clone(_pe)));
    _cpu.mem.map_memory_len(268435456, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::Uicr(std::sync::Arc::clone(_pe)));
    _cpu.mem.map_memory_len(268439552, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::Apb0(std::sync::Arc::clone(_pe)));
    _cpu.mem.map_memory_len(1073741824, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::Radio(std::sync::Arc::clone(_pe)));
    _cpu.mem.map_memory_len(1073745920, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::Apb2(std::sync::Arc::clone(_pe)));
    _cpu.mem.map_memory_len(1073750016, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::Apb3(std::sync::Arc::clone(_pe)));
    _cpu.mem.map_memory_len(1073754112, 4096u64, io);
    _cpu.mem.map_memory_len(1073758208, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::Nfct(std::sync::Arc::clone(_pe)));
    _cpu.mem.map_memory_len(1073762304, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::Gpiote(std::sync::Arc::clone(_pe)));
    _cpu.mem.map_memory_len(1073766400, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::Saadc(std::sync::Arc::clone(_pe)));
    _cpu.mem.map_memory_len(1073770496, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::Timer0(std::sync::Arc::clone(_pe)));
    _cpu.mem.map_memory_len(1073774592, 4096u64, io);
    _cpu.mem.map_memory_len(1073778688, 4096u64, io);
    _cpu.mem.map_memory_len(1073782784, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::Rtc0(std::sync::Arc::clone(_pe)));
    _cpu.mem.map_memory_len(1073786880, 4096u64, io);
    _cpu.mem.map_memory_len(1073811456, 4096u64, io);
    _cpu.mem.map_memory_len(1073889280, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::Temp(std::sync::Arc::clone(_pe)));
    _cpu.mem.map_memory_len(1073790976, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::Rng(std::sync::Arc::clone(_pe)));
    _cpu.mem.map_memory_len(1073795072, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::Ecb(std::sync::Arc::clone(_pe)));
    _cpu.mem.map_memory_len(1073799168, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::Apb15(std::sync::Arc::clone(_pe)));
    _cpu.mem.map_memory_len(1073803264, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::Wdt(std::sync::Arc::clone(_pe)));
    _cpu.mem.map_memory_len(1073807360, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::Qdec(std::sync::Arc::clone(_pe)));
    _cpu.mem.map_memory_len(1073815552, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::Apb19(std::sync::Arc::clone(_pe)));
    _cpu.mem.map_memory_len(1073819648, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::Apb20(std::sync::Arc::clone(_pe)));
    _cpu.mem.map_memory_len(1073823744, 4096u64, io);
    _cpu.mem.map_memory_len(1073827840, 4096u64, io);
    _cpu.mem.map_memory_len(1073831936, 4096u64, io);
    _cpu.mem.map_memory_len(1073836032, 4096u64, io);
    _cpu.mem.map_memory_len(1073840128, 4096u64, io);
    _cpu.mem.map_memory_len(1073844224, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::Timer3(std::sync::Arc::clone(_pe)));
    _cpu.mem.map_memory_len(1073848320, 4096u64, io);
    _cpu.mem.map_memory_len(1073852416, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::Pwm0(std::sync::Arc::clone(_pe)));
    _cpu.mem.map_memory_len(1073856512, 4096u64, io);
    _cpu.mem.map_memory_len(1073876992, 4096u64, io);
    _cpu.mem.map_memory_len(1073881088, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::Pdm(std::sync::Arc::clone(_pe)));
    _cpu.mem.map_memory_len(1073860608, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::Nvmc(std::sync::Arc::clone(_pe)));
    _cpu.mem.map_memory_len(1073864704, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::Ppi(std::sync::Arc::clone(_pe)));
    _cpu.mem.map_memory_len(1073868800, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::Mwu(std::sync::Arc::clone(_pe)));
    _cpu.mem.map_memory_len(1073872896, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::I2s(std::sync::Arc::clone(_pe)));
    _cpu.mem.map_memory_len(1073893376, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::Fpu(std::sync::Arc::clone(_pe)));
    _cpu.mem.map_memory_len(1073897472, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::P0(std::sync::Arc::clone(_pe)));
    _cpu.mem.map_memory_len(1342177280, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::Dwt(std::sync::Arc::clone(_pe)));
    _cpu.mem.map_memory_len(3758100480, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::Scs(std::sync::Arc::clone(_pe)));
    _cpu.mem.map_memory_len(3758153728, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::Apb35(std::sync::Arc::clone(_pe)));
    _cpu.mem.map_memory_len(1073885184, 4096u64, io);
}
