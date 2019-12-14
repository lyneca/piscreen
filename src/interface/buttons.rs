use rppal::gpio::{Gpio, InputPin, Level};
use std::time::SystemTime;

const HOLD_DURATION: u128 = 500;
const TICK_DURATION: u128 = 300;

pub struct Button {
    pin: Box<InputPin>,
    last_state: Level,
    last_pressed: Option<SystemTime>,
    last_ticked: Option<SystemTime>,
    pub is_hold: bool,
    has_been_pressed: bool
}

impl Button {
    pub fn new(pin: InputPin) -> Button {
        Button {
            pin: Box::new(pin),
            last_state: Level::Low,
            last_pressed: None,
            last_ticked: None,
            is_hold: false,
            has_been_pressed: false
        }
    }

    pub fn poll(&mut self) {
        if self.pin.is_low() {
            if self.last_state == Level::Low {
                self.last_pressed = Some(SystemTime::now());
                self.has_been_pressed = true;
                self.is_hold = false;
            }
            self.last_state = Level::High;
            match self.last_pressed {
                Some(t) => {
                    if SystemTime::now().duration_since(t).unwrap().as_millis() >= HOLD_DURATION {
                        self.last_ticked = Some(SystemTime::now());
                        self.has_been_pressed = true;
                        self.is_hold = true;
                    } else {
                        self.last_ticked = None;
                    }
                }
                None => {}
            };
            match self.last_ticked {
                Some(t) => {
                    if SystemTime::now().duration_since(t).unwrap().as_millis() >= TICK_DURATION {
                        self.last_ticked = Some(SystemTime::now());
                        self.has_been_pressed = true;
                        self.is_hold = true;
                    }
                }
                None => { }
            }
        } else if self.pin.is_high() {
            self.last_state = Level::Low;
            self.last_ticked = None;
            self.last_pressed = None;
        }
    }

    pub fn was_pressed(&mut self) -> bool {
        if self.has_been_pressed {
            self.has_been_pressed = false;
            return true;
        } else {
            return false;
        }
    }
}

pub struct ButtonSet {
    pub a: Button,
    pub b: Button,
    pub c: Button,
    pub up: Button,
    pub down: Button,
    pub left: Button,
    pub right: Button,
}

impl ButtonSet {
    pub fn poll_all(&mut self) {
        self.a.poll();
        self.b.poll();
        self.c.poll();
        self.up.poll();
        self.down.poll();
        self.left.poll();
        self.right.poll();
    }

    pub fn all_buttons(&mut self) -> Vec<&mut Button> {
        vec![
            &mut self.a,
            &mut self.b,
            &mut self.c,
            &mut self.up,
            &mut self.down,
            &mut self.left,
            &mut self.right,
        ]
    }

    pub fn get_button(n: u8) -> Button {
        let gpio = Gpio::new().expect("Could not init board");
        return Button::new(gpio.get(n)
            .expect("Could not get pin")
            .into_input_pullup());
    }

    pub fn default_pins() -> ButtonSet {
        ButtonSet {
            a: ButtonSet::get_button(5),
            b: ButtonSet::get_button(6),
            c: ButtonSet::get_button(4),
            up: ButtonSet::get_button(17),
            down: ButtonSet::get_button(22),
            left: ButtonSet::get_button(27),
            right: ButtonSet::get_button(23)
        }
    }

    pub fn flush(&mut self) {
        for button in self.all_buttons() {
            button.was_pressed();
        }
    }
}
