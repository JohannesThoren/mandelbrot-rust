extern crate sdl2;

use sdl2::{render::Canvas, video::Window, pixels::Color, rect::Rect, event::Event, keyboard::Keycode};
const SCREEN_W: u32 = 300;
const SCREEN_H: u32 = 300;

fn calculate_mandelbrot() -> Vec<(i32, i32, i32)>{
    let mut points_list: Vec<(i32, i32, i32)> = Vec::new();

    todo!();

    return points_list
}

fn main() {
    let mut running = true;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Example", SCREEN_W, SCREEN_H).build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut canvas : Canvas<Window> = window.into_canvas()
        .present_vsync() 
        .build().unwrap();

    
    
    
    // for x in 0..SCREEN_W {
    //     canvas.set_draw_color(Color::RGB(255, 210, 0));
    //     canvas.fill_rect(Rect::new(x as i32, 0, 1, x));
        
    // }


    while running {
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        // all rendering goes here
        canvas.present();
        for event in event_pump.poll_iter() {
            match event {
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