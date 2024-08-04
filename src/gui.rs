use eframe::egui;

#[derive(Default)]
pub struct Graffiti {}

impl eframe::App for Graffiti {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.image(egui::include_image!("../mbsetup-2016_1.jpg"));
            });
        });
    }
}
