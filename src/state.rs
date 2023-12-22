use anyhow::Error;

use crate::events;
use crate::actions;
use crate::model;

#[derive(Debug, Clone)]
pub struct State {
    pub current_profile_id: String,
    pub current_profile_index: usize,
    pub current_buttonset_id: String,
    pub current_buttonset_index: usize,
    pub current_wheel_id: String,
    pub current_wheel_index: usize,
    pub model: model::Model,
    pub button_state: actions::ButtonSet<events::ButtonStateMachine>,
    pub wheel_state: actions::WheelSet<events::WheelStateMachine, events::ButtonStateMachine>,
}

impl State {
    pub fn new(model: model::Model) -> Result<State, anyhow::Error> {
        let (profile_id, profile) = model.profiles.first().ok_or(Error::msg("No profiles"))?;
        let (buttonset_id, _) = profile.buttonsets.first().ok_or(Error::msg("No buttonsets"))?;
        let (wheel_id, _) = profile.wheels.first().ok_or(Error::msg("No wheels"))?;
        Ok(State {
            current_profile_id: profile_id.clone(),
            current_profile_index: 0,
            current_buttonset_id: buttonset_id.clone(),
            current_buttonset_index: 0,
            current_wheel_id: wheel_id.clone(),
            current_wheel_index: 0,
            model,
            button_state: actions::ButtonSet::default(),
            wheel_state: actions::WheelSet::default(),
        })
    }

    pub fn get_current_profile(&self) -> &model::Profile {
        self.model.profiles.get(&self.current_profile_id).unwrap()
    }

    pub fn get_current_buttonset(&self) -> &actions::ButtonSetCallback<actions::ButtonCallback<Vec<actions::Action>>,Vec<actions::Action>> {
        self.get_current_profile().buttonsets.get(&self.current_buttonset_id).unwrap()
    }

    pub fn get_current_wheel(&self) -> &actions::WheelSetCallback<Vec<actions::Action>> {
        self.get_current_profile().wheels.get(&self.current_wheel_id).unwrap()
    }

    pub fn process_goto(&self, goto: actions::GoTo) -> Result<Self, anyhow::Error> {
        let mut state = self.clone();
        match goto.profile {
            actions::ChangeRef::Next => {
                let next_index = (state.current_profile_index + 1) % state.model.profiles.len();
                let (profile_id, _) = state.model.profiles.get_index(next_index).ok_or(Error::msg("No profiles"))?;
                state.current_profile_id = profile_id.clone();
                state.current_profile_index = next_index;
            },
            actions::ChangeRef::Previous => {
                let next_index = if state.current_profile_index == 0 {
                    state.model.profiles.len() - 1
                } else {
                    state.current_profile_index - 1
                };
                let (profile_id, _) = state.model.profiles.get_index(next_index).ok_or(Error::msg("No profiles"))?;
                state.current_profile_id = profile_id.clone();
                state.current_profile_index = next_index;
            },
            actions::ChangeRef::First => {
                let (profile_id, _) = state.model.profiles.first().ok_or(Error::msg("No profiles"))?;
                state.current_profile_id = profile_id.clone();
                state.current_profile_index = 0;
            },
            actions::ChangeRef::Last => {
                let (profile_id, _) = state.model.profiles.last().ok_or(Error::msg("No profiles"))?;
                state.current_profile_id = profile_id.clone();
                state.current_profile_index = state.model.profiles.len() - 1;
            },
            actions::ChangeRef::Name(name) => {
                let profile_id = name;
                state.current_profile_id = profile_id.clone();
                state.current_profile_index = state.model.profiles.get_index_of(&profile_id).ok_or(Error::msg(format!("Profile {} not found", profile_id)))?;
            },
            actions::ChangeRef::This => {},
        }
        match goto.buttonset {
            actions::ChangeRef::Next => {
                let next_index = (state.current_buttonset_index + 1) % state.get_current_profile().buttonsets.len();
                let (buttonset_id, _) = state.get_current_profile().buttonsets.get_index(next_index).ok_or(Error::msg("No buttonsets"))?;
                state.current_buttonset_id = buttonset_id.clone();
                state.current_buttonset_index = next_index;
            },
            actions::ChangeRef::Previous => {
                let next_index = if state.current_buttonset_index == 0 {
                    state.get_current_profile().buttonsets.len() - 1
                } else {
                    state.current_buttonset_index - 1
                };
                let (buttonset_id, _) = state.get_current_profile().buttonsets.get_index(next_index).ok_or(Error::msg("No buttonsets"))?;
                state.current_buttonset_id = buttonset_id.clone();
                state.current_buttonset_index = next_index;
            },
            actions::ChangeRef::First => {
                let (buttonset_id, _) = state.get_current_profile().buttonsets.first().ok_or(Error::msg("No buttonsets"))?;
                state.current_buttonset_id = buttonset_id.clone();
                state.current_buttonset_index = 0;
            },
            actions::ChangeRef::Last => {
                let (buttonset_id, _) = state.get_current_profile().buttonsets.last().ok_or(Error::msg("No buttonsets"))?;
                state.current_buttonset_id = buttonset_id.clone();
                state.current_buttonset_index = state.get_current_profile().buttonsets.len() - 1;
            },
            actions::ChangeRef::Name(name) => {
                let buttonset_id = name;
                state.current_buttonset_id = buttonset_id.clone();
                state.current_buttonset_index = state.get_current_profile().buttonsets.get_index_of(&buttonset_id).ok_or(Error::msg(format!("Buttonset {} not found", buttonset_id)))?;
            },
            actions::ChangeRef::This => {},
        }
        match goto.wheel {
            actions::ChangeRef::Next => {
                let next_index = (state.current_wheel_index + 1) % state.get_current_profile().wheels.len();
                let (wheel_id, _) = state.get_current_profile().wheels.get_index(next_index).ok_or(Error::msg("No wheels"))?;
                state.current_wheel_id = wheel_id.clone();
                state.current_wheel_index = next_index;
            },
            actions::ChangeRef::Previous => {
                let next_index = if state.current_wheel_index == 0 {
                    state.get_current_profile().wheels.len() - 1
                } else {
                    state.current_wheel_index - 1
                };
                let (wheel_id, _) = state.get_current_profile().wheels.get_index(next_index).ok_or(Error::msg("No wheels"))?;
                state.current_wheel_id = wheel_id.clone();
                state.current_wheel_index = next_index;
            },
            actions::ChangeRef::First => {
                let (wheel_id, _) = state.get_current_profile().wheels.first().ok_or(Error::msg("No wheels"))?;
                state.current_wheel_id = wheel_id.clone();
                state.current_wheel_index = 0;
            },
            actions::ChangeRef::Last => {
                let (wheel_id, _) = state.get_current_profile().wheels.last().ok_or(Error::msg("No wheels"))?;
                state.current_wheel_id = wheel_id.clone();
                state.current_wheel_index = state.get_current_profile().wheels.len() - 1;
            },
            actions::ChangeRef::Name(name) => {
                let wheel_id = name;
                state.current_wheel_id = wheel_id.clone();
                state.current_wheel_index = state.get_current_profile().wheels.get_index_of(&wheel_id).ok_or(Error::msg(format!("Wheel {} not found", wheel_id)))?;
            },
            actions::ChangeRef::This => {},
        }
        Ok(state)
    }
}
