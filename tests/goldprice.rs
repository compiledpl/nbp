use nbp::openapi::apis::{configuration::Configuration, default_api as api};

#[tokio::test]
async fn test_basic() {
    let config = Configuration::new();
    dbg!(api::cenyzlota_get(&config).await.unwrap());
}

#[tokio::test]
async fn test_goldprice_date() {
    let config = Configuration::new();
    dbg!(
        api::cenyzlota_date_get(&config, "2025-06-10")
            .await
            .unwrap()
    );
}
