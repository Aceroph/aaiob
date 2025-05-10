pub mod error;
pub mod logger;
pub mod widgets;

use error::Error;
use gtk4::{Application, ApplicationWindow, Widget, glib::user_config_dir, prelude::GtkWindowExt};
use gtk4_layer_shell::{Edge, LayerShell};
use log::error;
use serde::Deserialize;
use std::{collections::HashMap, fs::read_to_string, path::Path, process};
use toml::Table;
use widgets::{create_widget_from_toml, get_widget};

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Module {
    Reference(String),
    Definition(Table),
}

impl Into<Widget> for Module {
    fn into(self) -> Widget {
        match self {
            Module::Definition(widget_config) => {
                create_widget_from_toml("app".to_string(), &widget_config).unwrap();
                get_widget("app")
            }
            Module::Reference(name) => get_widget(&name),
        }
        .unwrap()
    }
}

impl From<&Module> for Widget {
    fn from(value: &Module) -> Self {
        match value {
            Module::Definition(widget_config) => {
                create_widget_from_toml("app".to_string(), &widget_config).unwrap();
                get_widget("app")
            }
            Module::Reference(name) => get_widget(&name),
        }
        .unwrap()
    }
}

#[derive(Deserialize)]
pub struct Window {
    pub position: Option<String>,
    pub widget: Option<Module>,
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
                    "top-left" | "left-top" => vec![Edge::Left, Edge::Top],
                    "top-right" | "right-top" => vec![Edge::Top, Edge::Right],
                    "top-center" | "center-top" => vec![Edge::Top],
                    "left" => vec![Edge::Top, Edge::Left, Edge::Bottom],
                    "left-center" | "center-left" => vec![Edge::Left],
                    "center" => vec![],
                    "right" => vec![Edge::Top, Edge::Right, Edge::Bottom],
                    "right-center" | "center-right" => vec![Edge::Right],
                    "bottom" => vec![Edge::Left, Edge::Top, Edge::Right],
                    "bottom-left" | "left-bottom" => vec![Edge::Left, Edge::Bottom],
                    "bottom-center" | "center-bottom" => vec![Edge::Bottom],
                    "bottom-right" | "right-bottom" => vec![Edge::Bottom, Edge::Right],
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
            window.set_child(Some(&Into::<Widget>::into(widget)));
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
