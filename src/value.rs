use std::collections::HashSet;

use crate::numbers::Integer;

#[derive(Clone, Debug)]
pub enum Value<Int> {
    Bool(bool),
    Int(Int),
    SetOfInt(HashSet<Int>),
}

pub trait GetValue<T> {
    fn try_get(&self) -> Option<&T>;
}

impl<Int> GetValue<bool> for Value<Int> {
    fn try_get(&self) -> Option<&bool> {
        match self {
            Value::Bool(boolean) => Some(boolean),
            _ => None,
        }
    }
}

impl<Int: Integer> GetValue<Int> for Value<Int> {
    fn try_get(&self) -> Option<&Int> {
        match self {
            Value::Int(int) => Some(int),
            _ => None,
        }
    }
}
