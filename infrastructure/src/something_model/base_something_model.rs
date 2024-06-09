use domain::{id::Id, something::base_something::BaseSomething};

pub struct BaseSomethingModel {
    pub id: String,
    pub name: String,
}

impl TryFrom<BaseSomethingModel> for BaseSomething {
    type Error = anyhow::Error;

    fn try_from(model: BaseSomethingModel) -> Result<Self, Self::Error> {
        Ok(BaseSomething {
            id: Id::try_from(model.id)?,
            name: model.name,
        })
    }
}
