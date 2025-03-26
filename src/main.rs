#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod character;
mod gradient;
mod simulator;
mod util;

fn init_logging() {
    let default_level = if cfg!(debug_assertions) {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };
    env_logger::Builder::new()
        .filter_level(default_level)
        .parse_env("LOG_DSA")
        .init();
}

fn create_app() -> eframe::AppCreator<'static> {
    Box::new(|creation_context| Ok(Box::new(app::App::new(creation_context))))
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    init_logging();

    let viewport = egui::ViewportBuilder::default()
        .with_inner_size([400.0, 300.0])
        .with_min_inner_size([300.0, 220.0])
        .with_icon(
            egui::IconData::default(),
            // eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
            //     .expect("Failed to load icon"),
        );

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    eframe::run_native("DSA Hybrid Char Editor", native_options, create_app())
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    init_logging();

    // Redirect `log` message to `console.log` and friends:
    // eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let start_result = eframe::WebRunner::new()
            .start("the_canvas_id", web_options, create_app())
            .await;

        // Remove the loading text and spinner:
        let loading_text = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("loading_text"));
        if let Some(loading_text) = loading_text {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}
