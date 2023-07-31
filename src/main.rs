use std::sync::{Arc, Mutex};

use icicle_nrf51::{map_cpu, peripheral::Peripherals};

fn main() {
    let cpu_config =
        icicle_vm::cpu::Config::from_target_triple("i586-linux-musl");
    let mut vm = icicle_vm::build(&cpu_config).unwrap();

    vm.env = icicle_vm::env::build_auto(&mut vm).unwrap();

    let peripheral = Arc::new(Mutex::new(Peripherals::default()));
    map_cpu(&peripheral, &mut vm.cpu);
    todo!();
}
