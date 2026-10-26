mod facade {
    // use types::{Action, Event, GenError, Params, Rejection, Seed, ShipId, ShipView, ValidatedAction,Map};
    #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
    pub struct ShipId(pub u8);

    #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
    pub struct IslandId(pub u8);

    pub type Round = u32;

    pub type Seed = u64;

    #[non_exhaustive]
    #[derive(Clone, Copy, Debug)]
    pub struct Params {
        pub archipelago_radius: u32,
        pub island_count: u8,
        pub cliff_count: u16,
        pub rock_count: u16,
        pub vision_range: u32,
        pub bombard_range: u32,
        pub hull_strength: u32,
        pub round_cap: Round,
        pub round_deadline_ms: u32,
    }

    pub struct ShipView {}

    pub enum GenError {
        Stub,
    }

    pub enum Action {}

    pub struct ValidatedAction {}

    pub struct Rejection {}

    pub struct Event {}

    pub struct SpectatorView {}

    pub enum Outcome {}

    pub struct Map {}
}

use facade::{
    Action, Event, GenError, Map, Outcome, Params, Rejection, Seed, ShipId, ShipView, SpectatorView, ValidatedAction,
};

// Proposal
pub struct Chart {
    map: Map,
}

/// Protocol - 8
impl Chart {
    /// Main - 2.3 - How a sea is made
    pub fn generate(seed: Seed, params: &Params) -> Result<Chart, GenError> {
        // 1. Seed reproducible random generator from the generation stream (not wall clock).
        // TODO

        // 2. Create sea hex grid of `archipelago_radius` . Reject if radius ≤ `vision_range` .
        if params.archipelago_radius <= params.vision_range {
            return Err(GenError::Stub);
        }
        // TODO: Generate map (DEFINE)
        let ret_chart: Chart = Chart { map: Map {} };

        // 3. Place `island_count` connected island footprints.
        // Footprints must NOT touch each other, corners, or cells adjacent to corners.
        for i in 0..params.island_count {
            // TODO: Place island (DEFINE)
        }

        // 4. Place `cliff_count` cliffs (must not touch islands or sea corners/adjacent cells)
        for i in 0..params.cliff_count {
            // TODO: Place Cliffs (DEFINE)
        }

        // 5. Place `rock_count` rocks on clear water (never on corners or adjacent cells).
        for i in 0..params.rock_count {
            // TODO: Place Cliffs (DEFINE)
        }

        // 6. Connectivity Check: Verify clear water is fully connected without crossing rocks.
        // Otherwise, advance seed and retry.

        // TODO (DEFINE)

        // 7. Treasure Placement: Select clear water cell not adjacent to land/corners. Retry if impossible.

        // TODO (DEFINE)

        // 8. Assign one unique true CLUE to each Island (Section 4).

        // TODO (DEFINE)

        // 9. Solvability Check: Verify full clue intersection equals exactly 1 cell. Advance seed and retry if failed

        // TODO (DEFINE)

        Ok(ret_chart)
    }
    /// Decide whether an action is legal. The only way to obtain a ValidatedAction.
    pub fn validate(&self, ship: ShipId, action: &Action) -> Result<ValidatedAction, Rejection> {
        Ok(ValidatedAction {})
    }
    /// Apply an already validated action. Cannot fail.
    pub fn apply(&mut self, action: ValidatedAction) -> Vec<Event> {
        vec![]
    }
    /// End of round: the round number advances and the Islands take their turn.
    pub fn advance_round(&mut self) -> Vec<Event> {
        vec![]
    }
    /// What a Ship is permitted to see.
    pub fn view_for(&self, ship: ShipId) -> ShipView {
        ShipView {}
    }
    /// What a spectator sees: everything.
    pub fn spectator_view(&self) -> SpectatorView {
        SpectatorView {}
    }
    pub fn outcome(&self) -> Option<Outcome> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
