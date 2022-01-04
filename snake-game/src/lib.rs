use std::fmt::{Display, Formatter};
use std::rc::Rc;
use crate::control::Direction;
use crate::object::snake::Snake;

use crate::render::Cell;

mod render;
mod object;
mod control;

const EMPTY_CELL: &str = "⬜";

pub struct Board {
    width: usize,
    cells: Vec<Rc<dyn Cell>>,
    snake: Snake,
}

impl Board {
    pub fn new(width: usize) -> Board {
        if width % 2 != 1 || width < 7 {
            panic!("Invalid height or width")
        };

        let mut cells: Vec<Rc<dyn Cell>> = Vec::new();

        let mut count = 0;
        while count != width * width {
            cells.push(Rc::new(EmptyCell {}));
            count += 1;
        }

        let start = width * width / 2 - 1;
        let length = 3;
        let snake = Snake::new(Direction::Right, &mut cells[start..(start + length)]);

        Board {
            width,
            cells,
            snake,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn display(&self) {
        println!("{}", &self.to_string());
    }
}

impl Display for Board {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        let mut count = 0;
        for cell in &self.cells {
            fmt.write_str(cell.to_string().as_str())?;

            count += 1;
            if count == self.width {
                fmt.write_str("\n")?;
                count = 0;
            }
        }

        Ok(())
    }
}

struct EmptyCell {}

impl Display for EmptyCell {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_str(EMPTY_CELL)
    }
}

impl Cell for EmptyCell {}