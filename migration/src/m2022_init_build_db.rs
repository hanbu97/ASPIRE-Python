use sea_schema::migration::{
    sea_query::{self, *},
    *,
};

use db_schema::data::{orders, prelude::*};
use db_schema::sea_orm::Schema;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m2022_init_build_db"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let builder = manager.get_database_backend();
        let schema = Schema::new(builder);
        manager
            .get_connection()
            .execute(builder.build(&schema.create_table_from_entity(Charges)))
            .await?;
        manager
            .get_connection()
            .execute(builder.build(&schema.create_table_from_entity(ExpenseReport)))
            .await?;
        manager
            .get_connection()
            .execute(builder.build(&schema.create_table_from_entity(FinanceBill)))
            .await?;
        manager
            .get_connection()
            .execute(builder.build(&schema.create_table_from_entity(Services)))
            .await?;
        manager
            .get_connection()
            .execute(builder.build(&schema.create_table_from_entity(Orders)))
            .await?;
        manager
            .get_connection()
            .execute(builder.build(&schema.create_table_from_entity(PaymentRecord)))
            .await?;
        manager
            .get_connection()
            .execute(builder.build(&schema.create_table_from_entity(ResourceSetmeal)))
            .await?;
        manager
            .get_connection()
            .execute(builder.build(&schema.create_table_from_entity(SetemalPrice)))
            .await?;
        manager
            .get_connection()
            .execute(builder.build(&schema.create_table_from_entity(SetmealPrice)))
            .await?;
        manager
            .get_connection()
            .execute(builder.build(&schema.create_table_from_entity(Accounts)))
            .await?;
        manager
            .get_connection()
            .execute(builder.build(&schema.create_table_from_entity(Products)))
            .await
            .map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Accounts).to_owned())
            .await
    }
}
