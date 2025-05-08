pub mod error;
pub mod logger;
pub mod widgets;

use error::Error;
use gtk4::{
    Application, ApplicationWindow, builders::WindowBuilder, glib::user_config_dir,
    prelude::GtkWindowExt,
};
use gtk4_layer_shell::{Edge, LayerShell};
use log::error;
use serde::Deserialize;
use std::{collections::HashMap, fs::read_to_string, path::Path, process};
use toml::{Table, Value};
use widgets::{create_widget_from_toml, get_widget};

#[derive(Deserialize)]
pub struct Window {
    pub position: Option<String>,
    pub widget: Option<Value>,
    pub width: i32,
    pub height: i32,
}

impl Window {
    pub fn init(&self, app: &Application, name: &str) -> Result<(), error::Error> {
        let window = ApplicationWindow::builder()
            .title(name)
            .decorated(false)
            .default_width(self.width)
            .default_height(self.height)
            .application(app)
            .build();

        if let Some(position) = &self.position {
            if gtk4_layer_shell::is_supported() {
                let anchors = match position.as_str() {
                    "top" => vec![Edge::Left, Edge::Top, Edge::Right],
                    _ => Err(Error::InvalidValueForAttribute("position"))?,
                };
                window.init_layer_shell();
                for anchor in anchors {
                    window.set_anchor(anchor, true);
                }
            } else {
                Err(Error::X11NotSupported)?
            }
        }

        window.present();

        if let Some(widget) = &self.widget {
            let gtk_widget = match widget {
                Value::Table(widget_config) => {
                    let _ = create_widget_from_toml("app".to_string(), widget_config);
                    get_widget("app")
                }
                Value::String(name) => get_widget(&name),
                _ => todo!("Invalid type passed as widget"),
            }
            .unwrap();
            window.set_child(Some(&gtk_widget));
        }
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub windows: Option<HashMap<String, Window>>,
    pub modules: Option<HashMap<String, Table>>,
}

impl Config {
    pub fn new(path: Option<&Path>) -> Config {
        let home_dir = user_config_dir().join("aaiob/config.toml");
        let filepath = path.unwrap_or(home_dir.as_path());

        match read_to_string(filepath) {
            Ok(s) => toml::from_str(&s).expect("Invalid TOML format !"),
            _ => {
                error!("Couldn't find any configuration at\n{}", filepath.display());
                process::exit(1)
            }
        }
    }
}
