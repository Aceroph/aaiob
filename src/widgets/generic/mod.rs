use super::{Widget, register_factory};

mod button;
mod label;

pub fn register_all() {
    register_factory("generic/button", Box::new(button::Button::from_toml));
    register_factory("generic/label", Box::new(label::Label::from_toml));
}
