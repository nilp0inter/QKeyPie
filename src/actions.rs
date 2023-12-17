use enigo::agent;

use serde::Serialize;
use serde::Deserialize;

pub type ButtonId = String;
pub type WheelId = String;
pub type ButtonSetId = String;
pub type ProfileId = String;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WhichButton {
    ThisButton,
    Button1,
    Button2,
    Button3,
    Button4,
    Button5,
    Button6,
    Button7,
    Button8,
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
    SetButtonText(WhichButton),
    SetWheelColor(u8, u8, u8),
    ShowBanner(u8, String),
    ChangeWheel(ChangeRef),
    ChangeButtonSet(ChangeRef),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Action {
    Input(agent::Token),
    NonEnigo(NonEnigoAction),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LowLevelButtonInteraction<T> {
    pub on_press: T,
    pub on_release: T,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HighLevelButtonInteraction<T> {
    pub on_click: T,
    pub on_double_click: T,
    pub on_triple_click: T,
    pub on_long_press: T,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ButtonInteractionType<T> {
    LowLevel(LowLevelButtonInteraction<T>),
    HighLevel(HighLevelButtonInteraction<T>),
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Button<T> {
   pub interaction: ButtonInteractionType<T>,
   pub on_show: T,
   pub on_hide: T,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Wheel<T> {
    pub on_clockwise: T,
    pub on_clockwise_start: T,
    pub on_clockwise_stop: T,
    pub on_counterclockwise: T,
    pub on_counterclockwise_start: T,
    pub on_counterclockwise_stop: T,
    pub on_show: T,
    pub on_hide: T,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ButtonSet<T1, T2> {
    pub button1: T1,
    pub button2: T1,
    pub button3: T1,
    pub button4: T1,
    pub button5: T1,
    pub button6: T1,
    pub button7: T1,
    pub button8: T1,
    pub button9: T2,
}
