use crate::prelude::*;

impl Pawn {
    /// 📦📍 Called when a pawn has reached their destination and thus can pick up/drop off their cargo.
    /// TODO: Remove Status returns, always check if the status is as desired (e.g. destination.is_some());
    /// TODO: Simplify, avoid deadlocks?
    pub(crate) fn tick_delivery_work(&self, g: &G, home: &Building) {
        trace!(self);

        //👇 corner case: cargo can be on top of a building :(
        // TODO: Should not be allowed to walk on a building.
        // TODO: There should not be resources on top of a building.
        self.try_pick_up_cargo(g, home);

        let on_building = g.building_at(self.tile());
        let on_home = on_building.map(|b| b.id()) == Some(home.id());

        match on_building {
            _ if on_home => self.tick_on_home(g, home),
            Some(building) => self.tick_on_other_building(g, home, building),
            None => self.tick_away_from_building(g, home),
        }
    }

    ///   +home+
    ///   | 🦀 |    ☘️️️ ☘️️
    ///   +----+
    fn tick_on_home(&self, g: &G, home: &Building) {
        trace!(self, "{home}");

        self.try_deliver_cargo(home);

        if let Some(cargo) = self.cargo() {
            trace!(self, "failed to deliver cargo");
            // TODO: drop cargo if undelivered after a very long time. Might need hands to take output instead.
            if !home.has_input(cargo) {
                // unlikely, but possible e.g. when a worker who was already carrying something got re-assigned.
                trace!(self, "home does not have {cargo} input");
                self.drop_cargo(g);
            }
            self.sleep(Self::FAIL_DELAY);
            return;
        }

        // 🙌 Hands are now empty.
        debug_assert!(self.cargo().is_none());
        // Decide what to do next
        // Try serve the input/output in biggest need.
        debug_assert!(self.cargo().is_none());
        #[derive(Debug)]
        enum InOut {
            In,
            Out,
        }
        // inputs/outputs, sorted by who has the biggest need.
        // Input need = how empty they are, Output need is how full they are.
        // E.g. if the input is 60% full and and the output is 90% full, the output has the biggest need.
        for (io, slot) in home
            .inputs() //_
            .filter_map(|s| (!s.is_full()).then_some((InOut::In, s)))
            .chain(home.outputs().filter_map(|s| (!s.is_empty()).then_some((InOut::Out, s))))
            .sorted_by_key(|(io, s)| match io {
                InOut::In => s.fullness_pct() as i32,
                InOut::Out => 100 - (s.fullness_pct() as i32),
            })
        {
            trace!(self, "considering resource slot: {io:?} {} {}% full", slot.typ, slot.fullness_pct());
            match io {
                InOut::In => {
                    if let Some(tile) = self.find_near_resource(g, slot.typ) {
                        trace!(self, "most urgent: collect {}", slot.typ);
                        self.set_destination(g, tile);
                        return; //👈
                    }
                }
                InOut::Out => {
                    if let Some(building) = self.find_near_receptor(g, slot.typ) {
                        trace!(self, "most urgent: bring {} to {building}", slot.typ);
                        self.cargo.set(slot.try_take_one());
                        debug_assert!(self.cargo().is_some());
                        self.set_destination(g, building.tile());
                        return; //👈
                    }
                }
            }
        }
    }

    fn drop_cargo(&self, g: &G) -> Status {
        trace!(self, "{:?}", self.cargo());
        debug_assert!(self.cargo().is_some());
        g.resources.insert(self.tile(), self.cargo.take()?);
        OK
    }

    /// +-HQ-+   +home+
    /// | 🦀 |   |    |    ☘️️️ ☘️️
    /// +----+   +----+
    fn tick_on_other_building(&self, g: &G, home: &Building, building: &Building) {
        trace!(self, "on {building} with cargo {:?}", self.cargo());
        self.try_deliver_cargo(building);

        self.steal_any_resource(g, home, building);

        // Still holding cargo
        if let Some(cargo) = self.cargo() {
            if home.has_input(cargo) {
                trace!(self, "home has {cargo} input, going there");
                self.go_home(g);
                return;
            } else {
                if let Some(building) = self.find_near_receptor(g, cargo) {
                    trace!(self, "{building} can accept {cargo} input, going there");
                    self.set_destination(g, building.entrance());
                    return;
                } else {
                    trace!(self, "nowhere to drop {cargo}");
                    self.sleep(Self::FAIL_DELAY);
                    self.take_personal_space(g);
                }
            }
        }
    }

