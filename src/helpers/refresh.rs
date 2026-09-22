use std::{
    sync::mpsc::{Receiver, Sender},
    thread,
    time::Duration,
};

use sysinfo::{MemoryRefreshKind, ProcessesToUpdate, System};

use crate::monitor::{Commands, ProcessInfo, SystemData};

pub fn refresh_data_info(tx_data: Sender<SystemData>, rx_command: Receiver<Commands>) {
    let mut sys = System::new();

    loop {
        thread::sleep(Duration::from_secs(1));

        if let Ok(action) = rx_command.try_recv() {
            match action {
                Commands::KillProcess(process_id) => {
                    if let Some(process) = sys.process(process_id) {
                        if process.kill() {
                            println!("Proceso acabado!");
                        } else {
                            eprintln!("Proceso no acabado. PID: {}", process_id);
                        }
                    }
                }
            }
        }

        sys.refresh_cpu_usage();
        sys.refresh_memory_specifics(MemoryRefreshKind::nothing().with_ram());
        sys.refresh_processes(ProcessesToUpdate::All, true);

        let mut processes: Vec<ProcessInfo> = sys
            .processes()
            .values()
            .map(|process| ProcessInfo {
                pid: process.pid(),
                name: process.name().to_str().unwrap_or("No_Name").to_string(),
                cpu_usage: process.cpu_usage(),
                memory_usage: process.memory(),
            })
            .collect();

        processes.sort_by(|a, b| b.memory_usage.partial_cmp(&a.memory_usage).unwrap());
        processes.truncate(50);

        let data = SystemData {
            cpu_usage: sys.global_cpu_usage(),
            ram_usage: (sys.used_memory() as f32 / sys.total_memory() as f32) * 100.0,
            processes,
        };

        if tx_data.send(data).is_err() {
            break;
        }
    }
}
