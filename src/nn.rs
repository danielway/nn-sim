use std::collections::HashSet;
use std::fmt::Debug;

type ValueId = usize;

pub struct ValueTree {
    values: Vec<Value>,
}

impl ValueTree {
    pub fn new() -> ValueTree {
        ValueTree { values: Vec::new() }
    }

    pub fn create_value(&mut self, data: f64, label: &str, children: Option<Vec<usize>>) -> &Value {
        self.values
            .push(Value::new(self.values.len(), data, label, children));
        &self.values[self.values.len() - 1]
    }

    pub fn add_values(&mut self, first_id: usize, second_id: usize, label: &str) -> &Value {
        let first = &self.values[first_id];
        let second = &self.values[second_id];

        self.values
            .push(first.add(second, self.values.len(), label));
        &self.values[self.values.len() - 1]
    }

    pub fn mul_values(&mut self, first_id: usize, second_id: usize, label: &str) -> &Value {
        let first = &self.values[first_id];
        let second = &self.values[second_id];

        self.values
            .push(first.mul(second, self.values.len(), label));
        &self.values[self.values.len() - 1]
    }

    pub fn tanh_value(&mut self, id: usize, label: &str) -> &Value {
        let value = &self.values[id];

        self.values.push(value.tanh(self.values.len(), label));
        &self.values[self.values.len() - 1]
    }

    pub fn backward(&mut self, id: usize) {
        let mut topo: Vec<usize> = Vec::new();
        let mut visited: HashSet<usize> = HashSet::new();

        self.build_topo(&mut topo, &mut visited, id);
        topo.reverse();

        self.values[id].grad = 1.0;
        for id in topo {
            let value = &self.values[id];
            if let Some(prop_fn) = value.propagate {
                prop_fn(id, self);
            }
        }
    }

    fn build_topo(&self, topo: &mut Vec<usize>, visited: &mut HashSet<usize>, id: usize) {
        if !visited.contains(&id) {
            visited.insert(id);
            let value = &self.values[id];
            for child_id in &value.prev {
                self.build_topo(topo, visited, *child_id);
            }
            topo.push(id);
        }
    }
}

pub trait ValueAccessor {
    fn get(&self, id: usize) -> &Value;
    fn get_mut(&mut self, id: usize) -> &mut Value;
}

impl ValueAccessor for ValueTree {
    fn get(&self, id: usize) -> &Value {
        &self.values[id]
    }

    fn get_mut(&mut self, id: usize) -> &mut Value {
        &mut self.values[id]
    }
}

pub struct Value {
    id: ValueId,
    data: f64,
    grad: f64,
    label: Option<String>,
    op: Option<String>,
    prev: Vec<usize>,
    propagate: Option<fn(usize, &mut dyn ValueAccessor)>,
}

impl Value {
    pub fn new(id: ValueId, data: f64, label: &str, children: Option<Vec<usize>>) -> Value {
        Value {
            id,
            data,
            grad: 0.0,
            label: Some(label.to_string()),
            op: None,
            prev: children.unwrap_or(Vec::new()),
            propagate: None,
        }
    }

    fn new_back(
        id: ValueId,
        data: f64,
        label: &str,
        op: &str,
        children: Option<Vec<usize>>,
        propagate: fn(usize, &mut dyn ValueAccessor),
    ) -> Value {
        Value {
            id,
            data,
            grad: 0.0,
            label: Some(label.to_string()),
            op: Some(op.to_string()),
            prev: children.unwrap_or(Vec::new()),
            propagate: Some(propagate),
        }
    }

    pub fn id(&self) -> ValueId {
        self.id
    }

    fn add(&self, other: &Value, new_id: ValueId, label: &str) -> Value {
        Value::new_back(
            new_id,
            self.data + other.data,
            label,
            "+",
            Some(vec![self.id, other.id]),
            |id, accessor| {
                let value = accessor.get(id);
                let a_id = value.prev[0];
                let b_id = value.prev[1];
                let grad = value.grad;

                let a = accessor.get_mut(a_id);
                a.grad += 1.0 * grad;

                let b = accessor.get_mut(b_id);
                b.grad += 1.0 * grad;
            },
        )
    }

    fn mul(&self, other: &Value, new_id: ValueId, label: &str) -> Value {
        Value::new_back(
            new_id,
            self.data * other.data,
            label,
            "*",
            Some(vec![self.id, other.id]),
            |id, accessor| {
                let value = accessor.get(id);
                let a_id = value.prev[0];
                let b_id = value.prev[1];
                let grad = value.grad;

                let a_data = accessor.get(a_id).data;
                let b_data = accessor.get(b_id).data;

                let a = accessor.get_mut(a_id);
                a.grad += b_data * grad;

                let b = accessor.get_mut(b_id);
                b.grad += a_data * grad;
            },
        )
    }

    fn tanh(&self, new_id: ValueId, label: &str) -> Value {
        Value::new_back(
            new_id,
            self.data.tanh(),
            label,
            "tanh",
            Some(vec![self.id]),
            |id, accessor| {
                let value = accessor.get(id);
                let t = value.data;
                let grad = value.grad;

                let prev = accessor.get_mut(value.prev[0]);
                let new_val = (1.0 - t.powf(2.0)) * grad;
                prev.grad += new_val;
            },
        )
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Value")
            .field("label", &self.label)
            .field("op", &self.op)
            .field("data", &self.data)
            .field("grad", &self.grad)
            .finish()
    }
}
