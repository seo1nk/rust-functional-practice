use super::super::id::Id;

#[derive(Debug, Clone)]
pub struct BaseSomething {
    pub id: Id<BaseSomething>,
    pub name: String,
}

impl BaseSomething {
    pub fn new(name: &str) -> Self {
        Self {
            id: Id::gen(),
            name: name.to_string(),
        }
    }
}
