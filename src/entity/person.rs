use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    id: u64,
    first_name: String,
    last_name: String,
    age: u32,
}

impl Default for Person {
    fn default() -> Self {
        Self {
            id: 1001,
            first_name: String::from("Raphael"),
            last_name: String::from("Gray"),
            age: 96,
        }
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id:{}, first_name:{}, last_name:{}, age:{}",
            self.id(),
            self.first_name(),
            self.last_name(),
            self.age()
        )
    }
}

impl Person {
    #[must_use]
    pub fn new(
        id: u64,
        first_name: impl Into<String>,
        last_name: impl Into<String>,
        age: u32,
    ) -> Self {
        Self {
            id,
            first_name: first_name.into(),
            last_name: last_name.into(),
            age,
        }
    }

    pub const fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    pub fn set_first_name(&mut self, first_name: impl Into<String>) {
        self.first_name = first_name.into();
    }

    pub fn set_last_name(&mut self, last_name: impl Into<String>) {
        self.last_name = last_name.into();
    }

    pub const fn set_age(&mut self, age: u32) {
        self.age = age;
    }

    #[must_use]
    pub const fn id(&self) -> u64 {
        self.id
    }

    #[must_use]
    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    #[must_use]
    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    #[must_use]
    pub const fn age(&self) -> u32 {
        self.age
    }
}
