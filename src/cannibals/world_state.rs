use super::SideState;
use std::{fmt::Display, num::ParseIntError, rc::Rc};

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type WorldStateResult = Result<WorldState, WorldStateError>;

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

impl Into<String> for BoatSide {
    fn into(self) -> String {
        match self {
            Self::RightSide => "right".to_string(),
            Self::LeftSide => "left".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct WorldState {
    pub left_state: SideState,
    pub right_state: SideState,
    pub boat_side: BoatSide,
    backtrack: String,
}

/// World state:
impl WorldState {
    pub fn new(
        left_state: SideState,
        right_state: SideState,
        boat_side: BoatSide,
        backtrack: String,
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
                backtrack: backtrack,
            }),
        }
    }

    /// [`get_son_states`]
    /// gets all possible son states
    pub fn get_child_states(&self) -> Vec<WorldStateResult> {
        match self.boat_side {
            BoatSide::LeftSide => self
                .left_state
                .get_all_send_combinations()
                .into_iter()
                .map(|(cann, missi)| {
                    let mov = format!(
                        "send {} cannibals and {} missionaries to the right side",
                        cann, missi
                    );
                    return WorldState::new(
                        SideState::new(
                            self.left_state.cannibals - cann,
                            self.left_state.missionaries - missi,
                        ),
                        SideState::new(
                            self.right_state.cannibals + cann,
                            self.right_state.missionaries + missi,
                        ),
                        BoatSide::RightSide,
                        format!("{}|{}", self.backtrack, mov),
                    );
                })
                .collect(),
            BoatSide::RightSide => self
                .right_state
                .get_all_send_combinations()
                .into_iter()
                .map(|(cann, missi)| {
                    let mov = format!(
                        "send {} cannibals and {} missionaries to the left side",
                        cann, missi
                    );
                    return WorldState::new(
                        SideState::new(
                            self.left_state.cannibals + cann,
                            self.left_state.missionaries + missi,
                        ),
                        SideState::new(
                            self.right_state.cannibals - cann,
                            self.right_state.missionaries - missi,
                        ),
                        BoatSide::LeftSide,
                        format!("{}|{}", self.backtrack, mov),
                    );
                })
                .collect(),
        }
    }

    /// [`heuristic`]
    /// Returns a number that represents how far this state is from the goal state.
    /// The lower the value, the closest this state is from the goal state.
    /// # Example
    /// ```
    /// # use algoritmos_rust::cannibals::*;
    /// let state: WorldStateResult = "0 0 3 3 right".try_into();
    /// let state = state.unwrap();
    /// assert_eq!(state.get_heuristic(), 3)
    /// ```
    ///
    /// # Example - Comparison
    /// ```
    /// # use algoritmos_rust::cannibals::*;
    /// let state_1: WorldStateResult = "0 0 3 3 right".try_into();
    /// let state_2: WorldStateResult = "1 1 2 2 right".try_into();
    /// let (state_1, state_2) = (state_1.unwrap(), state_2.unwrap());
    /// assert!(state_2.get_heuristic() < state_1.get_heuristic(), "expect state 2 to be closest to the goal state.");
    /// ```
    pub fn get_heuristic(&self) -> u8 {
        3 - self.left_state.missionaries
    }

    /// [`get_step_by_step`]
    ///
    /// Returns the step by step of how to reach to this state.
    /// Used to get the final answer.
    pub fn get_step_by_step(&self) -> String {
        return self.backtrack.to_owned();
    }

    pub fn get_step_by_step_vec(&self) -> Vec<String> {
        let step_by_step_string = self.get_step_by_step();
        let step_by_step_vec = step_by_step_string.split("|").collect::<Vec<&str>>();

        let step_by_step = step_by_step_vec
            .into_iter()
            .map(|step_str| step_str.to_string())
            .collect::<Vec<String>>();
        step_by_step
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
            "root state".into(),
        )?;

        Ok(world_state)
    }
}

impl Into<String> for WorldState {
    fn into(self) -> String {
        let self_ref = &self;
        self_ref.into()
    }
}

