use crate::{Grid, Position};

enum Action {
  PlayerJoin,
  ChangeActivePlayer,
  AddConnection(Position, Position),
  ClaimSquare
}

struct Store {

}
impl Store {
  fn new(root_reducer, initial_state) -> Self {

  }

  fn dispatch(action: Action) -> Self {

  }
}

struct State {
  grid: Grid,
  players: Vec<String>,
  active_player: String,
  error: Option<GridError>
}

fn reducer(mut state: State, action: Action) -> State {
  match action {
    Action::AddConnection(first, second) => {
      match state.grid.add_connection(&first, &second, state.active_player) {
        Ok(_) => State {
          grid: state.grid,
          error: None,
          ..state
        },
        Err(e) => State {
          error: e,
          ..state
        }
      }

    }
  }
}

#[cfg(test)]
mod test {
  #[test]
  fn it_should_create_store() {}
}
