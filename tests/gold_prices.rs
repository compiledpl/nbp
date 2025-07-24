use chrono::NaiveDate;

#[tokio::test]
async fn get_gold_prices() {
    let client = nbp::client::NbpClient::default();
    dbg!(client.gold_prices().prices().send().await.unwrap());

    // dbg!(
    //     client
    //         .gold_prices()
    //         .prices()
    //         .today()
    //         .send()
    //         .await
    //         .unwrap()
    // );

    dbg!(
        client
            .gold_prices()
            .prices()
            .last_days(5)
            .send()
            .await
            .unwrap()
    );

    dbg!(
        client
            .gold_prices()
            .prices()
            .date(NaiveDate::from_ymd_opt(2024, 10, 1).unwrap())
            .send()
            .await
            .unwrap()
    );

    let client = nbp::client::NbpClient::default();
    dbg!(
        client
            .gold_prices()
            .prices()
            .date_range(
                NaiveDate::from_ymd_opt(2024, 10, 1).unwrap(),
                NaiveDate::from_ymd_opt(2024, 10, 5).unwrap()
            )
            .send()
            .await
            .unwrap()
    );

    dbg!(
        client
            .gold_prices()
            .prices()
            .last_days(10)
            .send()
            .await
            .unwrap()
    );

    dbg!(
        client
            .gold_prices()
            .prices()
            .date_range(
                NaiveDate::from_ymd_opt(2024, 12, 1).unwrap(),
                NaiveDate::from_ymd_opt(2024, 12, 10).unwrap()
            )
            .send()
            .await
            .unwrap()
    );
}
