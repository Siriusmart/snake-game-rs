extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;

use dialog::{Choice, DialogBox};
// use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::window::WindowSettings;
use piston_window::*;
use snake_game::*;
use std::{collections::LinkedList, time::Instant};

fn main() {
    loop {
        const WIDTH: i32 = 30;
        const HEIGHT: i32 = 30;
        let opengl = OpenGL::V4_5;
        let mut window: PistonWindow =
            WindowSettings::new("Snake game", [(WIDTH * 20) as u32, (HEIGHT * 20) as u32])
                .graphics_api(opengl)
                .exit_on_esc(true)
                .build()
                .unwrap();

        let mut game = Game::new(GlGraphics::new(opengl), Snake::new(), WIDTH, HEIGHT);

        let mut last_pressed = LinkedList::new();

        let mut events = Events::new(EventSettings::new()).ups(8);

        // cooldown in mills
        const KEY_COOLDOWN: u128 = 350;

        // cooldowns
        let mut up = Instant::now();
        let mut down = Instant::now();
        let mut right = Instant::now();
        let mut left = Instant::now();

        while let Some(e) = events.next(&mut window) {
            if let Some(k) = e.button_args() {
                match k.button {
                    Button::Keyboard(Key::Up) if up.elapsed().as_millis() > KEY_COOLDOWN => {
                        up = Instant::now();
                        last_pressed.push_back(k.button);
                    }
                    Button::Keyboard(Key::Down) if down.elapsed().as_millis() > KEY_COOLDOWN => {
                        down = Instant::now();
                        last_pressed.push_back(k.button);
                    }
                    Button::Keyboard(Key::Left) if left.elapsed().as_millis() > KEY_COOLDOWN => {
                        left = Instant::now();
                        last_pressed.push_back(k.button);
                    }
                    Button::Keyboard(Key::Right) if right.elapsed().as_millis() > KEY_COOLDOWN => {
                        right = Instant::now();
                        last_pressed.push_back(k.button);
                    }
                    _ => (),
                };
            }

            if let Some(r) = e.update_args() {
                game.update(&r, last_pressed.pop_front());

                events = Events::new(EventSettings::new())
                    .ups(((game.score() as f64 + 5 as f64).log(1.2)) as u64);
            }

            if let Some(r) = e.render_args() {
                if game.render(&r) {
                    println!("Your score: {}", game.score());
                    break;
                }
            }
        }

        window.hide();

        let choice = dialog::Question::new(format!(
            "Game ended! You scored {}\n\nDo you want play again?",
            game.score()
        ))
        .title("Game over")
        .show()
        .expect("Could not display dialog box");

        match choice {
            Choice::Yes => {}
            _ => break,
        };
    }
}
