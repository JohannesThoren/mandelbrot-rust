extern crate sdl2;

use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
};

const SCREEN_W: u32 = 500;
const SCREEN_H: u32 = 500;
const MAX_ITERS: u32 = 1000;
const XSCALE: f64 = 3.47;
const YSCALE: f64 = 2.24;

fn calculate_mandelbrot(
    x_scale: f64,
    y_scale: f64,
    x_off: f64,
    y_off: f64,
) -> Vec<(i32, i32, f64)> {
    let mut points_list: Vec<(i32, i32, f64)> = Vec::new();

    for px in 0..SCREEN_W {
        let x0 = ((px as f64 / SCREEN_W as f64) * x_scale) + x_off;
        for py in 0..SCREEN_H {
            let y0 = ((py as f64 / SCREEN_H as f64) * y_scale) + y_off;
            let mut x = 0.0;
            let mut y = 0.0;
            let mut iters = 0;
            while (x * x) + (y * y) <= (2.0 * 2.0) && iters < MAX_ITERS {
                let xtemp = (x * x) - (y * y) + x0;
                y = 2.0 * x * y + y0;
                x = xtemp;
                iters += 1;
            }

            points_list.push((px as i32, py as i32, iters as f64))
        }
    }

    return points_list;
}

fn draw(
    mut canvas: Canvas<Window>,
    x_scale: f64,
    y_scale: f64,
    x_off: f64,
    y_off: f64,
) -> Canvas<Window> {
    let points = calculate_mandelbrot(x_scale, y_scale, x_off, y_off);
    for point in points {
        let c = (point.2 * 0.2 + 1.0).sin() + 1.0;
        let c = c * 100.0;

        canvas.set_draw_color(Color::RGB(
            c.floor() as u8,
            c.floor() as u8,
            c.floor() as u8,
        ));
        canvas.fill_rect(Rect::new(point.0, point.1, 1, 1));
    }

    return canvas;
}

fn main() {
    let mut running = true;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Example", SCREEN_W, SCREEN_H)
        .build()
        .unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut canvas: Canvas<Window> = window.into_canvas().present_vsync().build().unwrap();

    // for x in 0..SCREEN_W {
    //     canvas.set_draw_color(Color::RGB(255, 210, 0));
    //     canvas.fill_rect(Rect::new(x as i32, 0, 1, x));

    // }

    let mut x_scale = 3.47;
    let mut y_scale = 2.24;
    let mut x_off = -2.0;
    let mut y_off = -1.12;
    let mut zoom = 1.0;

    let update_screen = |mut canvas: Canvas<Window>,
                         x_off: f64,
                         y_off: f64,
                         x_scale: f64,
                         y_scale: f64|
     -> Canvas<Window> {
        canvas.clear();
        canvas = draw(canvas, x_scale, y_scale, x_off, y_off);
        canvas.present();
        return canvas;
    };

    let mut do_zoom =
        |mut canvas: Canvas<Window>, zoom_factor: f64, pos: (i32, i32)| -> Canvas<Window> {
            println!("{:?}", pos);

            let x = pos.0 as f64 / SCREEN_W as f64;
            let y = pos.1 as f64 / SCREEN_H as f64;
            println!("{} {}", x, y);

            let w1 = XSCALE / zoom;
            let w2 = XSCALE / (zoom * zoom_factor);
            let h1 = YSCALE / zoom;
            let h2 = YSCALE / (zoom * zoom_factor);
            x_off = (x as f64 * (w1 - w2)) + x_off;
            y_off = (y as f64 * (h1 - h2)) + y_off;

            zoom = zoom * zoom_factor;

            x_scale = XSCALE / zoom;
            y_scale = YSCALE / zoom;

            return update_screen(
                canvas,
                x_off.clone(),
                y_off.clone(),
                x_scale.clone(),
                y_scale.clone(),
            );
        };

    canvas = do_zoom(canvas, 1.0, (SCREEN_W as i32 / 2, SCREEN_H as i32 / 2));

    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::MouseButtonDown {
                    which,
                    timestamp,
                    window_id,
                    mouse_btn,
                    clicks,
                    x,
                    y,
                } => match mouse_btn {
                    sdl2::mouse::MouseButton::Left => {
                        canvas = do_zoom(canvas, 5.0, (x, y));
                    }
                    sdl2::mouse::MouseButton::Right => {
                        canvas = do_zoom(canvas, 1.0 / 5.0, (x, y));
                    }
                    _ => {}
                },
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    running = false;
                }
                _ => {}
            }
        }
    }
}
