#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod mc;
use egui::Layout;
use mc::*;
use eframe::{egui, IconData};

fn main() {

    let image = image::load_from_memory(include_bytes!("moth.png")).unwrap();

    let options = eframe::NativeOptions {
        icon_data: Some(IconData {
            rgba: image.into_rgba8().into_raw(),
            height: 512,
            width: 512,
        }),
        initial_window_size: Some(egui::vec2(312.0, 155.0)),
        resizable: false,
        centered: true,
        ..Default::default()
    };
    eframe::run_native(
        "acutemoth",
        options,
        Box::new(|_cc| Box::new(App::default())),
    )
}

#[derive(PartialEq)]
enum SelectedTable {
    Vanilla,
    OldFast,
    NewFast,
}

struct App {
    text_angle: String,
    angle: f32,
    live: bool,
    selected_table: SelectedTable,
    vanilla_table: [f32; 65536],
    old_fast_table: [f32; 4096],
    new_fast_table: [f32; 4096],
}
impl Default for App {
    fn default() -> Self {
        Self {
            text_angle: "0".to_owned(),
            angle: 0.0,
            live: true,
            selected_table: SelectedTable:: Vanilla,
            vanilla_table: vanilla_table(),
            old_fast_table: old_fast_table(),
            new_fast_table: new_fast_table(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.allocate_ui_with_layout(
                egui::vec2(frame.info().window_info.size.x - 16.0, 0.0),
                Layout::right_to_left(egui::Align::Center),
                |ui|
            {
                let go_button = ui.add_sized([29.844, 0.0], egui::Button::new("Go"));
                if go_button.clicked() || self.live {
                    self.angle = self.text_angle.parse().unwrap_or(self.angle);
                }
                ui.text_edit_singleline(&mut self.text_angle);
            });

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.selected_table, SelectedTable::Vanilla, "Vanilla");
                ui.selectable_value(&mut self.selected_table, SelectedTable::OldFast, "Old Optifine");
                ui.selectable_value(&mut self.selected_table, SelectedTable::NewFast, "New Optifine");
                let remaining = frame.info().window_info.size.x - ui.cursor().min.x;
                ui.add_space(remaining - 38.0);
                ui.toggle_value(&mut self.live, "Live").rect.width();
            });
            
            ui.separator();

            egui::Grid::new("main").striped(true).start_row(2).show(ui, |ui| {

                let radians = mc_radians(self.angle);

                let sin_index;
                let cos_index;
                let cos_index_adj;
                let sin_value;
                let cos_value;
                match self.selected_table {
                    SelectedTable::Vanilla => {
                        sin_index = vanilla_sin_index(radians);
                        cos_index = vanilla_cos_index(radians);
                        sin_value = self.vanilla_table[sin_index];
                        cos_value = self.vanilla_table[cos_index];
                        cos_index_adj = (cos_index as isize - 16384).rem_euclid(65536);
                    },
                    SelectedTable::OldFast => {
                        sin_index = old_fast_sin_index(radians);
                        cos_index = old_fast_cos_index(radians);
                        sin_value = self.old_fast_table[sin_index];
                        cos_value = self.old_fast_table[cos_index];
                        cos_index_adj = (cos_index as isize - 1024).rem_euclid(4096);
                    },
                    SelectedTable::NewFast => {
                        sin_index = new_fast_sin_index(radians);
                        cos_index = new_fast_cos_index(radians);
                        sin_value = self.new_fast_table[sin_index];
                        cos_value = self.new_fast_table[cos_index];
                        cos_index_adj = (cos_index as isize - 1024).rem_euclid(4096);
                    }
                };
                let sin_angle = sin_value.asin().to_degrees();
                let cos_angle = cos_value.acos().to_degrees();
                let normal = ((sin_value as f64).powf(2.0) + (cos_value as f64).powf(2.0)).sqrt();

                ui.label("");
                ui.label("Value");
                ui.label("Angle");
                ui.label("Index");
                ui.end_row();

                ui.label("Sin");
                ui.label(format!("{:.8}", sin_value));
                ui.label(format!("{:.8}", sin_angle));
                ui.label(format!("{}", sin_index));
                ui.end_row();

                ui.label("Cos");
                ui.label(format!("{:.8}", cos_value));
                ui.label(format!("{:.8}", cos_angle));
                ui.label(format!("{} ({})", cos_index_adj, cos_index));
                ui.end_row();

                ui.label("Normal");
                ui.label(format!("{:.8}", normal));
                ui.label("");
                let remaining = frame.info().window_info.size.x - ui.cursor().min.x;
                ui.add_sized([remaining - 8.0, 0.0], egui::Label::new(""));

            });
        });
    }
}