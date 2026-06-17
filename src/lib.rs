// todo
// display owned squares
use std::{collections::HashMap, fmt::Display};

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum GridError {
  #[error("points not neighbours")]
  InvalidConnection,
  #[error("connection already exists")]
  AlreadyExists,
  #[error("square already claimed by {user_id}")]
  SquareAlreadyOwned { user_id: usize },
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
  pub x: usize,
  pub y: usize,
}

#[derive(Debug, PartialEq)]
pub struct Grid {
  width: usize,
  height: usize,
  // edges
  connections: HashMap<(Position, Position), usize>,
  // squares
  owned: HashMap<Position, usize>,
}

impl Grid {
  pub fn new(width: usize, height: usize) -> Self {
    Grid {
      width,
      height,
      connections: HashMap::new(),
      owned: HashMap::new(),
    }
  }

  fn neighbours(&self, pos: &Position) -> Vec<Position> {
    let mut neighbours = vec![];
    // top (0, -1)
    if pos.y > 0 {
      neighbours.push(Position {
        x: pos.x,
        y: pos.y - 1,
      });
    }
    // bottom (0, 1)
    if pos.y < self.height - 1 {
      neighbours.push(Position {
        x: pos.x,
        y: pos.y + 1,
      });
    }

    // left (-1, 0)
    if pos.x > 0 {
      neighbours.push(Position {
        x: pos.x - 1,
        y: pos.y,
      });
    }

    // right (1, 0)
    if pos.x < self.width - 1 {
      neighbours.push(Position {
        x: pos.x + 1,
        y: pos.y,
      });
    }

    neighbours
  }

  fn is_neighbour(&self, first: &Position, second: &Position) -> bool {
    self.neighbours(&first).contains(second)
  }

  pub fn add_connection(
    &mut self,
    first: &Position,
    second: &Position,
    user_id: usize,
  ) -> Result<(), GridError> {
    if !self.is_neighbour(first, second) {
      return Err(GridError::InvalidConnection);
    }

    if self.connections.contains_key(&(*first, *second)) {
      return Err(GridError::AlreadyExists);
    }

    self.connections.insert((*first, *second), user_id);

    // has this just made a square?
    // is this horz or vert?
    // horz: check squares below and above
    let is_horz = first.y == second.y;
    if is_horz {
      if self.check_square(first) {
        self.claim_square(first, user_id)?;
      }
      if first.y > 0 {
        let square_above = Position {
          x: first.x,
          y: first.y - 1,
        };
        if self.check_square(&square_above) {
          self.claim_square(&square_above, user_id)?;
        }
      }
    } else {
      if first.x > 0 {
        // vert: check squares left and right
        let left_square = Position {
          x: first.x - 1,
          y: first.y,
        };
        if self.check_square(&left_square) {
          self.claim_square(&left_square, user_id)?;
        }
      }

      if first.x < self.width - 1 {
        let right_square = Position {
          x: first.x,
          y: first.y,
        };
        if self.check_square(&right_square) {
          self.claim_square(&right_square, user_id)?;
        }
      }
    }

    // horz: take square first-second, second-second.y + 1, first.y+1 - second.y+1, first-first.y+1
    // vert: take square first-first.x+1, first.x+1-second.x+1,secod-second.x+1,first-second
    return Ok(());
  }

  /// takes top-left corner and checks all edges
  fn check_square(&self, top_left: &Position) -> bool {
    // at edge
    if top_left.x == self.width - 1 {
      return false;
    }
    if top_left.y == self.height - 1 {
      return false;
    }

    return self.connections.contains_key(&(
      *top_left,
      Position {
        x: top_left.x,
        y: top_left.y + 1,
      },
    )) && self.connections.contains_key(&(
      Position {
        x: top_left.x + 1,
        y: top_left.y,
      },
      Position {
        x: top_left.x + 1,
        y: top_left.y + 1,
      },
    )) && self.connections.contains_key(&(
      Position {
        x: top_left.x,
        y: top_left.y + 1,
      },
      Position {
        x: top_left.x + 1,
        y: top_left.y + 1,
      },
    )) && self.connections.contains_key(&(
      *top_left,
      Position {
        x: top_left.x + 1,
        y: top_left.y,
      },
    ));
  }

