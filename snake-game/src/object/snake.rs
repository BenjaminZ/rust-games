use std::fmt::{Display, Formatter};
use std::rc::Rc;

use crate::Cell;
use crate::control::Direction;

const SNAKE_BODY: &str = "⬛";

pub struct Snake {
    body_cells: Vec<Rc<SnakeBodyCell>>,
    direction: Direction,
}

impl Snake {
    pub fn new(direction: Direction, cells: &mut [Rc<dyn Cell>]) -> Snake {
        if cells.len() == 0 {
            panic!("Invalid cell length")
        }

        let mut body_cells: Vec<Rc<SnakeBodyCell>> = Vec::new();
        for i in 0..cells.len() {
            let body_cell = Rc::new(SnakeBodyCell {});
            let clone = Rc::clone(&body_cell);
            cells[i] = body_cell;
            body_cells.push(clone);
        }


        Snake {
            body_cells,
            direction,
        }
    }
}

struct SnakeBodyCell {}

impl Display for SnakeBodyCell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(SNAKE_BODY)?;
        Ok(())
    }
}

impl Cell for SnakeBodyCell {}