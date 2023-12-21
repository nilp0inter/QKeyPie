use anyhow::Error;

use crate::model::Model;
use crate::events;
use crate::actions;

pub struct State {
    pub current_profile_id: String,
    pub current_profile_index: usize,
    pub current_buttonset_id: String,
    pub current_buttonset_index: usize,
    pub current_wheel_id: String,
    pub current_wheel_index: usize,
    pub model: Model,
    pub button_state: actions::ButtonSet<events::ButtonStateMachine>,
    pub wheel_state: actions::WheelSet<events::WheelStateMachine, events::ButtonStateMachine>,
}

impl State {
    pub fn new(model: Model) -> Result<State, anyhow::Error> {
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
}
