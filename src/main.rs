#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use crate::{helpers::refresh_data_info, monitor::SystemMonitor};
use eframe::egui::{self};
use std::{sync::mpsc, thread};
mod helpers;
mod monitor;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    let (tx_data, rx_data) = mpsc::channel();
    let (tx_command, rx_command) = mpsc::channel();

    thread::spawn(move || refresh_data_info(tx_data, rx_command));

    eframe::run_native(
        "RSM",
        options,
        Box::new(|_cc| Ok(Box::new(SystemMonitor::new(rx_data, tx_command)))),
    )
}
