use crate::client::service_client::ServiceClient;
use crate::models::date_parameters::{DateParameter, NoDateParameter, WithDateParameter};
use crate::models::exchange_rates::CurrencyExchangeTable;
use crate::models::table_type::TableType;
use crate::nbp_error::NbpResult;
use chrono::NaiveDate;
use std::marker::PhantomData;

pub struct GetTablesBuilder<State> {
    route: String,
    service_client: ServiceClient,
    date_parameter: Option<DateParameter>,
    _state: PhantomData<State>,
}

impl GetTablesBuilder<NoDateParameter> {
    pub fn new(service_client: ServiceClient, table_type: TableType) -> Self {
        Self {
            route: format!("tables/{}", table_type),
            service_client: service_client.clone(),
            date_parameter: None,
            _state: PhantomData,
        }
    }

    pub fn today(self) -> GetTablesBuilder<WithDateParameter> {
        GetTablesBuilder {
            route: self.route,
            service_client: self.service_client,
            date_parameter: Some(DateParameter::Today),
            _state: PhantomData,
        }
    }

    pub fn last_day(self) -> GetTablesBuilder<WithDateParameter> {
        GetTablesBuilder {
            route: self.route,
            service_client: self.service_client,
            date_parameter: Some(DateParameter::LastDay),
            _state: PhantomData,
        }
    }

    pub fn last_days(self, days_number: u8) -> GetTablesBuilder<WithDateParameter> {
        GetTablesBuilder {
            route: self.route,
            service_client: self.service_client,
            date_parameter: Some(DateParameter::LastDays(days_number)),
            _state: PhantomData,
        }
    }

    pub fn date(self, date: NaiveDate) -> GetTablesBuilder<WithDateParameter> {
        GetTablesBuilder {
            route: self.route,
            service_client: self.service_client,
            date_parameter: Some(DateParameter::Date(date)),
            _state: PhantomData,
        }
    }

    pub fn date_range(
        self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> GetTablesBuilder<WithDateParameter> {
        GetTablesBuilder {
            route: self.route,
            service_client: self.service_client,
            date_parameter: Some(DateParameter::DateRange(start_date, end_date)),
            _state: PhantomData,
        }
    }
}

impl<State> GetTablesBuilder<State> {
    pub async fn send(self) -> NbpResult<Vec<CurrencyExchangeTable>> {
        self.service_client
            .get(self.route, self.date_parameter)
            .await
    }
}

// TODO error handling
// TODO tests
// TODO code cleanup
// TODO documentation
// TODO
