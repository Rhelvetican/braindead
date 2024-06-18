use std::{
    fmt::{Display, Formatter, Result},
    ops::{AddAssign, SubAssign},
};

#[derive(Debug)]
pub struct Cell<T> {
    value: T,
}

impl<T: Default> Default for Cell<T> {
    fn default() -> Self {
        Cell {
            value: Default::default(),
        }
    }
}

impl<T: Display> Display for Cell<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value)
    }
}

impl<T> Cell<T> {
    pub fn set_value(&mut self, value: T) {
        self.value = value;
    }
}

impl<T> Cell<T>
where
    T: AddAssign + SubAssign + From<u8>,
{
    pub fn inc(&mut self) {
        self.value += T::from(1);
    }
    pub fn dec(&mut self) {
        self.value -= T::from(1);
    }
}

#[allow(dead_code)]
impl Cell<u8> {
    pub fn to_char(&self) -> char {
        self.value as char
    }
}

impl Cell<u32> {
    pub fn to_char(&self) -> char {
        if let Some(c) = char::from_u32(self.value) {
            c
        } else {
            '\0'
        }
    }
}
