use eframe::egui::*;

use crate::model::{run_model, write_holds_to_file};

#[derive(Default)]
pub struct Graffiti {
    new_start_hold: String,
    new_finish_hold: String,
    new_intermediate_hold: String,
    start_holds: Vec<String>,
    finish_holds: Vec<String>,
    intermediate_holds: Vec<String>,
}

impl eframe::App for Graffiti {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                let window_margin = ui.spacing().window_margin;
                let size_2x1 = vec2(64.0 + window_margin.left, 26.0);
                //TODO: check for these that the holds are actually on the moonboard
                ui.horizontal(|ui| {
                    let name_label = ui.label("Add start hold: ");
                    ui.text_edit_singleline(&mut self.new_start_hold)
                        .labelled_by(name_label.id);
                    if ui.button("Add").clicked() {
                        self.start_holds.push(self.new_start_hold.clone());
                    }
                });

                ui.horizontal(|ui| {
                    let name_label = ui.label("Add finish hold: ");
                    ui.text_edit_singleline(&mut self.new_finish_hold)
                        .labelled_by(name_label.id);
                    if ui.button("Add").clicked() {
                        self.finish_holds.push(self.new_finish_hold.clone());
                    }
                });
                ui.horizontal(|ui| {
                    let name_label = ui.label("Add intermediate hold: ");
                    ui.text_edit_singleline(&mut self.new_intermediate_hold)
                        .labelled_by(name_label.id);
                    if ui.button("Add").clicked() {
                        self.intermediate_holds
                            .push(self.new_intermediate_hold.clone());
                    }
                });

                let mut start_holds = String::new();
                for h in self.start_holds.clone() {
                    start_holds += h.as_str();
                    start_holds += " ";
                }
                ui.label(format!("Start holds: {}", start_holds));

                let mut finish_holds = String::new();
                for h in self.finish_holds.clone() {
                    finish_holds += h.as_str();
                    finish_holds += " ";
                }
                ui.label(format!("Finish holds: {}", finish_holds));

                let mut intermediate_holds = String::new();
                for h in self.intermediate_holds.clone() {
                    intermediate_holds += h.as_str();
                    intermediate_holds += " ";
                }
                ui.label(format!("Intermediate holds: {}", intermediate_holds));

                let popup_id = Id::new("popup_id");

                let response = ui.add_sized(size_2x1, Button::new("Guess Grade"));

                let mut output = String::new();

                if response.clicked() {
                    ui.memory_mut(|mem| mem.toggle_popup(popup_id));
                }

                popup_below_widget(
                    ui,
                    popup_id,
                    &response,
                    PopupCloseBehavior::CloseOnClickOutside,
                    |ui| {
                        ui.set_min_height(20.0);
                        ui.set_min_width(160.0);
                        ui.heading(run_model(
                            self.start_holds.clone(),
                            self.finish_holds.clone(),
                            self.intermediate_holds.clone(),
                        ));
                    },
                );

                ui.add_sized(
                    vec2(300.0, 500.0),
                    egui::Image::new(egui::include_image!("../mbsetup-2016_1.jpg")),
                );
            });
        });
    }
}
