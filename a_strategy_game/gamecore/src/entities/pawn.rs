use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Pawn {
    base: Base,

    pub typ: PawnTyp,

    // move
    pub route: Route,

    // work
    pub home: Cel<Option<Id>>,
    pub cargo: Cel<Option<ResourceTyp>>,

    // attack
    pub target: Cel<Option<Id>>,
    pub rot: Cel<f32>,
}

impl BaseT for Pawn {
    fn base(&self) -> &Base {
        &self.base
    }
}

impl HasId3 for Pawn {
    fn set_id3(&mut self, id: Id) {
        self.base.id = id
    }
}

impl EntityT for Pawn {
    fn on_killed(&self, g: &G) {
        trace!(self);

        // ☠️ remove dead worker from factory
        self.home(g).map(|h| h.workers().remove(&self.id()));

        // ☘️ drop cargo on the floor (if floor empty).
        if let Some(res) = self.cargo() {
            if g.resources.at(self.tile()).is_none() {
                g.resources.insert(self.tile(), res);
            }
        }

        g.effects.add_crater(g, self.tile())
    }

    fn draw(&self, g: &G, out: &mut Out) {
        match self.typ {
            PawnTyp::Kitten => self.base_draw(g, out),
            PawnTyp::Cat => self.base_draw(g, out),
            PawnTyp::Crab => self.base_draw(g, out),
            PawnTyp::Turret => self.draw_turret(g, out),
            PawnTyp::Starfish => self.base_draw(g, out),
        }
    }

    fn can_move(&self) -> bool {
        self.typ.can_move()
    }
}

impl Pawn {
    // Ticks needed to pick up a resource
    const PICKUP_DELAY: u8 = 1;

    pub fn new(typ: PawnTyp, tile: vec2i16, team: Team) -> Self {
        Self {
            base: Base::new(tile, team, typ.default_health()),
            typ,
            route: default(),
            home: None.cel(),
            cargo: None.cel(),
            target: None.cel(),
            rot: default(),
        }
    }

    // ⏱️
    pub(crate) fn tick(&self, g: &G) {
        if self.base.tick_sleep() {
            return; // 👈
        }

        // 🥾 always first go where you were going
        if self.can_move() && !self.is_at_destination() {
            self.walk_to_destination(g);
            return; //👈
        }

        // 📍 now you are at destination
        // 🏭 for worker pawns
        if let Some(home) = self.home(g) {
            self.tick_delivery_work(g, home)
        }

        if self.can_attack() {
            self.tick_attack(g)
        }

        // 😴 nothing left to do
        if self.can_move() {
            self.take_personal_space(g);
        }
    }

    pub(crate) fn can_assign_to(&self, building: &Building) -> bool {
        if !self.typ.is_worker() {
            //trace!(self, "is not a worker");
            return false;
        }
        if self.team() != building.team() {
            //trace!(self, "wrong team: {} != {}", self.team(), building.team());
            return false;
        }
        //trace!(self, "can assign");
        true
    }

    /// 📦📍 Called when a pawn has reached their destination and thus can pick up/drop off their cargo.
    fn tick_delivery_work(&self, g: &G, home: &Building) {
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
        //may be holding output, go to dest

        if self.cargo().is_some() {
            trace!(self, "home is full, cannot deliver, waiting...");
            // TODO: drop cargo if undelivered after a very long time. Might need hands to take output instead.
            self.take_personal_space(g);
            self.sleep(20);
            return;
        }

        // Decide what to do next

        // Try serve the input/output in biggest need

        enum InOut {
            In,
            Out,
        }

        // inputs/outputs, sorted by who needs works most urgently
        for (io, slot) in home
            .inputs() //_
            .filter_map(|s| (!s.is_full()).then_some((InOut::In, s)))
            .chain(home.outputs().filter_map(|s| (!s.is_empty()).then_some((InOut::Out, s))))
            .sorted_by_key(|(io, s)| match io {
                InOut::In => s.fullness_pct() as i32,
                InOut::Out => 100 - (s.fullness_pct() as i32),
            })
        {
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

    /// +-HQ-+   +home+
    /// | 🦀 |   |    |    ☘️️️ ☘️️
    /// +----+   +----+
    fn tick_on_other_building(&self, g: &G, home: &Building, building: &Building) {
        trace!(self, "{building}");
        self.try_deliver_cargo(building);
        match self.cargo() {
            None => self.go_to_near_resource(g, home).or_else(|| self.go_home(g)),
            Some(_) => self.go_home(g),
        };
    }

    ///   +home+
    ///   |    |    🦀 ☘️️
    ///   +----+
    fn tick_away_from_building(&self, g: &G, home: &Building) {
        trace!(self);
        match self.cargo() {
            Some(_) => self.go_home(g),
            None => self.go_to_near_resource(g, home).or_else(|| self.go_home(g)),
        };
    }

    fn go_to_near_resource(&self, g: &G, home: &Building) -> Status {
        trace!(self);
        let new_dest = g.resources.iter().filter(|(_, res)| home.has_nonfull_input(*res)).min_by_key(|(tile, _)| tile.distance_squared(self.tile())).map(|(tile, _)| tile)?;
        self.set_destination(g, new_dest);
        OK
    }

    fn find_near_resource(&self, g: &G, typ: ResourceTyp) -> Option<vec2i16> {
        g.resources //__
            .iter()
            .filter(|&(_, res)| res == typ)
            .min_by_key(|(tile, _)| tile.distance_squared(self.tile()))
            .map(|(tile, _)| tile)
    }

    fn find_near_receptor<'g>(&self, g: &'g G, res: ResourceTyp) -> Option<&'g Building> {
        g.buildings() //__
            .filter(|b| b.has_nonfull_input(res))
            .min_by_key(|b| b.tile().distance_squared(self.tile()))
    }

