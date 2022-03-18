//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "outtages")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub loss: f32,
    pub downtime: String,
    pub created_at: Option<DateTime>,
    pub maintenance: i8,
    #[sea_orm(column_type = "Text")]
    pub info: Option<Value>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}