impl Into<String> for &WorldState {
    fn into(self) -> String {
        let boat_string: String = self.boat_side.into();
        format!(
            "{} {} {} {} {}",
            self.left_state.cannibals,
            self.left_state.missionaries,
            self.right_state.cannibals,
            self.right_state.missionaries,
            boat_string
        )
        .to_string()
    }
}

impl PartialEq for WorldState {
    fn eq(&self, other: &Self) -> bool {
        self.left_state == other.left_state
            && self.right_state == other.right_state
            && self.boat_side == other.boat_side
    }
}

impl PartialOrd for WorldState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self.get_heuristic(), other.get_heuristic()) {
            (this_heuristic, other_heuristic) if this_heuristic == other_heuristic => {
                Some(std::cmp::Ordering::Equal)
            }
            (this_heuristic, other_heuristic) if this_heuristic > other_heuristic => {
                Some(std::cmp::Ordering::Greater)
            }
            _ => Some(std::cmp::Ordering::Less),
        }
    }
}

impl Display for WorldState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string(self).unwrap_or_default();
        write!(f, "{}", json)
    }
}

/// [`WorldStateHeapWrapper`]
/// This struct only purpose is to allow to organize [`WorldState`] struct
/// into `min-heap`, this is needed in order organize "next nodes to visit"
/// using heuristic for `greedy` algorithms.
#[derive(Debug)]
pub struct WorldStateHeapWrapper {
    world_state: Rc<WorldState>,
}

impl WorldStateHeapWrapper {
    pub fn new(world_state: Rc<WorldState>) -> Self {
        Self { world_state }
    }
    pub fn get_world_state(&self) -> Rc<WorldState> {
        Rc::clone(&self.world_state)
    }
}

impl PartialEq for WorldStateHeapWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.world_state.get_heuristic() == other.world_state.get_heuristic()
    }
}

impl Eq for WorldStateHeapWrapper {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialOrd for WorldStateHeapWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let heuristics = (
            self.world_state.get_heuristic(),
            other.world_state.get_heuristic(),
        );

        match heuristics {
            (my_heuristic, other_heuristic) if my_heuristic > other_heuristic => {
                Some(std::cmp::Ordering::Greater)
            }
            (my_heuristic, other_heuristic) if my_heuristic == other_heuristic => {
                Some(std::cmp::Ordering::Equal)
            }
            _ => Some(std::cmp::Ordering::Less),
        }
    }
}

impl Ord for WorldStateHeapWrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other)
            .expect("PartialOrd implementation for WorldStateHeapWrapper should not return None!")
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
mod world_state_test {
    use super::*;

