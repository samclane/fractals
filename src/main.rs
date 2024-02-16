use std::time::Instant;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

const MAX_ITERATIONS_INIT: u16 = 500;
const MAX_ITERATIONS_STEP: u16 = 100;
const ZOOM_SPEED: f32 = 0.1;

// Measure the time it takes to execute a function
fn timeit(f: impl FnOnce()) {
    let start = Instant::now();
    f();
    let elapsed = start.elapsed();
    println!(
        "Elapsed: {}.{:03} seconds",
        elapsed.as_secs(),
        elapsed.subsec_millis()
    );
}

// Draw the Mandelbrot set on the canvas
fn draw_mandelbrot(
    canvas: &mut Canvas<sdl2::video::Window>,
    max_iterations: u16,
    zoom: f32,
    center_x: f32,
    center_y: f32,
) {
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let (c_re, c_im) = canvas_to_complex(x as f32, y as f32, zoom, center_x, center_y);
            let mut z_re = 0.0;
            let mut z_im = 0.0;

            let mut i: u16 = 0;
            while i < max_iterations && z_re * z_re + z_im * z_im < 4.0 {
                let temp = z_re * z_re - z_im * z_im + c_re;
                z_im = 2.0 * z_re * z_im + c_im;
                z_re = temp;
                i += 1;
            }

            let color = if i == max_iterations {
                Color::RGB(0, 0, 0)
            } else {
                Color::RGB(((i as f32 / max_iterations as f32) * 255.0) as u8, 0, 0)
            };

            canvas.set_draw_color(color);
            canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();
        }
    }
}

// Convert the canvas coordinates to the complex plane
fn canvas_to_complex(x: f32, y: f32, zoom: f32, center_x: f32, center_y: f32) -> (f32, f32) {
    let c_re = -2.0 + ((x + center_x) / WIDTH as f32) * 3.0 * zoom;
    let c_im = -1.5 + ((y + center_y) / HEIGHT as f32) * 3.0 * zoom;
    (c_re, c_im)
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Mandelbrot", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut max_iterations: u16 = MAX_ITERATIONS_INIT;
    let mut zoom: f32 = 1.0;
    let mut center_x: f32 = WIDTH as f32 / 2.0;
    let mut center_y: f32 = HEIGHT as f32 / 2.0;

    timeit(|| draw_mandelbrot(&mut canvas, max_iterations, zoom, center_x, center_y));
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::Escape) => break 'running,
                    Some(Keycode::Up) => {
                        max_iterations += MAX_ITERATIONS_STEP;
                    }
                    Some(Keycode::Down) => {
                        max_iterations -= MAX_ITERATIONS_STEP;
                        if max_iterations < MAX_ITERATIONS_STEP {
                            max_iterations = MAX_ITERATIONS_STEP;
                        }
                    }
                    _ => {}
                },
                Event::MouseWheel { y, .. } => {
                    let zoom_factor = if y > 0 {
                        1. - ZOOM_SPEED
                    } else {
                        1. + ZOOM_SPEED
                    };
                    zoom *= zoom_factor;
                }
                Event::MouseMotion { x, y, .. } => {
                    center_x = x as f32;
                    center_y = y as f32;
                }
                _ => {}
            }
        }

        // timeit(|| calculate_mandelbrot(&mut canvas, max_iterations, zoom, offset_x, offset_y));
        draw_mandelbrot(&mut canvas, max_iterations, zoom, center_x, center_y);
        canvas.present();
    }
}
