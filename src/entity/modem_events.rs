//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

use jsonapi::{
    jsonapi_model,
    model::{JsonApiModel, Relationships, Resources},
};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "modem_events")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub description: Option<String>,
    pub priority: Option<i32>,
    pub created_at: Option<DateTime>,
    pub maintenance: bool,
}

jsonapi_model!(Model; "modemEvent");

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
