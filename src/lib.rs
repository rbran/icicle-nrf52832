mod pages;
pub mod peripheral;
pub fn map_cpu(
    _pe: &std::sync::Arc<std::sync::Mutex<peripheral::Peripherals>>,
    _cpu: &mut icicle_vm::cpu::Cpu,
) {
    let io = _cpu
        .mem
        .register_io_handler(pages::PeripheralPage0x10000000(
            std::sync::Arc::clone(_pe),
        ));
    _cpu.mem.map_memory_len(268435456, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::PeripheralPage0x10001000(
            std::sync::Arc::clone(_pe),
        ));
    _cpu.mem.map_memory_len(268439552, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::PeripheralPage0x40000000(
            std::sync::Arc::clone(_pe),
        ));
    _cpu.mem.map_memory_len(1073741824, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::PeripheralPage0x40001000(
            std::sync::Arc::clone(_pe),
        ));
    _cpu.mem.map_memory_len(1073745920, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::PeripheralPage0x40002000(
            std::sync::Arc::clone(_pe),
        ));
    _cpu.mem.map_memory_len(1073750016, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::PeripheralPage0x40003000(
            std::sync::Arc::clone(_pe),
        ));
    _cpu.mem.map_memory_len(1073754112, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::PeripheralPage0x40004000(
            std::sync::Arc::clone(_pe),
        ));
    _cpu.mem.map_memory_len(1073758208, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::PeripheralPage0x40006000(
            std::sync::Arc::clone(_pe),
        ));
    _cpu.mem.map_memory_len(1073766400, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::PeripheralPage0x40007000(
            std::sync::Arc::clone(_pe),
        ));
    _cpu.mem.map_memory_len(1073770496, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::PeripheralPage0x40008000(
            std::sync::Arc::clone(_pe),
        ));
    _cpu.mem.map_memory_len(1073774592, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::PeripheralPage0x4000B000(
            std::sync::Arc::clone(_pe),
        ));
    _cpu.mem.map_memory_len(1073786880, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::PeripheralPage0x4000C000(
            std::sync::Arc::clone(_pe),
        ));
    _cpu.mem.map_memory_len(1073790976, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::PeripheralPage0x4000D000(
            std::sync::Arc::clone(_pe),
        ));
    _cpu.mem.map_memory_len(1073795072, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::PeripheralPage0x4000E000(
            std::sync::Arc::clone(_pe),
        ));
    _cpu.mem.map_memory_len(1073799168, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::PeripheralPage0x4000F000(
            std::sync::Arc::clone(_pe),
        ));
    _cpu.mem.map_memory_len(1073803264, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::PeripheralPage0x40010000(
            std::sync::Arc::clone(_pe),
        ));
    _cpu.mem.map_memory_len(1073807360, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::PeripheralPage0x40012000(
            std::sync::Arc::clone(_pe),
        ));
    _cpu.mem.map_memory_len(1073815552, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::PeripheralPage0x40013000(
            std::sync::Arc::clone(_pe),
        ));
    _cpu.mem.map_memory_len(1073819648, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::PeripheralPage0x40014000(
            std::sync::Arc::clone(_pe),
        ));
    _cpu.mem.map_memory_len(1073823744, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::PeripheralPage0x4001E000(
            std::sync::Arc::clone(_pe),
        ));
    _cpu.mem.map_memory_len(1073864704, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::PeripheralPage0x4001F000(
            std::sync::Arc::clone(_pe),
        ));
    _cpu.mem.map_memory_len(1073868800, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::PeripheralPage0x50000000(
            std::sync::Arc::clone(_pe),
        ));
    _cpu.mem.map_memory_len(1342177280, 4096u64, io);
    let io = _cpu
        .mem
        .register_io_handler(pages::PeripheralPage0xE000E000(
            std::sync::Arc::clone(_pe),
        ));
    _cpu.mem.map_memory_len(3758153728, 4096u64, io);
}
