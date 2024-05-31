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

#[test]
fn test_new() {
    let something_name = "test";
    let base = BaseSomething::new(something_name);
    assert_eq!(base.name, something_name.to_string())
}
