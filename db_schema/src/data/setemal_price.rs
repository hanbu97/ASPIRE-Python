//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "setemal_price")]
pub struct Model {
    pub r#type: String,
    pub cpu_core: i32,
    pub mem_size: i32,
    pub gpu_core: i32,
    pub storage: Option<i32>,
    pub price_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub is_deleted: bool,
    pub version: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}

use quickcheck::{Arbitrary, Gen};
impl Arbitrary for Model {
    fn arbitrary(g: &mut Gen) -> Model {
        let test_range = (1..100000).into_iter().collect::<Vec<u32>>();
        let status = vec!["pending", "success", "failed"];
        let names = vec![
            "service_name1",
            "product_name2",
            "service_name3",
            "product_name4",
            "service_name5",
            "product_name7",
            "service_name6",
            "product_name8",
        ];

        Model {
            is_deleted: bool::arbitrary(g),
            version: g.choose(&test_range).unwrap().to_owned() as i32,
            r#type: g.choose(&status).unwrap().to_string(),
            cpu_core: g.choose(&test_range).unwrap().to_owned() as i32,
            mem_size: g.choose(&test_range).unwrap().to_owned() as i32,
            gpu_core: g.choose(&test_range).unwrap().to_owned() as i32,
            storage: Some(g.choose(&test_range).unwrap().to_owned() as i32),
            price_id: g.choose(&test_range).unwrap().to_owned() as i64,
            id: g.choose(&test_range).unwrap().to_owned() as i64,
            created_at: chrono::NaiveDateTime::from_timestamp(
                g.choose(&test_range).unwrap().to_owned() as i64,
                0,
            ),
            updated_at: chrono::NaiveDateTime::from_timestamp(
                g.choose(&test_range).unwrap().to_owned() as i64,
                0,
            ),
        }
    }
}