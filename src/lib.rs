use opengl_graphics::GlGraphics;
use piston::input::*;
use rand::Rng;
use std::collections::LinkedList;

mod segments;

pub struct Game {
    gl: GlGraphics,
    snake: Snake,
    fruit: Option<Fruit>,
    width: i32,
    height: i32,
    dead: bool,
}

impl Game {
    pub fn score(&self) -> usize {
        self.snake.body.len() - 1
    }

    pub fn new(gl: GlGraphics, snake: Snake, width: i32, height: i32) -> Self {
        Game {
            gl,
            snake,
            fruit: None,
            width,
            height,
            dead: false,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) -> bool {
        const BACKGROUND: [f32; 4] = [0.13671875, 0.15234375, 0.1640625, 1.0];

        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(BACKGROUND, gl);
        });

        const SCORE_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        let display = segments::from_num(self.score(), 5, (300, 300));
        segments::render(display, &mut self.gl, &args, SCORE_COLOR);

        if let Some(fruit) = self.fruit {
            fruit.render(&mut self.gl, &args);
        }

        self.snake.render(&mut self.gl, &args);

        self.dead
    }

    pub fn update(&mut self, _args: &UpdateArgs, button: Option<Button>) {
        if self.fruit.is_none() {
            self.fruit = Some(Fruit::rand_position(
                &self.snake.body,
                self.width,
                self.height,
            ));
        }

        let fruit_unwrapped = &self.fruit.unwrap();

        if self.snake.x_pos == fruit_unwrapped.x_pos && self.snake.y_pos == fruit_unwrapped.y_pos {
            self.snake.ate = true;
            self.fruit = Some(Fruit::rand_position(
                &self.snake.body,
                self.width,
                self.height,
            ));
        }

        if let Some(button) = button {
            self.snake.pressed(button);
        }
        self.snake.update(self.width, self.height);

        self.dead = self.snake.check_death();
    }
}

pub struct Snake {
    x_pos: i32,
    y_pos: i32,
    direction: Direction,
    body: LinkedList<(i32, i32)>,
    ate: bool,
}

impl Snake {
    pub fn new() -> Self {
        Snake {
            x_pos: 0,
            y_pos: 0,
            direction: Direction::Right,
            body: LinkedList::from([(0, 0)]),
            ate: false,
        }
    }

    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        const COLOR: [f32; 4] = [1.0, 0.890625, 0.5546875, 0.7];
        let mut color = COLOR;

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            for (x_pos, y_pos) in &self.body {
                let square =
                    graphics::rectangle::square((x_pos * 20) as f64, (y_pos * 20) as f64, 20_f64);
                graphics::rectangle(color, square, transform, gl);
                color = change_hue(color, 6_f32, 255_f32, 204_f32);
            }
        })
    }

    fn update(&mut self, max_width: i32, max_height: i32) {
        match self.direction {
            Direction::Right => self.x_pos += 1,
            Direction::Left => self.x_pos -= 1,
            Direction::Down => self.y_pos += 1,
            Direction::Up => self.y_pos -= 1,
        }

        if self.x_pos < 0 {
            self.x_pos = max_width - 1;
        } else if self.x_pos > max_width - 1 {
            self.x_pos = 0
        }

        if self.y_pos < 0 {
            self.y_pos = max_height - 1;
        } else if self.y_pos > max_height - 1 {
            self.y_pos = 0
        }

        self.body.push_front((self.x_pos, self.y_pos));

        if self.ate {
            self.ate = false;
        } else {
            self.body.pop_back();
        }
    }

    fn check_death(&self) -> bool {
        for (body_x, body_y) in self.body.iter().skip(1) {
            if &self.x_pos == body_x && &self.y_pos == body_y {
                return true;
            }
        }
        false
    }

    fn pressed(&mut self, button: Button) {
        let before = self.direction.clone();
        self.direction = match button {
            Button::Keyboard(Key::Up) if self.direction != Direction::Down => Direction::Up,
            Button::Keyboard(Key::Down) if self.direction != Direction::Up => Direction::Down,
            Button::Keyboard(Key::Left) if self.direction != Direction::Right => Direction::Left,
            Button::Keyboard(Key::Right) if self.direction != Direction::Left => Direction::Right,
            _ => self.direction,
        };

        println!(
            "Original Direction: {:?} | KeyPressed: {:?} | Direction Afterwards: {:?}",
            before, button, self.direction
        );
    }
}

#[derive(Copy, Clone)]
struct Fruit {
    x_pos: i32,
    y_pos: i32,
}

impl Fruit {
    fn rand_position(snake_body: &LinkedList<(i32, i32)>, x_max: i32, y_max: i32) -> Self {
        let mut new_pos;
        loop {
            new_pos = (
                rand::thread_rng().gen_range(0..x_max),
                rand::thread_rng().gen_range(0..y_max),
            );

            if !snake_body.contains(&new_pos) {
                println!("[Success] Attempted to generate fruit in {:?}", new_pos);
                break;
            } else {
                println!("[Failed] Attempted to generate fruit in {:?}", new_pos);
            }
        }

        Fruit {
            x_pos: new_pos.0,
            y_pos: new_pos.1,
        }
    }

    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        const RED: [f32; 4] = [1.0, 0.0, 0.6796875, 0.7];

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            let square = graphics::rectangle::square(
                (self.x_pos * 20) as f64 + 2.5,
                (self.y_pos * 20) as f64 + 2.5,
                15_f64,
            );

            graphics::rectangle(RED, square, transform, gl)
        })
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn change_hue(mut rgb: [f32; 4], change: f32, max: f32, min: f32) -> [f32; 4] {
    let check = &mut rgb[0..3];

    for c in check {
        *c *= 255_f32;
    }

    if rgb[0] >= max {
        if rgb[2] >= min {
            rgb[2] -= change;
        } else {
            rgb[1] += change;
        }
    }

    if rgb[1] >= max {
        if rgb[0] >= min {
            rgb[0] -= change;
        } else {
            rgb[2] += change;
        }
    }

    if rgb[2] >= max {
        if rgb[1] >= min {
            rgb[1] -= change;
        } else {
            rgb[0] += change;
        }
    }

    let check = &mut rgb[0..3];

    for c in check {
        *c /= 255_f32;
    }
    rgb
}
