use xencelabs_quick_keys::{QKDevice, ConnectionMode, ScreenOrientation };

use hidapi::HidApi;
use std::{time, thread};
use enigo::{
    agent::Agent,
    Enigo, Settings,
};

use crate::model::Model;
use crate::actions::{Action, NonEnigoAction, WhichButton};
use crate::actions::{ButtonSet, WheelSet, ButtonCallback, WheelSetCallback, GoTo, ChangeRef};
use crate::events::{ButtonState, WheelState, ButtonEvent, WheelEvent};
use crate::state;

fn eval(enigo: &mut Enigo, dev: &QKDevice, action: &Action, current_button: Option<WhichButton>) -> anyhow::Result<Option<GoTo>> {
    match action {
        Action::NonEnigo(NonEnigoAction::Sleep(millis)) => {
            thread::sleep(time::Duration::from_millis(*millis));
            Ok(None)
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
                Ok(_) => Ok(None),
                Err(e) => anyhow::bail!("error: {:?}", e),
            }
        },
        Action::NonEnigo(NonEnigoAction::SetWheelColor(r, g, b)) => {
            match dev.set_ring_color(*r, *g, *b) {
                Ok(_) => Ok(None),
                Err(e) => anyhow::bail!("error: {:?}", e),
            }
        },
        Action::NonEnigo(NonEnigoAction::ShowBanner(seconds, txt)) => {
            match dev.show_overlay_text(txt, *seconds) {
                Ok(_) => Ok(None),
                Err(e) => anyhow::bail!("error: {:?}", e),
            }
        },
        Action::Input(token) => {
            match enigo.execute(token) {
                Ok(_) => Ok(None),
                Err(e) => anyhow::bail!("error: {:?}", e),
            }
        },
        Action::NonEnigo(NonEnigoAction::ChangeProfile(profile, buttonset, wheel)) => {
            Ok(Some(GoTo::Switch(profile.clone(), buttonset.clone(), wheel.clone())))
        },
        Action::NonEnigo(NonEnigoAction::ChangeWheel(wheel)) => {
            Ok(Some(GoTo::Switch(ChangeRef::This, ChangeRef::This, wheel.clone())))
        },
        Action::NonEnigo(NonEnigoAction::ChangeButtonSet(buttonset)) => {
            Ok(Some(GoTo::Switch(ChangeRef::This, buttonset.clone(), ChangeRef::This)))
        },
        Action::NonEnigo(NonEnigoAction::Swap) => {
            Ok(Some(GoTo::Swap))
        },
        Action::NonEnigo(NonEnigoAction::Macro(_)) => {
            anyhow::bail!("Macro action not resolved");
        },
        Action::NonEnigo(NonEnigoAction::Debug(txt)) => {
            println!("Debug: {}", txt);
            Ok(None)
        },
        Action::NonEnigo(NonEnigoAction::Run(args)) => {
            let mut cmd = std::process::Command::new(&args[0]);
            for arg in &args[1..] {
                cmd.arg(arg);
            }
            match cmd.spawn() {
                Ok(_) => Ok(None),
                Err(e) => anyhow::bail!("error: {:?}", e),
            }
        },
        Action::NonEnigo(NonEnigoAction::SetScreenOrientation(orientation)) => {
            match dev.set_screen_orientation(*orientation) {
                Ok(_) => Ok(None),
                Err(e) => anyhow::bail!("error: {:?}", e),
            }
        },
        Action::NonEnigo(NonEnigoAction::SetScreenBrightness(brightness)) => {
            match dev.set_screen_brightness(*brightness) {
                Ok(_) => Ok(None),
                Err(e) => anyhow::bail!("error: {:?}", e),
            }
        },
        Action::NonEnigo(NonEnigoAction::SetWheelSpeed(speed)) => {
            match dev.set_wheel_speed(*speed) {
                Ok(_) => Ok(None),
                Err(e) => anyhow::bail!("error: {:?}", e),
            }
        },
        Action::NonEnigo(NonEnigoAction::SetSleepTimeout(minutes)) => {
            match dev.set_sleep_timeout(*minutes) {
                Ok(_) => Ok(None),
                Err(e) => anyhow::bail!("error: {:?}", e),
            }
        },
    }
}

