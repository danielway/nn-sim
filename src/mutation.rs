use micrograd_rs::MLP;
use rand::{rngs::ThreadRng, thread_rng, Rng};

pub trait MutationStrategy {
    fn mutate(&mut self, base: &mut MLP);
}

pub struct ConstantMutationStrategy(ThreadRng, f64);

impl ConstantMutationStrategy {
    pub fn new(rate: f64) -> ConstantMutationStrategy {
        ConstantMutationStrategy(thread_rng(), rate)
    }
}

impl MutationStrategy for ConstantMutationStrategy {
    fn mutate(&mut self, base: &mut MLP) {
        for p in base.parameters() {
            if self.0.gen_range(0.0..1.0) > self.1 {
                if self.0.gen_bool(0.5) {
                    p.adjust(0.1);
                } else {
                    p.adjust(-0.1)
                }
            }
        }
    }
}
