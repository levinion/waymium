use anyhow::{Context, Result, bail};

mod ura;

#[derive(Clone, Copy, serde::Deserialize, serde::Serialize, Debug)]
pub struct WindowGeometry {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

pub trait Compositor {
    fn windows(&self) -> Result<Vec<WindowGeometry>>;
    fn activate(&self, n: usize) -> Result<()>;
}

pub struct Compositor_;

impl Compositor_ {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Result<Box<dyn Compositor>> {
        let name =
            std::env::var("XDG_CURRENT_DESKTOP").context("$XDG_CURRENT_DESKTOP is not set")?;
        let compositor = match name.as_str() {
            "ura" => ura::Ura::new()?,
            _ => bail!(format!("compositor not supported: {}", name)),
        };
        let compositor = Box::new(compositor);
        Ok(compositor)
    }
}
