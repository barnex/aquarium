use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Keymap(pub HashMap<Button, Button>);

impl Keymap {
    pub(crate) fn map(&self, button: Button) -> Button {
        self.0.get(&button).copied().unwrap_or(button)
    }
}

impl<T> From<T> for Keymap
where
    T: IntoIterator<Item = (Button, Button)>,
{
    fn from(value: T) -> Self {
        Self(value.into_iter().collect())
    }
}
