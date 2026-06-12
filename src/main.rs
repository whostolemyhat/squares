use std::{
  collections::{HashMap, HashSet},
  fmt::Display,
};

#[derive(thiserror::Error, Debug, PartialEq)]
enum GridError {
  #[error("points not neighbors")]
  InvalidConnection,
  #[error("connection already exists")]
  AlreadyExists,
  #[error("square already claimed by {user_id}")]
  SquareAlreadyOwned { user_id: usize },
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position {
  x: usize,
  y: usize,
}

#[derive(Debug, PartialEq)]
struct Grid<'a> {
  width: usize,
  height: usize,
  // edges
  connections: HashSet<(&'a Position, &'a Position)>,
  // squares
  owned: HashMap<&'a Position, usize>,
}

impl<'a> Grid<'a> {
  fn new(width: usize, height: usize) -> Self {
    Grid {
      width,
      height,
      connections: HashSet::new(),
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

  fn add_connection(&mut self, first: &'a Position, second: &'a Position) -> Result<(), GridError> {
    println!("adding {first:?} {second:?}");
    println!("neighbours? {:?}", self.is_neighbour(&first, &second));
    println!(
      "already there? {:?}",
      self.connections.contains(&(first, second))
    );

    if !self.is_neighbour(first, second) {
      return Err(GridError::InvalidConnection);
    }

    if self.connections.contains(&(first, second)) {
      return Err(GridError::AlreadyExists);
    }

    self.connections.insert((first, second));

    // TODO has this just made a square?
    // is this horz or vert?
    // horz: take square first-second, second-second.y + 1, first.y+1 - second.y+1, first-first.y+1
    // vert: take square first-first.x+1, first.x+1-second.x+1,secod-second.x+1,first-second
    return Ok(());
  }

  fn check_square(&self, square: &[Position; 4]) -> bool {
    if self.connections.contains(&(&square[0], &square[1]))
      && self.connections.contains(&(&square[1], &square[2]))
      && self.connections.contains(&(&square[3], &square[2]))
      && self.connections.contains(&(&square[0], &square[3]))
    {
      return true;
    }

    false
  }

  fn claim_square(&mut self, square_space: &'a Position, user_id: usize) -> Result<(), GridError> {
    match self.owned.get(square_space) {
      Some(id) => return Err(GridError::SquareAlreadyOwned { user_id: *id }),
      None => {
        self.owned.insert(square_space, user_id);
        Ok(())
      }
    }
  }

  // TODO
  fn has_gaps(&self) -> bool {
    // . .
    // . . = 4 edges, 1 square 1x1

    // . . .
    // . . . = 7e, 2s = 1x2

    // . . .
    // . . .
    // . . . = 12e, 4s = 2x2
    // num sq = nxm
    let total_squares = self.width * self.height;
    todo!()
  }
}

impl<'a> Display for Grid<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for y in 0..self.height {
      for x in 0..self.width {
        if y > 0
          && self
            .connections
            .contains(&(&Position { x, y: y - 1 }, &Position { x, y: y }))
        {
          if self
            .connections
            .contains(&(&Position { x, y }, &Position { x: x + 1, y }))
          {
            write!(f, "|-")?;
          } else {
            write!(f, "| ")?;
          }
        } else {
          if self
            .connections
            .contains(&(&Position { x, y }, &Position { x: x + 1, y }))
          {
            write!(f, ".-")?;
          } else {
            write!(f, ". ")?;
          }
        }
      }

      write!(f, "\n")?;
    }

    Ok(())
  }
}

fn main() {
  let mut grid = Grid::new(4, 5);
  println!("{grid}");

  let _result = grid.add_connection(&Position { x: 0, y: 1 }, &Position { x: 0, y: 2 });
  let _result = grid.add_connection(&Position { x: 0, y: 2 }, &Position { x: 0, y: 3 });
  let result = grid.add_connection(&Position { x: 0, y: 1 }, &Position { x: 1, y: 1 });
  println!("res {result:?}");
  let result = grid.add_connection(&Position { x: 0, y: 1 }, &Position { x: 0, y: 2 });
  println!("res {result:?}");
  let result = grid.add_connection(&Position { x: 10, y: 1 }, &Position { x: 0, y: 2 });
  println!("res {result:?}");
  println!("{grid}");
  println!("{grid:?}");

  let is_square = grid.check_square(&[
    Position { x: 0, y: 0 },
    Position { x: 0, y: 1 },
    Position { x: 0, y: 3 },
    Position { x: 0, y: 4 },
  ]);
  println!("{is_square}");
}
#[cfg(test)]
mod test {
  use std::collections::{HashMap, HashSet};

