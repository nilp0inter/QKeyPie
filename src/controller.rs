use xencelabs_quick_keys::{QKDevice, ConnectionMode, QKResult, ScreenOrientation, ScreenBrightness, WheelSpeed, Event, ButtonState, WheelDirection };

use hidapi::HidApi;
use std::{thread,time};
use enigo::{
    agent::{Agent, Token},
    Enigo, Settings,
};

use anyhow::Error;

use crate::model::Model;
use crate::actions::{Action, LowLevelButton, HighLevelButton, Button, NonEnigoAction, WhichButton};

// fn eval(action: &Action) {
fn eval(dev: &QKDevice, action: &Action, current_button: Option<WhichButton>) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap_or_else(|e| panic!("Failed to create enigo: {:?}", e));
    match action {
        Action::NonEnigo(NonEnigoAction::Sleep(millis)) => {
            thread::sleep(time::Duration::from_millis(*millis));
        },
        Action::NonEnigo(NonEnigoAction::SetButtonText(wb, txt)) => {
            match wb {
                WhichButton::Button0 => dev.set_key_text(0, txt),
                WhichButton::Button1 => dev.set_key_text(1, txt),
                WhichButton::Button2 => dev.set_key_text(2, txt),
                WhichButton::Button3 => dev.set_key_text(3, txt),
                WhichButton::Button4 => dev.set_key_text(4, txt),
                WhichButton::Button5 => dev.set_key_text(5, txt),
                WhichButton::Button6 => dev.set_key_text(6, txt),
                WhichButton::Button7 => dev.set_key_text(7, txt),
                WhichButton::ThisButton => match current_button {
                    Some(WhichButton::Button0) => dev.set_key_text(0, txt),
                    Some(WhichButton::Button1) => dev.set_key_text(1, txt),
                    Some(WhichButton::Button2) => dev.set_key_text(2, txt),
                    Some(WhichButton::Button3) => dev.set_key_text(3, txt),
                    Some(WhichButton::Button4) => dev.set_key_text(4, txt),
                    Some(WhichButton::Button5) => dev.set_key_text(5, txt),
                    Some(WhichButton::Button6) => dev.set_key_text(6, txt),
                    Some(WhichButton::Button7) => dev.set_key_text(7, txt),
                    _ => Ok(()),
                },
                _ => Ok(()),
            };
            println!("SetButtonText");
        },
        Action::NonEnigo(NonEnigoAction::SetWheelColor(r, g, b)) => {
            println!("SetWheelColor");
            dev.set_ring_color(*r, *g, *b);
        },
        Action::NonEnigo(NonEnigoAction::ShowBanner(seconds, txt)) => {
            println!("ShowBanner");
            dev.show_overlay_text(txt, *seconds);
        },
        Action::Input(token) => {
            println!("Input");
            enigo.execute(token);
        },
        _ => println!("Unknown action {:?}", action),
    }
}


fn run3(api: HidApi) -> QKResult<()> {
    match QKDevice::open(api, ConnectionMode::Auto) {
        Ok(dev) => {
            dev.set_screen_orientation(ScreenOrientation::Rotate270)?;
            dev.set_screen_brightness(ScreenBrightness::Medium)?;
            dev.set_wheel_speed(WheelSpeed::Normal)?;
            dev.set_sleep_timeout(1)?;
            dev.set_ring_color(255, 255, 255)?;
            dev.set_key_text(0, "red")?;
            thread::sleep(time::Duration::from_millis(1000));
            dev.show_overlay_text("Disco, disco!", 3)?;
            loop {
                match dev.read() {
                    Ok(ev) => match ev {
                        Event::Wheel { direction: WheelDirection::Left } => dev.set_ring_color(255, 0, 0),
                        Event::Wheel { direction: WheelDirection::Right } => dev.set_ring_color(0, 255, 0),
                        Event::Button { state: ButtonState { button_wheel: true, .. } } => dev.set_ring_color(0, 0, 255),
                        Event::Button { state: ButtonState { button_0: true, .. } } => dev.set_ring_color(255, 0, 0),
                        Event::Button { state: ButtonState { button_1: true, .. } } => dev.set_ring_color(0, 255, 0),
                        Event::Button { state: ButtonState { button_2: true, .. } } => dev.set_ring_color(0, 0, 255),
                        Event::Button { state: ButtonState { button_3: true, .. } } => dev.set_ring_color(255, 255, 0),
                        Event::Button { state: ButtonState { button_4: true, .. } } => dev.set_ring_color(255, 0, 255),
                        Event::Button { state: ButtonState { button_5: true, .. } } => dev.set_ring_color(0, 255, 255),
                        Event::Button { state: ButtonState { button_6: true, .. } } => dev.set_ring_color(255, 255, 255),
                        Event::Button { state: ButtonState { button_7: true, .. } } => dev.set_ring_color(0, 0, 0),
                        Event::Button { state: ButtonState { button_extra: true, .. } } => dev.show_overlay_text("Disco, disco!", 3),
                        Event::Button { state: ButtonState { .. } } => { println!("release"); Ok(()) },
                        Event::Unknown { data: d } => { println!("unknown! {:?}", d); Ok(()) },
                        Event::Battery { percent: p } => { println!("battery level: {:?}", p); Ok(()) },
                    },
                    Err(e) => Err(e),
                }?;
            }
        },
        Err(e) => { println!("Connection error!"); Err(e) },
    }
}

