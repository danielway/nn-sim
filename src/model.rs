use micrograd_rs::{Value, MLP};

use crate::mutation::MutationStrategy;

#[derive(Clone)]
pub struct Model(MLP);

impl Model {
    pub fn random() -> Model {
        Model(MLP::new(4, vec![8, 8, 4]))
    }

    pub fn mutate(&mut self, strategy: &mut dyn MutationStrategy) -> Model {
        let mut new_mlp = self.0.clone();
        strategy.mutate(&mut new_mlp);
        Model(new_mlp)
    }

    pub fn forward(&self, fs: Vec<f64>) -> Vec<f64> {
        let vs = fs.iter().map(|f| Value::from(*f)).collect();
        let os = self.0.forward(vs);
        os.iter().map(|v| v.data()).collect()
    }
}
