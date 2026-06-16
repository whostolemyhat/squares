use squares::{Grid, Position};

fn main() {
  let mut grid = Grid::new(3, 3);

  let user_id = 1;
  // // left
  let _result = grid.add_connection(&Position { x: 0, y: 0 }, &Position { x: 0, y: 1 }, user_id);
  let _result = grid.add_connection(&Position { x: 0, y: 0 }, &Position { x: 1, y: 0 }, user_id);
  let _result = grid.add_connection(&Position { x: 0, y: 1 }, &Position { x: 1, y: 1 }, user_id);
  let _result = grid.add_connection(&Position { x: 1, y: 0 }, &Position { x: 1, y: 1 }, user_id);

  let _result = grid.add_connection(&Position { x: 1, y: 0 }, &Position { x: 2, y: 0 }, 1);
  let _result = grid.add_connection(&Position { x: 2, y: 0 }, &Position { x: 2, y: 1 }, 1);
  let _result = grid.add_connection(&Position { x: 1, y: 1 }, &Position { x: 2, y: 1 }, 2);

  println!("{grid}");

  // // bottom
  // let _result = grid.add_connection(&Position { x: 0, y: 2 }, &Position { x: 1, y: 2 }, user_id);

  // // right
  // let _result = grid.add_connection(&Position { x: 0, y: 1 }, &Position { x: 1, y: 1 }, user_id);

  // // top
  // let _result = grid.add_connection(&Position { x: 1, y: 1 }, &Position { x: 1, y: 2 }, user_id);

  // // let result = grid.add_connection(&Position { x: 10, y: 1 }, &Position { x: 0, y: 2 }, user_id);
  // // println!("res {result:?}");
  // println!("{grid}");
  // println!("{grid:?}");

  // let is_square = grid.check_square(&Position { x: 0, y: 0 });
  // println!("{is_square}");
}
