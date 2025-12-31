use crate::prelude::*;

/// What to do upon reaching destination.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Task {
    None,
    /// Should pick up resource from map at destination.
    /// Expectation: map has this resources at destination.
    Harvest(ResourceTyp),
    /// Should take resource from factory output.
    /// TODO: Factory should drop it on the map instead?
    /// Expectation: Factory `Id` is at destination and has such output.
    TakeFromBuilding(Id, ResourceTyp),
    /// Bring your current cargo home.
    HomeDelivery,
    /// Bring your current cargo to factory `Id` at destination.
    DeliverToBuilding(Id),
}

impl Pawn {
    /// 📦📍 Called when a pawn has reached their destination and thus can pick up/drop off their cargo.
    /// TODO: Remove Status returns, always check if the status is as desired (e.g. destination.is_some());
    /// TODO: Simplify, avoid deadlocks?
    pub(crate) fn tick_delivery_work(&self, g: &G, home: &Building) {
        trace!(self, "plan: {:?}", self.task);

        // Plan == why am I here?
        match self.task.get() {
            Task::None if self.is_at_building(home) =>  self.make_plan(g, home),
            Task::None /*if not at home*/ =>  self.go_home(g),
            Task::Harvest(res) => self.harvest(g, res),
            Task::HomeDelivery => self.home_delivery(g),
            Task::DeliverToBuilding(id) => self.deliver_to_building(g, id),
            Task::TakeFromBuilding(id, res) => self.take_from_building(g, id, res),
        }
    }

    fn take_from_building(&self, g: &G, id: Id, res: ResourceTyp) {
        todo!()
        //trace!(self, "{id} {res}");
        //let Some(building) = g.building(id) else { return self.bail(g, "no such building") };
        //let Some(output) = building.output(res) else { return self.bail(g, "no output for {cargo}") };
        //debug_assert!(self.cargo().is_none());
        //if self.cargo().is_some() {
        //    return self.bail(g, "already carrying cargo");
        //}
        //match output.try_take_one(){
        //    None
        //}
    }

    fn home_delivery(&self, g: &G) {
        trace!(self, "cargo={:?}", self.cargo());
        let Some(home) = self.home(g) else { return self.bail(g, "home_delivery: no home") };

        self.try_deliver_at(g, home);
        if self.cargo().is_none() {
            trace!(self, "delivered")
        } else {
            trace!(self, "full");
            self.sleep(1);
        }
    }

    fn deliver_to_building(&self, g: &G, id: Id) {
        trace!(self, "{:?}", self.cargo());
        let Some(building) = g.building(id) else { return self.bail(g, "no such building") };
        self.try_deliver_at(g, building);
        if let Some(cargo) = self.cargo() {
            trace!(self, "failed");
            if let Some(alternative) = self.find_near_receptor(g, cargo) {
                self.set_destination(g, alternative.entrance());
                self.task.set(Task::DeliverToBuilding(alternative.id()));
            } else {
                trace!(self, "TODO: drop cargo if no building with such input even exists");
                self.sleep(Self::FAIL_DELAY);
            }
        }
    }

    fn try_deliver_at(&self, g: &G, building: &Building) {
        if !self.is_at_building(building) {
            return self.bail(g, "try_deliver_at: not at building");
        };
        let Some(cargo) = self.cargo() else { return self.bail(g, "no cargo") };
        let Some(input) = building.input(cargo) else { return self.bail(g, "no input for {cargo}") };
        match input.add_one() {
            OK => {
                self.cargo.set(None);
                self.task.set(Task::None);
                trace!(self, "delivered {cargo}")
            }
            FAIL => trace!(self, "failed"),
        }
    }

    fn harvest(&self, g: &G, res: ResourceTyp) {
        trace!(self, "{res}");
        debug_assert_eq!(self.cargo(), None);
        if self.cargo().is_some() {
            return self.bail(g, "harvest: already has cargo");
        }

        if g.resources.at(self.tile()) == Some(res) {
            trace!(self, "pick up {res}");
            let Some(home) = self.home(g) else { return self.bail(g, "harvest: no home") };
            self.cargo.set(g.resources.remove(self.tile()));
            self.task.set(Task::DeliverToBuilding(home.id()));
            return self.go_home(g);
        } else {
            trace!(self, "no {res} here");
            if let Some(dest) = self.find_near_resource(g, res) {
                return self.set_destination(g, dest);
            }
        }
        self.bail(g, "harvest failed, no fallback")
    }

