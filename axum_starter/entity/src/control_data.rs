//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.4

use super::sea_orm_active_enums::Type;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "ControlData")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub dancer_id: i32,
    pub part_id: i32,
    pub frame_id: i32,
    pub r#type: Type,
    pub color_id: Option<i32>,
    pub effect_id: Option<i32>,
    pub alpha: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::color::Entity",
        from = "Column::ColorId",
        to = "super::color::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Color,
    #[sea_orm(
        belongs_to = "super::control_frame::Entity",
        from = "Column::FrameId",
        to = "super::control_frame::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    ControlFrame,
    #[sea_orm(
        belongs_to = "super::dancer::Entity",
        from = "Column::DancerId",
        to = "super::dancer::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Dancer,
    #[sea_orm(has_many = "super::led_bulb::Entity")]
    LedBulb,
    #[sea_orm(
        belongs_to = "super::led_effect::Entity",
        from = "Column::EffectId",
        to = "super::led_effect::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    LedEffect,
    #[sea_orm(
        belongs_to = "super::part::Entity",
        from = "Column::PartId",
        to = "super::part::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Part,
}

impl Related<super::color::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Color.def()
    }
}

impl Related<super::control_frame::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ControlFrame.def()
    }
}

impl Related<super::dancer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Dancer.def()
    }
}

impl Related<super::led_bulb::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LedBulb.def()
    }
}

impl Related<super::led_effect::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LedEffect.def()
    }
}

impl Related<super::part::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Part.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    #[sea_orm(entity = "super::color::Entity")]
    Color,
    #[sea_orm(entity = "super::control_frame::Entity")]
    ControlFrame,
    #[sea_orm(entity = "super::dancer::Entity")]
    Dancer,
    #[sea_orm(entity = "super::led_bulb::Entity")]
    LedBulb,
    #[sea_orm(entity = "super::led_effect::Entity")]
    LedEffect,
    #[sea_orm(entity = "super::part::Entity")]
    Part,
}
