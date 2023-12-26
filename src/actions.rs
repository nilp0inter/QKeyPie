use std::time::Instant;

use enigo::agent;

use serde::Serialize;
use serde::Deserialize;

use crate::events;

pub type ButtonId = String;
pub type WheelId = String;
pub type ButtonSetId = String;
pub type ProfileId = String;
pub type MacroId = String;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WhichButton {
    ThisButton,
    Button0,
    Button1,
    Button2,
    Button3,
    Button4,
    Button5,
    Button6,
    Button7,
    ButtonExtra,
    WheelButton,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ChangeRef {
    Next,
    Previous,
    First,
    Last,
    Name(String),
    This,
}

#[derive(Debug, Clone)]
pub struct GoTo {
    pub profile: ChangeRef,
    pub buttonset: ChangeRef,
    pub wheel: ChangeRef,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NonEnigoAction {
    Sleep(u64),
    SetButtonText(WhichButton, String),
    SetWheelColor(u8, u8, u8),
    ShowBanner(u8, String),
    ChangeProfile(ChangeRef, ChangeRef, ChangeRef),
    ChangeWheel(ChangeRef),
    ChangeButtonSet(ChangeRef),
    Macro(MacroId),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Action {
    Input(agent::Token),
    NonEnigo(NonEnigoAction),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ButtonCallback<T> {
    pub on_click: T,
    pub on_double_click: T,
    pub on_triple_click: T,
    pub on_long_press: T,
    pub on_press: T,
    pub on_release: T,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WheelCallback<T> {
    pub on_clockwise: T,
    pub on_clockwise_start: T,
    pub on_clockwise_stop: T,
    pub on_counterclockwise: T,
    pub on_counterclockwise_start: T,
    pub on_counterclockwise_stop: T,
    #[serde(flatten)]
    pub button: ButtonCallback<T>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WheelSetCallback<T> {
    #[serde(flatten)]
    pub wheel: WheelCallback<T>,
    #[serde(flatten)]
    pub active: ActiveCallback<T>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActiveCallback<T> {
    pub on_enter: T,
    pub on_exit: T,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ButtonSet<T> {
    pub button0: T,
    pub button1: T,
    pub button2: T,
    pub button3: T,
    pub button4: T,
    pub button5: T,
    pub button6: T,
    pub button7: T,
    pub button_extra: T,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ButtonSetCallback<T1, T2> {
    #[serde(flatten)]
    pub buttonset: ButtonSet<T1>,
    #[serde(flatten)]
    pub active: ActiveCallback<T2>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct WheelSet<T1, T2> {
    pub wheel: T1,
    pub wheel_button: T2,
}

impl<T> Default for ButtonSet<T>
where T: Default {
    fn default() -> Self {
        ButtonSet {
            button0: Default::default(),
            button1: Default::default(),
            button2: Default::default(),
            button3: Default::default(),
            button4: Default::default(),
            button5: Default::default(),
            button6: Default::default(),
            button7: Default::default(),
            button_extra: Default::default(),
        }
    }
}

impl<T1, T2> Default for WheelSet<T1, T2>
where T1: Default, T2: Default {
    fn default() -> Self {
        WheelSet {
            wheel: Default::default(),
            wheel_button: Default::default(),
        }
    }
}

impl From<xencelabs_quick_keys::ButtonState> for ButtonSet<events::ButtonState> {
    fn from(b: xencelabs_quick_keys::ButtonState) -> Self {
        ButtonSet {
            button0: b.button_0.into(),
            button1: b.button_1.into(),
            button2: b.button_2.into(),
            button3: b.button_3.into(),
            button4: b.button_4.into(),
            button5: b.button_5.into(),
            button6: b.button_6.into(),
            button7: b.button_7.into(),
            button_extra: b.button_extra.into(),
        }
    }
}

impl From<xencelabs_quick_keys::Event> for ButtonSet<events::ButtonState> {
    fn from(b: xencelabs_quick_keys::Event) -> Self {
        match b {
            xencelabs_quick_keys::Event::Button { state } => state.into(),
            _ => ButtonSet {
                button0: events::ButtonState::Unknown,
                button1: events::ButtonState::Unknown,
                button2: events::ButtonState::Unknown,
                button3: events::ButtonState::Unknown,
                button4: events::ButtonState::Unknown,
                button5: events::ButtonState::Unknown,
                button6: events::ButtonState::Unknown,
                button7: events::ButtonState::Unknown,
                button_extra: events::ButtonState::Unknown,
            },
        }
    }
}

impl From<xencelabs_quick_keys::Event> for WheelSet<events::WheelState, events::ButtonState> {
    fn from(b: xencelabs_quick_keys::Event) -> Self {
        match b {
            xencelabs_quick_keys::Event::Wheel { direction } => WheelSet {
                wheel: match direction {
                    xencelabs_quick_keys::WheelDirection::Right => events::WheelState::RotatingClockwise,
                    xencelabs_quick_keys::WheelDirection::Left => events::WheelState::RotatingCounterClockwise
                },
                wheel_button: events::ButtonState::Unknown,
            },
            xencelabs_quick_keys::Event::Button { state } => WheelSet {
                wheel: events::WheelState::Unknown,
                wheel_button: state.button_wheel.into(),
            },
            _ => WheelSet {
                wheel: events::WheelState::Unknown,
                wheel_button: events::ButtonState::Unknown,
            },
        }
    }
}

impl ButtonSet<events::ButtonStateMachine> {
    pub fn transition(&self, event: ButtonSet<events::ButtonState>, when: Instant) -> (Self, ButtonSet<Vec<events::ButtonEvent>>) {
        let (button0_new_state, button0_events) = self.button0.transition(event.button0, when);
        let (button1_new_state, button1_events) = self.button1.transition(event.button1, when);
        let (button2_new_state, button2_events) = self.button2.transition(event.button2, when);
        let (button3_new_state, button3_events) = self.button3.transition(event.button3, when);
        let (button4_new_state, button4_events) = self.button4.transition(event.button4, when);
        let (button5_new_state, button5_events) = self.button5.transition(event.button5, when);
        let (button6_new_state, button6_events) = self.button6.transition(event.button6, when);
        let (button7_new_state, button7_events) = self.button7.transition(event.button7, when);
        let (button_extra_new_state, button_extra_events) = self.button_extra.transition(event.button_extra, when);
        (ButtonSet {
            button0: button0_new_state,
            button1: button1_new_state,
            button2: button2_new_state,
            button3: button3_new_state,
            button4: button4_new_state,
            button5: button5_new_state,
            button6: button6_new_state,
            button7: button7_new_state,
            button_extra: button_extra_new_state,
        }, ButtonSet {
            button0: button0_events,
            button1: button1_events,
            button2: button2_events,
            button3: button3_events,
            button4: button4_events,
            button5: button5_events,
            button6: button6_events,
            button7: button7_events,
            button_extra: button_extra_events,
        })
    }
}

impl WheelSet<events::WheelStateMachine, events::ButtonStateMachine> {
    pub fn transition(&self, event: WheelSet<events::WheelState, events::ButtonState>, when: Instant) -> (Self, WheelSet<Vec<events::WheelEvent>, Vec<events::ButtonEvent>>) {
        let (wheel_new_state, wheel_events) = self.wheel.transition(event.wheel, when);
        let (wheel_button_new_state, wheel_button_events) = self.wheel_button.transition(event.wheel_button, when);
        (WheelSet {
            wheel: wheel_new_state,
            wheel_button: wheel_button_new_state,
        }, WheelSet {
            wheel: wheel_events,
            wheel_button: wheel_button_events,
        })
    }
}

impl Default for ActiveCallback<Vec<Action>> {
    fn default() -> Self {
        ActiveCallback {
            on_enter: vec![],
            on_exit: vec![],
        }
    }
}

impl Default for ButtonCallback<Vec<Action>> {
    fn default() -> Self {
        ButtonCallback {
            on_click: vec![],
            on_double_click: vec![],
            on_triple_click: vec![],
            on_long_press: vec![],
            on_press: vec![],
            on_release: vec![],
        }
    }
}
