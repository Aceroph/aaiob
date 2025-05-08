use crate::error::Error;

use crate::widgets::Widget;
use gtk4::{Button as GtkButton, Widget as GtkWidget};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Button {
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
    fn from_toml(config: &toml::Table) -> Result<Box<dyn Widget>, Error>
    where
        Self: Sized,
    {
        let widget: Button = config.clone().try_into().unwrap();
        Ok(Box::new(widget))
    }
}
