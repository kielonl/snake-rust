use sdl2::rect::Point;

pub enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}

pub struct Player {
    pub body: Vec<Point>,
    pub speed: f32,
    direction: Direction,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            body: vec![Point::new(x, y)],
            speed: 0.0,
            direction: Direction::None,
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn update(&mut self) {
        let mut new_head = self.body[self.body.len() - 1].clone();

        match self.direction {
            Direction::Left => new_head.x -= 1,
            Direction::Right => new_head.x += 1,
            Direction::Up => new_head.y -= 1,
            Direction::Down => new_head.y += 1,
            Direction::None => {}
        }

        self.body.push(new_head);
        self.body.remove(0);
    }
}

pub struct GameState {
    pub player: Player,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            player: Player::new(12, 12),
        }
    }
}
