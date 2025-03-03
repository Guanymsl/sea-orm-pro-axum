//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "EditingPositionFrame")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: i32,
    #[sea_orm(unique)]
    pub frame_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::position_frame::Entity",
        from = "Column::FrameId",
        to = "super::position_frame::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    PositionFrame,
}

impl Related<super::position_frame::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PositionFrame.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    #[sea_orm(entity = "super::position_frame::Entity")]
    PositionFrame,
}
