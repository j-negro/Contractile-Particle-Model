use std::fs::File;

use anyhow::Result;

use crate::simulation::Simulation;

mod constants;
mod io;
mod particle;
mod simulation;

fn main() -> Result<()> {
    println!("Hello, world!");

    let mut simulation = Simulation::new();

    let file = File::create("simulation.xyz")?;
    io::output_simulation(&file, &simulation.particles)?;

    for _ in 1..=100 {
        simulation.run(100);

        io::output_simulation(&file, &simulation.particles)?;
    }

    Ok(())
}
