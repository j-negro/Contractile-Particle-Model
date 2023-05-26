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
        let mut neighbors_method = CellIndexMethod::new(SIMULATION_LENGHT, None, 1e-4, false);

        for i in 0..PARTICLE_COUNT {
            loop {
                let x = rng.gen_range(0.0..=SIMULATION_LENGHT);
                let y = rng.gen_range(0.0..=SIMULATION_LENGHT);
                let particle = Particle::new(i, x, y);
                particles.push(particle);

                neighbors_method.set_particles(particles.clone());
                let neighbors = neighbors_method.calculate_neighbors();
                if neighbors[i].len() == 1 {
                    break;
                }
                particles.pop();
            }
        }

        Simulation {
            particles,
            neighbors_method,
        }
    }
}
