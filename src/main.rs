use std::sync::{Arc, Mutex};

use icicle_nrf52832::{map_cpu, peripheral::Peripherals};

fn main() {
    let cpu_config =
        icicle_vm::cpu::Config::from_target_triple("thumbv7m-unknown-unknown");
    let mut vm = icicle_vm::build(&cpu_config).unwrap();
    vm.env = icicle_vm::env::build_auto(&mut vm).unwrap();

    let peripheral = Arc::new(Mutex::new(Peripherals::default()));
    map_cpu(&peripheral, &mut vm.cpu);

    // Load binary and run the vm...
    todo!();
}
