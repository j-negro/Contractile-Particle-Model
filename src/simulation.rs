use rand::Rng;

use crate::{
    constants::{MIN_PARTICLE_RADIUS, SIMULATION_LENGHT, TIME_STEP},
    particle::{Particle, TargetType},
    target::Target,
};

pub struct Simulation {
    pub particles: Vec<Particle>,
    pub target: Target,
    time: f64,
}

impl Simulation {
    pub fn new(particle_count: usize, target_size: f64) -> Simulation {
        let mut particles = Vec::with_capacity(particle_count);
        let mut rng = rand::thread_rng();

        let target = Target::new(target_size);

        for i in 0..particle_count {
            loop {
                let x =
                    rng.gen_range(MIN_PARTICLE_RADIUS..=(SIMULATION_LENGHT - MIN_PARTICLE_RADIUS));
                let y =
                    rng.gen_range(MIN_PARTICLE_RADIUS..=(SIMULATION_LENGHT - MIN_PARTICLE_RADIUS));
                let particle = Particle::new(i, x, y, &target);

                if particles.iter().any(|p| particle.is_colliding(p)) {
                    continue;
                }

                particles.push(particle);
                break;
            }
        }

        Simulation {
            particles,
            target,
            time: 0.0,
        }
    }

    pub fn run(&mut self, steps: usize) -> Vec<(f64, usize)> {
        let mut exit_particles = Vec::with_capacity(steps);

        for _ in 0..steps {
            self.time += TIME_STEP;

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
                    particle.update_target(&self.target);
                }
            }

            // NOTE: Step particles forward
            let mut to_remove = Vec::new();
            let mut first_target_hits = 0;
            for particle in &mut self.particles {
                particle.step();

                match particle.check_reached_target() {
                    TargetType::None => {}
                    TargetType::FirstTarget => first_target_hits += 1,
                    TargetType::SecondTarget => to_remove.push(particle.id),
                }
            }
            if first_target_hits > 0 {
                exit_particles.push((self.time, first_target_hits));
            }
            // NOTE: Remove particles that reached its target
            self.particles.retain(|p| !to_remove.contains(&p.id));
        }

        exit_particles
    }
}
