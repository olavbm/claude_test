use minifb::{Key, Window, WindowOptions};
use num_complex::Complex;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const MAX_ITER: u32 = 200;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut current_max_iter: u32 = 1;
    let mut window = Window::new(
        "Mandelbrot Set - Press ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

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
        if current_max_iter < MAX_ITER {
            current_max_iter += 1;
            
            // Update the visualization with new max_iter
            for (i, pixel) in buffer.iter_mut().enumerate() {
                let x = (i % WIDTH) as f64 / WIDTH as f64 * scale - scale/2.0 + offset_x;
                let y = (i / WIDTH) as f64 / HEIGHT as f64 * scale - scale/2.0 + offset_y;
                
                let c = Complex::new(x, y);
                let mut z = Complex::new(0.0, 0.0);
                let mut iter = 0;

                while iter < current_max_iter && z.norm_sqr() <= 4.0 {
                    z = z * z + c;
                    iter += 1;
                }

                *pixel = if iter == current_max_iter {
                    0x000000 // Black for points in the set
                } else {
                    let hue = (iter as f64 / current_max_iter as f64 * 360.0) as u32;
                    hsv_to_rgb(hue, 100, 100)
                };
            }
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
