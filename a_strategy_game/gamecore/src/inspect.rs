use crate::prelude::*;

impl G {
    pub fn inspect(&self, e: Entity) {
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

/// Utility for implementing a nice Debug
pub fn pretty_print(f: &mut fmt::Formatter<'_>, v: &impl Serialize) -> fmt::Result {
    let pretty_cfg = ron::ser::PrettyConfig::new();
    ron::ser::to_writer_pretty(f, v, pretty_cfg).map_err(|_| fmt::Error)
}
