use crate::prelude::*;
use std::fmt::Write;

/// Accumulates input events since the last tick,
/// allowing for queries like "is this key currently held down?".
///
/// Also de-bounces events faster than a tick,
/// and removes OS key repeats.
#[derive(Debug, Default)]
pub struct Inputs {
    buttons_pressed: HashSet<Button>,
    buttons_down: HashSet<Button>,
    buttons_released: HashSet<Button>,

    received_characters: String,

    // updated by engine
    pub(crate) console_input: Option<String>,
    pub(crate) raw_mouse_delta: vec2f,
    pub(crate) filtered_mouse_delta: vec2f,
    mouse_position: vec2i,

    keymap: Keymap,

    mouse_wheel: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Button(pub Str16);

impl Button {
    pub const SCREENSHOT: Self = Self(Str16::from_slice(b"F12\0\0\0\0\0\0\0\0\0\0\0\0\0"));
    pub const MOUSE1: Self = Self(Str16::from_slice(b"Mouse1\0\0\0\0\0\0\0\0\0\0"));
    pub const MOUSE2: Self = Self(Str16::from_slice(b"Mouse2\0\0\0\0\0\0\0\0\0\0"));
    pub const FORWARD: Self = Self(Str16::from_slice(b"Forward\0\0\0\0\0\0\0\0\0"));
    pub const BACKWARD: Self = Self(Str16::from_slice(b"Backward\0\0\0\0\0\0\0\0"));
    pub const LEFT: Self = Self(Str16::from_slice(b"Left\0\0\0\0\0\0\0\0\0\0\0\0"));
    pub const RIGHT: Self = Self(Str16::from_slice(b"Right\0\0\0\0\0\0\0\0\0\0\0"));
    pub const CROUCH: Self = Self(Str16::from_slice(b"KeyZ\0\0\0\0\0\0\0\0\0\0\0\0"));
    pub const JUMP: Self = Self(Str16::from_slice(b"Space\0\0\0\0\0\0\0\0\0\0\0"));
    pub const ESC: Self = Self(Str16::from_slice(b"Escape\0\0\0\0\0\0\0\0\0\0"));
    pub const TAB: Self = Self(Str16::from_slice(b"Tab\0\0\0\0\0\0\0\0\0\0\0\0\0"));
    pub const SPACE: Self = Self(Str16::from_slice(b"Space\0\0\0\0\0\0\0\0\0\0\0"));
    pub const DIGIT1: Self = Self(Str16::from_slice(b"Digit1\0\0\0\0\0\0\0\0\0\0"));
    pub const DIGIT2: Self = Self(Str16::from_slice(b"Digit2\0\0\0\0\0\0\0\0\0\0"));
    pub const DIGIT3: Self = Self(Str16::from_slice(b"Digit3\0\0\0\0\0\0\0\0\0\0"));
    pub const DIGIT4: Self = Self(Str16::from_slice(b"Digit4\0\0\0\0\0\0\0\0\0\0"));
    pub const DIGIT5: Self = Self(Str16::from_slice(b"Digit5\0\0\0\0\0\0\0\0\0\0"));
    pub const DIGIT6: Self = Self(Str16::from_slice(b"Digit6\0\0\0\0\0\0\0\0\0\0"));
    pub const DIGIT7: Self = Self(Str16::from_slice(b"Digit7\0\0\0\0\0\0\0\0\0\0"));
    pub const DIGIT8: Self = Self(Str16::from_slice(b"Digit8\0\0\0\0\0\0\0\0\0\0"));
    pub const DIGIT9: Self = Self(Str16::from_slice(b"Digit9\0\0\0\0\0\0\0\0\0\0"));
    pub const DIGIT0: Self = Self(Str16::from_slice(b"Digit0\0\0\0\0\0\0\0\0\0\0"));
}

impl Inputs {
    // To be called on each frame to advance time.
    // "just pressed" evolves to "is_down".
    // "released" gets forgotten.
    pub fn start_next_frame(&mut self) {
        // Note: NOT clearing bottons_down.
        self.buttons_pressed.clear();
        self.buttons_released.clear();
        self.received_characters.clear();
        self.raw_mouse_delta = vec2::ZERO;
        self.filtered_mouse_delta = vec2::ZERO;
    }

