use sysinfo::System;

pub fn get_ram(system: &mut System) -> (u64, u64) {

    let used = system.used_memory() / 1024 / 1024;
    let total = system.total_memory() / 1024 / 1024;

    (used, total)
}
