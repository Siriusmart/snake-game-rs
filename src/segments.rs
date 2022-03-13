use opengl_graphics::GlGraphics;
use piston::input::*;

pub fn render(segments: Vec<[f64; 4]>, gl: &mut GlGraphics, args: &RenderArgs, color: [f32; 4]) {
    gl.draw(args.viewport(), |c, gl| {
        let transform = c.transform;

        for square in  segments{
            graphics::rectangle(color, square, transform, gl);
        }
    });
}

pub fn from_num(num: usize, size: i32, mut offset: (i32, i32)) -> Vec<[f64; 4]> {
    let num = num.to_string();
    let mut out = Vec::new();

    for c in num.chars().rev() {
        match c {
            '0' => out.extend(segment(
                [true, true, true, true, true, true, false],
                size,
                offset,
            )),
            '1' => out.extend(segment(
                [false, true, true, false, false, false, false],
                size,
                offset,
            )),
            '2' => out.extend(segment(
                [true, true, false, true, true, false, true],
                size,
                offset,
            )),
            '3' => out.extend(segment(
                [true, true, true, true, false, false, true],
                size,
                offset,
            )),
            '4' => out.extend(segment(
                [false, true, true, false, false, true, true],
                size,
                offset,
            )),
            '5' => out.extend(segment(
                [true, false, true, true, false, true, true],
                size,
                offset,
            )),
            '6' => out.extend(segment(
                [true, false, true, true, true, true, true],
                size,
                offset,
            )),
            '7' => out.extend(segment(
                [true, true, true, false, false, false, false],
                size,
                offset,
            )),
            '8' => out.extend(segment(
                [true, true, true, true, true, true, true],
                size,
                offset,
            )),
            '9' => out.extend(segment(
                [true, true, true, true, false, true, true],
                size,
                offset,
            )),
            _ => {}
        };

        offset.0 -= 10 * size;
    }

    out
}

