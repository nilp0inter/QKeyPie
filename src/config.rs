use std::fs::File;
use std::io::Read;
use indexmap::IndexMap;

use serde::Serialize;
use serde::Deserialize;

use crate::actions::{Action, ButtonCallback, WheelSetCallback, ButtonSetCallback, ButtonId, WheelId, ButtonSetId, ProfileId, MacroId};

type Actions = Option<Vec<Action>>;


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Profile {
    pub buttonsets: Option<IndexMap<String, ButtonSetId>>,
    pub wheels: Option<IndexMap<String, WheelId>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub macros: Option<IndexMap<MacroId, Actions>>,
    pub buttons: Option<IndexMap<ButtonId, ButtonCallback<Actions>>>,
    pub wheels: Option<IndexMap<WheelId, WheelSetCallback<Actions>>>,
    pub buttonsets: Option<IndexMap<ButtonSetId, ButtonSetCallback<Option<ButtonSetId>,Actions>>>,
    pub profiles: Option<IndexMap<ProfileId, Profile>>,
}

pub fn read_config(filename: &str) -> anyhow::Result<Config> {
    let mut file = File::open(filename)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let config: Config = toml::from_str(&data)?;

    Ok(config)
}
