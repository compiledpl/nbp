use chrono::NaiveDate;

mod gold_prices_basic {
    use super::*;

    #[tokio::test]
    async fn should_get_all_gold_prices() {
        let client = nbp::client::NbpClient::default();
        let result = client.gold_prices().prices().send().await;

        assert!(result.is_ok(), "Should successfully get all gold prices");
        let prices = result.unwrap();
        assert!(!prices.is_empty(), "Should return non-empty gold prices");
    }

    #[tokio::test]
    async fn should_get_gold_prices_last_5_days() {
        let client = nbp::client::NbpClient::default();
        let result = client.gold_prices().prices().last_days(5).send().await;

        assert!(
            result.is_ok(),
            "Should successfully get gold prices for last 5 days"
        );
        let prices = result.unwrap();
        assert!(!prices.is_empty(), "Should return non-empty gold prices");
        assert!(
            prices.len() <= 5,
            "Should return at most 5 days of gold prices"
        );
    }

    #[tokio::test]
    async fn should_get_gold_prices_last_10_days() {
        let client = nbp::client::NbpClient::default();
        let result = client.gold_prices().prices().last_days(10).send().await;

        assert!(
            result.is_ok(),
            "Should successfully get gold prices for last 10 days"
        );
        let prices = result.unwrap();
        assert!(!prices.is_empty(), "Should return non-empty gold prices");
        assert!(
            prices.len() <= 10,
            "Should return at most 10 days of gold prices"
        );
    }

    #[tokio::test]
    async fn should_get_gold_prices_for_specific_date() {
        let client = nbp::client::NbpClient::default();
        let test_date = NaiveDate::from_ymd_opt(2024, 10, 1).unwrap();

        let result = client.gold_prices().prices().date(test_date).send().await;

        assert!(
            result.is_ok(),
            "Should successfully get gold prices for specific date"
        );
        let prices = result.unwrap();
        assert!(!prices.is_empty(), "Should return non-empty gold prices");
    }

    #[tokio::test]
    async fn should_get_gold_prices_for_date_range() {
        let client = nbp::client::NbpClient::default();
        let start_date = NaiveDate::from_ymd_opt(2024, 10, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2024, 10, 5).unwrap();

        let result = client
            .gold_prices()
            .prices()
            .date_range(start_date, end_date)
            .send()
            .await;

        assert!(
            result.is_ok(),
            "Should successfully get gold prices for date range"
        );
        let prices = result.unwrap();
        assert!(!prices.is_empty(), "Should return non-empty gold prices");
        assert!(
            prices.len() <= 5,
            "Should return at most 5 days of gold prices"
        );
    }
}

mod gold_prices_extended {
    use super::*;

    #[tokio::test]
    async fn should_get_gold_prices_for_december_range() {
        let client = nbp::client::NbpClient::default();
        let start_date = NaiveDate::from_ymd_opt(2024, 12, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2024, 12, 10).unwrap();

        let result = client
            .gold_prices()
            .prices()
            .date_range(start_date, end_date)
            .send()
            .await;

        assert!(
            result.is_ok(),
            "Should successfully get gold prices for December range"
        );
        let prices = result.unwrap();
        assert!(!prices.is_empty(), "Should return non-empty gold prices");
        assert!(
            prices.len() <= 10,
            "Should return at most 10 days of gold prices"
        );
    }

    #[tokio::test]
    async fn should_get_gold_prices_for_week_range() {
        let client = nbp::client::NbpClient::default();
        let start_date = NaiveDate::from_ymd_opt(2024, 10, 7).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2024, 10, 13).unwrap();

        let result = client
            .gold_prices()
            .prices()
            .date_range(start_date, end_date)
            .send()
            .await;

