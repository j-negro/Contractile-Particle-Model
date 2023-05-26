use crate::constants::{get_target, MIN_PARTICLE_RADIUS};
use neighbors::Particle as MethodParticle;
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
            target: get_target(x),
        }
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
