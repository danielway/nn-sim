use std::{
    cell::{Ref, RefCell},
    collections::HashSet,
    fmt::Debug,
    hash::Hash,
    ops::{Add, Deref, Mul, Neg},
    rc::Rc,
};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Value(Rc<RefCell<ValueInternal>>);

impl Value {
    pub fn from<T>(t: T) -> Value
    where
        T: Into<Value>,
    {
        t.into()
    }

    fn new(value: ValueInternal) -> Value {
        Value(Rc::new(RefCell::new(value)))
    }

    pub fn with_label(self, label: &str) -> Value {
        self.borrow_mut().label = Some(label.to_string());
        self
    }

    pub fn tanh(&self) -> Value {
        let result = self.borrow().data.tanh();

        let prop_fn: PropagateFn = |value| {
            let mut previous = value.previous[0].borrow_mut();
            previous.gradient += (1.0 - value.data.powf(2.0)) * value.gradient;
        };

        Value::new(ValueInternal::new(
            result,
            None,
            Some("tanh".to_string()),
            vec![self.clone()],
            Some(prop_fn),
        ))
    }

    pub fn backward(&self) {
        let mut visited: HashSet<Value> = HashSet::new();

        self.borrow_mut().gradient = 1.0;
        self.backward_internal(&mut visited, self);
    }

    fn backward_internal(&self, visited: &mut HashSet<Value>, value: &Value) {
        if !visited.contains(&value) {
            visited.insert(value.clone());

            let borrowed_value = value.borrow();
            if let Some(prop_fn) = borrowed_value.propagate {
                prop_fn(&borrowed_value);
            }

            for child_id in &value.borrow().previous {
                self.backward_internal(visited, child_id);
            }
        }
    }
}

impl Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.borrow().hash(state);
    }
}

impl Deref for Value {
    type Target = Rc<RefCell<ValueInternal>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Into<f64>> From<T> for Value {
    fn from(t: T) -> Value {
        Value::new(ValueInternal::new(t.into(), None, None, Vec::new(), None))
    }
}

impl Add<Value> for Value {
    type Output = Value;

    fn add(self, other: Value) -> Self::Output {
        add(&self, &other)
    }
}

impl<'a, 'b> Add<&'b Value> for &'a Value {
    type Output = Value;

    fn add(self, other: &'b Value) -> Self::Output {
        add(self, other)
    }
}

fn add(a: &Value, b: &Value) -> Value {
    let result = a.borrow().data + b.borrow().data;

    let prop_fn: PropagateFn = |value| {
        let mut first = value.previous[0].borrow_mut();
        let mut second = value.previous[1].borrow_mut();

        first.gradient += value.gradient;
        second.gradient += value.gradient;
    };

    Value::new(ValueInternal::new(
        result,
        None,
        Some("+".to_string()),
        vec![a.clone(), b.clone()],
        Some(prop_fn),
    ))
}

impl Mul<Value> for Value {
    type Output = Value;

    fn mul(self, other: Value) -> Self::Output {
        mul(&self, &other)
    }
}

impl<'a, 'b> Mul<&'b Value> for &'a Value {
    type Output = Value;

    fn mul(self, other: &'b Value) -> Self::Output {
        mul(self, other)
    }
}

fn mul(a: &Value, b: &Value) -> Value {
    let result = a.borrow().data * b.borrow().data;

    let prop_fn: PropagateFn = |value| {
        let mut first = value.previous[0].borrow_mut();
        let mut second = value.previous[1].borrow_mut();

        first.gradient += second.data * value.gradient;
        second.gradient += first.data * value.gradient;
    };

    Value::new(ValueInternal::new(
        result,
        None,
        Some("*".to_string()),
        vec![a.clone(), b.clone()],
        Some(prop_fn),
    ))
}

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        mul(&self, &Value::from(-1))
    }
}

impl<'a> Neg for &'a Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        mul(self, &Value::from(-1))
    }
}

type PropagateFn = fn(value: &Ref<ValueInternal>);

pub struct ValueInternal {
    data: f64,
    gradient: f64,
    label: Option<String>,
    operation: Option<String>,
    previous: Vec<Value>,
    propagate: Option<PropagateFn>,
}

impl ValueInternal {
    fn new(
        data: f64,
        label: Option<String>,
        op: Option<String>,
        prev: Vec<Value>,
        propagate: Option<PropagateFn>,
    ) -> ValueInternal {
        ValueInternal {
            data,
            gradient: 0.0,
            label,
            operation: op,
            previous: prev,
            propagate,
        }
    }
}

impl PartialEq for ValueInternal {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
            && self.gradient == other.gradient
            && self.label == other.label
            && self.operation == other.operation
            && self.previous == other.previous
    }
}

impl Eq for ValueInternal {}

impl Hash for ValueInternal {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.data.to_bits().hash(state);
        self.gradient.to_bits().hash(state);
        self.label.hash(state);
        self.operation.hash(state);
        self.previous.hash(state);
    }
}

impl Debug for ValueInternal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ValueInternal")
            .field("data", &self.data)
            .field("gradient", &self.gradient)
            .field("label", &self.label)
            .field("operation", &self.operation)
            .field("previous", &self.previous)
            .finish()
    }
}