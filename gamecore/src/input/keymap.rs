use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Keymap(pub HashMap<Button, Button>);

impl Keymap {
    pub(crate) fn map(&self, button: Button) -> Button {
        self.0.get(&button).copied().unwrap_or(button)
    }
}
