use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,

    #[sea_orm(column_type = "Text", nullable)]
    pub firstname: String,

    #[sea_orm(column_type = "Text", nullable)]
    pub lastname: String,

    #[sea_orm(column_type = "Text", nullable, unique)]
    pub email: String,

    #[sea_orm(column_type = "Text", nullable)]
    pub password: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No relations")
    }
}

impl ActiveModelBehavior for ActiveModel {}
