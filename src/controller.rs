use xencelabs_quick_keys::{QKDevice, ConnectionMode, ScreenOrientation, Event, ButtonState };

use hidapi::HidApi;
use std::{thread,time};
use enigo::{
    agent::Agent,
    Enigo, Settings,
};

use crate::model::Model;
use crate::actions::{Action, LowLevelButton, HighLevelButton, Button, NonEnigoAction, WhichButton, ButtonSet};
use crate::state;

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
    let mut state = state::State::new(model)?;
    // let mut states : ButtonSet<events::ClickStateMachine, events::ClickStateMachine> = ButtonSet {
    //     button1: events::ClickStateMachine::Idle,
    //     button2: events::ClickStateMachine::Idle,
    //     button3: events::ClickStateMachine::Idle,
    //     button4: events::ClickStateMachine::Idle,
    //     button5: events::ClickStateMachine::Idle,
    //     button6: events::ClickStateMachine::Idle,
    //     button7: events::ClickStateMachine::Idle,
    //     button8: events::ClickStateMachine::Idle,
    //     button9: events::ClickStateMachine::Idle,
    // };
    let mut last_state : Option<ButtonState> = None;
    // let (_, profile) = model.profiles.first().ok_or(Error::msg("No profiles"))?;
    // let (_, buttonset) = profile.buttonsets.first().ok_or(Error::msg("No buttonsets"))?;
    // let button_actions : [Vec<Action>; 8] = [
    //     on_press_actions(buttonset.button1.button.clone()),
    //     on_press_actions(buttonset.button2.button.clone()),
    //     on_press_actions(buttonset.button3.button.clone()),
    //     on_press_actions(buttonset.button4.button.clone()),
    //     on_press_actions(buttonset.button5.button.clone()),
    //     on_press_actions(buttonset.button6.button.clone()),
    //     on_press_actions(buttonset.button7.button.clone()),
    //     on_press_actions(buttonset.button8.button.clone()),
    // ];

    let api = HidApi::new()?;
    let dev = QKDevice::open(api, ConnectionMode::Auto)?;
    dev.set_screen_orientation(ScreenOrientation::Rotate180)?; 
    loop {
        let ev = dev.read_timeout(300)?;
        match ev {
            Event::Button { state: button_state } => {
                last_state = Some(button_state.clone());
            },
            Event::Unknown { .. } => {
                
            },
            _ => {
                println!("Ignoring event {:?}", ev);
            },
        }
        let states = &mut state.button_state;
        if let Some(ref bstate) = last_state {
            let now = time::Instant::now();
            let (button0_new_state, button0_events) = states.button0.transition(bstate.button_0.into(), now);
            let (button1_new_state, button1_events) = states.button1.transition(bstate.button_1.into(), now);
            let (button2_new_state, button2_events) = states.button2.transition(bstate.button_2.into(), now);
            let (button3_new_state, button3_events) = states.button3.transition(bstate.button_3.into(), now);
            let (button4_new_state, button4_events) = states.button4.transition(bstate.button_4.into(), now);
            let (button5_new_state, button5_events) = states.button5.transition(bstate.button_5.into(), now);
            let (button6_new_state, button6_events) = states.button6.transition(bstate.button_6.into(), now);
            let (button7_new_state, button7_events) = states.button7.transition(bstate.button_7.into(), now);
            let (button_extra_new_state, button_extra_events) = states.button_extra.transition(bstate.button_extra.into(), now);
            state.button_state = ButtonSet {
                button0: button0_new_state,
                button1: button1_new_state,
                button2: button2_new_state,
                button3: button3_new_state,
                button4: button4_new_state,
                button5: button5_new_state,
                button6: button6_new_state,
                button7: button7_new_state,
                button_extra: button_extra_new_state,
            };
            if !button0_events.is_empty() {
                println!("Button 0 events: {:?}", button0_events);
            }
            if !button1_events.is_empty() {
                println!("Button 1 events: {:?}", button1_events);
            }
            if !button2_events.is_empty() {
                println!("Button 2 events: {:?}", button2_events);
            }
            if !button3_events.is_empty() {
                println!("Button 3 events: {:?}", button3_events);
            }
            if !button4_events.is_empty() {
                println!("Button 4 events: {:?}", button4_events);
            }
            if !button5_events.is_empty() {
                println!("Button 5 events: {:?}", button5_events);
            }
            if !button6_events.is_empty() {
                println!("Button 6 events: {:?}", button6_events);
            }
            if !button7_events.is_empty() {
                println!("Button 7 events: {:?}", button7_events);
            }
            if !button_extra_events.is_empty() {
                println!("Button Extra events: {:?}", button_extra_events);
            }
        }

        // match ev {
        //     Event::Button { state: ButtonState { button_0: true, .. } } => button_actions[0].iter().try_for_each(|action| eval(&dev, &action, Some(WhichButton::Button0))),
        //     Event::Button { state: ButtonState { button_1: true, .. } } => button_actions[1].iter().try_for_each(|action| eval(&dev, &action, Some(WhichButton::Button1))),
        //     Event::Button { state: ButtonState { button_2: true, .. } } => button_actions[2].iter().try_for_each(|action| eval(&dev, &action, Some(WhichButton::Button2))),
        //     Event::Button { state: ButtonState { button_3: true, .. } } => button_actions[3].iter().try_for_each(|action| eval(&dev, &action, Some(WhichButton::Button3))),
        //     Event::Button { state: ButtonState { button_4: true, .. } } => button_actions[4].iter().try_for_each(|action| eval(&dev, &action, Some(WhichButton::Button4))),
        //     Event::Button { state: ButtonState { button_5: true, .. } } => button_actions[5].iter().try_for_each(|action| eval(&dev, &action, Some(WhichButton::Button5))),
        //     Event::Button { state: ButtonState { button_6: true, .. } } => button_actions[6].iter().try_for_each(|action| eval(&dev, &action, Some(WhichButton::Button6))),
        //     Event::Button { state: ButtonState { button_7: true, .. } } => button_actions[7].iter().try_for_each(|action| eval(&dev, &action, Some(WhichButton::Button7))),
        //     Event::Unknown{ .. } => Ok(()),
        //     e => {
        //         println!("Ignoring event {:?}", e);
        //         Ok(())
        //     },
        // }?;
    }
}

fn on_press_actions(button: Button<Vec<Action>>) -> Vec<Action> {
    match button {
        Button::LowLevel(LowLevelButton { on_press, .. }) => on_press,
        Button::HighLevel(HighLevelButton { on_click, .. }) => on_click,
    }
}
