use std::fs::File;
use std::io::Read;

use enigo::agent;
use std::collections::HashMap;

use serde::Serialize;
use serde::Deserialize;

type ButtonId = String;
type WheelId = String;
type ButtonSetId = String;
type ProfileId = String;

type Actions = Option<Vec<Action>>;

#[derive(Debug, Serialize, Deserialize)]
struct LowLevelButtonInteraction {
    on_press: Actions,
    on_release: Actions,
}

#[derive(Debug, Serialize, Deserialize)]
struct HighLevelButtonInteraction {
    on_click: Actions,
    on_double_click: Actions,
    on_triple_click: Actions,
    on_long_press: Actions,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum ButtonInteractionType {
    LowLevel(LowLevelButtonInteraction),
    HighLevel(HighLevelButtonInteraction),
}

#[derive(Debug, Serialize, Deserialize)]
enum WhichButton {
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
enum ChangeRef {
    Next,
    Previous,
    First,
    Last,
    Name(String),
}

#[derive(Debug, Serialize, Deserialize)]
enum NonEnigoAction {
    Sleep(u64),
    SetButtonText(WhichButton),
    SetWheelColor(u8, u8, u8),
    ShowBanner(u8, String),
    ChangeWheel(ChangeRef),
    ChangeButtonSet(ChangeRef),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Action {
    Input(agent::Token),
    NonEnigo(NonEnigoAction),
}

#[derive(Debug, Serialize, Deserialize)]
struct Button {
   interaction: ButtonInteractionType,
   on_show: Actions,
   on_hide: Actions,
}

#[derive(Debug, Serialize, Deserialize)]
struct Wheel {
    on_clockwise: Actions,
    on_clockwise_start: Actions,
    on_clockwise_stop: Actions,
    on_counterclockwise: Actions,
    on_counterclockwise_start: Actions,
    on_counterclockwise_stop: Actions,
    on_show: Actions,
    on_hide: Actions,
}

#[derive(Debug, Serialize, Deserialize)]
struct ButtonSet {
    button1: Option<ButtonId>,
    button2: Option<ButtonId>,
    button3: Option<ButtonId>,
    button4: Option<ButtonId>,
    button5: Option<ButtonId>,
    button6: Option<ButtonId>,
    button7: Option<ButtonId>,
    button8: Option<ButtonId>,
    button9: Option<ButtonId>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Profile {
    buttonsets: Option<HashMap<String, ButtonSetId>>,
    wheels: Option<HashMap<String, WheelId>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    buttons: Option<HashMap<ButtonId, Button>>,
    wheels: Option<HashMap<WheelId, Wheel>>,
    buttonsets: Option<HashMap<ButtonSetId, ButtonSet>>,
    profiles: Option<HashMap<ProfileId, Profile>>,
}

pub fn main() {
    let mut file = File::open("config.toml").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let config: Config = toml::from_str(&data).unwrap();

    // thread::sleep(Duration::from_secs(2));
    // let mut enigo = Enigo::new(&Settings::default()).unwrap();

    // let actions : Vec<Action> = vec![
    //     Action::Input(Token::Text("Hello World! ❤️".to_string())),
    //     Action::Input(Token::MoveMouse(10, 10, enigo::Coordinate::Rel)),
    //     Action::Input(Token::Scroll(5, enigo::Axis::Vertical)),
    //     Action::Input(Token::Button(Button::Left, enigo::Direction::Click)),
    //     Action::Input(Token::Key(Key::Unicode('🔥'), enigo::Direction::Click)),
    //     Action::Input(Token::Key(Key::Control, enigo::Direction::Press)),
    //     Action::NonEnigo(NonEnigoAction::Sleep(1000)),
    //     Action::Input(Token::Key(Key::Unicode('a'), enigo::Direction::Click)),
    //     Action::Input(Token::Key(Key::Control, enigo::Direction::Release)),
    //     Action::NonEnigo(NonEnigoAction::ScreenShot(ScreenShotType::Full)),
    //     Action::NonEnigo(NonEnigoAction::Shutdown),
    // ];

    // let panel = PanelConfig {
    //     name: "test".to_string(),
    //     button1: Some(ButtonConfig {
    //         text: "127.0.0.1".to_string(),
    //         actions: actions,
    //     }),
    //     button2: None,
    //     button3: None,
    //     button4: None,
    //     button5: Some(ButtonConfig {
    //         text: "help".to_string(),
    //         actions: vec![
    //             Action::Input(Token::Key(Key::F1, enigo::Direction::Click)),
    //         ],
    //     }),
    //     button6: None,
    //     button7: None,
    //     button8: None,
    // };
    //
    // let config = Config {
    //     buttons: HashMap::new(),
    //     wheels: HashMap::new(),
    //     buttonsets: HashMap::new(),
    //     profiles: HashMap::new(),
    // };
    println!("{:?}", config);


    // let serialized = toml::to_string_pretty(&Config).unwrap();
    // println!("{serialized}");

    // let deserialized_config: Config = toml::from_str(&serialized).unwrap();
    // for token in &deserialized_config.values {
    //     println!("{:?}", enigo.execute(token));
    // }
}
