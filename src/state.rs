use anyhow::Error;

use crate::events;
use crate::actions;
use crate::model;

#[derive(Debug, Clone)]
pub struct State {
    pub current_profile_id: String,
    pub current_profile_index: usize,
    pub last_profile_id: Option<String>,
    pub last_profile_index: Option<usize>,

    pub current_buttonset_id: String,
    pub current_buttonset_index: usize,
    pub last_buttonset_id: Option<String>,
    pub last_buttonset_index: Option<usize>,

    pub current_wheel_id: String,
    pub current_wheel_index: usize,
    pub last_wheel_id: Option<String>,
    pub last_wheel_index: Option<usize>,

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
            last_profile_id: None,
            last_profile_index: None,
            current_buttonset_id: buttonset_id.clone(),
            current_buttonset_index: 0,
            last_buttonset_id: None,
            last_buttonset_index: None,
            current_wheel_id: wheel_id.clone(),
            current_wheel_index: 0,
            last_wheel_id: None,
            last_wheel_index: None,
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

        let current_profile_id = state.current_profile_id.clone();
        let current_profile_index = state.current_profile_index;
        let current_buttonset_id = state.current_buttonset_id.clone();
        let current_buttonset_index = state.current_buttonset_index;
        let current_wheel_id = state.current_wheel_id.clone();
        let current_wheel_index = state.current_wheel_index;

