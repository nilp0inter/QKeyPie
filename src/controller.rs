use xencelabs_quick_keys::{QKDevice, ConnectionMode, ScreenOrientation, Event, ButtonState };

use hidapi::HidApi;
use std::{thread,time};
use enigo::{
    agent::Agent,
    Enigo, Settings,
};

use anyhow::Error;

use crate::model::Model;
use crate::actions::{Action, LowLevelButton, HighLevelButton, Button, NonEnigoAction, WhichButton};

// fn eval(action: &Action) {
fn eval(dev: &QKDevice, action: &Action, current_button: Option<WhichButton>) -> anyhow::Result<()> {
    let mut enigo = Enigo::new(&Settings::default()).unwrap_or_else(|e| panic!("Failed to create enigo: {:?}", e));
    match action {
        Action::NonEnigo(NonEnigoAction::Sleep(millis)) => {
            thread::sleep(time::Duration::from_millis(*millis));
            Ok(())
        },
        Action::NonEnigo(NonEnigoAction::SetButtonText(wb, txt)) => {
            let res = match wb {
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
            match res {
                Ok(_) => Ok(()),
                Err(e) => anyhow::bail!("error: {:?}", e),
            }
        },
        Action::NonEnigo(NonEnigoAction::SetWheelColor(r, g, b)) => {
            match dev.set_ring_color(*r, *g, *b) {
                Ok(_) => Ok(()),
                Err(e) => anyhow::bail!("error: {:?}", e),
            }
        },
        Action::NonEnigo(NonEnigoAction::ShowBanner(seconds, txt)) => {
            match dev.show_overlay_text(txt, *seconds) {
                Ok(_) => Ok(()),
                Err(e) => anyhow::bail!("error: {:?}", e),
            }
        },
        Action::Input(token) => {
            match enigo.execute(token) {
                Ok(_) => Ok(()),
                Err(e) => anyhow::bail!("error: {:?}", e),
            }
        },
        _ => {
            println!("Unknown action {:?}", action);
            Ok(())
        }
    }
}


pub fn run(model: Model) -> anyhow::Result<()> {
    let (_, profile) = model.profiles.first().ok_or(Error::msg("No profiles"))?;
    let (_, buttonset) = profile.buttonsets.first().ok_or(Error::msg("No buttonsets"))?;
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

    let api = HidApi::new()?;
    let dev = QKDevice::open(api, ConnectionMode::Auto)?;
    dev.set_screen_orientation(ScreenOrientation::Rotate180)?; 
    loop {
        let ev = dev.read_timeout(1000)?;
        match ev {
            Event::Button { state: ButtonState { button_0: true, .. } } => button_actions[0].iter().try_for_each(|action| eval(&dev, &action, Some(WhichButton::Button0))),
            Event::Button { state: ButtonState { button_1: true, .. } } => button_actions[1].iter().try_for_each(|action| eval(&dev, &action, Some(WhichButton::Button1))),
            Event::Button { state: ButtonState { button_2: true, .. } } => button_actions[2].iter().try_for_each(|action| eval(&dev, &action, Some(WhichButton::Button2))),
            Event::Button { state: ButtonState { button_3: true, .. } } => button_actions[3].iter().try_for_each(|action| eval(&dev, &action, Some(WhichButton::Button3))),
            Event::Button { state: ButtonState { button_4: true, .. } } => button_actions[4].iter().try_for_each(|action| eval(&dev, &action, Some(WhichButton::Button4))),
            Event::Button { state: ButtonState { button_5: true, .. } } => button_actions[5].iter().try_for_each(|action| eval(&dev, &action, Some(WhichButton::Button5))),
            Event::Button { state: ButtonState { button_6: true, .. } } => button_actions[6].iter().try_for_each(|action| eval(&dev, &action, Some(WhichButton::Button6))),
            Event::Button { state: ButtonState { button_7: true, .. } } => button_actions[7].iter().try_for_each(|action| eval(&dev, &action, Some(WhichButton::Button7))),
            Event::Unknown{ .. } => Ok(()),
            e => {
                println!("Ignoring event {:?}", e);
                Ok(())
            },
        }?;
    }
}

fn on_press_actions(button: Button<Vec<Action>>) -> Vec<Action> {
    match button {
        Button::LowLevel(LowLevelButton { on_press, .. }) => on_press,
        Button::HighLevel(HighLevelButton { on_click, .. }) => on_click,
    }
}
