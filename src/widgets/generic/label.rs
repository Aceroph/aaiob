use gtk4::Label as GtkLabel;
use serde::Deserialize;

use crate::widgets::{Widget, WidgetFactory};

pub struct LabelFactory;

#[derive(Deserialize)]
struct Label {
    format: Option<String>,
}

impl Widget for Label {
    fn load(&self) -> Result<gtk4::Widget, crate::error::Error> {
        let label_content = if let Some(text) = &self.format {
            Some(text.as_str())
        } else {
            None
        };
        let label = GtkLabel::new(label_content);
        Ok(label.into())
    }
}

impl WidgetFactory for LabelFactory {
    fn create_from_toml(
        &self,
        config: &toml::Table,
    ) -> Result<Box<dyn Widget>, crate::error::Error> {
        let widget: Label = config.clone().try_into().unwrap();
        Ok(Box::new(widget))
    }
}
