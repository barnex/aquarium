pub(crate) mod prelude;

mod animation;
mod control;
mod debugging;
mod drawing;
mod effects;
mod entities;
mod extension_traits;
mod game_commands;
mod gamestate;
mod inspect;
mod keybindings;
mod map_gen;
mod monotonic_clock;
mod pathfinder;
mod random;
mod resources;
mod sanity_check;
mod tilemap;
mod tracing;
mod ui;
mod water_sim;

#[cfg(test)]
mod tests;

pub use animation::*;
pub use control::*;
pub use debugging::*;
pub use drawing::*;
pub use effects::*;
pub use entities::*;
pub use game_commands::*;
pub use gamestate::*;
pub use inspect::*;
pub use keybindings::*;
pub use monotonic_clock::*;
pub use pathfinder::*;
pub use random::*;
pub use resources::*;
pub use sanity_check::*;
pub use tilemap::*;
pub use tracing::*;
pub use ui::*;
pub use water_sim::*;

pub(crate) use extension_traits::*;

/// Name of the function this is called from.
#[macro_export]
macro_rules! caller {
    () => {{
        fn f() {}
        let name = std::any::type_name_of_val(&f);
        let name = name.strip_suffix("::f").expect("test_name");
        name.split("::").last().expect("test_name")
    }};
}
