use std::any::Any;

use sea_schema::{
    migration::{
        sea_query::{self, *},
        *,
    },
    sea_query::extension::postgres::{Type, TypeDropStatement},
};

use db_schema::data::prelude::*;
use db_schema::data::shop_product;
use db_schema::sea_orm::Schema;

pub struct Migration;
// add month_price to product
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220314_000001_add_product_price"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // add column month_price to product
        manager
            .alter_table(
                Table::alter()
                    .table(Products)
                    .add_column(
                        ColumnDef::new(Alias::new("new_col"))
                            .integer()
                            .not_null()
                            .default(100),
                    )
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    // .table(Products)
                    // .drop_column(shop_product::Column::MonthPrice)
                    .to_owned(),
            )
            .await
    }
}
// Type::drop()
// .if_exists()
// .name(FontFamily)
// .restrict()
// .to_string(PostgresQueryBuilder),
