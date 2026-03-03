use anyhow::{Context, Result, bail};
use std::process::Command;

use crate::compositor::{Compositor, WindowGeometry};

#[derive(serde::Deserialize)]
struct UraWindow {
    geometry: WindowGeometry,
    id: usize,
}

#[derive(serde::Deserialize)]
pub struct Ura {
    wins: Vec<UraWindow>,
}

impl Ura {
    pub fn new() -> Result<Self> {
        let output = Command::new("sh")
            .args([
                "-c",
                "ura-cmd show-windows | jq 'map(select(.is_mapped == true))'",
            ])
            .output()?;
        if output.status.success() {
            let j = String::from_utf8(output.stdout)?;
            let wins: Vec<UraWindow> = serde_json::from_str(&j)?;
            Ok(Ura { wins })
        } else {
            bail!(String::from_utf8(output.stderr)?);
        }
    }
}

impl Compositor for Ura {
    fn windows(&self) -> anyhow::Result<Vec<super::WindowGeometry>> {
        let wins = self.wins.iter().map(|w| w.geometry).collect::<Vec<_>>();
        Ok(wins)
    }

    fn activate(&self, n: usize) -> anyhow::Result<()> {
        let win = self
            .wins
            .get(n)
            .context(format!("window #{} not found: ", n))?;
        Command::new("ura-shell")
            .args([
                "-c",
                &format!("ura.class.UraWindow:new({}):focus()", win.id),
            ])
            .spawn()?
            .wait()?;
        Ok(())
    }
}
