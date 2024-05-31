use super::{super::id::Id, base_something::BaseSomething};

#[derive(Debug)]
pub struct OpenSomething {
    pub id: Id<BaseSomething>,
    pub name: String,
    pub open_id: Id<OpenSomething>,
    pub open_reason: String,
}

impl OpenSomething {
    pub fn new(base: BaseSomething, open_reason: &str) -> Self {
        Self {
            id: base.id,
            name: base.name,
            open_id: Id::gen(),
            open_reason: open_reason.to_string(),
        }
    }
}

// TODO: なんか変な気がする
impl TryFrom<BaseSomething> for OpenSomething {
    type Error = anyhow::Error;

    fn try_from(base: BaseSomething) -> Result<Self, Self::Error> {
        Ok(OpenSomething {
            id: base.id,
            name: base.name,
            open_id: Id::gen(),
            open_reason: "try_from".to_string(),
        })
    }
}

pub fn convert_base_to_open(
    base: BaseSomething,
    open_reason: &str,
) -> Result<OpenSomething, anyhow::Error> {
    Ok(OpenSomething {
        id: base.id.clone(),
        name: base.name.clone(),
        open_id: Id::gen(),
        open_reason: open_reason.to_string(),
    })
}
