use std::io;

use squares::{Grid, Position};

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
  #[error("Invalid input")]
  Invalid,
}

fn parse_input(buffer: &String) -> Result<Position, ParseError> {
  println!("Parsing {}", buffer);

  let positions: Vec<u32> = buffer
    .split(",")
    .map(|i| i.trim().parse().unwrap_or(0))
    .collect();

  Ok(Position {
    x: positions[0] as usize,
    y: positions[1] as usize,
  })
}

fn main() -> Result<(), std::io::Error> {
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
  let stdin = io::stdin();

  loop {
    let mut buffer = String::new();
    println!("{grid}");
    println!("enter first point x,y");
    // get input
    stdin.read_line(&mut buffer)?;

    let first = parse_input(&buffer);
    println!("{first:?}");

    println!("enter second point x,y");
    let mut buffer = String::new();
    stdin.read_line(&mut buffer)?;

    let second = parse_input(&buffer);
    println!("{second:?}");
    if first.is_ok() && second.is_ok() {
      let result = grid.add_connection(&first.unwrap(), &second.unwrap(), 1);
      println!("{result:?}");
      println!("{grid:?}");
    }

    // get input
    // make sure it's sorted
    // add connection

    if !grid.has_gaps() {
      println!("full");
      break;
    }
  }
  Ok(())

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
