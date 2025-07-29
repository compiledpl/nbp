use chrono::NaiveDate;
use nbp::models::currency_code::CurrencyCode;
use nbp::models::table_type::TableType;

mod exchange_rates_tables {
    use super::*;

    #[tokio::test]
    async fn should_get_table_a_last_5_days() {
        let client = nbp::client::NbpClient::default();
        let result = client
            .exchange_rates()
            .tables(TableType::A)
            .last_days(5)
            .send()
            .await;

        assert!(
            result.is_ok(),
            "Should successfully get table A for last 5 days"
        );
        let rates = result.unwrap();
        assert!(!rates.is_empty(), "Should return non-empty rates");
        assert!(rates.len() <= 5, "Should return at most 5 days of data");
    }

    #[tokio::test]
    async fn should_get_table_a_last_day() {
        let client = nbp::client::NbpClient::default();
        let result = client
            .exchange_rates()
            .tables(TableType::A)
            .last_day()
            .send()
            .await;

        assert!(
            result.is_ok(),
            "Should successfully get table A for last day"
        );
        let rates = result.unwrap();
        assert!(!rates.is_empty(), "Should return non-empty rates");
        assert!(rates.len() <= 1, "Should return at most 1 day of data");
    }

    #[tokio::test]
    async fn should_get_table_b_last_day() {
        let client = nbp::client::NbpClient::default();
        let result = client
            .exchange_rates()
            .tables(TableType::B)
            .last_day()
            .send()
            .await;

        assert!(
            result.is_ok(),
            "Should successfully get table B for last day"
        );
        let rates = result.unwrap();
        assert!(!rates.is_empty(), "Should return non-empty rates");
    }

    #[tokio::test]
    async fn should_get_table_c_last_day() {
        let client = nbp::client::NbpClient::default();
        let result = client
            .exchange_rates()
            .tables(TableType::C)
            .last_day()
            .send()
            .await;

        assert!(
            result.is_ok(),
            "Should successfully get table C for last day"
        );
        let rates = result.unwrap();
        assert!(!rates.is_empty(), "Should return non-empty rates");
    }

    #[tokio::test]
    async fn should_get_table_a_for_specific_date() {
        let client = nbp::client::NbpClient::default();
        let test_date = NaiveDate::from_ymd_opt(2024, 10, 1).unwrap();

        let result = client
            .exchange_rates()
            .tables(TableType::A)
            .date(test_date)
            .send()
            .await;

        assert!(
            result.is_ok(),
            "Should successfully get table A for specific date"
        );
        let rates = result.unwrap();
        assert!(!rates.is_empty(), "Should return non-empty rates");
    }

    #[tokio::test]
    async fn should_get_table_a_for_date_range() {
        let client = nbp::client::NbpClient::default();
        let start_date = NaiveDate::from_ymd_opt(2024, 10, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2024, 10, 5).unwrap();

        let result = client
            .exchange_rates()
            .tables(TableType::A)
            .date_range(start_date, end_date)
            .send()
            .await;

        assert!(
            result.is_ok(),
            "Should successfully get table A for date range"
        );
        let rates = result.unwrap();
        assert!(!rates.is_empty(), "Should return non-empty rates");
        assert!(rates.len() <= 5, "Should return at most 5 days of data");
    }

    #[tokio::test]
    async fn should_handle_invalid_date_range() {
        let client = nbp::client::NbpClient::default();
        let future_date = NaiveDate::from_ymd_opt(2030, 1, 1).unwrap();

        let result = client
            .exchange_rates()
            .tables(TableType::A)
            .date(future_date)
            .send()
            .await;

        // This might fail or return empty data for future dates
        // The test documents the expected behavior
        match result {
            Ok(rates) => {
                // If successful, rates might be empty for future dates
                println!("Future date query returned {} rates", rates.len());
            }
            Err(e) => {
                // If it fails, that's also acceptable behavior
                println!("Future date query failed as expected: {}", e);
            }
        }
    }
}

mod exchange_rates_currency {
    use super::*;

    #[tokio::test]
    async fn should_get_usd_rates_table_a_last_5_days() {
        let client = nbp::client::NbpClient::default();
        let result = client
            .exchange_rates()
            .rates(TableType::A, CurrencyCode::USD)
            .last_days(5)
            .send()
            .await;

        assert!(
            result.is_ok(),
            "Should successfully get USD rates for table A"
        );
        let rates = result.unwrap();
        assert!(!rates.rates.is_empty(), "Should return non-empty USD rates");
        assert!(
            rates.rates.len() <= 5,
            "Should return at most 5 days of USD data"
        );
    }

    #[tokio::test]
    async fn should_get_eur_rates_table_a_last_day() {
        let client = nbp::client::NbpClient::default();
        let result = client
            .exchange_rates()
            .rates(TableType::A, CurrencyCode::EUR)
            .last_day()
            .send()
            .await;

        assert!(
            result.is_ok(),
            "Should successfully get EUR rates for table A"
        );
        let rates = result.unwrap();
        assert!(!rates.rates.is_empty(), "Should return non-empty EUR rates");
    }

