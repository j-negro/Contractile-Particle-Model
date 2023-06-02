use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Contractile Particle Model", author, version, about)]
pub struct Cli {
    #[arg(short, long)]
    pub max_steps: Option<usize>,

    #[arg(short, long, default_value_t = 1)]
    pub output_step_count: usize,

    #[arg(short, long, default_value_t = 200)]
    pub parcile_count: usize,

    #[arg(short, long, default_value_t = 1.2)]
    pub target_size: f64,

    #[arg(short, long, default_value_t = String::from("./simulation.xyz"))]
    pub xyz_output_path: String,

    #[arg(short, long, default_value_t = String::from("./particles_time.txt"))]
    pub data_output_path: String,
}