    fn go_home(&self, g: &G) -> Status {
        trace!(self);
        self.set_destination(g, g.building(self.home.get()?)?.entrance());
        OK
    }

    //////fn go_downstream(&self, g: &G, home: &Building) -> Status {
    //////    trace!(self);
    //////    debug_assert!(self.cargo.is_some());

    //////    let cargo = self.cargo()?;

    //////    for downstream in home
    //////        .downstream_buildings(g) //_
    //////        .sorted_by_key(|d| d.tile().distance_squared(self.tile()))
    //////    {
    //////        if downstream.can_accept(cargo) {
    //////            self.set_destination(g, downstream.entrance());
    //////            return OK;
    //////        }
    //////    }
    //////    FAIL
    //////}

    //fn move_resource_downstream(&self, g: &G, home: &Building) -> Status {
    //    //trace!(self, "cargo={:?}", &self.cargo);
    //    debug_assert!(self.cargo.is_none());

    //    if self.cargo.is_some() {
    //        return FAIL;
    //    }

    //    for downstream in home
    //        .downstream_buildings(g) //_
    //        .sorted_by_key(|d| d.tile().distance_squared(home.tile()))
    //    {
    //        for slot in home.inputs() {
    //            if slot.has_at_least(1) && downstream.can_accept(slot.typ) {
    //                self.cargo.set(slot.try_take_one());
    //                if self.set_destination(g, downstream.entrance()).is_some() {
    //                    //trace!(self, "taking {:?} to {}", self.cargo(), downstream);
    //                    return OK;
    //                }
    //            }
    //        }
    //    }
    //    FAIL
    //}

