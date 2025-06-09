use nbp::openapi::apis::{
    default_api as api,
    configuration::Configuration
};

#[tokio::test]
async fn test_basic() {
    let config = Configuration::new();
    dbg!(api::cenyzlota_get(&config).await.unwrap());

}