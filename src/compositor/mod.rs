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
    fn update(&mut self) -> Result<()>;
    fn windows(&self) -> Result<Vec<WindowGeometry>>;
    fn activate(&self, n: usize) -> Result<()>;
}

pub struct Compositor_;

impl Compositor_ {
    pub fn from(name: String) -> Result<Box<dyn Compositor>> {
        let name = match name.to_lowercase().as_str() {
            "auto" => {
                std::env::var("XDG_CURRENT_DESKTOP").context("$XDG_CURRENT_DESKTOP is not set")?
            }
            _ => name,
        };
        let compositor = match name.to_lowercase().as_str() {
            "ura" => ura::Ura::default(),
            _ => bail!(format!("compositor not supported: {}", name)),
        };
        let compositor = Box::new(compositor);
        Ok(compositor)
    }
}
