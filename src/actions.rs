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
pub enum GoTo {
    Swap,
    Switch(ChangeRef, ChangeRef, ChangeRef),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NonEnigoAction {
    // QKeyPie side-effects
    Debug(String),
    Run(Vec<String>),
    Sleep(u64),

    // QuickKeys state
    SetScreenOrientation(xencelabs_quick_keys::ScreenOrientation),
    SetScreenBrightness(xencelabs_quick_keys::ScreenBrightness),
    SetWheelSpeed(xencelabs_quick_keys::WheelSpeed),
    SetSleepTimeout(u8),
    SetButtonText(WhichButton, String),
    SetWheelColor(u8, u8, u8),
    ShowBanner(u8, String),

    // QKeyPie state
    ChangeProfile(ChangeRef, ChangeRef, ChangeRef),
    ChangeWheel(ChangeRef),
    ChangeButtonSet(ChangeRef),
    Swap,

    // QKeyPie config
    Macro(MacroId),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Action {
    Input(agent::Token),
    NonEnigo(NonEnigoAction),
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct ButtonCallback<T> {
    pub on_press: T,
    pub on_release: T,

    pub on_click_press: T,
    pub on_click: T,
    pub on_click_release: T,

    pub on_double_click_press: T,
    pub on_double_click: T,
    pub on_double_click_release: T,

    pub on_triple_click_press: T,
    pub on_triple_click: T,
    pub on_triple_click_release: T,

    pub on_long_press: T,

    #[serde(flatten)]
    pub active: ActiveCallback<T>,
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

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
pub struct ActiveCallback<T> {
    pub on_enter: T,
    pub on_exit: T,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ButtonSet<T> {
    pub button0: T,
    pub button1: T,
    pub button2: T,
    pub button3: T,
    pub button4: T,
    pub button5: T,
    pub button6: T,
    pub button7: T,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ButtonSetCallback<T1, T2> {
    #[serde(flatten)]
    pub buttonset: ButtonSet<T1>,
    #[serde(flatten)]
    pub active: ActiveCallback<T2>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct WheelSet<T1, T2> {
    pub wheel: T1,
    pub wheel_button: T2,
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
        (ButtonSet {
            button0: button0_new_state,
            button1: button1_new_state,
            button2: button2_new_state,
            button3: button3_new_state,
            button4: button4_new_state,
            button5: button5_new_state,
            button6: button6_new_state,
            button7: button7_new_state,
        }, ButtonSet {
            button0: button0_events,
            button1: button1_events,
            button2: button2_events,
            button3: button3_events,
            button4: button4_events,
            button5: button5_events,
            button6: button6_events,
            button7: button7_events,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ProfileCallback<T1, T2, T3, T4> {
    pub buttonsets: T1,
    pub wheels: T2,
    pub button: T3,
    #[serde(flatten)]
    pub active: ActiveCallback<T4>,
}
