use anyhow::Result;

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct Config {}

impl vipera::Configuration for Config {
    fn vipera() -> Result<vipera::Vipera> {
        let vipera = vipera::Vipera::new()
            .set_config_name("config.toml")?
            .add_config_path("$XDG_CONFIG_HOME/waymium")?
            .add_config_path("$HOME/.config/waymium")?
            .add_config_path("/etc/waymium")?;
        Ok(vipera)
    }
}
