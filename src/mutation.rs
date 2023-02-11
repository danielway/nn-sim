use micrograd_rs::MLP;

pub trait MutationStrategy {
    fn mutate(&self, base: &mut MLP);
}
