use crate::client::service_client::ServiceClient;
use crate::models::date_parameters::{DateParameter, NoDateParameter, WithDateParameter};
use crate::models::table_type::TableType;
use crate::nbp_error::{NbpError, NbpResult};
use chrono::NaiveDate;
use std::marker::PhantomData;

pub struct GetTablesBuilder<State> {
    service_client: ServiceClient,
    table_type: TableType,
    date_parameter: Option<DateParameter>,
    top_count: Option<u8>,
    _state: PhantomData<State>,
}

impl GetTablesBuilder<NoDateParameter> {
    pub fn new(service_client: ServiceClient, table_type: TableType) -> Self {
        Self {
            service_client: service_client.clone(),
            table_type,
            date_parameter: None,
            top_count: None,
            _state: PhantomData,
        }
    }

    pub fn top_count(self, count: u8) -> Self {
        GetTablesBuilder {
            service_client: self.service_client,
            table_type: self.table_type,
            date_parameter: self.date_parameter,
            top_count: Some(count),
            _state: PhantomData,
        }
    }

    pub fn today(self) -> GetTablesBuilder<WithDateParameter> {
        GetTablesBuilder {
            service_client: self.service_client,
            table_type: self.table_type,
            date_parameter: Some(DateParameter::Today),
            top_count: self.top_count,
            _state: PhantomData,
        }
    }

    pub fn last_day(self) -> GetTablesBuilder<WithDateParameter> {
        GetTablesBuilder {
            service_client: self.service_client,
            table_type: self.table_type,
            date_parameter: Some(DateParameter::LastDay),
            top_count: self.top_count,
            _state: PhantomData,
        }
    }

    pub fn date(self, date: NaiveDate) -> GetTablesBuilder<WithDateParameter> {
        GetTablesBuilder {
            service_client: self.service_client,
            table_type: self.table_type,
            date_parameter: Some(DateParameter::Date(date)),
            top_count: self.top_count,
            _state: PhantomData,
        }
    }

    pub fn date_range(
        self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> GetTablesBuilder<WithDateParameter> {
        GetTablesBuilder {
            service_client: self.service_client,
            table_type: self.table_type,
            date_parameter: Some(DateParameter::DateRange(start_date, end_date)),
            top_count: self.top_count,
            _state: PhantomData,
        }
    }
}

impl<State> GetTablesBuilder<State> {
    pub async fn send(self) -> NbpResult<String> {
        let mut route = format!("tables/{}", self.table_type);

        match self.date_parameter {
            Some(DateParameter::Today) => route.push_str("/today"),
            Some(DateParameter::LastDay) => route.push_str("/last"),
            Some(DateParameter::Date(date)) => {
                route.push('/');
                route.push_str(&date.to_string());
            }
            Some(DateParameter::DateRange(start_date, end_date)) => {
                route.push('/');
                route.push_str(&start_date.to_string());
                route.push('/');
                route.push_str(&end_date.to_string());
            }
            None => {}
        }

        if let Some(count) = self.top_count {
            route.push_str(&format!("/{count}"));
        }

        let response = self
            .service_client
            .get_http_client()
            .get(self.service_client.get_base_url().join(&route).unwrap())
            .send()
            .await
            .map_err(|e| NbpError::request_failed(e.to_string()))?;

        let result = response
            .text()
            .await
            .map_err(|e| NbpError::cannot_deserialize_body(e.to_string()))?;

        Ok(result)
    }
}
