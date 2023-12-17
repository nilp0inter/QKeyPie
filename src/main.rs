// // use xencelabs_quick_keys::{QKDevice, ConnectionMode, QKResult, ScreenOrientation, ScreenBrightness, WheelSpeed, Event, ButtonState};
// // use hidapi::HidApi;
// // use std::{thread,time};
// // use std::sync::{Arc, Mutex};
// // use enigo::*;

// use serde::Serialize;

// #[derive(Serialize)]
// struct Config {
//    ip: String,
//    port: Option<u16>,
//    keys: Keys,
// }

// #[derive(Serialize)]
// // #[serde(tag = "type", content = "value")]
// // #[serde(untagged)]
// enum Either<L, R> {
//    Left(L),
//    Right(R),
// }

// #[derive(Serialize)]
// struct Keys {
//    github: String,
//    travis: Option<String>,
//    niano: Either<String, u32>,
// }


// fn main() -> () {
//    let _ : Either<String, i32> = Either::Right(42);
//     let config = Config {
//        ip: "127.0.0.1".to_string(),
//        port: None,
//        keys: Keys {
//            github: "xxxxxxxxxxxxxxxxx".to_string(),
//            // travis: Some("yyyyyyyyyyyyyyyyy".to_string()),
//            travis: None,
//            // niano: Either::Left("niano".to_string()),
//            niano: Either::Right(42),
//        },
//     };

//     let toml = toml::to_string(&config).unwrap();
//     println!("{}", toml);
// }
//

use enigo::{
    agent::{Agent, Token},
    Button, Enigo, Key, Settings,
};
use std::{thread, time::Duration};

use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize)]
enum ScreenShotType {
    Full,
    Window,
}

#[derive(Serialize, Deserialize)]
// #[serde(tag = "type", content = "value")]
enum NonEnigoAction {
    Sleep(u64),
    Shutdown,
    ScreenShot(ScreenShotType),
}

// #[serde(tag = "type", content = "value")]
// #[serde(tag = "type")]
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Action {
    Input(Token),
    NonEnigo(NonEnigoAction),
}

#[derive(Serialize, Deserialize)]
struct ButtonConfig {
   text: String,
   actions: Vec<Action>,
}

#[derive(Serialize, Deserialize)]
struct PanelConfig {
    name: String,
    button1: Option<ButtonConfig>,
    button2: Option<ButtonConfig>,
    button3: Option<ButtonConfig>,
    button4: Option<ButtonConfig>,
    button5: Option<ButtonConfig>,
    button6: Option<ButtonConfig>,
    button7: Option<ButtonConfig>,
    button8: Option<ButtonConfig>,
}

fn main() {
    // thread::sleep(Duration::from_secs(2));
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    // write text, move the mouse (10/10) relative from the cursors position, scroll
    // down, enter the unicode U+1F525 (🔥) and then select all
    let actions : Vec<Action> = vec![
        Action::Input(Token::Text("Hello World! ❤️".to_string())),
        Action::Input(Token::MoveMouse(10, 10, enigo::Coordinate::Rel)),
        Action::Input(Token::Scroll(5, enigo::Axis::Vertical)),
        Action::Input(Token::Button(Button::Left, enigo::Direction::Click)),
        Action::Input(Token::Key(Key::Unicode('🔥'), enigo::Direction::Click)),
        Action::Input(Token::Key(Key::Control, enigo::Direction::Press)),
        Action::NonEnigo(NonEnigoAction::Sleep(1000)),
        Action::Input(Token::Key(Key::Unicode('a'), enigo::Direction::Click)),
        Action::Input(Token::Key(Key::Control, enigo::Direction::Release)),
        Action::NonEnigo(NonEnigoAction::ScreenShot(ScreenShotType::Full)),
        Action::NonEnigo(NonEnigoAction::Shutdown),
    ];

    let serialized = toml::to_string_pretty(&PanelConfig {
        name: "test".to_string(),
        button1: Some(ButtonConfig {
            text: "127.0.0.1".to_string(),
            actions: actions,
        }),
        button2: None,
        button3: None,
        button4: None,
        button5: Some(ButtonConfig {
            text: "help".to_string(),
            actions: vec![
                Action::Input(Token::Key(Key::F1, enigo::Direction::Click)),
            ],
        }),
        button6: None,
        button7: None,
        button8: None,
    }).unwrap();
    println!("serialized = {serialized}");

    // let deserialized_config: Config = toml::from_str(&serialized).unwrap();
    // for token in &deserialized_config.values {
    //     println!("{:?}", enigo.execute(token));
    // }
}
