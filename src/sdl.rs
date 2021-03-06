use sdl2::joystick::*;
use sdl2::event::Event;
use sdl2::{init, Sdl, INIT_GAME_CONTROLLER, INIT_EVENTS};

use std::borrow::Cow;
use std::error::Error;
use std::fmt;
use std::rc::Rc;

/// A native joystick using SDL
pub struct NativeJoystick {
    js: Joystick,
    sdl: Option<Rc<Sdl>>
}

impl NativeJoystick {
    /// Set the context of the joystick
    pub fn in_context(mut self, sdl: Rc<Sdl>) -> NativeJoystick {
        self.sdl = Some(sdl);
        self
    }
}

/// Convert the SDL event into a Reminisce event
pub fn convert_event(event: Event) -> ::Event {
    use std::mem::transmute as cast;
    match event {
        Event::JoyAxisMotion {axis_idx, value, ..} => {
            let index = unsafe { cast(axis_idx) };
            ::Event::AxisMoved(index, value)
        },
        Event::JoyButtonDown {button_idx, ..} => {
            let index = unsafe { cast(button_idx) };
            ::Event::ButtonPressed(index)
        },
        Event::JoyButtonUp {button_idx, ..} => {
            let index = unsafe { cast(button_idx) };
            ::Event::ButtonReleased(index)
        },
        _ => unimplemented!()
    }
}
/// Scan for joysticks and initialise SDL
pub fn scan() -> Vec<NativeJoystick> {
    let flags = INIT_GAME_CONTROLLER | INIT_EVENTS;
    let sdl = Rc::new(init(flags).unwrap());
    let num = num_joysticks().unwrap() as u8;
    (0..num).filter_map(|i| ::Joystick::open(i).ok().map(|js:NativeJoystick| js.in_context(sdl.clone()))).collect()
}

pub struct OpenError {
    err: String
}
impl Error for OpenError {
    fn description(&self) -> &str {
        &self.err
    }
    fn cause(&self) -> Option<&Error> {
        None
    }
}
impl fmt::Debug for OpenError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.err)
    }
}
impl fmt::Display for OpenError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.err)
    }
}

impl ::Joystick for NativeJoystick {
    type WithState = NativeJoystick;
    type NativeEvent = Event;
    type OpenError = OpenError;
    fn open(index: u8) -> Result<NativeJoystick, OpenError> {
        match Joystick::open(index as i32) {
            Ok(js) => Ok(NativeJoystick { js: js, sdl: None }),
            Err(err) => Err(OpenError { err: err })
        }
    }
    fn is_connected(&self) -> bool {
        self.js.get_attached()
    }
    fn get_index(&self) -> u8 {
        self.js.get_instance_id().unwrap() as u8
    }
    fn get_id(&self) -> Cow<str> {
        self.js.name().into()
    }
    fn get_num_buttons(&self) -> u8 {
        self.js.get_num_buttons().unwrap() as u8
    }
    fn get_num_axes(&self) -> u8 {
        self.js.get_num_axis().unwrap() as u8
    }
    fn get_battery(&self) -> Option<f32> {
        None
    }
    fn poll_native(&mut self) -> Option<Event> {
        if self.sdl.is_none() {
            let flags = INIT_GAME_CONTROLLER | INIT_EVENTS;
            self.sdl = Some(Rc::new(init(flags).unwrap()))
        }
        let sdl = self.sdl.clone().unwrap();
        let mut pump = sdl.event_pump();
        for event in pump.poll_iter() {
            match event {
                Event::JoyAxisMotion{ .. } | Event::JoyButtonDown{ .. } | Event::JoyButtonUp{ .. } => {
                    return Some(event)
                },
                _ => ()
            }
        }
        None
    }
    fn with_state(self) -> NativeJoystick {
        self
    }
}

impl ::StatefulJoystick for NativeJoystick {
    fn get_axis(&self, axis: ::Axis) -> Option<i16> {
        self.js.get_axis(axis as u8 as i32).ok()
    }
    fn get_button(&self, button: ::Button) -> Option<bool> {
        self.js.get_button(button as u8 as i32).ok()
    }
    fn update(&mut self) {
        update();
    }
}
