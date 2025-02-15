use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    // Colonne id : clé primaire auto-incrémentée
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    // Colonne firstname : type Text, nullable
                    .col(ColumnDef::new(User::Firstname).text().null())
                    // Colonne lastname : type Text, nullable
                    .col(ColumnDef::new(User::Lastname).text())
                    // Colonne email : type Text, nullable, unique
                    .col(ColumnDef::new(User::Email).text().unique_key())
                    .col(ColumnDef::new(User::Password).text().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Firstname,
    Lastname,
    Email,
    Password,
}
