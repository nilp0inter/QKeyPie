use indexmap::IndexMap;

use crate::actions::{Action, WheelCallback, WheelSetCallback, ButtonSet, ButtonCallback, ButtonSetCallback, ProfileCallback, WheelId, ButtonId, ButtonSetId, ProfileId, MacroId, ActiveCallback};
use crate::actions::NonEnigoAction;
use crate::config::Config;

type Actions = Vec<Action>;

type ButtonSetModel = ButtonSetCallback<ButtonCallback<Actions>,Actions>;
type WheelSetModel = WheelSetCallback<Actions>;
pub type ProfileModel = ProfileCallback<IndexMap<ButtonSetId, ButtonSetModel>, IndexMap<WheelId, WheelSetModel>, Actions>;

#[derive(Debug, Clone)]
pub struct Model {
    pub profiles: IndexMap<ProfileId, ProfileModel>,
}

fn replace_macros(opt: &Option<Actions>, macros: &IndexMap<MacroId, Actions>) -> Actions {
    opt.clone().unwrap_or_default().into_iter().flat_map(|action| {
        match action {
            Action::NonEnigo(NonEnigoAction::Macro(macro_id)) => {
                match macros.get(&macro_id) {
                    Some(actions) => actions.clone(),
                    None => {
                        println!("Macro {} not found", macro_id);
                        Vec::new()
                    },
                }
            },
            _ => vec![action],
        }
    }).collect()
}

fn get_button_by_id(cfg: &Config, id: &ButtonId, macros: &IndexMap<MacroId, Actions>) -> anyhow::Result<ButtonCallback<Actions>> {
    let cfg_button = cfg.buttons.as_ref().and_then(|buttons| buttons.get(id)).ok_or_else(|| anyhow::anyhow!("Button {} not found", id))?;

    let button : ButtonCallback<Actions> = ButtonCallback {
        on_press: replace_macros(&cfg_button.on_press, macros),
        on_release: replace_macros(&cfg_button.on_release, macros),
        on_click_press: replace_macros(&cfg_button.on_click_press, macros),
        on_click: replace_macros(&cfg_button.on_click, macros),
        on_click_release: replace_macros(&cfg_button.on_click_release, macros),
        on_double_click_press: replace_macros(&cfg_button.on_double_click_press, macros),
        on_double_click: replace_macros(&cfg_button.on_double_click, macros),
        on_double_click_release: replace_macros(&cfg_button.on_double_click_release, macros),
        on_triple_click_press: replace_macros(&cfg_button.on_triple_click_press, macros),
        on_triple_click: replace_macros(&cfg_button.on_triple_click, macros),
        on_triple_click_release: replace_macros(&cfg_button.on_triple_click_release, macros),
        on_long_press: replace_macros(&cfg_button.on_long_press, macros),
        active: ActiveCallback {
            on_enter: replace_macros(&cfg_button.active.on_enter, macros),
            on_exit: replace_macros(&cfg_button.active.on_exit, macros),
        },
    };
    Ok(button)
}

fn get_button(cfg: &Config, id: &Option<ButtonId>, macros: &IndexMap<MacroId, Actions>) -> anyhow::Result<ButtonCallback<Actions>> {
    match id {
        Some(id) => get_button_by_id(cfg, id, macros),
        None => Ok(ButtonCallback::default()),
    }
}

