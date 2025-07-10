impl NbpCliient {
    /// Creates a new [`GoldPricesHandler`] that allows you to access
    /// NBP Gold Prices API.
    pub fn gold_prices(&self) -> NbpResult<GoldPricesHandler> {
        Ok(GoldPricesHandler::new(
            ServiceClient::builder()
                .base_url(self.service_paths().gold_prices.to_url(self.base_url()))
                .build()?,
        ))
    }}