    /// When a very unexpected, but not neccesarily impossible state was reached,
    /// we bail out: go home and reset plan, then try again.
    /// E.g.: Should harvest but already carrying some cargo. Or home has exploded.
    fn bail(&self, g: &G, reason: &str) {
        trace!(self, "{reason}");
        self.task.set(Task::None);
        self.go_home(g);
        self.sleep(Self::FAIL_DELAY);
    }

    fn go_home(&self, g: &G) {
        trace!(self);
        match self.home(g) {
            Some(home) => self.set_destination(g, home.entrance()),
            None => self.find_new_home(g),
        }
    }

    fn find_new_home(&self, g: &G) {
        trace!(self, "TODO: unimplemented");
        self.sleep(255);
    }

    fn make_plan(&self, g: &G, home: &Building) {
        trace!(self);

        match self.cargo() {
            Some(cargo) if home.has_input(cargo) => {
                self.task.set(Task::DeliverToBuilding(home.id()));
                self.set_destination(g, home.entrance());
            }
            Some(cargo) /*if not for home */=> {
                if let Some(building) = self.find_near_receptor(g, cargo){
                    self.task.set(Task::DeliverToBuilding(building.id()));
                    self.set_destination(g, building.entrance());
                }
            }
            None => {
                // inputs/outputs, sorted by who has the biggest need.
                // Input need = how empty they are, Output need is how full they are.
                // E.g. if the input is 60% full and and the output is 90% full, the output has the biggest need.
                for (io, slot) in Self::slots_by_priority(home) {
                    trace!(self, "considering resource slot: {io:?} {} {}% full", slot.typ, slot.fullness_pct());
                    match io {
                        InOut::In => {
                            if let Some(tile) = self.find_near_resource(g, slot.typ) {
                                trace!(self, "most urgent: harvest {}", slot.typ);
                                debug_assert_eq!(g.resources.at(tile), Some(slot.typ));
                                self.set_destination(g, tile);
                                self.task.set(Task::Harvest(slot.typ));
                                return; //👈
                            }
                            if let Some(building) = self.find_near_provider(g, slot.typ) {
                                trace!(self, "most urgent: collect {}", slot.typ);
                                self.set_destination(g, building.entrance());
                                self.task.set(Task::TakeFromBuilding(building.id(), slot.typ));
                                return; //👈
                            }
                        }
                        InOut::Out => {
                            if let Some(building) = self.find_near_receptor(g, slot.typ) {
                                trace!(self, "most urgent: bring {} to {building}", slot.typ);
                                self.cargo.set(slot.try_take_one());
                                debug_assert!(self.cargo().is_some());
                                self.set_destination(g, building.tile());
                                self.task.set(Task::DeliverToBuilding(building.id()));
                                return; //👈
                            }
                        }
                    }
                }
            }
        }
        if self.task == Task::None {
            trace!(self, "failed to make a plan");
            self.sleep(Self::FAIL_DELAY);
        }
    }

    fn slots_by_priority(home: &Building) -> impl Iterator<Item = (InOut, &ResourceSlot)> {
        home.inputs() //_
            .filter_map(|s| (!s.is_full()).then_some((InOut::In, s)))
            .chain(home.outputs().filter_map(|s| (!s.is_empty()).then_some((InOut::Out, s))))
            .sorted_by_key(|(io, s)| match io {
                InOut::In => s.fullness_pct() as i32,
                InOut::Out => 100 - (s.fullness_pct() as i32),
            })
    }

    fn is_at_building(&self, building: &Building) -> bool {
        building.bounds().contains(self.tile())
    }

    #[must_use]
    fn find_near_resource(&self, g: &G, typ: ResourceTyp) -> Option<vec2i16> {
        g.resources //__
            .iter()
            .filter(|&(_, res)| res == typ)
            .min_by_key(|(tile, _)| tile.distance_squared(self.tile()))
            .map(|(tile, _)| tile)
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

    fn try_deliver_cargo(&self, building: &Building) {
        trace!(self, "{:?} to {building}", self.cargo);
        debug_assert!(self.is_at_building(building));
        debug_assert!(self.cargo().is_some());

        let Some(cargo) = self.cargo() else { return trace!(self, "ERROR: I have no cargo") };

        match building.add_resource(cargo) {
            OK => {
                trace!(self, "OK: delivered {}", cargo);
                self.cargo.set(None);
            }
            FAIL => {
                trace!(self, "FAILed to deliver {cargo}");
                self.cargo.set(Some(cargo));
            }
        }
    }
}
/*
    // -------------
    pub(crate) fn tick_delivery_work_OLD(&self, g: &G, home: &Building) {
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

    fn go_home_OLD(&self, g: &G) -> Status {
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
*/

#[derive(Debug)]
enum InOut {
    In,
    Out,
}
