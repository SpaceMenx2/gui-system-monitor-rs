// 1. Aquí definiremos el estado de nuestra aplicación.
// Por ahora, estará vacío, pero aquí es donde luego guardaremos los datos de sysinfo.

use std::{
    collections::VecDeque,
    sync::mpsc::{self, Receiver, Sender},
};

use eframe::egui::{self, Color32, RichText, Stroke};
use egui_extras::{Column, TableBuilder};
use sysinfo::Pid;

use crate::helpers::{GraphCreation, create_graph, create_line, format_bytes};

#[derive(Debug)]
pub struct SystemData {
    pub cpu_usage: f32,
    pub ram_usage: f32,
    pub processes: Vec<ProcessInfo>,
}

#[derive(Debug)]
pub struct ProcessInfo {
    pub pid: Pid,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_usage: u64,
}

pub enum Commands {
    KillProcess(Pid),
}

#[derive(Debug)]
pub struct SystemMonitor {
    // TODO: Aquí irán los datos de CPU, RAM, etc.
    pub data: SystemData,
    pub rx_data: Receiver<SystemData>,
    pub cpu_history: VecDeque<f32>,
    pub ram_history: VecDeque<f32>,
    pub tx_command: Sender<Commands>,
}

impl SystemMonitor {
    pub fn new(rx_data: mpsc::Receiver<SystemData>, tx_command: Sender<Commands>) -> Self {
        Self {
            data: SystemData {
                cpu_usage: 0.0,
                ram_usage: 0.0,
                processes: vec![],
            },
            rx_data,
            tx_command,
            cpu_history: VecDeque::new(),
            ram_history: VecDeque::new(),
        }
    }

    pub fn check_for_updates(&mut self) {
        if let Ok(data) = self.rx_data.try_recv() {
            self.data = data;
            self.register_historial();
        }
    }

    fn register_historial(&mut self) {
        self.cpu_history.push_back(self.data.cpu_usage);
        if self.cpu_history.len() > 61 {
            self.cpu_history.pop_front();
        }

        // RAM
        self.ram_history.push_back(self.data.ram_usage);
        if self.ram_history.len() > 61 {
            self.ram_history.pop_front();
        }
    }
}

impl eframe::App for SystemMonitor {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.check_for_updates();

        let height = (ui.available_height() - 200.0) / 3.0; // 300px
        let width = ui.available_width() * 0.85;
        let font_size = (height / 8.0).clamp(12.0, 16.0);

        let (x_text, y_text) = (
            RichText::new("Tiempo (segs)").size(font_size),
            RichText::new("Consumo (%)").size(font_size),
        );

        egui::CentralPanel::default().show(ui, |ui| {
            /* Plots */
            let ram_plot = create_graph(GraphCreation {
                id: "ram_plot",
                x: x_text.clone(),
                y: y_text.clone(),
                x_bounds: [0.0, 60.00001],
                y_bounds: [0.0, 100.00001],
                width,
                height,
            });
            let cpu_plot = create_graph(GraphCreation {
                id: "cpu_plot",
                x: x_text,
                y: y_text,
                x_bounds: [0.0, 60.00001],
                y_bounds: [0.0, 100.00001],
                width,
                height,
            });
            /* Plots */

            ui.heading("🦀 Monitor de Sistema en Rust");
            ui.add_space(10.0);
            ui.label("Uso del CPU:");
            ui.add_space(5.0);
            cpu_plot.show(ui, |plot_ui| {
                let line = create_line(
                    &self.cpu_history,
                    "cpu_line",
                    Stroke::new(1.0, Color32::from_rgb(255, 0, 0)),
                );
                plot_ui.line(line);
            });
            ui.add_space(15.0);
            ui.label("Consumo de RAM");
            ui.add_space(5.0);
            ram_plot.show(ui, |plot_ui| {
                let line = create_line(
                    &self.ram_history,
                    "ram_line",
                    Stroke::new(1.0, Color32::from_rgb(255, 200, 100)),
                );
                plot_ui.line(line);
            });
            TableBuilder::new(ui)
                .column(Column::exact(100.0)) // PID
                .column(Column::initial(250.0).resizable(true)) // Nombre
                .column(Column::exact(80.0)) // CPU
                .column(Column::exact(100.0)) // Memoria
                .column(Column::exact(120.0)) // Matar proceso
                .striped(true)
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.strong("PID");
                    });
                    header.col(|ui| {
                        ui.strong("Nombre");
                    });
                    header.col(|ui| {
                        ui.strong("CPU");
                    });
                    header.col(|ui| {
                        ui.strong("Memoria");
                    });
                    header.col(|ui| {
                        ui.strong("Matar proceso");
                    });
                })
                .body(|mut body| {
                    for process in &self.data.processes {
                        body.row(18.0, |mut row| {
                            row.col(|ui| {
                                ui.label(&process.pid.to_string());
                            });
                            row.col(|ui| {
                                ui.label(&process.name);
                            });
                            row.col(|ui| {
                                ui.label(format!("{:.1}", process.cpu_usage));
                            });
                            row.col(|ui| {
                                ui.label(format_bytes(process.memory_usage as f64));
                            });
                            row.col(|ui| {
                                if ui.button("Matar proceso").clicked() {
                                    let _ =
                                        self.tx_command.send(Commands::KillProcess(process.pid));
                                }
                            });
                        });
                    }
                });

            ui.ctx().request_repaint_after_secs(1.0);
        });
    }
}