    /// Is a button currently held down?
    /// (This repeats on every tick for as long as the button is held)
    pub fn is_down(&self, but: Button) -> bool {
        self.buttons_down.contains(&but)
    }

    /// Was a button pressed right before the current tick?
    /// This triggers only once per physical keypress.
    /// OS keyboard repeats are ignored.
    pub fn just_pressed(&self, but: Button) -> bool {
        self.buttons_pressed.contains(&but)
    }

    /// Was a button released right before the current tick?
    pub fn just_released(&self, but: Button) -> bool {
        self.buttons_released.contains(&but)
    }

    /// Iterate over all keys currently held down.
    pub fn iter_is_down(&self) -> impl Iterator<Item = Button> + use<'_> {
        self.buttons_down.iter().cloned()
    }

    /// Iterate over all keys pressed just before this tick.
    pub fn iter_just_pressed(&self) -> impl Iterator<Item = Button> + use<'_> {
        self.buttons_pressed.iter().cloned()
    }

    /// Iterate over all keys released right before this tick.
    pub fn iter_just_released(&self) -> impl Iterator<Item = Button> + use<'_> {
        self.buttons_released.iter().cloned()
    }

    /// The unicode characters typed since the last tick.
    pub fn received_characters(&self) -> &str {
        &self.received_characters
    }

    /// Command typed in the engine console.
    pub fn console_input(&self) -> Option<&str> {
        self.console_input.as_deref()
    }

    /// Mouse movement since last frame.
    /// Only recorded when the mouse cursor is grabbed ([`grab_cursor`]).
    pub fn mouse_delta(&self) -> vec2f {
        self.filtered_mouse_delta
    }

    /// Mouse position in logical pixels, relative to the top-left corner of the window.
    /// Useful when cursor is not grabbed.
    pub fn mouse_position(&self) -> vec2i {
        self.mouse_position
    }

    /// Record that this button was just pressed.
    pub fn record_press(&mut self, button: Button) {
        let button = self.keymap.map(button);
        if !self.buttons_down.contains(&button) {
            self.buttons_pressed.insert(button);
            self.buttons_down.insert(button);
        }
    }

    /// Record that this button was just released.
    pub fn record_release(&mut self, button: Button) {
        let button = self.keymap.map(button);
        self.buttons_released.insert(button);
        self.buttons_down.remove(&button);
    }

    pub fn record_mouse_position(&mut self, pos: vec2i) {
        self.mouse_position = pos
    }

    fn make_button(&mut self, key: impl Debug) -> Button {
        let mut buf = Str16::default();
        write!(&mut buf, "{key:?}").inspect_err(|_| log::error!("input too long: {key:?}")).swallow_err();
        Button(buf)
    }

    // The relative mouse wheel movement since last tick.
    //pub fn mouse_wheel_delta(&self) -> i32 {
    //	let mut delta = 0;
    //	if self.just_pressed(Button::MouseWheelDown) {
    //		delta -= 1;
    //	}
    //	if self.just_pressed(Button::MouseWheelUp) {
    //		delta += 1;
    //	}
    //	delta
    //}

    //fn record_mouse_wheel(&mut self, delta: &winit::event::MouseScrollDelta) {
    //	/*
    //		Mouse wheel delta's can vary wildly,
    //		reduce them just a single Up / Down event
    //		discarding the scroll amount.
    //	*/
    //	let dy = match delta {
    //		winit::event::MouseScrollDelta::LineDelta(_, y) => *y,
    //		winit::event::MouseScrollDelta::PixelDelta(PhysicalPosition { y, .. }) => *y as f32,
    //	};
    //	let button = match dy {
    //		_ if dy > 0.0 => Some(Button::MouseWheelUp),
    //		_ if dy < 0.0 => Some(Button::MouseWheelDown),
    //		_ => None,
    //	};
    //	/*
    //		Record both a press and release
    //		to make the scroll event appear as a button press
    //		(the scroll wheel cannot be "held down" continuously like a mouse button).
    //	*/
    //	if let Some(button) = button {
    //		self.buttons_pressed.insert(button);
    //		self.buttons_released.insert(button);
    //	}
    //}
}
