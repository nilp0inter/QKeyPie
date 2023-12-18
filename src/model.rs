use indexmap::IndexMap;

use crate::actions::{Action, Wheel, ButtonSet, Button, WheelId, ButtonId, ButtonSetId, ProfileId, LowLevelButton, HighLevelButton, LowLevelWheel, HighLevelWheel};
use crate::config::Config;

type Actions = Vec<Action>;

#[derive(Debug)]
struct LabeledButton<T> {
    label: String,
    button: Button<T>,
}

#[derive(Debug)]
pub struct Profile {
    buttonsets: IndexMap<ButtonSetId, ButtonSet<LabeledButton<Actions>, Button<Actions>>>,
    wheels: IndexMap<WheelId, Wheel<Actions>>,
}

#[derive(Debug)]
pub struct Model {
    profiles: IndexMap<ProfileId, Profile>,
}

fn get_button_by_id(cfg: &Config, id: &ButtonId) -> anyhow::Result<Button<Actions>> {
    let cfg_button = cfg.buttons.as_ref().and_then(|buttons| buttons.get(id)).ok_or_else(|| anyhow::anyhow!("Button {} not found", id))?;

    let button = match &cfg_button {
        Button::LowLevel(LowLevelButton { on_press, on_release, on_show, on_hide }) => Button::LowLevel(LowLevelButton {
            on_press: on_press.as_ref().unwrap_or(&vec![]).to_vec(),
            on_release: on_release.as_ref().unwrap_or(&vec![]).clone(),
            on_show: on_show.as_ref().unwrap_or(&vec![]).clone(),
            on_hide: on_hide.as_ref().unwrap_or(&vec![]).clone(),
        }),
        Button::HighLevel(HighLevelButton { on_click, on_double_click, on_triple_click, on_long_press, on_show, on_hide }) => Button::HighLevel(HighLevelButton {
            on_click: on_click.as_ref().unwrap_or(&vec![]).clone(),
            on_double_click: on_double_click.as_ref().unwrap_or(&vec![]).clone(),
            on_triple_click: on_triple_click.as_ref().unwrap_or(&vec![]).clone(),
            on_long_press: on_long_press.as_ref().unwrap_or(&vec![]).clone(),
            on_show: on_show.as_ref().unwrap_or(&vec![]).clone(),
            on_hide: on_hide.as_ref().unwrap_or(&vec![]).clone(),
        }),
    };
    Ok(button)
}

fn get_labeled_button(cfg: &Config, id: &Option<ButtonId>) -> anyhow::Result<LabeledButton<Actions>> {
    match id {
        Some(id) => Ok(LabeledButton {
            label: "".to_string(),
            button: get_button_by_id(cfg, id)?,
        }),
        None => Ok(LabeledButton {
            label: "".to_string(),
            button: Button::LowLevel(LowLevelButton {
                on_press: vec![],
                on_release: vec![],
                on_show: vec![],
                on_hide: vec![],
            }),
        }),
    }
}

fn get_button(cfg: &Config, id: &Option<ButtonId>) -> anyhow::Result<Button<Actions>> {
    match id {
        Some(id) => get_button_by_id(cfg, id),
        None => Ok(Button::LowLevel(LowLevelButton {
            on_press: vec![],
            on_release: vec![],
            on_show: vec![],
            on_hide: vec![],
        })),
    }
}

fn get_wheel(cfg: &Config, id: &WheelId) -> anyhow::Result<Wheel<Actions>> {
    let cfg_wheel = cfg.wheels.as_ref().and_then(|wheels| wheels.get(id)).ok_or_else(|| anyhow::anyhow!("Wheel {} not found", id))?;

    // let wheel = Wheel {
    //     on_clockwise: cfg_wheel.on_clockwise.as_ref().unwrap_or(&vec![]).clone(),
    //     on_clockwise_start: cfg_wheel.on_clockwise_start.as_ref().unwrap_or(&vec![]).clone(),
    //     on_clockwise_stop: cfg_wheel.on_clockwise_stop.as_ref().unwrap_or(&vec![]).clone(),
    //     on_counterclockwise: cfg_wheel.on_counterclockwise.as_ref().unwrap_or(&vec![]).clone(),
    //     on_counterclockwise_start: cfg_wheel.on_counterclockwise_start.as_ref().unwrap_or(&vec![]).clone(),
    //     on_counterclockwise_stop: cfg_wheel.on_counterclockwise_stop.as_ref().unwrap_or(&vec![]).clone(),
    //     on_show: cfg_wheel.on_show.as_ref().unwrap_or(&vec![]).clone(),
    //     on_hide: cfg_wheel.on_hide.as_ref().unwrap_or(&vec![]).clone(),
    // };
    let wheel = match cfg_wheel {
        Wheel::LowLevel(LowLevelWheel {
            on_clockwise,
            on_clockwise_start,
            on_clockwise_stop,
            on_counterclockwise,
            on_counterclockwise_start,
            on_counterclockwise_stop,
            on_show,
            on_hide,
            on_press,
            on_release,
        }) => Wheel::LowLevel(LowLevelWheel {
            on_clockwise: on_clockwise.as_ref().unwrap_or(&vec![]).clone(),
            on_clockwise_start: on_clockwise_start.as_ref().unwrap_or(&vec![]).clone(),
            on_clockwise_stop: on_clockwise_stop.as_ref().unwrap_or(&vec![]).clone(),
            on_counterclockwise: on_counterclockwise.as_ref().unwrap_or(&vec![]).clone(),
            on_counterclockwise_start: on_counterclockwise_start.as_ref().unwrap_or(&vec![]).clone(),
            on_counterclockwise_stop: on_counterclockwise_stop.as_ref().unwrap_or(&vec![]).clone(),
            on_show: on_show.as_ref().unwrap_or(&vec![]).clone(),
            on_hide: on_hide.as_ref().unwrap_or(&vec![]).clone(),
            on_press: on_press.as_ref().unwrap_or(&vec![]).clone(),
            on_release: on_release.as_ref().unwrap_or(&vec![]).clone(),
        }),
        Wheel::HighLevel(HighLevelWheel {
            on_clockwise,
            on_clockwise_start,
            on_clockwise_stop,
            on_counterclockwise,
            on_counterclockwise_start,
            on_counterclockwise_stop,
            on_show,
            on_hide,
            on_click,
            on_double_click,
            on_triple_click,
            on_long_press,
        }) => Wheel::HighLevel(HighLevelWheel {
            on_clockwise: on_clockwise.as_ref().unwrap_or(&vec![]).clone(),
            on_clockwise_start: on_clockwise_start.as_ref().unwrap_or(&vec![]).clone(),
            on_clockwise_stop: on_clockwise_stop.as_ref().unwrap_or(&vec![]).clone(),
            on_counterclockwise: on_counterclockwise.as_ref().unwrap_or(&vec![]).clone(),
            on_counterclockwise_start: on_counterclockwise_start.as_ref().unwrap_or(&vec![]).clone(),
            on_counterclockwise_stop: on_counterclockwise_stop.as_ref().unwrap_or(&vec![]).clone(),
            on_show: on_show.as_ref().unwrap_or(&vec![]).clone(),
            on_hide: on_hide.as_ref().unwrap_or(&vec![]).clone(),
            on_click: on_click.as_ref().unwrap_or(&vec![]).clone(),
            on_double_click: on_double_click.as_ref().unwrap_or(&vec![]).clone(),
            on_triple_click: on_triple_click.as_ref().unwrap_or(&vec![]).clone(),
            on_long_press: on_long_press.as_ref().unwrap_or(&vec![]).clone(),
        }),
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

