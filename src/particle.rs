use crate::{
    constants::{
        BETA, MAX_DESIRED_VELOCITY, MAX_PARTICLE_RADIUS, MIN_PARTICLE_RADIUS, RADIUS_INCREMENT,
        SIMULATION_LENGHT, TIME_STEP,
    },
    target::Target,
};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Particle {
    pub id: usize,
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    radius: f64,
    target: (f64, f64),
    reached_first_target: bool,
}

pub enum TargetType {
    None,
    FirstTarget,
    SecondTarget,
}

impl Particle {
    pub fn new(id: usize, x: f64, y: f64, target: &Target) -> Particle {
        let mut particle = Particle {
            id,
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            radius: MIN_PARTICLE_RADIUS,
            target: (0.0, 0.0),
            reached_first_target: false,
        };

        particle.update_target(target);

        particle
    }

    fn distance(&self, other: (f64, f64)) -> f64 {
        let dx = self.x - other.0;
        let dy = self.y - other.1;

        (dx.powi(2) + dy.powi(2)).sqrt()
    }

    pub fn is_colliding(&self, other: &Particle) -> bool {
        self.distance(other.get_coordinates()) <= self.radius + other.radius
    }

    pub fn update_target(&mut self, target: &Target) {
        if !self.reached_first_target {
            if self.x < target.min || self.x > target.max {
                let target_x = rand::thread_rng().gen_range(target.min..=target.max);
                self.target = (target_x, 0.0);
            } else {
                self.target = (self.x, 0.0);
            }
        }
    }

    pub fn check_wall_collisions(&self) -> Vec<(f64, f64)> {
        let mut wall_collisions = Vec::new();

        if self.reached_first_target {
            return wall_collisions;
        }

        if self.x - self.radius <= 0.0 {
            wall_collisions.push((0.0, self.y));
        } else if self.x + self.radius >= SIMULATION_LENGHT {
            wall_collisions.push((SIMULATION_LENGHT, self.y));
        }

        if self.y - self.radius <= 0.0 {
            wall_collisions.push((self.x, 0.0));
        } else if self.y + self.radius >= SIMULATION_LENGHT {
            wall_collisions.push((self.x, SIMULATION_LENGHT));
        }

        wall_collisions
    }

    /// Checks if the first or second target is reached
    ///
    /// If the first target is reached, then the target is updated to point to the second target
    ///
    /// If the second target is reached the simulation for this particle is finished
    pub fn check_reached_target(&mut self) -> TargetType {
        if self.distance(self.target) <= self.radius {
            if self.reached_first_target {
                return TargetType::SecondTarget;
            }
            self.target = (self.x, -3.0);
            self.reached_first_target = true;
            return TargetType::FirstTarget;
        }
        TargetType::None
    }

    pub fn step(&mut self) {
        self.x += self.vx * TIME_STEP;
        self.y += self.vy * TIME_STEP;
    }

    pub fn update_escape(&mut self, collision_points: &[(f64, f64)]) {
        self.radius = MIN_PARTICLE_RADIUS;

        let mut collision_vector = (0.0, 0.0);

        for (x, y) in collision_points {
            let diff = (self.x - x, self.y - y);
            let norm = (diff.0.powi(2) + diff.1.powi(2)).sqrt();

            collision_vector.0 += diff.0 / norm;
            collision_vector.1 += diff.1 / norm;
        }

        let norm = (collision_vector.0.powi(2) + collision_vector.1.powi(2)).sqrt();

        self.vx = MAX_DESIRED_VELOCITY * collision_vector.0 / norm;
        self.vy = MAX_DESIRED_VELOCITY * collision_vector.1 / norm;
    }

    fn calculate_desired_velocity(radius: f64) -> f64 {
        MAX_DESIRED_VELOCITY
            * ((radius - MIN_PARTICLE_RADIUS) / (MAX_PARTICLE_RADIUS - MIN_PARTICLE_RADIUS))
                .powf(BETA)
    }

    pub fn update_desired(&mut self) {
        if self.radius < MAX_PARTICLE_RADIUS {
            self.radius += RADIUS_INCREMENT;
        }

        let desired_velocity = Self::calculate_desired_velocity(self.radius);

        let target_direction = (self.target.0 - self.x, self.target.1 - self.y);
        let target_norm = (target_direction.0.powi(2) + target_direction.1.powi(2)).sqrt();

        self.vx = desired_velocity * target_direction.0 / target_norm;
        self.vy = desired_velocity * target_direction.1 / target_norm;
    }

    pub fn get_velocities(&self) -> (f64, f64) {
        (self.vx, self.vy)
    }

    pub fn get_coordinates(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    pub fn get_radius(&self) -> f64 {
        self.radius
    }
}