  fn claim_square(&mut self, square_space: &Position, user_id: usize) -> Result<(), GridError> {
    match self.owned.get(square_space) {
      Some(id) => return Err(GridError::SquareAlreadyOwned { user_id: *id }),
      None => {
        self.owned.insert(square_space.clone(), user_id);
        Ok(())
      }
    }
  }

  pub fn has_gaps(&self) -> bool {
    let total_squares = self.width * self.height;
    return total_squares != self.owned.len();
  }
}

impl Display for Grid {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for y in 0..self.height {
      for x in 0..self.width {
        // top
        if self
          .connections
          .contains_key(&(Position { x, y }, Position { x: x + 1, y }))
        {
          write!(f, ".---")?;
        } else {
          write!(f, ".   ")?;
        }
      }

      write!(f, "\n")?;
      for x in 0..self.width {
        // left
        if self
          .connections
          .contains_key(&(Position { x, y }, Position { x, y: y + 1 }))
        {
          if self.owned.contains_key(&Position { x, y }) {
            let user_id = self
              .owned
              .get(&Position { x, y })
              .expect("not actually owned");
            write!(f, "| {user_id} ",)?;
          } else {
            write!(f, "|   ")?;
          }
        } else {
          write!(f, "    ")?;
        }
      }
      write!(f, "\n")?;
    }

    Ok(())
  }
}

#[cfg(test)]
mod test {
  use std::collections::HashMap;

  use crate::{Grid, GridError, Position};

  #[test]
  fn it_should_create_grid() {
    let grid = Grid::new(3, 4);
    assert_eq!(
      grid,
      Grid {
        width: 3,
        height: 4,
        connections: HashMap::new(),
        owned: HashMap::new()
      }
    );
  }

  #[test]
  fn it_should_add_connection() {
    let mut grid = Grid::new(3, 4);
    let result = grid.add_connection(&Position { x: 1, y: 1 }, &Position { x: 1, y: 2 }, 2);
    assert!(result.is_ok());

    let result = grid.add_connection(&Position { x: 1, y: 1 }, &Position { x: 2, y: 1 }, 2);
    assert!(result.is_ok());
  }

  #[test]
  fn it_should_return_err_if_connection_exists() {
    let mut grid = Grid::new(3, 4);
    let _result = grid.add_connection(&Position { x: 1, y: 1 }, &Position { x: 1, y: 2 }, 2);
    let result = grid.add_connection(&Position { x: 1, y: 1 }, &Position { x: 1, y: 2 }, 2);
    assert_eq!(result, Err(GridError::AlreadyExists));
  }

  #[test]
  fn it_should_return_err_if_position_out_of_grid() {
    let mut grid = Grid::new(3, 4);
    let result = grid.add_connection(&Position { x: 25, y: 1 }, &Position { x: 1, y: 2 }, 2);
    assert_eq!(result, Err(GridError::InvalidConnection));
  }

  #[test]
  fn it_should_return_err_if_not_neighbours() {
    let mut grid = Grid::new(3, 4);
    let result = grid.add_connection(&Position { x: 0, y: 1 }, &Position { x: 2, y: 2 }, 2);
    assert_eq!(result, Err(GridError::InvalidConnection));
  }

