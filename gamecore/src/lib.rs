pub(crate) mod prelude;

mod c_set;
mod c_vec;
mod console;
mod control;
mod debugging;
mod drawing;
mod entities;
mod extension_traits;
mod font_layout;
mod game_commands;
mod gamestate;
mod map_gen;
mod random;
mod resources;
mod sanity_check;
mod tilemap;
mod ui;
mod water_sim;

#[cfg(test)]
mod tests;

pub use c_set::*;
pub use c_vec::*;
pub use console::*;
pub use control::*;
pub use debugging::*;
pub use drawing::*;
pub use entities::*;
pub use font_layout::*;
pub use game_commands::*;
pub use gamestate::*;
pub use random::*;
pub use resources::*;
pub use sanity_check::*;
pub use tilemap::*;
pub use ui::*;
pub use water_sim::*;

pub(crate) use extension_traits::*;

pub fn init() {
    #[cfg(debug_assertions)]
    log::warn!("gamecore: debug_assertions enabled, performance will suffer");
}