    pub fn try_deliver_cargo(&self, building: &Building) -> Status {
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
                self.sleep(5);
                FAIL
            }
        }
    }

    pub fn try_pick_up_cargo(&self, g: &G, home: &Building) -> Status {
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
                self.cargo.set(slot.try_take_one());
                if self.set_destination(g, home.entrance()).is_some() {
                    //trace!(self, "take {:?} home to {:?}", self.cargo(), home.typ);
                    return OK;
                }
            }
        }
        FAIL
    }

    /// 🏠 Assign pawn to work at building.
    pub fn assign_to(&self, g: &G, building: &Building) {
        trace!(self, "{building}");
        if !self.can_assign_to(building) {
            return;
        }
        if let Some(home) = self.home(g) {
            home.workers().remove(&self.id());
        }
        building.workers().insert(self.id());
        self.home.set(Some(building.id()));
    }

    pub fn home<'g>(&self, g: &'g G) -> Option<&'g Building> {
        g.building(self.home.get()?)
    }

    /// If standing on another pawn, move aside randomly.
    fn take_personal_space(&self, g: &G) {
        if !self.can_move() {
            return;
        }
        let standing_on_other = g.pawns().filter(|p| p.id() != self.id()).find(|p| p.tile() == self.tile()).is_some();
        if standing_on_other {
            // ⚠️ Don't move diagonally so you don't go trough walls.
            let random_step = vec2::from(g.pick_random([(-1, 0), (1, 0), (0, -1), (0, 1)]));
            self.teleport_to(g, self.tile() + random_step);
        }
    }

    fn teleport_to(&self, g: &G, dst: vec2i16) {
        if self.can_walk_on_pos(g, dst) {
            trace!(self, "dst={dst}");
            self.get_tile().set(dst);
            self.route.clear();
        } else {
            trace!(self, "dst={dst}: cannot walk here");
        }
    }

    pub(crate) fn can_move(&self) -> bool {
        self.typ.can_move()
    }

    fn walk_to_destination(&self, g: &G) {
        if let Some(next_tile) = self.route.next() {
            if self.can_walk_on_tile(g.tile_at(next_tile)) {
                self.get_tile().set(next_tile);
            } else {
                trace!(self, "cannot walk on {next_tile}, clearing route");
                self.route.clear(); // ☹️
            }
        }
    }

    pub fn set_destination(&self, g: &G, dest: vec2i16) -> Status {
        if !self.can_move() {
            trace!(self, "cannot move");
            return FAIL;
        }
        let max_dist = 42;
        let distance_map = DistanceMap::new(dest, max_dist, |p| self.can_walk_on_tile(g.tile_at(p)));
        let path = distance_map.path_to_center(self.tile());
        trace!(self, "dest={dest} path len={:?}", path.as_ref().map(|p| p.len()));
        self.route.set(path?);
        OK
    }

    pub fn destination(&self) -> Option<vec2i16> {
        self.route.destination()
    }

    pub fn cargo(&self) -> Option<ResourceTyp> {
        self.cargo.get()
    }

    pub fn is_at_destination(&self) -> bool {
        self.route.is_finished()
    }

    pub fn can_walk_on_tile(&self, tile: Tile) -> bool {
        self.typ.can_walk_on(tile)
    }

    pub fn can_walk_on_pos(&self, g: &G, idx: vec2i16) -> bool {
        self.can_walk_on_tile(g.tile_at(idx))
    }

    fn attack_strength(&self) -> u8 {
        match self.typ {
            PawnTyp::Kitten => 0,
            PawnTyp::Cat => 0,
            PawnTyp::Crab => 1,
            PawnTyp::Turret => 1,
            PawnTyp::Starfish => 0,
        }
    }

    fn can_attack(&self) -> bool {
        self.attack_strength() != 0
    }

    fn tick_attack(&self, g: &G) {
        debug_assert!(self.can_attack());

        match self.target(g) {
            Some(e) => self.attack(g, e),
            None => self.find_target(g),
        }
    }

    fn target<'g>(&self, g: &'g G) -> Option<Entity<'g>> {
        self.target.and_then(|id| g.entity(id))
    }

    fn find_target(&self, g: &G) {
        let attack_radius = 12; // TODO
        self.target.set(g.find_entity(self.tile(), attack_radius, |p| self.team().is_hostile_to(p.team())).map(|e| e.id()));

        //#[cfg(debug_assertions)]
        //if let Some(target) = self.target.get() {
        //    trace!(self, "target={target}")
        //}

        self.sleep(1);
    }

    fn attack<'g>(&self, g: &'g G, victim: Entity) {
        match self.typ {
            PawnTyp::Turret => self.turret_attack(g, victim),
            _ => self.attack_base(g, victim),
        }
    }

    fn attack_base(&self, g: &G, victim: Entity) {
        //trace!(self, "Attack {victim}");
        g.effects.add_bolt(g, self.center(), victim.center());
        g.deal_damage(victim, self.attack_strength());
        self.sleep(1);
    }

    fn turret_attack(&self, g: &G, victim: Entity) {
        let dir = (victim.center() - self.center()).as_f32();
        let rot = f32::atan2(dir.x(), -dir.y());
        self.rot.set(rot);
        self.attack_base(g, victim);
    }

    fn base_draw(&self, _g: &G, out: &mut Out) {
        out.draw_sprite_rot(L_SPRITES, self.sprite(), self.tile().pos(), self.rot.get());
        if let Some(res) = self.cargo.get() {
            out.draw_sprite(L_SPRITES + 1, res.sprite(), self.tile().pos() + vec2(0, 8));
        }
    }

    pub(crate) fn sprite(&self) -> Sprite {
        self.typ.sprite(self.team())
    }

    fn draw_turret(&self, g: &G, out: &mut Out) {
        self.base_draw(g, out);
    }
}

impl Display for Pawn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{}", self.typ, self.id())
    }
}

impl Debug for Pawn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        pretty_print(f, self)
    }
}
