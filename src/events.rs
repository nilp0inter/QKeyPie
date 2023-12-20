use std::time::Instant;

// let now = Instant::now();

pub enum ButtonState {
    Pressed,
    Released,
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

#[derive(Debug, Copy, Clone)]
pub enum ClickStateMachine {
    Idle,
    Pressed(Instant),
    LongPressed,
    WaitingForClick(Instant, u8),
    NonFirstPressed(Instant, u8),
}

impl Default for ClickStateMachine {
    fn default() -> Self {
        ClickStateMachine::Idle
    }
}

#[derive(Debug)]
pub enum Event {
    OnPress,
    OnRelease,
    OnLongPress,
    OnClick(u8),
}


impl ClickStateMachine {
    pub fn transition(&self, event: ButtonState, when: Instant) -> (Self, Vec<Event>) {
        match (self.clone(), event) {
            (ClickStateMachine::Idle, ButtonState::Pressed) => {
                (ClickStateMachine::Pressed(when), vec![Event::OnPress])
            },
            (ClickStateMachine::Idle, ButtonState::Released) => {
                (ClickStateMachine::Idle, vec![])
            },
            (ClickStateMachine::Pressed(pressed_at), ButtonState::Released) => {
                let duration = when.duration_since(pressed_at);
                if duration.as_millis() < 500 {
                    (ClickStateMachine::WaitingForClick(when, 1), vec![Event::OnRelease])
                } else {
                    (ClickStateMachine::LongPressed, vec![Event::OnLongPress])
                }
            }
            (ClickStateMachine::Pressed(pressed_at), ButtonState::Pressed) => {
                let duration = when.duration_since(pressed_at.clone());
                if duration.as_millis() > 500 {
                    (ClickStateMachine::LongPressed, vec![Event::OnLongPress])
                } else {
                    (ClickStateMachine::Pressed(pressed_at), vec![])
                }
            }
            (ClickStateMachine::LongPressed, ButtonState::Pressed) => {
                (ClickStateMachine::LongPressed, vec![])
            }
            (ClickStateMachine::LongPressed, ButtonState::Released) => {
                (ClickStateMachine::Idle, vec![Event::OnRelease])
            }
            (ClickStateMachine::WaitingForClick(_, count), ButtonState::Pressed) => {
                (ClickStateMachine::NonFirstPressed(when, count), vec![Event::OnPress])
            }
            (ClickStateMachine::WaitingForClick(pressed_at, count), ButtonState::Released) => {
                let duration = when.duration_since(pressed_at);
                if duration.as_millis() < 250 {
                    (ClickStateMachine::WaitingForClick(pressed_at, count), vec![])
                } else {
                    (ClickStateMachine::Idle, vec![Event::OnClick(count)])
                }
            }
            (ClickStateMachine::NonFirstPressed(pressed_at, count), ButtonState::Pressed) => {
                (ClickStateMachine::NonFirstPressed(pressed_at, count), vec![])
            }
            (ClickStateMachine::NonFirstPressed(pressed_at, count), ButtonState::Released) => {
                (ClickStateMachine::WaitingForClick(pressed_at, count + 1), vec![Event::OnRelease])
            }
        }
    }
}
