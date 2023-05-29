use crate::constants::{MIN_PARTICLE_RADIUS, TARGET_LEFT_X, TARGET_RIGHT_X, TARGET_SIZE};
use neighbors::Particle as MethodParticle;
use rand::Rng;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct Particle {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub radius: f64,
    pub target: (f64, f64),
}

impl Particle {
    pub fn new(id: usize, x: f64, y: f64) -> Particle {
        Particle {
            id,
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            radius: MIN_PARTICLE_RADIUS,
            target: Self::get_target(x),
        }
    }

    fn distance(&self, other: &Particle) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;

        (dx.powi(2) + dy.powi(2)).sqrt()
    }

    pub fn is_colliding(&self, other: &Particle) -> bool {
        self.distance(other) <= self.radius + other.radius
    }

    pub fn get_target(x_coordinate: f64) -> (f64, f64) {
        let x_min_target = TARGET_LEFT_X + 0.2 * TARGET_SIZE;
        let x_max_target = TARGET_RIGHT_X + 0.8 * TARGET_SIZE;
        if x_coordinate < x_min_target || x_coordinate > x_max_target {
            let target_x = rand::thread_rng().gen_range(x_min_target..=x_max_target);
            return (target_x, 0.0);
        }
        (x_coordinate, 0.0)
    }
}

impl MethodParticle for Particle {
    fn get_id(&self) -> u32 {
        self.id as u32
    }

    fn get_coordinates(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    fn get_radius(&self) -> f64 {
        self.radius
    }
}

impl PartialEq for Particle {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Particle {}

impl Hash for Particle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
