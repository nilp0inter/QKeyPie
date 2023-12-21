use xencelabs_quick_keys::{QKDevice, ConnectionMode, ScreenOrientation };

use hidapi::HidApi;
use std::time;
// use enigo::{
//     agent::Agent,
//     Enigo, Settings,
// };

use crate::model::Model;
// use crate::actions::{Action, NonEnigoAction, WhichButton};
use crate::actions::{ButtonSet, WheelSet};
use crate::events::{ButtonState, WheelState};
use crate::state;

// fn eval(dev: &QKDevice, action: &Action, current_button: Option<WhichButton>) -> anyhow::Result<()> {
//     let mut enigo = Enigo::new(&Settings::default()).unwrap_or_else(|e| panic!("Failed to create enigo: {:?}", e));
//     match action {
//         Action::NonEnigo(NonEnigoAction::Sleep(millis)) => {
//             thread::sleep(time::Duration::from_millis(*millis));
//             Ok(())
//         },
//         Action::NonEnigo(NonEnigoAction::SetButtonText(wb, txt)) => {
//             let res = match wb {
//                 WhichButton::Button0 => dev.set_key_text(0, txt),
//                 WhichButton::Button1 => dev.set_key_text(1, txt),
//                 WhichButton::Button2 => dev.set_key_text(2, txt),
//                 WhichButton::Button3 => dev.set_key_text(3, txt),
//                 WhichButton::Button4 => dev.set_key_text(4, txt),
//                 WhichButton::Button5 => dev.set_key_text(5, txt),
//                 WhichButton::Button6 => dev.set_key_text(6, txt),
//                 WhichButton::Button7 => dev.set_key_text(7, txt),
//                 WhichButton::ThisButton => match current_button {
//                     Some(WhichButton::Button0) => dev.set_key_text(0, txt),
//                     Some(WhichButton::Button1) => dev.set_key_text(1, txt),
//                     Some(WhichButton::Button2) => dev.set_key_text(2, txt),
//                     Some(WhichButton::Button3) => dev.set_key_text(3, txt),
//                     Some(WhichButton::Button4) => dev.set_key_text(4, txt),
//                     Some(WhichButton::Button5) => dev.set_key_text(5, txt),
//                     Some(WhichButton::Button6) => dev.set_key_text(6, txt),
//                     Some(WhichButton::Button7) => dev.set_key_text(7, txt),
//                     _ => Ok(()),
//                 },
//                 _ => Ok(()),
//             };
//             match res {
//                 Ok(_) => Ok(()),
//                 Err(e) => anyhow::bail!("error: {:?}", e),
//             }
//         },
//         Action::NonEnigo(NonEnigoAction::SetWheelColor(r, g, b)) => {
//             match dev.set_ring_color(*r, *g, *b) {
//                 Ok(_) => Ok(()),
//                 Err(e) => anyhow::bail!("error: {:?}", e),
//             }
//         },
//         Action::NonEnigo(NonEnigoAction::ShowBanner(seconds, txt)) => {
//             match dev.show_overlay_text(txt, *seconds) {
//                 Ok(_) => Ok(()),
//                 Err(e) => anyhow::bail!("error: {:?}", e),
//             }
//         },
//         Action::Input(token) => {
//             match enigo.execute(token) {
//                 Ok(_) => Ok(()),
//                 Err(e) => anyhow::bail!("error: {:?}", e),
//             }
//         },
//         _ => {
//             println!("Unknown action {:?}", action);
//             Ok(())
//         }
//     }
// }


pub fn run(model: Model) -> anyhow::Result<()> {
    let mut state = state::State::new(model)?;

    let api = HidApi::new()?;
    let dev = QKDevice::open(api, ConnectionMode::Auto)?;
    dev.set_screen_orientation(ScreenOrientation::Rotate180)?; 
    loop {
        let ev = dev.read_timeout(300)?;
        let buttonset_event : ButtonSet<ButtonState, ButtonState> = ev.clone().into();
        let wheel_event : WheelSet<WheelState, ButtonState> = ev.clone().into();
        let states = &mut state.button_state;

        let now = time::Instant::now();
        let (new_buttonset_state, buttonset_events) = states.transition(buttonset_event.into(), now);
        let (new_wheel_state, wheel_events) = state.wheel_state.transition(wheel_event.into(), now);
        state.button_state = new_buttonset_state;
        state.wheel_state = new_wheel_state;
        if !buttonset_events.button0.is_empty() {
            println!("Button 0 events: {:?}", buttonset_events.button0);
        }
        if !buttonset_events.button1.is_empty() {
            println!("Button 1 events: {:?}", buttonset_events.button1);
        }
        if !buttonset_events.button2.is_empty() {
            println!("Button 2 events: {:?}", buttonset_events.button2);
        }
        if !buttonset_events.button3.is_empty() {
            println!("Button 3 events: {:?}", buttonset_events.button3);
        }
        if !buttonset_events.button4.is_empty() {
            println!("Button 4 events: {:?}", buttonset_events.button4);
        }
        if !buttonset_events.button5.is_empty() {
            println!("Button 5 events: {:?}", buttonset_events.button5);
        }
        if !buttonset_events.button6.is_empty() {
            println!("Button 6 events: {:?}", buttonset_events.button6);
        }
        if !buttonset_events.button7.is_empty() {
            println!("Button 7 events: {:?}", buttonset_events.button7);
        }
        if !buttonset_events.button_extra.is_empty() {
            println!("Button extra events: {:?}", buttonset_events.button_extra);
        }
        if !wheel_events.wheel.is_empty() {
            println!("Wheel events: {:?}", wheel_events.wheel);
        }
        if !wheel_events.wheel_button.is_empty() {
            println!("Wheel button events: {:?}", wheel_events.wheel_button);
        }
    }
}

// fn on_press_actions(button: ButtonCallback<Vec<Action>>) -> Vec<Action> {
//     match button {
//         ButtonCallback::LowLevel(LowLevelButton { on_press, .. }) => on_press,
//         ButtonCallback::HighLevel(HighLevelButton { on_click, .. }) => on_click,
//     }
// }
