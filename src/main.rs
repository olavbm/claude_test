use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};

mod particle;
use particle::ParticleSystem;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const NUM_PARTICLES: usize = 5000; // Reduced for better performance with SPH

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut particle_system = ParticleSystem::new(NUM_PARTICLES);
    
    let mut window = Window::new(
        "Water Particle System - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(Duration::from_micros(16600)));

    let mut last_update = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();
        let dt = now.duration_since(last_update).as_secs_f32();
        last_update = now;

        // Update particle physics
        particle_system.update(dt);
        
        // Render particles with skybox and lighting
        particle_system.render(&mut buffer, WIDTH, HEIGHT);

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
