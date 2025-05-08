use super::{Widget, register_factory};

mod button;
mod label;

pub fn register_all() {
    register_factory("generic/button", button::Button::from_toml);
    register_factory("generic/label", label::Label::from_toml);
}
