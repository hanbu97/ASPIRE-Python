use anyhow::Result;
use db_schema::data::prelude::*;
use db_schema::sea_orm;
use sea_orm::{ConnectionTrait, Database, Schema};

#[tokio::main]
async fn main() -> Result<()> {
    // Read the database environment from the `.env` file
    let env_database_url = include_str!("../../.env").trim();
    // Split the env url
    let split_url: Vec<&str> = env_database_url.split("=").collect();
    // Get item with the format `database_backend://username:password@localhost/database`
    let database_url = split_url[1];

    let db = Database::connect(database_url).await?;

    let builder = db.get_database_backend();
    let schema = Schema::new(builder);

    db.execute(builder.build(&schema.create_table_from_entity(Charges)))
        .await?;
    db.execute(builder.build(&schema.create_table_from_entity(ExpenseReport)))
        .await?;
    db.execute(builder.build(&schema.create_table_from_entity(FinanceBill)))
        .await?;
    db.execute(builder.build(&schema.create_table_from_entity(Services)))
        .await?;
    db.execute(builder.build(&schema.create_table_from_entity(Orders)))
        .await?;
    db.execute(builder.build(&schema.create_table_from_entity(PaymentRecord)))
        .await?;
    db.execute(builder.build(&schema.create_table_from_entity(SetmealPrice)))
        .await?;
    db.execute(builder.build(&schema.create_table_from_entity(SetemalPrice)))
        .await?;
    db.execute(builder.build(&schema.create_table_from_entity(Products)))
        .await?;
    db.execute(builder.build(&schema.create_table_from_entity(Accounts)))
        .await?;

    // let create_table_op = db
    //     .execute(builder.build(&schema.create_table_from_entity(Fruits)))
    //     .await;
    // println!(
    //     "`CREATE TABLE fruits` {:?}",
    //     match create_table_op {
    //         Ok(_) => "Operation Successful".to_owned(),
    //         Err(e) => format!("Unsuccessful - Error {:?}", e),
    //     }
    // );
    Ok(())
}
