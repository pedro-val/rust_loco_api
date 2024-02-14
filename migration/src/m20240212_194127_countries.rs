use sea_orm_migration::{prelude::*, schema::*};
use sea_orm_migration::prelude::Iden;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Countries::Table)
                    .col(pk_auto(Countries::Id))
                    .col(string(Countries::Name))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Countries::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveMigrationName)]
pub enum Countries {
    Table,
    Id,
    Name,
}

impl Iden for Countries {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "{}", match self {
            Countries::Table => "countries",
            Countries::Id => "id",
            Countries::Name => "name",
        }).unwrap();
    }
}