  #[test]
  fn it_should_display() {
    let mut grid = Grid::new(3, 4);
    // e,pty
    assert_eq!(
      format!("{grid}"),
      ".   .   .   \n            \n.   .   .   \n            \n.   .   .   \n            \n.   .   .   \n            \n"
    );

    // vert
    let _result = grid.add_connection(&Position { x: 0, y: 1 }, &Position { x: 0, y: 2 }, 2);
    assert_eq!(
      format!("{grid}"),
      ".   .   .   \n            \n.   .   .   \n|           \n.   .   .   \n            \n.   .   .   \n            \n"
    );

    // horz
    let _result = grid.add_connection(&Position { x: 0, y: 1 }, &Position { x: 1, y: 1 }, 2);
    assert_eq!(
      format!("{grid}"),
      ".   .   .   \n            \n.---.   .   \n|           \n.   .   .   \n            \n.   .   .   \n            \n"
    );

    let mut grid = Grid::new(3, 3);

    // filled
    let _result = grid.add_connection(&Position { x: 0, y: 0 }, &Position { x: 0, y: 1 }, 1);
    let _result = grid.add_connection(&Position { x: 0, y: 0 }, &Position { x: 1, y: 0 }, 1);
    let _result = grid.add_connection(&Position { x: 0, y: 1 }, &Position { x: 1, y: 1 }, 1);
    let _result = grid.add_connection(&Position { x: 1, y: 0 }, &Position { x: 1, y: 1 }, 1);

    let _result = grid.add_connection(&Position { x: 1, y: 0 }, &Position { x: 2, y: 0 }, 1);
    let _result = grid.add_connection(&Position { x: 2, y: 0 }, &Position { x: 2, y: 1 }, 1);
    let _result = grid.add_connection(&Position { x: 1, y: 1 }, &Position { x: 2, y: 1 }, 2);
    assert_eq!(
      format!("{grid}"),
      ".---.---.   
| 1 | 2 |   
.---.---.   
            
.   .   .   
            
"
    );
  }

  #[test]
  fn it_should_check_square() {
    let mut grid = Grid::new(3, 4);
    // left
    let _result = grid.add_connection(&Position { x: 0, y: 1 }, &Position { x: 0, y: 2 }, 2);
    // top
    let _result = grid.add_connection(&Position { x: 0, y: 1 }, &Position { x: 1, y: 1 }, 2);
    // right
    let _result = grid.add_connection(&Position { x: 1, y: 1 }, &Position { x: 1, y: 2 }, 2);
    // bottom
    let _result = grid.add_connection(&Position { x: 0, y: 2 }, &Position { x: 1, y: 2 }, 2);
    assert!(grid.check_square(&Position { x: 0, y: 1 }));

    assert_eq!(grid.check_square(&Position { x: 1, y: 1 }), false);
    assert_eq!(grid.check_square(&Position { x: 10, y: 1 }), false);
  }

  #[test]
  fn it_should_fill_square() {
    let mut grid = Grid::new(3, 4);
    let _ = grid.claim_square(&Position { x: 0, y: 0 }, 1);
    let mut expected = HashMap::new();
    expected.insert(Position { x: 0, y: 0 }, 1);

    assert_eq!(grid.owned, expected);

    let _ = grid.claim_square(&Position { x: 1, y: 0 }, 2);
    expected.insert(Position { x: 1, y: 0 }, 2);
    assert_eq!(grid.owned, expected);

    let err = grid.claim_square(&Position { x: 0, y: 0 }, 2);
    assert_eq!(err, Err(GridError::SquareAlreadyOwned { user_id: 1 }));
  }

  #[test]
  fn adding_edges_should_claim_square() {
    let mut grid = Grid::new(3, 3);
    assert_eq!(grid.owned.len(), 0);
    // top
    let _ = grid.add_connection(&Position { x: 0, y: 0 }, &Position { x: 1, y: 0 }, 1);
    // left
    let _ = grid.add_connection(&Position { x: 0, y: 0 }, &Position { x: 0, y: 1 }, 2);
    // right
    let _ = grid.add_connection(&Position { x: 1, y: 0 }, &Position { x: 1, y: 1 }, 1);
    assert_eq!(grid.owned.len(), 0);

    let _ = grid.add_connection(&Position { x: 0, y: 1 }, &Position { x: 1, y: 1 }, 2);
    assert_eq!(grid.owned.len(), 1);

    assert!(grid.owned.contains_key(&Position { x: 0, y: 0 }));
  }

