use eframe::egui::*;
use egui::{Color32, Painter, Pos2, Rect, Rounding, Vec2};

use crate::model::run_model;

const START_CIRCLE_COLOUR: Color32 = Color32::GREEN;
const FINISH_CIRCLE_COLOUR: Color32 = Color32::RED;
const INTERMEDIATE_CIRCLE_COLOUR: Color32 = Color32::BLUE;

const A18_X_COORDINATE: f32 = 275.0;
const A18_Y_COORDINATE: f32 = 180.0;

const HORIZONTAL_GAP: f32 = 40.0;
const VERTICAL_GAP: f32 = 40.0;

const RADIUS: f32 = 20.0;

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
                let window_size = ctx.input(|i: &egui::InputState| i.screen_rect());
                //gets dimensions of the whole UI Rect
                //this can then be used to place the circles around holds in the correct places
                //the method for this is to calculate the gap between 2 adjacent holds
                //(e.g. the gap between centre of a18 and b18) both vertically and horizontally
                //and then to add the number of these vertical/horizontal gaps to the position of
                //a18 (which is furthest left and up)
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
                        ).expect("failed to run model"));
                    },
                );
                ui.add_sized(
                    vec2(900.0, 800.0),
                    egui::Image::new(egui::include_image!(
                        "/Users/seba/rs/graffiti/mbsetup-2016_1.jpg"
                    )),
                );

                let painter = Painter::new(
                    ctx.clone(),
                    LayerId::new(Order::Foreground, Id::new("painter_id")),
                    window_size,
                );

                for hold in self.start_holds.clone() {
                    let coordinate = calculate_centre(hold);

                    painter.circle_stroke(
                        Pos2 {
                            x: coordinate.0,
                            y: coordinate.1,
                        },
                        RADIUS,
                        egui::Stroke::new(3.0, START_CIRCLE_COLOUR),
                    );
                }

                for hold in self.intermediate_holds.clone() {
                    let coordinate = calculate_centre(hold);

                    painter.circle_stroke(
                        Pos2 {
                            x: coordinate.0,
                            y: coordinate.1,
                        },
                        RADIUS,
                        egui::Stroke::new(3.0, INTERMEDIATE_CIRCLE_COLOUR),
                    );
                }

                for hold in self.finish_holds.clone() {
                    let coordinate = calculate_centre(hold);

                    painter.circle_stroke(
                        Pos2 {
                            x: coordinate.0,
                            y: coordinate.1,
                        },
                        RADIUS,
                        egui::Stroke::new(3.0, FINISH_CIRCLE_COLOUR),
                    );
                }
            });
        });
    }
}

fn calculate_centre(hold: String) -> (f32, f32) {
    let letter = hold.as_str().as_bytes()[0].to_ascii_uppercase();
    let number = hold[1..]
        .parse::<usize>()
        .expect("failed to get number from hold");

    let horizontal_displacement = (letter - b'A') as f32 * HORIZONTAL_GAP;
    let vertical_displacement = (18 - number) as f32 * VERTICAL_GAP;

    (
        A18_X_COORDINATE + horizontal_displacement,
        A18_Y_COORDINATE + vertical_displacement,
    )
}