fn process_button_event(enigo: &mut Enigo, dev: &QKDevice, event: &ButtonEvent, callbacks: &ButtonCallback<Vec<Action>>, current_button: WhichButton) -> anyhow::Result<Option<GoTo>> {
    match event {
        ButtonEvent::OnPress => {
            callbacks.on_press.iter().try_fold(None, |acc, action| {
                eval(enigo, dev, action, Some(current_button.clone())).map(|opt_value| opt_value.or(acc))
            })
        },
        ButtonEvent::OnRelease => {
            callbacks.on_release.iter().try_fold(None, |acc, action| {
                eval(enigo, dev, action, Some(current_button.clone())).map(|opt_value| opt_value.or(acc))
            })
        },
        ButtonEvent::OnLongPress => {
            callbacks.on_long_press.iter().try_fold(None, |acc, action| {
                eval(enigo, dev, action, Some(current_button.clone())).map(|opt_value| opt_value.or(acc))
            })
        },
        ButtonEvent::OnClickPress(click_count) => {
            match click_count {
                1 => {
                    callbacks.on_click_press.iter().try_fold(None, |acc, action| {
                        eval(enigo, dev, action, Some(current_button.clone())).map(|opt_value| opt_value.or(acc))
                    })
                },
                2 => {
                    callbacks.on_double_click_press.iter().try_fold(None, |acc, action| {
                        eval(enigo, dev, action, Some(current_button.clone())).map(|opt_value| opt_value.or(acc))
                    })
                },
                3 => {
                    callbacks.on_triple_click_press.iter().try_fold(None, |acc, action| {
                        eval(enigo, dev, action, Some(current_button.clone())).map(|opt_value| opt_value.or(acc))
                    })
                },
                _ => {
                    println!("Unknown click count {}", click_count);
                    Ok(None)
                }
            }
        },
        ButtonEvent::OnClick(click_count) => {
            match click_count {
                1 => {
                    callbacks.on_click.iter().try_fold(None, |acc, action| {
                        eval(enigo, dev, action, Some(current_button.clone())).map(|opt_value| opt_value.or(acc))
                    })
                },
                2 => {
                    callbacks.on_double_click.iter().try_fold(None, |acc, action| {
                        eval(enigo, dev, action, Some(current_button.clone())).map(|opt_value| opt_value.or(acc))
                    })
                },
                3 => {
                    callbacks.on_triple_click.iter().try_fold(None, |acc, action| {
                        eval(enigo, dev, action, Some(current_button.clone())).map(|opt_value| opt_value.or(acc))
                    })
                },
                _ => {
                    println!("Unknown click count {}", click_count);
                    Ok(None)
                }
            }
        },
        ButtonEvent::OnClickRelease(click_count) => {
            match click_count {
                1 => {
                    callbacks.on_click_release.iter().try_fold(None, |acc, action| {
                        eval(enigo, dev, action, Some(current_button.clone())).map(|opt_value| opt_value.or(acc))
                    })
                },
                2 => {
                    callbacks.on_double_click_release.iter().try_fold(None, |acc, action| {
                        eval(enigo, dev, action, Some(current_button.clone())).map(|opt_value| opt_value.or(acc))
                    })
                },
                3 => {
                    callbacks.on_triple_click_release.iter().try_fold(None, |acc, action| {
                        eval(enigo, dev, action, Some(current_button.clone())).map(|opt_value| opt_value.or(acc))
                    })
                },
                _ => {
                    println!("Unknown click count {}", click_count);
                    Ok(None)
                }
            }
        },
    }
}

