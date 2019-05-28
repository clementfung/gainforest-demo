#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate quick_error;

extern crate oasis_game_core;
#[macro_use]
extern crate oasis_game_core_derive;

use serde_json::Value;
use std::error::Error;
use oasis_game_core::{*, Game as InnerGame};
use oasis_game_core_derive::{flow, moves};

const DEFORESTED_IDS: [usize; 6] = [5, 6, 7, 13, 14, 15];
const PAYOUT: i32 = 2;

/// Error types.
quick_error! {
    #[derive(Debug)]
    pub enum Errors {
        InvalidCell {
            description("invalid cell")
            display("A move must specify a valid cell.")
        }
    }
}

/// Define the state shape.
/// State type.
pub type Cells = [i32; 32];
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct State {
    pub cells: Cells,
    pub forest: Cells,
    pub stake: i32,
    pub time: i32
}

impl Default for State {
    fn default() -> Self {
        State {
            cells: [0; 32],
            forest: [0; 32],
            stake: 100,
            time: 0
        }
    }
}

fn forest_growth(state: &mut UserState<State>) {
    for id in 0..state.g.cells.len() {
        // The forest grows
        if DEFORESTED_IDS.into_iter().find(|&&x| x == id).is_none() { 
            state.g.forest[id] = 1;
            if state.g.cells[id] != 0 {
                state.g.stake += state.g.cells[id];
                state.g.stake += PAYOUT;
            }
        } else {
            state.g.forest[id] = -1;
        }
        state.g.cells[id] = 0;
    }
    state.g.time += 1;
}

/// Define your moves as methods in this trait.
#[moves]
trait Moves {
    fn click_cell(state: &mut UserState<State>, player_id: u16, args: &Option<Value>)
                -> Result<(), Box<Error>> {
        if let Some(value) = args {
            let id = value.as_array()
                .and_then(|arr| arr.get(0))
                .and_then(|cell| cell.as_u64())
                .and_then(|cell| Some(cell as usize))
                .ok_or(Box::new(Errors::InvalidCell))?;
            
            match id {
                99 => {
                    forest_growth(state);
                    Ok(())
                },
                _ => {
                    state.g.stake -= 5;
                    state.g.cells[id] += 5;
                    Ok(())
                }
            }
         
        } else {
            return Err(Box::new(Errors::InvalidCell))
        } 
    }
}

/// Define the game flow.
#[flow]
trait Flow {
    fn initial_state(&self, seed: Option<u128>) -> State {
        Default::default()
    }

    fn end_turn_if(&self, _: &UserState<State>) -> bool {
        // Do not end the turn
        false
    }

    fn end_game_if(&self, state: &UserState<State>) -> Option<(Option<Score>, Value)> {
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
