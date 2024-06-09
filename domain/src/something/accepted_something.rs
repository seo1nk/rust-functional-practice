use super::{super::id::Id, base_something::BaseSomething, open_something::OpenSomething};
use anyhow::anyhow;

#[derive(Debug)]
pub struct AcceptedSomething {
    pub id: Id<BaseSomething>,
    pub name: String,
    pub accepted_id: Id<AcceptedSomething>,
    pub accepted_reason_id: i32,
}

pub fn convert_open_to_accepted(
    open: OpenSomething,
    accepted_reason_id: i32,
) -> Result<AcceptedSomething, anyhow::Error> {
    if accepted_reason_id.is_negative() {
        return Err(anyhow!("error"));
    }
    Ok(AcceptedSomething {
        id: open.id,
        name: open.name,
        accepted_id: Id::gen(),
        accepted_reason_id,
    })
}
