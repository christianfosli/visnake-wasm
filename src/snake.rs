pub struct Snake {
    pub body: Vec<Position>,
    pub thickness: f64,
    direction: Direction,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            body: vec![Position { x: 0.0, y: 0.0 }],
            direction: Direction::Right,
            thickness: 25.0,
        }
    }

    fn next_position(&self) -> Position {
        let head = self.head();
        match self.direction {
            Direction::Up => Position {
                y: head.y - self.thickness,
                ..*head
            },
            Direction::Right => Position {
                x: head.x + self.thickness,
                ..*head
            },
            Direction::Down => Position {
                y: head.y + self.thickness,
                ..*head
            },
            Direction::Left => Position {
                x: head.x - self.thickness,
                ..*head
            },
        }
    }

    fn head(&self) -> &Position {
        self.body
            .last()
            .expect("snake has head because it has no body")
    }

    pub fn move_along(&mut self) {
        self.body.remove(0);
        self.body.push(self.next_position());
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_initially_moves_right() {
        let snake = Snake::new();
        assert_eq!(Direction::Right, snake.direction);
    }

    #[test]
    fn next_position_is_thickness_away_from_head() {
        let snake = Snake::new();
        let head = snake.body.first().expect("snake should have a body");
        let next = snake.next_position();
        assert_eq!(next.x, head.x + snake.thickness);
    }
}