  #[test]
  fn adding_edges_should_claim_multiple_squares_vertically() {
    let mut grid = Grid::new(3, 3);
    // top
    let _ = grid.add_connection(&Position { x: 0, y: 0 }, &Position { x: 1, y: 0 }, 1);
    // left
    let _ = grid.add_connection(&Position { x: 0, y: 0 }, &Position { x: 0, y: 1 }, 2);
    // right
    let _ = grid.add_connection(&Position { x: 1, y: 0 }, &Position { x: 1, y: 1 }, 1);

    // bottom left
    let _ = grid.add_connection(&Position { x: 0, y: 1 }, &Position { x: 0, y: 2 }, 2);
    // bottom
    let _ = grid.add_connection(&Position { x: 0, y: 2 }, &Position { x: 1, y: 2 }, 1);
    // bottom right
    let _ = grid.add_connection(&Position { x: 1, y: 1 }, &Position { x: 1, y: 2 }, 1);

    // no complete squares yet
    assert_eq!(grid.owned.len(), 0);

    // fill in middle
    let _ = grid.add_connection(&Position { x: 0, y: 1 }, &Position { x: 1, y: 1 }, 2);
    // should now be two squares vert
    assert_eq!(grid.owned.len(), 2);
    assert!(grid.owned.contains_key(&Position { x: 0, y: 0 }));
    assert!(grid.owned.contains_key(&Position { x: 0, y: 1 }));
  }

  #[test]
  fn adding_edges_should_claim_multiple_squares_horizontally() {
    let mut grid = Grid::new(3, 3);
    // top left
    let _ = grid.add_connection(&Position { x: 0, y: 0 }, &Position { x: 1, y: 0 }, 1);
    // top right
    let _ = grid.add_connection(&Position { x: 1, y: 0 }, &Position { x: 2, y: 0 }, 2);
    // left
    let _ = grid.add_connection(&Position { x: 0, y: 0 }, &Position { x: 0, y: 1 }, 1);

    // bottom left
    let _ = grid.add_connection(&Position { x: 0, y: 1 }, &Position { x: 1, y: 1 }, 2);
    // bottom right
    let _ = grid.add_connection(&Position { x: 1, y: 1 }, &Position { x: 2, y: 1 }, 1);
    // right
    let _ = grid.add_connection(&Position { x: 2, y: 0 }, &Position { x: 2, y: 1 }, 1);

    // no complete squares yet
    assert_eq!(grid.owned.len(), 0);

    // fill in middle
    let _ = grid.add_connection(&Position { x: 1, y: 0 }, &Position { x: 1, y: 1 }, 2);

    // should now be two squares vert
    assert_eq!(grid.owned.len(), 2);
    assert!(grid.owned.contains_key(&Position { x: 0, y: 0 }));
    assert!(grid.owned.contains_key(&Position { x: 1, y: 0 }));
  }

  #[test]
  fn it_should_report_gaps() {
    let mut grid = Grid::new(1, 1);
    assert_eq!(grid.has_gaps(), true);

    let _ = grid.claim_square(&Position { x: 0, y: 0 }, 12);

    assert_eq!(grid.has_gaps(), false);
  }

  #[test]
  fn edges_should_have_owners() {
    let mut grid = Grid::new(3, 3);

    let _ = grid.add_connection(&Position { x: 0, y: 0 }, &Position { x: 1, y: 0 }, 1);
    let _ = grid.add_connection(&Position { x: 1, y: 0 }, &Position { x: 2, y: 0 }, 2);

    assert_eq!(
      grid
        .connections
        .get(&(Position { x: 0, y: 0 }, Position { x: 1, y: 0 })),
      Some(&1)
    );
    assert_eq!(
      grid
        .connections
        .get(&(Position { x: 1, y: 0 }, Position { x: 2, y: 0 })),
      Some(&2)
    );
  }
}
