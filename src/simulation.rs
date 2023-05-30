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
            let mut collisions = Vec::new();

            // NOTE: Calculate wall collisions
            for particle in &self.particles {
                collisions.push(particle.check_wall_collisions());
            }

            // NOTE: Calculate particle collisions
            for i in 0..self.particles.len() {
                for j in i + 1..self.particles.len() {
                    if self.particles[i].is_colliding(&self.particles[j]) {
                        let particle_coords = self.particles[i].get_coordinates();
                        let colliding_coords = self.particles[j].get_coordinates();

                        collisions[i].push(colliding_coords);
                        collisions[j].push(particle_coords);
                    }
                }
            }

            // NOTE: Update velocity and radius
            for (idx, particle) in self.particles.iter_mut().enumerate() {
                if collisions[idx].is_empty() {
                    particle.update_desired();
                } else {
                    particle.update_escape(&collisions[idx]);
                }
            }

            // NOTE: Step particles forward
            let mut to_remove = Vec::new();
            for particle in self.particles.iter_mut() {
                particle.step();
                if particle.check_reached_target() {
                    to_remove.push(particle.id);
                }
            }

            // NOTE: Remove particles that reached its target
            self.particles.retain(|p| !to_remove.contains(&p.id));
        }
    }
}
