use crate::simulation::Simulation;

mod constants;
mod particle;
mod simulation;

fn main() {
    println!("Hello, world!");

    Simulation::new().run(1);
}
