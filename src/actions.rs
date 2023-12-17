use enigo::agent;

use serde::Serialize;
use serde::Deserialize;

pub type ButtonId = String;
pub type WheelId = String;
pub type ButtonSetId = String;
pub type ProfileId = String;

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub enum ChangeRef {
    Next,
    Previous,
    First,
    Last,
    Name(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NonEnigoAction {
    Sleep(u64),
    SetButtonText(WhichButton),
    SetWheelColor(u8, u8, u8),
    ShowBanner(u8, String),
    ChangeWheel(ChangeRef),
    ChangeButtonSet(ChangeRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Action {
    Input(agent::Token),
    NonEnigo(NonEnigoAction),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LowLevelButtonInteraction<T> {
    on_press: T,
    on_release: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HighLevelButtonInteraction<T> {
    on_click: T,
    on_double_click: T,
    on_triple_click: T,
    on_long_press: T,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ButtonInteractionType<T> {
    LowLevel(LowLevelButtonInteraction<T>),
    HighLevel(HighLevelButtonInteraction<T>),
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Button<T> {
   interaction: ButtonInteractionType<T>,
   on_show: T,
   on_hide: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Wheel<T> {
    on_clockwise: T,
    on_clockwise_start: T,
    on_clockwise_stop: T,
    on_counterclockwise: T,
    on_counterclockwise_start: T,
    on_counterclockwise_stop: T,
    on_show: T,
    on_hide: T,
}

#[derive(Debug, Serialize, Deserialize)]
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
