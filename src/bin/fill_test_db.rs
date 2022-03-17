use anyhow::Result;
use db_schema::sea_orm;
use db_schema::{data::prelude::*, sea_orm::ActiveModelTrait};
use sea_orm::{ConnectionTrait, Database, Schema};

// use rand::{thread_rng, Rng};
// use db_schema::data::charge_record as charges;
use db_schema::data::*;
use quickcheck::{Arbitrary, Gen};

const SAMPLES_NUM: usize = 100;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db = db_schema::sea_orm::Database::connect(db_url)
        .await
        .expect("Database connection failed");

    println!("Generating mock data for db...");

    // let mut ids = vec![];
    for _ in 0..SAMPLES_NUM {
        let item = charge_record::Model::arbitrary(&mut Gen::new(20));
        let item: charge_record::ActiveModel = item.into();
        if item.insert(&db).await.is_err() {
            continue;
        };

        let item = finance_account::Model::arbitrary(&mut Gen::new(20));
        let item: finance_account::ActiveModel = item.into();
        if item.insert(&db).await.is_err() {
            continue;
        };

        let item = expense_report::Model::arbitrary(&mut Gen::new(20));
        let item: expense_report::ActiveModel = item.into();
        if item.insert(&db).await.is_err() {
            continue;
        };

        let item = finance_bill::Model::arbitrary(&mut Gen::new(20));
        let item: finance_bill::ActiveModel = item.into();
        if item.insert(&db).await.is_err() {
            continue;
        };

        let item = services::Model::arbitrary(&mut Gen::new(20));
        let item: services::ActiveModel = item.into();
        if item.insert(&db).await.is_err() {
            continue;
        };

        let item = orders::Model::arbitrary(&mut Gen::new(20));
        let item: orders::ActiveModel = item.into();
        if item.insert(&db).await.is_err() {
            continue;
        };

        let item = payment_record::Model::arbitrary(&mut Gen::new(20));
        let item: payment_record::ActiveModel = item.into();
        if item.insert(&db).await.is_err() {
            continue;
        };

        let item = resource_setmeal::Model::arbitrary(&mut Gen::new(20));
        let item: resource_setmeal::ActiveModel = item.into();
        if item.insert(&db).await.is_err() {
            continue;
        };

        let item = setemal_price::Model::arbitrary(&mut Gen::new(20));
        let item: setemal_price::ActiveModel = item.into();
        if item.insert(&db).await.is_err() {
            continue;
        };

        let item = setmeal_price::Model::arbitrary(&mut Gen::new(20));
        let item: setmeal_price::ActiveModel = item.into();
        if item.insert(&db).await.is_err() {
            continue;
        };

        let item = setmeal_price::Model::arbitrary(&mut Gen::new(20));
        let item: setmeal_price::ActiveModel = item.into();
        if item.insert(&db).await.is_err() {
            continue;
        };

        let item = products::Model::arbitrary(&mut Gen::new(20));
        let item: products::ActiveModel = item.into();
        if item.insert(&db).await.is_err() {
            continue;
        };
    }

    Ok(())
}
