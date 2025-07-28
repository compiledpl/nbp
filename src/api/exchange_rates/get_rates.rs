use crate::client::service_client::ServiceClient;
use crate::models::currency_code::CurrencyCode;
use crate::models::date_parameters::{DateParameter, NoDateParameter, WithDateParameter};
use crate::models::exchange_rates::CurrencyExchangeRates;
use crate::models::table_type::TableType;
use crate::nbp_error::NbpResult;
use chrono::NaiveDate;
use std::marker::PhantomData;

pub struct GetRatesBuilder<State> {
    service_client: ServiceClient,
    date_parameter: Option<DateParameter>,
    _state: PhantomData<State>,
}

impl GetRatesBuilder<NoDateParameter> {
    pub fn new(
        mut service_client: ServiceClient,
        table: TableType,
        currency: CurrencyCode,
    ) -> Self {
        service_client
            .join_path(&format!("rates/{}/{}/", table, currency))
            .unwrap();

        Self {
            service_client,
            date_parameter: None,
            _state: PhantomData,
        }
    }

    pub fn today(self) -> GetRatesBuilder<WithDateParameter> {
        GetRatesBuilder {
            service_client: self.service_client,
            date_parameter: Some(DateParameter::Today),
            _state: PhantomData,
        }
    }

    pub fn last_day(self) -> GetRatesBuilder<WithDateParameter> {
        GetRatesBuilder {
            service_client: self.service_client,
            date_parameter: Some(DateParameter::LastDay),
            _state: PhantomData,
        }
    }

    pub fn last_days(self, days_number: u8) -> GetRatesBuilder<WithDateParameter> {
        GetRatesBuilder {
            service_client: self.service_client,
            date_parameter: Some(DateParameter::LastDays(days_number)),
            _state: PhantomData,
        }
    }

    pub fn date(self, date: NaiveDate) -> GetRatesBuilder<WithDateParameter> {
        GetRatesBuilder {
            service_client: self.service_client,
            date_parameter: Some(DateParameter::Date(date)),
            _state: PhantomData,
        }
    }

    pub fn date_range(
        self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> GetRatesBuilder<WithDateParameter> {
        GetRatesBuilder {
            service_client: self.service_client,
            date_parameter: Some(DateParameter::DateRange(start_date, end_date)),
            _state: PhantomData,
        }
    }
}

impl<State> GetRatesBuilder<State> {
    pub async fn send(mut self) -> NbpResult<CurrencyExchangeRates> {
        if let Some(date_param) = self.date_parameter {
            let path_segment = date_param.to_path_segment();
            self.service_client.join_path(&path_segment)?;
        }

        self.service_client.get().await
    }
}
