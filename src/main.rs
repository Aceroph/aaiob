use aaiob::Config;
use aaiob::logger;
use aaiob::widgets::{create_widget_from_toml, generic};
use gtk4::prelude::*;
use gtk4::{Application, glib};
use log::error;

#[allow(unused_must_use)]
fn main() -> glib::ExitCode {
    logger::init();

    // Register generic widgets
    generic::register_all();

    let app = Application::builder()
        .application_id("dev.aceroph.aaiob")
        .build();

    app.connect_activate(|app| {
        // Load config
        let config = Config::new(None);

        // Load widgets
        if let Some(modules) = config.modules {
            for (name, module) in modules.iter() {
                create_widget_from_toml(name.to_string(), module);
            }
        }

        // Load windows
        if let Some(windows) = config.windows {
            for (name, window) in windows.iter() {
                match window.init(app, name) {
                    Err(err) => error!("{err}"),
                    Ok(_) => (),
                }
            }
        }
    });

    app.run()
}
