use std::collections::HashMap;

use crate::actions::{Action, Wheel, ButtonSet, Button, WheelId, ButtonSetId, ProfileId};
use crate::config::Config;

type Actions = Vec<Action>;

#[derive(Debug)]
struct LabeledButton<T> {
    label: String,
    button: Button<T>,
}

#[derive(Debug)]
pub struct Profile {
    buttonsets: HashMap<ButtonSetId, ButtonSet<LabeledButton<Actions>, Button<Actions>>>,
    wheels: HashMap<WheelId, Wheel<Actions>>,
}

#[derive(Debug)]
pub struct Model {
    profiles: HashMap<ProfileId, Profile>,
}

pub fn from_config(cfg: Config) -> anyhow::Result<Model> {
    let profiles = HashMap::new();

    Ok(Model {
        profiles,
    })
}

