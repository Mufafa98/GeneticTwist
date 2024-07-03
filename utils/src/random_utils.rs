use config::random::Seed;
use rand::distributions::{self, Distribution};
use rand::rngs::StdRng;
use rand::SeedableRng;

// [start, end)
pub fn generate_between(start: f32, end: f32) -> f32 {
    let seed = Seed::new();
    let mut rng = StdRng::seed_from_u64(seed.get_seed());
    let uniform = distributions::Uniform::from(start..end);
    uniform.sample(&mut rng)
}
