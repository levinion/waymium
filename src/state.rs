use std::cell::RefCell;
use vipera::Configuration;

use crate::{
    compositor::{Compositor, Compositor_},
    config::Config,
};
use anyhow::Result;

pub struct State {
    pub config: Config,
    pub compositor: Box<dyn Compositor>,
    pub charset: RefCell<Vec<(String, usize)>>,
    pub buffer: RefCell<String>,
}

impl State {
    pub fn new() -> Result<Self> {
        let config = Config::read_in_config().unwrap_or_default();
        let compositor = Compositor_::new()?;
        let charset = RefCell::new(Vec::new());
        let buffer = RefCell::new(String::new());
        Ok(Self {
            config,
            compositor,
            charset,
            buffer,
        })
    }
}
