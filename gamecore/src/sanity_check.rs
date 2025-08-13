//! Sanity check (internal consistency) on each tick.
//! Enabled with `debug_assertions`.
//!
//! Failed check will:
//!   * Fail tests.
//!   * Pause the game and print error, if `debug_assertions`.
//!   * Be ignored in release builds.
use crate::prelude::*;
use anyhow::bail;

pub(crate) fn sanity_check(g: &G) -> Result<()> {
    check_pawns_on_walkable_tile(g)?;

    Ok(())
}

fn check_pawns_on_walkable_tile(g: &G) -> Result<()> {
    for pawn in g.pawns() {
        if !g.is_walkable(pawn.tile()) {
            bail!("{:?} {} on non-walkable tile at {}", pawn.typ, pawn.id, pawn.tile())
        }
    }
    Ok(())
}
