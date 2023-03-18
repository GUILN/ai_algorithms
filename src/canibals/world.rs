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

    pub fn cannibal_can_eat_missionary(&self) -> bool {
        self.canibals > self.missionaries && self.missionaries > 0
    }

    /// [`get_all_send_combinations`]
    /// ## Gets all the possible send combinations given the actual number of cannibals and missionaries.
    /// Returns a tuple containing `(number_of_cannibals, number_of_missionaries)` that can be sent.
    pub fn get_all_send_combinations(&self) -> Vec<(u8, u8)> {
        match (self.canibals, self.missionaries) {
            (c, m) if c >= 2 && m >= 2 => vec![(2, 0), (0, 2), (1, 1), (1, 0), (0, 1)],
            (c, m) if c >= 2 && m == 1 => vec![(2, 0), (0, 1), (1, 1), (1, 0)],
            (c, m) if c >= 2 && m == 0 => vec![(2, 0), (1, 0)],
            (c, m) if c == 1 && m == 1 => vec![(1, 0), (0, 1), (1, 1)],
            (c, m) if c == 1 && m == 0 => vec![(1, 0)],
            (c, m) if c == 0 && m == 1 => vec![(0, 1)],
            (c, m) if c == 0 && m >= 2 => vec![(0, 2), (0, 1)],
            (c, m) if c == 1 && m >= 2 => vec![(2, 0), (0, 1), (1, 1), (1, 0)],
            _ => vec![(0, 0)],
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

#[derive(Debug, Clone, Copy, Serialize)]
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

    /// [`get_son_states`]
    /// gets all possible son states
    pub fn get_son_states(&self) -> Vec<Result<WorldState, WorldStateError>> {
        match self.boat_side {
            BoatSide::LeftSide => self
                .left_state
                .get_all_send_combinations()
                .into_iter()
                .map(|(cann, missi)| {
                    WorldState::new(
                        SideState::new(
                            self.left_state.canibals - cann,
                            self.left_state.missionaries - missi,
                        ),
                        SideState::new(
                            self.left_state.canibals + cann,
                            self.left_state.missionaries + missi,
                        ),
                        BoatSide::RightSide,
                    )
                })
                .collect(),
            BoatSide::RightSide => self
                .right_state
                .get_all_send_combinations()
                .into_iter()
                .map(|(cann, missi)| {
                    WorldState::new(
                        SideState::new(
                            self.left_state.canibals + cann,
                            self.left_state.missionaries + missi,
                        ),
                        SideState::new(
                            self.left_state.canibals - cann,
                            self.left_state.missionaries - missi,
                        ),
                        BoatSide::LeftSide,
                    )
                })
                .collect(),
        }
    }

    pub fn is_solution(&self) -> bool {
        self.left_state.missionaries == 3
    }

    pub fn is_game_over(&self) -> bool {
        self.left_state.cannibal_can_eat_missionary()
            || self.right_state.cannibal_can_eat_missionary()
    }
}

impl PartialEq for WorldState {
    fn eq(&self, other: &Self) -> bool {
        self.left_state == other.left_state && self.right_state == other.right_state
    }
}

impl Display for WorldState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string(self).unwrap_or_default();
        write!(f, "{}", json)
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

    #[test]
    fn world_is_game_over_returns_expected_response() {
        let world_game_over_states = vec![
            WorldState::new(
                SideState::new(1, 2),
                SideState::new(2, 1),
                BoatSide::LeftSide,
            ),
            WorldState::new(
                SideState::new(0, 1),
                SideState::new(3, 2),
                BoatSide::LeftSide,
            ),
            WorldState::new(
                SideState::new(2, 1),
                SideState::new(1, 2),
                BoatSide::LeftSide,
            ),
        ];
        let world_non_game_over_states = vec![
            WorldState::new(
                SideState::new(0, 0),
                SideState::new(3, 3),
                BoatSide::RightSide,
            ),
            WorldState::new(
                SideState::new(2, 2),
                SideState::new(1, 1),
                BoatSide::LeftSide,
            ),
            WorldState::new(
                SideState::new(0, 3),
                SideState::new(3, 0),
                BoatSide::LeftSide,
            ),
        ];

        world_game_over_states
            .into_iter()
            .for_each(|state_result| assert_eq!(state_result.unwrap().is_game_over(), true));

        world_non_game_over_states
            .into_iter()
            .for_each(|state_result| assert_eq!(state_result.unwrap().is_game_over(), false));
    }
}