    #[test]
    fn world_state_new_returns_error_when_state_is_invalid() {
        let wrong_n_of_missionaries = WorldState::new(
            SideState::new(0, 0),
            SideState::new(3, 2),
            BoatSide::LeftSide,
            "root state".to_string(),
        )
        .unwrap_err();
        let wrong_n_of_cannibals = WorldState::new(
            SideState::new(2, 0),
            SideState::new(3, 1),
            BoatSide::RightSide,
            "root state".to_string(),
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
            "root state".to_string(),
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
            "root state".to_string(),
        )
        .unwrap();
        let non_solution_world_state = WorldState::new(
            SideState::new(1, 2),
            SideState::new(2, 1),
            BoatSide::LeftSide,
            "root state".to_string(),
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
                "root state".to_string(),
            ),
            WorldState::new(
                SideState::new(0, 1),
                SideState::new(3, 2),
                BoatSide::LeftSide,
                "root state".to_string(),
            ),
            WorldState::new(
                SideState::new(2, 1),
                SideState::new(1, 2),
                BoatSide::LeftSide,
                "root state".to_string(),
            ),
        ];
        let world_non_game_over_states = vec![
            WorldState::new(
                SideState::new(0, 0),
                SideState::new(3, 3),
                BoatSide::RightSide,
                "root state".to_string(),
            ),
            WorldState::new(
                SideState::new(2, 2),
                SideState::new(1, 1),
                BoatSide::LeftSide,
                "root state".to_string(),
            ),
            WorldState::new(
                SideState::new(0, 3),
                SideState::new(3, 0),
                BoatSide::LeftSide,
                "root state".to_string(),
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
        let solution_world_state: WorldStateResult = "0 0 3 3 right".try_into();
        let w_s = solution_world_state.expect("error to create world state");

        let expected_son_state: Vec<WorldStateResult> = vec![
            "1 0 2 3 left".try_into(), // 1 cannibal
            "0 1 3 2 left".try_into(), // 1 missionary
            "2 0 1 3 left".try_into(), // 2 cannibals
            "0 2 3 1 left".try_into(), // 2 missionaries
            "1 1 2 2 left".try_into(), // 1 cannibal and 1 missionary
        ];

        let expected_son_state = expected_son_state
            .into_iter()
            .map(|world_state_result| {
                world_state_result.expect("Specified expected son state is in faulty state.")
            })
            .collect::<Vec<WorldState>>();

        let actual_son_states = w_s
            .get_child_states()
            .into_iter()
            .map(|state_result| {
                state_result.expect("Get son states method generated faulty state.")
            })
            .collect::<Vec<WorldState>>();

        let expected_states_count = expected_son_state.len();
        let actual_son_states_count = actual_son_states.len();
        let mut matching_states_count = 0;

        // let matching_states_count = expected_son_state
        expected_son_state.into_iter().for_each(|expected_state| {
            // let expected_state_str: String = expected_state.into();
            assert!(
                actual_son_states.contains(&expected_state),
                "Expected state: [{}] was not generated",
                expected_state.to_string()
            );
            matching_states_count += 1;
        });

        assert_eq!(
            expected_states_count, actual_son_states_count,
            "Expected states count should be equal to the actual states count"
        );
        assert_eq!(
            actual_son_states_count, matching_states_count,
            "Actual states count should be equal to the matching states count"
        );
    }
}

#[cfg(test)]
mod world_state_heap_wrapper_test {
    use super::*;
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    #[test]
    fn test_world_state_heap_wrapper_maintains_expected_order() {
        let world_state_2: WorldStateResult = "1 1 2 2 right".try_into();
        let world_state_1: WorldStateResult = "0 0 3 3 right".try_into();
        let world_state_6: WorldStateResult = "0 3 3 0 right".try_into();
        let world_state_5: WorldStateResult = "2 3 1 0 right".try_into();
        let world_state_3: WorldStateResult = "2 2 1 1 right".try_into();
        let world_state_4: WorldStateResult = "0 3 3 0 right".try_into();
        let (
            world_state_1,
            world_state_2,
            world_state_3,
            world_state_4,
            world_state_5,
            world_state_6,
        ) = (
            world_state_1.unwrap(),
            world_state_2.unwrap(),
            world_state_3.unwrap(),
            world_state_4.unwrap(),
            world_state_5.unwrap(),
            world_state_6.unwrap(),
        );

        let mut heap: BinaryHeap<Reverse<WorldStateHeapWrapper>> = BinaryHeap::new();

        heap.push(Reverse(WorldStateHeapWrapper::new(Rc::new(world_state_1))));
        heap.push(Reverse(WorldStateHeapWrapper::new(Rc::new(world_state_2))));
        heap.push(Reverse(WorldStateHeapWrapper::new(Rc::new(world_state_5))));
        heap.push(Reverse(WorldStateHeapWrapper::new(Rc::new(world_state_3))));
        heap.push(Reverse(WorldStateHeapWrapper::new(Rc::new(world_state_4))));
        heap.push(Reverse(WorldStateHeapWrapper::new(Rc::new(world_state_6))));

        let expected_order_vec = vec![
            "2 3 1 0 right".to_string(),
            "0 3 3 0 right".to_string(),
            "0 3 3 0 right".to_string(),
            "2 2 1 1 right".to_string(),
            "1 1 2 2 right".to_string(),
            "0 0 3 3 right".to_string(),
        ];

        expected_order_vec.into_iter().for_each(|expected| {
            if let Some(Reverse(ws_wrapper)) = heap.pop() {
                let actual: String = ws_wrapper.get_world_state().as_ref().into();
                println!("{}", actual);
                assert_eq!(expected, actual);
            }
        });
    }
}
