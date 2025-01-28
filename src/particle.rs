use glam::{Vec3, Vec4};
use rand::Rng;
use rayon::prelude::*;

pub struct Particle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub color: Vec4,
    pub life: f32,
}

impl Particle {
    pub fn new(position: Vec3) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            position,
            velocity: Vec3::new(
                rng.gen_range(-1.0..1.0) * 0.1,
                rng.gen_range(-1.0..1.0) * 0.1,
                rng.gen_range(-1.0..1.0) * 0.1,
            ),
            color: Vec4::new(0.2, 0.4, 0.8, 1.0), // Blue-ish water color
            life: rng.gen_range(0.5..1.0),
        }
    }
}

pub struct ParticleSystem {
    particles: Vec<Particle>,
    gravity: Vec3,
}

impl ParticleSystem {
    pub fn new(num_particles: usize) -> Self {
        let mut particles = Vec::with_capacity(num_particles);
        let mut rng = rand::thread_rng();
        
        for _ in 0..num_particles {
            particles.push(Particle::new(Vec3::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            )));
        }

        Self {
            particles,
            gravity: Vec3::new(0.0, -9.81, 0.0),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.particles.par_iter_mut().for_each(|p| {
            p.velocity += self.gravity * dt;
            p.position += p.velocity * dt;
            p.life -= dt;

            // Basic collision with ground
            if p.position.y < -1.0 {
                p.position.y = -1.0;
                p.velocity.y *= -0.5; // Damping
            }

            // Reset dead particles
            if p.life <= 0.0 {
                *p = Particle::new(Vec3::new(0.0, 1.0, 0.0));
            }
        }).collect();

        // Apply all changes to buffer
        for (idx, color) in changes {
            buffer[idx] = color;
        }
    }

    pub fn render(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        // Clear buffer
        buffer.par_iter_mut().for_each(|pixel| *pixel = 0);

        // Very basic perspective projection
        let fov = 90.0_f32.to_radians();
        let aspect = width as f32 / height as f32;
        let near = 0.1;
        let far = 100.0;

        // Skybox gradient
        for y in 0..height {
            let gradient = y as f32 / height as f32;
            let sky_color = mix_colors(
                0x87CEEB, // Sky blue
                0x1E90FF, // Darker blue
                gradient
            );
            for x in 0..width {
                buffer[y * width + x] = sky_color;
            }
        }

        // Render particles and collect changes
        let changes: Vec<_> = self.particles.par_iter().filter_map(|p| {
            // Basic perspective projection
            let z = p.position.z + 3.0; // Move camera back
            if z <= near || z >= far {
                return;
            }

            let scale = 1.0 / (z * fov.tan());
            let screen_x = ((p.position.x * scale / aspect + 1.0) * width as f32 / 2.0) as i32;
            let screen_y = ((p.position.y * scale + 1.0) * height as f32 / 2.0) as i32;

            if screen_x >= 0 && screen_x < width as i32 && 
               screen_y >= 0 && screen_y < height as i32 {
                let idx = screen_y as usize * width + screen_x as usize;
                if idx < buffer.len() {
                    // Basic lighting
                    let light_dir = Vec3::new(1.0, 1.0, 1.0).normalize();
                    let normal = Vec3::new(0.0, 1.0, 0.0);
                    let diffuse = normal.dot(light_dir).max(0.2);
                    
                    let base_color = vec4_to_u32(p.color * diffuse);
                    // Create a tuple of index and new color
                    Some((idx, blend_colors(buffer[idx], base_color, p.life as f32)))
                } else {
                    None
                }
            }
        });
    }
}

fn vec4_to_u32(color: Vec4) -> u32 {
    let r = (color.x * 255.0) as u32;
    let g = (color.y * 255.0) as u32;
    let b = (color.z * 255.0) as u32;
    let a = (color.w * 255.0) as u32;
    (a << 24) | (r << 16) | (g << 8) | b
}

fn mix_colors(c1: u32, c2: u32, t: f32) -> u32 {
    let r1 = ((c1 >> 16) & 0xFF) as f32;
    let g1 = ((c1 >> 8) & 0xFF) as f32;
    let b1 = (c1 & 0xFF) as f32;

    let r2 = ((c2 >> 16) & 0xFF) as f32;
    let g2 = ((c2 >> 8) & 0xFF) as f32;
    let b2 = (c2 & 0xFF) as f32;

    let r = (r1 * (1.0 - t) + r2 * t) as u32;
    let g = (g1 * (1.0 - t) + g2 * t) as u32;
    let b = (b1 * (1.0 - t) + b2 * t) as u32;

    (r << 16) | (g << 8) | b
}

fn blend_colors(background: u32, foreground: u32, alpha: f32) -> u32 {
    let bg_r = ((background >> 16) & 0xFF) as f32;
    let bg_g = ((background >> 8) & 0xFF) as f32;
    let bg_b = (background & 0xFF) as f32;

    let fg_r = ((foreground >> 16) & 0xFF) as f32;
    let fg_g = ((foreground >> 8) & 0xFF) as f32;
    let fg_b = (foreground & 0xFF) as f32;

    let r = (fg_r * alpha + bg_r * (1.0 - alpha)) as u32;
    let g = (fg_g * alpha + bg_g * (1.0 - alpha)) as u32;
    let b = (fg_b * alpha + bg_b * (1.0 - alpha)) as u32;

    (r << 16) | (g << 8) | b
}