fn process_wheel_event(enigo: &mut Enigo, dev: &QKDevice, event: &WheelEvent, callbacks: &WheelSetCallback<Vec<Action>>) -> anyhow::Result<Option<GoTo>> {
    match event {
        WheelEvent::OnRotateClockwiseStart => {
            callbacks.wheel.on_clockwise_start.iter().try_fold(None, |acc, action| {
                eval(enigo, dev, action, None).map(|opt_value| opt_value.or(acc))
            })
        },
        WheelEvent::OnRotateClockwiseEnd => {
            callbacks.wheel.on_clockwise_stop.iter().try_fold(None, |acc, action| {
                eval(enigo, dev, action, None).map(|opt_value| opt_value.or(acc))
            })
        },
        WheelEvent::OnRotateClockwiseStep => {
            callbacks.wheel.on_clockwise.iter().try_fold(None, |acc, action| {
                eval(enigo, dev, action, None).map(|opt_value| opt_value.or(acc))
            })
        },
        WheelEvent::OnRotateCounterClockwiseStart => {
            callbacks.wheel.on_counterclockwise_start.iter().try_fold(None, |acc, action| {
                eval(enigo, dev, action, None).map(|opt_value| opt_value.or(acc))
            })
        },
        WheelEvent::OnRotateCounterClockwiseEnd => {
            callbacks.wheel.on_counterclockwise_stop.iter().try_fold(None, |acc, action| {
                eval(enigo, dev, action, None).map(|opt_value| opt_value.or(acc))
            })
        },
        WheelEvent::OnRotateCounterClockwiseStep => {
            callbacks.wheel.on_counterclockwise.iter().try_fold(None, |acc, action| {
                eval(enigo, dev, action, None).map(|opt_value| opt_value.or(acc))
            })
        },
    }
}


fn process_buttonset_events(enigo: &mut Enigo, dev: &QKDevice, events: Vec<ButtonEvent>, callbacks: &ButtonCallback<Vec<Action>>, current_button: WhichButton) -> anyhow::Result<Option<GoTo>> {
    events.iter().try_fold(None, |acc, event| {
        process_button_event(enigo, dev, event, callbacks, current_button.clone()).map(|opt_value| opt_value.or(acc))
    })
}

