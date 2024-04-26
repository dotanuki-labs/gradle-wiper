// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use crate::core::models::MemoryCached;
use crate::core::ram::jps::find_jvm_processes;
use crate::core::ram::memory_type_from_jvm_launcher_class;
use log::debug;
use std::path::PathBuf;
use sysinfo::Pid;

pub fn cleanup_memory(hsperfdata_locator: fn() -> PathBuf, caches: &[MemoryCached]) {
    debug!("Cleaning up JVM processes");
    debug!("");

    let jvm_processes = find_jvm_processes(hsperfdata_locator).unwrap_or_default();
    let mut system = sysinfo::System::new_all();
    system.refresh_all();

    jvm_processes.into_iter().for_each(|(pid, launcher_class_name)| {
        let cache_type = memory_type_from_jvm_launcher_class(&launcher_class_name);
        if caches.contains(&cache_type) {
            if let Some(process) = system.process(Pid::from_u32(pid)) {
                if process.kill() {
                    debug!("Killed : {} ({})", &launcher_class_name, pid);
                }
            }
        }
    })
}
