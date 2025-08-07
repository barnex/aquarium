use crate::prelude::*;

#[derive(Serialize, Deserialize, Default)]
pub struct DebugOpts {
    pub show_walkable: bool,
    pub show_buildable: bool,
    pub show_home: bool,
}

