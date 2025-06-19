use nbp::openapi::apis::{configuration::Configuration, default_api as api};

#[tokio::test]
async fn test_basic() {
    let config = Configuration::new();
    dbg!(api::cenyzlota_get(&config).await.unwrap());
}

#[tokio::test]
async fn test_goldprice_date_range() {
    let config = Configuration::new();
    dbg!(
        api::cenyzlota_start_date_end_date_get(&config, "2025-06-10", "2025-06-19")
            .await
            .unwrap()
    );
}
