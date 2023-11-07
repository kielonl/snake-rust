use std::time::Duration;

use sdl2::event::Event;
mod view;
use view::board_view::Renderer;

mod model;
use model::game::{Direction, GameState};

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 800;
const TARGET_FPS: u32 = 8;
fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("snake", SCREEN_WIDTH, SCREEN_HEIGHT)
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let game_view: Renderer = Renderer::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut game_state: GameState = GameState::new();

    let mut running = true;
    let target_frame_duration = Duration::from_secs(1) / TARGET_FPS;
    let mut event_queue = sdl_context.event_pump().unwrap();

    while running {
        let frame_start = std::time::Instant::now();

        for event in event_queue.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    running = false;
                }

                Event::KeyDown { keycode, .. } => match keycode.unwrap() {
                    sdl2::keyboard::Keycode::D => {
                        game_state.player.change_direction(Direction::Right)
                    }
                    sdl2::keyboard::Keycode::A => {
                        game_state.player.change_direction(Direction::Left)
                    }
                    sdl2::keyboard::Keycode::W => game_state.player.change_direction(Direction::Up),
                    sdl2::keyboard::Keycode::S => {
                        game_state.player.change_direction(Direction::Down)
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        game_state.player.update();
        game_view.render(&mut canvas, &game_state);
        canvas.present();

        let elapsed_frame_time = frame_start.elapsed();
        if elapsed_frame_time < target_frame_duration {
            std::thread::sleep(target_frame_duration - elapsed_frame_time);
        }
    }
    Ok(())
}
