use crate::model::game::GameState;
use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};
#[derive(Debug)]

pub struct Renderer {
    pub screen_area: Rect,
    pub clear_color: Color,
    cell_size: i32,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            screen_area: Rect::new(0, 0, width, height),
            clear_color: Color::RGB(154, 198, 3),
            cell_size: 800 / 25, // screen width by grid size
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, game: &GameState) {
        canvas.set_draw_color(self.clear_color);
        canvas.fill_rect(self.screen_area).ok().unwrap_or_default();

        self.update(canvas, game);
    }

    pub fn update(&self, canvas: &mut Canvas<Window>, game: &GameState) {
        self.draw_snake(canvas, game);
    }

    fn draw_snake(&self, canvas: &mut Canvas<Window>, game: &GameState) {
        for cell in &game.player.body {
            self.draw_cell(*cell, Color::BLACK, canvas);
        }
    }

    fn grid_to_coords(&self, position: Point) -> Point {
        Point::new(position.x * self.cell_size, position.y * self.cell_size)
    }

    pub fn draw_cell(&self, cell: Point, color: Color, canvas: &mut Canvas<Window>) {
        let coords = self.grid_to_coords(cell);
        let rect: Rect = Rect::new(
            coords.x,
            coords.y,
            (self.cell_size).try_into().unwrap(),
            (self.cell_size).try_into().unwrap(),
        );

        canvas.set_draw_color(color);
        canvas.fill_rect(rect).ok().unwrap_or_default()
    }
}
