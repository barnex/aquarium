use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Route {
    i: Cel<u32>,
    steps: RefCell<Box<[vec2i16]>>,
}

impl Route {
    pub fn next(&self) -> Option<vec2i16> {
        match self.steps.borrow().get(self.i.get() as usize) {
            Some(pos) => {
                self.i.add(1);
                Some(*pos)
            }
            None => None,
        }
    }

    pub(crate) fn set(&self, path: Vec<vec2i16>) {
        self.i.set(0);
        *self.steps.borrow_mut() = path.into_boxed_slice();
    }
}
