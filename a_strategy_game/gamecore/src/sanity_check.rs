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
    check_pawns_home_consistency(g)?;

    Ok(())
}

fn check_pawns_home_consistency(g: &G) -> Result<()> {
    // pawn -> home
    for pawn in g.pawns() {
        if let Some(home) = pawn.home.get() {
            // home should exist
            let Some(home) = g.building(home) else { bail!("{pawn}: home building does not exist: {home}") };

            // home should have pawn on record
            if !home.workers.contains(pawn.id) {
                bail!("{pawn}: home building {home} does not have Pawn in workers: {:?}", &home.workers)
            }
        }
    }
    // home -> pawn
    for building in g.buildings() {
        for pawn in building.workers.iter() {
            // worker should exist
            let Some(pawn) = g.pawn(pawn) else { bail!("building {building} has non-exiting worker {pawn}") };

            // worker should know home building
            if pawn.home != Some(building.id) {
                bail!("{pawn} does not have home {building}");
            }
        }
    }

    Ok(())
}

fn check_pawns_on_walkable_tile(g: &G) -> Result<()> {
    for pawn in g.pawns() {
        if !g.is_walkable_by(pawn.tile(), pawn) {
            bail!("{} on non-walkable tile at {}", pawn, pawn.tile())
        }
    }
    Ok(())
}
