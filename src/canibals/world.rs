use std::fmt::Display;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct SideState {
    pub canibals: u8,
    pub missionaries: u8,
}

impl SideState {
    pub fn new(canibals: u8, missionaries: u8) -> Self {
        Self {
            canibals,
            missionaries,
        }
    }
}

impl PartialEq for SideState {
    fn eq(&self, other: &Self) -> bool {
        self.canibals == other.canibals && self.missionaries == other.missionaries
    }
}

impl Display for SideState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string(self).unwrap_or_default();
        write!(f, "{}", json)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum BoatSide {
    RightSide,
    LeftSide,
}

#[derive(Debug, Clone, Copy)]
pub struct WorldState {
    pub left_state: SideState,
    pub right_state: SideState,
    pub boat_side: BoatSide,
}

/// World state:
impl WorldState {
    pub fn new(
        left_state: SideState,
        right_state: SideState,
        boat_side: BoatSide,
    ) -> Result<Self, WorldStateError> {
        let total_canibals = left_state.canibals + right_state.canibals;
        let total_missionaries = left_state.missionaries + right_state.missionaries;

        match (total_canibals, total_missionaries) {
            (can, _) if can != 3 => Err(WorldStateError::ImpossibleNumberOfCanibals(can)),
            (_, mis) if mis != 3 => Err(WorldStateError::ImpossibleNumberOfMissionaries(mis)),
            (_, _) => Ok(Self {
                left_state,
                right_state,
                boat_side,
            }),
        }
    }

    pub fn is_solution(&self) -> bool {
        self.left_state.missionaries == 3
    }
}

impl PartialEq for WorldState {
    fn eq(&self, other: &Self) -> bool {
        self.left_state == other.left_state && self.right_state == other.right_state
    }
}

#[non_exhaustive]
#[derive(Debug, Error, PartialEq)]
pub enum WorldStateError {
    #[error("Impossible number of missionaries")]
    ImpossibleNumberOfMissionaries(u8),
    #[error("Impossible number of canibals")]
    ImpossibleNumberOfCanibals(u8),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn world_state_new_returns_error_when_state_is_invalid() {
        let wrong_n_of_misionaries = WorldState::new(
            SideState::new(0, 0),
            SideState::new(3, 2),
            BoatSide::LeftSide,
        )
        .unwrap_err();
        let wrong_n_of_canibals = WorldState::new(
            SideState::new(2, 0),
            SideState::new(3, 1),
            BoatSide::RightSide,
        )
        .unwrap_err();

        assert_eq!(
            wrong_n_of_misionaries,
            WorldStateError::ImpossibleNumberOfMissionaries(2)
        );
        assert_eq!(
            wrong_n_of_canibals,
            WorldStateError::ImpossibleNumberOfCanibals(5)
        );
    }

    #[test]
    fn world_new_state_creates_expected_state() {
        let world_state = WorldState::new(
            SideState::new(3, 0),
            SideState::new(0, 3),
            BoatSide::LeftSide,
        )
        .unwrap();

        assert_eq!(world_state.left_state.canibals, 3);
        assert_eq!(world_state.left_state.missionaries, 0);

        assert_eq!(world_state.right_state.canibals, 0);
        assert_eq!(world_state.right_state.missionaries, 3);

        assert_eq!(world_state.boat_side, BoatSide::LeftSide);
    }

    #[test]
    fn world_is_solution_returns_expected_response() {
        let solution_world_state = WorldState::new(
            SideState::new(1, 3),
            SideState::new(2, 0),
            BoatSide::LeftSide,
        )
        .unwrap();
        let non_solution_world_state = WorldState::new(
            SideState::new(1, 2),
            SideState::new(2, 1),
            BoatSide::LeftSide,
        )
        .unwrap();

        assert_eq!(solution_world_state.is_solution(), true);
        assert_eq!(non_solution_world_state.is_solution(), false);
    }
}