fn segment(segments: [bool; 7], size: i32, offset: (i32, i32)) -> Vec<[f64; 4]> {
    let mut out = Vec::new();
    if segments[0] {
        out.push(graphics::rectangle::square(
            (-2 * size + offset.0) as f64,
            (-6 * size + offset.1) as f64,
            size as f64,
        ));
        out.push(graphics::rectangle::square(
            (-1 * size + offset.0) as f64,
            (-6 * size + offset.1) as f64,
            size as f64,
        ));
        out.push(graphics::rectangle::square(
            (-0 * size + offset.0) as f64,
            (-6 * size + offset.1) as f64,
            size as f64,
        ));
        out.push(graphics::rectangle::square(
            (1 * size + offset.0) as f64,
            (-6 * size + offset.1) as f64,
            size as f64,
        ));
        out.push(graphics::rectangle::square(
            (2 * size + offset.0) as f64,
            (-6 * size + offset.1) as f64,
            size as f64,
        ));
    }

    if segments[1] {
        out.push(graphics::rectangle::square(
            (3 * size + offset.0) as f64,
            (-1 * size + offset.1) as f64,
            size as f64,
        ));
        out.push(graphics::rectangle::square(
            (3 * size + offset.0) as f64,
            (-2 * size + offset.1) as f64,
            size as f64,
        ));
        out.push(graphics::rectangle::square(
            (3 * size + offset.0) as f64,
            (-3 * size + offset.1) as f64,
            size as f64,
        ));
        out.push(graphics::rectangle::square(
            (3 * size + offset.0) as f64,
            (-4 * size + offset.1) as f64,
            size as f64,
        ));
        out.push(graphics::rectangle::square(
            (3 * size + offset.0) as f64,
            (-5 * size + offset.1) as f64,
            size as f64,
        ));
    }

    if segments[2] {
        out.push(graphics::rectangle::square(
            (3 * size + offset.0) as f64,
            (1 * size + offset.1) as f64,
            size as f64,
        ));
        out.push(graphics::rectangle::square(
            (3 * size + offset.0) as f64,
            (2 * size + offset.1) as f64,
            size as f64,
        ));
        out.push(graphics::rectangle::square(
            (3 * size + offset.0) as f64,
            (3 * size + offset.1) as f64,
            size as f64,
        ));
        out.push(graphics::rectangle::square(
            (3 * size + offset.0) as f64,
            (4 * size + offset.1) as f64,
            size as f64,
        ));
        out.push(graphics::rectangle::square(
            (3 * size + offset.0) as f64,
            (5 * size + offset.1) as f64,
            size as f64,
        ));
    }

    if segments[3] {
        out.push(graphics::rectangle::square(
            (-2 * size + offset.0) as f64,
            (6 * size + offset.1) as f64,
            size as f64,
        ));
        out.push(graphics::rectangle::square(
            (-1 * size + offset.0) as f64,
            (6 * size + offset.1) as f64,
            size as f64,
        ));
        out.push(graphics::rectangle::square(
            (-0 * size + offset.0) as f64,
            (6 * size + offset.1) as f64,
            size as f64,
        ));
        out.push(graphics::rectangle::square(
            (1 * size + offset.0) as f64,
            (6 * size + offset.1) as f64,
            size as f64,
        ));
        out.push(graphics::rectangle::square(
            (2 * size + offset.0) as f64,
            (6 * size + offset.1) as f64,
            size as f64,
        ));
    }

    if segments[4] {
        out.push(graphics::rectangle::square(
            (-3 * size + offset.0) as f64,
            (1 * size + offset.1) as f64,
            size as f64,
        ));
        out.push(graphics::rectangle::square(
            (-3 * size + offset.0) as f64,
            (2 * size + offset.1) as f64,
            size as f64,
        ));
        out.push(graphics::rectangle::square(
            (-3 * size + offset.0) as f64,
            (3 * size + offset.1) as f64,
            size as f64,
        ));
        out.push(graphics::rectangle::square(
            (-3 * size + offset.0) as f64,
            (4 * size + offset.1) as f64,
            size as f64,
        ));
        out.push(graphics::rectangle::square(
            (-3 * size + offset.0) as f64,
            (5 * size + offset.1) as f64,
            size as f64,
        ));
    }

    if segments[5] {
        out.push(graphics::rectangle::square(
            (-3 * size + offset.0) as f64,
            (-1 * size + offset.1) as f64,
            size as f64,
        ));
        out.push(graphics::rectangle::square(
            (-3 * size + offset.0) as f64,
            (-2 * size + offset.1) as f64,
            size as f64,
        ));
        out.push(graphics::rectangle::square(
            (-3 * size + offset.0) as f64,
            (-3 * size + offset.1) as f64,
            size as f64,
        ));
        out.push(graphics::rectangle::square(
            (-3 * size + offset.0) as f64,
            (-4 * size + offset.1) as f64,
            size as f64,
        ));
        out.push(graphics::rectangle::square(
            (-3 * size + offset.0) as f64,
            (-5 * size + offset.1) as f64,
            size as f64,
        ));
    }

    if segments[6] {
        out.push(graphics::rectangle::square(
            (-2 * size + offset.0) as f64,
            (0 * size + offset.1) as f64,
            size as f64,
        ));
        out.push(graphics::rectangle::square(
            (-1 * size + offset.0) as f64,
            (0 * size + offset.1) as f64,
            size as f64,
        ));
        out.push(graphics::rectangle::square(
            (-0 * size + offset.0) as f64,
            (0 * size + offset.1) as f64,
            size as f64,
        ));
        out.push(graphics::rectangle::square(
            (1 * size + offset.0) as f64,
            (0 * size + offset.1) as f64,
            size as f64,
        ));
        out.push(graphics::rectangle::square(
            (2 * size + offset.0) as f64,
            (0 * size + offset.1) as f64,
            size as f64,
        ));
    }

    out
}
