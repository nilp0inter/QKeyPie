use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

use serde::Serialize;
use serde::Deserialize;

use crate::actions::{Action, Button, Wheel, ButtonSet, ButtonId, WheelId, ButtonSetId, ProfileId};

type Actions = Option<Vec<Action>>;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
    pub buttonsets: Option<HashMap<String, ButtonSetId>>,
    pub wheels: Option<HashMap<String, WheelId>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub buttons: Option<HashMap<ButtonId, Button<Actions>>>,
    pub wheels: Option<HashMap<WheelId, Wheel<Actions>>>,
    pub buttonsets: Option<HashMap<ButtonSetId, ButtonSet<Option<ButtonSetId>, Option<ButtonSetId>>>>,
    pub profiles: Option<HashMap<ProfileId, Profile>>,
}

pub fn read_config(filename: &str) -> anyhow::Result<Config> {
    let mut file = File::open(filename)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let config: Config = toml::from_str(&data)?;

    Ok(config)
}
