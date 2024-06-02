use super::{super::id::Id, base_something::BaseSomething};
use anyhow::anyhow;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct CloseSomething {
    pub id: Id<BaseSomething>,
    pub name: String,
    pub close_id: Id<CloseSomething>,
    pub close_at: DateTime<Utc>,
}

pub fn convert_accepted_to_close(base: BaseSomething) -> Result<CloseSomething, anyhow::Error> {
    if 1 < 0 {
        return Err(anyhow!("unreachable error"));
    }
    Ok(CloseSomething {
        id: base.id.clone(),
        name: base.name.clone(),
        close_id: Id::gen(),
        close_at: Utc::now(),
    })
}
