use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Keymap(HashMap<Button, Button>);

impl Keymap {
	pub(crate) fn map(&self, button: Button) -> Button {
		self.0.get(&button).copied().unwrap_or(button)
	}
}

impl Default for Keymap {
	fn default() -> Self {
		Self(
			[
				(Button(Str16::from_str("KeyE").unwrap()), Button::FORWARD),  //_
				(Button(Str16::from_str("KeyD").unwrap()), Button::BACKWARD), //_
				(Button(Str16::from_str("KeyS").unwrap()), Button::LEFT),     //_
				(Button(Str16::from_str("KeyF").unwrap()), Button::RIGHT),    //_
			]
			.into_iter()
			.collect(),
		)
	}
}
