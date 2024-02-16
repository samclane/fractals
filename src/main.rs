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
            let c_re = -2.0 + ((x as f32 + center_x) / WIDTH as f32) * 3.0 * zoom;
            let c_im = -1.5 + ((y as f32 + center_y) / HEIGHT as f32) * 3.0 * zoom;
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
    let mut last_mouse_x: i32 = 0;
    let mut last_mouse_y: i32 = 0;

    draw_mandelbrot(&mut canvas, max_iterations, zoom, center_x, center_y);
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
                    Some(Keycode::R) => {
                        max_iterations = MAX_ITERATIONS_INIT;
                        zoom = 1.0;
                        center_x = WIDTH as f32 / 2.0;
                        center_y = HEIGHT as f32 / 2.0;
                    }
                    _ => {}
                },
                Event::MouseWheel { y, .. } => {
                    let zoom_factor = if y > 0 {
                        1.0 + ZOOM_SPEED
                    } else {
                        1.0 / (1.0 + ZOOM_SPEED)
                    };

                    // Normalize mouse coordinates to range [-1, 1]
                    let norm_mouse_x =
                        (last_mouse_x as f32 - WIDTH as f32 / 2.0) / (WIDTH as f32 / 2.0);
                    let norm_mouse_y =
                        (last_mouse_y as f32 - HEIGHT as f32 / 2.0) / (HEIGHT as f32 / 2.0);

                    // Adjust zoom
                    zoom *= zoom_factor;

                    // Calculate new center considering the mouse position and zoom factor
                    center_x = center_x + norm_mouse_x * WIDTH as f32 * (1.0 - zoom_factor) / zoom;
                    center_y = center_y + norm_mouse_y * HEIGHT as f32 * (1.0 - zoom_factor) / zoom;
                }
                Event::MouseMotion { x, y, .. } => {
                    last_mouse_x = x;
                    last_mouse_y = y;
                }
                _ => {}
            }
        }

        draw_mandelbrot(&mut canvas, max_iterations, zoom, center_x, center_y);
        canvas.present();
    }
}
