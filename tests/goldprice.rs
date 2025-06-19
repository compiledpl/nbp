use nbp::openapi::apis::{configuration::Configuration, default_api as api};

#[tokio::test]
async fn test_basic() {
    let config = Configuration::new();
    dbg!(api::cenyzlota_get(&config).await.unwrap());
}

#[tokio::test]
async fn test_goldprice_last_top_count() {
    let config = Configuration::new();
    dbg!(
        api::cenyzlota_last_top_count_get(&config, 10)
            .await
            .unwrap()
    );
}
