use std::{cell::RefCell, path::PathBuf};
use vipera::Configuration;

use crate::{
    compositor::{Compositor, Compositor_},
    config::Config,
    label::Label,
};
use anyhow::Result;

pub struct State {
    pub config: Config,
    pub style: Option<PathBuf>,
    pub compositor: RefCell<Box<dyn Compositor>>,
    pub labels: RefCell<Vec<Label>>,
    pub buffer: RefCell<String>,
}

impl State {
    pub fn new() -> Result<Self> {
        let config = Config::read_in_config().unwrap_or_default();
        let style = Config::get_config_file().ok();
        let compositor = RefCell::new(Compositor_::from(config.compositor.clone())?);
        let labels = RefCell::new(Vec::new());
        let buffer = RefCell::new(String::new());
        Ok(Self {
            config,
            style,
            compositor,
            labels,
            buffer,
        })
    }
}
