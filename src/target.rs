use crate::constants::SIMULATION_LENGHT;

#[derive(Debug, Clone)]
pub struct Target {
    pub size: f64,
    pub left: f64,
    pub right: f64,
    pub min: f64,
    pub max: f64,
}

impl Target {
    pub fn new(size: f64) -> Target {
        let left = (SIMULATION_LENGHT - size) / 2.0;
        let right = left + size;
        let min = left + 0.2 * size;
        let max = left + 0.8 * size;

        Target {
            size,
            left,
            right,
            min,
            max,
        }
    }
}
