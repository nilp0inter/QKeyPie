use indexmap::IndexMap;

use crate::actions::{Action, WheelCallback, ButtonSet, ButtonCallback, WheelId, ButtonId, ButtonSetId, ProfileId};
use crate::config::Config;

type Actions = Vec<Action>;

#[derive(Debug)]
pub struct Profile {
    pub buttonsets: IndexMap<ButtonSetId, ButtonSet<ButtonCallback<Actions>>>,
    pub wheels: IndexMap<WheelId, WheelCallback<Actions>>,
}

#[derive(Debug)]
pub struct Model {
    pub profiles: IndexMap<ProfileId, Profile>,
}

fn clone_or_empty<T: Clone>(opt: &Option<Vec<T>>) -> Vec<T> {
    opt.clone().unwrap_or_else(Vec::new)
}

fn get_button_by_id(cfg: &Config, id: &ButtonId) -> anyhow::Result<ButtonCallback<Actions>> {
    let cfg_button = cfg.buttons.as_ref().and_then(|buttons| buttons.get(id)).ok_or_else(|| anyhow::anyhow!("Button {} not found", id))?;

    let button : ButtonCallback<Actions> = ButtonCallback {
        on_click: clone_or_empty(&cfg_button.on_click),
        on_double_click: clone_or_empty(&cfg_button.on_double_click),
        on_triple_click: clone_or_empty(&cfg_button.on_triple_click),
        on_long_press: clone_or_empty(&cfg_button.on_long_press),
        on_press: clone_or_empty(&cfg_button.on_press),
        on_release: clone_or_empty(&cfg_button.on_release),
    };
    Ok(button)
}

fn get_button(cfg: &Config, id: &Option<ButtonId>) -> anyhow::Result<ButtonCallback<Actions>> {
    match id {
        Some(id) => get_button_by_id(cfg, id),
        None => Ok(ButtonCallback::default()),
    }
}

fn get_wheel(cfg: &Config, id: &WheelId) -> anyhow::Result<WheelCallback<Actions>> {
    let cfg_wheel = cfg.wheels.as_ref().and_then(|wheels| wheels.get(id)).ok_or_else(|| anyhow::anyhow!("Wheel {} not found", id))?;

    let wheel = WheelCallback {
        on_clockwise: clone_or_empty(&cfg_wheel.on_clockwise),
        on_clockwise_start: clone_or_empty(&cfg_wheel.on_clockwise_start),
        on_clockwise_stop: clone_or_empty(&cfg_wheel.on_clockwise_stop),
        on_counterclockwise: clone_or_empty(&cfg_wheel.on_counterclockwise),
        on_counterclockwise_start: clone_or_empty(&cfg_wheel.on_counterclockwise_start),
        on_counterclockwise_stop: clone_or_empty(&cfg_wheel.on_counterclockwise_stop),
        button: ButtonCallback {
            on_click: clone_or_empty(&cfg_wheel.button.on_click),
            on_double_click: clone_or_empty(&cfg_wheel.button.on_double_click),
            on_triple_click: clone_or_empty(&cfg_wheel.button.on_triple_click),
            on_long_press: clone_or_empty(&cfg_wheel.button.on_long_press),
            on_press: clone_or_empty(&cfg_wheel.button.on_press),
            on_release: clone_or_empty(&cfg_wheel.button.on_release),
        },
    };

    Ok(wheel)
}

fn get_buttonset(cfg: &Config, id: &ButtonSetId) -> anyhow::Result<ButtonSet<ButtonCallback<Actions>>> {
    let cfg_buttonset = cfg.buttonsets.as_ref().and_then(|buttonsets| buttonsets.get(id)).ok_or_else(|| anyhow::anyhow!("Buttonset {} not found", id))?;

    let buttonset = ButtonSet {
        button0: get_button(cfg, &cfg_buttonset.button0)?,
        button1: get_button(cfg, &cfg_buttonset.button1)?,
        button2: get_button(cfg, &cfg_buttonset.button2)?,
        button3: get_button(cfg, &cfg_buttonset.button3)?,
        button4: get_button(cfg, &cfg_buttonset.button4)?,
        button5: get_button(cfg, &cfg_buttonset.button5)?,
        button6: get_button(cfg, &cfg_buttonset.button6)?,
        button7: get_button(cfg, &cfg_buttonset.button7)?,
        button_extra: get_button(cfg, &cfg_buttonset.button_extra)?,
    };

    Ok(buttonset)
}

pub fn from_config(cfg: Config) -> anyhow::Result<Model> {
    let mut profiles = IndexMap::new();

    for (cfg_profile_name, cfg_profile) in cfg.clone().profiles.unwrap_or_default() {
        let mut buttonsets = IndexMap::new();
        let mut wheels = IndexMap::new();

        for (cfg_buttonset_name, cfg_buttonset_id) in cfg_profile.buttonsets.unwrap_or_default() {
            let buttonset = get_buttonset(&cfg.clone(), &cfg_buttonset_id)?;
            buttonsets.insert(cfg_buttonset_name, buttonset);
        }

        for (cfg_wheel_name, cfg_wheel_id) in cfg_profile.wheels.unwrap_or_default() {
            let wheel = get_wheel(&cfg, &cfg_wheel_id)?;
            wheels.insert(cfg_wheel_name, wheel);
        }

        profiles.insert(cfg_profile_name, Profile {
            buttonsets,
            wheels,
        });
    }

    Ok(Model {
        profiles,
    })
}

