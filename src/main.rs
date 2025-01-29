mod state;

use state::State;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = State::new(window).await;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            state.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.window().inner_size()),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                state.window().request_redraw();
            }
            _ => {}
        }
    });
}

fn main() {
    pollster::block_on(run());

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let scale = 4.0;
    let offset_x = -0.5;
    let offset_y = 0.0;

    for (i, pixel) in buffer.iter_mut().enumerate() {
        let x = (i % WIDTH) as f64 / WIDTH as f64 * scale - scale/2.0 + offset_x;
        let y = (i / WIDTH) as f64 / HEIGHT as f64 * scale - scale/2.0 + offset_y;
        
        let c = Complex::new(x, y);
        let mut z = Complex::new(0.0, 0.0);
        let mut iter = 0;

        while iter < MAX_ITER && z.norm_sqr() <= 4.0 {
            z = z * z + c;
            iter += 1;
        }

        // Color mapping
        *pixel = if iter == MAX_ITER {
            0x000000 // Black for points in the set
        } else {
            let hue = (iter as f64 / MAX_ITER as f64 * 360.0) as u32;
            hsv_to_rgb(hue, 100, 100)
        };
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Switch between fractal types
        if window.is_key_pressed(Key::Key1, KeyRepeat::No) {
            fractal_type = FractalType::Mandelbrot;
            current_max_iter = 1;
        } else if window.is_key_pressed(Key::Key2, KeyRepeat::No) {
            fractal_type = FractalType::Julia;
            current_max_iter = 1;
        } else if window.is_key_pressed(Key::Key3, KeyRepeat::No) {
            fractal_type = FractalType::BurningShip;
            current_max_iter = 1;
        } else if window.is_key_pressed(Key::Key4, KeyRepeat::No) {
            fractal_type = FractalType::Tricorn;
            current_max_iter = 1;
        } else if window.is_key_pressed(Key::Key5, KeyRepeat::No) {
            fractal_type = FractalType::Newton;
            current_max_iter = 1;
        }

        if current_max_iter < MAX_ITER {
            current_max_iter += 1;
            
            // Update the visualization in parallel
            buffer.par_iter_mut().enumerate().for_each(|(i, pixel)| {
                let screen_x = i % WIDTH;
                let screen_y = i / WIDTH;
                
                let x = screen_x as f64 / WIDTH as f64 * scale - scale/2.0 + offset_x;
                let y = screen_y as f64 / HEIGHT as f64 * scale - scale/2.0 + offset_y;

                let point = Complex::new(x, y);
                let iter = match fractal_type {
                    FractalType::Mandelbrot => {
                        let mut z = Complex::new(0.0, 0.0);
                        let mut i = 0;
                        while i < current_max_iter && z.norm_sqr() <= 4.0 {
                            z = z * z + point;
                            i += 1;
                        }
                        i
                    },
                    FractalType::Julia => {
                        let mut z = point;
                        let mut i = 0;
                        while i < current_max_iter && z.norm_sqr() <= 4.0 {
                            z = z * z + JULIA_C;
                            i += 1;
                        }
                        i
                    },
                    FractalType::BurningShip => {
                        let mut z = Complex::new(0.0, 0.0);
                        let mut i = 0;
                        while i < current_max_iter && z.norm_sqr() <= 4.0 {
                            z = Complex::new(f64::abs(z.re), -f64::abs(z.im)) * 
                                Complex::new(f64::abs(z.re), -f64::abs(z.im)) + point;
                            i += 1;
                        }
                        i
                    },
                    FractalType::Tricorn => {
                        let mut z = Complex::new(0.0, 0.0);
                        let mut i = 0;
                        while i < current_max_iter && z.norm_sqr() <= 4.0 {
                            z = Complex::new(z.re, -z.im) * Complex::new(z.re, -z.im) + point;
                            i += 1;
                        }
                        i
                    },
                    FractalType::Newton => {
                        let mut z = point;
                        let mut i = 0;
                        while i < current_max_iter {
                            // f(z) = z³ - 1
                            // f'(z) = 3z²
                            let fz = z * z * z - Complex::new(1.0, 0.0);
                            let fpz = Complex::new(3.0, 0.0) * z * z;
                            let next = z - fz / fpz;
                            if (next - z).norm() < 1e-6 {
                                break;
                            }
                            z = next;
                            i += 1;
                        }
                        i
                    }
                };

                *pixel = if iter == current_max_iter {
                    0x000000 // Black for points in the set
                } else {
                    let hue = (iter as f64 / current_max_iter as f64 * 360.0) as u32;
                    hsv_to_rgb(hue, 100, 100)
                };
            });
        }

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}

fn hsv_to_rgb(hue: u32, sat: u32, val: u32) -> u32 {
    let h = (hue % 360) as f64 / 60.0;
    let s = sat as f64 / 100.0;
    let v = val as f64 / 100.0;

    let c = v * s;
    let x = c * (1.0 - ((h % 2.0) - 1.0).abs());
    let m = v - c;

    let (r, g, b) = match h as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    let (r, g, b) = (
        ((r + m) * 255.0) as u32,
        ((g + m) * 255.0) as u32,
        ((b + m) * 255.0) as u32,
    );

    (r << 16) | (g << 8) | b
}
