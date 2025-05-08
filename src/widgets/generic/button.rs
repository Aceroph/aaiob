use crate::error::Error;

use crate::widgets::{Widget, WidgetFactory, register_factory};
use gtk4::{Button as GtkButton, Widget as GtkWidget};

pub fn register() {
    register_factory("generic/button", Box::new(ButtonFactory));
}

struct ButtonFactory;
struct Button {
    label: String,
}

impl Widget for Button {
    fn load(&self) -> Result<GtkWidget, Error> {
        Ok(GtkButton::with_label(&self.label).into())
    }
}

impl WidgetFactory for ButtonFactory {
    fn create_from_toml(&self, config: &toml::Table) -> Result<Box<dyn Widget>, Error> {
        Ok(Box::new(Button {
            label: config
                .get("label")
                .ok_or(Error::MissingAttribute)?
                .to_string(),
        }))
    }
}
