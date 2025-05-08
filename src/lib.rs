pub mod error;
pub mod logger;
pub mod widgets;

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
    pub position: String,
    pub widget: Option<Value>,
    pub width: i32,
    pub height: i32,
}

impl Window {
    pub fn init(&self, app: &Application, name: &str) {
        let window = ApplicationWindow::builder()
            .title(name)
            .decorated(false)
            .default_width(self.width)
            .default_height(self.height)
            .application(app)
            .build();

        match self.position.as_str() {
            "top" => window.set_anchor(Edge::Top, true),
            e => panic!("Unknown position {}", e),
        }

        window.present();

        if let Some(widget) = &self.widget {
            println!("testt");
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
