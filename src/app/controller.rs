use gilrs::{Axis, Button, EventType};
use iced::{subscription, Subscription};

pub fn read_controller() -> Subscription<ControlEvent> {
    struct ControllerHandle;

    subscription::channel(
        std::any::TypeId::of::<ControllerHandle>(),
        100,
        |mut output| async move {
            // Anything sent on output with output.send needs to be a controller::ControllerEvent
            let mut gilrs = gilrs::Gilrs::new().expect("Couldn't create gilrs controller handle");
            let mut directions = DirectionToggles::new();

            tokio::task::spawn_blocking(move || loop {
                // Todo, check for keyboard input as well as gamepad input
                if let Some(event) = gilrs.next_event_blocking(None) {
                    match event.event {
                        EventType::ButtonPressed(Button::South, _) => {
                            output
                                .try_send(ControlEvent::Select)
                                .expect("Failed to send input to App");
                        }
                        EventType::ButtonPressed(Button::East, _) => {
                            output
                                .try_send(ControlEvent::Back)
                                .expect("Failed to send input to App");
                        }
                        EventType::ButtonPressed(Button::North, _) => {
                            output
                                .try_send(ControlEvent::Search)
                                .expect("Failed to send input to App");
                        }
                        EventType::AxisChanged(Axis::LeftStickX, amt, _) => {
                            if amt >= 0.34 && !directions.right {
                                directions.right = true;
                                output
                                    .try_send(ControlEvent::Right)
                                    .expect("Failed to send input to App");
                            }
                            if amt <= -0.34 && !directions.left {
                                directions.left = true;
                                output
                                    .try_send(ControlEvent::Left)
                                    .expect("Failed to send input to App");
                            }
                            if (-0.32..=0.34).contains(&amt) {
                                directions.left = false;
                                directions.right = false;
                            }
                        }
                        EventType::AxisChanged(Axis::LeftStickY, amt, _) => {
                            if amt >= 0.34 && !directions.up {
                                directions.up = true;
                                output
                                    .try_send(ControlEvent::Up)
                                    .expect("Failed to send input to App");
                            }
                            if amt <= -0.34 && !directions.down {
                                directions.down = true;
                                output
                                    .try_send(ControlEvent::Down)
                                    .expect("Failed to send input to App");
                            }
                            if (-0.32..=0.34).contains(&amt) {
                                directions.up = false;
                                directions.down = false;
                            }
                        }
                        _ => {}
                    }
                }
            })
            .await
            .expect("Failed to spawn tokio blocking thread");
            loop {
                iced::futures::pending!();
            }
        },
    )
}

#[derive(Debug, Clone)]
pub enum ControlEvent {
    Select,
    Back,
    Search,
    Left,
    Right,
    Up,
    Down,
}

struct DirectionToggles {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl DirectionToggles {
    fn new() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
}
