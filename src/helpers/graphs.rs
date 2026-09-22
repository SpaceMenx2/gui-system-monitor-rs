use std::collections::VecDeque;

use eframe::egui::{RichText, Stroke};
use egui_plot::{Line, Plot, PlotPoints};

const KB: f64 = 1024.0;
const MB: f64 = KB * 1024.0;
const GB: f64 = MB * 1024.0;

pub struct GraphCreation {
    pub id: &'static str,
    pub x: RichText,
    pub y: RichText,
    pub x_bounds: [f64; 2],
    pub y_bounds: [f64; 2],
    pub height: f32,
    pub width: f32,
}

pub fn create_graph(graph_info: GraphCreation) -> Plot<'static> {
    Plot::new(graph_info.id)
        .height(graph_info.height)
        .width(graph_info.width)
        .view_aspect(2.0)
        .allow_zoom(false)
        .allow_drag(false)
        .allow_scroll(false)
        .allow_boxed_zoom(false)
        .allow_axis_zoom_drag(false)
        .allow_double_click_reset(false)
        .x_axis_label(graph_info.x)
        .y_axis_label(graph_info.y)
        .default_x_bounds(
            graph_info.x_bounds.get(0).unwrap_or(&0.0).to_owned(),
            graph_info.x_bounds.get(1).unwrap_or(&100.0).to_owned(),
        )
        .default_y_bounds(
            graph_info.y_bounds.get(0).unwrap_or(&0.0).to_owned(),
            graph_info.y_bounds.get(1).unwrap_or(&100.0).to_owned(),
        )
}

pub fn create_line(
    history: &VecDeque<f32>,
    line_name: &'static str,
    stroke: Stroke,
) -> Line<'static> {
    let points: Vec<[f64; 2]> = history
        .iter()
        .enumerate()
        .map(|(i, &val)| [i as f64, val as f64])
        .collect();

    Line::new(line_name, PlotPoints::from(points))
        .stroke(stroke)
        .fill(0.0)
        .fill_alpha(0.2)
}

pub fn format_bytes(bytes: f64) -> String {
    if bytes <= KB {
        format!("{}B", bytes)
    } else if bytes <= MB {
        format!("{:.1}KB", bytes / KB)
    } else if bytes <= GB {
        format!("{:.1}MB", bytes / MB)
    } else {
        format!("{:.1}GB", bytes / GB)
    }
}
