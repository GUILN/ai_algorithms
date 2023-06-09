use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct SideState {
    pub cannibals: u8,
    pub missionaries: u8,
}

impl SideState {
    pub fn new(cannibals: u8, missionaries: u8) -> Self {
        Self {
            cannibals,
            missionaries,
        }
    }

    pub fn cannibal_can_eat_missionary(&self) -> bool {
        self.cannibals > self.missionaries && self.missionaries > 0
    }

    /// [`get_all_send_combinations`]
    /// ## Gets all the possible send combinations given the actual number of cannibals and missionaries.
    /// Returns a tuple containing `(number_of_cannibals, number_of_missionaries)` that can be sent.
    pub fn get_all_send_combinations(&self) -> Vec<(u8, u8)> {
        match (self.cannibals, self.missionaries) {
            (c, m) if c >= 2 && m >= 2 => vec![(2, 0), (0, 2), (1, 1), (1, 0), (0, 1)],
            (c, m) if c >= 2 && m == 1 => vec![(2, 0), (0, 1), (1, 1), (1, 0)],
            (c, m) if c >= 2 && m == 0 => vec![(2, 0), (1, 0)],
            (c, m) if c == 1 && m == 1 => vec![(1, 0), (0, 1), (1, 1)],
            (c, m) if c == 1 && m == 0 => vec![(1, 0)],
            (c, m) if c == 0 && m == 1 => vec![(0, 1)],
            (c, m) if c == 0 && m >= 2 => vec![(0, 2), (0, 1)],
            (c, m) if c == 1 && m >= 2 => vec![(0, 2), (0, 1), (1, 1), (1, 0)],
            _ => vec![(0, 0)],
        }
    }
}

impl PartialEq for SideState {
    fn eq(&self, other: &Self) -> bool {
        self.cannibals == other.cannibals && self.missionaries == other.missionaries
    }
}

impl Display for SideState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string(self).unwrap_or_default();
        write!(f, "{}", json)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn side_state_get_all_send_combinations_generates_expected_combinations() {
        let side_state = SideState::new(3, 3);
        let expected_combinations: Vec<(u8, u8)> = vec![(2, 0), (0, 2), (1, 1), (1, 0), (0, 1)];

        let combinations = side_state.get_all_send_combinations();

        let expected_combinations_count = expected_combinations.len();
        let actual_combinations_count = combinations.len();
        let mut matching_combinations_count = 0;

        expected_combinations.into_iter().for_each(|expected_comb| {
            assert!(
                combinations.contains(&expected_comb),
                "Expected combination: ({}, {}) is not contained in generated combinations",
                expected_comb.0,
                expected_comb.1
            );
            matching_combinations_count += 1;
        });

        let combination_matches = expected_combinations_count == actual_combinations_count
            && expected_combinations_count == matching_combinations_count;

        assert!(combination_matches, "Combinations count did not matched")
    }
}
