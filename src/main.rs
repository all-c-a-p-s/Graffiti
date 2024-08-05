#![allow(dead_code)]
#![allow(unused)]

use model::import_model;

use crate::climb::*;
use crate::gui::*;
use crate::json_parse::*;

pub mod climb;
pub mod gui;
pub mod json_parse;
#[path = "/Users/seba/rs/graffiti/src/model/model.rs"]
pub mod model;

fn main() {
    import_model();
}

/* 
fn main() -> eframe::Result {
    //let r = read_route();
    //println!("{}", r);
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Graffiti",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<Graffiti>::default())
        }),
    )
}
*/
