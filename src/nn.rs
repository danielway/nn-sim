pub struct Value {
    pub data: f64,
    grad: f64,
    label: Option<String>,
    op: Option<String>,
    prev: Vec<Value>,
    // back: TODO
}

impl Value {
    pub fn new(data: f64, label: &str, children: Option<Vec<Value>>) -> Value {
        Value {
            data,
            grad: 0.0,
            label: Some(label.to_string()),
            op: None,
            prev: children.unwrap_or(Vec::new()),
        }
    }

    fn new_back(data: f64, op: &str, children: Option<Vec<Value>>) -> Value {
        Value {
            data,
            grad: 0.0,
            label: None,
            op: Some(op.to_string()),
            prev: children.unwrap_or(Vec::new()),
        }
    }

    pub fn add(self, other: Value) -> Value {
        Value::new_back(self.data + other.data, "+", Some(vec![self, other]))
    }
}
