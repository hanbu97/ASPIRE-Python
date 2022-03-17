pub use sea_schema::migration::*;

mod m2022_init_build_db;
// mod m20220313_000001_create_table;
// mod m20220314_000001_add_product_price;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m2022_init_build_db::Migration),
            // Box::new(m20220313_000001_create_table::Migration),
            // Box::new(m20220314_000001_add_product_price::Migration),
        ]
    }
}