    ///   +home+
    ///   |    |    🦀 ☘️️
    ///   +----+
    fn tick_away_from_building(&self, g: &G, home: &Building) {
        trace!(self, "with cargo {:?}", self.cargo());
        match self.cargo() {
            Some(cargo) if home.has_input(cargo) => self.go_home(g).ignore(),
            Some(cargo) => {
                if let Some(building) = self.find_near_receptor(g, cargo) {
                    self.set_destination(g, building.entrance());
                }
            }
            None => self.go_to_near_resource(g, home).or_else(|| self.go_home(g)).ignore(),
        };
    }

    fn go_to_near_resource(&self, g: &G, home: &Building) -> Status {
        if let Some(new_dest) = g.resources.iter().filter(|(_, res)| home.has_nonfull_input(*res)).min_by_key(|(tile, _)| tile.distance_squared(self.tile())).map(|(tile, _)| tile) {
            trace!(self, "found {new_dest}");
            self.set_destination(g, new_dest)
        } else {
            trace!(self, "None found");
            FAIL
        }
    }

    #[must_use]
    fn find_near_resource(&self, g: &G, typ: ResourceTyp) -> Option<vec2i16> {
        g.resources //__
            .iter()
            .filter(|&(_, res)| res == typ)
            .min_by_key(|(tile, _)| tile.distance_squared(self.tile()))
            .map(|(tile, _)| tile)
            .or_else(|| self.find_near_provider(g, typ).map(|b| b.tile()))
            .with(|v| trace!(self, "find_near_resource: {v:?}"))
    }

    fn find_near_receptor<'g>(&self, g: &'g G, res: ResourceTyp) -> Option<&'g Building> {
        g.buildings() //__
            .filter(|b| b.has_nonfull_input(res))
            .min_by_key(|b| b.tile().distance_squared(self.tile()))
            .with(|v| trace!(self, "find_near_receptor: {v:?}"))
    }

    fn find_near_provider<'g>(&self, g: &'g G, res: ResourceTyp) -> Option<&'g Building> {
        g.buildings() //__
            .filter(|b| b.has_nonempty_output(res))
            .min_by_key(|b| b.tile().distance_squared(self.tile()))
            .with(|v| trace!(self, "find_near_provider: {v:?}"))
    }

    fn go_home(&self, g: &G) -> Status {
        trace!(self);
        self.set_destination(g, g.building(self.home.get()?)?.entrance());
        OK
    }

    fn try_deliver_cargo(&self, building: &Building) -> Status {
        trace!(self, "cargo={:?} to {building}", self.cargo);

        let resource = self.cargo.take()?;
        match building.add_resource(resource) {
            OK => {
                trace!(self, "OK: delivered {resource:?}");
                OK
            }
            FAIL => {
                trace!(self, "FAILed to deliver {resource:?}");
                self.cargo.set(Some(resource));
                FAIL
            }
        }
    }

    fn try_pick_up_cargo(&self, g: &G, home: &Building) -> Status {
        let res = match g.resources.at(self.tile()) {
            Some(res) => {
                trace!(self, "found {res}");
                res
            }
            None => {
                trace!(self, "nothing here");
                return FAIL;
            }
        };

        if home.has_input(res) {
            trace!(self, "home has {res} input: picking up");
            self.cargo.set(g.resources.remove(self.tile()));
            self.sleep(Self::PICKUP_DELAY);
            debug_assert_eq!(self.cargo(), Some(res));
            OK
        } else {
            trace!(self, "home does not have {res} input");
            return FAIL;
        }
    }

    fn steal_any_resource(&self, g: &G, home: &Building, building: &Building) -> Status {
        trace!(self, "from {building}?");
        debug_assert!(self.home.get() == Some(home.id()));

        for slot in building.outputs() {
            if slot.has_at_least(1) && home.has_nonfull_input(slot.typ) {
                self.cargo.set(slot.try_take_one().with(|v| trace!(self, "steal_any_resource: took {v:?}")));
                self.go_home(g);
                return OK;
            }
        }
        FAIL
    }
}
