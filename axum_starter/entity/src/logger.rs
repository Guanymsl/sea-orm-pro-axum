//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "Logger")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user: i32,
    pub variable_value: Option<Json>,
    pub field_name: String,
    pub time: DateTimeUtc,
    pub status: String,
    pub error_message: Option<Json>,
    pub result: Option<Json>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {}
