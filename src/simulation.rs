use neighbors::Particle as MethodParticle;
use rand::Rng;

use crate::{
    constants::{MIN_PARTICLE_RADIUS, PARTICLE_COUNT, SIMULATION_LENGHT},
    particle::Particle,
};

pub struct Simulation {
    pub particles: Vec<Particle>,
}

impl Simulation {
    pub fn new() -> Simulation {
        let mut particles = Vec::with_capacity(PARTICLE_COUNT);
        let mut rng = rand::thread_rng();

        for i in 0..PARTICLE_COUNT {
            loop {
                let x =
                    rng.gen_range(MIN_PARTICLE_RADIUS..=(SIMULATION_LENGHT - MIN_PARTICLE_RADIUS));
                let y =
                    rng.gen_range(MIN_PARTICLE_RADIUS..=(SIMULATION_LENGHT - MIN_PARTICLE_RADIUS));
                let particle = Particle::new(i, x, y);

                if particles.iter().any(|p| particle.is_colliding(&p)) {
                    continue;
                }

                particles.push(particle);
                break;
            }
        }

        Simulation { particles }
    }

    pub fn run(&mut self, steps: usize) {
        for _ in 0..steps {
            let mut neighbors = Vec::new();

            for particle in &self.particles {
                neighbors.push((particle.id, Vec::new()));
            }

            for i in 0..self.particles.len() {
                for j in i + 1..self.particles.len() {
                    if self.particles[i].is_colliding(&self.particles[j]) {
                        let particle_coords = self.particles[i].get_coordinates();
                        let neighbor_coords = self.particles[j].get_coordinates();

                        neighbors[i].1.push(neighbor_coords);
                        neighbors[j].1.push(particle_coords);
                    }
                }
            }

            let mut to_remove = Vec::new();

            for (idx, particle) in self.particles.iter_mut().enumerate() {
                let mut collisions_points = particle.check_wall_collisions();

                let neighbors_coords = neighbors[idx].1.clone();

                if neighbors_coords.is_empty() && collisions_points.is_empty() {
                    particle.update_desired();
                } else {
                    collisions_points.extend(neighbors_coords);

                    particle.update_escape(&collisions_points);
                }
            }

            for particle in self.particles.iter_mut() {
                particle.step();
                if particle.check_reached_target() {
                    to_remove.push(particle.id);
                }
            }

            self.particles.retain(|p| !to_remove.contains(&p.id));
        }
    }
}