  use crate::{Grid, GridError, Position};

  #[test]
  fn it_should_create_grid() {
    let grid = Grid::new(3, 4);
    assert_eq!(
      grid,
      Grid {
        width: 3,
        height: 4,
        connections: HashSet::new(),
        owned: HashMap::new()
      }
    );
  }

  #[test]
  fn it_should_add_connection() {
    let mut grid = Grid::new(3, 4);
    let result = grid.add_connection(&Position { x: 1, y: 1 }, &Position { x: 1, y: 2 });
    assert!(result.is_ok());

    let result = grid.add_connection(&Position { x: 1, y: 1 }, &Position { x: 2, y: 1 });
    assert!(result.is_ok());
  }

  #[test]
  fn it_should_return_err_if_connection_exists() {
    let mut grid = Grid::new(3, 4);
    let _result = grid.add_connection(&Position { x: 1, y: 1 }, &Position { x: 1, y: 2 });
    let result = grid.add_connection(&Position { x: 1, y: 1 }, &Position { x: 1, y: 2 });
    assert_eq!(result, Err(GridError::AlreadyExists));
  }

  #[test]
  fn it_should_return_err_if_position_out_of_grid() {
    let mut grid = Grid::new(3, 4);
    let result = grid.add_connection(&Position { x: 25, y: 1 }, &Position { x: 1, y: 2 });
    assert_eq!(result, Err(GridError::InvalidConnection));
  }

  #[test]
  fn it_should_return_err_if_not_neighbours() {
    let mut grid = Grid::new(3, 4);
    let result = grid.add_connection(&Position { x: 0, y: 1 }, &Position { x: 2, y: 2 });
    assert_eq!(result, Err(GridError::InvalidConnection));
  }

  #[test]
  fn it_should_display() {
    let mut grid = Grid::new(3, 4);
    assert_eq!(format!("{grid}"), ". . . \n. . . \n. . . \n. . . \n");

    let _result = grid.add_connection(&Position { x: 0, y: 1 }, &Position { x: 0, y: 2 });
    assert_eq!(format!("{grid}"), ". . . \n. . . \n| . . \n. . . \n");

    let _result = grid.add_connection(&Position { x: 0, y: 1 }, &Position { x: 1, y: 1 });
    assert_eq!(format!("{grid}"), ". . . \n.-. . \n| . . \n. . . \n");
  }

  #[test]
  fn it_should_check_square() {
    let mut grid = Grid::new(3, 4);
    // left
    let _result = grid.add_connection(&Position { x: 0, y: 1 }, &Position { x: 0, y: 2 });
    // top
    let _result = grid.add_connection(&Position { x: 0, y: 1 }, &Position { x: 1, y: 1 });
    // right
    let _result = grid.add_connection(&Position { x: 1, y: 1 }, &Position { x: 1, y: 2 });
    // bottom
    let _result = grid.add_connection(&Position { x: 0, y: 2 }, &Position { x: 1, y: 2 });
    assert!(grid.check_square(&[
      Position { x: 0, y: 1 },
      Position { x: 1, y: 1 },
      Position { x: 1, y: 2 },
      Position { x: 0, y: 2 },
    ]));

    assert_eq!(
      grid.check_square(&[
        Position { x: 0, y: 1 },
        Position { x: 1, y: 1 },
        Position { x: 1, y: 2 },
        Position { x: 1, y: 2 },
      ]),
      false
    );
    assert_eq!(
      grid.check_square(&[
        Position { x: 10, y: 1 },
        Position { x: 1, y: 1 },
        Position { x: 1, y: 2 },
        Position { x: 0, y: 2 },
      ]),
      false
    );
  }

  #[test]
  fn it_should_fill_square() {
    let mut grid = Grid::new(3, 4);
    let _ = grid.claim_square(&Position { x: 0, y: 0 }, 1);
    let mut expected = HashMap::new();
    expected.insert(&Position { x: 0, y: 0 }, 1);

    assert_eq!(grid.owned, expected);

    let _ = grid.claim_square(&Position { x: 1, y: 0 }, 2);
    expected.insert(&Position { x: 1, y: 0 }, 2);
    assert_eq!(grid.owned, expected);

    let err = grid.claim_square(&Position { x: 0, y: 0 }, 2);
    assert_eq!(err, Err(GridError::SquareAlreadyOwned { user_id: 1 }));
  }

  fn it_should_find_gaps() {}
}
