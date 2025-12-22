use crate::prelude::*;

impl G {
    //pub(crate) fn tick_inspect(&self) {
    //    for e in self.inspected.iter().filter_map(|id| self.entities.get(id)) {
    //        inspect(e)
    //    }
    //}

    pub(crate) fn inspect(&self, e: &EntityStorage) {
        let pretty_cfg = ron::ser::PrettyConfig::new();
        match ron::ser::to_string_pretty(e, pretty_cfg) {
            Ok(str) => self.console.print(format!("{str}")),
            Err(err) => self.console.print(format!("{err}")),
        }
    }
}
