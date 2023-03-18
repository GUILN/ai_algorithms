use super::SideState;
use std::{fmt::Display, num::ParseIntError};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum BoatSide {
    RightSide,
    LeftSide,
}

impl TryFrom<&str> for BoatSide {
    type Error = WorldStateError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim() {
            "right" => Ok(Self::RightSide),
            "left" => Ok(Self::LeftSide),
            _ => Err(WorldStateError::ParseFromStringError(
                "Invalid Value For BoatSide".into(),
            )),
        }
    }
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
        let total_cannibals = left_state.cannibals + right_state.cannibals;
        let total_missionaries = left_state.missionaries + right_state.missionaries;

        match (total_cannibals, total_missionaries) {
            (can, _) if can != 3 => Err(WorldStateError::ImpossibleNumberOfCannibals(can)),
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
                            self.left_state.cannibals - cann,
                            self.left_state.missionaries - missi,
                        ),
                        SideState::new(
                            self.left_state.cannibals + cann,
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
                            self.left_state.cannibals + cann,
                            self.left_state.missionaries + missi,
                        ),
                        SideState::new(
                            self.left_state.cannibals - cann,
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

/// [TryFrom<&str>]
/// This TryFrom<&str> accepts the following format:
/// `"u8 u8 u8 u8 left | right"`
/// Meaning:
/// `"n_cannibals_left n_missionaries_left n_cannibals_right n_missionaries_right boat_side"`
/// # Examples:
/// `"1 1 2 2 right"`
/// means:
/// * left: 1 cannibal and 1 missionary
/// * right: 2 cannibals and 2 missionaries and the boat
/// `"1 0 2 3 left"`
/// means:
/// * left: 1 cannibal and 0 missionary and the boat
/// * right: 2 cannibals and 3 missionaries
impl TryFrom<&str> for WorldState {
    type Error = WorldStateError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let v = value.split(" ").collect::<Vec<&str>>();
        if v.len() < 5 {
            return Err(WorldStateError::ParseFromStringError(
                "Non sufficient number of args".into(),
            ));
        }
        let (l_c, l_m, r_c, r_m, b) = (
            v[0].trim(),
            v[1].trim(),
            v[2].trim(),
            v[3].trim(),
            v[4].trim(),
        );
        let world_state = WorldState::new(
            SideState::new(l_c.parse()?, l_m.parse()?),
            SideState::new(r_c.parse()?, r_m.parse()?),
            b.try_into()?,
        )?;

        Ok(world_state)
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
    #[error("Impossible number of cannibals")]
    ImpossibleNumberOfCannibals(u8),
    #[error("Error when trying to parse from WorldState string")]
    ParseFromStringError(String),
}

impl From<ParseIntError> for WorldStateError {
    fn from(_: ParseIntError) -> Self {
        Self::ParseFromStringError("Error while trying to parse a number from string".into())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn world_state_new_returns_error_when_state_is_invalid() {
        let wrong_n_of_missionaries = WorldState::new(
            SideState::new(0, 0),
            SideState::new(3, 2),
            BoatSide::LeftSide,
        )
        .unwrap_err();
        let wrong_n_of_cannibals = WorldState::new(
            SideState::new(2, 0),
            SideState::new(3, 1),
            BoatSide::RightSide,
        )
        .unwrap_err();

        assert_eq!(
            wrong_n_of_missionaries,
            WorldStateError::ImpossibleNumberOfMissionaries(2)
        );
        assert_eq!(
            wrong_n_of_cannibals,
            WorldStateError::ImpossibleNumberOfCannibals(5)
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

        assert_eq!(world_state.left_state.cannibals, 3);
        assert_eq!(world_state.left_state.missionaries, 0);

        assert_eq!(world_state.right_state.cannibals, 0);
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

    #[test]
    fn world_tryfrom_str_trait_works_as_expected() {
        let cases = vec![
            ("3 0 0 3 left", 3, 0, 0, 3, BoatSide::LeftSide),
            ("1 2 2 1 right", 1, 2, 2, 1, BoatSide::RightSide),
            ("0 0 3 3 right", 0, 0, 3, 3, BoatSide::RightSide),
        ];

        for case in cases {
            let (world_state_str, l_c, l_m, r_c, r_m, b_s) = case;
            let world_state_result: Result<WorldState, WorldStateError> =
                world_state_str.try_into();
            if let Ok(ws) = world_state_result {
                assert_eq!(ws.left_state.cannibals, l_c);
                assert_eq!(ws.left_state.missionaries, l_m);
                assert_eq!(ws.right_state.cannibals, r_c);
                assert_eq!(ws.right_state.missionaries, r_m);
                assert_eq!(ws.boat_side, b_s);
            } else {
                panic!("world state should not have failed")
            }
        }
    }

    #[test]
    fn world_get_son_states_returns_expected_states() {
        let solution_world_state = WorldState::new(
            SideState::new(1, 3),
            SideState::new(2, 0),
            BoatSide::LeftSide,
        )
        .unwrap();
    }
}