        // Profile
        match goto.clone() {
            actions::GoTo::Switch(actions::ChangeRef::Next, _, _) => {
                let next_index = (state.current_profile_index + 1) % state.model.profiles.len();
                let (profile_id, _) = state.model.profiles.get_index(next_index).ok_or(Error::msg("No profiles"))?;
                state.current_profile_id = profile_id.clone();
                state.current_profile_index = next_index;
            },
            actions::GoTo::Switch(actions::ChangeRef::Previous, _, _) => {
                let next_index = if state.current_profile_index == 0 {
                    state.model.profiles.len() - 1
                } else {
                    state.current_profile_index - 1
                };
                let (profile_id, _) = state.model.profiles.get_index(next_index).ok_or(Error::msg("No profiles"))?;
                state.current_profile_id = profile_id.clone();
                state.current_profile_index = next_index;
            },
            actions::GoTo::Switch(actions::ChangeRef::First, _, _) => {
                let (profile_id, _) = state.model.profiles.first().ok_or(Error::msg("No profiles"))?;
                state.current_profile_id = profile_id.clone();
                state.current_profile_index = 0;
            },
            actions::GoTo::Switch(actions::ChangeRef::Last, _, _) => {
                let (profile_id, _) = state.model.profiles.last().ok_or(Error::msg("No profiles"))?;
                state.current_profile_id = profile_id.clone();
                state.current_profile_index = state.model.profiles.len() - 1;
            },
            actions::GoTo::Switch(actions::ChangeRef::Name(name), _, _) => {
                let profile_id = name;
                state.current_profile_id = profile_id.clone();
                state.current_profile_index = state.model.profiles.get_index_of(&profile_id).ok_or(Error::msg(format!("Profile {} not found", profile_id)))?;
            },
            actions::GoTo::Switch(actions::ChangeRef::This, _, _) => {},
            actions::GoTo::Swap => {
                state.current_profile_id = state.last_profile_id.clone().unwrap_or(current_profile_id.clone());
                state.current_profile_index = state.last_profile_index.unwrap_or(current_profile_index);
            },
        }
        // Buttonset
        match goto.clone() {
            actions::GoTo::Switch(_, actions::ChangeRef::Next, _) => {
                let next_index = (state.current_buttonset_index + 1) % state.get_current_profile().buttonsets.len();
                let (buttonset_id, _) = state.get_current_profile().buttonsets.get_index(next_index).ok_or(Error::msg("No buttonsets"))?;
                state.current_buttonset_id = buttonset_id.clone();
                state.current_buttonset_index = next_index;
            },
            actions::GoTo::Switch(_, actions::ChangeRef::Previous, _) => {
                let next_index = if state.current_buttonset_index == 0 {
                    state.get_current_profile().buttonsets.len() - 1
                } else {
                    state.current_buttonset_index - 1
                };
                let (buttonset_id, _) = state.get_current_profile().buttonsets.get_index(next_index).ok_or(Error::msg("No buttonsets"))?;
                state.current_buttonset_id = buttonset_id.clone();
                state.current_buttonset_index = next_index;
            },
            actions::GoTo::Switch(_, actions::ChangeRef::First, _) => {
                let (buttonset_id, _) = state.get_current_profile().buttonsets.first().ok_or(Error::msg("No buttonsets"))?;
                state.current_buttonset_id = buttonset_id.clone();
                state.current_buttonset_index = 0;
            },
            actions::GoTo::Switch(_, actions::ChangeRef::Last, _) => {
                let (buttonset_id, _) = state.get_current_profile().buttonsets.last().ok_or(Error::msg("No buttonsets"))?;
                state.current_buttonset_id = buttonset_id.clone();
                state.current_buttonset_index = state.get_current_profile().buttonsets.len() - 1;
            },
            actions::GoTo::Switch(_, actions::ChangeRef::Name(name), _) => {
                let buttonset_id = name;
                state.current_buttonset_id = buttonset_id.clone();
                state.current_buttonset_index = state.get_current_profile().buttonsets.get_index_of(&buttonset_id).ok_or(Error::msg(format!("Buttonset {} not found", buttonset_id)))?;
            },
            actions::GoTo::Switch(_, actions::ChangeRef::This, _) => {},
            actions::GoTo::Swap => {
                state.current_buttonset_id = state.last_buttonset_id.clone().unwrap_or(current_buttonset_id.clone());
                state.current_buttonset_index = state.last_buttonset_index.unwrap_or(current_buttonset_index);
            },
        }
        // Wheel
        match goto {
            actions::GoTo::Switch(_, _, actions::ChangeRef::Next) => {
                let next_index = (state.current_wheel_index + 1) % state.get_current_profile().wheels.len();
                let (wheel_id, _) = state.get_current_profile().wheels.get_index(next_index).ok_or(Error::msg("No wheels"))?;
                state.current_wheel_id = wheel_id.clone();
                state.current_wheel_index = next_index;
            },
            actions::GoTo::Switch(_, _, actions::ChangeRef::Previous) => {
                let next_index = if state.current_wheel_index == 0 {
                    state.get_current_profile().wheels.len() - 1
                } else {
                    state.current_wheel_index - 1
                };
                let (wheel_id, _) = state.get_current_profile().wheels.get_index(next_index).ok_or(Error::msg("No wheels"))?;
                state.current_wheel_id = wheel_id.clone();
                state.current_wheel_index = next_index;
            },
            actions::GoTo::Switch(_, _, actions::ChangeRef::First) => {
                let (wheel_id, _) = state.get_current_profile().wheels.first().ok_or(Error::msg("No wheels"))?;
                state.current_wheel_id = wheel_id.clone();
                state.current_wheel_index = 0;
            },
            actions::GoTo::Switch(_, _, actions::ChangeRef::Last) => {
                let (wheel_id, _) = state.get_current_profile().wheels.last().ok_or(Error::msg("No wheels"))?;
                state.current_wheel_id = wheel_id.clone();
                state.current_wheel_index = state.get_current_profile().wheels.len() - 1;
            },
            actions::GoTo::Switch(_, _, actions::ChangeRef::Name(name)) => {
                let wheel_id = name;
                state.current_wheel_id = wheel_id.clone();
                state.current_wheel_index = state.get_current_profile().wheels.get_index_of(&wheel_id).ok_or(Error::msg(format!("Wheel {} not found", wheel_id)))?;
            },
            actions::GoTo::Switch(_, _, actions::ChangeRef::This) => {},
            actions::GoTo::Swap => {
                state.current_wheel_id = state.last_wheel_id.clone().unwrap_or(current_wheel_id.clone());
                state.current_wheel_index = state.last_wheel_index.unwrap_or(current_wheel_index);
            },
        }
        state.last_profile_id = Some(current_profile_id);
        state.last_profile_index = Some(current_profile_index);
        state.last_buttonset_id = Some(current_buttonset_id);
        state.last_buttonset_index = Some(current_buttonset_index);
        state.last_wheel_id = Some(current_wheel_id);
        state.last_wheel_index = Some(current_wheel_index);

        Ok(state)
    }
}
