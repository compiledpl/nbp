use crate::client::service_client::ServiceClient;
use crate::models::date_parameters::{DateParameter, NoDateParameter, WithDateParameter};
use crate::models::gold_prices::GoldPrice;
use crate::nbp_error::NbpResult;
use chrono::NaiveDate;
use std::marker::PhantomData;

pub struct GetGoldPricesBuilder<State> {
    service_client: ServiceClient,
    date_parameter: Option<DateParameter>,
    _state: PhantomData<State>,
}

impl GetGoldPricesBuilder<NoDateParameter> {
    pub fn new(service_client: ServiceClient) -> Self {
        Self {
            service_client,
            date_parameter: None,
            _state: PhantomData,
        }
    }

    pub fn today(self) -> GetGoldPricesBuilder<WithDateParameter> {
        GetGoldPricesBuilder {
            service_client: self.service_client,
            date_parameter: Some(DateParameter::Today),
            _state: PhantomData,
        }
    }

    pub fn last_days(self, days_number: u8) -> GetGoldPricesBuilder<WithDateParameter> {
        GetGoldPricesBuilder {
            service_client: self.service_client,
            date_parameter: Some(DateParameter::LastDays(days_number)),
            _state: PhantomData,
        }
    }

    pub fn date(self, date: NaiveDate) -> GetGoldPricesBuilder<WithDateParameter> {
        GetGoldPricesBuilder {
            service_client: self.service_client,
            date_parameter: Some(DateParameter::Date(date)),
            _state: PhantomData,
        }
    }

    pub fn date_range(
        self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> GetGoldPricesBuilder<WithDateParameter> {
        GetGoldPricesBuilder {
            service_client: self.service_client,
            date_parameter: Some(DateParameter::DateRange(start_date, end_date)),
            _state: PhantomData,
        }
    }
}

impl<State> GetGoldPricesBuilder<State> {
    pub async fn send(mut self) -> NbpResult<Vec<GoldPrice>> {
        if let Some(date_param) = self.date_parameter {
            let path_segment = date_param.to_path_segment();
            self.service_client.join_path(&path_segment)?;
        }

        self.service_client.get().await
    }
}
