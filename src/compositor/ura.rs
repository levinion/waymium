use anyhow::{Context, bail};
use std::process::Command;

use crate::compositor::{Compositor, WindowGeometry};

#[derive(serde::Deserialize)]
struct UraWindow {
    geometry: WindowGeometry,
    id: usize,
}

#[derive(serde::Deserialize, Default)]
pub struct Ura {
    wins: Vec<UraWindow>,
}

impl Compositor for Ura {
    fn update(&mut self) -> anyhow::Result<()> {
        let output = Command::new("sh")
            .args([
                "-c",
                "ura-cmd show-windows | jq 'map(select(.is_mapped == true))'",
            ])
            .output()?;
        if output.status.success() {
            let j = String::from_utf8(output.stdout)?;
            self.wins = serde_json::from_str(&j)?;
        } else {
            bail!(String::from_utf8(output.stderr)?);
        }
        Ok(())
    }

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
