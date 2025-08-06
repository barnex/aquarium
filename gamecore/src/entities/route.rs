use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Route {
    i: Cel<u32>,
    steps: RefCell<Vec<vec2i16>>,
}

impl Route {
    pub fn next(&self) -> Option<vec2i16> {
        match self.steps.borrow().get(self.i.get() as usize) {
            Some(pos) => {
                self.i.inc(1);
                Some(*pos)
            }
            None => None,
        }
    }

    pub(crate) fn set(&self, path: Vec<vec2i16>) {
        self.i.set(0);
        *self.steps.borrow_mut() = path;
    }

    pub fn clear(&self) {
        self.i.set(0);
        self.steps.borrow_mut().clear();
    }

    pub fn destination(&self) -> Option<vec2i16> {
        self.steps.borrow().last().copied()
    }

    pub fn is_finished(&self) -> bool {
        self.i == self.steps.borrow().len() as u32
    }
}
