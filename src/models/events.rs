use super::_entities::events::{ActiveModel, Entity};
use sea_orm::entity::prelude::*;
pub type Events = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
