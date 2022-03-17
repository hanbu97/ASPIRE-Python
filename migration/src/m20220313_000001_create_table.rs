use sea_schema::migration::{
    sea_query::{self, *},
    *,
};

use db_schema::data::prelude::*;
use db_schema::sea_orm::Schema;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let builder = manager.get_database_backend();
        let schema = Schema::new(builder);
        manager
            .get_connection()
            .execute(builder.build(&schema.create_table_from_entity(Accounts)))
            .await
            .map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Accounts).to_owned())
            .await
    }
}
