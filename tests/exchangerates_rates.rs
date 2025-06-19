use nbp::openapi::{
    apis::{configuration::Configuration, default_api as api},
    models::TableType,
};

#[tokio::test]
async fn test_exchangerates_tables_table_get() {
    dbg!(
        api::exchangerates_tables_table_get(&Configuration::new(), TableType::A)
            .await
            .unwrap()
    );
}

#[tokio::test]
async fn test_exchangereates_tables_table_last_top_count_get() {
    for top_count in [1, 3, 10, 67] {
        dbg!(
            api::exchangerates_tables_table_last_top_count_get(
                &Configuration::new(),
                TableType::A,
                top_count
            )
            .await
            .expect("Querying last {topCount} exchange rates tables failed.")
        );
    }
}

#[tokio::test]
async fn test_exchangerates_rates_table_code_start_date_end_date_get() {
    dbg!(
        api::exchangerates_rates_table_code_start_date_end_date_get(
            &Configuration::new(),
            TableType::A,
            "USD",
            "2025-06-10",
            "2025-06-19"
        )
        .await
        .unwrap()
    );
}
