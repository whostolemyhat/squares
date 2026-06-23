use crate::{Grid, GridError, Position};

enum Action {
  PlayerJoin,
  ChangeActivePlayer,
  AddConnection(Position, Position),
  CheckSquares(Position, Position),
}

struct Store {}
// impl Store {
//   fn new(root_reducer, initial_state) -> Self {

//   }

//   fn dispatch(action: Action) -> Self {

//   }
// }

struct State<'a> {
  grid: Grid<'a>,
  players: Vec<String>,
  active_player: String,
  error: Option<GridError>,
}

fn reducer(state: State, action: Action) -> State {
  match action {
    Action::AddConnection(first, second) => {
      match state
        .grid
        .add_connection_pure(&first, &second, &state.active_player)
      {
        // TODO tidy this
        // TODO probably shouldn't be in state
        Ok(updated) => State {
          grid: Grid {
            connections: updated,
            ..state.grid
          },
          error: None,
          ..state
        },

        Err(e) => State {
          error: Some(e),
          ..state
        },
      }
    }
    Action::CheckSquares(first, second) => {
      match state
        .grid
        .check_last_move(&first, &second, &state.active_player)
      {
        Ok(updated_owned) => State {
          grid: Grid {
            owned: updated_owned,
            ..state.grid
          },
          error: None,
          ..state
        },
        Err(e) => State {
          error: Some(e),
          ..state
        },
      }
    }
    Action::ChangeActivePlayer => {
      let mut current_index = state
        .players
        .iter()
        .position(|p| *p == state.active_player)
        .unwrap_or_default();

      let next_index = (current_index + 1) % state.players.len();
      let next_player = state.players[next_index];

      State {
        active_player: next_player,
        ..state
      }
    }
  }
}

#[cfg(test)]
mod test {
  #[test]
  fn it_should_create_store() {}
}

// flow
// dispatch(Action::AddConnection)
// handle error - if invalid, show msg and let player retry
// dispatch(Action::CheckSquares)
// dispatch(Action::SetActivePlayer)