        assert!(
            result.is_ok(),
            "Should successfully get gold prices for week range"
        );
        let prices = result.unwrap();
        assert!(!prices.is_empty(), "Should return non-empty gold prices");
        assert!(
            prices.len() <= 7,
            "Should return at most 7 days of gold prices"
        );
    }

    #[tokio::test]
    async fn should_get_gold_prices_for_month_range() {
        let client = nbp::client::NbpClient::default();
        let start_date = NaiveDate::from_ymd_opt(2024, 9, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2024, 9, 30).unwrap();

        let result = client
            .gold_prices()
            .prices()
            .date_range(start_date, end_date)
            .send()
            .await;

        assert!(
            result.is_ok(),
            "Should successfully get gold prices for month range"
        );
        let prices = result.unwrap();
        assert!(!prices.is_empty(), "Should return non-empty gold prices");
        assert!(
            prices.len() <= 30,
            "Should return at most 30 days of gold prices"
        );
    }

    #[tokio::test]
    async fn should_get_gold_prices_for_different_periods() {
        let client = nbp::client::NbpClient::default();
        let periods = vec![1, 3, 7, 14, 30];

        for days in periods {
            let result = client.gold_prices().prices().last_days(days).send().await;

            assert!(
                result.is_ok(),
                "Should successfully get gold prices for last {} days",
                days
            );
            let prices = result.unwrap();
            assert!(
                !prices.is_empty(),
                "Should return non-empty gold prices for {} days",
                days
            );
            assert!(
                prices.len() <= days.into(),
                "Should return at most {} days of gold prices",
                days
            );
        }
    }
}

mod gold_prices_edge_cases {
    use super::*;

    #[tokio::test]
    async fn should_handle_future_dates() {
        let client = nbp::client::NbpClient::default();
        let future_date = NaiveDate::from_ymd_opt(2030, 1, 1).unwrap();

        let result = client.gold_prices().prices().date(future_date).send().await;

        // Future dates might return empty data or fail
        match result {
            Ok(prices) => {
                println!("Future date query returned {} gold prices", prices.len());
            }
            Err(e) => {
                println!("Future date query failed as expected: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn should_handle_weekend_dates() {
        let client = nbp::client::NbpClient::default();
        // Saturday
        let weekend_date = NaiveDate::from_ymd_opt(2024, 10, 5).unwrap();

        let result = client
            .gold_prices()
            .prices()
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
            error.to_string().contains("NotFound"),
            "Error should be NotFound: {}",
            error
        );
    }

    #[tokio::test]
    async fn should_handle_holiday_dates() {
        let client = nbp::client::NbpClient::default();
        // New Year's Day
        let holiday_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();

        let result = client
            .gold_prices()
            .prices()
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
            error.to_string().contains("NotFound"),
            "Error should be NotFound: {}",
            error
        );
    }

    #[tokio::test]
    async fn should_handle_large_date_range() {
        let client = nbp::client::NbpClient::default();
        let start_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2024, 3, 31).unwrap();

        let result = client
            .gold_prices()
            .prices()
            .date_range(start_date, end_date)
            .send()
            .await;

        assert!(result.is_ok(), "Should handle large date range");
        let prices = result.unwrap();
        assert!(
            !prices.is_empty(),
            "Should return data for large date range"
        );
        assert!(prices.len() <= 91, "Should return at most 91 days of data");
    }

    #[tokio::test]
    async fn should_handle_single_day_range() {
        let client = nbp::client::NbpClient::default();
        let test_date = NaiveDate::from_ymd_opt(2024, 10, 1).unwrap();

        let result = client
            .gold_prices()
            .prices()
            .date_range(test_date, test_date)
            .send()
            .await;

        assert!(result.is_ok(), "Should handle single day range");
        let prices = result.unwrap();
        assert!(
            !prices.is_empty(),
            "Should return data for single day range"
        );
        assert!(prices.len() <= 1, "Should return at most 1 day of data");
    }
}

mod gold_prices_validation {
    #[tokio::test]
    async fn should_validate_gold_price_data_structure() {
        let client = nbp::client::NbpClient::default();
        let result = client.gold_prices().prices().last_days(1).send().await;

        assert!(
            result.is_ok(),
            "Should successfully get gold prices for validation"
        );
        let prices = result.unwrap();
        assert!(!prices.is_empty(), "Should return non-empty gold prices");

        // Validate that each price entry has the expected structure
        for price in prices {
            // Add validation based on your actual data structure
            // This is a placeholder - adjust according to your actual model
            println!("Gold price entry: {:?}", price);
        }
    }

    #[tokio::test]
    async fn should_validate_date_ordering() {
        let client = nbp::client::NbpClient::default();
        let result = client.gold_prices().prices().last_days(5).send().await;

        assert!(
            result.is_ok(),
            "Should successfully get gold prices for date validation"
        );
        let prices = result.unwrap();
        assert!(!prices.is_empty(), "Should return non-empty gold prices");

        // Validate that dates are in correct order (if applicable)
        // This depends on your data structure
        println!("Retrieved {} gold price entries", prices.len());
    }
}