pub fn run(model: Model) -> anyhow::Result<()> {
    let mut state = state::State::new(model)?;

    let mut enigo = Enigo::new(&Settings::default()).unwrap_or_else(|e| panic!("Failed to create enigo: {:?}", e));
    let api = HidApi::new()?;
    let dev = QKDevice::open(api, ConnectionMode::Auto)?;

    dev.set_screen_orientation(ScreenOrientation::Rotate180)?; 
    dev.set_wheel_speed(xencelabs_quick_keys::WheelSpeed::Slower)?;

    // Enter the initial state
    {
        let current_profile = state.get_current_profile();
        let current_buttonset = state.get_current_buttonset();
        let current_wheel = state.get_current_wheel();
        for action in &current_profile.active.on_enter {
            eval(&mut enigo, &dev, action, None)?;
        }
        for action in &current_buttonset.active.on_enter {
            eval(&mut enigo, &dev, action, None)?;
        }
        for action in &current_buttonset.buttonset.button0.active.on_enter {
            eval(&mut enigo, &dev, action, Some(WhichButton::Button0))?;
        }
        for action in &current_buttonset.buttonset.button1.active.on_enter {
            eval(&mut enigo, &dev, action, Some(WhichButton::Button1))?;
        }
        for action in &current_buttonset.buttonset.button2.active.on_enter {
            eval(&mut enigo, &dev, action, Some(WhichButton::Button2))?;
        }
        for action in &current_buttonset.buttonset.button3.active.on_enter {
            eval(&mut enigo, &dev, action, Some(WhichButton::Button3))?;
        }
        for action in &current_buttonset.buttonset.button4.active.on_enter {
            eval(&mut enigo, &dev, action, Some(WhichButton::Button4))?;
        }
        for action in &current_buttonset.buttonset.button5.active.on_enter {
            eval(&mut enigo, &dev, action, Some(WhichButton::Button5))?;
        }
        for action in &current_buttonset.buttonset.button6.active.on_enter {
            eval(&mut enigo, &dev, action, Some(WhichButton::Button6))?;
        }
        for action in &current_buttonset.buttonset.button7.active.on_enter {
            eval(&mut enigo, &dev, action, Some(WhichButton::Button7))?;
        }
        for action in &current_buttonset.buttonset.button_extra.active.on_enter {
            eval(&mut enigo, &dev, action, Some(WhichButton::ButtonExtra))?;
        }
        for action in &current_wheel.active.on_enter {
            eval(&mut enigo, &dev, action, None)?;
        }
        for action in &current_wheel.wheel.button.active.on_enter {
            eval(&mut enigo, &dev, action, Some(WhichButton::WheelButton))?;
        }
    }

    loop {
        let ev = dev.read_timeout(100)?;
        let buttonset_event : ButtonSet<ButtonState> = ev.clone().into();
        let wheel_event : WheelSet<WheelState, ButtonState> = ev.clone().into();

        let now = time::Instant::now();
        let (new_buttonset_state, buttonset_events) = state.button_state.transition(buttonset_event, now);
        let (new_wheel_state, wheel_events) = state.wheel_state.transition(wheel_event, now);

        state.button_state = new_buttonset_state;
        state.wheel_state = new_wheel_state;

        let current_profile = state.get_current_profile();
        let current_buttonset = state.get_current_buttonset();
        let current_wheel = state.get_current_wheel();

        let mut final_goto = None;

        if let Some(goto) = process_buttonset_events(&mut enigo, &dev, buttonset_events.button0, &current_buttonset.buttonset.button0, WhichButton::Button0)? {
            final_goto = Some(goto);
        }
        if let Some(goto) = process_buttonset_events(&mut enigo, &dev, buttonset_events.button1, &current_buttonset.buttonset.button1, WhichButton::Button1)? {
            final_goto = Some(goto);
        }
        if let Some(goto) = process_buttonset_events(&mut enigo, &dev, buttonset_events.button2, &current_buttonset.buttonset.button2, WhichButton::Button2)? {
            final_goto = Some(goto);
        }
        if let Some(goto) = process_buttonset_events(&mut enigo, &dev, buttonset_events.button3, &current_buttonset.buttonset.button3, WhichButton::Button3)? {
            final_goto = Some(goto);
        }
        if let Some(goto) = process_buttonset_events(&mut enigo, &dev, buttonset_events.button4, &current_buttonset.buttonset.button4, WhichButton::Button4)? {
            final_goto = Some(goto);
        }
        if let Some(goto) = process_buttonset_events(&mut enigo, &dev, buttonset_events.button5, &current_buttonset.buttonset.button5, WhichButton::Button5)? {
            final_goto = Some(goto);
        }
        if let Some(goto) = process_buttonset_events(&mut enigo, &dev, buttonset_events.button6, &current_buttonset.buttonset.button6, WhichButton::Button6)? {
            final_goto = Some(goto);
        }
        if let Some(goto) = process_buttonset_events(&mut enigo, &dev, buttonset_events.button7, &current_buttonset.buttonset.button7, WhichButton::Button7)? {
            final_goto = Some(goto);
        }
        if let Some(goto) = process_buttonset_events(&mut enigo, &dev, buttonset_events.button_extra, &current_buttonset.buttonset.button_extra, WhichButton::ButtonExtra)? {
            final_goto = Some(goto);
        }
        if let Some(goto) = process_buttonset_events(&mut enigo, &dev, wheel_events.wheel_button, &current_wheel.wheel.button, WhichButton::WheelButton)? {
            final_goto = Some(goto);
        }
        if let Some(goto) = wheel_events.wheel.iter().try_fold(None, |_, event| {
            process_wheel_event(&mut enigo, &dev, event, current_wheel)
        })? {
            final_goto = Some(goto);
        }

        if let Some(goto) = final_goto.clone() {
            let new_state = state.process_goto(goto)?;
            println!("current_profile_id: {}, current_buttonset_id: {}, current_wheel_id: {}", new_state.current_profile_id, new_state.current_buttonset_id, new_state.current_wheel_id);
            if new_state.current_profile_id != state.current_profile_id {
                for action in &current_profile.active.on_exit {
                    eval(&mut enigo, &dev, action, None)?;
                }
                for action in &new_state.get_current_profile().active.on_enter {
                    eval(&mut enigo, &dev, action, None)?;
                }
            }
            if new_state.current_profile_id != state.current_profile_id || new_state.current_buttonset_id != state.current_buttonset_id {
                for action in &current_buttonset.buttonset.button0.active.on_exit {
                    eval(&mut enigo, &dev, action, Some(WhichButton::Button0))?;
                }
                for action in &current_buttonset.buttonset.button1.active.on_exit {
                    eval(&mut enigo, &dev, action, Some(WhichButton::Button1))?;
                }
                for action in &current_buttonset.buttonset.button2.active.on_exit {
                    eval(&mut enigo, &dev, action, Some(WhichButton::Button2))?;
                }
                for action in &current_buttonset.buttonset.button3.active.on_exit {
                    eval(&mut enigo, &dev, action, Some(WhichButton::Button3))?;
                }
                for action in &current_buttonset.buttonset.button4.active.on_exit {
                    eval(&mut enigo, &dev, action, Some(WhichButton::Button4))?;
                }
                for action in &current_buttonset.buttonset.button5.active.on_exit {
                    eval(&mut enigo, &dev, action, Some(WhichButton::Button5))?;
                }
                for action in &current_buttonset.buttonset.button6.active.on_exit {
                    eval(&mut enigo, &dev, action, Some(WhichButton::Button6))?;
                }
                for action in &current_buttonset.buttonset.button7.active.on_exit {
                    eval(&mut enigo, &dev, action, Some(WhichButton::Button7))?;
                }
                for action in &current_buttonset.buttonset.button_extra.active.on_exit {
                    eval(&mut enigo, &dev, action, Some(WhichButton::ButtonExtra))?;
                }
                for action in &current_buttonset.active.on_exit {
                    eval(&mut enigo, &dev, action, None)?;
                }

                for action in &new_state.get_current_buttonset().active.on_enter {
                    eval(&mut enigo, &dev, action, None)?;
                }
                for action in &new_state.get_current_buttonset().buttonset.button0.active.on_enter {
                    eval(&mut enigo, &dev, action, Some(WhichButton::Button0))?;
                }
                for action in &new_state.get_current_buttonset().buttonset.button1.active.on_enter {
                    eval(&mut enigo, &dev, action, Some(WhichButton::Button1))?;
                }
                for action in &new_state.get_current_buttonset().buttonset.button2.active.on_enter {
                    eval(&mut enigo, &dev, action, Some(WhichButton::Button2))?;
                }
                for action in &new_state.get_current_buttonset().buttonset.button3.active.on_enter {
                    eval(&mut enigo, &dev, action, Some(WhichButton::Button3))?;
                }
                for action in &new_state.get_current_buttonset().buttonset.button4.active.on_enter {
                    eval(&mut enigo, &dev, action, Some(WhichButton::Button4))?;
                }
                for action in &new_state.get_current_buttonset().buttonset.button5.active.on_enter {
                    eval(&mut enigo, &dev, action, Some(WhichButton::Button5))?;
                }
                for action in &new_state.get_current_buttonset().buttonset.button6.active.on_enter {
                    eval(&mut enigo, &dev, action, Some(WhichButton::Button6))?;
                }
                for action in &new_state.get_current_buttonset().buttonset.button7.active.on_enter {
                    eval(&mut enigo, &dev, action, Some(WhichButton::Button7))?;
                }
                for action in &new_state.get_current_buttonset().buttonset.button_extra.active.on_enter {
                    eval(&mut enigo, &dev, action, Some(WhichButton::ButtonExtra))?;
                }
            }
            if new_state.current_profile_id != state.current_profile_id || new_state.current_wheel_id != state.current_wheel_id {
                for action in &current_wheel.wheel.button.active.on_exit {
                    eval(&mut enigo, &dev, action, Some(WhichButton::WheelButton))?;
                }
                for action in &current_wheel.active.on_exit {
                    eval(&mut enigo, &dev, action, None)?;
                }
                for action in &new_state.get_current_wheel().active.on_enter {
                    eval(&mut enigo, &dev, action, None)?;
                }
                for action in &new_state.get_current_wheel().wheel.button.active.on_enter {
                    eval(&mut enigo, &dev, action, Some(WhichButton::WheelButton))?;
                }
            }
            state = new_state;
        }
    }
}
