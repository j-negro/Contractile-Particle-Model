use neighbors::{cell_index_method::CellIndexMethod, NeighborMethod};
use rand::Rng;

use crate::{
    constants::{PARTICLE_COUNT, SIMULATION_LENGHT},
    particle::Particle,
};

pub struct Simulation {
    pub particles: Vec<Particle>,
    neighbors_method: CellIndexMethod<Particle>,
}

impl Simulation {
    pub fn new() -> Simulation {
        let mut particles = Vec::with_capacity(PARTICLE_COUNT);
        let mut rng = rand::thread_rng();
        // TODO: Validate issue with interaction range too small.
        let neighbors_method = CellIndexMethod::new(SIMULATION_LENGHT, Some(20), 0.0, false);

        for i in 0..PARTICLE_COUNT {
            loop {
                let x = rng.gen_range(0.0..=SIMULATION_LENGHT);
                let y = rng.gen_range(0.0..=SIMULATION_LENGHT);
                let particle = Particle::new(i, x, y);

                if particles.iter().any(|p| particle.is_colliding(&p)) {
                    continue;
                }

                particles.push(particle);
            }
        }

        Simulation {
            particles,
            neighbors_method,
        }
    }
}
