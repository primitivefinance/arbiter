use anyhow::{Ok, Result};
pub use RustQuant::{
    statistics::distributions::{Distribution, Poisson},
    stochastics::{
        cox_ingersoll_ross::CoxIngersollRoss, extended_vasicek::ExtendedVasicek, ho_lee::HoLee,
        hull_white::HullWhite, BlackDermanToy, BrownianMotion, OrnsteinUhlenbeck,
        StochasticProcess, Trajectories,
    },
};

/// Type enum for process
pub enum StochasticProcessType {
    /// Brownian motion
    BrownianMotion(BrownianMotion),
    /// Ornstein-Uhlenbeck
    OrnsteinUhlenbeck(OrnsteinUhlenbeck),
    /// Black-Derman-Toy
    BlackDermanToy(BlackDermanToy),
    /// Cox-Ingersoll-Ross
    CoxIngersollRoss(CoxIngersollRoss),
    /// Extended Vasicek
    ExtendedVasicek(ExtendedVasicek),
    /// Ho-Lee
    HoLee(HoLee),
    /// Hull-White
    HullWhite(HullWhite),
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

/// Sample Poisson process.
pub fn sample_poisson(lambda: f64) -> Result<Vec<i32>> {
    let poisson = Poisson::new(lambda);
    let float_samples = poisson.sample(1);
    let int_sample: Vec<i32> = float_samples.iter().map(|&x| x.round() as i32).collect();
    Ok(int_sample)
}

#[cfg(test)]
mod tests {

    use RustQuant::stochastics::Sigma;

    use super::*;

    #[test]
    fn poisson_process_test() {
        let lambda = 1.0;
        let result = sample_poisson(lambda);

        assert!(result.is_ok());
        let samples = result.unwrap();
        assert_eq!(samples.len(), 1);
        // Because Poisson distribution is a random process,
        // we cannot predict exact values, but we can check if mean is close to lambda.
        let mean: f64 = samples.iter().map(|&x| x as f64).sum::<f64>();
        assert!((mean - lambda).abs() < 0.2 * lambda); // tolerance of 20%
    }

    #[test]
    pub fn brownian_motion() {
        let bm = BrownianMotion::new();
        let trajectory = bm.euler_maruyama(10.0, 0.0, 0.5, 1000, 1000, false);
        assert_eq!(trajectory.times.len(), 1001);
    }

    #[test]
    pub fn ornstein_uhlenbeck() {
        let ou = OrnsteinUhlenbeck::new(1.0, 1.0, 1.0);
        let trajectory = ou.euler_maruyama(10.0, 0.0, 0.5, 1000, 1000, false);
        assert_eq!(trajectory.times.len(), 1001);
    }

    #[test]
    pub fn black_derman_toy() {
        fn theta_t(_t: f64) -> f64 {
            1.5
        }

        let sig = Sigma::Const(0.13);
        let bdt = BlackDermanToy::new(sig, theta_t);
        let trajectory = bdt.euler_maruyama(10.0, 0.0, 0.5, 1000, 1000, false);
        assert_eq!(trajectory.times.len(), 1001);
    }

    #[test]
    pub fn cox_ingersoll_ross() {
        let cir = CoxIngersollRoss::new(0.15, 0.45, 0.01);
        let trajectory = cir.euler_maruyama(10.0, 0.0, 0.5, 1000, 1000, false);
        assert_eq!(trajectory.times.len(), 1001);
    }

    #[test]
    pub fn extended_vasicek() {
        fn theta_t(_t: f64) -> f64 {
            0.5
        }
        fn alpha_t(_t: f64) -> f64 {
            2.0
        }
        let sig = 2.0;

        let ev = ExtendedVasicek::new(alpha_t, sig, theta_t);
        let trajectory = ev.euler_maruyama(10.0, 0.0, 0.5, 1000, 1000, false);
        assert_eq!(trajectory.times.len(), 1001);
    }

    #[test]
    pub fn ho_lee() {
        fn theta_t(_t: f64) -> f64 {
            2.0
        }
        let hl = HoLee::new(1.6, theta_t);
        let trajectory = hl.euler_maruyama(10.0, 0.0, 0.5, 1000, 1000, false);
        assert_eq!(trajectory.times.len(), 1001);
    }

    #[test]
    pub fn hull_white() {
        fn theta_t(_t: f64) -> f64 {
            0.5
        }
        let alpha = 2.0;
        let sig = 2.0;

        let hw = HullWhite::new(alpha, sig, theta_t);
        let trajectory = hw.euler_maruyama(10.0, 0.0, 0.5, 1000, 1000, false);
        assert_eq!(trajectory.times.len(), 1001);
    }
}
