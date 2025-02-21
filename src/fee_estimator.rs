use lightning::chain::chaininterface::{ConfirmationTarget, FeeEstimator};

pub struct YourFeeEstimator;

impl YourFeeEstimator {
    pub fn new() -> Self {
        YourFeeEstimator
    }
}

impl FeeEstimator for YourFeeEstimator {
    fn get_est_sat_per_1000_weight(&self, _confirmation_target: ConfirmationTarget) -> u32 {
        return 1000;
    }
}
