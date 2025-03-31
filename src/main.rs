#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use std::fs;
use std::path::Path;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 140.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Raspberry Pi Pico UF2 Autoloader",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    uf2_name: String,
    mount_point: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            uf2_name: "".to_owned(),
            mount_point: "/tmp/".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let uf2_label = ui.label("UF2 file: ");
                ui.text_edit_singleline(&mut self.uf2_name)
                    .labelled_by(uf2_label.id);
            });
            if ui.button("Open File...").clicked() {
                println!("Open file dialog");
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.uf2_name = path.display().to_string();
                }
            }

            ui.separator();

            ui.horizontal(|ui| {
                let mount_point_label = ui.label("Pico mount point: ");
                ui.text_edit_singleline(&mut self.mount_point)
                    .labelled_by(mount_point_label.id);
            });

            if ui.button("Open Folder...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.mount_point = path.display().to_string();
                }
            }

            ui.separator();

            ui.horizontal(|ui| {
                let mut checked = true;
                ui.checkbox(&mut checked, "autoload");

                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    if ui.button("Load").clicked() {
                        println!("manually load uf2 to pico");
                    }
                });
            });

            // copy the UF2 file to the mount point
            if Path::new(&self.uf2_name).exists() && Path::new(&self.mount_point).exists() {
                let output_name = Path::new(&self.uf2_name).file_name().unwrap();
                let dest = Path::new(&self.mount_point).join(output_name);
                println!("Copying {} to {}", self.uf2_name, dest.display());
                match fs::copy(&self.uf2_name, dest) {
                    Ok(_) => {
                        println!("Successfully copied file");
                    }
                    Err(e) => {
                        println!("Error copying file: {:?}", e);
                    }
                }
            }
        });
    }
}
