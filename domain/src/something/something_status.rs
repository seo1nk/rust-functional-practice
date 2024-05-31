use super::{base_something::BaseSomething, open_something::OpenSomething};

pub enum Something {
    Base(BaseSomething),
    Open(OpenSomething),
    // Accepted,
    // Rejected,
}
