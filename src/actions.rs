use xencelabs_quick_keys;
use std::time::Instant;

use enigo::agent;

use serde::Serialize;
use serde::Deserialize;

use crate::events;

pub type ButtonId = String;
pub type WheelId = String;
pub type ButtonSetId = String;
pub type ProfileId = String;

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
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ChangeRef {
    Next,
    Previous,
    First,
    Last,
    Name(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NonEnigoAction {
    Sleep(u64),
    SetButtonText(WhichButton, String),
    SetWheelColor(u8, u8, u8),
    ShowBanner(u8, String),
    ChangeProfile(ChangeRef),
    ChangeWheel(ChangeRef),
    ChangeButtonSet(ChangeRef),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Action {
    Input(agent::Token),
    NonEnigo(NonEnigoAction),
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct LowLevelButton<T> {
    pub on_press: T,
    pub on_release: T,
    pub on_show: T,
    pub on_hide: T,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct HighLevelButton<T> {
    pub on_click: T,
    pub on_double_click: T,
    pub on_triple_click: T,
    pub on_long_press: T,
    pub on_show: T,
    pub on_hide: T,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Button<T> {
    LowLevel(LowLevelButton<T>),
    HighLevel(HighLevelButton<T>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct LowLevelWheel<T> {
    pub on_clockwise: T,
    pub on_clockwise_start: T,
    pub on_clockwise_stop: T,
    pub on_counterclockwise: T,
    pub on_counterclockwise_start: T,
    pub on_counterclockwise_stop: T,
    pub on_show: T,
    pub on_hide: T,
    pub on_press: T,
    pub on_release: T,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct HighLevelWheel<T> {
    pub on_clockwise: T,
    pub on_clockwise_start: T,
    pub on_clockwise_stop: T,
    pub on_counterclockwise: T,
    pub on_counterclockwise_start: T,
    pub on_counterclockwise_stop: T,
    pub on_show: T,
    pub on_hide: T,
    pub on_click: T,
    pub on_double_click: T,
    pub on_triple_click: T,
    pub on_long_press: T,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Wheel<T> {
    LowLevel(LowLevelWheel<T>),
    HighLevel(HighLevelWheel<T>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ButtonSet<T1, T2> {
    pub button0: T1,
    pub button1: T1,
    pub button2: T1,
    pub button3: T1,
    pub button4: T1,
    pub button5: T1,
    pub button6: T1,
    pub button7: T1,
    pub button_extra: T2,
}

impl<T1, T2> Default for ButtonSet<T1, T2>
where T1: Default, T2: Default {
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

impl From<xencelabs_quick_keys::ButtonState> for ButtonSet<events::ButtonState, events::ButtonState> {
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

impl ButtonSet<events::ClickStateMachine, events::ClickStateMachine> {
    pub fn transition(&self, event: ButtonSet<events::ButtonState, events::ButtonState>, when: Instant) -> (Self, ButtonSet<Vec<events::Event>, Vec<events::Event>>) {
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
