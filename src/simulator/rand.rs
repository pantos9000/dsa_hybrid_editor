use std::time;

use rand::distr::uniform::{SampleRange, SampleUniform};
use rand_xoshiro::Xoshiro256StarStar as Rng;

#[derive(Debug)]
pub struct Rand {
    rng: Rng,
}

impl Default for Rand {
    fn default() -> Self {
        Self::new()
    }
}

impl Rand {
    pub fn new() -> Self {
        use rand::SeedableRng as _; // for seed_from_u64()
        let now = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        #[allow(clippy::cast_possible_truncation, reason = "truncation is intentional")]
        let now = now as u64;
        if now == 0 {
            log::warn!("using 0 as roller seed");
        }
        let rng = Rng::seed_from_u64(now);
        Self { rng }
    }

    pub fn random_range<T, R>(&mut self, range: R) -> T
    where
        T: SampleUniform,
        R: SampleRange<T>,
    {
        use rand::RngExt as _; // for random_range()
        self.rng.random_range(range)
    }

    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        use rand::seq::SliceRandom as _; // for shuffle()
        slice.shuffle(&mut self.rng);
    }
}
