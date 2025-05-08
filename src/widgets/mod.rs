pub mod generic;

use super::error::Error;
use gtk4::Widget as GtkWidget;
use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, Mutex},
};
use toml::Table;

type Factory = Box<dyn Fn(&Table) -> Result<Box<dyn Widget>, Error> + Sync + Send>;

pub trait Widget: Send + Sync {
    fn load(&self) -> Result<GtkWidget, Error>;
    fn from_toml(config: &Table) -> Result<Box<dyn Widget>, Error>
    where
        Self: Sized;
}

pub static WIDGET_REGISTRY: LazyLock<Arc<Mutex<HashMap<String, Box<dyn Widget>>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(HashMap::new())));

static WIDGET_FACTORIES: LazyLock<Arc<Mutex<HashMap<&'static str, Factory>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(HashMap::new())));

pub fn register_factory(widget_type: &'static str, factory: Factory) {
    let mut factories = WIDGET_FACTORIES.lock().unwrap();
    factories.insert(widget_type, factory);
}

pub fn create_widget_from_toml(name: String, config: &Table) -> Result<(), Error> {
    let widget_type = config
        .get("type")
        .ok_or(Error::MissingAttribute)?
        .as_str()
        .unwrap();

    let factories = WIDGET_FACTORIES.lock().unwrap();
    let factory = factories.get(widget_type).ok_or(Error::ModuleNotFound)?;

    let widget = factory(config)?;

    let mut registry = WIDGET_REGISTRY.lock().unwrap();
    registry.insert(name, widget);
    Ok(())
}

pub fn get_widget(name: &str) -> Result<GtkWidget, Error> {
    let registry = WIDGET_REGISTRY.lock().unwrap();
    registry.get(name).unwrap().load()
}
