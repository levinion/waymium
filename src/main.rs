mod app;
mod compositor;
mod config;
mod hint;
mod label;
mod state;
mod window;

use crate::state::State;
use anyhow::Result;

fn main() -> Result<()> {
    let state = State::new()?;
    let app = app::Application::new(state);
    app.run();
    Ok(())
}
