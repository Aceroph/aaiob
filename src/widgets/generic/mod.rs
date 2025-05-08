use super::register_factory;

mod button;
mod label;

pub fn register_all() {
    register_factory("generic/button", Box::new(button::ButtonFactory));
    register_factory("generic/label", Box::new(label::LabelFactory));
}
