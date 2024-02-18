use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

const MAX_ITERATIONS_INIT: u16 = 500;
const MAX_ITERATIONS_STEP: u16 = 100;
const ZOOM_SPEED: f32 = 0.1;

struct MandelbrotSet {
    max_iterations: u16,
}

impl MandelbrotSet {
    fn new(max_iterations: u16) -> Self {
        Self { max_iterations }
    }

    fn calculate(&self, c_re: f32, c_im: f32) -> u16 {
        let mut z_re = 0.0;
        let mut z_im = 0.0;
        let mut i: u16 = 0;

        while i < self.max_iterations && z_re * z_re + z_im * z_im < 4.0 {
            let temp = z_re * z_re - z_im * z_im + c_re;
            z_im = 2.0 * z_re * z_im + c_im;
            z_re = temp;
            i += 1;
        }

        i
    }
}

struct CanvasRenderer<'a> {
    canvas: &'a mut Canvas<Window>,
    mandelbrot_set: MandelbrotSet,
    zoom: f32,
    center_x: f32,
    center_y: f32,
}

impl<'a> CanvasRenderer<'a> {
    fn new(
        canvas: &'a mut Canvas<Window>,
        mandelbrot_set: MandelbrotSet,
        zoom: f32,
        center_x: f32,
        center_y: f32,
    ) -> Self {
        Self {
            canvas,
            mandelbrot_set,
            zoom,
            center_x,
            center_y,
        }
    }

    fn draw(&mut self) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let c_re = -2.0 + ((x as f32 + self.center_x) / WIDTH as f32) * 3.0 * self.zoom;
                let c_im = -1.5 + ((y as f32 + self.center_y) / HEIGHT as f32) * 3.0 * self.zoom;

                let i = self.mandelbrot_set.calculate(c_re, c_im);

                let color = if i == self.mandelbrot_set.max_iterations {
                    Color::RGB(0, 0, 0)
                } else {
                    Color::RGB(
                        ((i as f32 / self.mandelbrot_set.max_iterations as f32) * 255.0) as u8,
                        0,
                        0,
                    )
                };

                self.canvas.set_draw_color(color);
                self.canvas
                    .draw_point(Point::new(x as i32, y as i32))
                    .unwrap();
            }
        }
        self.canvas.present();
    }

    pub fn set_max_iterations(&mut self, max_iterations: u16) {
        if max_iterations < MAX_ITERATIONS_STEP {
            return;
        }
        self.mandelbrot_set.max_iterations = max_iterations;
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
    }

    pub fn set_center(&mut self, center_x: f32, center_y: f32) {
        self.center_x = center_x;
        self.center_y = center_y;
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
    let mandelbrot_set = MandelbrotSet::new(MAX_ITERATIONS_INIT);
    let mut canvas_renderer = CanvasRenderer::new(
        &mut canvas,
        mandelbrot_set,
        1.0,
        WIDTH as f32 / 2.0,
        HEIGHT as f32 / 2.0,
    );

    let mut last_mouse_x: i32 = 0;
    let mut last_mouse_y: i32 = 0;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::Escape) => break 'running,
                    Some(Keycode::Up) => {
                        canvas_renderer.set_max_iterations(
                            canvas_renderer.mandelbrot_set.max_iterations + MAX_ITERATIONS_STEP,
                        );
                    }
                    Some(Keycode::Down) => {
                        canvas_renderer.set_max_iterations(
                            canvas_renderer.mandelbrot_set.max_iterations - MAX_ITERATIONS_STEP,
                        );
                    }
                    Some(Keycode::R) => {
                        canvas_renderer.set_max_iterations(MAX_ITERATIONS_INIT);
                        canvas_renderer.set_zoom(1.0);
                        canvas_renderer.set_center(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0);
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
                    let zoom = canvas_renderer.zoom * zoom_factor;

                    // Calculate new center considering the mouse position and zoom factor
                    canvas_renderer.set_center(
                        canvas_renderer.center_x
                            + norm_mouse_x * WIDTH as f32 * (1.0 - zoom_factor) / zoom,
                        canvas_renderer.center_y
                            + norm_mouse_y * HEIGHT as f32 * (1.0 - zoom_factor) / zoom,
                    );
                    canvas_renderer.set_zoom(zoom);
                }
                Event::MouseMotion { x, y, .. } => {
                    last_mouse_x = x;
                    last_mouse_y = y;
                }
                _ => {}
            }
        }

        canvas_renderer.draw();
    }
}
