use chrono::NaiveDate;
use nbp::models::table_type::TableType;

#[tokio::test]
async fn exchange_rates_get_table() {
    let client = nbp::client::NbpClient::default(); //TODO nbp api rate limit is 60 per second - to verify
    dbg!(
        client
            .exchange_rates()
            .tables(TableType::A)
            .last_days(5)
            .send()
            .await
            .unwrap()
    );

    dbg!(
        client
            .exchange_rates()
            .tables(TableType::A)
            .last_day()
            .send()
            .await
            .unwrap()
    );

    dbg!(
        client
            .exchange_rates()
            .tables(TableType::A)
            .date(NaiveDate::from_ymd_opt(2024, 10, 1).unwrap())
            .send()
            .await
            .unwrap()
    );

    dbg!(
        client
            .exchange_rates()
            .tables(TableType::A)
            .date_range(
                NaiveDate::from_ymd_opt(2024, 10, 1).unwrap(),
                NaiveDate::from_ymd_opt(2024, 10, 5).unwrap()
            )
            .send()
            .await
            .unwrap()
    );
    //
    // dbg!(
    //     client
    //         .exchange_rates()
    //         .currency(CurrencyCode::PLN)
    //         .date_range(
    //             NaiveDate::from_ymd_opt(2024, 10, 1).unwrap(),
    //             NaiveDate::from_ymd_opt(2024, 10, 5).unwrap()
    //         )
    //         .send()
    //         .await
    //         .unwrap()
    // );
    //
    // dbg!(
    //     client
    //         .gold_prices()
    //         .date_range(
    //             NaiveDate::from_ymd_opt(2024, 10, 1).unwrap(),
    //             NaiveDate::from_ymd_opt(2024, 10, 5).unwrap()
    //         )
    //         .send()
    //         .await
    //         .unwrap()
    // );
}
