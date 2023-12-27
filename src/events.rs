use std::time::Instant;

// use crate::actions;

pub enum ButtonState {
    Pressed,
    Released,
    Unknown
}

impl From<bool> for ButtonState {
    fn from(b: bool) -> Self {
        if b {
            ButtonState::Pressed
        } else {
            ButtonState::Released
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub enum ButtonStateMachine {
    #[default]
    Idle,
    Pressed(Instant),
    LongPressed,
    WaitingForClick(Instant, u8),
    NonFirstPressed(Instant, u8),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum ButtonEvent {
    OnPress,
    OnRelease,
    OnLongPress,
    OnClickPress(u8),
    OnClick(u8),
    OnClickRelease(u8),
}

impl ButtonStateMachine {
    pub fn transition(self, event: ButtonState, when: Instant) -> (Self, Vec<ButtonEvent>) {
        match (self, event) {
            (ButtonStateMachine::Idle, ButtonState::Pressed) => {
                (ButtonStateMachine::Pressed(when), vec![ButtonEvent::OnPress, ButtonEvent::OnClickPress(1)])
            },
            (ButtonStateMachine::Idle, _) => {
                (ButtonStateMachine::Idle, vec![])
            },
            (ButtonStateMachine::Pressed(pressed_at), ButtonState::Released) => {
                let duration = when.duration_since(pressed_at);
                if duration.as_millis() < 500 {
                    (ButtonStateMachine::WaitingForClick(when, 1), vec![ButtonEvent::OnRelease, ButtonEvent::OnClickRelease(1)])
                } else {
                    (ButtonStateMachine::LongPressed, vec![ButtonEvent::OnLongPress])
                }
            }
            (ButtonStateMachine::Pressed(pressed_at), _) => {
                let duration = when.duration_since(pressed_at);
                if duration.as_millis() > 500 {
                    (ButtonStateMachine::LongPressed, vec![ButtonEvent::OnLongPress])
                } else {
                    (ButtonStateMachine::Pressed(pressed_at), vec![])
                }
            }
            (ButtonStateMachine::LongPressed, ButtonState::Released) => {
                (ButtonStateMachine::Idle, vec![ButtonEvent::OnRelease])
            }
            (ButtonStateMachine::LongPressed, _) => {
                (ButtonStateMachine::LongPressed, vec![])
            }
            (ButtonStateMachine::WaitingForClick(_, count), ButtonState::Pressed) => {
                (ButtonStateMachine::NonFirstPressed(when, count), vec![ButtonEvent::OnPress, ButtonEvent::OnClickPress(count + 1)])
            }
            (ButtonStateMachine::WaitingForClick(pressed_at, count), _) => {
                let duration = when.duration_since(pressed_at);
                if duration.as_millis() < 400 {
                    (ButtonStateMachine::WaitingForClick(pressed_at, count), vec![])
                } else {
                    (ButtonStateMachine::Idle, vec![ButtonEvent::OnClick(count)])
                }
            }
            (ButtonStateMachine::NonFirstPressed(pressed_at, count), ButtonState::Released) => {
                (ButtonStateMachine::WaitingForClick(pressed_at, count + 1), vec![ButtonEvent::OnRelease, ButtonEvent::OnClickRelease(count + 1)])
            }
            (ButtonStateMachine::NonFirstPressed(pressed_at, count), _) => {
                (ButtonStateMachine::NonFirstPressed(pressed_at, count), vec![])
            }
        }
    }
}

pub enum WheelState {
    Unknown,
    RotatingClockwise,
    RotatingCounterClockwise,
}

#[derive(Debug, Copy, Clone, Default)]
pub enum WheelStateMachine {
    #[default]
    Idle,
    RotatingClockwise(Instant),
    RotatingCounterClockwise(Instant),
}

impl From<xencelabs_quick_keys::Event> for WheelState {
    fn from(b: xencelabs_quick_keys::Event) -> Self {
        match b {
            xencelabs_quick_keys::Event::Wheel { direction } => {
                match direction {
                    xencelabs_quick_keys::WheelDirection::Right => WheelState::RotatingClockwise,
                    xencelabs_quick_keys::WheelDirection::Left => WheelState::RotatingCounterClockwise,
                }
            },
            _ => WheelState::Unknown,
        }
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum WheelEvent {
    OnRotateClockwiseStart,
    OnRotateClockwiseStep,
    OnRotateClockwiseEnd,
    OnRotateCounterClockwiseStart,
    OnRotateCounterClockwiseStep,
    OnRotateCounterClockwiseEnd,
}

impl WheelStateMachine {
    pub fn transition(self, event: WheelState, when: Instant) -> (Self, Vec<WheelEvent>) {
        match (self, event) {
            (WheelStateMachine::Idle, WheelState::Unknown) => {
                (WheelStateMachine::Idle, vec![])
            },
            (WheelStateMachine::Idle, WheelState::RotatingClockwise) => {
                (WheelStateMachine::RotatingClockwise(when), vec![WheelEvent::OnRotateClockwiseStart])
            },
            (WheelStateMachine::Idle, WheelState::RotatingCounterClockwise) => {
                (WheelStateMachine::RotatingCounterClockwise(when), vec![WheelEvent::OnRotateCounterClockwiseStart])
            },
            (WheelStateMachine::RotatingClockwise(started_at), WheelState::Unknown) => {
                if when.duration_since(started_at).as_millis() < 500 {
                    (WheelStateMachine::RotatingClockwise(started_at), vec![])
                } else {
                    (WheelStateMachine::Idle, vec![WheelEvent::OnRotateClockwiseEnd])
                }
            },
            (WheelStateMachine::RotatingClockwise(_), WheelState::RotatingClockwise) => {
                (WheelStateMachine::RotatingClockwise(when), vec![WheelEvent::OnRotateClockwiseStep])
            },
            (WheelStateMachine::RotatingClockwise(_), WheelState::RotatingCounterClockwise) => {
                (WheelStateMachine::RotatingCounterClockwise(when), vec![WheelEvent::OnRotateClockwiseEnd, WheelEvent::OnRotateCounterClockwiseStart])
            },
            (WheelStateMachine::RotatingCounterClockwise(started_at), WheelState::Unknown) => {
                if when.duration_since(started_at).as_millis() < 500 {
                    (WheelStateMachine::RotatingCounterClockwise(started_at), vec![])
                } else {
                    (WheelStateMachine::Idle, vec![WheelEvent::OnRotateCounterClockwiseEnd])
                }
            },
            (WheelStateMachine::RotatingCounterClockwise(_), WheelState::RotatingCounterClockwise) => {
                (WheelStateMachine::RotatingCounterClockwise(when), vec![WheelEvent::OnRotateCounterClockwiseStep])
            },
            (WheelStateMachine::RotatingCounterClockwise(_), WheelState::RotatingClockwise) => {
                (WheelStateMachine::RotatingClockwise(when), vec![WheelEvent::OnRotateCounterClockwiseEnd, WheelEvent::OnRotateClockwiseStart])
            },
        }
    }
}
