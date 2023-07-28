use anyhow::{Ok, Result};
use RustQuant::{
    statistics::distributions::{Distribution, Poisson},
    stochastics::{BrownianMotion, OrnsteinUhlenbeck, StochasticProcess, Trajectories},
};

/// Type enum for process
pub enum StochasticProcessType {
    /// Brownian motion
    BrownianMotion(BrownianMotion),
    /// Ornstein-Uhlenbeck
    OrnsteinUhlenbeck(OrnsteinUhlenbeck),
}
/// Struct for all processes init parameters.
pub struct EulerMaruyamaInput {
    /// initial value at t_0
    pub x_0: f64,
    /// initial time point
    pub t_0: f64,
    /// terminal time point
    pub t_n: f64,
    /// number of time steps between t_0 and t_n
    pub n_steps: usize,
    /// how many process trajectories to simulate
    pub m_paths: usize,
    /// run in parallel or not (recommended for > 1000 paths)
    pub parallel: bool,
}

/// Create new process and run euler maruyama.
pub fn new_procces(
    proccess_type: StochasticProcessType,
    config: EulerMaruyamaInput,
) -> Result<Trajectories> {
    let trajectories: Trajectories = match proccess_type {
        StochasticProcessType::BrownianMotion(process) => process.euler_maruyama(
            config.x_0,
            config.t_0,
            config.t_n,
            config.n_steps,
            config.m_paths,
            config.parallel,
        ),
        StochasticProcessType::OrnsteinUhlenbeck(process) => process.euler_maruyama(
            config.x_0,
            config.t_0,
            config.t_n,
            config.n_steps,
            config.m_paths,
            config.parallel,
        ),
    };

    Ok(trajectories)
}

/// Sample Poisson process.
pub fn poisson_process(lambda: f64, length: usize) -> Result<Vec<i32>> {
    let poisson = Poisson::new(lambda);
    let float_samples = poisson.sample(length);
    let int_samples: Vec<i32> = float_samples.iter().map(|&x| x.round() as i32).collect();
    Ok(int_samples)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_process_brownian_motion() {
        let bm = BrownianMotion::new();
        let config = EulerMaruyamaInput {
            x_0: 0.0,
            t_0: 0.0,
            t_n: 1.0,
            n_steps: 100,
            m_paths: 10,
            parallel: false,
        };
        let process = StochasticProcessType::BrownianMotion(bm);
        let result = new_procces(process, config);

        assert!(result.is_ok());
        let trajectories = result.unwrap();
        assert_eq!(trajectories.times.len(), 101);
        assert_eq!(trajectories.paths.len(), 10);
    }

    #[test]
    fn new_process_ornstein_uhlenbeck() {
        let ou = OrnsteinUhlenbeck::new(1.0, 1.0, 1.0);
        let config = EulerMaruyamaInput {
            x_0: 0.0,
            t_0: 0.0,
            t_n: 1.0,
            n_steps: 100,
            m_paths: 10,
            parallel: false,
        };
        let process = StochasticProcessType::OrnsteinUhlenbeck(ou);
        let result = new_procces(process, config);

        assert!(result.is_ok());
        let trajectories = result.unwrap();
        assert_eq!(trajectories.times.len(), 101);
        assert_eq!(trajectories.paths.len(), 10);
    }

    #[test]
    fn poisson_process_test() {
        let lambda = 1.0;
        let length = 100;
        let result = poisson_process(lambda, length);

        assert!(result.is_ok());
        let samples = result.unwrap();
        assert_eq!(samples.len(), length);
        // Because Poisson distribution is a random process,
        // we cannot predict exact values, but we can check if mean is close to lambda.
        let mean: f64 = samples.iter().map(|&x| x as f64).sum::<f64>() / length as f64;
        assert!((mean - lambda).abs() < 0.2 * lambda); // tolerance of 20%
    }

    #[test]
    pub fn brownian_motion() {
        let bm = BrownianMotion::new();
        let trajectory = bm.euler_maruyama(10.0, 0.0, 0.5, 1000, 1000, false);
        assert_eq!(trajectory.times.len(), 1001);
    }
}
