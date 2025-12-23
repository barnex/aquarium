use crate::prelude::*;

impl G {
    pub fn inspect(&self, e: &dyn Entity) {
        self.console.print(format!("{e:?}"))
    }

    //pub(crate) fn inspect(&self, e: &EntityStorage) {
    //    let pretty_cfg = ron::ser::PrettyConfig::new();
    //    match ron::ser::to_string_pretty(e, pretty_cfg) {
    //        Ok(str) => self.console.print(format!("{str}")),
    //        Err(err) => self.console.print(format!("{err}")),
    //    }
    //}
}
