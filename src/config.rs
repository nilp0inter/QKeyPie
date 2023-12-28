use std::fs::File;
use std::io::Read;
use indexmap::IndexMap;

use serde::Serialize;
use serde::Deserialize;

use crate::actions::{Action, ButtonCallback, WheelSetCallback, ButtonSetCallback, ProfileCallback, ButtonId, WheelId, ButtonSetId, ProfileId, MacroId, ActiveCallback};

type Actions = Option<Vec<Action>>;


type ButtonSetConfig = ButtonSetCallback<Option<ButtonSetId>,Actions>;
type ProfileConfig = ProfileCallback<Option<IndexMap<String, ButtonSetId>>, Option<IndexMap<String, WheelId>>, Actions>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub server: Option<ActiveCallback<Actions>>,
    pub macros: Option<IndexMap<MacroId, Actions>>,
    pub buttons: Option<IndexMap<ButtonId, ButtonCallback<Actions>>>,
    pub wheels: Option<IndexMap<WheelId, WheelSetCallback<Actions>>>,
    pub buttonsets: Option<IndexMap<ButtonSetId, ButtonSetConfig>>,
    pub profiles: Option<IndexMap<ProfileId, ProfileConfig>>,
}

pub fn read_config(filename: &str) -> anyhow::Result<Config> {
    let mut file = File::open(filename)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let config: Config = toml::from_str(&data)?;

    Ok(config)
}
