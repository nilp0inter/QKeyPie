use xencelabs_quick_keys::{QKDevice, ConnectionMode, ScreenOrientation };

use hidapi::HidApi;
use std::{time, thread};
use enigo::{
    agent::Agent,
    Enigo, Settings,
};

use crate::model::Model;
use crate::actions::{Action, NonEnigoAction, WhichButton};
use crate::actions::{ButtonSet, WheelSet, ButtonCallback, WheelCallback};
use crate::events::{ButtonState, WheelState, ButtonEvent, WheelEvent};
use crate::state;

fn eval(enigo: &mut Enigo, dev: &QKDevice, action: &Action, current_button: Option<WhichButton>) -> anyhow::Result<()> {
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

fn process_button_event(enigo: &mut Enigo, dev: &QKDevice, event: &ButtonEvent, callbacks: &ButtonCallback<Vec<Action>>, current_button: WhichButton) -> anyhow::Result<()> {
    match event {
        ButtonEvent::OnPress => {
            for action in &callbacks.on_press {
                eval(enigo, dev, action, Some(current_button.clone()))?;
            }
        },
        ButtonEvent::OnRelease => {
            for action in &callbacks.on_release {
                eval(enigo, dev, action, Some(current_button.clone()))?;
            }
        },
        ButtonEvent::OnLongPress => {
            for action in &callbacks.on_long_press {
                eval(enigo, dev, action, Some(current_button.clone()))?;
            }
        },
        ButtonEvent::OnClick(click_count) => {
            match click_count {
                1 => {
                    for action in &callbacks.on_click {
                        eval(enigo, dev, action, Some(current_button.clone()))?;
                    }
                },
                2 => {
                    for action in &callbacks.on_double_click {
                        eval(enigo, dev, action, Some(current_button.clone()))?;
                    }
                },
                3 => {
                    for action in &callbacks.on_triple_click {
                        eval(enigo, dev, action, Some(current_button.clone()))?;
                    }
                },
                _ => {
                    println!("Unknown click count {}", click_count);
                }
            }
        },
    }
    Ok(())
}

fn process_wheel_event(enigo: &mut Enigo, dev: &QKDevice, event: &WheelEvent, callbacks: &WheelCallback<Vec<Action>>) -> anyhow::Result<()> {
    match event {
        WheelEvent::OnRotateClockwiseStart => {
            for action in &callbacks.on_clockwise_start {
                eval(enigo, dev, action, None)?;
            }
        },
        WheelEvent::OnRotateClockwiseEnd => {
            for action in &callbacks.on_clockwise_stop {
                eval(enigo, dev, action, None)?;
            }
        },
        WheelEvent::OnRotateClockwiseStep => {
            for action in &callbacks.on_clockwise {
                eval(enigo, dev, action, None)?;
            }
        },
        WheelEvent::OnRotateCounterClockwiseStart => {
            for action in &callbacks.on_counterclockwise_start {
                eval(enigo, dev, action, None)?;
            }
        },
        WheelEvent::OnRotateCounterClockwiseEnd => {
            for action in &callbacks.on_counterclockwise_stop {
                eval(enigo, dev, action, None)?;
            }
        },
        WheelEvent::OnRotateCounterClockwiseStep => {
            for action in &callbacks.on_counterclockwise {
                eval(enigo, dev, action, None)?;
            }
        },
    }
    Ok(())
}


pub fn run(model: Model) -> anyhow::Result<()> {
    let mut state = state::State::new(model)?;

    let mut enigo = Enigo::new(&Settings::default()).unwrap_or_else(|e| panic!("Failed to create enigo: {:?}", e));
    let api = HidApi::new()?;
    let dev = QKDevice::open(api, ConnectionMode::Auto)?;
    dev.set_screen_orientation(ScreenOrientation::Rotate180)?; 
    loop {
        let ev = dev.read_timeout(300)?;
        let buttonset_event : ButtonSet<ButtonState> = ev.clone().into();
        let wheel_event : WheelSet<WheelState, ButtonState> = ev.clone().into();

        let now = time::Instant::now();
        let (new_buttonset_state, buttonset_events) = state.button_state.transition(buttonset_event.into(), now);
        let (new_wheel_state, wheel_events) = state.wheel_state.transition(wheel_event.into(), now);

        state.button_state = new_buttonset_state;
        state.wheel_state = new_wheel_state;

        for event in buttonset_events.button0 {
            let callbacks = state.model.profiles[state.current_profile_index].buttonsets[state.current_buttonset_index].button0.clone();
            process_button_event(&mut enigo, &dev, &event, &callbacks, WhichButton::Button0)?;
        }
        for event in buttonset_events.button1 {
            let callbacks = state.model.profiles[state.current_profile_index].buttonsets[state.current_buttonset_index].button1.clone();
            process_button_event(&mut enigo, &dev, &event, &callbacks, WhichButton::Button1)?;
        }
        for event in buttonset_events.button2 {
            let callbacks = state.model.profiles[state.current_profile_index].buttonsets[state.current_buttonset_index].button2.clone();
            process_button_event(&mut enigo, &dev, &event, &callbacks, WhichButton::Button2)?;
        }
        for event in buttonset_events.button3 {
            let callbacks = state.model.profiles[state.current_profile_index].buttonsets[state.current_buttonset_index].button3.clone();
            process_button_event(&mut enigo, &dev, &event, &callbacks, WhichButton::Button3)?;
        }
        for event in buttonset_events.button4 {
            let callbacks = state.model.profiles[state.current_profile_index].buttonsets[state.current_buttonset_index].button4.clone();
            process_button_event(&mut enigo, &dev, &event, &callbacks, WhichButton::Button4)?;
        }
        for event in buttonset_events.button5 {
            let callbacks = state.model.profiles[state.current_profile_index].buttonsets[state.current_buttonset_index].button5.clone();
            process_button_event(&mut enigo, &dev, &event, &callbacks, WhichButton::Button5)?;
        }
        for event in buttonset_events.button6 {
            let callbacks = state.model.profiles[state.current_profile_index].buttonsets[state.current_buttonset_index].button6.clone();
            process_button_event(&mut enigo, &dev, &event, &callbacks, WhichButton::Button6)?;
        }
        for event in buttonset_events.button7 {
            let callbacks = state.model.profiles[state.current_profile_index].buttonsets[state.current_buttonset_index].button7.clone();
            process_button_event(&mut enigo, &dev, &event, &callbacks, WhichButton::Button7)?;
        }
        for event in buttonset_events.button_extra {
            let callbacks = state.model.profiles[state.current_profile_index].buttonsets[state.current_buttonset_index].button_extra.clone();
            process_button_event(&mut enigo, &dev, &event, &callbacks, WhichButton::ButtonExtra)?;
        }
        for event in wheel_events.wheel_button {
            let callbacks = state.model.profiles[state.current_profile_index].wheels[state.current_wheel_index].button.clone();
            process_button_event(&mut enigo, &dev, &event, &callbacks, WhichButton::WheelButton)?;
        }

        for event in wheel_events.wheel {
            let callbacks = state.model.profiles[state.current_profile_index].wheels[state.current_wheel_index].clone();
            process_wheel_event(&mut enigo, &dev, &event, &callbacks)?;
        }
    }
}
