use nbp::openapi::{
    models::{
        TableType
    },
    apis::{
        default_api as api,
        configuration::Configuration
    }
};

#[tokio::test]
async fn test_basic() {
    let config = Configuration::new();
    dbg!(api::exchangerates_tables_table_get(&config, TableType::A).await.unwrap());

}