pub fn run(model: Model) -> anyhow::Result<()> {
    match HidApi::new() {
        Ok(api) => match run2(model, api) {
            Ok(_) => { println!("all good"); Ok(())}
            Err(e) => anyhow::bail!("error: {:?}", e),
        },
        Err(e) => anyhow::bail!("error: {:?}", e),
    }
}

fn on_press_actions(button: Button<Vec<Action>>) -> Vec<Action> {
    match button {
        Button::LowLevel(LowLevelButton { on_press, .. }) => on_press,
        Button::HighLevel(HighLevelButton { on_click, .. }) => on_click,
    }
}

// fn run2(model: Model) -> anyhow::Result<()> {
fn run2(model: Model, api: HidApi) -> anyhow::Result<()> {
    let (profile_name, profile) = model.profiles.first().ok_or(Error::msg("No profiles"))?;
    let (buttonset_name, buttonset) = profile.buttonsets.first().ok_or(Error::msg("No buttonsets"))?;
    let button_actions : [Vec<Action>; 8] = [
        on_press_actions(buttonset.button1.button.clone()),
        on_press_actions(buttonset.button2.button.clone()),
        on_press_actions(buttonset.button3.button.clone()),
        on_press_actions(buttonset.button4.button.clone()),
        on_press_actions(buttonset.button5.button.clone()),
        on_press_actions(buttonset.button6.button.clone()),
        on_press_actions(buttonset.button7.button.clone()),
        on_press_actions(buttonset.button8.button.clone()),
    ];
    // println!("Hello, profile = {:?}", profile);
    // println!("Hello, buttonset = {:?}", buttonset);
    // println!("Hello, button1 = {:?}", button1);
    // let actions = match button1 {
    //     Button::LowLevel(LowLevelButton { on_press, .. }) => on_press,
    //     Button::HighLevel(HighLevelButton { on_click, .. }) => on_click,
    // };

    match QKDevice::open(api, ConnectionMode::Auto) {
        Ok(dev) => {
            dev.set_screen_orientation(ScreenOrientation::Rotate180);
            loop {
                match dev.read() {
                    Ok(ev) => match ev {
                        Event::Button { state: ButtonState { button_0: true, .. } } => button_actions[0].iter().for_each(|action| eval(&dev, &action, Some(WhichButton::Button0))),
                        Event::Button { state: ButtonState { button_1: true, .. } } => button_actions[1].iter().for_each(|action| eval(&dev, &action, Some(WhichButton::Button1))),
                        Event::Button { state: ButtonState { button_2: true, .. } } => button_actions[2].iter().for_each(|action| eval(&dev, &action, Some(WhichButton::Button2))),
                        Event::Button { state: ButtonState { button_3: true, .. } } => button_actions[3].iter().for_each(|action| eval(&dev, &action, Some(WhichButton::Button3))),
                        Event::Button { state: ButtonState { button_4: true, .. } } => button_actions[4].iter().for_each(|action| eval(&dev, &action, Some(WhichButton::Button4))),
                        Event::Button { state: ButtonState { button_5: true, .. } } => button_actions[5].iter().for_each(|action| eval(&dev, &action, Some(WhichButton::Button5))),
                        Event::Button { state: ButtonState { button_6: true, .. } } => button_actions[6].iter().for_each(|action| eval(&dev, &action, Some(WhichButton::Button6))),
                        Event::Button { state: ButtonState { button_7: true, .. } } => button_actions[7].iter().for_each(|action| eval(&dev, &action, Some(WhichButton::Button7))),
                        _ => (),
                    },
                    Err(e) => anyhow::bail!("error: {:?}", e),
                };
            }
        },
        Err(e) => { println!("Connection error!"); anyhow::bail!("error: {:?}", e) },
    };
    Ok(())
}
