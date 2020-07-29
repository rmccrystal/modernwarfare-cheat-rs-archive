use crate::hacks::aimbot::AimbotConfig;

// The config struct passed in the main hack loop
pub struct Config {
    pub aimbot_config: AimbotConfig
}

impl Config {
    // Creates a config with the default settings
    pub fn default() -> Self {
        Self {
            aimbot_config: AimbotConfig::default()
        }
    }
}
