use std::collections::HashMap;

use crate::actions::{Action, Wheel, ButtonSet, Button, WheelId, ButtonId, ButtonSetId, ProfileId, ButtonInteractionType, LowLevelButtonInteraction, HighLevelButtonInteraction};
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

fn get_labeled_button_by_id(cfg: &Config, id: &ButtonId) -> anyhow::Result<LabeledButton<Actions>> {
    let cfg_button = cfg.buttons.as_ref().and_then(|buttons| buttons.get(id)).ok_or_else(|| anyhow::anyhow!("Button {} not found", id))?;

    let button : LabeledButton<Actions> = LabeledButton {
        label: "".to_string(),
        button: Button {
            interaction: match &cfg_button.interaction {
                ButtonInteractionType::LowLevel(LowLevelButtonInteraction { on_press, on_release }) => ButtonInteractionType::LowLevel(LowLevelButtonInteraction {
                    on_press: on_press.as_ref().unwrap_or(&vec![]).to_vec(),
                    on_release: on_release.as_ref().unwrap_or(&vec![]).clone(),
                }),
                ButtonInteractionType::HighLevel(HighLevelButtonInteraction { on_click, on_double_click, on_triple_click, on_long_press }) => ButtonInteractionType::HighLevel(HighLevelButtonInteraction {
                    on_click: on_click.as_ref().unwrap_or(&vec![]).clone(),
                    on_double_click: on_double_click.as_ref().unwrap_or(&vec![]).clone(),
                    on_triple_click: on_triple_click.as_ref().unwrap_or(&vec![]).clone(),
                    on_long_press: on_long_press.as_ref().unwrap_or(&vec![]).clone(),
                }),
            },
            on_show: cfg_button.on_show.as_ref().unwrap_or(&vec![]).clone(),
            on_hide: cfg_button.on_hide.as_ref().unwrap_or(&vec![]).clone(),
        },
    };

    Ok(button)
}

fn get_labeled_button(cfg: &Config, id: &Option<ButtonId>) -> anyhow::Result<LabeledButton<Actions>> {
    match id {
        Some(id) => get_labeled_button_by_id(cfg, &id.clone()),
        None => Ok(LabeledButton {
            label: "".to_string(),
            button: Button {
                interaction: ButtonInteractionType::LowLevel(LowLevelButtonInteraction {
                    on_press: vec![],
                    on_release: vec![],
                }),
                on_show: vec![],
                on_hide: vec![],
            },
        }),
    }
}

fn get_button_by_id(cfg: &Config, id: &ButtonId) -> anyhow::Result<Button<Actions>> {
    let cfg_button = cfg.buttons.as_ref().and_then(|buttons| buttons.get(id)).ok_or_else(|| anyhow::anyhow!("Button {} not found", id))?;

    let button = Button {
        interaction: match &cfg_button.interaction {
            ButtonInteractionType::LowLevel(LowLevelButtonInteraction { on_press, on_release }) => ButtonInteractionType::LowLevel(LowLevelButtonInteraction {
                on_press: on_press.as_ref().unwrap_or(&vec![]).to_vec(),
                on_release: on_release.as_ref().unwrap_or(&vec![]).clone(),
            }),
            ButtonInteractionType::HighLevel(HighLevelButtonInteraction { on_click, on_double_click, on_triple_click, on_long_press }) => ButtonInteractionType::HighLevel(HighLevelButtonInteraction {
                on_click: on_click.as_ref().unwrap_or(&vec![]).clone(),
                on_double_click: on_double_click.as_ref().unwrap_or(&vec![]).clone(),
                on_triple_click: on_triple_click.as_ref().unwrap_or(&vec![]).clone(),
                on_long_press: on_long_press.as_ref().unwrap_or(&vec![]).clone(),
            }),
        },
        on_show: cfg_button.on_show.as_ref().unwrap_or(&vec![]).clone(),
        on_hide: cfg_button.on_hide.as_ref().unwrap_or(&vec![]).clone(),
    };

    Ok(button)
}

fn get_button(cfg: &Config, id: &Option<ButtonId>) -> anyhow::Result<Button<Actions>> {
    match id {
        Some(id) => get_button_by_id(cfg, id),
        None => Ok(Button {
            interaction: ButtonInteractionType::LowLevel(LowLevelButtonInteraction {
                on_press: vec![],
                on_release: vec![],
            }),
            on_show: vec![],
            on_hide: vec![],
        }),
    }
}

fn get_wheel(cfg: &Config, id: &WheelId) -> anyhow::Result<Wheel<Actions>> {
    let cfg_wheel = cfg.wheels.as_ref().and_then(|wheels| wheels.get(id)).ok_or_else(|| anyhow::anyhow!("Wheel {} not found", id))?;

    let wheel = Wheel {
        on_clockwise: cfg_wheel.on_clockwise.as_ref().unwrap_or(&vec![]).clone(),
        on_clockwise_start: cfg_wheel.on_clockwise_start.as_ref().unwrap_or(&vec![]).clone(),
        on_clockwise_stop: cfg_wheel.on_clockwise_stop.as_ref().unwrap_or(&vec![]).clone(),
        on_counterclockwise: cfg_wheel.on_counterclockwise.as_ref().unwrap_or(&vec![]).clone(),
        on_counterclockwise_start: cfg_wheel.on_counterclockwise_start.as_ref().unwrap_or(&vec![]).clone(),
        on_counterclockwise_stop: cfg_wheel.on_counterclockwise_stop.as_ref().unwrap_or(&vec![]).clone(),
        on_show: cfg_wheel.on_show.as_ref().unwrap_or(&vec![]).clone(),
        on_hide: cfg_wheel.on_hide.as_ref().unwrap_or(&vec![]).clone(),
    };

    Ok(wheel)
}

fn get_buttonset(cfg: &Config, id: &ButtonSetId) -> anyhow::Result<ButtonSet<LabeledButton<Actions>, Button<Actions>>> {
    let cfg_buttonset = cfg.buttonsets.as_ref().and_then(|buttonsets| buttonsets.get(id)).ok_or_else(|| anyhow::anyhow!("Buttonset {} not found", id))?;

    let buttonset = ButtonSet {
        button1: get_labeled_button(cfg, &cfg_buttonset.button1)?,
        button2: get_labeled_button(cfg, &cfg_buttonset.button2)?,
        button3: get_labeled_button(cfg, &cfg_buttonset.button3)?,
        button4: get_labeled_button(cfg, &cfg_buttonset.button4)?,
        button5: get_labeled_button(cfg, &cfg_buttonset.button5)?,
        button6: get_labeled_button(cfg, &cfg_buttonset.button6)?,
        button7: get_labeled_button(cfg, &cfg_buttonset.button7)?,
        button8: get_labeled_button(cfg, &cfg_buttonset.button8)?,
        button9: get_button(cfg, &cfg_buttonset.button9)?,
    };

    Ok(buttonset)
}

pub fn from_config(cfg: Config) -> anyhow::Result<Model> {
    let mut profiles = HashMap::new();

    for (cfg_profile_name, cfg_profile) in cfg.clone().profiles.unwrap_or_default() {
        let mut buttonsets = HashMap::new();
        let mut wheels = HashMap::new();

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