fn get_wheel(cfg: &Config, id: &WheelId, macros: &IndexMap<MacroId, Actions>) -> anyhow::Result<WheelSetCallback<Actions>> {
    let cfg_wheel = cfg.wheels.as_ref().and_then(|wheels| wheels.get(id)).ok_or_else(|| anyhow::anyhow!("Wheel {} not found", id))?;

    let wheel = WheelSetCallback {
        wheel: WheelCallback {
            on_clockwise: replace_macros(&cfg_wheel.wheel.on_clockwise, macros),
            on_clockwise_start: replace_macros(&cfg_wheel.wheel.on_clockwise_start, macros),
            on_clockwise_stop: replace_macros(&cfg_wheel.wheel.on_clockwise_stop, macros),
            on_counterclockwise: replace_macros(&cfg_wheel.wheel.on_counterclockwise, macros),
            on_counterclockwise_start: replace_macros(&cfg_wheel.wheel.on_counterclockwise_start, macros),
            on_counterclockwise_stop: replace_macros(&cfg_wheel.wheel.on_counterclockwise_stop, macros),
            button: ButtonCallback {
                on_press: replace_macros(&cfg_wheel.wheel.button.on_press, macros),
                on_release: replace_macros(&cfg_wheel.wheel.button.on_release, macros),
                on_click_press: replace_macros(&cfg_wheel.wheel.button.on_click_press, macros),
                on_click: replace_macros(&cfg_wheel.wheel.button.on_click, macros),
                on_click_release: replace_macros(&cfg_wheel.wheel.button.on_click_release, macros),
                on_double_click_press: replace_macros(&cfg_wheel.wheel.button.on_double_click_press, macros),
                on_double_click: replace_macros(&cfg_wheel.wheel.button.on_double_click, macros),
                on_double_click_release: replace_macros(&cfg_wheel.wheel.button.on_double_click_release, macros),
                on_triple_click_press: replace_macros(&cfg_wheel.wheel.button.on_triple_click_press, macros),
                on_triple_click: replace_macros(&cfg_wheel.wheel.button.on_triple_click, macros),
                on_triple_click_release: replace_macros(&cfg_wheel.wheel.button.on_triple_click_release, macros),
                on_long_press: replace_macros(&cfg_wheel.wheel.button.on_long_press, macros),
                active: ActiveCallback {
                    on_enter: replace_macros(&cfg_wheel.wheel.button.active.on_enter, macros),
                    on_exit: replace_macros(&cfg_wheel.wheel.button.active.on_exit, macros),
                },
            },
        },
        active: ActiveCallback {
            on_enter: replace_macros(&cfg_wheel.active.on_enter, macros),
            on_exit: replace_macros(&cfg_wheel.active.on_exit, macros),
        },
    };

    Ok(wheel)
}

fn get_buttonset(cfg: &Config, id: &ButtonSetId, macros: &IndexMap<MacroId, Actions>) -> anyhow::Result<ButtonSetCallback<ButtonCallback<Actions>,Actions>> {
    let cfg_buttonset = cfg.buttonsets.as_ref().and_then(|buttonsets| buttonsets.get(id)).ok_or_else(|| anyhow::anyhow!("Buttonset {} not found", id))?;

    let buttonset = ButtonSetCallback {
        buttonset: ButtonSet {
            button0: get_button(cfg, &cfg_buttonset.buttonset.button0, macros)?,
            button1: get_button(cfg, &cfg_buttonset.buttonset.button1, macros)?,
            button2: get_button(cfg, &cfg_buttonset.buttonset.button2, macros)?,
            button3: get_button(cfg, &cfg_buttonset.buttonset.button3, macros)?,
            button4: get_button(cfg, &cfg_buttonset.buttonset.button4, macros)?,
            button5: get_button(cfg, &cfg_buttonset.buttonset.button5, macros)?,
            button6: get_button(cfg, &cfg_buttonset.buttonset.button6, macros)?,
            button7: get_button(cfg, &cfg_buttonset.buttonset.button7, macros)?,
            button_extra: get_button(cfg, &cfg_buttonset.buttonset.button_extra, macros)?,
        },
        active: ActiveCallback {
            on_enter: replace_macros(&cfg_buttonset.active.on_enter, macros),
            on_exit: replace_macros(&cfg_buttonset.active.on_exit, macros),
        },
    };

    Ok(buttonset)
}

pub fn from_config(cfg: Config) -> anyhow::Result<Model> {
    let mut profiles = IndexMap::new();

    for (cfg_profile_name, cfg_profile) in cfg.clone().profiles.unwrap_or_default() {
        let mut buttonsets = IndexMap::new();
        let mut wheels = IndexMap::new();
        let mut macros = IndexMap::new();

        for (cfg_macro_name, cfg_macro_actions) in cfg.macros.clone().unwrap_or_default() {
            macros.insert(cfg_macro_name, cfg_macro_actions.unwrap_or_default());
        }

        for (cfg_buttonset_name, cfg_buttonset_id) in cfg_profile.buttonsets.unwrap_or_default() {
            let buttonset = get_buttonset(&cfg.clone(), &cfg_buttonset_id, &macros)?;
            buttonsets.insert(cfg_buttonset_name, buttonset);
        }

        for (cfg_wheel_name, cfg_wheel_id) in cfg_profile.wheels.unwrap_or_default() {
            let wheel = get_wheel(&cfg, &cfg_wheel_id, &macros)?;
            wheels.insert(cfg_wheel_name, wheel);
        }

        profiles.insert(cfg_profile_name, ProfileModel {
            buttonsets,
            wheels,
            active: ActiveCallback {
                on_enter: replace_macros(&cfg_profile.active.on_enter, &macros),
                on_exit: replace_macros(&cfg_profile.active.on_exit, &macros),
            },
        });
    }

    Ok(Model {
        profiles,
    })
}

