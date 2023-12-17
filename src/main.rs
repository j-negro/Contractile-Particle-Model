use std::fs::File;

use anyhow::Result;
use clap::Parser;

use crate::args::Cli;
use crate::simulation::Simulation;

mod args;
mod constants;
mod io;
mod particle;
mod simulation;
mod target;

fn main() -> Result<()> {
    let args = Cli::parse();

    let mut simulation = Simulation::new(args.particle_count, args.target_size);

    let file = File::create(args.xyz_output_path)?;
    io::output_simulation(&file, &simulation.particles, &simulation.target)?;

    let mut removed_times = Vec::new();

    let mut i = 0;
    loop {
        removed_times.append(&mut simulation.run(args.output_step_count));

        io::output_simulation(&file, &simulation.particles, &simulation.target)?;

        if simulation.particles.is_empty() {
            break;
        }

        if let Some(max) = args.max_steps {
            if i > max {
                break;
            }
        }

        i += 1;
    }

    io::output_times(&args.data_output_path, &removed_times)?;

    Ok(())
}
