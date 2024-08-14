#![warn(clippy::all)]
#![allow(unused)]

pub mod climb;
pub mod gui;
pub mod json_parse;
pub mod model;
pub mod utils;
/*
use wasm_bindgen::JsValue;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

use crate::gui::Graffiti;

#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
pub fn run() -> Result<(), JsValue> {
    set_panic_hook();
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let start_result = eframe::WebRunner::new()
            .start(
                "graffiti_id",
                web_options,
                Box::new(|cc| {
                    // This gives us image support:
                    egui_extras::install_image_loaders(&cc.egui_ctx);
                    Ok(Box::<Graffiti>::default())
                }),
            )
            .await;
        });
    Ok(())
}
    */
