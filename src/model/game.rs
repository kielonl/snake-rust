use rand::{thread_rng, Rng};
use sdl2::{keyboard::Keycode, pixels::Color, rect::Point};

#[derive(PartialEq)]
pub enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}

pub struct Player {
    pub body: Vec<Point>,
    direction: Direction,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            body: vec![Point::new(x, y)],
            direction: Direction::None,
        }
    }

    pub fn update(&mut self) {
        let new_head: Point = self.next_position();

        self.body.push(new_head);
        self.body.remove(0);
    }

    pub fn change_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Left => {
                if self.direction != Direction::Right {
                    self.direction = Direction::Left
                }
            }
            Direction::Right => {
                if self.direction != Direction::Left {
                    self.direction = Direction::Right
                }
            }
            Direction::Down => {
                if self.direction != Direction::Up {
                    self.direction = Direction::Down
                }
            }
            Direction::Up => {
                if self.direction != Direction::Down {
                    self.direction = Direction::Up
                }
            }
            Direction::None => {}
        }
    }

    pub fn extend(&mut self) {
        let last_element = self.body[self.body.len() - 1].clone();
        self.body.push(last_element);
    }

    pub fn next_position(&mut self) -> Point {
        let mut current_position = self.body[self.body.len() - 1].clone();

        match self.direction {
            Direction::Left => current_position.x -= 1,
            Direction::Right => current_position.x += 1,
            Direction::Up => current_position.y -= 1,
            Direction::Down => current_position.y += 1,
            Direction::None => {}
        }

        return current_position;
    }

    pub fn is_alive(&mut self) -> bool {
        let next_pos = self.next_position();

        if self.is_self_colliding(next_pos.x, next_pos.y) {
            return false;
        }

        next_pos.x > 0 && next_pos.y > 0 && next_pos.x < 25 - 1 && next_pos.y < 25 - 1
    }

    pub fn is_self_colliding(&mut self, x: i32, y: i32) -> bool {
        let mut checked = 0;
        for cell in &self.body {
            if checked == self.body.len() - 1 {
                break;
            }

            if x == cell.x && y == cell.y {
                return true;
            }

            checked += 1;
        }

        return false;
    }
}

pub struct Apple {
    pub position: Point,
    pub color: Color,
}

impl Apple {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            position: Point::new(x, y),
            color: Color::RED,
        }
    }
}

pub struct GameState {
    pub player: Player,
    pub apple: Apple,
    pub is_game_over: bool,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            player: Player::new(12, 12),
            apple: Apple::new(15, 15),
            is_game_over: false,
        }
    }

    pub fn respawn_apple(&mut self) {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1..24);
        let mut new_y = rng.gen_range(1..24);
        while self.player.is_self_colliding(new_x, new_y) {
            new_x = rng.gen_range(1..24);
            new_y = rng.gen_range(1..24);
        }

        self.apple.position = Point::new(new_x, new_y);
    }

    pub fn update(&mut self) {
        if self.is_game_over {
            return;
        }

        if !self.player.is_alive() {
            self.is_game_over = true
        }

        if self.player.next_position() == self.apple.position {
            self.respawn_apple();
            self.player.extend();
        }

        self.player.update();
    }

    pub fn get_direction_key(&mut self, keycode: Option<Keycode>) {
        match keycode {
            Some(Keycode::A) => self.player.change_direction(Direction::Left),
            Some(Keycode::D) => self.player.change_direction(Direction::Right),
            Some(Keycode::W) => self.player.change_direction(Direction::Up),
            Some(Keycode::S) => self.player.change_direction(Direction::Down),
            _ => {}
        }
    }
}
