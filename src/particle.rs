use crate::constants::{
    BETA, MAX_DESIRED_VELOCITY, MAX_PARTICLE_RADIUS, MIN_PARTICLE_RADIUS, RADIUS_INCREMENT,
    SIMULATION_LENGHT, TARGET_LEFT_X, TARGET_RIGHT_X, TARGET_SIZE,
};
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
    pub reached_first_target: bool,
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
            reached_first_target: false,
        }
    }

    fn distance(&self, other: (f64, f64)) -> f64 {
        let dx = self.x - other.0;
        let dy = self.y - other.1;

        (dx.powi(2) + dy.powi(2)).sqrt()
    }

    pub fn is_colliding(&self, other: &Particle) -> bool {
        self.distance(other.get_coordinates()) <= self.radius + other.radius
    }

    fn get_target(x_coordinate: f64) -> (f64, f64) {
        let x_min_target = TARGET_LEFT_X + 0.2 * TARGET_SIZE;
        let x_max_target = TARGET_RIGHT_X + 0.8 * TARGET_SIZE;
        if x_coordinate < x_min_target || x_coordinate > x_max_target {
            let target_x = rand::thread_rng().gen_range(x_min_target..=x_max_target);
            return (target_x, 0.0);
        }
        (x_coordinate, 0.0)
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

    pub fn check_reached_target(&mut self) -> bool {
        // Check if the particle has reached the first or second target.
        // If the particle has reached the first target, update the target
        // to the second target and return false.
        // If the particle has reached the second target, return true.
        if self.distance(self.target) <= self.radius {
            if self.reached_first_target {
                return true;
            } else {
                self.target = (self.x, -3.0);
                self.reached_first_target = true;
            }
        }
        false
    }

    pub fn step(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
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
