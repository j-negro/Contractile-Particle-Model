use std::fs::File;

use anyhow::Result;

use crate::simulation::Simulation;

mod constants;
mod io;
mod particle;
mod simulation;

fn main() -> Result<()> {
    let mut simulation = Simulation::new();

    let file = File::create("simulation.xyz")?;
    io::output_simulation(&file, &simulation.particles)?;

    for _ in 1..=1000 {
        simulation.run(100);

        io::output_simulation(&file, &simulation.particles)?;
    }

    Ok(())
}
