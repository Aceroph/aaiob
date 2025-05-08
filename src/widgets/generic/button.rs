use crate::error::Error;

use crate::widgets::{Widget, WidgetFactory};
use gtk4::{Button as GtkButton, Widget as GtkWidget};
use serde::Deserialize;

pub struct ButtonFactory;

#[derive(Deserialize)]
struct Button {
    label: Option<String>,
}

impl Widget for Button {
    fn load(&self) -> Result<GtkWidget, Error> {
        let button = if let Some(label) = &self.label {
            GtkButton::with_label(&label)
        } else {
            GtkButton::new()
        };

        Ok(button.into())
    }
}

impl WidgetFactory for ButtonFactory {
    fn create_from_toml(&self, config: &toml::Table) -> Result<Box<dyn Widget>, Error> {
        let widget: Button = config.clone().try_into().unwrap();
        Ok(Box::new(widget))
    }
}