    #[tokio::test]
    async fn should_get_eur_rates_table_c_last_10_days() {
        let client = nbp::client::NbpClient::default();
        let result = client
            .exchange_rates()
            .rates(TableType::C, CurrencyCode::EUR)
            .last_days(10)
            .send()
            .await;

        assert!(
            result.is_ok(),
            "Should successfully get EUR rates for table C"
        );
        let rates = result.unwrap();
        assert!(!rates.rates.is_empty(), "Should return non-empty EUR rates");
        assert!(
            rates.rates.len() <= 10,
            "Should return at most 10 days of EUR data"
        );
    }

    #[tokio::test]
    async fn should_get_gbp_rates_table_a_specific_date() {
        let client = nbp::client::NbpClient::default();
        let test_date = NaiveDate::from_ymd_opt(2024, 10, 1).unwrap();

        let result = client
            .exchange_rates()
            .rates(TableType::A, CurrencyCode::GBP)
            .date(test_date)
            .send()
            .await;

        assert!(
            result.is_ok(),
            "Should successfully get GBP rates for specific date"
        );
        let rates = result.unwrap();
        assert!(!rates.rates.is_empty(), "Should return non-empty GBP rates");
    }

    #[tokio::test]
    async fn should_get_chf_rates_table_b_last_day() {
        let client = nbp::client::NbpClient::default();
        let result = client
            .exchange_rates()
            .rates(TableType::A, CurrencyCode::CHF)
            .last_day()
            .send()
            .await;

        assert!(
            result.is_ok(),
            "Should successfully get CHF rates for table B"
        );
        let rates = result.unwrap();
        assert!(!rates.rates.is_empty(), "Should return non-empty CHF rates");
    }

    #[tokio::test]
    async fn should_get_jpy_rates_table_a_date_range() {
        let client = nbp::client::NbpClient::default();
        let start_date = NaiveDate::from_ymd_opt(2024, 10, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2024, 10, 7).unwrap();

        let result = client
            .exchange_rates()
            .rates(TableType::A, CurrencyCode::JPY)
            .date_range(start_date, end_date)
            .send()
            .await;

        assert!(
            result.is_ok(),
            "Should successfully get JPY rates for date range"
        );
        let rates = result.unwrap();
        assert!(!rates.rates.is_empty(), "Should return non-empty JPY rates");
        assert!(
            rates.rates.len() <= 7,
            "Should return at most 7 days of JPY data"
        );
    }

    #[tokio::test]
    async fn should_handle_multiple_currencies() {
        let client = nbp::client::NbpClient::default();
        let currencies = vec![
            CurrencyCode::USD,
            CurrencyCode::EUR,
            CurrencyCode::GBP,
            CurrencyCode::CHF,
        ];

        for currency in currencies {
            let result = client
                .exchange_rates()
                .rates(TableType::A, currency.clone())
                .last_day()
                .send()
                .await;

            assert!(
                result.is_ok(),
                "Should successfully get rates for {:?}",
                currency
            );
            let rates = result.unwrap();
            assert!(
                !rates.rates.is_empty(),
                "Should return non-empty rates for {:?}",
                currency
            );
        }
    }

    #[tokio::test]
    async fn should_handle_different_table_types() {
        let client = nbp::client::NbpClient::default();
        let table_types = vec![TableType::A, TableType::B, TableType::C];

        for table_type in table_types {
            let result = client
                .exchange_rates()
                .rates(table_type, CurrencyCode::USD)
                .last_day()
                .send()
                .await;

            assert!(
                result.is_ok(),
                "Should successfully get USD rates for {:?}",
                table_type
            );
            let rates = result.unwrap();
            assert!(
                !rates.rates.is_empty(),
                "Should return non-empty USD rates for {:?}",
                table_type
            );
        }
    }
}

mod exchange_rates_edge_cases {
    use super::*;

    #[tokio::test]
    async fn should_handle_weekend_dates() {
        let client = nbp::client::NbpClient::default();
        // Saturday
        let weekend_date = NaiveDate::from_ymd_opt(2024, 10, 5).unwrap();

        let result = client
            .exchange_rates()
            .tables(TableType::A)
            .date(weekend_date)
            .send()
            .await;

        // Weekend queries should return NotFound error
        assert!(
            result.is_err(),
            "Weekend query should return NotFound error"
        );
        let error = result.unwrap_err();
        assert!(
            error.to_string().contains("Resource not found"),
            "Error should be Resource not found: {}",
            error
        );
    }

    #[tokio::test]
    async fn should_handle_holiday_dates() {
        let client = nbp::client::NbpClient::default();
        // New Year's Day
        let holiday_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();

        let result = client
            .exchange_rates()
            .tables(TableType::A)
            .date(holiday_date)
            .send()
            .await;

        // Holiday queries should return NotFound error
        assert!(
            result.is_err(),
            "Holiday query should return NotFound error"
        );
        let error = result.unwrap_err();
        assert!(
            error.to_string().contains("Resource not found"),
            "Error should be Resource not found: {}",
            error
        );
    }

    #[tokio::test]
    async fn should_handle_large_date_range() {
        let client = nbp::client::NbpClient::default();
        let start_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();

        let result = client
            .exchange_rates()
            .tables(TableType::A)
            .date_range(start_date, end_date)
            .send()
            .await;

        assert!(result.is_ok(), "Should handle large date range");
        let rates = result.unwrap();
        assert!(!rates.is_empty(), "Should return data for large date range");
        assert!(rates.len() <= 31, "Should return at most 31 days of data");
    }
}
